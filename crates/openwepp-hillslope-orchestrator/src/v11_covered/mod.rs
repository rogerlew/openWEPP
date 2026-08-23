//! Snow-covered V11 carrier iteration, receipts, and imported-stack execution.

use super::*;

mod owner_finalization;
pub use owner_finalization::CoveredParentOwnerJoinReceiptV1;
pub(super) use owner_finalization::*;

struct CoveredFixedPointPolicy {
    max_iterations: usize,
    temperature_abs_k: f64,
    temperature_rel: f64,
    humidity_abs_kg_kg: f64,
    humidity_rel: f64,
    flux_abs_w_m2: f64,
    flux_rel: f64,
    vapor_abs_kg_m2_s: f64,
    vapor_rel: f64,
    depth_abs_m: f64,
    state_temperature_abs_k: f64,
    mass_abs_kg_m2: f64,
    energy_abs_j_m2: f64,
}

// Reviewed execution policy for the covered outer solve. Each norm is kept in
// its native units; no mixed-unit scalar tolerance is used.
const COVERED_FIXED_POINT_POLICY: CoveredFixedPointPolicy = CoveredFixedPointPolicy {
    max_iterations: 32,
    temperature_abs_k: 1.0e-8,
    temperature_rel: 1.0e-10,
    humidity_abs_kg_kg: 1.0e-12,
    humidity_rel: 1.0e-8,
    flux_abs_w_m2: 1.0e-7,
    flux_rel: 1.0e-9,
    vapor_abs_kg_m2_s: 1.0e-12,
    vapor_rel: 1.0e-6,
    depth_abs_m: 1.0e-9,
    state_temperature_abs_k: 1.0e-8,
    mass_abs_kg_m2: 1.0e-6,
    energy_abs_j_m2: 1.0e-6,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ComponentResolvedCarrierReceiptV1 {
    pub support: openwepp_coupled_time::TimeSupport,
    pub destination: (OfeId, TileId),
    pub final_boundary_receipt_sha256: Digest32,
    pub optical_receipt_sha256: Digest32,
    pub reciprocal_longwave_receipt_sha256: Digest32,
    pub components: Vec<CoveredCarrierComponentState>,
    pub shared_air_temperature_k: f64,
    pub shared_air_specific_humidity_kg_kg: f64,
    pub canopy_sensible_w_m2: f64,
    pub canopy_vapor_kg_m2_s: f64,
    pub snow_sensible_to_canopy_air_w_m2: f64,
    pub snow_vapor_to_canopy_air_kg_m2_s: f64,
    pub sensible_to_reference_air_w_m2: f64,
    pub vapor_to_reference_air_kg_m2_s: f64,
    pub receipt_sha256: Digest32,
}

impl ComponentResolvedCarrierReceiptV1 {
    fn try_new(
        destination: (OfeId, TileId),
        state: &CoveredLseIterationState,
        boundary: &FinalStage3CanopyBoundaryReceiptV1,
    ) -> Result<Self, DirectV11RealConsumerError> {
        let sensible = state
            .component_carrier_surfaces
            .iter()
            .map(|surface| surface.sensible_to_canopy_air_w_m2)
            .sum::<f64>();
        let vapor = state
            .component_carrier_surfaces
            .iter()
            .map(|surface| surface.vapor_to_canopy_air_kg_m2_s)
            .sum::<f64>();
        if state.component_carrier_surfaces.is_empty()
            || sensible.to_bits() != state.canopy_sensible_w_m2.to_bits()
            || vapor.to_bits() != state.canopy_vapor_kg_m2_s.to_bits()
            || boundary.sensible_to_canopy_air_w_m2.to_bits() != state.snow_sensible_w_m2.to_bits()
            || boundary.vapor_to_canopy_air_kg_m2_s.to_bits() != state.snow_vapor_kg_m2_s.to_bits()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "component-resolved carrier surface join",
            ));
        }
        let heat_reference = sensible + state.snow_sensible_w_m2;
        let vapor_reference = vapor + state.snow_vapor_kg_m2_s;
        if !close_with_policy(
            heat_reference,
            state.sensible_to_reference_air_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_rel,
        ) || !close_with_policy(
            vapor_reference,
            state.vapor_to_reference_air_kg_m2_s,
            COVERED_FIXED_POINT_POLICY.vapor_abs_kg_m2_s,
            COVERED_FIXED_POINT_POLICY.vapor_rel,
        ) {
            return Err(DirectV11RealConsumerError::Identity(
                "component-resolved carrier reference flux join",
            ));
        }
        let mut value = Self {
            support: boundary.support,
            destination,
            final_boundary_receipt_sha256: boundary.receipt_sha256,
            optical_receipt_sha256: boundary.optical_receipt_sha256,
            reciprocal_longwave_receipt_sha256: boundary.reciprocal_longwave_receipt_sha256,
            components: state.component_carrier_surfaces.clone(),
            shared_air_temperature_k: state.canopy_air_temperature_k,
            shared_air_specific_humidity_kg_kg: state.canopy_air_specific_humidity_kg_kg,
            canopy_sensible_w_m2: sensible,
            canopy_vapor_kg_m2_s: vapor,
            snow_sensible_to_canopy_air_w_m2: state.snow_sensible_w_m2,
            snow_vapor_to_canopy_air_kg_m2_s: state.snow_vapor_kg_m2_s,
            sensible_to_reference_air_w_m2: state.sensible_to_reference_air_w_m2,
            vapor_to_reference_air_kg_m2_s: state.vapor_to_reference_air_kg_m2_s,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest()?;
        value.validate(boundary)?;
        Ok(value)
    }

    fn validate(
        &self,
        boundary: &FinalStage3CanopyBoundaryReceiptV1,
    ) -> Result<(), DirectV11RealConsumerError> {
        boundary.validate()?;
        if self.support != boundary.support
            || self.destination != boundary.destination
            || self.final_boundary_receipt_sha256 != boundary.receipt_sha256
            || self.optical_receipt_sha256 != boundary.optical_receipt_sha256
            || self.reciprocal_longwave_receipt_sha256
                != boundary.reciprocal_longwave_receipt_sha256
            || self.snow_sensible_to_canopy_air_w_m2.to_bits()
                != boundary.sensible_to_canopy_air_w_m2.to_bits()
            || self.snow_vapor_to_canopy_air_kg_m2_s.to_bits()
                != boundary.vapor_to_canopy_air_kg_m2_s.to_bits()
            || !self.shared_air_temperature_k.is_finite()
            || !(200.0..=350.0).contains(&self.shared_air_temperature_k)
            || !self.shared_air_specific_humidity_kg_kg.is_finite()
            || self.shared_air_specific_humidity_kg_kg < 0.0
            || self.receipt_sha256 != self.reconstructed_digest()?
        {
            return Err(DirectV11RealConsumerError::Identity(
                "component carrier boundary/seal join",
            ));
        }
        let mut prior: Option<(u32, &str, u8)> = None;
        let mut sensible = 0.0;
        let mut vapor = 0.0;
        let mut emissive_area = 0.0;
        let mut occupancy_ids = BTreeSet::new();
        for (index, component) in self.components.iter().enumerate() {
            let identity = (
                component.vertical_occupancy_ordinal,
                component.occupancy_id.as_str(),
                component.component_ordinal,
            );
            if component.occupancy_id.is_empty()
                || component.component_ordinal != (index % 4) as u8
                || component.vertical_occupancy_ordinal != (index / 4) as u32
                || (index % 4 != 0
                    && self.components[index - 1].occupancy_id != component.occupancy_id)
                || (index % 4 == 0 && !occupancy_ids.insert(component.occupancy_id.as_str()))
                || prior.is_some_and(|value| value >= identity)
                || [
                    component.surface_area_m2_m2_tile,
                    component.emissive_area_m2_m2_tile,
                    component.heat_conductance_m_s_tile,
                    component.vapor_conductance_m_s_tile,
                    component.temperature_k,
                    component.specific_humidity_kg_kg,
                    component.sensible_to_canopy_air_w_m2,
                    component.vapor_to_canopy_air_kg_m2_s,
                ]
                .iter()
                .any(|value| !value.is_finite())
                || component
                    .vapor_authorization_kg_m2_tile_s
                    .is_some_and(|value| !value.is_finite() || value < 0.0)
                || component.surface_area_m2_m2_tile < 0.0
                || component.emissive_area_m2_m2_tile < 0.0
                || component.heat_conductance_m_s_tile < 0.0
                || component.vapor_conductance_m_s_tile < 0.0
                || !(200.0..=350.0).contains(&component.temperature_k)
                || component.specific_humidity_kg_kg < 0.0
                || (component.component_ordinal != 2
                    && component.vapor_authorization_kg_m2_tile_s.is_some())
                || (component.component_ordinal == 3
                    && (component.vapor_conductance_m_s_tile.to_bits() != 0.0_f64.to_bits()
                        || component.vapor_to_canopy_air_kg_m2_s.to_bits() != 0.0_f64.to_bits()))
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "component carrier canonical component",
                ));
            }
            sensible += component.sensible_to_canopy_air_w_m2;
            vapor += component.vapor_to_canopy_air_kg_m2_s;
            emissive_area += component.emissive_area_m2_m2_tile;
            prior = Some(identity);
        }
        if !emissive_area.is_finite()
            || emissive_area <= 0.0
            || sensible.to_bits() != self.canopy_sensible_w_m2.to_bits()
            || vapor.to_bits() != self.canopy_vapor_kg_m2_s.to_bits()
            || !close_with_policy(
                sensible + self.snow_sensible_to_canopy_air_w_m2,
                self.sensible_to_reference_air_w_m2,
                COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
                COVERED_FIXED_POINT_POLICY.flux_rel,
            )
            || !close_with_policy(
                vapor + self.snow_vapor_to_canopy_air_kg_m2_s,
                self.vapor_to_reference_air_kg_m2_s,
                COVERED_FIXED_POINT_POLICY.vapor_abs_kg_m2_s,
                COVERED_FIXED_POINT_POLICY.vapor_rel,
            )
        {
            return Err(DirectV11RealConsumerError::Identity(
                "component carrier independent flux reconstruction",
            ));
        }
        Ok(())
    }

    fn reconstructed_digest(&self) -> Result<Digest32, DirectV11RealConsumerError> {
        let component_bytes = self
            .components
            .iter()
            .flat_map(|surface| {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(&surface.vertical_occupancy_ordinal.to_be_bytes());
                bytes.extend_from_slice(&(surface.occupancy_id.len() as u64).to_be_bytes());
                bytes.extend_from_slice(surface.occupancy_id.as_bytes());
                bytes.push(surface.component_ordinal);
                for value in [
                    surface.surface_area_m2_m2_tile,
                    surface.emissive_area_m2_m2_tile,
                    surface.heat_conductance_m_s_tile,
                    surface.vapor_conductance_m_s_tile,
                    surface.temperature_k,
                    surface.specific_humidity_kg_kg,
                    surface.sensible_to_canopy_air_w_m2,
                    surface.vapor_to_canopy_air_kg_m2_s,
                ] {
                    bytes.extend_from_slice(&value.to_bits().to_be_bytes());
                }
                match surface.vapor_authorization_kg_m2_tile_s {
                    Some(value) => {
                        bytes.push(1);
                        bytes.extend_from_slice(&value.to_bits().to_be_bytes());
                    }
                    None => bytes.push(0),
                }
                bytes
            })
            .collect::<Vec<_>>();
        let scalar_bytes = [
            self.canopy_sensible_w_m2,
            self.canopy_vapor_kg_m2_s,
            self.snow_sensible_to_canopy_air_w_m2,
            self.snow_vapor_to_canopy_air_kg_m2_s,
            self.sensible_to_reference_air_w_m2,
            self.vapor_to_reference_air_kg_m2_s,
        ]
        .into_iter()
        .flat_map(|value| value.to_bits().to_be_bytes())
        .collect::<Vec<_>>();
        openwepp_coupled_time::framed_sha256(
            "component-resolved-covered-carrier-v1",
            &[
                openwepp_coupled_time::FramedField {
                    tag: "support_start_ns",
                    value: &self.support.start_ns().get().to_be_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "support_end_ns",
                    value: &self.support.end_ns().get().to_be_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "ofe_id",
                    value: self.destination.0.as_str().as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "tile_id",
                    value: self.destination.1.as_str().as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "components",
                    value: &component_bytes,
                },
                openwepp_coupled_time::FramedField {
                    tag: "final_boundary_receipt",
                    value: self.final_boundary_receipt_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "optical_receipt",
                    value: self.optical_receipt_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "reciprocal_longwave_receipt",
                    value: self.reciprocal_longwave_receipt_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "shared_air_temperature_k",
                    value: &self.shared_air_temperature_k.to_bits().to_be_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "shared_air_specific_humidity",
                    value: &self
                        .shared_air_specific_humidity_kg_kg
                        .to_bits()
                        .to_be_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "fluxes",
                    value: &scalar_bytes,
                },
            ],
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("component carrier receipt digest"))
    }
}

struct LaneStage3BoundaryTerms {
    fractions: f64,
    provisional_carrier_bytes: Vec<u8>,
    provisional_carrier_receipt_sha256: Digest32,
    sensible_to_canopy_air_w_m2: f64,
    vapor_to_canopy_air_kg_m2_s: f64,
    latent_energy_to_canopy_air_j_m2: f64,
    snow_absorbed_shortwave_w_m2: f64,
    snow_net_longwave_w_m2: f64,
    snow_temperature_k: f64,
    latent_heat_j_kg: f64,
    common_snow_temperature_k: Option<f64>,
    common_latent_heat_j_kg: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CoveredCarrierInitialGuessV1 {
    snow_temperature_k: f64,
    snow_sensible_into_surface_w_m2: f64,
    snow_vapor_into_surface_kg_m2_s: f64,
    snow_longwave_net_w_m2: f64,
    pub(crate) diagnostic_sha256: Digest32,
}

/// One explicit default-off invocation of the actual `DirectV10` owner stack.
#[derive(Clone)]
pub struct DirectV11RealConsumerStack<'a> {
    pub beginning: DirectV10RealConsumerShadow,
    pub interval: &'a DirectV9ShadowIntervalInput,
    pub day_index: usize,
    pub interval_index: usize,
    pub(super) ending: Option<DirectV10RealConsumerShadow>,
    pub(super) last_support_receipt: Option<LseSupportAdmissibilityReceiptV1>,
    pub(super) ending_snow_owner_bytes: Option<Vec<u8>>,
}

/// Explicit covered lower-boundary adopter for the V11 imported transaction.
///
/// This type is intentionally separate from [`DirectV11RealConsumerStack`].
/// It evaluates the Child-2C carrier and the actual persistent Stage-3
/// transition from the same beginning states and support before it constructs
/// the V11 canopy/soil owner candidate.
#[derive(Clone)]
pub struct DirectV11SnowCoveredRealConsumerStack<'a> {
    pub beginning: DirectV10RealConsumerShadow,
    pub interval: &'a DirectV11SnowCoveredSegmentInput,
    pub stage3_inputs_by_lane: &'a BTreeMap<u32, DirectActiveSnowPartitionInputs>,
    pub stage3_forcing_by_lane: &'a BTreeMap<u32, DirectSnowStage3SupportInput>,
    pub carrier_forcing_by_lane: &'a BTreeMap<u32, SealedCoveredCarrierForcing>,
    pub stage3_beginning_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub day_index: usize,
    pub interval_index: usize,
    ending: Option<DirectV10RealConsumerShadow>,
    ending_stage3_by_lane: Option<BTreeMap<u32, DirectSnowStage3PersistentState>>,
    last_support_receipt: Option<LseSupportAdmissibilityReceiptV1>,
    last_final_boundary_receipts:
        Option<BTreeMap<(OfeId, TileId), FinalStage3CanopyBoundaryReceiptV1>>,
    last_lane_boundary_receipts: Option<BTreeMap<u32, LaneStage3BoundaryReceiptV1>>,
    last_component_carrier_receipts:
        Option<BTreeMap<(OfeId, TileId), ComponentResolvedCarrierReceiptV1>>,
}

pub struct DirectV11SnowCoveredStackInputs<'a> {
    pub interval: &'a DirectV11SnowCoveredSegmentInput,
    pub stage3_inputs_by_lane: &'a BTreeMap<u32, DirectActiveSnowPartitionInputs>,
    pub stage3_forcing_by_lane: &'a BTreeMap<u32, DirectSnowStage3SupportInput>,
    pub carrier_forcing_by_lane: &'a BTreeMap<u32, SealedCoveredCarrierForcing>,
    pub stage3_beginning_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub day_index: usize,
    pub interval_index: usize,
}

impl<'a> DirectV11SnowCoveredRealConsumerStack<'a> {
    #[must_use]
    pub fn new(
        beginning: &DirectV10RealConsumerShadow,
        inputs: DirectV11SnowCoveredStackInputs<'a>,
    ) -> Self {
        Self {
            beginning: beginning.clone(),
            interval: inputs.interval,
            stage3_inputs_by_lane: inputs.stage3_inputs_by_lane,
            stage3_forcing_by_lane: inputs.stage3_forcing_by_lane,
            carrier_forcing_by_lane: inputs.carrier_forcing_by_lane,
            stage3_beginning_by_lane: inputs.stage3_beginning_by_lane,
            day_index: inputs.day_index,
            interval_index: inputs.interval_index,
            ending: None,
            ending_stage3_by_lane: None,
            last_support_receipt: None,
            last_final_boundary_receipts: None,
            last_lane_boundary_receipts: None,
            last_component_carrier_receipts: None,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn derive_live_carrier_input(
        &self,
        lane_id: u32,
        stage3_state: &DirectSnowStage3PersistentState,
        vegetation_state: &V8CoupledOwnedState,
        _stage3_forcing: DirectSnowStage3SupportInput,
        sealed: &SealedCoveredCarrierForcing,
        tile_override: Option<&TileId>,
        _interval_s: f64,
    ) -> Result<CoveredCarrierInitialGuessV1, DirectV11RealConsumerError> {
        let lane_index = self
            .stage3_beginning_by_lane
            .keys()
            .position(|value| *value == lane_id)
            .ok_or(DirectV11RealConsumerError::Identity("covered lane order"))?;
        let tile = match tile_override {
            Some(tile_id) => self
                .beginning
                .vegetation_configuration
                .topology_tiles
                .iter()
                .find(|tile| tile.tile_id == *tile_id)
                .ok_or(DirectV11RealConsumerError::Identity("covered carrier tile"))?,
            None => self
                .beginning
                .vegetation_configuration
                .topology_tiles
                .get(lane_index % self.beginning.vegetation_configuration.topology_tiles.len())
                .ok_or(DirectV11RealConsumerError::Identity("covered carrier tile"))?,
        };
        let tile_air = vegetation_state.tile_canopy_air.get(&tile.tile_id).ok_or(
            DirectV11RealConsumerError::Identity("committed canopy-air owner"),
        )?;
        let occupancies = self
            .beginning
            .vegetation_configuration
            .strata
            .iter()
            .filter(|stratum| stratum.tile_ids.iter().any(|id| id == &tile.tile_id))
            .filter_map(|stratum| {
                let identity = openwepp_kernel_contract::OccupancyId {
                    stratum_id: stratum.stratum_id.clone(),
                    tile_id: tile.tile_id.clone(),
                };
                vegetation_state
                    .occupancies
                    .get(&identity)
                    .map(|state| (stratum, state))
            })
            .collect::<Vec<_>>();
        if occupancies.is_empty() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered canopy owner topology",
            ));
        }
        let count = f64::from(
            u32::try_from(occupancies.len())
                .map_err(|_| DirectV11RealConsumerError::Identity("covered occupancy count"))?,
        );
        let leaf_temperature_k = occupancies
            .iter()
            .map(|(_, state)| {
                state
                    .sun_leaf_temperature_k
                    .midpoint(state.shade_leaf_temperature_k)
            })
            .sum::<f64>()
            / count;
        let stem_temperature_k = occupancies
            .iter()
            .map(|(_, state)| state.dry_stem_temperature_k)
            .sum::<f64>()
            / count;
        let canopy_wind = sealed.exposure.wind_m_s;
        let (canopy_heat, canopy_vapor) = occupancies.iter().try_fold(
            (0.0, 0.0),
            |(heat, vapor), (stratum, _)| -> Result<(f64, f64), DirectV11RealConsumerError> {
                let u_star = canopy_surface_friction_velocity(
                    canopy_wind,
                    self.interval.vegetation_forcing.reference_height_m,
                    stratum.displacement_m,
                    stratum.z0m_m,
                )
                .map_err(|_| DirectV11RealConsumerError::Identity("canopy wind exposure"))?;
                let leaf = leaf_boundary_conductance(u_star, stratum.leaf_dimension_m)
                    .map_err(|_| DirectV11RealConsumerError::Identity("leaf conductance"))?;
                let wet = leaf_boundary_conductance(u_star, stratum.wet_surface_dimension_m)
                    .map_err(|_| DirectV11RealConsumerError::Identity("wet conductance"))?;
                let stem = leaf_boundary_conductance(u_star, stratum.stem_dimension_m)
                    .map_err(|_| DirectV11RealConsumerError::Identity("stem conductance"))?;
                Ok((heat + (leaf + wet + stem) / 3.0, vapor + leaf.midpoint(wet)))
            },
        )?;
        let canopy_heat = canopy_heat / count;
        let canopy_vapor = canopy_vapor / count;
        let reference_resistance = neutral_resistance(
            sealed.exposure.transfer_height_m,
            0.0,
            sealed.exposure.roughness_m,
            sealed.exposure.roughness_m,
            sealed.exposure.wind_m_s,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("reference exposure"))?;
        let snow_resistance = reference_resistance;
        let snow_conductance = 1.0 / snow_resistance;
        let snow_temperature_k = stage3_state
            .layers
            .first()
            .map_or(273.15, |layer| layer.temperature_c + 273.15);
        let snow_temperature = TemperatureCelsius::try_new(snow_temperature_k - 273.15)
            .map_err(|_| DirectV11RealConsumerError::Identity("snow temperature"))?;
        let saturation_pressure_pa = kilopascals_to_pascals(
            saturation_vapor_pressure_ice_kpa(snow_temperature)
                .map_err(|_| DirectV11RealConsumerError::Identity("snow saturation pressure"))?
                .as_kilopascals(),
        );
        let air_pressure_pa = self.interval.lse_forcing.air_pressure_pa;
        if !air_pressure_pa.is_finite() || air_pressure_pa <= 0.378 * saturation_pressure_pa {
            return Err(DirectV11RealConsumerError::Identity(
                "snow surface humidity pressure",
            ));
        }
        let snow_humidity = (0.622 * saturation_pressure_pa
            / (air_pressure_pa - 0.378 * saturation_pressure_pa))
            .min(1.0);
        let reference_heat = 1.0 / reference_resistance;
        let reference = CarrierSurface {
            temperature_k: sealed.reference_temperature_k,
            specific_humidity: sealed.reference_specific_humidity,
            heat_conductance_m_s: reference_heat,
            vapor_conductance_m_s: reference_heat,
        };
        let canopy = CarrierSurface {
            temperature_k: tile_air.canopy_air_temperature_k,
            specific_humidity: tile_air.canopy_air_specific_humidity_kg_kg,
            heat_conductance_m_s: canopy_heat,
            vapor_conductance_m_s: canopy_vapor,
        };
        let snow = CarrierSurface {
            temperature_k: snow_temperature_k,
            specific_humidity: snow_humidity,
            heat_conductance_m_s: snow_conductance,
            vapor_conductance_m_s: snow_conductance,
        };
        let weight_sum = leaf_temperature_k + stem_temperature_k;
        let components = vec![
            CanopyLongwaveComponent {
                temperature_k: leaf_temperature_k,
                emissive_area_weight: leaf_temperature_k / weight_sum,
            },
            CanopyLongwaveComponent {
                temperature_k: stem_temperature_k,
                emissive_area_weight: stem_temperature_k / weight_sum,
            },
        ];
        let heat_total = reference.heat_conductance_m_s
            + canopy.heat_conductance_m_s
            + snow.heat_conductance_m_s;
        let vapor_total = reference.vapor_conductance_m_s
            + canopy.vapor_conductance_m_s
            + snow.vapor_conductance_m_s;
        let shared_temperature = (reference.heat_conductance_m_s * reference.temperature_k
            + canopy.heat_conductance_m_s * canopy.temperature_k
            + snow.heat_conductance_m_s * snow.temperature_k)
            / heat_total;
        let shared_humidity = (reference.vapor_conductance_m_s * reference.specific_humidity
            + canopy.vapor_conductance_m_s * canopy.specific_humidity
            + snow.vapor_conductance_m_s * snow.specific_humidity)
            / vapor_total;
        let snow_sensible = -sealed.rho_air_kg_m3
            * sealed.cp_air_j_kg_k
            * snow.heat_conductance_m_s
            * (snow.temperature_k - shared_temperature);
        let snow_vapor = -sealed.rho_air_kg_m3
            * snow.vapor_conductance_m_s
            * (snow.specific_humidity - shared_humidity);
        let sky_view = (1.0 - sealed.effective_canopy_cover).powf(1.6);
        let canopy_longwave = components
            .iter()
            .map(|component| {
                component.emissive_area_weight * 5.670_374_419e-8 * component.temperature_k.powi(4)
            })
            .sum::<f64>();
        let snow_emission = 5.670_374_419e-8 * snow.temperature_k.powi(4);
        let snow_longwave_net_w_m2 = sky_view * sealed.atmospheric_longwave_w_m2
            + (1.0 - sky_view) * canopy_longwave
            - snow_emission;
        let scalar_bytes = [
            shared_temperature,
            shared_humidity,
            snow_temperature_k,
            snow_sensible,
            snow_vapor,
            snow_longwave_net_w_m2,
        ]
        .into_iter()
        .flat_map(|value| value.to_bits().to_be_bytes())
        .collect::<Vec<_>>();
        let diagnostic_sha256 = openwepp_coupled_time::framed_sha256(
            "covered-carrier-initial-guess-v1",
            &[openwepp_coupled_time::FramedField {
                tag: "numerical_guess",
                value: &scalar_bytes,
            }],
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("covered carrier initial guess"))?;
        Ok(CoveredCarrierInitialGuessV1 {
            snow_temperature_k,
            snow_sensible_into_surface_w_m2: -snow_sensible,
            snow_vapor_into_surface_kg_m2_s: -snow_vapor,
            snow_longwave_net_w_m2,
            diagnostic_sha256,
        })
    }

    fn lane_stage3_terms_from_boundaries(
        &self,
        destination_receipts: &BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>,
        boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        interval_s: f64,
    ) -> Result<BTreeMap<u32, LaneStage3BoundaryTerms>, DirectV11RealConsumerError> {
        let mut lanes = BTreeMap::<u32, LaneStage3BoundaryTerms>::new();
        for (destination, carrier) in destination_receipts {
            let boundary =
                boundaries
                    .get(destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered Stage-3 lane boundary destination",
                    ))?;
            let binding = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered Stage-3 lane boundary OFE",
                ))?;
            let fraction = self.covered_destination_fraction(&destination.0, &destination.1)?;
            let entry = lanes.entry(binding.production_lane_id).or_insert_with(|| {
                LaneStage3BoundaryTerms {
                    fractions: 0.0,
                    provisional_carrier_bytes: Vec::new(),
                    provisional_carrier_receipt_sha256: Digest32::zero(),
                    sensible_to_canopy_air_w_m2: 0.0,
                    vapor_to_canopy_air_kg_m2_s: 0.0,
                    latent_energy_to_canopy_air_j_m2: 0.0,
                    snow_absorbed_shortwave_w_m2: 0.0,
                    snow_net_longwave_w_m2: 0.0,
                    snow_temperature_k: 0.0,
                    latent_heat_j_kg: 0.0,
                    common_snow_temperature_k: None,
                    common_latent_heat_j_kg: None,
                }
            });
            if entry
                .common_snow_temperature_k
                .is_some_and(|value| value.to_bits() != boundary.snow_temperature_k.to_bits())
                || entry
                    .common_latent_heat_j_kg
                    .is_some_and(|value| value.to_bits() != boundary.latent_heat_j_kg.to_bits())
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered Stage-3 lane common snow state",
                ));
            }
            entry.common_snow_temperature_k = Some(boundary.snow_temperature_k);
            entry.common_latent_heat_j_kg = Some(boundary.latent_heat_j_kg);
            entry.fractions += fraction;
            entry
                .provisional_carrier_bytes
                .extend_from_slice(destination.0.as_str().as_bytes());
            entry.provisional_carrier_bytes.push(0);
            entry
                .provisional_carrier_bytes
                .extend_from_slice(destination.1.as_str().as_bytes());
            entry
                .provisional_carrier_bytes
                .extend_from_slice(&fraction.to_bits().to_le_bytes());
            entry
                .provisional_carrier_bytes
                .extend_from_slice(carrier.diagnostic_sha256.as_bytes());
            entry.sensible_to_canopy_air_w_m2 += fraction * boundary.sensible_to_canopy_air_w_m2;
            entry.vapor_to_canopy_air_kg_m2_s += fraction * boundary.vapor_to_canopy_air_kg_m2_s;
            entry.latent_energy_to_canopy_air_j_m2 += fraction
                * boundary.vapor_to_canopy_air_kg_m2_s
                * boundary.latent_heat_j_kg
                * interval_s;
            entry.snow_absorbed_shortwave_w_m2 += fraction * boundary.shortwave_absorbed_w_m2;
            entry.snow_net_longwave_w_m2 += fraction * boundary.net_longwave_w_m2;
            entry.snow_temperature_k += fraction * boundary.snow_temperature_k;
            entry.latent_heat_j_kg += fraction * boundary.latent_heat_j_kg;
        }
        for terms in lanes.values_mut() {
            if !terms.fractions.is_finite()
                || (terms.fractions - 1.0).abs() > STAGE3_OFE_TILE_FRACTION_CLOSURE_TOLERANCE
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered Stage-3 lane is missing a snow-surface contribution",
                ));
            }
            terms.provisional_carrier_receipt_sha256 =
                digest_bytes(&terms.provisional_carrier_bytes);
            terms.snow_temperature_k =
                terms
                    .common_snow_temperature_k
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered Stage-3 lane snow temperature",
                    ))?;
            terms.latent_heat_j_kg =
                terms
                    .common_latent_heat_j_kg
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered Stage-3 lane latent heat",
                    ))?;
        }
        Ok(lanes)
    }

    fn final_lane_boundary_receipts(
        &self,
        input: &V11ImportedV10SegmentInput,
        final_receipts: &BTreeMap<(OfeId, TileId), FinalStage3CanopyBoundaryReceiptV1>,
    ) -> Result<BTreeMap<u32, LaneStage3BoundaryReceiptV1>, DirectV11RealConsumerError> {
        let topology_configuration_sha256 = self.covered_topology_digest();
        let mut grouped =
            BTreeMap::<u32, Vec<((OfeId, TileId), f64, &FinalStage3CanopyBoundaryReceiptV1)>>::new(
            );
        for (destination, receipt) in final_receipts {
            let lane_id = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered final lane boundary OFE",
                ))?
                .production_lane_id;
            grouped.entry(lane_id).or_default().push((
                destination.clone(),
                self.covered_destination_fraction(&destination.0, &destination.1)?,
                receipt,
            ));
        }
        grouped
            .into_iter()
            .map(|(lane_id, mut values)| {
                values.sort_by(|left, right| left.0.cmp(&right.0));
                let ofe_id = values.first().map(|value| value.0.0.clone()).ok_or(
                    DirectV11RealConsumerError::Identity("empty covered final lane boundary"),
                )?;
                let fraction_sum = values.iter().map(|value| value.1).sum::<f64>();
                if !fraction_sum.is_finite()
                    || (fraction_sum - 1.0).abs() > STAGE3_OFE_TILE_FRACTION_CLOSURE_TOLERANCE
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "covered final lane boundary is missing a snow-surface contribution",
                    ));
                }
                let mut contributions = Vec::with_capacity(values.len());
                let mut expected_topology = Vec::with_capacity(values.len());
                let mut aggregate = [0.0; 7];
                for (destination, fraction, receipt) in values {
                    if !fraction.is_finite() || fraction <= 0.0 {
                        return Err(DirectV11RealConsumerError::Identity(
                            "covered final lane boundary fraction",
                        ));
                    }
                    expected_topology.push(LaneBoundaryTopologyExpectationV1 {
                        tile_id: destination.1.clone(),
                        tile_fraction_bits: fraction.to_bits(),
                        boundary_class: Stage3TileBoundaryClassV1::V11CanopyCovered,
                        boundary_model_definition_sha256: digest_bytes(
                            b"OPENWEPP_FINAL_STAGE3_CANOPY_BOUNDARY_V1",
                        ),
                    });
                    let contribution = LaneBoundaryContributionV1 {
                        tile_id: destination.1.clone(),
                        tile_fraction: fraction,
                        boundary_class: Stage3TileBoundaryClassV1::V11CanopyCovered,
                        boundary_model_definition_sha256: digest_bytes(
                            b"OPENWEPP_FINAL_STAGE3_CANOPY_BOUNDARY_V1",
                        ),
                        beginning_stage3_state_sha256: receipt.beginning_stage3_state_sha256,
                        provisional_carrier_receipt_sha256: receipt
                            .provisional_carrier_receipt_sha256,
                        optical_receipt_sha256: receipt.optical_receipt_sha256,
                        reciprocal_longwave_receipt_sha256: receipt
                            .reciprocal_longwave_receipt_sha256,
                        final_boundary_receipt_sha256: receipt.receipt_sha256,
                        sensible_to_canopy_air_w_m2: receipt.sensible_to_canopy_air_w_m2,
                        vapor_to_canopy_air_kg_m2_s: receipt.vapor_to_canopy_air_kg_m2_s,
                        latent_energy_to_canopy_air_j_m2: receipt.latent_energy_to_canopy_air_j_m2,
                        snow_absorbed_shortwave_w_m2: receipt.snow_absorbed_shortwave_w_m2,
                        snow_net_longwave_w_m2: receipt.snow_net_longwave_w_m2,
                        snow_temperature_k: receipt.snow_temperature_k,
                        latent_heat_j_kg: receipt.latent_heat_j_kg,
                    };
                    for (index, value) in [
                        contribution.sensible_to_canopy_air_w_m2,
                        contribution.vapor_to_canopy_air_kg_m2_s,
                        contribution.latent_energy_to_canopy_air_j_m2,
                        contribution.snow_absorbed_shortwave_w_m2,
                        contribution.snow_net_longwave_w_m2,
                        contribution.snow_temperature_k,
                        contribution.latent_heat_j_kg,
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        aggregate[index] += fraction * value;
                    }
                    contributions.push(contribution);
                }
                let common_snow_temperature_k = contributions[0].snow_temperature_k;
                let common_latent_heat_j_kg = contributions[0].latent_heat_j_kg;
                let lane_receipt = LaneStage3BoundaryReceiptV1::try_new(
                    LaneStage3BoundaryReceiptV1 {
                        lane_id,
                        ofe_id,
                        support: input.support,
                        area_basis: Stage3LaneAreaBasisV1::OfeGround,
                        topology_configuration_sha256,
                        provisional_carrier_receipt_sha256: Digest32::zero(),
                        optical_receipt_sha256: Digest32::zero(),
                        reciprocal_longwave_receipt_sha256: Digest32::zero(),
                        final_destination_receipt_sha256: Digest32::zero(),
                        ordered_destinations: contributions,
                        aggregate_sensible_to_canopy_air_w_m2: aggregate[0],
                        aggregate_vapor_to_canopy_air_kg_m2_s: aggregate[1],
                        aggregate_latent_energy_to_canopy_air_j_m2: aggregate[2],
                        aggregate_snow_absorbed_shortwave_w_m2: aggregate[3],
                        aggregate_snow_net_longwave_w_m2: aggregate[4],
                        aggregate_snow_temperature_k: common_snow_temperature_k,
                        aggregate_latent_heat_j_kg: common_latent_heat_j_kg,
                        receipt_sha256: Digest32::zero(),
                    },
                    &expected_topology,
                )?;
                Ok((lane_id, lane_receipt))
            })
            .collect()
    }

    fn covered_topology_digest(&self) -> Digest32 {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"OPENWEPP_COVERED_TILE_TOPOLOGY_V1\0");
        for record in &self.beginning.inner.surface_configuration.records {
            bytes.extend_from_slice(record.key.ofe_id.as_str().as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(record.key.tile_id.as_str().as_bytes());
            bytes.extend_from_slice(&record.tile_fraction.to_bits().to_le_bytes());
        }
        digest_bytes(&bytes)
    }

    fn carrier_receipts_by_destination(
        &self,
        interval_s: f64,
        vegetation_state: &V8CoupledOwnedState,
        stage3_state_by_lane: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        stage3_forcing_by_lane: &BTreeMap<u32, DirectSnowStage3SupportInput>,
    ) -> Result<BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>, DirectV11RealConsumerError>
    {
        let surface = &self.beginning.inner.surface_configuration;
        let lane_to_ofe = self.covered_lane_to_ofe(stage3_state_by_lane)?;
        let expected_destinations = self.covered_expected_destinations();
        let configured_destinations = surface
            .records
            .iter()
            .map(|record| (record.key.ofe_id.clone(), record.key.tile_id.clone()))
            .filter(|destination| expected_destinations.contains(destination))
            .collect::<BTreeSet<_>>();
        if expected_destinations != configured_destinations {
            return Err(DirectV11RealConsumerError::Identity(
                "covered surface/LSE destination set",
            ));
        }

        let mut receipts = BTreeMap::new();
        for (ofe_id, tile_id) in expected_destinations {
            let binding = surface
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == ofe_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered destination OFE binding",
                ))?;
            if lane_to_ofe.get(&binding.production_lane_id) != Some(&ofe_id) {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered destination lane/OFE binding",
                ));
            }
            let carrier = self.carrier_for_destination(
                interval_s,
                binding.production_lane_id,
                &ofe_id,
                &tile_id,
                vegetation_state,
                stage3_state_by_lane,
                stage3_forcing_by_lane,
            )?;
            if receipts.insert((ofe_id, tile_id), carrier).is_some() {
                return Err(DirectV11RealConsumerError::Identity(
                    "duplicate covered destination carrier receipt",
                ));
            }
        }
        Ok(receipts)
    }

    fn covered_destination_fraction(
        &self,
        ofe_id: &OfeId,
        tile_id: &TileId,
    ) -> Result<f64, DirectV11RealConsumerError> {
        let record = self
            .beginning
            .inner
            .surface_configuration
            .records
            .iter()
            .find(|record| record.key.ofe_id == *ofe_id && record.key.tile_id == *tile_id)
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered destination tile fraction",
            ))?;
        if !record.tile_fraction.is_finite() || record.tile_fraction <= 0.0 {
            return Err(DirectV11RealConsumerError::Identity(
                "covered destination tile fraction domain",
            ));
        }
        Ok(record.tile_fraction)
    }

    fn corrected_covered_boundaries_from_envelope(
        &self,
        base: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<u32, f64>,
            BTreeMap<u32, f64>,
        ),
        DirectV11RealConsumerError,
    > {
        let shortwave = envelope
            .covered_snow_shortwave_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered optical shortwave set"))?;
        let longwave = envelope
            .covered_snow_longwave_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered reciprocal longwave set"))?;
        if shortwave.keys().collect::<BTreeSet<_>>() != base.keys().collect::<BTreeSet<_>>()
            || longwave.keys().collect::<BTreeSet<_>>() != base.keys().collect::<BTreeSet<_>>()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered corrected boundary destination set",
            ));
        }
        let mut corrected = base.clone();
        let mut shortwave_by_lane = BTreeMap::<u32, (f64, f64)>::new();
        let mut longwave_by_lane = BTreeMap::<u32, (f64, f64)>::new();
        for (destination, value) in shortwave {
            let boundary =
                corrected
                    .get_mut(&destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered corrected shortwave destination",
                    ))?;
            boundary.shortwave_absorbed_w_m2 = value;
            let lane_id = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered corrected shortwave OFE binding",
                ))?
                .production_lane_id;
            let fraction = self.covered_destination_fraction(&destination.0, &destination.1)?;
            let entry = shortwave_by_lane.entry(lane_id).or_default();
            entry.0 += fraction * value;
            entry.1 += fraction;
        }
        for (destination, value) in longwave {
            let boundary =
                corrected
                    .get_mut(&destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered corrected longwave destination",
                    ))?;
            boundary.net_longwave_w_m2 = value;
            let lane_id = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered corrected longwave OFE binding",
                ))?
                .production_lane_id;
            let fraction = self.covered_destination_fraction(&destination.0, &destination.1)?;
            let entry = longwave_by_lane.entry(lane_id).or_default();
            entry.0 += fraction * value;
            entry.1 += fraction;
        }
        Ok((
            corrected,
            shortwave_by_lane
                .into_iter()
                .map(|(lane, (value, weight))| (lane, value / weight))
                .collect(),
            longwave_by_lane
                .into_iter()
                .map(|(lane, (value, weight))| (lane, value / weight))
                .collect(),
        ))
    }

    fn apply_lse_iteration_exchange(
        &self,
        boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        states: &BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
    ) -> Result<BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>, DirectV11RealConsumerError>
    {
        if boundaries.keys().collect::<BTreeSet<_>>() != states.keys().collect::<BTreeSet<_>>() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered LSE iteration exchange destination set",
            ));
        }
        let mut next = boundaries.clone();
        for (destination, state) in states {
            let boundary =
                next.get_mut(destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered LSE iteration exchange destination",
                    ))?;
            boundary.sensible_to_canopy_air_w_m2 = state.snow_sensible_w_m2;
            boundary.vapor_to_canopy_air_kg_m2_s = state.snow_vapor_kg_m2_s;
        }
        Ok(next)
    }

    fn seal_final_covered_boundaries(
        &self,
        input: &V11ImportedV10SegmentInput,
        boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        destination_receipts: &BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>,
        envelope: &UncommittedCoveredV8OwnerEnvelope,
        ending_v8_physical_candidate_sha256: Digest32,
        ending_stage3_state_sha256: Digest32,
    ) -> Result<
        (
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
            BTreeMap<(OfeId, TileId), FinalStage3CanopyBoundaryReceiptV1>,
        ),
        DirectV11RealConsumerError,
    > {
        let optical = envelope
            .covered_snow_optical_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered final optical receipts"))?;
        let longwave = envelope
            .covered_snow_longwave_by_destination()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered final longwave receipts"))?;
        let beginning_v11_state_sha256 = digest32_from_lower_hex(&input.beginning.0.state_sha256)?;
        let mut final_boundaries = boundaries.clone();
        let mut final_receipts = BTreeMap::new();
        for (destination, boundary) in boundaries {
            let carrier = destination_receipts.get(destination).ok_or(
                DirectV11RealConsumerError::Identity("covered final carrier receipt join"),
            )?;
            let optical = optical
                .get(destination)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered final optical receipt join",
                ))?;
            let final_longwave =
                longwave
                    .get(destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered final longwave receipt join",
                    ))?;
            if optical.absorbed_w_m2_tile.total().to_bits()
                != boundary.shortwave_absorbed_w_m2.to_bits()
                || final_longwave.to_bits() != boundary.net_longwave_w_m2.to_bits()
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered final boundary self-reconstruction",
                ));
            }
            let lane_id = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered final boundary lane join",
                ))?
                .production_lane_id;
            let beginning_stage3 = self.stage3_beginning_by_lane.get(&lane_id).ok_or(
                DirectV11RealConsumerError::Identity("covered final beginning Stage-3 state"),
            )?;
            let beginning_stage3_state_sha256 =
                digest_bytes(&canonical_stage3_snow_owner_bytes_v11(&BTreeMap::from([
                    (lane_id, beginning_stage3.clone()),
                ]))?);
            let optical_receipt_sha256 = digest32_from_lower_hex(optical.receipt_sha256.as_str())?;
            let reciprocal_longwave_receipt_sha256 =
                reciprocal_longwave_receipt_digest(destination, input.support, *final_longwave);
            let final_receipt = FinalStage3CanopyBoundaryReceiptV1::try_new(
                FinalStage3CanopyBoundaryReceiptInputs {
                    support: input.support,
                    destination: destination.clone(),
                    beginning_v11_state_sha256,
                    beginning_stage3_state_sha256,
                    ending_v8_physical_candidate_sha256,
                    ending_stage3_state_sha256,
                    provisional_carrier_receipt_sha256: carrier.diagnostic_sha256,
                    optical_receipt_sha256,
                    reciprocal_longwave_receipt_sha256,
                    sensible_to_canopy_air_w_m2: boundary.sensible_to_canopy_air_w_m2,
                    vapor_to_canopy_air_kg_m2_s: boundary.vapor_to_canopy_air_kg_m2_s,
                    latent_energy_to_canopy_air_j_m2: boundary.vapor_to_canopy_air_kg_m2_s
                        * boundary.latent_heat_j_kg
                        * f64::from_bits(input.support.duration_s_bits()),
                    snow_temperature_k: boundary.snow_temperature_k,
                    latent_heat_j_kg: boundary.latent_heat_j_kg,
                    snow_absorbed_shortwave_w_m2: optical.absorbed_w_m2_tile.total(),
                    snow_net_longwave_w_m2: *final_longwave,
                },
            )?;
            let final_boundary = final_boundaries.get_mut(destination).ok_or(
                DirectV11RealConsumerError::Identity("covered final boundary storage"),
            )?;
            final_boundary.optical_receipt_sha256 = Some(
                Sha256Digest::try_new(digest32_hex(optical_receipt_sha256)).map_err(|_| {
                    DirectV11RealConsumerError::Identity("covered optical receipt digest")
                })?,
            );
            final_boundary.reciprocal_longwave_receipt_sha256 = Some(
                Sha256Digest::try_new(digest32_hex(reciprocal_longwave_receipt_sha256)).map_err(
                    |_| {
                        DirectV11RealConsumerError::Identity(
                            "covered reciprocal longwave receipt digest",
                        )
                    },
                )?,
            );
            final_boundary.final_canopy_boundary_receipt_sha256 = Some(
                Sha256Digest::try_new(digest32_hex(final_receipt.receipt_sha256)).map_err(
                    |_| {
                        DirectV11RealConsumerError::Identity(
                            "covered final boundary receipt digest",
                        )
                    },
                )?,
            );
            final_receipts.insert(destination.clone(), final_receipt);
        }
        Ok((final_boundaries, final_receipts))
    }

    fn covered_lane_to_ofe(
        &self,
        stage3_beginning_by_lane: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<BTreeMap<u32, OfeId>, DirectV11RealConsumerError> {
        let mut lane_to_ofe = BTreeMap::new();
        for binding in &self.beginning.inner.surface_configuration.ofe_bindings {
            if lane_to_ofe
                .insert(binding.production_lane_id, binding.ofe_id.clone())
                .is_some()
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "duplicate covered lane/OFE binding",
                ));
            }
        }
        if stage3_beginning_by_lane
            .keys()
            .copied()
            .collect::<BTreeSet<_>>()
            != lane_to_ofe.keys().copied().collect::<BTreeSet<_>>()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier lane/OFE set",
            ));
        }
        Ok(lane_to_ofe)
    }

    fn covered_expected_destinations(&self) -> BTreeSet<(OfeId, TileId)> {
        let covered_tile_ids = self
            .beginning
            .vegetation_configuration
            .strata
            .iter()
            .flat_map(|stratum| stratum.tile_ids.iter().cloned())
            .collect::<BTreeSet<_>>();
        self.beginning
            .inner
            .lse_configuration
            .ofes
            .iter()
            .flat_map(|ofe| {
                ofe.tiles
                    .iter()
                    .filter(|tile| covered_tile_ids.contains(&tile.vegetation_tile_id))
                    .map(|tile| (ofe.ofe_id.clone(), tile.tile_id.clone()))
            })
            .collect()
    }

    fn carrier_for_destination(
        &self,
        interval_s: f64,
        lane_id: u32,
        ofe_id: &OfeId,
        tile_id: &TileId,
        vegetation_state: &V8CoupledOwnedState,
        stage3_state_by_lane: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        stage3_forcing_by_lane: &BTreeMap<u32, DirectSnowStage3SupportInput>,
    ) -> Result<CoveredCarrierInitialGuessV1, DirectV11RealConsumerError> {
        let stage3_state =
            stage3_state_by_lane
                .get(&lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered destination Stage-3 state",
                ))?;
        let stage3_forcing = stage3_forcing_by_lane.get(&lane_id).copied().ok_or(
            DirectV11RealConsumerError::Identity("covered destination Stage-3 forcing"),
        )?;
        let sealed = self.carrier_forcing_by_lane.get(&lane_id).ok_or(
            DirectV11RealConsumerError::Identity("covered destination carrier forcing"),
        )?;
        let vegetation_tile_id = self
            .beginning
            .inner
            .lse_configuration
            .ofes
            .iter()
            .find(|ofe| ofe.ofe_id == *ofe_id)
            .and_then(|ofe| ofe.tiles.iter().find(|tile| tile.tile_id == *tile_id))
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered destination vegetation tile",
            ))?
            .vegetation_tile_id
            .clone();
        let mut guess = self.derive_live_carrier_input(
            lane_id,
            stage3_state,
            vegetation_state,
            stage3_forcing,
            sealed,
            Some(&vegetation_tile_id),
            interval_s,
        )?;
        let stage3_beginning_sha256 =
            digest_bytes(&canonical_stage3_snow_owner_bytes_v11(&BTreeMap::from([
                (lane_id, stage3_state.clone()),
            ]))?);
        let forcing_sha256 = stage3_support_forcing_digest(stage3_forcing)?;
        let duration_bits = interval_s.to_bits().to_be_bytes();
        guess.diagnostic_sha256 = openwepp_coupled_time::framed_sha256(
            "covered-carrier-initial-guess-diagnostic-v1",
            &[
                openwepp_coupled_time::FramedField {
                    tag: "ofe_id",
                    value: ofe_id.as_str().as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "tile_id",
                    value: tile_id.as_str().as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "duration_bits",
                    value: &duration_bits,
                },
                openwepp_coupled_time::FramedField {
                    tag: "exposure_receipt",
                    value: sealed.exposure.receipt_id.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "forcing_receipt",
                    value: forcing_sha256.as_str().as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "beginning_v11_state",
                    value: self.beginning.vegetation_state.0.state_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "beginning_stage3_state",
                    value: stage3_beginning_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "guess_values",
                    value: guess.diagnostic_sha256.as_bytes(),
                },
            ],
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("covered initial guess diagnostic"))?;
        Ok(guess)
    }

    fn stage3_lower_boundaries_by_destination(
        &self,
        receipts: &BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>,
        stage3_inputs_by_lane: &BTreeMap<u32, DirectActiveSnowPartitionInputs>,
        stage3_forcing_by_lane: &BTreeMap<u32, DirectSnowStage3SupportInput>,
    ) -> Result<BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>, DirectV11RealConsumerError>
    {
        let expected_destinations = self.covered_expected_destinations();
        if receipts.keys().cloned().collect::<BTreeSet<_>>() != expected_destinations {
            return Err(DirectV11RealConsumerError::Identity(
                "covered destination carrier receipt set",
            ));
        }
        let mut boundaries = BTreeMap::new();
        for (destination, receipt) in receipts {
            let carrier_receipt_id = Sha256Digest::try_new(digest32_hex(receipt.diagnostic_sha256))
                .map_err(|_| DirectV11RealConsumerError::Identity("covered carrier receipt ID"))?;
            let binding = self
                .beginning
                .inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == destination.0)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered boundary OFE binding",
                ))?;
            let stage3_input = stage3_inputs_by_lane
                .get(&binding.production_lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered boundary Stage-3 inputs",
                ))?;
            let stage3_forcing = stage3_forcing_by_lane
                .get(&binding.production_lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered boundary Stage-3 forcing",
                ))?;
            let snow_albedo = stage3_input
                .snow_albedo_state
                .map_or(STAGE3_DEFAULT_SNOW_ALBEDO, |state| state.albedo);
            let boundary = Stage3SnowCoveredLowerBoundary {
                snow_temperature_k: receipt.snow_temperature_k,
                latent_heat_j_kg:
                    openwepp_meteorology::surface_energy::latent_heat_for_surface_temperature(
                        TemperatureCelsius::try_new(receipt.snow_temperature_k - 273.15).map_err(
                            |_| DirectV11RealConsumerError::Identity("covered temperature"),
                        )?,
                    )
                    .map_err(|_| DirectV11RealConsumerError::Identity("covered latent heat"))?
                    .as_joules_per_kilogram(),
                sensible_to_canopy_air_w_m2: -receipt.snow_sensible_into_surface_w_m2,
                vapor_to_canopy_air_kg_m2_s: -receipt.snow_vapor_into_surface_kg_m2_s,
                net_longwave_w_m2: receipt.snow_longwave_net_w_m2,
                // The current released carrier receipt does not yet expose a
                // canonical shortwave or precipitation-advection term. Keep
                // those owners explicit and zero only at this default-off
                // seam; the physical covered cutover remains blocked on their
                // Stage-3 projections and ledger reconstruction.
                shortwave_absorbed_w_m2: 0.0,
                precipitation_advection_w_m2: 0.0,
                carrier_receipt_id,
                snow_vis_albedo: snow_albedo,
                snow_nir_albedo: snow_albedo,
                stage3_albedo_state_sha256: stage3_albedo_state_digest(stage3_input)?,
                forcing_receipt_sha256: stage3_support_forcing_digest(*stage3_forcing)?,
                optical_receipt_sha256: None,
                reciprocal_longwave_receipt_sha256: None,
                final_canopy_boundary_receipt_sha256: None,
            };
            boundary
                .validate()
                .map_err(|_| DirectV11RealConsumerError::Identity("covered boundary operands"))?;
            if boundaries.insert(destination.clone(), boundary).is_some() {
                return Err(DirectV11RealConsumerError::Identity(
                    "duplicate covered destination lower boundary",
                ));
            }
        }
        Ok(boundaries)
    }

    /// Merge the latest persistent Stage-3 state operands into the boundary
    /// whose radiative and turbulent terms came from the preceding LSE solve.
    /// Stage-3 owns snow temperature (and therefore latent heat); LSE owns the
    /// exchanged fluxes.  Neither side may replace the other's operands.
    fn merge_latest_stage3_state_operands(
        &self,
        flux_boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        stage3_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>, DirectV11RealConsumerError>
    {
        flux_boundaries
            .iter()
            .map(|(destination, boundary)| {
                let lane_id = self
                    .beginning
                    .inner
                    .surface_configuration
                    .ofe_bindings
                    .iter()
                    .find(|binding| binding.ofe_id == destination.0)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered Stage-3 state boundary OFE",
                    ))?
                    .production_lane_id;
                let state =
                    stage3_states
                        .get(&lane_id)
                        .ok_or(DirectV11RealConsumerError::Identity(
                            "covered Stage-3 state boundary lane",
                        ))?;
                let snow_temperature_k = state
                    .layers
                    .first()
                    .map_or(273.15, |layer| layer.temperature_c + 273.15);
                let latent_heat_j_kg =
                    openwepp_meteorology::surface_energy::latent_heat_for_surface_temperature(
                        TemperatureCelsius::try_new(snow_temperature_k - 273.15).map_err(|_| {
                            DirectV11RealConsumerError::Identity(
                                "covered Stage-3 state boundary temperature",
                            )
                        })?,
                    )
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "covered Stage-3 state boundary latent heat",
                        )
                    })?
                    .as_joules_per_kilogram();
                let mut merged = boundary.clone();
                merged.snow_temperature_k = snow_temperature_k;
                merged.latent_heat_j_kg = latent_heat_j_kg;
                merged.validate().map_err(|_| {
                    DirectV11RealConsumerError::Identity("covered merged Stage-3/LSE boundary")
                })?;
                Ok((destination.clone(), merged))
            })
            .collect()
    }

    pub fn take_staged_ending(&mut self) -> Option<DirectV10RealConsumerShadow> {
        self.ending.take()
    }

    pub fn take_staged_stage3(&mut self) -> Option<BTreeMap<u32, DirectSnowStage3PersistentState>> {
        self.ending_stage3_by_lane.take()
    }

    #[must_use]
    pub fn last_final_boundary_receipts(
        &self,
    ) -> Option<&BTreeMap<(OfeId, TileId), FinalStage3CanopyBoundaryReceiptV1>> {
        self.last_final_boundary_receipts.as_ref()
    }

    #[must_use]
    pub(crate) fn last_lane_boundary_receipts(
        &self,
    ) -> Option<&BTreeMap<u32, LaneStage3BoundaryReceiptV1>> {
        self.last_lane_boundary_receipts.as_ref()
    }

    #[must_use]
    pub(crate) fn last_component_carrier_receipts(
        &self,
    ) -> Option<&BTreeMap<(OfeId, TileId), ComponentResolvedCarrierReceiptV1>> {
        self.last_component_carrier_receipts.as_ref()
    }
}

fn reciprocal_longwave_receipt_digest(
    destination: &(OfeId, TileId),
    support: openwepp_coupled_time::TimeSupport,
    net_longwave_w_m2: f64,
) -> Digest32 {
    let mut bytes = Vec::with_capacity(192);
    bytes.extend_from_slice(b"OPENWEPP_RECIPROCAL_LONGWAVE_RECEIPT_V1\0");
    bytes.extend_from_slice(&support.start_ns().get().to_le_bytes());
    bytes.extend_from_slice(&support.end_ns().get().to_le_bytes());
    bytes.extend_from_slice(destination.0.as_str().as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(destination.1.as_str().as_bytes());
    bytes.extend_from_slice(&net_longwave_w_m2.to_bits().to_le_bytes());
    digest_bytes(&bytes)
}

fn covered_fixed_point_boundaries_equal(
    left: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    right: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
) -> bool {
    if left.keys().collect::<BTreeSet<_>>() != right.keys().collect::<BTreeSet<_>>() {
        return false;
    }
    left.iter().all(|(destination, lhs)| {
        let Some(rhs) = right.get(destination) else {
            return false;
        };
        close_with_policy(
            lhs.snow_temperature_k,
            rhs.snow_temperature_k,
            COVERED_FIXED_POINT_POLICY.temperature_abs_k,
            COVERED_FIXED_POINT_POLICY.temperature_rel,
        ) && close_with_policy(
            lhs.sensible_to_canopy_air_w_m2,
            rhs.sensible_to_canopy_air_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_rel,
        ) && close_with_policy(
            lhs.vapor_to_canopy_air_kg_m2_s,
            rhs.vapor_to_canopy_air_kg_m2_s,
            COVERED_FIXED_POINT_POLICY.vapor_abs_kg_m2_s,
            COVERED_FIXED_POINT_POLICY.vapor_rel,
        ) && close_with_policy(
            lhs.net_longwave_w_m2,
            rhs.net_longwave_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_rel,
        ) && close_with_policy(
            lhs.shortwave_absorbed_w_m2,
            rhs.shortwave_absorbed_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_rel,
        ) && close_with_policy(
            lhs.precipitation_advection_w_m2,
            rhs.precipitation_advection_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_rel,
        )
    })
}

fn close_with_policy(left: f64, right: f64, absolute: f64, relative: f64) -> bool {
    left.is_finite()
        && right.is_finite()
        && (left - right).abs() <= absolute + relative * left.abs().max(right.abs())
}

fn covered_fixed_point_lse_states_equal(
    left: &BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
    right: &BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
) -> bool {
    if left.keys().collect::<BTreeSet<_>>() != right.keys().collect::<BTreeSet<_>>() {
        return false;
    }
    let close_temperature = |a: f64, b: f64| {
        close_with_policy(
            a,
            b,
            COVERED_FIXED_POINT_POLICY.temperature_abs_k,
            COVERED_FIXED_POINT_POLICY.temperature_rel,
        )
    };
    let close_humidity = |a: f64, b: f64| {
        close_with_policy(
            a,
            b,
            COVERED_FIXED_POINT_POLICY.humidity_abs_kg_kg,
            COVERED_FIXED_POINT_POLICY.humidity_rel,
        )
    };
    let close_flux = |a: f64, b: f64| {
        close_with_policy(
            a,
            b,
            COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_rel,
        )
    };
    let close_vapor = |a: f64, b: f64| {
        close_with_policy(
            a,
            b,
            COVERED_FIXED_POINT_POLICY.vapor_abs_kg_m2_s,
            COVERED_FIXED_POINT_POLICY.vapor_rel,
        )
    };
    left.iter().all(|(destination, lhs)| {
        let Some(rhs) = right.get(destination) else {
            return false;
        };
        close_temperature(lhs.canopy_air_temperature_k, rhs.canopy_air_temperature_k)
            && close_humidity(
                lhs.canopy_air_specific_humidity_kg_kg,
                rhs.canopy_air_specific_humidity_kg_kg,
            )
            && close_temperature(lhs.snow_temperature_k, rhs.snow_temperature_k)
            && close_flux(lhs.snow_sensible_w_m2, rhs.snow_sensible_w_m2)
            && close_vapor(lhs.snow_vapor_kg_m2_s, rhs.snow_vapor_kg_m2_s)
            && close_flux(lhs.snow_latent_w_m2, rhs.snow_latent_w_m2)
            && close_flux(lhs.snow_net_longwave_w_m2, rhs.snow_net_longwave_w_m2)
            && close_flux(lhs.canopy_sensible_w_m2, rhs.canopy_sensible_w_m2)
            && close_vapor(lhs.canopy_vapor_kg_m2_s, rhs.canopy_vapor_kg_m2_s)
            && close_flux(
                lhs.sensible_to_reference_air_w_m2,
                rhs.sensible_to_reference_air_w_m2,
            )
            && close_vapor(
                lhs.vapor_to_reference_air_kg_m2_s,
                rhs.vapor_to_reference_air_kg_m2_s,
            )
            && lhs.component_temperatures_k.len() == rhs.component_temperatures_k.len()
            && lhs
                .component_temperatures_k
                .iter()
                .zip(&rhs.component_temperatures_k)
                .all(|((left_id, left_values), (right_id, right_values))| {
                    left_id == right_id
                        && left_values
                            .iter()
                            .zip(right_values)
                            .all(|(left, right)| close_temperature(*left, *right))
                })
            && lhs.component_carrier_surfaces.len() == rhs.component_carrier_surfaces.len()
            && lhs
                .component_carrier_surfaces
                .iter()
                .zip(&rhs.component_carrier_surfaces)
                .all(|(left, right)| {
                    left.occupancy_id == right.occupancy_id
                        && left.component_ordinal == right.component_ordinal
                        && left.surface_area_m2_m2_tile.to_bits()
                            == right.surface_area_m2_m2_tile.to_bits()
                        && left.emissive_area_m2_m2_tile.to_bits()
                            == right.emissive_area_m2_m2_tile.to_bits()
                        && close_with_policy(
                            left.heat_conductance_m_s_tile,
                            right.heat_conductance_m_s_tile,
                            0.0,
                            COVERED_FIXED_POINT_POLICY.flux_rel,
                        )
                        && close_with_policy(
                            left.vapor_conductance_m_s_tile,
                            right.vapor_conductance_m_s_tile,
                            0.0,
                            COVERED_FIXED_POINT_POLICY.vapor_rel,
                        )
                        && match (
                            left.vapor_authorization_kg_m2_tile_s,
                            right.vapor_authorization_kg_m2_tile_s,
                        ) {
                            (Some(left), Some(right)) => close_vapor(left, right),
                            (None, None) => true,
                            _ => false,
                        }
                        && close_temperature(left.temperature_k, right.temperature_k)
                        && close_humidity(
                            left.specific_humidity_kg_kg,
                            right.specific_humidity_kg_kg,
                        )
                        && close_flux(
                            left.sensible_to_canopy_air_w_m2,
                            right.sensible_to_canopy_air_w_m2,
                        )
                        && close_vapor(
                            left.vapor_to_canopy_air_kg_m2_s,
                            right.vapor_to_canopy_air_kg_m2_s,
                        )
                })
    })
}

fn covered_fixed_point_stage3_states_equal(
    left: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    right: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> bool {
    let close_depth =
        |left, right| close_with_policy(left, right, COVERED_FIXED_POINT_POLICY.depth_abs_m, 0.0);
    let close_temperature = |left, right| {
        close_with_policy(
            left,
            right,
            COVERED_FIXED_POINT_POLICY.state_temperature_abs_k,
            0.0,
        )
    };
    let close_mass = |left, right| {
        close_with_policy(left, right, COVERED_FIXED_POINT_POLICY.mass_abs_kg_m2, 0.0)
    };
    let close_energy = |left, right| {
        close_with_policy(left, right, COVERED_FIXED_POINT_POLICY.energy_abs_j_m2, 0.0)
    };
    left.keys().collect::<BTreeSet<_>>() == right.keys().collect::<BTreeSet<_>>()
        && left.iter().all(|(lane_id, lhs)| {
            let Some(rhs) = right.get(lane_id) else {
                return false;
            };
            if lhs.fingerprint != Wb11HydrologyKernel::stage3_persistent_state_fingerprint(lhs)
                || rhs.fingerprint != Wb11HydrologyKernel::stage3_persistent_state_fingerprint(rhs)
            {
                return false;
            }
            lhs.schema_version == rhs.schema_version
                && lhs.terminal_event_model == rhs.terminal_event_model
                && lhs.lane_id == rhs.lane_id
                && lhs.next_interval_index == rhs.next_interval_index
                && lhs.layers.len() == rhs.layers.len()
                && lhs.layers.iter().zip(&rhs.layers).all(|(left, right)| {
                    close_depth(left.mass_swe_m, right.mass_swe_m)
                        && close_depth(left.thickness_m, right.thickness_m)
                        && left.density_kg_m3.to_bits() == right.density_kg_m3.to_bits()
                        && left.settle_day_count.to_bits() == right.settle_day_count.to_bits()
                        && close_temperature(left.temperature_c, right.temperature_c)
                        && close_depth(left.liquid_water_m, right.liquid_water_m)
                        && close_energy(left.cold_content_j_m2, right.cold_content_j_m2)
                        && close_depth(left.refrozen_liquid_m, right.refrozen_liquid_m)
                })
                && [
                    (
                        lhs.detached_retained_liquid_kg_m2,
                        rhs.detached_retained_liquid_kg_m2,
                    ),
                    (lhs.initial_ice_kg_m2, rhs.initial_ice_kg_m2),
                    (
                        lhs.initial_retained_liquid_kg_m2,
                        rhs.initial_retained_liquid_kg_m2,
                    ),
                    (lhs.cumulative_snowfall_kg_m2, rhs.cumulative_snowfall_kg_m2),
                    (
                        lhs.cumulative_external_liquid_kg_m2,
                        rhs.cumulative_external_liquid_kg_m2,
                    ),
                    (
                        lhs.cumulative_deposition_kg_m2,
                        rhs.cumulative_deposition_kg_m2,
                    ),
                    (
                        lhs.cumulative_sublimation_kg_m2,
                        rhs.cumulative_sublimation_kg_m2,
                    ),
                    (lhs.cumulative_melt_kg_m2, rhs.cumulative_melt_kg_m2),
                    (
                        lhs.cumulative_unresolved_liquid_kg_m2,
                        rhs.cumulative_unresolved_liquid_kg_m2,
                    ),
                ]
                .into_iter()
                .all(|(left, right)| close_mass(left, right))
                && [
                    (
                        lhs.cumulative_complete_energy_j_m2,
                        rhs.cumulative_complete_energy_j_m2,
                    ),
                    (
                        lhs.cumulative_cold_energy_change_j_m2,
                        rhs.cumulative_cold_energy_change_j_m2,
                    ),
                    (
                        lhs.cumulative_terminal_unallocated_energy_j_m2,
                        rhs.cumulative_terminal_unallocated_energy_j_m2,
                    ),
                ]
                .into_iter()
                .all(|(left, right)| close_energy(left, right))
        })
}

impl<'a> DirectV11RealConsumerStack<'a> {
    #[must_use]
    pub fn new(
        beginning: &DirectV10RealConsumerShadow,
        interval: &'a DirectV9ShadowIntervalInput,
        day_index: usize,
        interval_index: usize,
    ) -> Self {
        Self {
            beginning: beginning.clone(),
            interval,
            day_index,
            interval_index,
            ending: None,
            last_support_receipt: None,
            ending_snow_owner_bytes: None,
        }
    }

    /// Bind the Stage-3 state that the shared parent transaction has already
    /// staged as the sole ending snow owner. This constructor remains the
    /// snow-free lower-boundary executor; it does not admit snow forcing.
    #[must_use]
    pub fn new_with_ending_snow_owner(
        beginning: &DirectV10RealConsumerShadow,
        interval: &'a DirectV9ShadowIntervalInput,
        day_index: usize,
        interval_index: usize,
        ending_snow_owner_bytes: Vec<u8>,
    ) -> Self {
        let mut value = Self::new(beginning, interval, day_index, interval_index);
        value.ending_snow_owner_bytes = Some(ending_snow_owner_bytes);
        value
    }

    /// Consume the isolated staged ending only after the V11 parent accepts
    /// the corresponding segment candidate.
    pub fn take_staged_ending(&mut self) -> Option<DirectV10RealConsumerShadow> {
        self.ending.take()
    }

    #[must_use]
    pub fn last_support_receipt(&self) -> Option<&LseSupportAdmissibilityReceiptV1> {
        self.last_support_receipt.as_ref()
    }
}

impl crate::v11_vegetation_consumer::DirectV11ImportedStack
    for DirectV11SnowCoveredRealConsumerStack<'_>
{
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
            || self.carrier_forcing_by_lane.is_empty()
            || self.carrier_forcing_by_lane.keys().collect::<Vec<_>>()
                != self.stage3_beginning_by_lane.keys().collect::<Vec<_>>()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered support / DirectV10 beginning join",
            ));
        }
        let interval_s = f64::from_bits(input.duration_s_bits);
        for stage3_forcing in self.stage3_forcing_by_lane.values() {
            if stage3_forcing.duration_seconds.to_bits() != interval_s.to_bits() {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered Stage-3/V11 support duration",
                ));
            }
        }
        let (_, initial_vegetation_state) = project_v9_runtime_to_v8(
            &self.beginning.inner.vegetation_configuration,
            &self.beginning.inner.vegetation_state,
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::V9(error),
            ))
        })?;
        let evaluate_stage3 =
            |destination_receipts: &BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>,
             boundaries: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
             final_lane_receipts: Option<&BTreeMap<u32, LaneStage3BoundaryReceiptV1>>| {
                let terms = self.lane_stage3_terms_from_boundaries(
                    destination_receipts,
                    boundaries,
                    interval_s,
                )?;
                let mut ending_stage3 = BTreeMap::new();
                for (lane_id, beginning) in &self.stage3_beginning_by_lane {
                    let stage3_inputs = self.stage3_inputs_by_lane.get(lane_id).ok_or(
                        DirectV11RealConsumerError::Identity("covered Stage-3 input lane"),
                    )?;
                    let stage3_forcing = self.stage3_forcing_by_lane.get(lane_id).copied().ok_or(
                        DirectV11RealConsumerError::Identity("covered Stage-3 forcing lane"),
                    )?;
                    let lane_terms =
                        terms
                            .get(lane_id)
                            .ok_or(DirectV11RealConsumerError::Identity(
                                "covered Stage-3 lane terms",
                            ))?;
                    let beginning_stage3_digest = canonical_stage3_snow_owner_bytes_v11(
                        &BTreeMap::from([(*lane_id, beginning.clone())]),
                    )?;
                    let (
                        sensible_to_canopy_air_w_m2,
                        vapor_to_canopy_air_kg_m2_s,
                        latent_energy_to_canopy_air_j_m2,
                        snow_absorbed_shortwave_w_m2,
                        snow_net_longwave_w_m2,
                        latent_heat_j_kg,
                        identity,
                    ) = if let Some(receipts) = final_lane_receipts {
                        let receipt =
                            receipts
                                .get(lane_id)
                                .ok_or(DirectV11RealConsumerError::Identity(
                                    "covered final lane boundary receipt",
                                ))?;
                        if receipt.aggregate_sensible_to_canopy_air_w_m2.to_bits()
                            != lane_terms.sensible_to_canopy_air_w_m2.to_bits()
                            || receipt.aggregate_vapor_to_canopy_air_kg_m2_s.to_bits()
                                != lane_terms.vapor_to_canopy_air_kg_m2_s.to_bits()
                            || receipt.aggregate_latent_energy_to_canopy_air_j_m2.to_bits()
                                != lane_terms.latent_energy_to_canopy_air_j_m2.to_bits()
                            || receipt.aggregate_snow_absorbed_shortwave_w_m2.to_bits()
                                != lane_terms.snow_absorbed_shortwave_w_m2.to_bits()
                            || receipt.aggregate_snow_net_longwave_w_m2.to_bits()
                                != lane_terms.snow_net_longwave_w_m2.to_bits()
                            || receipt.aggregate_snow_temperature_k.to_bits()
                                != lane_terms.snow_temperature_k.to_bits()
                            || receipt.aggregate_latent_heat_j_kg.to_bits()
                                != lane_terms.latent_heat_j_kg.to_bits()
                        {
                            return Err(DirectV11RealConsumerError::Identity(
                                "covered lane receipt boundary reconstruction",
                            ));
                        }
                        (
                            receipt.aggregate_sensible_to_canopy_air_w_m2,
                            receipt.aggregate_vapor_to_canopy_air_kg_m2_s,
                            receipt.aggregate_latent_energy_to_canopy_air_j_m2,
                            receipt.aggregate_snow_absorbed_shortwave_w_m2,
                            receipt.aggregate_snow_net_longwave_w_m2,
                            receipt.aggregate_latent_heat_j_kg,
                            Stage3BoundaryIdentity::Final {
                                provisional_carrier_receipt_sha256: receipt
                                    .provisional_carrier_receipt_sha256,
                                optical_receipt_sha256: receipt.optical_receipt_sha256,
                                reciprocal_longwave_receipt_sha256: receipt
                                    .reciprocal_longwave_receipt_sha256,
                                final_destination_receipt_sha256: receipt
                                    .final_destination_receipt_sha256,
                                final_lane_receipt_sha256: receipt.receipt_sha256,
                            },
                        )
                    } else {
                        (
                            lane_terms.sensible_to_canopy_air_w_m2,
                            lane_terms.vapor_to_canopy_air_kg_m2_s,
                            lane_terms.latent_energy_to_canopy_air_j_m2,
                            lane_terms.snow_absorbed_shortwave_w_m2,
                            lane_terms.snow_net_longwave_w_m2,
                            lane_terms.latent_heat_j_kg,
                            Stage3BoundaryIdentity::Provisional {
                                carrier_receipt_sha256: lane_terms
                                    .provisional_carrier_receipt_sha256,
                            },
                        )
                    };
                    let boundary = Stage3SnowSurfaceBoundaryReceiptV1::try_new(
                        Stage3SnowSurfaceBoundaryReceiptInputs {
                            support: input.support,
                            sensible_energy_j_m2: sensible_to_canopy_air_w_m2 * interval_s,
                            vapor_mass_kg_m2: vapor_to_canopy_air_kg_m2_s * interval_s,
                            latent_energy_j_m2: latent_energy_to_canopy_air_j_m2,
                            shortwave_energy_j_m2: snow_absorbed_shortwave_w_m2 * interval_s,
                            net_longwave_energy_j_m2: snow_net_longwave_w_m2 * interval_s,
                            precipitation_advection_j_m2: 0.0,
                            latent_heat_j_kg,
                            beginning_stage3_state_sha256: digest_bytes(&beginning_stage3_digest),
                            identity,
                        },
                    )?;
                    let result =
                        Wb11HydrologyKernel::evaluate_stage3_persistent_support_with_boundary(
                            stage3_inputs,
                            beginning,
                            *lane_id,
                            beginning.next_interval_index,
                            stage3_forcing,
                            boundary,
                        )?;
                    let flux_tolerance = 1.0e-6_f64;
                    let evaluation = &result.evaluation;
                    if (evaluation.complete_arm_sensible_j_m2 - boundary.sensible_energy_j_m2).abs()
                        > flux_tolerance
                        || (evaluation.complete_arm_shortwave_j_m2 - boundary.shortwave_energy_j_m2)
                            .abs()
                            > flux_tolerance
                        || (evaluation.complete_arm_latent_j_m2 - boundary.latent_energy_j_m2).abs()
                            > flux_tolerance
                        || (evaluation.complete_arm_longwave_j_m2
                            - boundary.net_longwave_energy_j_m2)
                            .abs()
                            > flux_tolerance
                        || (evaluation.complete_arm_advected_j_m2
                            - boundary.precipitation_advection_j_m2)
                            .abs()
                            > flux_tolerance
                        || (evaluation.complete_arm_vapor_mass_exchange_kg_m2
                            - boundary.vapor_mass_kg_m2)
                            .abs()
                            > 1.0e-9
                        || result.evaluation.evaluated_seconds.to_bits() != interval_s.to_bits()
                        || result.lifecycle != "active"
                    {
                        return Err(DirectV11RealConsumerError::Identity(
                            "Stage-3 covered boundary/result ledger join",
                        ));
                    }
                    if result.terminal_event.is_some() {
                        return Err(DirectV11RealConsumerError::Identity(
                            "covered adopter received terminal event before terminal chronology",
                        ));
                    }
                    ending_stage3.insert(*lane_id, result.state);
                }
                Ok::<_, DirectV11RealConsumerError>(ending_stage3)
            };
        let _interval_index = u8::try_from(self.interval_index)
            .map_err(|_| DirectV11RealConsumerError::Identity("V11 interval index overflow"))?;
        let iteration_vegetation_state = initial_vegetation_state;
        let mut iteration_stage3_states = self.stage3_beginning_by_lane.clone();
        let mut iteration_boundaries: Option<
            BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        > = None;
        let mut previous_lse_states: Option<BTreeMap<(OfeId, TileId), CoveredLseIterationState>> =
            None;
        let mut previous_stage3_states: Option<BTreeMap<u32, DirectSnowStage3PersistentState>> =
            None;
        // The legacy reduced carrier is evaluated once, solely to seed the
        // nonlinear iteration.  Its receipt never changes with candidate
        // state and is not the accepted component-carrier authority.
        let initial_guess_receipts = self.carrier_receipts_by_destination(
            interval_s,
            &iteration_vegetation_state,
            &iteration_stage3_states,
            self.stage3_forcing_by_lane,
        )?;
        let initial_guess_boundaries = self.stage3_lower_boundaries_by_destination(
            &initial_guess_receipts,
            self.stage3_inputs_by_lane,
            self.stage3_forcing_by_lane,
        )?;
        let covered_destinations = initial_guess_receipts
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        let (
            candidate,
            envelope,
            ending_stage3,
            _final_lower_boundaries,
            final_boundary_receipts,
            final_lane_boundary_receipts,
            _final_destination_receipts,
            final_component_carrier_receipts,
            _final_shortwave_by_lane,
            _final_longwave_by_lane,
        ) = 'fixed_point: {
            for _iteration in 0..COVERED_FIXED_POINT_POLICY.max_iterations {
                let destination_receipts = initial_guess_receipts.clone();
                let carrier_boundaries = initial_guess_boundaries.clone();
                // The reduced carrier supplies only the first numerical guess.
                // After one LSE evaluation, the complete component-resolved
                // boundary is the sole iterate consumed by Stage 3.
                let flux_boundaries = iteration_boundaries.as_ref().unwrap_or(&carrier_boundaries);
                let current_boundaries = self.merge_latest_stage3_state_operands(
                    flux_boundaries,
                    &iteration_stage3_states,
                )?;
                let mut provisional_candidate = self.beginning.clone();
                provisional_candidate.inner.authority = CoveredColumnAuthority::V11SnowCovered;
                let provisional_envelope = provisional_candidate
                    .inner
                    .construct_covered_interval_envelope_with_duration(
                        self.day_index,
                        self.interval_index,
                        self.interval,
                        interval_s,
                        input.duration_s_bits,
                        &covered_destinations,
                        &current_boundaries,
                        true,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })?;
                let (next_boundaries, _next_shortwave_by_lane, _next_longwave_by_lane) = self
                    .corrected_covered_boundaries_from_envelope(
                        &current_boundaries,
                        &provisional_envelope,
                    )?;
                let lse_states = provisional_envelope
                    .covered_lse_iteration_state_by_destination()
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "covered provisional LSE iteration state",
                        )
                    })?;
                let next_boundaries =
                    self.apply_lse_iteration_exchange(&next_boundaries, &lse_states)?;
                let stage3_candidate =
                    evaluate_stage3(&destination_receipts, &next_boundaries, None)?;
                let next_destination_receipts = initial_guess_receipts.clone();
                let lse_converged = previous_lse_states.as_ref().is_some_and(|previous| {
                    covered_fixed_point_lse_states_equal(previous, &lse_states)
                });
                let stage3_converged = previous_stage3_states.as_ref().is_some_and(|previous| {
                    covered_fixed_point_stage3_states_equal(previous, &stage3_candidate)
                });
                let boundary_converged =
                    covered_fixed_point_boundaries_equal(&current_boundaries, &next_boundaries);
                let converged = lse_converged && stage3_converged && boundary_converged;
                if !converged {
                    previous_lse_states = Some(lse_states);
                    previous_stage3_states = Some(stage3_candidate.clone());
                    iteration_stage3_states = stage3_candidate;
                    iteration_boundaries = Some(next_boundaries);
                    continue;
                }

                let mut final_candidate = self.beginning.clone();
                final_candidate.inner.authority = CoveredColumnAuthority::V11SnowCovered;
                let final_input_boundaries = next_boundaries.clone();
                let final_envelope = final_candidate
                    .inner
                    .construct_covered_interval_envelope_with_duration(
                        self.day_index,
                        self.interval_index,
                        self.interval,
                        interval_s,
                        input.duration_s_bits,
                        &covered_destinations,
                        &final_input_boundaries,
                        false,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })?;
                let (final_corrected_boundaries, final_shortwave_by_lane, final_longwave_by_lane) =
                    self.corrected_covered_boundaries_from_envelope(
                        &final_input_boundaries,
                        &final_envelope,
                    )?;
                let final_lse_states = final_envelope
                    .covered_lse_iteration_state_by_destination()
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity("covered final LSE iteration state")
                    })?;
                let final_rebuilt_boundaries = self
                    .apply_lse_iteration_exchange(&final_corrected_boundaries, &final_lse_states)?;
                let final_stage3_candidate =
                    evaluate_stage3(&next_destination_receipts, &final_rebuilt_boundaries, None)?;
                let final_next_destination_receipts = initial_guess_receipts.clone();
                if !covered_fixed_point_boundaries_equal(
                    &final_input_boundaries,
                    &final_rebuilt_boundaries,
                ) || !covered_fixed_point_lse_states_equal(&lse_states, &final_lse_states)
                    || !covered_fixed_point_stage3_states_equal(
                        &stage3_candidate,
                        &final_stage3_candidate,
                    )
                {
                    previous_lse_states = Some(final_lse_states);
                    previous_stage3_states = Some(final_stage3_candidate.clone());
                    iteration_stage3_states = final_stage3_candidate;
                    iteration_boundaries = Some(final_rebuilt_boundaries);
                    continue;
                }
                let sealed_source_envelope = final_candidate
                    .inner
                    .construct_covered_interval_envelope_with_duration(
                        self.day_index,
                        self.interval_index,
                        self.interval,
                        interval_s,
                        input.duration_s_bits,
                        &covered_destinations,
                        &final_rebuilt_boundaries,
                        false,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })?;
                let (sealed_source_corrected_boundaries, _, _) = self
                    .corrected_covered_boundaries_from_envelope(
                        &final_rebuilt_boundaries,
                        &sealed_source_envelope,
                    )?;
                let sealed_source_lse_states = sealed_source_envelope
                    .covered_lse_iteration_state_by_destination()
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "covered sealed-source LSE iteration state",
                        )
                    })?;
                let sealed_source_boundaries = self.apply_lse_iteration_exchange(
                    &sealed_source_corrected_boundaries,
                    &sealed_source_lse_states,
                )?;
                let ending_v8_physical_candidate_sha256 = digest32_from_lower_hex(
                    &sealed_source_envelope
                        .vegetation()
                        .ending_state()
                        .state_sha256,
                )?;
                let ending_stage3_state_sha256 = digest_bytes(
                    &canonical_stage3_snow_owner_bytes_v11(&final_stage3_candidate)?,
                );
                let (final_lower_boundaries, final_boundary_receipts) = self
                    .seal_final_covered_boundaries(
                        input,
                        &sealed_source_boundaries,
                        &final_next_destination_receipts,
                        &sealed_source_envelope,
                        ending_v8_physical_candidate_sha256,
                        ending_stage3_state_sha256,
                    )?;
                let final_lane_boundary_receipts =
                    self.final_lane_boundary_receipts(input, &final_boundary_receipts)?;
                let final_envelope = final_candidate
                    .inner
                    .construct_covered_interval_envelope_with_duration(
                        self.day_index,
                        self.interval_index,
                        self.interval,
                        interval_s,
                        input.duration_s_bits,
                        &covered_destinations,
                        &final_lower_boundaries,
                        false,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })?;
                let (self_reconstructed_boundaries, _, _) = self
                    .corrected_covered_boundaries_from_envelope(
                        &final_lower_boundaries,
                        &final_envelope,
                    )?;
                let self_reconstructed_lse_states = final_envelope
                    .covered_lse_iteration_state_by_destination()
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity("covered sealed LSE iteration state")
                    })?;
                let self_reconstructed_boundaries = self.apply_lse_iteration_exchange(
                    &self_reconstructed_boundaries,
                    &self_reconstructed_lse_states,
                )?;
                if !covered_fixed_point_boundaries_equal(
                    &final_lower_boundaries,
                    &self_reconstructed_boundaries,
                ) || !covered_fixed_point_lse_states_equal(
                    &sealed_source_lse_states,
                    &self_reconstructed_lse_states,
                ) {
                    return Err(DirectV11RealConsumerError::Identity(
                        "final covered boundary self-reconstruction",
                    ));
                }
                let final_ending_stage3 = evaluate_stage3(
                    &final_next_destination_receipts,
                    &final_lower_boundaries,
                    Some(&final_lane_boundary_receipts),
                )?;
                if !covered_fixed_point_stage3_states_equal(
                    &final_stage3_candidate,
                    &final_ending_stage3,
                ) {
                    return Err(DirectV11RealConsumerError::Identity(
                        "final Stage-3 lane receipt self-reconstruction",
                    ));
                }
                // The retained receipts must describe the candidate that is
                // actually installed, not the tolerance-equivalent precursor
                // used to discover the fixed point.  Re-seal from the replay
                // outputs, then prove that receipt metadata cannot perturb any
                // physical result.
                let installed_v8_digest = digest32_from_lower_hex(
                    &final_envelope.vegetation().ending_state().state_sha256,
                )?;
                let installed_stage3_digest = digest_bytes(&canonical_stage3_snow_owner_bytes_v11(
                    &final_ending_stage3,
                )?);
                let (installed_lower_boundaries, installed_boundary_receipts) = self
                    .seal_final_covered_boundaries(
                        input,
                        &self_reconstructed_boundaries,
                        &final_next_destination_receipts,
                        &final_envelope,
                        installed_v8_digest,
                        installed_stage3_digest,
                    )?;
                let installed_lane_boundary_receipts =
                    self.final_lane_boundary_receipts(input, &installed_boundary_receipts)?;
                let installed_component_carrier_receipts = self_reconstructed_lse_states
                    .iter()
                    .map(|(destination, state)| {
                        let boundary = installed_boundary_receipts.get(destination).ok_or(
                            DirectV11RealConsumerError::Identity(
                                "installed component carrier boundary destination",
                            ),
                        )?;
                        Ok((
                            destination.clone(),
                            ComponentResolvedCarrierReceiptV1::try_new(
                                destination.clone(),
                                state,
                                boundary,
                            )?,
                        ))
                    })
                    .collect::<Result<BTreeMap<_, _>, DirectV11RealConsumerError>>()?;
                let installed_envelope = final_candidate
                    .inner
                    .construct_covered_interval_envelope_with_duration(
                        self.day_index,
                        self.interval_index,
                        self.interval,
                        interval_s,
                        input.duration_s_bits,
                        &covered_destinations,
                        &installed_lower_boundaries,
                        false,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })?;
                let installed_lse_states = installed_envelope
                    .covered_lse_iteration_state_by_destination()
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "installed covered LSE iteration state",
                        )
                    })?;
                if installed_lse_states != self_reconstructed_lse_states
                    || installed_envelope.vegetation().ending_state()
                        != final_envelope.vegetation().ending_state()
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "sealed covered replay exact physical identity",
                    ));
                }
                let installed_stage3 = evaluate_stage3(
                    &final_next_destination_receipts,
                    &installed_lower_boundaries,
                    Some(&installed_lane_boundary_receipts),
                )?;
                if installed_stage3 != final_ending_stage3 {
                    return Err(DirectV11RealConsumerError::Identity(
                        "sealed Stage-3 replay exact physical identity",
                    ));
                }
                break 'fixed_point Ok::<_, DirectV11RealConsumerError>((
                    final_candidate,
                    installed_envelope,
                    installed_stage3,
                    installed_lower_boundaries,
                    installed_boundary_receipts,
                    installed_lane_boundary_receipts,
                    final_next_destination_receipts,
                    installed_component_carrier_receipts,
                    final_shortwave_by_lane,
                    final_longwave_by_lane,
                ));
            }
            Err(DirectV11RealConsumerError::CoveredBoundary(
                SnowStage3HandoffError::FixedPointIterationLimit,
            ))
        }?;
        let ending_snow_owner_bytes = canonical_stage3_snow_owner_bytes_v11_with_receipts(
            &ending_stage3,
            &final_lane_boundary_receipts,
            &final_boundary_receipts,
        )?;
        let (output, candidate, support_receipt) = finalize_v11_imported_segment(
            &candidate,
            input,
            &envelope,
            ending_snow_owner_bytes,
            self.day_index,
        )?;
        self.last_support_receipt = Some(support_receipt);
        self.last_final_boundary_receipts = Some(final_boundary_receipts);
        self.last_lane_boundary_receipts = Some(final_lane_boundary_receipts);
        self.last_component_carrier_receipts = Some(final_component_carrier_receipts);
        self.ending_stage3_by_lane = Some(ending_stage3);
        self.ending = Some(candidate);
        Ok(output)
    }
}

#[cfg(test)]
mod component_carrier_tests {
    use super::*;
    use openwepp_coupled_time::{ModelTimeNs, TimeSupport};

    fn make_boundary(optical: u8) -> FinalStage3CanopyBoundaryReceiptV1 {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_000_000_000))
            .expect("support");
        FinalStage3CanopyBoundaryReceiptV1::try_new(FinalStage3CanopyBoundaryReceiptInputs {
            support,
            destination: (
                OfeId::try_new("ofe-1").expect("OFE"),
                TileId::try_new("forest").expect("tile"),
            ),
            beginning_v11_state_sha256: Digest32::from_bytes([1; 32]),
            beginning_stage3_state_sha256: Digest32::from_bytes([2; 32]),
            ending_v8_physical_candidate_sha256: Digest32::from_bytes([3; 32]),
            ending_stage3_state_sha256: Digest32::from_bytes([4; 32]),
            provisional_carrier_receipt_sha256: Digest32::from_bytes([5; 32]),
            optical_receipt_sha256: Digest32::from_bytes([optical; 32]),
            reciprocal_longwave_receipt_sha256: Digest32::from_bytes([7; 32]),
            sensible_to_canopy_air_w_m2: 2.0,
            vapor_to_canopy_air_kg_m2_s: 1.0e-6,
            latent_energy_to_canopy_air_j_m2: 2.5,
            snow_temperature_k: 270.0,
            latent_heat_j_kg: 2_500_000.0,
            snow_absorbed_shortwave_w_m2: 10.0,
            snow_net_longwave_w_m2: -5.0,
        })
        .expect("boundary")
    }

    fn state() -> CoveredLseIterationState {
        CoveredLseIterationState {
            canopy_air_temperature_k: 290.0,
            canopy_air_specific_humidity_kg_kg: 0.01,
            snow_temperature_k: 270.0,
            snow_sensible_w_m2: 2.0,
            snow_vapor_kg_m2_s: 1.0e-6,
            snow_latent_w_m2: 2.5,
            snow_net_longwave_w_m2: -5.0,
            component_temperatures_k: vec![("canopy".into(), [292.0; 4])],
            component_carrier_surfaces: (0_u8..4)
                .map(|component_ordinal| CoveredCarrierComponentState {
                    vertical_occupancy_ordinal: 0,
                    occupancy_id: "canopy".into(),
                    component_ordinal,
                    surface_area_m2_m2_tile: 0.25,
                    emissive_area_m2_m2_tile: 0.25,
                    heat_conductance_m_s_tile: 0.25,
                    vapor_conductance_m_s_tile: if component_ordinal == 3 { 0.0 } else { 0.25 },
                    vapor_authorization_kg_m2_tile_s: None,
                    temperature_k: 292.0,
                    specific_humidity_kg_kg: 0.011,
                    sensible_to_canopy_air_w_m2: 0.75,
                    vapor_to_canopy_air_kg_m2_s: if component_ordinal == 3 {
                        0.0
                    } else if component_ordinal == 2 {
                        1.0e-6
                    } else {
                        0.5e-6
                    },
                })
                .collect(),
            canopy_sensible_w_m2: 3.0,
            canopy_vapor_kg_m2_s: 2.0e-6,
            sensible_to_reference_air_w_m2: 5.0,
            vapor_to_reference_air_kg_m2_s: 3.0e-6,
        }
    }

    #[test]
    fn component_carrier_rejects_stale_inner_seal_and_fresh_boundary_substitution() {
        let boundary = make_boundary(6);
        let mut receipt = ComponentResolvedCarrierReceiptV1::try_new(
            boundary.destination.clone(),
            &state(),
            &boundary,
        )
        .expect("component receipt");
        receipt.components[0].temperature_k += 1.0;
        assert!(receipt.validate(&boundary).is_err());

        let alternate_boundary = make_boundary(8);
        let receipt = ComponentResolvedCarrierReceiptV1::try_new(
            boundary.destination.clone(),
            &state(),
            &boundary,
        )
        .expect("component receipt");
        assert!(receipt.validate(&alternate_boundary).is_err());
    }

    #[test]
    fn component_carrier_uses_vertical_order_not_lexical_occupancy_order() {
        let boundary = make_boundary(6);
        let mut physical = state();
        let upper = physical
            .component_carrier_surfaces
            .iter()
            .cloned()
            .map(|mut component| {
                component.occupancy_id = "z-upper".into();
                component.surface_area_m2_m2_tile *= 0.5;
                component.emissive_area_m2_m2_tile *= 0.5;
                component.heat_conductance_m_s_tile *= 0.5;
                component.vapor_conductance_m_s_tile *= 0.5;
                component.sensible_to_canopy_air_w_m2 *= 0.5;
                component.vapor_to_canopy_air_kg_m2_s *= 0.5;
                component
            })
            .collect::<Vec<_>>();
        let lower = upper
            .iter()
            .cloned()
            .map(|mut component| {
                component.vertical_occupancy_ordinal = 1;
                component.occupancy_id = "a-lower".into();
                component
            })
            .collect::<Vec<_>>();
        physical.component_carrier_surfaces = upper.into_iter().chain(lower).collect();
        ComponentResolvedCarrierReceiptV1::try_new(
            boundary.destination.clone(),
            &physical,
            &boundary,
        )
        .expect("physical vertical order is authoritative");
    }

    #[test]
    fn component_carrier_rejects_duplicate_occupancy_across_vertical_ordinals() {
        let boundary = make_boundary(6);
        let mut physical = state();
        let mut duplicate = physical.component_carrier_surfaces.clone();
        for component in &mut duplicate {
            component.vertical_occupancy_ordinal = 1;
        }
        physical.component_carrier_surfaces.extend(duplicate);
        physical.canopy_sensible_w_m2 *= 2.0;
        physical.canopy_vapor_kg_m2_s *= 2.0;
        physical.sensible_to_reference_air_w_m2 =
            physical.canopy_sensible_w_m2 + physical.snow_sensible_w_m2;
        physical.vapor_to_reference_air_kg_m2_s =
            physical.canopy_vapor_kg_m2_s + physical.snow_vapor_kg_m2_s;
        assert!(
            ComponentResolvedCarrierReceiptV1::try_new(
                boundary.destination.clone(),
                &physical,
                &boundary,
            )
            .is_err()
        );
    }
}

#[cfg(test)]
mod covered_convergence_policy_tests {
    use super::*;
    use crate::DirectSnowLayerState;

    fn state() -> DirectSnowStage3PersistentState {
        Wb11HydrologyKernel::initialize_stage3_persistent_state(
            7,
            vec![DirectSnowLayerState::new(0.1, 0.2, 500.0, 3.0)],
        )
        .expect("persistent state")
    }

    fn equal(
        left: DirectSnowStage3PersistentState,
        right: DirectSnowStage3PersistentState,
    ) -> bool {
        covered_fixed_point_stage3_states_equal(
            &BTreeMap::from([(7, left)]),
            &BTreeMap::from([(7, right)]),
        )
    }

    fn reseal(state: &mut DirectSnowStage3PersistentState) {
        state.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(state);
    }

    #[test]
    fn structural_fingerprint_and_count_fields_are_exact() {
        let original = state();
        let mut changed = original.clone();
        changed.fingerprint ^= 1;
        assert!(!equal(original.clone(), changed));
        let mut changed = original.clone();
        changed.layers[0].settle_day_count =
            f64::from_bits(changed.layers[0].settle_day_count.to_bits() + 1);
        reseal(&mut changed);
        assert!(!equal(original, changed));
    }

    #[test]
    fn unit_specific_state_tolerances_do_not_share_one_scale() {
        let original = state();
        let mut within = original.clone();
        within.layers[0].mass_swe_m += 0.5e-9;
        within.layers[0].temperature_c += 0.5e-8;
        within.layers[0].cold_content_j_m2 += 0.5e-6;
        reseal(&mut within);
        assert!(equal(original.clone(), within));
        let mut outside = original.clone();
        outside.layers[0].cold_content_j_m2 += 2.0e-6;
        reseal(&mut outside);
        assert!(!equal(original, outside));
    }

    #[test]
    fn density_is_exact_after_each_state_fingerprint_is_reconstructed() {
        let original = state();
        let mut changed = original.clone();
        changed.layers[0].density_kg_m3 =
            f64::from_bits(changed.layers[0].density_kg_m3.to_bits() + 1);
        reseal(&mut changed);
        assert!(!equal(original, changed));
    }

    #[test]
    fn cumulative_mass_uses_its_area_mass_tolerance() {
        let original = state();
        let mut within = original.clone();
        within.cumulative_snowfall_kg_m2 += 0.5e-6;
        reseal(&mut within);
        assert!(equal(original.clone(), within));

        let mut outside = original.clone();
        outside.cumulative_snowfall_kg_m2 += 2.0e-6;
        reseal(&mut outside);
        assert!(!equal(original, outside));
    }
}
