use std::fs;

const ENERGY: &str = "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md";
const FREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const FIXED_POINT: &str = "crates/openwepp-hillslope-orchestrator/src/v11_covered/fixed_point.rs";
const OPEN_SNOW: &str = "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow.rs";
const OPEN_SNOW_PHYSICAL_SUPPORT: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_physical_support.rs";
const OPEN_SNOW_TESTS: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_tests.rs";
const OPEN_SNOW_V51_TESTS: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v51_tests.rs";
const OPEN_SNOW_V52_TESTS: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v52_tests.rs";
const OPEN_SNOW_V53_TESTS: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v53_tests.rs";
const OPEN_SNOW_V54_TESTS: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v54_tests.rs";
const OPEN_SNOW_V55_TESTS: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v55_tests.rs";
const OPEN_SNOW_V56_TESTS: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_v56_tests.rs";
const COUPLED_SOLVE: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/phase_consistent_coupled_solve.rs";
const PRIVATE_Q_LATTICE: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/phase_consistent_private_q_lattice.rs";
const TEMPERATURE_PRIMARY: &str = "crates/openwepp-hillslope-orchestrator/src/v11_covered/phase_consistent_temperature_primary.rs";
const SNOW_ENTHALPY_CARRY: &str =
    "crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_snow_enthalpy_carry.rs";
const SNOW_RESTART_V5: &str =
    "crates/openwepp-persisted-restart-v1/src/snow_stage3_v11_v5_snow_enthalpy.rs";
const ORCHESTRATOR_LIB: &str = "crates/openwepp-hillslope-orchestrator/src/lib.rs";
const EXACT_DYADIC_ENTHALPY: &str =
    "crates/openwepp-land-surface-energy/src/exact_dyadic_enthalpy.rs";
const REAL_HYDROLOGY: &str = "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/real_hydrology_execution.rs";
const COVERED_V8_OWNER: &str =
    "crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_v8_owner.rs";
const V10_SOIL_THERMAL_TESTS: &str = "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2_tests.rs";
const V10_SOIL_THERMAL_V49_TESTS: &str = "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2_v49_tests.rs";
const V10_SOIL_THERMAL: &str =
    "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2.rs";
const OWNER_FINALIZATION: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/owner_finalization.rs";
const OWNER_FINALIZATION_V50_TRANSITION: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/owner_finalization_v50_transition.rs";
const PARENT_CHRONOLOGY_TESTS: &str =
    "crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_parent_chronology_tests.rs";
const PACKAGE: &str =
    "docs/work-packages/20260807-snow-terminal-enthalpy-event-numerics-001/package.md";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

fn read_optional(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

#[test]
fn contracts_admit_only_event_local_terminal_snow_numerics() {
    let energy = read(ENERGY);
    let freeze = read(FREEZE);
    for required in [
        "contract_version: 57",
        "Version 18 defines the persistent Stage 3 snow--soil conductive boundary",
        "INV-SNOWENERGY-034",
        "OBL-SNOWENERGY-P-008",
        "OBL-SNOWENERGY-C-015",
        "TOL-SNOWENERGY-001",
        "H=-Q_cc+L_f m_l",
        "g(tau)=m_i,start+m_refrozen(tau)+m_deposition(tau)-m_sublimation(tau)-m_melt(tau)",
        "h_min=1e-9 s",
        "safeguarded bisection",
        "unevaluated_seconds=requested-t_event",
        "neither is a land-surface recipient",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
    for required in [
        "contract_version: 140",
        "| `2026-08-07` | `135` | `Codex` |",
        "INV-SNOWFREEZE-101",
        "OBL-SNOWFREEZE-P-073",
        "OBL-SNOWFREEZE-C-015",
        "TOL-SNOWFREEZE-022",
        "Enabled internal schema v8",
        "Ordinary persistent evaluation remains exact schema v7",
    ] {
        assert!(freeze.contains(required), "{FREEZE} missing {required}");
    }
}

#[test]
fn v49_contract_binds_three_domain_prepared_install_authority() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V49",
        "INV-SNOWENERGY-073",
        "OBL-SNOWENERGY-C-041",
        "source 42",
        "resident is transaction 43",
        "target 44 with predecessor 43",
        "opaque authenticated prepared-install authority",
        "no successor arithmetic",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v49_real_finalizer_requires_exact_resident_predecessor_authority() {
    let production = read(V10_SOIL_THERMAL);
    let finalization = read(OWNER_FINALIZATION);
    let behavior = format!(
        "{}\n{}",
        read(V10_SOIL_THERMAL_TESTS),
        read(V10_SOIL_THERMAL_V49_TESTS),
    );
    let mut defects = Vec::new();
    for required in [
        "DirectSoilThermalPreparedBeginningInstallAuthorityV2",
        "authenticate_soil_thermal_prepared_beginning_install_authority_v3",
        "install_soil_thermal_accepted_v2_from_authenticated_beginning_v3",
        "AuthenticatedPreparedBeginning",
        "validated_authenticated_prepared_accepted_resident_v2",
        "V2 multi-child prepared-beginning authoritative source join",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-49 production seam {required}"));
        }
    }
    for required in [
        "authenticate_soil_thermal_prepared_beginning_install_authority_v3",
        "install_soil_thermal_accepted_v2_from_authenticated_beginning_v3",
    ] {
        if !finalization.contains(required) {
            defects.push(format!("missing version-49 real-finalizer seam {required}"));
        }
    }
    for required in [
        "v49_r124_three_domain_prepared_install_succeeds",
        "v49_repeated_same_parent_soil_successors_remain_exact",
        "v49_prepared_install_authority_refuses_resident_and_prepared_substitution",
        "v49_prepared_install_authority_refuses_accepted_and_authority_substitution",
        "v49_prepared_install_rolls_back_noops_and_never_publishes",
        "install_v2_soil_from_authenticated_prepared_beginning_v1",
        "jointly rebased outer owners must not replace the authoritative source",
        "authoritative resident {label} poison must refuse",
    ] {
        if !behavior.contains(required) {
            defects.push(format!("missing version-49 behavior obligation {required}"));
        }
    }
    for forbidden in [
        "DFF_V49",
        "R124_PREDECESSOR_AUDIT",
        "soil_predecessor_transaction_id.0 + 1",
        "source_transaction_id.0 + 1",
        "wrapping_add(1)",
        "saturating_add(1)",
    ] {
        if production.contains(forbidden) || finalization.contains(forbidden) {
            defects.push(format!("forbidden version-49 seam {forbidden}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-49 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v50_contract_binds_validated_envelope_ending_source() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V50",
        "INV-SNOWENERGY-074",
        "OBL-SNOWENERGY-C-042",
        "vegetation 41, LSE 40, BGC 41",
        "UncommittedCoveredV8OwnerEnvelope.transaction_id()",
        "heterogeneous authenticated beginning",
        "no source is inferred from numeric adjacency",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v50_real_finalizer_passes_exact_envelope_source_anchor() {
    let production = read(V10_SOIL_THERMAL);
    let finalization = read(OWNER_FINALIZATION);
    let transition = read(OWNER_FINALIZATION_V50_TRANSITION);
    let behavior = read(V10_SOIL_THERMAL_V49_TESTS);
    let real_behavior = read(PARENT_CHRONOLOGY_TESTS);
    let envelope = read(COVERED_V8_OWNER);
    let mut defects = Vec::new();
    for required in [
        "authenticated_complete_owner_source_transaction_id",
        "DirectSoilThermalOuterOwnerTransitionAuthorityV2",
        "authenticate_soil_thermal_prepared_beginning_install_authority_v4",
        "install_soil_thermal_accepted_v2_from_authenticated_beginning_v4",
        "V2 multi-child prepared-beginning envelope source join",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-50 production seam {required}"));
        }
    }
    for required in [
        "AuthenticatedCoveredV8OuterOwnerTransitionV1",
        "envelope.transaction_id()",
        "stage_unpublished_v2_carrier_owners(&mut expected, envelope)",
    ] {
        if !transition.contains(required) {
            defects.push(format!("missing version-50 real-finalizer seam {required}"));
        }
    }
    for required in [
        "authenticate_v50_covered_v8_outer_owner_transition_v1(beginning, envelope)",
        "authenticate_soil_thermal_outer_owner_transition_v2(\n                &authenticated_outer_owner_transition,",
        "install_v2_soil_from_authenticated_prepared_beginning_v2(",
    ] {
        if !finalization.contains(required) {
            defects.push(format!("missing version-50 real-finalizer call {required}"));
        }
    }
    for required in [
        "v50_mixed_beginning_uses_exact_envelope_source",
        "v50_envelope_source_and_candidate_owner_poisons_refuse",
    ] {
        if !behavior.contains(required) {
            defects.push(format!("missing version-50 behavior obligation {required}"));
        }
    }
    for required in [
        "v50_envelope_transaction_join_refuses_each_owner_substitution",
        "v50_envelope_material_receipt_substitution_refuses",
    ] {
        if !envelope.contains(required) {
            defects.push(format!(
                "missing version-50 envelope poison obligation {required}"
            ));
        }
    }
    if !real_behavior.contains("v50_native_v2_real_finalizer_uses_validated_envelope_transition") {
        defects.push("missing version-50 executed native-V2 real-finalizer vector".to_owned());
    }
    let v50_production = production
        .rsplit("pub fn authenticate_soil_thermal_prepared_beginning_install_authority_v4(")
        .next()
        .expect("V50 production authority")
        .split("pub(crate) fn authenticate_soil_thermal_outer_owner_transition_v2(")
        .next()
        .expect("V50 production authority body");
    let v50_finalization = transition
        .rsplit("fn authenticate_v50_covered_v8_outer_owner_transition_v1(")
        .next()
        .expect("V50 finalizer")
        .split("pub(crate) fn digest32_hex")
        .next()
        .expect("V50 finalizer body");
    for forbidden in [
        "direct_soil_thermal_complete_source_transaction_v2(authoritative_beginning)",
        "DFF_V50_R129",
        "source_transaction_id.0 + 1",
        "normalize_v11_staged_parent_lineage(&mut expected",
    ] {
        if v50_production.contains(forbidden) || v50_finalization.contains(forbidden) {
            defects.push(format!("forbidden version-50 seam {forbidden}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-50 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v51_contract_binds_post_crossing_alternating_contraction() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V51",
        "INV-SNOWENERGY-075",
        "OBL-SNOWENERGY-C-043",
        "alternating within-phase contraction",
        "strictly smaller than the preceding magnitude",
        "exact 60-second floor",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v51_post_crossing_contraction_production_seams_are_required() {
    let coupled = read(COUPLED_SOLVE);
    let open_snow = read(OPEN_SNOW);
    let behavior = format!("{}\n{}", read(OPEN_SNOW_TESTS), read(OPEN_SNOW_V51_TESTS));
    let production = format!("{coupled}\n{open_snow}");
    let mut defects = Vec::new();
    for required in [
        "covered_one_way_post_crossing_contraction_eligibility_v1",
        "post_crossing_previous_direction",
        "previous_step_magnitude",
        "canonical_boundary_crossings",
        "covered_one_way_phase_boundary_eligibility_v1",
        "phase_consistent_coupled_physical_solve_v1",
        "phase_consistent_coupled_root_polish_v1",
        "covered_authentic_receipt_stabilize_v1",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-51 production seam {required}"));
        }
    }
    for required in [
        "v51_post_crossing_contraction_accepts_exact_r132_chain",
        "v51_post_crossing_contraction_refuses_pre_crossing_reversal_or_predicate_recross",
        "v51_post_crossing_contraction_refuses_noncontracting_equal_nonalternating_or_nonfinite",
        "v51_post_crossing_contraction_retains_water_static_cadence_side_raw_owner_budget_guards",
        "v51_post_crossing_contraction_dispatch_retains_unchanged_authentic_solver",
    ] {
        if !behavior.contains(required) {
            defects.push(format!("missing version-51 behavior obligation {required}"));
        }
    }
    for forbidden in [
        "DFF_V51",
        "receipt_bit_distance",
        "support_start_ns() - 1",
        "STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS / 2",
    ] {
        if production.contains(forbidden) {
            defects.push(format!("forbidden version-51 seam {forbidden}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-51 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v52_contract_binds_explicit_cn_heat_coordinate() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V52",
        "INV-SNOWENERGY-076",
        "OBL-SNOWENERGY-C-044",
        "positive into snow",
        "R_Q=Q_cn,1-Q_cn,physical",
        "unchanged existing lane energy tolerance",
        "exact 60-second floor",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v52_cn_heat_coordinate_production_seams_are_required() {
    let production = format!("{}\n{}", read(COUPLED_SOLVE), read(OPEN_SNOW));
    let behavior = read(OPEN_SNOW_V52_TESTS);
    let mut defects = Vec::new();
    for required in [
        "r_q_cn_j_m2",
        "snow_candidate_cn_heat_j_m2",
        "covered_cn_heat_coordinate_residual_evaluate_v1",
        "phase_consistent_coupled_physical_solve_v1",
        "covered_authentic_receipt_stabilize_v1",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-52 production seam {required}"));
        }
    }
    for required in [
        "v52_cn_heat_coordinate_closes_r134_receipt_two_cycle",
        "v52_cn_heat_coordinate_preserves_zero_heat_and_order",
        "v52_cn_heat_coordinate_refuses_omission_sign_and_static_geometry_poison",
        "v52_cn_heat_coordinate_retains_shared_budget_exact_receipt_replay_and_rollback",
        "v52_cn_heat_coordinate_never_repairs_or_publishes_receipts",
    ] {
        if !behavior.contains(required) {
            defects.push(format!("missing version-52 behavior obligation {required}"));
        }
    }
    for forbidden in [
        "DFF_V52",
        "receipt_bit_distance",
        "average_receipt",
        "interpolate_receipt",
        "STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS / 2",
    ] {
        if production.contains(forbidden) {
            defects.push(format!("forbidden version-52 seam {forbidden}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-52 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v53_contract_binds_same_map_cn_heat_seed() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V53",
        "INV-SNOWENERGY-077",
        "OBL-SNOWENERGY-C-045",
        "exact already-produced endpoint Stage 3 and endpoint soil candidates associated with that solver dispatch",
        "representational and uncharged",
        "maximum-96 budget",
        "exact 60-second floor",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v53_same_map_cn_heat_seed_production_seams_are_required() {
    let production = format!("{}\n{}", read(COUPLED_SOLVE), read(OPEN_SNOW));
    let behavior = read(OPEN_SNOW_V53_TESTS);
    let mut defects = Vec::new();
    for required in [
        "covered_phase_consistent_same_map_cn_heat_seed_v1",
        "endpoint_seed_snow_soil_receipts",
        "phase_consistent_coupled_physical_solve_v1",
        "covered_authentic_receipt_stabilize_v1",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-53 production seam {required}"));
        }
    }
    for required in [
        "v53_same_map_cn_heat_seed_uses_endpoint_receipts",
        "v53_same_map_cn_heat_seed_rebinds_legacy_seed",
        "v53_same_map_cn_heat_seed_refuses_cross_map_receipt_substitution",
        "v53_same_map_cn_heat_seed_refuses_lane_and_finite_poison",
        "v53_same_map_cn_heat_seed_is_uncharged_and_retains_admission_guards",
    ] {
        if !behavior.contains(required) {
            defects.push(format!("missing version-53 behavior obligation {required}"));
        }
    }
    for forbidden in [
        "DFF_V53",
        "accepted_snow_soil_receipts.get(lane_id).unwrap",
        "average_cn_heat_seed",
        "repair_cn_heat_seed",
        "STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS / 2",
    ] {
        if production.contains(forbidden) {
            defects.push(format!("forbidden version-53 seam {forbidden}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-53 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v54_contract_binds_representable_receipt_cycle_witness() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V54",
        "INV-SNOWENERGY-078",
        "OBL-SNOWENERGY-C-046",
        "deterministic first-seen cycle order",
        "one to three members",
        "exact high-plus-carry E",
        "same-coordinate/same-input independent replay",
        "Q-only enumeration is forbidden",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v54_receipt_cycle_witness_production_seams_are_required() {
    let production = format!("{}\n{}", read(COUPLED_SOLVE), read(OPEN_SNOW));
    let behavior = read(OPEN_SNOW_V54_TESTS);
    let mut defects = Vec::new();
    for required in [
        "CoveredAuthenticReceiptExactCycleV1",
        "covered_authentic_receipt_stabilize_or_cycle_v1",
        "covered_receipt_cycle_endpoint_coordinates_v1",
        "covered_authentic_receipt_cycle_endpoint_witness_v1",
        "stable_monotone_stage_coordinates_v1",
        "stable_monotone_v2_carry_coordinates_v1",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-54 production seam {required}"));
        }
    }
    for required in [
        "v54_exact_cycle_chronology_retains_first_seen_members",
        "v54_endpoint_projection_uses_each_members_own_artifacts_and_receipt",
        "v54_q_only_variation_cannot_change_authentic_physical_map",
        "v54_cycle_witness_preflights_all_members_plus_replay",
        "v54_cycle_witness_refuses_poison_and_rolls_back_without_publication",
    ] {
        if !behavior.contains(required) {
            defects.push(format!("missing version-54 behavior obligation {required}"));
        }
    }
    for forbidden in [
        "DFF_V54",
        "nextafter",
        "average_receipt_cycle",
        "interpolate_receipt_cycle",
        "repair_receipt_cycle",
        "STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS / 2",
    ] {
        if production.contains(forbidden) {
            defects.push(format!("forbidden version-54 seam {forbidden}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-54 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v55_contract_binds_private_q_lattice_witness() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V55",
        "INV-SNOWENERGY-079",
        "OBL-SNOWENERGY-C-047",
        "canonical reconstructed endpoint receipt",
        "root-exclusive/own-output-inclusive cardinality",
        "protected authentic probe and independent replay",
        "Authentic probes/replay consume their supplied sealed receipt Q unchanged",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v55_private_q_lattice_production_seams_are_required() {
    let production = format!("{}\n{}", read(COUPLED_SOLVE), read(PRIVATE_Q_LATTICE));
    let behavior = read(OPEN_SNOW_V55_TESTS);
    let mut defects = Vec::new();
    for required in [
        "physical_q_cn_j_m2",
        "covered_private_q_lattice_witness_v1",
        "PrivateQLatticeExactWitness",
        "let Some(required) = candidate_count",
        ".checked_add(COVERED_ROOT_POLISH_RECEIPT_RESERVE_V1)",
        "candidate_r_q.to_bits() == 0.0_f64.to_bits()",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-55 production seam {required}"));
        }
    }
    for required in [
        "v55_private_q_lattice_closes_r140_before_polish",
        "v55_private_q_lattice_requires_exactly_one_charge_per_candidate",
        "v55_private_q_lattice_descends_deterministically_and_fails_closed_without_witness",
        "v55_private_q_lattice_preflight_is_atomic_exact_fit_and_one_short",
        "v55_private_q_lattice_refuses_lineage_interval_and_coordinate_poisons",
        "v55_private_q_lattice_refuses_multi_q_and_preserves_authentic_receipt_q",
        "v55_private_q_lattice_refuses_merit_z_side_and_artifact_custody_poisons",
        "v55_r142_overcapacity_is_zero_charge_then_v45_polish_continues",
        "v55_hard_shape_and_post_commit_failures_never_fall_back",
        "v55_private_q_lattice_contract_and_source_forbid_authentic_repair",
    ] {
        if !behavior.contains(required) {
            defects.push(format!("missing version-55 behavior obligation {required}"));
        }
    }
    for forbidden in [
        "DFF_V54_R140",
        "nextafter",
        "average_private_q",
        "repair_private_q",
        "STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS / 2",
    ] {
        if production.contains(forbidden) {
            defects.push(format!("forbidden version-55 seam {forbidden}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-55 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v56_contract_binds_frozen_temperature_primary_compound_owner() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V56",
        "INV-SNOWENERGY-080",
        "OBL-SNOWENERGY-C-048",
        "strictly frozen and noncrossing",
        "(W,T_s,rho)",
        "H_exact=-exact(W)*exact(c_ice)*(exact(273.15)-exact(T_s))",
        "round-nearest-even",
        "AuthenticatedCoveredSnowMaterialOwnerV1",
        "CoveredSnowEnthalpyCarryReceiptV1",
        "committed, pending, and in-progress",
        "canonical exact zero carry",
        "nonzero carry blocks downgrade",
        "before V55",
        "shared maximum 96",
        "60-second floor",
        "161712621295b503da41b065846304ce0e0198a26a9d9b97efa6d4012fa36c65",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v56_frozen_temperature_primary_production_seams_are_required() {
    let production = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        read(COUPLED_SOLVE),
        read(OPEN_SNOW),
        read(FIXED_POINT),
        read_optional(TEMPERATURE_PRIMARY),
        read_optional(SNOW_ENTHALPY_CARRY),
        read_optional(SNOW_RESTART_V5),
        read(ORCHESTRATOR_LIB),
        read(EXACT_DYADIC_ENTHALPY),
    );
    let behavior = read_optional(OPEN_SNOW_V56_TESTS);
    let mut defects = Vec::new();
    for required in [
        "CoveredFrozenTemperaturePrimaryEligibilityV1",
        "covered_frozen_temperature_primary_eligibility_v1",
        "CoveredFrozenTemperaturePrimarySolveV1",
        "covered_frozen_temperature_primary_solve_v1",
        "AuthenticatedCoveredSnowMaterialOwnerV1",
        "CoveredSnowEnthalpyCarryReceiptV1",
        "DirectSnowStage3V11SnowEnthalpyRestartV5",
        "round_nearest_even",
        "phase_consistent_temperature_primary",
        "snow_stage3_v11_snow_enthalpy_carry",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-56 production seam {required}"));
        }
    }
    for required in [
        "v56_frozen_temperature_primary_dispatches_before_v55",
        "v56_exact_enthalpy_rounds_high_nearest_even_and_retains_carry",
        "v56_cn_heat_is_derived_and_consumed_exactly_once",
        "v56_compound_owner_stabilizes_whole_receipt_and_replays_exactly",
        "v56_refuses_phase_crossing_liquid_event_and_static_join_poisons",
        "v56_refuses_transient_v54_v55_witness_promotion",
        "v56_restart_round_trips_committed_pending_and_in_progress",
        "v56_restart_migrates_zero_carry_and_refuses_nonzero_downgrade",
        "v56_shared_budget_floor_ledger_and_rollback_are_unchanged",
    ] {
        if !behavior.contains(required) {
            defects.push(format!("missing version-56 behavior obligation {required}"));
        }
    }
    for forbidden in [
        "DFF_V55_R144",
        "promote_private_q_lattice_witness",
        "repair_snow_enthalpy_carry",
        "STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS / 2",
    ] {
        if production.contains(forbidden) {
            defects.push(format!("forbidden version-56 seam {forbidden}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-56 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v57_contract_binds_bounded_liquid_eligibility_without_normalization() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V57",
        "INV-SNOWENERGY-081",
        "OBL-SNOWENERGY-C-049",
        "1.0e-12 kg m^-2 OFE-ground",
        "eligibility-neutral only",
        "immediately before V55",
        "retains the shared budget already used",
        "No external liquid or refreeze is zeroed, clamped, dropped",
        "1b95e317d65cf831933ef7778f20c7295ef2e590a199b986fcfee8dc97b759fc",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v57_bounded_liquid_eligibility_and_post_root_transition_are_required() {
    let production = format!("{}\n{}", read(TEMPERATURE_PRIMARY), read(OPEN_SNOW),);
    let behavior = read_optional(OPEN_SNOW_V56_TESTS);
    let mut defects = Vec::new();
    for required in [
        "COVERED_FROZEN_EXTERNAL_LIQUID_ELIGIBILITY_MAX_KG_M2_V1",
        "covered_frozen_external_liquid_eligibility_neutral_v1",
        "covered_frozen_temperature_primary_post_root_transition_v1",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-57 production seam {required}"));
        }
    }
    for required in [
        "v57_external_liquid_zero_and_inclusive_boundary_are_eligible_without_mutation",
        "v57_external_liquid_one_bit_above_negative_and_nonfinite_are_ineligible_zero_charge",
        "v57_bounded_refreeze_is_exactly_ledgered_without_phase_or_event_change",
        "v57_post_root_transition_retains_used_budget_and_precedes_v55",
        "v57_post_charge_failure_never_falls_back_to_v55",
        "v57_canonical_r147_operand_bits_remain_in_mass_energy_receipts",
    ] {
        if !behavior.contains(required) {
            defects.push(format!("missing version-57 behavior obligation {required}"));
        }
    }
    for forbidden in [
        "DFF_V56_R145_POLISH",
        "DFF_V56_R147_INELIGIBLE",
        "normalize_external_liquid",
        "external_liquid_kg_m2 = 0.0",
        "STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS / 2",
    ] {
        if production.contains(forbidden) {
            defects.push(format!("forbidden version-57 seam {forbidden}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-57 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn terminal_endpoint_identity_includes_deposition_and_refreeze() {
    let energy = read(ENERGY);
    for required in [
        "deposition/refreeze cannot retroactively enlarge same-trial melt availability",
        "no event while deposited or refrozen solid remains",
        "complete solid identity—not a debit clamp—must establish zero ice",
        "Q_terminal_unallocated=Q_complete+Q_refreeze-Delta H_cc-L_f m_melt >= 0",
        "No snow-domain state receives energy and no snow flux is evaluated after the event",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn package_and_index_preserve_receiving_surface_and_production_boundaries() {
    let package = read(PACKAGE);
    let energy = read(ENERGY);
    let freeze = read(FREEZE);
    let index = read(INDEX);
    assert!(package.contains("No land-surface, soil, frost, infiltration, runoff"));
    assert!(package.contains("No assignment of terminal unallocated energy"));
    assert!(package.contains("No physical seasonal efficacy"));
    assert!(energy.contains("60-second floor remain unchanged"));
    assert!(energy.contains("fresh `CoupledAuthentic` evaluation"));
    assert!(
        index.contains("v140 owner amendment selects an exact 60-second adaptive Stage-3 floor")
    );
    assert!(
        energy.contains("Version 18 defines the persistent Stage 3 snow--soil conductive boundary")
    );
    assert!(
        freeze.contains("Version 140 supersedes the active mechanics of `INV-SNOWFREEZE-101/102`")
    );
    assert!(freeze.contains("The old receiver evidence remains\nhistorical evidence"));
}

#[test]
fn v32_contract_binds_pure_opposite_sign_vapor_active_set_authority() {
    let energy = read(ENERGY);
    for required in [
        "REF-SNOWENERGY-WGHL-V32",
        "INV-SNOWENERGY-056",
        "OBL-SNOWENERGY-C-024",
        "alpha_v=-V_0/(V_1-V_0)",
        "V=D=S=Q_v=+0",
        "alpha_v=0.04393657257739406",
        "+45.77845449909091 J m^-2",
        "w_p=max(0.25,min(0.5,120 s/h))",
        "same-disposition endpoints",
        "any exact covered support at or above the unchanged 60-second floor",
        "synthetic numerical states",
        "only a later fresh authentic image",
        "No diagnostic from this numerical path is persisted",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
    assert!(energy.contains("v31/v32 midpoint, vapor-root, and branch-entry authority"));
    assert!(energy.contains("active-set root/interface/branch-entry/opposite-authentic/reset"));
    assert!(energy.contains("Affine interpolation of endpoint latent energy is forbidden"));
}

#[test]
fn v32_captured_root_rejects_affine_latent_interpolation() {
    let current_vapor_kg_m2 = 2.121_596_912_395_713_5e-4_f64;
    let authentic_vapor_kg_m2 = -4.616_612_304_251_271e-3_f64;
    let current_latent_j_m2 = 649.057_936_925_198_f64;
    let authentic_latent_j_m2 = -13_081.632_625_326_402_f64;

    let alpha = current_vapor_kg_m2 / (current_vapor_kg_m2 - authentic_vapor_kg_m2);
    let localized_vapor =
        current_vapor_kg_m2 + alpha * (authentic_vapor_kg_m2 - current_vapor_kg_m2);
    let forbidden_affine_latent =
        current_latent_j_m2 + alpha * (authentic_latent_j_m2 - current_latent_j_m2);

    assert_eq!(alpha.to_bits(), 0.043_936_572_577_394_06_f64.to_bits());
    assert_eq!(localized_vapor.to_bits(), 0.0_f64.to_bits());
    assert_eq!(
        forbidden_affine_latent.to_bits(),
        45.778_454_499_090_91_f64.to_bits()
    );
    assert_ne!(forbidden_affine_latent.to_bits(), 0.0_f64.to_bits());
}

#[test]
fn v32_production_symbols_and_larger_direct_support_behavior_are_required() {
    let fixed_point = read(FIXED_POINT);
    let open_snow = read(OPEN_SNOW);
    let open_snow_tests = read(OPEN_SNOW_TESTS);

    for required in [
        "CoveredVaporActiveSetInterfaceV1",
        "covered_vapor_active_set_interface_v1",
        "covered_vapor_active_set_branch_entry_v1",
        "COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS",
    ] {
        assert!(
            fixed_point.contains(required),
            "{FIXED_POINT} missing version-32 production symbol {required}"
        );
    }
    assert!(
        open_snow.contains("covered_vapor_active_set_interface_v1"),
        "{OPEN_SNOW} must dispatch the version-32 private transition"
    );
    for required in [
        "v32_vapor_active_set_accepts_direct_support_above_exact_floor",
        "v32_vapor_active_set_retains_same_sign_v31_dispatch",
        "v32_vapor_active_set_synthetic_images_cannot_publish",
    ] {
        assert!(
            open_snow_tests.contains(required),
            "{OPEN_SNOW_TESTS} missing production behavior obligation {required}"
        );
    }
}

#[test]
fn v33_contract_binds_phase_consistent_coupled_authority() {
    let energy = read(ENERGY);
    for required in [
        "REF-SNOWENERGY-WGHL-V33",
        "INV-SNOWENERGY-057",
        "OBL-SNOWENERGY-C-025",
        "root/interface -> one-sided branch-entry -> opposite pure-vapor raw-authentic",
        "not required to compare bitwise equal",
        "CoveredPhaseConsistentResidualInputsV1",
        "CoveredPhaseConsistentResidualEvaluationV1",
        "covered_phase_consistent_residual_evaluate_v1",
        "x = (W_1,l, H_1,l, E_soil,1,n, T_soil,1,n)_affected",
        "(I_1,l, L_1,l, C_1,l, U_1,l) = Pi(W_1,l, H_1,l)",
        "R_W,l = W_1,l - W_0,l - DeltaW_physical,l",
        "R_H,l = H_1,l - H_0,l - Q_complete,l",
        "R_E,n = E_soil,1,n - E_soil,0,n - DeltaE_CN+other,n",
        "R_T,n = T_soil,1,n - T_soil_owner(E_soil,1,n, sealed soil state)",
        "Q_v,l = V_l * L_s,l",
        "`F(x)-x` residuals are forbidden",
        "deterministic safeguarded semismooth Newton",
        "CoveredPhysicalEvaluationBudgetV1",
        "CoveredConvergenceAdmissionV1::CoupledAuthentic",
        "ordinary Picard current/candidate equality and convergence",
        "phase kink is internal complementarity",
        "Version-31/32 midpoint, vapor-interface, and branch-entry",
        "phase_consistent_coupled_authentic_final_evaluation_v1",
        "phase_consistent_coupled_authentic_final_replay_reseal_v1",
        "No microstepping diagnostic may persist in production",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
    assert!(energy.contains("without raw-owner bit equality"));
    assert!(energy.contains("concrete `R_W/R_H/R_E/R_T`"));
    assert!(energy.contains("fresh `CoupledAuthentic` admission"));
}

#[test]
fn v34_contract_binds_stable_monotone_coupled_eligibility() {
    let energy = read(ENERGY);
    for required in [
        "Version 34 retains every version-33 equation",
        "REF-SNOWENERGY-WGHL-V34",
        "INV-SNOWENERGY-058",
        "OBL-SNOWENERGY-C-026",
        "CoveredStableMonotoneSolveEligibilityV1",
        "covered_stable_monotone_solve_eligibility_v1",
        "exactly eight consecutive raw authentic maps",
        "strictly decreasing governed residual merit",
        "no `A/B/A`, active-set transition, or finalization restart",
        "support/source/event/topology/custody/static-receipt/phase-branch/carry-authority-and-representation joins",
        "Static receipt joins mean the unchanged receipt schema",
        "physical applied-energy value, candidate-ending identity",
        "resealed receipt digest to remain byte-equal",
        "enthalpy as `E=exact(H_hi)+R`",
        "to remain byte-equal while the physical solution evolves",
        "receipt and carry remains subject to unchanged exact custody",
        "same unchanged 96-physical-evaluation budget",
        "discards all private trials and resumes ordinary raw authentic Picard",
        "only the remaining budget",
        "No private trial may enter acceptance",
        "unchanged fresh `CoupledAuthentic`",
        "exact V2 carry",
        "exact 60-second floor",
        "No microstepping diagnostic may persist in production",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v34_stable_monotone_production_seams_are_required() {
    let fixed_point = read(FIXED_POINT);
    let open_snow = read(OPEN_SNOW);
    let open_snow_tests = read(OPEN_SNOW_TESTS);
    let coupled = fs::read_to_string(COUPLED_SOLVE).unwrap_or_default();
    let production = format!("{fixed_point}\n{open_snow}\n{coupled}");
    let mut defects = Vec::new();

    for required in [
        "CoveredStableMonotoneSolveEligibilityV1",
        "covered_stable_monotone_solve_eligibility_v1",
        "COVERED_STABLE_MONOTONE_RAW_MAPS_REQUIRED",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-34 production seam {required}"));
        }
    }
    for required in [
        "v34_stable_monotone_eligibility_accepts_exactly_eight_raw_authentic_maps",
        "v34_stable_monotone_eligibility_allows_physical_receipt_and_carry_coordinate_evolution",
        "v34_stable_monotone_eligibility_refuses_static_join_phase_or_merit_change",
        "v34_pre_root_refusal_discards_private_trials_and_resumes_raw_picard",
        "v34_stable_monotone_uses_existing_shared_physical_evaluation_budget",
        "v34_stable_monotone_private_trial_cannot_accept_or_publish",
    ] {
        if !open_snow_tests.contains(required) {
            defects.push(format!("missing version-34 behavior obligation {required}"));
        }
    }

    assert!(
        defects.is_empty(),
        "current version-34 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v35_contract_binds_exact_authentic_receipt_stabilization() {
    let energy = read(ENERGY);
    for required in [
        "Version 35 retains version-34 solver eligibility",
        "REF-SNOWENERGY-WGHL-V35",
        "INV-SNOWENERGY-059",
        "OBL-SNOWENERGY-C-027",
        "CoveredAuthenticReceiptStabilizationV1",
        "covered_authentic_receipt_stabilization_probe_v1",
        "covered_authentic_receipt_stabilization_replay_v1",
        "The first private-root-to-authentic reseal is therefore a probe",
        "be compared for exact acceptance against artifacts produced",
        "input `R_n` is exactly equal",
        "`R_(n+1)`",
        "without digest repair, tolerance comparison",
        "in-place mutation",
        "CoveredPhysicalEvaluationBudgetV1",
        "one independent authentic replay",
        "exact equality of physical",
        "candidate artifacts, and reconstructed receipts",
        "Exact receipt oscillation",
        "all private/probe artifacts are discarded",
        "No physical equation",
        "exact-carry",
        "60-second floor",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v35_authentic_receipt_stabilization_production_seams_are_required() {
    let fixed_point = read(FIXED_POINT);
    let open_snow = read(OPEN_SNOW);
    let open_snow_tests = read(OPEN_SNOW_TESTS);
    let coupled = fs::read_to_string(COUPLED_SOLVE).unwrap_or_default();
    let production = format!("{fixed_point}\n{open_snow}\n{coupled}");
    let mut defects = Vec::new();

    for required in [
        "CoveredAuthenticReceiptStabilizationV1",
        "covered_authentic_receipt_stabilization_probe_v1",
        "covered_authentic_receipt_stabilization_replay_v1",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-35 production seam {required}"));
        }
    }
    for required in [
        "v35_receipt_stabilization_feeds_reconstructed_output_as_next_immutable_input",
        "v35_first_root_reseal_is_probe_not_cross_input_replay",
        "v35_receipt_stabilization_requires_exact_input_output_receipts",
        "v35_same_input_replay_requires_exact_residual_artifact_and_receipt_equality",
        "v35_receipt_oscillation_nonfinite_constraint_budget_discards_probe_artifacts",
    ] {
        if !open_snow_tests.contains(required) {
            defects.push(format!("missing version-35 behavior obligation {required}"));
        }
    }

    assert!(
        defects.is_empty(),
        "current version-35 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v36_contract_binds_geometry_complete_physical_solver() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V36",
        "INV-SNOWENERGY-060",
        "OBL-SNOWENERGY-C-028",
        "CoveredTerminalDensityGeometryCoordinateV1",
        "covered_terminal_density_geometry_residual_evaluate_v1",
        "rho_1,l",
        "z_1,l=I_1,l/rho_1,l",
        "R_rho,l",
        "R_W/R_H/R_rho/R_E/R_T",
        "unchanged Stage 3 density/settling constitutive map",
        "immutable beginning state",
        "exact layer identity/order, settling authority",
        "density-model branch while allowing",
        "physical `rho_1,l` and reconstructed",
        "generic `F(x)-x` remains forbidden",
        "same unchanged shared 96",
        "retains version-35 receipt stabilization and authentic finalization",
        "Density or thickness interpolation, repair",
        "uncharged physics",
        "60-second floor",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v36_geometry_complete_production_seams_are_required() {
    let fixed_point = read(FIXED_POINT);
    let open_snow = read(OPEN_SNOW);
    let open_snow_tests = read(OPEN_SNOW_TESTS);
    let coupled = fs::read_to_string(COUPLED_SOLVE).unwrap_or_default();
    let production = format!("{fixed_point}\n{open_snow}\n{coupled}");
    let mut defects = Vec::new();
    for required in [
        "CoveredTerminalDensityGeometryCoordinateV1",
        "covered_terminal_density_geometry_residual_evaluate_v1",
        "r_rho_kg_m3",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-36 production seam {required}"));
        }
    }
    for required in [
        "v36_geometry_complete_solver_reconstructs_thickness_from_mass_and_density",
        "v36_geometry_complete_solver_evaluates_physical_r_rho_not_map_difference",
        "v36_stable_eligibility_preserves_density_branch_while_rho_and_z_evolve",
        "v36_geometry_physics_charges_shared_budget_and_retains_v35_stabilization",
        "v36_geometry_solver_refuses_branch_poison_interpolation_repair_or_bypass",
    ] {
        if !open_snow_tests.contains(required) {
            defects.push(format!("missing version-36 behavior obligation {required}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-36 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v37_contract_binds_derived_thickness_root_closure() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V37",
        "INV-SNOWENERGY-061",
        "OBL-SNOWENERGY-C-029",
        "CoveredDerivedThicknessClosureV1",
        "covered_derived_thickness_closure_evaluate_v1",
        "R_z,l=z_1,l-z_phys,l",
        "existing `depth_abs_m` bound",
        "continues to solve only `R_W/R_H/R_rho/R_E/R_T`",
        "same already-charged unchanged Stage 3 map",
        "V35 exact receipt stabilization",
        "unchanged authentic finalization comparison",
        "not a new independently solved coordinate",
        "Density or thickness copying, interpolation, post-hoc repair",
        "60-second floor",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v37_derived_thickness_production_seams_are_required() {
    let fixed_point = read(FIXED_POINT);
    let open_snow = read(OPEN_SNOW);
    let open_snow_tests = read(OPEN_SNOW_TESTS);
    let coupled = fs::read_to_string(COUPLED_SOLVE).unwrap_or_default();
    let production = format!("{fixed_point}\n{open_snow}\n{coupled}");
    let mut defects = Vec::new();
    for required in [
        "CoveredDerivedThicknessClosureV1",
        "covered_derived_thickness_closure_evaluate_v1",
        "r_z_m",
        "derived_constraints_scaled_merit",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-37 production seam {required}"));
        }
    }
    for required in [
        "v37_derived_thickness_closure_blocks_low_density_amplified_water_error",
        "v37_derived_thickness_closure_uses_same_charged_physical_image",
        "v37_root_admission_requires_residual_and_derived_thickness_merit",
        "v37_receipt_replay_and_finalization_retain_derived_thickness_closure",
        "v37_refuses_independent_z_omission_interpolation_repair_or_bypass",
    ] {
        if !open_snow_tests.contains(required) {
            defects.push(format!("missing version-37 behavior obligation {required}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-37 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v38_contract_binds_finalization_equivalent_charged_map() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V38",
        "INV-SNOWENERGY-062",
        "OBL-SNOWENERGY-C-030",
        "covered_phase_consistent_finalization_equivalent_map_v1",
        "finalization-equivalent endpoint map",
        "canonical proposed `W/H/rho/E/T` coordinates",
        "first ordered, snow-coupled node of each OFE",
        "preserve every deeper layer bit-exact",
        "zeroing or rerounding a deeper-layer carry",
        "non-provisional carrier posture",
        "Exactly one Stage 3 physical map",
        "provisional-map closure",
        "independent authentic finalization",
        "exact Stage 3, LSE, boundary, receipt, and ledger equality",
        "shared-budget charge",
        "60-second floor",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v38_finalization_equivalent_production_seams_are_required() {
    let open_snow = read(OPEN_SNOW);
    let physical_support = read(OPEN_SNOW_PHYSICAL_SUPPORT);
    let open_snow_tests = read(OPEN_SNOW_TESTS);
    let coupled = fs::read_to_string(COUPLED_SOLVE).unwrap_or_default();
    let production = format!("{open_snow}\n{coupled}");
    let mut defects = Vec::new();
    for required in [
        "CoveredFinalizationEquivalentPhysicalMapV1",
        "covered_phase_consistent_finalization_equivalent_map_v1",
        "CoveredPhaseConsistentPhysicalMapPostureV1::FinalizationEquivalent",
        "stage3_physical_map_count",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-38 production seam {required}"));
        }
    }
    for required in [
        "v38_charged_evaluation_uses_finalization_equivalent_endpoint_map",
        "v38_provisional_map_closure_cannot_admit_root",
        "v38_finalization_equivalent_map_is_receipt_replay_stable",
        "v38_finalization_independent_replay_requires_exact_same_map_image",
        "v38_refuses_extra_physics_map_repair_bypass_or_publication",
    ] {
        if !open_snow_tests.contains(required) {
            defects.push(format!("missing version-38 behavior obligation {required}"));
        }
    }
    let initial_soil_body = physical_support
        .split_once("fn initial_unpublished_soil_iteration_candidate_v1")
        .and_then(|(_, suffix)| {
            suffix
                .split_once("fn unpublished_soil_candidate_for_covered_iteration_v1")
                .map(|(body, _)| body)
        });
    match initial_soil_body {
        Some(body) => {
            for required in [
                "prepare_next_soil_thermal_support_v2",
                "support.start_ns().get()",
                "support.end_ns().get()",
            ] {
                if !body.contains(required) {
                    defects.push(format!(
                        "version-38 initial soil support is not bound to {required}"
                    ));
                }
            }
            if body.contains(".prepare_soil_thermal_support_v2(")
                || body.contains("next_transaction_id")
                || body.contains("lse_forcing.transaction_id")
            {
                defects.push(
                    "version-39 initial soil support still derives soil custody from an outer transaction"
                        .to_owned(),
                );
            }
        }
        None => defects.push("missing bounded version-38 initial soil support body".to_owned()),
    }
    let charged_body = open_snow
        .split_once("let charged_map =")
        .and_then(|(_, suffix)| {
            suffix
                .split_once("charged_map.validate()")
                .map(|(body, _)| body)
        });
    match charged_body {
        Some(body) => {
            for required in [
                "covered_phase_consistent_finalization_equivalent_map_v1",
                "covered_phase_consistent_project_stage3_coordinates_v1",
                "prepare_next_soil_thermal_support_v2",
                "project_soil_thermal_unpublished_top_layer_coordinates_v2",
                "build_covered_carrier_envelope_value_with_soil_beginning_v1",
                "provisional: closure_posture.carrier_is_provisional()",
                "CoveredFinalizationEquivalentReplayInputsV1",
            ] {
                if !body.contains(required) {
                    defects.push(format!("version-38 charged map is not wired to {required}"));
                }
            }
            if body.contains("self.interval.lse_forcing.transaction_id")
                || body.contains(".prepare_soil_thermal_support_v2(")
            {
                defects.push(
                    "version-39 finalization-equivalent soil projection still substitutes the outer source transaction for authenticated soil successor custody"
                        .to_owned(),
                );
            }
            if body.matches("evaluate_stage3(").count() != 1 {
                defects.push(
                    "version-38 charged map must contain exactly one Stage-3 evaluation".to_owned(),
                );
            }
            if body.contains("build_provisional_covered_iteration_evidence_v1") {
                defects.push(
                    "version-38 charged map still routes through provisional evidence".to_owned(),
                );
            }
        }
        None => defects.push("missing bounded version-38 charged-map body".to_owned()),
    }
    let finalization_body = open_snow
        .split_once("if let Some(replay) = coupled_finalization_inputs.as_ref()")
        .and_then(|(_, suffix)| {
            suffix
                .split_once("let sealed_source_started")
                .map(|(body, _)| body)
        });
    match finalization_body {
        Some(body) => {
            for required in [
                "replay.input_covered_boundaries",
                "replay.input_open_boundaries",
                "replay.proposed_stage3",
                "replay.proposed_soil",
                "replay.destination_receipts",
                "covered_fixed_point_stage3_states_equal",
            ] {
                if !body.contains(required) {
                    defects.push(format!(
                        "version-38 independent finalization is not wired to {required}"
                    ));
                }
            }
            if body.matches("evaluate_stage3(").count() != 1 {
                defects.push(
                    "version-38 independent finalization must contain exactly one Stage-3 replay"
                        .to_owned(),
                );
            }
        }
        None => defects.push("missing bounded version-38 finalization replay body".to_owned()),
    }
    assert!(
        defects.is_empty(),
        "current version-38 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v39_contract_binds_distinct_soil_energy_source_and_target_transactions() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V39",
        "INV-SNOWENERGY-063",
        "OBL-SNOWENERGY-C-031",
        "PhysicalSoilEnergyTransactionAuthorityV2",
        "source_transaction_id",
        "soil_thermal_transaction_id",
        "every operand digest must bind both values",
        "No caller may infer a target by incrementing the source transaction",
        "exact rollback",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v39_soil_energy_transaction_separation_production_seams_are_required() {
    let production = read(REAL_HYDROLOGY);
    let open_snow = read(OPEN_SNOW);
    let physical_support = read(OPEN_SNOW_PHYSICAL_SUPPORT);
    let open_snow_tests = read(OPEN_SNOW_TESTS);
    let v10_soil_thermal_tests = read(V10_SOIL_THERMAL_TESTS);
    let mut defects = Vec::new();
    for required in [
        "PhysicalSoilEnergyTransactionAuthorityV2",
        "source_transaction_id",
        "soil_thermal_transaction_id",
        "validate_pre_ingress_candidates",
        "ingress.transaction_id() != authority.source_transaction_id",
        "OPENWEPP_ACCEPTED_SOIL_INTERNAL_ENERGY_V2",
        "OPENWEPP_ACCEPTED_SOIL_INFILTRATION_ENERGY_V2",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-39 production seam {required}"));
        }
    }
    for required in [
        "v39_physical_soil_energy_operands_bind_outer_source_and_soil_target_transactions",
        "v39_second_child_soil_operands_keep_outer_ingress_transaction",
        "v39_soil_operand_transaction_substitution_refuses_without_publication",
    ] {
        if !open_snow_tests.contains(required) && !production.contains(required) {
            defects.push(format!("missing version-39 behavior obligation {required}"));
        }
    }
    if !v10_soil_thermal_tests.contains(
        "v39_second_child_continuation_uses_authenticated_soil_transaction_and_refuses_foreign_custody",
    ) {
        defects.push("missing version-39 downstream continuation custody behavior".to_owned());
    }
    let charged_body = open_snow
        .split_once("let charged_map =")
        .and_then(|(_, suffix)| {
            suffix
                .split_once("charged_map.validate()")
                .map(|(body, _)| body.to_owned())
        });
    match charged_body {
        Some(body) => {
            if !body.contains("prepare_next_soil_thermal_support_v2") {
                defects.push(
                    "version-39 finalization-equivalent projection omits authenticated soil successor preparation"
                        .to_owned(),
                );
            }
            if body.contains("self.interval.lse_forcing.transaction_id")
                || body.contains(".prepare_soil_thermal_support_v2(")
            {
                defects.push(
                    "version-39 finalization-equivalent projection still feeds outer source transaction into soil-only support preparation"
                        .to_owned(),
                );
            }
        }
        None => defects.push("missing bounded version-39 charged-map body".to_owned()),
    }
    let initial_soil_body = physical_support
        .split_once("fn initial_unpublished_soil_iteration_candidate_v1")
        .and_then(|(_, suffix)| {
            suffix
                .split_once("fn unpublished_soil_candidate_for_covered_iteration_v1")
                .map(|(body, _)| body)
        });
    match initial_soil_body {
        Some(body) => {
            if !body.contains("prepare_next_soil_thermal_support_v2") {
                defects.push(
                    "version-39 initial covered soil trial omits authenticated successor preparation"
                        .to_owned(),
                );
            }
            for forbidden in [
                ".prepare_soil_thermal_support_v2(",
                "next_transaction_id",
                "lse_forcing.transaction_id",
            ] {
                if body.contains(forbidden) {
                    defects.push(format!(
                        "version-39 initial covered soil trial retains forbidden raw transaction seam {forbidden}"
                    ));
                }
            }
        }
        None => defects.push("missing bounded version-39 initial-soil body".to_owned()),
    }
    let builder = production
        .split_once("pub fn physical_soil_energy_operands_v2")
        .and_then(|(_, suffix)| {
            suffix
                .split_once("fn append_v2_soil_internal_operands")
                .map(|(body, _)| body)
        });
    match builder {
        Some(body) => {
            if body.contains("ingress.transaction_id() != transaction_id") {
                defects.push(
                    "version-39 operand builder still conflates ingress and soil transactions"
                        .to_owned(),
                );
            }
            for required in [
                "authority.source_transaction_id",
                "authority.soil_thermal_transaction_id",
            ] {
                if !body.contains(required) {
                    defects.push(format!("version-39 operand builder omits {required}"));
                }
            }
        }
        None => defects.push("missing bounded version-39 operand builder".to_owned()),
    }
    assert!(
        defects.is_empty(),
        "current version-39 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v40_contract_binds_parity_monotone_active_set_trigger() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V40",
        "INV-SNOWENERGY-064",
        "OBL-SNOWENERGY-C-032",
        "exactly four consecutive failed-but-valid rolling active-set reset windows",
        "finite positive dimensionless Euclidean norm",
        "never by a physical tolerance",
        "enough unused capacity in the same existing 96-evaluation budget",
        "cannot establish residual closure",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v40_parity_monotone_production_seams_are_required() {
    let coupled = fs::read_to_string(COUPLED_SOLVE).unwrap_or_default();
    let open_snow = read(OPEN_SNOW);
    let open_snow_tests = read(OPEN_SNOW_TESTS);
    let production = format!("{coupled}\n{open_snow}");
    let mut defects = Vec::new();
    for required in [
        "CoveredParityMonotoneActiveSetResetV1",
        "CoveredParityMonotoneActiveSetEligibilityV1",
        "covered_parity_monotone_active_set_eligibility_v1",
        "covered_parity_monotone_active_set_observe_v1",
        "COVERED_PARITY_MONOTONE_ACTIVE_SET_WINDOWS_REQUIRED",
        "minimum_solver_reserve",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-40 production seam {required}"));
        }
    }
    for required in [
        "v40_parity_monotone_accepts_four_exact_static_decreasing_windows",
        "v40_parity_monotone_refuses_nonfinite_stagnation_or_reversal",
        "v40_parity_monotone_refuses_static_chain_phase_side_or_cadence_poison",
        "v40_parity_monotone_requires_shared_budget_reserve_without_publication",
        "v40_parity_monotone_dispatch_retains_authentic_only_acceptance",
    ] {
        if !open_snow_tests.contains(required) {
            defects.push(format!("missing version-40 behavior obligation {required}"));
        }
    }
    assert!(!production.contains("DFF_R107_SOLVER_REFUSAL"));
    assert!(
        defects.is_empty(),
        "current version-40 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v41_contract_binds_one_way_canonical_enthalpy_boundary_trigger() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V41",
        "INV-SNOWENERGY-065",
        "OBL-SNOWENERGY-C-033",
        "bit-identical at all five points",
        "strictly one-way at every transition",
        "Exactly one adjacent transition",
        "no tolerance, interpolation, bisection, repair, phase projection, convergence, or root admission",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v41_one_way_phase_boundary_production_seams_are_required() {
    let coupled = fs::read_to_string(COUPLED_SOLVE).unwrap_or_default();
    let open_snow = read(OPEN_SNOW);
    let open_snow_tests = read(OPEN_SNOW_TESTS);
    let production = format!("{coupled}\n{open_snow}");
    let mut defects = Vec::new();
    for required in [
        "CoveredOneWayPhaseBoundaryEligibilityV1",
        "covered_one_way_phase_boundary_eligibility_v1",
        "covered_canonical_phase_predicate_v1",
        "canonical_boundary_crossings",
        "minimum_solver_reserve",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-41 production seam {required}"));
        }
    }
    for required in [
        "v41_one_way_phase_boundary_accepts_exact_monotone_single_crossing",
        "v41_one_way_phase_boundary_refuses_reversal_stagnation_or_multiple_crossing",
        "v41_one_way_phase_boundary_refuses_water_drift_join_side_or_cadence_poison",
        "v41_one_way_phase_boundary_requires_shared_budget_reserve_without_publication",
        "v41_one_way_phase_boundary_dispatch_retains_unchanged_authentic_solver",
    ] {
        if !open_snow_tests.contains(required) {
            defects.push(format!("missing version-41 behavior obligation {required}"));
        }
    }
    assert!(!production.contains("DFF_R109"));
    assert!(
        defects.is_empty(),
        "current version-41 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v42_contract_binds_exact_cold_content_export_coordinate() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V42",
        "INV-SNOWENERGY-066",
        "OBL-SNOWENERGY-C-034",
        "H=-C_0+L_f(L_0+L_in)+Q+X_c",
        "complete_arm_cold_content_export_j_m2",
        "zero-`X_c` case is byte-identical",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v42_cold_content_export_production_seams_are_required() {
    let fixed_point = read(FIXED_POINT);
    let coupled = fs::read_to_string(COUPLED_SOLVE).unwrap_or_default();
    let open_snow = read(OPEN_SNOW);
    let open_snow_tests = read(OPEN_SNOW_TESTS);
    let production = format!("{fixed_point}\n{coupled}\n{open_snow}");
    let mut defects = Vec::new();
    for required in [
        "cold_content_export_j_m2",
        "complete_arm_cold_content_export_j_m2",
        "+ support.cold_content_export_j_m2",
        "interpolate(\n            current.cold_content_export_j_m2",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-42 production seam {required}"));
        }
    }
    for required in [
        "v42_cold_content_export_closes_captured_snow_reappearance_endpoint",
        "v42_zero_export_retains_v31_v32_coordinate_bits",
        "v42_private_active_set_contracts_export_with_exact_shared_weight",
        "v42_cold_content_export_refuses_nonfinite_negative_or_substituted_endpoint",
        "v42_cold_content_export_refuses_omission_order_weight_or_closure_poison",
    ] {
        if !open_snow_tests.contains(required) {
            defects.push(format!("missing version-42 behavior obligation {required}"));
        }
    }
    assert!(!production.contains("DFF_V42"));
    assert!(
        defects.is_empty(),
        "current version-42 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v43_contract_binds_typed_projected_base_custody() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V43",
        "INV-SNOWENERGY-067",
        "OBL-SNOWENERGY-C-035",
        "NumericalCoordinateProjection",
        "ordinary base reconstruction",
        "canonical zero exact carry",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v43_projected_base_custody_production_seams_are_required() {
    let soil = read(
        "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2.rs",
    );
    let soil_tests = read(
        "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2_tests.rs",
    );
    let mut defects = Vec::new();
    for required in [
        "DirectSoilThermalUnpublishedFixedPointPostureV2",
        "validate_projected_fixed_point_v2",
        "numerical_coordinate_authority_sha256",
        "numerical_coordinate_set_sha256",
        "V2 projected fixed-point",
    ] {
        if !soil.contains(required) {
            defects.push(format!("missing version-43 production seam {required}"));
        }
    }
    for required in [
        "v43_projected_fixed_point_accepts_typed_coordinate_custody",
        "v43_projected_fixed_point_retains_base_reconstruction_byte_lock",
        "v43_projected_fixed_point_refuses_erased_mixed_or_foreign_custody",
        "v43_projected_fixed_point_refuses_support_receipt_authority_or_order_poison",
        "v43_projected_fixed_point_cannot_advance_accept_install_or_publish",
    ] {
        if !soil_tests.contains(required) {
            defects.push(format!("missing version-43 behavior obligation {required}"));
        }
    }
    assert!(!soil.contains("DFF_R113"));
    assert!(
        defects.is_empty(),
        "current version-43 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v44_contract_binds_uncommitted_lse_then_strict_authentic_closure() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V44",
        "INV-SNOWENERGY-068",
        "OBL-SNOWENERGY-C-036",
        "uncommitted provisional LSE posture",
        "unchanged weighted-OFE energy closure",
        "exactly once",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v44_uncommitted_lse_closure_production_seams_are_required() {
    let coupled = fs::read_to_string(COUPLED_SOLVE).unwrap_or_default();
    let open_snow = read(OPEN_SNOW);
    let v9_direct = read(
        "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/\
direct_v9_real_consumer_shadow_impl.rs",
    );
    let v10_soil_tests = read(
        "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/\
v10_soil_thermal_v2_tests.rs",
    );
    let open_snow_tests = read(OPEN_SNOW_TESTS);
    let open_snow_v44_tests = read(
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/\
open_snow_convergence_v44_tests.rs",
    );
    let production = format!("{coupled}\n{open_snow}\n{v9_direct}");
    let behavior_tests = format!("{open_snow_tests}\n{open_snow_v44_tests}\n{v10_soil_tests}");
    let mut defects = Vec::new();
    for required in [
        "CoveredPhaseConsistentCarrierClosurePostureV1",
        "covered_phase_consistent_carrier_closure_posture_v1",
        "UncommittedPrivateLseExchange",
        "StrictAuthenticWeightedOfe",
        "requires_strict_weighted_ofe_closure",
        "covered_phase_consistent_carrier_input_exchange_v1",
        "covered_phase_consistent_projected_soil_exact_once_v1",
        "direct_v9_select_v8_soil_beginning_v44",
        "V44 projected soil must not enter Stage3-covered V8",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-44 production seam {required}"));
        }
    }
    for required in [
        "v44_private_trial_defers_only_aggregate_weighted_ofe_closure",
        "v44_receipt_probe_and_replay_require_strict_weighted_ofe_closure",
        "v44_corrected_exchange_enters_single_stage3_map",
        "v44_projected_soil_coordinate_is_consumed_once_by_cn_not_v8",
        "v44_deferred_private_closure_cannot_admit_or_publish",
        "v44_numerical_projection_selects_resident_v8_and_rejects_double_use",
    ] {
        if !behavior_tests.contains(required) {
            defects.push(format!("missing version-44 behavior obligation {required}"));
        }
    }
    assert!(!production.contains("DFF_R116"));
    assert!(
        defects.is_empty(),
        "current version-44 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v45_contract_binds_authentic_receipt_root_polishing_and_replay_reserve() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V45",
        "INV-SNOWENERGY-069",
        "OBL-SNOWENERGY-C-037",
        "phase_consistent_coupled_root_polish_v1",
        "strictly lowers the existing canonical scaled merit",
        "fewer than three charges",
        "fewer than two",
        "final replay charge",
        "Exact receipt input/output equality",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v45_authentic_receipt_root_polishing_production_seams_are_required() {
    let coupled = fs::read_to_string(COUPLED_SOLVE).unwrap_or_default();
    let open_snow = read(OPEN_SNOW);
    let v45_tests = fs::read_to_string(
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/\
open_snow_convergence_v45_tests.rs",
    )
    .unwrap_or_default();
    let production = format!("{coupled}\n{open_snow}");
    let mut defects = Vec::new();
    for required in [
        "CoveredCoupledPolishedRootV1",
        "CoveredCoupledPolishStopV1",
        "CoveredPhaseConsistentPhysicalEvaluationV1",
        "phase_consistent_coupled_root_polish_v1",
        "phase_consistent_coupled_safeguarded_step_v1",
        "phase_consistent_coupled_physical_solve_v1",
        "CoveredPhaseConsistentPhysicalBranchIdentityV1",
        "physical_evaluation_ordinal",
        "covered_physical_evaluation_budget_preserve_v1",
        "COVERED_PRIVATE_SOLVE_POST_ROOT_RESERVE_V1",
        "COVERED_ROOT_POLISH_RECEIPT_RESERVE_V1",
        "COVERED_RECEIPT_REPLAY_RESERVE_V1",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-45 production seam {required}"));
        }
    }
    for required in [
        "v45_private_solve_preserves_polish_probe_and_replay_capacity",
        "v45_root_polish_carries_best_bundle_across_subtolerance_non_descent",
        "v45_root_polish_refuses_above_tolerance_stagnation_and_side_poison",
        "v45_nonstable_probe_cannot_consume_protected_replay_slot",
        "v45_exact_receipt_stabilization_replays_at_shared_budget_96",
        "v45_polished_private_root_cannot_admit_or_publish",
        "v45_root_polish_admits_strict_descent_and_carries_solver_trust_state",
        "v45_polish_refuses_branch_stale_bundle_singular_and_nonfinite_poisons",
        "v45_polish_stagnation_and_receipt_entry_reserve_are_private_stops",
        "v45_receipt_replay_refuses_finalization_input_substitution",
    ] {
        if !v45_tests.contains(required) {
            defects.push(format!("missing version-45 behavior obligation {required}"));
        }
    }
    for forbidden in [
        "DFF_V45",
        "receipt_bit_distance",
        "receipt_digest_repair",
        "latest_artifacts",
        "latest_finalization_inputs",
    ] {
        if production.contains(forbidden) {
            defects.push(format!("forbidden version-45 production seam {forbidden}"));
        }
    }
    if coupled.matches("let mut jacobian").count() != 1 {
        defects.push(
            "version-45 ordinary solve and polishing must share one safeguarded Jacobian/trust step"
                .to_string(),
        );
    }
    assert!(
        defects.is_empty(),
        "current version-45 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v46_contract_binds_dimension_complete_safeguarded_step_budget_preflight() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V46",
        "INV-SNOWENERGY-070",
        "OBL-SNOWENERGY-C-038",
        "dimension-complete safeguarded-step budget preflight",
        "`d+1+r`",
        "before evaluating the first generalized-Jacobian column",
        "maximum 96",
        "adds no quasi-Newton/chord/secant",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v46_complete_step_budget_preflight_production_seams_are_required() {
    let coupled = fs::read_to_string(COUPLED_SOLVE).unwrap_or_default();
    let open_snow = read(OPEN_SNOW);
    let v46_tests = fs::read_to_string(
        "crates/openwepp-hillslope-orchestrator/src/v11_covered/\
open_snow_convergence_v46_tests.rs",
    )
    .unwrap_or_default();
    let production = format!("{coupled}\n{open_snow}");
    let mut defects = Vec::new();
    for required in [
        "covered_safeguarded_complete_step_capacity_v1",
        "CoveredSafeguardedCompleteStepCapacityV1",
        "minimum_physical_evaluations",
        "reserve_after_step",
        "phase_consistent_coupled_safeguarded_step_v1",
        "CoveredSafeguardedStepV1::ReceiptEntryReserve",
        "covered_physical_evaluation_budget_preserve_v1",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-46 production seam {required}"));
        }
    }
    for required in [
        "v46_polish_preflight_refuses_before_partial_jacobian_charge",
        "v46_above_tolerance_preflight_is_typed_budget_failure",
        "v46_exact_complete_step_boundary_runs_unchanged_jacobian_and_trust",
        "v46_reverse_and_backtrack_maps_retain_per_call_reserve",
        "v46_recovered_capacity_runs_evolving_receipt_chain_and_replay_at_96",
        "v46_preflight_refuses_malformed_overflow_reset_and_enlarged_budgets",
        "v46_dimension_one_five_and_multi_exact_fit_and_one_short_vectors",
        "v46_partial_or_rejected_artifacts_cannot_admit_or_publish",
    ] {
        if !v46_tests.contains(required) {
            defects.push(format!("missing version-46 behavior obligation {required}"));
        }
    }
    for forbidden in [
        "DFF_V46",
        "CoveredBroyden",
        "covered_broyden",
        "receipt_bit_distance",
        "receipt_digest_repair",
        "receipt_map_residual",
    ] {
        if production.contains(forbidden) {
            defects.push(format!("forbidden version-46 production seam {forbidden}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-46 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v47_contract_binds_typed_atomic_complete_owner_transaction_posture() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V47",
        "INV-SNOWENERGY-071",
        "OBL-SNOWENERGY-C-039",
        "typed native-V2 atomic complete-owner transaction posture",
        "exact source==target",
        "exact authenticated soil predecessor==source",
        "Neither posture permits inferred numeric adjacency",
        "atomic replacement after complete validation",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v47_atomic_complete_owner_transaction_production_seams_are_required() {
    let production = read(V10_SOIL_THERMAL);
    let behavior = read(V10_SOIL_THERMAL_TESTS);
    let mut defects = Vec::new();
    for required in [
        "DirectSoilThermalAtomicCompleteOwnerTransactionPostureV2",
        "direct_soil_thermal_atomic_complete_owner_transaction_posture_v2",
        "source_transaction_id",
        "soil_target_transaction_id",
        "soil_expected_predecessor_transaction_id",
        "SameSourceAndSoilTarget",
        "AuthenticatedSoilSuccessor",
        "install_validated_soil_thermal_resident_v2",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-47 production seam {required}"));
        }
    }
    for required in [
        "v47_atomic_transaction_posture_accepts_same_source_and_soil_target",
        "v47_atomic_transaction_posture_accepts_exact_authenticated_soil_successor",
        "v47_atomic_transaction_posture_refuses_foreign_swapped_or_missing_identity",
        "v47_atomic_transaction_posture_refuses_source_owner_disagreement",
        "v47_composed_second_child_installs_with_exact_source_target_predecessor_chain",
        "v47_atomic_install_rolls_back_and_never_publishes_on_refusal",
    ] {
        if !behavior.contains(required) {
            defects.push(format!("missing version-47 behavior obligation {required}"));
        }
    }
    for forbidden in [
        "DFF_V47",
        "soil_target_transaction_id.0 - 1",
        "source_transaction_id.0 + 1",
        "wrapping_add(1)",
        "saturating_add(1)",
    ] {
        if production.contains(forbidden) {
            defects.push(format!("forbidden version-47 production seam {forbidden}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-47 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[test]
fn v48_contract_binds_authenticated_prepared_beginning_final_install() {
    let energy = read(ENERGY);
    for required in [
        "contract_version: 57",
        "REF-SNOWENERGY-WGHL-V48",
        "INV-SNOWENERGY-072",
        "OBL-SNOWENERGY-C-040",
        "authenticated-prepared-beginning final install",
        "generic/public installer remains unchanged",
        "authority-erasure failure",
        "real fixed-point finalization call site",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn v48_real_fixed_point_finalizer_requires_explicit_prepared_authority() {
    let production = read(V10_SOIL_THERMAL);
    let finalization = read(OWNER_FINALIZATION);
    let behavior = read(V10_SOIL_THERMAL_TESTS);
    let mut defects = Vec::new();
    for required in [
        "authenticate_soil_thermal_prepared_beginning_install_authority_v2",
        "install_soil_thermal_accepted_v2_from_authenticated_beginning",
        "V2 prepared-beginning install explicit transaction authority",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing version-48 production seam {required}"));
        }
    }
    for required in [
        "install_soil_thermal_accepted_v2_from_authenticated_beginning",
        "authenticate_soil_thermal_prepared_beginning_install_authority_v2",
    ] {
        if !finalization.contains(required) {
            defects.push(format!("missing version-48 real-finalizer seam {required}"));
        }
    }
    for required in [
        "v48_authenticated_prepared_beginning_installs_exact_split",
        "v48_generic_install_remains_strict_same_id",
        "v48_prepared_beginning_authority_refuses_substitution",
        "v48_authenticated_final_install_rolls_back_on_refusal",
        "v48_authenticated_same_id_and_exact_noop_do_not_publish",
        "v48_authenticated_prepared_custody_poison_matrix",
        "v48_real_finalizer_source_never_erases_split_authority",
    ] {
        if !behavior.contains(required) && !finalization.contains(required) {
            defects.push(format!("missing version-48 behavior obligation {required}"));
        }
    }
    for forbidden in [
        "DFF_V48",
        "soil_target_transaction_id.0 - 1",
        "source_transaction_id.0 + 1",
        "wrapping_add(1)",
        "saturating_add(1)",
    ] {
        if production.contains(forbidden) || finalization.contains(forbidden) {
            defects.push(format!("forbidden version-48 seam {forbidden}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-48 implementation is nonconforming:\n{}",
        defects.join("\n")
    );
}

#[derive(Clone)]
struct V33TransitionResetOracle {
    exact_joins: [u64; 4],
    root_coordinates: [u64; 2],
    reset_coordinates: [u64; 2],
    root_branch_predicate: u8,
    reset_branch_predicate: u8,
    branch_entry_vapor_side: i8,
    opposite_raw_vapor_side: i8,
    first_raw_owner_coordinates: [u64; 2],
    later_raw_owner_coordinates: [u64; 2],
}

fn v33_transition_reset_oracle(trace: &V33TransitionResetOracle) -> bool {
    trace
        .exact_joins
        .iter()
        .all(|join| *join == trace.exact_joins[0])
        && trace.root_coordinates == trace.reset_coordinates
        && trace.root_branch_predicate == trace.reset_branch_predicate
        && matches!(
            (trace.branch_entry_vapor_side, trace.opposite_raw_vapor_side),
            (1, -1) | (-1, 1)
        )
}

#[test]
fn v33_transition_reset_does_not_require_raw_owner_bit_equality() {
    let trace = V33TransitionResetOracle {
        exact_joins: [0x55; 4],
        root_coordinates: [0.0_f64.to_bits(), 45_778.454_f64.to_bits()],
        reset_coordinates: [0.0_f64.to_bits(), 45_778.454_f64.to_bits()],
        root_branch_predicate: 7,
        reset_branch_predicate: 7,
        branch_entry_vapor_side: 1,
        opposite_raw_vapor_side: -1,
        first_raw_owner_coordinates: [0.305_845_f64.to_bits(), 1_001.0_f64.to_bits()],
        later_raw_owner_coordinates: [0.305_846_f64.to_bits(), 1_000.5_f64.to_bits()],
    };
    assert_ne!(
        trace.first_raw_owner_coordinates,
        trace.later_raw_owner_coordinates
    );
    assert!(v33_transition_reset_oracle(&trace));

    let mut changed_join = trace.clone();
    changed_join.exact_joins[3] ^= 1;
    assert!(!v33_transition_reset_oracle(&changed_join));
    let mut changed_reset = trace.clone();
    changed_reset.reset_coordinates[1] ^= 1;
    assert!(!v33_transition_reset_oracle(&changed_reset));
    let mut changed_predicate = trace.clone();
    changed_predicate.reset_branch_predicate ^= 1;
    assert!(!v33_transition_reset_oracle(&changed_predicate));
    let mut same_vapor_side = trace;
    same_vapor_side.opposite_raw_vapor_side = 1;
    assert!(!v33_transition_reset_oracle(&same_vapor_side));
}

fn canonical_phase_projection(total_water: f64, enthalpy: f64) -> (f64, f64, f64, f64) {
    const L_F: f64 = 333_600.0;
    if enthalpy < 0.0 {
        (total_water, 0.0, -enthalpy, 0.0)
    } else if enthalpy < L_F * total_water {
        let liquid = enthalpy / L_F;
        (total_water - liquid, liquid, 0.0, 0.0)
    } else {
        (0.0, total_water, 0.0, enthalpy - L_F * total_water)
    }
}

#[test]
fn v33_known_phase_roots_use_the_unchanged_canonical_projection() {
    let cold = canonical_phase_projection(0.31, -1_234.0);
    assert_eq!(cold, (0.31, 0.0, 1_234.0, 0.0));

    let mixed = canonical_phase_projection(0.31, 333_600.0 * 0.12);
    assert!((mixed.0 - 0.19).abs() <= f64::EPSILON);
    assert_eq!(mixed.1.to_bits(), 0.12_f64.to_bits());
    assert_eq!((mixed.2, mixed.3), (0.0, 0.0));

    let fusion = canonical_phase_projection(0.31, 333_600.0 * 0.31);
    assert_eq!(fusion, (0.0, 0.31, 0.0, 0.0));
}

#[test]
fn v33_corrective_production_seams_are_required() {
    let fixed_point = read(FIXED_POINT);
    let open_snow = read(OPEN_SNOW);
    let open_snow_tests = read(OPEN_SNOW_TESTS);
    let coupled = fs::read_to_string(COUPLED_SOLVE).unwrap_or_default();
    let production = format!("{fixed_point}\n{open_snow}\n{coupled}");
    let mut defects = Vec::new();
    if coupled.contains(".map(|(mapped, coordinate)| mapped - coordinate)") {
        defects.push("forbidden coordinate-map F(x)-x residual draft".to_owned());
    }
    if fixed_point.contains("previous_previous_stage3 == candidate_stage3") {
        defects.push("forbidden bitwise raw-authentic A==A trigger".to_owned());
    }
    for required in [
        "phase_consistent_coupled_active_set_transition_reset_v1",
        "phase_consistent_coupled_active_set_transition_window_v1",
        "CoveredPhaseConsistentResidualInputsV1",
        "CoveredPhaseConsistentResidualEvaluationV1",
        "covered_phase_consistent_residual_evaluate_v1",
        "r_w_kg_m2",
        "r_h_j_m2",
        "r_e_j_m2",
        "r_t_k",
        "CoveredPhysicalEvaluationBudgetV1",
        "covered_physical_evaluation_budget_charge_v1",
        "enum CoveredConvergenceAdmissionV1",
        "CoveredConvergenceAdmissionV1::CoupledAuthentic",
    ] {
        if !production.contains(required) {
            defects.push(format!("missing corrective production seam {required}"));
        }
    }
    for required in [
        "v33_transition_reset_allows_asymptotically_changing_authentic_owner",
        "v33_transition_reset_refuses_join_or_reset_mutation",
        "v33_transition_window_rearms_stale_root_then_dispatches_exact_reset",
        "v33_transition_window_never_dispatches_nonexact_reset",
        "v33_physical_residual_evaluator_reconstructs_r_w_r_h_r_e_r_t",
        "v33_one_budget_spans_jacobian_rejections_fresh_and_replay",
        "v33_coupled_authentic_bypasses_only_picard_equality",
    ] {
        if !open_snow_tests.contains(required) {
            defects.push(format!("missing corrective behavior obligation {required}"));
        }
    }
    assert!(
        defects.is_empty(),
        "current version-33 draft is nonconforming:\n{}",
        defects.join("\n")
    );
}
