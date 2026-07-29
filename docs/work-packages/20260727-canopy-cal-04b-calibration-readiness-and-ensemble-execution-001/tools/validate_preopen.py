#!/usr/bin/env python3
"""Harvard-free semantic verifier for the frozen CAL-04B calibration state."""

from __future__ import annotations

import argparse
import json
import math
import sys
from pathlib import Path

import validate as terminal
from custody import sha256_file

PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
OBJECTS = Path("/home/workdir/cal04b-objects")


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--execution-root", type=Path, required=True)
    parser.add_argument("--custody-root", type=Path, required=True)
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=True)
    custody_root = options.custody_root.resolve(strict=True)
    if not execution_root.is_dir() or not custody_root.is_dir():
        raise ValueError("execution and custody roots must be existing directories")
    attempt_root = execution_root.parent
    global ARTIFACTS, OBJECTS
    ARTIFACTS = (
        attempt_root / "publication" / PACKAGE.relative_to(terminal.ROOT) / "artifacts"
    )
    OBJECTS = execution_root
    terminal.ARTIFACTS = terminal.ReadOverlay(ARTIFACTS, PACKAGE / "artifacts")
    terminal.OBJECTS = OBJECTS
    terminal.EXECUTION_ROOT = attempt_root
    completion_path = attempt_root / "direct-evidence/calibration-complete.json"
    completion = json.loads(completion_path.read_text(encoding="utf-8"))
    require(
        completion.get("state") == "PASS"
        and completion.get("plan_sha256")
        == sha256_file(PACKAGE / "artifacts/direct-execution-plan.json"),
        "direct calibration completion differs",
    )
    authority_rows = terminal.rows("input-and-authority-manifest.csv")
    authority_entry = [
        row
        for row in authority_rows
        if row["input_id"] == "calibration_forcing_authority_resolution"
    ]
    require(
        len(authority_entry) == 1,
        "calibration forcing authority resolution entry differs",
    )
    authority = (terminal.ROOT / authority_entry[0]["path"]).resolve(strict=True)
    try:
        authority.relative_to(terminal.ROOT.resolve())
    except ValueError:
        raise ValueError(
            "calibration forcing authority resolution escapes repository"
        ) from None
    require(
        authority_entry[0]["role"] == "RESULT_BLIND_BINDING_AUTHORITY"
        and authority_entry[0]["state"] == "PASS"
        and authority_entry[0]["expected_sha256"] == sha256_file(authority)
        and authority_entry[0]["observed_sha256"] == sha256_file(authority),
        "calibration forcing authority resolution is not custody-bound",
    )
    plan = terminal.direct_command_plan()
    plan_by_id = {row["command_id"]: row for row in plan}
    candidates = terminal.rows("candidate-ledger.csv")
    accepted = terminal.rows("accepted-calibration-ensemble.csv")
    require(
        [row["candidate_id"] for row in candidates]
        == [f"GSI-{serial:04d}" for serial in range(1, 9_262)],
        "candidate result inventory differs",
    )
    require(bool(accepted), "accepted ensemble is empty")
    finite = [
        float(row["objective"])
        for row in candidates
        if math.isfinite(float(row["objective"]))
    ]
    require(bool(finite), "no finite calibration objective")
    threshold = min(finite) + 1.0
    expected_accepted = [
        row["candidate_id"]
        for row in candidates
        if math.isfinite(float(row["objective"]))
        and float(row["objective"]) <= threshold
    ]
    accepted_ids = [row["candidate_id"] for row in accepted]
    require(
        accepted_ids == expected_accepted
        and all(abs(float(row["acceptance_threshold"]) - threshold) <= 1.0e-10 for row in accepted),
        "accepted ensemble is not the complete minimum-plus-one set",
    )

    trace_identity = terminal.validate_calibration_semantics(candidates, accepted, plan_by_id)
    primary = terminal.field_map(OBJECTS / "primary/reconstruction-receipt.csv")
    verification = terminal.field_map(OBJECTS / "verification/verification-receipt.csv")
    for command_id, receipt in (
        ("hubbard_primary_reconstruct", primary),
        ("hubbard_verify_reconstruct", verification),
    ):
        require(
            receipt["state"] == "PASS"
            and receipt["exact_command"] == plan_by_id[command_id]["argv"]
            and Path(receipt["source_path"]) == Path(plan_by_id[command_id]["source_path"])
            and sha256_file(Path(receipt["source_path"])) == receipt["source_sha256"]
            and sha256_file(Path(receipt["binary_path"])) == receipt["binary_sha256"]
            and receipt["objective_grouping"]
            == "equal_year_mean_of_all_admitted_record_squared_distances"
            and receipt["crossing_eligibility_yday"] == "60-180"
            and receipt["state_initialization"] == "FRESH_GSI_STATE_EACH_CANDIDATE_PLOT_YEAR",
            f"{command_id} execution receipt differs",
        )
    for name in (
        "candidate-ledger.csv",
        "accepted-calibration-ensemble.csv",
        "failure-ledger.csv",
    ):
        require(
            sha256_file(ARTIFACTS / name)
            == sha256_file(OBJECTS / "verification" / name),
            f"dual reconstruction differs for {name}",
        )

    synthetic_primary = terminal.field_map(
        OBJECTS / "synthetic-primary/primary-reconstruction-receipt.csv"
    )
    synthetic_verification = terminal.field_map(
        OBJECTS / "synthetic-verification/verification-reconstruction-receipt.csv"
    )
    synthetic_common = (
        "state",
        "case_id",
        "trace_sha256",
        "hidden_candidate",
        "hidden_objective",
        "recovered_set",
        "nonvacuous_competitor",
        "components_sha256",
        "annual_sha256",
        "candidate_ledger_sha256",
        "accepted_ensemble_sha256",
    )
    require(
        all(synthetic_primary.get(key) == synthetic_verification.get(key) for key in synthetic_common)
        and synthetic_primary["state"] == "PASS"
        and synthetic_primary["hidden_candidate"] == "GSI-5557"
        and synthetic_primary["hidden_objective"] == "0.000000000000"
        and synthetic_primary["nonvacuous_competitor"] == "TRUE"
        and synthetic_verification["exact_primary_match"] == "TRUE",
        "dual synthetic reconstruction differs",
    )
    native = terminal.rows("native-consumer-proof.csv")
    require(
        len(native) == 12
        and all(row["state"].startswith("PASS") for row in native)
        and all(
            row["case_id"] == "invalid_threshold_order"
            or int(row["compared_values"]) == int(row["compared_days"]) * 8
            for row in native
        ),
        "native production-consumer proof differs",
    )

    retention = terminal.rows("trace-retention.csv")
    require(len(retention) == 1 and retention[0]["state"] == "PASS", "retention receipt differs")
    compressed = Path(retention[0]["compressed_path"])
    require(
        retention[0]["schema"] == "CAL04B03"
        and retention[0]["value_count"] == str(terminal.TRACE_VALUE_COUNT)
        and retention[0]["raw_bytes"] == str(terminal.TRACE_BYTES)
        and retention[0]["decompressed_bytes"] == str(terminal.TRACE_BYTES)
        and retention[0]["exact_command"] == plan_by_id["retain_trace"]["argv"]
        and retention[0]["raw_sha256"] == trace_identity["trace_sha256"]
        and terminal.zstd_expanded_sha(compressed) == trace_identity["trace_sha256"],
        "retained compressed trace differs",
    )

    later_results = terminal.rows("later-stage-results.csv")
    result_hash = sha256_file(ARTIFACTS / "later-stage-results.csv")
    recovery = {row["design_id"]: row for row in terminal.rows("later-stage-recovery.csv")}
    hidden = {
        "REC-BFBS-01": "Bf=0.20;Bs=0.10",
        "REC-FE-01": "fe=0.50",
        "REC-LAI-01": "xmxlai=6.00",
        "REC-CSBB-01": "Cs=0.20;bb=5.00",
    }
    require(set(recovery) == set(hidden), "later recovery inventory differs")
    for design_id, truth in hidden.items():
        design_rows = [
            row
            for row in later_results
            if row["design_id"] == design_id
            and row["evidence_role"] == "ASSUMED_FOR_EXECUTION"
            and row["failure"] == "NONE"
        ]
        minimum = min(float(row["objective"]) for row in design_rows)
        recovered = [
            row["operand_values"].replace(";", "_")
            for row in design_rows
            if abs(float(row["objective"]) - minimum) <= 1.0e-15
        ]
        expected_status = "RECOVERED_UNIQUE" if len(recovered) == 1 else "RECOVERED_EQUIFINAL"
        require(
            recovery[design_id]["hidden_truth"] == truth
            and recovery[design_id]["recovered_set"] == "|".join(recovered)
            and recovery[design_id]["recovery_status"] == expected_status
            and recovery[design_id]["results_sha256"] == result_hash
            and truth.replace(";", "_") in recovered,
            f"later recovery differs for {design_id}",
        )

    membership = terminal.rows("later-stage-membership.csv")
    require(len(membership) == 4, "readiness membership index differs")
    for row in membership:
        terminal.validate_stage_propagation(row, accepted_ids, result_hash)
    readiness = terminal.field_map(OBJECTS / "readiness/execution-receipt.csv")
    require(
        readiness["state"] == "PASS"
        and readiness["exact_command"] == plan_by_id["readiness"]["argv"]
        and readiness["results_sha256"] == result_hash
        and readiness["membership_index_sha256"]
        == sha256_file(ARTIFACTS / "later-stage-membership.csv"),
        "readiness execution receipt differs",
    )
    require(
        len(terminal.rows("stage-status-ledger.csv")) == 5
        and "Status: `PASS`"
        in (ARTIFACTS / "calibration-readiness-matrix.md").read_text(encoding="utf-8"),
        "pre-Harvard readiness disposition differs",
    )
    print(
        f"PASS preopen_semantics candidates=9261 accepted={len(accepted_ids)} "
        f"membership_stages={len(membership)}"
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, KeyError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
