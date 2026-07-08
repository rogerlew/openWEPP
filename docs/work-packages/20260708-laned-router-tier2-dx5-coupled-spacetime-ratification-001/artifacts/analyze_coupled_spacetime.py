#!/usr/bin/env python3
"""Adjudicate the dx5 coupled space-time ladder summary."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE_DIR / "artifacts"
SUMMARY_JSON = ARTIFACTS / "coupled-spacetime-summary.json"
OUTPUT_JSON = ARTIFACTS / "mesh-policy-ratification.json"
OUTPUT_MD = ARTIFACTS / "mesh-policy-ratification.md"

REAL_MEMBERS = ["mn_corn_h4", "n_idaho_forest_h1", "wa_cascades_forest_h1"]

ACCEPTANCE_THRESHOLDS = {
    "terminal_outlet_l1_rel": 0.01,
    "max_shape_l1": 0.05,
    "end_storage_delta_rel_source": 0.01,
    "tail_fold_delta_rel_source": 0.01,
    "annual_sediment_max_rel": 0.02,
}

ADEQUACY_THRESHOLDS = {
    "terminal_outlet_l1_rel": ACCEPTANCE_THRESHOLDS["terminal_outlet_l1_rel"] / 3.0,
    "max_shape_l1": ACCEPTANCE_THRESHOLDS["max_shape_l1"] / 3.0,
    "end_storage_delta_rel_source": ACCEPTANCE_THRESHOLDS["end_storage_delta_rel_source"] / 3.0,
    "tail_fold_delta_rel_source": ACCEPTANCE_THRESHOLDS["tail_fold_delta_rel_source"] / 3.0,
    "annual_sediment_max_rel": ACCEPTANCE_THRESHOLDS["annual_sediment_max_rel"] / 3.0,
}

ONE_THIRD_ROLES = {"fine_reference_adequacy_dt300", "fine_reference_adequacy_dt75"}
ACCEPTANCE_ROLES = {
    "candidate_vs_reference_dt300",
    "candidate_vs_reference_dt75",
    "timestep_control_dx5",
    "timestep_control_dx2p5",
    "timestep_control_dx1p25",
}
REPORT_ONLY_ROLES: set[str] = set()


def fmt(value: float | int | str | None) -> str:
    if value is None:
        return "n/a"
    if isinstance(value, str):
        return value
    if isinstance(value, int):
        return str(value)
    return f"{value:.8g}"


def annual_max_rel(comparison: dict[str, Any]) -> float:
    return float((comparison.get("annual_pass_sediment") or {}).get("max_rel") or 0.0)


def comparison_by_key(summary: dict[str, Any]) -> dict[tuple[str, str], dict[str, Any]]:
    out: dict[tuple[str, str], dict[str, Any]] = {}
    for comparison in summary.get("comparisons", []):
        if "comparison_skip" in comparison:
            skip = comparison["comparison_skip"]
            out[(skip["member_id"], skip["comparison_role"])] = {"skip": skip}
        else:
            out[(comparison["member_id"], comparison["comparison_role"])] = comparison
    return out


def metric_failures(comparison: dict[str, Any], thresholds: dict[str, float]) -> list[str]:
    if "skip" in comparison:
        skip = comparison["skip"]
        return [f"skipped: {skip.get('reason', 'unknown')}"]
    checks = [
        ("terminal_outlet_l1_rel", float(comparison["terminal_outlet_l1_rel"])),
        ("max_shape_l1", float(comparison["max_shape_l1"])),
        ("end_storage_delta_rel_source", float(comparison["end_storage_delta_rel_source"])),
        ("tail_fold_delta_rel_source", float(comparison["tail_fold_delta_rel_source"])),
        ("annual_sediment_max_rel", annual_max_rel(comparison)),
    ]
    failures = [
        f"{name} {value:.8g} > {thresholds[name]:.8g}"
        for name, value in checks
        if value > thresholds[name]
    ]
    if int(comparison["shape_exceedances_gt_0p05"]) > 0:
        failures.append(
            f"shape_exceedances_gt_0p05 {comparison['shape_exceedances_gt_0p05']} > 0"
        )
    if int(comparison["uniform_shape_row_increase"]) > 0:
        failures.append(
            f"uniform_shape_row_increase {comparison['uniform_shape_row_increase']} > 0"
        )
    if int(comparison["degenerate_shape_row_increase"]) > 0:
        failures.append(
            "degenerate_shape_row_increase "
            f"{comparison['degenerate_shape_row_increase']} > 0"
        )
    return failures


def run_failures(summary: dict[str, Any]) -> list[str]:
    failures = []
    for run in summary.get("runs", []):
        if run.get("member_id") not in REAL_MEMBERS:
            continue
        if run.get("status") != "PASS":
            failures.append(f"{run['member_id']} {run['rung']} status {run.get('status')}")
    return failures


def aggregate_user(summary: dict[str, Any], rung: str) -> float:
    total = 0.0
    for run in summary.get("runs", []):
        if run.get("member_id") not in REAL_MEMBERS or run.get("rung") != rung:
            continue
        value = (run.get("timing") or {}).get("user_seconds")
        if value is not None:
            total += float(value)
    return total


def role_rows(
    keyed: dict[tuple[str, str], dict[str, Any]],
    role: str,
    thresholds: dict[str, float],
) -> list[dict[str, Any]]:
    rows = []
    for member in REAL_MEMBERS:
        comparison = keyed.get((member, role), {"skip": {"reason": "missing comparison"}})
        failures = metric_failures(comparison, thresholds)
        rows.append(
            {
                "member_id": member,
                "role": role,
                "candidate": comparison.get("candidate_rung", "n/a"),
                "reference": comparison.get("reference_rung", "n/a"),
                "terminal_outlet_l1_rel": comparison.get("terminal_outlet_l1_rel"),
                "max_shape_l1": comparison.get("max_shape_l1"),
                "end_storage_delta_rel_source": comparison.get("end_storage_delta_rel_source"),
                "tail_fold_delta_rel_source": comparison.get("tail_fold_delta_rel_source"),
                "annual_sediment_max_rel": annual_max_rel(comparison)
                if "skip" not in comparison
                else None,
                "uniform_shape_row_increase": comparison.get("uniform_shape_row_increase"),
                "degenerate_shape_row_increase": comparison.get(
                    "degenerate_shape_row_increase"
                ),
                "verdict": "PASS" if not failures else "FAIL",
                "failures": failures,
            }
        )
    return rows


def main() -> None:
    summary = json.loads(SUMMARY_JSON.read_text())
    keyed = comparison_by_key(summary)

    blocking_roles: dict[str, list[dict[str, Any]]] = {}
    report_roles: dict[str, list[dict[str, Any]]] = {}
    for role in sorted(ONE_THIRD_ROLES):
        blocking_roles[role] = role_rows(keyed, role, ADEQUACY_THRESHOLDS)
    for role in sorted(ACCEPTANCE_ROLES):
        blocking_roles[role] = role_rows(keyed, role, ACCEPTANCE_THRESHOLDS)
    for role in sorted(REPORT_ONLY_ROLES):
        report_roles[role] = role_rows(keyed, role, ACCEPTANCE_THRESHOLDS)

    blockers = run_failures(summary)
    for rows in blocking_roles.values():
        for row in rows:
            if row["verdict"] != "PASS":
                blockers.append(
                    f"{row['member_id']} {row['role']}: {'; '.join(row['failures'])}"
                )

    fixed10_user = aggregate_user(summary, "baseline_fixed10_dt300")
    dx5_user = aggregate_user(summary, "dx5_dt300")
    cost_ratio = dx5_user / fixed10_user if fixed10_user > 0.0 else None

    ratified = not blockers
    verdict = "DX5_PRODUCTION_RATIFIED" if ratified else "EXECUTED-HOLD-DX5-UNRATIFIED"
    result = {
        "status": verdict,
        "ratified": ratified,
        "blockers": blockers,
        "thresholds": {
            "acceptance": ACCEPTANCE_THRESHOLDS,
            "one_third_adequacy": ADEQUACY_THRESHOLDS,
        },
        "blocking_roles": blocking_roles,
        "report_only_roles": report_roles,
        "aggregate_user_seconds": {
            "baseline_fixed10_dt300": fixed10_user,
            "dx5_dt300": dx5_user,
            "dx5_over_fixed10": cost_ratio,
        },
        "summary_path": str(SUMMARY_JSON),
        "release_binary": summary.get("release_binary"),
    }
    OUTPUT_JSON.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")

    lines = [
        "# Mesh-Policy Ratification",
        "",
        "Evidence mode: Ran.",
        "",
        f"Status: `{verdict}`",
        "",
    ]
    if ratified:
        lines.append(
            "`dx5` passes the conservative coupled space-time promotion gate for the "
            "selected real cohort."
        )
    else:
        lines.append(
            "`dx5` is not production-ratified by this package because at least one "
            "blocking coupled comparison failed."
        )
    lines.extend(
        [
            "",
            "## Runtime Cost",
            "",
            "| Rung | Aggregate real-cohort user seconds |",
            "|---|---:|",
            f"| `baseline_fixed10_dt300` | `{fmt(fixed10_user)}` |",
            f"| `dx5_dt300` | `{fmt(dx5_user)}` |",
            f"| `dx5 / fixed10` | `{fmt(cost_ratio)}` |",
            "",
            "Cost is priced evidence only and is not used as a fidelity blocker.",
            "",
            "## Blocking Comparisons",
            "",
        ]
    )
    for role, rows in blocking_roles.items():
        lines.extend(
            [
                f"### {role}",
                "",
                "| Member | Candidate | Reference | Outlet rel | Shape L1 | End storage rel | Tail fold rel | Annual sed rel | Verdict | Failures |",
                "|---|---|---|---:|---:|---:|---:|---:|---|---|",
            ]
        )
        for row in rows:
            lines.append(
                "| {member} | `{candidate}` | `{reference}` | `{outlet}` | `{shape}` | `{storage}` | `{tail}` | `{sed}` | {verdict} | {failures} |".format(
                    member=row["member_id"],
                    candidate=row["candidate"],
                    reference=row["reference"],
                    outlet=fmt(row["terminal_outlet_l1_rel"]),
                    shape=fmt(row["max_shape_l1"]),
                    storage=fmt(row["end_storage_delta_rel_source"]),
                    tail=fmt(row["tail_fold_delta_rel_source"]),
                    sed=fmt(row["annual_sediment_max_rel"]),
                    verdict=row["verdict"],
                    failures="; ".join(row["failures"]) if row["failures"] else "none",
                )
            )
        lines.append("")
    lines.extend(
        [
            "## Report-Only Controls",
            "",
            "None. All same-`dx` timestep controls are gate-class comparisons in "
            "this package.",
            "",
        ]
    )
    for role, rows in report_roles.items():
        lines.extend(
            [
                f"### {role}",
                "",
                "| Member | Candidate | Reference | Outlet rel | Shape L1 | Annual sed rel | Verdict | Failures |",
                "|---|---|---|---:|---:|---:|---|---|",
            ]
        )
        for row in rows:
            lines.append(
                "| {member} | `{candidate}` | `{reference}` | `{outlet}` | `{shape}` | `{sed}` | {verdict} | {failures} |".format(
                    member=row["member_id"],
                    candidate=row["candidate"],
                    reference=row["reference"],
                    outlet=fmt(row["terminal_outlet_l1_rel"]),
                    shape=fmt(row["max_shape_l1"]),
                    sed=fmt(row["annual_sediment_max_rel"]),
                    verdict=row["verdict"],
                    failures="; ".join(row["failures"]) if row["failures"] else "none",
                )
            )
        lines.append("")
    lines.extend(
        [
            "## Blockers",
            "",
        ]
    )
    if blockers:
        for blocker in blockers:
            lines.append(f"- {blocker}")
    else:
        lines.append("- none")
    lines.extend(
        [
            "",
            "Detailed JSON:",
            "",
            f"- `{OUTPUT_JSON}`",
        ]
    )
    OUTPUT_MD.write_text("\n".join(lines) + "\n")
    print(json.dumps({"status": verdict, "ratified": ratified, "blockers": len(blockers)}))


if __name__ == "__main__":
    main()
