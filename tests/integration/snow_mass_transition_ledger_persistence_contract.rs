const CONTRACT: &str =
    include_str!("../../docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
const SUPPORT: &str = include_str!(
    "../../crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs"
);
const LEDGERS: &str = include_str!(
    "../../crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/snow_mass_transition.rs"
);
const RUNOFF: &str = include_str!(
    "../../crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs"
);
const STORAGE: &str =
    include_str!("../../crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs");
const RUNNER: &str = include_str!(
    "../../crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs"
);
const RUNNER_SNOW_AUTHORITY: &str = include_str!(
    "../../crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs"
);
const RUNNER_CAPTURE: &str = include_str!(
    "../../crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00g_snow_diagnostic_capture.rs"
);
use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowDiagnosticCapture, DirectSnowHourlyForcing,
    DirectSnowLayerState, DirectSnowLiquidDispositionLedger, DirectSnowMassTransitionLedgerError,
    DirectSnowMassTransitionLedgers, DirectSnowSolidToLiquidLedger, DirectSnowStage3Outcome,
    SnowDensityModel, SnowMeltModel, SnowStage3LiquidRoutingModel, Wb11HydrologyKernel,
};

#[test]
fn snowfreeze_v124_binds_one_authority_two_ledgers_and_optional_capture() {
    for required in [
        "contract_version: 124",
        "INV-SNOWFREEZE-091",
        "OBL-SNOWFREEZE-P-064",
        "TOL-SNOWFREEZE-016",
        "snow_solid_to_liquid_ledger",
        "snow_liquid_disposition_ledger",
        "snow_stage3_outcome",
        "snow_verbose_diagnostic_capture",
        "liquid_handoff_m - snowpack_swe_loss_m - rain_released_m",
    ] {
        assert!(
            CONTRACT.contains(required),
            "missing v124 authority: {required}"
        );
    }
}

#[test]
fn compact_carriers_replace_duplicate_transition_fields_and_eager_payloads() {
    for required in [
        "pub enum DirectSnowDiagnosticCapture",
        "pub struct DirectSnowSolidToLiquidLedger",
        "pub struct DirectSnowLiquidDispositionLedger",
        "pub struct DirectSnowMassTransitionLedgers",
        "pub struct DirectSnowStage3Outcome",
        "pub struct DirectSnowVerboseDiagnostics",
        "pub mass_transition_ledgers: Box<DirectSnowMassTransitionLedgers>",
        "pub verbose_diagnostics: Option<Box<DirectSnowVerboseDiagnostics>>",
    ] {
        assert!(
            SUPPORT.contains(required) || LEDGERS.contains(required) || STORAGE.contains(required),
            "missing compact carrier: {required}"
        );
    }
    assert!(
        !SUPPORT
            .contains("pub accumulation_melt_diagnostics: DirectSnowAccumulationMeltDiagnostics")
    );
    assert!(!SUPPORT.contains("pub stage3_diagnostics: DirectSnowStage3Diagnostics"));
    assert!(RUNOFF.contains("compute_direct_snow_liquid_partition_with_capture"));
}

#[test]
fn runner_resolves_capture_before_the_snow_solve_and_real_writer_requires_payload() {
    for required in [
        "DirectSnowDiagnosticCaptureRequest::resolve",
        "snow_diagnostic_capture",
        "compute_direct_snow_liquid_partition_with_capture",
        "selected_snow_verbose_diagnostics",
    ] {
        assert!(
            RUNNER.contains(required)
                || RUNNER_SNOW_AUTHORITY.contains(required)
                || RUNNER_CAPTURE.contains(required),
            "missing real capture path: {required}"
        );
    }
    let capture_position = RUNNER
        .find("DirectSnowDiagnosticCaptureRequest::resolve")
        .expect("capture request must exist");
    let solve_position = RUNNER
        .find("snow_liquid_partition(")
        .expect("snow solve must exist");
    assert!(capture_position < solve_position);
}

#[test]
fn compact_type_footprints_remove_eager_hourly_arrays() {
    use openwepp_hillslope_orchestrator::{
        DirectSnowLiquidPartition, DirectSnowVerboseDiagnostics,
    };

    let partition_bytes = std::mem::size_of::<DirectSnowLiquidPartition>();
    let compact_ledgers_bytes = std::mem::size_of::<DirectSnowSolidToLiquidLedger>()
        + std::mem::size_of::<DirectSnowLiquidDispositionLedger>()
        + std::mem::size_of::<DirectSnowStage3Outcome>();
    let optional_verbose_handle_bytes =
        std::mem::size_of::<Option<Box<DirectSnowVerboseDiagnostics>>>();

    println!(
        "partition_bytes={partition_bytes} compact_ledgers_bytes={compact_ledgers_bytes} optional_verbose_handle_bytes={optional_verbose_handle_bytes}"
    );
    assert!(partition_bytes < 15_816, "must improve on scaffold carrier");
    assert!(compact_ledgers_bytes <= 128);
    assert_eq!(optional_verbose_handle_bytes, std::mem::size_of::<usize>());
}

fn capture_inputs() -> DirectActiveSnowPartitionInputs {
    let mut hourly = [DirectSnowHourlyForcing {
        air_temperature_c: 6.0,
        cloud_fraction: 0.2,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    for hour in hourly.iter_mut().take(2) {
        hour.air_temperature_c = 2.0;
    }
    hourly[2] = DirectSnowHourlyForcing {
        active_precipitation_m: 0.05,
        rain_m: 0.05,
        rain_fraction: 1.0,
        air_temperature_c: 4.0,
        cloud_fraction: 0.2,
        ..DirectSnowHourlyForcing::zero()
    };
    DirectActiveSnowPartitionInputs {
        hyetograph_rainfall_m: 0.05,
        rst_c: 0.0,
        newsnw_kg_m3: 100.0,
        ssd_kg_m3: 522.0,
        runtime_swe_m: 0.18,
        runtime_depth_m: 0.40,
        runtime_density_kg_m3: 450.0,
        runtime_settle_day_count: 12.0,
        liquid_water_retained_m: 0.0,
        tmax_c: 6.0,
        tmin_c: 2.0,
        canopy_cover_fraction: 0.0,
        wind_m_s: 2.0,
        dewpoint_c: 0.0,
        snow_melt_model: SnowMeltModel::CoeLiquidHoldingCapacityV1,
        snow_density_model: SnowDensityModel::PhysicsBulkMultilayerDensityV1,
        stage3_liquid_routing_model: SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1,
        surface_energy_options:
            openwepp_hillslope_orchestrator::DirectSnowSurfaceEnergyOptions::default(),
        sturm_climate_class: None,
        sturm_day_of_year: None,
        coe_boundary_depth_m: 0.40,
        coe_boundary_density_kg_m3: 450.0,
        coe_boundary_settle_day_count: 12.0,
        snow_albedo_model: None,
        snow_albedo_state: None,
        snow_layers: vec![
            DirectSnowLayerState::new(0.09, 0.20, 450.0, 12.0)
                .with_stage3_thermal_liquid_state(-0.2, 0.0, 0.0, 0.0),
            DirectSnowLayerState::new(0.09, 0.20, 450.0, 12.0),
        ],
        underlying_surface_albedo: 0.2,
        hourly,
    }
}

#[test]
fn disabled_capture_removes_payload_without_changing_production_or_guards() {
    let inputs = capture_inputs();
    let disabled = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_capture(
        &inputs,
        DirectSnowDiagnosticCapture::Disabled,
    )
    .expect("disabled capture solve");
    let mut verbose = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_capture(
        &inputs,
        DirectSnowDiagnosticCapture::Verbose,
    )
    .expect("verbose capture solve");
    assert!(disabled.verbose_diagnostics.is_none());
    assert!(verbose.verbose_diagnostics.is_some());
    assert!(disabled.stage3_outcome().enabled);
    assert!(disabled.solid_to_liquid_ledger().liquid_handoff_m > 0.0);
    assert!(disabled.liquid_disposition_ledger().incoming_liquid_m > 0.0);
    verbose.verbose_diagnostics = None;
    assert_eq!(
        disabled, verbose,
        "capture cannot alter production outcomes"
    );
    disabled
        .mass_transition_ledgers
        .validate()
        .expect("disabled capture retains compact closure guards");
}

#[test]
fn immutable_linked_ledgers_reconstruct_both_boundaries_and_reject_aliases() {
    let solid = DirectSnowSolidToLiquidLedger {
        raw_signed_melt_m: 0.20,
        redistributed_positive_melt_m: 0.15,
        snowpack_swe_loss_m: 0.10,
        rain_released_m: 0.03,
        liquid_handoff_m: 0.13,
    };
    let liquid = DirectSnowLiquidDispositionLedger {
        incoming_liquid_m: 0.13,
        routed_liquid_m: 0.08,
        retained_liquid_delta_m: 0.02,
        refrozen_liquid_m: 0.03,
        liquid_closure_residual_m: 0.0,
    };
    let outcome = DirectSnowStage3Outcome {
        enabled: true,
        meltwater_temperature_c: Some(
            openwepp_unit_boundary::TemperatureCelsius::try_new(0.0).expect("freezing temperature"),
        ),
        sublimation_m: 0.0,
    };
    let linked = DirectSnowMassTransitionLedgers::try_from_parts(solid, liquid, outcome)
        .expect("deliberately separated operands close");
    let persisted_solid = linked.solid_to_liquid();
    let persisted_liquid = linked.liquid_disposition();
    assert!(
        (persisted_solid.liquid_handoff_m
            - persisted_solid.snowpack_swe_loss_m
            - persisted_solid.rain_released_m)
            .abs()
            <= 1.0e-9
    );
    assert!(
        (persisted_liquid.incoming_liquid_m
            - persisted_liquid.routed_liquid_m
            - persisted_liquid.retained_liquid_delta_m
            - persisted_liquid.refrozen_liquid_m
            - persisted_liquid.liquid_closure_residual_m)
            .abs()
            <= 1.0e-9
    );
    assert!(
        (persisted_solid.raw_signed_melt_m - persisted_solid.snowpack_swe_loss_m).abs() > 1.0e-9
    );
    assert!(
        (persisted_solid.redistributed_positive_melt_m - persisted_solid.snowpack_swe_loss_m).abs()
            > 1.0e-9
    );
    assert!((persisted_liquid.routed_liquid_m - persisted_solid.liquid_handoff_m).abs() > 1.0e-9);

    assert_alias_substitutions_fail(solid, liquid, outcome);
}

fn assert_alias_substitutions_fail(
    solid: DirectSnowSolidToLiquidLedger,
    liquid: DirectSnowLiquidDispositionLedger,
    outcome: DirectSnowStage3Outcome,
) {
    for (mutated_solid, mutated_liquid, expected) in [
        (
            DirectSnowSolidToLiquidLedger {
                snowpack_swe_loss_m: solid.raw_signed_melt_m,
                ..solid
            },
            liquid,
            DirectSnowMassTransitionLedgerError::UpstreamClosure,
        ),
        (
            DirectSnowSolidToLiquidLedger {
                snowpack_swe_loss_m: solid.redistributed_positive_melt_m,
                ..solid
            },
            liquid,
            DirectSnowMassTransitionLedgerError::UpstreamClosure,
        ),
        (
            solid,
            DirectSnowLiquidDispositionLedger {
                incoming_liquid_m: 0.12,
                routed_liquid_m: 0.07,
                ..liquid
            },
            DirectSnowMassTransitionLedgerError::Stage3HandoffLink,
        ),
        (
            solid,
            DirectSnowLiquidDispositionLedger {
                routed_liquid_m: solid.liquid_handoff_m,
                ..liquid
            },
            DirectSnowMassTransitionLedgerError::Stage3Closure,
        ),
        (
            solid,
            DirectSnowLiquidDispositionLedger {
                retained_liquid_delta_m: 0.0,
                ..liquid
            },
            DirectSnowMassTransitionLedgerError::Stage3Closure,
        ),
        (
            solid,
            DirectSnowLiquidDispositionLedger {
                refrozen_liquid_m: 2.0 * liquid.refrozen_liquid_m,
                ..liquid
            },
            DirectSnowMassTransitionLedgerError::Stage3Closure,
        ),
        (
            solid,
            DirectSnowLiquidDispositionLedger {
                liquid_closure_residual_m: 0.02,
                ..liquid
            },
            DirectSnowMassTransitionLedgerError::Stage3Closure,
        ),
    ] {
        assert_eq!(
            DirectSnowMassTransitionLedgers::try_from_parts(
                mutated_solid,
                mutated_liquid,
                outcome,
            )
            .expect_err("alias substitution must fail closed"),
            expected
        );
    }
}

#[test]
fn signed_raw_melt_and_retained_delta_are_valid_ledger_operands() {
    let linked = DirectSnowMassTransitionLedgers::try_from_parts(
        DirectSnowSolidToLiquidLedger {
            raw_signed_melt_m: -0.04,
            redistributed_positive_melt_m: 0.01,
            snowpack_swe_loss_m: 0.10,
            rain_released_m: 0.03,
            liquid_handoff_m: 0.13,
        },
        DirectSnowLiquidDispositionLedger {
            incoming_liquid_m: 0.13,
            routed_liquid_m: 0.10,
            retained_liquid_delta_m: -0.02,
            refrozen_liquid_m: 0.05,
            liquid_closure_residual_m: 0.0,
        },
        DirectSnowStage3Outcome {
            enabled: true,
            meltwater_temperature_c: Some(
                openwepp_unit_boundary::TemperatureCelsius::try_new(0.0)
                    .expect("freezing temperature"),
            ),
            sublimation_m: 0.0,
        },
    )
    .expect("signed transition operands are authoritative and close");

    assert!(linked.solid_to_liquid().raw_signed_melt_m < 0.0);
    assert!(linked.liquid_disposition().retained_liquid_delta_m < 0.0);
}

#[test]
fn ledger_validation_preserves_failure_categories() {
    let valid_solid = DirectSnowSolidToLiquidLedger {
        snowpack_swe_loss_m: 0.10,
        rain_released_m: 0.03,
        liquid_handoff_m: 0.13,
        ..DirectSnowSolidToLiquidLedger::default()
    };
    let valid_liquid = DirectSnowLiquidDispositionLedger {
        incoming_liquid_m: 0.13,
        routed_liquid_m: 0.10,
        retained_liquid_delta_m: -0.02,
        refrozen_liquid_m: 0.05,
        ..DirectSnowLiquidDispositionLedger::default()
    };
    let valid_outcome = DirectSnowStage3Outcome {
        enabled: true,
        meltwater_temperature_c: Some(
            openwepp_unit_boundary::TemperatureCelsius::try_new(0.0).expect("freezing temperature"),
        ),
        sublimation_m: 0.0,
    };

    for (solid, liquid, outcome, expected) in [
        (
            DirectSnowSolidToLiquidLedger {
                raw_signed_melt_m: f64::NAN,
                ..valid_solid
            },
            valid_liquid,
            valid_outcome,
            DirectSnowMassTransitionLedgerError::NonFinite {
                field: "raw_signed_melt_m",
            },
        ),
        (
            DirectSnowSolidToLiquidLedger {
                snowpack_swe_loss_m: -0.01,
                ..valid_solid
            },
            valid_liquid,
            valid_outcome,
            DirectSnowMassTransitionLedgerError::Negative {
                field: "snowpack_swe_loss_m",
            },
        ),
        (
            valid_solid,
            valid_liquid,
            DirectSnowStage3Outcome {
                meltwater_temperature_c: None,
                ..valid_outcome
            },
            DirectSnowMassTransitionLedgerError::Stage3Outcome,
        ),
        (
            DirectSnowSolidToLiquidLedger::default(),
            DirectSnowLiquidDispositionLedger {
                incoming_liquid_m: 0.01,
                ..DirectSnowLiquidDispositionLedger::default()
            },
            DirectSnowStage3Outcome::default(),
            DirectSnowMassTransitionLedgerError::DisabledStage3Ledger,
        ),
        (
            DirectSnowSolidToLiquidLedger::default(),
            DirectSnowLiquidDispositionLedger::default(),
            DirectSnowStage3Outcome {
                meltwater_temperature_c: Some(
                    openwepp_unit_boundary::TemperatureCelsius::try_new(0.0)
                        .expect("freezing temperature"),
                ),
                ..DirectSnowStage3Outcome::default()
            },
            DirectSnowMassTransitionLedgerError::DisabledStage3Outcome,
        ),
    ] {
        assert_eq!(
            DirectSnowMassTransitionLedgers::try_from_parts(solid, liquid, outcome)
                .expect_err("invalid ledger category must fail closed"),
            expected
        );
    }
}
