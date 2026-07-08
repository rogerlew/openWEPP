#!/usr/bin/env python3
"""Replay WA refined-75 pass-sediment attribution from prior ladder outputs."""

from __future__ import annotations

import hashlib
import json
import math
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import pyarrow.parquet as pq


REPO = Path(__file__).resolve().parents[4]
PACKAGE = REPO / "docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001"
ARTIFACTS = PACKAGE / "artifacts"
SOURCE_PACKAGE = (
    REPO
    / "docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001"
)
SOURCE_ARTIFACTS = SOURCE_PACKAGE / "artifacts"
RUN_ROOT = SOURCE_ARTIFACTS / "coupled-spacetime-runs/wa_cascades_forest_h1"
CANDIDATE_RUNG = "dx2p5_dt75"
REFERENCE_RUNG = "dx1p25_dt75"
MEMBER_ID = "wa_cascades_forest_h1"
ROLE = "fine_reference_adequacy_dt75"
SEDIMENT_COLUMN = "tdep"
TARGET_YEAR = 4
SEDIMENT_TOLERANCE = 0.02
ADEQUACY_THRESHOLD = SEDIMENT_TOLERANCE / 3.0
DRY_EPS_M3 = 1.0e-12


def rel(path: Path) -> str:
    return str(path.relative_to(REPO))


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_json(path: Path) -> Any:
    return json.loads(path.read_text())


def read_pass(path: Path) -> list[dict[str, Any]]:
    table = pq.read_table(path)
    data = table.to_pydict()
    rows = []
    for idx in range(table.num_rows):
        rows.append({name: data[name][idx] for name in table.column_names})
    return rows


def read_trace(path: Path) -> dict[int, dict[int, dict[str, Any]]]:
    by_day: dict[int, dict[int, dict[str, Any]]] = defaultdict(dict)
    with path.open() as handle:
        for line in handle:
            row = json.loads(line)
            by_day[int(row["sim_day_index"])][int(row["lane_index"])] = row
    return dict(by_day)


def annual_sums(rows: list[dict[str, Any]], column: str) -> dict[int, float]:
    sums: dict[int, float] = defaultdict(float)
    for row in rows:
        sums[int(row["year"])] += float(row[column])
    return dict(sorted(sums.items()))


def relative_delta(candidate: float, reference: float) -> float:
    if abs(reference) <= 0.0:
        return math.inf if abs(candidate) > 0.0 else 0.0
    return abs(candidate - reference) / abs(reference)


def l1(weights_a: list[float], weights_b: list[float]) -> float:
    return sum(abs(float(a) - float(b)) for a, b in zip(weights_a, weights_b))


def trace_day_summary(
    candidate: dict[int, dict[str, Any]], reference: dict[int, dict[str, Any]]
) -> dict[str, Any]:
    lanes = sorted(set(candidate) | set(reference))
    per_lane = []
    aggregate = {
        "source_m3_delta": 0.0,
        "outlet_m3_delta": 0.0,
        "mesh_end_storage_m3_delta": 0.0,
        "tail_fold_m3_delta": 0.0,
        "clamp_m3_candidate": 0.0,
        "clamp_m3_reference": 0.0,
        "max_shape_l1": 0.0,
    }
    terminal = None
    for lane in lanes:
        cand = candidate[lane]
        ref = reference[lane]
        shape_l1 = l1(cand["routed_hourly_weights"], ref["routed_hourly_weights"])
        aggregate["source_m3_delta"] += float(cand["source_m3"]) - float(ref["source_m3"])
        aggregate["outlet_m3_delta"] += float(cand["outlet_m3"]) - float(ref["outlet_m3"])
        aggregate["mesh_end_storage_m3_delta"] += float(cand["mesh_end_storage_m3"]) - float(
            ref["mesh_end_storage_m3"]
        )
        aggregate["tail_fold_m3_delta"] += float(cand["tail_fold_m3"]) - float(
            ref["tail_fold_m3"]
        )
        aggregate["clamp_m3_candidate"] += float(cand["clamp_m3"])
        aggregate["clamp_m3_reference"] += float(ref["clamp_m3"])
        aggregate["max_shape_l1"] = max(aggregate["max_shape_l1"], shape_l1)
        record = {
            "lane_index": lane,
            "candidate_source_m3": float(cand["source_m3"]),
            "reference_source_m3": float(ref["source_m3"]),
            "source_m3_delta": float(cand["source_m3"]) - float(ref["source_m3"]),
            "candidate_outlet_m3": float(cand["outlet_m3"]),
            "reference_outlet_m3": float(ref["outlet_m3"]),
            "outlet_m3_delta": float(cand["outlet_m3"]) - float(ref["outlet_m3"]),
            "candidate_mesh_end_storage_m3": float(cand["mesh_end_storage_m3"]),
            "reference_mesh_end_storage_m3": float(ref["mesh_end_storage_m3"]),
            "mesh_end_storage_m3_delta": float(cand["mesh_end_storage_m3"])
            - float(ref["mesh_end_storage_m3"]),
            "candidate_tail_fold_m3": float(cand["tail_fold_m3"]),
            "reference_tail_fold_m3": float(ref["tail_fold_m3"]),
            "tail_fold_m3_delta": float(cand["tail_fold_m3"])
            - float(ref["tail_fold_m3"]),
            "candidate_clamp_m3": float(cand["clamp_m3"]),
            "reference_clamp_m3": float(ref["clamp_m3"]),
            "shape_l1": shape_l1,
            "candidate_uniform_shape": bool(cand["uniform_shape"]),
            "reference_uniform_shape": bool(ref["uniform_shape"]),
            "candidate_source_shape_degenerate": bool(cand["erosion_source_shape_degenerate"]),
            "reference_source_shape_degenerate": bool(ref["erosion_source_shape_degenerate"]),
        }
        if cand["is_terminal_lane"]:
            terminal = {
                "lane_index": lane,
                "candidate_terminal_day_outlet_m3": float(cand["terminal_day_outlet_m3"]),
                "reference_terminal_day_outlet_m3": float(ref["terminal_day_outlet_m3"]),
                "terminal_day_outlet_m3_delta": float(cand["terminal_day_outlet_m3"])
                - float(ref["terminal_day_outlet_m3"]),
                "terminal_shape_l1": shape_l1,
            }
        per_lane.append(record)
    aggregate["terminal"] = terminal
    return {"aggregate": aggregate, "per_lane": per_lane}


def run_trace_summary(trace: dict[int, dict[int, dict[str, Any]]]) -> dict[str, Any]:
    rows = [row for by_lane in trace.values() for row in by_lane.values()]
    lanes = sorted({int(row["lane_index"]) for row in rows})
    terminal_rows = [row for row in rows if row["is_terminal_lane"]]
    return {
        "row_count": len(rows),
        "day_count": len(trace),
        "lanes": lanes,
        "terminal_row_count": len(terminal_rows),
        "uniform_shape_rows": sum(1 for row in rows if row["uniform_shape"]),
        "erosion_source_shape_degenerate_rows": sum(
            1 for row in rows if row["erosion_source_shape_degenerate"]
        ),
        "total_clamp_m3": sum(float(row["clamp_m3"]) for row in rows),
        "total_source_m3": sum(float(row["source_m3"]) for row in rows),
        "total_outlet_m3": sum(float(row["outlet_m3"]) for row in rows),
        "total_mesh_end_storage_m3": sum(float(row["mesh_end_storage_m3"]) for row in rows),
        "total_tail_fold_m3": sum(float(row["tail_fold_m3"]) for row in rows),
        "terminal_outlet_m3": sum(
            float(row["terminal_day_outlet_m3"]) for row in terminal_rows
        ),
    }


def pass_day_by_day(
    candidate_rows: list[dict[str, Any]], reference_rows: list[dict[str, Any]]
) -> list[dict[str, Any]]:
    cand_by_day = {int(row["sim_day_index"]): row for row in candidate_rows}
    ref_by_day = {int(row["sim_day_index"]): row for row in reference_rows}
    days = sorted(set(cand_by_day) & set(ref_by_day))
    output = []
    for day in days:
        cand = cand_by_day[day]
        ref = ref_by_day[day]
        output.append(
            {
                "sim_day_index": day,
                "year": int(cand["year"]),
                "julian": int(cand["julian"]),
                "candidate_tdep_kg": float(cand["tdep"]),
                "reference_tdep_kg": float(ref["tdep"]),
                "tdep_delta_kg": float(cand["tdep"]) - float(ref["tdep"]),
                "abs_tdep_delta_kg": abs(float(cand["tdep"]) - float(ref["tdep"])),
                "candidate_tdet_kg": float(cand["tdet"]),
                "reference_tdet_kg": float(ref["tdet"]),
                "tdet_delta_kg": float(cand["tdet"]) - float(ref["tdet"]),
                "candidate_runvol_m3": float(cand["runvol"]),
                "reference_runvol_m3": float(ref["runvol"]),
                "runvol_delta_m3": float(cand["runvol"]) - float(ref["runvol"]),
                "candidate_sbrunv_m3": float(cand["sbrunv"]),
                "reference_sbrunv_m3": float(ref["sbrunv"]),
                "sbrunv_delta_m3": float(cand["sbrunv"]) - float(ref["sbrunv"]),
                "candidate_peakro_m3_s": float(cand["peakro"]),
                "reference_peakro_m3_s": float(ref["peakro"]),
                "peakro_delta_m3_s": float(cand["peakro"]) - float(ref["peakro"]),
            }
        )
    return output


def find_prior_comparison(summary: dict[str, Any]) -> dict[str, Any]:
    for row in summary["comparisons"]:
        if (
            row["member_id"] == MEMBER_ID
            and row["comparison_role"] == ROLE
            and row["candidate_rung"] == CANDIDATE_RUNG
            and row["reference_rung"] == REFERENCE_RUNG
        ):
            return row
    raise RuntimeError("prior comparison not found")


def write_markdown(result: dict[str, Any]) -> None:
    failing = result["surface_confirmation"]
    top = result["daily_attribution"]["target_year_top_abs_tdep_deltas"][0]
    trace = top["trace"]
    terminal = trace["aggregate"]["terminal"]
    md = f"""# WA Sediment Attribution

Evidence mode: Ran.

## Surface Confirmation

- Member: `{MEMBER_ID}`
- Role: `{ROLE}`
- Candidate: `{CANDIDATE_RUNG}`
- Reference: `{REFERENCE_RUNG}`
- Failing surface: `{failing['prior_summary_max_surface']}`
- Prior annual max relative delta: `{failing['prior_summary_max_rel']:.12g}`
- Recomputed year-{TARGET_YEAR} relative delta: `{failing['recomputed_target_year_rel']:.12g}`
- One-third adequacy threshold: `{ADEQUACY_THRESHOLD:.12g}`
- Verdict: `FAIL` for the broader mesh-policy gate.

## Daily Attribution

Year {TARGET_YEAR} has exactly `{result['daily_attribution']['target_year_nonzero_tdep_delta_days']}`
nonzero daily `tdep` delta day. The whole annual miss is day
`{top['sim_day_index']}` / julian `{top['julian']}`:

| Surface | Candidate | Reference | Delta |
|---|---:|---:|---:|
| `tdep` kg | {top['candidate_tdep_kg']:.12g} | {top['reference_tdep_kg']:.12g} | {top['tdep_delta_kg']:.12g} |
| `tdet` kg | {top['candidate_tdet_kg']:.12g} | {top['reference_tdet_kg']:.12g} | {top['tdet_delta_kg']:.12g} |
| pass `runvol` m3 | {top['candidate_runvol_m3']:.12g} | {top['reference_runvol_m3']:.12g} | {top['runvol_delta_m3']:.12g} |
| pass `sbrunv` m3 | {top['candidate_sbrunv_m3']:.12g} | {top['reference_sbrunv_m3']:.12g} | {top['sbrunv_delta_m3']:.12g} |
| pass `peakro` m3/s | {top['candidate_peakro_m3_s']:.12g} | {top['reference_peakro_m3_s']:.12g} | {top['peakro_delta_m3_s']:.12g} |

## Routed Trace Comparison On Day {top['sim_day_index']}

| Surface | Value |
|---|---:|
| aggregate source delta m3 | {trace['aggregate']['source_m3_delta']:.12g} |
| aggregate outlet delta m3 | {trace['aggregate']['outlet_m3_delta']:.12g} |
| aggregate end-storage delta m3 | {trace['aggregate']['mesh_end_storage_m3_delta']:.12g} |
| aggregate tail-fold delta m3 | {trace['aggregate']['tail_fold_m3_delta']:.12g} |
| candidate clamp m3 | {trace['aggregate']['clamp_m3_candidate']:.12g} |
| reference clamp m3 | {trace['aggregate']['clamp_m3_reference']:.12g} |
| max lane routed-shape L1 | {trace['aggregate']['max_shape_l1']:.12g} |
| terminal outlet delta m3 | {terminal['terminal_day_outlet_m3_delta']:.12g} |
| terminal routed-shape L1 | {terminal['terminal_shape_l1']:.12g} |

## Classification

`{result['classification']['mechanism']}`.

{result['classification']['rationale']}

## Follow-On

{result['classification']['promotion_disposition']}

Next package: `20260708-laned-router-annual-sediment-adequacy-metric-authority-001`.

## Provenance

- Prior summary: `{rel(SOURCE_ARTIFACTS / 'coupled-spacetime-summary.json')}`
- Prior mesh ratification: `{rel(SOURCE_ARTIFACTS / 'mesh-policy-ratification.json')}`
- Release binary SHA-256:
  `{result['provenance']['release_binary'].get('sha256')}`
"""
    (ARTIFACTS / "wa-sediment-attribution.md").write_text(md)


def main() -> None:
    summary_path = SOURCE_ARTIFACTS / "coupled-spacetime-summary.json"
    mesh_path = SOURCE_ARTIFACTS / "mesh-policy-ratification.json"
    summary = read_json(summary_path)
    mesh = read_json(mesh_path)
    prior = find_prior_comparison(summary)

    candidate_output = RUN_ROOT / CANDIDATE_RUNG / "run_dir/output"
    reference_output = RUN_ROOT / REFERENCE_RUNG / "run_dir/output"
    candidate_pass = candidate_output / "H1.pass.parquet"
    reference_pass = reference_output / "H1.pass.parquet"
    candidate_trace_path = candidate_output / "laned_active_trace.jsonl"
    reference_trace_path = reference_output / "laned_active_trace.jsonl"

    candidate_rows = read_pass(candidate_pass)
    reference_rows = read_pass(reference_pass)
    candidate_trace = read_trace(candidate_trace_path)
    reference_trace = read_trace(reference_trace_path)

    candidate_annual = annual_sums(candidate_rows, SEDIMENT_COLUMN)
    reference_annual = annual_sums(reference_rows, SEDIMENT_COLUMN)
    target_rel = relative_delta(
        candidate_annual[TARGET_YEAR], reference_annual[TARGET_YEAR]
    )
    daily_rows = pass_day_by_day(candidate_rows, reference_rows)
    target_daily = [row for row in daily_rows if row["year"] == TARGET_YEAR]
    target_nonzero = [row for row in target_daily if row["abs_tdep_delta_kg"] > 0.0]
    target_sorted = sorted(target_daily, key=lambda row: row["abs_tdep_delta_kg"], reverse=True)
    top_rows = []
    for row in target_sorted[:10]:
        day = row["sim_day_index"]
        row_with_trace = dict(row)
        row_with_trace["trace"] = trace_day_summary(candidate_trace[day], reference_trace[day])
        top_rows.append(row_with_trace)

    all_year_sediment_rels = {
        year: relative_delta(candidate_annual.get(year, 0.0), reference_annual.get(year, 0.0))
        for year in sorted(set(candidate_annual) | set(reference_annual))
    }
    target_abs_delta = candidate_annual[TARGET_YEAR] - reference_annual[TARGET_YEAR]
    target_abs_threshold = ADEQUACY_THRESHOLD * abs(reference_annual[TARGET_YEAR])
    top = top_rows[0]
    terminal = top["trace"]["aggregate"]["terminal"]

    prior_water_surfaces_pass = (
        bool(prior["terminal_outlet_passes_1pct"])
        and bool(prior["shape_passes_0p05"])
        and bool(prior["end_storage_passes_1pct_source"])
        and bool(prior["tail_fold_passes_1pct_source"])
        and bool(prior["uniform_shape_passes_no_increase"])
        and bool(prior["degenerate_shape_passes_no_increase"])
    )
    implicated_day_clean = (
        top["runvol_delta_m3"] == 0.0
        and top["sbrunv_delta_m3"] == 0.0
        and top["peakro_delta_m3_s"] == 0.0
        and top["trace"]["aggregate"]["source_m3_delta"] == 0.0
        and top["trace"]["aggregate"]["tail_fold_m3_delta"] == 0.0
        and top["trace"]["aggregate"]["clamp_m3_candidate"] == 0.0
        and top["trace"]["aggregate"]["clamp_m3_reference"] == 0.0
    )
    if prior_water_surfaces_pass and implicated_day_clean:
        mechanism = "sediment response to sub-threshold routed-hydrograph shape perturbation"
        rationale = (
            "The failing annual sediment value is produced by a single low-mass "
            "erosion day. Pass-parquet daily water magnitude operands and active "
            "source mass are identical on that day. The routed hydrograph shape "
            "does differ, and that difference is a consumed water-timing input to "
            "the erosion path, but the prior rev-43 mesh-policy routed-water "
            "surfaces all passed: terminal outlet, routed shape, end-window "
            "storage, tail-fold, uniform-shape, and source-shape-degenerate "
            "counters. The package therefore classifies the blocker as a "
            "low-denominator annual sediment response to a sub-threshold routed "
            "hydrograph timing/shape perturbation, not as active-router numerics "
            "or daily water-magnitude drift."
        )
    else:
        mechanism = "needs-review"
        rationale = (
            "The discriminators did not match the package's erosion-consumer "
            "sensitivity rule. Inspect the JSON daily trace comparison before "
            "closing classification."
        )

    result = {
        "created_utc": datetime.now(timezone.utc).isoformat(),
        "member_id": MEMBER_ID,
        "role": ROLE,
        "candidate_rung": CANDIDATE_RUNG,
        "reference_rung": REFERENCE_RUNG,
        "surface_confirmation": {
            "prior_summary_max_surface": prior["annual_pass_sediment"]["max_surface"],
            "prior_summary_max_rel": prior["annual_pass_sediment"]["max_rel"],
            "prior_summary_passes_2pct": prior["annual_pass_sediment"]["passes_2pct"],
            "prior_gate_verdict": next(
                row["verdict"]
                for row in mesh["blocking_roles"][ROLE]
                if row["member_id"] == MEMBER_ID
            ),
            "recomputed_target_year_candidate_kg": candidate_annual[TARGET_YEAR],
            "recomputed_target_year_reference_kg": reference_annual[TARGET_YEAR],
            "recomputed_target_year_delta_kg": target_abs_delta,
            "recomputed_target_year_abs_delta_kg": abs(target_abs_delta),
            "recomputed_target_year_rel": target_rel,
            "one_third_adequacy_threshold_rel": ADEQUACY_THRESHOLD,
            "one_third_adequacy_threshold_abs_kg": target_abs_threshold,
            "excess_over_abs_threshold_kg": abs(target_abs_delta) - target_abs_threshold,
            "all_year_tdep_candidate_kg": candidate_annual,
            "all_year_tdep_reference_kg": reference_annual,
            "all_year_tdep_rel_delta": all_year_sediment_rels,
            "prior_comparison": prior,
        },
        "daily_attribution": {
            "target_year_days": len(target_daily),
            "target_year_nonzero_tdep_delta_days": len(target_nonzero),
            "target_year_top_abs_tdep_deltas": top_rows,
            "target_year_abs_tdep_delta_sum_kg": sum(
                row["abs_tdep_delta_kg"] for row in target_daily
            ),
            "target_year_signed_tdep_delta_sum_kg": sum(
                row["tdep_delta_kg"] for row in target_daily
            ),
        },
        "trace_run_summaries": {
            "candidate": run_trace_summary(candidate_trace),
            "reference": run_trace_summary(reference_trace),
        },
        "classification": {
            "mechanism": mechanism,
            "rationale": rationale,
            "promotion_disposition": (
                "dx5 promotion remains blocked until a contract-authorized "
                "annual pass-sediment adequacy metric policy is adjudicated."
            ),
        },
        "provenance": {
            "release_binary": summary["release_binary"],
            "input_paths": {
                "candidate_pass_parquet": rel(candidate_pass),
                "reference_pass_parquet": rel(reference_pass),
                "candidate_trace": rel(candidate_trace_path),
                "reference_trace": rel(reference_trace_path),
                "candidate_manifest": rel(candidate_output / "openwepp_hillslope_run_manifest.json"),
                "reference_manifest": rel(reference_output / "openwepp_hillslope_run_manifest.json"),
                "summary_json": rel(summary_path),
                "mesh_policy_json": rel(mesh_path),
            },
            "sha256": {
                "candidate_pass_parquet": sha256(candidate_pass),
                "reference_pass_parquet": sha256(reference_pass),
                "candidate_trace": sha256(candidate_trace_path),
                "reference_trace": sha256(reference_trace_path),
                "candidate_manifest": sha256(candidate_output / "openwepp_hillslope_run_manifest.json"),
                "reference_manifest": sha256(reference_output / "openwepp_hillslope_run_manifest.json"),
                "summary_json": sha256(summary_path),
                "mesh_policy_json": sha256(mesh_path),
            },
        },
    }
    output_path = ARTIFACTS / "wa-sediment-attribution.json"
    output_path.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    write_markdown(result)
    print(rel(output_path))


if __name__ == "__main__":
    main()
