//! Explicit default-off V9/LSE consumer over the real direct scheduler owner.
//!
//! This module owns only isolated shadow state. It has no production commit,
//! selector, publication, or output API.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_biogeochemistry::{BiogeochemistryError, BiogeochemistryState, available_by_key};
use openwepp_kernel_contract::{
    MineralNitrogenKey, ResourceAmountBasis, ResourceOwnerId, TileId, TransactionId,
    authorize_proportionally,
};
use openwepp_land_surface_energy::{
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyError, LandSurfaceEnergyState,
    LandSurfaceForcing, LiquidParcel, LiquidParcelKind, LiquidTemperatureProvider, OfeId, ParcelId,
    Sha256Digest, SoilThermalLayerSnapshot, SoilThermalOfeSnapshot, SoilThermalSnapshot,
    SoilThermalTileCandidate, build_lse_ending_state,
};
use openwepp_meteorology::snow_free_forcing::{
    celsius_to_kelvin, kilopascals_to_pascals, liquid_specific_enthalpy_j_kg,
};
use openwepp_vegetation::{
    NitrogenArbiter, NitrogenAuthorization, NitrogenRequest, SnowFreeForcing, V9CoupledOwnedState,
    V9StateError, VegetationConfiguration, VegetationError, project_v8_runtime_to_v9,
    project_v9_runtime_to_v8,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::land_surface_energy_shadow::{
    CoveredV8OwnerEnvelopeError, ExecuteV8LseRuntimeShadowError,
    LandSurfaceEnergyRealHydrologyAdapter, LandSurfaceEnergyShadowError,
    UncommittedCoveredV8OwnerEnvelope, V8CanopyForcingReceipt, V8InputProjectionError,
    execute_v8_lse_runtime_shadow, unified_beginning_hydrology_snapshot_sha256,
};
use crate::runtime_inputs::{
    SnowFreeHalfHourDestination, SnowFreeHalfHourIntervalReceipt,
    SnowFreeHalfHourProviderConfiguration, SnowFreePrecipitationParcelReceipt,
    ValidatedSnowFreeHalfHourForcingReceipts,
};
use crate::vegetation_real_hydrology_shadow::{
    RealHydrologyLaneLayerMap, RealHydrologyShadowAdapter, RealHydrologyShadowError,
};
use crate::{
    DirectDayFrame, DirectOfeWb14Parameters, DirectPublicationDayInput, DirectRunFrame,
    DirectSurfaceLiquidConfiguration,
};

const INTERVALS_PER_DAY: usize = 48;
const INTERVAL_S: f64 = 1_800.0;

#[derive(Clone, Debug, PartialEq)]
pub struct DirectV9ShadowIntervalInput {
    pub lse_forcing: LandSurfaceForcing,
    pub vegetation_forcing: SnowFreeForcing,
    pub wb14_parameters: Vec<DirectOfeWb14Parameters>,
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

/// Replace atmospheric/precipitation template operands with a sealed
/// repository-provider receipt while retaining live-owner fields (soil,
/// albedo, runon, snow guards, and transaction identity) from the template.
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

fn validate_wb14_provider_bindings(
    receipts: &[crate::runtime_inputs::SnowFreeHalfHourDayReceipt],
    interval_index: usize,
    template: &DirectV9ShadowIntervalInput,
) -> Result<(), DirectV9RealConsumerError> {
    for receipt in receipts {
        let provider = &receipt.intervals[interval_index];
        let parameter = template
            .wb14_parameters
            .iter()
            .find(|value| value.ofe_id.as_str() == provider.ofe_id)
            .ok_or(DirectV9RealConsumerError::Identity(
                "repository WB14 OFE binding",
            ))?;
        if provider.wb14_configuration_sha256 != wb14_parameter_sha256(parameter) {
            return Err(DirectV9RealConsumerError::Identity(
                "repository WB14 configuration receipt",
            ));
        }
    }
    Ok(())
}

fn wb14_parameter_sha256(value: &DirectOfeWb14Parameters) -> String {
    let mut digest = Sha256::new();
    digest.update(value.ofe_id.as_str().as_bytes());
    for operand in [
        value.effective_conductivity_m_s,
        value.matric_potential_m,
        value.infiltration_storage_capacity_m,
    ] {
        digest.update(operand.to_bits().to_le_bytes());
    }
    format!("{:x}", digest.finalize())
}

fn validate_global_provider_interval(
    receipts: &[crate::runtime_inputs::SnowFreeHalfHourDayReceipt],
    interval_index: usize,
    expected: &SnowFreeHalfHourIntervalReceipt,
) -> Result<(), DirectV9RealConsumerError> {
    let expected_values = provider_global_values(expected);
    for receipt in receipts {
        let candidate = &receipt.intervals[interval_index];
        if provider_global_values(candidate)
            .iter()
            .zip(expected_values)
            .any(|(left, right)| left.to_bits() != right.to_bits())
            || candidate.co2_pa.to_bits() != expected.co2_pa.to_bits()
            || candidate.reference_height_m.to_bits() != expected.reference_height_m.to_bits()
            || candidate.gsi.to_bits() != expected.gsi.to_bits()
            || candidate.gsi_receipt_sha256 != expected.gsi_receipt_sha256
        {
            return Err(DirectV9RealConsumerError::Unsupported(
                "repository global atmospheric forcing heterogeneity",
            ));
        }
    }
    Ok(())
}

fn provider_global_values(value: &SnowFreeHalfHourIntervalReceipt) -> [f64; 15] {
    [
        value.air_temperature_c,
        value.dew_point_c,
        value.wind_m_s,
        value.pressure_kpa,
        value.actual_vapor_pressure_kpa,
        value.specific_humidity_kg_kg,
        value.vpd_kpa,
        value.cloud_fraction,
        value.solar_zenith_cosine,
        value.global_horizontal_shortwave_w_m2,
        value.direct_visible_w_m2,
        value.diffuse_visible_w_m2,
        value.direct_nir_w_m2,
        value.diffuse_nir_w_m2,
        value.downward_longwave_w_m2,
    ]
}

fn project_lse_atmosphere(
    receipts: &[crate::runtime_inputs::SnowFreeHalfHourDayReceipt],
    interval_index: usize,
    atmospheric: &SnowFreeHalfHourIntervalReceipt,
    forcing: &mut LandSurfaceForcing,
) -> Result<(), DirectV9RealConsumerError> {
    forcing.air_temperature_k = celsius_to_kelvin(atmospheric.air_temperature_c);
    forcing.air_specific_humidity_kg_kg = atmospheric.specific_humidity_kg_kg;
    forcing.air_pressure_pa = kilopascals_to_pascals(atmospheric.pressure_kpa);
    forcing.reference_wind_m_s = atmospheric.wind_m_s;
    forcing.direct_vis_w_m2 = atmospheric.direct_visible_w_m2;
    forcing.diffuse_vis_w_m2 = atmospheric.diffuse_visible_w_m2;
    forcing.direct_nir_w_m2 = atmospheric.direct_nir_w_m2;
    forcing.diffuse_nir_w_m2 = atmospheric.diffuse_nir_w_m2;
    forcing.atmospheric_downward_longwave_w_m2 = atmospheric.downward_longwave_w_m2;
    forcing.precipitation_parcels.clear();
    for receipt in receipts {
        let source = &receipt.intervals[interval_index];
        for parcel in &source.precipitation_parcels {
            forcing.precipitation_parcels.push(project_lse_parcel(
                source,
                parcel,
                forcing.interval_s,
            )?);
        }
    }
    forcing.forcing_sha256 = Sha256Digest::try_new("0".repeat(64))?;
    forcing.forcing_sha256 = forcing.canonical_sha256()?;
    forcing.validate(forcing.transaction_id)?;
    Ok(())
}

fn project_lse_parcel(
    interval: &SnowFreeHalfHourIntervalReceipt,
    parcel: &SnowFreePrecipitationParcelReceipt,
    interval_s: f64,
) -> Result<LiquidParcel, DirectV9RealConsumerError> {
    let interval_start = f64::from(
        u32::try_from(interval.start_s)
            .map_err(|_| DirectV9RealConsumerError::Identity("provider interval support"))?,
    );
    let start_s = parcel.start_s - interval_start;
    let end_s = parcel.end_s - interval_start;
    if start_s < 0.0 || end_s > interval_s {
        return Err(DirectV9RealConsumerError::Identity(
            "provider parcel interval support",
        ));
    }
    let destination_ofe = OfeId::try_new(parcel.destination_ofe_id.clone())?;
    let destination_tile = TileId::try_new(parcel.destination_tile_id.clone())
        .map_err(|_| DirectV9RealConsumerError::Identity("provider parcel tile"))?;
    Ok(LiquidParcel {
        parcel_kind: LiquidParcelKind::Precipitation,
        parcel_id: ParcelId::try_new(format!(
            "{}:{}:{}",
            parcel.parcel_id, parcel.destination_ofe_id, parcel.destination_tile_id
        ))?,
        source_owner_id: ResourceOwnerId::try_new(parcel.source_owner_id.clone())
            .map_err(|_| DirectV9RealConsumerError::Identity("provider parcel owner"))?,
        source_ofe_id: destination_ofe.clone(),
        source_tile_id: destination_tile.clone(),
        destination_ofe_id: destination_ofe,
        destination_tile_id: destination_tile,
        start_s,
        end_s,
        amount_kg_m2_destination_tile_ground: parcel.mass_kg_m2,
        temperature_provider: LiquidTemperatureProvider::HarderPomeroyHourly,
        temperature_k: Some(parcel.temperature_k),
        specific_liquid_enthalpy_j_kg: Some(liquid_specific_enthalpy_j_kg(parcel.temperature_k)),
        source_state_sha256: Some(Sha256Digest::try_new(parcel.source_owner_id.clone())?),
    })
}

fn project_vegetation_atmosphere(
    provider: &SnowFreeHalfHourIntervalReceipt,
    forcing: &mut SnowFreeForcing,
) {
    forcing.air_temperature_k = celsius_to_kelvin(provider.air_temperature_c);
    forcing.pressure_pa = kilopascals_to_pascals(provider.pressure_kpa);
    forcing.co2_pa = provider.co2_pa;
    forcing.vapor_pressure_deficit_kpa = provider.vpd_kpa;
    forcing.wind_m_s = provider.wind_m_s;
    forcing.rain_kg_m2 = provider
        .precipitation_parcels
        .iter()
        .map(|parcel| parcel.mass_kg_m2)
        .fold(0.0, |sum, value| sum + value);
    forcing.direct_par_w_m2 = provider.direct_visible_w_m2;
    forcing.diffuse_par_w_m2 = provider.diffuse_visible_w_m2;
    forcing.direct_nir_w_m2 = provider.direct_nir_w_m2;
    forcing.diffuse_nir_w_m2 = provider.diffuse_nir_w_m2;
    forcing.solar_zenith_cosine = provider.solar_zenith_cosine;
    forcing.longwave_down_w_m2 = provider.downward_longwave_w_m2;
    forcing.specific_humidity = provider.specific_humidity_kg_kg;
    forcing.reference_height_m = provider.reference_height_m;
    forcing.gsi = provider.gsi;
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectV9RealConsumerShadow {
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
}

/// Complete typed restart owner for the default-off V9 real-consumer shadow.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectV9RealConsumerCheckpoint {
    shadow: DirectV9RealConsumerShadow,
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
            gsi_receipt_sha256: self.vegetation_state.0.state_sha256.clone(),
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
            &self.vegetation_state.0.state_sha256,
            &expected_destinations,
        )
    }

    #[allow(clippy::too_many_arguments)]
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

    fn execute_interval(
        &mut self,
        day_index: usize,
        interval_index: usize,
        input: &DirectV9ShadowIntervalInput,
    ) -> Result<(), DirectV9RealConsumerError> {
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
            || input.lse_forcing.interval_s.to_bits() != INTERVAL_S.to_bits()
            || input.lse_forcing.snow_present_at_beginning
            || input.lse_forcing.snow_present_at_end
            || input.lse_forcing.snow_terminal_payload_present
        {
            return Err(DirectV9RealConsumerError::Unsupported(
                "forcing transaction, cadence, or snow domain",
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
            INTERVAL_S,
            self.surface_configuration.owner_id.clone(),
            &self.layer_maps,
        )?;
        let soil_adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&hydrology);
        let hydrology_snapshot = unified_beginning_hydrology_snapshot_sha256(
            &soil_adapter,
            &self.surface_configuration,
        )?;
        let forcing_sha256 = input.lse_forcing.canonical_sha256()?;
        let vegetation_forcing = project_live_vegetation_forcing(
            &input.vegetation_forcing,
            &hydrology,
            &self.soil_thermal,
        )?;
        let canopy_forcing = V8CanopyForcingReceipt::try_new(
            v8_configuration.configuration_sha256.clone(),
            v8_beginning.state_sha256.clone(),
            self.lse_configuration.configuration_sha256.clone(),
            forcing_sha256,
            hydrology_snapshot,
            self.soil_thermal.snapshot_sha256.clone(),
            transaction_id,
            vegetation_forcing,
        )?;
        let nitrogen = BiogeochemistryNitrogenArbiter::try_new(&self.biogeochemistry)?;
        let envelope = execute_v8_lse_runtime_shadow(
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
        )?;
        self.accept_envelope(transaction_id, &envelope)
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

fn project_live_vegetation_forcing(
    provider: &SnowFreeForcing,
    hydrology: &RealHydrologyShadowAdapter,
    soil_thermal: &SoilThermalSnapshot,
) -> Result<SnowFreeForcing, DirectV9RealConsumerError> {
    let mut forcing = provider.clone();
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
        let water = common_provider_value(&water_values, "vegetation soil-water projection")?;
        let temperature = common_provider_value(
            &temperature_values,
            "vegetation soil-temperature projection",
        )?;
        layer.water_beginning_kg_m2 = water;
        layer.temperature_k = temperature;
    }
    Ok(forcing)
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

fn digest_serialized<T: Serialize>(value: &T) -> Result<Sha256Digest, DirectV9RealConsumerError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| DirectV9RealConsumerError::Serialization(error.to_string()))?;
    Sha256Digest::try_new(format!("{:x}", Sha256::digest(bytes))).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use openwepp_input_contract::parsers::climate::{ParserMode, parse_climate_from_str};
    use openwepp_kernel_contract::TileId;
    use openwepp_land_surface_energy::{OfeId, SoilThermalLayerCandidate};
    use openwepp_vegetation::{V9_MODEL_SHA256, V9CoupledOwnedState};

    use super::*;
    use crate::land_surface_energy_shadow::{EndpointFixture, endpoint_fixture};
    use crate::runtime_inputs::{
        SnowFreeHalfHourProviderCursor, build_hillslope_climate_runtime_request,
    };
    use crate::{
        DirectExecutorMode, DirectFrameExecutor, DirectLanedActiveConfig,
        DirectLanedActiveLaneConfig, DirectLanedActiveMeshPolicy, DirectPublicationCalendarDay,
        DirectPublicationDayInput, DirectPublicationRunMetadata,
    };

    fn v9_configuration_and_state(
        fixture: &EndpointFixture,
    ) -> (VegetationConfiguration, V9CoupledOwnedState) {
        let mut configuration = fixture.vegetation_configuration.clone();
        configuration.model_definition_sha256 = V9_MODEL_SHA256.into();
        configuration.configuration_sha256 = configuration
            .canonical_sha256()
            .expect("V9 configuration digest");
        let mut state = fixture.vegetation_state.clone();
        state.model_definition_sha256 = V9_MODEL_SHA256.into();
        state
            .configuration_sha256
            .clone_from(&configuration.configuration_sha256);
        state.state_sha256 = state.canonical_sha256();
        let state = V9CoupledOwnedState(state);
        state.validate(&configuration).expect("V9 fixture state");
        (configuration, state)
    }

    fn shadow_fixture() -> (DirectV9RealConsumerShadow, EndpointFixture) {
        let fixture = endpoint_fixture();
        let (configuration, state) = v9_configuration_and_state(&fixture);
        let shadow = DirectV9RealConsumerShadow::try_new(
            configuration,
            state,
            ResourceOwnerId::try_new("vegetation-v8").expect("owner"),
            fixture.lse_configuration.clone(),
            fixture.lse_state.clone(),
            fixture.surface_configuration.clone(),
            fixture.hydrology.layer_maps().to_vec(),
            fixture.thermal.clone(),
            fixture.biogeochemistry.clone(),
            fixture.hydrology.beginning_frame().clone(),
            0,
        )
        .expect("shadow fixture");
        (shadow, fixture)
    }

    fn day_input(fixture: &EndpointFixture) -> DirectV9ShadowDayInput {
        let base_vegetation = fixture.receipt.forcing().clone();
        let intervals = (0..INTERVALS_PER_DAY)
            .map(|index| {
                let mut forcing = fixture.forcing.clone();
                forcing.transaction_id = TransactionId(41 + index as u128);
                forcing.forcing_sha256 = forcing.canonical_sha256().expect("forcing digest");
                DirectV9ShadowIntervalInput {
                    lse_forcing: forcing,
                    vegetation_forcing: base_vegetation.clone(),
                    wb14_parameters: vec![DirectOfeWb14Parameters {
                        ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                        effective_conductivity_m_s: 1e-6,
                        matric_potential_m: 0.1,
                        infiltration_storage_capacity_m: 0.04,
                    }],
                }
            })
            .collect();
        DirectV9ShadowDayInput::try_new(0, intervals).expect("shadow day input")
    }

    fn production_day_input() -> DirectPublicationDayInput {
        let mut input = DirectPublicationDayInput::calendar_only(DirectPublicationCalendarDay {
            year: 2026,
            julian_day: 1,
            month: 1,
            day_of_month: 1,
            water_year: 2026,
        });
        input.precipitation_m = 0.0;
        input.effective_temperature_c = 7.5;
        input
    }

    #[test]
    fn sealed_repository_receipts_project_into_real_child4_forcing_types() {
        let (mut shadow, fixture) = shadow_fixture();
        let template = day_input(&fixture);
        let source = "5.30\n1 0 0\nTEST STATION 1500\nDAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT\n41.1 -120.0 1225.0 30 2000 1 CLIGEN 5.30 --seed 123\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n20 6 2000 0.0 0.0 0.0 0.0 28.0 22.0 420.0 2.5 180.0 20.0\n";
        let climate = parse_climate_from_str(source, ParserMode::Strict).expect("strict climate");
        let request = build_hillslope_climate_runtime_request(&climate).expect("climate request");
        let configuration = shadow
            .snow_free_provider_configuration(&template)
            .expect("owner-derived provider configuration");
        let receipts = request
            .snow_free_half_hour_forcing_receipts(
                0,
                &configuration,
                &mut SnowFreeHalfHourProviderCursor::default(),
            )
            .expect("sealed provider receipts");
        let projected = shadow
            .project_repository_forcing_receipts(&receipts, template)
            .expect("real Child4 forcing projection");
        assert_eq!(projected.intervals.len(), 48);
        assert_eq!(
            projected.intervals[0]
                .vegetation_forcing
                .air_temperature_k
                .to_bits(),
            celsius_to_kelvin(receipts[0].intervals[0].air_temperature_c).to_bits()
        );
        for interval in &projected.intervals {
            interval
                .lse_forcing
                .validate(interval.lse_forcing.transaction_id)
                .expect("projected LSE forcing");
        }
        let production = fixture.hydrology.beginning_frame().clone();
        let production_input = production_day_input();
        let day_frame = projected_day(&production, &production_input);
        let receipt = shadow
            .execute_day(&production, &[day_frame], &[production_input], &projected)
            .expect("real Child4 consumes provider forcing");
        assert_eq!(receipt.accepted_interval_count, 48);
    }

    fn projected_day(
        production: &DirectRunFrame,
        input: &DirectPublicationDayInput,
    ) -> DirectDayFrame {
        let mut day = production.seed_day_frame(0, 0).expect("repository day");
        day.forcing.precipitation_m = input.precipitation_m;
        day.forcing.effective_temperature_c = input.effective_temperature_c;
        day
    }

    fn soil_candidates(fixture: &EndpointFixture) -> Vec<SoilThermalTileCandidate> {
        fixture
            .lse_configuration
            .ofes
            .iter()
            .flat_map(|ofe| {
                let beginning = fixture
                    .thermal
                    .ofes
                    .iter()
                    .find(|value| value.ofe_id == ofe.ofe_id)
                    .expect("beginning OFE");
                ofe.tiles.iter().enumerate().map(move |(tile_index, tile)| {
                    SoilThermalTileCandidate {
                        owner_id: fixture.thermal.owner_id.clone(),
                        beginning_state_sha256: fixture.thermal.state_sha256.clone(),
                        ofe_id: ofe.ofe_id.clone(),
                        tile_id: tile.tile_id.clone(),
                        layers: beginning
                            .ordered_layers
                            .iter()
                            .enumerate()
                            .map(|(layer_index, layer)| {
                                let credit = if layer_index == 0 {
                                    if tile_index == 0 { 10.0 } else { 20.0 }
                                } else {
                                    0.0
                                };
                                SoilThermalLayerCandidate {
                                    layer_id: layer.layer_id.clone(),
                                    beginning_enthalpy_j_m2_ofe_ground: layer
                                        .enthalpy_j_m2_ofe_ground,
                                    ground_heat_credit_j_m2_ofe_ground: credit,
                                    infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
                                    ending_enthalpy_j_m2_ofe_ground: layer.enthalpy_j_m2_ofe_ground
                                        + credit,
                                    ending_temperature_k: layer.temperature_k,
                                }
                            })
                            .collect(),
                    }
                })
            })
            .collect()
    }

    #[test]
    fn forty_eight_interval_day_replaces_only_complete_shadow_state() {
        let (mut shadow, fixture) = shadow_fixture();
        let production = fixture.hydrology.beginning_frame().clone();
        let production_before = production.clone();
        let input = day_input(&fixture);
        let production_input = production_day_input();
        let projected = projected_day(&production, &production_input);
        let receipt = shadow
            .execute_day(&production, &[projected], &[production_input], &input)
            .expect("complete shadow day");
        assert_eq!(receipt.accepted_interval_count, 48);
        assert_eq!(receipt.first_transaction_id, TransactionId(41));
        assert_eq!(receipt.last_transaction_id, TransactionId(88));
        assert_eq!(shadow.accepted_interval_count(), 48);
        assert_eq!(shadow.vegetation_state().0.last_transaction_id, 88);
        assert_eq!(production, production_before);
        assert_ne!(
            receipt.beginning_shadow_diagnostic_fingerprint,
            receipt.ending_shadow_diagnostic_fingerprint
        );
    }

    #[test]
    fn failed_late_interval_rolls_back_every_shadow_and_production_byte() {
        let (mut shadow, fixture) = shadow_fixture();
        let production = fixture.hydrology.beginning_frame().clone();
        let production_before = production.clone();
        let shadow_before = shadow.clone();
        let mut input = day_input(&fixture);
        input.intervals[47].lse_forcing.snow_present_at_end = true;
        let production_input = production_day_input();
        let projected = projected_day(&production, &production_input);
        assert!(matches!(
            shadow.execute_day(&production, &[projected], &[production_input], &input),
            Err(DirectV9RealConsumerError::Unsupported(_))
        ));
        assert_eq!(shadow, shadow_before);
        assert_eq!(production, production_before);
    }

    #[test]
    fn retained_half_day_restart_is_byte_identical_to_uninterrupted_day() {
        let (mut uninterrupted, fixture) = shadow_fixture();
        let input = day_input(&fixture);
        for (index, interval) in input.intervals.iter().enumerate() {
            uninterrupted
                .execute_interval(0, index, interval)
                .expect("uninterrupted interval");
        }
        let (mut first_half, _) = shadow_fixture();
        for (index, interval) in input.intervals[..24].iter().enumerate() {
            first_half
                .execute_interval(0, index, interval)
                .expect("first restart half");
        }
        let vegetation: V9CoupledOwnedState = serde_json::from_slice(
            &serde_json::to_vec(&first_half.vegetation_state).expect("vegetation checkpoint"),
        )
        .expect("vegetation reload");
        let lse: LandSurfaceEnergyState = serde_json::from_slice(
            &serde_json::to_vec(&first_half.lse_state).expect("LSE checkpoint"),
        )
        .expect("LSE reload");
        let soil: SoilThermalSnapshot = serde_json::from_slice(
            &serde_json::to_vec(&first_half.soil_thermal).expect("soil checkpoint"),
        )
        .expect("soil reload");
        let bgc: BiogeochemistryState = serde_json::from_slice(
            &serde_json::to_vec(&first_half.biogeochemistry).expect("BGC checkpoint"),
        )
        .expect("BGC reload");
        let mut checkpoint = first_half.checkpoint();
        checkpoint.shadow.vegetation_state = vegetation;
        checkpoint.shadow.lse_state = lse;
        checkpoint.shadow.soil_thermal = soil;
        checkpoint.shadow.biogeochemistry = bgc;
        let mut restarted = DirectV9RealConsumerShadow::restore(checkpoint)
            .expect("complete typed restart owner reload");
        for (index, interval) in input.intervals[24..].iter().enumerate() {
            restarted
                .execute_interval(0, index + 24, interval)
                .expect("second restart half");
        }
        assert_eq!(restarted, uninterrupted);
        assert_eq!(
            restarted
                .diagnostic_fingerprint()
                .expect("restarted fingerprint"),
            uninterrupted
                .diagnostic_fingerprint()
                .expect("uninterrupted fingerprint")
        );
    }

    #[test]
    fn shared_soil_thermal_aggregation_is_ordered_complete_and_owner_bound() {
        let (_, fixture) = shadow_fixture();
        let candidates = soil_candidates(&fixture);
        let ending = aggregate_soil_thermal_ending(
            &fixture.thermal,
            &fixture.lse_configuration,
            TransactionId(41),
            &candidates,
        )
        .expect("complete shared aggregate");
        let expected_credit = candidates
            .iter()
            .map(|candidate| candidate.layers[0].ground_heat_credit_j_m2_ofe_ground)
            .sum::<f64>();
        assert_eq!(
            ending.ofes[0].ordered_layers[0]
                .enthalpy_j_m2_ofe_ground
                .to_bits(),
            (fixture.thermal.ofes[0].ordered_layers[0].enthalpy_j_m2_ofe_ground + expected_credit)
                .to_bits()
        );
        let mut reversed = candidates.clone();
        reversed.reverse();
        assert_eq!(
            aggregate_soil_thermal_ending(
                &fixture.thermal,
                &fixture.lse_configuration,
                TransactionId(41),
                &reversed,
            )
            .expect("canonical tile order"),
            ending
        );
        let mut omitted = candidates.clone();
        omitted.pop();
        assert!(
            aggregate_soil_thermal_ending(
                &fixture.thermal,
                &fixture.lse_configuration,
                TransactionId(41),
                &omitted,
            )
            .is_err()
        );
        let mut duplicate = candidates.clone();
        duplicate.push(candidates[0].clone());
        assert!(
            aggregate_soil_thermal_ending(
                &fixture.thermal,
                &fixture.lse_configuration,
                TransactionId(41),
                &duplicate,
            )
            .is_err()
        );
        let mut wrong_owner = candidates;
        wrong_owner[0].owner_id = ResourceOwnerId::try_new("wrong-soil-owner").expect("owner");
        assert!(
            aggregate_soil_thermal_ending(
                &fixture.thermal,
                &fixture.lse_configuration,
                TransactionId(41),
                &wrong_owner,
            )
            .is_err()
        );
        let mut extra_tile = wrong_owner;
        extra_tile[0].owner_id = fixture.thermal.owner_id.clone();
        extra_tile[0].tile_id = TileId::try_new("nonexistent-extra-tile").expect("tile");
        assert!(
            aggregate_soil_thermal_ending(
                &fixture.thermal,
                &fixture.lse_configuration,
                TransactionId(41),
                &extra_tile,
            )
            .is_err()
        );
        let mut extra_layer = soil_candidates(&fixture);
        let repeated_layer = extra_layer[0].layers[0].clone();
        extra_layer[0].layers.push(repeated_layer);
        assert!(
            aggregate_soil_thermal_ending(
                &fixture.thermal,
                &fixture.lse_configuration,
                TransactionId(41),
                &extra_layer,
            )
            .is_err()
        );
    }

    #[test]
    fn mixed_complete_owner_lineage_is_rejected_before_execution() {
        let (mut shadow, _) = shadow_fixture();
        shadow.lse_state.last_accepted_transaction_id = Some(TransactionId(39));
        assert!(shadow.validate_complete_owner_set().is_err());
        let (mut shadow, _) = shadow_fixture();
        shadow.soil_thermal.last_accepted_transaction_id = Some(TransactionId(39));
        assert!(shadow.validate_complete_owner_set().is_err());
        let (mut shadow, _) = shadow_fixture();
        shadow.layer_maps[0].ofe_lane.lane_id = u32::MAX;
        assert!(shadow.validate_complete_owner_set().is_err());
    }

    #[test]
    fn explicit_scheduler_consumer_advances_shadow_without_changing_production() {
        let (mut shadow, fixture) = shadow_fixture();
        let mut baseline = fixture.hydrology.beginning_frame().clone();
        let mut observed = baseline.clone();
        let shadow_input = day_input(&fixture);
        let production_input = production_day_input();
        let metadata = DirectPublicationRunMetadata {
            run_name: "v9-real-consumer-shadow".into(),
            runtime_selection: "direct-default-off-shadow-test".into(),
            output_policy: "test-only".into(),
        };
        let executor = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly);
        let mut baseline_rows = Vec::new();
        let baseline_report = executor
            .run_publication_stream_with_interleaved_day_inputs_and_day_frames(
                &mut baseline,
                metadata.clone(),
                |_, _, _| Ok(production_input.clone()),
                |row, _| {
                    baseline_rows.push(row.clone());
                    Ok(())
                },
            )
            .expect("baseline production run");
        let mut observed_rows = Vec::new();
        let observed_report = executor
            .run_publication_stream_with_v9_real_consumer_shadow(
                &mut observed,
                metadata,
                |_, _, _| Ok(production_input.clone()),
                |_, _, _| Ok(shadow_input.clone()),
                |row, _| {
                    observed_rows.push(row.clone());
                    Ok(())
                },
                &mut shadow,
            )
            .expect("explicit default-off shadow run");
        assert_eq!(observed, baseline);
        assert_eq!(observed_rows, baseline_rows);
        assert_eq!(observed_report, baseline_report);
        assert_eq!(shadow.accepted_interval_count(), INTERVALS_PER_DAY as u64);
    }

    #[test]
    fn downstream_scheduler_failure_discards_production_and_complete_shadow_candidate() {
        let (mut shadow, fixture) = shadow_fixture();
        let mut production = fixture.hydrology.beginning_frame().clone();
        let production_before = production.clone();
        let shadow_before = shadow.clone();
        let shadow_input = day_input(&fixture);
        let production_input = production_day_input();
        let error = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
            .run_publication_stream_with_v9_real_consumer_shadow(
                &mut production,
                DirectPublicationRunMetadata {
                    run_name: "v9-shadow-rollback".into(),
                    runtime_selection: "direct-default-off-shadow-test".into(),
                    output_policy: "test-only".into(),
                },
                |_, _, _| Ok(production_input.clone()),
                |_, _, _| Ok(shadow_input.clone()),
                |_, _| {
                    Err(crate::DirectRuntimeError::PublicationSinkFailure {
                        detail: "injected after shadow day".into(),
                    })
                },
                &mut shadow,
            )
            .expect_err("injected downstream failure");
        assert!(matches!(
            error,
            crate::DirectRuntimeError::PublicationSinkFailure { .. }
        ));
        assert_eq!(production, production_before);
        assert_eq!(shadow, shadow_before);
    }

    #[test]
    fn active_routing_is_typed_unsupported_before_any_shadow_or_production_change() {
        let (mut shadow, fixture) = shadow_fixture();
        let mut production = fixture.hydrology.beginning_frame().clone();
        production.laned_active = Some(Box::new(DirectLanedActiveConfig {
            lanes: vec![DirectLanedActiveLaneConfig {
                slplen_m: 10.0,
                width_m: 10.0,
                mean_gradient: 0.01,
                skin_friction_coefficient_ko: 500.0,
                form_drag_coefficient: 0.0,
                roughness_element_height_m: 0.0,
                roughness_concentration: 0.0,
                vegetation_drag_coefficient: 0.0,
                canopy_height_m: None,
            }],
            mesh_policy: DirectLanedActiveMeshPolicy::FixedCells { cells: 10 },
            max_dt_s: 300.0,
            trace_enabled: false,
            trace_detail_filter: None,
            step_trace_enabled: false,
        }));
        let production_before = production.clone();
        let shadow_before = shadow.clone();
        let shadow_input = day_input(&fixture);
        let production_input = production_day_input();
        let error = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
            .run_publication_stream_with_v9_real_consumer_shadow(
                &mut production,
                DirectPublicationRunMetadata {
                    run_name: "v9-active-unsupported".into(),
                    runtime_selection: "direct-default-off-shadow-test".into(),
                    output_policy: "test-only".into(),
                },
                |_, _, _| Ok(production_input.clone()),
                |_, _, _| Ok(shadow_input.clone()),
                |_, _| Ok(()),
                &mut shadow,
            )
            .expect_err("active routing must reject");
        assert!(matches!(
            error,
            crate::DirectRuntimeError::DirectDomainViolation {
                field: "v9_shadow.laned_active_unsupported"
            }
        ));
        assert_eq!(production, production_before);
        assert_eq!(shadow, shadow_before);
    }

    #[test]
    fn repository_day_receipt_mismatch_discards_both_candidates() {
        let (mut shadow, fixture) = shadow_fixture();
        let mut production = fixture.hydrology.beginning_frame().clone();
        let production_before = production.clone();
        let shadow_before = shadow.clone();
        let shadow_input = day_input(&fixture);
        let mut actual_input = production_day_input();
        actual_input.precipitation_m = f64::from_bits(actual_input.precipitation_m.to_bits() ^ 1);
        let mut published_row_count = 0_usize;
        let error = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
            .run_publication_stream_with_v9_real_consumer_shadow(
                &mut production,
                DirectPublicationRunMetadata {
                    run_name: "v9-provider-poison".into(),
                    runtime_selection: "direct-default-off-shadow-test".into(),
                    output_policy: "test-only".into(),
                },
                |_, _, _| Ok(actual_input.clone()),
                |_, _, _| Ok(shadow_input.clone()),
                |_, _| {
                    published_row_count += 1;
                    Ok(())
                },
                &mut shadow,
            )
            .expect_err("repository receipt mismatch");
        assert!(matches!(
            error,
            crate::DirectRuntimeError::V9RealConsumerShadowFailure {
                category: "identity",
                ..
            }
        ));
        assert_eq!(published_row_count, 0);
        assert_eq!(production, production_before);
        assert_eq!(shadow, shadow_before);
    }
}
