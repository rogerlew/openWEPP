#!/usr/bin/env python3
"""Harvard-free static validation for the direct CAL-04B executor."""

from __future__ import annotations

import csv
import hashlib
import importlib.util
import sys
from collections import Counter
from pathlib import Path

PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
TOOLS = PACKAGE / "tools"


def rows(name: str) -> list[dict[str, str]]:
    with (ARTIFACTS / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def direct_executor_module():
    spec = importlib.util.spec_from_file_location(
        "cal04b_direct_executor_validation", TOOLS / "execute-prefix.py"
    )
    if spec is None or spec.loader is None:
        raise ValueError("direct executor module cannot be loaded")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def main() -> int:
    inherited_grid = (
        PACKAGE.parent
        / "20260726-canopy-cal-04a-best-available-evidence-daymet-001"
        / "artifacts/proposed-domain-grid.csv"
    )
    require(
        sha(ARTIFACTS / "gsi-domain-grid.csv") == sha(inherited_grid),
        "CAL-04B GSI grid is not byte-identical to CAL-04A",
    )
    candidates = rows("candidate-configurations.csv")
    require(len(candidates) == 9_261, f"candidate count {len(candidates)}")
    require(
        [row["candidate_id"] for row in candidates]
        == [f"GSI-{serial:04d}" for serial in range(1, 9_262)],
        "candidate identifiers/order differ",
    )
    require(
        len(
            {
                (
                    row["temperature_pair_id"],
                    row["vpd_pair_id"],
                    row["photoperiod_pair_id"],
                )
                for row in candidates
            }
        )
        == 9_261,
        "candidate configurations are not unique",
    )
    for row in candidates:
        require(
            float(row["minimum_temperature_inactive_c"])
            < float(row["minimum_temperature_unconstrained_c"])
            and float(row["vapor_pressure_deficit_unconstrained_pa"])
            < float(row["vapor_pressure_deficit_inactive_pa"])
            and float(row["photoperiod_inactive_hours"])
            < float(row["photoperiod_unconstrained_hours"]),
            f"thresholds unordered for {row['candidate_id']}",
        )
    saturation = rows("saturation-evidence.csv")
    require(len(saturation) == 27_783, f"saturation row count {len(saturation)}")
    require(
        Counter(row["candidate_id"] for row in saturation)
        == Counter({row["candidate_id"]: 3 for row in candidates}),
        "saturation evidence is not three families per candidate",
    )
    authority = ARTIFACTS / "calibration-forcing-authority-resolution.md"
    authority_rows = [
        row
        for row in rows("input-and-authority-manifest.csv")
        if row["input_id"] == "calibration_forcing_authority_resolution"
    ]
    require(
        len(authority_rows) == 1
        and authority_rows[0]["role"] == "RESULT_BLIND_BINDING_AUTHORITY"
        and authority_rows[0]["state"] == "PASS"
        and authority_rows[0]["expected_sha256"] == sha(authority)
        and authority_rows[0]["observed_sha256"] == sha(authority),
        "calibration forcing authority resolution is not custody-bound",
    )
    executor = direct_executor_module()
    plan = executor.load_plan(ARTIFACTS / "direct-execution-plan.json")
    nodes = [node for phase in plan["phases"].values() for node in phase]
    require(
        all(node["harvard_access"] == "NONE" for node in nodes[:15])
        and nodes[15]["harvard_access"] == "OPENS_HARVARD"
        and all(node["harvard_access"] == "NONE" for node in nodes[16:]),
        "Harvard policy differs from the direct phase boundary",
    )
    prospective = (
        "execute-prefix.py",
        "publish-results.py",
        "custody.py",
        "freeze.py",
        "freeze-verify.py",
        "holdout.py",
        "summarize.py",
        "validate_preopen.py",
        "validate.py",
    )
    forbidden = (
        "openwepp-" + "gate-plan",
        "openwepp-" + "gate-planner",
        "run-" + "external-transition",
        "publish-" + "external-results",
        "calibration-v1." + "receipt.json",
        "holdout-v1." + "receipt.json",
    )
    for name in prospective:
        source = (TOOLS / name).read_text(encoding="utf-8")
        require(
            all(item not in source for item in forbidden),
            f"retired planner integration remains in {name}",
        )
    native_cases = {row["case_id"] for row in rows("native-proof-case-plan.csv")}
    require(
        len(native_cases) == 12 and "invalid_threshold_order" in native_cases,
        "native proof case inventory differs",
    )
    print(
        f"PASS direct executor candidates={len(candidates)} "
        f"saturation_rows={len(saturation)} commands={len(nodes)}"
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, KeyError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
