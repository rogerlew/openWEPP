use std::fs;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const PACKAGE: &str = "docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/package.md";
const SUPPORT: &str =
    "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs";
const SOLVER: &str = "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs";
const SOLVER_PARENT: &str = "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs";
const EVALUATION: &str = "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs";
const ERRORS: &str = "crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs";
const RUNNER: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs";
const RUNNER_EVALUATION: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00h_snow_stage3_evaluation_trace.rs";
const PUBLICATION: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/02_publication_and_manifest_helpers.rs";
const BUILDERS: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn v128_admits_only_bounded_typed_shadow_observability() {
    let contract = read(CONTRACT);

    for required in [
        "contract_version: 128",
        "REF-SNOWFREEZE-STAGE3-SHADOW-OBSERVABILITY",
        "INV-SNOWFREEZE-095",
        "OBL-SNOWFREEZE-P-068",
        "OBL-SNOWFREEZE-C-010",
        "absent-by-default typed request",
        "same_state_paired_carrier_v1",
        "sequential_resolved_shadow_v1",
        "stage3_carrier_pair_v1",
        "stage3_surface_energy_v1",
        "stage3_complete_carrier_v1",
        "bounded_response_experiment",
        "Enabled evaluation emits internal schema v5 only",
        "authoritative state/ledgers/outputs remain exact",
        "Internal conduction may not be relabeled snow-ground flux",
        "Turbulent primitive failures preserve their typed meteorology source",
        "move `runoff_reconciliation.rs` below 3,000 lines",
        "Bounded realization closed 2026-08-06",
        "No longer blocks claims that runtime conforms to the bounded operators",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn v128_binds_real_consumer_and_retains_campaign_holds() {
    let contract = read(CONTRACT);

    for required in [
        "real internal JSONL consumer must read every new operand",
        "independently reject production/adjacent aliases",
        "requested/evaluated support and coverage",
        "consumer may read schema-v5 evidence as state or authority",
        "cross-interval persistence",
        "complete same-substep phase/liquid chronology",
        "seasonal/terminal claims",
        "production consumers",
        "cutover remain held",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn lifecycle_index_and_package_name_the_same_realization() {
    let index = read(INDEX);
    let package = read(PACKAGE);

    for required in ["SC-SNOWFREEZE-001", "v128", "schema-v5 observability"] {
        assert!(index.contains(required), "{INDEX} missing {required}");
    }
    for required in [
        "Stage 3 Shadow Solver Extraction And Observability",
        "schema-v5",
        "CoE remains the sole current melt owner",
        "No equation, coefficient, threshold",
    ] {
        assert!(package.contains(required), "{PACKAGE} missing {required}");
    }
}

#[test]
fn runtime_uses_typed_operators_and_bounded_extracted_modules() {
    let support = read(SUPPORT);
    let solver_core = read(SOLVER);
    let solver_parent = read(SOLVER_PARENT);
    let solver = format!("{solver_core}\n{solver_parent}");
    let evaluation = read(EVALUATION);
    let errors = read(ERRORS);

    for required in [
        "pub enum SnowStage3EvaluationOperator",
        "SameStatePairedCarrierV1",
        "SequentialResolvedShadowV1",
        "pub complete_carrier_shadow: bool",
        "pub evaluation: Option<DirectSnowStage3EvaluationDiagnostics>",
        "pub struct DirectSnowStage3EvaluationResult",
        "pub struct DirectSnowStage3EvaluationHourDiagnostics",
    ] {
        assert!(support.contains(required), "{SUPPORT} missing {required}");
    }
    assert!(
        !support.contains("pub stage3_evaluation_operator:"),
        "legacy public options record must remain exact"
    );
    for required in [
        "struct Stage3EvaluationTag",
        "pub fn compute_direct_snow_liquid_partition_with_evaluation",
        "pub fn compute_direct_snow_liquid_partition_with_capture_and_evaluation",
        "coverage_id: \"evaluated_seconds_over_requested_seconds_v1\"",
        "evaluate_stage3_same_state_paired_carrier",
        "evaluate_stage3_sequential_melt_shadow",
        "complete_arm_component_residual_j_m2",
        "snow_ground_cross_day_terminal_recipient_unresolved_v1",
    ] {
        assert!(solver.contains(required), "{SOLVER} missing {required}");
    }
    let tag_position = solver
        .find("let tag = Stage3EvaluationTag::new(operator)")
        .expect("typed tag construction");
    let clone_position = solver
        .find("layers.clone()")
        .expect("bounded sequential clone");
    assert!(
        tag_position < clone_position,
        "complete tag must precede clone allocation"
    );
    for required in [
        "stage3_shadow_fingerprints",
        "stage3_fnv1a_u64",
        "evaluated_seconds",
        "SnowStage3TurbulentTransferError",
    ] {
        assert!(
            evaluation.contains(required),
            "{EVALUATION} missing {required}"
        );
    }
    assert!(errors.contains("SnowStage3TurbulentTransfer"));
    let legacy_hour = support
        .split("pub struct DirectSnowSurfaceEnergyHourDiagnostics")
        .nth(1)
        .and_then(|text| {
            text.split("impl DirectSnowSurfaceEnergyHourDiagnostics")
                .next()
        })
        .expect("legacy hourly diagnostics body");
    for forbidden in [
        "shadow_shortwave_energy_j_m2",
        "shadow_longwave_energy_j_m2",
        "shadow_internal_active_lower_conduction_j_m2",
        "shadow_cold_content_export_j_m2",
        "shadow_requested_seconds",
        "shadow_evaluated_seconds",
    ] {
        assert!(
            !legacy_hour.contains(forbidden),
            "legacy hourly API gained {forbidden}"
        );
    }
    let legacy_stage3 = support
        .split("pub struct DirectSnowStage3Diagnostics")
        .nth(1)
        .and_then(|text| {
            text.split("pub struct DirectSnowStage3EvaluationDiagnostics")
                .next()
        })
        .expect("legacy Stage 3 diagnostics body");
    assert!(!legacy_stage3.contains("pub evaluation:"));
    let legacy_error = errors
        .split("pub enum Wb11HydrologyKernelGuardError")
        .nth(1)
        .and_then(|text| text.split("impl Wb11HydrologyKernelGuardError").next())
        .expect("legacy kernel error body");
    assert!(!legacy_error.contains("SnowStage3TurbulentTransfer"));
    assert!(solver_core.lines().count() < 3_000);
    assert!(solver_parent.lines().count() < 3_000);
    assert!(evaluation.lines().count() < 3_000);
}

#[test]
fn real_trace_consumer_has_enabled_only_v5_and_all_audit_families() {
    let runner = format!("{}\n{}", read(RUNNER), read(RUNNER_EVALUATION));
    for required in [
        "openwepp-r7h-direct-production-snow-trace-v4",
        "openwepp-r7h-direct-production-snow-trace-v5",
        "stage3_evaluation_operator_id",
        "stage3_evaluation_pairing_id",
        "stage3_evaluation_non_formulation_fingerprint_fnv1a64",
        "stage3_evaluation_surface_arm_non_formulation_fingerprint_fnv1a64",
        "stage3_evaluation_complete_arm_non_formulation_fingerprint_fnv1a64",
        "stage3_evaluation_complete_arm_shortwave_j_m2",
        "stage3_evaluation_complete_arm_longwave_j_m2",
        "stage3_evaluation_complete_arm_sensible_j_m2",
        "stage3_evaluation_complete_arm_latent_j_m2",
        "stage3_evaluation_complete_arm_advected_j_m2",
        "stage3_evaluation_complete_arm_internal_active_lower_conduction_j_m2",
        "stage3_evaluation_complete_arm_vapor_mass_exchange_kg_m2",
        "stage3_evaluation_complete_arm_cold_content_export_j_m2",
        "stage3_evaluation_hourly_cold_content_export_j_m2",
        "stage3_evaluation_complete_arm_available_ice_kg_m2",
        "stage3_evaluation_complete_arm_terminal_unallocated_j_m2",
        "stage3_evaluation_complete_arm_component_residual_j_m2",
        "stage3_evaluation_hourly_complete_energy_j_m2",
        "stage3_evaluation_hourly_melt_kg_m2",
        "stage3_evaluation_hourly_energy_closure_residual_j_m2",
        "stage3_evaluation_requested_seconds",
        "stage3_evaluation_evaluated_seconds",
        "schema_v5_consumer_reconstructs_shadow_operands_and_rejects_production_aliases",
        "full_solver_rows_reconstruct_all_v5_operands_and_reject_adjacent_aliases",
    ] {
        assert!(runner.contains(required), "{RUNNER} missing {required}");
    }
    assert!(!runner.contains("complete_carrier_shadow: bool"));
}

#[test]
fn public_wat_hbp_pass_projection_cannot_read_evaluation_evidence() {
    for path in [PUBLICATION, BUILDERS] {
        let source = read(path);
        assert!(
            !source.contains(".evaluation")
                && !source.contains("stage3_evaluation_complete_arm")
                && !source.contains("stage3_evaluation_hourly"),
            "{path} must not consume evaluation evidence"
        );
    }
}
