from __future__ import annotations

import json
import unittest
from pathlib import Path

TOOLS = Path(__file__).resolve().parent
PACKAGE = TOOLS.parent
PROSPECTIVE = (
    "execute-prefix.py",
    "publish-results.py",
    "custody.py",
    "freeze.py",
    "freeze-verify.py",
    "holdout.py",
    "summarize.py",
    "validate_preopen.py",
    "validate.py",
    "validate_executor.py",
)


class DirectPathTest(unittest.TestCase):
    def test_prospective_tools_have_no_planner_transaction_dependency(self) -> None:
        forbidden = (
            "openwepp-gate-plan",
            "openwepp-gate-planner",
            "run-external-transition",
            "verify-external-transaction",
            "publish-external-results",
            "Generation-B",
            "calibration-v1.receipt.json",
            "holdout-v1.receipt.json",
        )
        for name in PROSPECTIVE:
            source = (TOOLS / name).read_text(encoding="utf-8")
            for value in forbidden:
                self.assertNotIn(value, source, f"{name}: {value}")

    def test_direct_plan_is_non_authoritative_and_harvard_starts_sealed(self) -> None:
        plan = json.loads(
            (PACKAGE / "artifacts/direct-execution-plan.json").read_text()
        )
        self.assertEqual(plan["schema"], "cal04b-direct-execution-plan-v1")
        self.assertFalse(plan["planner_state"])
        self.assertFalse(plan["ci"])
        self.assertEqual(plan["harvard_default"], "SEALED")
        self.assertEqual(
            plan["holdout_sandbox"],
            "BUBBLEWRAP_READ_ONLY_REPOSITORY_AND_CALIBRATION",
        )
        nodes = [
            node for phase in plan["phases"].values() for node in phase
        ]
        self.assertEqual(len(nodes), 18)
        self.assertEqual(len({node["command_id"] for node in nodes}), 18)
        self.assertTrue(all(isinstance(node["argv"], list) for node in nodes))

    def test_holdout_has_separate_writable_output_operand(self) -> None:
        plan = json.loads(
            (PACKAGE / "artifacts/direct-execution-plan.json").read_text()
        )
        holdout = next(
            node
            for node in plan["phases"]["holdout"]
            if node["command_id"] == "holdout"
        )
        self.assertIn("--holdout-output-root", holdout["argv"])
        self.assertIn("${HOLDOUT_OUTPUT_ROOT}", holdout["argv"])
        self.assertNotIn("${PUBLICATION_ROOT}", holdout["declared_outputs"])
        by_id = {node["command_id"]: node for node in plan["phases"]["holdout"]}
        for command_id in ("summarize_post_holdout", "terminal_validate"):
            self.assertIn("--holdout-output-root", by_id[command_id]["argv"])
            self.assertIn("${HOLDOUT_OUTPUT_ROOT}", by_id[command_id]["argv"])

    def test_harvard_read_occurs_only_after_durable_token(self) -> None:
        source = (TOOLS / "holdout.py").read_text(encoding="utf-8")
        run = source.split("def run_sandboxed", 1)[1]
        self.assertLess(
            run.index("create_token(token, digest, command)"),
            run.index("validate_harvard_after_open("),
        )
        token = source.split("def create_token", 1)[1].split("def validate_harvard", 1)[0]
        self.assertLess(token.index("os.fsync(descriptor)"), token.index("fsync_directory(token.parent)"))
        validator = (TOOLS / "validate.py").read_text(encoding="utf-8")
        self.assertNotIn('OBJECTS / "holdout-opened-once.lock"', validator)
        self.assertIn("HOLDOUT_TOKEN.is_file()", validator)


if __name__ == "__main__":
    unittest.main()
