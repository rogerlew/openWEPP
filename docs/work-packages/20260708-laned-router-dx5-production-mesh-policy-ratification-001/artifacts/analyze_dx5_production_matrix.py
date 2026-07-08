#!/usr/bin/env python3
"""Rebuild the dx5 production-promotion matrix under SC-OFEROUTE rev 44."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE_DIR / "artifacts"
COUPLED_SUMMARY = Path(
    "docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/"
    "artifacts/coupled-spacetime-summary.json"
)
ANNUAL_REPLAY = Path(
    "docs/work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001/"
    "artifacts/annual-sediment-metric-replay.json"
)
OUTPUT_JSON = ARTIFACTS / "rev44-promotion-matrix.json"
OUTPUT_MD = ARTIFACTS / "rev44-promotion-matrix.md"

REAL_MEMBERS = ["mn_corn_h4", "n_idaho_forest_h1", "wa_cascades_forest_h1"]
ACCEPTANCE_THRESHOLDS = {
    "terminal_outlet_l1_rel": 0.01,
    "max_shape_l1": 0.05,
    "end_storage_delta_rel_source": 0.01,
    "tail_fold_delta_rel_source": 0.01,
}
ADEQUACY_THRESHOLDS = {
    name: value / 3.0 for name, value in ACCEPTANCE_THRESHOLDS.items()
}

ONE_THIRD_ROLES = {"fine_reference_adequacy_dt75"}
REPORT_ONLY_ROLES = {"fine_reference_adequacy_dt300"}
ACCEPTANCE_ROLES = {
    "candidate_vs_reference_dt300",
    "candidate_vs_reference_dt75",
    "timestep_control_dx5",
    "timestep_control_dx2p5",
    "timestep_control_dx1p25",
}


def fmt(value: Any) -> str:
    if value is None:
        return "n/a"
    if isinstance(value, str):
        return value
    if isinstance(value, int):
        return str(value)
    return f"{float(value):.8g}"


def annual_key(record: dict[str, Any]) -> tuple[str, str, str, str]:
    return (
        record["member_id"],
        record["role"],
        record["candidate_rung"],
        record["reference_rung"],
    )


def annual_lookup(replay: dict[str, Any]) -> dict[tuple[str, str, str, str], dict[str, Any]]:
    return {annual_key(record): record for record in replay["comparisons"]}


def comparison_key(record: dict[str, Any]) -> tuple[str, str, str, str]:
    return (
        record["member_id"],
        record["comparison_role"],
        record["candidate_rung"],
        record["reference_rung"],
    )


def metric_failures(
    comparison: dict[str, Any],
    annual: dict[str, Any],
    thresholds: dict[str, float],
) -> list[str]:
    failures: list[str] = []
    for name, threshold in thresholds.items():
        value = float(comparison[name])
        if value > threshold:
            failures.append(f"{name} {value:.8g} > {threshold:.8g}")
    if int(comparison["shape_exceedances_gt_0p05"]) > 0:
        failures.append(f"shape_exceedances_gt_0p05 {comparison['shape_exceedances_gt_0p05']} > 0")
    if int(comparison["uniform_shape_row_increase"]) > 0:
        failures.append(f"uniform_shape_row_increase {comparison['uniform_shape_row_increase']} > 0")
    if int(comparison["degenerate_shape_row_increase"]) > 0:
        failures.append(
            f"degenerate_shape_row_increase {comparison['degenerate_shape_row_increase']} > 0"
        )
    if not bool(annual["rev44"]["passes"]):
        failures.extend(f"annual_sediment_rev44: {item}" for item in annual["rev44"]["failures"])
    return failures


def row_for(
    comparison: dict[str, Any],
    annual: dict[str, Any],
    thresholds: dict[str, float],
) -> dict[str, Any]:
    failures = metric_failures(comparison, annual, thresholds)
    rev44 = annual["rev44"]
    strict = annual["strict_relative"]
    return {
        "member_id": comparison["member_id"],
        "role": comparison["comparison_role"],
        "candidate": comparison["candidate_rung"],
        "reference": comparison["reference_rung"],
        "terminal_outlet_l1_rel": comparison["terminal_outlet_l1_rel"],
        "max_shape_l1": comparison["max_shape_l1"],
        "end_storage_delta_rel_source": comparison["end_storage_delta_rel_source"],
        "tail_fold_delta_rel_source": comparison["tail_fold_delta_rel_source"],
        "annual_strict_max_rel": strict["max_rel"],
        "annual_strict_surface": strict["max_surface"],
        "annual_rev44_vector_max_rel": rev44["vector_max_rel"],
        "annual_rev44_material_max_rel": rev44["material_max_rel"],
        "annual_rev44_low_contribution_max_rel": rev44["low_contribution_max_rel"],
        "annual_rev44_passes": rev44["passes"],
        "verdict": "PASS" if not failures else "FAIL",
        "failures": failures,
    }


def aggregate_user(summary: dict[str, Any], rung: str) -> float:
    total = 0.0
    for run in summary.get("runs", []):
        if run.get("member_id") not in REAL_MEMBERS or run.get("rung") != rung:
            continue
        value = (run.get("timing") or {}).get("user_seconds")
        if value is not None:
            total += float(value)
    return total


def main() -> None:
    summary = json.loads(COUPLED_SUMMARY.read_text())
    replay = json.loads(ANNUAL_REPLAY.read_text())
    annual = annual_lookup(replay)
    rows: list[dict[str, Any]] = []
    missing_annual = []

    for comparison in summary["comparisons"]:
        if "comparison_skip" in comparison:
            continue
        if comparison["member_id"] not in REAL_MEMBERS:
            continue
        role = comparison["comparison_role"]
        if role in ONE_THIRD_ROLES or role in REPORT_ONLY_ROLES:
            thresholds = ADEQUACY_THRESHOLDS
        elif role in ACCEPTANCE_ROLES:
            thresholds = ACCEPTANCE_THRESHOLDS
        else:
            continue
        key = comparison_key(comparison)
        annual_record = annual.get(key)
        if annual_record is None:
            missing_annual.append(key)
            continue
        row = row_for(comparison, annual_record, thresholds)
        row["gate_class"] = "report-only" if role in REPORT_ONLY_ROLES else "gate"
        if role in REPORT_ONLY_ROLES:
            row["verdict"] = "REPORT"
        rows.append(row)

    blockers = [
        f"{row['member_id']} {row['role']}: {'; '.join(row['failures'])}"
        for row in rows
        if row["gate_class"] == "gate" and row["verdict"] != "PASS"
    ]
    blockers.extend(f"missing annual rev44 replay for {key}" for key in missing_annual)
    fixed10_user = aggregate_user(summary, "baseline_fixed10_dt300")
    dx5_user = aggregate_user(summary, "dx5_dt300")
    result = {
        "status": "DX5_PRODUCTION_RATIFIED_BY_EVIDENCE" if not blockers else "HOLD",
        "ratified_by_evidence": not blockers,
        "blockers": blockers,
        "rows": rows,
        "row_count": len(rows),
        "missing_annual_count": len(missing_annual),
        "annual_replay_source": str(ANNUAL_REPLAY),
        "coupled_summary_source": str(COUPLED_SUMMARY),
        "release_binary": summary.get("release_binary"),
        "aggregate_user_seconds": {
            "baseline_fixed10_dt300": fixed10_user,
            "dx5_dt300": dx5_user,
            "dx5_over_fixed10": dx5_user / fixed10_user if fixed10_user > 0.0 else None,
        },
    }
    OUTPUT_JSON.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")

    lines = [
        "# Rev-44 DX5 Promotion Matrix",
        "",
        "Evidence mode: Ran.",
        "",
        f"Status: `{result['status']}`",
        "",
        f"- Rows adjudicated: `{len(rows)}`",
        f"- Blockers: `{len(blockers)}`",
        f"- Missing annual replay rows: `{len(missing_annual)}`",
        f"- Aggregate real-cohort fixed10 user seconds: `{fmt(fixed10_user)}`",
        f"- Aggregate real-cohort dx5 user seconds: `{fmt(dx5_user)}`",
        f"- Runtime cost ratio, dx5/fixed10: `{fmt(result['aggregate_user_seconds']['dx5_over_fixed10'])}`",
        "",
        "Cost is priced evidence only under the standing fidelity-first posture.",
        "",
        "Rev 43 makes fixed-300 fine-reference rows report-only when a same-pair "
        "refined-`dt` spatial adequacy row and same-`dx` timestep controls are "
        "present. Gate-class fine-reference adequacy is therefore the shared "
        "`dt75` row; fixed-300 rows remain listed as sensitivity evidence.",
        "",
        "| Role | Class | Member | Candidate | Reference | Outlet rel | Shape L1 | End storage rel | Tail fold rel | Annual vector rel | Annual material rel | Annual low rel | Verdict | Failures |",
        "|---|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|---|---|",
    ]
    for row in rows:
        lines.append(
            "| {role} | {gate_class} | {member} | `{candidate}` | `{reference}` | {outlet} | {shape} | {storage} | {tail} | {avec} | {amat} | {alow} | {verdict} | {failures} |".format(
                role=row["role"],
                gate_class=row["gate_class"],
                member=row["member_id"],
                candidate=row["candidate"],
                reference=row["reference"],
                outlet=fmt(row["terminal_outlet_l1_rel"]),
                shape=fmt(row["max_shape_l1"]),
                storage=fmt(row["end_storage_delta_rel_source"]),
                tail=fmt(row["tail_fold_delta_rel_source"]),
                avec=fmt(row["annual_rev44_vector_max_rel"]),
                amat=fmt(row["annual_rev44_material_max_rel"]),
                alow=fmt(row["annual_rev44_low_contribution_max_rel"]),
                verdict=row["verdict"],
                failures="; ".join(row["failures"]) if row["failures"] else "none",
            )
        )
    if blockers:
        lines.extend(["", "## Blockers", ""])
        lines.extend(f"- {blocker}" for blocker in blockers)
    lines.extend(["", "Detailed JSON:", "", f"- `{OUTPUT_JSON}`"])
    OUTPUT_MD.write_text("\n".join(lines) + "\n")
    print(OUTPUT_JSON)
    raise SystemExit(0 if not blockers else 1)


if __name__ == "__main__":
    main()
