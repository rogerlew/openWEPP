#!/usr/bin/env python3
"""Validate the persistent CAL-04B scaffold and control invariants."""

from __future__ import annotations

import csv
import sys
from pathlib import Path

PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
TERMINAL_HOLD_STATUS = "Status: `COMPLETE / HOLD / PRODUCTION PARAMETER PATH BLOCKED`"
TERMINAL_HOLD_PATH_VALUES = {
    "not-created-native-proof-hold",
    "not-frozen-native-proof-hold",
    "not-run-native-proof-hold",
    "not-opened-native-proof-hold",
}


def read_csv(name: str) -> list[dict[str, str]]:
    with (ARTIFACTS / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def validate_freeze_state(
    freeze: list[dict[str, str]],
    *,
    terminal_hold: bool,
) -> None:
    if terminal_hold:
        require(
            {row["state"] for row in freeze} == {"SEALED"},
            "terminal-HOLD freeze identities must remain uniformly sealed",
        )
        require(
            {row["sha256"] for row in freeze} == {"not-applicable"},
            "terminal-HOLD freeze identities must remain uniformly not-applicable",
        )
        require(
            {row["path_or_command"] for row in freeze} <= TERMINAL_HOLD_PATH_VALUES,
            "terminal-HOLD freeze paths contain an unsupported sentinel",
        )
        return
    require(
        {row["state"] for row in freeze} in ({"SEALED"}, {"FROZEN"}),
        "freeze snapshot identities must be uniformly sealed or frozen",
    )
    require(
        all(row["sha256"] == "pending" for row in freeze)
        or all(len(row["sha256"]) == 64 for row in freeze),
        "freeze identities must be uniformly pending or frozen",
    )


def main() -> int:
    findings = read_csv("prospective-finding-ledger.csv")
    freeze = read_csv("holdout-freeze-manifest.csv")
    require(len(findings) == 14, "prospective finding ledger must contain 14 rows")
    require(len({row["finding_id"] for row in findings}) == 14, "finding IDs must be unique")
    require({row["disposition"] for row in findings} == {"accepted"}, "every prior finding must remain accepted")
    require({row["status"] for row in findings} == {"CONTROL_ACCEPTED"}, "every prior finding must have an accepted control")
    require(len(freeze) == 16, "holdout freeze manifest must contain 16 identities")
    package_text = (PACKAGE / "package.md").read_text(encoding="utf-8")
    terminal_hold = TERMINAL_HOLD_STATUS in package_text
    validate_freeze_state(freeze, terminal_hold=terminal_hold)
    if terminal_hold:
        active = PACKAGE / "prompts/active/kickoff.md"
        archived = PACKAGE / "prompts/archived/kickoff.md"
        require(not active.exists(), "terminal-HOLD kickoff must not remain active")
        kickoff_text = archived.read_text(encoding="utf-8")
        for name in (
            "execution-incident-004.md",
            "hold-legitimacy-audit.md",
            "worker-handoff.md",
        ):
            require((ARTIFACTS / name).is_file(), f"terminal-HOLD evidence missing: {name}")
        verifier_states = {
            row["state"] for row in read_csv("freeze-verifier-receipts.csv")
        }
        require(
            verifier_states == {"SEALED_NATIVE_PROOF_HOLD"},
            "terminal-HOLD freeze-verifier rows must remain uniformly sealed",
        )
    else:
        kickoff_text = (PACKAGE / "prompts/active/kickoff.md").read_text(
            encoding="utf-8"
        )
    require("Harvard" in package_text and "OPENED_ONCE" in package_text, "package lacks one-shot holdout control")
    require("Harvard" in kickoff_text, "kickoff lacks Harvard control")
    lifecycle = "TERMINAL_HOLD" if terminal_hold else "PRE_HEAVY"
    print(
        f"PASS scaffold findings=14 holdout_snapshot=SEALED "
        f"controls=accepted lifecycle={lifecycle}"
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
