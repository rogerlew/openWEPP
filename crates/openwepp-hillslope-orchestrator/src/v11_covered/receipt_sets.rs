use super::*;

pub(crate) mod owner_finalization;
pub use owner_finalization::CoveredParentOwnerJoinReceiptV1;
pub(crate) use owner_finalization::CoveredPhysicalCustodyJoinInputs;
pub(crate) use owner_finalization::stage3_support_forcing_digest;
pub(crate) use owner_finalization::normalize_v11_staged_parent_lineage;
#[cfg(test)]
pub(crate) use owner_finalization::{
    begin_v50_outer_owner_transition_evidence_v1,
    take_v50_outer_owner_transition_evidence_v1,
};
pub(super) use owner_finalization::*;

#[cfg(test)]
std::thread_local! {
    static FORCE_COVERED_FULL_PROVISIONAL_ENVELOPE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
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
    pub opaque_physical_projection:
        crate::land_surface_energy_shadow::CanonicalCoveredPrivatePhysicalProjectionV1,
    pub complete_physical_projection_sha256: Option<Digest32>,
    pub precipitation_sets: BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>,
    pub corrected_boundaries:
        BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    pub lse_states: BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
    pub transaction_id: TransactionId,
    pub soil_candidates: Vec<SoilThermalTileCandidate>,
    pub physical_endpoint_captured: bool,
    pub soil_candidate: Option<DirectSoilThermalCandidate>,
    pub soil_continuation: Option<Option<DirectSoilThermalUnpublishedContinuationResultV2>>,
    pub batch_boundaries_by_lane: Option<BTreeMap<u32, Stage3SnowSurfaceBoundaryReceiptV1>>,
    pub carrier_source_receipts:
        Option<BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>>,
    pub open_snow_candidates:
        Option<BTreeMap<(OfeId, TileId), OpenSnowTileBoundaryCandidateV1>>,
    pub terminal_soil_trials:
        Option<BTreeMap<u32, physical_outcome_ledger::TerminalSnowBottomSoilTrialResultV1>>,
    pub terminal_soil_credits: Option<BTreeMap<u32, SoilThermalTopBoundaryCreditV1>>,
    surface_custody: Option<CoveredPhysicalSurfaceCustodyV1>,
    pub beginning_stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub ending_stage3_by_lane: Option<BTreeMap<u32, DirectSnowStage3PersistentState>>,
    pub stage3_refreeze_by_lane: BTreeMap<u32, f64>,
    pub wet_canopy_destinations: BTreeSet<(OfeId, TileId)>,
    pub stage3_surface_destinations: Vec<(OfeId, TileId)>,
    pub wb14_child_receipt_set_sha256: String,
    pub wb14_parent_receipt_set_sha256: Option<String>,
    pub wb14_child_replay_bytes: Vec<u8>,
    pub wb14_parent_replay_bytes: Option<Vec<u8>>,
    pub surface_ofe_topology: Vec<OfeId>,
    pub stage3_covered_native: bool,
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
fn covered_provisional_physical_audit_enabled_v1() -> bool {
    COVERED_PROVISIONAL_PHYSICAL_AUDIT.with(|audit| audit.borrow().is_some())
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

#[cfg(test)]
fn record_covered_physical_endpoint_audit_v1(value: &CoveredCarrierPhysicalPhaseResultV1) {
    COVERED_PROVISIONAL_PHYSICAL_AUDIT.with(|audit| {
        let mut audit = audit.borrow_mut();
        let Some(values) = audit.as_mut() else {
            return;
        };
        if let Some(record) = values
            .iter_mut()
            .rev()
            .find(|record| !record.physical_endpoint_captured)
        {
            record.physical_endpoint_captured = true;
            record.soil_candidate = Some(value.validated_soil_ending.candidate().clone());
            record.soil_continuation =
                Some(value.validated_soil_ending.continuation().cloned());
            record.batch_boundaries_by_lane = Some(value.batch_boundaries_by_lane.clone());
            record.carrier_source_receipts = Some(value.carrier_source_receipts.clone());
            record.open_snow_candidates = Some(value.open_snow_candidates.clone());
            record.terminal_soil_trials = Some(value.terminal_soil_trials.clone());
            record.terminal_soil_credits = Some(value.terminal_soil_credits.clone());
            record.surface_custody = Some(value.surface_custody.clone());
            record.complete_physical_projection_sha256 =
                canonical_complete_physical_projection_v1(record);
        }
    });
}

#[cfg(test)]
fn record_covered_provisional_stage3_endpoint_audit_v1(
    ending_stage3_by_lane: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    stage3_refreeze_by_lane: &BTreeMap<u32, f64>,
) {
    COVERED_PROVISIONAL_PHYSICAL_AUDIT.with(|audit| {
        let mut audit = audit.borrow_mut();
        let Some(values) = audit.as_mut() else {
            return;
        };
        if let Some(value) = values
            .iter_mut()
            .rev()
            .find(|value| value.ending_stage3_by_lane.is_none())
        {
            value.ending_stage3_by_lane = Some(ending_stage3_by_lane.clone());
            value.stage3_refreeze_by_lane = stage3_refreeze_by_lane.clone();
            value.complete_physical_projection_sha256 =
                canonical_complete_physical_projection_v1(value);
        }
    });
}

#[cfg(test)]
fn canonical_complete_physical_projection_v1(
    value: &CoveredProvisionalPhysicalAuditV1,
) -> Option<Digest32> {
    fn bytes(out: &mut Vec<u8>, value: &[u8]) {
        out.extend_from_slice(&(value.len() as u64).to_be_bytes());
        out.extend_from_slice(value);
    }
    fn text(out: &mut Vec<u8>, value: &str) {
        bytes(out, value.as_bytes());
    }
    fn float(out: &mut Vec<u8>, value: f64) {
        out.extend_from_slice(&value.to_bits().to_be_bytes());
    }
    fn digest(out: &mut Vec<u8>, value: Digest32) {
        out.extend_from_slice(value.as_bytes());
    }

    let soil_candidate = value.soil_candidate.as_ref()?;
    let soil_continuation = value.soil_continuation.as_ref()?;
    let boundaries = value.batch_boundaries_by_lane.as_ref()?;
    let carriers = value.carrier_source_receipts.as_ref()?;
    let open_snow = value.open_snow_candidates.as_ref()?;
    let terminal_trials = value.terminal_soil_trials.as_ref()?;
    let terminal_credits = value.terminal_soil_credits.as_ref()?;
    let surface = value.surface_custody.as_ref()?;
    let ending_stage3 = value.ending_stage3_by_lane.as_ref()?;
    let mut out = b"OPENWEPP_CANONICAL_COVERED_COMPLETE_PHYSICAL_PROJECTION_V1\0".to_vec();
    digest(&mut out, value.opaque_physical_projection.sha256);
    out.extend_from_slice(&value.transaction_id.0.to_be_bytes());
    for count in [
        value.precipitation_sets.len(),
        value.corrected_boundaries.len(),
        value.lse_states.len(),
        value.soil_candidates.len(),
        boundaries.len(),
        carriers.len(),
        open_snow.len(),
        terminal_trials.len(),
        terminal_credits.len(),
        value.beginning_stage3_by_lane.len(),
        ending_stage3.len(),
    ] {
        out.extend_from_slice(&(count as u64).to_be_bytes());
    }
    for (lane, set) in &value.precipitation_sets {
        out.extend_from_slice(&lane.to_be_bytes());
        digest(&mut out, set.receipt_sha256);
    }
    for ((ofe, tile), boundary) in &value.corrected_boundaries {
        text(&mut out, ofe.as_str());
        text(&mut out, tile.as_str());
        for scalar in [
            boundary.snow_temperature_k,
            boundary.latent_heat_j_kg,
            boundary.sensible_to_canopy_air_w_m2,
            boundary.vapor_to_canopy_air_kg_m2_s,
            boundary.net_longwave_w_m2,
            boundary.shortwave_absorbed_w_m2,
            boundary.precipitation_advection_w_m2,
            boundary.snow_vis_albedo,
            boundary.snow_nir_albedo,
        ] {
            float(&mut out, scalar);
        }
        text(&mut out, boundary.carrier_receipt_id.as_str());
        text(&mut out, boundary.stage3_albedo_state_sha256.as_str());
        text(&mut out, boundary.forcing_receipt_sha256.as_str());
        for optional in [
            boundary.optical_receipt_sha256.as_ref(),
            boundary.reciprocal_longwave_receipt_sha256.as_ref(),
            boundary.final_canopy_boundary_receipt_sha256.as_ref(),
        ] {
            out.push(u8::from(optional.is_some()));
            if let Some(optional) = optional {
                text(&mut out, optional.as_str());
            }
        }
    }
    for candidate in &value.soil_candidates {
        text(&mut out, candidate.owner_id.as_str());
        text(&mut out, candidate.beginning_state_sha256.as_str());
        text(&mut out, candidate.ofe_id.as_str());
        text(&mut out, candidate.tile_id.as_str());
        out.extend_from_slice(&(candidate.layers.len() as u64).to_be_bytes());
        for layer in &candidate.layers {
            text(&mut out, layer.layer_id.as_str());
            for scalar in [
                layer.beginning_enthalpy_j_m2_ofe_ground,
                layer.ground_heat_credit_j_m2_ofe_ground,
                layer.infiltration_enthalpy_credit_j_m2_ofe_ground,
                layer.ending_enthalpy_j_m2_ofe_ground,
                layer.ending_temperature_k,
            ] {
                float(&mut out, scalar);
            }
        }
    }
    match soil_candidate {
        DirectSoilThermalCandidate::V1(candidate) => {
            out.push(1);
            text(&mut out, candidate.state_sha256.as_str());
        }
        DirectSoilThermalCandidate::V2(candidate) => {
            out.push(2);
            text(&mut out, candidate.unpublished_trial_sha256().as_str());
        }
    }
    match soil_continuation {
        Some(continuation) => {
            out.push(1);
            text(
                &mut out,
                continuation
                    .physical_trial()
                    .unpublished_trial_sha256()
                    .as_str(),
            );
            bytes(
                &mut out,
                &serde_json::to_vec(continuation.accumulated_operands()).ok()?,
            );
        }
        None => out.push(0),
    }
    for (lane, boundary) in boundaries {
        out.extend_from_slice(&lane.to_be_bytes());
        out.extend_from_slice(&boundary.support.start_ns().get().to_be_bytes());
        out.extend_from_slice(&boundary.support.end_ns().get().to_be_bytes());
        for scalar in [
            boundary.sensible_energy_j_m2,
            boundary.vapor_mass_kg_m2,
            boundary.latent_energy_j_m2,
            boundary.shortwave_energy_j_m2,
            boundary.net_longwave_energy_j_m2,
            boundary.precipitation_advection_j_m2,
            boundary.snow_soil_heat_j_m2,
            boundary.latent_heat_j_kg,
        ] {
            float(&mut out, scalar);
        }
        digest(&mut out, boundary.beginning_stage3_state_sha256);
        match boundary.identity {
            Stage3BoundaryIdentity::Provisional {
                carrier_receipt_sha256,
            } => {
                out.push(0);
                digest(&mut out, carrier_receipt_sha256);
            }
            Stage3BoundaryIdentity::Final {
                provisional_carrier_receipt_sha256,
                optical_receipt_sha256,
                reciprocal_longwave_receipt_sha256,
                final_destination_receipt_sha256,
                final_lane_receipt_sha256,
            } => {
                out.push(1);
                for value in [
                    provisional_carrier_receipt_sha256,
                    optical_receipt_sha256,
                    reciprocal_longwave_receipt_sha256,
                    final_destination_receipt_sha256,
                    final_lane_receipt_sha256,
                ] {
                    digest(&mut out, value);
                }
            }
        }
    }
    for ((ofe, tile), carrier) in carriers {
        text(&mut out, ofe.as_str());
        text(&mut out, tile.as_str());
        digest(&mut out, carrier.diagnostic_sha256);
    }
    for ((ofe, tile), candidate) in open_snow {
        text(&mut out, ofe.as_str());
        text(&mut out, tile.as_str());
        bytes(&mut out, &serde_json::to_vec(candidate).ok()?);
    }
    for (lane, trial) in terminal_trials {
        out.extend_from_slice(&lane.to_be_bytes());
        digest(&mut out, trial.receipt.receipt_sha256);
    }
    for (lane, credit) in terminal_credits {
        out.extend_from_slice(&lane.to_be_bytes());
        bytes(&mut out, &serde_json::to_vec(credit).ok()?);
    }
    out.extend_from_slice(&surface.transaction_id.0.to_be_bytes());
    out.extend_from_slice(&(surface.ofe_topology.len() as u64).to_be_bytes());
    for ofe_id in &surface.ofe_topology {
        text(&mut out, ofe_id.as_str());
    }
    text(&mut out, &surface.wb14_child_receipt_set_sha256);
    bytes(&mut out, &surface.wb14_child_replay_bytes);
    match (
        &surface.wb14_parent_receipt_set_sha256,
        &surface.wb14_parent_replay_bytes,
    ) {
        (Some(receipt), Some(replay)) => {
            out.push(1);
            text(&mut out, receipt);
            bytes(&mut out, replay);
        }
        (None, None) => out.push(0),
        _ => return None,
    }
    for states in [&value.beginning_stage3_by_lane, ending_stage3] {
        for (lane, state) in states {
            out.extend_from_slice(&lane.to_be_bytes());
            bytes(
                &mut out,
                &Wb11HydrologyKernel::serialize_stage3_persistent_state(state).ok()?,
            );
        }
    }
    Some(digest_bytes(&out))
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
