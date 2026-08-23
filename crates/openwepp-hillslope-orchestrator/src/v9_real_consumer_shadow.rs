//! Explicit default-off V9/LSE consumer over the real direct scheduler owner.
//!
//! This module owns only isolated shadow state. It has no production commit,
//! selector, publication, or output API.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use openwepp_biogeochemistry::{BiogeochemistryError, BiogeochemistryState, available_by_key};
use openwepp_coupled_time::{Digest32, digest_bytes};
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
use openwepp_meteorology::psychrometrics::saturation_vapor_pressure_ice_kpa;
use openwepp_meteorology::snow_free_forcing::{
    celsius_to_kelvin, kilopascals_to_pascals, liquid_specific_enthalpy_j_kg,
};
use openwepp_plant_phenology::{GsiParameters, GsiState};
use openwepp_unit_boundary::TemperatureCelsius;
use openwepp_vegetation::energy::{
    canopy_surface_friction_velocity, leaf_boundary_conductance, neutral_resistance,
};
use openwepp_vegetation::v11::{
    V11AdmittedResourceFlux, V11ImportedV10SegmentInput, V11ImportedV10SegmentOutput,
    V11LseSupportReceiptEnvelope, V11OwnerEnvelope, V11ResourceDebit, V11ResourceKey,
    V11SharedResourceKey, V11SharedResourceKind, V11SharedResourceOwnerTransition,
};
use openwepp_vegetation::{
    NitrogenArbiter, NitrogenAuthorization, NitrogenRequest, SnowFreeForcing, V8CoupledOwnedState,
    V9CoupledOwnedState, V9StateError, V10CoupledOwnedState, V10StateError,
    VegetationConfiguration, VegetationError, project_v8_runtime_to_v9, project_v9_runtime_to_v8,
    project_v9_runtime_to_v10, project_v10_runtime_to_v9,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::hydrology::{
    DirectActiveSnowPartitionInputs, DirectSnowStage3EvaluationError,
    DirectSnowStage3PersistentState, DirectSnowStage3SupportInput, STAGE3_DEFAULT_SNOW_ALBEDO,
    Wb11HydrologyKernel,
};
use crate::land_surface_energy_shadow::{
    CoveredLseIterationState, CoveredV8OwnerEnvelopeError, ExecuteV8LseRuntimeShadowError,
    LandSurfaceEnergyRealHydrologyAdapter, LandSurfaceEnergyShadowError,
    UncommittedCoveredV8OwnerEnvelope, V8CanopyForcingReceipt, V8InputProjectionError,
    V10RootZoneLayerReceipt, V10RootZoneReceiptKey, V10RootZoneReceiptSet,
    execute_v8_lse_runtime_shadow_internal, execute_v8_lse_runtime_shadow_v11_with_carriers,
    unified_beginning_hydrology_snapshot_sha256,
};
use crate::runtime_inputs::{
    DirectGsiOwnerConfigurationV1, PreparedSnowFreeGsiDayV1, SnowFreeHalfHourDestination,
    SnowFreeHalfHourForcingError, SnowFreeHalfHourIntervalReceipt,
    SnowFreeHalfHourProviderConfiguration, SnowFreeHalfHourProviderCursor,
    SnowFreeHalfHourStaticConfiguration, SnowFreePrecipitationParcelReceipt,
    ValidatedSnowFreeHalfHourForcingReceipts,
};
use crate::snow_stage3_terminal_handoff::{
    CanopyLongwaveComponent, CarrierSurface, FinalStage3CanopyBoundaryReceiptInputs,
    FinalStage3CanopyBoundaryReceiptV1, LaneBoundaryContributionV1,
    LaneBoundaryTopologyExpectationV1, LaneStage3BoundaryReceiptV1,
    STAGE3_OFE_TILE_FRACTION_CLOSURE_TOLERANCE, SealedCoveredCarrierForcing, SharedCarrierInput,
    SharedCarrierReceipt, SnowCarrierLedgerInput, SnowStage3HandoffError, Stage3BoundaryIdentity,
    Stage3LaneAreaBasisV1, Stage3SnowSurfaceBoundaryReceiptInputs,
    Stage3SnowSurfaceBoundaryReceiptV1, Stage3TileBoundaryClassV1, evaluate_shared_carrier,
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
#[path = "v11_covered/mod.rs"]
mod v11_covered;

use v11_covered::*;
pub use v11_covered::{
    DirectV11RealConsumerStack, DirectV11SnowCoveredRealConsumerStack,
    DirectV11SnowCoveredStackInputs,
};

const INTERVALS_PER_DAY: usize = 48;
const INTERVAL_S: f64 = 1_800.0;
#[derive(Debug, Error)]
pub enum DirectV11RealConsumerError {
    #[error(transparent)]
    Runtime(#[from] DirectV10RealConsumerError),
    #[error(transparent)]
    Vegetation(#[from] openwepp_vegetation::v11::V11Error),
    #[error("V11 owner serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("V11 actual-consumer identity mismatch: {0}")]
    Identity(&'static str),
    #[error(transparent)]
    CoveredBoundary(#[from] SnowStage3HandoffError),
    #[error(transparent)]
    Stage3(#[from] DirectSnowStage3EvaluationError),
}

#[derive(Clone, Debug, PartialEq)]
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
        if !lse_forcing.snow_present_at_beginning
            || !lse_forcing.snow_present_at_end
            || lse_forcing.snow_terminal_payload_present
        {
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
    soil_thermal: SoilThermalSnapshot,
    biogeochemistry: BiogeochemistryState,
    hydrology_frame: DirectRunFrame,
    next_day_index: usize,
    accepted_interval_count: u64,
    root_zone_hydraulic_configuration: Option<DirectRootZoneHydraulicConfiguration>,
}

/// Complete typed restart owner for the default-off V9 real-consumer shadow.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectV9RealConsumerCheckpoint {
    shadow: DirectV9RealConsumerShadow,
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectRootZoneLayerConfiguration {
    production_lane_index: usize,
    production_lane_id: u32,
    layer_id: SoilLayerId,
    saturated_matric_potential_mm: f64,
    clapp_hornberger_b: f64,
}

impl DirectRootZoneLayerConfiguration {
    pub fn try_new(
        production_lane_index: usize,
        production_lane_id: u32,
        layer_id: SoilLayerId,
        saturated_matric_potential_mm: f64,
        clapp_hornberger_b: f64,
    ) -> Result<Self, DirectV10RealConsumerError> {
        if !saturated_matric_potential_mm.is_finite()
            || saturated_matric_potential_mm >= 0.0
            || !clapp_hornberger_b.is_finite()
            || clapp_hornberger_b <= 0.0
        {
            return Err(DirectV10RealConsumerError::RootDomain(
                "root-zone configuration layer",
            ));
        }
        Ok(Self {
            production_lane_index,
            production_lane_id,
            layer_id,
            saturated_matric_potential_mm,
            clapp_hornberger_b,
        })
    }

    #[must_use]
    pub fn restart_identity_fields(&self) -> (usize, u32, &SoilLayerId, f64, f64) {
        (
            self.production_lane_index,
            self.production_lane_id,
            &self.layer_id,
            self.saturated_matric_potential_mm,
            self.clapp_hornberger_b,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectRootZoneStratumGeometry {
    stratum_id: StratumId,
    root_tissue_lateral_path_m: f64,
}

impl DirectRootZoneStratumGeometry {
    pub fn try_new(
        stratum_id: StratumId,
        root_tissue_lateral_path_m: f64,
    ) -> Result<Self, DirectV10RealConsumerError> {
        if !root_tissue_lateral_path_m.is_finite() || root_tissue_lateral_path_m < 0.0 {
            return Err(DirectV10RealConsumerError::RootDomain(
                "root-zone stratum path",
            ));
        }
        Ok(Self {
            stratum_id,
            root_tissue_lateral_path_m,
        })
    }

    #[must_use]
    pub fn restart_identity_fields(&self) -> (&StratumId, f64) {
        (&self.stratum_id, self.root_tissue_lateral_path_m)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectRootZoneHydraulicConfiguration {
    ordered_layers: Vec<DirectRootZoneLayerConfiguration>,
    ordered_strata: Vec<DirectRootZoneStratumGeometry>,
}

impl DirectRootZoneHydraulicConfiguration {
    pub fn try_new(
        ordered_layers: Vec<DirectRootZoneLayerConfiguration>,
        ordered_strata: Vec<DirectRootZoneStratumGeometry>,
    ) -> Result<Self, DirectV10RealConsumerError> {
        let layer_keys = ordered_layers
            .iter()
            .map(|v| {
                (
                    v.production_lane_index,
                    v.production_lane_id,
                    v.layer_id.clone(),
                )
            })
            .collect::<Vec<_>>();
        let stratum_keys = ordered_strata
            .iter()
            .map(|v| v.stratum_id.clone())
            .collect::<Vec<_>>();
        let unique_layers = layer_keys.iter().cloned().collect::<BTreeSet<_>>();
        let unique_strata = stratum_keys.iter().cloned().collect::<BTreeSet<_>>();
        if ordered_layers.is_empty()
            || ordered_strata.is_empty()
            || unique_layers.len() != layer_keys.len()
            || unique_strata.len() != stratum_keys.len()
        {
            return Err(DirectV10RealConsumerError::RootConfigurationIdentity(
                "root-zone configuration order",
            ));
        }
        Ok(Self {
            ordered_layers,
            ordered_strata,
        })
    }

    pub fn restart_identity_sha256(&self) -> Result<String, DirectV10RealConsumerError> {
        #[derive(Serialize)]
        struct Identity<'a> {
            schema: &'static str,
            ordered_layers: Vec<(usize, u32, &'a SoilLayerId, String, String)>,
            ordered_strata: Vec<(&'a StratumId, String)>,
        }
        let identity = Identity {
            schema: "OPENWEPP_ROOT_ZONE_HYDRAULIC_CONFIGURATION_IDENTITY_V1",
            ordered_layers: self
                .ordered_layers
                .iter()
                .map(|layer| {
                    (
                        layer.production_lane_index,
                        layer.production_lane_id,
                        &layer.layer_id,
                        format!("{:016x}", layer.saturated_matric_potential_mm.to_bits()),
                        format!("{:016x}", layer.clapp_hornberger_b.to_bits()),
                    )
                })
                .collect(),
            ordered_strata: self
                .ordered_strata
                .iter()
                .map(|stratum| {
                    (
                        &stratum.stratum_id,
                        format!("{:016x}", stratum.root_tissue_lateral_path_m.to_bits()),
                    )
                })
                .collect(),
        };
        let bytes = serde_json::to_vec(&identity).map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                "root-zone configuration identity serialization",
            ))
        })?;
        Ok(format!("{:x}", Sha256::digest(bytes)))
    }

    #[must_use]
    pub fn ordered_layers(&self) -> &[DirectRootZoneLayerConfiguration] {
        &self.ordered_layers
    }

    #[must_use]
    pub fn ordered_strata(&self) -> &[DirectRootZoneStratumGeometry] {
        &self.ordered_strata
    }
}

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
            self.beginning.inner.soil_thermal.state_sha256.clone(),
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(error))
        })?;
        let mut candidate = self.beginning.clone();
        let envelope = candidate
            .inner
            .construct_snow_free_interval_envelope_with_duration(
                self.day_index,
                self.interval_index,
                self.interval,
                f64::from_bits(input.duration_s_bits),
                Some(input.duration_s_bits),
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
            })?;
        envelope.validate().map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
        })?;

        let mut resource_debits = v11_nitrogen_resource_debits(&envelope, input)?;
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
        self.ending = Some(candidate);
        Ok(output)
    }
}

impl DirectV10RealConsumerShadow {
    #[must_use]
    pub const fn hydrology_frame(&self) -> &DirectRunFrame {
        self.inner.hydrology_frame()
    }

    /// Install the provider/GSI owner transition only on a cloned candidate
    /// after all coupled Stage-3/V11 supports have accepted. This keeps the
    /// runner cursor out of the live state on any failed support.
    pub(crate) fn commit_prepared_provider_day(
        &mut self,
        prepared: PreparedSnowFreeGsiDayV1,
    ) -> Result<(), DirectV11RealConsumerError> {
        let accepted_receipt = prepared.gsi_receipt().receipt_sha256.clone();
        prepared
            .commit(&mut self.gsi_state, &mut self.provider_cursor)
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::ForcingProvider(
                    error,
                ))
            })?;
        self.inner.provider_gsi_receipt_sha256 = accepted_receipt;
        Ok(())
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

    #[must_use]
    pub const fn vegetation_state(&self) -> &V10CoupledOwnedState {
        &self.vegetation_state
    }

    #[must_use]
    pub const fn vegetation_configuration(&self) -> &VegetationConfiguration {
        &self.vegetation_configuration
    }

    #[must_use]
    pub const fn lse_state(&self) -> &LandSurfaceEnergyV2State {
        &self.lse_state
    }

    #[must_use]
    pub const fn lse_configuration(&self) -> &LandSurfaceEnergyConfiguration {
        &self.lse_configuration
    }

    #[must_use]
    pub const fn gsi_parameters(&self) -> GsiParameters {
        self.gsi_owner_configuration.parameters()
    }

    #[must_use]
    pub const fn gsi_owner_configuration(&self) -> &DirectGsiOwnerConfigurationV1 {
        &self.gsi_owner_configuration
    }

    #[must_use]
    pub const fn root_zone_hydraulic_configuration(&self) -> &DirectRootZoneHydraulicConfiguration {
        &self.root_zone_hydraulic_configuration
    }

    #[must_use]
    pub const fn gsi_state(&self) -> &GsiState {
        &self.gsi_state
    }

    #[must_use]
    pub const fn provider_cursor(&self) -> &SnowFreeHalfHourProviderCursor {
        &self.provider_cursor
    }

    #[must_use]
    pub const fn provider_static_configuration(&self) -> &SnowFreeHalfHourStaticConfiguration {
        &self.provider_static_configuration
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
    pub const fn restart_authority_soil_thermal(&self) -> &SoilThermalSnapshot {
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
            soil_thermal,
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
        soil_thermal: SoilThermalSnapshot,
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
    pub const fn soil_thermal(&self) -> &SoilThermalSnapshot {
        &self.soil_thermal
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
        self.construct_canopy_soil_interval_envelope_with_duration(
            day_index,
            interval_index,
            input,
            interval_s,
            v11_duration_s_bits,
            None,
            false,
            None,
        )
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
        covered_carrier_receipts: Option<&BTreeMap<(OfeId, TileId), SharedCarrierReceipt>>,
    ) -> Result<UncommittedCoveredV8OwnerEnvelope, DirectV9RealConsumerError> {
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
        if input.lse_forcing.transaction_id != transaction_id
            || input.lse_forcing.interval_s.to_bits() != interval_s.to_bits()
            || v11_duration_s_bits.is_some_and(|bits| bits != interval_s.to_bits())
            || input.lse_forcing.snow_present_at_beginning
            || input.lse_forcing.snow_present_at_end
            || input.lse_forcing.snow_terminal_payload_present
        {
            return Err(DirectV9RealConsumerError::Unsupported(
                "forcing transaction, cadence, or snow domain",
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
        let hydrology = RealHydrologyShadowAdapter::try_from_day_start(
            &self.hydrology_frame,
            day_index,
            transaction_id,
            interval_s,
            self.surface_configuration.owner_id.clone(),
            &self.layer_maps,
        )?;
        let soil_adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&hydrology);
        let hydrology_snapshot = unified_beginning_hydrology_snapshot_sha256(
            &soil_adapter,
            &self.surface_configuration,
        )?;
        let forcing_sha256 = input.lse_forcing.canonical_sha256()?;
        let (vegetation_forcing, root_zone_hydraulics) = project_live_vegetation_forcing(
            &input.vegetation_forcing,
            &hydrology,
            &self.soil_thermal,
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
                self.soil_thermal.snapshot_sha256.clone(),
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
                self.soil_thermal.snapshot_sha256.clone(),
                transaction_id,
                vegetation_forcing,
            )?,
        };
        let nitrogen = BiogeochemistryNitrogenArbiter::try_new(&self.biogeochemistry)?;
        let envelope = match v11_duration_s_bits {
            Some(bits) => match covered_carrier_receipts {
                Some(carriers) => execute_v8_lse_runtime_shadow_v11_with_carriers(
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
                    &self.soil_thermal,
                    &nitrogen,
                    &self.biogeochemistry,
                    self.authority,
                    covered_lower_boundaries,
                    bits,
                    !provisional_v11,
                    Some(carriers),
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
                    &self.soil_thermal,
                    &nitrogen,
                    &self.biogeochemistry,
                    self.authority,
                    covered_lower_boundaries,
                    bits,
                    !provisional_v11,
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
                &self.soil_thermal,
                &nitrogen,
                &self.biogeochemistry,
                None,
                self.authority,
            )?,
        };
        Ok(envelope)
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
        carrier_receipts: &BTreeMap<(OfeId, TileId), SharedCarrierReceipt>,
        lower_boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        provisional_v11: bool,
    ) -> Result<UncommittedCoveredV8OwnerEnvelope, DirectV9RealConsumerError> {
        if !input.lse_forcing.snow_present_at_beginning
            || !input.lse_forcing.snow_present_at_end
            || input.lse_forcing.snow_terminal_payload_present
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
            Some(carrier_receipts),
        )
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
            &self.soil_thermal,
            &self.lse_configuration,
            transaction_id,
            envelope.hydrology().soil_thermal_candidates(),
        )?;
        self.vegetation_state = vegetation_state;
        self.lse_state = lse_state;
        self.soil_thermal = soil_thermal;
        self.biogeochemistry = envelope.biogeochemistry().ending().clone();
        self.hydrology_frame = envelope.hydrology().ending_frame().clone();
        self.accepted_interval_count = self.accepted_interval_count.checked_add(1).ok_or(
            DirectV9RealConsumerError::Identity("accepted interval count overflow"),
        )?;
        Ok(())
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
            .last_accepted_transaction_id
            .is_none_or(|value| value == transaction_id);
        let complete_accepted_lineage = self.accepted_interval_count == 0
            || (self.lse_state.last_accepted_transaction_id == Some(transaction_id)
                && self.soil_thermal.last_accepted_transaction_id == Some(transaction_id));
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
            soil_thermal: &'a SoilThermalSnapshot,
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
        let temperature = common_provider_value(
            &temperature_values,
            "vegetation soil-temperature projection",
        )?;
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
                    if matching_ofes.is_empty() {
                        return Err(DirectV9RealConsumerError::Identity(
                            "root-zone OFE/lane join",
                        ));
                    }
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct RootZoneHydraulicSourceValues {
    liquid_water_depth_m: f64,
    layer_thickness_m: f64,
    porosity: f64,
    saturated_conductivity_m_s: f64,
    saturated_matric_potential_mm: f64,
    clapp_hornberger_b: f64,
    layer_top_m: f64,
    root_tissue_lateral_path_m: f64,
    frozen: bool,
}

fn root_zone_hydraulic_receipt(
    key: V10RootZoneReceiptKey,
    source: RootZoneHydraulicSourceValues,
    lateral_root_length_m: f64,
) -> Result<V10RootZoneLayerReceipt, DirectV9RealConsumerError> {
    if !lateral_root_length_m.is_finite() || lateral_root_length_m <= 0.0 {
        return Err(V8InputProjectionError::RootDomain.into());
    }
    let capacity = source.porosity * source.layer_thickness_m;
    let capacity_limit = f64::from_bits(capacity.to_bits() + 1);
    if source.liquid_water_depth_m > capacity_limit {
        return Err(V8InputProjectionError::WaterAbovePoreCapacity.into());
    }
    let relative_saturation_raw = source.liquid_water_depth_m / capacity;
    let mut relative_saturation = relative_saturation_raw.clamp(0.0, 1.0);
    if relative_saturation == 0.0 {
        relative_saturation = 0.0;
    }
    let matric_potential_mm = (source.saturated_matric_potential_mm
        * libm::pow(relative_saturation.max(0.01), -source.clapp_hornberger_b))
    .max(-1.0e8);
    let hydraulic_conductivity_mm_s = 1000.0
        * source.saturated_conductivity_m_s.min(
            source.saturated_conductivity_m_s
                * libm::pow(relative_saturation, 2.0 * source.clapp_hornberger_b + 3.0),
        );
    let layer_node_depth_m = source.layer_top_m + 0.5 * source.layer_thickness_m;
    Ok(V10RootZoneLayerReceipt {
        key,
        matric_potential_mm,
        hydraulic_conductivity_mm_s,
        root_path_length_mm: 1000.0 * (layer_node_depth_m + source.root_tissue_lateral_path_m),
        gravity_root_mm: -1000.0 * layer_node_depth_m,
        lateral_root_length_m,
    })
}

fn root_zone_hydraulic_values(
    fact: &crate::vegetation_real_hydrology_shadow::RealHydrologyLayerFact,
    configuration: &DirectRootZoneLayerConfiguration,
    top_m: f64,
    root_tissue_lateral_path_m: f64,
) -> Result<RootZoneHydraulicSourceValues, DirectV9RealConsumerError> {
    let values = [
        fact.liquid_water_depth_m,
        fact.layer_thickness_m,
        fact.porosity,
        fact.saturated_conductivity_m_s,
        configuration.saturated_matric_potential_mm,
        configuration.clapp_hornberger_b,
        top_m,
        root_tissue_lateral_path_m,
    ];
    if values.iter().any(|value| !value.is_finite())
        || fact.liquid_water_depth_m < 0.0
        || fact.layer_thickness_m <= 0.0
        || !(0.0 < fact.porosity && fact.porosity <= 1.0)
        || fact.saturated_conductivity_m_s <= 0.0
        || configuration.saturated_matric_potential_mm >= 0.0
        || configuration.clapp_hornberger_b <= 0.0
        || top_m < 0.0
        || root_tissue_lateral_path_m < 0.0
    {
        return Err(V8InputProjectionError::RootDomain.into());
    }
    if fact.frozen {
        return Err(V8InputProjectionError::FrozenRootedLayerUnsupported.into());
    }
    Ok(RootZoneHydraulicSourceValues {
        liquid_water_depth_m: fact.liquid_water_depth_m,
        layer_thickness_m: fact.layer_thickness_m,
        porosity: fact.porosity,
        saturated_conductivity_m_s: fact.saturated_conductivity_m_s,
        saturated_matric_potential_mm: configuration.saturated_matric_potential_mm,
        clapp_hornberger_b: configuration.clapp_hornberger_b,
        layer_top_m: top_m,
        root_tissue_lateral_path_m,
        frozen: fact.frozen,
    })
}

fn common_provider_value(
    values: &[f64],
    detail: &'static str,
) -> Result<f64, DirectV9RealConsumerError> {
    let first = values
        .first()
        .copied()
        .ok_or(DirectV9RealConsumerError::Identity(detail))?;
    if values
        .iter()
        .any(|value| value.to_bits() != first.to_bits())
    {
        return Err(DirectV9RealConsumerError::Unsupported(detail));
    }
    Ok(first)
}

struct BiogeochemistryNitrogenArbiter {
    available: BTreeMap<MineralNitrogenKey, f64>,
}

impl BiogeochemistryNitrogenArbiter {
    fn try_new(state: &BiogeochemistryState) -> Result<Self, DirectV9RealConsumerError> {
        Ok(Self {
            available: available_by_key(state)?,
        })
    }
}

impl NitrogenArbiter for BiogeochemistryNitrogenArbiter {
    fn beginning_amount(&self, key: &MineralNitrogenKey) -> Result<f64, VegetationError> {
        self.available
            .get(key)
            .copied()
            .ok_or(VegetationError::Domain("unknown nitrogen inventory"))
    }

    fn authorize(
        &self,
        requests: &[NitrogenRequest],
    ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
        authorize_proportionally(
            requests,
            &self.available,
            ResourceAmountBasis::NitrogenKgPerSquareMeterInterval,
        )
        .map_err(VegetationError::from)
    }
}

fn aggregate_soil_thermal_ending(
    beginning: &SoilThermalSnapshot,
    configuration: &LandSurfaceEnergyConfiguration,
    transaction_id: TransactionId,
    candidates: &[SoilThermalTileCandidate],
) -> Result<SoilThermalSnapshot, DirectV9RealConsumerError> {
    validate_soil_thermal_candidate_set(beginning, configuration, candidates)?;
    let mut ofes = Vec::with_capacity(beginning.ofes.len());
    for beginning_ofe in &beginning.ofes {
        ofes.push(aggregate_soil_thermal_ofe(
            beginning,
            beginning_ofe,
            configuration,
            candidates,
        )?);
    }
    let state_sha256 = digest_soil_state(&beginning.owner_id, transaction_id, &ofes)?;
    let snapshot_sha256 = digest_soil_snapshot(
        &beginning.owner_id,
        &beginning.configuration_sha256,
        &state_sha256,
        transaction_id,
        &ofes,
    )?;
    let ending = SoilThermalSnapshot {
        owner_id: beginning.owner_id.clone(),
        configuration_sha256: beginning.configuration_sha256.clone(),
        state_sha256,
        snapshot_sha256,
        last_accepted_transaction_id: Some(transaction_id),
        ofes,
    };
    ending.validate()?;
    Ok(ending)
}

fn validate_soil_thermal_candidate_set(
    beginning: &SoilThermalSnapshot,
    configuration: &LandSurfaceEnergyConfiguration,
    candidates: &[SoilThermalTileCandidate],
) -> Result<(), DirectV9RealConsumerError> {
    let configured_tiles = configuration
        .ofes
        .iter()
        .flat_map(|ofe| {
            ofe.tiles.iter().map(move |tile| {
                (
                    (ofe.ofe_id.clone(), tile.tile_id.clone()),
                    tile.fraction_ofe_ground,
                )
            })
        })
        .collect::<BTreeMap<_, _>>();
    let actual_tiles = candidates
        .iter()
        .map(|candidate| (candidate.ofe_id.clone(), candidate.tile_id.clone()))
        .collect::<BTreeSet<_>>();
    let configured_ofes = configuration
        .ofes
        .iter()
        .map(|ofe| ofe.ofe_id.clone())
        .collect::<BTreeSet<_>>();
    let beginning_ofes = beginning
        .ofes
        .iter()
        .map(|ofe| ofe.ofe_id.clone())
        .collect::<BTreeSet<_>>();
    if actual_tiles.len() != candidates.len()
        || actual_tiles != configured_tiles.keys().cloned().collect()
        || beginning_ofes.len() != beginning.ofes.len()
        || beginning_ofes != configured_ofes
    {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            "soil-thermal tile candidate set",
        ));
    }
    Ok(())
}

fn aggregate_soil_thermal_ofe(
    beginning: &SoilThermalSnapshot,
    beginning_ofe: &SoilThermalOfeSnapshot,
    configuration: &LandSurfaceEnergyConfiguration,
    candidates: &[SoilThermalTileCandidate],
) -> Result<SoilThermalOfeSnapshot, DirectV9RealConsumerError> {
    let configured_ofe = configuration
        .ofes
        .iter()
        .find(|ofe| ofe.ofe_id == beginning_ofe.ofe_id)
        .ok_or(DirectV9RealConsumerError::OwnerClosure(
            "soil-thermal OFE configuration",
        ))?;
    let mut tile_candidates = candidates
        .iter()
        .filter(|candidate| candidate.ofe_id == beginning_ofe.ofe_id)
        .collect::<Vec<_>>();
    tile_candidates.sort_unstable_by(|left, right| left.tile_id.cmp(&right.tile_id));
    if tile_candidates.len() != configured_ofe.tiles.len() {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            "soil-thermal OFE tile cardinality",
        ));
    }
    if beginning_ofe.ordered_layers.len() != configured_ofe.soil_interface_layers.len() {
        return Err(DirectV9RealConsumerError::OwnerClosure(
            "soil-thermal beginning/configured layer cardinality",
        ));
    }
    let mut ordered_layers = Vec::with_capacity(beginning_ofe.ordered_layers.len());
    for (layer_index, beginning_layer) in beginning_ofe.ordered_layers.iter().enumerate() {
        let configured_layer = configured_ofe
            .soil_interface_layers
            .get(layer_index)
            .ok_or(DirectV9RealConsumerError::OwnerClosure(
                "soil-thermal configured layer order",
            ))?;
        if configured_layer.layer_id != beginning_layer.layer_id
            || !configured_layer.areal_heat_capacity_j_m2_k.is_finite()
            || configured_layer.areal_heat_capacity_j_m2_k <= 0.0
        {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "soil-thermal layer identity or capacity",
            ));
        }
        let mut ending_enthalpy = beginning_layer.enthalpy_j_m2_ofe_ground;
        for candidate in &tile_candidates {
            if candidate.owner_id != beginning.owner_id
                || candidate.beginning_state_sha256 != beginning.state_sha256
                || candidate.layers.len() != beginning_ofe.ordered_layers.len()
            {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "soil-thermal candidate owner lineage or layer cardinality",
                ));
            }
            let layer = candidate.layers.get(layer_index).ok_or(
                DirectV9RealConsumerError::OwnerClosure("soil-thermal candidate layer cardinality"),
            )?;
            if layer.layer_id != beginning_layer.layer_id
                || layer.beginning_enthalpy_j_m2_ofe_ground.to_bits()
                    != beginning_layer.enthalpy_j_m2_ofe_ground.to_bits()
            {
                return Err(DirectV9RealConsumerError::OwnerClosure(
                    "soil-thermal candidate beginning layer",
                ));
            }
            ending_enthalpy +=
                layer.ending_enthalpy_j_m2_ofe_ground - layer.beginning_enthalpy_j_m2_ofe_ground;
        }
        let ending_temperature_k = beginning_layer.temperature_k
            + (ending_enthalpy - beginning_layer.enthalpy_j_m2_ofe_ground)
                / configured_layer.areal_heat_capacity_j_m2_k;
        if !ending_enthalpy.is_finite() || !(200.0..=350.0).contains(&ending_temperature_k) {
            return Err(DirectV9RealConsumerError::Unsupported(
                "aggregated soil-thermal ending domain",
            ));
        }
        ordered_layers.push(SoilThermalLayerSnapshot {
            layer_id: beginning_layer.layer_id.clone(),
            temperature_k: ending_temperature_k,
            enthalpy_j_m2_ofe_ground: ending_enthalpy,
        });
    }
    Ok(SoilThermalOfeSnapshot {
        ofe_id: beginning_ofe.ofe_id.clone(),
        ordered_layers,
    })
}

fn digest_soil_state(
    owner_id: &ResourceOwnerId,
    transaction_id: TransactionId,
    ofes: &[SoilThermalOfeSnapshot],
) -> Result<Sha256Digest, DirectV9RealConsumerError> {
    digest_serialized(&(owner_id, transaction_id, ofes))
}

fn digest_soil_snapshot(
    owner_id: &ResourceOwnerId,
    configuration_sha256: &Sha256Digest,
    state_sha256: &Sha256Digest,
    transaction_id: TransactionId,
    ofes: &[SoilThermalOfeSnapshot],
) -> Result<Sha256Digest, DirectV9RealConsumerError> {
    digest_serialized(&(
        owner_id,
        configuration_sha256,
        state_sha256,
        transaction_id,
        ofes,
    ))
}

/// Verify both nested soil-owner digests using the real consumer's exact
/// digest recipes. This is exposed only to the package-local authority
/// evidence feature.
#[cfg(any(
    feature = "restart-authority-evidence",
    feature = "persisted-restart-v1"
))]
pub fn restart_authority_validate_soil_thermal_digests(
    snapshot: &SoilThermalSnapshot,
) -> Result<(), DirectV9RealConsumerError> {
    let transaction_id =
        snapshot
            .last_accepted_transaction_id
            .ok_or(DirectV9RealConsumerError::Identity(
                "soil-thermal transaction lineage",
            ))?;
    let state = digest_soil_state(&snapshot.owner_id, transaction_id, &snapshot.ofes)?;
    let outer = digest_soil_snapshot(
        &snapshot.owner_id,
        &snapshot.configuration_sha256,
        &state,
        transaction_id,
        &snapshot.ofes,
    )?;
    if state != snapshot.state_sha256 || outer != snapshot.snapshot_sha256 {
        return Err(DirectV9RealConsumerError::Identity(
            "soil-thermal nested digest",
        ));
    }
    Ok(())
}

#[cfg(any(
    feature = "restart-authority-evidence",
    feature = "persisted-restart-v1"
))]
pub fn restart_authority_seal_soil_thermal_digests(
    snapshot: &mut SoilThermalSnapshot,
) -> Result<(), DirectV9RealConsumerError> {
    let transaction_id =
        snapshot
            .last_accepted_transaction_id
            .ok_or(DirectV9RealConsumerError::Identity(
                "soil-thermal transaction lineage",
            ))?;
    snapshot.state_sha256 = digest_soil_state(&snapshot.owner_id, transaction_id, &snapshot.ofes)?;
    snapshot.snapshot_sha256 = digest_soil_snapshot(
        &snapshot.owner_id,
        &snapshot.configuration_sha256,
        &snapshot.state_sha256,
        transaction_id,
        &snapshot.ofes,
    )?;
    Ok(())
}

fn digest_serialized<T: Serialize>(value: &T) -> Result<Sha256Digest, DirectV9RealConsumerError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| DirectV9RealConsumerError::Serialization(error.to_string()))?;
    Sha256Digest::try_new(format!("{:x}", Sha256::digest(bytes))).map_err(Into::into)
}

#[cfg(test)]
include!("v9_real_consumer_shadow_tests.rs");
