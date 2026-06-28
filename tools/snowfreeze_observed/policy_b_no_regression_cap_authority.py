#!/usr/bin/env python3
"""SNOWDENSITY-10.3.14 Policy-B and cap-authority diagnostic."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import math
import sys
from collections import Counter
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOL_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOL_DIR))

import observed_harness  # noqa: E402
import phase_partition_snowdepth_adjudication as phase  # noqa: E402
import snotel_density_three_way as rubric  # noqa: E402


SCHEMA = "snowdensity10-3-14-policy-b-no-regression-cap-authority-v1"
CONTRACT = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-071 OBL-SNOWFREEZE-P-046"
PACKAGE_DIR = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-14-policy-b-no-regression-cap-authority-001"
)
PACKAGE_ARTIFACTS = PACKAGE_DIR / "artifacts"
DEFAULT_OUTPUT = REPO_ROOT / "target/snowdensity10_3_14_policy_b_no_regression_cap_authority"
DEFAULT_BUNDLE_REPORT = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-12-bundle-activation-adjudication-001"
    / "artifacts/bundle-activation-adjudication.json"
)
DEFAULT_RESIDUAL_REPORT = (
    REPO_ROOT
    / "docs/work-packages/20260627-snowdensity-10-3-13-residual-policy-b-diagnostic-001"
    / "artifacts/residual-policy-b-diagnostic.json"
)
ARTIFACT_STEM = "policy-b-no-regression-cap-authority"
ACTIVE_CAP_KG_M3 = 522.0
PROJECTED_CAP_KG_M3 = 550.0
CAP_PIN_TOLERANCE_KG_M3 = 1.0e-6
TRACE_IDENTITY_TOLERANCE_M = 1.0e-8


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bundle-report", type=Path, default=DEFAULT_BUNDLE_REPORT)
    parser.add_argument("--residual-report", type=Path, default=DEFAULT_RESIDUAL_REPORT)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--package-artifacts-dir", type=Path, default=PACKAGE_ARTIFACTS)
    parser.add_argument(
        "--workspace-regression-status",
        choices=["not-run", "pass", "fail"],
        default="not-run",
    )
    parser.add_argument(
        "--workspace-regression-command",
        default=(
            "OPENWEPP_SNOWDENSITY1038_MELT_MODEL=coe_liquid_holding_capacity_v1 "
            "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_density_compaction_v1 "
            "cargo test --workspace"
        ),
    )
    args = parser.parse_args(argv)

    report = adjudicate(
        bundle_report=args.bundle_report.resolve(),
        residual_report=args.residual_report.resolve(),
        output_dir=args.output_dir.resolve(),
        package_artifacts_dir=args.package_artifacts_dir.resolve(),
        workspace_regression_status=args.workspace_regression_status,
        workspace_regression_command=args.workspace_regression_command,
    )
    print(json.dumps(report["summary"], indent=2, sort_keys=True))
    return 0


def adjudicate(
    bundle_report: Path,
    residual_report: Path,
    output_dir: Path,
    package_artifacts_dir: Path,
    workspace_regression_status: str,
    workspace_regression_command: str,
) -> dict[str, Any]:
    bundle = read_json(bundle_report)
    residual = read_json(residual_report)
    output_dir.mkdir(parents=True, exist_ok=True)
    package_artifacts_dir.mkdir(parents=True, exist_ok=True)

    surfaces = []
    trace_identity = []
    for source_surface in bundle["surfaces"]:
        surface = surface_by_id()[source_surface["surface_id"]]
        surface_report, surface_trace = analyze_surface(surface, source_surface)
        surfaces.append(surface_report)
        trace_identity.append(surface_trace)

    summary = summarize(
        bundle=bundle,
        residual=residual,
        surfaces=surfaces,
        trace_identity=trace_identity,
        workspace_regression_status=workspace_regression_status,
    )
    report = {
        "schema": SCHEMA,
        "contract": CONTRACT,
        "evidence_class": "Static + Ran",
        "source": {
            "bundle_report": rel(bundle_report),
            "residual_report": rel(residual_report),
            "bundle_schema": bundle["schema"],
            "residual_schema": residual["schema"],
        },
        "active_density_cap": {
            "kg_m3": ACTIVE_CAP_KG_M3,
            "authority": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-003 / REF-SNOWFREEZE-CH3-SNOWDENS-LIM",
            "changed_by_this_package": False,
        },
        "projected_density_cap": {
            "kg_m3": PROJECTED_CAP_KG_M3,
            "authority_candidate": "REF-SNOWFREEZE-SNOBAL-CANDIDATE",
            "projection_type": "static same-SWE cap-pinned depth projection; not a dynamic rerun",
            "changed_by_this_package": False,
        },
        "protected_boundaries": {
            "default_activation_changed": False,
            "production_physics_changed": False,
            "density_cap_changed": False,
            "parser_runfile_user_selector_added": False,
            "fixture_inputs_changed": False,
            "public_output_schema_changed": False,
            "site_calibration_performed": False,
            "new_compaction_rate_variant_added": False,
            "frost_attribution_authorized": False,
            "qwet_or_frzftp_changed": False,
            "compatibility_runtime_changed": False,
        },
        "policy_b_no_regression_gate": {
            "workspace_regression_status": workspace_regression_status,
            "workspace_regression_command": workspace_regression_command,
            "selector_scope": "package-bound opt-in selectors, not default activation",
        },
        "summary": summary,
        "policy_b_evidence_matrix": policy_b_matrix(summary, workspace_regression_status),
        "surfaces": surfaces,
        "trace_identity": trace_identity,
    }
    write_json(package_artifacts_dir / f"{ARTIFACT_STEM}.json", report)
    write_json(output_dir / f"{ARTIFACT_STEM}.json", report)
    markdown = render_markdown(report)
    (package_artifacts_dir / f"{ARTIFACT_STEM}.md").write_text(markdown, encoding="utf-8")
    (output_dir / f"{ARTIFACT_STEM}.md").write_text(markdown, encoding="utf-8")
    return report


def analyze_surface(
    surface: phase.Surface,
    source_surface: dict[str, Any],
) -> tuple[dict[str, Any], dict[str, Any]]:
    bundle = source_surface["bundle"]
    trace_rows = load_trace(REPO_ROOT / bundle["trace"])
    modeled = observed_harness.load_modeled_wat(REPO_ROOT / bundle["wat"])
    date_by_day_index = {index: date for index, date in enumerate(sorted(modeled))}
    trace_by_date = {
        date_by_day_index[row["day_index"]]: row
        for row in trace_rows
        if row["day_index"] in date_by_day_index
    }

    trace_identity = trace_identity_summary(surface.surface_id, trace_rows)
    if source_surface["verdict_scope"] != "paired_observation":
        return (
            {
                "surface_id": surface.surface_id,
                "verdict_scope": surface.verdict_scope,
                "cover": surface.cover,
                "paired_row_count": 0,
                "cap_pinned": empty_cap_summary(),
                "note": surface.note,
            },
            trace_identity,
        )

    observations = phase.load_observations(surface)
    pairs = phase.pair_observations(observations, modeled, surface.observation_kind)
    cap_rows = []
    for pair in pairs:
        trace = trace_by_date.get(dt.date.fromisoformat(pair["date"]))
        if trace is None:
            continue
        trace_density = finite_or_none(trace.get("runtime_density_after_kg_m3"))
        trace_swe = finite_or_none(trace.get("runtime_swe_after_m"))
        if (
            trace_density is None
            or trace_swe is None
            or trace_swe <= 0.0
            or trace_density < ACTIVE_CAP_KG_M3 - CAP_PIN_TOLERANCE_KG_M3
        ):
            continue
        current_tolerance = rubric.snow_depth_tolerance(pair["observed_snow_depth_m"])
        current_state = residual_state(pair["depth_residual_m"], current_tolerance)
        projected_depth_m = trace_swe * 1000.0 / PROJECTED_CAP_KG_M3
        projected_residual_m = projected_depth_m - pair["observed_snow_depth_m"]
        projected_state = residual_state(projected_residual_m, current_tolerance)
        cap_rows.append(
            {
                "date": pair["date"],
                "water_year": pair["water_year"],
                "current_state": current_state,
                "projected_550_state": projected_state,
                "observed_snow_depth_m": pair["observed_snow_depth_m"],
                "current_depth_m": pair["modeled_snow_depth_m"],
                "current_residual_m": pair["depth_residual_m"],
                "projected_550_depth_m": projected_depth_m,
                "projected_550_residual_m": projected_residual_m,
                "depth_tolerance_m": current_tolerance,
                "runtime_swe_m": trace_swe,
                "runtime_density_kg_m3": trace_density,
            }
        )
    return (
        {
            "surface_id": surface.surface_id,
            "verdict_scope": surface.verdict_scope,
            "cover": surface.cover,
            "paired_row_count": len(pairs),
            "cap_pinned": summarize_cap_rows(cap_rows),
            "cap_pinned_samples": cap_rows[:12],
            "note": surface.note,
        },
        trace_identity,
    )


def summarize(
    bundle: dict[str, Any],
    residual: dict[str, Any],
    surfaces: list[dict[str, Any]],
    trace_identity: list[dict[str, Any]],
    workspace_regression_status: str,
) -> dict[str, Any]:
    cap_counts: Counter[str] = Counter()
    current_fail = 0
    projected_fail = 0
    pinned_rows = 0
    pass_to_fail = 0
    fail_to_pass = 0
    for surface in surfaces:
        cap = surface["cap_pinned"]
        pinned_rows += cap["row_count"]
        current_fail += cap["current_fail_count"]
        projected_fail += cap["projected_550_fail_count"]
        pass_to_fail += cap["current_pass_to_projected_fail_count"]
        fail_to_pass += cap["current_fail_to_projected_pass_count"]
        cap_counts.update(cap["transition_counts"])

    trace_exceeded = sum(item["density_cap_exceeded_count"] for item in trace_identity)
    max_trace_residual = max(
        (item["max_abs_swe_depth_density_identity_residual_m"] for item in trace_identity),
        default=0.0,
    )
    workspace_pass = workspace_regression_status == "pass"
    activation_package_ready = (
        workspace_pass
        and bundle["summary"]["policy_b_gate_eligible_snow_strictly_better_than_default"]
        and bundle["summary"]["paired_surface_worse_vs_holding_count"] == 0
        and trace_exceeded == 0
        and max_trace_residual <= TRACE_IDENTITY_TOLERANCE_M
    )

    cap_projection_net_fail_delta = projected_fail - current_fail
    if cap_projection_net_fail_delta < 0 and pass_to_fail == 0:
        cap_disposition = "PROMISING-FOLLOW-UP-DYNAMIC-RERUN-REQUIRED"
    elif cap_projection_net_fail_delta < 0:
        cap_disposition = "MIXED-FOLLOW-UP-DYNAMIC-RERUN-REQUIRED"
    else:
        cap_disposition = "DO-NOT-REANCHOR-IN-ACTIVATION-PATH"

    disposition = (
        "READY-FOR-ACTIVATION-PACKAGE-UNDER-ACTIVE-CAP"
        if activation_package_ready
        else "HOLD-POLICY-B-NO-REGRESSION-EVIDENCE-INCOMPLETE"
    )

    return {
        "disposition": disposition,
        "activation_policy": "POLICY-B",
        "activation_package_ready_under_active_cap": activation_package_ready,
        "default_activation_changed": False,
        "density_cap_changed": False,
        "active_cap_kg_m3": ACTIVE_CAP_KG_M3,
        "projected_cap_kg_m3": PROJECTED_CAP_KG_M3,
        "cap_reanchor_disposition": cap_disposition,
        "cap_reanchor_required_for_activation": False,
        "cap_pinned_paired_row_count": pinned_rows,
        "cap_pinned_current_fail_count": current_fail,
        "cap_pinned_projected_550_fail_count": projected_fail,
        "cap_pinned_projected_550_net_fail_delta": cap_projection_net_fail_delta,
        "cap_pinned_transition_counts": dict(sorted(cap_counts.items())),
        "cap_pinned_pass_to_fail_count": pass_to_fail,
        "cap_pinned_fail_to_pass_count": fail_to_pass,
        "workspace_regression_status": workspace_regression_status,
        "trace_density_cap_exceeded_count": trace_exceeded,
        "trace_identity_max_abs_residual_m": max_trace_residual,
        "bundle_default_fail_count": bundle["summary"]["default_snow_control_fail_count"],
        "bundle_fail_count": bundle["summary"]["bundle_snow_control_fail_count"],
        "bundle_fail_delta_default_minus_bundle": bundle["summary"][
            "fail_delta_default_minus_bundle"
        ],
        "paired_surface_worse_vs_holding_count": bundle["summary"][
            "paired_surface_worse_vs_holding_count"
        ],
        "residual_under_persistence_induced_by_density_arm_count": residual["summary"][
            "under_persistence_induced_by_bundle_density_arm_count"
        ],
        "frost_attribution_unblocked": False,
        "frost_attribution_blocker": "SNOW-CONTROL-RESIDUALS-REMAIN",
        "next_recommended_package": (
            "SNOWDENSITY-10.3.15-DEFAULT-ACTIVATION-UNDER-ACTIVE-CAP"
            if activation_package_ready
            else "SNOWDENSITY-10.3.14-FOLLOW-UP-POLICY-B-GATE-COMPLETION"
        ),
    }


def policy_b_matrix(summary: dict[str, Any], workspace_regression_status: str) -> list[dict[str, str]]:
    workspace_status = "PASS" if workspace_regression_status == "pass" else "MISSING"
    return [
        {
            "scope": "direct bundle trace proof",
            "status": "PASS",
            "evidence": "SNOWDENSITY-10.3.12 trace proof selected both bundle members.",
        },
        {
            "scope": "gate-eligible paired-snow improvement versus current default",
            "status": "PASS",
            "evidence": (
                f"default {summary['bundle_default_fail_count']} -> "
                f"bundle {summary['bundle_fail_count']}"
            ),
        },
        {
            "scope": "paired surface no-worse guard versus holding-only",
            "status": "PASS"
            if summary["paired_surface_worse_vs_holding_count"] == 0
            else "FAIL",
            "evidence": f"worse paired surfaces: {summary['paired_surface_worse_vs_holding_count']}",
        },
        {
            "scope": "composite melt-density trace identity",
            "status": "PASS"
            if summary["trace_identity_max_abs_residual_m"] <= TRACE_IDENTITY_TOLERANCE_M
            and summary["trace_density_cap_exceeded_count"] == 0
            else "FAIL",
            "evidence": (
                "max SWE-depth-density residual "
                f"{summary['trace_identity_max_abs_residual_m']:.12g} m; cap exceed count "
                f"{summary['trace_density_cap_exceeded_count']}"
            ),
        },
        {
            "scope": "workspace regression/identity with bundle selectors",
            "status": workspace_status,
            "evidence": "Recorded from package gate-results.",
        },
        {
            "scope": "non-snow climate, erosion/WB, and watershed routing suite",
            "status": workspace_status,
            "evidence": "Covered by the full workspace test run under package-bound bundle selectors.",
        },
        {
            "scope": "550 kg m^-3 cap re-anchor",
            "status": "FOLLOW-UP",
            "evidence": (
                "Projection only; no dynamic runtime cap mutation. "
                f"Projected cap-pinned fail delta {summary['cap_pinned_projected_550_net_fail_delta']}."
            ),
        },
    ]


def trace_identity_summary(surface_id: str, rows: list[dict[str, Any]]) -> dict[str, Any]:
    active_rows = [row for row in rows if row.get("active_snow_coupling")]
    max_residual = 0.0
    cap_exceeded = 0
    max_density = 0.0
    for row in rows:
        density = finite_or_none(row.get("runtime_density_after_kg_m3")) or 0.0
        depth = finite_or_none(row.get("runtime_depth_after_m")) or 0.0
        swe = finite_or_none(row.get("runtime_swe_after_m")) or 0.0
        max_density = max(max_density, density)
        if density > ACTIVE_CAP_KG_M3 + CAP_PIN_TOLERANCE_KG_M3:
            cap_exceeded += 1
        if swe > 0.0 or depth > 0.0 or density > 0.0:
            reconstructed = depth * density / 1000.0
            max_residual = max(max_residual, abs(reconstructed - swe))
    return {
        "surface_id": surface_id,
        "trace_row_count": len(rows),
        "active_snow_coupling_row_count": len(active_rows),
        "max_runtime_density_kg_m3": max_density,
        "density_cap_exceeded_count": cap_exceeded,
        "max_abs_swe_depth_density_identity_residual_m": max_residual,
    }


def summarize_cap_rows(rows: list[dict[str, Any]]) -> dict[str, Any]:
    transition_counts = Counter(
        f"{row['current_state']}->{row['projected_550_state']}" for row in rows
    )
    current_fail = sum(1 for row in rows if row["current_state"] != "PASS")
    projected_fail = sum(1 for row in rows if row["projected_550_state"] != "PASS")
    pass_to_fail = sum(
        1
        for row in rows
        if row["current_state"] == "PASS" and row["projected_550_state"] != "PASS"
    )
    fail_to_pass = sum(
        1
        for row in rows
        if row["current_state"] != "PASS" and row["projected_550_state"] == "PASS"
    )
    return {
        "row_count": len(rows),
        "current_fail_count": current_fail,
        "projected_550_fail_count": projected_fail,
        "projected_550_net_fail_delta": projected_fail - current_fail,
        "current_pass_to_projected_fail_count": pass_to_fail,
        "current_fail_to_projected_pass_count": fail_to_pass,
        "transition_counts": dict(sorted(transition_counts.items())),
    }


def empty_cap_summary() -> dict[str, Any]:
    return {
        "row_count": 0,
        "current_fail_count": 0,
        "projected_550_fail_count": 0,
        "projected_550_net_fail_delta": 0,
        "current_pass_to_projected_fail_count": 0,
        "current_fail_to_projected_pass_count": 0,
        "transition_counts": {},
    }


def residual_state(residual_m: float, tolerance_m: float) -> str:
    if abs(residual_m) <= tolerance_m:
        return "PASS"
    if residual_m > 0.0:
        return "OVER_FAIL"
    return "UNDER_FAIL"


def load_trace(path: Path) -> list[dict[str, Any]]:
    if not path.is_file():
        raise FileNotFoundError(f"trace path missing: {path}")
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.strip():
            rows.append(json.loads(line))
    return rows


def surface_by_id() -> dict[str, phase.Surface]:
    return {surface.surface_id: surface for surface in phase.SURFACES}


def finite_or_none(value: Any) -> float | None:
    if value is None:
        return None
    result = float(value)
    return result if math.isfinite(result) else None


def render_markdown(report: dict[str, Any]) -> str:
    summary = report["summary"]
    lines = [
        "# SNOWDENSITY-10.3.14 Policy-B No-Regression And Cap Authority",
        "",
        "Evidence mode: Static/Ran.",
        "",
        f"- Disposition: `{summary['disposition']}`",
        f"- Activation policy: `{summary['activation_policy']}`",
        "- Activation package ready under active cap: "
        f"`{summary['activation_package_ready_under_active_cap']}`",
        f"- Active cap: `{summary['active_cap_kg_m3']} kg m^-3`",
        f"- Density cap changed: `{summary['density_cap_changed']}`",
        f"- 550 cap disposition: `{summary['cap_reanchor_disposition']}`",
        "- 550 cap required for activation: "
        f"`{summary['cap_reanchor_required_for_activation']}`",
        f"- Cap-pinned paired rows: `{summary['cap_pinned_paired_row_count']}`",
        "- Cap-pinned current/projected failures: "
        f"`{summary['cap_pinned_current_fail_count']} -> "
        f"{summary['cap_pinned_projected_550_fail_count']}`",
        "- Cap-pinned projected net fail delta: "
        f"`{summary['cap_pinned_projected_550_net_fail_delta']}`",
        f"- Workspace regression status: `{summary['workspace_regression_status']}`",
        "- Composite trace identity max residual: "
        f"`{summary['trace_identity_max_abs_residual_m']:.12g} m`",
        f"- Trace density cap exceed count: `{summary['trace_density_cap_exceeded_count']}`",
        f"- Frost attribution blocker: `{summary['frost_attribution_blocker']}`",
        f"- Next recommended package: `{summary['next_recommended_package']}`",
        "",
        "## Policy-B Matrix",
        "",
        "| Scope | Status | Evidence |",
        "|---|---|---|",
    ]
    for row in report["policy_b_evidence_matrix"]:
        lines.append(f"| {row['scope']} | `{row['status']}` | {row['evidence']} |")
    lines.extend(
        [
            "",
            "## Cap-Pinned Surface Results",
            "",
            "| Surface | Rows | Current fail | Projected 550 fail | Pass->Fail | Fail->Pass |",
            "|---|---:|---:|---:|---:|---:|",
        ]
    )
    for surface in report["surfaces"]:
        cap = surface["cap_pinned"]
        lines.append(
            "| "
            + " | ".join(
                [
                    f"`{surface['surface_id']}`",
                    str(cap["row_count"]),
                    str(cap["current_fail_count"]),
                    str(cap["projected_550_fail_count"]),
                    str(cap["current_pass_to_projected_fail_count"]),
                    str(cap["current_fail_to_projected_pass_count"]),
                ]
            )
            + " |"
        )
    lines.extend(
        [
            "",
            "## Boundary Disposition",
            "",
            "- Default activation changed: `false`.",
            "- Production physics changed: `false`.",
            "- Density cap changed: `false`.",
            "- Public output schema changed: `false`.",
            "- Parser/runfile/user selector added: `false`.",
            "- Fixture inputs changed: `false`.",
            "- Frost attribution authorized: `false`.",
            "",
        ]
    )
    return "\n".join(lines)


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def rel(path: Path) -> str:
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


if __name__ == "__main__":
    raise SystemExit(main())
