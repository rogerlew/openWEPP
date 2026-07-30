#!/usr/bin/env python3
"""Generate deterministic machine-readable and figure artifacts for EB-01."""

from __future__ import annotations

import argparse
import csv
import hashlib
import io
import json
from pathlib import Path
from xml.sax.saxutils import escape

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402


ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"


def csv_bytes(header: list[str], rows: list[list[object]]) -> bytes:
    stream = io.StringIO(newline="")
    writer = csv.writer(stream, lineterminator="\n")
    writer.writerow(header)
    writer.writerows(rows)
    return stream.getvalue().encode()


def write_or_check(path: Path, content: bytes, check: bool) -> None:
    if check:
        if not path.exists() or path.read_bytes() != content:
            raise SystemExit(f"stale generated artifact: {path.relative_to(ROOT)}")
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(content)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stratum_stats(path: Path, stratum: str) -> dict[str, object]:
    with path.open(newline="") as stream:
        rows = [row for row in csv.DictReader(stream) if row["observed_stratum"] == stratum]
    bindings = {row["binding_status"] for row in rows}
    fixtures = {row["model_fixture"] for row in rows}
    assert len(bindings) == 1 and len(fixtures) == 1
    return {
        "total": len(rows),
        "start": min(row["date"] for row in rows),
        "end": max(row["date"] for row in rows),
        "depth": sum(bool(row["observed_snow_depth_m"]) for row in rows),
        "swe": sum(bool(row["observed_swe_mm"]) for row in rows),
        "density": sum(bool(row["observed_density_kg_m3"]) for row in rows),
        "binding": next(iter(bindings)),
        "fixture": next(iter(fixtures)),
    }


def column_stats(path: Path, columns: list[str]) -> dict[str, object]:
    with path.open(newline="") as stream:
        rows = list(csv.DictReader(stream))
    return {
        "total": len(rows),
        "start": min(row["date"] for row in rows),
        "end": max(row["date"] for row in rows),
        **{column: sum(bool(row[column]) for row in rows) for column in columns},
    }


def save_svg(fig: plt.Figure, path: Path, title: str, desc: str, check: bool) -> None:
    stream = io.BytesIO()
    fig.savefig(
        stream,
        format="svg",
        bbox_inches="tight",
        metadata={"Title": title, "Description": desc, "Date": None},
    )
    plt.close(fig)
    text = stream.getvalue().decode()
    text = text.replace("<svg ", '<svg role="img" ')
    title_end = text.index("</title>") + len("</title>")
    text = text[:title_end] + f"\n <desc>{escape(desc)}</desc>" + text[title_end:]
    write_or_check(path, text.encode(), check)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    plt.rcParams.update(
        {
            "svg.hashsalt": "snow-surface-eb-01",
            "font.family": "DejaVu Sans",
            "font.size": 10,
            "axes.axisbelow": True,
        }
    )

    deps = [
        "crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs",
        "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs",
        "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs",
        "crates/openwepp-meteorology/src/surface_energy.rs",
        "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs",
        "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
        "docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001/artifacts/sublimation-stage-b-unlock.json",
        "tests/fixtures/cancov_forest/observations/manifest.json",
        "tests/fixtures/cancov_forest/observations/sites/marcell_rds_2021_0016_stratum_means.csv",
        "tests/fixtures/cancov_forest/observations/sites/harvard_hf237_strata.csv",
        "tests/fixtures/snotel_observed/observations/manifest.json",
        "tests/fixtures/snotel_observed/observations/sites/snotel_mica_creek_st_joe_id.csv",
        "tests/fixtures/snotel_observed/observations/sites/snotel_paradise_wa.csv",
        "tests/fixtures/snotel_observed/observations/sites/snotel_css_lab_ca.csv",
        "tests/fixtures/snotel_observed/observations/sites/snotel_snowbird_ut.csv",
        "tests/fixtures/snotel_observed/observations/sites/snotel_niwot_co.csv",
        "tests/fixtures/snowfreeze_observed/observations/sites/site1_sleepers_south_field_vt.csv",
        "tests/fixtures/snowfreeze_observed/observations/sites/site2_sleepers_w9_hardwood_vt.csv",
        "references/copyrighted/lundquist2013.pdf",
        "references/copyrighted/marks1999.pdf",
        "references/copyrighted/source_pdfs/marks1998.pdf",
    ]
    dep_rows = []
    for rel in deps:
        path = ROOT / rel
        dep_rows.append([rel, path.stat().st_size, sha256(path), "read_only_input"])
    write_or_check(
        ARTIFACTS / "dependency-manifest.csv",
        csv_bytes(["path", "bytes", "sha256", "role"], dep_rows),
        args.check,
    )

    cancov_manifest = json.loads((ROOT / "tests/fixtures/cancov_forest/observations/manifest.json").read_text())
    assert cancov_manifest["schema"] == "cancov-stratified-observations-manifest-v1"
    assert cancov_manifest["normal_depth_units"] == "m"
    assert cancov_manifest["normal_swe_units"] == "mm water equivalent"
    assert cancov_manifest["normal_density_units"] == "kg m^-3"
    for entry in cancov_manifest["output_files"]:
        source = ROOT / entry["path"]
        assert source.stat().st_size == entry["bytes"] and sha256(source) == entry["sha256"]

    implementation = [
        ["absorbed_shortwave", "implemented_opt_in", "W m^-2", "Opt-in Stage 3 surface sum", "AUTHORITATIVE_CURRENT", "Available operand; production default has Stage 3 disabled"],
        ["atmospheric_longwave", "helper_only", "W m^-2", "No snow runtime consumer", "AUTHORITY_MISSING", "No admitted incoming forcing/runtime operand"],
        ["canopy_longwave", "absent", "W m^-2", "None", "AUTHORITY_MISSING", "No admitted view-factor/emission formulation"],
        ["sensible_heat", "helper_only", "W m^-2", "Stage 3 passes zero", "AUTHORITY_ADMISSIBLE", "Typed turbulent helper exists"],
        ["latent_heat", "missing_runtime", "W m^-2", "None; Stage A/B produces mass only", "AUTHORITY_ADMISSIBLE", "No latent-energy flux is produced or debited"],
        ["vapor_mass", "opt_in_candidate", "m SWE step^-1", "Stage A/B mass path", "REJECTED_PRIOR", "Mass removal is bounded and conserved; candidate not promoted"],
        ["ground_conduction", "partial", "W m^-2", "Interlayer only", "AUTHORITY_ADMISSIBLE", "No explicit surface-ground term in Stage 3 sum"],
        ["advected_precipitation_heat", "helper_only", "W m^-2", "Stage 3 passes zero", "AUTHORITATIVE_CURRENT", "Typed helper exists"],
        ["surface_temperature", "candidate_approximation", "degC", "Stage A/B", "AUTHORITY_ADMISSIBLE", "Stage A fixes 0 C; Stage B uses min(Tair,0)"],
        ["cold_content", "implemented_opt_in", "J m^-2", "Stage 3 multilayer", "AUTHORITATIVE_CURRENT", "Internal diagnostic closure retained"],
        ["melt_refreeze_liquid", "implemented", "m SWE step^-1", "Bulk and Stage 3", "AUTHORITATIVE_CURRENT", "Current default liquid holding remains active"],
        ["canopy_projection", "implemented", "fraction", "Snow/canopy runtime", "AUTHORITATIVE_CURRENT", "Seasonal projection available"],
    ]
    write_or_check(
        ARTIFACTS / "current-implementation-ledger.csv",
        csv_bytes(["operand", "implementation_state", "units", "consumer", "classification", "finding"], implementation),
        args.check,
    )

    authority = [
        ["incoming_atmospheric_longwave", "AUTHORITY_MISSING", "EB-02", "Forcing source or admitted estimator with uncertainty", "HOLD_FOR_AUTHORITY"],
        ["canopy_sky_view_partition", "AUTHORITY_MISSING", "EB-02", "Authoritative sky-view/canopy-view composition", "HOLD_FOR_AUTHORITY"],
        ["canopy_emissivity_temperature", "AUTHORITY_MISSING", "EB-02", "Emissivity and canopy radiometric-temperature rule", "HOLD_FOR_AUTHORITY"],
        ["net_longwave_arithmetic", "AUTHORITATIVE_CURRENT", "EB-02", "Typed shared helper", "READY"],
        ["turbulent_vapor_transfer", "AUTHORITY_ADMISSIBLE", "EB-03", "Marks/SNOBAL lineage plus contract amendment", "READY_FOR_CONTRACT_WORK"],
        ["latent_mass_equivalence", "AUTHORITATIVE_CURRENT", "EB-03", "Shared conversions and exact-one coupling invariant", "READY_FOR_CONTRACT_WORK"],
        ["surface_temperature_state", "AUTHORITY_ADMISSIBLE", "EB-03", "One shared temperature/cold-content state", "READY_FOR_CONTRACT_WORK"],
        ["warm_maritime_conifer_validation", "AUTHORITY_MISSING", "EB-04", "Paired under-canopy/open observations and meteorology", "HOLD_FOR_DATA"],
    ]
    write_or_check(
        ARTIFACTS / "authority-gap-ledger.csv",
        csv_bytes(["item", "classification", "owner", "closure_evidence", "disposition"], authority),
        args.check,
    )

    selectors = [
        ["phase_model", "independent", "Harder-Pomeroy hourly", "legacy options retained", "Freeze across B/L/S/LS"],
        ["density_model", "independent", "bulk compaction", "legacy/options retained", "Freeze across B/L/S/LS"],
        ["liquid_model", "entangled_with_sublimation", "CoeLiquidHoldingCapacityV1", "Stage A/B melt-enum variants", "Split sublimation into independent typed selector"],
        ["longwave_model", "missing", "off", "none", "Add independent typed selector only after authority admission"],
        ["sublimation_model", "entangled_with_melt_enum", "off", "Stage A/B nonpromoted variants", "Move behind independent typed selector; no default change"],
        ["surface_energy_carrier", "partial_opt_in", "bulk CoE path; Stage 3 disabled", "shortwave-only Stage 3", "Prospectively choose one identical carrier for all cells; do not relabel opt-in as baseline"],
        ["canopy_projection", "independent", "current seasonal projection", "same projection available to opt-ins", "Freeze input trace across cells"],
    ]
    write_or_check(
        ARTIFACTS / "selector-composition-ledger.csv",
        csv_bytes(["selector", "state", "production_default", "available_opt_in", "factorial_requirement"], selectors),
        args.check,
    )

    result_path = ROOT / "docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001/artifacts/sublimation-stage-b-unlock.json"
    prior = json.loads(result_path.read_text())
    assert prior["schema"] == "snowdensity10-3-20-sublimation-stage-b-unlock-v1"
    aggregates = prior["model_summaries"]
    stage_a_trace = prior["candidate_disposition"]["partition_sublimation_stage_a"]["trace_sublimation"]
    stage_b_trace = prior["candidate_disposition"]["stage_b_surface_layer"]["trace_sublimation"]
    candidates = [
        ["current_default", aggregates["activated_bundle"]["aggregate"]["robust_fail_count"], aggregates["activated_bundle"]["aggregate"]["robust_ordinal_score"], "", "", "RETAINED_DEFAULT", "Reference; no candidate promotion"],
        ["partition_sublimation_stage_a", aggregates["partition_sublimation_stage_a"]["aggregate"]["robust_fail_count"], aggregates["partition_sublimation_stage_a"]["aggregate"]["robust_ordinal_score"], stage_a_trace["total_sublimation_m"], stage_a_trace["max_daily_sublimation_m"], "REJECTED_PRIOR", "Worse robust score; eight worse robust cells"],
        ["stage_a_legacy_phase_10_3_16", aggregates["stage_a_legacy_phase_10_3_16"]["aggregate"]["robust_fail_count"], aggregates["stage_a_legacy_phase_10_3_16"]["aggregate"]["robust_ordinal_score"], "", "", "REJECTED_PRIOR", "Worse robust score"],
        ["stage_b_surface_layer", aggregates["stage_b_surface_layer"]["aggregate"]["robust_fail_count"], aggregates["stage_b_surface_layer"]["aggregate"]["robust_ordinal_score"], stage_b_trace["total_sublimation_m"], stage_b_trace["max_daily_sublimation_m"], "REJECTED_PRIOR", "Conserved mass but scored one point below default"],
    ]
    assert prior["summary"]["stage_b_robust_fail_count"] == 15
    write_or_check(
        ARTIFACTS / "prior-candidate-disposition.csv",
        csv_bytes(
            ["candidate", "robust_fail_count", "robust_ordinal_score", "cohort_sublimation_m", "max_daily_sublimation_m", "disposition", "basis"],
            candidates,
        ),
        args.check,
    )

    marcell_path = ROOT / "tests/fixtures/cancov_forest/observations/sites/marcell_rds_2021_0016_stratum_means.csv"
    harvard_path = ROOT / "tests/fixtures/cancov_forest/observations/sites/harvard_hf237_strata.csv"
    mstats = {s: stratum_stats(marcell_path, s) for s in ["conifer", "deciduous", "open"]}
    hstats = {s: stratum_stats(harvard_path, s) for s in ["hardwood", "hemlock", "open"]}
    snotel = json.loads((ROOT / "tests/fixtures/snotel_observed/observations/manifest.json").read_text())
    assert snotel["schema"] == "snotel-observed-manifest-v1"
    assert snotel["normal_depth_units"] == "m"
    assert snotel["normal_swe_units"] == "mm water equivalent"
    assert snotel["normal_density_units"] == "kg m^-3"
    cancov_periods = {site["site_id"]: (site["start_date"], site["end_date"]) for site in cancov_manifest["sites"]}
    assert (min(v["start"] for v in mstats.values()), max(v["end"] for v in mstats.values())) == cancov_periods["marcell_rds_2021_0016_stratum_means"]
    assert (min(v["start"] for v in hstats.values()), max(v["end"] for v in hstats.values())) == cancov_periods["harvard_hf237"]
    assert {mstats[s]["binding"] for s in mstats} == {"bound"}
    assert hstats["hardwood"]["binding"] == "bound" and hstats["open"]["binding"] == "bound"
    assert hstats["hemlock"]["binding"] == "unbound_no_pure_conifer_fixture"

    def measured_row(
        source_id: str,
        custody: str,
        site: str,
        location: str,
        climate: str,
        stratum: str,
        fixture: str,
        binding: str,
        stats: dict[str, object],
        resolution: str,
        operator: str,
        role: str,
        discriminates: str,
        limitation: str,
    ) -> list[object]:
        return [
            source_id, custody, site, location, climate, f"{stats['start']}..{stats['end']}",
            resolution, stratum, fixture, binding, "depth:m; SWE:mm; density:kg m^-3",
            "fixture climate forcing; observation-coincidence uncertainty retained", operator,
            stats["total"], stats["depth"], stats["swe"], stats["density"], role,
            discriminates, limitation,
        ]

    obs = [
        measured_row("marcell_rds_2021_0016", "USDA FS RDS DOI 10.2737/RDS-2021-0016", "Marcell", "Minnesota, USA", "cold_continental", s, f"marcell_{'deciduous' if s == 'deciduous' else s}_mn", "bound", mstats[s], "biweekly/irregular snow course", "exact-date modeled stratum state; canopy-minus-open contrast", "INDEPENDENT_VALIDATION", "both" if s != "open" else "control", "Spatial snow-course mean; forcing limited")
        for s in ["conifer", "deciduous", "open"]
    ]
    obs += [
        measured_row("harvard_hf237", "Harvard Forest HF237 DOI 10.6073/pasta/be69b1f46b57354a25d85a437c0679c8; CC0", "Harvard", "Massachusetts, USA", "humid_continental", s, "" if s == "hemlock" else f"harvard_{'deciduous' if s == 'hardwood' else s}_ma", "unbound" if s == "hemlock" else "bound", hstats[s], "daily rows; measurements intermittently non-null", "exact-date modeled stratum state; canopy-minus-open contrast", "DIAGNOSTIC_ONLY" if s == "hemlock" else "INDEPENDENT_VALIDATION", "both" if s != "open" else "control", "SWE uses observed/interpolated density; hemlock has no pure-conifer fixture" if s == "hemlock" else "Intermittent non-null observations; forcing limited")
        for s in ["hardwood", "hemlock", "open"]
    ]
    for site in snotel["sites"]:
        site_path = ROOT / "tests/fixtures/snotel_observed/observations" / site["observation_file"]
        stats = column_stats(site_path, ["observed_snow_depth_m", "observed_swe_mm", "observed_density_kg_m3"])
        obs.append([
            f"nrcs_{site['station_triplet'].replace(':', '_').lower()}", "USDA NRCS AWDB public station record",
            site["site_id"], site["station_triplet"], site["snow_climate"],
            f"{stats['start']}..{stats['end']}", "daily", "open", site["fixture"], "bound",
            "depth:m; SWE:mm; density:kg m^-3", "fixture climate forcing; station/forcing representativeness retained",
            "same-date modeled/observed state and persistence signatures", stats["total"],
            stats["observed_snow_depth_m"], stats["observed_swe_mm"], stats["observed_density_kg_m3"],
            "INDEPENDENT_VALIDATION", "sublimation", "Open control; cannot identify canopy longwave",
        ])
    sleepers1 = column_stats(ROOT / "tests/fixtures/snowfreeze_observed/observations/sites/site1_sleepers_south_field_vt.csv", ["observed_snow_depth_m"])
    sleepers2 = column_stats(ROOT / "tests/fixtures/snowfreeze_observed/observations/sites/site2_sleepers_w9_hardwood_vt.csv", ["observed_snow_depth_m"])
    obs += [
        ["usgs_sleepers_p96753gi", "USGS DOI 10.5066/P96753GI", "Sleepers South Field", "Vermont, USA", "humid_cold", f"{sleepers1['start']}..{sleepers1['end']}", "field-visit observations", "open", "site1_sleepers_south_field_vt", "bound", "depth:m; frost depth:m", "fixture forcing; record alignment uncertainty", "same-date snow-depth/frost signatures", sleepers1["total"], sleepers1["observed_snow_depth_m"], 0, 0, "DIAGNOSTIC_ONLY", "downstream", "No SWE"],
        ["usgs_sleepers_p96753gi", "USGS DOI 10.5066/P96753GI", "Sleepers W9", "Vermont, USA", "humid_cold", f"{sleepers2['start']}..{sleepers2['end']}", "field-visit observations", "hardwood", "site2_sleepers_w9_hardwood_vt", "bound", "depth:m; frost depth:m", "fixture forcing; record alignment uncertainty", "same-date snow-depth/frost signatures", sleepers2["total"], sleepers2["observed_snow_depth_m"], 0, 0, "DIAGNOSTIC_ONLY", "downstream", "No SWE; source periods are not fully coincident"],
        ["not_installed", "none", "HJ Andrews", "Oregon, USA", "warm_maritime", "missing", "missing", "conifer", "", "missing", "needed: depth:m and/or SWE:mm", "needed: observation-coincident meteorology", "paired canopy/open time series", 0, 0, 0, 0, "DIAGNOSTIC_ONLY", "both", "Required for decisive warm-maritime conifer transfer claim"],
        ["not_installed", "none", "Hubbard Brook", "New Hampshire, USA", "humid_cold", "missing", "missing", "hardwood", "", "missing", "needed: depth:m and/or SWE:mm", "needed: observation-coincident meteorology", "paired canopy/open time series", 0, 0, 0, 0, "DIAGNOSTIC_ONLY", "both", "No installed normalized observation"],
    ]
    obs_header = ["source_id", "custody_identity", "site", "location", "climate", "period", "temporal_resolution", "observed_stratum", "model_fixture", "binding", "observed_quantities_units", "forcing_source_uncertainty", "comparison_operator", "total_rows", "depth_rows", "swe_rows", "density_rows", "role", "discriminates", "limitation"]
    write_or_check(ARTIFACTS / "observation-fixture-ledger.csv", csv_bytes(obs_header, obs), args.check)
    role_rows = [[r[2], r[7], r[17], "frozen_before_result_execution", r[19]] for r in obs]
    write_or_check(
        ARTIFACTS / "observation-role-freeze.csv",
        csv_bytes(["site", "stratum", "role", "freeze_basis", "limitation"], role_rows),
        args.check,
    )

    cells = [
        ["B", "off", "off", "reference", "All frozen shared settings"],
        ["L", "on", "off", "longwave_effect=Y(L)-Y(B)", "Requires EB-02 authority and orthogonal selector"],
        ["S", "off", "on", "sublimation_effect=Y(S)-Y(B)", "Requires EB-03 exact-one mass/energy coupling"],
        ["LS", "on", "on", "combined_effect=Y(LS)-Y(B); interaction=Y(LS)-Y(L)-Y(S)+Y(B)", "Requires both mechanisms on one carrier"],
    ]
    write_or_check(
        ARTIFACTS / "factorial-cells.csv",
        csv_bytes(["cell", "longwave", "sublimation", "estimand", "admission_condition"], cells),
        args.check,
    )

    responses = [
        ["surface_energy_component", "W m^-2", "direct signed hourly step-mean component", "B/L/S/LS", "energy"],
        ["vapor_mass_exchange", "kg m^-2", "direct signed amount integrated over hourly step: positive deposition / negative sublimation; loss-positive view is max(0,-exchange)", "S/LS", "mass"],
        ["swe", "mm", "same-time modeled minus observed", "all", "state"],
        ["snow_depth", "m", "same-time modeled minus observed", "all", "state"],
        ["cold_content", "J m^-2", "direct state and daily change", "all", "energy"],
        ["melt_refreeze", "kg m^-2", "direct amounts integrated over hourly step", "all", "mass_energy"],
        ["retained_routed_liquid", "kg m^-2", "direct amounts integrated over hourly step and retained state", "all", "mass"],
        ["snow_disappearance", "day of water year", "first snow-free day; threshold/window authority owned by EB-03/04 and blocks EB-04 until frozen", "all", "timing"],
        ["runoff_timing", "day of water year", "centroid and peak within identical water-year snow-season window frozen before results", "all", "downstream"],
        ["frost_depth", "m", "same-date paired difference where supported", "all", "downstream"],
    ]
    write_or_check(
        ARTIFACTS / "response-operator-ledger.csv",
        csv_bytes(["response", "units", "operator", "cells", "ledger"], responses),
        args.check,
    )

    operands = [
        ["total_water_storage_before", "kg m^-2", "state", "whole snowpack over one ground-area column", "1 m2 snow column per 1 m2 ground", "step start", "snow ice + retained liquid state", "independent mass reconstruction", "successor diagnostic", "diagnostic"],
        ["solid_input_to_pack", "kg m^-2", "+", "same", "same", "amount integrated over exact step", "phase-partitioned forcing", "snow state", "current contract", "diagnostic"],
        ["liquid_input_to_pack", "kg m^-2", "+", "same", "same", "amount integrated over exact step", "rain admitted to pack after bypass", "snow liquid state", "current contract", "diagnostic"],
        ["vapor_mass_exchange", "kg m^-2", "+ deposition / - sublimation", "same", "same", "amount integrated over exact step", "same turbulent exchange as latent_heat", "snow mass state", "EB-03 contract required", "diagnostic"],
        ["liquid_outflow", "kg m^-2", "-", "same", "same", "amount integrated over exact step", "liquid routing", "runoff/infiltration", "current contract", "public+diagnostic"],
        ["total_water_storage_after", "kg m^-2", "state", "same", "same", "step end after vapor and liquid routing", "snow ice + retained liquid state", "independent mass reconstruction", "successor diagnostic", "diagnostic"],
        ["step_duration", "s", "multiplier", "surface/phase solve", "same", "exact runtime step", "runtime clock", "all W m^-2 operands", "current runtime", "diagnostic"],
        ["thermal_energy_before", "J m^-2", "state", "surface/phase solve before routing", "same", "step start", "signed internal thermal energy relative to admitted 0 C ice reference", "independent energy reconstruction", "EB-03 contract required", "diagnostic"],
        ["net_shortwave", "W m^-2", "+ toward snow", "same", "same", "step mean", "radiation", "surface energy sum", "current contract", "diagnostic"],
        ["net_longwave", "W m^-2", "+ toward snow", "same", "same", "step mean", "future canopy/sky radiation", "surface energy sum", "EB-02 authority required", "diagnostic"],
        ["sensible_heat", "W m^-2", "+ toward snow", "same", "same", "step mean", "turbulent exchange", "surface energy sum", "admissible helper", "diagnostic"],
        ["latent_heat", "W m^-2", "+ toward snow", "same", "same", "step mean", "same turbulent exchange as vapor_mass_exchange", "surface energy sum", "EB-03 contract required", "diagnostic"],
        ["ground_conduction", "W m^-2", "+ toward snow", "same", "same", "step mean", "snow/ground exchange", "surface energy sum", "admissible helper", "diagnostic"],
        ["advected_precipitation_heat", "W m^-2", "+ toward snow", "same", "same", "step mean", "precipitation", "surface energy sum", "authoritative helper", "diagnostic"],
        ["thermal_energy_after", "J m^-2", "state", "same", "same", "after warming/cooling before liquid routing", "same reference as thermal_energy_before", "independent energy reconstruction", "EB-03 contract required", "diagnostic"],
        ["net_phase_change_mass", "kg m^-2", "+ melt / - refreeze", "same", "same", "amount integrated over surface/phase solve step", "melt/refreeze operator", "snow phase state", "current contract", "diagnostic"],
        ["latent_heat_fusion", "J kg^-1", "multiplier", "same", "same", "constant for step", "canonical physical constant", "independent energy reconstruction", "current contract", "diagnostic"],
        ["latent_heat_exchange", "J kg^-1", "multiplier", "same", "same", "temperature/phase appropriate", "shared meteorology conversion", "latent/mass equivalence reconstruction", "EB-03 contract required", "diagnostic"],
    ]
    source_paths = {
        "total_water_storage_before": "EB-03 successor contract/runtime operand",
        "solid_input_to_pack": "SC-SNOWFREEZE-001; infiltration_reconciliation.rs",
        "liquid_input_to_pack": "SC-SNOWFREEZE-001; infiltration_reconciliation.rs",
        "vapor_mass_exchange": "surface_energy.rs; EB-03 successor contract/runtime operand",
        "liquid_outflow": "SC-SNOWFREEZE-001; infiltration_reconciliation.rs",
        "total_water_storage_after": "EB-03 successor contract/runtime operand",
        "step_duration": "runner hourly runtime clock",
        "thermal_energy_before": "SC-SNOWFREEZE-001 cold-content state; EB-03 successor operand",
        "net_shortwave": "runoff_reconciliation.rs; surface_energy.rs",
        "net_longwave": "surface_energy.rs; EB-02 successor authority/runtime operand",
        "sensible_heat": "surface_energy.rs",
        "latent_heat": "surface_energy.rs; EB-03 successor runtime operand",
        "ground_conduction": "surface_energy.rs",
        "advected_precipitation_heat": "surface_energy.rs",
        "thermal_energy_after": "SC-SNOWFREEZE-001 cold-content state; EB-03 successor operand",
        "net_phase_change_mass": "SC-SNOWFREEZE-001; runoff_reconciliation.rs",
        "latent_heat_fusion": "openwepp-meteorology physical constants",
        "latent_heat_exchange": "surface_energy.rs::latent_heat_for_surface_temperature",
    }
    operands = [row[:-1] + [source_paths[str(row[0])], row[-1]] for row in operands]
    write_or_check(
        ARTIFACTS / "mass-energy-operand-lineage.csv",
        csv_bytes(["operand", "units", "sign", "control_volume", "area_basis", "time_basis", "producer", "consumer", "authority", "source_path", "visibility"], operands),
        args.check,
    )

    decisions = [
        ["trace_identity", "hard_gate", "all frozen non-target inputs byte/equality identical across cells", "exact", "any mismatch", "REJECT_CELL_SET", "EB-04"],
        ["mass_closure", "hard_gate", "independent residual from named raw operands", "tolerance must be admitted by EB-03 before execution", "outside tolerance", "REJECT_CANDIDATE", "EB-03"],
        ["energy_closure", "hard_gate", "independent residual from named raw operands", "tolerance must be admitted by EB-03 before execution", "outside tolerance", "REJECT_CANDIDATE", "EB-03"],
        ["latent_mass_equivalence", "hard_gate", "Qlatent*dt equals L_exchange*vapor_mass_exchange", "tolerance must be admitted by EB-03 before execution", "outside tolerance or wrong sign", "REJECT_CANDIDATE", "EB-03"],
        ["snow_disappearance", "operator", "first snow-free day followed by persistence window", "snow-free threshold and window must be admitted before EB-04", "operator unresolved", "HOLD_EB04", "EB-03/EB-04"],
        ["runoff_timing", "operator", "centroid and peak within common water-year snow season", "window frozen before results", "window unresolved", "HOLD_EB04", "EB-04"],
        ["primary_scientific_improvement", "decision", "forcing-robust rubric cells, direct operands first", "accepted SC-SNOWFREEZE rubric plus prospectively frozen tie rule", "no net improvement or tradeoff", "NONPROMOTION", "EB-04"],
        ["protected_lanes", "decision", "current default, open controls, canopy strata, conservation", "no worse forcing-robust failure count and no hard-gate failure", "regression", "NONPROMOTION", "EB-04"],
        ["warm_maritime_conifer", "claim_limit", "HJ Andrews or equivalent bound lane", "required only for this transfer claim", "data absent", "WITHHOLD_TRANSFER_CLAIM", "EB-04"],
    ]
    write_or_check(
        ARTIFACTS / "decision-rules.csv",
        csv_bytes(["rule", "class", "operator", "threshold_or_window", "failure_condition", "outcome", "owner"], decisions),
        args.check,
    )
    successor_rows = [
        ["SNOW-SURFACE-EB-02", "HOLD_FOR_AUTHORITY", "Canopy/sky longwave composition and temperature/emissivity authority absent"],
        ["SNOW-SURFACE-EB-03", "GO", "Contract-first exact-one latent-energy/vapor-mass composition; no prior promotion"],
        ["SNOW-SURFACE-EB-04", "HOLD_FOR_AUTHORITY", "Requires EB-02 authority, EB-03 coupling, and frozen decision thresholds"],
    ]
    write_or_check(
        ARTIFACTS / "successor-admission-decision.csv",
        csv_bytes(["successor", "decision", "basis"], successor_rows),
        args.check,
    )
    stop_rows = [
        ["authority_exhausted", "EB-02 acquisition priorities 1 and 2 exhausted without admitted formulation", "CLOSE_AS_MODEL_LIMITATION"],
        ["coupling_impossible", "EB-03 cannot express one independently reconstructable latent/mass transfer", "CLOSE_AS_MODEL_LIMITATION"],
        ["physical_gate_failure", "Any candidate fails trace, mass, energy, or latent/mass equivalence gate", "REJECT_CANDIDATE"],
        ["inadmissible_compensation", "Improvement requires tuning, forcing correction, unbounded parameter, or noncomparable cells", "REJECT_CANDIDATE"],
        ["one_round_no_win", "One preregistered EB-04 round yields no admissible improvement without protected regression", "CLOSE_NONPROMOTION"],
        ["no_new_information", "Another round lacks new authority, identifying data, or falsifiable attribution", "DO_NOT_OPEN_ROUND"],
    ]
    write_or_check(
        ARTIFACTS / "stop-loss.csv",
        csv_bytes(["trigger", "operational_condition", "outcome"], stop_rows),
        args.check,
    )

    acquisition = [
        [1, "sub-canopy longwave process formulation", "Peer-reviewed formulation for sky/canopy view partition, emissivity, and outgoing snow longwave", "EB-02", "required", "Search/acquire before contract amendment"],
        [2, "canopy radiometric temperature rule", "Measured or authoritative diagnostic operator under snow-season conditions", "EB-02", "required unless item 1 supplies rule", "Search/acquire; do not substitute air temperature without authority"],
        [3, "HJ Andrews paired canopy/open snow observations", "SWE or depth plus meteorology and stratum metadata", "EB-04 warm-maritime conifer claim", "required for transfer claim only", "Acquire if campaign seeks that promotion claim"],
    ]
    write_or_check(
        ARTIFACTS / "source-acquisition-needed.csv",
        csv_bytes(["priority", "source_or_data", "minimum_content", "owner", "necessity", "action"], acquisition),
        args.check,
    )

    # Figure 1: authority and implementation coverage.
    labels = ["Shortwave", "Atmos. LW", "Canopy LW", "Sensible", "Latent/vapor", "Conduction", "Advected heat", "Cold content"]
    plotted_operands = ["absorbed_shortwave", "atmospheric_longwave", "canopy_longwave", "sensible_heat", "latent_heat", "ground_conduction", "advected_precipitation_heat", "cold_content"]
    by_operand = {row[0]: row for row in implementation}
    authority_value = {
        "AUTHORITATIVE_CURRENT": 2,
        "AUTHORITY_ADMISSIBLE": 1,
        "AUTHORITY_MISSING": 0,
        "IMPLEMENTATION_MISSING": 0,
        "REJECTED_PRIOR": 1,
    }
    runtime_value = {
        "implemented_active": 2,
        "implemented": 2,
        "implemented_opt_in": 1,
        "partial": 1,
        "helper_only": 1,
        "missing_runtime": 0,
        "opt_in_candidate": 1,
        "candidate_approximation": 1,
        "absent": 0,
    }
    authority_score = [authority_value[by_operand[name][4]] for name in plotted_operands]
    implementation_score = [runtime_value[by_operand[name][1]] for name in plotted_operands]
    fig, ax = plt.subplots(figsize=(9.2, 4.4))
    x = range(len(labels))
    ax.bar([i - 0.18 for i in x], authority_score, 0.36, label="Authority", color="#2a6f97")
    ax.bar([i + 0.18 for i in x], implementation_score, 0.36, label="Runtime", color="#e9c46a")
    ax.set_xticks(list(x), labels, rotation=25, ha="right")
    ax.set_yticks([0, 1, 2], ["Missing", "Partial/admissible", "Current"])
    ax.set_ylim(0, 2.35)
    ax.set_ylabel("Readiness class")
    ax.grid(axis="y", alpha=0.25)
    ax.legend(frameon=False, ncol=2, loc="upper center")
    save_svg(
        fig,
        FIGURES / "snow-eb01-authority-coverage.svg",
        "Authority and runtime readiness by snow surface-energy component",
        "Grouped bars show current, partial, or missing authority and runtime coverage. Atmospheric and canopy longwave are missing; shortwave and cold content are current.",
        args.check,
    )

    # Figure 2: prior candidate outcomes.
    fig, ax = plt.subplots(figsize=(8.7, 4.8))
    names = ["Default", "Stage A", "Stage A +\nlegacy phase", "Stage B"]
    scores = [int(row[2]) for row in candidates]
    fails = [int(row[1]) for row in candidates]
    colors = ["#2a9d8f", "#e76f51", "#b56576", "#f4a261"]
    ax.scatter(fails, scores, s=150, c=colors, edgecolor="#333333", linewidth=0.8, zorder=3)
    offsets = [(0.12, 1.6), (0.12, 1.2), (0.12, 1.2), (0.12, -3.0)]
    for name, xval, yval, offset in zip(names, fails, scores, offsets, strict=True):
        ax.annotate(name, (xval, yval), xytext=(xval + offset[0], yval + offset[1]))
    ax.axhline(179, color="#2a9d8f", linewidth=1, linestyle="--", alpha=0.7)
    ax.set_xlabel("Robust failure count (lower is better)")
    ax.set_ylabel("Robust ordinal score (higher is better)")
    ax.set_xlim(14.3, 21.2)
    ax.set_ylim(149, 184)
    ax.grid(alpha=0.25)
    save_svg(
        fig,
        FIGURES / "snow-eb01-prior-candidates.svg",
        "Prior sublimation candidate outcomes",
        "Scatter plot compares robust failure count and ordinal score. Stage B matches the default failure count but scores one point lower; all candidates remain nonpromoted.",
        args.check,
    )

    # Figure 3: observation discrimination and binding.
    fixture_labels = ["Marcell\nconifer", "Marcell\ndeciduous", "Harvard\nhardwood", "Harvard\nhemlock", "SNOTEL\nopen", "Sleepers\nsnow/frost", "HJ Andrews\nconifer"]
    selected = [
        next(row for row in obs if row[2] == "Marcell" and row[7] == "conifer"),
        next(row for row in obs if row[2] == "Marcell" and row[7] == "deciduous"),
        next(row for row in obs if row[2] == "Harvard" and row[7] == "hardwood"),
        next(row for row in obs if row[2] == "Harvard" and row[7] == "hemlock"),
        next(row for row in obs if str(row[2]).startswith("snotel_")),
        next(row for row in obs if row[2] == "Sleepers W9"),
        next(row for row in obs if row[2] == "HJ Andrews"),
    ]
    longwave = [2 if row[18] == "both" else 0 for row in selected]
    sublimation = [2 if row[18] in {"both", "sublimation"} else 1 if row[18] == "downstream" else 0 for row in selected]
    binding = [2 if row[9] == "bound" else 1 if row[9] == "unbound" else 0 for row in selected]
    fig, ax = plt.subplots(figsize=(9.4, 4.4))
    data = [longwave, sublimation, binding]
    image = ax.imshow(data, cmap=matplotlib.colors.ListedColormap(["#eeeeee", "#e9c46a", "#2a9d8f"]), vmin=0, vmax=2, aspect="auto")
    del image
    ax.set_yticks([0, 1, 2], ["Longwave contrast", "Sublimation response", "Model binding"])
    ax.set_xticks(range(len(fixture_labels)), fixture_labels)
    for row in range(3):
        for col in range(len(fixture_labels)):
            ax.text(col, row, ["Missing", "Partial", "Strong"][data[row][col]], ha="center", va="center", fontsize=8)
    ax.tick_params(length=0)
    save_svg(
        fig,
        FIGURES / "snow-eb01-observation-discrimination.svg",
        "Observation discrimination and model binding",
        "Matrix shows whether candidate fixture lanes strongly, partially, or do not support longwave contrast, sublimation response, and model binding. HJ Andrews is a missing decisive warm-maritime conifer lane.",
        args.check,
    )


if __name__ == "__main__":
    main()
