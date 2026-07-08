#!/usr/bin/env python3
"""Analyze mn_corn_h4 day-792 raw-hydrograph nonconvergence traces."""

from __future__ import annotations

import json
from itertools import accumulate
from pathlib import Path
from typing import Any

PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE_DIR / "artifacts"
SUMMARY_JSON = ARTIFACTS / "raw-hydrograph-numerics-summary.json"
OUTPUT_JSON = ARTIFACTS / "mechanism-attribution.json"
OUTPUT_MD = ARTIFACTS / "mechanism-attribution.md"

MEMBER_ID = "mn_corn_h4"
DAY_INDEX = 792
LANE_INDEX = 1
RUNGS = ["dx2p5", "dx1p25", "dx0p625"]
PAIRS = [
    ("dx2p5_vs_dx1p25", "dx2p5", "dx1p25"),
    ("dx1p25_vs_dx0p625", "dx1p25", "dx0p625"),
]


def fmt(value: float) -> str:
    return f"{value:.17g}"


def l1(a: list[float], b: list[float]) -> float:
    return sum(abs(x - y) for x, y in zip(a, b))


def linf(a: list[float], b: list[float]) -> float:
    return max((abs(x - y) for x, y in zip(a, b)), default=0.0)


def cdf(values: list[float]) -> list[float]:
    return list(accumulate(values))


def top_diffs(a: list[float], b: list[float], count: int = 8) -> list[dict[str, float | int]]:
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
            "run_raw_hydrograph_numerics_ladder.py before analyzer replay"
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


def compare(name: str, a_name: str, b_name: str, runs: dict[str, dict[str, Any]]) -> dict[str, Any]:
    a = runs[a_name]
    b = runs[b_name]
    if a["hydrograph_time_s"] != b["hydrograph_time_s"]:
        raise SystemExit(f"{name}: hydrograph sample times differ")
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
        "hour_weight_l1": l1(a["weights"], b["weights"]),
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


def classify(step_summaries: dict[str, dict[str, Any]], fine_pair: dict[str, Any]) -> dict[str, str]:
    fine = step_summaries["dx0p625"]
    mid = step_summaries["dx1p25"]
    if abs(fine["source_total_m3"] - mid["source_total_m3"]) > 1.0e-10:
        return {
            "classification": "CORRECTABLE-CANDIDATE-SOURCE-SAMPLING",
            "reason": "fine-pair source totals diverge materially",
        }
    if abs(fine["upstream_inflow_total_m3"]) > 1.0e-14 or abs(mid["upstream_inflow_total_m3"]) > 1.0e-14:
        return {
            "classification": "CORRECTABLE-CANDIDATE-UPSTREAM-HANDOFF",
            "reason": "target lane has nonzero upstream injection",
        }
    if fine["reconstructed_bin_linf_m3"] > 1.0e-12 or mid["reconstructed_bin_linf_m3"] > 1.0e-12:
        return {
            "classification": "CORRECTABLE-CANDIDATE-BIN-ATTRIBUTION",
            "reason": "clipped step reconstruction does not reproduce published outlet bins",
        }
    if fine["negative_step_outflow_steps"] or mid["negative_step_outflow_steps"]:
        return {
            "classification": "MECHANISM-HOLD-BOUNDARY-FLUX-SIGN",
            "reason": "fine-pair traces include negative scheme-actual outlet outflow steps",
        }
    if fine["predictor_limiter_steps"] or fine["corrector_limiter_steps"] or mid["predictor_limiter_steps"] or mid["corrector_limiter_steps"]:
        return {
            "classification": "MECHANISM-HOLD-STAGE-FLUX-LIMITER",
            "reason": "mesh-dependent stage-face limiter events are active on the fine pair",
        }
    if fine["tvd_limited_steps"] or mid["tvd_limited_steps"]:
        return {
            "classification": "MECHANISM-HOLD-TVD-LIMITER",
            "reason": "mesh-dependent TVD limiter scaling is active on the fine pair",
        }
    if (
        fine["step_count"] > mid["step_count"]
        and fine["max_courant"] >= 0.899
        and mid["max_courant"] < 0.89
    ):
        return {
            "classification": "MECHANISM-HOLD-CFL-TIMESTEP-TRANSITION",
            "reason": (
                "fine rung leaves the 300 s cap and becomes CFL-limited while "
                "the middle rung remains cap-limited; the failing comparison is "
                "not a pure spatial reference check"
            ),
        }
    if fine_pair["bin_mass_cdf_linf_m3"] > 0.0:
        return {
            "classification": "MECHANISM-HOLD-RAW-HYDROGRAPH-NONCONVERGENCE",
            "reason": "source and upstream are controlled, but raw outlet CDF still worsens",
        }
    return {
        "classification": "NO-REPRO",
        "reason": "fine-pair raw outlet discrepancy did not reproduce",
    }


def write_markdown(
    summary: dict[str, Any],
    runs: dict[str, dict[str, Any]],
    step_summaries: dict[str, dict[str, Any]],
    comparisons: list[dict[str, Any]],
    verdict: dict[str, str],
) -> None:
    release = summary["release_binary"]
    fine_pair = next(pair for pair in comparisons if pair["pair"] == "dx1p25_vs_dx0p625")
    lines = [
        "# Raw-Hydrograph Mechanism Attribution",
        "",
        "Evidence mode: Ran.",
        "",
        "## Verdict",
        "",
        f"Classification: `{verdict['classification']}`.",
        "",
        verdict["reason"][0].upper() + verdict["reason"][1:],
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
        "  docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001/artifacts/run_raw_hydrograph_numerics_ladder.py \\",
        "  --members mn_corn_h4 --rungs dx2p5 dx1p25 dx0p625",
        "```",
        "",
        "Analysis command:",
        "",
        "```bash",
        ".venv/bin/python \\",
        "  docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001/artifacts/analyze_raw_hydrograph_numerics.py",
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
        "| Rung | Cells | dx m | Source m3 | Outlet m3 | End storage m3 | Step count | Max Courant | Bin recon Linf m3 | Clamp m3 | TVD-limited steps | Stage-limiter reductions |",
        "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for rung in RUNGS:
        run = runs[rung]
        step = step_summaries[rung]
        reductions = int(step["predictor_limiter_reductions"]) + int(
            step["corrector_limiter_reductions"]
        )
        lines.append(
            f"| `{rung}` | {step['mesh_cell_count']} | {fmt(step['mesh_dx_m'])} | "
            f"{fmt(run['source_m3'])} | {fmt(run['outlet_m3'])} | "
            f"{fmt(run['mesh_end_storage_m3'])} | {step['step_count']} | "
            f"{fmt(step['max_courant'])} | {fmt(step['reconstructed_bin_linf_m3'])} | "
            f"{fmt(step['clamp_total_m3'])} | "
            f"{step['tvd_limited_steps']} | {reductions} |"
        )
    lines.extend(
        [
            "",
            "## Fine-Pair Raw-Hydrograph Evidence",
            "",
            "| Pair | Hour shape L1 | Hour CDF Linf | Bin mass L1 m3 | Bin CDF Linf m3 | Sampled hydrograph L1 m3/s | Outlet delta m3 | Storage delta m3 |",
            "|---|---:|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for pair in comparisons:
        lines.append(
            f"| `{pair['pair'].replace('_vs_', '` vs `')}` | "
            f"{fmt(pair['hour_weight_l1'])} | {fmt(pair['hour_weight_cdf_linf'])} | "
            f"{fmt(pair['bin_mass_l1_m3'])} | {fmt(pair['bin_mass_cdf_linf_m3'])} | "
            f"{fmt(pair['sampled_hydrograph_l1_m3_s'])} | {fmt(pair['outlet_delta_m3'])} | "
            f"{fmt(pair['storage_delta_m3'])} |"
        )
    window = fine_pair["top_bin_window"]
    cdf_window = fine_pair["max_cdf_window"]
    lines.extend(
        [
            "",
            "Top fine-pair outlet-bin difference:",
            "",
            f"- Bin index: `{window['index']}`",
            f"- Window: `{fmt(window['start_s'])}` to `{fmt(window['end_s'])}` s",
            f"- Absolute delta: `{fmt(window['abs_delta_m3'])} m3`",
            f"- Signed delta (`dx1p25 - dx0p625`): `{fmt(window['signed_delta_m3'])} m3`",
            "",
            "Maximum fine-pair cumulative outlet difference:",
            "",
            f"- Bin index: `{cdf_window['index']}`",
            f"- Window: `{fmt(cdf_window['start_s'])}` to `{fmt(cdf_window['end_s'])}` s",
            f"- Absolute CDF delta: `{fmt(cdf_window['abs_cdf_delta_m3'])} m3`",
            f"- Signed CDF delta (`dx1p25 - dx0p625`): `{fmt(cdf_window['signed_cdf_delta_m3'])} m3`",
            "",
            "## Step-Trace Discriminants",
            "",
            "Fine-pair source and upstream controls:",
            "",
            f"- `dx1p25` step source total: `{fmt(step_summaries['dx1p25']['source_total_m3'])} m3`",
            f"- `dx0p625` step source total: `{fmt(step_summaries['dx0p625']['source_total_m3'])} m3`",
            f"- `dx1p25` upstream inflow total: `{fmt(step_summaries['dx1p25']['upstream_inflow_total_m3'])} m3`",
            f"- `dx0p625` upstream inflow total: `{fmt(step_summaries['dx0p625']['upstream_inflow_total_m3'])} m3`",
            f"- `dx1p25` clipped step-to-bin reconstruction Linf: `{fmt(step_summaries['dx1p25']['reconstructed_bin_linf_m3'])} m3`",
            f"- `dx0p625` clipped step-to-bin reconstruction Linf: `{fmt(step_summaries['dx0p625']['reconstructed_bin_linf_m3'])} m3`",
            f"- `dx1p25` max-Courant cell/x: `{step_summaries['dx1p25']['max_courant_step']['max_courant_cell_index']}` / `{fmt(float(step_summaries['dx1p25']['max_courant_step']['max_courant_cell_center_x_m']))} m`",
            f"- `dx0p625` max-Courant cell/x: `{step_summaries['dx0p625']['max_courant_step']['max_courant_cell_index']}` / `{fmt(float(step_summaries['dx0p625']['max_courant_step']['max_courant_cell_center_x_m']))} m`",
            "",
            "Fine-pair boundary and limiter controls:",
            "",
            f"- `dx1p25` negative outlet-outflow steps: `{step_summaries['dx1p25']['negative_step_outflow_steps']}`",
            f"- `dx0p625` negative outlet-outflow steps: `{step_summaries['dx0p625']['negative_step_outflow_steps']}`",
            f"- `dx1p25` min predictor/corrector outlet face: `{fmt(step_summaries['dx1p25']['min_pred_out_face_m3_s'])}` / `{fmt(step_summaries['dx1p25']['min_corr_out_face_m3_s'])}` m3/s",
            f"- `dx0p625` min predictor/corrector outlet face: `{fmt(step_summaries['dx0p625']['min_pred_out_face_m3_s'])}` / `{fmt(step_summaries['dx0p625']['min_corr_out_face_m3_s'])}` m3/s",
            f"- `dx1p25` TVD-limited steps: `{step_summaries['dx1p25']['tvd_limited_steps']}`",
            f"- `dx0p625` TVD-limited steps: `{step_summaries['dx0p625']['tvd_limited_steps']}`",
            f"- `dx1p25` stage-limiter reductions: `{int(step_summaries['dx1p25']['predictor_limiter_reductions']) + int(step_summaries['dx1p25']['corrector_limiter_reductions'])}`",
            f"- `dx0p625` stage-limiter reductions: `{int(step_summaries['dx0p625']['predictor_limiter_reductions']) + int(step_summaries['dx0p625']['corrector_limiter_reductions'])}`",
            "",
            "Top-bin step-window summaries are recorded in",
            "`mechanism-attribution.json` under",
            "`comparisons[*].top_bin_step_windows`.",
        ]
    )
    OUTPUT_MD.write_text("\n".join(lines) + "\n")


def main() -> None:
    summary = json.loads(SUMMARY_JSON.read_text())
    runs = {rung: detail_for(summary, rung) for rung in RUNGS}
    step_summaries = {rung: step_summary(rung, runs[rung]) for rung in RUNGS}
    comparisons = [
        compare(name, a_name, b_name, runs) for name, a_name, b_name in PAIRS
    ]
    fine_pair = next(pair for pair in comparisons if pair["pair"] == "dx1p25_vs_dx0p625")
    verdict = classify(step_summaries, fine_pair)
    rung_overview = {
        rung: {
            "trace_path": runs[rung]["trace_path"],
            "trace_sha256": runs[rung]["trace_sha256"],
            "hbp_sha256": runs[rung]["hbp_sha256"],
            "pass_parquet_sha256": runs[rung]["pass_parquet_sha256"],
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
        "schema": "openwepp-mn-corn-h4-day792-raw-hydrograph-mechanism-v1",
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
