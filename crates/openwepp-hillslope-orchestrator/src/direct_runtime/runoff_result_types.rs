#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ),
    derive(serde::Serialize)
)]
pub struct DirectLiquidInputInputs {
    pub liquid_input_handoff_m: f64,
}

impl DirectLiquidInputInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_handoff_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLiquidInputState {
    pub liquid_input_m: f64,
}

impl DirectLiquidInputState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLiquidInputDownstreamOperands {
    pub liquid_input_m: f64,
}

impl DirectLiquidInputDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_m: 0.0,
        }
    }
}

impl From<DirectLiquidInputState> for DirectLiquidInputDownstreamOperands {
    fn from(state: DirectLiquidInputState) -> Self {
        Self {
            liquid_input_m: state.liquid_input_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLiquidInputShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub liquid_input_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ),
    derive(serde::Serialize)
)]
pub struct DirectRunonCarryInputs {
    pub surface_runon_handoff_m: f64,
    pub subsurface_carry_handoff_m: f64,
}

impl DirectRunonCarryInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            surface_runon_handoff_m: 0.0,
            subsurface_carry_handoff_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunonCarryState {
    pub runon_input_m: f64,
    pub subsurface_carry_m: f64,
}

impl DirectRunonCarryState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            runon_input_m: 0.0,
            subsurface_carry_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunonCarryDownstreamOperands {
    pub runon_input_m: f64,
    pub subsurface_carry_m: f64,
}

impl DirectRunonCarryDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            runon_input_m: 0.0,
            subsurface_carry_m: 0.0,
        }
    }
}

impl From<DirectRunonCarryState> for DirectRunonCarryDownstreamOperands {
    fn from(state: DirectRunonCarryState) -> Self {
        Self {
            runon_input_m: state.runon_input_m,
            subsurface_carry_m: state.subsurface_carry_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunonCarryShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub runon_input_m: f64,
    pub subsurface_carry_m: f64,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ),
    derive(serde::Serialize)
)]
pub struct DirectInfiltrationDepressionInputs {
    pub cumulative_infiltration_handoff_m: f64,
    pub depression_storage_delta_handoff_m: f64,
    pub producer_inputs: Option<DirectWb14InfiltrationProducerInputs>,
}

impl DirectInfiltrationDepressionInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            cumulative_infiltration_handoff_m: 0.0,
            depression_storage_delta_handoff_m: 0.0,
            producer_inputs: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectCanopyInterceptionInputs {
    pub hyetograph_rainfall_m: f64,
    pub interception_rainfall_input_m: f64,
    pub canopy_cover_fraction: f64,
    pub leaf_area_index: f64,
    pub interception_live_biomass_kg_m2: f64,
}

impl DirectCanopyInterceptionInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            hyetograph_rainfall_m: 0.0,
            interception_rainfall_input_m: 0.0,
            canopy_cover_fraction: 0.0,
            leaf_area_index: 0.0,
            interception_live_biomass_kg_m2: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectCanopyInterceptionState {
    pub interception_m: f64,
    pub liquid_after_interception_m: f64,
    pub rainfall_scale: f64,
}

impl DirectCanopyInterceptionState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            interception_m: 0.0,
            liquid_after_interception_m: 0.0,
            rainfall_scale: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ),
    derive(serde::Serialize)
)]
pub struct DirectWb14InfiltrationProducerInputs {
    pub hyetograph: Vec<DirectWb14HyetographInterval>,
    /// Producer-owned hourly liquid supplied in addition to direct rain.
    /// Routed melt is seeded by the runner before WB14; inter-OFE runon is
    /// added at R4K from the R4J-resolved totals. Both pass through the same
    /// infiltration/depression partition before any runoff timing is claimed.
    pub hourly_additional_supply_m: [f64; DC01_HOUR_BIN_COUNT],
    pub effective_conductivity_m_s: f64,
    pub matric_potential_m: f64,
    pub storage_capacity_m: f64,
    pub depression_storage_capacity_m: f64,
}

/// Closed SC-OUTPUT-WAT5-001 v5 source classification for exact accepted
/// non-rain liquid supplied to WB14 from the sealed ingress receipts.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Wat5AdditionalSupplySourceKindV1 {
    SnowTerminalReceiver,
    RoutedRunon,
    LitterPhaseOverflow,
    CondensationOverflow,
}

/// Exact accepted non-rain source custody consumed only by the diagnostic
/// WAT5 replay. Ordinary WB14 continues to consume its already-authoritative
/// hourly additional-supply owner.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Wat5AdditionalSupplySegmentV1 {
    pub source_kind: Wat5AdditionalSupplySourceKindV1,
    pub source_identity: String,
    pub source_receipt_sha256: String,
    pub transaction_id: String,
    pub destination_ofe_id: openwepp_land_surface_energy::OfeId,
    pub start_s: f64,
    pub end_s: f64,
    pub depth_m_ofe_ground: f64,
}

pub fn compute_direct_canopy_interception(
    inputs: DirectCanopyInterceptionInputs,
) -> Result<DirectCanopyInterceptionState, DirectRuntimeError> {
    validate_direct_canopy_interception_inputs(inputs)?;

    let interception_m = if inputs.canopy_cover_fraction <= WB11_ZERO_THRESHOLD
        || inputs.leaf_area_index <= WB11_ZERO_THRESHOLD
    {
        0.0
    } else {
        let biomass_kg_ha = inputs.interception_live_biomass_kg_m2 * WB15_BIOMASS_TO_KG_HA;
        validate_finite("canopy_interception.biomass_kg_ha", biomass_kg_ha)?;
        let interception_biomass_kg_ha = biomass_kg_ha.min(WB15_INTERCEPT_BIOMASS_MAX_KG_HA);
        let potential_interception_m = inputs.canopy_cover_fraction
            * ((WB15_INTERCEPT_LINEAR_COEFF * interception_biomass_kg_ha
                - WB15_INTERCEPT_QUADRATIC_COEFF * interception_biomass_kg_ha.powi(2))
                / WB15_INTERCEPT_MM_TO_M);
        validate_nonnegative_direct_m(
            "canopy_interception.potential_interception_m",
            potential_interception_m,
        )?;
        potential_interception_m.min(inputs.interception_rainfall_input_m)
    };
    validate_nonnegative_direct_m("canopy_interception.interception_m", interception_m)?;
    if interception_m > inputs.interception_rainfall_input_m + WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "canopy_interception.interception_m",
        });
    }

    let liquid_after_interception_raw = inputs.interception_rainfall_input_m - interception_m;
    validate_finite(
        "canopy_interception.liquid_after_interception_m",
        liquid_after_interception_raw,
    )?;
    if liquid_after_interception_raw < -WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "canopy_interception.liquid_after_interception_m",
        });
    }
    let liquid_after_interception_m = liquid_after_interception_raw.max(0.0);
    let rainfall_scale = if inputs.hyetograph_rainfall_m <= WB11_ZERO_THRESHOLD {
        0.0
    } else {
        liquid_after_interception_m / inputs.hyetograph_rainfall_m
    };
    validate_finite("canopy_interception.rainfall_scale", rainfall_scale)?;
    validate_nonnegative_direct_m("canopy_interception.rainfall_scale", rainfall_scale)?;

    Ok(DirectCanopyInterceptionState {
        interception_m,
        liquid_after_interception_m,
        rainfall_scale,
    })
}

fn validate_direct_canopy_interception_inputs(
    inputs: DirectCanopyInterceptionInputs,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m(
        "canopy_interception.hyetograph_rainfall_m",
        inputs.hyetograph_rainfall_m,
    )?;
    validate_nonnegative_direct_m(
        "canopy_interception.interception_rainfall_input_m",
        inputs.interception_rainfall_input_m,
    )?;
    validate_finite(
        "canopy_interception.canopy_cover_fraction",
        inputs.canopy_cover_fraction,
    )?;
    if inputs.canopy_cover_fraction < 0.0 || inputs.canopy_cover_fraction > WB15_CANCOV_MAX {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "canopy_interception.canopy_cover_fraction",
        });
    }
    validate_nonnegative_direct_m(
        "canopy_interception.leaf_area_index",
        inputs.leaf_area_index,
    )?;
    validate_nonnegative_direct_m(
        "canopy_interception.interception_live_biomass_kg_m2",
        inputs.interception_live_biomass_kg_m2,
    )?;
    if inputs.interception_rainfall_input_m > inputs.hyetograph_rainfall_m + WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "canopy_interception.interception_rainfall_input_m",
        });
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ),
    derive(serde::Serialize)
)]
pub struct DirectWb14HyetographInterval {
    pub start_s: f64,
    pub end_s: f64,
    pub intensity_m_s: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectInfiltrationDepressionState {
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
}

impl DirectInfiltrationDepressionState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            cumulative_infiltration_m: 0.0,
            depression_storage_delta_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectInfiltrationDepressionDownstreamOperands {
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
}

impl DirectInfiltrationDepressionDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            cumulative_infiltration_m: 0.0,
            depression_storage_delta_m: 0.0,
        }
    }
}

impl From<DirectInfiltrationDepressionState> for DirectInfiltrationDepressionDownstreamOperands {
    fn from(state: DirectInfiltrationDepressionState) -> Self {
        Self {
            cumulative_infiltration_m: state.cumulative_infiltration_m,
            depression_storage_delta_m: state.depression_storage_delta_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectInfiltrationDepressionShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ),
    derive(serde::Serialize)
)]
pub struct DirectSaturationAddbackInputs {
    pub surface_saturation_runoff_handoff_m: f64,
}

impl DirectSaturationAddbackInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            surface_saturation_runoff_handoff_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSaturationAddbackState {
    pub surface_saturation_runoff_m: f64,
}

impl DirectSaturationAddbackState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            surface_saturation_runoff_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSaturationAddbackDownstreamOperands {
    pub surface_saturation_runoff_m: f64,
}

impl DirectSaturationAddbackDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            surface_saturation_runoff_m: 0.0,
        }
    }
}

impl From<DirectSaturationAddbackState> for DirectSaturationAddbackDownstreamOperands {
    fn from(state: DirectSaturationAddbackState) -> Self {
        Self {
            surface_saturation_runoff_m: state.surface_saturation_runoff_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSaturationAddbackShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub surface_saturation_runoff_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ),
    derive(serde::Serialize)
)]
pub struct DirectRunoffPartitionInputs {
    pub liquid_input_m: f64,
    pub runon_input_m: f64,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
    pub surface_saturation_runoff_m: f64,
    pub frost_retained_local_liquid_m: f64,
    pub frost_preprojected_local_liquid_m: f64,
}

impl DirectRunoffPartitionInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_m: 0.0,
            runon_input_m: 0.0,
            cumulative_infiltration_m: 0.0,
            depression_storage_delta_m: 0.0,
            surface_saturation_runoff_m: 0.0,
            frost_retained_local_liquid_m: 0.0,
            frost_preprojected_local_liquid_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunoffPartitionState {
    pub liquid_input_m: f64,
    pub runon_input_m: f64,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
    pub surface_saturation_runoff_m: f64,
    pub partition_runoff_m: f64,
    pub q_runoff_m: f64,
    pub closure_residual_m: f64,
}

impl DirectRunoffPartitionState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_m: 0.0,
            runon_input_m: 0.0,
            cumulative_infiltration_m: 0.0,
            depression_storage_delta_m: 0.0,
            surface_saturation_runoff_m: 0.0,
            partition_runoff_m: 0.0,
            q_runoff_m: 0.0,
            closure_residual_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunoffDownstreamOperands {
    pub liquid_input_m: f64,
    pub runon_input_m: f64,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
    pub surface_saturation_runoff_m: f64,
    pub partition_runoff_m: f64,
    pub q_runoff_m: f64,
    pub closure_residual_m: f64,
}

impl DirectRunoffDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_m: 0.0,
            runon_input_m: 0.0,
            cumulative_infiltration_m: 0.0,
            depression_storage_delta_m: 0.0,
            surface_saturation_runoff_m: 0.0,
            partition_runoff_m: 0.0,
            q_runoff_m: 0.0,
            closure_residual_m: 0.0,
        }
    }
}

impl From<DirectRunoffPartitionState> for DirectRunoffDownstreamOperands {
    fn from(state: DirectRunoffPartitionState) -> Self {
        Self {
            liquid_input_m: state.liquid_input_m,
            runon_input_m: state.runon_input_m,
            cumulative_infiltration_m: state.cumulative_infiltration_m,
            depression_storage_delta_m: state.depression_storage_delta_m,
            surface_saturation_runoff_m: state.surface_saturation_runoff_m,
            partition_runoff_m: state.partition_runoff_m,
            q_runoff_m: state.q_runoff_m,
            closure_residual_m: state.closure_residual_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunoffShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub liquid_input_m: f64,
    pub runon_input_m: f64,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
    pub surface_saturation_runoff_m: f64,
    pub partition_runoff_m: f64,
    pub q_runoff_m: f64,
    pub closure_residual_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPeakRunoffState {
    pub q_runoff_m: f64,
    pub peak_runoff_rate_m_s: f64,
    pub runoff_duration_s: f64,
    pub peak_hour_index: Option<usize>,
    pub method_branch: f64,
    pub tstar: f64,
    pub qpstar: f64,
    pub vstar: f64,
}

impl DirectPeakRunoffState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            q_runoff_m: 0.0,
            peak_runoff_rate_m_s: 0.0,
            runoff_duration_s: 0.0,
            peak_hour_index: None,
            method_branch: 0.0,
            tstar: 0.0,
            qpstar: 0.0,
            vstar: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPeakRunoffDownstreamOperands {
    pub q_runoff_m: f64,
    pub peak_runoff_rate_m_s: f64,
    pub runoff_duration_s: f64,
    pub peak_hour_index: Option<usize>,
    pub method_branch: f64,
    pub tstar: f64,
    pub qpstar: f64,
    pub vstar: f64,
}

impl DirectPeakRunoffDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            q_runoff_m: 0.0,
            peak_runoff_rate_m_s: 0.0,
            runoff_duration_s: 0.0,
            peak_hour_index: None,
            method_branch: 0.0,
            tstar: 0.0,
            qpstar: 0.0,
            vstar: 0.0,
        }
    }
}

impl From<DirectPeakRunoffState> for DirectPeakRunoffDownstreamOperands {
    fn from(state: DirectPeakRunoffState) -> Self {
        Self {
            q_runoff_m: state.q_runoff_m,
            peak_runoff_rate_m_s: state.peak_runoff_rate_m_s,
            runoff_duration_s: state.runoff_duration_s,
            peak_hour_index: state.peak_hour_index,
            method_branch: state.method_branch,
            tstar: state.tstar,
            qpstar: state.qpstar,
            vstar: state.vstar,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPeakRunoffShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub q_runoff_m: f64,
    pub peak_runoff_rate_m_s: f64,
    pub runoff_duration_s: f64,
    pub peak_hour_index: Option<usize>,
    pub method_branch: f64,
    pub tstar: f64,
    pub qpstar: f64,
    pub vstar: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLiquidInputSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub liquid_input_shadow_projection: DirectLiquidInputShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunonCarrySpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub runon_carry_shadow_projection: DirectRunonCarryShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectInfiltrationDepressionSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub infiltration_depression_shadow_projection: DirectInfiltrationDepressionShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSaturationAddbackSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub saturation_addback_shadow_projection: DirectSaturationAddbackShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunoffPartitionSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub runoff_shadow_projection: DirectRunoffShadowProjection,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPeakRunoffSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub peak_runoff_shadow_projection: DirectPeakRunoffShadowProjection,
}
