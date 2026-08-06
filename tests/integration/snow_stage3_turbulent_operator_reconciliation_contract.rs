use std::fs;

use openwepp_hillslope_orchestrator::{
    DirectSnowLiquidPartition, DirectSnowStage3EvaluationDiagnostics,
    DirectSnowStage3EvaluationHourDiagnostics, DirectSnowStage3EvaluationResult,
    SnowStage3EvaluationOperator,
};

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const PACKAGE: &str = "docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001/package.md";
const PROTOCOL: &str = "docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001/artifacts/protocol-freeze.json";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[allow(dead_code, clippy::too_many_lines)]
fn protected_external_struct_literals_remain_exhaustive(authoritative: DirectSnowLiquidPartition) {
    let hour = DirectSnowStage3EvaluationHourDiagnostics {
        sensible_flux_w_m2: 0.0,
        latent_flux_w_m2: 0.0,
        advected_flux_w_m2: 0.0,
        shortwave_energy_j_m2: 0.0,
        longwave_energy_j_m2: 0.0,
        internal_active_lower_conduction_j_m2: 0.0,
        complete_energy_j_m2: 0.0,
        vapor_mass_exchange_kg_m2: 0.0,
        cold_required_j_m2: 0.0,
        cold_energy_change_j_m2: 0.0,
        cold_content_export_j_m2: 0.0,
        excess_energy_j_m2: 0.0,
        ice_available_kg_m2: 0.0,
        sublimation_kg_m2: 0.0,
        melt_kg_m2: 0.0,
        unallocated_after_exhaustion_j_m2: 0.0,
        energy_closure_residual_j_m2: 0.0,
        complete_carrier_evaluated: false,
        requested_seconds: 0.0,
        evaluated_seconds: 0.0,
    };
    let evaluation = DirectSnowStage3EvaluationDiagnostics {
        operator: SnowStage3EvaluationOperator::SameStatePairedCarrierV1,
        source_snapshot_id: "source",
        support_id: "support",
        cadence_id: "cadence",
        carrier_id: "carrier",
        coverage_id: "coverage",
        claim_class: "claim",
        unresolved_boundaries_id: "boundaries",
        pairing_id: None,
        arm_ids: ["surface", "complete"],
        arm_count: 2,
        source_fingerprint: 0,
        forcing_fingerprint: 0,
        geometry_fingerprint: 0,
        non_formulation_fingerprint: 0,
        surface_arm_non_formulation_fingerprint: 0,
        complete_arm_non_formulation_fingerprint: 0,
        requested_seconds: 0.0,
        evaluated_seconds: 0.0,
        coverage_fraction: 0.0,
        surface_arm_applicable: false,
        surface_arm_shortwave_j_m2: 0.0,
        surface_arm_longwave_j_m2: 0.0,
        surface_arm_latent_j_m2: 0.0,
        surface_arm_sensible_applicable: false,
        surface_arm_advected_applicable: false,
        surface_arm_internal_conduction_applicable: false,
        surface_arm_total_j_m2: 0.0,
        complete_arm_shortwave_j_m2: 0.0,
        complete_arm_longwave_j_m2: 0.0,
        complete_arm_sensible_j_m2: 0.0,
        complete_arm_latent_j_m2: 0.0,
        complete_arm_advected_j_m2: 0.0,
        complete_arm_internal_active_lower_conduction_j_m2: 0.0,
        complete_arm_applicable: false,
        complete_arm_internal_conduction_applicable: false,
        complete_arm_vapor_mass_exchange_kg_m2: 0.0,
        complete_arm_cold_content_export_j_m2: 0.0,
        complete_arm_cold_content_export_applicable: false,
        complete_arm_available_ice_kg_m2: 0.0,
        complete_arm_available_ice_applicable: false,
        complete_arm_total_j_m2: 0.0,
        complete_arm_sequential_ledger_applicable: false,
        complete_arm_cold_energy_change_j_m2: 0.0,
        complete_arm_excess_energy_j_m2: 0.0,
        complete_arm_sublimation_kg_m2: 0.0,
        complete_arm_melt_kg_m2: 0.0,
        complete_arm_terminal_unallocated_j_m2: 0.0,
        complete_arm_terminal_unallocated_applicable: false,
        complete_arm_component_residual_j_m2: 0.0,
        complete_arm_maximum_thermodynamic_residual_j_m2: 0.0,
        hourly: [hour; 24],
    };
    let result = DirectSnowStage3EvaluationResult {
        authoritative,
        evaluation: Some(evaluation),
    };
    let _ = (hour, result);
}

fn section<'a>(document: &'a str, start: &str, end: &str) -> &'a str {
    let start_index = document
        .find(start)
        .unwrap_or_else(|| panic!("missing section start {start}"));
    let remainder = &document[start_index..];
    let end_index = remainder
        .find(end)
        .unwrap_or_else(|| panic!("missing section end {end}"));
    &remainder[..end_index]
}

#[test]
fn v129_admits_only_additive_default_off_operator_reconciliation() {
    let contract = read(CONTRACT);

    for required in [
        "contract_version: 129",
        "REF-SNOWFREEZE-STAGE3-OPERATOR-RECONCILIATION",
        "INV-SNOWFREEZE-096",
        "OBL-SNOWFREEZE-P-069",
        "OBL-SNOWFREEZE-C-011",
        "one boxed, enabled-only reconciliation companion",
        "internal schema-v6 row",
        "share one private Monin-Obukhov solver",
        "duplicated formulas, coefficients, convergence logic, or arithmetic order are prohibited",
        "Historical schema-v5 meaning remains unchanged",
        "disabled/default rows remain exact schema v4 with no companion allocation",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn v129_binds_exact_tuple_lineage_and_independent_reconstruction() {
    let contract = read(CONTRACT);

    for required in [
        "ordered duration-tagged tuples",
        "typed applicability instead of numeric-zero N/A",
        "`z_T/z_q/z_u/z_0,aero`",
        "solver options/status, stability length/corrections/log factors",
        "explicit albedo source/model/state",
        "active/lower cold-content change",
        "mixed sequential complete = comparable external subset + active internal conduction",
        "`F-S` projection and `Q-F` evolution effects",
        "without importing producer calculation helpers",
        "No production or public consumer may read schema-v6 evidence as state or",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn v129_retains_production_and_claim_holds() {
    let contract = read(CONTRACT);
    let index = read(INDEX);

    for required in [
        "no result may change equations, constants, thresholds, defaults, ownership, state, ledgers, fixtures, observations, or public outputs",
        "claim seasonal physical chronology, terminal closure, validation, promotion, CoE retirement, or cutover",
        "CoE ownership",
        "seasonal/terminal/cutover holds remain unchanged",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    for required in [
        "v129",
        "schema-v6 turbulent/state lineage",
        "historical schema-v5 semantics",
        "CoE ownership",
    ] {
        assert!(index.contains(required), "{INDEX} missing {required}");
    }
}

#[test]
fn admitted_protocol_freezes_support_bridge_and_decision_rules() {
    let package = read(PACKAGE);
    let protocol = read(PROTOCOL);

    for required in [
        "result-blind protocol admitted PASS/PASS",
        "same_state_paired_carrier_v1",
        "sequential_resolved_shadow_v1",
        "frozen_active_projection_reference",
        "CoE remains the sole authoritative production melt",
    ] {
        assert!(package.contains(required), "{PACKAGE} missing {required}");
    }
    for required in [
        "frozen_result_blind_admitted_pass_pass",
        "common_support_integration",
        "legacy_sequential_bridge",
        "ordered_three_way_support",
        "delta_projection=F-S and delta_evolution=Q-F",
        "INITIAL_CONTROL_VOLUME_PROJECTION_RECONCILES_SIGN_CONTRADICTION",
        "STATE_EVOLUTION_RECONCILES_SIGN_CONTRADICTION",
        "first active mass/depth/density/cold/temperature differs exactly",
        "MULTIFACTOR_UNRESOLVED is the exclusive fallback",
        "not promotion or cutover",
        "zero_alias_prohibited",
    ] {
        assert!(protocol.contains(required), "{PROTOCOL} missing {required}");
    }
}

#[test]
fn v129_canonical_addendum_pins_exact_algorithm_units_and_failures() {
    let contract = read(CONTRACT);
    let addendum = section(
        &contract,
        "## Stage 3 Turbulent Carrier And Operator Reconciliation Addendum",
        "## Wet-Compaction Operand Authority And Duplicate-Alias Closure Addendum",
    );
    let addendum = addendum.split_whitespace().collect::<Vec<_>>().join(" ");

    for required in [
        "SC-SNOWENERGY-001#INV-SNOWENERGY-029",
        "SC-SNOWENERGY-001#INV-SNOWENERGY-030",
        "SC-SNOWFREEZE-001#INV-SNOWFREEZE-085/086/094/095",
        "SC-SNOWENERGY-001#INV-SNOWENERGY-015/016/017/019/020/021/022/023/025/026/031",
        "aerodynamic_roughness_length_m",
        "longwave_model_id",
        "sublimation_model_id",
        "`site_id` is not duplicated into the snow row",
        "bare `z_0` remains a rejected alias",
        "snow_albedo_source_id",
        "stage3_default_snow_albedo_0p82",
        "snow_albedo_accumulated_positive_temperature_c_day` uses `degC day`",
        "24 * 60 = 1,440",
        "neither active snow forcing nor resolved snow",
        "all 24 statuses exactly `operator_not_selected`",
        "receipt-joinable zero sentinel source, forcing, geometry, and non-formulation fingerprints",
        "Disabled/default execution retains the existing inactive schema-v4 bytes",
        "post_substep_no_resolved_surface",
        "total_ice_mass_after_kg_m2",
        "- lower_cold_energy_change_j_m2 - cold_content_export_j_m2",
        "legacy_sequential_complete_j_m2",
        "fails without authoritative mutation",
        "frozen_active_projection_reference",
        "min(S_evaluated_seconds, 3600, Q_evaluated_seconds)",
        "delta_projection = F - S",
        "delta_evolution = Q - F",
        "`S < -tol`, `F > +tol`, and `Q > +tol`",
        "`S < -tol`, `F < -tol`, and `Q > +tol`",
        "+170.2536089 MJ m^-2",
        "SUPPORT_CENSORING_MATERIALLY_CONTRIBUTES",
        "A zero denominator makes the ratio N/A",
        "`PREDECESSOR_NOT_REPRODUCED` applies when the reconstructed legacy-bridge Snowbird median differs from `+170.2536089 MJ m^-2` by more than `1e-7 MJ m^-2` or any contributing water-year bridge exceeds its `TOL-SNOWFREEZE-019` scale-aware tolerance",
        "`LEGACY_ESTIMAND_INTERNAL_CONDUCTION_SIGN_DIFFERENCE` applies only when the reconstructed legacy sequential total is positive, the comparable sequential external subset is non-positive, and the external-plus-active-conduction bridge closes",
        "the first effective-input fingerprint differs exactly; the first active layer count, membership, or state fingerprint differs exactly; or the first active mass, depth, density, cold content, or temperature has different finite IEEE-754 `to_bits`",
        "`TOL-SNOWFREEZE-018` governs reconstruction and closure only; it cannot erase or create an initial-projection identity difference",
        "`MULTIFACTOR_UNRESOLVED` is the exclusive fallback and applies only when valid evidence emits none of the seven preceding classes, including positive Q when either ordered predecessor lies inside the inclusive zero band `[-tol, +tol]` or neither single ordered step uniquely crosses sign",
        "`LINEAGE_OR_IDENTITY_FAILURE`, `PREDECESSOR_NOT_REPRODUCED`, `LEGACY_ESTIMAND_INTERNAL_CONDUCTION_SIGN_DIFFERENCE`, `INITIAL_CONTROL_VOLUME_PROJECTION_DIFFERENCE`, `INITIAL_CONTROL_VOLUME_PROJECTION_RECONCILES_SIGN_CONTRADICTION`, `STATE_EVOLUTION_RECONCILES_SIGN_CONTRADICTION`, `SUPPORT_CENSORING_MATERIALLY_CONTRIBUTES`, then `MULTIFACTOR_UNRESOLVED`",
        "the six named non-lineage classes after `LINEAGE_OR_IDENTITY_FAILURE` and before `MULTIFACTOR_UNRESOLVED` may coexist in listed order",
        "The exclusive fallback requires that none of the seven preceding classes, comprising lineage failure plus those six non-lineage classes, is emitted and cannot coexist with any preceding class",
    ] {
        assert!(
            addendum.contains(required),
            "canonical addendum missing {required}"
        );
    }

    for status_row in [
        "`zero_wind` | `0` / null | `zero_wind`; corrections, friction, and fluxes zero",
        "`initial_potential_temperature_neutral` | `0` / null | `neutral`; nonzero-wind neutral state and fluxes",
        "`iterative_zero_buoyancy` | `>=1` / null | retained actual state; `neutral` if all corrections are exactly zero, `stable` if all are non-positive with at least one negative, `unstable` if all are non-negative with at least one positive; mixed/nonfinite corrections fail lineage",
        "`iterative_invalid_obukhov` | `>=1` / null | `indeterminate_obukhov`; retain the actual last state/corrections when computed length is nonfinite or zero",
        "`converged_stable` | `>=1` / positive finite | `stable`; final stable state/corrections",
        "`converged_unstable` | `>=1` / negative finite | `unstable`; final unstable state/corrections",
        "`did_not_converge` | typed error / N/A | no successful tuple; enabled request fails without authoritative mutation",
    ] {
        assert!(
            addendum.contains(status_row),
            "canonical status row missing {status_row}"
        );
    }

    let aliases = section(
        &contract,
        "## Symbol Alias Map",
        "## Allowed Degenerate States",
    );
    let aliases = aliases.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        "snow_stage3_operator_reconciliation_tuple",
        "scalar/vector diagnostic exception",
        "HillslopeProductionStateSymbol",
        "hourly radiation `MJ m^-2 * 10^6 / 3600 -> W m^-2`",
    ] {
        assert!(aliases.contains(required), "alias map missing {required}");
    }

    let tolerances = section(
        &contract,
        "## Tolerance and Numeric Notes",
        "## Stage 3 Evaluation Shadow Authority Addendum",
    );
    let tolerances = tolerances.split_whitespace().collect::<Vec<_>>().join(" ");
    for required in [
        "TOL-SNOWFREEZE-018",
        "max(1e-10 W m^-2, 1e-12 * sum_abs_operands)",
        "max(1e-12 kg m^-2, 1e-12 * sum_abs_operands)",
        "TOL-SNOWFREEZE-019",
        "1e-7 MJ m^-2",
        "support materiality ratio threshold `0.05`",
    ] {
        assert!(
            tolerances.contains(required),
            "tolerance table missing {required}"
        );
    }

    let variables = section(&contract, "## Variables and Units", "## Invariants");
    assert!(
        variables.contains("`degC day`")
            && variables.contains("snow_albedo_accumulated_positive_temperature_c_day"),
        "variables table must preserve albedo accumulation as degC day"
    );

    let operator_invariants = section(&contract, "| INV-SNOWFREEZE-094", "### HPHYS0298");
    assert!(
        !operator_invariants.contains("z_T/z_q/z_u/z_0`"),
        "operator invariants must reject bare z_0 geometry"
    );
    assert!(
        operator_invariants.matches("z_T/z_q/z_u/z_0,aero").count() >= 2,
        "INV-094 and INV-095 must both bind z_0,aero"
    );
}

#[test]
fn v129_binding_surfaces_each_retain_inv_096() {
    let contract = read(CONTRACT);
    let guard = section(&contract, "## Invariant Guard Map", "## Symbol Alias Map");
    let boundary = section(
        &contract,
        "## Boundary Disposition",
        "## Tolerance and Numeric Notes",
    );
    let exposure = section(&contract, "## Binding Exposure Index", "## Known Gaps");

    for (name, scoped) in [
        ("guard map", guard),
        ("boundary disposition", boundary),
        ("binding exposure", exposure),
    ] {
        assert!(
            scoped.contains("INV-SNOWFREEZE-096"),
            "{name} lost INV-SNOWFREEZE-096"
        );
    }
    for required in [
        "OBL-SNOWFREEZE-P-069",
        "OBL-SNOWFREEZE-C-011",
        "TOL-SNOWFREEZE-018",
        "TOL-SNOWFREEZE-019",
        "`flagged-binding-addition`",
    ] {
        assert!(
            exposure.contains(required),
            "binding exposure missing {required}"
        );
    }
}
