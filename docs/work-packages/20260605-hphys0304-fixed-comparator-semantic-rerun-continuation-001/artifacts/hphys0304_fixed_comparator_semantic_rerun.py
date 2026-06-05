#!/usr/bin/env python3
"""Run HPHYS0304 fixed-baseline semantic rerun and window reclassification."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Any

PACKAGE = "20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001"
REPO = Path(__file__).resolve().parents[4]
ARTIFACT_DIR = REPO / "docs" / "work-packages" / PACKAGE / "artifacts"
H0302_LEDGER = (
    REPO
    / "docs"
    / "work-packages"
    / "20260605-hphys0302-comparator-surface-audit-closure-001"
    / "artifacts"
    / "comparator-surface-audit-ledger.json"
)
H0300_RAW_LEDGER = (
    REPO
    / "docs"
    / "work-packages"
    / "20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001"
    / "artifacts"
    / "raw-post-raw-lineage-ledger.json"
)
H0303_MANIFEST = (
    REPO
    / "docs"
    / "work-packages"
    / "20260605-hphys0303-adr0016-fixed-comparator-ratification-001"
    / "artifacts"
    / "fixed-baseline-parquet-manifest.json"
)
H0303_LEDGER = (
    REPO
    / "docs"
    / "work-packages"
    / "20260605-hphys0303-adr0016-fixed-comparator-ratification-001"
    / "artifacts"
    / "comparator-ratification-ledger.json"
)
SEMANTIC_COMPARATOR = (
    REPO / "tools" / "legacy_comparison_suite" / "semantic_hillslope_wat_compare.py"
)
TOLERANCE_CONFIG = (
    REPO / "tools" / "legacy_comparison_suite" / "configs" / "pl14s_wat_tolerances.json"
)
CANDIDATE_DIR = Path("/tmp/hphys0300_full_20260605T155527Z/hillslope_output")
CANDIDATE_HEAD = "ab0801b58a4a038eda780ce5a108c27ea263a5d6"
FIXED_BASELINE_COMMIT = "47ac4c32faeea81bb99081f955a14c38b815ef4d"
YEAR_OFFSET = 2012
MATERIAL_WINDOW_DELTA_MM = 0.01
TARGET_COLUMNS = [
    "RM",
    "Snow-Water",
    "Total-Soil",
    "SoilWaterTotal",
    "Ep",
    "Es",
    "Dp",
    "Q",
    "latqcc",
]
WINDOW_SUM_COLUMNS = [
    "P",
    "RM",
    "Q",
    "Snow-Water",
    "Total-Soil",
    "SoilWaterTotal",
    "Ep",
    "Es",
    "Dp",
    "latqcc",
]
PARQUET_TO_CANONICAL = {
    "OFE": "OFE",
    "ofe_id": "OFE",
    "julian": "J",
    "year": "Y",
    "P": "P",
    "RM": "RM",
    "Q": "Q",
    "Ep": "Ep",
    "Es": "Es",
    "Er": "Er",
    "Dp": "Dp",
    "UpStrmQ": "UpStrmQ",
    "SubRIn": "SubRIn",
    "latqcc": "latqcc",
    "Total-Soil": "Total-Soil",
    "Total-Soil Water": "Total-Soil",
    "frozwt": "frozwt",
    "Snow-Water": "Snow-Water",
    "QOFE": "QOFE",
    "Tile": "Tile",
    "Irr": "Irr",
    "Area": "Area",
    "SoilWaterTotal": "SoilWaterTotal",
    "ProfileDepth": "ProfileDepth",
    "ProfilePorosityCap": "ProfilePorosityCap",
    "ProfileFCStore": "ProfileFCStore",
    "ProfileWPStore": "ProfileWPStore",
}


@dataclass(frozen=True)
class Window:
    hillslope_id: int
    window: str
    year: int
    start_julian: int
    end_julian: int

    @property
    def key(self) -> tuple[int, str, int, int, int]:
        return (
            self.hillslope_id,
            self.window,
            self.year,
            self.start_julian,
            self.end_julian,
        )


def run_command(args: list[str], cwd: Path = REPO) -> dict[str, Any]:
    completed = subprocess.run(
        args,
        cwd=cwd,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    return {
        "args": args,
        "returncode": completed.returncode,
        "stdout": completed.stdout,
        "stderr": completed.stderr,
    }


def git_stdout(args: list[str]) -> str:
    completed = subprocess.run(
        ["git", *args],
        cwd=REPO,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=True,
    )
    return completed.stdout.strip()


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def command_or_fail(command_log: list[dict[str, Any]], args: list[str]) -> None:
    result = run_command(args)
    command_log.append(result)
    if result["returncode"] != 0:
        write_json(ARTIFACT_DIR / "hphys0304-runner-command-log.json", command_log)
        raise RuntimeError(
            f"command failed rc={result['returncode']}: {' '.join(args)}\n"
            f"stdout={result['stdout']}\nstderr={result['stderr']}"
        )


def runtime_source_diff() -> dict[str, Any]:
    current_head = git_stdout(["rev-parse", "HEAD"])
    diff_text = git_stdout(["diff", "--name-only", f"{CANDIDATE_HEAD}..HEAD"])
    changed = [line for line in diff_text.splitlines() if line]
    runtime_changed = [
        path
        for path in changed
        if path.startswith("crates/") or path.startswith("src/")
    ]
    return {
        "candidate_output_commit": CANDIDATE_HEAD,
        "current_head": current_head,
        "changed_paths_since_candidate": changed,
        "runtime_source_paths_changed": runtime_changed,
        "candidate_outputs_reused": len(runtime_changed) == 0,
        "reuse_basis": (
            "openWEPP runtime source paths under crates/ and src/ are unchanged "
            "since the HPHYS0300 candidate-output commit"
            if len(runtime_changed) == 0
            else "runtime source paths changed; candidate outputs are stale"
        ),
    }


def load_rows(path: Path, year_offset: int) -> dict[tuple[int, int, int], dict[str, float]]:
    try:
        import pyarrow.parquet as pq
    except Exception as exc:
        raise RuntimeError("HPHYS0304 requires pyarrow to read parquet rows") from exc

    table = pq.read_table(path)
    arrays = {name: table[name].to_pylist() for name in table.column_names}
    rows: dict[tuple[int, int, int], dict[str, float]] = {}
    for index in range(table.num_rows):
        row: dict[str, float] = {}
        for src, dst in PARQUET_TO_CANONICAL.items():
            if src not in arrays:
                continue
            value = arrays[src][index]
            if value is None:
                continue
            row[dst] = float(value)
        if "OFE" not in row or "J" not in row or "Y" not in row:
            continue
        year = int(row["Y"]) + year_offset
        row["Y"] = float(year)
        key = (int(row["OFE"]), int(row["J"]), year)
        if key in rows:
            raise RuntimeError(f"duplicate row key {key} in {path}")
        rows[key] = row
    if not rows:
        raise RuntimeError(f"no keyed rows parsed from {path}")
    return rows


def sum_window(
    rows: dict[tuple[int, int, int], dict[str, float]],
    window: Window,
) -> dict[str, float]:
    totals = {column: 0.0 for column in WINDOW_SUM_COLUMNS}
    row_count = 0
    for (ofe, julian, year), row in rows.items():
        if ofe != 1 or year != window.year:
            continue
        if window.start_julian <= julian <= window.end_julian:
            row_count += 1
            for column in WINDOW_SUM_COLUMNS:
                totals[column] += float(row.get(column, 0.0))
    totals["row_count"] = float(row_count)
    return totals


def extract_windows(h0302_ledger: dict[str, Any]) -> list[Window]:
    seen: dict[tuple[int, str, int, int, int], Window] = {}
    for row in h0302_ledger["surface_rows"]:
        if row.get("surface") != "RM":
            continue
        window = Window(
            hillslope_id=int(row["hillslope_id"]),
            window=str(row["window"]),
            year=int(row["year"]),
            start_julian=int(row["start_julian"]),
            end_julian=int(row["end_julian"]),
        )
        seen[window.key] = window
    return sorted(seen.values(), key=lambda item: item.key)


def prior_residual_maps(
    h0302_ledger: dict[str, Any],
    h0300_raw_ledger: list[dict[str, Any]],
) -> dict[tuple[int, str, int, int, int], dict[str, float]]:
    residuals: dict[tuple[int, str, int, int, int], dict[str, float]] = defaultdict(dict)
    for row in h0302_ledger["surface_rows"]:
        if row.get("surface") not in {"RM", "Snow-Water"}:
            continue
        key = (
            int(row["hillslope_id"]),
            str(row["window"]),
            int(row["year"]),
            int(row["start_julian"]),
            int(row["end_julian"]),
        )
        residuals[key][f"previous_{row['surface']}_baseline_minus_candidate_mm"] = float(
            row["residual_baseline_minus_openwepp_mm"]
        )
    for row in h0300_raw_ledger:
        key = (
            int(row["hillslope_id"]),
            str(row["window"]),
            int(row["year"]),
            int(row["start_julian"]),
            int(row["end_julian"]),
        )
        for source, target in [
            ("observed_baseline_minus_candidate_total_soil_mm", "previous_Total-Soil_baseline_minus_candidate_mm"),
            ("observed_baseline_minus_candidate_snow_mm", "previous_Snow-Water_baseline_minus_candidate_mm"),
            ("observed_baseline_minus_candidate_rm_mm", "previous_RM_baseline_minus_candidate_mm"),
            ("observed_baseline_minus_candidate_q_mm", "previous_Q_baseline_minus_candidate_mm"),
        ]:
            if source in row:
                residuals[key][target] = float(row[source])
    return dict(residuals)


def aggregate_reports(report_dir: Path) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    by_column: dict[str, list[dict[str, Any]]] = defaultdict(list)
    pass_hillslopes: list[int] = []
    structural_failures: list[dict[str, Any]] = []
    for hillslope_id in range(1, 40):
        report = load_json(report_dir / f"H{hillslope_id}.semantic.json")
        comparison = report["comparison"]
        if comparison["semantic_pass"]:
            pass_hillslopes.append(hillslope_id)
        if comparison["only_baseline_count"] or comparison["only_candidate_count"]:
            structural_failures.append(
                {
                    "hillslope_id": hillslope_id,
                    "only_baseline_count": comparison["only_baseline_count"],
                    "only_candidate_count": comparison["only_candidate_count"],
                }
            )
        for stat in comparison["column_stats"]:
            by_column[stat["column"]].append(stat)

    summary_rows = []
    for column, stats in sorted(by_column.items()):
        summary_rows.append(
            {
                "column": column,
                "hillslope_fail_count": sum(1 for stat in stats if not stat["pass"]),
                "total_fail_count": sum(int(stat["fail_count"]) for stat in stats),
                "mean_abs_diff_mean": sum(float(stat["mean_abs_diff"]) for stat in stats)
                / len(stats),
                "max_abs_diff": max(float(stat["max_abs_diff"]) for stat in stats),
                "max_abs_key": max(
                    (
                        (
                            float(stat["max_abs_diff"]),
                            stat.get("max_abs_key"),
                        )
                        for stat in stats
                    ),
                    key=lambda item: item[0],
                )[1],
            }
        )
    suite = {
        "semantic_pass_hillslope_count": len(pass_hillslopes),
        "semantic_pass_hillslopes": pass_hillslopes,
        "semantic_fail_hillslope_count": 39 - len(pass_hillslopes),
        "structural_failures": structural_failures,
    }
    return summary_rows, suite


def previous_summary_delta(current_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    previous_path = (
        REPO
        / "docs"
        / "work-packages"
        / "20260605-hphys0302-comparator-surface-audit-closure-001"
        / "artifacts"
        / "full-39-suite-summary.json"
    )
    if not previous_path.exists():
        return []
    previous = {row["column"]: row for row in load_json(previous_path)}
    deltas = []
    for row in current_rows:
        prior = previous.get(row["column"])
        if prior is None:
            continue
        deltas.append(
            {
                "column": row["column"],
                "previous_hillslope_fail_count": prior["hillslope_fail_count"],
                "fixed_hillslope_fail_count": row["hillslope_fail_count"],
                "previous_total_fail_count": prior["total_fail_count"],
                "fixed_total_fail_count": row["total_fail_count"],
                "total_fail_count_delta": row["total_fail_count"]
                - prior["total_fail_count"],
                "previous_mean_abs_diff_mean": prior["mean_abs_diff_mean"],
                "fixed_mean_abs_diff_mean": row["mean_abs_diff_mean"],
                "mean_abs_diff_mean_delta": row["mean_abs_diff_mean"]
                - prior["mean_abs_diff_mean"],
                "previous_max_abs_diff": prior["max_abs_diff"],
                "fixed_max_abs_diff": row["max_abs_diff"],
                "max_abs_diff_delta": row["max_abs_diff"] - prior["max_abs_diff"],
            }
        )
    return deltas


def markdown_metrics(
    summary_rows: list[dict[str, Any]],
    suite: dict[str, Any],
    delta_rows: list[dict[str, Any]],
) -> str:
    rows = {row["column"]: row for row in summary_rows}
    delta = {row["column"]: row for row in delta_rows}
    lines = [
        "# Fixed-Baseline Semantic Metrics",
        "",
        "Status: complete",
        "",
        "Evidence mode: ran",
        "",
        "Static:",
        "",
        "- ADR-0016 makes the fixed `wepp_260430` comparator the active H1..H39 baseline artifact source.",
        "- Candidate openWEPP parquets are reused only after runtime-source diff validation.",
        "",
        "Ran:",
        "",
        f"- Semantic pass hillslopes: `{suite['semantic_pass_hillslope_count']}/39`.",
        f"- Structural row/key failures: `{len(suite['structural_failures'])}`.",
        "",
        "## Focus Columns",
        "",
        "| Column | Hillslope Failures | Row Failures | Mean Abs Diff Mean | Max Abs Diff | Fail Delta vs HPHYS0302 |",
        "| --- | ---: | ---: | ---: | ---: | ---: |",
    ]
    for column in TARGET_COLUMNS:
        row = rows.get(column)
        if row is None:
            continue
        fail_delta = delta.get(column, {}).get("total_fail_count_delta", "n/a")
        lines.append(
            "| {column} | {hillslope_fail_count} | {total_fail_count} | {mean:.6f} | {max:.6f} | {delta} |".format(
                column=column,
                hillslope_fail_count=row["hillslope_fail_count"],
                total_fail_count=row["total_fail_count"],
                mean=row["mean_abs_diff_mean"],
                max=row["max_abs_diff"],
                delta=fail_delta,
            )
        )
    lines.extend(
        [
            "",
            "Interpretation: these are higher-confidence single-OFE daily WAT investigation signals under ADR-0011. They do not by themselves identify term-level melt producer defects.",
            "",
        ]
    )
    return "\n".join(lines)


def classify_window(
    residuals: dict[str, float],
    previous: dict[str, float],
) -> tuple[str, str]:
    rm_abs = abs(residuals["RM"])
    snow_abs = abs(residuals["Snow-Water"])
    total_abs = abs(residuals.get("Total-Soil", 0.0))
    if rm_abs <= 0.1 and snow_abs <= 0.2 and total_abs <= 0.5:
        return (
            "fixed-baseline-daily-window-closed-no-production-edit",
            "Fixed-baseline daily WAT residual sums are within configured daily tolerances; no producer edit is authorized without paired term/state evidence.",
        )
    if previous:
        previous_rm = abs(previous.get("previous_RM_baseline_minus_candidate_mm", residuals["RM"]))
        previous_snow = abs(previous.get("previous_Snow-Water_baseline_minus_candidate_mm", residuals["Snow-Water"]))
        rm_reduction = previous_rm - rm_abs
        snow_reduction = previous_snow - snow_abs
        if (
            rm_reduction > MATERIAL_WINDOW_DELTA_MM
            or snow_reduction > MATERIAL_WINDOW_DELTA_MM
        ):
            return (
                "fixed-baseline-reduced-but-term-state-hold",
                "Fixed comparator changes reduce at least one aggregate daily residual, but paired melt-term/state surfaces are still required before producer or downstream edits.",
            )
        if (
            abs(rm_reduction) <= MATERIAL_WINDOW_DELTA_MM
            and abs(snow_reduction) <= MATERIAL_WINDOW_DELTA_MM
        ):
            return (
                "fixed-baseline-unchanged-term-state-hold",
                "Fixed comparator changes do not materially move aggregate daily residuals; paired melt-term/state surfaces remain required before producer or downstream edits.",
            )
    return (
        "fixed-baseline-term-state-hold",
        "Fixed-baseline daily WAT residual remains an ADR-0011 investigation signal; aggregate daily and hourly surfaces are not term-level producer authority.",
    )


def reclassify_windows(
    manifest: dict[str, Any],
    windows: list[Window],
    previous_residuals: dict[tuple[int, str, int, int, int], dict[str, float]],
) -> list[dict[str, Any]]:
    partition_dir = Path(manifest["partition_dir"])
    rows = []
    for window in windows:
        baseline = load_rows(partition_dir / f"baseline_H{window.hillslope_id}.parquet", 0)
        candidate = load_rows(
            CANDIDATE_DIR / f"H{window.hillslope_id}.wat.parquet",
            YEAR_OFFSET,
        )
        baseline_sums = sum_window(baseline, window)
        candidate_sums = sum_window(candidate, window)
        residuals = {
            column: baseline_sums[column] - candidate_sums[column]
            for column in WINDOW_SUM_COLUMNS
        }
        previous = previous_residuals.get(window.key, {})
        classification, reason = classify_window(residuals, previous)
        rows.append(
            {
                "hillslope_id": window.hillslope_id,
                "window": window.window,
                "year": window.year,
                "start_julian": window.start_julian,
                "end_julian": window.end_julian,
                "baseline_rows": int(baseline_sums["row_count"]),
                "candidate_rows": int(candidate_sums["row_count"]),
                "fixed_baseline_minus_candidate_mm": residuals,
                "previous_original_baseline_minus_candidate_mm": previous,
                "adr0011_confidence_tier": "higher-confidence single-OFE daily WAT investigation signal",
                "reclassification": classification,
                "reason": reason,
                "production_edit_authorized": False,
                "required_next_package": "20260605-hphys0305-paired-melt-term-state-instrumentation-001",
                "required_next_surfaces": [
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
                ],
            }
        )
    return rows


def markdown_reclassification(rows: list[dict[str, Any]]) -> str:
    lines = [
        "# Snow/RM Window Reclassification",
        "",
        "Status: complete",
        "",
        "Evidence mode: ran",
        "",
        "Static:",
        "",
        "- ADR-0011 treats single-OFE daily water-balance deltas as higher-confidence investigation signals.",
        "- Aggregate daily WAT residuals still do not identify `amelt`/`bmelt`/`cmelt`/`dmelt` term ownership.",
        "",
        "Ran:",
        "",
        f"- Reclassified `{len(rows)}` H1/H7/H39 target windows against the fixed baseline.",
        "- Production edit authorized: `false` for every row.",
        "",
        "| H | Window | Year | J Range | RM Residual | Snow-Water Residual | Total-Soil Residual | Classification |",
        "| ---: | --- | ---: | --- | ---: | ---: | ---: | --- |",
    ]
    for row in rows:
        residuals = row["fixed_baseline_minus_candidate_mm"]
        lines.append(
            "| H{h} | {window} | {year} | {start}-{end} | {rm:.6f} | {snow:.6f} | {soil:.6f} | {classification} |".format(
                h=row["hillslope_id"],
                window=row["window"],
                year=row["year"],
                start=row["start_julian"],
                end=row["end_julian"],
                rm=residuals["RM"],
                snow=residuals["Snow-Water"],
                soil=residuals["Total-Soil"],
                classification=row["reclassification"],
            )
        )
    lines.extend(
        [
            "",
            "Continuation: HPHYS0305 is the required paired melt-term/state instrumentation package. No snow, forcing, WB13, WB17, WB18, WB19, or WB12 production edit is authorized by HPHYS0304.",
            "",
        ]
    )
    return "\n".join(lines)


def write_text_artifact(path: Path, text: str) -> None:
    path.write_text(text, encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--python", default=sys.executable)
    args = parser.parse_args()

    ARTIFACT_DIR.mkdir(parents=True, exist_ok=True)
    report_dir = ARTIFACT_DIR / "fixed-baseline-semantic-reports"
    report_dir.mkdir(parents=True, exist_ok=True)
    command_log: list[dict[str, Any]] = []

    manifest = load_json(H0303_MANIFEST)
    h0303_ledger = load_json(H0303_LEDGER)
    h0302_ledger = load_json(H0302_LEDGER)
    h0300_raw_ledger = load_json(H0300_RAW_LEDGER)
    source_diff = runtime_source_diff()
    if not source_diff["candidate_outputs_reused"]:
        raise RuntimeError(
            "candidate outputs are stale because runtime source paths changed: "
            + ", ".join(source_diff["runtime_source_paths_changed"])
        )

    partition_dir = Path(manifest["partition_dir"])
    if not partition_dir.exists():
        raise RuntimeError(f"fixed baseline partition directory is missing: {partition_dir}")
    if not CANDIDATE_DIR.exists():
        raise RuntimeError(f"candidate parquet directory is missing: {CANDIDATE_DIR}")
    if h0303_ledger["fixed_sha"] != FIXED_BASELINE_COMMIT:
        raise RuntimeError(
            f"unexpected fixed comparator commit {h0303_ledger['fixed_sha']}"
        )

    for hillslope_id in range(1, 40):
        baseline = partition_dir / f"baseline_H{hillslope_id}.parquet"
        candidate = CANDIDATE_DIR / f"H{hillslope_id}.wat.parquet"
        report = report_dir / f"H{hillslope_id}.semantic.json"
        command_or_fail(
            command_log,
            [
                args.python,
                str(SEMANTIC_COMPARATOR),
                "--baseline-wat",
                str(baseline),
                "--candidate-wat",
                str(candidate),
                "--candidate-year-offset",
                str(YEAR_OFFSET),
                "--tolerance-config",
                str(TOLERANCE_CONFIG),
                "--report-json",
                str(report),
            ],
        )

    summary_rows, suite = aggregate_reports(report_dir)
    delta_rows = previous_summary_delta(summary_rows)
    windows = extract_windows(h0302_ledger)
    reclassification = reclassify_windows(
        manifest,
        windows,
        prior_residual_maps(h0302_ledger, h0300_raw_ledger),
    )
    ledger = {
        "package": PACKAGE,
        "status": "executed-hold",
        "baseline_comparator": {
            "source": "/workdir/wepp-forest_260430_baseline",
            "fixed_commit": FIXED_BASELINE_COMMIT,
            "partition_dir": manifest["partition_dir"],
            "manifest": str(H0303_MANIFEST),
            "year_key_validation_pass": manifest.get("year_key_validation_pass"),
        },
        "candidate": {
            "run_root": "/tmp/hphys0300_full_20260605T155527Z",
            "hillslope_output_dir": str(CANDIDATE_DIR),
            **source_diff,
            "candidate_year_offset": YEAR_OFFSET,
        },
        "semantic_suite": suite,
        "target_windows": {
            "count": len(reclassification),
            "production_edit_authorized": False,
            "required_next_package": "20260605-hphys0305-paired-melt-term-state-instrumentation-001",
            "all_rows_production_edit_authorized_false": all(
                not row["production_edit_authorized"] for row in reclassification
            ),
            "classifications": sorted(
                {
                    row["reclassification"]
                    for row in reclassification
                }
            ),
        },
        "hphys0302_hold_carried_forward": True,
        "production_edit_authorized": False,
    }

    write_json(ARTIFACT_DIR / "fixed-baseline-semantic-summary.json", summary_rows)
    write_json(ARTIFACT_DIR / "fixed-baseline-semantic-suite-ledger.json", ledger)
    write_json(ARTIFACT_DIR / "fixed-vs-previous-summary-delta.json", delta_rows)
    write_json(ARTIFACT_DIR / "snow-rm-window-reclassification.json", reclassification)
    write_json(ARTIFACT_DIR / "hphys0304-runner-command-log.json", command_log)
    write_text_artifact(
        ARTIFACT_DIR / "fixed-baseline-semantic-metrics.md",
        markdown_metrics(summary_rows, suite, delta_rows),
    )
    write_text_artifact(
        ARTIFACT_DIR / "snow-rm-window-reclassification.md",
        markdown_reclassification(reclassification),
    )
    write_text_artifact(
        ARTIFACT_DIR / "continuation-decision.md",
        "\n".join(
            [
                "# Continuation Decision",
                "",
                "Status: complete",
                "",
                "Evidence mode: ran",
                "",
                "Static:",
                "",
                "- ADR-0016 Required Continuation Order step 1 is complete.",
                "- HPHYS0305 is scaffolded as Required Continuation Order step 2.",
                "",
                "Ran:",
                "",
                f"- Fixed-baseline semantic pass: `{suite['semantic_pass_hillslope_count']}/39` hillslopes.",
                f"- Target windows reclassified: `{len(reclassification)}`.",
                "- Production edit authorized: `false`.",
                "",
                "Decision: continue with HPHYS0305 paired melt-term/state instrumentation. Do not patch snow, melt, forcing, WB13, WB17, WB18, WB19, or WB12 from aggregate residuals.",
                "",
            ]
        ),
    )


if __name__ == "__main__":
    main()
