use std::error::Error;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::constants::{WB11_ZERO_THRESHOLD, WB16_PEAKRO_FLOOR, WB16_RUNOFF_NEAR_ZERO_THRESHOLD};
use crate::hydrology::{DirectSnowStage3Diagnostics, SnowAlbedoState};
use crate::winter_column::{
    DirectFrostFineLayerState, DirectFrostLaneState, DirectFrostLayerShadowState,
    DirectSnowLaneState, DirectSnowLayerState, DirectWinterColumnState,
};

pub const DIRECT_TRANSFER_HOUR_COUNT: usize = 24;
pub const DIRECT_PHASE_COUNT: usize = 14;
pub const DIRECT_R3A_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R3A_INPUT_ACCOUNTING_SPAN: [DirectPhaseKind; DIRECT_R3A_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Normalization,
    DirectPhaseKind::LateralTransfer,
];
pub const DIRECT_R3B_PHASE_SPAN_COUNT: usize = 3;
pub const DIRECT_R3B_WATER_LEDGER_SPAN: [DirectPhaseKind; DIRECT_R3B_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::StorageReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R3C_PHASE_SPAN_COUNT: usize = 3;
pub const DIRECT_R3C_LANE_TRANSFER_SPAN: [DirectPhaseKind; DIRECT_R3C_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::LateralTransfer,
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R4A_PHASE_SPAN_COUNT: usize = 3;
pub const DIRECT_R4A_RUNOFF_PARTITION_SPAN: [DirectPhaseKind; DIRECT_R4A_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::StorageReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R4B_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4B_STORAGE_RECONCILIATION_SPAN: [DirectPhaseKind; DIRECT_R4B_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::StorageReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R4C_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4C_STORAGE_INPUT_SPAN: [DirectPhaseKind; DIRECT_R4C_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Normalization,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4D_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4D_DEEP_SEEPAGE_SPAN: [DirectPhaseKind; DIRECT_R4D_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::PercolationDeepSeepage,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4E_PHASE_SPAN_COUNT: usize = 3;
pub const DIRECT_R4E_SUBSURFACE_LOSS_SPAN: [DirectPhaseKind; DIRECT_R4E_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Drainage,
    DirectPhaseKind::LateralTransfer,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4F_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4F_EVAPOTRANSPIRATION_SPAN: [DirectPhaseKind; DIRECT_R4F_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Evapotranspiration,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4G_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4G_SNOW_COUPLING_SPAN: [DirectPhaseKind; DIRECT_R4G_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Normalization,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4I_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4I_LIQUID_INPUT_SPAN: [DirectPhaseKind; DIRECT_R4I_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Normalization,
    DirectPhaseKind::RunoffReconciliation,
];
pub const DIRECT_R4J_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4J_RUNON_CARRY_SPAN: [DirectPhaseKind; DIRECT_R4J_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::LateralTransfer,
    DirectPhaseKind::RunoffReconciliation,
];
pub const DIRECT_R4K_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4K_INFILTRATION_DEPRESSION_SPAN: [DirectPhaseKind; DIRECT_R4K_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4L_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4L_SATURATION_ADDBACK_SPAN: [DirectPhaseKind; DIRECT_R4L_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4M_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4M_PERCOLATION_SPAN: [DirectPhaseKind; DIRECT_R4M_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::PercolationDeepSeepage,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4O_PHASE_SPAN_COUNT: usize = 3;
pub const DIRECT_R4O_SUBSURFACE_SPAN: [DirectPhaseKind; DIRECT_R4O_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Drainage,
    DirectPhaseKind::LateralTransfer,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4N_SURFACE_PHASE_SPAN_COUNT: usize = 1;
pub const DIRECT_R4N_SURFACE_ET_SPAN: [DirectPhaseKind; DIRECT_R4N_SURFACE_PHASE_SPAN_COUNT] =
    [DirectPhaseKind::Evapotranspiration];
pub const DIRECT_R4N_ROOT_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4N_ROOT_UPTAKE_SPAN: [DirectPhaseKind; DIRECT_R4N_ROOT_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::PlantRootUptake,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4N_PHASE_SPAN_COUNT: usize =
    DIRECT_R4N_SURFACE_PHASE_SPAN_COUNT + DIRECT_R4N_ROOT_PHASE_SPAN_COUNT;
pub const DIRECT_R4PQZ_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4PQZ_HYDROLOGY_PROJECTION_SPAN: [DirectPhaseKind; DIRECT_R4PQZ_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::StorageReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R7D6_PEAK_RUNOFF_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R7D6_PEAK_RUNOFF_SPAN: [DirectPhaseKind;
    DIRECT_R7D6_PEAK_RUNOFF_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT: usize = 1;
pub const DIRECT_R5B_NORMALIZATION_SPAN: [DirectPhaseKind;
    DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT] = [DirectPhaseKind::Normalization];
pub const DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT: usize = 1;
pub const DIRECT_R5B_STORAGE_BOUNDS_SPAN: [DirectPhaseKind;
    DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT] = [DirectPhaseKind::StorageBounds];
pub const DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT: usize = 1;
pub const DIRECT_R5C_DECOMPOSITION_SPAN: [DirectPhaseKind;
    DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT] = [DirectPhaseKind::DecompositionTransition];
pub const DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT: usize = 1;
pub const DIRECT_R5C_RESIDUE_PARTITION_SPAN: [DirectPhaseKind;
    DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT] = [DirectPhaseKind::ResiduePartitionTransition];
pub const DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT: usize = 1;
pub const DIRECT_R5D_ANNUAL_GROWTH_SPAN: [DirectPhaseKind;
    DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT] = [DirectPhaseKind::AnnualGrowthTransition];
pub const DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT: usize = 1;
pub const DIRECT_R5D_PERENNIAL_GROWTH_SPAN: [DirectPhaseKind;
    DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT] = [DirectPhaseKind::PerennialGrowthTransition];
pub const DIRECT_R7D6_EROSION_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R7D6_EROSION_SPAN: [DirectPhaseKind; DIRECT_R7D6_EROSION_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];

static DIRECT_AUDIT: DirectRuntimeAuditCounters = DirectRuntimeAuditCounters::new();

mod decomposition;
mod diagnostic_events;
mod erosion;
mod erosion_continuity;
mod erosion_operands;
mod evapotranspiration;
mod growth;
mod normalization;
mod projection;
mod runoff;
mod storage;
mod subsurface;

pub use decomposition::{
    DirectDecompositionAction, DirectDecompositionActiveContext,
    DirectDecompositionDownstreamOperands, DirectDecompositionInputs,
    DirectDecompositionShadowProjection, DirectDecompositionSpanReport, DirectDecompositionState,
    DirectResiduePartitionDownstreamOperands, DirectResiduePartitionInputs,
    DirectResiduePartitionShadowProjection, DirectResiduePartitionSpanReport,
    DirectResiduePartitionState,
};
pub use diagnostic_events::{
    DirectEvapotranspirationTraceEvent, DirectPercolationTraceEvent,
    DirectSubsurfaceSaturationTraceEvent,
};
pub use erosion::{
    DirectErod13Inputs, DirectErod13State, DirectErod14ClassInputs, DirectErod14ClassState,
    DirectErod14Inputs, DirectErod14State, DirectErosionDownstreamOperands, DirectErosionInputs,
    DirectErosionShadowProjection, DirectErosionSpanReport, DirectErosionState,
};
pub use erosion_continuity::{
    DIRECT_WAVE1_GRID_POINTS, DirectWave1ContinuityInputs, DirectWave1ContinuityState,
    DirectWave1SlopeSegment, Wave1ShearClassification, Wave1ShearRegime,
    compute_direct_wave1_continuity, derive_wave1_slope_segments, wave1_classifier_shear,
    wave1_depc, wave1_depend, wave1_depeqs, wave1_runge_step, wave1_xcrit,
};
pub use erosion_operands::{
    EROSION_PARTICLE_CLASS_COUNT, ErosionParticleClass, ErosionRillCoverInputs,
    ErosionRillHydraulics, ErosionShearSlopes, ErosionTextureInputs, ErosionTransportCoefficients,
    erosion_detinr, erosion_effective_particle, erosion_falvel, erosion_interrill_delivery_ratio,
    erosion_particle_composition, erosion_rill_hydraulics, erosion_shield,
    erosion_transport_coefficients, erosion_trcoef, erosion_yalin,
};
pub use evapotranspiration::{
    DirectEvapotranspirationComputeDownstreamOperands, DirectEvapotranspirationComputeInputs,
    DirectEvapotranspirationComputeShadowProjection, DirectEvapotranspirationComputeSpanReport,
    DirectEvapotranspirationComputeState, DirectEvapotranspirationPmetComputeInputs,
    DirectEvapotranspirationPmetInputs, DirectEvapotranspirationStageState,
    DirectEvapotranspirationSurfaceDownstreamOperands,
    DirectEvapotranspirationSurfaceShadowProjection, DirectEvapotranspirationSurfaceSpanReport,
    DirectEvapotranspirationSurfaceState,
};
pub use growth::{
    DirectGrowthAction, DirectGrowthActiveContext, DirectGrowthDownstreamOperands,
    DirectGrowthInputs, DirectGrowthShadowProjection, DirectGrowthSpanReport, DirectGrowthState,
    DirectGrowthStateSurface,
};
pub use normalization::{
    DirectNormalizationDownstreamOperands, DirectNormalizationInputs,
    DirectNormalizationShadowProjection, DirectNormalizationSpanReport, DirectNormalizationState,
    DirectStorageBoundsDownstreamOperands, DirectStorageBoundsInputs,
    DirectStorageBoundsShadowProjection, DirectStorageBoundsSpanReport, DirectStorageBoundsState,
};
pub use projection::{
    DirectHydrologyProjectionDownstreamOperands, DirectHydrologyProjectionInputs,
    DirectHydrologyProjectionShadowProjection, DirectHydrologyProjectionSpanReport,
    DirectHydrologyProjectionState,
};
#[cfg(test)]
pub(crate) fn dc01_test_hourly_supply_basis(
    hyetograph: &[DirectWb14HyetographInterval],
    runon_hourly_supply_m: &[f64; 24],
) -> Vec<DirectWb14HyetographInterval> {
    runoff::dc01_hourly_supply_basis(hyetograph, runon_hourly_supply_m)
}
#[cfg(test)]
pub(crate) fn dc01_test_wb14_with_profile(
    inputs: &DirectWb14InfiltrationProducerInputs,
) -> Result<runoff::DirectWb14OutcomeWithProfile, DirectRuntimeError> {
    runoff::compute_wb14_infiltration_depression_with_profile(inputs)
}
pub use runoff::{
    DirectCanopyInterceptionInputs, DirectCanopyInterceptionState,
    DirectInfiltrationDepressionDownstreamOperands, DirectInfiltrationDepressionInputs,
    DirectInfiltrationDepressionShadowProjection, DirectInfiltrationDepressionSpanReport,
    DirectInfiltrationDepressionState, DirectLiquidInputDownstreamOperands,
    DirectLiquidInputInputs, DirectLiquidInputShadowProjection, DirectLiquidInputSpanReport,
    DirectLiquidInputState, DirectPeakRunoffDownstreamOperands, DirectPeakRunoffInputs,
    DirectPeakRunoffShadowProjection, DirectPeakRunoffSpanReport, DirectPeakRunoffState,
    DirectRunoffDownstreamOperands, DirectRunoffPartitionInputs, DirectRunoffPartitionSpanReport,
    DirectRunoffPartitionState, DirectRunoffShadowProjection, DirectRunonCarryDownstreamOperands,
    DirectRunonCarryInputs, DirectRunonCarryShadowProjection, DirectRunonCarrySpanReport,
    DirectRunonCarryState, DirectSaturationAddbackDownstreamOperands,
    DirectSaturationAddbackInputs, DirectSaturationAddbackShadowProjection,
    DirectSaturationAddbackSpanReport, DirectSaturationAddbackState, DirectWb14HyetographInterval,
    DirectWb14InfiltrationProducerInputs, compute_direct_canopy_interception,
    wp2_frost_pair_trace_path, write_wp2_frost_pair_trace,
};
pub use storage::{
    DirectDeepSeepageDownstreamOperands, DirectDeepSeepageInputs,
    DirectDeepSeepageShadowProjection, DirectDeepSeepageSpanReport, DirectDeepSeepageState,
    DirectEvapotranspirationDownstreamOperands, DirectEvapotranspirationInputs,
    DirectEvapotranspirationShadowProjection, DirectEvapotranspirationSpanReport,
    DirectEvapotranspirationState, DirectSnowCouplingDownstreamOperands, DirectSnowCouplingInputs,
    DirectSnowCouplingShadowProjection, DirectSnowCouplingSpanReport, DirectSnowCouplingState,
    DirectStorageDownstreamOperands, DirectStorageInputDownstreamOperands,
    DirectStorageInputInputs, DirectStorageInputShadowProjection, DirectStorageInputSpanReport,
    DirectStorageInputState, DirectStorageReconciliationInputs,
    DirectStorageReconciliationSpanReport, DirectStorageReconciliationState,
    DirectStorageShadowProjection, DirectSubsurfaceLossDownstreamOperands,
    DirectSubsurfaceLossInputs, DirectSubsurfaceLossShadowProjection,
    DirectSubsurfaceLossSpanReport, DirectSubsurfaceLossState,
};
pub use subsurface::{
    DirectPercolationDownstreamOperands, DirectPercolationInputs,
    DirectPercolationShadowProjection, DirectPercolationSpanReport, DirectPercolationState,
    DirectSubsurfaceComputeDownstreamOperands, DirectSubsurfaceComputeInputs,
    DirectSubsurfaceComputeShadowProjection, DirectSubsurfaceComputeSpanReport,
    DirectSubsurfaceComputeState, DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState,
};

include!("direct_runtime/00_core_frames.rs");
include!("direct_runtime/01_publication.rs");
include!("direct_runtime/02_state_reports.rs");
include!("direct_runtime/03_executor.rs");
include!("direct_runtime/04_audit_error_helpers.rs");

#[cfg(test)]
mod cqr_row7_publication_tests {
    use super::*;

    fn valid_subsurface_layer() -> DirectSubsurfaceLayerState {
        let mut layer = DirectSubsurfaceLayerState::neutral();
        layer.depth_m = 0.20;
        layer.theta_m = 0.03;
        layer.residual_theta = 0.12;
        layer.frozen_depth_m = 0.08;
        layer.frozen_water_m = 0.01;
        layer
    }

    #[test]
    fn cqr_row7_frost_layer_carry_projection_validates_domains_and_projects_theta() {
        let layer = valid_subsurface_layer();
        let projection = DirectFrostLayerCarryProjection {
            layer_index: 1,
            fine_layer_count: 4,
            fine_layer_thickness_m: 0.05,
        };

        projection
            .validate_for_layer(1, &layer)
            .expect("valid frost carry projection should pass");
        assert!(projection.projected_theta_m(&layer).is_finite());

        let invalid_projection_cases = [
            DirectFrostLayerCarryProjection {
                layer_index: 2,
                ..projection
            },
            DirectFrostLayerCarryProjection {
                fine_layer_count: 0,
                ..projection
            },
            DirectFrostLayerCarryProjection {
                fine_layer_thickness_m: f64::NAN,
                ..projection
            },
            DirectFrostLayerCarryProjection {
                fine_layer_thickness_m: 0.30,
                ..projection
            },
        ];
        for invalid in invalid_projection_cases {
            assert!(invalid.validate_for_layer(1, &layer).is_err());
        }

        let mut invalid_layer = layer.clone();
        invalid_layer.frozen_depth_m = -1.0e-6;
        assert!(projection.validate_for_layer(1, &invalid_layer).is_err());

        invalid_layer = layer;
        invalid_layer.frozen_depth_m = 0.30;
        assert!(projection.validate_for_layer(1, &invalid_layer).is_err());
    }
}

#[cfg(test)]
mod cqr_row9_direct_runtime_tests {
    use super::evapotranspiration::{
        compute_stage_soil_evaporation, pmet_adjusted_crop_coefficient,
        pmet_evaporation_reduction_coefficient, pmet_raw_soil_evaporation_m,
        pmet_soil_evaporation_coefficient, pmet_transpiration_stress_coefficient,
    };
    use super::*;

    const TOL: f64 = 1.0e-12;

    fn row9_layer(theta_m: f64, depth_m: f64) -> DirectSubsurfaceLayerState {
        DirectSubsurfaceLayerState {
            theta_m,
            field_capacity_m: 0.04,
            upper_limit_m: 0.08,
            conductivity_m_s: 1.0e-6,
            depth_m,
            residual_theta: 0.08,
            frozen_depth_m: 0.0,
            frozen_water_m: 0.0,
            porosity: 0.45,
            field_capacity_theta: 0.30,
            coca: 1.0,
            lateral_conductivity_m_s: 1.0e-6,
        }
    }

    fn row9_pmet_inputs(radpot_ly: Option<f64>) -> DirectEvapotranspirationPmetComputeInputs {
        DirectEvapotranspirationPmetComputeInputs {
            runtime_day_of_year: 120,
            radiation_ly: 450.0,
            wind_m_s: 2.4,
            dew_point_c: 3.0,
            temperature_max_c: 18.0,
            temperature_min_c: 4.0,
            latitude_degrees: 45.0,
            elevation_m: 1_200.0,
            kcb: 1.05,
            rawp: 0.5,
            canopy_height_m: 0.30,
            radpot_ly,
            solthk_m: vec![Some(0.08), Some(0.25)],
        }
    }

    fn row9_et_inputs() -> DirectEvapotranspirationComputeInputs {
        DirectEvapotranspirationComputeInputs {
            et_demand_m: 0.006,
            leaf_area_index: 2.5,
            canopy_cover_fraction: 0.45,
            residue_interception_m: 0.0002,
            same_pass_infiltration_m: 0.0,
            outside_water_depth_m: 0.0,
            root_depth_m: 0.18,
            plant_tolerance: 0.25,
            growth_context_required: false,
            stage_state: None,
            pmet: None,
            pmet_compute: None,
        }
    }

    fn row9_layer_inputs(layer: &DirectSubsurfaceLayerState) -> DirectSubsurfaceLayerInputs {
        layer.clone().into()
    }

    fn row9_snow_carry() -> DirectSnowRuntimeCarry {
        let mut snow = DirectSnowLaneState::zero();
        snow.runtime_swe_m = 0.03;
        snow.runtime_depth_m = 0.10;
        snow.runtime_density_kg_m3 = 300.0;
        snow.coe_boundary_depth_m = 0.10;
        snow.coe_boundary_density_kg_m3 = 300.0;
        snow.layers = vec![DirectSnowLayerState::new(0.03, 0.10, 300.0, 0.0)];
        snow.into()
    }

    fn row9_frost_carry() -> DirectFrostRuntimeCarry {
        let mut frost = DirectFrostRuntimeCarry::from(DirectFrostLaneState::zero());
        frost.fine_layers = vec![DirectFrostFineLayerCarry {
            layer_index: 1,
            fine_index: 1,
            fgfrst: 0.0,
            slfsd_m: 0.0,
            slsic_m: 0.0,
            slsw_theta: 0.0,
            sltime_s: 0.0,
        }];
        frost
    }

    fn row9_aggregate_soil_water(layers: &[DirectSubsurfaceLayerState]) -> f64 {
        layers
            .iter()
            .map(|layer| {
                let unfrozen_depth_m = (layer.depth_m - layer.frozen_depth_m).max(0.0);
                layer.theta_m + layer.residual_theta * unfrozen_depth_m
            })
            .sum()
    }

    fn row9_day_with_percolation(
        identity: DirectRunIdentity,
        layers: Vec<DirectSubsurfaceLayerState>,
    ) -> DirectDayFrame {
        let soil_water_after_m = row9_aggregate_soil_water(&layers);
        let mut day = DirectDayFrame::seed(identity, 0, 0).expect("day frame");
        day.percolation = DirectPercolationState {
            soil_water_before_m: soil_water_after_m,
            computed_soil_water_before_m: soil_water_after_m,
            soil_water_after_m,
            deep_seepage_m: 0.0,
            recharge_m: 0.0,
            per_layer_flux_m: vec![0.0; layers.len()],
            layer_state_after: layers.clone(),
        };
        day.percolation_shadow_projection = Some(DirectPercolationShadowProjection {
            lane_index: 0,
            day_index: 0,
            soil_water_before_m: soil_water_after_m,
            soil_water_after_m,
            deep_seepage_m: 0.0,
            recharge_m: 0.0,
            per_layer_flux_m: vec![0.0; layers.len()],
            layer_state_after: layers,
        });
        day.evapotranspiration_compute_inputs = row9_et_inputs();
        day
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= TOL,
            "expected {actual} to equal {expected}"
        );
    }

    #[test]
    fn cqr_row9_pmet_compute_covers_nominal_storage_and_guard_paths() {
        let layers = vec![row9_layer(0.018, 0.08), row9_layer(0.030, 0.17)];
        let et = row9_et_inputs();
        let pmet = row9_pmet_inputs(Some(620.0));

        let result = pmet
            .compute(&layers, &et)
            .expect("valid PMET compute inputs should project ET components");
        assert!(result.soil_evaporation_m.is_finite());
        assert!(result.plant_transpiration_m.is_finite());
        assert!(result.soil_evaporation_m >= 0.0);
        assert!(result.plant_transpiration_m >= 0.0);

        let legacy_radpot_result = row9_pmet_inputs(None)
            .compute(&layers, &et)
            .expect("missing radpot should use the legacy sunmap path");
        assert!(legacy_radpot_result.soil_evaporation_m.is_finite());

        let (total_evaporable_water_mm, readily_evaporable_water_mm, wfevp_mm) = pmet
            .evaporation_storage_terms(&layers, 0.10)
            .expect("evaporation storage terms should cover partial top profile");
        assert!(total_evaporable_water_mm > readily_evaporable_water_mm);
        assert!(wfevp_mm > 0.0);

        let (total_available_water_mm, wftrp_mm) = pmet
            .transpiration_storage_terms(&layers, 0.12, wfevp_mm)
            .expect("transpiration storage terms should cover partial root profile");
        assert!(total_available_water_mm > 0.0);
        assert!(wftrp_mm > 0.0);

        let mut residual_gt_fc = layers.clone();
        residual_gt_fc[0].residual_theta = residual_gt_fc[0].field_capacity_theta + 0.01;
        assert!(pmet.compute(&residual_gt_fc, &et).is_err());

        let mut bad_solthk = pmet.clone();
        bad_solthk.solthk_m = vec![Some(0.08), Some(0.07)];
        assert!(bad_solthk.compute(&layers, &et).is_err());
    }

    #[test]
    fn cqr_row9_pmet_branch_helpers_cover_coefficient_edges() {
        let et = row9_et_inputs();
        let height_factor = (0.30_f64 / 3.0).powf(0.3);
        assert!(pmet_adjusted_crop_coefficient(&et, 1.05, 2.4, 50.0, height_factor) > 0.0);

        let mut bare_et = et;
        bare_et.leaf_area_index = 0.0;
        assert_eq!(
            pmet_adjusted_crop_coefficient(&bare_et, 1.05, 2.4, 50.0, height_factor).to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            pmet_soil_evaporation_coefficient(0.0, 0.0).to_bits(),
            1.2_f64.to_bits()
        );
        assert!(pmet_soil_evaporation_coefficient(1.0, 2.0) > 0.0);
        assert_eq!(
            pmet_evaporation_reduction_coefficient(10.0, 3.0, 8.0).to_bits(),
            1.0_f64.to_bits()
        );
        assert!(pmet_evaporation_reduction_coefficient(10.0, 3.0, 1.0) < 1.0);
        assert_eq!(
            pmet_transpiration_stress_coefficient(10.0, 9.0, 2.0).to_bits(),
            1.0_f64.to_bits()
        );
        assert!(pmet_transpiration_stress_coefficient(10.0, 2.0, 1.0) < 1.0);
        assert_eq!(
            pmet_raw_soil_evaporation_m(&bare_et, 0.0001, 1.2, 1.0, 2.0, 45.0, height_factor)
                .to_bits(),
            0.0001_f64.to_bits()
        );
        assert!(
            pmet_raw_soil_evaporation_m(&bare_et, 0.002, 1.2, 1.0, 2.0, 45.0, height_factor)
                > bare_et.residue_interception_m
        );
    }

    #[test]
    fn cqr_row9_stage_soil_evaporation_covers_stage_one_stage_two_and_reset() {
        let stage_one = DirectEvapotranspirationStageState {
            s1_m: 0.001,
            s2_m: 0.0,
            threshold_m: 0.003,
            counter: 0.0,
        };
        let (es_stage_one, next_stage_one) = compute_stage_soil_evaporation(stage_one, 0.0, 0.004)
            .expect("stage one evaporation should compute");
        assert!(es_stage_one > 0.0 && es_stage_one <= 0.004);
        assert!(next_stage_one.s2_m > 0.0);

        let stage_two = DirectEvapotranspirationStageState {
            s1_m: 0.004,
            s2_m: 0.003,
            threshold_m: 0.003,
            counter: 1.0,
        };
        let (es_stage_two, next_stage_two) = compute_stage_soil_evaporation(stage_two, 0.0, 0.004)
            .expect("stage two evaporation should compute");
        assert!((0.0..=0.004).contains(&es_stage_two));
        assert!(next_stage_two.counter >= stage_two.counter);

        let reset_stage = DirectEvapotranspirationStageState {
            s1_m: 0.004,
            s2_m: 0.001,
            threshold_m: 0.003,
            counter: 1.0,
        };
        let (_es_reset, next_reset) = compute_stage_soil_evaporation(reset_stage, 0.006, 0.002)
            .expect("infiltration reset branch should compute");
        assert!(next_reset.counter >= 0.0);
        assert!(next_reset.s1_m >= 0.0);
    }

    #[test]
    fn cqr_row9_surface_et_span_covers_pmet_and_staged_manual_demands() {
        let identity = DirectRunIdentity::new(904, 2637, 1, 1).expect("identity");
        let layers = vec![row9_layer(0.020, 0.10), row9_layer(0.028, 0.15)];
        let mut pmet_day = row9_day_with_percolation(identity, layers.clone());
        pmet_day.evapotranspiration_compute_inputs.pmet_compute =
            Some(row9_pmet_inputs(Some(620.0)));

        let pmet_report = pmet_day
            .run_r4n_surface_et_span()
            .expect("PMET-driven R4N surface ET should compute");
        assert_eq!(pmet_report.compatibility_edge_invocation_count, 0);
        assert!(
            pmet_day
                .evapotranspiration_surface
                .soil_evaporation_storage_return_m
                >= 0.0
        );

        let mut manual_day = row9_day_with_percolation(identity, layers);
        manual_day.evapotranspiration_compute_inputs.stage_state =
            Some(DirectEvapotranspirationStageState {
                s1_m: 0.001,
                s2_m: 0.0,
                threshold_m: 0.003,
                counter: 0.0,
            });
        manual_day
            .run_r4n_surface_et_span()
            .expect("manual staged R4N surface ET should compute");
        assert!(
            manual_day
                .evapotranspiration_surface
                .stage_state_after
                .is_some()
        );
        assert!(manual_day.evapotranspiration_surface.transpiration_demand_m > 0.0);
    }

    #[test]
    fn cqr_row9_core_validators_cover_day_lane_snow_and_frost_guards() {
        let valid_day = DirectDayConstructorInputs::zero();
        validate_direct_day_constructor_inputs(&valid_day)
            .expect("zero day constructor inputs should be valid");

        let layer = row9_layer(0.020, 0.10);
        let mut populated_day = valid_day.clone();
        populated_day.storage_input_inputs.precip_input_handoff_m = Some(0.0);
        populated_day.percolation_inputs.layers = vec![layer.clone()];
        populated_day.subsurface_compute_inputs.layers = vec![row9_layer_inputs(&layer)];
        populated_day.frost_layer_carry_projection = Some(vec![DirectFrostLayerCarryProjection {
            layer_index: 1,
            fine_layer_count: 2,
            fine_layer_thickness_m: 0.05,
        }]);
        populated_day.snow_runtime_carry = Some(Box::new(row9_snow_carry()));
        populated_day.frost_runtime_carry = Some(row9_frost_carry());
        validate_direct_day_constructor_inputs(&populated_day)
            .expect("populated day constructor inputs should validate optional branches");

        let mut invalid_projection_day = populated_day.clone();
        invalid_projection_day
            .frost_layer_carry_projection
            .as_mut()
            .expect("projection")[0]
            .fine_layer_count = 0;
        assert!(validate_direct_day_constructor_inputs(&invalid_projection_day).is_err());

        let mut invalid_day = valid_day.clone();
        invalid_day.storage_input_inputs.precip_input_handoff_m = Some(-1.0e-6);
        assert!(validate_direct_day_constructor_inputs(&invalid_day).is_err());

        let valid_lane = DirectLaneConstructorInputs::from_topology(0, 1, 1)
            .expect("single lane topology should construct");
        validate_direct_lane_constructor_inputs(
            DirectRunIdentity::new(900, 2637, 1, 1).expect("identity"),
            0,
            &valid_lane,
        )
        .expect("valid lane constructor should pass");

        let mut populated_lane = valid_lane.clone();
        populated_lane.subsurface_layers = vec![layer];
        populated_lane.evapotranspiration_stage_state =
            Some(Box::new(DirectEvapotranspirationStageState {
                s1_m: 0.001,
                s2_m: 0.0,
                threshold_m: 0.003,
                counter: 0.0,
            }));
        populated_lane.snow_runtime_carry = Some(row9_snow_carry());
        populated_lane.day_inputs = vec![populated_day];
        validate_direct_lane_constructor_inputs(
            DirectRunIdentity::new(902, 2637, 1, 1).expect("identity"),
            0,
            &populated_lane,
        )
        .expect("populated lane constructor should validate nested optional state");

        let mut invalid_day_count_lane = populated_lane.clone();
        invalid_day_count_lane
            .day_inputs
            .push(DirectDayConstructorInputs::zero());
        assert!(
            validate_direct_lane_constructor_inputs(
                DirectRunIdentity::new(903, 2637, 1, 1).expect("identity"),
                0,
                &invalid_day_count_lane,
            )
            .is_err()
        );

        let mut invalid_lane = valid_lane.clone();
        invalid_lane.lane_id = 2;
        assert!(
            validate_direct_lane_constructor_inputs(
                DirectRunIdentity::new(901, 2637, 1, 1).expect("identity"),
                0,
                &invalid_lane,
            )
            .is_err()
        );

        let mut snow = DirectSnowLaneState::zero();
        snow.runtime_swe_m = 0.03;
        snow.runtime_depth_m = 0.10;
        snow.runtime_density_kg_m3 = 300.0;
        snow.coe_boundary_depth_m = 0.10;
        snow.coe_boundary_density_kg_m3 = 300.0;
        snow.layers = vec![DirectSnowLayerState::new(0.03, 0.10, 300.0, 0.0)];
        validate_direct_snow_layers("cqr.row9.snow", &snow)
            .expect("matching snow layer aggregate should validate");

        let mut bad_snow = snow.clone();
        bad_snow.layers[0].mass_swe_m += 0.01;
        assert!(validate_direct_snow_layers("cqr.row9.snow", &bad_snow).is_err());

        let mut frost_carry = DirectFrostRuntimeCarry::from(DirectFrostLaneState::zero());
        frost_carry.fine_layers = vec![DirectFrostFineLayerCarry {
            layer_index: 1,
            fine_index: 1,
            fgfrst: 0.0,
            slfsd_m: 0.0,
            slsic_m: 0.0,
            slsw_theta: 0.0,
            sltime_s: 0.0,
        }];
        validate_direct_frost_runtime_fine_layers(&frost_carry)
            .expect("valid frost fine layer carry should validate");
        frost_carry.fine_layers[0].fine_index = 0;
        assert!(validate_direct_frost_runtime_fine_layers(&frost_carry).is_err());
    }

    #[test]
    fn cqr_row9_commit_day_prefers_latest_layer_sources_and_rejects_wrong_lane() {
        let mut lane = DirectLaneFrame::from_constructor_inputs(
            DirectLaneConstructorInputs::from_topology(0, 1, 1).expect("lane constructor"),
        );
        let identity = DirectRunIdentity::new(902, 2637, 1, 1).expect("identity");
        let mut day = DirectDayFrame::seed(identity, 0, 0).expect("day frame");
        day.storage_reconciliation.storage_reconciled_m = 0.123;
        day.evapotranspiration_compute.layer_state_after_root_uptake =
            vec![row9_layer(0.011, 0.10)];
        day.subsurface_compute.layer_state_after = vec![row9_layer(0.022, 0.10)];
        day.percolation.layer_state_after = vec![row9_layer(0.033, 0.10)];

        lane.commit_day(&day)
            .expect("commit should accept matching lane and root uptake layers");
        assert_close(lane.water.soil_water_m, 0.123);
        assert_close(lane.subsurface_layers[0].theta_m, 0.011);

        let mut subsurface_lane = DirectLaneFrame::from_constructor_inputs(
            DirectLaneConstructorInputs::from_topology(0, 1, 1).expect("lane constructor"),
        );
        let mut subsurface_day = day.clone();
        subsurface_day
            .evapotranspiration_compute
            .layer_state_after_root_uptake
            .clear();
        subsurface_lane
            .commit_day(&subsurface_day)
            .expect("commit should fall back to subsurface layers");
        assert_close(subsurface_lane.subsurface_layers[0].theta_m, 0.022);

        let mut wrong_lane_day = day;
        wrong_lane_day.lane_index = 1;
        assert!(lane.commit_day(&wrong_lane_day).is_err());
    }
}
