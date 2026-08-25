use super::*;

mod owner_finalization;
pub use owner_finalization::CoveredParentOwnerJoinReceiptV1;
pub(crate) use owner_finalization::CoveredPhysicalCustodyJoinInputs;
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
