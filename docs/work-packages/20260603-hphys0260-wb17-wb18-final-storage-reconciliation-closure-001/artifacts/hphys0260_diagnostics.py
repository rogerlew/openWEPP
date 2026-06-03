#!/usr/bin/env python3
"""Run HPHYS0260 WB17/WB18/final-storage trace localization diagnostics."""

from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any

import pandas as pd


REPO = Path("/home/workdir/openWEPP")
WEPPPY_PYTHON = Path("/workdir/wepppy/.venv/bin/python")
HPHYS0254_DIAGNOSTICS = (
    REPO
    / "docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/artifacts/hphys0254_diagnostics.py"
)
BASELINE_PARTITIONS = Path(
    "/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions"
)
TARGETED_IDS = [1, 7, 39]
TOL = 1.0e-9


def markdown_table(headers: list[str], rows: list[list[Any]]) -> str:
    def fmt(item: Any) -> str:
        if item is None:
            return ""
        if isinstance(item, float):
            return f"{item:.6f}"
        return str(item)

    lines = [
        "| " + " | ".join(headers) + " |",
        "| " + " | ".join(["---"] * len(headers)) + " |",
    ]
    for row in rows:
        lines.append("| " + " | ".join(fmt(item) for item in row) + " |")
    return "\n".join(lines) + "\n"


def load_trace_rows(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.strip():
            rows.append(json.loads(line))
    return rows


def find_day1_row(rows: list[dict[str, Any]], boundary: str, phase: str | None) -> dict[str, Any]:
    for row in rows:
        if (
            row.get("sim_day_index") == 1
            and row.get("boundary") == boundary
            and row.get("phase") == phase
        ):
            return row
    phase_label = phase if phase is not None else "none"
    raise ValueError(f"missing day-1 {boundary}:{phase_label} trace row")


def number(row: dict[str, Any], name: str) -> float | None:
    value = row.get(name)
    if value is None:
        return None
    return float(value)


def layer_values(row: dict[str, Any], name: str) -> dict[str, float]:
    values = row.get(name) or {}
    return {str(key): float(value) for key, value in values.items()}


def layer_sum(row: dict[str, Any], name: str) -> float:
    return sum(layer_values(row, name).values())


def wat_day1(path: Path, candidate: bool) -> pd.Series:
    df = pd.read_parquet(path).copy()
    if candidate:
        df["comparison_year"] = df["year"].astype(int) + 2012
        return df.sort_values(["comparison_year", "julian"]).iloc[0]
    return df.sort_values(["year", "julian"]).iloc[0]


def wat_value(row: pd.Series, *names: str) -> float:
    for name in names:
        if name in row and pd.notna(row[name]):
            return float(row[name])
    return 0.0


def finite_abs_close(value: float | None) -> bool:
    return value is not None and abs(value) <= TOL


def classify_wb17(row: dict[str, Any]) -> tuple[str, dict[str, float | int | None]]:
    upi = number(row, "upi_m")
    ui = number(row, "ui_m")
    ep = number(row, "ep_m")
    etp = number(row, "etp_m")
    ws = number(row, "ws")
    upi_layers = layer_values(row, "wb17_upi_layers_m")
    ui_layers = layer_values(row, "wb17_ui_layers_m")
    upi_sum = sum(upi_layers.values())
    ui_sum = sum(ui_layers.values())
    ep_ui_residual = None if ep is None else ep - ui_sum
    ui_sum_residual = None if ui is None else ui - ui_sum
    upi_sum_residual = None if upi is None else upi - upi_sum
    if ep is None or etp is None or ws is None or etp <= TOL:
        ws_residual = None
    else:
        ws_residual = ws - (ep / etp)
    layer_bound_violations = 0
    max_bound_violation = 0.0
    for suffix, actual in ui_layers.items():
        potential = upi_layers.get(suffix)
        if potential is None:
            layer_bound_violations += 1
            max_bound_violation = max(max_bound_violation, abs(actual))
            continue
        violation = max(actual - potential, -actual, 0.0)
        if violation > TOL:
            layer_bound_violations += 1
            max_bound_violation = max(max_bound_violation, violation)

    diagnostics: dict[str, float | int | None] = {
        "upi": upi,
        "ui": ui,
        "ep": ep,
        "etp": etp,
        "ws": ws,
        "upi_sum": upi_sum,
        "ui_sum": ui_sum,
        "ep_ui_residual": ep_ui_residual,
        "ui_sum_residual": ui_sum_residual,
        "upi_sum_residual": upi_sum_residual,
        "ws_residual": ws_residual,
        "layer_bound_violations": layer_bound_violations,
        "max_bound_violation": max_bound_violation,
    }
    required = [ep_ui_residual, ui_sum_residual, upi_sum_residual]
    if not upi_layers or not ui_layers or any(item is None for item in required):
        return "WB17_TRACE_INCOMPLETE", diagnostics
    if (
        all(finite_abs_close(float(item)) for item in required)
        and (ws_residual is None or finite_abs_close(ws_residual))
        and layer_bound_violations == 0
    ):
        return "WB17_IDENTITIES_CLOSED_MAGNITUDE_FOCUS", diagnostics
    return "WB17_INTERNAL_DIVERGENCE", diagnostics


def classify_wb18(row: dict[str, Any]) -> tuple[str, dict[str, float | None]]:
    d = number(row, "d_m")
    pe = number(row, "pe_m")
    recomputed_minus_wb11 = number(row, "wb18_recomputed_minus_wb11_m")
    d_pe_residual = None if d is None or pe is None else d - pe
    pei_sum = layer_sum(row, "wb18_pei_layers_m")
    diagnostics: dict[str, float | None] = {
        "d": d,
        "pe": pe,
        "d_pe_residual": d_pe_residual,
        "pei_sum": pei_sum,
        "recomputed_minus_wb11": recomputed_minus_wb11,
    }
    required = [d_pe_residual, recomputed_minus_wb11]
    if any(item is None for item in required):
        return "WB18_TRACE_INCOMPLETE", diagnostics
    if all(finite_abs_close(float(item)) for item in required):
        return "WB18_IDENTITIES_CLOSED_MAGNITUDE_FOCUS", diagnostics
    return "WB18_INTERNAL_DIVERGENCE", diagnostics


def classify_final_storage(row: dict[str, Any]) -> tuple[str, dict[str, float | None]]:
    wb11_mm = number(row, "wb11_soil_water_mm")
    total_soil_mm = number(row, "wb13_total_soil_mm")
    soil_water_total_mm = number(row, "wb13_soil_water_total_mm")
    recomputed_minus_wb11_m = number(row, "wb18_recomputed_minus_wb11_m")
    total_minus_wb11_mm = (
        None if total_soil_mm is None or wb11_mm is None else total_soil_mm - wb11_mm
    )
    soil_water_total_minus_total_mm = (
        None
        if soil_water_total_mm is None or total_soil_mm is None
        else soil_water_total_mm - total_soil_mm
    )
    diagnostics = {
        "wb11_mm": wb11_mm,
        "total_soil_mm": total_soil_mm,
        "soil_water_total_mm": soil_water_total_mm,
        "recomputed_minus_wb11_m": recomputed_minus_wb11_m,
        "total_minus_wb11_mm": total_minus_wb11_mm,
        "soil_water_total_minus_total_mm": soil_water_total_minus_total_mm,
    }
    required = [
        recomputed_minus_wb11_m,
        total_minus_wb11_mm,
        soil_water_total_minus_total_mm,
    ]
    if any(item is None for item in required):
        return "FINAL_STORAGE_TRACE_INCOMPLETE", diagnostics
    if (
        finite_abs_close(float(recomputed_minus_wb11_m))
        and abs(float(total_minus_wb11_mm)) <= TOL * 1_000.0
        and abs(float(soil_water_total_minus_total_mm)) <= TOL * 1_000.0
    ):
        return "FINAL_STORAGE_IDENTITIES_CLOSED_MAGNITUDE_FOCUS", diagnostics
    return "FINAL_STORAGE_INTERNAL_DIVERGENCE", diagnostics


def build_report(run_root: Path) -> None:
    reports = run_root / "reports"
    rows = []
    json_rows = []
    for hillslope_id in TARGETED_IDS:
        trace_path = run_root / f"hillslope_output/H{hillslope_id}.hphys0254.trace.jsonl"
        trace_rows = load_trace_rows(trace_path)
        root_row = find_day1_row(trace_rows, "post_phase", "plant_root_uptake")
        perc_row = find_day1_row(trace_rows, "post_phase", "percolation_deep_seepage")
        final_row = find_day1_row(trace_rows, "post_wb13", None)
        wb17_class, wb17 = classify_wb17(root_row)
        wb18_class, wb18 = classify_wb18(perc_row)
        final_class, final = classify_final_storage(final_row)
        candidate_wat = wat_day1(run_root / f"hillslope_output/H{hillslope_id}.wat.parquet", True)
        baseline_wat = wat_day1(BASELINE_PARTITIONS / f"baseline_H{hillslope_id}.parquet", False)
        ep_diff = wat_value(candidate_wat, "Ep") - wat_value(baseline_wat, "Ep")
        dp_diff = wat_value(candidate_wat, "Dp") - wat_value(baseline_wat, "Dp")
        total_diff = wat_value(candidate_wat, "Total-Soil", "Total-Soil Water") - wat_value(
            baseline_wat, "Total-Soil", "Total-Soil Water"
        )
        swt_diff = wat_value(candidate_wat, "SoilWaterTotal") - wat_value(
            baseline_wat, "SoilWaterTotal", "Total-Soil Water"
        )
        rows.append(
            [
                f"H{hillslope_id}",
                root_row.get("schema"),
                wb17_class,
                wb17["ep_ui_residual"],
                wb17["ui_sum_residual"],
                wb17["ws_residual"],
                wb17["layer_bound_violations"],
                wb18_class,
                wb18["d_pe_residual"],
                wb18["recomputed_minus_wb11"],
                final_class,
                final["recomputed_minus_wb11_m"],
                final["total_minus_wb11_mm"],
                final["soil_water_total_minus_total_mm"],
                ep_diff,
                dp_diff,
                total_diff,
                swt_diff,
            ]
        )
        json_rows.append(
            {
                "hillslope": f"H{hillslope_id}",
                "trace_schema": root_row.get("schema"),
                "trace_path": str(trace_path),
                "wb17_classification": wb17_class,
                "wb17": wb17,
                "wb18_classification": wb18_class,
                "wb18": wb18,
                "final_storage_classification": final_class,
                "final_storage": final,
                "residuals_mm": {
                    "Ep": ep_diff,
                    "Dp": dp_diff,
                    "Total-Soil": total_diff,
                    "SoilWaterTotal": swt_diff,
                },
            }
        )

    md = "# HPHYS0260 WB17/WB18/Storage Trace Classification\n\n"
    md += "Status: complete\n\nEvidence mode: ran\n\n"
    md += "Ran: HPHYS0254 diagnostic harness plus HPHYS0260 trace classification.\n\n"
    md += markdown_table(
        [
            "Hillslope",
            "Trace schema",
            "WB17 classification",
            "Ep-ΣUi m",
            "Ui-ΣUi m",
            "Ws residual",
            "Ui bound violations",
            "WB18 classification",
            "D-Pe m",
            "WB18 aggregate-wb11 m",
            "Final classification",
            "Final aggregate-wb11 m",
            "Total-Soil-wb11 mm",
            "SoilWaterTotal-Total mm",
            "Ep diff mm",
            "Dp diff mm",
            "Total-Soil diff mm",
            "SoilWaterTotal diff mm",
        ],
        rows,
    )
    md += "\nInterpretation:\n\n"
    md += (
        "- `*_IDENTITIES_CLOSED_MAGNITUDE_FOCUS` means trace-publication and "
        "aggregate identity checks close; continuation should target "
        "baseline-authoritative magnitude or initialization lineage.\n"
    )
    md += (
        "- `*_INTERNAL_DIVERGENCE` means the next package should focus on that "
        "process family before changing numerical magnitude.\n"
    )
    reports.joinpath("hphys0260_wb17_wb18_storage_classification.md").write_text(
        md, encoding="utf-8"
    )
    reports.joinpath("hphys0260_wb17_wb18_storage_classification.json").write_text(
        json.dumps(json_rows, indent=2) + "\n", encoding="utf-8"
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-root", required=True, type=Path)
    args = parser.parse_args()

    result = subprocess.run(
        [
            str(WEPPPY_PYTHON),
            str(HPHYS0254_DIAGNOSTICS),
            "--run-root",
            str(args.run_root),
        ],
        cwd=REPO,
        check=False,
    )
    if result.returncode != 0:
        return result.returncode
    build_report(args.run_root)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
