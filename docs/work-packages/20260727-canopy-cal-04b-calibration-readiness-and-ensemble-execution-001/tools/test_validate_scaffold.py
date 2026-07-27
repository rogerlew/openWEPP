#!/usr/bin/env python3
"""Regression tests for CAL-04B scaffold lifecycle validation."""

from __future__ import annotations

import unittest

import validate_scaffold


def freeze_rows(
    *,
    state: str,
    digest: str,
    path: str,
) -> list[dict[str, str]]:
    return [
        {
            "identity_id": f"id-{index}",
            "path_or_command": path,
            "role": "test",
            "sha256": digest,
            "state": state,
        }
        for index in range(16)
    ]


class FreezeLifecycleTests(unittest.TestCase):
    def test_preheavy_pending_is_accepted(self) -> None:
        validate_scaffold.validate_freeze_state(
            freeze_rows(state="SEALED", digest="pending", path="pending"),
            terminal_hold=False,
        )

    def test_terminal_hold_is_accepted(self) -> None:
        validate_scaffold.validate_freeze_state(
            freeze_rows(
                state="SEALED",
                digest="not-applicable",
                path="not-created-native-proof-hold",
            ),
            terminal_hold=True,
        )

    def test_terminal_hold_rejects_mixed_digest(self) -> None:
        rows = freeze_rows(
            state="SEALED",
            digest="not-applicable",
            path="not-created-native-proof-hold",
        )
        rows[-1]["sha256"] = "pending"
        with self.assertRaisesRegex(ValueError, "uniformly not-applicable"):
            validate_scaffold.validate_freeze_state(rows, terminal_hold=True)

    def test_terminal_hold_rejects_unknown_sentinel(self) -> None:
        rows = freeze_rows(
            state="SEALED",
            digest="not-applicable",
            path="not-created-native-proof-hold",
        )
        rows[-1]["path_or_command"] = "pending"
        with self.assertRaisesRegex(ValueError, "unsupported sentinel"):
            validate_scaffold.validate_freeze_state(rows, terminal_hold=True)


if __name__ == "__main__":
    unittest.main()
