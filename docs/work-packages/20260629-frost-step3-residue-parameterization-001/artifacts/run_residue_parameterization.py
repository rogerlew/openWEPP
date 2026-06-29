#!/usr/bin/env python3
"""Run FROST STEP 3 residue-parameterization diagnostics."""

from __future__ import annotations

import argparse
import datetime as dt
import importlib.util
import json
import math
import os
import shutil
import subprocess
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[4]
PACKAGE = REPO_ROOT / "docs/work-packages/20260629-frost-step3-residue-parameterization-001"
ARTIFACTS = PACKAGE / "artifacts"
TARGET = REPO_ROOT / "target/frost_step3_residue_parameterization"
OBS_DIR = REPO_ROOT / "tests/fixtures/snowfreeze_observed/observations"
STEP1 = REPO_ROOT / "docs/work-packages/20260629-frost-step1-current-snow-control-rerun-001"
STEP2_ANALYZER = (
    REPO_ROOT
    / "docs/work-packages/20260629-frost-step2-sleepers-attribution-001/artifacts/attribute_sleepers.py"
)
HARNESS_PATH = REPO_ROOT / "tools/snowfreeze_observed/observed_harness.py"
SEASONAL_FIXTURE = REPO_ROOT / "tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh"
SEASONAL_MANAGEMENT = SEASONAL_FIXTURE / "p10.man"

SITES = {
    "site1_sleepers_south_field_vt": {
        "fixture": REPO_ROOT
        / "tests/fixtures/snowfreeze_observed/site1_sleepers_south_field_vt",
        "run_stem": "p1",
    },
    "site2_sleepers_w9_hardwood_vt": {
        "fixture": REPO_ROOT
        / "tests/fixtures/snowfreeze_observed/site2_sleepers_w9_hardwood_vt",
        "run_stem": "p3",
    },
}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--binary",
        type=Path,
        default=REPO_ROOT / "target/release/openwepp-cli-hill",
        help="openwepp-cli-hill binary to run",
    )
    parser.add_argument("--keep-target", action="store_true")
    args = parser.parse_args()

    if not args.binary.is_file():
        raise SystemExit(f"missing binary: {args.binary}")

    harness = load_module("observed_harness", HARNESS_PATH)
    step2 = load_module("step2_attribute_sleepers", STEP2_ANALYZER)

    if not args.keep_target and TARGET.exists():
        shutil.rmtree(TARGET)
    (TARGET / "runs").mkdir(parents=True, exist_ok=True)
    (TARGET / "fixtures").mkdir(parents=True, exist_ok=True)
    (ARTIFACTS / "site_reports").mkdir(parents=True, exist_ok=True)

    entry = run_entry_gate(args.binary.resolve(), harness)
    payload: dict[str, Any] = {
        "schema": "frost-step3-residue-parameterization-v1",
        "evidence_mode": "Ran",
        "scope": {
            "included_sites": sorted(SITES),
            "excluded_sites": {
                "site4_ggd498_morris_mn": "Step 1 BLOCKED",
                "site3_scan_mandan_nd": "Step 1 INCONCLUSIVE-NO-PAIRED-SNOW",
                "site5_reynolds_creek_us_rls_id": "Step 1 INCONCLUSIVE-NO-PAIRED-SNOW",
            },
        },
        "entry_gate": entry,
        "seasonal_management_source": str(SEASONAL_MANAGEMENT.relative_to(REPO_ROOT)),
        "step2_analyzer": str(STEP2_ANALYZER.relative_to(REPO_ROOT)),
        "step4_note": (
            "The Step 1 >0.25 systematic-timing-fraction cutoff is diagnostic-script-local; "
            "only TOLERANCE_DAYS=14 is inherited by this package."
        ),
    }

    if not entry["passes"]:
        payload["decision_branch"] = {
            "branch": "C",
            "label": "Dec_* does not drive seasonal residue_depth_m",
            "justification": entry["decision_reason"],
        }
        payload["sites"] = []
    else:
        payload["sites"] = run_core_test(args.binary.resolve(), harness, step2)
        payload["decision_branch"] = decide_branch(payload["sites"])

    payload["gap_snowfreeze_002"] = gap_disposition(payload)
    write_json(ARTIFACTS / "residue_parameterization_diagnostic.json", payload)
    (ARTIFACTS / "residue_parameterization_diagnostic.md").write_text(
        render_markdown(payload), encoding="utf-8"
    )
    return 0


def load_module(name: str, path: Path) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def run_entry_gate(binary: Path, harness: Any) -> dict[str, Any]:
    output_dir = TARGET / "runs/entry_gate_hubbardbrook_deciduous"
    report = run_fixture(
        binary=binary,
        harness=harness,
        site_id="entry_gate_hubbardbrook_deciduous_nh",
        fixture_dir=SEASONAL_FIXTURE,
        run_stem="p10",
        output_dir=output_dir,
        observation_file=None,
    )
    trace_summary = summarize_trace(output_dir / "frost_trace.jsonl")
    trace_summary_path = ARTIFACTS / "entry_gate_residue_trace_summary.json"
    write_json(trace_summary_path, trace_summary)
    trajectory_path = ARTIFACTS / "entry_gate_residue_monthly_trajectory.csv"
    write_monthly_csv(trajectory_path, trace_summary["monthly"])

    seasonal = trace_summary["max_residue_depth_m"] - trace_summary["min_residue_depth_m"] > 1.0e-6
    physically_reasonable = bool(
        seasonal
        and trace_summary["month_of_max_mean_residue_depth_m"] in {9, 10, 11, 12}
        and trace_summary["spring_mean_residue_depth_m"] < trace_summary["autumn_mean_residue_depth_m"]
    )
    return {
        "fixture": str(SEASONAL_FIXTURE.relative_to(REPO_ROOT)),
        "run_stem": "p10",
        "run_output": str(output_dir.relative_to(REPO_ROOT)),
        "report_path": report.get("report_path"),
        "trace_summary_path": str(trace_summary_path.relative_to(REPO_ROOT)),
        "monthly_trajectory_path": str(trajectory_path.relative_to(REPO_ROOT)),
        "trace_summary": trace_summary,
        "seasonal_residue_depth_m": seasonal,
        "physically_reasonable": physically_reasonable,
        "passes": physically_reasonable,
        "decision_reason": (
            "Dec_* residue_depth_m reaches the frost solver as a seasonal trajectory "
            "with autumn peak and lower spring mean."
            if physically_reasonable
            else "Dec_* residue_depth_m is flat or does not show the required autumn-to-spring decline."
        ),
    }


def run_core_test(binary: Path, harness: Any, step2: Any) -> list[dict[str, Any]]:
    routing = json.loads((STEP1 / "artifacts/current_snow_control_routing.json").read_text())
    routing_by_site = {site["site_id"]: site for site in routing["sites"]}
    sites = []
    for site_id in sorted(SITES):
        route = routing_by_site[site_id]
        baseline_report_path = (
            STEP1 / "artifacts/site_reports" / f"{site_id}.comparison_report.json"
        )
        baseline_report = json.loads(baseline_report_path.read_text(encoding="utf-8"))
        baseline_analysis = step2.analyze_site(baseline_report_path, baseline_report, route)

        variant_fixture = create_seasonal_fixture(site_id)
        seasonal_output = TARGET / "runs" / site_id / "seasonal_dec"
        seasonal_report = run_fixture(
            binary=binary,
            harness=harness,
            site_id=site_id,
            fixture_dir=variant_fixture,
            run_stem=SITES[site_id]["run_stem"],
            output_dir=seasonal_output,
            observation_file=observation_file_for_site(site_id),
        )
        report_copy_path = ARTIFACTS / "site_reports" / f"{site_id}.seasonal_dec.comparison_report.json"
        report_copy_path.write_text(
            json.dumps(seasonal_report["report"], indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )
        seasonal_analysis = step2.analyze_site(
            report_copy_path, seasonal_report["report"], route
        )
        trace_summary = summarize_trace(seasonal_output / "frost_trace.jsonl")
        trace_path = ARTIFACTS / "site_reports" / f"{site_id}.seasonal_dec.residue_trace_summary.json"
        write_json(trace_path, trace_summary)

        comparison = compare_site_analyses(baseline_analysis, seasonal_analysis)
        sites.append(
            {
                "site_id": site_id,
                "baseline": compact_analysis(baseline_analysis),
                "seasonal_dec": compact_analysis(seasonal_analysis),
                "comparison": comparison,
                "seasonal_fixture": str(variant_fixture.relative_to(REPO_ROOT)),
                "seasonal_report_path": str(report_copy_path.relative_to(REPO_ROOT)),
                "seasonal_trace_summary_path": str(trace_path.relative_to(REPO_ROOT)),
                "seasonal_trace_summary": trace_summary,
            }
        )
    return sites


def run_fixture(
    *,
    binary: Path,
    harness: Any,
    site_id: str,
    fixture_dir: Path,
    run_stem: str,
    output_dir: Path,
    observation_file: str | None,
) -> dict[str, Any]:
    output_dir.mkdir(parents=True, exist_ok=True)
    runfile = output_dir / f"{site_id}.run"
    harness.write_runfile(runfile, fixture_dir, run_stem, output_dir, site_id)
    command = harness.cli_command(
        binary,
        fixture_dir,
        runfile,
        output_dir,
        "direct-production-executor",
    )
    env = os.environ.copy()
    env["OPENWEPP_R7G_FROST_TRACE_PATH"] = str(output_dir / "frost_trace.jsonl")
    completed = subprocess.run(
        command,
        cwd=REPO_ROOT,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        env=env,
    )
    (output_dir / "openwepp-cli-hill.stdout").write_text(completed.stdout, encoding="utf-8")
    (output_dir / "openwepp-cli-hill.stderr").write_text(completed.stderr, encoding="utf-8")
    if completed.returncode != 0:
        raise SystemExit(f"{site_id} run failed with exit code {completed.returncode}")

    wat_path = output_dir / f"{site_id}.wat.parquet"
    if not wat_path.is_file():
        raise FileNotFoundError(wat_path)
    report_path = output_dir / "comparison_report.json"
    if observation_file is None:
        report = {
            "schema": "frost-step3-entry-gate-run-v1",
            "site_id": site_id,
            "fixture_dir": str(fixture_dir),
            "runfile": str(runfile),
            "wat_output": str(wat_path),
            "runtime": "direct-production-executor",
            "verdict": "TRACE-ONLY",
        }
    else:
        observations = harness.load_observations(OBS_DIR / observation_file)
        modeled = harness.load_modeled_wat(wat_path)
        metrics = harness.compute_metrics(observations, modeled)
        report = {
            "schema": "snowfreeze-observed-comparison-v1",
            "site_id": site_id,
            "fixture_dir": str(fixture_dir),
            "runfile": str(runfile),
            "wat_output": str(wat_path),
            "runtime": "direct-production-executor",
            "verdict": "UNRESOLVED" if metrics["matched_count"] else "HARNESS-SURFACE-MISMATCH",
            "measurement_contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-047",
            "snow_control_status": harness.snow_control_status(metrics),
            "modeled_snow_depth_source": "hillslope_wat.Snow-Depth:mm from snow.runtime_depth_m",
            "metrics": metrics,
        }
    harness.write_comparison_reports(output_dir, report)
    report["report_path"] = str(report_path.relative_to(REPO_ROOT))
    return {"report": report, "report_path": str(report_path.relative_to(REPO_ROOT))}


def create_seasonal_fixture(site_id: str) -> Path:
    source = SITES[site_id]["fixture"]
    run_stem = SITES[site_id]["run_stem"]
    destination = TARGET / "fixtures" / f"{site_id}_seasonal_dec"
    if destination.exists():
        shutil.rmtree(destination)
    shutil.copytree(source, destination)
    (destination / f"{run_stem}.man").write_text(
        seasonal_management_text(), encoding="utf-8"
    )
    patch_pmetpara(destination / "pmetpara.txt")
    return destination


def seasonal_management_text() -> str:
    text = SEASONAL_MANAGEMENT.read_text(encoding="utf-8")
    return text.replace("45 # sim_years", "45 # sim_years")


def patch_pmetpara(path: Path) -> None:
    if not path.is_file():
        return
    lines = path.read_text(encoding="utf-8").splitlines()
    patched: list[str] = []
    for line in lines:
        if line.startswith("Tah_4899,"):
            fields = line.split(",")
            fields[0] = "Dec_4899"
            if len(fields) >= 5:
                soil = fields[4].split("-", 1)[0]
                fields[4] = f"{soil}-deciduous_forest"
            patched.append(",".join(fields))
        else:
            patched.append(line)
    path.write_text("\n".join(patched) + "\n", encoding="utf-8")


def observation_file_for_site(site_id: str) -> str:
    manifest = json.loads((OBS_DIR / "manifest.json").read_text(encoding="utf-8"))
    for site in manifest["sites"]:
        if site["site_id"] == site_id:
            return site["observation_file"]
    raise KeyError(site_id)


def summarize_trace(trace_path: Path) -> dict[str, Any]:
    rows = []
    with trace_path.open(encoding="utf-8") as handle:
        for line in handle:
            if not line.strip():
                continue
            record = json.loads(line)
            if record.get("schema") != "openwepp-r7g-frost-trace-v1":
                continue
            residue = record.get("residue_depth_m")
            if residue is None or not math.isfinite(float(residue)):
                continue
            day = int(round(float(record["day"])))
            raw_year = record.get("year")
            year = int(round(float(raw_year))) if raw_year is not None else None
            date = synthetic_date_from_day(day)
            rows.append(
                {
                    "date": date,
                    "year": year,
                    "day": day,
                    "month": date.month,
                    "residue_depth_m": float(residue),
                    "snow_depth_m": float(record.get("snow_depth_m") or 0.0),
                }
            )
    if not rows:
        raise ValueError(f"{trace_path} has no residue trace rows")
    values = [row["residue_depth_m"] for row in rows]
    monthly = []
    by_month: dict[int, list[float]] = defaultdict(list)
    for row in rows:
        by_month[row["month"]].append(row["residue_depth_m"])
    for month in range(1, 13):
        month_values = by_month.get(month, [])
        monthly.append(
            {
                "month": month,
                "mean_residue_depth_m": mean(month_values),
                "min_residue_depth_m": min(month_values) if month_values else None,
                "max_residue_depth_m": max(month_values) if month_values else None,
                "row_count": len(month_values),
            }
        )
    monthly_means = [
        row for row in monthly if row["mean_residue_depth_m"] is not None
    ]
    max_month = max(monthly_means, key=lambda row: row["mean_residue_depth_m"])
    autumn = [
        row["residue_depth_m"] for row in rows if row["month"] in {9, 10, 11}
    ]
    spring = [
        row["residue_depth_m"] for row in rows if row["month"] in {3, 4, 5}
    ]
    return {
        "trace_path": str(trace_path.relative_to(REPO_ROOT)),
        "row_count": len(rows),
        "first_date": rows[0]["date"].isoformat(),
        "last_date": rows[-1]["date"].isoformat(),
        "min_residue_depth_m": min(values),
        "max_residue_depth_m": max(values),
        "mean_residue_depth_m": mean(values),
        "median_residue_depth_m": median(values),
        "unique_rounded_residue_depth_count_1e6": len({round(value, 6) for value in values}),
        "month_of_max_mean_residue_depth_m": max_month["month"],
        "max_month_mean_residue_depth_m": max_month["mean_residue_depth_m"],
        "autumn_mean_residue_depth_m": mean(autumn),
        "spring_mean_residue_depth_m": mean(spring),
        "monthly": monthly,
        "first_rows": [
            {
                "date": row["date"].isoformat(),
                "residue_depth_m": row["residue_depth_m"],
                "snow_depth_m": row["snow_depth_m"],
            }
            for row in rows[:10]
        ],
    }


def synthetic_date_from_day(day: int) -> dt.date:
    if day >= 366:
        return dt.date(2001, 12, 31)
    return dt.date(2001, 1, 1) + dt.timedelta(days=day - 1)


def compact_analysis(analysis: dict[str, Any]) -> dict[str, Any]:
    return {
        "report_path": analysis["report_path"],
        "candidate_defect_count": analysis["candidate_defect_count"],
        "timing_summary": analysis["timing_summary"],
        "candidate_defects": analysis["candidate_defects"],
        "magnitude": analysis["magnitude"],
        "site_disposition": analysis["site_disposition"],
        "timing_rows": analysis["timing_rows"],
    }


def compare_site_analyses(
    baseline: dict[str, Any], seasonal: dict[str, Any]
) -> dict[str, Any]:
    baseline_by_key = timing_by_key(baseline)
    seasonal_by_key = timing_by_key(seasonal)
    changed = []
    for key, base in sorted(baseline_by_key.items()):
        seas = seasonal_by_key.get(key)
        if seas is None:
            continue
        if base["residual"] != seas["residual"] or base["attribution"] != seas["attribution"]:
            changed.append(
                {
                    "water_year": key[0],
                    "signature": key[1],
                    "baseline_residual": base["residual"],
                    "seasonal_residual": seas["residual"],
                    "baseline_attribution": base["attribution"],
                    "seasonal_attribution": seas["attribution"],
                    "baseline_verdict": base["verdict"],
                    "seasonal_verdict": seas["verdict"],
                    "absolute_residual_change_days": abs_days(seas["residual"])
                    - abs_days(base["residual"]),
                }
            )
    baseline_candidates = baseline["candidate_defect_count"]
    seasonal_candidates = seasonal["candidate_defect_count"]
    return {
        "baseline_candidate_defect_count": baseline_candidates,
        "seasonal_candidate_defect_count": seasonal_candidates,
        "candidate_defect_delta": seasonal_candidates - baseline_candidates,
        "candidate_defects_shrank": seasonal_candidates < baseline_candidates,
        "changed_timing_cells": changed,
        "baseline_candidate_cells_after_seasonal": [
            {
                "water_year": item["water_year"],
                "signature": item["signature"],
                "baseline_residual": item["residual"],
                "seasonal_residual": seasonal_by_key.get(
                    (item["water_year"], item["signature"]), {}
                ).get("residual"),
                "seasonal_attribution": seasonal_by_key.get(
                    (item["water_year"], item["signature"]), {}
                ).get("attribution"),
                "seasonal_verdict": seasonal_by_key.get(
                    (item["water_year"], item["signature"]), {}
                ).get("verdict"),
            }
            for item in baseline["candidate_defects"]
        ],
    }


def timing_by_key(analysis: dict[str, Any]) -> dict[tuple[int, str], dict[str, Any]]:
    output = {}
    for row in analysis["timing_rows"]:
        for item in row["signatures"]:
            output[(int(row["water_year"]), item["signature"])] = item
    return output


def decide_branch(sites: list[dict[str, Any]]) -> dict[str, Any]:
    total_baseline = sum(
        site["comparison"]["baseline_candidate_defect_count"] for site in sites
    )
    total_seasonal = sum(
        site["comparison"]["seasonal_candidate_defect_count"] for site in sites
    )
    if total_seasonal < total_baseline:
        branch = "A"
        label = "Parameterization cause confirmed"
        justification = (
            f"Seasonal Dec_* residue reduced candidate-defect timing cells "
            f"from {total_baseline} to {total_seasonal}."
        )
    else:
        branch = "B"
        label = "Residue is not the primary cause"
        justification = (
            f"Seasonal Dec_* residue did not shrink candidate-defect timing cells "
            f"({total_baseline} baseline, {total_seasonal} seasonal)."
        )
    return {
        "branch": branch,
        "label": label,
        "total_baseline_candidate_defect_count": total_baseline,
        "total_seasonal_candidate_defect_count": total_seasonal,
        "justification": justification,
    }


def gap_disposition(payload: dict[str, Any]) -> str:
    branch = payload["decision_branch"]["branch"]
    if branch == "A":
        return (
            "GAP-SNOWFREEZE-002 remains open but the Sleepers timing candidate "
            "defects are attributed to fixture residue parameterization; follow-on "
            "fix is production-fixture repoint or first-class forest litter cover."
        )
    if branch == "B":
        return (
            "GAP-SNOWFREEZE-002 remains open as a genuine frost-model candidate "
            "after the residue parameterization hypothesis did not shrink timing defects."
        )
    return (
        "GAP-SNOWFREEZE-002 remains open; cropland Dec_* management did not prove "
        "a physically seasonal residue_depth_m path to the frost solver, so first-class "
        "forest litter cover should be promoted before fixture repointing."
    )


def render_markdown(payload: dict[str, Any]) -> str:
    lines = [
        "# FROST STEP 3 Residue Parameterization Diagnostic",
        "",
        "Evidence mode: Ran.",
        "",
        f"- Decision branch: `{payload['decision_branch']['branch']}` "
        f"{payload['decision_branch']['label']}",
        f"- Justification: {payload['decision_branch']['justification']}",
        f"- GAP-SNOWFREEZE-002: {payload['gap_snowfreeze_002']}",
        f"- Step 2 analyzer: `{payload['step2_analyzer']}`",
        "",
        "## Entry Gate",
        "",
    ]
    entry = payload["entry_gate"]
    summary = entry["trace_summary"]
    lines.extend(
        [
            f"- Fixture: `{entry['fixture']}`",
            f"- Trace summary: `{entry['trace_summary_path']}`",
            f"- Monthly trajectory: `{entry['monthly_trajectory_path']}`",
            f"- Seasonal: `{entry['seasonal_residue_depth_m']}`",
            f"- Physically reasonable: `{entry['physically_reasonable']}`",
            f"- Residue depth min/max m: `{summary['min_residue_depth_m']:.6g}` / `{summary['max_residue_depth_m']:.6g}`",
            f"- Autumn mean m: `{summary['autumn_mean_residue_depth_m']:.6g}`",
            f"- Spring mean m: `{summary['spring_mean_residue_depth_m']:.6g}`",
            f"- Max monthly mean month: `{summary['month_of_max_mean_residue_depth_m']}`",
            "",
        ]
    )
    if not payload["sites"]:
        lines.extend(
            [
                "## Core Test",
                "",
                "Not run because the entry gate did not pass.",
                "",
                "## Step 4 Note",
                "",
                payload["step4_note"],
                "",
            ]
        )
        return "\n".join(lines)

    lines.extend(
        [
            "## A-vs-B Timing Comparison",
            "",
            "| Site | Baseline candidate defects | Seasonal candidate defects | Delta |",
            "| --- | ---: | ---: | ---: |",
        ]
    )
    for site in payload["sites"]:
        cmp = site["comparison"]
        lines.append(
            f"| `{site['site_id']}` | `{cmp['baseline_candidate_defect_count']}` | "
            f"`{cmp['seasonal_candidate_defect_count']}` | `{cmp['candidate_defect_delta']}` |"
        )
    lines.append("")
    for site in payload["sites"]:
        lines.extend(render_site(site))
    lines.extend(
        [
            "## Step 4 Note",
            "",
            payload["step4_note"],
            "",
        ]
    )
    return "\n".join(lines)


def render_site(site: dict[str, Any]) -> list[str]:
    lines = [
        f"## {site['site_id']}",
        "",
        f"- Seasonal fixture: `{site['seasonal_fixture']}`",
        f"- Seasonal report: `{site['seasonal_report_path']}`",
        f"- Seasonal trace: `{site['seasonal_trace_summary_path']}`",
        f"- Candidate defects baseline -> seasonal: "
        f"`{site['comparison']['baseline_candidate_defect_count']}` -> "
        f"`{site['comparison']['seasonal_candidate_defect_count']}`",
        "",
        "### Baseline Candidate Cells After Seasonal Run",
        "",
        "| WY | Signature | Baseline residual | Seasonal residual | Seasonal attribution |",
        "| ---: | --- | ---: | ---: | --- |",
    ]
    for row in site["comparison"]["baseline_candidate_cells_after_seasonal"]:
        lines.append(
            f"| `{row['water_year']}` | `{row['signature']}` | `{row['baseline_residual']}` | "
            f"`{row['seasonal_residual']}` | `{row['seasonal_attribution']}` |"
        )
    lines.append("")
    return lines


def write_monthly_csv(path: Path, monthly: list[dict[str, Any]]) -> None:
    lines = ["month,mean_residue_depth_m,min_residue_depth_m,max_residue_depth_m,row_count"]
    for row in monthly:
        lines.append(
            "{month},{mean},{minv},{maxv},{count}".format(
                month=row["month"],
                mean=format_optional(row["mean_residue_depth_m"]),
                minv=format_optional(row["min_residue_depth_m"]),
                maxv=format_optional(row["max_residue_depth_m"]),
                count=row["row_count"],
            )
        )
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def write_json(path: Path, payload: Any) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def mean(values: list[float]) -> float | None:
    return sum(values) / len(values) if values else None


def median(values: list[float]) -> float | None:
    if not values:
        return None
    ordered = sorted(values)
    midpoint = len(ordered) // 2
    if len(ordered) % 2 == 1:
        return ordered[midpoint]
    return (ordered[midpoint - 1] + ordered[midpoint]) / 2.0


def abs_days(value: int | None) -> int:
    return 0 if value is None else abs(value)


def format_optional(value: float | None) -> str:
    return "" if value is None else f"{value:.12g}"


if __name__ == "__main__":
    raise SystemExit(main())
