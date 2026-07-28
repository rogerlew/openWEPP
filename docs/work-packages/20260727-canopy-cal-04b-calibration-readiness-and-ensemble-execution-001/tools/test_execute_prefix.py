from __future__ import annotations

import importlib.util
import json
import shutil
import copy
import sys
import tempfile
import unittest
from pathlib import Path

TOOLS = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("cal04b_execute_prefix", TOOLS / "execute-prefix.py")
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)


def node(command_id: str, order: int, exit_code: int) -> dict[str, object]:
    return {
        "command_id": command_id,
        "order": order,
        "argv": [sys.executable, "-c", f"raise SystemExit({exit_code})"],
        "cwd": "${REPO}",
        "env": {"PYTHONDONTWRITEBYTECODE": "1"},
        "source_path": str(Path(sys.executable)),
        "declared_outputs": [],
        "harvard_access": "NONE",
        "timeout_seconds": 30,
    }


class ExecutePrefixTest(unittest.TestCase):
    def test_plan_is_direct_and_contains_literal_argv(self) -> None:
        plan = MODULE.load_plan()
        self.assertFalse(plan["planner_state"])
        self.assertFalse(plan["ci"])
        self.assertEqual(len(plan["phases"]["calibration"]), 11)
        self.assertTrue(
            all(isinstance(item["argv"], list) for item in plan["phases"]["calibration"])
        )

    def test_first_failure_is_durable_and_cleanup_does_not_touch_it(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            attempt = Path(temporary) / "attempt"
            attempt.mkdir()
            context = MODULE.prepare_context(attempt / "objects")
            plan = {
                "phases": {
                    "calibration": [node("pass", 1, 0), node("fail", 2, 17)]
                }
            }
            with self.assertRaises(RuntimeError):
                MODULE.run_calibration(context, plan)
            failure = context.evidence_root / "primary-failure.json"
            value = json.loads(failure.read_text(encoding="utf-8"))
            self.assertEqual(value["command_id"], "fail")
            self.assertEqual(value["exit_code"], 17)
            self.assertEqual(value["state"], "FAIL")
            self.assertTrue(Path(value["stdout_path"]).is_file())
            self.assertTrue(Path(value["stderr_path"]).is_file())
            shutil.rmtree(context.publication_root)
            self.assertEqual(json.loads(failure.read_text())["command_id"], "fail")

    def test_fresh_execution_root_is_required(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            existing = Path(temporary) / "objects"
            existing.mkdir()
            with self.assertRaises(ValueError):
                MODULE.prepare_context(existing)

    def test_launch_error_is_primary_failure_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            attempt = Path(temporary) / "attempt"
            attempt.mkdir()
            context = MODULE.prepare_context(attempt / "objects")
            broken = node("missing", 1, 0)
            broken["argv"] = [str(attempt / "does-not-exist")]
            record = MODULE.execute_node(broken, context)
            self.assertEqual(record["state"], "ERROR")
            self.assertIn("FileNotFoundError", record["error"])
            self.assertTrue((context.evidence_root / "primary-failure.json").is_file())

    def test_prospective_executor_has_no_planner_or_shell_launch(self) -> None:
        source = (TOOLS / "execute-prefix.py").read_text(encoding="utf-8")
        for forbidden in (
            "openwepp-gate-plan",
            "openwepp-gate-planner",
            "run-external-transition",
            "shell=True",
        ):
            self.assertNotIn(forbidden, source)

    def test_plan_rejects_reorder_unknown_executable_and_harvard_drift(self) -> None:
        value = json.loads(MODULE.PLAN.read_text())
        mutations = []
        reordered = copy.deepcopy(value)
        reordered["phases"]["calibration"][0], reordered["phases"]["calibration"][1] = (
            reordered["phases"]["calibration"][1],
            reordered["phases"]["calibration"][0],
        )
        mutations.append(reordered)
        executable = copy.deepcopy(value)
        executable["phases"]["calibration"][0]["argv"][0] = "/tmp/arbitrary"
        mutations.append(executable)
        harvard = copy.deepcopy(value)
        harvard["phases"]["calibration"][0]["harvard_access"] = "OPENS_HARVARD"
        mutations.append(harvard)
        prerequisite = copy.deepcopy(value)
        prerequisite["phases"]["calibration"][1]["prerequisites"] = []
        mutations.append(prerequisite)
        with tempfile.TemporaryDirectory() as temporary:
            for index, mutation in enumerate(mutations):
                path = Path(temporary) / f"{index}.json"
                path.write_text(json.dumps(mutation))
                with self.assertRaises(ValueError):
                    MODULE.load_plan(path)


if __name__ == "__main__":
    unittest.main()
