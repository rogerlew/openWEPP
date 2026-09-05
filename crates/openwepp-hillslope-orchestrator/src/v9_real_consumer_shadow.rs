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
    ValidatedV11ParentFinalizationV1, VegetationConfigurationV11,
    project_v11_parent_finalization_to_v10,
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
    execute_v8_lse_runtime_shadow_v11_with_native_soil_beginning,
    unified_beginning_hydrology_snapshot_sha256,
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
    begin_adaptive_parent_fixed_point_phase_v1 as profile_start,
    reconstruct_precipitation_mass_and_advected_heat,
    record_adaptive_parent_profile_detail_v1 as profile_record,
    validate_precipitation_phase_parcel_set,
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
pub(crate) use frozen_litter_v4_adoption::{
    CoveredNativePhysicalPathAuditV1, begin_covered_native_physical_path_audit_v1,
    record_native_surface_ingress_entry_v1, record_native_surface_resource_entry_v1,
    record_native_wb14_physics_entry_v1, take_covered_native_physical_path_audit_v1,
};
#[cfg(test)]
pub(crate) use v11_covered::audit_covered_carrier_support;
pub(crate) use v11_covered::normalize_v11_staged_parent_lineage;
pub(crate) use v11_covered::physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1;
pub(crate) use v11_covered::physical_outcome_ledger::ledger_set_digest as stage3_physical_outcome_ledger_set_digest;
pub use v11_covered::physical_outcome_ledger::{
    Stage3PhysicalOutcomeClosureAuditV1, begin_stage3_physical_outcome_closure_audit_v1,
    take_stage3_physical_outcome_closure_audit_v1,
};
#[cfg(test)]
pub(crate) use v11_covered::{
    CanonicalCoveredFinalConstructorStageV1, CanonicalCoveredPhysicalParityPoisonV1,
    canonical_covered_final_constructor_boundary_v1,
    canonical_covered_final_validation_boundary_v1, canonical_covered_parity_poison_v1,
    record_canonical_covered_accepted_parent_adoption_v1,
    record_canonical_covered_successful_history_append_v1,
};
#[cfg(test)]
pub(crate) use v11_covered::{
    CoveredProvisionalPhysicalAuditV1, begin_covered_provisional_physical_audit_v1,
    begin_v50_outer_owner_transition_evidence_v1, force_covered_full_provisional_envelope_for_test,
    take_covered_provisional_physical_audit_v1, take_v50_outer_owner_transition_evidence_v1,
};

pub(crate) use v11_covered::CoveredCarrierEphemeralCandidatesV1;
pub(crate) use v11_covered::CoveredCarrierPhaseResultV1;
pub(crate) use v11_covered::CoveredPhysicalCustodyJoinInputs;
pub(crate) use v11_covered::DeferredNativeV2SoilCustodyV1;
pub(crate) use v11_covered::covered_carrier_initial_owner_bytes_with_deferred_native_v2_soil_custody_v1;
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
    /// Native successor owner set for snow-free forest litter. Retained V1/V2
    /// checkpoints leave this absent; the V3 execution path requires it.
    frozen_litter_v3: Option<FrozenLitterV3Resident>,
    /// Mandatory exact surface-enthalpy successor. When present, the parallel
    /// V3 resident is a nonauthoritative physical/high-mirror owner only.
    frozen_litter_v4: Option<FrozenLitterV4Resident>,
}

include!("v9_real_consumer_shadow_equilibrium_fixture.rs");

include!("v9_real_consumer_shadow_publication_retention.rs");
include!("v9_real_consumer_shadow_root_zone_configuration.rs");
#[path = "v9_real_consumer_shadow/accepted_publication_support_capability.rs"]
mod accepted_publication_support_capability;
use accepted_publication_support_capability::{
    AcceptedPublicationHistoryLiveRevisionV1, PreparedStage3AcceptedPublicationSupportV1,
    ValidatedStage3AcceptedPublicationSupportV1,
};
#[cfg(test)]
pub(crate) use accepted_publication_support_capability::{
    AcceptedPublicationLiveRevisionPoisonV1, AcceptedPublicationSupportCapabilityAuditV1,
    begin_accepted_publication_support_capability_audit_v1,
    take_accepted_publication_support_capability_audit_v1,
};
#[path = "v9_real_consumer_shadow/frozen_litter_v3_adoption.rs"]
mod frozen_litter_v3_adoption;
pub use frozen_litter_v3_adoption::FrozenLitterV3Resident;
pub(crate) use frozen_litter_v3_adoption::ValidatedV9ToV8ProjectionV1;
#[path = "v9_real_consumer_shadow/frozen_litter_v4_adoption.rs"]
mod frozen_litter_v4_adoption;
pub use frozen_litter_v4_adoption::FrozenLitterV4Resident;
#[path = "v9_real_consumer_shadow/frozen_litter_v3_publication_retention.rs"]
mod frozen_litter_v3_publication_retention;
#[path = "v9_real_consumer_shadow/snow_free_physical_reuse.rs"]
mod snow_free_physical_reuse;
#[cfg(any(test, feature = "restart-authority-evidence"))]
pub(crate) use frozen_litter_v3_publication_retention::FrozenLitterV3PublicationSupportV1;
#[cfg(any(test, feature = "persisted-restart-v1"))]
pub(crate) use snow_free_physical_reuse::record_snow_free_outer_accepted_publication_v1;
pub(crate) use snow_free_physical_reuse::{
    SnowFreePhysicalReusePendingV1, SnowFreePhysicalReuseSeedV1, prepare_snow_free_physical_reuse,
};
#[cfg(test)]
pub(crate) use snow_free_physical_reuse::{
    record_snow_free_ingress_operation_v1, record_snow_free_phase_operation_v1,
    record_snow_free_provider_projection_v1, record_snow_free_routing_operation_v1,
    record_snow_free_vapor_operation_v1, record_snow_free_wb14_operations_v1,
};
#[cfg(feature = "persisted-restart-v1")]
pub use snow_free_physical_reuse::{
    restart_authority_execute_fresh_snow_free_segment_v1,
    restart_authority_v11_parent_owner_envelopes_v1,
};
#[cfg(test)]
#[path = "v9_real_consumer_shadow/accepted_publication_support_capability_tests.rs"]
mod accepted_publication_support_capability_tests;
#[cfg(test)]
#[path = "v9_real_consumer_shadow/v3_publication_retention_tests.rs"]
mod v3_publication_retention_tests;

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
    LseV3(#[from] openwepp_land_surface_energy::LseV3StateError),
    #[error(transparent)]
    SurfaceLiquidV2(#[from] crate::DirectSurfaceLiquidError),
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
            Self::LseV3(_) => "land_surface_energy_v3",
            Self::SurfaceLiquidV2(_) => "surface_liquid_v2",
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

pub(crate) fn v11_soil_thermal_owner_envelope(
    owner: &DirectSoilThermalResident,
) -> Result<V11OwnerEnvelope, DirectV11RealConsumerError> {
    V11OwnerEnvelope::try_new(
        "soil_thermal".to_owned(),
        owner
            .canonical_active_owner_bytes()
            .map_err(DirectV10RealConsumerError::Runtime)?,
    )
    .map_err(Into::into)
}

/// Select the LSE beginning that exactly owns the staged V11 bytes.
///
/// Frozen-litter V3/V4 retains the legacy LSE state only as an inner
/// compatibility projection. Support receipts must bind the native V3 owner
/// whenever that owner occupies the staged complete-owner set.
pub(crate) fn v11_support_lse_beginning<'a>(
    beginning: &'a DirectV10RealConsumerShadow,
    staged_lse_bytes: &[u8],
) -> Result<
    (
        &'a LandSurfaceEnergyConfiguration,
        &'a LandSurfaceEnergyState,
    ),
    DirectV11RealConsumerError,
> {
    let inner_lse_bytes = serde_json::to_vec(&beginning.inner.lse_state).map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
            DirectV9RealConsumerError::Serialization(error.to_string()),
        ))
    })?;
    let resident_bytes = beginning
        .frozen_litter_v3
        .as_ref()
        .map(|resident| {
            serde_json::to_vec(resident.lse_state()).map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                    DirectV9RealConsumerError::Serialization(error.to_string()),
                ))
            })
        })
        .transpose()?;
    let exact_resident_bytes =
        beginning
            .frozen_litter_v4
            .as_ref()
            .map(|exact| {
                let physical = beginning.frozen_litter_v3.as_ref().ok_or(
                    DirectV11RealConsumerError::Identity(
                        "native V4 support selection requires physical resident",
                    ),
                )?;
                exact
                    .v11_complete_lse_owner_bytes(physical)
                    .map_err(DirectV11RealConsumerError::Runtime)
            })
            .transpose()?;
    match select_v11_support_lse_bytes(
        staged_lse_bytes,
        &inner_lse_bytes,
        resident_bytes.as_deref(),
        exact_resident_bytes.as_deref(),
        beginning.inner.authority == CoveredColumnAuthority::V11SnowCovered,
    )? {
        V11SupportLseSelection::Legacy => Ok((
            &beginning.inner.lse_configuration,
            &beginning.inner.lse_state,
        )),
        V11SupportLseSelection::NativeV3 | V11SupportLseSelection::NativeV4 => {
            let resident =
                beginning
                    .frozen_litter_v3
                    .as_ref()
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "native V3 support selection requires resident",
                    ))?;
            Ok((resident.lse_configuration(), &resident.lse_state().0))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum V11SupportLseSelection {
    Legacy,
    NativeV3,
    NativeV4,
}

fn select_v11_support_lse_bytes(
    staged_lse_bytes: &[u8],
    legacy_lse_bytes: &[u8],
    native_v3_lse_bytes: Option<&[u8]>,
    native_v4_lse_bytes: Option<&[u8]>,
    allow_covered_inner_with_inactive_native_v4: bool,
) -> Result<V11SupportLseSelection, DirectV11RealConsumerError> {
    if allow_covered_inner_with_inactive_native_v4 && staged_lse_bytes == legacy_lse_bytes {
        return Ok(V11SupportLseSelection::Legacy);
    }
    if let Some(native_v4) = native_v4_lse_bytes {
        return (staged_lse_bytes == native_v4)
            .then_some(V11SupportLseSelection::NativeV4)
            .ok_or(DirectV11RealConsumerError::Identity(
                "staged LSE owner does not match the exact native V4 resident",
            ));
    }
    if staged_lse_bytes == legacy_lse_bytes {
        return Ok(V11SupportLseSelection::Legacy);
    }
    if native_v3_lse_bytes.is_some_and(|native| staged_lse_bytes == native) {
        return Ok(V11SupportLseSelection::NativeV3);
    }
    Err(DirectV11RealConsumerError::Identity(
        if native_v3_lse_bytes.is_some() {
            "staged LSE owner is neither legacy beginning nor native V3 resident"
        } else {
            "staged LSE owner does not match the resident beginning"
        },
    ))
}

#[cfg(test)]
mod covered_support_receipt_selection_tests {
    use super::*;

    #[test]
    fn covered_receipt_selects_exact_legacy_staged_bytes() {
        assert_eq!(
            select_v11_support_lse_bytes(b"legacy", b"legacy", Some(b"native"), None, false)
                .expect("legacy bytes"),
            V11SupportLseSelection::Legacy,
        );
    }

    #[test]
    fn covered_receipt_selects_exact_native_v3_staged_bytes() {
        assert_eq!(
            select_v11_support_lse_bytes(b"native", b"legacy", Some(b"native"), None, false)
                .expect("native V3 bytes"),
            V11SupportLseSelection::NativeV3,
        );
    }

    #[test]
    fn covered_receipt_selects_exact_native_v4_staged_bytes() {
        assert_eq!(
            select_v11_support_lse_bytes(
                b"native-v4",
                b"legacy",
                Some(b"native-v3"),
                Some(b"native-v4"),
                false,
            )
            .expect("native V4 bytes"),
            V11SupportLseSelection::NativeV4,
        );
    }

    #[test]
    fn covered_receipt_refuses_legacy_or_v3_downgrade_after_v4_adoption() {
        for staged in [b"legacy".as_slice(), b"native-v3".as_slice()] {
            assert!(
                select_v11_support_lse_bytes(
                    staged,
                    b"legacy",
                    Some(b"native-v3"),
                    Some(b"native-v4"),
                    false,
                )
                .is_err()
            );
        }
    }

    #[test]
    fn represented_snow_selects_active_inner_with_inactive_native_v4() {
        assert_eq!(
            select_v11_support_lse_bytes(
                b"covered-inner",
                b"covered-inner",
                Some(b"native-v3"),
                Some(b"native-v4"),
                true,
            )
            .expect("represented-snow active inner LSE"),
            V11SupportLseSelection::Legacy,
        );
    }

    #[test]
    fn covered_receipt_rejects_mismatched_or_missing_native_owner() {
        for native in [None, Some(b"native".as_slice())] {
            assert!(
                select_v11_support_lse_bytes(b"poison", b"legacy", native, None, false).is_err()
            );
        }
    }

    #[test]
    fn covered_receipt_mismatch_is_exact_rollback_with_no_publication() {
        let staged = b"poison".to_vec();
        let legacy = b"legacy".to_vec();
        let native = b"native".to_vec();
        let before = (
            staged.clone(),
            legacy.clone(),
            native.clone(),
            Vec::<Vec<u8>>::new(),
        );
        assert!(
            select_v11_support_lse_bytes(&staged, &legacy, Some(&native), None, false).is_err(),
            "mismatch must fail before receipt construction or publication",
        );
        assert_eq!(
            (staged, legacy, native, Vec::<Vec<u8>>::new()),
            before,
            "pure selection failure preserves all owner bytes and publishes nothing",
        );
    }
}

struct ImportedStackProfileScopeV1 {
    phase: &'static str,
    started: Option<std::time::Instant>,
}

impl ImportedStackProfileScopeV1 {
    fn begin(phase: &'static str) -> Self {
        Self {
            phase,
            started: profile_start(),
        }
    }
}

impl Drop for ImportedStackProfileScopeV1 {
    fn drop(&mut self) {
        profile_record(self.phase, self.started.take());
    }
}

impl crate::v11_vegetation_consumer::DirectV11ImportedStack for DirectV11RealConsumerStack<'_> {
    type Error = DirectV11RealConsumerError;

    #[allow(clippy::too_many_lines)]
    fn execute_imported_v10_stack(
        &mut self,
        input: &V11ImportedV10SegmentInput,
    ) -> Result<V11ImportedV10SegmentOutput, Self::Error> {
        if self.snow_free_physical_reuse_pending.is_some()
            || self.snow_free_physical_reuse_seed.is_some()
        {
            return self.execute_snow_free_physical_reuse(input);
        }
        #[cfg(any(test, feature = "persisted-restart-v1"))]
        snow_free_physical_reuse::record_snow_free_physical_execution_v1();
        let entry_profile = ImportedStackProfileScopeV1::begin("imported entry validation");
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
        // V3 imports the unchanged V1 support-admission receipt schema and
        // policy, but its identity fields must bind the native successor
        // configuration/state published in the staged complete-owner set.
        // The legacy/V2 paths continue to bind their existing inner owner.
        let staged_lse_bytes = &input
            .staged_resource_owners
            .get("land_surface_energy")
            .ok_or(DirectV11RealConsumerError::Identity(
                "missing staged LSE owner",
            ))?
            .state_bytes;
        let (support_configuration, support_beginning) =
            v11_support_lse_beginning(&self.beginning, staged_lse_bytes)?;
        let support_receipt = LseSupportAdmissibilityReceiptV1::admit(
            support_configuration,
            support_beginning,
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
        drop(entry_profile);
        let physical_candidate_profile =
            ImportedStackProfileScopeV1::begin("imported physical candidate");
        let physical_setup_profile = ImportedStackProfileScopeV1::begin("imported physical setup");
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
        match self.native_inactive_wb14_prefix {
            Some(prefix) => candidate
                .stage_frozen_litter_wb14_parent_after_native_inactive_prefix_v1(
                    self.day_index,
                    self.interval_index,
                    self.interval,
                    self.wb14_coupled_child_binding
                        .ok_or(DirectV11RealConsumerError::Identity(
                            "native inactive prefix coupled-child binding",
                        ))?,
                    prefix,
                ),
            None => candidate.stage_frozen_litter_wb14_parent_from_inner_v1(),
        }
        .map_err(DirectV11RealConsumerError::Runtime)?;
        let deferred_v4_soil_candidate = self
            .deferred_native_v2_soil_custody
            .as_ref()
            .map(DeferredNativeV2SoilCustodyV1::candidate);
        let deferred_v4_soil_continuation = self
            .deferred_native_v2_soil_custody
            .as_ref()
            .and_then(DeferredNativeV2SoilCustodyV1::continuation);
        drop(physical_setup_profile);
        let frozen_evaluation_profile =
            ImportedStackProfileScopeV1::begin("imported frozen evaluation");
        let mut frozen_litter_v4 = if candidate.frozen_litter_v4.is_some() {
            let binding =
                self.wb14_coupled_child_binding
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "native frozen-litter V4 requires coupled-child binding",
                    ))?;
            let frozen_preparation_profile =
                ImportedStackProfileScopeV1::begin("imported frozen preparation");
            let fixed = candidate.prepare_frozen_litter_v3_fixed_final(
                self.day_index,
                self.interval_index,
                self.interval,
                input.duration_s_bits,
                self.finalize_wb14_parent_interval,
                binding,
                deferred_v4_soil_candidate,
                deferred_v4_soil_continuation,
            )?;
            drop(frozen_preparation_profile);
            let frozen_execute_accept_profile =
                ImportedStackProfileScopeV1::begin("imported frozen execute accept");
            let accepted = candidate.execute_and_accept_frozen_litter_v4(
                &fixed,
                input.support.start_ns().get(),
                input.support.end_ns().get(),
                self.finalize_wb14_parent_interval,
                binding,
                deferred_v4_soil_continuation,
            )?;
            drop(frozen_execute_accept_profile);
            Some((fixed, accepted))
        } else {
            None
        };
        let frozen_litter_v3 = if frozen_litter_v4.is_none() && candidate.frozen_litter_v3.is_some()
        {
            let binding =
                self.wb14_coupled_child_binding
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "native frozen-litter V3 requires coupled-child binding",
                    ))?;
            let frozen_preparation_profile =
                ImportedStackProfileScopeV1::begin("imported frozen preparation");
            let fixed = candidate.prepare_frozen_litter_v3_fixed_final(
                self.day_index,
                self.interval_index,
                self.interval,
                input.duration_s_bits,
                self.finalize_wb14_parent_interval,
                binding,
                None,
                None,
            )?;
            drop(frozen_preparation_profile);
            let frozen_execute_accept_profile =
                ImportedStackProfileScopeV1::begin("imported frozen execute accept");
            let accepted = candidate.execute_and_accept_frozen_litter_v3(
                &fixed,
                input.support.start_ns().get(),
                input.support.end_ns().get(),
                self.finalize_wb14_parent_interval,
                binding,
            )?;
            drop(frozen_execute_accept_profile);
            Some((fixed, accepted))
        } else {
            None
        };
        drop(frozen_evaluation_profile);
        let envelope_construction_profile =
            ImportedStackProfileScopeV1::begin("imported envelope construction");
        let deferred_soil = self.deferred_native_v2_soil_custody.as_ref();
        let envelope = if let Some((fixed, accepted)) = frozen_litter_v4.as_ref() {
            candidate.construct_frozen_litter_v3_complete_envelope(
                self.day_index,
                input.duration_s_bits,
                fixed,
                &accepted.physical,
                true,
            )?
        } else if let Some((fixed, accepted)) = frozen_litter_v3.as_ref() {
            if deferred_soil.is_some() {
                return Err(DirectV11RealConsumerError::Identity(
                    "native frozen-litter V3 deferred soil custody is not yet joined",
                ));
            }
            candidate.construct_frozen_litter_v3_complete_envelope(
                self.day_index,
                input.duration_s_bits,
                fixed,
                accepted,
                false,
            )?
        } else {
            candidate
                .inner
                .construct_snow_free_parent_child_envelope_with_duration_and_soil_beginning(
                    self.day_index,
                    self.interval_index,
                    self.interval,
                    f64::from_bits(input.duration_s_bits),
                    Some(input.duration_s_bits),
                    self.finalize_wb14_parent_interval,
                    self.wb14_coupled_child_binding,
                    deferred_soil.map(DeferredNativeV2SoilCustodyV1::candidate),
                    deferred_soil.and_then(DeferredNativeV2SoilCustodyV1::continuation),
                )
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })?
        };
        drop(envelope_construction_profile);
        let envelope_validation_profile =
            ImportedStackProfileScopeV1::begin("imported envelope validation");
        envelope.validate().map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
        })?;
        drop(envelope_validation_profile);
        drop(physical_candidate_profile);
        let accepted_candidate_profile =
            ImportedStackProfileScopeV1::begin("imported accepted candidate");

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

        let completed_deferred_soil = deferred_soil
            .map(|custody| custody.advance_snow_free_child(&self.beginning, input, &envelope))
            .transpose()?;
        if let Some(custody) = completed_deferred_soil.as_ref() {
            let continuation =
                custody
                    .continuation()
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "deferred native V2 soil final continuation",
                    ))?;
            let accepted = continuation
                .compose_accepted_outer_candidate(&self.beginning.inner.lse_configuration)
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })?;
            let seals = seal_soil_thermal_accepted_candidate_v2(
                continuation.original_prepared().beginning_owner(),
                &accepted,
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
            })?;
            candidate
                .validate_soil_thermal_accepted_v2_from_unpublished_continuation(
                    continuation.physical_trial(),
                    continuation,
                    continuation.original_prepared().beginning_owner(),
                    &accepted,
                )
                .map_err(DirectV11RealConsumerError::Runtime)?;
            candidate
                .inner
                .accept_envelope_preserving_native_v2_soil(envelope.transaction_id(), &envelope)
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
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(
                        error,
                    ))
                })?,
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LseV2(error))
            })?;
            let authoritative_complete_envelope = candidate.clone();
            let transaction_authority = candidate
                .authenticate_soil_thermal_unpublished_continuation_install_authority_v3(
                    &authoritative_complete_envelope,
                    continuation,
                    continuation.original_prepared().beginning_owner(),
                )
                .map_err(DirectV11RealConsumerError::Runtime)?;
            candidate
                .install_soil_thermal_accepted_v2_from_unpublished_continuation_v3(
                    &authoritative_complete_envelope,
                    continuation,
                    continuation.original_prepared().beginning_owner(),
                    transaction_authority,
                    accepted,
                    seals,
                )
                .map_err(DirectV11RealConsumerError::Runtime)?;
            if let Some((_, accepted)) = frozen_litter_v4.as_mut() {
                candidate
                    .accept_promoted_candidate_only_frozen_litter_v4(
                        &self.beginning,
                        continuation.original_prepared().beginning_owner(),
                        accepted,
                    )
                    .map_err(DirectV11RealConsumerError::Runtime)?;
            }
        } else if let Some((_, accepted)) = frozen_litter_v4.as_ref() {
            candidate
                .accept_frozen_litter_v3_complete_envelope(
                    &self.beginning,
                    input.support.start_ns().get(),
                    input.support.end_ns().get(),
                    &accepted.physical,
                    &envelope,
                )
                .map_err(DirectV11RealConsumerError::Runtime)?;
        } else if frozen_litter_v3.is_some() {
            candidate
                .accept_frozen_litter_v3_complete_envelope(
                    &self.beginning,
                    input.support.start_ns().get(),
                    input.support.end_ns().get(),
                    frozen_litter_v3
                        .as_ref()
                        .map(|(_, accepted)| accepted)
                        .ok_or(DirectV11RealConsumerError::Identity(
                            "missing accepted frozen-litter V3 candidate",
                        ))?,
                    &envelope,
                )
                .map_err(DirectV11RealConsumerError::Runtime)?;
        } else {
            candidate
                .inner
                .accept_envelope(envelope.transaction_id(), &envelope)
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })?;
        }
        if frozen_litter_v4.is_none() && frozen_litter_v3.is_none() {
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
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(
                        error,
                    ))
                })?,
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LseV2(error))
            })?;
        }
        drop(accepted_candidate_profile);
        let owner_publication_profile =
            ImportedStackProfileScopeV1::begin("imported owner publication");

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
        let surface_bytes = if let Some(resident) = candidate.frozen_litter_v3.as_ref() {
            resident
                .surface_owner()
                .canonical_bytes(
                    resident.surface_configuration().parent(),
                    Some(resident.surface_configuration()),
                )
                .map_err(DirectV10RealConsumerError::SurfaceLiquidV2)?
        } else {
            let surface = candidate
                .inner
                .hydrology_frame
                .surface_liquid_shadow
                .as_ref()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "missing staged surface-liquid owner",
                ))?;
            surface
                .canonical_bytes(&candidate.inner.surface_configuration)
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                        DirectV9RealConsumerError::Serialization(error.to_string()),
                    ))
                })?
        };
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
        let ending_resource_owners: BTreeMap<String, V11OwnerEnvelope> = [
            ("snow".to_owned(), snow),
            (
                "land_surface_energy".to_owned(),
                if let Some(exact) = candidate.frozen_litter_v4.as_ref() {
                    let physical = candidate.frozen_litter_v3.as_ref().ok_or(
                        DirectV11RealConsumerError::Identity(
                            "native V4 ending requires physical resident",
                        ),
                    )?;
                    V11OwnerEnvelope::try_new(
                        "land_surface_energy".to_owned(),
                        exact
                            .v11_complete_lse_owner_bytes(physical)
                            .map_err(DirectV11RealConsumerError::Runtime)?,
                    )?
                } else if let Some(resident) = candidate.frozen_litter_v3.as_ref() {
                    v11_owner_envelope("land_surface_energy", resident.lse_state())?
                } else {
                    v11_owner_envelope("land_surface_energy", &candidate.inner.lse_state)?
                },
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
                v11_soil_thermal_owner_envelope(&candidate.inner.soil_thermal)?,
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
        let prepared_publication_support = candidate
            .prepare_unvalidated_accepted_publication_support(
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
        let deferred_publication_support = if self.wb14_coupled_child_binding.is_some() {
            Some(prepared_publication_support)
        } else {
            let capability = prepared_publication_support
                .validate_and_mint(candidate.accepted_publication_history.live_revision_v1())?;
            candidate.push_validated_accepted_publication_support(capability)?;
            None
        };

        if frozen_litter_v3.is_some()
            && candidate
                .frozen_litter_v3
                .as_ref()
                .is_none_or(|resident| resident.accepted_publication_count() == 0)
        {
            return Err(DirectV11RealConsumerError::Identity(
                "missing accepted frozen-litter V3 publication",
            ));
        }
        if frozen_litter_v4.is_some()
            && candidate.frozen_litter_v4.as_ref().is_none_or(|resident| {
                resident
                    .accepted_publication_supports_canonical_bytes()
                    .is_empty()
            })
        {
            return Err(DirectV11RealConsumerError::Identity(
                "missing accepted frozen-litter V4 publication",
            ));
        }

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
        #[cfg(test)]
        {
            self.last_hydrology_candidate = Some(envelope.hydrology().clone());
        }
        drop(owner_publication_profile);
        let install_profile = ImportedStackProfileScopeV1::begin("imported install");
        let physical_reuse_pending = match self.wb14_coupled_child_binding {
            Some(provisional_binding) => Some(SnowFreePhysicalReusePendingV1::stage(
                input,
                provisional_binding,
                deferred_publication_support.ok_or(DirectV11RealConsumerError::Identity(
                    "deferred accepted publication prepared support",
                ))?,
                candidate.clone(),
                output.clone(),
                &self.beginning,
                self.interval,
                self.day_index,
                self.interval_index,
                self.finalize_wb14_parent_interval,
                self.native_inactive_wb14_prefix,
                self.ending_snow_owner_bytes.clone(),
                self.deferred_native_v2_soil_custody.clone(),
                self.ending.clone(),
                self.last_support_receipt.clone(),
            )),
            None => None,
        };
        self.last_support_receipt = Some(support_receipt);
        self.ending = Some(candidate);
        self.snow_free_physical_reuse_pending = physical_reuse_pending;
        drop(install_profile);
        Ok(output)
    }

    fn authenticate_outer_validated_candidate(
        &mut self,
        candidate: &openwepp_vegetation::v11::V11AcceptedSegmentCandidate,
    ) -> Result<(), Self::Error> {
        self.authenticate_snow_free_outer_candidate(candidate)
    }
}

#[cfg(test)]
mod native_v4_deferred_soil_join_source_guards {
    #[test]
    fn v4_uses_the_authenticated_deferred_join_and_v3_remains_refused() {
        let source = include_str!("v9_real_consumer_shadow.rs");
        let body = source
            .split("let deferred_soil = self.deferred_native_v2_soil_custody.as_ref();")
            .nth(1)
            .expect("deferred native-V2 soil branch")
            .split("let segment_ending = candidate.vegetation_state.clone();")
            .next()
            .expect("snow-free acceptance body");
        assert!(!body.contains("native frozen-litter V4 deferred soil custody is not yet joined"));
        assert!(body.contains("native frozen-litter V3 deferred soil custody is not yet joined"));

        let replay = body
            .find("validate_soil_thermal_accepted_v2_from_unpublished_continuation")
            .expect("continuation replay validation");
        let accept = body[replay..]
            .find("accept_envelope_preserving_native_v2_soil")
            .map(|offset| replay + offset)
            .expect("non-soil envelope acceptance");
        let vegetation = body[accept..]
            .find("project_v9_runtime_to_v10")
            .map(|offset| accept + offset)
            .expect("V10 vegetation projection");
        let lse = body[vegetation..]
            .find("project_validated_v1_runtime_to_v2")
            .map(|offset| vegetation + offset)
            .expect("V10 LSE projection");
        let authenticate = body[lse..]
            .find("authenticate_soil_thermal_unpublished_continuation_install_authority_v3")
            .map(|offset| lse + offset)
            .expect("three-domain install authentication");
        let install = body[authenticate..]
            .find("install_soil_thermal_accepted_v2_from_unpublished_continuation_v3")
            .map(|offset| authenticate + offset)
            .expect("three-domain continuation install");
        assert!(replay < accept && accept < vegetation && vegetation < lse);
        assert!(lse < authenticate && authenticate < install);
        assert!(
            !body.contains(
                "authenticate_soil_thermal_unpublished_continuation_install_authority_v2("
            )
        );
        assert!(!body.contains("install_soil_thermal_accepted_v2_from_unpublished_continuation("));
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
    pub(crate) const fn surface_configuration(&self) -> &DirectSurfaceLiquidConfiguration {
        self.inner.surface_configuration()
    }

    pub(crate) fn validate_pending_v11_parent_finalization_source_v1(
        &self,
        configuration: &VegetationConfigurationV11,
        finalized: &V11CoupledOwnedState,
    ) -> Result<TransactionId, DirectV11RealConsumerError> {
        let predecessor = self.vegetation_state.0.last_transaction_id;
        let source = TransactionId(finalized.last_parent_transaction_id);
        if source.0 == 0
            || source.0 == predecessor
            || self.lse_state.0.last_accepted_transaction_id != Some(source)
            || self.inner.biogeochemistry.last_transaction_id != source.0
        {
            return Err(DirectV11RealConsumerError::Identity(
                "pending V11 parent-finalization record source",
            ));
        }

        let mut projected = self.clone();
        projected.accept_v11_parent_finalization(configuration, finalized)?;
        normalize_v8_parent_lineage(&mut projected.vegetation_state.0, predecessor);
        normalize_v8_parent_lineage(&mut projected.inner.vegetation_state.0, predecessor);
        if projected.vegetation_state != self.vegetation_state
            || projected.inner.vegetation_state != self.inner.vegetation_state
            || projected.inner.biogeochemistry != self.inner.biogeochemistry
        {
            return Err(DirectV11RealConsumerError::Identity(
                "pending V11 parent-finalization lineage-only projection",
            ));
        }
        Ok(source)
    }

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
        let (v9_configuration, v9_state) =
            project_v10_runtime_to_v9(&v10_configuration, &v10_state).map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::V10(error))
            })?;
        self.accept_projected_v11_parent_finalization(
            configuration,
            finalized,
            v10_configuration,
            v10_state,
            v9_configuration,
            v9_state,
        )
    }

    pub(crate) fn accept_v11_parent_finalization_with_validated_handoff(
        &mut self,
        configuration: &VegetationConfigurationV11,
        finalized: &V11CoupledOwnedState,
        handoff: ValidatedV11ParentFinalizationV1,
    ) -> Result<V11OwnerEnvelope, DirectV11RealConsumerError> {
        let (v10_configuration, v10_state, v9_configuration, v9_state) =
            handoff.project_to_v10(configuration, &self.vegetation_state, finalized)?;
        self.accept_projected_v11_parent_finalization(
            configuration,
            finalized,
            v10_configuration,
            v10_state,
            v9_configuration,
            v9_state,
        )
    }

    fn accept_projected_v11_parent_finalization(
        &mut self,
        _configuration: &VegetationConfigurationV11,
        finalized: &V11CoupledOwnedState,
        v10_configuration: VegetationConfiguration,
        v10_state: V10CoupledOwnedState,
        v9_configuration: VegetationConfiguration,
        v9_state: V9CoupledOwnedState,
    ) -> Result<V11OwnerEnvelope, DirectV11RealConsumerError> {
        if v10_configuration != self.vegetation_configuration {
            return Err(DirectV11RealConsumerError::Identity(
                "V11 parent finalization V10 configuration",
            ));
        }
        if v9_configuration != self.inner.vegetation_configuration {
            return Err(DirectV11RealConsumerError::Identity(
                "V11 parent finalization V9 configuration",
            ));
        }
        let prior_transaction = self.vegetation_state.0.last_transaction_id;
        let expected_transaction =
            prior_transaction
                .checked_add(1)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "V11 parent finalization transaction overflow",
                ))?;
        if finalized.last_parent_transaction_id != expected_transaction {
            return Err(DirectV11RealConsumerError::Identity(
                if finalized.last_parent_transaction_id <= prior_transaction {
                    "V11 parent finalization nonadvancing vegetation predecessor"
                } else {
                    "V11 parent finalization skipped vegetation predecessor"
                },
            ));
        }
        if !matches!(
            self.inner.biogeochemistry.last_transaction_id,
            value if value == prior_transaction || value == finalized.last_parent_transaction_id
        ) {
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
    pub(crate) fn prepare_accepted_publication_support(
        &self,
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
    ) -> Result<ValidatedStage3AcceptedPublicationSupportV1, DirectV11RealConsumerError> {
        Stage3AcceptedPublicationSupportV1::try_new(
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
            self.accepted_publication_history.live_revision_v1(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn prepare_unvalidated_accepted_publication_support(
        &self,
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
    ) -> Result<PreparedStage3AcceptedPublicationSupportV1, DirectV11RealConsumerError> {
        #[cfg(test)]
        v11_covered::record_canonical_covered_publication_retain_entry_v1();
        #[cfg(test)]
        let ending_complete_owner_set_sha256 = if matches!(
            v11_covered::canonical_covered_parity_poison_v1(),
            Some(v11_covered::CanonicalCoveredPhysicalParityPoisonV1::PublicationSupport)
        ) {
            Digest32::zero()
        } else {
            ending_complete_owner_set_sha256
        };
        Stage3AcceptedPublicationSupportV1::prepare(
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
        )
    }

    pub(crate) fn push_validated_accepted_publication_support(
        &mut self,
        capability: ValidatedStage3AcceptedPublicationSupportV1,
    ) -> Result<(), DirectV11RealConsumerError> {
        self.accepted_publication_history
            .push_validated_support(capability)
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

    #[cfg(test)]
    fn restore_accepted_publication_authority(
        &mut self,
        supports: Vec<Stage3AcceptedPublicationSupportV1>,
        event_handoffs: Vec<AcceptedEventReceiptV1>,
    ) -> Result<(), DirectV11RealConsumerError> {
        let mut restored = AcceptedPublicationHistoryV1::default();
        restored.replace(supports, &event_handoffs)?;
        self.accepted_publication_history = restored;
        Ok(())
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
            frozen_litter_v3: None,
            frozen_litter_v4: None,
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
        let mut restored_history = AcceptedPublicationHistoryV1::default();
        restored_history
            .replace(supports, &wire.event_handoffs)
            .map_err(|_| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                    "accepted publication authority wire chronology",
                ))
            })?;
        let traversed = restored_history
            .inner
            .tail_authority
            .traversed_ending_owner_sha256;
        if wire.schema_version != 2
            || canonical != bytes
            || wire.receipt_sha256 != digest_bytes(&preimage)
            || wire.traversed_ending_complete_owner_set_sha256 != traversed
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("accepted publication support wire seal"),
            ));
        }
        self.accepted_publication_history = restored_history;
        Ok(())
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

include!("v9_real_consumer_shadow/direct_v9_real_consumer_shadow_impl.rs");

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

include!("v9_real_consumer_shadow/live_vegetation_forcing.rs");

include!("v9_real_consumer_shadow_soil_thermal.rs");
#[cfg(test)]
include!("v9_real_consumer_shadow_tests.rs");
