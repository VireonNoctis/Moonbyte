from pathlib import Path
import runpy
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[1]
VALIDATOR = runpy.run_path(str(ROOT / "scripts" / "validate.py"))
validate_action_reference = VALIDATOR["validate_action_reference"]
validate_action_pins = VALIDATOR["validate_action_pins"]


class ActionPinValidationTests(unittest.TestCase):
    def test_external_actions_require_full_commit_shas(self):
        valid_sha = "a" * 40
        self.assertIsNone(validate_action_reference(f"actions/checkout@{valid_sha}"))
        self.assertIsNotNone(validate_action_reference("actions/checkout@v4"))
        self.assertIsNotNone(validate_action_reference("actions/checkout@abc123"))

    def test_local_actions_remain_supported(self):
        self.assertIsNone(validate_action_reference("./.github/actions/local-check"))

    def test_docker_actions_require_full_sha256_digests(self):
        digest = "b" * 64
        self.assertIsNone(
            validate_action_reference(f"docker://ghcr.io/example/action@sha256:{digest}")
        )
        self.assertIsNotNone(
            validate_action_reference("docker://ghcr.io/example/action:latest")
        )
        self.assertIsNotNone(
            validate_action_reference("docker://ghcr.io/example/action@sha256:short")
        )

    def test_shorthand_and_expanded_uses_entries_are_both_checked(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            workflow = root / ".github" / "workflows" / "ci.yml"
            workflow.parent.mkdir(parents=True)
            workflow.write_text(
                """jobs:
  validate:
    steps:
      - uses: actions/checkout@v4
      - name: Setup
        uses: actions/setup-python@v5
      - uses: docker://ghcr.io/example/action:latest
      - uses: ./.github/actions/local-check
""",
                encoding="utf-8",
            )

            errors = validate_action_pins(root)

        self.assertEqual(len(errors), 3)
        self.assertIn(":4:", errors[0])
        self.assertIn(":6:", errors[1])
        self.assertIn(":7:", errors[2])

    def test_committed_workflows_are_pinned(self):
        self.assertEqual(validate_action_pins(ROOT), [])




validate_visual_catalogue = VALIDATOR["validate_visual_catalogue"]


class VisualCatalogueValidationTests(unittest.TestCase):
    def write_fixture(
        self, root: Path, rust_ids: list[str], catalogue_ids: list[str]
    ) -> None:
        model = root / "src" / "game" / "model.rs"
        catalogue = root / "visual-library" / "src" / "lib" / "catalog.ts"
        model.parent.mkdir(parents=True)
        catalogue.parent.mkdir(parents=True)
        model.write_text(
            "\n".join(f'GameDefinition {{ id: "{game_id}" }}' for game_id in rust_ids),
            encoding="utf-8",
        )
        catalogue.write_text(
            "\n".join(f'    id: "{game_id}",' for game_id in catalogue_ids),
            encoding="utf-8",
        )

    def test_committed_visual_catalogue_matches_rust_game_definitions(self):
        self.assertEqual(validate_visual_catalogue(ROOT), [])

    def test_missing_extra_and_duplicate_game_ids_are_reported(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.write_fixture(root, ["alpha", "beta"], ["alpha", "alpha", "gamma"])
            errors = validate_visual_catalogue(root)

        self.assertTrue(any("duplicate game IDs: alpha" in error for error in errors))
        self.assertTrue(
            any("missing implemented game IDs: beta" in error for error in errors)
        )
        self.assertTrue(any("unknown game IDs: gamma" in error for error in errors))


class CargoLockPolicyTests(unittest.TestCase):
    def test_ci_uses_the_committed_lockfile_without_regenerating_it(self):
        workflow = (ROOT / ".github" / "workflows" / "ci.yml").read_text(
            encoding="utf-8"
        )
        self.assertIn("git ls-files --error-unmatch Cargo.lock", workflow)
        self.assertIn("cargo test --workspace --locked", workflow)
        self.assertIn(
            "cargo clippy --workspace --all-targets --locked -- -D warnings",
            workflow,
        )
        self.assertNotIn("cargo generate-lockfile", workflow)
        self.assertNotIn("Upload resolved Cargo.lock", workflow)

if __name__ == "__main__":
    unittest.main()
