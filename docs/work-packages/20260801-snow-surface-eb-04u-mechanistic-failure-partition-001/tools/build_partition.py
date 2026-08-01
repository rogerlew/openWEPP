#!/usr/bin/env python3
"""Build the EB-04U prospective mechanistic partition from retained evidence."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import sys
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"

EB04T = REPO / (
    "docs/work-packages/20260801-snow-surface-eb-04t-unchanged-failure-"
    "attribution-001/artifacts/failure-attribution.json"
)
EB04S = REPO / (
    "docs/work-packages/20260801-snow-surface-eb-04s-authority-reconciliation-"
    "retained-adjudication-001/artifacts/retained-adjudication.json"
)
RESIDUAL = REPO / (
    "docs/work-packages/20260628-snowdensity-10-3-21-post-partition-residual-"
    "decomposition-001/artifacts/post-partition-residual-decomposition.json"
)
SNOWFREEZE = REPO / "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md"
SNOWENERGY = REPO / "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md"
INPUTS = (EB04T, EB04S, RESIDUAL, SNOWFREEZE, SNOWENERGY)
CELLS = ("B", "L", "S", "LS")

COHORT_RULES: dict[str, dict[str, Any]] = {
    "seasonal_densification_trajectory": {
        "cohort": "density_structure",
        "successor": "EB-04V",
        "phases": ["accumulation", "dry_settling", "wet_compaction", "ablation"],
        "operator": "phase-conditioned KGE(r,beta,gamma) of bulk density",
        "operator_units": "density kg m^-3; ratios dimensionless",
        "primary_owner": "openWEPP snow density and persistent-layer evolution",
        "competing_explanations": [
            "fresh-snow density", "destructive metamorphism", "local overburden",
            "liquid-water compaction", "layer merge/projection", "SWE/depth forcing alias",
        ],
        "discriminating_evidence": (
            "KGE components and signed residuals by phase, conditioned on load, layer "
            "temperature, liquid water, snow age, and process-specific density tendency"
        ),
    },
    "seasonal_depth_swe_slope": {
        "cohort": "harvard_geometry_interception",
        "successor": "EB-04X",
        "phases": ["accumulation", "settling", "peak_transition", "ablation"],
        "operator": "depth-SWE slope ratio after rho=rho_w*SWE/depth closure",
        "operator_units": "m depth per m SWE; ratio dimensionless",
        "primary_owner": "shared density/geometry first; paired canopy interception residual second",
        "competing_explanations": [
            "bulk density", "layer geometry", "snowfall reaching ground",
            "interception storage/unloading/drip", "canopy sublimation", "canopy energy",
        ],
        "discriminating_evidence": (
            "daily algebraic closure in open and hardwood lanes, then matched-date "
            "forest-minus-open residual after snowfall and density controls"
        ),
    },
    "seasonal_peak_swe_date": {
        "cohort": "mountain_under_persistence",
        "successor": "EB-04W",
        "phases": ["accumulation", "peak_transition"],
        "operator": "median modeled-minus-observed annual peak-SWE date",
        "operator_units": "days",
        "primary_owner": "forcing/phase/redistribution before peak; snow mass-energy after deposition",
        "competing_explanations": [
            "precipitation forcing", "rain-snow phase", "wind redistribution/deposition",
            "interception", "sublimation", "premature melt",
        ],
        "discriminating_evidence": (
            "pre-peak observed SWE increments versus precipitation/snowfall input and "
            "cumulative modeled sublimation/melt before the observed peak"
        ),
    },
    "seasonal_peak_depth_date": {
        "cohort": "mountain_under_persistence",
        "successor": "EB-04W",
        "phases": ["accumulation", "settling", "peak_transition"],
        "operator": "median modeled-minus-observed annual peak-depth date",
        "operator_units": "days",
        "primary_owner": "density/geometry plus forcing/phase/redistribution and mass-energy timing",
        "competing_explanations": [
            "density timing", "precipitation forcing", "rain-snow phase",
            "wind redistribution/deposition", "sublimation", "premature melt",
        ],
        "discriminating_evidence": (
            "paired SWE/depth peak chronology, density trajectory, pre-peak input, and "
            "cumulative mass-loss/energy operands"
        ),
    },
    "seasonal_ablation_meltout_date": {
        "cohort": "mountain_under_persistence",
        "successor": "EB-04W",
        "phases": ["peak_transition", "ablation", "meltout"],
        "operator": "median modeled-minus-observed annual melt-out date",
        "operator_units": "days",
        "primary_owner": "available peak mass plus post-peak sublimation and ablation energy",
        "competing_explanations": [
            "pre-peak mass deficit", "sublimation", "shortwave/longwave energy",
            "turbulent exchange", "rain heat", "liquid routing", "forcing representativeness",
        ],
        "discriminating_evidence": (
            "mass at observed/model peak, ablation onset, cumulative post-peak vapor/melt "
            "losses, and independently reconstructed energy components"
        ),
    },
}

EXPECTED = Counter(
    {"density_structure": 9, "harvard_geometry_interception": 2, "mountain_under_persistence": 5}
)

OPERANDS: list[dict[str, Any]] = [
    {"id": "observed_date_water_year", "cohorts": "all", "source": "observation", "fields": ["date", "water_year"], "units": "date/year", "sign": "n/a", "lineage": "bound observation CSV", "anti_alias": "date and water year must remain paired"},
    {"id": "observed_swe", "cohorts": "all", "source": "observation", "fields": ["observed_swe_mm"], "units": "mm converted once to m", "sign": "storage nonnegative", "lineage": "SNOTEL/cancov observation CSV", "anti_alias": "never substitute physical depth"},
    {"id": "observed_depth", "cohorts": "all", "source": "observation", "fields": ["observed_snow_depth_m"], "units": "m", "sign": "storage nonnegative", "lineage": "SNOTEL/cancov observation CSV", "anti_alias": "physical depth, never SWE"},
    {"id": "observed_density", "cohorts": "density_structure|harvard_geometry_interception", "source": "observation", "fields": ["observed_density_kg_m3"], "units": "kg m^-3", "sign": "nonnegative", "lineage": "paired SWE/depth-derived or source density", "anti_alias": "exclude missing/zero-depth pairs; preserve quality flag"},
    {"id": "observed_precipitation", "cohorts": "mountain_under_persistence", "source": "observation", "fields": ["observed_precip_mm"], "units": "mm", "sign": "input positive", "lineage": "SNOTEL observation CSV", "anti_alias": "not snowfall without phase proof"},
    {"id": "modeled_day", "cohorts": "all", "source": "trace", "fields": ["day_index"], "units": "simulation day index", "sign": "monotonic", "lineage": "retained snow JSONL", "anti_alias": "join through run calendar; do not assume row offset equals observation date"},
    {"id": "modeled_swe", "cohorts": "all", "source": "trace", "fields": ["runtime_swe_before_m", "runtime_swe_after_m"], "units": "m SWE", "sign": "storage nonnegative", "lineage": "direct snow runtime", "anti_alias": "reconstruct against WAT Snow-Water; not depth"},
    {"id": "modeled_depth", "cohorts": "all", "source": "trace", "fields": ["runtime_depth_before_m", "runtime_depth_after_m"], "units": "m", "sign": "storage nonnegative", "lineage": "direct snow runtime", "anti_alias": "reconstruct against WAT Snow-Depth; not SWE"},
    {"id": "modeled_density", "cohorts": "density_structure|harvard_geometry_interception", "source": "trace", "fields": ["runtime_density_before_kg_m3", "runtime_density_after_kg_m3"], "units": "kg m^-3", "sign": "nonnegative", "lineage": "direct snow runtime", "anti_alias": "must close from SWE/depth when both positive"},
    {"id": "snow_accumulation", "cohorts": "density_structure|mountain_under_persistence|harvard_geometry_interception", "source": "trace", "fields": ["accumulation_m"], "units": "m SWE d^-1", "sign": "input positive", "lineage": "retained daily snow trace", "anti_alias": "requires semantic proof against phase-partition snowfall; not generic precipitation"},
    {"id": "daily_precipitation", "cohorts": "mountain_under_persistence", "source": "wat", "fields": ["P"], "units": "WAT precipitation units; normalize explicitly", "sign": "input positive", "lineage": "direct WAT publication", "anti_alias": "not snowfall or redistribution"},
    {"id": "hourly_phase_partition", "cohorts": "mountain_under_persistence", "source": "missing", "fields": [], "units": "m SWE h^-1 rain and snow", "sign": "input positive", "lineage": "future trace producer", "anti_alias": "daily accumulation cannot prove hourly rain/snow partition"},
    {"id": "wind_redistribution", "cohorts": "mountain_under_persistence", "source": "missing", "fields": [], "units": "kg m^-2 h^-1 or m SWE h^-1", "sign": "signed deposition/export", "lineage": "future forcing/process authority", "anti_alias": "cannot infer from unexplained SWE residual"},
    {"id": "sublimation", "cohorts": "mountain_under_persistence|harvard_geometry_interception", "source": "trace", "fields": ["sublimation_m", "stage3_vapor_mass_exchange_kg_m2"], "units": "m SWE and signed kg m^-2", "sign": "loss-positive depth; vapor exchange sublimation negative", "lineage": "shared Stage 3 exchange", "anti_alias": "must not alias melt, liquid, or refreeze"},
    {"id": "melt_and_routed_liquid", "cohorts": "mountain_under_persistence", "source": "trace", "fields": ["raw_melt_m", "routed_melt_m", "snowpack_swe_loss_m"], "units": "m SWE/liquid", "sign": "loss/routing positive", "lineage": "CoE and snow trace ledgers", "anti_alias": "separate raw melt, SWE loss, and routed liquid"},
    {"id": "stage3_cold_content_vapor_energy", "cohorts": "mountain_under_persistence|harvard_geometry_interception", "source": "trace", "fields": ["stage3_hourly_net_shortwave_w_m2", "stage3_hourly_net_longwave_w_m2", "stage3_hourly_latent_flux_w_m2", "stage3_conduction_energy_j_m2", "stage3_surface_energy_j_m2", "stage3_cold_content_before_j_m2", "stage3_cold_content_after_j_m2", "stage3_cold_content_export_j_m2", "stage3_latent_refreeze_energy_j_m2", "stage3_unused_positive_energy_j_m2", "stage3_energy_closure_residual_j_m2"], "units": "W m^-2 hourly; J m^-2 integrated", "sign": "surface energy positive toward snow; cold content positive deficit", "lineage": "shared Stage 3 cold-content/vapor carrier", "anti_alias": "reconstruct INV-SNOWENERGY-019; Stage 3 unused positive energy is not CoE melt"},
    {"id": "coe_melt_energy_drivers", "cohorts": "mountain_under_persistence", "source": "missing", "fields": [], "units": "J m^-2 by authoritative CoE melt driver", "sign": "positive energy available to melt", "lineage": "future CoE melt diagnostic producer", "anti_alias": "Stage 3 surface or unused-positive energy cannot stand in for CoE melt authority"},
    {"id": "rain_and_sensible_heat", "cohorts": "mountain_under_persistence", "source": "missing", "fields": [], "units": "J m^-2 or W m^-2 with explicit integration", "sign": "positive toward snow", "lineage": "future CoE/forcing energy diagnostic producer", "anti_alias": "rain heat and sensible heat must be named separately from Stage 3 radiation/latent terms"},
    {"id": "layer_state", "cohorts": "density_structure|harvard_geometry_interception", "source": "trace", "fields": ["snow_layers_before", "snow_layers_after", "snow_layer_count_after", "snow_layer_swe_sum_after_m", "snow_layer_depth_sum_after_m"], "units": "per-layer m, kg m^-3, degC, liquid m, days", "sign": "state-domain specific", "lineage": "persistent direct snow layers", "anti_alias": "aggregate reconstruction must use layer sums, not public-state aliases"},
    {"id": "snow_age", "cohorts": "density_structure", "source": "trace", "fields": ["runtime_settle_day_count_before", "runtime_settle_day_count_after"], "units": "days", "sign": "nonnegative", "lineage": "runtime snow state", "anti_alias": "not calendar days since first snow when reset/merge semantics differ"},
    {"id": "layer_overburden", "cohorts": "density_structure", "source": "derived_trace", "fields": ["snow_layers_before"], "units": "kg m^-2", "sign": "load positive", "lineage": "sum overlying layer SWE times rho_w", "anti_alias": "local overburden, not total-pack SWE for every layer"},
    {"id": "process_density_tendency", "cohorts": "density_structure", "source": "missing", "fields": [], "units": "kg m^-3 d^-1 by process", "sign": "signed density change", "lineage": "future density diagnostic producer", "anti_alias": "total before/after change cannot uniquely assign metamorphism/overburden/wet compaction"},
    {"id": "fresh_snow_density", "cohorts": "density_structure", "source": "partial", "fields": ["accumulation_m", "snow_layers_before", "snow_layers_after"], "units": "kg m^-3", "sign": "nonnegative", "lineage": "potential transition reconstruction", "anti_alias": "same-day compaction/merge prevents treating layer difference as direct fresh density"},
    {"id": "paired_harvard_observations", "cohorts": "harvard_geometry_interception", "source": "observation", "fields": ["observed_stratum", "date", "observed_snow_depth_m", "observed_swe_mm", "observed_density_kg_m3"], "units": "m, mm SWE, kg m^-3", "sign": "state-domain specific", "lineage": "HF237 matched-date strata", "anti_alias": "match dates and preserve interpolation/quality flags"},
    {"id": "snow_canopy_interception_ledger", "cohorts": "harvard_geometry_interception", "source": "missing", "fields": [], "units": "m SWE or kg m^-2", "sign": "storage/input/export explicit", "lineage": "future canopy-snow producer", "anti_alias": "generic vegetation interception cannot stand in for snow storage/unloading/drip"},
    {"id": "canopy_state_at_snow_step", "cohorts": "harvard_geometry_interception", "source": "missing", "fields": [], "units": "fraction, LAI/geometry as applicable", "sign": "bounded state", "lineage": "future trace of existing canopy producer", "anti_alias": "fixture summary cannot replace daily state consumed by snow"},
]


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def rel(path: Path) -> str:
    return path.relative_to(REPO).as_posix()


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def normalize_text(path: Path) -> None:
    lines = path.read_text(encoding="utf-8").splitlines()
    path.write_text("\n".join(line.rstrip() for line in lines) + "\n", encoding="utf-8")


def residual_directions(report: dict[str, Any]) -> dict[tuple[str, str], str]:
    return {
        (row["site_id"], row["cell_id"]): row["direction"]
        for row in report["robust_fail_rows"]
    }


def reconstruct_rows(eb04t: dict[str, Any], residual: dict[str, Any]) -> list[dict[str, Any]]:
    directions = residual_directions(residual)
    rows: list[dict[str, Any]] = []
    seen: set[tuple[str, str]] = set()
    for source in eb04t["failures"]:
        cell_id = source["cell_id"]
        if cell_id not in COHORT_RULES:
            raise RuntimeError(f"unclassified failure signature: {cell_id}")
        key = (source["lane_id"], cell_id)
        if key in seen:
            raise RuntimeError(f"duplicate failure row: {key}")
        seen.add(key)
        rule = COHORT_RULES[cell_id]
        direction = directions.get(key)
        if direction is None and cell_id == "seasonal_peak_depth_date":
            direction = "under_persistence_with_density_coupling"
        if direction is None:
            raise RuntimeError(f"missing retained residual direction: {key}")
        rows.append(
            {
                "lane_id": source["lane_id"],
                "stratum": source["stratum"],
                "climate": source["climate"],
                "cell_id": cell_id,
                "cohort": rule["cohort"],
                "successor": rule["successor"],
                "retained_direction": direction,
                "phases": rule["phases"],
                "observation_operator": rule["operator"],
                "operator_units": rule["operator_units"],
                "primary_owner": rule["primary_owner"],
                "competing_explanations": rule["competing_explanations"],
                "discriminating_evidence": rule["discriminating_evidence"],
                "current_data_role": "DIAGNOSTIC_ONLY",
                "future_promotion_role": "INDEPENDENT_VALIDATION_REQUIRED",
                "eb04t_primary_metric": source["primary_metric"],
                "eb04t_baseline_value": source["primary_values"]["B"],
                "eb04t_ls_direction": source["directions_vs_b"]["LS"],
            }
        )
    rows.sort(key=lambda row: (row["cohort"], row["lane_id"], row["cell_id"]))
    counts = Counter(row["cohort"] for row in rows)
    if counts != EXPECTED or len(rows) != 16:
        raise RuntimeError(f"cohort inventory mismatch: {counts}")
    return rows


def retained_schemas(eb04s: dict[str, Any]) -> dict[str, Any]:
    try:
        import pyarrow.parquet as pq
    except ImportError as exc:
        raise RuntimeError("pyarrow is required to inspect retained WAT schemas") from exc

    trace_sets: list[set[str]] = []
    wat_sets: list[set[str]] = []
    observation_sets: list[set[str]] = []
    trace_count = 0
    for lane in eb04s["lanes"]:
        if lane["role"] != "INDEPENDENT_VALIDATION":
            continue
        observation = REPO / lane["observation_file"]
        with observation.open(newline="", encoding="utf-8") as handle:
            reader = csv.reader(handle)
            observation_sets.append(set(next(reader)))
        for cell in CELLS:
            item = lane["cells"][cell]
            trace = REPO / item["trace"]
            wat = REPO / item["wat"]
            with trace.open(encoding="utf-8") as handle:
                first = json.loads(next(handle))
            trace_sets.append(set(first))
            wat_sets.append(set(pq.read_schema(wat).names))
            trace_count += 1
    if trace_count != 40:
        raise RuntimeError(f"expected 40 retained EB-04S trace cells, got {trace_count}")
    return {
        "trace_cell_count": trace_count,
        "trace_fields_all": sorted(set.intersection(*trace_sets)),
        "trace_fields_any": sorted(set.union(*trace_sets)),
        "wat_fields_all": sorted(set.intersection(*wat_sets)),
        "observation_fields_any": sorted(set.union(*observation_sets)),
        "observation_fields_all": sorted(set.intersection(*observation_sets)),
    }


def availability(operand: dict[str, Any], schemas: dict[str, Any]) -> str:
    source = operand["source"]
    fields = set(operand["fields"])
    if source == "missing":
        return "MISSING_REQUIRED"
    if source == "partial":
        return "PARTIAL_AMBIGUOUS"
    if source == "derived_trace":
        return "AVAILABLE_DERIVED" if fields <= set(schemas["trace_fields_all"]) else "MISSING_REQUIRED"
    if source == "trace":
        return "AVAILABLE_DIRECT" if fields <= set(schemas["trace_fields_all"]) else "MISSING_REQUIRED"
    if source == "wat":
        return "AVAILABLE_DIRECT" if fields <= set(schemas["wat_fields_all"]) else "MISSING_REQUIRED"
    if source == "observation":
        any_fields = set(schemas["observation_fields_any"])
        return "AVAILABLE_DIRECT" if fields <= any_fields else "MISSING_REQUIRED"
    raise RuntimeError(f"unknown operand source: {source}")


def build_observability(schemas: dict[str, Any]) -> list[dict[str, Any]]:
    rows = []
    for operand in OPERANDS:
        row = dict(operand)
        row["availability"] = availability(row, schemas)
        if row["id"] == "snow_accumulation":
            row["availability"] = "AVAILABLE_SEMANTIC_PROOF_REQUIRED"
        rows.append(row)
    return rows


def evidence_roles(eb04s: dict[str, Any]) -> list[dict[str, Any]]:
    roles: list[dict[str, Any]] = []
    for lane in eb04s["lanes"]:
        if lane["role"] != "INDEPENDENT_VALIDATION":
            continue
        path = REPO / lane["observation_file"]
        years: set[int] = set()
        records = 0
        with path.open(newline="", encoding="utf-8") as handle:
            for row in csv.DictReader(handle):
                if row.get("observed_stratum") and row["observed_stratum"] != lane["stratum"]:
                    continue
                if row.get("water_year"):
                    years.add(int(row["water_year"]))
                records += 1
        roles.append(
            {
                "lane_id": lane["lane_id"],
                "stratum": lane["stratum"],
                "corpus": lane["corpus"],
                "observation_file": lane["observation_file"],
                "observation_sha256": lane["observation_sha256"],
                "record_count": records,
                "water_year_min": min(years) if years else None,
                "water_year_max": max(years) if years else None,
                "water_year_count": len(years),
                "eb04u_role": "DIAGNOSTIC_ONLY",
                "reason": "previously consumed by EB-04S/04T and now used to design mechanisms/operators",
            }
        )
    roles.sort(key=lambda row: row["lane_id"])
    if len(roles) != 10:
        raise RuntimeError(f"expected 10 observed lanes, got {len(roles)}")
    return roles


def successor_admission(observability: list[dict[str, Any]]) -> list[dict[str, Any]]:
    by_id = {row["id"]: row["availability"] for row in observability}
    return [
        {
            "successor": "EB-04V", "cohort": "density_structure",
            "population_operator": "READY",
            "authority": "PARTIAL_EXISTING_FAMILY_SELECTION_REQUIRED",
            "observability": "PREREQUISITE",
            "data_authority": "DIAGNOSTIC_ONLY_INDEPENDENT_VALIDATION_REQUIRED",
            "ownership": "OPENWEPP",
            "admission": "ADMIT_DIAGNOSTIC_RESEARCH_WITH_PREREQUISITES",
            "prerequisites": [
                "publish process-specific density tendencies",
                "disambiguate fresh-snow density from same-day compaction/merge",
                "select authoritative missing or misapplied process before implementation",
                "freeze numeric materiality, replication, and site-spread efficacy rules",
                "freeze new independent validation before result-bearing promotion",
            ],
            "observability_checks": {
                "layer_state": by_id["layer_state"],
                "process_density_tendency": by_id["process_density_tendency"],
                "fresh_snow_density": by_id["fresh_snow_density"],
            },
        },
        {
            "successor": "EB-04W", "cohort": "mountain_under_persistence",
            "population_operator": "READY",
            "authority": "SPLIT_PROCESS_AND_FORCING_OWNERSHIP_UNRESOLVED",
            "observability": "PREREQUISITE",
            "data_authority": "DIAGNOSTIC_ONLY_INDEPENDENT_VALIDATION_REQUIRED",
            "ownership": "OPENWEPP_SNOW_PROCESS_AND_WEPPPY_FORCING_BOUNDARY",
            "admission": "ADMIT_DIAGNOSTIC_RESEARCH_WITH_PREREQUISITES",
            "prerequisites": [
                "prove accumulation_m semantics and hourly phase lineage",
                "expose or otherwise authoritatively bound wind redistribution/deposition",
                "separate pre-peak deficit from post-peak loss before process selection",
                "publish CoE melt-driver, rain/sensible-heat, and Stage 3 cold-content closure ledgers",
                "freeze numeric materiality, replication, and site-spread efficacy rules",
                "freeze new independent validation before result-bearing promotion",
            ],
            "observability_checks": {
                "snow_accumulation": by_id["snow_accumulation"],
                "hourly_phase_partition": by_id["hourly_phase_partition"],
                "wind_redistribution": by_id["wind_redistribution"],
                "stage3_cold_content_vapor_energy": by_id["stage3_cold_content_vapor_energy"],
                "coe_melt_energy_drivers": by_id["coe_melt_energy_drivers"],
                "rain_and_sensible_heat": by_id["rain_and_sensible_heat"],
            },
        },
        {
            "successor": "EB-04X", "cohort": "harvard_geometry_interception",
            "population_operator": "READY",
            "authority": "DENSITY_FIRST_CANOPY_RESIDUAL_SECOND",
            "observability": "PREREQUISITE",
            "data_authority": "DIAGNOSTIC_ONLY_INDEPENDENT_VALIDATION_REQUIRED",
            "ownership": "OPENWEPP_DENSITY_AND_CANOPY_SNOW_PROCESS",
            "admission": "ADMIT_DIAGNOSTIC_RESEARCH_WITH_PREREQUISITES",
            "prerequisites": [
                "prove daily SWE-density-depth closure in both lanes",
                "publish snow-specific interception storage/unloading/drip ledger",
                "publish daily canopy state consumed by the snow step",
                "freeze numeric materiality, replication, and site-spread efficacy rules",
                "freeze new paired independent validation before longwave/interception promotion",
            ],
            "observability_checks": {
                "paired_harvard_observations": by_id["paired_harvard_observations"],
                "snow_canopy_interception_ledger": by_id["snow_canopy_interception_ledger"],
                "canopy_state_at_snow_step": by_id["canopy_state_at_snow_step"],
            },
        },
    ]


def write_csv(path: Path, rows: list[dict[str, Any]], fields: list[str]) -> None:
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        for row in rows:
            encoded = {
                key: "|".join(str(item) for item in value) if isinstance(value, list) else value
                for key, value in row.items() if key in fields
            }
            writer.writerow(encoded)


def write_machine_artifacts(result: dict[str, Any]) -> None:
    (ARTIFACTS / "failure-cohort-manifest.json").write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    write_csv(
        ARTIFACTS / "failure-mechanics-matrix.csv", result["failures"],
        ["lane_id", "stratum", "climate", "cell_id", "cohort", "successor",
         "retained_direction", "phases", "observation_operator", "operator_units",
         "primary_owner", "competing_explanations", "discriminating_evidence",
         "current_data_role", "future_promotion_role"],
    )
    write_csv(
        ARTIFACTS / "operand-lineage.csv", result["observability"],
        ["id", "cohorts", "units", "sign", "lineage", "fields", "availability", "anti_alias"],
    )
    write_csv(
        ARTIFACTS / "evidence-role-manifest.csv", result["evidence_roles"],
        ["lane_id", "stratum", "corpus", "observation_file", "observation_sha256",
         "record_count", "water_year_min", "water_year_max", "water_year_count",
         "eb04u_role", "reason"],
    )


def write_protocols(result: dict[str, Any]) -> None:
    (ARTIFACTS / "seasonal-phase-protocol.md").write_text("""# Seasonal-Phase Diagnostic Protocol

Evidence mode: `Static + Reused Ran`.

## Phase Definitions

| Primary phase | Frozen observed-date boundary | Primary question |
| --- | --- | --- |
| Pre-peak / accumulation | Accepted observed snow-on date through the day before the observed peak anchor | Is snow mass absent before loss processes can explain it? |
| Peak anchor | Earliest accepted date attaining the observed seasonal maximum for the scored observable | Are peak dates separated by input, density timing, or early loss? |
| Post-peak / ablation | Day after the observed peak anchor through accepted observed persistent disappearance | Does a credible peak pack lose mass too rapidly? |
| Melt-out | Accepted persistent-disappearance operator and tie rule from the frozen observation rubric | Is disappearance early because peak mass was deficient or post-peak loss was excessive? |

The primary frame is computed once from observations by water year and
observable, then sealed before any candidate executes. Baseline and every
candidate are sampled on exactly those observed dates. All maximum ties are
reported and the earliest accepted maximum is the deterministic anchor. An
invalid or missing observed peak, inadequate coverage, or missing accepted
snow-on/disappearance boundary makes that phase inconclusive; a modeled date
may never replace it.

Dry-settling, wet-compaction, and model-peak labels are secondary diagnostics.
They may use candidate state, but cannot change dates entering primary efficacy
or promotion operators. Any transition-window width, dry/wet threshold, or
alternative phase operator must be authority-backed and prospectively sealed
by the successor before a result-bearing attempt; none is admitted by EB-04U.

## Cohort Operators

Density work reports KGE correlation, bias ratio, and variability ratio by
the frozen primary frame, plus signed density residuals. Geometry work first reconstructs
`rho_bulk = rho_water * SWE / depth` and requires layer aggregates to close
public SWE and depth. Under-persistence work separates cumulative input through
the observed peak from cumulative vapor and mass loss before and after that
peak. Stage 3 energy may reconstruct cold-content/vapor exchange only; it may
not be interpreted as the authoritative CoE melt-energy ledger.

No phase-conditioned association uniquely identifies a process. EB-04V/04W/04X
must expose the missing process-specific operands named in `operand-lineage.csv`
before selecting a correction.
""", encoding="utf-8")

    (ARTIFACTS / "evidence-role-protocol.md").write_text("""# Prospective Evidence-Role Protocol

Evidence mode: `Static`.

All ten current EB-04S observation lanes and every water year summarized in
`evidence-role-manifest.csv` are `DIAGNOSTIC_ONLY` for EB-04V–04X. They were
already consumed by EB-04S/04T and are now used to select cohorts and operators;
they cannot regain independent-validation status by splitting years after this
design was informed by their aggregate outcomes.

No calibration dataset is assigned because EB-04U selects no tunable candidate.
If a successor introduces an empirically estimable parameter, it must freeze a
separate `CALIBRATION` dataset before fitting and may not use it for independent
validation.

Promotion-grade `INDEPENDENT_VALIDATION` requires one of:

1. a new observation source or site not used in EB-04S/04T;
2. newly acquired later water years whose values were unavailable during
   EB-04U design; or
3. a genuinely sealed record partition whose outcomes were never inspected or
   summarized during design or calibration.

The source identity, record-unit assignment, exclusions, observation operator,
and release condition must be frozen before candidate execution. Existing data
remain valuable for mechanism diagnosis and calibration-readiness work; their
loss of independence limits promotion claims, not authoritative implementation.
""", encoding="utf-8")

    (ARTIFACTS / "prospective-decision-protocol.md").write_text("""# Prospective Successor Decision Protocol

Evidence mode: `Static`.

## Materiality

Exact sign is descriptive only. A response is materially improved or degraded
only when it changes an accepted `SC-SNOWFREEZE-001#TOL-SNOWFREEZE-010/011`
rubric band, or when a successor prospectively supplies measurement-uncertainty
authority and sensitivity evidence for a stricter threshold. EB-04S/04T effect
magnitudes may not define a threshold.

## Mechanism Efficacy

Directional movement and a one-cell band crossing are diagnostic only. EB-04U
does not admit a result-bearing efficacy study because no authoritative numeric
replication or site-spread threshold is yet available. Before such a study, the
successor must seal the materiality threshold, record-level replication unit,
minimum improved fraction/count, minimum number of independent sites, missing-
record rule, and stratification rule. Values must come from measurement or
process authority and cannot be chosen from EB-04S/04T effects or after a
candidate result is inspected.

- EB-04V must assess both retained high- and low-density-bias strata separately;
  a global score cannot compensate for worsening either direction.
- EB-04W must first assign each case to pre-peak input deficit or post-peak loss.
  A snow-process candidate cannot claim efficacy for a forcing-owned deficit or
  infer CoE melt causality from Stage 3 cold-content energy.
- EB-04X must protect open-lane geometry before interpreting a matched hardwood-
  minus-open residual. Canopy longwave or interception cannot receive credit
  for a shared open/hardwood density error.

The sealed predicate must reproduce on prospectively independent evidence
before default promotion.

## Adjacent-Process Noninferiority

No forcing-robust protected cell may fall to a worse accepted rubric band. Mass,
energy, SWE-depth-density, layer, selector/default, and forcing-lineage gates
remain hard prerequisites. Forcing-limited magnitude cells remain visible but
cannot be converted into physics defects or hidden by an aggregate score.

## Component And Interaction Rule

Longwave, sublimation, density, redistribution, and interception candidates are
adjudicated first on mechanism-identifying evidence. Combined LS or later
compositions are eligible only after their components are separately
admissible. The interaction residual is always reported; a composition cannot
promote when interaction reverses a component's material benefit, creates a new
forcing-robust failure, or reveals compensating mass/energy error.

## Stop-Loss

One prospectively frozen result-bearing attempt is allowed per sealed,
authority-backed candidate identity. The seal binds authority/formulation and
its hashes, source commit and executable, selectors/defaults, the complete
parameter vector, forcing and fixture hashes, population and evidence roles,
observation/phase operators, and every decision threshold. A result is consumed
when any candidate outcome is inspected. A preflight that emits no result may
be repaired without spending the attempt; otherwise changing a name, tuned
value, evidence subset, or threshold cannot create a new candidate. A later
attempt requires a prospectively documented authority-backed mechanism or
formulation change expected to address a named failure.

Stop without promotion on observation leakage,
site fitting, forcing rescaling, missing independent closure, unresolved owner,
proxy physics, failure to improve the owning cohort, or protected regression.
A failed candidate may open a new defect/process package only when it yields a
named mechanism, authority route, write set, and acceptance test.
""", encoding="utf-8")

    lines = [
        "# Successor Admission Matrix", "", "Evidence mode: `Static + Reused Ran`.", "",
        "| Successor | Cohort | Current admission | Ownership | Binding prerequisites |",
        "| --- | --- | --- | --- | --- |",
    ]
    for row in result["successor_admission"]:
        prereq = "; ".join(row["prerequisites"])
        lines.append(
            f"| {row['successor']} | `{row['cohort']}` | `{row['admission']}` | "
            f"`{row['ownership']}` | {prereq} |"
        )
    lines.extend([
        "", "All three successors are admitted for bounded diagnostic research, not for a",
        "production amendment or result-bearing promotion run. Each prerequisite is a",
        "successor entry gate declared before implementation. Missing independent data",
        "limits promotion but does not authorize proxy physics or block implementation",
        "once authoritative process science is admitted.", "",
    ])
    (ARTIFACTS / "successor-admission-matrix.md").write_text("\n".join(lines), encoding="utf-8")

    (ARTIFACTS / "calibration-readiness-matrix.md").write_text("""# Calibration-Readiness Matrix

Evidence mode: `Static + Reused Ran`.

## ADR-0042 Status By Mechanism

| Mechanism | Science implementation | Calibration evidence | Identifiability | Read |
| --- | --- | --- | --- | --- |
| Existing sub-canopy longwave | `IMPLEMENTED` | `NOT_APPLICABLE` | `NONIDENTIFIABLE` in current failures | All timing failures are open controls; EB-04X needs new paired evidence. |
| Existing sublimation | `IMPLEMENTED` | `NOT_APPLICABLE` | `PARTIALLY_IDENTIFIABLE` | Open timing cells respond, but forcing and interactions remain confounded. |
| Density-process amendment | `NOT_IMPLEMENTED` | `NOT_APPLICABLE` | `PARTIALLY_IDENTIFIABLE` | Mixed bias and missing process tendencies require EB-04V selection before implementation. |
| Wind-redistribution snow process | `NOT_IMPLEMENTED` | `NOT_APPLICABLE` | `NONIDENTIFIABLE` | openWEPP process authority is unreconciled and redistribution is not separately observed. |
| Precipitation/phase forcing correction | `NOT_APPLICABLE` in openWEPP | `NOT_APPLICABLE` | `NONIDENTIFIABLE` | wepppy/forcing-provider ownership must be resolved; openWEPP may diagnose but cannot compensate for forcing undercatch or phase error. |
| Canopy-snow interception amendment | `NOT_IMPLEMENTED` | `NOT_APPLICABLE` | `NONIDENTIFIABLE` | Authority has not yet been reconciled; Harvard pairing exists, but the snow-interception ledger is absent. |

## Current EB-04U Readiness Obligations

| Obligation | Disposition | Evidence and rationale |
| --- | --- | --- |
| Typed/enumerable parameter surface | `NOT_APPLICABLE` | EB-04U selects no candidate or tunable parameter. |
| Observation operator with units and scale | `PASS` | Failure mechanics matrix and seasonal-phase protocol. |
| Deterministic candidate execution | `NOT_APPLICABLE` | Candidate/model execution is explicitly excluded. |
| Objective reconstruction | `PASS` | Operators and independent operand requirements are frozen prospectively. |
| Sensitivity analysis | `NOT_APPLICABLE` | No candidate parameter or threshold is introduced. |
| Identifiability/confounding analysis | `PASS` | Cohort matrix, observability matrix, and ownership split. |
| Boundary, saturation, and failure reporting | `PASS` | Decision protocol and successor admission prerequisites. |
| Equifinality/uncertainty retention | `PASS` | Competing explanations remain explicit; no unique-cause claim. |
| Synthetic recovery | `NOT_APPLICABLE` | No estimable parameter surface or calibration machinery is in scope. |
| Additional-data inventory | `PASS` | Evidence-role protocol and missing observability entries. |

The successor statuses are prospective intake findings, not unmet EB-04U
implementation gates. EB-04U's current-scope readiness-design obligations pass.
""", encoding="utf-8")

    (ARTIFACTS / "scientific-synthesis.md").write_text("""# Mechanistic Partition Synthesis

Evidence mode: `Static + Reused Ran`.

EB-04U freezes three separate study populations. Nine failures require a
density-process investigation, two require a density-first paired Harvard
geometry/interception investigation, and five require an open-control
accumulation/under-persistence investigation. No current failure identifies
canopy longwave.

The retained outputs are much more informative than the prior ordinal count:
all 40 retained EB-04S B/L/S/LS cells publish daily SWE, depth, density,
layer state, snow age, sublimation, melt/liquid, and hourly Stage 3 cold-content
and vapor-energy components. They do not publish authoritative CoE melt-energy
drivers, separately named rain/sensible heat, process-specific density tendencies, direct
fresh-snow density uncontaminated by same-day compaction/merge, hourly
rain/snow phase inputs, wind redistribution/deposition, daily snow-canopy state,
or a snow-specific interception/unloading/drip ledger.

Those gaps determine the next mechanics. EB-04V must expose and distinguish
density tendencies before selecting an amendment. EB-04W must decide whether
mass is missing before peak or removed too early and must route forcing-owned
deficits to wepppy rather than compensating in snow physics. EB-04X must prove
SWE-density-depth closure in the open lane before interpreting a paired forest
residual as interception or canopy energy.

All observations already used by EB-04S/04T are now diagnostic-only. This does
not block authoritative implementation under ADR-0042, but default promotion
requires genuinely new or previously sealed independent validation. The frozen
decision protocol separates component efficacy, adjacent-process
noninferiority, interaction, and closure; it prohibits result-derived
thresholds, site fitting, forcing rescaling, and aggregate-score compensation.

Disposition: `PROSPECTIVE_DIAGNOSTIC_DESIGN_COMPLETE`. Admit EB-04V, EB-04W,
and EB-04X for bounded diagnostic operator/observability research with their
named prerequisites. Do not admit a result-bearing efficacy or promotion
factorial.
""", encoding="utf-8")


def make_figures(result: dict[str, Any]) -> None:
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    import numpy as np

    matplotlib.rcParams["svg.hashsalt"] = "snow-surface-eb04u-v1"
    FIGURES.mkdir(parents=True, exist_ok=True)

    counts = result["summary"]["cohort_counts"]
    order = ["density_structure", "harvard_geometry_interception", "mountain_under_persistence"]
    labels = ["Density structure", "Harvard geometry / interception", "Mountain under-persistence"]
    colors = ["#3973ac", "#8b6bb3", "#d47f2f"]
    fig, ax = plt.subplots(figsize=(9.2, 4.8))
    bars = ax.barh(labels, [counts[name] for name in order], color=colors)
    ax.bar_label(bars, padding=4)
    ax.set_xlabel("Forcing-robust EB-04T failures")
    ax.set_title("EB-04U partitions 16 failures into three mechanistic studies")
    ax.set_xlim(0, 10)
    ax.grid(axis="x", alpha=0.25)
    ax.set_axisbelow(True)
    fig.tight_layout()
    path = FIGURES / "eb04u-cohort-partition.svg"
    fig.savefig(path, format="svg", metadata={"Date": None})
    plt.close(fig)
    normalize_text(path)

    cohorts = order
    status_order = ["AVAILABLE_DIRECT", "AVAILABLE_DERIVED", "AVAILABLE_SEMANTIC_PROOF_REQUIRED", "PARTIAL_AMBIGUOUS", "MISSING_REQUIRED"]
    status_labels = ["Direct", "Derived", "Semantic proof needed", "Partial / ambiguous", "Missing"]
    status_colors = ["#2a9d8f", "#70a288", "#e9c46a", "#f4a261", "#d1495b"]
    values = []
    for cohort in cohorts:
        relevant = [row for row in result["observability"] if row["cohorts"] == "all" or cohort in row["cohorts"].split("|")]
        counts_by_status = Counter(row["availability"] for row in relevant)
        values.append([counts_by_status.get(status, 0) for status in status_order])
    fig, ax = plt.subplots(figsize=(10.2, 5.3))
    left = np.zeros(len(cohorts))
    for index, status in enumerate(status_order):
        segment = np.array([row[index] for row in values])
        ax.barh(labels, segment, left=left, color=status_colors[index], label=status_labels[index])
        left += segment
    ax.set_xlabel("Required diagnostic operands")
    ax.set_title("Retained observability is useful but not mechanism-complete")
    ax.legend(ncol=3, loc="lower center", bbox_to_anchor=(0.5, -0.34))
    ax.grid(axis="x", alpha=0.25)
    ax.set_axisbelow(True)
    fig.tight_layout()
    path = FIGURES / "eb04u-observability-status.svg"
    fig.savefig(path, format="svg", metadata={"Date": None})
    plt.close(fig)
    normalize_text(path)

    admission = result["successor_admission"]
    categories = ["Population/operator", "Authority", "Observability", "Independent data"]
    matrix = np.array([
        [2, 1, 1, 0],
        [2, 0, 1, 0],
        [2, 1, 1, 0],
    ])
    fig, ax = plt.subplots(figsize=(9.6, 4.7))
    cmap = matplotlib.colors.ListedColormap(["#d1495b", "#e9c46a", "#2a9d8f"])
    ax.imshow(matrix, cmap=cmap, vmin=0, vmax=2, aspect="auto")
    ax.set_xticks(range(len(categories)), categories)
    ax.set_yticks(range(3), [row["successor"] for row in admission])
    ax.set_title("Successors are admitted for diagnosis, not promotion execution")
    words = {0: "Missing", 1: "Prerequisite", 2: "Ready"}
    for y in range(matrix.shape[0]):
        for x in range(matrix.shape[1]):
            ax.text(x, y, words[int(matrix[y, x])], ha="center", va="center", fontsize=9)
    fig.tight_layout()
    path = FIGURES / "eb04u-successor-readiness.svg"
    fig.savefig(path, format="svg", metadata={"Date": None})
    plt.close(fig)
    normalize_text(path)


def write_sidecars(result: dict[str, Any]) -> None:
    sidecars = {
        "eb04u-cohort-partition": (
            "Mechanistic partition of the EB-04T failures",
            "Counts show the exact 16 forcing-robust EB-04T failure rows assigned once to the density, paired Harvard geometry/interception, or open-control mountain under-persistence study.",
            "The density cohort is largest, but count does not imply priority or unique causality. The three cohorts use different operators and cannot share one efficacy verdict.",
        ),
        "eb04u-observability-status": (
            "Retained diagnostic observability by cohort",
            "Bars count required operands for each cohort by current retained-output status. Direct and derived fields are usable now; semantic-proof, partial, and missing fields are successor prerequisites.",
            "Existing traces support substantial reconstruction but not unique process attribution. Density tendencies, hourly phase inputs, wind redistribution, and snow-specific interception remain important gaps.",
        ),
        "eb04u-successor-readiness": (
            "Readiness of the three mechanistic successors",
            "Cells summarize whether each successor has a frozen population/operator, adequate authority, mechanism-complete observability, and independent promotion data.",
            "EB-04V/04W/04X may begin bounded diagnostic research. None may begin a result-bearing promotion experiment until its yellow/red prerequisites are closed prospectively.",
        ),
    }
    for stem, (title, caption, interpretation) in sidecars.items():
        (FIGURES / f"{stem}.md").write_text(f"""# {title}

## Caption

{caption}

## Population And Method

The population is the immutable EB-04T 16-row forcing-robust failure inventory.
Operand availability is audited across 40 retained EB-04S B/L/S/LS trace and
WAT cells, originally assigned independent validation there but diagnostic-only
for EB-04U through EB-04X. No model or candidate was executed. Counts are
categorical; they are not empirical effect sizes.

## Interpretation

{interpretation}

## Limitations

This is prospective study design built from previously consumed diagnostic
evidence. It does not identify a unique cause, establish a material effect,
calibrate a coefficient, amend production physics, supply independent
validation, or authorize default promotion.
""", encoding="utf-8")

    (ARTIFACTS / "figure-inventory.md").write_text("""# Figure Inventory

| Figure | Question | Sidecar |
| --- | --- | --- |
| `figures/eb04u-cohort-partition.svg` | How are the 16 failures partitioned? | same-stem `.md` |
| `figures/eb04u-observability-status.svg` | Which discriminating operands exist or are missing? | same-stem `.md` |
| `figures/eb04u-successor-readiness.svg` | What may each successor do next? | same-stem `.md` |

All figures are explanatory plots without embedded prose blocks. Sidecars carry
population, units/status meaning, method, interpretation, and limitations.
""", encoding="utf-8")


def summarize(rows: list[dict[str, Any]], observability: list[dict[str, Any]]) -> dict[str, Any]:
    return {
        "failure_count": len(rows),
        "cohort_counts": dict(Counter(row["cohort"] for row in rows)),
        "successor_counts": dict(Counter(row["successor"] for row in rows)),
        "all_current_data_roles": dict(Counter(row["current_data_role"] for row in rows)),
        "observability_counts": dict(Counter(row["availability"] for row in observability)),
        "canopy_longwave_identifying_failure_count": 0,
        "new_candidate_results_inspected": 0,
        "model_subprocesses_launched": 0,
        "promotion_authorized": False,
        "disposition": "PROSPECTIVE_DIAGNOSTIC_DESIGN_COMPLETE",
    }


def self_check() -> None:
    for path in INPUTS:
        if not path.is_file():
            raise RuntimeError(f"missing required input: {path}")
    rows = reconstruct_rows(read_json(EB04T), read_json(RESIDUAL))
    if Counter(row["cohort"] for row in rows) != EXPECTED:
        raise RuntimeError("cohort self-check failed")
    mutated = read_json(EB04T)
    mutated["failures"][0]["cell_id"] = "rejected_unknown_signature"
    try:
        reconstruct_rows(mutated, read_json(RESIDUAL))
    except RuntimeError:
        pass
    else:
        raise RuntimeError("rejected signature alias was accepted")
    duplicated = read_json(EB04T)
    duplicated["failures"].append(duplicated["failures"][0])
    try:
        reconstruct_rows(duplicated, read_json(RESIDUAL))
    except RuntimeError:
        pass
    else:
        raise RuntimeError("overlapping cohort row was accepted")


def analyze() -> dict[str, Any]:
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    eb04t = read_json(EB04T)
    eb04s = read_json(EB04S)
    residual = read_json(RESIDUAL)
    rows = reconstruct_rows(eb04t, residual)
    schemas = retained_schemas(eb04s)
    observability = build_observability(schemas)
    roles = evidence_roles(eb04s)
    admission = successor_admission(observability)
    result = {
        "schema": "snow-surface-eb04u-mechanistic-partition-v1",
        "evidence_class": "Static + Reused Ran",
        "diagnostic_design_only": True,
        "inputs": {rel(path): sha256(path) for path in INPUTS},
        "summary": summarize(rows, observability),
        "retained_schema_audit": schemas,
        "failures": rows,
        "observability": observability,
        "evidence_roles": roles,
        "successor_admission": admission,
    }
    write_machine_artifacts(result)
    write_protocols(result)
    make_figures(result)
    write_sidecars(result)
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--self-check", action="store_true")
    mode.add_argument("--analyze", action="store_true")
    args = parser.parse_args()
    if args.self_check:
        self_check()
        print("EB-04U frozen inventory and rejected-alias/overlap self-check: PASS")
        return
    result = analyze()
    print(json.dumps(result["summary"], indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
