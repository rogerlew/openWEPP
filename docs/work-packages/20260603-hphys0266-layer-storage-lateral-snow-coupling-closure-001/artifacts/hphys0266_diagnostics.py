#!/usr/bin/env python3
"""Run HPHYS0266 layer/lateral/snow coupling diagnostics and full metrics."""

from __future__ import annotations

import argparse
import importlib.util
import json
import math
import sys
from pathlib import Path
from typing import Any


REPO = Path("/home/workdir/openWEPP")
HPHYS0265_SCRIPT = (
    REPO
    / "docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/hphys0265_diagnostics.py"
)
TARGETED_HILLSLOPES = [1, 7, 39]
SELECTED_SYMBOLS = [
    "Ep",
    "Total-Soil",
    "SoilWaterTotal",
    "Dp",
    "latqcc",
    "Q",
    "RM",
    "Snow-Water",
]
FIRST_EP_THRESHOLD_MM = 0.05
LARGE_EP_THRESHOLD_MM = 1.0
IDENTITY_TOLERANCE_MM = 1.0e-6
IDENTITY_TOLERANCE_M = 1.0e-9
LAYER_ACTIVITY_TOLERANCE = 1.0e-12
STRESS_TOLERANCE = 1.0e-6


def load_hphys0265_module() -> Any:
    spec = importlib.util.spec_from_file_location("hphys0265_diagnostics", HPHYS0265_SCRIPT)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {HPHYS0265_SCRIPT}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


HPHYS0265 = load_hphys0265_module()


def markdown_table(headers: list[str], rows: list[list[Any]]) -> str:
    return HPHYS0265.markdown_table(headers, rows)


def trace_float(row: dict[str, Any] | None, name: str, scale: float = 1.0) -> float | None:
    return HPHYS0265.trace_float(row, name, scale)


def trace_string(row: dict[str, Any] | None, name: str) -> str | None:
    return HPHYS0265.trace_string(row, name)


def trace_layers_sum_mm(row: dict[str, Any] | None, name: str) -> float | None:
    return HPHYS0265.trace_layers_sum_mm(row, name)


def trace_layers_min(row: dict[str, Any] | None, name: str) -> float | None:
    return HPHYS0265.trace_layers_min(row, name)


def layer_map(row: dict[str, Any] | None, name: str) -> dict[str, float]:
    if row is None:
        return {}
    value = row.get(name)
    if not isinstance(value, dict):
        return {}
    return {str(layer_id): float(layer_value) for layer_id, layer_value in value.items()}


def active_layers(row: dict[str, Any] | None, name: str, threshold: float) -> list[int]:
    layers = layer_map(row, name)
    return sorted(int(layer_id) for layer_id, value in layers.items() if value > threshold)


def stressed_layers(row: dict[str, Any] | None) -> list[int]:
    layers = layer_map(row, "wb17_swu_storage_to_threshold_layers")
    return sorted(
        int(layer_id)
        for layer_id, ratio in layers.items()
        if ratio < 1.0 - STRESS_TOLERANCE
    )


def layer_preview(layer_ids: list[int]) -> str:
    if not layer_ids:
        return "none"
    return ",".join(f"{layer_id:04d}" for layer_id in layer_ids)


def trace_layers_preview(row: dict[str, Any] | None, name: str, limit: int = 9) -> str:
    return HPHYS0265.trace_layers_preview(row, name, limit)


def zone_split_summary(identity_row: dict[str, Any] | None, lateral_row: dict[str, Any] | None) -> dict[str, Any]:
    stress_layer_ids = stressed_layers(identity_row)
    capacity_layer_ids = active_layers(
        lateral_row, "wb19_lateral_capacity_active_count_layers", LAYER_ACTIVITY_TOLERANCE
    )
    conductivity_layer_ids = active_layers(
        lateral_row, "wb19_lateral_conductivity_active_count_layers", LAYER_ACTIVITY_TOLERANCE
    )
    withdrawal_layer_ids = active_layers(
        lateral_row, "wb19_lateral_withdrawal_layers_m", LAYER_ACTIVITY_TOLERANCE
    )
    lateral_layer_ids = sorted(
        set(capacity_layer_ids) | set(conductivity_layer_ids) | set(withdrawal_layer_ids)
    )
    stress_set = set(stress_layer_ids)
    lateral_set = set(lateral_layer_ids)
    overlap_layer_ids = sorted(stress_set & lateral_set)
    bottom_zone_separated = bool(
        stress_layer_ids
        and lateral_layer_ids
        and max(stress_layer_ids) < min(lateral_layer_ids)
    )
    return {
        "stress_layer_ids": stress_layer_ids,
        "capacity_layer_ids": capacity_layer_ids,
        "conductivity_layer_ids": conductivity_layer_ids,
        "withdrawal_layer_ids": withdrawal_layer_ids,
        "lateral_layer_ids": lateral_layer_ids,
        "overlap_layer_ids": overlap_layer_ids,
        "stress_lateral_overlap_count": len(overlap_layer_ids),
        "bottom_zone_separated": bottom_zone_separated,
    }


def material_context_symbols(row: Any) -> list[str]:
    return HPHYS0265.material_context_symbols(row)


def wat_candidate(row: Any, symbol: str) -> float:
    return HPHYS0265.wat_candidate(row, symbol)


def wat_baseline(row: Any, symbol: str) -> float:
    return HPHYS0265.wat_baseline(row, symbol)


def wat_delta(row: Any, symbol: str) -> float:
    return HPHYS0265.wat_delta(row, symbol)


def find_trace_row(
    rows: list[dict[str, Any]],
    sim_day_index: int,
    boundary: str,
    phase: str | None = None,
) -> dict[str, Any] | None:
    return HPHYS0265.find_trace_row(rows, sim_day_index, boundary, phase)


def identity_close(left: float | None, right: float | None, tolerance: float) -> bool:
    return left is not None and right is not None and abs(left - right) <= tolerance


def classify_first_divergence(
    hillslope_id: int,
    merged: Any,
    trace_rows: list[dict[str, Any]],
) -> dict[str, Any]:
    first = HPHYS0265.first_crossing(merged, FIRST_EP_THRESHOLD_MM)
    first_large = HPHYS0265.first_crossing(merged, LARGE_EP_THRESHOLD_MM)
    max_row = HPHYS0265.max_crossing(merged)
    if first is None:
        return {
            "hillslope_id": hillslope_id,
            "classification": "NO_EP_THRESHOLD_CROSSING",
            "first_ep_threshold_mm": FIRST_EP_THRESHOLD_MM,
            "max_abs_ep_diff_mm": HPHYS0265.row_float(max_row, "abs_ep_diff_mm"),
        }

    sim_day_index = int(first["sim_day_index_candidate"])
    trace_by_key = {
        "post_seed": find_trace_row(trace_rows, sim_day_index, "post_seed"),
        "evapotranspiration": find_trace_row(
            trace_rows, sim_day_index, "post_phase", "evapotranspiration"
        ),
        "plant_root_uptake": find_trace_row(
            trace_rows, sim_day_index, "post_phase", "plant_root_uptake"
        ),
        "percolation_deep_seepage": find_trace_row(
            trace_rows, sim_day_index, "post_phase", "percolation_deep_seepage"
        ),
        "lateral_transfer": find_trace_row(
            trace_rows, sim_day_index, "post_phase", "lateral_transfer"
        ),
        "runoff_reconciliation": find_trace_row(
            trace_rows, sim_day_index, "post_phase", "runoff_reconciliation"
        ),
        "storage_reconciliation": find_trace_row(
            trace_rows, sim_day_index, "post_phase", "storage_reconciliation"
        ),
        "post_scheduler": find_trace_row(trace_rows, sim_day_index, "post_scheduler"),
        "post_wb13": find_trace_row(trace_rows, sim_day_index, "post_wb13"),
    }
    identity_row = (
        trace_by_key["plant_root_uptake"]
        or trace_by_key["post_scheduler"]
        or trace_by_key["post_wb13"]
    )
    et_row = trace_by_key["evapotranspiration"] or identity_row
    lateral_row = trace_by_key["lateral_transfer"]
    storage_row = trace_by_key["storage_reconciliation"] or trace_by_key["post_wb13"] or identity_row

    pmet_ep_mm = trace_float(et_row, "pmet_ep_m", 1000.0)
    etp_mm = trace_float(et_row, "etp_m", 1000.0)
    final_ep_mm = trace_float(identity_row, "ep_m", 1000.0)
    ui_mm = trace_float(identity_row, "ui_m", 1000.0)
    ui_sum_mm = trace_layers_sum_mm(identity_row, "wb17_ui_layers_m")
    ws = trace_float(identity_row, "ws")
    ep_minus_ui_sum_mm = (
        final_ep_mm - ui_sum_mm if final_ep_mm is not None and ui_sum_mm is not None else None
    )
    ui_minus_sum_mm = ui_mm - ui_sum_mm if ui_mm is not None and ui_sum_mm is not None else None
    ws_minus_ep_over_etp = None
    if ws is not None and final_ep_mm is not None and etp_mm is not None and etp_mm > 0.0:
        ws_minus_ep_over_etp = ws - (final_ep_mm / etp_mm)

    wb11_soil_water_mm = trace_float(storage_row, "wb11_soil_water_m", 1000.0)
    wb18_theta_sum_mm = trace_float(storage_row, "wb18_theta_sum_m", 1000.0)
    wb18_recomputed_minus_wb11_mm = trace_float(
        storage_row, "wb18_recomputed_minus_wb11_m", 1000.0
    )

    q_potential_mm = trace_float(lateral_row, "wb19_q_lateral_potential_m", 1000.0)
    q_target_mm = trace_float(lateral_row, "wb19_q_lateral_target_m", 1000.0)
    q_unrealized_mm = trace_float(lateral_row, "wb19_q_lateral_unrealized_m", 1000.0)
    q_realized_mm = trace_float(lateral_row, "q_m", 1000.0)
    qdd_mm = trace_float(lateral_row, "qdd_m", 1000.0)
    qd_mm = trace_float(lateral_row, "qd_m", 1000.0)
    withdrawal_sum_mm = trace_layers_sum_mm(lateral_row, "wb19_lateral_withdrawal_layers_m")
    target_minus_q_mm = (
        q_target_mm - q_realized_mm
        if q_target_mm is not None and q_realized_mm is not None
        else None
    )
    potential_minus_target_mm = (
        q_potential_mm - q_target_mm
        if q_potential_mm is not None and q_target_mm is not None
        else None
    )
    q_minus_withdrawal_sum_mm = (
        q_realized_mm - withdrawal_sum_mm
        if q_realized_mm is not None and withdrawal_sum_mm is not None
        else None
    )
    qd_minus_q_qdd_mm = (
        qd_mm - q_realized_mm - qdd_mm
        if qd_mm is not None and q_realized_mm is not None and qdd_mm is not None
        else None
    )

    missing_trace_keys = [key for key, row in trace_by_key.items() if row is None]
    wb17_identity_closed = (
        not missing_trace_keys
        and identity_close(pmet_ep_mm, etp_mm, IDENTITY_TOLERANCE_MM)
        and ep_minus_ui_sum_mm is not None
        and abs(ep_minus_ui_sum_mm) <= IDENTITY_TOLERANCE_MM
        and ui_minus_sum_mm is not None
        and abs(ui_minus_sum_mm) <= IDENTITY_TOLERANCE_MM
        and ws_minus_ep_over_etp is not None
        and abs(ws_minus_ep_over_etp) <= STRESS_TOLERANCE
    )
    wb11_wb18_aggregate_closed = (
        wb18_recomputed_minus_wb11_mm is not None
        and abs(wb18_recomputed_minus_wb11_mm) <= IDENTITY_TOLERANCE_MM
    )
    lateral_realized_identity_closed = (
        not missing_trace_keys
        and target_minus_q_mm is not None
        and abs(target_minus_q_mm) <= IDENTITY_TOLERANCE_MM
        and q_minus_withdrawal_sum_mm is not None
        and abs(q_minus_withdrawal_sum_mm) <= IDENTITY_TOLERANCE_MM
        and qd_minus_q_qdd_mm is not None
        and abs(qd_minus_q_qdd_mm) <= IDENTITY_TOLERANCE_MM
        and q_unrealized_mm is not None
        and abs(q_unrealized_mm) <= IDENTITY_TOLERANCE_MM
    )
    potential_target_q_closed = (
        potential_minus_target_mm is not None
        and abs(potential_minus_target_mm) <= IDENTITY_TOLERANCE_MM
        and target_minus_q_mm is not None
        and abs(target_minus_q_mm) <= IDENTITY_TOLERANCE_MM
    )

    zone_split = zone_split_summary(identity_row, lateral_row)
    material_symbols = material_context_symbols(first)
    snow_runoff_context_present = any(
        symbol in material_symbols for symbol in ["Q", "RM", "Snow-Water"]
    )
    storage_context_present = any(
        symbol in material_symbols for symbol in ["Total-Soil", "SoilWaterTotal"]
    )
    lateral_context_present = "latqcc" in material_symbols
    swu_stress_limited = (
        etp_mm is not None
        and final_ep_mm is not None
        and final_ep_mm < etp_mm - IDENTITY_TOLERANCE_MM
    )

    if missing_trace_keys:
        classification = "FIRST_DIVERGENCE_TRACE_INCOMPLETE"
    elif not wb17_identity_closed:
        classification = "WB17_INTERNAL_IDENTITY_DIVERGENCE"
    elif not wb11_wb18_aggregate_closed:
        classification = "WB11_WB18_AGGREGATE_RECOMPUTE_DIVERGENCE"
    elif not lateral_realized_identity_closed:
        classification = "WB19_LATERAL_REALIZED_IDENTITY_DIVERGENCE"
    elif (
        swu_stress_limited
        and zone_split["bottom_zone_separated"]
        and material_symbols
    ):
        classification = "WB17_WB19_IDENTITIES_CLOSED_LAYER_DISTRIBUTION_CONTEXT"
    elif material_symbols:
        classification = "WB17_WB19_IDENTITIES_CLOSED_COUPLED_CONTEXT"
    else:
        classification = "WB17_WB19_IDENTITIES_CLOSED_UPSTREAM_CONTEXT"

    return {
        "hillslope_id": hillslope_id,
        "classification": classification,
        "first_ep_threshold_mm": FIRST_EP_THRESHOLD_MM,
        "first_large_ep_threshold_mm": LARGE_EP_THRESHOLD_MM,
        "comparison_year": int(first["_comparison_year"]),
        "julian": int(first["julian"]),
        "candidate_sim_day_index": sim_day_index,
        "baseline_sim_day_index": int(first["sim_day_index_baseline"]),
        "candidate_ep_mm": HPHYS0265.row_float(first, "Ep_candidate"),
        "baseline_ep_mm": HPHYS0265.row_float(first, "Ep_baseline"),
        "ep_diff_mm": HPHYS0265.row_float(first, "ep_diff_mm"),
        "abs_ep_diff_mm": HPHYS0265.row_float(first, "abs_ep_diff_mm"),
        "first_large_comparison_year": None if first_large is None else int(first_large["_comparison_year"]),
        "first_large_julian": None if first_large is None else int(first_large["julian"]),
        "first_large_abs_ep_diff_mm": None if first_large is None else HPHYS0265.row_float(first_large, "abs_ep_diff_mm"),
        "max_comparison_year": int(max_row["_comparison_year"]),
        "max_julian": int(max_row["julian"]),
        "max_abs_ep_diff_mm": HPHYS0265.row_float(max_row, "abs_ep_diff_mm"),
        "wb11_et_seed_branch": trace_string(et_row, "wb11_et_seed_branch"),
        "pmet_ep_mm": pmet_ep_mm,
        "etp_mm": etp_mm,
        "final_ep_trace_mm": final_ep_mm,
        "ui_aggregate_mm": ui_mm,
        "ui_layer_sum_mm": ui_sum_mm,
        "ep_minus_ui_sum_mm": ep_minus_ui_sum_mm,
        "ui_aggregate_minus_layer_sum_mm": ui_minus_sum_mm,
        "ws": ws,
        "ws_minus_ep_over_etp": ws_minus_ep_over_etp,
        "wb17_identity_closed": wb17_identity_closed,
        "swu_stress_limited": swu_stress_limited,
        "pl_lai": trace_float(identity_row, "pl_lai"),
        "pl_rtd": trace_float(identity_row, "pl_rtd"),
        "pl_pltol": trace_float(identity_row, "pl_pltol"),
        "pl_swu_effective_pltol": trace_float(identity_row, "pl_swu_effective_pltol"),
        "min_storage_to_threshold": trace_layers_min(
            identity_row, "wb17_swu_storage_to_threshold_layers"
        ),
        "wb11_soil_water_mm": wb11_soil_water_mm,
        "wb18_theta_sum_mm": wb18_theta_sum_mm,
        "wb18_recomputed_minus_wb11_mm": wb18_recomputed_minus_wb11_mm,
        "wb11_wb18_aggregate_closed": wb11_wb18_aggregate_closed,
        "q_potential_mm": q_potential_mm,
        "q_target_mm": q_target_mm,
        "q_realized_mm": q_realized_mm,
        "qdd_mm": qdd_mm,
        "qd_mm": qd_mm,
        "q_unrealized_mm": q_unrealized_mm,
        "withdrawal_sum_mm": withdrawal_sum_mm,
        "potential_minus_target_mm": potential_minus_target_mm,
        "target_minus_q_mm": target_minus_q_mm,
        "q_minus_withdrawal_sum_mm": q_minus_withdrawal_sum_mm,
        "qd_minus_q_qdd_mm": qd_minus_q_qdd_mm,
        "lateral_realized_identity_closed": lateral_realized_identity_closed,
        "potential_target_q_closed": potential_target_q_closed,
        "zone_split": zone_split,
        "stress_layers": layer_preview(zone_split["stress_layer_ids"]),
        "lateral_layers": layer_preview(zone_split["lateral_layer_ids"]),
        "withdrawal_layers": layer_preview(zone_split["withdrawal_layer_ids"]),
        "stress_lateral_overlap_count": zone_split["stress_lateral_overlap_count"],
        "bottom_zone_separated": zone_split["bottom_zone_separated"],
        "storage_to_threshold_preview": trace_layers_preview(
            identity_row, "wb17_swu_storage_to_threshold_layers"
        ),
        "withdrawal_preview": trace_layers_preview(
            lateral_row, "wb19_lateral_withdrawal_layers_m"
        ),
        "capacity_active_preview": trace_layers_preview(
            lateral_row, "wb19_lateral_capacity_active_count_layers"
        ),
        "wat_context": {
            symbol: {
                "candidate": wat_candidate(first, symbol),
                "baseline": wat_baseline(first, symbol),
                "diff": wat_delta(first, symbol),
            }
            for symbol in SELECTED_SYMBOLS
        },
        "material_context_symbols": material_symbols,
        "snow_runoff_context_present": snow_runoff_context_present,
        "storage_context_present": storage_context_present,
        "lateral_context_present": lateral_context_present,
        "trace_rows_missing": missing_trace_keys,
        "trace_day_row_count": sum(
            1 for row in trace_rows if int(row["sim_day_index"]) == sim_day_index
        ),
    }


def summarize_layer_lateral_diagnostics(run_root: Path) -> list[dict[str, Any]]:
    reports = run_root / "reports"
    classifications: list[dict[str, Any]] = []
    for hillslope_id in TARGETED_HILLSLOPES:
        candidate_wat = run_root / f"hillslope_output/H{hillslope_id}.wat.parquet"
        baseline_wat = HPHYS0265.BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet"
        trace_path = run_root / f"hillslope_output/H{hillslope_id}.hphys0266.trace.jsonl"
        merged = HPHYS0265.candidate_baseline_merge(
            candidate_wat, baseline_wat, candidate_year_offset=2012
        )
        trace_rows = HPHYS0265.load_trace_rows(trace_path)
        classifications.append(classify_first_divergence(hillslope_id, merged, trace_rows))

    json_path = reports / "hphys0266_layer_storage_lateral_classification.json"
    json_path.write_text(json.dumps(classifications, indent=2) + "\n", encoding="utf-8")

    summary_rows = []
    wb17_rows = []
    storage_rows = []
    lateral_rows = []
    zone_rows = []
    context_rows = []
    for item in classifications:
        summary_rows.append(
            [
                f"H{item['hillslope_id']}",
                item["classification"],
                item.get("comparison_year"),
                item.get("julian"),
                item.get("candidate_sim_day_index"),
                item.get("candidate_ep_mm"),
                item.get("baseline_ep_mm"),
                item.get("ep_diff_mm"),
                item.get("first_large_julian"),
                item.get("max_julian"),
                item.get("max_abs_ep_diff_mm"),
            ]
        )
        wb17_rows.append(
            [
                f"H{item['hillslope_id']}",
                item.get("wb11_et_seed_branch"),
                item.get("pmet_ep_mm"),
                item.get("etp_mm"),
                item.get("final_ep_trace_mm"),
                item.get("ui_layer_sum_mm"),
                item.get("ep_minus_ui_sum_mm"),
                item.get("ui_aggregate_minus_layer_sum_mm"),
                item.get("ws"),
                item.get("ws_minus_ep_over_etp"),
                item.get("min_storage_to_threshold"),
            ]
        )
        storage_rows.append(
            [
                f"H{item['hillslope_id']}",
                item.get("wb11_soil_water_mm"),
                item.get("wb18_theta_sum_mm"),
                item.get("wb18_recomputed_minus_wb11_mm"),
                item.get("wb11_wb18_aggregate_closed"),
            ]
        )
        lateral_rows.append(
            [
                f"H{item['hillslope_id']}",
                item.get("q_potential_mm"),
                item.get("q_target_mm"),
                item.get("q_realized_mm"),
                item.get("qdd_mm"),
                item.get("qd_mm"),
                item.get("q_unrealized_mm"),
                item.get("withdrawal_sum_mm"),
                item.get("target_minus_q_mm"),
                item.get("q_minus_withdrawal_sum_mm"),
                item.get("qd_minus_q_qdd_mm"),
                item.get("lateral_realized_identity_closed"),
            ]
        )
        zone_rows.append(
            [
                f"H{item['hillslope_id']}",
                item.get("stress_layers"),
                item.get("lateral_layers"),
                item.get("withdrawal_layers"),
                item.get("stress_lateral_overlap_count"),
                item.get("bottom_zone_separated"),
                item.get("snow_runoff_context_present"),
                item.get("storage_context_present"),
                item.get("lateral_context_present"),
            ]
        )
        wat_context = item.get("wat_context", {})
        for symbol in SELECTED_SYMBOLS:
            symbol_context = wat_context.get(symbol, {})
            context_rows.append(
                [
                    f"H{item['hillslope_id']}",
                    symbol,
                    symbol_context.get("candidate"),
                    symbol_context.get("baseline"),
                    symbol_context.get("diff"),
                ]
            )

    markdown = "# HPHYS0266 Layer Storage, Lateral, and Snow Coupling Classification\n\n"
    markdown += "Ran:\n\n"
    markdown += f"- Root: `{run_root}`\n"
    markdown += f"- Threshold: first `|candidate Ep - baseline Ep| > {FIRST_EP_THRESHOLD_MM} mm`.\n"
    markdown += f"- Classification JSON: `{json_path}`.\n\n"
    markdown += "## First Divergence Summary\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Classification",
            "Year",
            "Julian",
            "Candidate Day",
            "Cand Ep",
            "Base Ep",
            "Ep Diff",
            "First >1mm Julian",
            "Max Julian",
            "Max Abs Ep Diff",
        ],
        summary_rows,
    )
    markdown += "\n## WB17 Identity Surfaces\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Seed Branch",
            "PMET Ep",
            "Etp",
            "Trace Ep",
            "ΣUi",
            "Ep-ΣUi",
            "Ui-ΣUi",
            "Ws",
            "Ws-Ep/Etp",
            "Min Storage/Threshold",
        ],
        wb17_rows,
    )
    markdown += "\n## WB11/WB18 Aggregate Closure\n\n"
    markdown += markdown_table(
        ["Hill", "WB11 Soil Water", "ΣTheta", "Recomputed-WB11", "Closed"],
        storage_rows,
    )
    markdown += "\n## WB19 Lateral Identity Surfaces\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Potential q",
            "Target q",
            "Realized q",
            "Qdd",
            "Qd",
            "Unrealized",
            "ΣWithdrawal",
            "Target-q",
            "q-ΣWithdrawal",
            "Qd-q-Qdd",
            "Closed",
        ],
        lateral_rows,
    )
    markdown += "\n## Layer-Zone Split\n\n"
    markdown += markdown_table(
        [
            "Hill",
            "Stress Layers",
            "Lateral Layers",
            "Withdrawal Layers",
            "Overlap Count",
            "Bottom Separated",
            "Snow/Runoff Ctx",
            "Storage Ctx",
            "Lateral Ctx",
        ],
        zone_rows,
    )
    markdown += "\n## Same-Day WAT Context\n\n"
    markdown += markdown_table(["Hill", "Symbol", "Candidate", "Baseline", "Diff"], context_rows)
    markdown += "\n## Per-Hill Layer Previews\n\n"
    for item in classifications:
        markdown += (
            f"- H{item['hillslope_id']} material context: "
            f"`{', '.join(item.get('material_context_symbols', [])) or 'none'}`; "
            f"storage/threshold layers: `{item.get('storage_to_threshold_preview', '')}`; "
            f"capacity-active layers: `{item.get('capacity_active_preview', '')}`; "
            f"withdrawals: `{item.get('withdrawal_preview', '')}`.\n"
        )
    (reports / "hphys0266_layer_storage_lateral_classification.md").write_text(
        markdown, encoding="utf-8"
    )
    return classifications


def run_semantics(run_root: Path) -> None:
    reports = run_root / "reports"
    semantic_dir = reports / "semantic_reports"
    semantic_dir.mkdir(parents=True, exist_ok=True)
    status_rows = []
    summary: dict[str, dict[str, Any]] = {}
    for hillslope_id in range(1, 40):
        report_json = semantic_dir / f"H{hillslope_id}.semantic.json"
        command = [
            str(HPHYS0265.WEPPPY_PYTHON),
            str(HPHYS0265.COMPARATOR),
            "--baseline-wat",
            str(HPHYS0265.BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet"),
            "--candidate-wat",
            str(run_root / f"hillslope_output/H{hillslope_id}.wat.parquet"),
            "--report-json",
            str(report_json),
            "--candidate-year-offset",
            "2012",
            "--tolerance-config",
            str(HPHYS0265.TOLERANCES),
        ]
        result = HPHYS0265.run_command(
            f"semantic_H{hillslope_id}", command, run_root / "logs/semantic"
        )
        semantic_pass = False
        common_rows = None
        if report_json.exists():
            data = json.loads(report_json.read_text(encoding="utf-8"))
            comparison = data["comparison"]
            semantic_pass = bool(comparison["semantic_pass"])
            common_rows = int(comparison["common_row_count"])
            for stat in comparison["column_stats"]:
                column = stat["column"]
                entry = summary.setdefault(
                    column,
                    {
                        "hillslope_fail_count": 0,
                        "total_fail_count": 0,
                        "mean_abs_diff_values": [],
                        "max_abs_diff": 0.0,
                    },
                )
                if not stat["pass"]:
                    entry["hillslope_fail_count"] += 1
                entry["total_fail_count"] += int(stat["fail_count"])
                entry["mean_abs_diff_values"].append(float(stat["mean_abs_diff"]))
                entry["max_abs_diff"] = max(
                    entry["max_abs_diff"], float(stat["max_abs_diff"])
                )
        status_rows.append(
            {
                "hillslope_id": hillslope_id,
                "rc": result.rc,
                "semantic_pass": semantic_pass,
                "common_rows": common_rows,
                "report_json": report_json,
            }
        )
    HPHYS0265.write_status(reports / "semantic_status.tsv", status_rows)

    summary_rows = []
    for column, entry in sorted(summary.items()):
        values = entry["mean_abs_diff_values"]
        summary_rows.append(
            {
                "column": column,
                "hillslope_fail_count": entry["hillslope_fail_count"],
                "total_fail_count": entry["total_fail_count"],
                "mean_abs_diff_mean": sum(values) / len(values) if values else 0.0,
                "max_abs_diff": entry["max_abs_diff"],
            }
        )
    (reports / "hillslope_semantic_summary.json").write_text(
        json.dumps(summary_rows, indent=2) + "\n", encoding="utf-8"
    )

    by_column = {row["column"]: row for row in summary_rows}
    selected_rows = []
    for symbol in SELECTED_SYMBOLS:
        row = by_column[symbol]
        selected_rows.append(
            [
                symbol,
                f"{39 - row['hillslope_fail_count']}/39",
                row["total_fail_count"],
                row["mean_abs_diff_mean"],
                row["max_abs_diff"],
            ]
        )
    markdown = "# HPHYS0266 Full 39 Semantic Summary\n\n"
    markdown += "Ran:\n\n"
    markdown += f"- Root: `{run_root}`\n"
    markdown += f"- Runtime status: `{reports / 'hillslope_batch_status.tsv'}`\n"
    markdown += f"- Semantic status: `{reports / 'semantic_status.tsv'}`\n"
    markdown += f"- Semantic pass: `{sum(1 for row in status_rows if row['semantic_pass'])}/39`\n\n"
    markdown += markdown_table(
        ["Symbol", "Pass Hillslopes", "Total Fail Count", "Mean Abs Diff Mean", "Max Abs Diff"],
        selected_rows,
    )
    (reports / "hillslope_semantic_summary.md").write_text(markdown, encoding="utf-8")


def run_targeted_traces(
    run_root: Path,
    runs_dir: Path,
    output: Path,
    logs: Path,
    trace_max_days: int,
) -> int:
    reports = run_root / "reports"
    trace_rows = []
    for hillslope_id in TARGETED_HILLSLOPES:
        trace_path = output / f"H{hillslope_id}.hphys0266.trace.jsonl"
        result = HPHYS0265.run_command(
            f"H{hillslope_id}_trace",
            [
                str(HPHYS0265.HILL_BIN),
                "--run-dir",
                str(runs_dir),
                "--run-file",
                f"p{hillslope_id}_openwepp.run",
                "--output-dir",
                str(output),
                "--policy",
                "compat",
            ],
            logs / "targeted",
            env={
                "OPENWEPP_HPHYS0245_TRACE_PATH": str(trace_path),
                "OPENWEPP_HPHYS0245_TRACE_MAX_DAYS": str(trace_max_days),
            },
        )
        trace_rows.append(
            {
                "hillslope_id": hillslope_id,
                "rc": result.rc,
                "seconds": f"{result.seconds:.3f}",
                "trace_path": trace_path,
                "stdout": result.stdout,
                "stderr": result.stderr,
            }
        )
        if result.rc != 0:
            HPHYS0265.write_status(reports / "targeted_trace_status.tsv", trace_rows)
            return int(result.rc)
    HPHYS0265.write_status(reports / "targeted_trace_status.tsv", trace_rows)
    summarize_layer_lateral_diagnostics(run_root)
    return 0


def run_full_hillslope_suite(run_root: Path, runs_dir: Path, output: Path, logs: Path) -> int:
    reports = run_root / "reports"
    batch_rows = []
    for hillslope_id in range(1, 40):
        result = HPHYS0265.run_command(
            f"H{hillslope_id}",
            [
                str(HPHYS0265.HILL_BIN),
                "--run-dir",
                str(runs_dir),
                "--run-file",
                f"p{hillslope_id}_openwepp.run",
                "--output-dir",
                str(output),
                "--policy",
                "compat",
            ],
            logs / "hillslopes",
        )
        batch_rows.append(
            {
                "hillslope_id": hillslope_id,
                "rc": result.rc,
                "seconds": f"{result.seconds:.3f}",
                "stdout": result.stdout,
                "stderr": result.stderr,
            }
        )
    HPHYS0265.write_status(reports / "hillslope_batch_status.tsv", batch_rows)
    failed = [row for row in batch_rows if row["rc"] != 0]
    if failed:
        return int(failed[0]["rc"])
    run_semantics(run_root)
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-root", required=True, type=Path)
    parser.add_argument("--trace-max-days", type=int, default=130)
    parser.add_argument("--skip-full-suite", action="store_true")
    args = parser.parse_args()

    run_root = args.run_root
    reports = run_root / "reports"
    logs = run_root / "logs"
    output = run_root / "hillslope_output"
    reports.mkdir(parents=True, exist_ok=True)
    logs.mkdir(parents=True, exist_ok=True)
    output.mkdir(parents=True, exist_ok=True)
    runs_dir = HPHYS0265.copy_runfiles(run_root)

    for required in [
        HPHYS0265.WEPPPY_PYTHON,
        HPHYS0265.COMPARATOR,
        HPHYS0265.TOLERANCES,
        HPHYS0265.BASELINE_PARTITIONS,
    ]:
        HPHYS0265.require_path(required)

    build = HPHYS0265.run_command(
        "cargo_build_openwepp_cli_hill",
        ["cargo", "build", "-p", "openwepp-runner", "--bin", "openwepp-cli-hill"],
        logs,
    )
    HPHYS0265.write_status(
        reports / "build_status.tsv",
        [
            {
                "command": "cargo build -p openwepp-runner --bin openwepp-cli-hill",
                "rc": build.rc,
                "seconds": f"{build.seconds:.3f}",
                "stdout": build.stdout,
                "stderr": build.stderr,
            }
        ],
    )
    if build.rc != 0:
        return int(build.rc)

    targeted_rc = run_targeted_traces(run_root, runs_dir, output, logs, args.trace_max_days)
    if targeted_rc != 0:
        return int(targeted_rc)
    if args.skip_full_suite:
        return 0
    return run_full_hillslope_suite(run_root, runs_dir, output, logs)


if __name__ == "__main__":
    raise SystemExit(main())
