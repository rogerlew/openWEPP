//! Explicit default-off V9/LSE consumer over the real direct scheduler owner.
//!
//! This module owns only isolated shadow state. It has no production commit,
//! selector, publication, or output API.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

#[cfg(test)]
use crate::v11_vegetation_consumer::{accept_direct_v11_segment, execute_direct_v11_segment};
use openwepp_biogeochemistry::{BiogeochemistryError, BiogeochemistryState, available_by_key};
use openwepp_coupled_time::{
    AcceptedEventReceiptV1, Digest32, FramedField, ParentTransactionId, TimeSupport,
    complete_owner_set_digest, digest_bytes, framed_sha256,
};
use openwepp_kernel_contract::{
    MineralNitrogenKey, MineralNitrogenSpecies, ResourceAmountBasis, ResourceOwnerId, SoilLayerId,
    StratumId, TileId, TransactionId, authorize_proportionally,
};
use openwepp_land_surface_energy::{
    CoveredColumnAuthority, LandSurfaceEnergyConfiguration, LandSurfaceEnergyError,
    LandSurfaceEnergyState, LandSurfaceEnergyV2State, LandSurfaceForcing, LiquidParcel,
    LiquidParcelKind, LiquidTemperatureProvider, LseSupportAdmissibilityReceiptV1, LseV2StateError,
    OfeId, ParcelId, Sha256Digest, SoilThermalLayerSnapshot, SoilThermalOfeSnapshot,
    SoilThermalSnapshot, SoilThermalTileCandidate, Stage3SnowCoveredLowerBoundary,
    build_lse_ending_state, project_v2_runtime_to_v1, project_validated_v1_runtime_to_v2,
};
use openwepp_meteorology::snow_free_forcing::{
    celsius_to_kelvin, kilopascals_to_pascals, liquid_specific_enthalpy_j_kg,
};
use openwepp_plant_phenology::{GsiParameters, GsiState};
use openwepp_unit_boundary::TemperatureCelsius;
use openwepp_vegetation::energy::{
    canopy_surface_friction_velocity, leaf_boundary_conductance, neutral_resistance,
};
use openwepp_vegetation::v11::{
    V11AdmittedResourceFlux, V11BgcDebitScope, V11CoupledOwnedState, V11ImportedV10SegmentInput,
    V11ImportedV10SegmentOutput, V11LseSupportReceiptEnvelope, V11OwnerEnvelope, V11ResourceDebit,
    V11ResourceKey, V11SharedResourceKey, V11SharedResourceKind, V11SharedResourceOwnerTransition,
    VegetationConfigurationV11, project_v11_parent_finalization_to_v10,
};
use openwepp_vegetation::{
    NitrogenArbiter, NitrogenAuthorization, NitrogenRequest, SnowFreeForcing, V8CoupledOwnedState,
    V9CoupledOwnedState, V9StateError, V10CoupledOwnedState, V10StateError,
    VegetationConfiguration, VegetationError, project_v8_runtime_to_v9, project_v9_runtime_to_v8,
    project_v9_runtime_to_v10, project_v10_runtime_to_v9,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::hydrology::{
    DirectActiveSnowPartitionInputs, DirectSnowStage3EvaluationError,
    DirectSnowStage3PersistentState, DirectSnowStage3SupportInput, DirectSnowTerminalEventResult,
    STAGE3_DEFAULT_SNOW_ALBEDO, Wb11HydrologyKernel,
};
use crate::land_surface_energy_shadow::{
    CoveredCarrierComponentState, CoveredLseIterationState, CoveredV8OwnerEnvelopeError,
    ExecuteV8LseRuntimeShadowError, LandSurfaceEnergyRealHydrologyAdapter,
    LandSurfaceEnergyShadowError, ProvisionalCoveredV8PhysicalEvaluationV1,
    UncommittedCoveredV8OwnerEnvelope, V8CanopyForcingReceipt, V8InputProjectionError,
    V10RootZoneLayerReceipt, V10RootZoneReceiptKey, V10RootZoneReceiptSet,
    execute_v8_lse_runtime_shadow_internal,
    execute_v8_lse_runtime_shadow_v11_physical_with_carriers,
    execute_v8_lse_runtime_shadow_v11_with_carriers, unified_beginning_hydrology_snapshot_sha256,
};
use crate::runtime_inputs::{
    DirectGsiDailyReceiptV1, DirectGsiOwnerConfigurationV1, PreparedSnowFreeGsiDayV1,
    SnowFreeHalfHourDestination, SnowFreeHalfHourForcingError, SnowFreeHalfHourIntervalReceipt,
    SnowFreeHalfHourProviderConfiguration, SnowFreeHalfHourProviderCursor,
    SnowFreeHalfHourStaticConfiguration, SnowFreePrecipitationParcelReceipt,
    ValidatedSnowFreeHalfHourForcingReceipts,
};
use crate::snow_stage3_open_boundary::{
    FinalStage3OpenSnowBoundaryReceiptV1, FinalStage3TileBoundaryReceiptV1,
    OpenSnowTileBoundaryCandidateV1, SealedStage3TileBoundaryForcingV1,
    evaluate_open_snow_tile_boundary,
};
use crate::snow_stage3_terminal_handoff::{
    CanopyLongwaveComponent, CarrierSurface, FinalStage3CanopyBoundaryReceiptInputs,
    FinalStage3CanopyBoundaryReceiptV1, LaneBoundaryContributionV1,
    LaneBoundaryTopologyExpectationV1, LaneStage3BoundaryReceiptV1,
    STAGE3_OFE_TILE_FRACTION_CLOSURE_TOLERANCE, SealedCoveredCarrierForcing,
    SnowStage3HandoffError, Stage3BoundaryIdentity, Stage3LaneAreaBasisV1,
    Stage3SnowSurfaceBoundaryReceiptInputs, Stage3SnowSurfaceBoundaryReceiptV1,
    Stage3TileBoundaryClassV1, outward_snow_fluxes_to_stage3,
};
use crate::snow_stage3_v11_attachment::{
    DirectSnowStage3V11AttachmentError, DirectSnowStage3V11TerminalParcel,
    DirectSnowStage3V11TerminalParcelPosture, SnowSoilHeatReceiptV1,
    Stage3PrecipitationDestinationV1, Stage3PrecipitationEnthalpyProviderV1,
    Stage3PrecipitationPhaseParcelSetV1, Stage3PrecipitationPhaseParcelV1,
    Stage3PrecipitationPhaseV1, Stage3PrecipitationSourceV1,
    reconstruct_precipitation_mass_and_advected_heat,
};
use crate::vegetation_real_hydrology_shadow::{
    RealHydrologyLaneLayerMap, RealHydrologyShadowAdapter, RealHydrologyShadowError,
};
use crate::{
    DirectDayFrame, DirectOfeWb14Parameters, DirectPublicationDayInput, DirectRunFrame,
    DirectSurfaceLiquidConfiguration,
};

#[path = "canonical_owner_bytes.rs"]
mod canonical_owner_bytes;
pub use canonical_owner_bytes::canonical_soil_thermal_v2_bundle_bytes;
pub(crate) use canonical_owner_bytes::{
    AdaptiveCompleteOwnerComparisonV1, AdaptiveDiscreteSurfaceKindV1, AdaptiveOwnerDimensionV1,
    adaptive_scalar_policy,
};
#[path = "v11_covered/mod.rs"]
mod v11_covered;
#[cfg(test)]
pub(crate) use v11_covered::audit_covered_carrier_support;
pub(crate) use v11_covered::physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1;
pub(crate) use v11_covered::physical_outcome_ledger::ledger_set_digest as stage3_physical_outcome_ledger_set_digest;
pub use v11_covered::physical_outcome_ledger::{
    Stage3PhysicalOutcomeClosureAuditV1, begin_stage3_physical_outcome_closure_audit_v1,
    take_stage3_physical_outcome_closure_audit_v1,
};
#[cfg(test)]
pub(crate) use v11_covered::{
    CoveredProvisionalPhysicalAuditV1, begin_covered_provisional_physical_audit_v1,
    force_covered_full_provisional_envelope_for_test, take_covered_provisional_physical_audit_v1,
};

pub(crate) use v11_covered::CoveredCarrierEphemeralCandidatesV1;
pub(crate) use v11_covered::CoveredCarrierPhaseResultV1;
pub(crate) use v11_covered::CoveredPhysicalCustodyJoinInputs;
pub(crate) use v11_covered::stage3_support_forcing_digest;
use v11_covered::*;
pub use v11_covered::{
    CoveredParentOwnerJoinReceiptV1, DirectV11RealConsumerStack,
    DirectV11SnowCoveredRealConsumerStack, DirectV11SnowCoveredStackInputs,
};
pub(crate) use v11_covered::{
    PrecomputedTerminalAcceptedEndpointV1, precomputed_terminal_pre_event_authority_sha256_v1,
};

const INTERVALS_PER_DAY: usize = 48;
const INTERVAL_S: f64 = 1_800.0;
include!("v9_real_consumer_shadow_v11_error.rs");
include!("v9_real_consumer_shadow_physical_custody_error.rs");
include!("v9_real_consumer_shadow_serialization.rs");

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectV9ShadowIntervalInput {
    pub lse_forcing: LandSurfaceForcing,
    pub vegetation_forcing: SnowFreeForcing,
    pub wb14_parameters: Vec<DirectOfeWb14Parameters>,
}

/// Sealed atmospheric and receiver projection for a snow-covered V11
/// segment. This is deliberately distinct from [`DirectV9ShadowIntervalInput`]:
/// the covered adopter owns the snow/carrier boundary and may not silently
/// enter the snow-free interval path.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectV11SnowCoveredSegmentInput {
    pub(crate) lse_forcing: LandSurfaceForcing,
    pub(crate) vegetation_forcing: SnowFreeForcing,
    pub(crate) wb14_parameters: Vec<DirectOfeWb14Parameters>,
}

impl DirectV11SnowCoveredSegmentInput {
    pub fn try_new(
        lse_forcing: LandSurfaceForcing,
        vegetation_forcing: SnowFreeForcing,
        wb14_parameters: Vec<DirectOfeWb14Parameters>,
    ) -> Result<Self, DirectV11RealConsumerError> {
        // A support that begins snow-free and receives authorized solid
        // precipitation is a covered reappearance support. Its lower
        // boundary is constructed from the sealed precipitation custody, so
        // only the ending covered posture is mandatory here.
        if !lse_forcing.snow_present_at_end || lse_forcing.snow_terminal_payload_present {
            return Err(DirectV11RealConsumerError::Identity(
                "covered input requires persistent snow operands",
            ));
        }
        Ok(Self {
            lse_forcing,
            vegetation_forcing,
            wb14_parameters,
        })
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn from_snow_free(input: &DirectV9ShadowIntervalInput) -> Self {
        Self {
            lse_forcing: input.lse_forcing.clone(),
            vegetation_forcing: input.vegetation_forcing.clone(),
            wb14_parameters: input.wb14_parameters.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectV9ShadowDayInput {
    pub day_index: usize,
    pub intervals: Vec<DirectV9ShadowIntervalInput>,
    precipitation_custody: Option<DirectV9PrecipitationCustody>,
}

impl DirectV9ShadowDayInput {
    /// Construct a caller template. Repository precipitation custody remains
    /// absent until a sealed provider projection is applied.
    #[allow(clippy::too_many_lines)]
    pub fn try_new(
        day_index: usize,
        intervals: Vec<DirectV9ShadowIntervalInput>,
    ) -> Result<Self, DirectV9RealConsumerError> {
        if intervals.len() != INTERVALS_PER_DAY {
            return Err(DirectV9RealConsumerError::Unsupported(
                "a shadow day requires exactly 48 intervals",
            ));
        }
        Ok(Self {
            day_index,
            intervals,
            precipitation_custody: None,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct DirectV9PrecipitationCustody {
    current_source_and_outgoing_mass: BTreeMap<(String, String), (String, f64)>,
}

/// Replace atmospheric and precipitation template operands with a sealed
/// repository-provider receipt. The consumer subsequently reprojects every
/// nonmeteorological operand from its live owners and treats remaining
/// template values only as checked expectations, never as physics authority.
fn project_repository_forcing_receipts_to_v9_day(
    provider: &ValidatedSnowFreeHalfHourForcingReceipts,
    mut template: DirectV9ShadowDayInput,
    expected_run_id: u64,
    expected_gsi_receipt_sha256: &str,
    expected_destinations: &BTreeSet<(String, String)>,
) -> Result<DirectV9ShadowDayInput, DirectV9RealConsumerError> {
    let receipts = provider.receipts();
    let first = receipts.first().ok_or(DirectV9RealConsumerError::Identity(
        "repository forcing receipt set",
    ))?;
    if first.day_index != template.day_index
        || first.run_id != expected_run_id.to_string()
        || template.intervals.len() != INTERVALS_PER_DAY
        || receipts.iter().any(|receipt| {
            receipt.day_index != template.day_index || receipt.intervals.len() != INTERVALS_PER_DAY
        })
    {
        return Err(DirectV9RealConsumerError::Identity(
            "repository forcing day projection",
        ));
    }
    let found_destinations = receipts
        .iter()
        .map(|receipt| {
            (
                receipt.intervals[0].ofe_id.clone(),
                receipt.intervals[0].tile_id.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    if &found_destinations != expected_destinations {
        return Err(DirectV9RealConsumerError::Identity(
            "repository forcing destination topology",
        ));
    }
    let mut current_source_and_outgoing_mass = BTreeMap::new();
    for receipt in receipts {
        let identity = (
            receipt.intervals[0].ofe_id.clone(),
            receipt.intervals[0].tile_id.clone(),
        );
        let outgoing_mass = receipt
            .next_day_precipitation_carry
            .iter()
            .map(|parcel| parcel.mass_kg_m2)
            .sum();
        current_source_and_outgoing_mass.insert(
            identity,
            (receipt.source_climate_sha256.clone(), outgoing_mass),
        );
    }
    for interval_index in 0..INTERVALS_PER_DAY {
        let atmospheric = &first.intervals[interval_index];
        if atmospheric.gsi_receipt_sha256 != expected_gsi_receipt_sha256 {
            return Err(DirectV9RealConsumerError::Identity(
                "repository GSI owner receipt",
            ));
        }
        let live = &template.intervals[interval_index].vegetation_forcing;
        if atmospheric.co2_pa.to_bits() != live.co2_pa.to_bits()
            || atmospheric.reference_height_m.to_bits() != live.reference_height_m.to_bits()
            || atmospheric.gsi.to_bits() != live.gsi.to_bits()
        {
            return Err(DirectV9RealConsumerError::Identity(
                "repository forcing live-owner scalar join",
            ));
        }
        validate_wb14_provider_bindings(
            receipts,
            interval_index,
            &template.intervals[interval_index],
        )?;
        validate_global_provider_interval(receipts, interval_index, atmospheric)?;
        let interval = &mut template.intervals[interval_index];
        project_lse_atmosphere(
            receipts,
            interval_index,
            atmospheric,
            &mut interval.lse_forcing,
        )?;
        project_vegetation_atmosphere(atmospheric, &mut interval.vegetation_forcing);
    }
    template.precipitation_custody = Some(DirectV9PrecipitationCustody {
        current_source_and_outgoing_mass,
    });
    Ok(template)
}

include!("v9_real_consumer_shadow_forcing.rs");
include!("v9_real_consumer_shadow/v10_soil_thermal_v2.rs");
#[derive(Clone, Debug, PartialEq)]
pub struct DirectV9RealConsumerShadow {
    authority: CoveredColumnAuthority,
    provider_gsi_receipt_sha256: String,
    vegetation_configuration: VegetationConfiguration,
    vegetation_state: V9CoupledOwnedState,
    vegetation_owner_id: ResourceOwnerId,
    lse_configuration: LandSurfaceEnergyConfiguration,
    lse_state: LandSurfaceEnergyState,
    surface_configuration: DirectSurfaceLiquidConfiguration,
    layer_maps: Vec<RealHydrologyLaneLayerMap>,
    soil_thermal: DirectSoilThermalResident,
    biogeochemistry: BiogeochemistryState,
    hydrology_frame: DirectRunFrame,
    next_day_index: usize,
    accepted_interval_count: u64,
    wb14_parent_working_state: Option<crate::direct_runtime::DirectWb14ParentWorkingState>,
    root_zone_hydraulic_configuration: Option<DirectRootZoneHydraulicConfiguration>,
}

/// Complete typed restart owner for the default-off V9 real-consumer shadow.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectV9RealConsumerCheckpoint {
    shadow: DirectV9RealConsumerShadow,
}

enum CanopySoilEvaluationV1 {
    Complete(UncommittedCoveredV8OwnerEnvelope),
    PhysicalOnly(ProvisionalCoveredV8PhysicalEvaluationV1),
}

struct PreparedCoveredCanopySoilInputV1(DirectV9ShadowIntervalInput);

impl CanopySoilEvaluationV1 {
    fn into_complete(self) -> Result<UncommittedCoveredV8OwnerEnvelope, DirectV9RealConsumerError> {
        match self {
            Self::Complete(value) => Ok(value),
            Self::PhysicalOnly(_) => Err(DirectV9RealConsumerError::Identity(
                "complete canopy/soil construction returned physical-only evaluation",
            )),
        }
    }

    fn into_physical(
        self,
    ) -> Result<ProvisionalCoveredV8PhysicalEvaluationV1, DirectV9RealConsumerError> {
        match self {
            Self::PhysicalOnly(value) => Ok(value),
            Self::Complete(_) => Err(DirectV9RealConsumerError::Identity(
                "physical-only canopy/soil construction returned complete evaluation",
            )),
        }
    }
}

/// Explicit default-off V10/LSE-V2 owner. It retains successor identities at
/// the public boundary and uses transient V9/V1 projections only to reuse the
/// unchanged positive-PAR and owner plumbing.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectV10RealConsumerShadow {
    inner: DirectV9RealConsumerShadow,
    vegetation_configuration: VegetationConfiguration,
    vegetation_state: V10CoupledOwnedState,
    lse_configuration: LandSurfaceEnergyConfiguration,
    lse_state: LandSurfaceEnergyV2State,
    gsi_owner_configuration: DirectGsiOwnerConfigurationV1,
    gsi_state: GsiState,
    provider_static_configuration: SnowFreeHalfHourStaticConfiguration,
    provider_cursor: SnowFreeHalfHourProviderCursor,
    root_zone_hydraulic_configuration: DirectRootZoneHydraulicConfiguration,
    /// Unpublished accepted support operands retained verbatim for the
    /// complete-day publication candidate. Trial shadows own independent
    /// clones, so rejected adaptive paths discard these with their owner set.
    /// Accepted zero-duration events remain distinct from the positive support
    /// receipts they follow. They are the only authority that may bridge a
    /// pre-event publication owner set to a post-event receiver support.
    accepted_publication_history: AcceptedPublicationHistoryV1,
}

include!("v9_real_consumer_shadow_equilibrium_fixture.rs");

include!("v9_real_consumer_shadow_publication_retention.rs");
include!("v9_real_consumer_shadow_root_zone_configuration.rs");

pub type DirectV10ShadowDayInput = DirectV9ShadowDayInput;
pub type DirectV10ShadowDayReceipt = DirectV9ShadowDayReceipt;

#[derive(Debug, Error, PartialEq)]
pub enum DirectV10RealConsumerError {
    #[error("root-zone configuration identity failure: {0}")]
    RootConfigurationIdentity(&'static str),
    #[error("root-zone scalar domain failure: {0}")]
    RootDomain(&'static str),
    #[error(transparent)]
    V10(#[from] V10StateError),
    #[error(transparent)]
    LseV2(#[from] LseV2StateError),
    #[error(transparent)]
    LandSurface(#[from] LandSurfaceEnergyError),
    #[error(transparent)]
    ForcingProvider(#[from] SnowFreeHalfHourForcingError),
    #[error(transparent)]
    Runtime(#[from] DirectV9RealConsumerError),
}

impl DirectV10RealConsumerError {
    #[must_use]
    pub const fn category(&self) -> &'static str {
        match self {
            Self::RootConfigurationIdentity(_) => "root_configuration_identity",
            Self::RootDomain(_) => "root_domain",
            Self::V10(_) => "vegetation_v10",
            Self::LseV2(_) => "land_surface_energy_v2",
            Self::LandSurface(_) => "land_surface_energy",
            Self::ForcingProvider(_) => "forcing_provider",
            Self::Runtime(error) => error.category(),
        }
    }
}

fn v11_owner_envelope<T: Serialize>(
    owner_id: &str,
    value: &T,
) -> Result<V11OwnerEnvelope, DirectV11RealConsumerError> {
    Ok(V11OwnerEnvelope::try_new(
        owner_id.to_owned(),
        serde_json::to_vec(value)?,
    )?)
}

impl crate::v11_vegetation_consumer::DirectV11ImportedStack for DirectV11RealConsumerStack<'_> {
    type Error = DirectV11RealConsumerError;

    #[allow(clippy::too_many_lines)]
    fn execute_imported_v10_stack(
        &mut self,
        input: &V11ImportedV10SegmentInput,
    ) -> Result<V11ImportedV10SegmentOutput, Self::Error> {
        if input.configuration != self.beginning.vegetation_configuration
            || input.beginning != self.beginning.vegetation_state
            || input.duration_s_bits != input.support.duration_s_bits()
            || self.interval.lse_forcing.interval_s.to_bits() != input.duration_s_bits
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted slab / DirectV10 beginning join",
            ));
        }
        let _interval_index = u8::try_from(self.interval_index)
            .map_err(|_| DirectV11RealConsumerError::Identity("V11 interval index overflow"))?;
        let support_receipt = LseSupportAdmissibilityReceiptV1::admit(
            &self.beginning.inner.lse_configuration,
            &self.beginning.inner.lse_state,
            digest32_hex(input.parent_transaction_id.digest()),
            digest32_hex(input.accepted_slab_receipt.segment_id().digest()),
            digest32_hex(input.accepted_slab_receipt.slab_id().digest()),
            input.accepted_slab_receipt.slab_ordinal(),
            input.support.start_ns().get(),
            input.support.end_ns().get(),
            input.duration_s_bits,
            self.beginning.inner.soil_thermal.state_sha256().clone(),
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(error))
        })?;
        let mut candidate = self.beginning.clone();
        // This imported stack is the explicit snow-free V11 successor. A
        // preceding covered/terminal segment leaves the retained carrier
        // authority as `V11SnowCovered`; carrying that authority into this
        // segment would demand a Stage-3 lower-boundary set after the snow
        // owner has disappeared. Install the imported V10 authority on the
        // unpublished candidate before projection. V10, unlike historical
        // V8, owns the exact inactive hydraulic anchors required by a
        // structurally zero occupancy while retaining the snow-free boundary.
        install_imported_v10_snow_free_authority(&mut candidate);
        let envelope = candidate
            .inner
            .construct_snow_free_parent_child_envelope_with_duration(
                self.day_index,
                self.interval_index,
                self.interval,
                f64::from_bits(input.duration_s_bits),
                Some(input.duration_s_bits),
                self.finalize_wb14_parent_interval,
                self.wb14_coupled_child_binding,
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
            })?;
        envelope.validate().map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
        })?;

        let mut resource_debits = v11_nitrogen_resource_debits(
            &envelope,
            &self.beginning.inner.lse_configuration,
            input,
        )?;
        resource_debits.extend(v11_water_resource_debits(
            &envelope,
            &input.configuration,
            input,
        )?);

        candidate
            .inner
            .accept_envelope(envelope.transaction_id(), &envelope)
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
            })?;
        candidate.vegetation_state = project_v9_runtime_to_v10(
            candidate.inner.vegetation_state(),
            &candidate.vegetation_configuration,
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::V10(error))
        })?;
        candidate.lse_state = project_validated_v1_runtime_to_v2(
            &candidate.inner.lse_configuration,
            candidate.inner.lse_state(),
            &candidate.lse_configuration,
            &openwepp_land_surface_energy::Sha256Digest::try_new(
                candidate
                    .vegetation_configuration
                    .configuration_sha256
                    .clone(),
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(error))
            })?,
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LseV2(error))
        })?;

        let segment_ending = candidate.vegetation_state.clone();
        normalize_v11_staged_parent_lineage(&mut candidate, input.beginning.0.last_transaction_id)?;

        let snow = self
            .ending_snow_owner_bytes
            .as_ref()
            .map(|bytes| V11OwnerEnvelope::try_new("snow".to_owned(), bytes.clone()))
            .transpose()?
            .or_else(|| input.staged_resource_owners.get("snow").cloned())
            .ok_or(DirectV11RealConsumerError::Identity(
                "missing staged snow owner",
            ))?;
        let surface = candidate
            .inner
            .hydrology_frame
            .surface_liquid_shadow
            .as_ref()
            .ok_or(DirectV11RealConsumerError::Identity(
                "missing staged surface-liquid owner",
            ))?;
        let surface_bytes = surface
            .canonical_bytes(&candidate.inner.surface_configuration)
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                    DirectV9RealConsumerError::Serialization(error.to_string()),
                ))
            })?;
        let beginning_hydrology_adapter = RealHydrologyShadowAdapter::try_from_day_start(
            &self.beginning.inner.hydrology_frame,
            self.day_index,
            TransactionId(input.beginning.0.last_transaction_id),
            f64::from_bits(input.duration_s_bits),
            candidate.inner.surface_configuration.owner_id.clone(),
            &candidate.inner.layer_maps,
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
        })?;
        let hydrology_adapter = RealHydrologyShadowAdapter::try_from_day_start(
            envelope.hydrology().ending_frame(),
            self.day_index,
            TransactionId(input.beginning.0.last_transaction_id),
            f64::from_bits(input.duration_s_bits),
            candidate.inner.surface_configuration.owner_id.clone(),
            &candidate.inner.layer_maps,
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
        })?;
        let ending_resource_owners = [
            ("snow".to_owned(), snow),
            (
                "land_surface_energy".to_owned(),
                // The support receipt and the actual LSE solver are bound to
                // the immutable V1 physical owner.  Keep the staged owner on
                // that same canonical payload; the surrounding V2 wrapper is
                // an identity projection, not a second physical owner.
                v11_owner_envelope("land_surface_energy", &candidate.inner.lse_state)?,
            ),
            (
                "surface_liquid".to_owned(),
                V11OwnerEnvelope::try_new("surface_liquid".to_owned(), surface_bytes)?,
            ),
            (
                "hydrology".to_owned(),
                V11OwnerEnvelope::try_new(
                    "hydrology".to_owned(),
                    hydrology_adapter.snapshot_bytes().to_vec(),
                )?,
            ),
            (
                "bgc".to_owned(),
                v11_owner_envelope("bgc", &candidate.inner.biogeochemistry)?,
            ),
            (
                "soil_thermal".to_owned(),
                v11_owner_envelope("soil_thermal", &candidate.inner.soil_thermal)?,
            ),
        ]
        .into_iter()
        .collect();

        let shared_resource_transitions = v11_shared_resource_transitions(
            &envelope,
            input,
            &resource_debits,
            &ending_resource_owners,
            &beginning_hydrology_adapter,
            &hydrology_adapter,
            &self.beginning.inner.biogeochemistry,
            &candidate.inner.biogeochemistry,
            false,
        )?;
        let mut accepted_ending_owners = ending_resource_owners.clone();
        accepted_ending_owners.insert(
            "vegetation".to_owned(),
            accepted_v11_vegetation_owner(input, &segment_ending)?,
        );
        let ending_owner_states = accepted_ending_owners
            .values()
            .map(V11OwnerEnvelope::to_owner_state)
            .collect::<Result<Vec<_>, _>>()?;
        let ending_complete_owner_set_sha256 = complete_owner_set_digest(&ending_owner_states)
            .map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "accepted publication ending complete-owner set",
                )
            })?;
        candidate.retain_accepted_publication_support(
            self.day_index,
            self.interval_index,
            input,
            ending_complete_owner_set_sha256,
            support_receipt.clone(),
            self.interval.lse_forcing.clone(),
            self.interval.vegetation_forcing.clone(),
            self.interval.wb14_parameters.clone(),
            resource_debits.clone(),
            envelope.vegetation().material_proposals().to_vec(),
            envelope.hydrology(),
            None,
        )?;

        let output = V11ImportedV10SegmentOutput {
            ending: segment_ending,
            lse_support_receipt: V11LseSupportReceiptEnvelope::from_canonical_json(
                serde_json::to_vec(&support_receipt).map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                        DirectV9RealConsumerError::Serialization(error.to_string()),
                    ))
                })?,
            )
            .map_err(|_| DirectV11RealConsumerError::Identity("V11 LSE support receipt"))?,
            resource_debits,
            admitted_resource_fluxes: Vec::<V11AdmittedResourceFlux>::new(),
            shared_resource_transitions,
            ending_resource_owners,
            material_transfers: envelope.vegetation().material_proposals().to_vec(),
        };
        self.last_support_receipt = Some(support_receipt);
        #[cfg(test)]
        {
            self.last_hydrology_candidate = Some(envelope.hydrology().clone());
        }
        self.ending = Some(candidate);
        Ok(output)
    }
}

fn install_imported_v10_snow_free_authority(candidate: &mut DirectV10RealConsumerShadow) {
    candidate.inner.authority = CoveredColumnAuthority::V10NonpositiveAssimilation;
}

include!("v9_real_consumer_shadow_v10_accessors.rs");

fn publication_owner_digest_after_event_handoffs(
    previous: &Stage3AcceptedPublicationSupportV1,
    events: &[AcceptedEventReceiptV1],
) -> Result<Digest32, DirectV11RealConsumerError> {
    publication_owner_chain_after_event_handoffs(previous, events).map(|chain| {
        chain
            .last()
            .copied()
            .unwrap_or(previous.ending_complete_owner_set_sha256)
    })
}

fn publication_owner_chain_after_event_handoffs(
    previous: &Stage3AcceptedPublicationSupportV1,
    events: &[AcceptedEventReceiptV1],
) -> Result<Vec<Digest32>, DirectV11RealConsumerError> {
    let mut owner_chain = vec![previous.ending_complete_owner_set_sha256];
    for (event_index, event) in events
        .iter()
        .enumerate()
        .filter(|(_, event)| event.tick() == previous.support.end_ns())
    {
        let prior_same_parent = events[..event_index]
            .iter()
            .rev()
            .find(|prior| prior.parent_transaction_id() == event.parent_transaction_id());
        event.validate().map_err(|_| {
            DirectV11RealConsumerError::Identity("accepted publication event handoff seal")
        })?;
        if event.beginning_owner_set_digest()
            != owner_chain
                .last()
                .copied()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "accepted publication event handoff chain",
                ))?
            || prior_same_parent.map_or(event.ordinal() != 0, |prior| {
                prior.ordinal().checked_add(1) != Some(event.ordinal())
            })
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication event handoff chronology",
            ));
        }
        owner_chain.push(event.ending_owner_set_digest());
    }
    Ok(owner_chain)
}

impl DirectV10RealConsumerShadow {
    pub(crate) fn accept_v11_parent_finalization(
        &mut self,
        configuration: &VegetationConfigurationV11,
        finalized: &V11CoupledOwnedState,
    ) -> Result<V11OwnerEnvelope, DirectV11RealConsumerError> {
        let (v10_configuration, v10_state) = project_v11_parent_finalization_to_v10(
            configuration,
            &self.vegetation_state,
            finalized,
        )?;
        if v10_configuration != self.vegetation_configuration {
            return Err(DirectV11RealConsumerError::Identity(
                "V11 parent finalization V10 configuration",
            ));
        }
        let (v9_configuration, v9_state) =
            project_v10_runtime_to_v9(&v10_configuration, &v10_state).map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::V10(error))
            })?;
        if v9_configuration != self.inner.vegetation_configuration {
            return Err(DirectV11RealConsumerError::Identity(
                "V11 parent finalization V9 configuration",
            ));
        }
        let prior_transaction = self.vegetation_state.0.last_transaction_id;
        if finalized.last_parent_transaction_id
            != prior_transaction
                .checked_add(1)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "V11 parent finalization transaction overflow",
                ))?
            || !matches!(
                self.inner.biogeochemistry.last_transaction_id,
                value if value == prior_transaction || value == finalized.last_parent_transaction_id
            )
        {
            return Err(DirectV11RealConsumerError::Identity(
                "V11 parent finalization BGC predecessor",
            ));
        }
        let mut finalized_bgc = self.inner.biogeochemistry.clone();
        finalized_bgc.last_transaction_id = finalized.last_parent_transaction_id;
        let bgc_owner = v11_owner_envelope("bgc", &finalized_bgc)?;
        self.vegetation_state = v10_state;
        self.inner.vegetation_state = v9_state;
        self.inner.biogeochemistry = finalized_bgc;
        Ok(bgc_owner)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn retain_accepted_publication_support(
        &mut self,
        day_index: usize,
        interval_index: usize,
        input: &V11ImportedV10SegmentInput,
        ending_complete_owner_set_sha256: Digest32,
        lse_support_receipt: LseSupportAdmissibilityReceiptV1,
        lse_forcing: LandSurfaceForcing,
        vegetation_forcing: SnowFreeForcing,
        wb14_parameters: Vec<DirectOfeWb14Parameters>,
        resource_debits: Vec<V11ResourceDebit>,
        material_transfers: Vec<openwepp_vegetation::carbon_nitrogen::MaterialTransfer>,
        hydrology: &crate::land_surface_energy_shadow::UnifiedRealHydrologyCandidate,
        physical_outcome_ledgers: Option<
            &BTreeMap<u32, v11_covered::physical_outcome_ledger::Stage3LanePhysicalOutcomeLedgerV1>,
        >,
    ) -> Result<(), DirectV11RealConsumerError> {
        let accepted = Stage3AcceptedPublicationSupportV1::try_new(
            day_index,
            interval_index,
            input,
            ending_complete_owner_set_sha256,
            lse_support_receipt,
            lse_forcing,
            vegetation_forcing,
            wb14_parameters,
            resource_debits,
            material_transfers,
            hydrology,
            physical_outcome_ledgers,
        )?;
        self.accepted_publication_history.push_support(accepted)
    }

    pub(crate) fn accepted_publication_supports_for_day(
        &self,
        day_index: usize,
    ) -> Result<Vec<&Stage3AcceptedPublicationSupportV1>, DirectV11RealConsumerError> {
        let supports = self
            .accepted_publication_history
            .supports()
            .iter()
            .filter(|support| support.day_index == day_index)
            .map(std::sync::Arc::as_ref)
            .collect::<Vec<_>>();
        if supports.is_empty()
            || supports.windows(2).any(|pair| {
                pair[0].support.end_ns() != pair[1].support.start_ns()
                    || publication_owner_digest_after_event_handoffs(
                        pair[0],
                        self.accepted_publication_history.event_handoffs(),
                    )
                    .is_err()
                    || publication_owner_digest_after_event_handoffs(
                        pair[0],
                        self.accepted_publication_history.event_handoffs(),
                    )
                    .is_ok_and(|ending| ending != pair[1].beginning_complete_owner_set_sha256)
            })
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication day support set",
            ));
        }
        Ok(supports)
    }

    pub(crate) fn accepted_publication_ordered_owner_chain(
        &self,
        support: &Stage3AcceptedPublicationSupportV1,
    ) -> Result<Vec<Digest32>, DirectV11RealConsumerError> {
        publication_owner_chain_after_event_handoffs(
            support,
            self.accepted_publication_history.event_handoffs(),
        )
    }

    #[must_use]
    #[cfg(test)]
    pub(crate) fn accepted_publication_supports(&self) -> Vec<Stage3AcceptedPublicationSupportV1> {
        self.accepted_publication_history
            .supports()
            .iter()
            .map(|support| (**support).clone())
            .collect()
    }

    #[must_use]
    pub(crate) fn accepted_publication_event_handoffs(&self) -> &[AcceptedEventReceiptV1] {
        self.accepted_publication_history.event_handoffs()
    }

    #[cfg(test)]
    pub(crate) fn restore_accepted_publication_supports(
        &mut self,
        supports: Vec<Stage3AcceptedPublicationSupportV1>,
    ) -> Result<(), DirectV11RealConsumerError> {
        self.restore_accepted_publication_authority(supports, Vec::new())?;
        Ok(())
    }

    #[cfg(any(
        test,
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    fn restore_accepted_publication_authority(
        &mut self,
        supports: Vec<Stage3AcceptedPublicationSupportV1>,
        event_handoffs: Vec<AcceptedEventReceiptV1>,
    ) -> Result<(), DirectV11RealConsumerError> {
        let supports = supports
            .into_iter()
            .map(std::sync::Arc::new)
            .collect::<Vec<_>>();
        validate_accepted_publication_authority(&supports, &event_handoffs)?;
        self.accepted_publication_history.replace(
            supports
                .into_iter()
                .map(|support| (*support).clone())
                .collect(),
            &event_handoffs,
        )
    }

    pub(crate) fn next_transaction_id(&self) -> Result<TransactionId, DirectV11RealConsumerError> {
        self.vegetation_state
            .0
            .last_transaction_id
            .checked_add(1)
            .map(TransactionId)
            .ok_or(DirectV11RealConsumerError::Identity(
                "V11 live transaction overflow",
            ))
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub fn try_new(
        vegetation_configuration: VegetationConfiguration,
        vegetation_state: V10CoupledOwnedState,
        vegetation_owner_id: ResourceOwnerId,
        lse_configuration: LandSurfaceEnergyConfiguration,
        lse_state: LandSurfaceEnergyV2State,
        surface_configuration: DirectSurfaceLiquidConfiguration,
        layer_maps: Vec<RealHydrologyLaneLayerMap>,
        soil_thermal: SoilThermalSnapshot,
        biogeochemistry: BiogeochemistryState,
        hydrology_frame: DirectRunFrame,
        next_day_index: usize,
        gsi_owner_configuration: DirectGsiOwnerConfigurationV1,
        gsi_state: GsiState,
        provider_static_configuration: SnowFreeHalfHourStaticConfiguration,
        provider_cursor: SnowFreeHalfHourProviderCursor,
        root_zone_hydraulic_configuration: DirectRootZoneHydraulicConfiguration,
    ) -> Result<Self, DirectV10RealConsumerError> {
        Self::try_new_with_soil_resident(
            vegetation_configuration,
            vegetation_state,
            vegetation_owner_id,
            lse_configuration,
            lse_state,
            surface_configuration,
            layer_maps,
            DirectSoilThermalResident::try_new_v1(soil_thermal)?,
            biogeochemistry,
            hydrology_frame,
            next_day_index,
            gsi_owner_configuration,
            gsi_state,
            provider_static_configuration,
            provider_cursor,
            root_zone_hydraulic_configuration,
        )
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub fn try_new_v2(
        vegetation_configuration: VegetationConfiguration,
        vegetation_state: V10CoupledOwnedState,
        vegetation_owner_id: ResourceOwnerId,
        lse_configuration: LandSurfaceEnergyConfiguration,
        lse_state: LandSurfaceEnergyV2State,
        surface_configuration: DirectSurfaceLiquidConfiguration,
        layer_maps: Vec<RealHydrologyLaneLayerMap>,
        prepared_soil_thermal: openwepp_land_surface_energy::PreparedSoilThermalSupportV2,
        soil_thermal_seals: openwepp_land_surface_energy::SoilThermalReceiptFreeOwnerSealsV2,
        biogeochemistry: BiogeochemistryState,
        hydrology_frame: DirectRunFrame,
        next_day_index: usize,
        gsi_owner_configuration: DirectGsiOwnerConfigurationV1,
        gsi_state: GsiState,
        provider_static_configuration: SnowFreeHalfHourStaticConfiguration,
        provider_cursor: SnowFreeHalfHourProviderCursor,
        root_zone_hydraulic_configuration: DirectRootZoneHydraulicConfiguration,
    ) -> Result<Self, DirectV10RealConsumerError> {
        Self::try_new_with_soil_resident(
            vegetation_configuration,
            vegetation_state,
            vegetation_owner_id,
            lse_configuration,
            lse_state,
            surface_configuration,
            layer_maps,
            DirectSoilThermalResident::try_new_v2(prepared_soil_thermal, soil_thermal_seals)?,
            biogeochemistry,
            hydrology_frame,
            next_day_index,
            gsi_owner_configuration,
            gsi_state,
            provider_static_configuration,
            provider_cursor,
            root_zone_hydraulic_configuration,
        )
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn try_new_with_soil_resident(
        vegetation_configuration: VegetationConfiguration,
        vegetation_state: V10CoupledOwnedState,
        vegetation_owner_id: ResourceOwnerId,
        lse_configuration: LandSurfaceEnergyConfiguration,
        lse_state: LandSurfaceEnergyV2State,
        surface_configuration: DirectSurfaceLiquidConfiguration,
        layer_maps: Vec<RealHydrologyLaneLayerMap>,
        soil_thermal: DirectSoilThermalResident,
        biogeochemistry: BiogeochemistryState,
        hydrology_frame: DirectRunFrame,
        next_day_index: usize,
        gsi_owner_configuration: DirectGsiOwnerConfigurationV1,
        gsi_state: GsiState,
        provider_static_configuration: SnowFreeHalfHourStaticConfiguration,
        provider_cursor: SnowFreeHalfHourProviderCursor,
        root_zone_hydraulic_configuration: DirectRootZoneHydraulicConfiguration,
    ) -> Result<Self, DirectV10RealConsumerError> {
        gsi_owner_configuration.validate()?;
        provider_static_configuration.validate()?;
        let expected_root_layers = layer_maps
            .iter()
            .flat_map(|map| {
                map.layer_ids.iter().map(move |layer_id| {
                    (
                        map.ofe_lane.lane_index,
                        map.ofe_lane.lane_id,
                        layer_id.clone(),
                    )
                })
            })
            .collect::<Vec<_>>();
        let actual_root_layers = root_zone_hydraulic_configuration
            .ordered_layers
            .iter()
            .map(|layer| {
                (
                    layer.production_lane_index,
                    layer.production_lane_id,
                    layer.layer_id.clone(),
                )
            })
            .collect::<Vec<_>>();
        let expected_strata = vegetation_configuration
            .strata
            .iter()
            .map(|stratum| stratum.stratum_id.clone())
            .collect::<Vec<_>>();
        let actual_strata = root_zone_hydraulic_configuration
            .ordered_strata
            .iter()
            .map(|stratum| stratum.stratum_id.clone())
            .collect::<Vec<_>>();
        if actual_root_layers != expected_root_layers || actual_strata != expected_strata {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("root-zone topology/configuration join"),
            ));
        }
        let expected_provider_destinations = lse_configuration
            .ofes
            .iter()
            .flat_map(|ofe| {
                ofe.tiles
                    .iter()
                    .map(move |tile| (ofe.ofe_id.as_str(), tile.tile_id.as_str()))
            })
            .collect::<Vec<_>>();
        let actual_provider_destinations = provider_static_configuration
            .destinations
            .iter()
            .map(|destination| (destination.ofe_id.as_str(), destination.tile_id.as_str()))
            .collect::<Vec<_>>();
        if actual_provider_destinations != expected_provider_destinations {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("provider/LSE destination topology"),
            ));
        }
        if provider_static_configuration.gsi_owner_configuration_sha256
            != gsi_owner_configuration.configuration_sha256
            || provider_static_configuration.run_id != hydrology_frame.identity.run_id.to_string()
        {
            return Err(DirectV10RealConsumerError::ForcingProvider(
                SnowFreeHalfHourForcingError::Identity("initial GSI/provider owner join"),
            ));
        }
        provider_cursor
            .validate_for_configuration(&provider_static_configuration, next_day_index)?;
        vegetation_state.validate(&vegetation_configuration)?;
        lse_state.validate(&lse_configuration)?;
        // No daily GSI receipt exists at construction. Closure-eligible
        // execution installs the accepted receipt before projecting forcing.
        let provider_gsi_receipt_sha256 = "0".repeat(64);
        let (v9_configuration, v9_state) =
            project_v10_runtime_to_v9(&vegetation_configuration, &vegetation_state)?;
        let (v8_configuration, _) = project_v9_runtime_to_v8(&v9_configuration, &v9_state)
            .map_err(DirectV9RealConsumerError::V9)?;
        let v10_configuration_sha256 = openwepp_land_surface_energy::Sha256Digest::try_new(
            vegetation_configuration.configuration_sha256.clone(),
        )?;
        if lse_configuration
            .vegetation_configuration
            .configuration_sha256
            != v10_configuration_sha256
        {
            return Err(DirectV10RealConsumerError::LseV2(
                LseV2StateError::VegetationIdentity,
            ));
        }
        let v8_configuration_sha256 = openwepp_land_surface_energy::Sha256Digest::try_new(
            v8_configuration.configuration_sha256.clone(),
        )?;
        let (v1_configuration, v1_state) =
            project_v2_runtime_to_v1(&lse_configuration, &lse_state, &v8_configuration_sha256)?;
        let mut inner = DirectV9RealConsumerShadow::try_new_with_authority(
            v9_configuration,
            v9_state,
            vegetation_owner_id,
            v1_configuration,
            v1_state,
            surface_configuration,
            layer_maps,
            soil_thermal,
            biogeochemistry,
            hydrology_frame,
            next_day_index,
            CoveredColumnAuthority::V10NonpositiveAssimilation,
            provider_gsi_receipt_sha256,
        )?;
        inner.root_zone_hydraulic_configuration = Some(root_zone_hydraulic_configuration.clone());
        Ok(Self {
            inner,
            vegetation_configuration,
            vegetation_state,
            lse_configuration,
            lse_state,
            gsi_owner_configuration,
            gsi_state,
            provider_static_configuration,
            provider_cursor,
            root_zone_hydraulic_configuration,
            accepted_publication_history: AcceptedPublicationHistoryV1::default(),
        })
    }

    /// Execute and commit one complete provider/owner day atomically. Every
    /// fallible projection, physical solve, successor reconstruction, and
    /// GSI/cursor guard completes on `candidate` before the single assignment.
    pub fn execute_prepared_gsi_day(
        &mut self,
        production_frame: &DirectRunFrame,
        projected_day_frames: &[DirectDayFrame],
        projected_day_inputs: &[DirectPublicationDayInput],
        prepared: PreparedSnowFreeGsiDayV1,
        mut template: DirectV10ShadowDayInput,
    ) -> Result<DirectV10ShadowDayReceipt, DirectV10RealConsumerError> {
        let gsi_receipt = prepared.gsi_receipt();
        if gsi_receipt.run_id != self.provider_static_configuration.run_id
            || gsi_receipt.configuration_sha256 != self.gsi_owner_configuration.configuration_sha256
            || prepared.forcing_receipts().len()
                != self.provider_static_configuration.destinations.len()
            || prepared
                .forcing_receipts()
                .iter()
                .zip(&self.provider_static_configuration.destinations)
                .any(|(receipt, destination)| {
                    receipt.intervals.len() != INTERVALS_PER_DAY
                        || receipt.intervals.iter().any(|interval| {
                            interval.ofe_id != destination.ofe_id
                                || interval.tile_id != destination.tile_id
                                || interval.wb14_configuration_sha256
                                    != destination.wb14_configuration_sha256
                                || interval.co2_pa.to_bits()
                                    != self.provider_static_configuration.co2_pa.to_bits()
                                || interval.reference_height_m.to_bits()
                                    != self
                                        .provider_static_configuration
                                        .reference_height_m
                                        .to_bits()
                        })
                })
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("prepared provider static owner"),
            ));
        }
        let mut candidate = self.clone();
        let accepted_gsi_receipt_sha256 = gsi_receipt.receipt_sha256.clone();
        let accepted_gsi_day_index = gsi_receipt.day_index;
        for interval in &mut template.intervals {
            interval.vegetation_forcing.gsi = gsi_receipt.result.growing_season_index;
        }
        candidate
            .inner
            .provider_gsi_receipt_sha256
            .clone_from(&gsi_receipt.receipt_sha256);
        let projected =
            candidate.project_repository_forcing_receipts(prepared.forcing_receipts(), template)?;
        let receipt = candidate.execute_day(
            production_frame,
            projected_day_frames,
            projected_day_inputs,
            &projected,
        )?;
        candidate.inner.provider_gsi_receipt_sha256 = accepted_gsi_receipt_sha256;
        prepared.commit(&mut candidate.gsi_state, &mut candidate.provider_cursor)?;
        candidate.provider_cursor.validate_for_configuration(
            &candidate.provider_static_configuration,
            usize::try_from(accepted_gsi_day_index)
                .map_err(|_| DirectV9RealConsumerError::Identity("provider day width"))?
                .checked_add(1)
                .ok_or(DirectV9RealConsumerError::Identity("provider day overflow"))?,
        )?;
        *self = candidate;
        Ok(receipt)
    }

    #[cfg(test)]
    fn snow_free_provider_configuration(
        &self,
        template: &DirectV10ShadowDayInput,
    ) -> Result<SnowFreeHalfHourProviderConfiguration, DirectV10RealConsumerError> {
        Ok(self.inner.snow_free_provider_configuration(template)?)
    }

    fn project_repository_forcing_receipts(
        &self,
        provider: &ValidatedSnowFreeHalfHourForcingReceipts,
        template: DirectV10ShadowDayInput,
    ) -> Result<DirectV10ShadowDayInput, DirectV10RealConsumerError> {
        Ok(self
            .inner
            .project_repository_forcing_receipts(provider, template)?)
    }

    /// Construct all 48 V11 interval capabilities from one externally
    /// validated static forcing template and the live repository owner day.
    ///
    /// The template supplies only configuration-owned, non-climate science
    /// (soil/root geometry and optical/static forcing operands). Repository
    /// receipts replace atmospheric, radiation, GSI, and precipitation
    /// custody, while the live owner sequence supplies transaction identity.
    pub fn prepare_v11_intervals_from_repository(
        &self,
        provider: &PreparedSnowFreeGsiDayV1,
        template: &DirectV9ShadowIntervalInput,
    ) -> Result<
        Vec<(
            DirectV9ShadowIntervalInput,
            DirectV11SnowCoveredSegmentInput,
        )>,
        DirectV11RealConsumerError,
    > {
        let projected = self.project_v11_repository_forcing_on_unpublished_candidate(
            provider.gsi_receipt(),
            provider.forcing_receipts(),
            template,
        )?;
        projected
            .intervals
            .into_iter()
            .map(|snow_free| {
                let mut covered_lse = snow_free.lse_forcing.clone();
                covered_lse.snow_present_at_end = true;
                covered_lse.forcing_sha256 = covered_lse
                    .canonical_sha256()
                    .map_err(DirectV10RealConsumerError::from)?;
                let covered = DirectV11SnowCoveredSegmentInput::try_new(
                    covered_lse,
                    snow_free.vegetation_forcing.clone(),
                    snow_free.wb14_parameters.clone(),
                )?;
                Ok((snow_free, covered))
            })
            .collect()
    }

    fn project_v11_repository_forcing_on_unpublished_candidate(
        &self,
        gsi_receipt: &DirectGsiDailyReceiptV1,
        forcing_receipts: &ValidatedSnowFreeHalfHourForcingReceipts,
        template: &DirectV9ShadowIntervalInput,
    ) -> Result<DirectV10ShadowDayInput, DirectV11RealConsumerError> {
        gsi_receipt
            .validate()
            .map_err(DirectV10RealConsumerError::from)?;
        if gsi_receipt.run_id != self.provider_static_configuration.run_id
            || gsi_receipt.configuration_sha256 != self.gsi_owner_configuration.configuration_sha256
            || forcing_receipts.len() != self.provider_static_configuration.destinations.len()
            || forcing_receipts
                .iter()
                .zip(&self.provider_static_configuration.destinations)
                .any(|(receipt, destination)| {
                    receipt.intervals.len() != INTERVALS_PER_DAY
                        || receipt.intervals.iter().any(|interval| {
                            interval.ofe_id != destination.ofe_id
                                || interval.tile_id != destination.tile_id
                                || interval.wb14_configuration_sha256
                                    != destination.wb14_configuration_sha256
                                || interval.co2_pa.to_bits()
                                    != self.provider_static_configuration.co2_pa.to_bits()
                                || interval.reference_height_m.to_bits()
                                    != self
                                        .provider_static_configuration
                                        .reference_height_m
                                        .to_bits()
                        })
                })
        {
            return Err(DirectV11RealConsumerError::Identity(
                "prepared V11 provider static owner",
            ));
        }
        let day_index = usize::try_from(gsi_receipt.day_index)
            .map_err(|_| DirectV11RealConsumerError::Identity("provider day width"))?;
        let gsi = gsi_receipt.result.growing_season_index;
        let mut intervals = Vec::with_capacity(INTERVALS_PER_DAY);
        for interval_index in 0..INTERVALS_PER_DAY {
            let ordinal = u128::try_from(interval_index)
                .map_err(|_| DirectV11RealConsumerError::Identity("interval transaction width"))?
                .checked_add(1)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "interval transaction overflow",
                ))?;
            let transaction_id = self
                .vegetation_state
                .0
                .last_transaction_id
                .checked_add(ordinal)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "interval transaction overflow",
                ))?;
            let mut interval = template.clone();
            interval.lse_forcing.transaction_id = TransactionId(transaction_id);
            interval.lse_forcing.interval_s = 1_800.0;
            interval.lse_forcing.snow_present_at_beginning = false;
            interval.lse_forcing.snow_present_at_end = false;
            interval.lse_forcing.snow_terminal_payload_present = false;
            interval.lse_forcing.precipitation_parcels.clear();
            interval.lse_forcing.runon_parcels.clear();
            interval.lse_forcing.forcing_sha256 = interval
                .lse_forcing
                .canonical_sha256()
                .map_err(DirectV10RealConsumerError::from)?;
            interval.vegetation_forcing.gsi = gsi;
            intervals.push(interval);
        }
        let day = DirectV9ShadowDayInput::try_new(day_index, intervals)
            .map_err(DirectV10RealConsumerError::from)?;
        let mut candidate = self.clone();
        candidate
            .inner
            .provider_gsi_receipt_sha256
            .clone_from(&gsi_receipt.receipt_sha256);
        Ok(candidate.project_repository_forcing_receipts(forcing_receipts, day)?)
    }

    fn execute_day(
        &mut self,
        production_frame: &DirectRunFrame,
        projected_day_frames: &[DirectDayFrame],
        projected_day_inputs: &[DirectPublicationDayInput],
        input: &DirectV10ShadowDayInput,
    ) -> Result<DirectV10ShadowDayReceipt, DirectV10RealConsumerError> {
        let mut candidate = self.clone();
        let receipt = candidate.inner.execute_day(
            production_frame,
            projected_day_frames,
            projected_day_inputs,
            input,
        )?;
        candidate.vegetation_state = project_v9_runtime_to_v10(
            candidate.inner.vegetation_state(),
            &candidate.vegetation_configuration,
        )?;
        candidate.lse_state = project_validated_v1_runtime_to_v2(
            &candidate.inner.lse_configuration,
            candidate.inner.lse_state(),
            &candidate.lse_configuration,
            &openwepp_land_surface_energy::Sha256Digest::try_new(
                candidate
                    .vegetation_configuration
                    .configuration_sha256
                    .clone(),
            )?,
        )?;
        *self = candidate;
        Ok(receipt)
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    #[must_use]
    pub const fn restart_authority_vegetation_configuration(&self) -> &VegetationConfiguration {
        &self.vegetation_configuration
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    #[must_use]
    pub const fn restart_authority_lse_configuration(&self) -> &LandSurfaceEnergyConfiguration {
        &self.lse_configuration
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    #[must_use]
    pub const fn restart_authority_hydrology_frame(&self) -> &DirectRunFrame {
        self.inner.hydrology_frame()
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    #[must_use]
    pub fn restart_authority_soil_thermal(
        &self,
    ) -> Result<&SoilThermalSnapshot, DirectV9RealConsumerError> {
        self.inner.soil_thermal()
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    #[must_use]
    pub const fn restart_authority_biogeochemistry(&self) -> &BiogeochemistryState {
        self.inner.biogeochemistry()
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    #[must_use]
    pub const fn restart_authority_vegetation_owner_id(&self) -> &ResourceOwnerId {
        &self.inner.vegetation_owner_id
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    #[must_use]
    pub const fn restart_authority_surface_configuration(
        &self,
    ) -> &DirectSurfaceLiquidConfiguration {
        self.inner.restart_authority_surface_configuration()
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    pub fn restart_authority_accepted_publication_supports_canonical_bytes(
        &self,
    ) -> Result<Vec<u8>, DirectV10RealConsumerError> {
        let supports = self
            .accepted_publication_history
            .supports()
            .iter()
            .map(|support| support.to_wire())
            .collect::<Vec<_>>();
        let traversed_ending_complete_owner_set_sha256 = self
            .accepted_publication_history
            .validate_cached_tail_against_full_scan()
            .map_err(|_| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                    "accepted publication authority chronology",
                ))
            })?;
        let authority_bytes = serde_json::to_vec(&(
            &supports,
            self.accepted_publication_history.event_handoffs(),
        ))
        .map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Serialization(
                "accepted publication authority projection".to_owned(),
            ))
        })?;
        let mut preimage = b"OPENWEPP_ACCEPTED_PUBLICATION_AUTHORITY_RESTART_V2\0".to_vec();
        preimage.extend_from_slice(&authority_bytes);
        let wire = Stage3AcceptedPublicationSupportSetWireV2 {
            schema_version: 2,
            supports,
            event_handoffs: self.accepted_publication_history.event_handoffs().to_vec(),
            traversed_ending_complete_owner_set_sha256,
            receipt_sha256: digest_bytes(&preimage),
        };
        serde_json::to_vec(&wire).map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Serialization(
                "accepted publication support encoding".to_owned(),
            ))
        })
    }

    pub(crate) fn adaptive_parent_telemetry_publication_shape_v1(&self) -> (usize, usize) {
        (
            self.accepted_publication_history.supports().len(),
            self.accepted_publication_history.event_handoffs().len(),
        )
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    pub fn restart_authority_restore_accepted_publication_supports_canonical_bytes(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), DirectV10RealConsumerError> {
        let wire: Stage3AcceptedPublicationSupportSetWireV2 = serde_json::from_slice(bytes)
            .map_err(|_| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Serialization(
                    "accepted publication support decoding".to_owned(),
                ))
            })?;
        let canonical = serde_json::to_vec(&wire).map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Serialization(
                "accepted publication support canonicalization".to_owned(),
            ))
        })?;
        let authority_bytes =
            serde_json::to_vec(&(&wire.supports, &wire.event_handoffs)).map_err(|_| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Serialization(
                    "accepted publication authority seal projection".to_owned(),
                ))
            })?;
        let mut preimage = b"OPENWEPP_ACCEPTED_PUBLICATION_AUTHORITY_RESTART_V2\0".to_vec();
        preimage.extend_from_slice(&authority_bytes);
        let supports = wire
            .supports
            .iter()
            .cloned()
            .map(Stage3AcceptedPublicationSupportV1::try_from_wire)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                    "accepted publication support wire operands",
                ))
            })?;
        let shared_supports = supports
            .iter()
            .cloned()
            .map(std::sync::Arc::new)
            .collect::<Vec<_>>();
        let traversed =
            validate_accepted_publication_authority(&shared_supports, &wire.event_handoffs)
                .map_err(|_| {
                    DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                        "accepted publication authority wire chronology",
                    ))
                })?;
        if wire.schema_version != 2
            || canonical != bytes
            || wire.receipt_sha256 != digest_bytes(&preimage)
            || wire.traversed_ending_complete_owner_set_sha256 != traversed
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("accepted publication support wire seal"),
            ));
        }
        self.restore_accepted_publication_authority(supports, wire.event_handoffs)
            .map_err(|_| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                    "accepted publication support wire chronology",
                ))
            })
    }

    /// Ending complete-owner digest reached by traversing every retained
    /// accepted support and its zero-duration accepted-event handoffs. The
    /// persisted attachment restore must join this value to its restored V11
    /// complete-owner authority, which makes omission of a terminal handoff
    /// fail even when no later positive-duration support exists yet.
    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    pub fn restart_authority_accepted_publication_traversed_ending_owner_sha256(
        &self,
    ) -> Result<Option<Digest32>, DirectV10RealConsumerError> {
        self.accepted_publication_history
            .validate_cached_tail_against_full_scan()
            .map_err(|_| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                    "accepted publication traversed ending owner",
                ))
            })
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    pub fn restart_authority_install_staged_daily_owners(
        &mut self,
        gsi_state: GsiState,
        provider_cursor: SnowFreeHalfHourProviderCursor,
        expected_next_day_index: usize,
    ) -> Result<(), DirectV10RealConsumerError> {
        provider_cursor.validate_for_configuration(
            &self.provider_static_configuration,
            expected_next_day_index,
        )?;
        self.gsi_state = gsi_state;
        self.provider_cursor = provider_cursor;
        Ok(())
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    pub fn restart_authority_install_scheduler_position(
        &mut self,
        accepted_interval_count: u64,
    ) -> Result<(), DirectV10RealConsumerError> {
        if accepted_interval_count == 0
            || accepted_interval_count > u64::from(u32::MAX) * INTERVALS_PER_DAY as u64
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Unsupported("restart evidence scheduler position"),
            ));
        }
        self.inner.accepted_interval_count = accepted_interval_count;
        Ok(())
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    #[must_use]
    pub const fn restart_authority_accepted_interval_count(&self) -> u64 {
        self.inner.accepted_interval_count()
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    #[must_use]
    pub const fn restart_authority_next_day_index(&self) -> usize {
        self.inner.next_day_index()
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    pub fn restart_authority_advance_staged_intervals(
        &mut self,
        prepared: &PreparedSnowFreeGsiDayV1,
        mut template: DirectV10ShadowDayInput,
        start_interval: usize,
        end_interval_exclusive: usize,
    ) -> Result<(), DirectV10RealConsumerError> {
        if start_interval >= end_interval_exclusive || end_interval_exclusive > INTERVALS_PER_DAY {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Unsupported("restart evidence interval range"),
            ));
        }
        let mut candidate = self.clone();
        let receipt = prepared.gsi_receipt();
        for interval in &mut template.intervals {
            interval.vegetation_forcing.gsi = receipt.result.growing_season_index;
        }
        candidate
            .inner
            .provider_gsi_receipt_sha256
            .clone_from(&receipt.receipt_sha256);
        let template_day_index = template.day_index;
        let projected =
            candidate.project_repository_forcing_receipts(prepared.forcing_receipts(), template)?;
        let day_index =
            usize::try_from(candidate.inner.accepted_interval_count / INTERVALS_PER_DAY as u64)
                .map_err(|_| {
                    DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                        "restart evidence day index overflow",
                    ))
                })?;
        if template_day_index != day_index || candidate.inner.next_day_index != day_index {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("restart evidence continuation day"),
            ));
        }
        for interval_index in start_interval..end_interval_exclusive {
            candidate.inner.execute_interval(
                day_index,
                interval_index,
                &projected.intervals[interval_index],
            )?;
        }
        candidate.vegetation_state = project_v9_runtime_to_v10(
            candidate.inner.vegetation_state(),
            &candidate.vegetation_configuration,
        )?;
        candidate.lse_state = project_validated_v1_runtime_to_v2(
            &candidate.inner.lse_configuration,
            candidate.inner.lse_state(),
            &candidate.lse_configuration,
            &openwepp_land_surface_energy::Sha256Digest::try_new(
                candidate
                    .vegetation_configuration
                    .configuration_sha256
                    .clone(),
            )?,
        )?;
        if end_interval_exclusive == INTERVALS_PER_DAY {
            candidate.inner.next_day_index =
                day_index
                    .checked_add(1)
                    .ok_or(DirectV10RealConsumerError::Runtime(
                        DirectV9RealConsumerError::Identity("restart evidence next day overflow"),
                    ))?;
            candidate.inner.validate_complete_owner_set()?;
        }
        *self = candidate;
        Ok(())
    }

    #[cfg(test)]
    fn execute_first_interval_for_test(
        &mut self,
        input: &DirectV10ShadowDayInput,
    ) -> Result<(), DirectV10RealConsumerError> {
        let mut candidate = self.clone();
        candidate
            .inner
            .execute_interval(0, 0, &input.intervals[0])?;
        candidate.vegetation_state = project_v9_runtime_to_v10(
            candidate.inner.vegetation_state(),
            &candidate.vegetation_configuration,
        )?;
        candidate.lse_state = project_validated_v1_runtime_to_v2(
            &candidate.inner.lse_configuration,
            candidate.inner.lse_state(),
            &candidate.lse_configuration,
            &openwepp_land_surface_energy::Sha256Digest::try_new(
                candidate
                    .vegetation_configuration
                    .configuration_sha256
                    .clone(),
            )?,
        )?;
        *self = candidate;
        Ok(())
    }

    #[cfg(test)]
    fn execute_intervals_for_test(
        &mut self,
        input: &DirectV10ShadowDayInput,
        through_interval: usize,
    ) -> Result<(), DirectV10RealConsumerError> {
        let mut candidate = self.clone();
        for interval_index in 0..=through_interval {
            candidate.inner.execute_interval(
                0,
                interval_index,
                &input.intervals[interval_index],
            )?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectV9ShadowDayReceipt {
    pub day_index: usize,
    pub accepted_interval_count: usize,
    pub first_transaction_id: TransactionId,
    pub last_transaction_id: TransactionId,
    pub beginning_shadow_diagnostic_fingerprint: String,
    pub ending_shadow_diagnostic_fingerprint: String,
}

#[derive(Debug, Error, PartialEq)]
pub enum DirectV9RealConsumerError {
    #[error("V9 real-consumer identity failure: {0}")]
    Identity(&'static str),
    #[error("V9 real-consumer unsupported domain: {0}")]
    Unsupported(&'static str),
    #[error("V9 real-consumer owner closure failure: {0}")]
    OwnerClosure(&'static str),
    #[error(transparent)]
    Vegetation(#[from] VegetationError),
    #[error(transparent)]
    V9(#[from] V9StateError),
    #[error(transparent)]
    Physical(#[from] ExecuteV8LseRuntimeShadowError),
    #[error(transparent)]
    LandSurface(#[from] LandSurfaceEnergyError),
    #[error(transparent)]
    LandSurfaceShadow(#[from] LandSurfaceEnergyShadowError),
    #[error(transparent)]
    RealHydrology(#[from] RealHydrologyShadowError),
    #[error(transparent)]
    Biogeochemistry(#[from] BiogeochemistryError),
    #[error(transparent)]
    Projection(#[from] V8InputProjectionError),
    #[error(transparent)]
    OwnerEnvelope(#[from] CoveredV8OwnerEnvelopeError),
    #[error("V9 real-consumer serialization failure: {0}")]
    Serialization(String),
}

impl DirectV9RealConsumerError {
    #[must_use]
    pub const fn category(&self) -> &'static str {
        match self {
            Self::Identity(_) => "identity",
            Self::Unsupported(_) => "unsupported",
            Self::OwnerClosure(_) => "owner_closure",
            Self::Vegetation(_) => "vegetation",
            Self::V9(_) => "v9_identity",
            Self::Physical(_) => "strict_v8_lse_runtime",
            Self::LandSurface(_) => "land_surface",
            Self::LandSurfaceShadow(_) => "land_surface_shadow",
            Self::RealHydrology(_) => "real_hydrology",
            Self::Biogeochemistry(_) => "biogeochemistry",
            Self::Projection(_) => "projection",
            Self::OwnerEnvelope(_) => "owner_envelope",
            Self::Serialization(_) => "serialization",
        }
    }
}

impl DirectV9RealConsumerShadow {
    /// Derive provider identity exclusively from canonical shadow owners and
    /// the live interval template.
    pub fn snow_free_provider_configuration(
        &self,
        template: &DirectV9ShadowDayInput,
    ) -> Result<SnowFreeHalfHourProviderConfiguration, DirectV9RealConsumerError> {
        let first = template
            .intervals
            .first()
            .ok_or(DirectV9RealConsumerError::Identity("shadow day intervals"))?;
        if template.intervals.len() != INTERVALS_PER_DAY {
            return Err(DirectV9RealConsumerError::Identity(
                "shadow day interval cardinality",
            ));
        }
        let mut destinations = Vec::new();
        for ofe in &self.lse_configuration.ofes {
            let wb14 = first
                .wb14_parameters
                .iter()
                .find(|value| value.ofe_id == ofe.ofe_id)
                .ok_or(DirectV9RealConsumerError::Identity(
                    "repository WB14 OFE binding",
                ))?;
            for tile in &ofe.tiles {
                destinations.push(SnowFreeHalfHourDestination {
                    ofe_id: ofe.ofe_id.as_str().to_string(),
                    tile_id: tile.tile_id.as_str().to_string(),
                    wb14_configuration_sha256: wb14_parameter_sha256(wb14),
                });
            }
        }
        Ok(SnowFreeHalfHourProviderConfiguration {
            run_id: self.hydrology_frame.identity.run_id.to_string(),
            co2_pa: first.vegetation_forcing.co2_pa,
            reference_height_m: first.vegetation_forcing.reference_height_m,
            gsi: first.vegetation_forcing.gsi,
            gsi_receipt_sha256: self.provider_gsi_receipt_sha256.clone(),
            destinations,
        })
    }

    /// Project a sealed repository forcing receipt into real Child-4 interval
    /// types while joining run, GSI-owner, and WB14-owner identity.
    pub fn project_repository_forcing_receipts(
        &self,
        provider: &ValidatedSnowFreeHalfHourForcingReceipts,
        template: DirectV9ShadowDayInput,
    ) -> Result<DirectV9ShadowDayInput, DirectV9RealConsumerError> {
        let expected_destinations = self
            .lse_configuration
            .ofes
            .iter()
            .flat_map(|ofe| {
                ofe.tiles.iter().map(|tile| {
                    (
                        ofe.ofe_id.as_str().to_string(),
                        tile.tile_id.as_str().to_string(),
                    )
                })
            })
            .collect();
        project_repository_forcing_receipts_to_v9_day(
            provider,
            template,
            self.hydrology_frame.identity.run_id,
            &self.provider_gsi_receipt_sha256,
            &expected_destinations,
        )
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub fn try_new(
        vegetation_configuration: VegetationConfiguration,
        vegetation_state: V9CoupledOwnedState,
        vegetation_owner_id: ResourceOwnerId,
        lse_configuration: LandSurfaceEnergyConfiguration,
        lse_state: LandSurfaceEnergyState,
        surface_configuration: DirectSurfaceLiquidConfiguration,
        layer_maps: Vec<RealHydrologyLaneLayerMap>,
        soil_thermal: SoilThermalSnapshot,
        biogeochemistry: BiogeochemistryState,
        hydrology_frame: DirectRunFrame,
        next_day_index: usize,
    ) -> Result<Self, DirectV9RealConsumerError> {
        let provider_gsi_receipt_sha256 = vegetation_state.0.state_sha256.clone();
        Self::try_new_with_authority(
            vegetation_configuration,
            vegetation_state,
            vegetation_owner_id,
            lse_configuration,
            lse_state,
            surface_configuration,
            layer_maps,
            DirectSoilThermalResident::try_new_v1(soil_thermal)?,
            biogeochemistry,
            hydrology_frame,
            next_day_index,
            CoveredColumnAuthority::HistoricalV8,
            provider_gsi_receipt_sha256,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn try_new_with_authority(
        vegetation_configuration: VegetationConfiguration,
        vegetation_state: V9CoupledOwnedState,
        vegetation_owner_id: ResourceOwnerId,
        lse_configuration: LandSurfaceEnergyConfiguration,
        lse_state: LandSurfaceEnergyState,
        surface_configuration: DirectSurfaceLiquidConfiguration,
        layer_maps: Vec<RealHydrologyLaneLayerMap>,
        soil_thermal: DirectSoilThermalResident,
        biogeochemistry: BiogeochemistryState,
        hydrology_frame: DirectRunFrame,
        next_day_index: usize,
        authority: CoveredColumnAuthority,
        provider_gsi_receipt_sha256: String,
    ) -> Result<Self, DirectV9RealConsumerError> {
        vegetation_state.validate(&vegetation_configuration)?;
        let (v8_configuration, v8_state) =
            project_v9_runtime_to_v8(&vegetation_configuration, &vegetation_state)?;
        lse_configuration.validate()?;
        lse_state.validate(&lse_configuration)?;
        soil_thermal.validate()?;
        if lse_configuration
            .vegetation_configuration
            .configuration_sha256
            .as_str()
            != v8_configuration.configuration_sha256
        {
            return Err(DirectV9RealConsumerError::Identity(
                "initial V9/V8/LSE configuration join",
            ));
        }
        if lse_state
            .last_accepted_transaction_id
            .is_some_and(|value| value.0 != v8_state.last_transaction_id)
        {
            return Err(DirectV9RealConsumerError::Identity(
                "initial vegetation/LSE transaction lineage",
            ));
        }
        if next_day_index >= hydrology_frame.identity.day_count
            || surface_configuration.run_id != hydrology_frame.identity.run_id
        {
            return Err(DirectV9RealConsumerError::Identity(
                "initial scheduler/surface owner identity",
            ));
        }
        let value = Self {
            authority,
            provider_gsi_receipt_sha256,
            vegetation_configuration,
            vegetation_state,
            vegetation_owner_id,
            lse_configuration,
            lse_state,
            surface_configuration,
            layer_maps,
            soil_thermal,
            biogeochemistry,
            hydrology_frame,
            next_day_index,
            accepted_interval_count: 0,
            wb14_parent_working_state: None,
            root_zone_hydraulic_configuration: None,
        };
        value.validate_complete_owner_set()?;
        Ok(value)
    }

    #[must_use]
    pub fn checkpoint(&self) -> DirectV9RealConsumerCheckpoint {
        DirectV9RealConsumerCheckpoint {
            shadow: self.clone(),
        }
    }

    pub fn wb14_parent_restart_bytes(&self) -> Result<Option<Vec<u8>>, DirectV9RealConsumerError> {
        self.wb14_parent_working_state
            .as_ref()
            .map(|state| state.restart_bytes(&self.surface_configuration))
            .transpose()
            .map_err(|error| DirectV9RealConsumerError::Serialization(error.to_string()))
    }

    pub fn restore_wb14_parent_restart_bytes(
        &mut self,
        bytes: Option<&[u8]>,
    ) -> Result<(), DirectV9RealConsumerError> {
        let restored = bytes
            .map(|bytes| {
                crate::direct_runtime::DirectWb14ParentWorkingState::from_restart_bytes(
                    &self.surface_configuration,
                    bytes,
                )
            })
            .transpose()
            .map_err(|error| DirectV9RealConsumerError::Serialization(error.to_string()))?;
        if let Some(restored) = &restored {
            let current = self
                .hydrology_frame
                .surface_liquid_shadow
                .as_deref()
                .ok_or(DirectV9RealConsumerError::Identity(
                    "missing restart receiving surface owner",
                ))?;
            restored
                .validate_receiving_owner(current)
                .map_err(|error| DirectV9RealConsumerError::Serialization(error.to_string()))?;
        }
        self.validate_complete_owner_set()?;
        self.wb14_parent_working_state = restored;
        Ok(())
    }

    pub fn restore(
        checkpoint: DirectV9RealConsumerCheckpoint,
    ) -> Result<Self, DirectV9RealConsumerError> {
        checkpoint.shadow.validate_complete_owner_set()?;
        Ok(checkpoint.shadow)
    }

    #[must_use]
    pub const fn next_day_index(&self) -> usize {
        self.next_day_index
    }

    #[must_use]
    pub const fn accepted_interval_count(&self) -> u64 {
        self.accepted_interval_count
    }

    #[must_use]
    pub const fn vegetation_state(&self) -> &V9CoupledOwnedState {
        &self.vegetation_state
    }

    #[must_use]
    pub const fn lse_state(&self) -> &LandSurfaceEnergyState {
        &self.lse_state
    }

    #[must_use]
    pub fn soil_thermal(&self) -> Result<&SoilThermalSnapshot, DirectV9RealConsumerError> {
        self.soil_thermal.v1()
    }

    #[must_use]
    pub const fn hydrology_frame(&self) -> &DirectRunFrame {
        &self.hydrology_frame
    }

    #[must_use]
    pub const fn biogeochemistry(&self) -> &BiogeochemistryState {
        &self.biogeochemistry
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    #[must_use]
    pub const fn restart_authority_surface_configuration(
        &self,
    ) -> &DirectSurfaceLiquidConfiguration {
        &self.surface_configuration
    }

    pub(crate) fn execute_day(
        &mut self,
        production_frame: &DirectRunFrame,
        projected_day_frames: &[DirectDayFrame],
        projected_day_inputs: &[DirectPublicationDayInput],
        input: &DirectV9ShadowDayInput,
    ) -> Result<DirectV9ShadowDayReceipt, DirectV9RealConsumerError> {
        if input.day_index != self.next_day_index
            || input.day_index >= production_frame.identity.day_count
            || production_frame.identity != self.hydrology_frame.identity
        {
            return Err(DirectV9RealConsumerError::Identity(
                "scheduler day or production frame identity",
            ));
        }
        if input.intervals.len() != INTERVALS_PER_DAY {
            return Err(DirectV9RealConsumerError::Unsupported(
                "a shadow day requires exactly 48 intervals",
            ));
        }
        validate_repository_day_projection(
            production_frame,
            projected_day_frames,
            projected_day_inputs,
            input,
            &self.lse_configuration,
            &self.surface_configuration,
        )?;
        let beginning_shadow_diagnostic_fingerprint = self.diagnostic_fingerprint()?;
        let first_transaction_id = input.intervals[0].lse_forcing.transaction_id;
        let last_transaction_id = input.intervals[INTERVALS_PER_DAY - 1]
            .lse_forcing
            .transaction_id;
        let mut candidate = self.clone();
        for (interval_index, interval) in input.intervals.iter().enumerate() {
            candidate.execute_interval(input.day_index, interval_index, interval)?;
        }
        candidate.next_day_index = candidate
            .next_day_index
            .checked_add(1)
            .ok_or(DirectV9RealConsumerError::Identity("shadow day overflow"))?;
        candidate.validate_complete_owner_set()?;
        let ending_shadow_diagnostic_fingerprint = candidate.diagnostic_fingerprint()?;
        *self = candidate;
        Ok(DirectV9ShadowDayReceipt {
            day_index: input.day_index,
            accepted_interval_count: INTERVALS_PER_DAY,
            first_transaction_id,
            last_transaction_id,
            beginning_shadow_diagnostic_fingerprint,
            ending_shadow_diagnostic_fingerprint,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn execute_interval(
        &mut self,
        day_index: usize,
        interval_index: usize,
        input: &DirectV9ShadowIntervalInput,
    ) -> Result<(), DirectV9RealConsumerError> {
        let envelope = self.construct_snow_free_interval_envelope_with_duration(
            day_index,
            interval_index,
            input,
            INTERVAL_S,
            None,
        )?;
        self.accept_envelope(envelope.vegetation().transaction_id(), &envelope)
    }

    #[allow(clippy::too_many_lines)]
    fn construct_snow_free_interval_envelope_with_duration(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &DirectV9ShadowIntervalInput,
        interval_s: f64,
        v11_duration_s_bits: Option<u64>,
    ) -> Result<UncommittedCoveredV8OwnerEnvelope, DirectV9RealConsumerError> {
        self.construct_snow_free_parent_child_envelope_with_duration(
            day_index,
            interval_index,
            input,
            interval_s,
            v11_duration_s_bits,
            true,
            None,
        )
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn construct_snow_free_parent_child_envelope_with_duration(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &DirectV9ShadowIntervalInput,
        interval_s: f64,
        v11_duration_s_bits: Option<u64>,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: Option<crate::direct_runtime::DirectWb14CoupledChildBindingV1>,
    ) -> Result<UncommittedCoveredV8OwnerEnvelope, DirectV9RealConsumerError> {
        self.construct_canopy_soil_interval_envelope_with_duration(
            day_index,
            interval_index,
            input,
            interval_s,
            v11_duration_s_bits,
            None,
            false,
            None,
            finalize_wb14_parent_interval,
            wb14_coupled_child_binding,
            false,
        )
        .and_then(CanopySoilEvaluationV1::into_complete)
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn construct_canopy_soil_interval_envelope_with_duration(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &DirectV9ShadowIntervalInput,
        interval_s: f64,
        v11_duration_s_bits: Option<u64>,
        covered_lower_boundaries: Option<
            &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        >,
        provisional_v11: bool,
        covered_destinations: Option<&BTreeSet<(OfeId, TileId)>>,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: Option<crate::direct_runtime::DirectWb14CoupledChildBindingV1>,
        physical_only: bool,
    ) -> Result<CanopySoilEvaluationV1, DirectV9RealConsumerError> {
        let transaction_id = TransactionId(
            self.vegetation_state
                .0
                .last_transaction_id
                .checked_add(1)
                .ok_or(DirectV9RealConsumerError::Identity(
                    "vegetation transaction overflow",
                ))?,
        );
        let interval_index = u8::try_from(interval_index)
            .map_err(|_| DirectV9RealConsumerError::Identity("interval index overflow"))?;
        if input.lse_forcing.transaction_id != transaction_id {
            return Err(DirectV9RealConsumerError::Unsupported(
                "forcing transaction identity",
            ));
        }
        if input.lse_forcing.interval_s.to_bits() != interval_s.to_bits()
            || v11_duration_s_bits.is_some_and(|bits| bits != interval_s.to_bits())
        {
            return Err(DirectV9RealConsumerError::Unsupported(
                "forcing cadence identity",
            ));
        }
        if input.lse_forcing.snow_present_at_beginning
            || input.lse_forcing.snow_present_at_end
            || input.lse_forcing.snow_terminal_payload_present
        {
            return Err(DirectV9RealConsumerError::Unsupported(
                "forcing snow domain",
            ));
        }
        if !input.lse_forcing.runon_parcels.is_empty() {
            return Err(DirectV9RealConsumerError::Unsupported(
                "runon requires an accepted routing publication owner",
            ));
        }
        input.lse_forcing.validate(transaction_id)?;
        let (v8_configuration, v8_beginning) =
            project_v9_runtime_to_v8(&self.vegetation_configuration, &self.vegetation_state)?;
        if self
            .lse_configuration
            .vegetation_configuration
            .configuration_sha256
            .as_str()
            != v8_configuration.configuration_sha256
        {
            return Err(DirectV9RealConsumerError::Identity(
                "V9/V8/LSE configuration join",
            ));
        }
        let mut effective_hydrology_frame = self.hydrology_frame.clone();
        if let Some(parent) = &self.wb14_parent_working_state {
            effective_hydrology_frame.surface_liquid_shadow =
                Some(Box::new(parent.candidate_state().clone()));
        }
        let hydrology = RealHydrologyShadowAdapter::try_from_day_start(
            &effective_hydrology_frame,
            day_index,
            transaction_id,
            interval_s,
            self.surface_configuration.owner_id.clone(),
            &self.layer_maps,
        )?;
        let soil_adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&hydrology);
        let soil_thermal = self.soil_thermal.v1()?;
        let hydrology_snapshot = unified_beginning_hydrology_snapshot_sha256(
            &soil_adapter,
            &self.surface_configuration,
        )?;
        let forcing_sha256 = input.lse_forcing.canonical_sha256()?;
        let (vegetation_forcing, root_zone_hydraulics) = project_live_vegetation_forcing(
            &input.vegetation_forcing,
            &hydrology,
            soil_thermal,
            self.root_zone_hydraulic_configuration.as_ref(),
            &self.surface_configuration,
            &self.lse_configuration,
            &self.vegetation_configuration,
            &self.vegetation_state,
            v8_configuration.configuration_sha256.clone(),
            hydrology_snapshot.clone(),
            transaction_id,
            day_index,
            interval_index,
        )?;
        let canopy_forcing = match root_zone_hydraulics {
            Some(receipts) => V8CanopyForcingReceipt::try_new_with_root_zone(
                v8_configuration.configuration_sha256.clone(),
                v8_beginning.state_sha256.clone(),
                self.lse_configuration.configuration_sha256.clone(),
                forcing_sha256,
                hydrology_snapshot,
                soil_thermal.snapshot_sha256.clone(),
                transaction_id,
                vegetation_forcing,
                receipts,
            )?,
            None => V8CanopyForcingReceipt::try_new(
                v8_configuration.configuration_sha256.clone(),
                v8_beginning.state_sha256.clone(),
                self.lse_configuration.configuration_sha256.clone(),
                forcing_sha256,
                hydrology_snapshot,
                soil_thermal.snapshot_sha256.clone(),
                transaction_id,
                vegetation_forcing,
            )?,
        };
        let nitrogen = BiogeochemistryNitrogenArbiter::try_new(&self.biogeochemistry)?;
        if physical_only {
            let bits = v11_duration_s_bits.ok_or(DirectV9RealConsumerError::Identity(
                "physical-only endpoint requires V11 duration",
            ))?;
            let destinations = covered_destinations.ok_or(DirectV9RealConsumerError::Identity(
                "physical-only covered endpoint destination set",
            ))?;
            let physical = execute_v8_lse_runtime_shadow_v11_physical_with_carriers(
                &v8_configuration,
                &v8_beginning,
                &self.vegetation_owner_id,
                &canopy_forcing,
                &self.lse_configuration,
                &self.lse_state,
                &input.lse_forcing,
                &soil_adapter,
                &self.surface_configuration,
                day_index,
                interval_index,
                &input.wb14_parameters,
                soil_thermal,
                &nitrogen,
                &self.biogeochemistry,
                openwepp_land_surface_energy::CoveredColumnAuthority::V11SnowCovered,
                covered_lower_boundaries,
                bits,
                Some(destinations),
                finalize_wb14_parent_interval,
                self.wb14_parent_working_state.as_ref(),
                wb14_coupled_child_binding,
            )?;
            return Ok(CanopySoilEvaluationV1::PhysicalOnly(
                ProvisionalCoveredV8PhysicalEvaluationV1::try_new(physical)?,
            ));
        }
        let envelope = match v11_duration_s_bits {
            Some(bits) => match covered_destinations {
                Some(destinations) => execute_v8_lse_runtime_shadow_v11_with_carriers(
                    &v8_configuration,
                    &v8_beginning,
                    &self.vegetation_owner_id,
                    &canopy_forcing,
                    &self.lse_configuration,
                    &self.lse_state,
                    &input.lse_forcing,
                    &soil_adapter,
                    &self.surface_configuration,
                    day_index,
                    interval_index,
                    &input.wb14_parameters,
                    soil_thermal,
                    &nitrogen,
                    &self.biogeochemistry,
                    // This entry point is the typed V11 snow-covered carrier
                    // path even for unpublished probe envelopes. Read-only
                    // disposition changes publication authority, not the
                    // physical lower-boundary model used by LSE.
                    openwepp_land_surface_energy::CoveredColumnAuthority::V11SnowCovered,
                    covered_lower_boundaries,
                    bits,
                    !provisional_v11,
                    Some(destinations),
                    finalize_wb14_parent_interval,
                    self.wb14_parent_working_state.as_ref(),
                    wb14_coupled_child_binding,
                )?,
                None => crate::land_surface_energy_shadow::execute_v8_lse_runtime_shadow_v11(
                    &v8_configuration,
                    &v8_beginning,
                    &self.vegetation_owner_id,
                    &canopy_forcing,
                    &self.lse_configuration,
                    &self.lse_state,
                    &input.lse_forcing,
                    &soil_adapter,
                    &self.surface_configuration,
                    day_index,
                    interval_index,
                    &input.wb14_parameters,
                    soil_thermal,
                    &nitrogen,
                    &self.biogeochemistry,
                    self.authority,
                    covered_lower_boundaries,
                    bits,
                    !provisional_v11,
                    finalize_wb14_parent_interval,
                    self.wb14_parent_working_state.as_ref(),
                    wb14_coupled_child_binding,
                )?,
            },
            None => execute_v8_lse_runtime_shadow_internal(
                &v8_configuration,
                &v8_beginning,
                &self.vegetation_owner_id,
                &canopy_forcing,
                &self.lse_configuration,
                &self.lse_state,
                &input.lse_forcing,
                &soil_adapter,
                &self.surface_configuration,
                day_index,
                interval_index,
                &input.wb14_parameters,
                soil_thermal,
                &nitrogen,
                &self.biogeochemistry,
                None,
                self.authority,
            )?,
        };
        Ok(CanopySoilEvaluationV1::Complete(envelope))
    }

    /// Construct the V11 canopy/soil envelope for a Child-2C covered slab.
    ///
    /// Snow is not admitted to the snow-free LSE owner. The Stage-3 snow
    /// column and the canopy/snow air carrier are evaluated and sealed by the
    /// covered adopter before this projection. The V8/LSE endpoint here is
    /// consequently a typed canopy/soil continuation with the carrier's
    /// shared air state; it is not the snow-free lower-boundary selector.
    #[allow(clippy::too_many_arguments)]
    fn construct_covered_interval_envelope_with_duration(
        &self,
        day_index: usize,
        interval_index: usize,
        input: &DirectV11SnowCoveredSegmentInput,
        interval_s: f64,
        v11_duration_s_bits: u64,
        covered_destinations: &BTreeSet<(OfeId, TileId)>,
        lower_boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        provisional_v11: bool,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    ) -> Result<UncommittedCoveredV8OwnerEnvelope, DirectV9RealConsumerError> {
        if !input.lse_forcing.snow_present_at_end || input.lse_forcing.snow_terminal_payload_present
        {
            return Err(DirectV9RealConsumerError::Unsupported(
                "covered adopter requires persistent snow operands",
            ));
        }
        // The carrier owns the snow surface. The LSE endpoint receives only
        // shared air state by keyed destination; no parent aggregate is used.
        let mut canopy_soil_forcing = input.lse_forcing.clone();
        canopy_soil_forcing.snow_present_at_beginning = false;
        canopy_soil_forcing.snow_present_at_end = false;
        canopy_soil_forcing.forcing_sha256 = canopy_soil_forcing.canonical_sha256()?;
        let covered_vegetation_forcing = input.vegetation_forcing.clone();
        let covered_input = DirectV9ShadowIntervalInput {
            lse_forcing: canopy_soil_forcing,
            vegetation_forcing: covered_vegetation_forcing,
            wb14_parameters: input.wb14_parameters.clone(),
        };
        self.construct_canopy_soil_interval_envelope_with_duration(
            day_index,
            interval_index,
            &covered_input,
            interval_s,
            Some(v11_duration_s_bits),
            Some(lower_boundaries),
            provisional_v11,
            Some(covered_destinations),
            finalize_wb14_parent_interval,
            Some(wb14_coupled_child_binding),
            false,
        )
        .and_then(CanopySoilEvaluationV1::into_complete)
    }

    fn prepare_covered_canopy_soil_input(
        input: &DirectV11SnowCoveredSegmentInput,
    ) -> Result<PreparedCoveredCanopySoilInputV1, DirectV9RealConsumerError> {
        if !input.lse_forcing.snow_present_at_end || input.lse_forcing.snow_terminal_payload_present
        {
            return Err(DirectV9RealConsumerError::Unsupported(
                "covered adopter requires persistent snow operands",
            ));
        }
        let mut canopy_soil_forcing = input.lse_forcing.clone();
        canopy_soil_forcing.snow_present_at_beginning = false;
        canopy_soil_forcing.snow_present_at_end = false;
        canopy_soil_forcing.forcing_sha256 = canopy_soil_forcing.canonical_sha256()?;
        let covered_input = DirectV9ShadowIntervalInput {
            lse_forcing: canopy_soil_forcing,
            vegetation_forcing: input.vegetation_forcing.clone(),
            wb14_parameters: input.wb14_parameters.clone(),
        };
        Ok(PreparedCoveredCanopySoilInputV1(covered_input))
    }

    #[allow(clippy::too_many_arguments)]
    fn construct_prepared_covered_interval_physical_with_duration(
        &self,
        day_index: usize,
        interval_index: usize,
        prepared: &PreparedCoveredCanopySoilInputV1,
        interval_s: f64,
        v11_duration_s_bits: u64,
        covered_destinations: &BTreeSet<(OfeId, TileId)>,
        lower_boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        finalize_wb14_parent_interval: bool,
        wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    ) -> Result<ProvisionalCoveredV8PhysicalEvaluationV1, DirectV9RealConsumerError> {
        self.construct_canopy_soil_interval_envelope_with_duration(
            day_index,
            interval_index,
            &prepared.0,
            interval_s,
            Some(v11_duration_s_bits),
            Some(lower_boundaries),
            true,
            Some(covered_destinations),
            finalize_wb14_parent_interval,
            Some(wb14_coupled_child_binding),
            true,
        )
        .and_then(CanopySoilEvaluationV1::into_physical)
    }

    fn accept_envelope(
        &mut self,
        transaction_id: TransactionId,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
    ) -> Result<(), DirectV9RealConsumerError> {
        envelope.validate()?;
        let vegetation_state = project_v8_runtime_to_v9(
            envelope.vegetation().ending_state(),
            &self.vegetation_configuration,
        )?;
        let lse_state = build_lse_ending_state(
            &self.lse_state,
            transaction_id,
            envelope.hydrology().ending_lse_tile_states().to_vec(),
        )?;
        let soil_thermal = aggregate_soil_thermal_ending(
            self.soil_thermal.v1()?,
            &self.lse_configuration,
            transaction_id,
            envelope.hydrology().soil_thermal_candidates(),
        )?;
        self.vegetation_state = vegetation_state;
        self.lse_state = lse_state;
        self.soil_thermal = DirectSoilThermalResident::try_new_v1(soil_thermal)?;
        self.biogeochemistry = envelope.biogeochemistry().ending().clone();
        self.hydrology_frame = envelope.hydrology().ending_frame().clone();
        self.wb14_parent_working_state = envelope
            .hydrology()
            .surface_ingress()
            .parent_working_state()
            .cloned();
        if envelope
            .hydrology()
            .surface_ingress()
            .advances_persistent_parent_interval()
        {
            self.accepted_interval_count = self.accepted_interval_count.checked_add(1).ok_or(
                DirectV9RealConsumerError::Identity("accepted parent interval count overflow"),
            )?;
        }
        Ok(())
    }

    fn accept_envelope_with_soil_top_boundary_credits(
        &mut self,
        transaction_id: TransactionId,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
        credits: &[SoilThermalTopBoundaryCreditV1],
    ) -> Result<SoilThermalTopBoundaryCreditSetV1, DirectV9RealConsumerError> {
        for credit in credits {
            if credit.snow_soil_heat_receipt_sha256.as_str().len() != 64 {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "snow-soil receipt digest encoding",
                ));
            }
        }
        let beginning_soil = self.soil_thermal.v1()?.clone();
        self.accept_envelope(transaction_id, envelope)?;
        let accepted = aggregate_soil_thermal_ending_with_top_boundary_credits(
            &beginning_soil,
            &self.lse_configuration,
            transaction_id,
            envelope.hydrology().soil_thermal_candidates(),
            credits,
        )?;
        self.soil_thermal = DirectSoilThermalResident::try_new_v1(accepted.ending.clone())?;
        Ok(accepted)
    }

    fn validate_complete_owner_set(&self) -> Result<(), DirectV9RealConsumerError> {
        self.vegetation_state
            .validate(&self.vegetation_configuration)?;
        self.lse_state.validate(&self.lse_configuration)?;
        self.soil_thermal.validate()?;
        let transaction_id = TransactionId(self.vegetation_state.0.last_transaction_id);
        let lse_transaction_matches = self
            .lse_state
            .last_accepted_transaction_id
            .is_none_or(|value| value == transaction_id);
        let soil_transaction_matches = self
            .soil_thermal
            .last_accepted_transaction_id()
            .is_none_or(|value| value == transaction_id);
        let complete_accepted_lineage = self.accepted_interval_count == 0
            || (self.lse_state.last_accepted_transaction_id == Some(transaction_id)
                && self.soil_thermal.last_accepted_transaction_id() == Some(transaction_id));
        let mapping_matches = self
            .surface_configuration
            .ofe_bindings
            .iter()
            .zip(&self.layer_maps)
            .all(|(binding, map)| {
                binding.production_lane_index == map.ofe_lane.lane_index
                    && binding.production_lane_id == map.ofe_lane.lane_id
                    && binding.ordered_soil_layer_ids == map.layer_ids
            });
        if self.surface_configuration.ofe_bindings.len() != self.hydrology_frame.lanes.len()
            || self.layer_maps.len() != self.hydrology_frame.lanes.len()
            || self.biogeochemistry.last_transaction_id
                != self.vegetation_state.0.last_transaction_id
            || !lse_transaction_matches
            || !soil_transaction_matches
            || !complete_accepted_lineage
            || !mapping_matches
        {
            return Err(DirectV9RealConsumerError::Identity(
                "incomplete or mixed complete-owner state",
            ));
        }
        Ok(())
    }

    fn diagnostic_fingerprint(&self) -> Result<String, DirectV9RealConsumerError> {
        #[derive(Serialize)]
        struct ShadowBytes<'a> {
            vegetation: &'a V9CoupledOwnedState,
            lse: &'a LandSurfaceEnergyState,
            soil_thermal: &'a DirectSoilThermalResident,
            biogeochemistry: &'a BiogeochemistryState,
            hydrology_debug: String,
            next_day_index: usize,
            accepted_interval_count: u64,
        }
        let bytes = serde_json::to_vec(&ShadowBytes {
            vegetation: &self.vegetation_state,
            lse: &self.lse_state,
            soil_thermal: &self.soil_thermal,
            biogeochemistry: &self.biogeochemistry,
            hydrology_debug: format!("{:?}", self.hydrology_frame),
            next_day_index: self.next_day_index,
            accepted_interval_count: self.accepted_interval_count,
        })
        .map_err(|error| DirectV9RealConsumerError::Serialization(error.to_string()))?;
        Ok(format!("{:x}", Sha256::digest(bytes)))
    }
}

fn validate_repository_day_projection(
    production_frame: &DirectRunFrame,
    projected_day_frames: &[DirectDayFrame],
    projected_day_inputs: &[DirectPublicationDayInput],
    shadow_input: &DirectV9ShadowDayInput,
    lse_configuration: &LandSurfaceEnergyConfiguration,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<(), DirectV9RealConsumerError> {
    if projected_day_frames.len() != production_frame.identity.lane_count
        || projected_day_inputs.len() != production_frame.identity.lane_count
        || projected_day_frames.len() != projected_day_inputs.len()
    {
        return Err(DirectV9RealConsumerError::Identity(
            "complete repository day projection",
        ));
    }
    for (lane_index, (day_frame, day_input)) in projected_day_frames
        .iter()
        .zip(projected_day_inputs)
        .enumerate()
    {
        if day_frame.identity != production_frame.identity
            || day_frame.lane_index != lane_index
            || day_frame.day_index != shadow_input.day_index
            || day_frame.forcing.precipitation_m.to_bits() != day_input.precipitation_m.to_bits()
            || day_frame.forcing.effective_temperature_c.to_bits()
                != day_input.effective_temperature_c.to_bits()
        {
            return Err(DirectV9RealConsumerError::Identity(
                "repository day input/frame receipt",
            ));
        }
        let binding = surface_configuration
            .ofe_bindings
            .iter()
            .find(|binding| binding.production_lane_index == lane_index)
            .ok_or(DirectV9RealConsumerError::Identity(
                "repository surface-owner OFE/lane projection",
            ))?;
        let ofe = lse_configuration
            .ofes
            .iter()
            .find(|ofe| ofe.ofe_id == binding.ofe_id)
            .ok_or(DirectV9RealConsumerError::Identity(
                "repository LSE/surface-owner OFE projection",
            ))?;
        let expected_precipitation_kg_m2 = day_input.precipitation_m * 1_000.0;
        for tile in &ofe.tiles {
            let custody = shadow_input
                .precipitation_custody
                .as_ref()
                .and_then(|value| {
                    value.current_source_and_outgoing_mass.get(&(
                        ofe.ofe_id.as_str().to_string(),
                        tile.tile_id.as_str().to_string(),
                    ))
                });
            let tile_precipitation_kg_m2 = shadow_input
                .intervals
                .iter()
                .flat_map(|interval| &interval.lse_forcing.precipitation_parcels)
                .filter(|parcel| {
                    parcel.parcel_kind == LiquidParcelKind::Precipitation
                        && parcel.destination_ofe_id == ofe.ofe_id
                        && parcel.destination_tile_id == tile.tile_id
                })
                .filter(|parcel| {
                    custody.is_none_or(|(source, _)| parcel.source_owner_id.as_str() == source)
                })
                .map(|parcel| parcel.amount_kg_m2_destination_tile_ground)
                .fold(0.0, |sum, value| sum + value);
            let reconstructed_source_mass =
                tile_precipitation_kg_m2 + custody.map_or(0.0, |(_, outgoing_mass)| *outgoing_mass);
            let matches = if custody.is_some() {
                (reconstructed_source_mass - expected_precipitation_kg_m2).abs()
                    <= 1.0e-12 * expected_precipitation_kg_m2.abs().max(1.0)
            } else {
                reconstructed_source_mass.to_bits() == expected_precipitation_kg_m2.to_bits()
            };
            if !matches {
                return Err(DirectV9RealConsumerError::Identity(
                    "repository daily precipitation/subdaily LSE parcel mass",
                ));
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn project_live_vegetation_forcing(
    provider: &SnowFreeForcing,
    hydrology: &RealHydrologyShadowAdapter,
    soil_thermal: &SoilThermalSnapshot,
    root_zone: Option<&DirectRootZoneHydraulicConfiguration>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    lse_configuration: &LandSurfaceEnergyConfiguration,
    vegetation_configuration: &VegetationConfiguration,
    vegetation_state: &V9CoupledOwnedState,
    receipt_vegetation_configuration_sha256: String,
    hydrology_snapshot_sha256: Sha256Digest,
    transaction_id: TransactionId,
    day_index: usize,
    interval_index: u8,
) -> Result<(SnowFreeForcing, Option<V10RootZoneReceiptSet>), DirectV9RealConsumerError> {
    let mut forcing = provider.clone();
    // V10 does not consume the legacy global ground scalars. Shortwave optics
    // come from each LSE tile, while reciprocal longwave uses the current
    // coupled ground trial. Canonical zeros prevent caller control and permit
    // heterogeneous tile configurations.
    forcing.ground_albedo_vis = 0.0;
    forcing.ground_albedo_nir = 0.0;
    forcing.longwave_up_w_m2 = 0.0;
    for layer in &mut forcing.soil_layers {
        let water_values = hydrology
            .layer_facts()
            .iter()
            .filter(|(source, _)| source.layer_id == layer.layer_id)
            .map(|(_, fact)| fact.liquid_supply_kg_m2)
            .collect::<Vec<_>>();
        let temperature_values = soil_thermal
            .ofes
            .iter()
            .filter_map(|ofe| {
                ofe.ordered_layers
                    .iter()
                    .find(|candidate| candidate.layer_id == layer.layer_id)
                    .map(|candidate| candidate.temperature_k)
            })
            .collect::<Vec<_>>();
        let water = if root_zone.is_some() {
            water_values
                .first()
                .copied()
                .ok_or(DirectV9RealConsumerError::Identity(
                    "vegetation soil-water projection",
                ))?
        } else {
            common_provider_value(&water_values, "vegetation soil-water projection")?
        };
        let temperature = if root_zone.is_some() {
            temperature_values
                .first()
                .copied()
                .ok_or(DirectV9RealConsumerError::Identity(
                    "vegetation soil-temperature projection",
                ))?
        } else {
            common_provider_value(
                &temperature_values,
                "vegetation soil-temperature projection",
            )?
        };
        layer.water_beginning_kg_m2 = water;
        layer.temperature_k = temperature;
    }
    if let Some(root_zone) = root_zone {
        let mut receipts = Vec::new();
        for occupancy_id in vegetation_state.0.occupancies.keys() {
            let stratum = vegetation_configuration
                .strata
                .iter()
                .find(|value| value.stratum_id == occupancy_id.stratum_id)
                .ok_or(DirectV9RealConsumerError::Identity(
                    "root-zone occupancy/stratum join",
                ))?;
            let geometry = root_zone
                .ordered_strata
                .iter()
                .find(|value| value.stratum_id == stratum.stratum_id)
                .ok_or(DirectV9RealConsumerError::Identity(
                    "root-zone stratum geometry",
                ))?;
            for root in &stratum.root_layers {
                if root.root_fraction == 0.0 {
                    continue;
                }
                for configured in root_zone
                    .ordered_layers
                    .iter()
                    .filter(|value| value.layer_id == root.layer_id)
                {
                    let source = crate::vegetation_real_hydrology_shadow::RealHydrologySourceKey {
                        ofe_lane: crate::vegetation_real_hydrology_shadow::RealHydrologyOfeLaneId {
                            lane_index: configured.production_lane_index,
                            lane_id: configured.production_lane_id,
                        },
                        layer_id: configured.layer_id.clone(),
                    };
                    let fact = hydrology.layer_facts().get(&source).ok_or(
                        DirectV9RealConsumerError::Identity("root-zone live hydrology layer"),
                    )?;
                    let mut top_m = 0.0;
                    for value in root_zone.ordered_layers.iter().take_while(|value| {
                        (
                            value.production_lane_index,
                            value.production_lane_id,
                            &value.layer_id,
                        ) != (
                            configured.production_lane_index,
                            configured.production_lane_id,
                            &configured.layer_id,
                        )
                    }) {
                        if value.production_lane_index == configured.production_lane_index
                            && value.production_lane_id == configured.production_lane_id
                        {
                            let prior = crate::vegetation_real_hydrology_shadow::RealHydrologySourceKey {
                                ofe_lane: crate::vegetation_real_hydrology_shadow::RealHydrologyOfeLaneId {
                                    lane_index: value.production_lane_index,
                                    lane_id: value.production_lane_id,
                                },
                                layer_id: value.layer_id.clone(),
                            };
                            top_m += hydrology
                                .layer_facts()
                                .get(&prior)
                                .ok_or(DirectV9RealConsumerError::Identity(
                                    "root-zone predecessor hydrology layer",
                                ))?
                                .layer_thickness_m;
                        }
                    }
                    let source_values = root_zone_hydraulic_values(
                        fact,
                        configured,
                        top_m,
                        geometry.root_tissue_lateral_path_m,
                    )?;
                    let matching_ofes = surface_configuration
                        .ofe_bindings
                        .iter()
                        .filter(|binding| {
                            binding.production_lane_index == configured.production_lane_index
                                && binding.production_lane_id == configured.production_lane_id
                                && lse_configuration.ofes.iter().any(|ofe| {
                                    ofe.ofe_id == binding.ofe_id
                                        && ofe.tiles.iter().any(|tile| {
                                            tile.vegetation_tile_id == occupancy_id.tile_id
                                        })
                                })
                        })
                        .collect::<Vec<_>>();
                    // Root-zone configuration is complete over production
                    // lanes, while a vegetation occupancy exists only on OFEs
                    // containing that configured topology tile. A snow-free
                    // or open-only lane therefore contributes no receipt for
                    // this occupancy; it is not an identity failure.
                    for ofe in matching_ofes {
                        receipts.push(root_zone_hydraulic_receipt(
                            V10RootZoneReceiptKey {
                                ofe_id: ofe.ofe_id.clone(),
                                production_lane_index: configured.production_lane_index,
                                production_lane_id: configured.production_lane_id,
                                occupancy_id: occupancy_id.clone(),
                                stratum_id: stratum.stratum_id.clone(),
                                layer_id: root.layer_id.clone(),
                            },
                            source_values,
                            root.lateral_root_length_m,
                        )?);
                    }
                }
            }
        }
        return Ok((
            forcing,
            Some(V10RootZoneReceiptSet::try_new(
                root_zone.restart_identity_sha256().map_err(|_| {
                    DirectV9RealConsumerError::Identity("root-zone configuration identity")
                })?,
                lse_configuration
                    .hydrology_configuration
                    .configuration_sha256
                    .clone(),
                receipt_vegetation_configuration_sha256,
                lse_configuration.configuration_sha256.clone(),
                hydrology_snapshot_sha256,
                transaction_id,
                day_index,
                interval_index,
                receipts,
            )?),
        ));
    }
    Ok((forcing, None))
}

include!("v9_real_consumer_shadow_soil_thermal.rs");
#[cfg(test)]
include!("v9_real_consumer_shadow_tests.rs");
