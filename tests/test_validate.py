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


if __name__ == "__main__":
    unittest.main()
