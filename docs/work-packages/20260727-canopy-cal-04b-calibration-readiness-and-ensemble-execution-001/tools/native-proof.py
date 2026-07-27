#!/usr/bin/env python3
"""Prove copied configurations reach the real production canopy consumer."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import os
import shutil
import struct
import subprocess
import sys
from datetime import date, timedelta
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
SOURCE_ARTIFACTS = PACKAGE / "artifacts"
ARTIFACTS = SOURCE_ARTIFACTS
FIXTURE = ROOT / "tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh"
RUNNER = Path("/nonexistent/cal04b-execution-root-required")
EXPECTED = Path("/nonexistent/cal04b-execution-root-required")

GSI_KEYS = (
    "minimum_temperature_inactive_c", "minimum_temperature_unconstrained_c",
    "vapor_pressure_deficit_unconstrained_pa", "vapor_pressure_deficit_inactive_pa",
    "photoperiod_inactive_hours", "photoperiod_unconstrained_hours",
)
CANOPY_KEYS = (
    "summer_foliar_biomass_kg_m2",
    "structural_biomass_kg_m2",
    "evergreen_fraction",
    "xmxlai",
    "structural_canopy_cover_fraction",
    "bb",
)
YAML_KEYS = GSI_KEYS + CANOPY_KEYS
TRACE_FIELDS = (
    ("gsi", "gsi21", "gsi21"),
    ("canopy", "evergreen_foliar_biomass_kg_m2", "evergreen_biomass"),
    ("canopy", "deciduous_foliar_biomass_kg_m2", "deciduous_biomass"),
    ("canopy", "total_foliar_biomass_kg_m2", "foliar_biomass"),
    ("canopy", "structural_biomass_kg_m2", "structural_biomass"),
    ("canopy", "total_aboveground_live_biomass_kg_m2", "total_aboveground_biomass"),
    ("canopy", "leaf_area_index_m2_m2", "lai"),
    ("canopy", "cover_fraction", "cover"),
)
VALID_EXPECTATIONS = {
    "bit_exact_gsi_and_canopy",
    "bit_exact_all_native_canopy_fields",
}
ISOLATED_CASE_FIELDS = {
    "perturb_bf_max": "summer_foliar_biomass_kg_m2",
    "perturb_bs": "structural_biomass_kg_m2",
    "perturb_fe": "evergreen_fraction",
    "perturb_xmxlai": "xmxlai",
    "perturb_cs": "structural_canopy_cover_fraction",
    "perturb_bb": "bb",
}
CASE_FIELDS = (
    "case_id",
    "selector",
    "gsi_candidate_id",
    "overrides",
    "workdir",
    "run_file",
    "output_dir",
    "trace_path",
    "stdout_log",
    "stderr_log",
    "expected",
)


def has_typed_temperature_threshold_order_error(error_text: str) -> bool:
    """Recognize both runtime and canonical intake forms of the typed error."""
    normalized = error_text.casefold()
    return "temperature" in normalized and (
        "lower threshold must be less than upper threshold" in normalized
        or (
            "minimum_temperature_inactive_c must be less than "
            "minimum_temperature_unconstrained_c"
        )
        in normalized
    )


def replace_yaml(path: Path, values: dict[str, float]) -> None:
    output = []
    seen = set()
    for line in path.read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        key = stripped.split(":", 1)[0]
        if key in values:
            if key in seen:
                raise ValueError(f"duplicate YAML key {key}")
            indent = line[: len(line) - len(line.lstrip())]
            output.append(f"{indent}{key}: {values[key]:.12g}")
            seen.add(key)
        else:
            output.append(line)
    if seen != set(values):
        raise ValueError(f"missing YAML keys {set(values) - seen}")
    path.write_text("\n".join(output) + "\n", encoding="utf-8")


def yaml_values(path: Path) -> dict[str, float]:
    wanted = set(YAML_KEYS)
    values: dict[str, float] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        if ":" not in stripped:
            continue
        key, value = stripped.split(":", 1)
        if key in wanted:
            if key in values:
                raise ValueError(f"duplicate native YAML key {key}")
            try:
                values[key] = float(value.strip())
            except ValueError:
                raise ValueError(f"non-numeric native YAML value for {key}") from None
            if not math.isfinite(values[key]):
                raise ValueError(f"nonfinite native YAML value for {key}")
    if wanted != set(values):
        raise ValueError(f"failed to read native YAML values: {sorted(wanted - set(values))}")
    return values


def configs() -> tuple[dict[str, dict[str, str]], str]:
    with (ARTIFACTS / "candidate-configurations.csv").open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream))
    index = {row["candidate_id"]: row for row in rows}
    saturated = next(row["candidate_id"] for row in rows if row["saturation_flags"] != "NONE")
    return index, saturated


def bits(value: float) -> bytes:
    return struct.pack("<d", value)


def sha(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def climate_calendar(path: Path) -> list[date]:
    """Read the CLIGEN daily calendar without trusting the expected probe."""
    calendar: list[date] = []
    for line_number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        fields = line.split()
        if len(fields) < 3:
            continue
        try:
            day_value, month_value, year_value = map(int, fields[:3])
        except ValueError:
            continue
        if not 1900 <= year_value <= 2200:
            continue
        if len(fields) != 13:
            raise ValueError(f"malformed climate daily row at line {line_number}")
        try:
            current = date(year_value, month_value, day_value)
            measurements = tuple(float(value) for value in fields[3:])
        except ValueError as error:
            raise ValueError(f"malformed climate daily row at line {line_number}: {error}") from None
        if not all(math.isfinite(value) for value in measurements):
            raise ValueError(f"nonfinite climate daily row at line {line_number}")
        if calendar and current != calendar[-1] + timedelta(days=1):
            raise ValueError(
                f"climate calendar gap/duplicate/nonchronology at line {line_number}: {current}"
            )
        calendar.append(current)
    if not calendar:
        raise ValueError("climate has no daily rows")
    return calendar


def parse_overrides(case: dict[str, str]) -> dict[str, float]:
    overrides: dict[str, float] = {}
    for item in case["overrides"].split(";"):
        if not item:
            continue
        if item.count("=") != 1:
            raise ValueError(f"{case['case_id']} malformed override {item!r}")
        key, raw_value = item.split("=", 1)
        if key not in YAML_KEYS or key in overrides:
            raise ValueError(f"{case['case_id']} invalid/duplicate override {key!r}")
        try:
            overrides[key] = float(raw_value)
        except ValueError:
            raise ValueError(f"{case['case_id']} nonnumeric override {key!r}") from None
        if not math.isfinite(overrides[key]):
            raise ValueError(f"{case['case_id']} nonfinite override {key!r}")
    return overrides


def validate_case_plan(cases: list[dict[str, str]]) -> None:
    ids = [case["case_id"] for case in cases]
    required = {
        "native_default",
        "interior",
        "double_boundary",
        "saturated_first",
        "all_operands",
        "invalid_threshold_order",
        *ISOLATED_CASE_FIELDS,
    }
    if len(ids) != len(set(ids)) or set(ids) != required:
        raise ValueError(f"native proof case inventory differs: {ids}")
    path_fields = ("workdir", "output_dir", "trace_path", "stdout_log", "stderr_log")
    for field in path_fields:
        values = [case[field] for case in cases]
        if len(values) != len(set(values)):
            raise ValueError(f"native proof {field} values are not unique")
    proof_roots = {Path(case["workdir"]).resolve().parent for case in cases}
    if len(proof_roots) != 1:
        raise ValueError("native proof workdirs do not share one planned object root")
    proof_root = next(iter(proof_roots))
    emitted_paths: set[Path] = set()
    for case in cases:
        if set(case) != set(CASE_FIELDS) or not all(
            isinstance(value, str) for value in case.values()
        ):
            raise ValueError("native proof case row/schema differs")
        case_id = case["case_id"]
        if any("harvard" in value.casefold() for value in case.values()):
            raise ValueError(f"{case_id} violates native-proof Harvard prohibition")
        overrides = parse_overrides(case)
        expectation = case["expected"]
        workdir = Path(case["workdir"]).resolve()
        try:
            workdir.relative_to(proof_root)
        except ValueError:
            raise ValueError(f"{case_id} workdir escapes native-proof object root") from None
        for field in ("output_dir", "trace_path", "stdout_log", "stderr_log"):
            output_path = Path(case[field]).resolve()
            try:
                output_path.relative_to(workdir)
            except ValueError:
                raise ValueError(f"{case_id} {field} escapes its workdir") from None
            if output_path in emitted_paths:
                raise ValueError(f"{case_id} reuses emitted path {output_path}")
            emitted_paths.add(output_path)
        candidate_id = case["gsi_candidate_id"]
        if case_id == "interior" and candidate_id != "GSI-5557":
            raise ValueError("interior case is not frozen GSI-5557")
        if case_id == "double_boundary" and candidate_id != "GSI-0001":
            raise ValueError("double-boundary case is not frozen GSI-0001")
        if case_id not in {"interior", "double_boundary"} and candidate_id:
            raise ValueError(f"{case_id} has an unexpected fixed GSI candidate")
        if case_id == "saturated_first" and not case["selector"].startswith("lowest candidate"):
            raise ValueError("saturated case selector differs")
        if case_id == "invalid_threshold_order":
            expected_overrides = {
                "minimum_temperature_inactive_c",
                "minimum_temperature_unconstrained_c",
            }
            if expectation != "typed_threshold_order_failure_and_trace_absent":
                raise ValueError("invalid threshold case expectation differs")
            if set(overrides) != expected_overrides:
                raise ValueError("invalid threshold case must alter only its threshold pair")
        else:
            if expectation not in VALID_EXPECTATIONS:
                raise ValueError(f"{case_id} has unsupported expectation {expectation!r}")
            if case_id in ISOLATED_CASE_FIELDS:
                if set(overrides) != {ISOLATED_CASE_FIELDS[case_id]}:
                    raise ValueError(f"{case_id} is not a one-at-a-time perturbation")
            elif case_id == "all_operands":
                if set(overrides) != set(CANOPY_KEYS):
                    raise ValueError("all_operands must alter all six canopy operands")
            elif overrides:
                raise ValueError(f"{case_id} has undeclared overrides")


def compare(trace: Path, expected: Path, calendar: list[date]) -> tuple[int, int]:
    with expected.open(newline="", encoding="utf-8") as expected_stream:
        expected_reader = csv.DictReader(expected_stream)
        if tuple(expected_reader.fieldnames or ()) != (
            "year", "ordinal", "gsi21", "evergreen_biomass", "deciduous_biomass",
            "foliar_biomass", "structural_biomass", "total_aboveground_biomass",
            "lai", "cover",
        ):
            raise ValueError("expected-probe schema differs")
        expected_rows = list(expected_reader)
    if len(expected_rows) != len(calendar):
        raise ValueError(
            f"expected-probe row count {len(expected_rows)} differs from climate {len(calendar)}"
        )
    trace_lines = trace.read_text(encoding="utf-8").splitlines()
    if len(trace_lines) != len(calendar):
        raise ValueError(f"trace row count {len(trace_lines)} differs from climate {len(calendar)}")

    for index, (line, reference, current) in enumerate(
        zip(trace_lines, expected_rows, calendar, strict=True)
    ):
        expected_ordinal = (current - date(current.year, 1, 1)).days + 1
        if int(reference["year"]) != current.year or int(reference["ordinal"]) != expected_ordinal:
            raise ValueError(f"expected-probe calendar differs at {current}")
        try:
            record = json.loads(line)
        except json.JSONDecodeError as error:
            raise ValueError(f"invalid trace JSON at row {index + 1}: {error}") from None
        if not isinstance(record, dict):
            raise ValueError(f"trace row {index + 1} is not a JSON object")
        required_identity = {
            "schema": "openwepp-canopy-research-daily-v1",
            "date": current.isoformat(),
            "year": current.year,
            "day_of_year": expected_ordinal,
            "day_index": index,
            "lane_index": 0,
            "site_id": "hubbard_brook",
            "arm_id": "deciduous",
        }
        for key, value in required_identity.items():
            if type(record.get(key)) is not type(value) or record.get(key) != value:
                raise ValueError(
                    f"trace identity/calendar mismatch at {current}: {key}={record.get(key)!r}"
                )
        for group, key, expected_key in TRACE_FIELDS:
            try:
                raw_actual = record[group][key]
                if type(raw_actual) not in {int, float}:
                    raise TypeError
                actual_value = float(raw_actual)
                reference_value = float(reference[expected_key])
            except (KeyError, TypeError, ValueError):
                raise ValueError(f"missing/non-numeric trace field at {current}: {group}/{key}") from None
            if not math.isfinite(actual_value) or not math.isfinite(reference_value):
                raise ValueError(f"nonfinite comparison field at {current}: {group}/{key}")
            if bits(actual_value) != bits(reference_value):
                raise ValueError(f"bit mismatch at {current} {group}/{key}")
    return len(trace_lines), len(trace_lines) * len(TRACE_FIELDS)


def remap_case_path(value: str, planned_workdir: str, execution_root: Path) -> Path:
    planned_root = Path(planned_workdir).parent
    return execution_root / "objects/native-proof" / Path(value).relative_to(planned_root)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--case-plan", required=True)
    parser.add_argument("--execution-root", type=Path, required=True)
    options = parser.parse_args(argv)
    execution_root = options.execution_root.resolve(strict=True)
    if not execution_root.is_dir():
        raise ValueError("execution root must be an existing directory")
    global ARTIFACTS, RUNNER, EXPECTED
    ARTIFACTS = execution_root.parent / "publication" / PACKAGE.relative_to(ROOT) / "artifacts"
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    RUNNER = execution_root.parent / "cargo-target/debug/openwepp-cli-hill"
    EXPECTED = execution_root.parent / "cargo-target/release/expected-probe"
    config_index, saturated = configs()
    with Path(options.case_plan).open(newline="", encoding="utf-8") as stream:
        reader = csv.DictReader(stream)
        if tuple(reader.fieldnames or ()) != CASE_FIELDS:
            raise ValueError("native proof case-plan schema differs")
        cases = list(reader)
    validate_case_plan(cases)
    results = []
    for case in cases:
        workdir = remap_case_path(case["workdir"], case["workdir"], execution_root.parent)
        if workdir.exists():
            raise ValueError(f"refusing existing case workdir {workdir}")
        shutil.copytree(FIXTURE, workdir)
        management = workdir / "p10.man.yaml"
        values = yaml_values(management)
        native_values = values.copy()
        patched_keys: set[str] = set()
        selector = case["selector"]
        candidate_id = case["gsi_candidate_id"]
        if candidate_id:
            selected = config_index[candidate_id]
            values.update({key: float(selected[key]) for key in GSI_KEYS})
            patched_keys.update(GSI_KEYS)
        elif selector.startswith("lowest candidate"):
            selected = config_index[saturated]
            values.update({key: float(selected[key]) for key in GSI_KEYS})
            patched_keys.update(GSI_KEYS)
        overrides = parse_overrides(case)
        if any(bits(native_values[key]) == bits(value) for key, value in overrides.items()):
            raise ValueError(f"{case['case_id']} contains a no-op override")
        values.update(overrides)
        patched_keys.update(overrides)
        if patched_keys:
            replace_yaml(management, {key: values[key] for key in patched_keys})
        for source in FIXTURE.iterdir():
            copied = workdir / source.name
            if source.name != management.name and source.is_file() and sha(source) != sha(copied):
                raise ValueError(f"{case['case_id']} changed protected fixture member {source.name}")
        trace = remap_case_path(case["trace_path"], case["workdir"], execution_root.parent)
        trace.parent.mkdir(parents=True, exist_ok=True)
        environment = os.environ.copy()
        environment.update({
            "OPENWEPP_CANOPY_RESEARCH_TRACE_PATH": str(trace),
            "OPENWEPP_CANOPY_RESEARCH_SITE_ID": "hubbard_brook",
            "OPENWEPP_CANOPY_RESEARCH_ARM_ID": "deciduous",
        })
        output_dir = remap_case_path(case["output_dir"], case["workdir"], execution_root.parent)
        command = [str(RUNNER), "--run-dir", str(workdir), "--run-file", case["run_file"],
                   "--output-dir", str(output_dir), "--direct-production-executor"]
        stdout_path = remap_case_path(case["stdout_log"], case["workdir"], execution_root.parent)
        stderr_path = remap_case_path(case["stderr_log"], case["workdir"], execution_root.parent)
        with stdout_path.open("w", encoding="utf-8") as stdout, stderr_path.open("w", encoding="utf-8") as stderr:
            completed = subprocess.run(command, env=environment, stdout=stdout, stderr=stderr, check=False)
        if case["case_id"] == "invalid_threshold_order":
            if completed.returncode == 0 or trace.exists():
                raise ValueError("invalid case did not fail before trace creation")
            error_text = stderr_path.read_text(encoding="utf-8")
            if not has_typed_temperature_threshold_order_error(error_text):
                raise ValueError("invalid case lacked typed threshold error")
            compared = 0
            state = "PASS_TYPED_FAILURE_TRACE_ABSENT"
        else:
            if completed.returncode != 0 or not trace.is_file():
                raise ValueError(f"{case['case_id']} production run failed")
            expected = workdir / "expected.csv"
            expected_command = [
                str(EXPECTED), "--climate", str(workdir / "p10.cli"), "--output", str(expected),
                "--tmin-inactive", str(values[GSI_KEYS[0]]), "--tmin-unconstrained", str(values[GSI_KEYS[1]]),
                "--vpd-unconstrained", str(values[GSI_KEYS[2]]), "--vpd-inactive", str(values[GSI_KEYS[3]]),
                "--photo-inactive", str(values[GSI_KEYS[4]]), "--photo-unconstrained", str(values[GSI_KEYS[5]]),
                "--bf", str(values["summer_foliar_biomass_kg_m2"]), "--bs", str(values["structural_biomass_kg_m2"]),
                "--fe", str(values["evergreen_fraction"]), "--lai", str(values["xmxlai"]),
                "--cs", str(values["structural_canopy_cover_fraction"]), "--bb", str(values["bb"]),
            ]
            subprocess.run(expected_command, check=True)
            compared, compared_values = compare(trace, expected, climate_calendar(workdir / "p10.cli"))
            state = "PASS_BIT_EXACT"
        results.append({
            "case_id": case["case_id"], "selector": saturated if selector.startswith("lowest candidate") else selector,
            "production_returncode": completed.returncode, "compared_days": compared,
            "compared_values": compared_values if case["case_id"] != "invalid_threshold_order" else 0,
            "state": state, "trace": str(trace),
            "patched_keys": ";".join(sorted(patched_keys)) or "NONE",
            "management_sha256": sha(management),
            "runner_sha256": sha(RUNNER),
            "expected_probe_sha256": sha(EXPECTED),
            "native_proof_source_sha256": sha(Path(__file__)),
            "production_argv": " ".join(command),
            "expected_argv": " ".join(expected_command) if case["case_id"] != "invalid_threshold_order" else "NOT_RUN",
            "research_environment": (
                f"OPENWEPP_CANOPY_RESEARCH_TRACE_PATH={trace};"
                "OPENWEPP_CANOPY_RESEARCH_SITE_ID=hubbard_brook;"
                "OPENWEPP_CANOPY_RESEARCH_ARM_ID=deciduous"
            ),
            "toolchain": subprocess.run(
                ["rustc", "--version"], capture_output=True, text=True, check=True
            ).stdout.strip(),
        })
    with (ARTIFACTS / "native-consumer-proof.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(results[0]), lineterminator="\n")
        writer.writeheader(); writer.writerows(results)
    print(f"PASS native consumer cases={len(results)}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
