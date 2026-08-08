use std::fs;

use openwepp_hillslope_orchestrator::{
    DirectSnowLiquidPartition, DirectSnowStage3EvaluationDiagnostics,
    DirectSnowStage3EvaluationHourDiagnostics, DirectSnowStage3EvaluationResult,
    SnowStage3EvaluationOperator,
};

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const ENERGY_CONTRACT: &str =
    "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const PACKAGE: &str = "docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001/package.md";
const PROTOCOL: &str = "docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001/artifacts/protocol-freeze.json";
const PLAUSIBILITY_PACKAGE: &str = "docs/work-packages/20260807-snow-stage3-evolving-state-carrier-plausibility-reconciliation-001/package.md";
const PLAUSIBILITY_PROTOCOL: &str = "docs/work-packages/20260807-snow-stage3-evolving-state-carrier-plausibility-reconciliation-001/artifacts/protocol-freeze.json";
const RUNNER_SNOW_AUTHORITY: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs";

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

#[derive(Clone, Copy, Debug, PartialEq)]
struct BoundedVapor {
    deposition: f64,
    sublimation: f64,
    signed: f64,
}

fn independently_bound_vapor(raw: f64, active_ice_before: f64) -> BoundedVapor {
    let deposition = raw.max(0.0);
    let sublimation = (-raw).max(0.0).min(active_ice_before);
    BoundedVapor {
        deposition,
        sublimation,
        signed: deposition - sublimation,
    }
}

fn producer_transfer_is_valid(
    mutating_operator: bool,
    producer: Option<(f64, f64)>,
    independent: BoundedVapor,
) -> bool {
    match (mutating_operator, producer) {
        (false, None) => true,
        (false, Some(_)) | (true, None) => false,
        (true, Some((deposition, sublimation))) => {
            deposition >= 0.0
                && sublimation >= 0.0
                && !(deposition > 0.0 && sublimation > 0.0)
                && (deposition - independent.deposition).abs() <= 1.0e-12
                && (sublimation - independent.sublimation).abs() <= 1.0e-12
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct IndependentCarrier {
    active_cold_change: f64,
    lower_cold_change: f64,
    complete_raw: f64,
    excess_raw: f64,
    ice_available: f64,
    melt: f64,
    unallocated: f64,
}

fn independently_reconstruct_carrier(
    cold_before: f64,
    active_conduction: f64,
    external_raw: f64,
    active_ice_before: f64,
    sublimation: f64,
) -> IndependentCarrier {
    const LATENT_HEAT_FUSION_J_KG: f64 = 333_600.0;
    let cold_after_conduction = cold_before - active_conduction;
    let surface_change = if external_raw >= 0.0 {
        external_raw.min(cold_after_conduction)
    } else {
        external_raw
    };
    let active_cold_change = active_conduction + surface_change;
    let lower_cold_change = -active_conduction;
    let complete_raw = external_raw + active_conduction;
    let excess_raw = (complete_raw - active_cold_change).max(0.0);
    let ice_available = (active_ice_before - sublimation).max(0.0);
    let melt = (excess_raw / LATENT_HEAT_FUSION_J_KG).min(ice_available);
    let unallocated = (excess_raw - LATENT_HEAT_FUSION_J_KG * melt).max(0.0);
    IndependentCarrier {
        active_cold_change,
        lower_cold_change,
        complete_raw,
        excess_raw,
        ice_available,
        melt,
        unallocated,
    }
}

fn ice_endpoint_closes(ice_before: f64, ice_after: f64, vapor: BoundedVapor, melt: f64) -> bool {
    let expected = ice_before + vapor.deposition - vapor.sublimation - melt;
    (ice_after - expected).abs() <= 1.0e-12
}

fn assert_v131_normative_sections(snow: &str, energy: &str) {
    let snow_producer = section(snow, "## Producer Obligations", "## Consumer Obligations");
    let snow_consumer = section(snow, "## Consumer Obligations", "## Boundary Disposition");
    let snow_addendum = section(
        snow,
        "## Stage 3 Evolving-Carrier Plausibility Addendum",
        "## Wet-Compaction Operand Authority And Duplicate-Alias Closure Addendum",
    );
    let energy_addendum = section(
        energy,
        "### Raw opportunity versus bounded vapor transfer",
        "## Branch and Guard Table",
    );
    let energy_obligations = section(
        energy,
        "## Producer and Consumer Obligations",
        "## Symbol Alias Map",
    );

    assert!(snow_producer.contains("OBL-SNOWFREEZE-P-071"));
    assert!(snow_consumer.contains("OBL-SNOWFREEZE-C-013"));
    for required in ["OBL-SNOWENERGY-P-007", "OBL-SNOWENERGY-C-014"] {
        assert!(energy_obligations.contains(required));
    }
    for required in [
        "Q_latent_raw = latent_flux_w_m2 * duration_seconds = m_v,raw * L_s(T_s)",
        "Q_latent_bounded = m_v * L_s(T_s)",
        "m_melt_raw_carrier = min(Q_excess_raw / L_f, m_ice_available)",
        "relabel `Q_latent_raw` or `m_melt_raw_carrier`",
    ] {
        assert!(energy_addendum.contains(required));
    }
    for required in [
        "Producer-versus-",
        "independent disagreement is invalid evidence",
        "VAPOR_OPPORTUNITY_TRANSFER_MISMATCH",
        "All window totals precede",
    ] {
        assert!(snow_addendum.contains(required));
    }
}

#[test]
fn v130_retains_additive_default_off_operator_reconciliation() {
    let contract = read(CONTRACT);

    for required in [
        "contract_version: 134",
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
fn v130_retains_exact_tuple_lineage_and_independent_reconstruction() {
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
fn v130_retains_production_and_claim_holds() {
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
    for required in ["v134", "schema-v7 evidence", "production", "cutover"] {
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

fn assert_canonical_addendum(contract: &str) {
    let addendum = section(
        contract,
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
        "`PREDECESSOR_NOT_REPRODUCED` applies only to a fixed-forcing comparison",
        "`FORCING_IDENTITY_DIFFERENCE` and cannot support source/state attribution",
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
}

fn assert_canonical_tables(contract: &str) {
    let aliases = section(
        contract,
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
        contract,
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

    let variables = section(contract, "## Variables and Units", "## Invariants");
    assert!(
        variables.contains("`degC day`")
            && variables.contains("snow_albedo_accumulated_positive_temperature_c_day"),
        "variables table must preserve albedo accumulation as degC day"
    );

    let operator_invariants = section(contract, "| INV-SNOWFREEZE-094", "### HPHYS0298");
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
fn v130_canonical_addendum_retains_exact_algorithm_units_and_failures() {
    let contract = read(CONTRACT);
    assert_canonical_addendum(&contract);
    assert_canonical_tables(&contract);
}

#[test]
fn inactive_operator_lifecycle_returns_before_hourly_forcing_acquisition() {
    let source = read(RUNNER_SNOW_AUTHORITY);
    let function = section(
        &source,
        "    fn snow_liquid_partition(",
        "\n}\n\nfn inactive_direct_snow_evaluation_result(",
    );
    let inactive_guard = function
        .find("if !Self::active_forcing(hyetograph_rainfall_m, snow_lane_state.runtime_swe_m)?")
        .expect("inactive guard");
    let inactive_return = function
        .find("return Ok(DirectProductionSnowPartitionResult {")
        .expect("inactive return");
    let forcing_acquisition = function
        .find(".direct_winter_hourly_forcing(")
        .expect("typed hourly forcing acquisition");

    assert!(inactive_guard < inactive_return);
    assert!(inactive_return < forcing_acquisition);
    assert!(function.contains("self.snow_stage3_evaluation_operator,"));
    assert!(function.contains("&& !persistent_requested"));
}

#[test]
fn v130_binding_surfaces_each_retain_inv_096() {
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

#[test]
fn v131_binds_raw_opportunity_separately_from_bounded_transfer() {
    let snow = read(CONTRACT);
    let energy = read(ENERGY_CONTRACT);
    let protocol = read(PLAUSIBILITY_PROTOCOL);

    let snow_guard = section(&snow, "## Invariant Guard Map", "## Symbol Alias Map");
    let snow_boundary = section(
        &snow,
        "## Boundary Disposition",
        "## Tolerance and Numeric Notes",
    );
    let snow_exposure = section(&snow, "## Binding Exposure Index", "## Known Gaps");
    let energy_invariants = section(&energy, "### Invariants", "### Guard Map");
    let energy_guard = section(
        &energy,
        "### Guard Map",
        "## Producer and Consumer Obligations",
    );
    let energy_alias = section(
        &energy,
        "## Symbol Alias Map",
        "## Constants and Parameters",
    );
    let energy_units = section(
        &energy,
        "## Unit Governance Map",
        "## Tolerance and Numeric Notes",
    );
    assert_v131_normative_sections(&snow, &energy);

    for required in [
        "contract_version: 134",
        "INV-SNOWFREEZE-098",
        "OBL-SNOWFREEZE-P-071",
        "OBL-SNOWFREEZE-C-013",
        "Stage 3 Evolving-Carrier Plausibility Addendum",
        "VAPOR_OPPORTUNITY_TRANSFER_MISMATCH",
        "`S/F` actual transfer is N/A",
        "Any such field blocks",
    ] {
        assert!(snow.contains(required), "{CONTRACT} missing {required}");
    }
    for required in [
        "contract_version: 11",
        "`m_v,raw`",
        "Signed bounded vapor transfer",
        "INV-SNOWENERGY-032",
        "OBL-SNOWENERGY-P-007",
        "OBL-SNOWENERGY-C-014",
        "latent_flux_w_m2` is raw turbulent latent-energy",
        "Numeric zero cannot",
        "Q_latent_raw = latent_flux_w_m2 * duration_seconds = m_v,raw * L_s(T_s)",
        "Q_latent_bounded = m_v * L_s(T_s)",
        "Q_latent_truncation = Q_latent_raw - Q_latent_bounded",
        "C1 = C0 - G",
        "surface_change = if E_raw >= 0 then min(E_raw, C1) else E_raw",
        "m_melt_raw_carrier = min(Q_excess_raw / L_f, m_ice_available)",
        "relabel `Q_latent_raw` or `m_melt_raw_carrier`",
    ] {
        assert!(
            energy.contains(required),
            "{ENERGY_CONTRACT} missing {required}"
        );
    }
    for scoped in [energy_invariants, energy_guard] {
        assert!(scoped.contains("INV-SNOWENERGY-032"));
    }
    for scoped in [snow_guard, snow_boundary, snow_exposure] {
        assert!(scoped.contains("INV-SNOWFREEZE-098"));
    }
    for scoped in [energy_alias, energy_units] {
        assert!(scoped.contains("`m_v,raw`"));
        assert!(scoped.contains("`m_v`"));
        assert!(scoped.contains("vapor_mass_exchange_kg_m2"));
        assert!(scoped.contains("deposition_kg_m2"));
        assert!(scoped.contains("sublimation_kg_m2"));
    }
    assert!(!energy_alias.contains("| `m_v` | `vapor_mass_exchange_kg_m2` |"));
    for required in [
        "independent_bounded_vapor_transfer",
        "producer_transfer_comparison",
        "independent_melt",
        "INVALID_EVIDENCE_PACKAGE_HOLD",
        "valid_raw_capacity_truncation",
    ] {
        assert!(
            protocol.contains(required),
            "{PLAUSIBILITY_PROTOCOL} missing {required}"
        );
    }
}

#[test]
#[allow(clippy::float_cmp)]
fn v131_contract_vectors_distinguish_raw_bounded_and_na_transfer() {
    let no_cap = independently_bound_vapor(-2.0, 5.0);
    assert_eq!(
        no_cap,
        BoundedVapor {
            deposition: 0.0,
            sublimation: 2.0,
            signed: -2.0,
        }
    );
    assert_eq!(no_cap.signed, -2.0);

    let truncated = independently_bound_vapor(-7.0, 3.0);
    assert_eq!(truncated.sublimation, 3.0);
    assert_eq!(truncated.signed, -3.0);
    assert_eq!(-7.0_f64 - truncated.signed, -4.0);

    let deposition = independently_bound_vapor(1.25, 0.5);
    assert_eq!(deposition.deposition, 1.25);
    assert_eq!(deposition.sublimation, 0.0);
    assert_eq!(deposition.signed, 1.25);

    assert!(producer_transfer_is_valid(
        true,
        Some((0.0, 3.0)),
        truncated
    ));
    assert!(!producer_transfer_is_valid(
        true,
        Some((0.0, 2.5)),
        truncated
    ));
    assert!(!producer_transfer_is_valid(
        true,
        Some((3.0, 0.0)),
        truncated
    ));
    assert!(!producer_transfer_is_valid(
        true,
        Some((0.5, 3.0)),
        truncated
    ));
    assert!(!producer_transfer_is_valid(true, None, truncated));
    assert!(producer_transfer_is_valid(false, None, truncated));
    assert!(!producer_transfer_is_valid(
        false,
        Some((0.0, 0.0)),
        truncated
    ));
}

#[test]
#[allow(clippy::float_cmp)]
fn v131_contract_vectors_reconstruct_operator_order_before_endpoints() {
    let vapor = independently_bound_vapor(-0.25, 1.0);
    let carrier = independently_reconstruct_carrier(100.0, 20.0, 500_000.0, 1.0, 0.25);

    assert_eq!(carrier.active_cold_change, 100.0);
    assert_eq!(carrier.lower_cold_change, -20.0);
    assert_eq!(carrier.complete_raw, 500_020.0);
    assert_eq!(carrier.excess_raw, 499_920.0);
    assert_eq!(carrier.ice_available, 0.75);
    assert_eq!(carrier.melt, 0.75);
    assert_eq!(carrier.unallocated, 249_720.0);

    let without_sublimation = independently_reconstruct_carrier(100.0, 20.0, 500_000.0, 1.0, 0.0);
    assert_eq!(without_sublimation.melt, 1.0);
    assert_eq!(without_sublimation.unallocated, 166_320.0);

    assert!(ice_endpoint_closes(1.0, 0.0, vapor, carrier.melt));
    assert!(ice_endpoint_closes(
        1.0,
        0.0,
        BoundedVapor {
            deposition: 0.0,
            sublimation: 0.75,
            signed: -0.75,
        },
        0.25,
    ));
    assert_ne!(vapor.sublimation, 0.75);
    assert_ne!(carrier.melt, 0.25);
    assert!(!producer_transfer_is_valid(true, Some((0.0, 0.75)), vapor));
}

#[test]
fn v131_retains_fail_closed_authority_gaps_and_protected_boundaries() {
    let snow = read(CONTRACT);
    let energy = read(ENERGY_CONTRACT);
    let package = read(PLAUSIBILITY_PACKAGE);
    let index = read(INDEX);

    for required in [
        "GRIDMET/CLI wind custody",
        "`NOT_EVALUABLE`/`AUTHORITY_MISSING`",
        "No fixed wind attenuation",
        "No production correction",
        "No production/public consumer",
        "CoE retirement",
    ] {
        assert!(snow.contains(required), "{CONTRACT} missing {required}");
    }
    assert!(energy.contains("makes no production correction"));
    assert!(package.contains("Quantify an isolated consequence only if"));
    assert!(package.contains("otherwise record implementation/reference parity"));
    assert!(
        index.contains(
            "v134 admits only default-off evaluator-owned persistent continuity mechanics"
        )
    );
    assert!(index.contains("v11 recovers direct retained-value custody"));
}
