from __future__ import annotations

import importlib.util
import tempfile
import unittest
from argparse import Namespace
from pathlib import Path
from unittest.mock import patch

TOOLS = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location(
    "cal04b_execute_prefix", TOOLS / "execute-prefix.py"
)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class ExecutePrefixTest(unittest.TestCase):
    def options(self, root: Path, transaction: str = "calibration-v1") -> Namespace:
        return Namespace(
            execution_root=root,
            transaction_id=transaction,
            principal="worker",
            repository="openwepp/openwepp",
            source_event="local",
            source_ref="refs/heads/main",
            workflow="cal04b",
            job="external-transition",
            runner="forest1",
            attempt=1,
            control_root=root.with_name(f"{root.name}.control"),
            custody_root=root.with_name(f"{root.name}.custody"),
        )

    def test_exact_transaction_command_is_argument_array(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            command = MODULE.command(self.options(root), root / "planner")
        self.assertEqual(command[1], "run-external-transition")
        self.assertIn("calibration-v1", command)
        self.assertNotIn("--opening-token", command)

    def test_holdout_binds_custody_and_opening_token(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            options = self.options(root, "holdout-v1")
            options.custody_root.mkdir()
            command = MODULE.command(options, root / "planner")
        self.assertIn("--custody-root", command)
        self.assertIn("--opening-token", command)
        self.assertIn(str(options.custody_root / "holdout-opened-once.lock"), command)

    def test_planner_build_uses_attempt_local_cargo_target(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            with patch.object(MODULE.subprocess, "run") as run:
                binary = MODULE.planner_binary(root)
            environment = run.call_args.kwargs["env"]
        target = root.with_name(f"{root.name}.planner-target")
        self.assertEqual(environment["CARGO_TARGET_DIR"], str(target))
        self.assertEqual(binary, target / "debug/openwepp-gate-plan")


if __name__ == "__main__":
    unittest.main()
