#!/usr/bin/env python3
"""Fail-closed pre-heavy validation for the frozen CAL-04B executor."""

from __future__ import annotations

import csv
import sys
from collections import Counter
import hashlib
import shlex
from pathlib import Path

PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
TOOLS = PACKAGE / "tools"
OBJECTS = Path("/home/workdir/cal04b-objects")


def rows(name: str) -> list[dict[str, str]]:
    with (ARTIFACTS / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def exact_options(argv: str) -> tuple[str, dict[str, str]]:
    tokens = shlex.split(argv)
    if len(tokens) < 3 or len(tokens[1:]) % 2 != 0:
        raise ValueError("semantic command argv is not binary plus option/value pairs")
    options = dict(zip(tokens[1::2], tokens[2::2]))
    if len(options) * 2 != len(tokens) - 1:
        raise ValueError("semantic command argv has duplicate options")
    return tokens[0], options


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
        len({row["configuration_id"] if "configuration_id" in row else (
            row["temperature_pair_id"],
            row["vpd_pair_id"],
            row["photoperiod_pair_id"],
        ) for row in candidates}) == 9_261,
        "candidate configurations are not unique",
    )
    for row in candidates:
        require(
            float(row["minimum_temperature_inactive_c"])
            < float(row["minimum_temperature_unconstrained_c"]),
            f"temperature thresholds unordered for {row['candidate_id']}",
        )
        require(
            float(row["vapor_pressure_deficit_unconstrained_pa"])
            < float(row["vapor_pressure_deficit_inactive_pa"]),
            f"VPD thresholds unordered for {row['candidate_id']}",
        )
        require(
            float(row["photoperiod_inactive_hours"])
            < float(row["photoperiod_unconstrained_hours"]),
            f"photoperiod thresholds unordered for {row['candidate_id']}",
        )

    saturation = rows("saturation-evidence.csv")
    require(len(saturation) == 27_783, f"saturation row count {len(saturation)}")
    require(
        Counter(row["candidate_id"] for row in saturation)
        == Counter({row["candidate_id"]: 3 for row in candidates}),
        "saturation evidence is not exactly three families per candidate",
    )
    require(
        {row["family"] for row in saturation} == {"temperature", "vpd", "photoperiod"},
        "saturation family inventory differs",
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

    commands = rows("executor-command-plan.csv")
    expected_commands = [
        "prepare",
        "build_executor",
        "build_production_runner",
        "native_proof",
        "synthetic_gsi",
        "hubbard_producer",
        "hubbard_primary_reconstruct",
        "hubbard_verify_reconstruct",
        "retain_trace",
        "readiness",
        "summarize_pre_freeze",
        "freeze",
        "freeze_verify_a",
        "freeze_verify_b",
        "freeze_barrier",
        "holdout",
        "summarize_post_holdout",
        "terminal_validate",
    ]
    require([row["command_id"] for row in commands] == expected_commands, "command DAG differs")
    observed_contract = rows("observed-command-contract.csv")
    require(
        [row["command_id"] for row in observed_contract] == expected_commands
        and all(row["prerequisites"] and row["receipt_outputs"] for row in observed_contract),
        "observed execution contract does not exactly cover the command DAG",
    )
    receipt_outputs = [
        path
        for row in observed_contract
        for path in row["receipt_outputs"].split(";")
        if path and path != "-"
    ]
    require(
        len(receipt_outputs) == len(set(receipt_outputs)),
        "observed output ownership overlaps across command receipts",
    )
    command_by_id = {row["command_id"]: row for row in commands}
    package_path = (
        "docs/work-packages/"
        "20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001"
    )
    cal04a = (
        "docs/work-packages/"
        "20260726-canopy-cal-04a-best-available-evidence-daymet-001/artifacts"
    )
    expected_calibration_outputs = {
        "hubbard_producer": {
            "/home/workdir/cal04b-objects/hubbard-gsi.bin",
            "/home/workdir/cal04b-objects/hubbard-gsi.calendar.csv",
            "/home/workdir/cal04b-objects/hubbard-gsi.lanes.csv",
            "/home/workdir/cal04b-objects/hubbard-gsi-identity.csv",
            f"{package_path}/artifacts/producer-failure-ledger.csv",
        },
        "hubbard_primary_reconstruct": {
            "/home/workdir/cal04b-objects/primary/candidate-crossing-components.csv",
            "/home/workdir/cal04b-objects/primary/candidate-observation-components.csv",
            "/home/workdir/cal04b-objects/primary/candidate-annual-components.csv",
            "/home/workdir/cal04b-objects/primary/candidate-diagnostics.csv",
            "/home/workdir/cal04b-objects/primary/reconstruction-receipt.csv",
            f"{package_path}/artifacts/candidate-ledger.csv",
            f"{package_path}/artifacts/accepted-calibration-ensemble.csv",
            f"{package_path}/artifacts/failure-ledger.csv",
        },
        "hubbard_verify_reconstruct": {
            "/home/workdir/cal04b-objects/verification/candidate-crossing-components.csv",
            "/home/workdir/cal04b-objects/verification/candidate-observation-components.csv",
            "/home/workdir/cal04b-objects/verification/candidate-annual-components.csv",
            "/home/workdir/cal04b-objects/verification/candidate-diagnostics.csv",
            "/home/workdir/cal04b-objects/verification/candidate-ledger.csv",
            "/home/workdir/cal04b-objects/verification/accepted-calibration-ensemble.csv",
            "/home/workdir/cal04b-objects/verification/failure-ledger.csv",
            "/home/workdir/cal04b-objects/verification/verification-receipt.csv",
        },
    }
    contract_by_id = {row["command_id"]: row for row in observed_contract}
    require(
        all(
            set(contract_by_id[command_id]["receipt_outputs"].split(";"))
            == expected_outputs
            for command_id, expected_outputs in expected_calibration_outputs.items()
        ),
        "observed calibration output ownership differs from the exact immutable set",
    )
    producer_binary, producer_options = exact_options(
        command_by_id["hubbard_producer"]["argv"]
    )
    require(
        producer_binary == f"{package_path}/tools/executor/target/release/native-producer"
        and producer_options
        == {
            "--configs": f"{package_path}/artifacts/candidate-configurations.csv",
            "--forcing": f"{cal04a}/daymet-daily-derived.csv",
            "--geometry": f"{cal04a}/hubbard-plot-geometry.csv",
            "--source-manifest": f"{cal04a}/source-and-request-manifest.csv",
            "--authority-manifest": f"{package_path}/artifacts/input-and-authority-manifest.csv",
            "--forcing-authority-resolution": (
                f"{package_path}/artifacts/calibration-forcing-authority-resolution.md"
            ),
            "--trace": "/home/workdir/cal04b-objects/hubbard-gsi.bin",
            "--identity": "/home/workdir/cal04b-objects/hubbard-gsi-identity.csv",
            "--failures": f"{package_path}/artifacts/producer-failure-ledger.csv",
        },
        "Hubbard producer argv is not the exact nine-plot Daymet authority command",
    )
    primary_binary, primary_options = exact_options(
        command_by_id["hubbard_primary_reconstruct"]["argv"]
    )
    require(
        primary_binary == f"{package_path}/tools/executor/target/release/reconstruct"
        and primary_options
        == {
            "--trace": "/home/workdir/cal04b-objects/hubbard-gsi.bin",
            "--identity": "/home/workdir/cal04b-objects/hubbard-gsi-identity.csv",
            "--configs": f"{package_path}/artifacts/candidate-configurations.csv",
            "--observations": f"{cal04a}/phenology-forcing-join.csv",
            "--out": f"{package_path}/artifacts",
        },
        "primary reconstruction argv differs from the exact plot-keyed command",
    )
    verification_binary, verification_options = exact_options(
        command_by_id["hubbard_verify_reconstruct"]["argv"]
    )
    require(
        verification_binary
        == f"{package_path}/tools/executor/target/release/verify-reconstruct"
        and verification_options
        == {
            "--trace": "/home/workdir/cal04b-objects/hubbard-gsi.bin",
            "--identity": "/home/workdir/cal04b-objects/hubbard-gsi-identity.csv",
            "--configs": f"{package_path}/artifacts/candidate-configurations.csv",
            "--observations": f"{cal04a}/phenology-forcing-join.csv",
            "--primary-components": "/home/workdir/cal04b-objects/primary",
            "--primary-ledgers": f"{package_path}/artifacts",
            "--out": "/home/workdir/cal04b-objects/verification",
        },
        "verification reconstruction argv differs from the exact independent command",
    )
    require(
        all(
            "p10.cli" not in command_by_id[command_id]["argv"]
            and "--climate" not in shlex.split(command_by_id[command_id]["argv"])
            for command_id in (
                "hubbard_producer",
                "hubbard_primary_reconstruct",
                "hubbard_verify_reconstruct",
            )
        ),
        "protected p10 composite climate re-entered calibration argv",
    )
    holdout_index = expected_commands.index("holdout")
    require(
        [row["command_id"] for row in commands if row["harvard_access"] == "OPENED_ONCE"]
        == ["holdout"],
        "holdout command is not the exclusive Harvard opener",
    )
    require(
        all(
            row["harvard_access"] in {"FORBIDDEN", "EXPECTED_IDENTITIES_ONLY"}
            for row in commands[:holdout_index]
        ),
        "pre-holdout command has Harvard content access",
    )
    require(
        all(row["harvard_access"] == "OPENED_RESULTS_ONLY" for row in commands[holdout_index + 1 :]),
        "post-holdout command access is not limited to opened results",
    )

    native_cases = rows("native-proof-case-plan.csv")
    require(
        {row["case_id"] for row in native_cases}
        == {
            "native_default",
            "interior",
            "double_boundary",
            "saturated_first",
            "all_operands",
            "perturb_bf_max",
            "perturb_bs",
            "perturb_fe",
            "perturb_xmxlai",
            "perturb_cs",
            "perturb_bb",
            "invalid_threshold_order",
        },
        "native proof cases differ",
    )
    required_tools = {
        "prepare.py",
        "native-proof.py",
        "synthetic-gsi.py",
        "retain.py",
        "observe.py",
        "execute-prefix.py",
        "test_observe.py",
        "summarize.py",
        "validate.py",
        "validate_preopen.py",
        "freeze.py",
        "freeze-verify.py",
        "holdout.py",
    }
    require(all((TOOLS / name).is_file() for name in required_tools), "required Python tool missing")
    native_proof_source = (TOOLS / "native-proof.py").read_text(encoding="utf-8")
    require(
        'target/release/expected-probe"' in native_proof_source
        and 'target/release/expected_probe"' not in native_proof_source,
        "native proof does not resolve the declared expected-probe Cargo binary",
    )
    prefix_source = (TOOLS / "execute-prefix.py").read_text(encoding="utf-8")
    require(
        "AUTHORIZED_PREFIX = (" in prefix_source
        and 'ids[prefix_length] != "freeze"' in prefix_source
        and "shell=True" not in prefix_source
        and "eval(" not in prefix_source
        and "exec(" not in prefix_source,
        "observed prefix coordinator is not statically bounded and shell-free",
    )
    required_bins = {
        "native_producer.rs",
        "reconstruct.rs",
        "verify_reconstruct.rs",
        "readiness.rs",
        "expected_probe.rs",
        "synthetic_trace.rs",
        "holdout_producer.rs",
        "holdout_reconstruct.rs",
    }
    bin_dir = TOOLS / "executor/src/bin"
    require(all((bin_dir / name).is_file() for name in required_bins), "required Rust binary source missing")
    require(
        not (OBJECTS / "holdout-opened-once.lock").exists(),
        "one-shot Harvard token already exists before heavy execution",
    )
    print(
        f"PASS executor candidates={len(candidates)} "
        f"saturation_rows={len(saturation)} commands={len(commands)}"
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, KeyError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
