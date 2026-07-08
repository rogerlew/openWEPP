#!/usr/bin/env python3
"""Analyze the mn_corn_h4 day-792 routed-shape attribution fixture."""

from __future__ import annotations

import json
from itertools import accumulate
from pathlib import Path
from typing import Any

PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE_DIR / "artifacts"
SUMMARY_JSON = ARTIFACTS / "shape-attribution-summary.json"
OUTPUT_JSON = ARTIFACTS / "day792-attribution.json"
OUTPUT_MD = ARTIFACTS / "day792-attribution.md"

MEMBER_ID = "mn_corn_h4"
DAY_INDEX = 792
LANE_INDEX = 1
RUNGS = ["dx2p5", "dx1p25", "dx0p625"]
PAIRS = [
    ("dx2p5_vs_dx1p25", "dx2p5", "dx1p25"),
    ("dx1p25_vs_dx0p625", "dx1p25", "dx0p625"),
    ("dx2p5_vs_dx0p625", "dx2p5", "dx0p625"),
]


def l1(a: list[float], b: list[float]) -> float:
    return sum(abs(x - y) for x, y in zip(a, b))


def linf(a: list[float], b: list[float]) -> float:
    return max((abs(x - y) for x, y in zip(a, b)), default=0.0)


def cdf(values: list[float]) -> list[float]:
    return list(accumulate(values))


def top_diffs(a: list[float], b: list[float], count: int = 8) -> list[list[float | int]]:
    diffs = [(abs(x - y), index, x - y) for index, (x, y) in enumerate(zip(a, b))]
    diffs.sort(reverse=True)
    return [[abs_diff, index, diff] for abs_diff, index, diff in diffs[:count]]


def load_trace_row(trace_path: Path) -> dict[str, Any]:
    for line in trace_path.read_text().splitlines():
        if not line.strip():
            continue
        row = json.loads(line)
        if row.get("sim_day_index") == DAY_INDEX and row.get("lane_index") == LANE_INDEX:
            detail = row.get("trace_detail")
            if not detail:
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
    hour_masses_m3 = [weight * outlet_m3 for weight in weights]
    return {
        "source_m3": float(row["source_m3"]),
        "outlet_m3": outlet_m3,
        "mesh_end_storage_m3": float(row["mesh_end_storage_m3"]),
        "tail_fold_m3": float(row["tail_fold_m3"]),
        "weights": weights,
        "hour_masses_m3": hour_masses_m3,
        "outlet_bin_m3": [float(value) for value in detail["outlet_bin_m3"]],
        "outlet_bin_spans_s": [float(value) for value in detail["outlet_bin_spans_s"]],
        "hydrograph_time_s": [float(value) for value in detail["hydrograph_time_s"]],
        "hydrograph_outlet_m3_s": [
            float(value) for value in detail["hydrograph_outlet_m3_s"]
        ],
        "hydrograph_outlet_depth_m": [
            float(value) for value in detail["hydrograph_outlet_depth_m"]
        ],
    }


def compare(name: str, a_name: str, b_name: str, runs: dict[str, dict[str, Any]]) -> dict[str, Any]:
    a = runs[a_name]
    b = runs[b_name]
    if a["hydrograph_time_s"] != b["hydrograph_time_s"]:
        raise SystemExit(f"{name}: hydrograph sample times differ")
    return {
        "pair": name,
        "hour_weight_l1": l1(a["weights"], b["weights"]),
        "hour_weight_cdf_linf": linf(cdf(a["weights"]), cdf(b["weights"])),
        "hour_mass_l1_m3": l1(a["hour_masses_m3"], b["hour_masses_m3"]),
        "hour_mass_cdf_linf_m3": linf(cdf(a["hour_masses_m3"]), cdf(b["hour_masses_m3"])),
        "bin_mass_l1_m3": l1(a["outlet_bin_m3"], b["outlet_bin_m3"]),
        "bin_mass_cdf_l1_m3": l1(cdf(a["outlet_bin_m3"]), cdf(b["outlet_bin_m3"])),
        "bin_mass_cdf_linf_m3": linf(cdf(a["outlet_bin_m3"]), cdf(b["outlet_bin_m3"])),
        "sampled_hydrograph_l1_m3_s": l1(
            a["hydrograph_outlet_m3_s"],
            b["hydrograph_outlet_m3_s"],
        ),
        "sampled_hydrograph_linf_m3_s": linf(
            a["hydrograph_outlet_m3_s"],
            b["hydrograph_outlet_m3_s"],
        ),
        "outlet_delta_m3": a["outlet_m3"] - b["outlet_m3"],
        "storage_delta_m3": a["mesh_end_storage_m3"] - b["mesh_end_storage_m3"],
        "tail_delta_m3": a["tail_fold_m3"] - b["tail_fold_m3"],
        "top_hour_mass_diffs_m3": top_diffs(a["hour_masses_m3"], b["hour_masses_m3"]),
        "top_bin_mass_diffs_m3": top_diffs(a["outlet_bin_m3"], b["outlet_bin_m3"]),
    }


def fmt(value: float) -> str:
    return f"{value:.17g}"


def write_markdown(
    summary: dict[str, Any],
    runs: dict[str, dict[str, Any]],
    pairs: list[dict[str, Any]],
) -> None:
    release = summary["release_binary"]
    fine_pair = next(pair for pair in pairs if pair["pair"] == "dx1p25_vs_dx0p625")
    coarse_pair = next(pair for pair in pairs if pair["pair"] == "dx2p5_vs_dx1p25")
    amplification = fine_pair["hour_mass_l1_m3"] / abs(fine_pair["storage_delta_m3"])
    lines = [
        "# Day-792 Routed-Shape Attribution",
        "",
        "Evidence mode: Ran.",
        "",
        "## Verdict",
        "",
        f"`{MEMBER_ID}` day {DAY_INDEX} lane {LANE_INDEX} is classified as",
        "`SOLVER-CLASS / RAW-HYDROGRAPH-NONCONVERGED` for this package. The binding",
        "metric-repair path is not available because the normalized shape miss is not",
        "noise-scale, the hourly CDF distance does not converge, and the raw outlet",
        "hydrograph comparison also worsens on the finer rung pair.",
        "",
        "No `SC-OFEROUTE-001` shape-gate amendment lands in this package.",
        "",
        "## Command Provenance",
        "",
        "Rerun command:",
        "",
        "```bash",
        "OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1 \\",
        "  .venv/bin/python \\",
        "  docs/work-packages/20260708-laned-router-mn-corn-h4-routed-shape-attribution-001/artifacts/run_shape_attribution_ladder.py \\",
        "  --members mn_corn_h4 --rungs dx2p5 dx1p25 dx0p625",
        "```",
        "",
        "Analysis command:",
        "",
        "```bash",
        ".venv/bin/python \\",
        "  docs/work-packages/20260708-laned-router-mn-corn-h4-routed-shape-attribution-001/artifacts/analyze_day792_attribution.py",
        "```",
        "",
        "Release binary:",
        "",
        f"- Build command: `{release['build_command']}`",
        f"- Binary: `{release['path']}`",
        f"- SHA256: `{release['sha256']}`",
        f"- Git HEAD at build: `{release['git_head']}`",
        "",
        "Runner environment:",
        "",
        "- `OPENWEPP_LANED_ACTIVE=1`",
        "- `OPENWEPP_LANED_ACTIVE_TRACE=1`",
        "- `OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1`",
        "- `OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M=<rung-metres>`",
        "",
        "Trace detail selector:",
        "",
        "- Env var: `OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1`",
        "- Selector convention: one-based `sim_day:lane`",
        "- Captured detail rows: one row per rung for `sim_day_index=792`,",
        "  `lane_index=1`",
        "- Detail sizes: 76 outlet bins and 77 raw hydrograph samples per rung",
        "",
        "## Rung Masses",
        "",
        f"All values are for `{MEMBER_ID}`, day {DAY_INDEX}, lane {LANE_INDEX}.",
        "",
        "| Rung | Source m3 | Outlet m3 | End storage m3 | Tail fold m3 |",
        "|---|---:|---:|---:|---:|",
    ]
    for rung in RUNGS:
        rec = runs[rung]
        lines.append(
            f"| `{rung}` | {fmt(rec['source_m3'])} | {fmt(rec['outlet_m3'])} | "
            f"{fmt(rec['mesh_end_storage_m3'])} | {fmt(rec['tail_fold_m3'])} |"
        )
    lines.extend(
        [
            "",
            "## Discriminating Tests",
            "",
            "### 1. Normalization-Amplification",
            "",
            "The failing fine-reference pair is `dx1p25` vs `dx0p625`. Its normalized",
            f"hourly-shape L1 is `{fmt(fine_pair['hour_weight_l1'])}`, above the one-third adequacy",
            "threshold `0.0166667`.",
            "",
            "Converted to absolute hourly mass, the same delta is",
            f"`{fmt(fine_pair['hour_mass_l1_m3'])} m3`. The outlet/storage total difference on that pair is",
            f"only `{fmt(abs(fine_pair['outlet_delta_m3']))} m3` outlet and",
            f"`{fmt(abs(fine_pair['storage_delta_m3']))} m3` storage.",
            f"The reshuffled hourly mass is therefore about `{amplification:.1f}x` the end-window storage",
            "difference. This is not a near-zero denominator or noise-scale amplification.",
            "",
            "Result: `FAIL-METRIC-CLASS`.",
            "",
            "### 2. Hour-Edge Aliasing",
            "",
            "Hourly CDF max distance does not converge on the finer pair:",
            "",
            "| Pair | Hourly L1 weight | Hourly CDF Linf weight | Hourly L1 mass m3 | Hourly CDF Linf m3 |",
            "|---|---:|---:|---:|---:|",
        ]
    )
    for pair in [coarse_pair, fine_pair]:
        lines.append(
            f"| `{pair['pair'].replace('_vs_', '` vs `')}` | {fmt(pair['hour_weight_l1'])} | "
            f"{fmt(pair['hour_weight_cdf_linf'])} | {fmt(pair['hour_mass_l1_m3'])} | "
            f"{fmt(pair['hour_mass_cdf_linf_m3'])} |"
        )
    lines.extend(
        [
            "",
            "The binned L1 increase is accompanied by a larger CDF distance, so this is not",
            "only a mass packet straddling an hour boundary with converged cumulative",
            "arrival.",
            "",
            "Result: `FAIL-PROJECTION-ALIASING`.",
            "",
            "### 3. Raw Unbinned Outlet Hydrograph",
            "",
            "The raw outlet-hydrograph comparison also worsens on the finer pair:",
            "",
            "| Pair | Bin mass L1 m3 | Bin CDF Linf m3 | Sampled hydrograph L1 m3/s | Sampled hydrograph Linf m3/s |",
            "|---|---:|---:|---:|---:|",
        ]
    )
    for pair in [coarse_pair, fine_pair]:
        lines.append(
            f"| `{pair['pair'].replace('_vs_', '` vs `')}` | {fmt(pair['bin_mass_l1_m3'])} | "
            f"{fmt(pair['bin_mass_cdf_linf_m3'])} | {fmt(pair['sampled_hydrograph_l1_m3_s'])} | "
            f"{fmt(pair['sampled_hydrograph_linf_m3_s'])} |"
        )
    lines.extend(
        [
            "",
            "The raw unbinned outlet signal is therefore not converged under the tested",
            "rungs. This satisfies the handoff's solver/day classification branch.",
            "",
            "Result: `SOLVER-CLASS-HOLD`.",
            "",
            "## Important Row Clarification",
            "",
            "The prior hold shorthand said the `mn_corn_h4` miss was flat around",
            "`0.0202..0.0209` on day 792. The trace-enabled rerun sharpens that statement:",
            "day 792 is the `dx1p25` vs `dx0p625` max at",
            f"`{fmt(fine_pair['hour_weight_l1'])}`, but day 792 is only",
            f"`{fmt(coarse_pair['hour_weight_l1'])}` on the `dx2p5` vs `dx1p25` pair.",
            "The `dx2p5` vs `dx1p25` package-level max remains `0.02018051100943346` on a",
            "different positive-source day. This package's attribution is therefore for the",
            "actual day-792 fine-reference blocker named by the handoff.",
            "",
            "## Stored Evidence",
            "",
            "- Compact run summary:",
            "  `artifacts/shape-attribution-summary.md`",
            "- Machine-readable run summary:",
            "  `artifacts/shape-attribution-summary.json`",
            "- Machine-readable day attribution:",
            "  `artifacts/day792-attribution.json`",
            "- Raw run trees:",
            "  `artifacts/shape-attribution-runs/`, package-ignored",
        ]
    )
    OUTPUT_MD.write_text("\n".join(lines) + "\n")


def main() -> None:
    summary = json.loads(SUMMARY_JSON.read_text())
    runs = {rung: detail_for(summary, rung) for rung in RUNGS}
    pairs = [compare(name, a, b, runs) for name, a, b in PAIRS]
    result = {
        "member_id": MEMBER_ID,
        "sim_day_index": DAY_INDEX,
        "lane_index": LANE_INDEX,
        "runs": runs,
        "pairs": pairs,
        "classification": "SOLVER-CLASS / RAW-HYDROGRAPH-NONCONVERGED",
    }
    OUTPUT_JSON.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    write_markdown(summary, runs, pairs)


if __name__ == "__main__":
    main()
