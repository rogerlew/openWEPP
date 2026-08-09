#!/usr/bin/env python3
"""Run the frozen Topanga mutation design with native openWEPP."""

from __future__ import annotations

import argparse
import concurrent.futures
import datetime
import hashlib
import json
import math
import shutil
import subprocess
import time
from dataclasses import dataclass
from pathlib import Path

import numpy as np
import pyarrow as pa
import pyarrow.parquet as pq


HOUR_SECONDS = 3_600.0
SHAPE_MIN = 1.0 / 24.0
NUMERIC_TOLERANCE = 1.0e-10
RECORD_SCHEMA = "openwepp-topanga-case-record-v3"
CANONICAL_PLAN_SHA256 = "32e6f5e99a77747fcdd93388302f2a5ffb496a87b764ac4505e09691955db756"
CANONICAL_ELIGIBLE_TRIALS = 1_088
CANONICAL_BASELINES = 280
DISCOVERED_SIDECARS = (
    "frost.txt",
    "snow.txt",
    "wepp_ui.txt",
    "pmetpara.txt",
    "irrigation_depletion.txt",
    "irrigation_fixeddate.ifd",
    "gwcoeff.txt",
    "phosphorus.txt",
    "tc.txt",
    "tcr.txt",
    "lcwb.txt",
    "chan.inp",
)


@dataclass(frozen=True)
class Case:
    case_id: str
    scenario: str
    hillslope_id: int
    source_dir: Path
    record_path: Path
    trial: dict | None


@dataclass(frozen=True)
class Provenance:
    plan_sha256: str
    binary_sha256: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--plan", type=Path, required=True)
    parser.add_argument("--source-root", type=Path, required=True)
    parser.add_argument("--evidence-root", type=Path, required=True)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--jobs", type=int, default=8)
    parser.add_argument("--limit", type=int)
    parser.add_argument("--resume", action="store_true")
    return parser.parse_args()


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def toml_string(value: str) -> str:
    return json.dumps(value)


def input_path(source_dir: Path, hillslope_id: int, suffix: str) -> Path:
    path = source_dir / f"p{hillslope_id}.{suffix}"
    if not path.is_file():
        raise FileNotFoundError(path)
    return path.resolve()


def write_runfile(case: Case, stage_dir: Path) -> tuple[Path, Path]:
    output_dir = stage_dir / "output"
    output_dir.mkdir(parents=True, exist_ok=True)
    runfile = stage_dir / "case.run.toml"
    hill = case.hillslope_id
    pmetpara = case.source_dir / "pmetpara.txt"
    lines = [
        'schema = "openwepp-hillslope-runfile-v1"',
        f"run_name = {toml_string(case.case_id)}",
        'unit_system = "metric"',
        "",
        "[inputs]",
        f"soil = {toml_string(str(input_path(case.source_dir, hill, 'sol')))}",
        f"management = {toml_string(str(input_path(case.source_dir, hill, 'man')))}",
        f"slope = {toml_string(str(input_path(case.source_dir, hill, 'slp')))}",
        f"climate = {toml_string(str(input_path(case.source_dir, hill, 'cli')))}",
        "wepp_ui = true",
    ]
    if pmetpara.is_file():
        lines.append(f"pmetpara = {toml_string(str(pmetpara.resolve()))}")
    lines.extend(
        [
            "",
            "[outputs]",
            f"pass = {toml_string(str((output_dir / f'H{hill}.hbp').resolve()))}",
            f"loss = {toml_string(str((output_dir / f'H{hill}.loss.json').resolve()))}",
            f"pass_parquet = {toml_string(str((output_dir / f'H{hill}.pass.parquet').resolve()))}",
            "",
        ]
    )
    runfile.write_text("\n".join(lines), encoding="utf-8")
    return runfile, output_dir


def load_record(path: Path) -> dict[str, np.ndarray]:
    with np.load(path) as record:
        return {name: record[name].copy() for name in record.files}


def case_input_hashes(case: Case) -> str:
    paths = [
        input_path(case.source_dir, case.hillslope_id, suffix)
        for suffix in ["sol", "man", "slp", "cli"]
    ]
    for name in DISCOVERED_SIDECARS:
        optional = case.source_dir / name
        if optional.is_file():
            paths.append(optional.resolve())
    return json.dumps(
        {str(path): sha256(path) for path in sorted(paths)}, sort_keys=True
    )


def expected_calendar(case: Case) -> tuple[np.ndarray, np.ndarray]:
    years: list[int] = []
    julians: list[int] = []
    simulation_year_by_calendar_year: dict[int, int] = {}
    climate_path = input_path(case.source_dir, case.hillslope_id, "cli")
    for line in climate_path.read_text(encoding="utf-8").splitlines():
        fields = line.split()
        if len(fields) < 3:
            continue
        try:
            day, month, year = (int(fields[index]) for index in range(3))
            calendar_day = datetime.date(year, month, day)
        except (ValueError, OverflowError):
            continue
        simulation_year = simulation_year_by_calendar_year.setdefault(
            year, len(simulation_year_by_calendar_year) + 1
        )
        years.append(simulation_year)
        julians.append(calendar_day.timetuple().tm_yday)
    if not years:
        raise ValueError(f"{case.case_id} climate contains no daily calendar rows")
    return np.asarray(years, dtype=np.int16), np.asarray(julians, dtype=np.int16)


def calendar_sha256(year: np.ndarray, julian: np.ndarray) -> str:
    digest = hashlib.sha256()
    digest.update(np.asarray(year, dtype="<i2").tobytes())
    digest.update(np.asarray(julian, dtype="<i2").tobytes())
    return digest.hexdigest()


def record_matches(path: Path, case: Case, provenance: Provenance) -> bool:
    try:
        record = load_record(path)
        expected_year, expected_julian = expected_calendar(case)
        expected = {
            "record_schema": RECORD_SCHEMA,
            "case_id": case.case_id,
            "plan_sha256": provenance.plan_sha256,
            "binary_sha256": provenance.binary_sha256,
            "input_hashes_json": case_input_hashes(case),
            "expected_row_count": len(expected_year),
            "calendar_sha256": calendar_sha256(expected_year, expected_julian),
        }
        for name, value in expected.items():
            if name not in record or str(record[name].item()) != str(value):
                return False
        required_arrays = ["year", "julian", "runvol_m3", "peakro_m3_s"]
        lengths = {len(record[name]) for name in required_arrays}
        if lengths != {len(expected_year)}:
            return False
        if not np.array_equal(record["year"], expected_year) or not np.array_equal(
            record["julian"], expected_julian
        ):
            return False
        runvol = record["runvol_m3"]
        peakro = record["peakro_m3_s"]
        return bool(
            np.isfinite(runvol).all()
            and np.isfinite(peakro).all()
            and (runvol >= 0.0).all()
            and (peakro >= 0.0).all()
        )
    except (OSError, ValueError, KeyError, TypeError):
        return False


def run_case(
    case: Case,
    binary: Path,
    stage_root: Path,
    resume: bool,
    provenance: Provenance,
) -> dict:
    if resume and case.record_path.is_file() and record_matches(case.record_path, case, provenance):
        return {"case_id": case.case_id, "status": "reused", "runtime_s": 0.0}

    stage_dir = stage_root / case.case_id
    if stage_dir.exists():
        shutil.rmtree(stage_dir)
    stage_dir.mkdir(parents=True)
    runfile, output_dir = write_runfile(case, stage_dir)
    parquet_path = output_dir / f"H{case.hillslope_id}.pass.parquet"
    command = [
        str(binary),
        "--run-dir",
        str(case.source_dir),
        "--run-file",
        str(runfile),
        "--output-dir",
        str(output_dir),
        "--policy",
        "compat",
        "--legacy-sidecar-discovery",
        "--direct-production-executor",
    ]
    started = time.monotonic()
    completed = subprocess.run(command, text=True, capture_output=True, check=False)
    runtime_s = time.monotonic() - started
    (stage_dir / "command.json").write_text(
        json.dumps({"command": command, "returncode": completed.returncode}, indent=2) + "\n",
        encoding="utf-8",
    )
    (stage_dir / "stdout.log").write_text(completed.stdout, encoding="utf-8")
    (stage_dir / "stderr.log").write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0 or not parquet_path.is_file():
        raise RuntimeError(
            f"{case.case_id} failed rc={completed.returncode}; retained at {stage_dir}"
        )

    table = pq.read_table(parquet_path, columns=["year", "julian", "runvol", "peakro"])
    year = table.column("year").to_numpy(zero_copy_only=False).astype(np.int16)
    julian = table.column("julian").to_numpy(zero_copy_only=False).astype(np.int16)
    runvol = table.column("runvol").to_numpy(zero_copy_only=False).astype(np.float64)
    peakro = table.column("peakro").to_numpy(zero_copy_only=False).astype(np.float64)
    expected_year, expected_julian = expected_calendar(case)
    if not np.array_equal(year, expected_year) or not np.array_equal(
        julian, expected_julian
    ):
        raise RuntimeError(f"{case.case_id} output calendar does not match climate input")
    if not (np.isfinite(runvol).all() and np.isfinite(peakro).all()):
        raise RuntimeError(f"{case.case_id} emitted non-finite runoff operands")
    if (runvol < 0.0).any() or (peakro < 0.0).any():
        raise RuntimeError(f"{case.case_id} emitted negative runoff operands")
    case.record_path.parent.mkdir(parents=True, exist_ok=True)
    temporary_record_path = case.record_path.with_suffix(".tmp.npz")
    np.savez_compressed(
        temporary_record_path,
        record_schema=np.asarray(RECORD_SCHEMA),
        case_id=np.asarray(case.case_id),
        plan_sha256=np.asarray(provenance.plan_sha256),
        binary_sha256=np.asarray(provenance.binary_sha256),
        input_hashes_json=np.asarray(case_input_hashes(case)),
        expected_row_count=np.asarray(len(expected_year)),
        calendar_sha256=np.asarray(calendar_sha256(expected_year, expected_julian)),
        year=year,
        julian=julian,
        runvol_m3=runvol,
        peakro_m3_s=peakro,
    )
    temporary_record_path.replace(case.record_path)
    shutil.rmtree(stage_dir)
    return {
        "case_id": case.case_id,
        "status": "ran",
        "runtime_s": runtime_s,
        "rows": int(len(year)),
        "event_rows": int(np.count_nonzero((runvol > 0.0) | (peakro > 0.0))),
    }


def run_batch(
    cases: list[Case],
    args: argparse.Namespace,
    stage_root: Path,
    provenance: Provenance,
) -> list[dict]:
    results: list[dict] = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as executor:
        futures = {
            executor.submit(
                run_case, case, args.binary, stage_root, args.resume, provenance
            ): case
            for case in cases
        }
        for future in concurrent.futures.as_completed(futures):
            case = futures[future]
            try:
                result = future.result()
            except Exception as error:
                for pending in futures:
                    pending.cancel()
                raise RuntimeError(f"case failure: {case.case_id}: {error}") from error
            results.append(result)
            print(json.dumps(result, sort_keys=True), flush=True)
    return results


def shape_fraction(runvol_m3: float, peakro_m3_s: float) -> float | None:
    if runvol_m3 == 0.0:
        return None
    return peakro_m3_s * HOUR_SECONDS / runvol_m3


def ratio(numerator: float, denominator: float) -> float | None:
    if numerator <= 0.0 or denominator <= 0.0:
        return None
    return numerator / denominator


def symmetric_percent_change(mutated: float, baseline: float) -> float:
    denominator = abs(mutated) + abs(baseline)
    return 0.0 if denominator == 0.0 else 200.0 * (mutated - baseline) / denominator


def paired_event_rows(trial: dict, baseline_path: Path, mutation_path: Path) -> list[dict]:
    baseline = load_record(baseline_path)
    mutation = load_record(mutation_path)
    for name in ["year", "julian"]:
        if not np.array_equal(baseline[name], mutation[name]):
            raise RuntimeError(f"{trial['trial_id']} changed the calendar key surface")
    rows: list[dict] = []
    for index in range(len(baseline["year"])):
        base_volume = float(baseline["runvol_m3"][index])
        mutated_volume = float(mutation["runvol_m3"][index])
        base_peak = float(baseline["peakro_m3_s"][index])
        mutated_peak = float(mutation["peakro_m3_s"][index])
        if base_volume == mutated_volume == base_peak == mutated_peak == 0.0:
            continue
        base_shape = shape_fraction(base_volume, base_peak)
        mutated_shape = shape_fraction(mutated_volume, mutated_peak)
        volume_ratio = ratio(mutated_volume, base_volume)
        peak_ratio = ratio(mutated_peak, base_peak)
        shape_ratio = (
            ratio(mutated_shape, base_shape)
            if mutated_shape is not None and base_shape is not None
            else None
        )
        decomposition_expected = (
            volume_ratio * shape_ratio
            if volume_ratio is not None and shape_ratio is not None
            else None
        )
        decomposition_residual = (
            (peak_ratio - decomposition_expected)
            / max(abs(peak_ratio), abs(decomposition_expected), 1.0)
            if peak_ratio is not None and decomposition_expected is not None
            else None
        )
        rows.append(
            {
                "trial_id": trial["trial_id"],
                "scenario": trial["scenario"],
                "hillslope_id": trial["hillslope_id"],
                "family": trial["family"],
                "direction": trial["direction"],
                "source_value_json": json.dumps(trial["source_value"], sort_keys=True),
                "expected_value_json": json.dumps(trial["expected_value"], sort_keys=True),
                "year": int(baseline["year"][index]),
                "julian": int(baseline["julian"][index]),
                "baseline_runvol_m3": base_volume,
                "mutated_runvol_m3": mutated_volume,
                "baseline_peakro_m3_s": base_peak,
                "mutated_peakro_m3_s": mutated_peak,
                "baseline_max_hour_fraction": base_shape,
                "mutated_max_hour_fraction": mutated_shape,
                "runvol_ratio": volume_ratio,
                "peakro_ratio": peak_ratio,
                "max_hour_fraction_ratio": shape_ratio,
                "runvol_symmetric_percent_change": symmetric_percent_change(
                    mutated_volume, base_volume
                ),
                "peakro_symmetric_percent_change": symmetric_percent_change(
                    mutated_peak, base_peak
                ),
                "max_hour_fraction_symmetric_percent_change": (
                    symmetric_percent_change(mutated_shape, base_shape)
                    if mutated_shape is not None and base_shape is not None
                    else None
                ),
                "peakro_log_ratio": math.log(peak_ratio) if peak_ratio is not None else None,
                "runvol_log_ratio": math.log(volume_ratio) if volume_ratio is not None else None,
                "max_hour_fraction_log_ratio": (
                    math.log(shape_ratio) if shape_ratio is not None else None
                ),
                "ratio_decomposition_relative_residual": decomposition_residual,
            }
        )
    return rows


def validate_event_rows(rows: list[dict]) -> dict:
    if not rows:
        raise RuntimeError("mutation census produced no paired runoff events")
    zero_topology_mismatches = sum(
        (row["baseline_runvol_m3"] == 0.0) != (row["baseline_peakro_m3_s"] == 0.0)
        or (row["mutated_runvol_m3"] == 0.0) != (row["mutated_peakro_m3_s"] == 0.0)
        for row in rows
    )
    shape_values = [
        row[name]
        for row in rows
        for name in ["baseline_max_hour_fraction", "mutated_max_hour_fraction"]
        if row[name] is not None
    ]
    invalid_shapes = [
        value
        for value in shape_values
        if value < SHAPE_MIN - NUMERIC_TOLERANCE or value > 1.0 + NUMERIC_TOLERANCE
    ]
    residuals = [
        abs(row["ratio_decomposition_relative_residual"])
        for row in rows
        if row["ratio_decomposition_relative_residual"] is not None
    ]
    if invalid_shapes:
        raise RuntimeError(f"{len(invalid_shapes)} maximum-hour fractions violate [1/24, 1]")
    if zero_topology_mismatches:
        raise RuntimeError(
            f"{zero_topology_mismatches} event rows disagree on zero runoff/zero peak topology"
        )
    max_residual = max(residuals, default=0.0)
    if max_residual > NUMERIC_TOLERANCE:
        raise RuntimeError(f"peak volume/shape decomposition residual {max_residual}")
    peak_ratios = sorted(
        row["peakro_ratio"] for row in rows if row["peakro_ratio"] is not None
    )
    shape_ratios = sorted(
        row["max_hour_fraction_ratio"]
        for row in rows
        if row["max_hour_fraction_ratio"] is not None
    )
    finite_metrics = [
        row[name]
        for row in rows
        for name in [
            "runvol_ratio",
            "peakro_ratio",
            "max_hour_fraction_ratio",
            "ratio_decomposition_relative_residual",
        ]
        if row[name] is not None
    ]
    if not all(math.isfinite(value) for value in finite_metrics):
        raise RuntimeError("paired response metrics contain non-finite values")
    volume_stable_peak_doublings = sum(
        abs(row["runvol_log_ratio"]) <= math.log(1.05)
        and abs(row["peakro_log_ratio"]) >= math.log(2.0)
        for row in rows
        if row["runvol_log_ratio"] is not None and row["peakro_log_ratio"] is not None
    )
    return {
        "event_pair_rows": len(rows),
        "finite_positive_peak_pairs": len(peak_ratios),
        "invalid_max_hour_fraction_count": len(invalid_shapes),
        "zero_runoff_peak_topology_mismatch_count": zero_topology_mismatches,
        "max_abs_ratio_decomposition_relative_residual": max_residual,
        "peak_ratio_p99": percentile(peak_ratios, 99.0),
        "peak_ratio_max": max(peak_ratios, default=None),
        "max_hour_fraction_ratio_p99": percentile(shape_ratios, 99.0),
        "max_hour_fraction_ratio_max": max(shape_ratios, default=None),
        "volume_within_5pct_peak_at_least_2x_count": volume_stable_peak_doublings,
    }


def percentile(values: list[float], quantile: float) -> float | None:
    if not values:
        return None
    return float(np.percentile(np.asarray(values), quantile))


def main() -> int:
    args = parse_args()
    if args.jobs <= 0:
        raise ValueError("--jobs must be positive")
    args.plan = args.plan.resolve()
    args.source_root = args.source_root.resolve()
    args.evidence_root = args.evidence_root.resolve()
    args.binary = args.binary.resolve()
    if not args.binary.is_file():
        raise FileNotFoundError(args.binary)
    plan = json.loads(args.plan.read_text(encoding="utf-8"))
    plan_sha256 = sha256(args.plan)
    binary_sha256 = sha256(args.binary)
    provenance = Provenance(plan_sha256=plan_sha256, binary_sha256=binary_sha256)
    eligible = [trial for trial in plan["trials"] if trial["eligibility"] == "eligible"]
    eligible.sort(key=lambda trial: trial["trial_id"])
    if len({trial["trial_id"] for trial in eligible}) != len(eligible):
        raise ValueError("eligible trial IDs must be unique")
    selected = eligible[: args.limit] if args.limit is not None else eligible

    records = args.evidence_root / "records"
    stage_root = args.evidence_root / "stage"
    stage_root.mkdir(parents=True, exist_ok=True)
    baseline_keys = sorted({(trial["scenario"], trial["hillslope_id"]) for trial in selected})
    baselines = [
        Case(
            case_id=f"baseline-{scenario}-h{hill}",
            scenario=scenario,
            hillslope_id=hill,
            source_dir=args.source_root / "input-snapshot" / scenario / "runs",
            record_path=records / "baselines" / f"{scenario}-h{hill}.npz",
            trial=None,
        )
        for scenario, hill in baseline_keys
    ]
    trials = [
        Case(
            case_id=trial["trial_id"],
            scenario=trial["scenario"],
            hillslope_id=trial["hillslope_id"],
            source_dir=args.source_root / Path(trial["evidence_locator"]).relative_to(
                args.source_root.name
            ) / "runs",
            record_path=records / "trials" / f"{trial['trial_id']}.npz",
            trial=trial,
        )
        for trial in selected
    ]

    started = time.monotonic()
    baseline_results = run_batch(baselines, args, stage_root, provenance)
    trial_results = run_batch(trials, args, stage_root, provenance)
    event_rows: list[dict] = []
    for case in trials:
        trial = case.trial
        if trial is None:
            raise RuntimeError(f"trial metadata missing for {case.case_id}")
        baseline_path = records / "baselines" / f"{case.scenario}-h{case.hillslope_id}.npz"
        event_rows.extend(paired_event_rows(trial, baseline_path, case.record_path))
    validation = validate_event_rows(event_rows)
    event_path = args.evidence_root / "topanga-openwepp-event-pairs.parquet"
    pq.write_table(pa.Table.from_pylist(event_rows), event_path, compression="zstd")
    summary = {
        "schema": "openwepp-topanga-hourly-peak-mutation-census-v1",
        "plan_sha256": plan_sha256,
        "binary_sha256": binary_sha256,
        "eligible_trials_in_plan": len(eligible),
        "selected_trials": len(selected),
        "complete_frozen_cohort": (
            args.limit is None
            and plan_sha256 == CANONICAL_PLAN_SHA256
            and len(eligible) == CANONICAL_ELIGIBLE_TRIALS
            and len(selected) == CANONICAL_ELIGIBLE_TRIALS
            and len(baselines) == CANONICAL_BASELINES
        ),
        "unique_baselines": len(baselines),
        "jobs": args.jobs,
        "elapsed_s": time.monotonic() - started,
        "baseline_runs": baseline_results,
        "trial_runs": trial_results,
        "validation": validation,
        "event_pairs_parquet": str(event_path),
    }
    summary_path = args.evidence_root / "summary.json"
    summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"summary": str(summary_path), **validation}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
