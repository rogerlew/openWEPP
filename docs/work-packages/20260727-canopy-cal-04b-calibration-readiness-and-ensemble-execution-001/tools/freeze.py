#!/usr/bin/env python3
"""Create the canonical noncircular CAL-04B holdout freeze manifest."""

from __future__ import annotations

import csv
import subprocess
import sys
from pathlib import Path

from custody import sha256_file, validate_freeze
import observe

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
OBJECTS = Path("/home/workdir/cal04b-objects")


def sha(path: Path) -> str:
    return sha256_file(path)


def write_bundle(path: Path, entries: list[tuple[str, Path]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.writer(stream, lineterminator="\n")
        writer.writerow(["identity", "path", "sha256"])
        for identity, item in sorted(entries):
            if not item.is_file():
                raise ValueError(f"missing freeze input {item}")
            writer.writerow([identity, str(item), sha(item)])


def authenticated_inputs() -> list[tuple[str, Path]]:
    manifest = ARTIFACTS / "input-and-authority-manifest.csv"
    entries: list[tuple[str, Path]] = [
        ("authority_manifest", manifest),
        ("candidate_configurations", ARTIFACTS / "candidate-configurations.csv"),
        ("gsi_domain_grid", ARTIFACTS / "gsi-domain-grid.csv"),
    ]
    with manifest.open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream))
    if not rows:
        raise ValueError("input authority manifest is empty")
    for row in rows:
        path = ROOT / row["path"]
        observed = sha(path)
        if (
            row["state"] != "PASS"
            or row["expected_sha256"] != row["observed_sha256"]
            or row["observed_sha256"] != observed
        ):
            raise ValueError(f"unauthenticated freeze input {row['input_id']}")
        entries.append((row["input_id"], path))
    return entries


def readiness_entries() -> list[tuple[str, Path]]:
    index = ARTIFACTS / "later-stage-membership.csv"
    entries = [
        ("source", PACKAGE / "tools/executor/src/bin/readiness.rs"),
        ("binary", PACKAGE / "tools/executor/target/release/readiness"),
        ("design", ARTIFACTS / "later-stage-design.csv"),
        ("results", ARTIFACTS / "later-stage-results.csv"),
        ("membership_index", index),
        ("gsi_recovery", ARTIFACTS / "synthetic-recovery-results.csv"),
        ("later_stage_recovery", ARTIFACTS / "later-stage-recovery.csv"),
        ("saturation_evidence", ARTIFACTS / "saturation-evidence.csv"),
        ("saturation_window_inventory", ARTIFACTS / "saturation-window-inventory.csv"),
    ]
    with index.open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream))
    if len(rows) != 4:
        raise ValueError("readiness membership index must contain four stages")
    for row in rows:
        stage = row["stage"]
        for prefix in ("membership", "parent_results"):
            path = Path(row[f"{prefix}_path"])
            if sha(path) != row[f"{prefix}_sha256"]:
                raise ValueError(f"readiness {prefix} identity differs for {stage}")
            entries.append((f"{stage}_{prefix}", path))
    return entries


def synthetic_entries() -> list[tuple[str, Path]]:
    executor = PACKAGE / "tools/executor/target/release"
    primary = OBJECTS / "synthetic-primary"
    verification = OBJECTS / "synthetic-verification"
    entries = [
        ("synthetic_orchestrator", PACKAGE / "tools/synthetic-gsi.py"),
        ("synthetic_trace", OBJECTS / "synthetic-gsi.bin"),
        ("synthetic_identity", OBJECTS / "synthetic-gsi-identity.csv"),
    ]
    for stem in (
        "synthetic-trace",
        "synthetic-reconstruct",
        "synthetic-verify-reconstruct",
    ):
        entries.extend(
            [
                (f"{stem}_binary", executor / stem),
                (f"{stem}_source", PACKAGE / f"tools/executor/src/bin/{stem.replace('-', '_')}.rs"),
            ]
        )
    for label, directory, receipt in (
        ("synthetic_primary", primary, "primary-reconstruction-receipt.csv"),
        ("synthetic_verification", verification, "verification-reconstruction-receipt.csv"),
    ):
        for name in (
            "candidate-observation-components.csv",
            "candidate-annual-components.csv",
            "candidate-ledger.csv",
            "accepted-synthetic-ensemble.csv",
            receipt,
        ):
            entries.append((f"{label}_{name}", directory / name))
    return entries


def native_proof_entries() -> list[tuple[str, Path]]:
    plan = ARTIFACTS / "native-proof-case-plan.csv"
    result = ARTIFACTS / "native-consumer-proof.csv"
    entries = [
        ("native_proof_source", PACKAGE / "tools/native-proof.py"),
        ("native_proof_plan", plan),
        ("native_proof_result", result),
        ("native_proof_runner", ROOT / "target/debug/openwepp-cli-hill"),
        ("native_proof_expected_probe", PACKAGE / "tools/executor/target/release/expected-probe"),
    ]
    with plan.open(newline="", encoding="utf-8") as stream:
        cases = list(csv.DictReader(stream))
    for case in cases:
        case_id = case["case_id"]
        entries.extend(
            [
                (f"native_{case_id}_management", Path(case["workdir"]) / "p10.man.yaml"),
                (f"native_{case_id}_stdout", Path(case["stdout_log"])),
                (f"native_{case_id}_stderr", Path(case["stderr_log"])),
            ]
        )
        if case_id != "invalid_threshold_order":
            entries.append((f"native_{case_id}_trace", Path(case["trace_path"])))
    return entries


def main() -> int:
    if (OBJECTS / "holdout-opened-once.lock").exists():
        raise ValueError("refusing to regenerate freeze after Harvard was opened")
    accepted = ARTIFACTS / "accepted-calibration-ensemble.csv"
    with accepted.open(newline="", encoding="utf-8") as stream:
        if sum(1 for _ in csv.DictReader(stream)) == 0:
            raise ValueError("accepted ensemble is empty")
    OBJECTS.mkdir(parents=True, exist_ok=True)
    bundles = OBJECTS / "freeze-bundles"
    bundles.mkdir(exist_ok=True)
    executor = PACKAGE / "tools/executor/target/release"
    input_bundle = bundles / "authenticated-inputs.csv"
    write_bundle(input_bundle, authenticated_inputs())
    primary_bundle = bundles / "primary-components.csv"
    write_bundle(primary_bundle, [
        ("candidate_ledger", ARTIFACTS / "candidate-ledger.csv"),
        ("accepted", accepted),
        ("failure", ARTIFACTS / "failure-ledger.csv"),
        ("crossing_components", OBJECTS / "primary/candidate-crossing-components.csv"),
        ("observation_components", OBJECTS / "primary/candidate-observation-components.csv"),
        ("annual_components", OBJECTS / "primary/candidate-annual-components.csv"),
        ("diagnostics", OBJECTS / "primary/candidate-diagnostics.csv"),
        ("receipt", OBJECTS / "primary/reconstruction-receipt.csv"),
    ])
    verification_bundle = bundles / "verification-components.csv"
    write_bundle(verification_bundle, [
        ("candidate_ledger", OBJECTS / "verification/candidate-ledger.csv"),
        ("accepted", OBJECTS / "verification/accepted-calibration-ensemble.csv"),
        ("failure", OBJECTS / "verification/failure-ledger.csv"),
        ("crossing_components", OBJECTS / "verification/candidate-crossing-components.csv"),
        ("observation_components", OBJECTS / "verification/candidate-observation-components.csv"),
        ("annual_components", OBJECTS / "verification/candidate-annual-components.csv"),
        ("diagnostics", OBJECTS / "verification/candidate-diagnostics.csv"),
        ("receipt", OBJECTS / "verification/verification-receipt.csv"),
    ])
    producer_bundle = bundles / "producer.csv"
    producer_entries = [
        ("source", PACKAGE / "tools/executor/src/bin/native_producer.rs"),
        ("binary", executor / "native-producer"),
        ("trace", OBJECTS / "hubbard-gsi.bin"),
        ("trace_calendar", OBJECTS / "hubbard-gsi.calendar.csv"),
        ("trace_lanes", OBJECTS / "hubbard-gsi.lanes.csv"),
        ("trace_identity", OBJECTS / "hubbard-gsi-identity.csv"),
        ("trace_retention", ARTIFACTS / "trace-retention.csv"),
        ("trace_zstd", OBJECTS / "hubbard-gsi.bin.zst"),
        ("failures", ARTIFACTS / "producer-failure-ledger.csv"),
        ("executor_cargo_lock", PACKAGE / "tools/executor/Cargo.lock"),
        ("executor_cargo_toml", PACKAGE / "tools/executor/Cargo.toml"),
        ("executor_library", PACKAGE / "tools/executor/src/lib.rs"),
        ("expected_probe_source", PACKAGE / "tools/executor/src/bin/expected_probe.rs"),
        ("retain_source", PACKAGE / "tools/retain.py"),
    ]
    producer_entries.extend(synthetic_entries())
    producer_entries.extend(native_proof_entries())
    write_bundle(producer_bundle, producer_entries)
    recon_bundle = bundles / "primary-reconstructor.csv"
    write_bundle(recon_bundle, [
        ("source", PACKAGE / "tools/executor/src/bin/reconstruct.rs"),
        ("binary", executor / "reconstruct"),
    ])
    verify_bundle = bundles / "verification-reconstructor.csv"
    write_bundle(verify_bundle, [
        ("source", PACKAGE / "tools/executor/src/bin/verify_reconstruct.rs"),
        ("binary", executor / "verify-reconstruct"),
    ])
    readiness_bundle = bundles / "readiness.csv"
    readiness_inputs = readiness_entries()
    readiness_inputs.extend(
        [
            ("readiness_execution_receipt", OBJECTS / "readiness/execution-receipt.csv"),
            ("calibration_readiness_matrix", ARTIFACTS / "calibration-readiness-matrix.md"),
            ("stage_status_ledger", ARTIFACTS / "stage-status-ledger.csv"),
            ("additional_data_inventory", ARTIFACTS / "additional-data-inventory.csv"),
            ("identifiability_disposition", ARTIFACTS / "identifiability-and-equifinality.md"),
        ]
    )
    write_bundle(readiness_bundle, readiness_inputs)
    holdout_bundle = bundles / "holdout-command.csv"
    write_bundle(holdout_bundle, [
        ("script", PACKAGE / "tools/holdout.py"),
        ("producer_source", PACKAGE / "tools/executor/src/bin/holdout_producer.rs"),
        ("producer_binary", executor / "holdout-producer"),
        ("reconstructor_source", PACKAGE / "tools/executor/src/bin/holdout_reconstruct.rs"),
        ("reconstructor_binary", executor / "holdout-reconstruct"),
        ("command_plan", ARTIFACTS / "executor-command-plan.csv"),
        ("observed_command_contract", ARTIFACTS / "observed-command-contract.csv"),
    ])
    observed_rows = observe.validate_snapshot(
        "pre-freeze",
        "summarize_pre_freeze",
    )
    observed_entries: list[tuple[str, Path]] = [
        ("observed_pre_freeze_snapshot", observe.LEDGER / "pre-freeze-snapshot.csv")
    ]
    for row in observed_rows:
        command_id = row["command_id"]
        receipt = Path(row["receipt_path"])
        receipt_row = observe.read_receipt(receipt)
        observed_entries.extend([
            (f"observed_{command_id}_receipt", receipt),
            (f"observed_{command_id}_stdout", Path(receipt_row["stdout_path"])),
            (f"observed_{command_id}_stderr", Path(receipt_row["stderr_path"])),
            (
                f"observed_{command_id}_outputs",
                Path(receipt_row["output_manifest_path"]),
            ),
        ])
    custody_bundle = bundles / "freeze-custody-controls.csv"
    custody_entries = [
        ("custody_library", PACKAGE / "tools/custody.py"),
        ("freeze_script", PACKAGE / "tools/freeze.py"),
        ("verifier_script", PACKAGE / "tools/freeze-verify.py"),
        ("holdout_script", PACKAGE / "tools/holdout.py"),
        ("observed_runner", PACKAGE / "tools/observe.py"),
        ("observed_prefix_coordinator", PACKAGE / "tools/execute-prefix.py"),
        ("observed_runner_test", PACKAGE / "tools/test_observe.py"),
        ("observed_command_contract", ARTIFACTS / "observed-command-contract.csv"),
        ("observed_execution_procedure", ARTIFACTS / "observed-execution-procedure.md"),
        (
            "calibration_forcing_authority_resolution",
            ARTIFACTS / "calibration-forcing-authority-resolution.md",
        ),
        ("preopen_validator", PACKAGE / "tools/validate_preopen.py"),
        ("summarize_script", PACKAGE / "tools/summarize.py"),
        ("terminal_validator", PACKAGE / "tools/validate.py"),
        ("executor_validator", PACKAGE / "tools/validate_executor.py"),
    ]
    custody_entries.extend(observed_entries)
    write_bundle(custody_bundle, custody_entries)
    toolchain = bundles / "toolchain.txt"
    with toolchain.open("w", encoding="utf-8") as stream:
        for command in (["rustc", "--version"], ["cargo", "--version"], [str(ROOT / ".venv/bin/python"), "--version"], ["zstd", "--version"]):
            result = subprocess.run(command, capture_output=True, text=True, check=True)
            stream.write((result.stdout or result.stderr).strip() + "\n")
    expected = ARTIFACTS / "harvard-expected-input-manifest.csv"
    harvard_paths = [
        "docs/work-packages/20260726-canopy-cal-04-05-authority-evidence-admission-001/artifacts/cal04-timing-windows.csv",
        "tests/fixtures/cancov_forest/harvard_deciduous_ma/p6.native.run.toml",
        "tests/fixtures/cancov_forest/harvard_deciduous_ma/p6.man.yaml",
        "tests/fixtures/cancov_forest/harvard_deciduous_ma/p6.cli",
        "tests/fixtures/cancov_forest/harvard_deciduous_ma/p6.sol",
        "tests/fixtures/cancov_forest/harvard_deciduous_ma/p6.slp",
    ]
    with expected.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.writer(stream, lineterminator="\n")
        writer.writerow(["path", "expected_git_blob", "state"])
        for item in harvard_paths:
            output = subprocess.run(
                ["git", "ls-tree", "HEAD", "--", item],
                cwd=ROOT,
                capture_output=True,
                text=True,
                check=True,
            ).stdout.strip()
            blob = output.split()[2] if output else ""
            if not blob:
                raise ValueError(f"missing committed expected Git identity for {item}")
            writer.writerow([item, blob, "EXPECTED_COMMITTED_PREOPEN_NOT_READ"])

    timing = ROOT / "docs/work-packages/20260726-canopy-cal-04a-best-available-evidence-daymet-001/artifacts/phenology-forcing-join.csv"
    operator = ROOT / "docs/work-packages/20260726-canopy-cal-04-process-calibration-identifiability-001/artifacts/objective-and-observation-operator.md"
    command_plan = ARTIFACTS / "executor-command-plan.csv"
    rows = [
        ("accepted_ensemble", accepted, "calibration_output"),
        ("candidate_configurations", input_bundle, "frozen_configuration_and_authenticated_inputs"),
        ("calibration_daily_trace", OBJECTS / "hubbard-gsi.bin", "raw_native_evidence"),
        ("calibration_trace_identity", OBJECTS / "hubbard-gsi-identity.csv", "trace_identity_sidecar"),
        ("timing_observation_ledger", timing, "calibration_and_holdout_authority"),
        ("objective_operator", operator, "frozen_operator"),
        ("primary_component_ledgers", primary_bundle, "reconstructed_calibration_results"),
        ("verification_component_ledgers", verification_bundle, "independent_reconstruction_results"),
        ("producer_source_binary", producer_bundle, "immutable_producer"),
        ("primary_reconstructor_source_binary", recon_bundle, "immutable_primary_reconstructor"),
        ("verification_reconstructor_source_binary", verify_bundle, "immutable_verification_reconstructor"),
        ("readiness_source_binary_results", readiness_bundle, "ordered_stage_identity"),
        ("freeze_source_command", custody_bundle, "canonical_freeze_and_verifier_identity"),
        ("holdout_source_binary_command", holdout_bundle, "one_time_holdout_identity"),
        ("harvard_expected_input_manifest", expected, "expected_identity_without_content_read"),
        ("toolchain_identity", toolchain, "rust_python_zstd_identity"),
    ]
    manifest = ARTIFACTS / "holdout-freeze-manifest.csv"
    with manifest.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.writer(stream, lineterminator="\n")
        writer.writerow(["identity_id", "path_or_command", "role", "sha256", "state"])
        for identity, path, role in sorted(rows):
            writer.writerow([identity, str(path), role, sha(path), "FROZEN"])
    digest = sha(manifest)
    (ARTIFACTS / "holdout-freeze-digest.txt").write_text(digest + "\n", encoding="ascii")
    verified_digest, members = validate_freeze(
        manifest,
        ARTIFACTS / "holdout-freeze-digest.txt",
        bundles,
    )
    print(f"PASS freeze_digest={verified_digest} transitive_members={members}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
