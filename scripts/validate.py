#!/usr/bin/env python3
"""Validate the template's own configuration without third-party packages."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CONFIG_FILES = (
    ".devcontainer/devcontainer.json",
    ".vscode/extensions.json",
    ".vscode/settings.json",
)
WORKFLOW_DIR = Path(".github/workflows")
ACTION_REFERENCE = re.compile(r"^[0-9a-fA-F]{40}$")


def validate_json_files() -> list[str]:
    errors: list[str] = []

    for relative_path in CONFIG_FILES:
        path = ROOT / relative_path
        if not path.exists():
            continue

        try:
            json.loads(path.read_text(encoding="utf-8"))
        except (OSError, UnicodeError, json.JSONDecodeError) as exc:
            errors.append(f"{relative_path}: invalid JSON ({exc})")

    return errors


def validate_action_pins() -> list[str]:
    errors: list[str] = []
    workflow_root = ROOT / WORKFLOW_DIR

    if not workflow_root.exists():
        return errors

    for path in sorted((*workflow_root.rglob("*.yml"), *workflow_root.rglob("*.yaml"))):
        try:
            lines = path.read_text(encoding="utf-8").splitlines()
        except (OSError, UnicodeError) as exc:
            errors.append(f"{path.relative_to(ROOT)}: cannot read workflow ({exc})")
            continue

        for line_number, line in enumerate(lines, start=1):
            stripped = line.strip()
            if not stripped.startswith("uses:"):
                continue

            reference = stripped.split(":", 1)[1].split("#", 1)[0].strip().strip("'\"")
            if reference.startswith(("./", "docker://")):
                continue

            if "@" not in reference:
                errors.append(
                    f"{path.relative_to(ROOT)}:{line_number}: "
                    f"Action reference is not pinned: {reference}"
                )
                continue

            revision = reference.rsplit("@", 1)[1]
            if not ACTION_REFERENCE.fullmatch(revision):
                errors.append(
                    f"{path.relative_to(ROOT)}:{line_number}: "
                    f"Action must use a full 40-character commit SHA: {reference}"
                )

    return errors


def validate_atlas() -> list[str]:
    """Validate the connected Moonbyte World Loom design graph."""
    errors: list[str] = []
    atlas_root = ROOT / "docs" / "atlas"
    expected_volumes = (
        "01_hearth_bazaar_and_communities.md",
        "02_atlas_expeditions_and_story.md",
        "03_crafting_collections_and_arcade.md",
        "04_characters_mastery_and_homesteads.md",
        "05_creation_competition_and_trust.md",
    )
    volume_lines = 0
    volume_text = ""

    for name in expected_volumes:
        path = atlas_root / name
        if not path.is_file():
            errors.append(f"docs/atlas/{name}: missing World Loom volume")
            continue
        try:
            content = path.read_text(encoding="utf-8")
        except (OSError, UnicodeError) as exc:
            errors.append(f"docs/atlas/{name}: cannot read volume ({exc})")
            continue
        volume_lines += len(content.splitlines())
        volume_text += content

    if volume_lines < 20_000:
        errors.append(
            f"docs/atlas: expected at least 20,000 design lines, found {volume_lines}"
        )

    graph_path = atlas_root / "world_graph.json"
    graph: dict[str, object] | None = None
    try:
        parsed = json.loads(graph_path.read_text(encoding="utf-8"))
        if not isinstance(parsed, dict):
            raise ValueError("graph root must be an object")
        graph = parsed
    except (OSError, UnicodeError, json.JSONDecodeError, ValueError) as exc:
        errors.append(f"docs/atlas/world_graph.json: invalid graph ({exc})")

    if graph is not None:
        nodes = graph.get("nodes")
        edges = graph.get("edges")
        if not isinstance(nodes, list) or not isinstance(edges, list):
            errors.append("docs/atlas/world_graph.json: nodes and edges must be arrays")
        else:
            node_ids = {
                node.get("id")
                for node in nodes
                if isinstance(node, dict) and isinstance(node.get("id"), str)
            }
            if len(node_ids) != len(nodes):
                errors.append("docs/atlas/world_graph.json: node IDs are missing or duplicated")
            idea_nodes = [
                node for node in nodes
                if isinstance(node, dict) and node.get("type") == "idea"
            ]
            if len(idea_nodes) != 200:
                errors.append(
                    f"docs/atlas/world_graph.json: expected 200 ideas, found {len(idea_nodes)}"
                )
            outgoing: dict[str, int] = {}
            for edge in edges:
                if not isinstance(edge, dict):
                    errors.append("docs/atlas/world_graph.json: edge must be an object")
                    continue
                source = edge.get("from")
                target = edge.get("to")
                if source not in node_ids or target not in node_ids:
                    errors.append(
                        f"docs/atlas/world_graph.json: dangling edge {source!r} -> {target!r}"
                    )
                    continue
                outgoing[source] = outgoing.get(source, 0) + 1
                if not isinstance(edge.get("rationale"), str) or not edge["rationale"].strip():
                    errors.append("docs/atlas/world_graph.json: every edge needs a rationale")
            for node in idea_nodes:
                node_id = node.get("id")
                if outgoing.get(node_id, 0) < 4:
                    errors.append(
                        f"docs/atlas/world_graph.json: {node_id} has fewer than four outgoing links"
                    )
                if f"# {node_id} — " not in volume_text:
                    errors.append(
                        f"docs/atlas: graph idea {node_id} has no matching volume specification"
                    )
            if graph.get("idea_count") != 200 or graph.get("domain_count") != 20:
                errors.append("docs/atlas/world_graph.json: declared counts do not match the atlas")

    map_path = atlas_root / "world_map.mmd"
    try:
        map_content = map_path.read_text(encoding="utf-8")
        if "mindmap" not in map_content or "flowchart" not in map_content:
            errors.append("docs/atlas/world_map.mmd: expected both hierarchy and relationship diagrams")
    except (OSError, UnicodeError) as exc:
        errors.append(f"docs/atlas/world_map.mmd: cannot read map ({exc})")

    index_path = atlas_root / "README.md"
    try:
        index_content = index_path.read_text(encoding="utf-8")
        if "world_graph.json" not in index_content or "200 individual" not in index_content:
            errors.append("docs/atlas/README.md: missing graph or idea-count documentation")
    except (OSError, UnicodeError) as exc:
        errors.append(f"docs/atlas/README.md: cannot read index ({exc})")

    return errors


def main() -> int:
    errors = validate_json_files()
    errors.extend(validate_action_pins())
    errors.extend(validate_atlas())

    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 1

    print("Repository configuration checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
