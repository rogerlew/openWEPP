use super::*;

mod owner_finalization;
pub use owner_finalization::CoveredParentOwnerJoinReceiptV1;
pub(crate) use owner_finalization::CoveredPhysicalCustodyJoinInputs;
pub(crate) use owner_finalization::stage3_support_forcing_digest;
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
    // The 60-second coupled floor can require more contractions than the
    // former subsecond cadence. Keep the reviewed norms unchanged and allow
    // the deterministic iteration to reach them; exhaustion remains
    // fail-closed.
    max_iterations: 96,
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

#[cfg(test)]
std::thread_local! {
    static FORCE_COVERED_FIXED_POINT_EXHAUSTION: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static FORCE_COVERED_FULL_PROVISIONAL_ENVELOPE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static COVERED_RECEIPT_RESEAL_DENSITY_MODE: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static COVERED_RECEIPT_RESEAL_DENSITY_CALLS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(crate) struct ForceCoveredFixedPointExhaustionGuardV1;

#[cfg(test)]
impl Drop for ForceCoveredFixedPointExhaustionGuardV1 {
    fn drop(&mut self) {
        FORCE_COVERED_FIXED_POINT_EXHAUSTION.with(|forced| forced.set(false));
    }
}

#[cfg(test)]
pub(crate) fn force_covered_fixed_point_exhaustion_for_test(
) -> ForceCoveredFixedPointExhaustionGuardV1 {
    FORCE_COVERED_FIXED_POINT_EXHAUSTION.with(|forced| forced.set(true));
    ForceCoveredFixedPointExhaustionGuardV1
}

#[cfg(test)]
fn covered_fixed_point_exhaustion_forced_for_test() -> bool {
    FORCE_COVERED_FIXED_POINT_EXHAUSTION.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(crate) struct CoveredReceiptResealDensityGuardV1;

#[cfg(test)]
impl Drop for CoveredReceiptResealDensityGuardV1 {
    fn drop(&mut self) {
        COVERED_RECEIPT_RESEAL_DENSITY_MODE.with(|mode| mode.set(0));
        COVERED_RECEIPT_RESEAL_DENSITY_CALLS.with(|calls| calls.set(0));
    }
}

#[cfg(test)]
fn begin_covered_receipt_reseal_density_mode_v1(mode: u8) -> CoveredReceiptResealDensityGuardV1 {
    COVERED_RECEIPT_RESEAL_DENSITY_MODE.with(|value| value.set(mode));
    COVERED_RECEIPT_RESEAL_DENSITY_CALLS.with(|calls| calls.set(0));
    CoveredReceiptResealDensityGuardV1
}

#[cfg(test)]
pub(crate) fn force_covered_receipt_reseal_density_ulp_once_for_test(
) -> CoveredReceiptResealDensityGuardV1 {
    begin_covered_receipt_reseal_density_mode_v1(1)
}

#[cfg(test)]
pub(crate) fn force_covered_receipt_reseal_density_noncontraction_for_test(
) -> CoveredReceiptResealDensityGuardV1 {
    begin_covered_receipt_reseal_density_mode_v1(2)
}

#[cfg(test)]
fn apply_covered_receipt_reseal_density_perturbation_for_test(
    states: &mut BTreeMap<u32, DirectSnowStage3PersistentState>,
) {
    let mode = COVERED_RECEIPT_RESEAL_DENSITY_MODE.with(std::cell::Cell::get);
    let call = COVERED_RECEIPT_RESEAL_DENSITY_CALLS.with(|calls| {
        let call = calls.get();
        calls.set(call.saturating_add(1));
        call
    });
    if mode == 0 || (mode == 1 && call != 0) {
        return;
    }
    if let Some(state) = states.values_mut().find(|state| !state.layers.is_empty()) {
        state.layers[0].density_kg_m3 =
            f64::from_bits(state.layers[0].density_kg_m3.to_bits().saturating_add(1));
        state.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(state);
    }
}

#[cfg(test)]
pub(crate) struct ForceCoveredFullProvisionalEnvelopeGuardV1;

#[cfg(test)]
impl Drop for ForceCoveredFullProvisionalEnvelopeGuardV1 {
    fn drop(&mut self) {
        FORCE_COVERED_FULL_PROVISIONAL_ENVELOPE.with(|forced| forced.set(false));
    }
}

#[cfg(test)]
pub(crate) fn force_covered_full_provisional_envelope_for_test(
) -> ForceCoveredFullProvisionalEnvelopeGuardV1 {
    FORCE_COVERED_FULL_PROVISIONAL_ENVELOPE.with(|forced| forced.set(true));
    ForceCoveredFullProvisionalEnvelopeGuardV1
}

#[cfg(test)]
fn covered_full_provisional_envelope_forced_for_test() -> bool {
    FORCE_COVERED_FULL_PROVISIONAL_ENVELOPE.with(std::cell::Cell::get)
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CoveredProvisionalPhysicalAuditV1 {
    pub precipitation_sets: BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>,
    pub corrected_boundaries:
        BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    pub lse_states: BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
    pub transaction_id: TransactionId,
    pub soil_candidates: Vec<SoilThermalTileCandidate>,
}

#[cfg(test)]
std::thread_local! {
    static COVERED_PROVISIONAL_PHYSICAL_AUDIT: std::cell::RefCell<Option<Vec<CoveredProvisionalPhysicalAuditV1>>> = const {
        std::cell::RefCell::new(None)
    };
}

#[cfg(test)]
pub(crate) struct CoveredProvisionalPhysicalAuditGuardV1;

#[cfg(test)]
impl Drop for CoveredProvisionalPhysicalAuditGuardV1 {
    fn drop(&mut self) {
        COVERED_PROVISIONAL_PHYSICAL_AUDIT.with(|audit| *audit.borrow_mut() = None);
    }
}

#[cfg(test)]
pub(crate) fn begin_covered_provisional_physical_audit_v1(
) -> CoveredProvisionalPhysicalAuditGuardV1 {
    COVERED_PROVISIONAL_PHYSICAL_AUDIT.with(|audit| {
        *audit.borrow_mut() = Some(Vec::new());
    });
    CoveredProvisionalPhysicalAuditGuardV1
}

#[cfg(test)]
pub(crate) fn take_covered_provisional_physical_audit_v1(
) -> Vec<CoveredProvisionalPhysicalAuditV1> {
    COVERED_PROVISIONAL_PHYSICAL_AUDIT.with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

#[cfg(test)]
fn record_covered_provisional_physical_audit_v1(
    value: CoveredProvisionalPhysicalAuditV1,
) {
    COVERED_PROVISIONAL_PHYSICAL_AUDIT.with(|audit| {
        if let Some(values) = audit.borrow_mut().as_mut() {
            values.push(value);
        }
    });
}

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
    pub shared_heat_residual_w_m2: f64,
    pub shared_heat_tolerance_w_m2: f64,
    pub shared_vapor_residual_kg_m2_s: f64,
    pub shared_vapor_tolerance_kg_m2_s: f64,
    pub receipt_sha256: Digest32,
}

fn exact_inactive_component_carrier_v1(
    components: &[CoveredCarrierComponentState],
    canopy_sensible_w_m2: f64,
    canopy_vapor_kg_m2_s: f64,
) -> bool {
    canopy_sensible_w_m2.to_bits() == 0.0_f64.to_bits()
        && canopy_vapor_kg_m2_s.to_bits() == 0.0_f64.to_bits()
        && components.iter().all(|component| {
            component.surface_area_m2_m2_tile.to_bits() == 0.0_f64.to_bits()
                && component.emissive_area_m2_m2_tile.to_bits() == 0.0_f64.to_bits()
                && component.heat_conductance_m_s_tile.to_bits() == 0.0_f64.to_bits()
                && component.vapor_conductance_m_s_tile.to_bits() == 0.0_f64.to_bits()
                && component.sensible_to_canopy_air_w_m2.to_bits() == 0.0_f64.to_bits()
                && component.vapor_to_canopy_air_kg_m2_s.to_bits() == 0.0_f64.to_bits()
        })
}

fn reconstruct_component_carrier_by_occupancy_v1(
    components: &[CoveredCarrierComponentState],
) -> (f64, f64) {
    let mut sensible_by_occupancy = Vec::new();
    let mut vapor_by_occupancy = Vec::new();
    for component in components {
        if component.component_ordinal == 0 {
            sensible_by_occupancy.push(0.0);
            vapor_by_occupancy.push(0.0);
        }
        if let Some(value) = sensible_by_occupancy.last_mut() {
            *value += component.sensible_to_canopy_air_w_m2;
        }
        if let Some(value) = vapor_by_occupancy.last_mut() {
            *value += component.vapor_to_canopy_air_kg_m2_s;
        }
    }
    (
        sensible_by_occupancy.into_iter().sum(),
        vapor_by_occupancy.into_iter().sum(),
    )
}

impl ComponentResolvedCarrierReceiptV1 {
    fn try_new(
        destination: (OfeId, TileId),
        state: &CoveredLseIterationState,
        boundary: &FinalStage3CanopyBoundaryReceiptV1,
    ) -> Result<Self, DirectV11RealConsumerError> {
        let (sensible, vapor) = reconstruct_component_carrier_by_occupancy_v1(
            &state.component_carrier_surfaces,
        );
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
        let reconstructed_heat_residual =
            heat_reference - state.sensible_to_reference_air_w_m2;
        let reconstructed_vapor_residual =
            vapor_reference - state.vapor_to_reference_air_kg_m2_s;
        let sensible_joined = reconstructed_heat_residual.to_bits()
            == state.shared_heat_residual_w_m2.to_bits()
            && state.shared_heat_tolerance_w_m2.is_finite()
            && state.shared_heat_tolerance_w_m2 > 0.0
            && reconstructed_heat_residual.abs() <= state.shared_heat_tolerance_w_m2;
        let vapor_joined = reconstructed_vapor_residual.to_bits()
            == state.shared_vapor_residual_kg_m2_s.to_bits()
            && state.shared_vapor_tolerance_kg_m2_s.is_finite()
            && state.shared_vapor_tolerance_kg_m2_s > 0.0
            && reconstructed_vapor_residual.abs() <= state.shared_vapor_tolerance_kg_m2_s;
        if !sensible_joined || !vapor_joined {
            return Err(
                DirectV11RealConsumerError::ComponentCarrierReferenceFluxCustody {
                    ofe_id: destination.0.as_str().to_owned(),
                    tile_id: destination.1.as_str().to_owned(),
                    start_ns: boundary.support.start_ns().get(),
                    end_ns: boundary.support.end_ns().get(),
                    boundary_receipt_sha256: boundary.receipt_sha256,
                    canopy_sensible_w_m2: sensible,
                    snow_sensible_w_m2: state.snow_sensible_w_m2,
                    reconstructed_sensible_w_m2: heat_reference,
                    stated_sensible_w_m2: state.sensible_to_reference_air_w_m2,
                    sensible_delta_w_m2: reconstructed_heat_residual,
                    sensible_allowance_w_m2: state.shared_heat_tolerance_w_m2,
                    canopy_vapor_kg_m2_s: vapor,
                    snow_vapor_kg_m2_s: state.snow_vapor_kg_m2_s,
                    reconstructed_vapor_kg_m2_s: vapor_reference,
                    stated_vapor_kg_m2_s: state.vapor_to_reference_air_kg_m2_s,
                    vapor_delta_kg_m2_s: reconstructed_vapor_residual,
                    vapor_allowance_kg_m2_s: state.shared_vapor_tolerance_kg_m2_s,
                },
            );
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
            shared_heat_residual_w_m2: state.shared_heat_residual_w_m2,
            shared_heat_tolerance_w_m2: state.shared_heat_tolerance_w_m2,
            shared_vapor_residual_kg_m2_s: state.shared_vapor_residual_kg_m2_s,
            shared_vapor_tolerance_kg_m2_s: state.shared_vapor_tolerance_kg_m2_s,
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
        let (sensible, vapor) = reconstruct_component_carrier_by_occupancy_v1(&self.components);
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
            emissive_area += component.emissive_area_m2_m2_tile;
            prior = Some(identity);
        }
        let exact_inactive = exact_inactive_component_carrier_v1(
            &self.components,
            self.canopy_sensible_w_m2,
            self.canopy_vapor_kg_m2_s,
        );
        let invalid_reconstruction = !emissive_area.is_finite()
            || (!exact_inactive && emissive_area <= 0.0)
            || sensible.to_bits() != self.canopy_sensible_w_m2.to_bits()
            || vapor.to_bits() != self.canopy_vapor_kg_m2_s.to_bits()
            || ((sensible + self.snow_sensible_to_canopy_air_w_m2)
                - self.sensible_to_reference_air_w_m2)
                .to_bits()
                != self.shared_heat_residual_w_m2.to_bits()
            || ((vapor + self.snow_vapor_to_canopy_air_kg_m2_s)
                - self.vapor_to_reference_air_kg_m2_s)
                .to_bits()
                != self.shared_vapor_residual_kg_m2_s.to_bits()
            || !self.shared_heat_tolerance_w_m2.is_finite()
            || self.shared_heat_tolerance_w_m2 <= 0.0
            || self.shared_heat_residual_w_m2.abs() > self.shared_heat_tolerance_w_m2
            || !self.shared_vapor_tolerance_kg_m2_s.is_finite()
            || self.shared_vapor_tolerance_kg_m2_s <= 0.0
            || self.shared_vapor_residual_kg_m2_s.abs()
                > self.shared_vapor_tolerance_kg_m2_s;
        if invalid_reconstruction {
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
            self.shared_heat_residual_w_m2,
            self.shared_heat_tolerance_w_m2,
            self.shared_vapor_residual_kg_m2_s,
            self.shared_vapor_tolerance_kg_m2_s,
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

#[cfg(test)]
mod exact_inactive_component_carrier_tests {
    use super::*;

    fn inactive_component() -> CoveredCarrierComponentState {
        CoveredCarrierComponentState {
            vertical_occupancy_ordinal: 0,
            occupancy_id: "inactive-occupancy".into(),
            component_ordinal: 0,
            surface_area_m2_m2_tile: 0.0,
            emissive_area_m2_m2_tile: 0.0,
            heat_conductance_m_s_tile: 0.0,
            vapor_conductance_m_s_tile: 0.0,
            vapor_authorization_kg_m2_tile_s: None,
            temperature_k: 273.15,
            specific_humidity_kg_kg: 0.003_8,
            sensible_to_canopy_air_w_m2: 0.0,
            vapor_to_canopy_air_kg_m2_s: 0.0,
        }
    }

    #[test]
    fn exact_inactive_carrier_rejects_nonzero_component_emissive_and_flux_aggregates() {
        let component = inactive_component();
        assert!(exact_inactive_component_carrier_v1(
            std::slice::from_ref(&component),
            0.0,
            0.0,
        ));

        for poison in [
            |component: &mut CoveredCarrierComponentState| {
                component.surface_area_m2_m2_tile = f64::from_bits(1);
            },
            |component: &mut CoveredCarrierComponentState| {
                component.emissive_area_m2_m2_tile = f64::from_bits(1);
            },
            |component: &mut CoveredCarrierComponentState| {
                component.heat_conductance_m_s_tile = f64::from_bits(1);
            },
            |component: &mut CoveredCarrierComponentState| {
                component.vapor_conductance_m_s_tile = f64::from_bits(1);
            },
            |component: &mut CoveredCarrierComponentState| {
                component.sensible_to_canopy_air_w_m2 = f64::from_bits(1);
            },
            |component: &mut CoveredCarrierComponentState| {
                component.vapor_to_canopy_air_kg_m2_s = f64::from_bits(1);
            },
        ] {
            let mut poisoned = component.clone();
            poison(&mut poisoned);
            assert!(!exact_inactive_component_carrier_v1(
                std::slice::from_ref(&poisoned),
                0.0,
                0.0,
            ));
        }
        assert!(!exact_inactive_component_carrier_v1(
            std::slice::from_ref(&component),
            f64::from_bits(1),
            0.0,
        ));
        assert!(!exact_inactive_component_carrier_v1(
            std::slice::from_ref(&component),
            0.0,
            f64::from_bits(1),
        ));
    }
}
