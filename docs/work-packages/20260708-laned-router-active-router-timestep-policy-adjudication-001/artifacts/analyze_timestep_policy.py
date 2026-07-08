#!/usr/bin/env python3
"""Analyze mn_corn_h4 day-792 active-router timestep-policy traces."""

from __future__ import annotations

import json
import hashlib
from itertools import accumulate
from pathlib import Path
from typing import Any

PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE_DIR / "artifacts"
SUMMARY_JSON = ARTIFACTS / "timestep-policy-summary.json"
EXTRACT_JSON = ARTIFACTS / "timestep-policy-analysis-inputs.json"
OUTPUT_JSON = ARTIFACTS / "timestep-policy-adjudication.json"
OUTPUT_MD = ARTIFACTS / "timestep-policy-adjudication.md"

MEMBER_ID = "mn_corn_h4"
DAY_INDEX = 792
LANE_INDEX = 1
RUNGS = [
    "dx1p25_dt300",
    "dx1p25_dt150",
    "dx1p25_dt75",
    "dx0p625_dt300",
    "dx0p625_dt150",
    "dx0p625_dt75",
]
PAIRS = [
    ("dx1p25_timestep_300_vs_150", "dx1p25_dt300", "dx1p25_dt150", "same_dx_timestep"),
    ("dx1p25_timestep_150_vs_75", "dx1p25_dt150", "dx1p25_dt75", "same_dx_timestep"),
    ("dx0p625_timestep_300_vs_150", "dx0p625_dt300", "dx0p625_dt150", "same_dx_timestep"),
    ("dx0p625_timestep_150_vs_75", "dx0p625_dt150", "dx0p625_dt75", "same_dx_timestep"),
    ("spatial_dx1p25_vs_dx0p625_dt300", "dx1p25_dt300", "dx0p625_dt300", "same_dt_spatial"),
    ("spatial_dx1p25_vs_dx0p625_dt150", "dx1p25_dt150", "dx0p625_dt150", "same_dt_spatial"),
    ("spatial_dx1p25_vs_dx0p625_dt75", "dx1p25_dt75", "dx0p625_dt75", "same_dt_spatial"),
]
SHAPE_THRESHOLD = 1.0 / 60.0


def fmt(value: float) -> str:
    return f"{value:.17g}"


def sha256(path: Path) -> str | None:
    if not path.is_file():
        return None
    digest = hashlib.sha256()
    with path.open("rb") as fp:
        for chunk in iter(lambda: fp.read(65536), b""):
            digest.update(chunk)
    return digest.hexdigest()


def l1(a: list[float], b: list[float]) -> float:
    if len(a) != len(b):
        raise SystemExit(f"L1 length mismatch: {len(a)} vs {len(b)}")
    return sum(abs(x - y) for x, y in zip(a, b))


def linf(a: list[float], b: list[float]) -> float:
    if len(a) != len(b):
        raise SystemExit(f"Linf length mismatch: {len(a)} vs {len(b)}")
    return max((abs(x - y) for x, y in zip(a, b)), default=0.0)


def cdf(values: list[float]) -> list[float]:
    return list(accumulate(values))


def top_diffs(a: list[float], b: list[float], count: int = 8) -> list[dict[str, float | int]]:
    if len(a) != len(b):
        raise SystemExit(f"top diff length mismatch: {len(a)} vs {len(b)}")
    diffs = [
        {"abs_delta": abs(x - y), "index": index, "signed_delta": x - y}
        for index, (x, y) in enumerate(zip(a, b))
    ]
    diffs.sort(key=lambda row: float(row["abs_delta"]), reverse=True)
    return diffs[:count]


def load_trace_row(trace_path: Path) -> dict[str, Any]:
    if not trace_path.is_file():
        raise SystemExit(
            f"{trace_path}: trace file missing; rerun "
            "run_timestep_policy_ladder.py before analyzer replay"
        )
    for line in trace_path.read_text().splitlines():
        if not line.strip():
            continue
        row = json.loads(line)
        if row.get("sim_day_index") == DAY_INDEX and row.get("lane_index") == LANE_INDEX:
            if not row.get("trace_detail"):
                raise SystemExit(f"{trace_path}: target row has no trace_detail payload")
            return row
    raise SystemExit(f"{trace_path}: target row not found")


def run_record(summary: dict[str, Any], rung: str) -> dict[str, Any]:
    for record in summary["runs"]:
        if record.get("member_id") == MEMBER_ID and record.get("rung") == rung:
            return record
    raise SystemExit(f"summary lacks {MEMBER_ID} {rung} run record")


def detail_for(summary: dict[str, Any], rung: str) -> dict[str, Any]:
    record = run_record(summary, rung)
    trace_path = Path(record["trace_path"])
    if not trace_path.is_file() and EXTRACT_JSON.is_file():
        extract = json.loads(EXTRACT_JSON.read_text())
        try:
            return extract["rungs"][rung]
        except KeyError as error:
            raise SystemExit(
                f"{trace_path}: trace missing and {EXTRACT_JSON} lacks {rung}"
            ) from error
    row = load_trace_row(trace_path)
    detail = row["trace_detail"]
    weights = [float(value) for value in row["routed_hourly_weights"]]
    outlet_m3 = float(row["terminal_day_outlet_m3"])
    step_trace = detail.get("step_trace")
    if step_trace is None:
        raise SystemExit(f"{trace_path}: target row has no step_trace payload")
    return {
        "trace_path": str(trace_path),
        "trace_sha256": record.get("trace_sha256"),
        "hbp_sha256": record.get("hbp_sha256"),
        "pass_parquet_sha256": record.get("pass_parquet_sha256"),
        "max_dt_s": float(row.get("max_dt_s", detail.get("max_dt_s", record["max_dt_s"]))),
        "mesh_cell_count": int(detail["mesh_cell_count"]),
        "mesh_dx_m": float(detail["mesh_dx_m"]),
        "source_m3": float(row["source_m3"]),
        "outlet_m3": outlet_m3,
        "mesh_end_storage_m3": float(row["mesh_end_storage_m3"]),
        "tail_fold_m3": float(row["tail_fold_m3"]),
        "weights": weights,
        "hour_masses_m3": [weight * outlet_m3 for weight in weights],
        "outlet_bin_m3": [float(value) for value in detail["outlet_bin_m3"]],
        "outlet_bin_spans_s": [float(value) for value in detail["outlet_bin_spans_s"]],
        "hydrograph_time_s": [float(value) for value in detail["hydrograph_time_s"]],
        "hydrograph_outlet_m3_s": [
            float(value) for value in detail["hydrograph_outlet_m3_s"]
        ],
        "hydrograph_outlet_depth_m": [
            float(value) for value in detail["hydrograph_outlet_depth_m"]
        ],
        "step_trace": step_trace,
    }


def write_analysis_inputs(
    summary: dict[str, Any],
    runs: dict[str, dict[str, Any]],
) -> None:
    extract = {
        "schema": "openwepp-mn-corn-h4-day792-timestep-policy-analysis-inputs-v1",
        "member_id": MEMBER_ID,
        "sim_day_index": DAY_INDEX,
        "lane_index": LANE_INDEX,
        "summary_path": str(SUMMARY_JSON),
        "summary_sha256": sha256(SUMMARY_JSON),
        "release_binary": summary["release_binary"],
        "rungs": {},
    }
    for rung in RUNGS:
        run = dict(runs[rung])
        if "trace_path" in run:
            run["raw_trace_path_recorded"] = run.pop("trace_path")
        extract["rungs"][rung] = run
    EXTRACT_JSON.write_text(json.dumps(extract, sort_keys=True, separators=(",", ":")) + "\n")


def bin_edges(spans: list[float]) -> list[tuple[float, float]]:
    edges = []
    t = 0.0
    for span in spans:
        edges.append((t, t + span))
        t += span
    return edges


def reconstruct_bins_from_steps(run: dict[str, Any]) -> list[float]:
    reconstructed = [0.0 for _ in run["outlet_bin_spans_s"]]
    edges = bin_edges(run["outlet_bin_spans_s"])
    for step in run["step_trace"]:
        start_s = float(step["t_start_s"])
        end_s = float(step["t_end_s"])
        duration_s = end_s - start_s
        if duration_s <= 0.0:
            raise SystemExit(f"nonpositive step duration in {run['trace_path']}")
        outflow_m3 = float(step["outflow_m3"])
        for index, (bin_start_s, bin_end_s) in enumerate(edges):
            overlap_s = max(0.0, min(end_s, bin_end_s) - max(start_s, bin_start_s))
            if overlap_s > 0.0:
                reconstructed[index] += outflow_m3 * overlap_s / duration_s
    return reconstructed


def step_summary(rung: str, run: dict[str, Any]) -> dict[str, Any]:
    steps = run["step_trace"]
    reconstructed_bins = reconstruct_bins_from_steps(run)
    bin_errors = [
        reconstructed - published
        for reconstructed, published in zip(reconstructed_bins, run["outlet_bin_m3"])
    ]
    max_courant_step = max(steps, key=lambda step: float(step["max_courant"]), default=None)
    pred_negative = [step for step in steps if float(step["pred_out_face_m3_s"]) < 0.0]
    corr_negative = [step for step in steps if float(step["corr_out_face_m3_s"]) < 0.0]
    out_negative = [step for step in steps if float(step["outflow_m3"]) < 0.0]
    tvd_limited = [
        step for step in steps if float(step["tvd"]["scale"]) < 1.0 - 1.0e-15
    ]
    pred_limited = [
        step for step in steps if int(step["predictor_limiter"]["reductions"]) > 0
    ]
    corr_limited = [
        step for step in steps if int(step["corrector_limiter"]["reductions"]) > 0
    ]

    def max_by(rows: list[dict[str, Any]], key_path: tuple[str, ...]) -> dict[str, Any] | None:
        if not rows:
            return None

        def value(row: dict[str, Any]) -> float:
            current: Any = row
            for key in key_path:
                current = current[key]
            return abs(float(current))

        return max(rows, key=value)

    return {
        "rung": rung,
        "step_count": len(steps),
        "mesh_cell_count": run["mesh_cell_count"],
        "mesh_dx_m": run["mesh_dx_m"],
        "source_total_m3": sum(float(step["source_m3"]) for step in steps),
        "upstream_inflow_total_m3": sum(float(step["upstream_inflow_m3"]) for step in steps),
        "outflow_total_m3": sum(float(step["outflow_m3"]) for step in steps),
        "clamp_total_m3": sum(float(step["clamp_injected_m3"]) for step in steps),
        "max_courant": max((float(step["max_courant"]) for step in steps), default=0.0),
        "max_courant_step": max_courant_step,
        "reconstructed_bin_l1_m3": sum(abs(value) for value in bin_errors),
        "reconstructed_bin_linf_m3": max((abs(value) for value in bin_errors), default=0.0),
        "reconstructed_bin_total_m3": sum(reconstructed_bins),
        "published_bin_total_m3": sum(run["outlet_bin_m3"]),
        "min_pred_out_face_m3_s": min(
            (float(step["pred_out_face_m3_s"]) for step in steps), default=0.0
        ),
        "min_corr_out_face_m3_s": min(
            (float(step["corr_out_face_m3_s"]) for step in steps), default=0.0
        ),
        "negative_pred_face_steps": len(pred_negative),
        "negative_corr_face_steps": len(corr_negative),
        "negative_step_outflow_steps": len(out_negative),
        "negative_step_outflow_m3": sum(
            float(step["outflow_m3"]) for step in out_negative
        ),
        "predictor_limiter_steps": len(pred_limited),
        "corrector_limiter_steps": len(corr_limited),
        "predictor_limiter_reductions": sum(
            int(step["predictor_limiter"]["reductions"]) for step in steps
        ),
        "corrector_limiter_reductions": sum(
            int(step["corrector_limiter"]["reductions"]) for step in steps
        ),
        "tvd_limited_steps": len(tvd_limited),
        "min_tvd_scale": min((float(step["tvd"]["scale"]) for step in steps), default=1.0),
        "max_abs_tvd_delta_m": max(
            (abs(float(step["tvd"]["max_abs_delta_m"])) for step in steps), default=0.0
        ),
        "max_predictor_reduction_step": max_by(
            pred_limited, ("predictor_limiter", "max_reduction_m3_s")
        ),
        "max_corrector_reduction_step": max_by(
            corr_limited, ("corrector_limiter", "max_reduction_m3_s")
        ),
        "max_tvd_delta_step": max_by(steps, ("tvd", "max_abs_delta_m")),
    }


def overlapping_steps(
    run: dict[str, Any],
    start_s: float,
    end_s: float,
) -> list[dict[str, Any]]:
    return [
        step
        for step in run["step_trace"]
        if float(step["t_start_s"]) < end_s and float(step["t_end_s"]) > start_s
    ]


def window_summary(rung: str, run: dict[str, Any], start_s: float, end_s: float) -> dict[str, Any]:
    steps = overlapping_steps(run, start_s, end_s)
    if not steps:
        return {"rung": rung, "step_count": 0}
    return {
        "rung": rung,
        "step_count": len(steps),
        "source_m3": sum(float(step["source_m3"]) for step in steps),
        "outflow_m3": sum(float(step["outflow_m3"]) for step in steps),
        "clamp_m3": sum(float(step["clamp_injected_m3"]) for step in steps),
        "negative_outflow_m3": sum(
            float(step["outflow_m3"]) for step in steps if float(step["outflow_m3"]) < 0.0
        ),
        "min_pred_out_face_m3_s": min(float(step["pred_out_face_m3_s"]) for step in steps),
        "min_corr_out_face_m3_s": min(float(step["corr_out_face_m3_s"]) for step in steps),
        "predictor_limiter_reductions": sum(
            int(step["predictor_limiter"]["reductions"]) for step in steps
        ),
        "corrector_limiter_reductions": sum(
            int(step["corrector_limiter"]["reductions"]) for step in steps
        ),
        "tvd_limited_steps": sum(
            1 for step in steps if float(step["tvd"]["scale"]) < 1.0 - 1.0e-15
        ),
        "max_abs_tvd_delta_m": max(
            abs(float(step["tvd"]["max_abs_delta_m"])) for step in steps
        ),
        "first_step": {
            key: steps[0][key]
            for key in ["step_index", "t_start_s", "t_end_s", "dt_s", "max_courant"]
        },
        "last_step": {
            key: steps[-1][key]
            for key in ["step_index", "t_start_s", "t_end_s", "dt_s", "max_courant"]
        },
    }


def compare(
    name: str,
    a_name: str,
    b_name: str,
    role: str,
    runs: dict[str, dict[str, Any]],
) -> dict[str, Any]:
    a = runs[a_name]
    b = runs[b_name]
    if a["hydrograph_time_s"] != b["hydrograph_time_s"]:
        raise SystemExit(f"{name}: hydrograph sample times differ")
    if a["outlet_bin_spans_s"] != b["outlet_bin_spans_s"]:
        raise SystemExit(f"{name}: outlet bin spans differ")
    if len(a["weights"]) != len(b["weights"]):
        raise SystemExit(f"{name}: routed weight lengths differ")
    bin_diffs = top_diffs(a["outlet_bin_m3"], b["outlet_bin_m3"])
    cdf_deltas = [x - y for x, y in zip(cdf(a["outlet_bin_m3"]), cdf(b["outlet_bin_m3"]))]
    max_cdf_index = max(
        range(len(cdf_deltas)),
        key=lambda index: abs(cdf_deltas[index]),
        default=0,
    )
    edges = bin_edges(a["outlet_bin_spans_s"])
    top_bin = bin_diffs[0] if bin_diffs else {"index": 0, "abs_delta": 0.0, "signed_delta": 0.0}
    top_start_s, top_end_s = edges[int(top_bin["index"])]
    cdf_start_s, cdf_end_s = edges[max_cdf_index]
    return {
        "pair": name,
        "role": role,
        "candidate": a_name,
        "reference": b_name,
        "hour_weight_l1": l1(a["weights"], b["weights"]),
        "hour_weight_passes_1_over_60": l1(a["weights"], b["weights"]) <= SHAPE_THRESHOLD,
        "hour_weight_cdf_linf": linf(cdf(a["weights"]), cdf(b["weights"])),
        "hour_mass_l1_m3": l1(a["hour_masses_m3"], b["hour_masses_m3"]),
        "hour_mass_cdf_linf_m3": linf(cdf(a["hour_masses_m3"]), cdf(b["hour_masses_m3"])),
        "bin_mass_l1_m3": l1(a["outlet_bin_m3"], b["outlet_bin_m3"]),
        "bin_mass_cdf_linf_m3": linf(cdf(a["outlet_bin_m3"]), cdf(b["outlet_bin_m3"])),
        "sampled_hydrograph_l1_m3_s": l1(
            a["hydrograph_outlet_m3_s"], b["hydrograph_outlet_m3_s"]
        ),
        "sampled_hydrograph_linf_m3_s": linf(
            a["hydrograph_outlet_m3_s"], b["hydrograph_outlet_m3_s"]
        ),
        "outlet_delta_m3": a["outlet_m3"] - b["outlet_m3"],
        "storage_delta_m3": a["mesh_end_storage_m3"] - b["mesh_end_storage_m3"],
        "top_bin_mass_diffs_m3": bin_diffs,
        "top_bin_window": {
            "index": int(top_bin["index"]),
            "start_s": top_start_s,
            "end_s": top_end_s,
            "abs_delta_m3": float(top_bin["abs_delta"]),
            "signed_delta_m3": float(top_bin["signed_delta"]),
        },
        "max_cdf_window": {
            "index": max_cdf_index,
            "start_s": cdf_start_s,
            "end_s": cdf_end_s,
            "signed_cdf_delta_m3": cdf_deltas[max_cdf_index],
            "abs_cdf_delta_m3": abs(cdf_deltas[max_cdf_index]),
        },
        "top_bin_step_windows": {
            a_name: window_summary(a_name, a, top_start_s, top_end_s),
            b_name: window_summary(b_name, b, top_start_s, top_end_s),
        },
        "max_cdf_step_windows": {
            a_name: window_summary(a_name, a, cdf_start_s, cdf_end_s),
            b_name: window_summary(b_name, b, cdf_start_s, cdf_end_s),
        },
    }


def pair_by_name(comparisons: list[dict[str, Any]], name: str) -> dict[str, Any]:
    for pair in comparisons:
        if pair["pair"] == name:
            return pair
    raise SystemExit(f"comparison {name} missing")


def classify(
    step_summaries: dict[str, dict[str, Any]],
    comparisons: list[dict[str, Any]],
) -> dict[str, Any]:
    original = pair_by_name(comparisons, "spatial_dx1p25_vs_dx0p625_dt300")
    refined = pair_by_name(comparisons, "spatial_dx1p25_vs_dx0p625_dt75")
    dx1_dt300_150 = pair_by_name(comparisons, "dx1p25_timestep_300_vs_150")
    dx1_dt150_75 = pair_by_name(comparisons, "dx1p25_timestep_150_vs_75")
    dx0625_dt300_150 = pair_by_name(comparisons, "dx0p625_timestep_300_vs_150")
    dx0625_dt150_75 = pair_by_name(comparisons, "dx0p625_timestep_150_vs_75")

    controls = []
    for rung in RUNGS:
        summary = step_summaries[rung]
        if summary["reconstructed_bin_linf_m3"] > 1.0e-12:
            controls.append(f"{rung} clipped step-to-bin reconstruction mismatch")
        if summary["negative_step_outflow_steps"]:
            controls.append(f"{rung} has negative scheme-actual outlet outflow steps")
        if summary["predictor_limiter_steps"] or summary["corrector_limiter_steps"]:
            controls.append(f"{rung} has stage-limiter activity")
        if summary["tvd_limited_steps"]:
            controls.append(f"{rung} has TVD limiter activity")
    if controls:
        return {
            "classification": "MECHANISM-HOLD-NUMERICS-CONTROL-FAIL",
            "reason": "; ".join(controls[:4]),
            "contract_action": "no timestep-policy amendment; isolate the failing numeric control first",
            "original_shape_l1": original["hour_weight_l1"],
            "refined_shape_l1": refined["hour_weight_l1"],
            "threshold": SHAPE_THRESHOLD,
        }

    original_pass = original["hour_weight_l1"] <= SHAPE_THRESHOLD
    refined_pass = refined["hour_weight_l1"] <= SHAPE_THRESHOLD
    timestep_signal = max(
        dx1_dt300_150["hour_weight_l1"],
        dx1_dt150_75["hour_weight_l1"],
        dx0625_dt300_150["hour_weight_l1"],
        dx0625_dt150_75["hour_weight_l1"],
    )
    improved = refined["hour_weight_l1"] < original["hour_weight_l1"]
    improvement_ratio = (
        refined["hour_weight_l1"] / original["hour_weight_l1"]
        if original["hour_weight_l1"] > 0.0
        else 0.0
    )

    if not original_pass and refined_pass:
        classification = "TIMESTEP-POLICY-ARTIFACT-CLOSED"
        reason = (
            "the fixed-300 s miss closes when the spatial pair is compared under "
            "the same refined 75 s timestep cap"
        )
        contract_action = (
            "amend SC-OFEROUTE-001 before any renewed mesh promotion so target-dx "
            "adequacy is evaluated on a coupled space-time basis"
        )
    elif not original_pass and improved and timestep_signal > 0.0:
        classification = "TIMESTEP-POLICY-COUPLED-HOLD"
        reason = (
            "same-dx timestep refinement changes the day-792 shape and improves "
            "the spatial pair, but the refined pair still misses the gate"
        )
        contract_action = (
            "hold target-dx promotion; define a stricter coupled space-time ladder "
            "or lower diagnostic dt before ratification"
        )
    elif not original_pass and not improved:
        classification = "MECHANISM-HOLD-NONCONVERGENT-AFTER-DT-REFINEMENT"
        reason = (
            "same-timestep spatial refinement still misses and does not improve "
            "materially under lower max-dt controls"
        )
        contract_action = (
            "no mesh-policy promotion; continue numerics investigation beyond timestep cap"
        )
    else:
        classification = "NO-TIMESTEP-BLOCKER"
        reason = "the fixed-300 s spatial pair does not reproduce the adequacy miss"
        contract_action = "no contract amendment from this package"

    return {
        "classification": classification,
        "reason": reason,
        "contract_action": contract_action,
        "original_shape_l1": original["hour_weight_l1"],
        "refined_shape_l1": refined["hour_weight_l1"],
        "threshold": SHAPE_THRESHOLD,
        "improvement_ratio_refined_over_original": improvement_ratio,
        "max_same_dx_timestep_shape_l1": timestep_signal,
    }


def write_markdown(
    summary: dict[str, Any],
    runs: dict[str, dict[str, Any]],
    step_summaries: dict[str, dict[str, Any]],
    comparisons: list[dict[str, Any]],
    verdict: dict[str, str],
) -> None:
    release = summary["release_binary"]
    original_pair = pair_by_name(comparisons, "spatial_dx1p25_vs_dx0p625_dt300")
    refined_pair = pair_by_name(comparisons, "spatial_dx1p25_vs_dx0p625_dt75")
    lines = [
        "# Timestep Policy Adjudication",
        "",
        "Evidence mode: Ran.",
        "",
        "## Verdict",
        "",
        f"Classification: `{verdict['classification']}`.",
        "",
        verdict["reason"][0].upper() + verdict["reason"][1:],
        "",
        f"Contract action: {verdict['contract_action']}.",
        "",
        "This package does not amend the routed-shape tolerance and does not promote",
        "a production target `dx` default.",
        "",
        "## Command Provenance",
        "",
        "Rerun command:",
        "",
        "```bash",
        "OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1 \\",
        "  .venv/bin/python \\",
        "  docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001/artifacts/run_timestep_policy_ladder.py \\",
        "  --members mn_corn_h4",
        "```",
        "",
        "Analysis command:",
        "",
        "```bash",
        ".venv/bin/python \\",
        "  docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001/artifacts/analyze_timestep_policy.py",
        "```",
        "",
        "Release binary:",
        "",
        f"- Build command: `{release['build_command']}`",
        f"- Binary: `{release['path']}`",
        f"- SHA256: `{release['sha256']}`",
        f"- Git HEAD at build: `{release['git_head']}`",
        "",
        "## Rung Masses",
        "",
        "| Rung | Cells | dx m | max dt s | Source m3 | Outlet m3 | End storage m3 | Step count | Max Courant | Bin recon Linf m3 | Clamp m3 | TVD-limited steps | Stage-limiter reductions |",
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for rung in RUNGS:
        run = runs[rung]
        step = step_summaries[rung]
        reductions = int(step["predictor_limiter_reductions"]) + int(
            step["corrector_limiter_reductions"]
        )
        lines.append(
            f"| `{rung}` | {step['mesh_cell_count']} | {fmt(step['mesh_dx_m'])} | "
            f"{fmt(float(summary['rungs'][rung]['max_dt_s']))} | "
            f"{fmt(run['source_m3'])} | {fmt(run['outlet_m3'])} | "
            f"{fmt(run['mesh_end_storage_m3'])} | {step['step_count']} | "
            f"{fmt(step['max_courant'])} | {fmt(step['reconstructed_bin_linf_m3'])} | "
            f"{fmt(step['clamp_total_m3'])} | "
            f"{step['tvd_limited_steps']} | {reductions} |"
        )
    lines.extend(
        [
            "",
            "## Pair Evidence",
            "",
            f"Shape threshold: `{fmt(SHAPE_THRESHOLD)}`.",
            "",
            "| Pair | Role | Hour shape L1 | Passes 1/60 | Hour CDF Linf | Bin mass L1 m3 | Bin CDF Linf m3 | Sampled hydrograph L1 m3/s | Outlet delta m3 | Storage delta m3 |",
            "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for pair in comparisons:
        lines.append(
            f"| `{pair['pair'].replace('_vs_', '` vs `')}` | "
            f"{pair['role']} | {fmt(pair['hour_weight_l1'])} | "
            f"{pair['hour_weight_passes_1_over_60']} | "
            f"{fmt(pair['hour_weight_cdf_linf'])} | "
            f"{fmt(pair['bin_mass_l1_m3'])} | {fmt(pair['bin_mass_cdf_linf_m3'])} | "
            f"{fmt(pair['sampled_hydrograph_l1_m3_s'])} | {fmt(pair['outlet_delta_m3'])} | "
            f"{fmt(pair['storage_delta_m3'])} |"
        )
    window = original_pair["top_bin_window"]
    cdf_window = original_pair["max_cdf_window"]
    lines.extend(
        [
            "",
            "Top original fixed-300 outlet-bin difference:",
            "",
            f"- Bin index: `{window['index']}`",
            f"- Window: `{fmt(window['start_s'])}` to `{fmt(window['end_s'])}` s",
            f"- Absolute delta: `{fmt(window['abs_delta_m3'])} m3`",
            f"- Signed delta (`dx1p25_dt300 - dx0p625_dt300`): `{fmt(window['signed_delta_m3'])} m3`",
            "",
            "Maximum original fixed-300 cumulative outlet difference:",
            "",
            f"- Bin index: `{cdf_window['index']}`",
            f"- Window: `{fmt(cdf_window['start_s'])}` to `{fmt(cdf_window['end_s'])}` s",
            f"- Absolute CDF delta: `{fmt(cdf_window['abs_cdf_delta_m3'])} m3`",
            f"- Signed CDF delta (`dx1p25_dt300 - dx0p625_dt300`): `{fmt(cdf_window['signed_cdf_delta_m3'])} m3`",
            "",
            "Refined same-75 spatial pair:",
            "",
            f"- Hour shape L1: `{fmt(refined_pair['hour_weight_l1'])}`",
            f"- Bin mass CDF Linf: `{fmt(refined_pair['bin_mass_cdf_linf_m3'])} m3`",
            f"- Outlet delta: `{fmt(refined_pair['outlet_delta_m3'])} m3`",
            "",
            "## Step-Trace Discriminants",
            "",
            "Original and refined spatial-pair source and upstream controls:",
            "",
            f"- `dx1p25_dt300` step source total: `{fmt(step_summaries['dx1p25_dt300']['source_total_m3'])} m3`",
            f"- `dx0p625_dt300` step source total: `{fmt(step_summaries['dx0p625_dt300']['source_total_m3'])} m3`",
            f"- `dx1p25_dt75` step source total: `{fmt(step_summaries['dx1p25_dt75']['source_total_m3'])} m3`",
            f"- `dx0p625_dt75` step source total: `{fmt(step_summaries['dx0p625_dt75']['source_total_m3'])} m3`",
            f"- `dx1p25_dt300` upstream inflow total: `{fmt(step_summaries['dx1p25_dt300']['upstream_inflow_total_m3'])} m3`",
            f"- `dx0p625_dt300` upstream inflow total: `{fmt(step_summaries['dx0p625_dt300']['upstream_inflow_total_m3'])} m3`",
            f"- `dx1p25_dt75` upstream inflow total: `{fmt(step_summaries['dx1p25_dt75']['upstream_inflow_total_m3'])} m3`",
            f"- `dx0p625_dt75` upstream inflow total: `{fmt(step_summaries['dx0p625_dt75']['upstream_inflow_total_m3'])} m3`",
            "",
            "Boundary and limiter controls:",
            "",
            f"- `dx1p25_dt300` negative outlet-outflow steps: `{step_summaries['dx1p25_dt300']['negative_step_outflow_steps']}`",
            f"- `dx0p625_dt300` negative outlet-outflow steps: `{step_summaries['dx0p625_dt300']['negative_step_outflow_steps']}`",
            f"- `dx1p25_dt75` negative outlet-outflow steps: `{step_summaries['dx1p25_dt75']['negative_step_outflow_steps']}`",
            f"- `dx0p625_dt75` negative outlet-outflow steps: `{step_summaries['dx0p625_dt75']['negative_step_outflow_steps']}`",
            f"- `dx1p25_dt300` min predictor/corrector outlet face: `{fmt(step_summaries['dx1p25_dt300']['min_pred_out_face_m3_s'])}` / `{fmt(step_summaries['dx1p25_dt300']['min_corr_out_face_m3_s'])}` m3/s",
            f"- `dx0p625_dt300` min predictor/corrector outlet face: `{fmt(step_summaries['dx0p625_dt300']['min_pred_out_face_m3_s'])}` / `{fmt(step_summaries['dx0p625_dt300']['min_corr_out_face_m3_s'])}` m3/s",
            f"- `dx1p25_dt75` min predictor/corrector outlet face: `{fmt(step_summaries['dx1p25_dt75']['min_pred_out_face_m3_s'])}` / `{fmt(step_summaries['dx1p25_dt75']['min_corr_out_face_m3_s'])}` m3/s",
            f"- `dx0p625_dt75` min predictor/corrector outlet face: `{fmt(step_summaries['dx0p625_dt75']['min_pred_out_face_m3_s'])}` / `{fmt(step_summaries['dx0p625_dt75']['min_corr_out_face_m3_s'])}` m3/s",
            f"- `dx1p25_dt300` TVD-limited steps: `{step_summaries['dx1p25_dt300']['tvd_limited_steps']}`",
            f"- `dx0p625_dt300` TVD-limited steps: `{step_summaries['dx0p625_dt300']['tvd_limited_steps']}`",
            f"- `dx1p25_dt75` TVD-limited steps: `{step_summaries['dx1p25_dt75']['tvd_limited_steps']}`",
            f"- `dx0p625_dt75` TVD-limited steps: `{step_summaries['dx0p625_dt75']['tvd_limited_steps']}`",
            f"- `dx1p25_dt300` stage-limiter reductions: `{int(step_summaries['dx1p25_dt300']['predictor_limiter_reductions']) + int(step_summaries['dx1p25_dt300']['corrector_limiter_reductions'])}`",
            f"- `dx0p625_dt300` stage-limiter reductions: `{int(step_summaries['dx0p625_dt300']['predictor_limiter_reductions']) + int(step_summaries['dx0p625_dt300']['corrector_limiter_reductions'])}`",
            f"- `dx1p25_dt75` stage-limiter reductions: `{int(step_summaries['dx1p25_dt75']['predictor_limiter_reductions']) + int(step_summaries['dx1p25_dt75']['corrector_limiter_reductions'])}`",
            f"- `dx0p625_dt75` stage-limiter reductions: `{int(step_summaries['dx0p625_dt75']['predictor_limiter_reductions']) + int(step_summaries['dx0p625_dt75']['corrector_limiter_reductions'])}`",
            "",
            "Top-bin step-window summaries are recorded in",
            "`timestep-policy-adjudication.json` under",
            "`comparisons[*].top_bin_step_windows`.",
        ]
    )
    OUTPUT_MD.write_text("\n".join(lines) + "\n")


def main() -> None:
    summary = json.loads(SUMMARY_JSON.read_text())
    runs = {rung: detail_for(summary, rung) for rung in RUNGS}
    write_analysis_inputs(summary, runs)
    step_summaries = {rung: step_summary(rung, runs[rung]) for rung in RUNGS}
    comparisons = [
        compare(name, a_name, b_name, role, runs)
        for name, a_name, b_name, role in PAIRS
    ]
    verdict = classify(step_summaries, comparisons)
    rung_overview = {
        rung: {
            "trace_path": runs[rung].get("trace_path") or runs[rung].get("raw_trace_path_recorded"),
            "trace_sha256": runs[rung]["trace_sha256"],
            "hbp_sha256": runs[rung]["hbp_sha256"],
            "pass_parquet_sha256": runs[rung]["pass_parquet_sha256"],
            "max_dt_s": runs[rung]["max_dt_s"],
            "mesh_cell_count": runs[rung]["mesh_cell_count"],
            "mesh_dx_m": runs[rung]["mesh_dx_m"],
            "source_m3": runs[rung]["source_m3"],
            "outlet_m3": runs[rung]["outlet_m3"],
            "mesh_end_storage_m3": runs[rung]["mesh_end_storage_m3"],
            "tail_fold_m3": runs[rung]["tail_fold_m3"],
        }
        for rung in RUNGS
    }
    output = {
        "schema": "openwepp-mn-corn-h4-day792-timestep-policy-v1",
        "member_id": MEMBER_ID,
        "sim_day_index": DAY_INDEX,
        "lane_index": LANE_INDEX,
        "release_binary": summary["release_binary"],
        "rungs": rung_overview,
        "step_summaries": step_summaries,
        "comparisons": comparisons,
        "verdict": verdict,
    }
    OUTPUT_JSON.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n")
    write_markdown(summary, runs, step_summaries, comparisons, verdict)


if __name__ == "__main__":
    main()
