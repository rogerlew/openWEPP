#!/usr/bin/env python3
"""Run HPHYS0300 raw hourly melt/post-raw routing lineage diagnostics."""

from __future__ import annotations

import argparse
import importlib.util
import json
import math
import subprocess
import sys
from collections import Counter
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACT_DIR = PACKAGE_DIR / "artifacts"
HPHYS0299_SCRIPT = (
    REPO
    / "docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/hphys0299_corrected_partition.py"
)
HPHYS0299_LEDGER = (
    REPO
    / "docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/corrected-partition-ledger.json"
)

WINDOW_TOLERANCE_MM = 2.0
TERM_STATE_REQUIRED_SYMBOLS = (
    "amelt",
    "bmelt",
    "cmelt",
    "dmelt",
    "hrrain",
    "hrtemp",
    "tdpt",
    "hrad",
    "cloudC",
    "vwind",
    "snodpt",
    "densgt",
)


def load_module(path: Path, name: str) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


HPHYS0299 = load_module(HPHYS0299_SCRIPT, "hphys0299_corrected_partition")
HPHYS0265 = HPHYS0299.HPHYS0265


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def rounded(value: Any, digits: int = 6) -> Any:
    if value is None:
        return None
    if isinstance(value, float):
        if math.isnan(value):
            return None
        return round(value, digits)
    return value


def current_git_head() -> str:
    proc = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=REPO,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    return proc.stdout.strip() if proc.returncode == 0 else "unknown"


def run_hphys0299(run_root: Path, artifact_dir: Path, trace_max_days: int) -> int:
    cmd = [
        sys.executable,
        str(HPHYS0299_SCRIPT),
        "--run-root",
        str(run_root),
        "--artifact-dir",
        str(artifact_dir),
        "--trace-max-days",
        str(trace_max_days),
    ]
    proc = subprocess.run(
        cmd,
        cwd=REPO,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    logs = run_root / "logs"
    logs.mkdir(parents=True, exist_ok=True)
    (logs / "hphys0300_hphys0299.stdout").write_text(proc.stdout, encoding="utf-8")
    (logs / "hphys0300_hphys0299.stderr").write_text(proc.stderr, encoding="utf-8")
    return proc.returncode


def normalize_hphys0300_artifact_labels(artifact_dir: Path) -> None:
    metrics = artifact_dir / "full-39-suite-metrics.md"
    if not metrics.exists():
        return
    text = metrics.read_text(encoding="utf-8")
    text = text.replace("# HPHYS0299 Full-39 Suite Metrics", "# HPHYS0300 Full-39 Suite Metrics", 1)
    metrics.write_text(text, encoding="utf-8")


def route_window(row: dict[str, Any]) -> dict[str, Any]:
    cut = row["first_divergent_cut_point"]
    raw_delta = float(row["baseline_raw_melt_minus_openwepp_raw_melt_mm"])
    post_delta = float(row["baseline_post_wmelt_minus_openwepp_routed_melt_mm"])
    post_minus_raw = post_delta - raw_delta
    baseline_negative = float(row.get("baseline_negative_raw_melt_sum_mm", 0.0))
    openwepp_negative = float(row.get("openwepp_negative_raw_melt_sum_mm", 0.0))

    term_state_evidence_status = "aggregate-only"
    required_symbols = list(TERM_STATE_REQUIRED_SYMBOLS)
    production_edit_authorized = False
    follow_on = "add paired baseline melt-term/state instrumentation before production edits"
    reason = (
        "HPHYS0300 has aggregate corrected HPHYS0299 cut-point evidence, but not paired "
        "baseline/openWEPP melt-term and snow-state input evidence."
    )

    if cut == "hourly-forcing":
        route = "corrected-depth-hourly-forcing-hold"
        follow_on = "resolve H39 first-2013 corrected-depth hrrain/hrsnow forcing seam separately"
        reason = (
            "Corrected HPHYS0299 depth-vs-depth forcing still diverges before raw melt; "
            "raw/post-raw production edits are not authorized for this window."
        )
    elif cut == "raw-hourly-melt":
        route = "raw-hourly-melt-term-state-hold"
        reason = (
            "Raw hrmlt differs before post-raw routed-melt handling, but aggregate raw melt sums do not "
            "identify whether the source is melt.for term math, forcing input state, or snowd state."
        )
    elif cut == "negative-melt-correction":
        if baseline_negative == 0.0 and abs(raw_delta) <= WINDOW_TOLERANCE_MM:
            route = "post-raw-routing-without-baseline-negative-melt-hold"
            reason = (
                "Raw melt is within tolerance and pinned-baseline negative raw melt is zero; "
                "the row is openWEPP post-raw/routing evidence, not legacy-defective acceptance."
            )
            required_symbols = [
                "pstvML",
                "ngtvML",
                "pstvhr",
                "hrmlt_before_rain_addition",
                "hrrain",
                "wmelt",
                "totmel",
            ]
        else:
            route = "negative-melt-reconstruction-hold"
            reason = (
                "Post-raw divergence involves signed melt; retain corrected negative-melt authority "
                "and require reconstruction before any verdict changes."
            )
    elif abs(post_delta) > WINDOW_TOLERANCE_MM:
        route = "post-raw-routing-hold"
        required_symbols = [
            "pstvML",
            "ngtvML",
            "pstvhr",
            "hrrain",
            "wmelt",
            "totmel",
        ]
    else:
        route = "raw-post-raw-closed"
        term_state_evidence_status = "not-required-for-closed-aggregate"
        required_symbols = []
        follow_on = "return to next first-divergent cut-point"
        reason = "Corrected HPHYS0299 aggregate raw/post-raw evidence is within tolerance."

    return {
        "hillslope_id": row["hillslope_id"],
        "window": row["window"],
        "year": row["year"],
        "start_julian": row["start_julian"],
        "end_julian": row["end_julian"],
        "source_hphys0299_cut_point": cut,
        "source_hphys0299_verdict": row["verdict"],
        "hphys0300_route": route,
        "reason": reason,
        "follow_on": follow_on,
        "term_state_evidence_status": term_state_evidence_status,
        "required_term_state_symbols": required_symbols,
        "production_edit_authorized": production_edit_authorized,
        "baseline_raw_melt_minus_openwepp_raw_melt_mm": rounded(raw_delta),
        "baseline_post_wmelt_minus_openwepp_routed_melt_mm": rounded(post_delta),
        "post_raw_minus_raw_delta_mm": rounded(post_minus_raw),
        "baseline_raw_rain_minus_openwepp_raw_rain_mm": rounded(
            float(row["baseline_raw_rain_minus_openwepp_raw_rain_mm"])
        ),
        "baseline_raw_snow_minus_openwepp_raw_snow_mm": rounded(
            float(row["baseline_raw_snow_minus_openwepp_raw_snow_mm"])
        ),
        "baseline_negative_raw_melt_sum_mm": rounded(baseline_negative),
        "openwepp_negative_raw_melt_sum_mm": rounded(openwepp_negative),
        "observed_baseline_minus_candidate_rm_mm": rounded(
            float(row["observed_baseline_minus_candidate_rm_mm"])
        ),
        "observed_baseline_minus_candidate_snow_mm": rounded(
            float(row["observed_baseline_minus_candidate_snow_mm"])
        ),
        "observed_baseline_minus_candidate_total_soil_mm": rounded(
            float(row["observed_baseline_minus_candidate_total_soil_mm"])
        ),
        "source_provenance": row.get("source_provenance", []),
    }


def write_summary(run_root: Path, artifact_dir: Path, ledger: list[dict[str, Any]]) -> None:
    route_counts = Counter(row["hphys0300_route"] for row in ledger)
    cut_counts = Counter(row["source_hphys0299_cut_point"] for row in ledger)
    headers = [
        "Hill",
        "Window",
        "Days",
        "HPHYS0299 Cut",
        "HPHYS0300 Route",
        "Raw Δ",
        "Post Δ",
        "Post-Raw Δ",
        "Base Neg",
        "Open Neg",
        "Edit?",
    ]
    rows = []
    for row in ledger:
        rows.append(
            [
                f"H{row['hillslope_id']}",
                row["window"],
                f"{row['year']} {row['start_julian']}-{row['end_julian']}",
                row["source_hphys0299_cut_point"],
                row["hphys0300_route"],
                row["baseline_raw_melt_minus_openwepp_raw_melt_mm"],
                row["baseline_post_wmelt_minus_openwepp_routed_melt_mm"],
                row["post_raw_minus_raw_delta_mm"],
                row["baseline_negative_raw_melt_sum_mm"],
                row["openwepp_negative_raw_melt_sum_mm"],
                row["production_edit_authorized"],
            ]
        )

    text = "# HPHYS0300 Raw/Post-Raw Melt Lineage Summary\n\n"
    text += "Ran:\n\n"
    text += f"- Run root: `{run_root}`\n"
    text += f"- Candidate HEAD: `{current_git_head()}`\n"
    text += f"- Corrected partition source: `{artifact_dir / 'corrected-partition-ledger.json'}`\n"
    text += "- Contract authority: `SC-SNOWFREEZE-001#INV-SNOWFREEZE-031` and `SC-WATBAL-001#INV-WATBAL-075`.\n"
    text += "- Scope: all nine H1/H7/H39 target windows plus same-HEAD full H1..H39 metrics.\n\n"
    text += "## Route Counts\n\n"
    for route, count in sorted(route_counts.items()):
        text += f"- `{route}`: `{count}` windows\n"
    text += "\n## HPHYS0299 Cut-Point Counts\n\n"
    for cut, count in sorted(cut_counts.items()):
        text += f"- `{cut}`: `{count}` windows\n"
    text += "\n## Ledger\n\n"
    text += HPHYS0265.markdown_table(headers, rows)
    text += "\n\n## Disposition\n\n"
    text += (
        "- Production edits are not authorized by this run because raw/post-raw rows still "
        "have `term_state_evidence_status = aggregate-only`.\n"
    )
    text += (
        "- Seven windows require paired `melt.for` term/state evidence before raw-hourly-melt "
        "migration or correction.\n"
    )
    text += (
        "- H7 first-2013 remains a post-raw routed-melt hold, not legacy-defective acceptance, "
        "because `baseline_negative_raw_melt_sum_mm = 0.0`.\n"
    )
    text += (
        "- H39 first-2013 remains a corrected-depth hourly-forcing seam and must be handled "
        "separately from raw/post-raw melt closure.\n"
    )
    text += (
        "- WB17/WB18/WB19/WB13 compensation remains prohibited by `INV-WATBAL-075`.\n"
    )
    (artifact_dir / "raw-post-raw-lineage-summary.md").write_text(text, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-root", type=Path, required=True)
    parser.add_argument("--artifact-dir", type=Path, default=ARTIFACT_DIR)
    parser.add_argument("--trace-max-days", type=int, default=1_800)
    parser.add_argument("--use-existing-corrected-ledger", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    args.run_root.mkdir(parents=True, exist_ok=True)
    args.artifact_dir.mkdir(parents=True, exist_ok=True)

    corrected_ledger_path = args.artifact_dir / "corrected-partition-ledger.json"
    if args.use_existing_corrected_ledger:
        source = corrected_ledger_path if corrected_ledger_path.exists() else HPHYS0299_LEDGER
        corrected = read_json(source)
        if source != corrected_ledger_path:
            write_json(corrected_ledger_path, corrected)
    else:
        rc = run_hphys0299(args.run_root, args.artifact_dir, args.trace_max_days)
        if rc != 0:
            return int(rc)
        normalize_hphys0300_artifact_labels(args.artifact_dir)
        corrected = read_json(corrected_ledger_path)

    ledger = [route_window(row) for row in corrected]
    write_json(args.artifact_dir / "raw-post-raw-lineage-ledger.json", ledger)
    write_summary(args.run_root, args.artifact_dir, ledger)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
