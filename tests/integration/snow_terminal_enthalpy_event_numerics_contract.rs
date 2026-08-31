use std::fs;

const ENERGY: &str = "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md";
const FREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const FIXED_POINT: &str = "crates/openwepp-hillslope-orchestrator/src/v11_covered/fixed_point.rs";
const OPEN_SNOW: &str = "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow.rs";
const OPEN_SNOW_TESTS: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_tests.rs";
const COUPLED_SOLVE: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/phase_consistent_coupled_solve.rs";
const PACKAGE: &str =
    "docs/work-packages/20260807-snow-terminal-enthalpy-event-numerics-001/package.md";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn contracts_admit_only_event_local_terminal_snow_numerics() {
    let energy = read(ENERGY);
    let freeze = read(FREEZE);
    for required in [
        "contract_version: 33",
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
    assert!(index.contains("v33 retains the exact 60-second floor"));
    assert!(index.contains("fresh `CoupledAuthentic` admission"));
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
    let index = read(INDEX);
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
    assert!(index.contains("active-set root/interface"));
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
    let index = read(INDEX);
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
    assert!(index.contains("without raw-owner bit equality"));
    assert!(index.contains("concrete `R_W/R_H/R_E/R_T`"));
    assert!(index.contains("fresh `CoupledAuthentic` admission"));
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
