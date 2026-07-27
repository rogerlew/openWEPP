"""Fail-closed tests for the bounded observed-prefix coordinator."""

from __future__ import annotations

import importlib.util
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).with_name("execute-prefix.py")
SPEC = importlib.util.spec_from_file_location("cal04b_execute_prefix", MODULE_PATH)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("unable to load execute-prefix.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class ExecutePrefixTest(unittest.TestCase):
    def rows(self) -> list[dict[str, str]]:
        return [
            {"command_id": command_id}
            for command_id in (*MODULE.AUTHORIZED_PREFIX, "freeze", "holdout")
        ]

    def test_exact_prefix_is_selected(self) -> None:
        selected = MODULE.select_prefix(self.rows())
        self.assertEqual(
            tuple(row["command_id"] for row in selected),
            MODULE.AUTHORIZED_PREFIX,
        )

    def test_injected_preterminal_command_is_rejected(self) -> None:
        rows = self.rows()
        rows.insert(4, {"command_id": "out_of_scope_probe"})
        with self.assertRaisesRegex(ValueError, "exact authorized prefix"):
            MODULE.select_prefix(rows)

    def test_prefix_must_be_followed_by_freeze(self) -> None:
        rows = self.rows()
        rows[len(MODULE.AUTHORIZED_PREFIX)] = {"command_id": "holdout"}
        with self.assertRaisesRegex(ValueError, "followed immediately by freeze"):
            MODULE.select_prefix(rows)


if __name__ == "__main__":
    unittest.main()

