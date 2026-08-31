// Pure, unpublished covered-carrier trial construction.
//
// This phase deliberately stops at the sealed Stage-3 boundary.  It does not
// evaluate Stage 3, adopt an owner envelope, accept a coupled-time slab, or
// publish any receipt into the owning stack.

use crate::hydrology::{
    CoveredProbeChildIdentityV1, CoveredTerminalBatchCarrierCandidatesV2,
    CoveredTerminalBatchTrialRequestV2, CoveredTerminalJointTrialStateV1,
    CoveredTerminalLaneTrialStateV2, CoveredTerminalTrialRequestV1,
    CoveredTerminalTrialTransitionV1,
};

#[cfg(test)]
std::thread_local! {
    static COVERED_CARRIER_SUPPORT_AUDIT: std::cell::RefCell<Option<Vec<TimeSupport>>> = const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub(crate) fn begin_covered_carrier_support_audit() {
    COVERED_CARRIER_SUPPORT_AUDIT.with(|audit| *audit.borrow_mut() = Some(Vec::new()));
}

#[cfg(test)]
pub(crate) fn take_covered_carrier_support_audit() -> Vec<TimeSupport> {
    COVERED_CARRIER_SUPPORT_AUDIT.with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

#[cfg(test)]
pub(crate) fn audit_covered_carrier_support(support: TimeSupport) {
    COVERED_CARRIER_SUPPORT_AUDIT.with(|audit| {
        if let Some(supports) = audit.borrow_mut().as_mut() {
            supports.push(support);
        }
    });
}

#[cfg(not(test))]
pub(crate) fn audit_covered_carrier_support(_: TimeSupport) {}

/// Snow operand presented to the shared covered carrier engine.
///
/// Persistent execution uses the canonical Stage-3 lane map. Terminal trials
/// retain that map only as lineage and replace the target lane's physical
/// bottom/surface operands with the aggregate one-volume trial state.
#[derive(Clone, Debug)]
pub(crate) enum CoveredSnowBoundaryStateV1 {
    TerminalTrial {
        lane_id: u32,
        ice_kg_m2: f64,
        liquid_kg_m2: f64,
        cold_content_j_m2: f64,
        surface_temperature_k: f64,
        depth_m: f64,
        density_kg_m3: f64,
    },
    BatchTerminalTrial {
        lanes: BTreeMap<u32, CoveredTerminalLaneTrialStateV2>,
    },
}

/// Whether a shared carrier result is an unpublished probe or the candidate
/// used by the accepted covered execution path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CoveredCarrierExecutionIdentityV1 {
    Probe(CoveredProbeChildIdentityV1),
}

impl CoveredSnowBoundaryStateV1 {
    fn lane_states(
        &self,
        request: &CoveredTerminalTrialRequestV1,
    ) -> BTreeMap<u32, CoveredTerminalLaneTrialStateV2> {
        match self {
            Self::TerminalTrial {
                lane_id,
                ice_kg_m2,
                liquid_kg_m2,
                cold_content_j_m2,
                surface_temperature_k,
                depth_m,
                density_kg_m3,
            } => BTreeMap::from([(
                *lane_id,
                CoveredTerminalLaneTrialStateV2 {
                    lane_id: *lane_id,
                    ice_kg_m2: *ice_kg_m2,
                    liquid_kg_m2: *liquid_kg_m2,
                    cold_content_j_m2: *cold_content_j_m2,
                    surface_temperature_c: *surface_temperature_k - 273.15,
                    snow_depth_m: *depth_m,
                    snow_density_kg_m3: *density_kg_m3,
                    resolved_beginning: crate::hydrology::stage3_is_resolved_thermal_domain(
                        &request.beginning_stage3_state,
                    ),
                    candidate_event_tick: None,
                },
            )]),
            Self::BatchTerminalTrial { lanes } => lanes.clone(),
        }
    }

    fn apply_to_boundary_sets(
        &self,
        bindings: &[crate::direct_runtime::DirectSurfaceLiquidOfeBinding],
        covered_boundaries: &mut BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        open_boundaries: &mut BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    ) -> Result<(), DirectV11RealConsumerError> {
        self.apply_to_boundary_sets_with_topology(
            bindings,
            covered_boundaries,
            open_boundaries,
            true,
        )
    }

    fn apply_to_boundaries(
        &self,
        bindings: &[crate::direct_runtime::DirectSurfaceLiquidOfeBinding],
        boundaries: &mut BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    ) -> Result<(), DirectV11RealConsumerError> {
        self.apply_to_boundary_sets_with_topology(bindings, boundaries, &mut BTreeMap::new(), false)
    }

    fn apply_to_boundary_sets_with_topology(
        &self,
        bindings: &[crate::direct_runtime::DirectSurfaceLiquidOfeBinding],
        covered_boundaries: &mut BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        open_boundaries: &mut BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
        require_every_lane: bool,
    ) -> Result<(), DirectV11RealConsumerError> {
        let lanes = match self {
            Self::TerminalTrial {
                lane_id,
                ice_kg_m2,
                liquid_kg_m2,
                cold_content_j_m2,
                surface_temperature_k,
                depth_m,
                density_kg_m3,
            } => BTreeMap::from([(
                *lane_id,
                CoveredTerminalLaneTrialStateV2 {
                    lane_id: *lane_id,
                    ice_kg_m2: *ice_kg_m2,
                    liquid_kg_m2: *liquid_kg_m2,
                    cold_content_j_m2: *cold_content_j_m2,
                    surface_temperature_c: *surface_temperature_k - 273.15,
                    snow_depth_m: *depth_m,
                    snow_density_kg_m3: *density_kg_m3,
                    resolved_beginning: false,
                    candidate_event_tick: None,
                },
            )]),
            Self::BatchTerminalTrial { lanes } => lanes.clone(),
        };
        for (lane_id, lane) in lanes {
            let surface_temperature_k = lane.surface_temperature_c + 273.15;
            if lane.lane_id != lane_id
                || !surface_temperature_k.is_finite()
                || surface_temperature_k <= 0.0
                || !lane.ice_kg_m2.is_finite()
                || lane.ice_kg_m2 < 0.0
                || !lane.liquid_kg_m2.is_finite()
                || lane.liquid_kg_m2 < 0.0
                || !lane.cold_content_j_m2.is_finite()
                || lane.cold_content_j_m2 < 0.0
                || !lane.snow_depth_m.is_finite()
                || lane.snow_depth_m < 0.0
                || !lane.snow_density_kg_m3.is_finite()
                || lane.snow_density_kg_m3 <= 0.0
                || (lane.ice_kg_m2 - lane.snow_density_kg_m3 * lane.snow_depth_m).abs() > 1.0e-9
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered terminal trial snow boundary state",
                ));
            }
            let surface_temperature = TemperatureCelsius::try_new(lane.surface_temperature_c)
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity(
                        "covered terminal trial snow boundary temperature",
                    )
                })?;
            let latent_heat_j_kg =
                openwepp_meteorology::surface_energy::latent_heat_for_surface_temperature(
                    surface_temperature,
                )
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity(
                        "covered terminal trial snow boundary latent heat",
                    )
                })?
                .as_joules_per_kilogram();
            let mut matched = false;
            for boundaries in [&mut *covered_boundaries, &mut *open_boundaries] {
                for (destination, boundary) in boundaries {
                    if bindings.iter().any(|binding| {
                        binding.ofe_id == destination.0 && binding.production_lane_id == lane_id
                    }) {
                        matched = true;
                        boundary.snow_temperature_k = surface_temperature_k;
                        boundary.latent_heat_j_kg = latent_heat_j_kg;
                    }
                }
            }
            if require_every_lane && !matched {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered terminal trial lane topology",
                ));
            }
        }
        Ok(())
    }

    fn project_trial_stage3_states(
        &self,
        request: &CoveredTerminalTrialRequestV1,
        beginning: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<BTreeMap<u32, DirectSnowStage3PersistentState>, DirectV11RealConsumerError> {
        let mut projected = beginning.clone();
        for (lane_id, lane) in self.lane_states(request) {
            let state = projected
                .get_mut(&lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered terminal trial Stage-3 lane",
                ))?;
            let settle_day_count = state
                .layers
                .first()
                .map_or(0.0, |layer| layer.settle_day_count);
            let refrozen_liquid_m = state
                .layers
                .iter()
                .map(|layer| layer.refrozen_liquid_m)
                .sum::<f64>();
            state.layers = if lane.ice_kg_m2 > 0.0 {
                let mass_swe_m = lane.ice_kg_m2 / 1_000.0;
                vec![crate::DirectSnowLayerState {
                    mass_swe_m,
                    thickness_m: mass_swe_m * 1_000.0 / lane.snow_density_kg_m3,
                    density_kg_m3: lane.snow_density_kg_m3,
                    settle_day_count,
                    temperature_c: lane.surface_temperature_c,
                    liquid_water_m: lane.liquid_kg_m2 / 1_000.0,
                    cold_content_j_m2: lane.cold_content_j_m2,
                    refrozen_liquid_m,
                }]
            } else {
                Vec::new()
            };
            state.detached_retained_liquid_kg_m2 = if lane.ice_kg_m2 > 0.0 {
                0.0
            } else {
                lane.liquid_kg_m2
            };
            state.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(state);
            Wb11HydrologyKernel::validate_stage3_persistent_state(state).map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "covered terminal trial projected Stage-3 state",
                )
            })?;
        }
        Ok(projected)
    }
}

/// Typed companions for opaque canonical joint-owner bytes.
///
/// Canonical owner bytes are intentionally not a deserialization protocol.
/// The probe therefore retains the typed, unpublished candidates beside their
/// canonical joint identity and validates the pair before every trial.
#[derive(Clone)]
pub(crate) struct CoveredCarrierEphemeralCandidatesV1 {
    joint: CoveredTerminalJointTrialStateV1,
    shadow: DirectV10RealConsumerShadow,
    stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    soil_candidate: Option<DirectSoilThermalCandidate>,
    terminal_snow_soil_trial_receipt:
        Option<physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1>,
}

impl CoveredCarrierEphemeralCandidatesV1 {
    pub(crate) fn try_new(
        joint: CoveredTerminalJointTrialStateV1,
        shadow: DirectV10RealConsumerShadow,
        stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<Self, DirectV11RealConsumerError> {
        Self::try_new_with_joint_posture(
            joint,
            shadow,
            stage3_by_lane,
            None,
            CoveredCarrierTypedJointPostureV1::ResidentBeginning,
        )
    }

    fn try_new_with_soil_candidate(
        joint: CoveredTerminalJointTrialStateV1,
        shadow: DirectV10RealConsumerShadow,
        stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
        soil_candidate: Option<DirectSoilThermalCandidate>,
    ) -> Result<Self, DirectV11RealConsumerError> {
        Self::try_new_with_joint_posture(
            joint,
            shadow,
            stage3_by_lane,
            soil_candidate,
            CoveredCarrierTypedJointPostureV1::CandidateEnding,
        )
    }

    fn try_new_with_joint_posture(
        joint: CoveredTerminalJointTrialStateV1,
        shadow: DirectV10RealConsumerShadow,
        stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
        soil_candidate: Option<DirectSoilThermalCandidate>,
        posture: CoveredCarrierTypedJointPostureV1,
    ) -> Result<Self, DirectV11RealConsumerError> {
        let actual =
            covered_carrier_typed_owner_bytes_v1(&shadow, soil_candidate.as_ref(), posture)?;
        validate_covered_carrier_typed_joint_v1(&joint, &actual, posture)?;
        Ok(Self {
            joint,
            shadow,
            stage3_by_lane,
            soil_candidate,
            terminal_snow_soil_trial_receipt: None,
        })
    }

    pub(crate) const fn joint(&self) -> &CoveredTerminalJointTrialStateV1 {
        &self.joint
    }

    pub(crate) const fn shadow(&self) -> &DirectV10RealConsumerShadow {
        &self.shadow
    }

    pub(crate) const fn stage3_by_lane(&self) -> &BTreeMap<u32, DirectSnowStage3PersistentState> {
        &self.stage3_by_lane
    }

    pub(crate) const fn terminal_snow_soil_trial_receipt(
        &self,
    ) -> Option<&physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1> {
        self.terminal_snow_soil_trial_receipt.as_ref()
    }

    pub(crate) fn try_with_selected_stage3_by_lane(
        &self,
        joint: CoveredTerminalJointTrialStateV1,
        stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<Self, DirectV11RealConsumerError> {
        let beginning = covered_carrier_typed_owner_bytes_v1(
            &self.shadow,
            self.soil_candidate.as_ref(),
            CoveredCarrierTypedJointPostureV1::ResidentBeginning,
        )?;
        let ending = covered_carrier_typed_owner_bytes_v1(
            &self.shadow,
            self.soil_candidate.as_ref(),
            CoveredCarrierTypedJointPostureV1::CandidateEnding,
        )?;
        let beginning_matches = covered_carrier_typed_joint_matches_v1(&joint, &beginning);
        let ending_matches = covered_carrier_typed_joint_matches_v1(&joint, &ending);
        let posture = match (beginning_matches, ending_matches) {
            (true, false) => CoveredCarrierTypedJointPostureV1::ResidentBeginning,
            (false, true) => CoveredCarrierTypedJointPostureV1::CandidateEnding,
            (true, true) if beginning == ending => {
                CoveredCarrierTypedJointPostureV1::ResidentBeginning
            }
            _ => {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered carrier selected typed/joint cardinality",
                ));
            }
        };
        let mut selected = Self::try_new_with_joint_posture(
            joint,
            self.shadow.clone(),
            stage3_by_lane,
            self.soil_candidate.clone(),
            posture,
        )?;
        selected.terminal_snow_soil_trial_receipt = self.terminal_snow_soil_trial_receipt.clone();
        Ok(selected)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CoveredCarrierTypedJointPostureV1 {
    ResidentBeginning,
    CandidateEnding,
}

fn covered_carrier_typed_owner_bytes_v1(
    shadow: &DirectV10RealConsumerShadow,
    soil_candidate: Option<&DirectSoilThermalCandidate>,
    posture: CoveredCarrierTypedJointPostureV1,
) -> Result<BTreeMap<String, Vec<u8>>, DirectV11RealConsumerError> {
    let mut actual = shadow.canonical_owner_state_bytes()?;
    if posture == CoveredCarrierTypedJointPostureV1::CandidateEnding
        && let Some(DirectSoilThermalCandidate::V2(trial)) = soil_candidate
    {
        actual.insert(
            "soil_thermal".to_owned(),
            serde_json::to_vec(trial.ending_state()).map_err(|_| {
                DirectV11RealConsumerError::Identity("covered carrier V2 trial owner bytes")
            })?,
        );
    }
    Ok(actual)
}

fn covered_carrier_typed_joint_matches_v1(
    joint: &CoveredTerminalJointTrialStateV1,
    actual: &BTreeMap<String, Vec<u8>>,
) -> bool {
    !actual.contains_key("snow")
        && joint.owner_bytes().contains_key("snow")
        && joint.owner_bytes().len() == actual.len() + 1
        && actual.iter().all(|(owner_id, bytes)| {
            joint
                .owner_bytes()
                .get(owner_id)
                .is_some_and(|joint_bytes| joint_bytes == bytes)
        })
}

fn validate_covered_carrier_typed_joint_v1(
    joint: &CoveredTerminalJointTrialStateV1,
    actual: &BTreeMap<String, Vec<u8>>,
    posture: CoveredCarrierTypedJointPostureV1,
) -> Result<(), DirectV11RealConsumerError> {
    if covered_carrier_typed_joint_matches_v1(joint, actual) {
        return Ok(());
    }
    Err(DirectV11RealConsumerError::Identity(match posture {
        CoveredCarrierTypedJointPostureV1::ResidentBeginning => {
            "covered carrier typed/joint beginning"
        }
        CoveredCarrierTypedJointPostureV1::CandidateEnding => {
            "covered carrier typed/joint candidate ending"
        }
    }))
}

/// Result of one genuine carrier-only mapping at an exact trial support.
#[derive(Clone)]
pub(crate) struct CoveredCarrierPhaseResultV1 {
    pub transition: CoveredTerminalTrialTransitionV1,
    /// Typed physical beginning for this exact child. This is not the
    /// enclosing accepted slab beginning when terminal integration composes
    /// multiple children.
    pub beginning_candidates: CoveredCarrierEphemeralCandidatesV1,
    pub ending_candidates: CoveredCarrierEphemeralCandidatesV1,
    pub beginning_stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pub precipitation_sets: BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>,
    /// Winning unpublished carrier envelope retained for exact accepted
    /// evidence sealing. Publication must not rerun LSE to recover it.
    pub carrier_envelope: UncommittedCoveredV8OwnerEnvelope,
    /// Complete covered/open lower-boundary candidate consumed by Stage 3.
    pub complete_lower_boundaries: BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    /// Reduced-carrier source receipts needed to seal final destination
    /// evidence without recomputing carrier physics.
    pub carrier_source_receipts: BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>,
    pub open_snow_candidates: BTreeMap<(OfeId, TileId), OpenSnowTileBoundaryCandidateV1>,
    pub covered_lse_states: BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
    pub soil_candidate: DirectSoilThermalCandidate,
    #[cfg(test)]
    pub soil_top_boundary_credit: SoilThermalTopBoundaryCreditV1,
    pub batch_boundaries_by_lane: BTreeMap<u32, Stage3SnowSurfaceBoundaryReceiptV1>,
    pub batch_terminal_snow_soil_trial_receipts_by_lane:
        BTreeMap<u32, physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1>,
    pub batch_soil_top_boundary_credits_by_lane: BTreeMap<u32, SoilThermalTopBoundaryCreditV1>,
    pub wb14_child_receipt_set_sha256: String,
    pub wb14_parent_receipt_set_sha256: Option<String>,
    pub wb14_child_replay_bytes: Vec<u8>,
    pub wb14_parent_replay_bytes: Option<Vec<u8>>,
}

struct CoveredCarrierPhysicalTrialV1 {
    envelope: Box<UncommittedCoveredV8OwnerEnvelope>,
    carrier_receipts: BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>,
    corrected: BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    lse_states: BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
    precipitation_sets: BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>,
    open_snow_candidates: BTreeMap<(OfeId, TileId), OpenSnowTileBoundaryCandidateV1>,
    terminal_soil_trials:
        BTreeMap<u32, physical_outcome_ledger::TerminalSnowBottomSoilTrialResultV1>,
    terminal_soil_credits: BTreeMap<u32, SoilThermalTopBoundaryCreditV1>,
    boundaries_by_lane: BTreeMap<u32, Stage3SnowSurfaceBoundaryReceiptV1>,
}

struct CoveredCarrierPreparedTrialV1 {
    envelope: Box<UncommittedCoveredV8OwnerEnvelope>,
    carrier_receipts: BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>,
    corrected: BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    lse_states: BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
    precipitation_sets: BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>,
    open_snow_candidates: BTreeMap<(OfeId, TileId), OpenSnowTileBoundaryCandidateV1>,
    terminal_soil_trials:
        BTreeMap<u32, physical_outcome_ledger::TerminalSnowBottomSoilTrialResultV1>,
    terminal_soil_credits: BTreeMap<u32, SoilThermalTopBoundaryCreditV1>,
    destination_receipts: BTreeMap<(OfeId, TileId), Digest32>,
    lane_states: BTreeMap<u32, CoveredTerminalLaneTrialStateV2>,
}

struct CoveredCarrierBoundaryPreparedTrialV1 {
    envelope: Box<UncommittedCoveredV8OwnerEnvelope>,
    carrier_receipts: BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>,
    corrected: BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    lse_states: BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
    precipitation_sets: BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>,
    open_snow_candidates: BTreeMap<(OfeId, TileId), OpenSnowTileBoundaryCandidateV1>,
    terminal_soil_trials:
        BTreeMap<u32, physical_outcome_ledger::TerminalSnowBottomSoilTrialResultV1>,
    terminal_soil_credits: BTreeMap<u32, SoilThermalTopBoundaryCreditV1>,
    lane_states: BTreeMap<u32, CoveredTerminalLaneTrialStateV2>,
    terms: BTreeMap<u32, LaneStage3BoundaryTerms>,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub(crate) struct CoveredCarrierCandidateLayoutCountsV1 {
    pub owner_count: usize,
    pub snow_lane_count: usize,
    pub soil_layer_count: usize,
    pub covered_destination_count: usize,
    pub lse_component_surface_count: usize,
    pub lower_boundary_count: usize,
    pub precipitation_lane_count: usize,
}

impl CoveredCarrierPhaseResultV1 {
    pub(crate) fn batch_carrier_candidates_v2(&self) -> CoveredTerminalBatchCarrierCandidatesV2 {
        CoveredTerminalBatchCarrierCandidatesV2 {
            support: self.transition.boundary.support,
            beginning_joint_sha256: self.transition.beginning_joint.receipt_sha256(),
            carrier_joint: self.transition.ending_joint.clone(),
            boundaries_by_lane: self.batch_boundaries_by_lane.clone(),
            ordered_q_ss_receipts_by_lane: self
                .batch_terminal_snow_soil_trial_receipts_by_lane
                .clone(),
        }
    }

    #[cfg(test)]
    pub(crate) fn candidate_layout_counts_v1(&self) -> CoveredCarrierCandidateLayoutCountsV1 {
        CoveredCarrierCandidateLayoutCountsV1 {
            owner_count: self.ending_candidates.joint.owner_bytes().len(),
            snow_lane_count: self.ending_candidates.stage3_by_lane.len(),
            soil_layer_count: self
                .soil_candidate
                .read_view()
                .ordered_ofes()
                .into_iter()
                .map(|ofe| ofe.ordered_layers().len())
                .sum(),
            covered_destination_count: self.covered_lse_states.len(),
            lse_component_surface_count: self
                .covered_lse_states
                .values()
                .map(|state| state.component_carrier_surfaces.len())
                .sum(),
            lower_boundary_count: self.complete_lower_boundaries.len(),
            precipitation_lane_count: self.precipitation_sets.len(),
        }
    }
}

pub(crate) fn stage_unpublished_v2_carrier_owners(
    candidate: &mut DirectV10RealConsumerShadow,
    envelope: &UncommittedCoveredV8OwnerEnvelope,
) -> Result<(), DirectV11RealConsumerError> {
    envelope.validate().map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
    })?;
    candidate.inner.vegetation_state = project_v8_runtime_to_v9(
        envelope.vegetation().ending_state(),
        &candidate.inner.vegetation_configuration,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
    })?;
    candidate.inner.lse_state = build_lse_ending_state(
        &candidate.inner.lse_state,
        envelope.transaction_id(),
        envelope.hydrology().ending_lse_tile_states().to_vec(),
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
            DirectV9RealConsumerError::LandSurface(error),
        ))
    })?;
    candidate.inner.biogeochemistry = envelope.biogeochemistry().ending().clone();
    candidate.inner.hydrology_frame = envelope.hydrology().ending_frame().clone();
    candidate.inner.wb14_parent_working_state = envelope
        .hydrology()
        .surface_ingress()
        .parent_working_state()
        .cloned();
    if envelope
        .hydrology()
        .surface_ingress()
        .advances_persistent_parent_interval()
    {
        candidate.inner.accepted_interval_count = candidate
            .inner
            .accepted_interval_count
            .checked_add(1)
            .ok_or(DirectV11RealConsumerError::Identity(
                "unpublished V2 carrier accepted interval count overflow",
            ))?;
    }
    Ok(())
}

fn unpublished_v2_soil_trial(
    beginning: &CoveredCarrierEphemeralCandidatesV1,
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    support: TimeSupport,
    credits: &[SoilThermalTopBoundaryCreditV1],
) -> Result<DirectSoilThermalCandidate, DirectV11RealConsumerError> {
    let prepared = beginning
        .shadow
        .prepare_soil_thermal_support_v2(
            envelope.transaction_id(),
            support.start_ns().get(),
            support.end_ns().get(),
        )
        .map_err(DirectV11RealConsumerError::Runtime)?;
    let source_owner_id = ResourceOwnerId::try_new("snow").map_err(|_| {
        DirectV11RealConsumerError::Identity("unpublished V2 terminal soil source owner")
    })?;
    let mut operands = beginning
        .soil_candidate
        .as_ref()
        .and_then(|candidate| candidate.v2().ok())
        .map(|trial| {
            trial
                .layer_credits()
                .iter()
                .flat_map(|credit| credit.accepted_operands.iter().cloned())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    operands.extend(
        crate::land_surface_energy_shadow::physical_soil_energy_operands_v2(
            envelope.transaction_id(),
            support.start_ns().get(),
            support.end_ns().get(),
            &beginning.shadow.inner.lse_configuration.owner_id,
            &beginning.shadow.inner.surface_configuration.owner_id,
            envelope.hydrology().pre_ingress_soil_thermal_candidates(),
            envelope.hydrology().surface_ingress(),
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::LandSurfaceShadow(error),
            ))
        })?,
    );
    operands.extend(
        soil_thermal_top_boundary_operands_v2(
            prepared.beginning_owner(),
            credits,
            &source_owner_id,
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
        })?,
    );
    let mut ordinals = BTreeMap::new();
    for operand in &mut operands {
        let ordinal = ordinals
            .entry((
                operand.ofe_id.clone(),
                operand.layer_id.clone(),
                operand.source_kind,
            ))
            .or_insert(0_u32);
        operand.ordinal = *ordinal;
        *ordinal = ordinal
            .checked_add(1)
            .ok_or(DirectV11RealConsumerError::Identity(
                "unpublished V2 soil operand ordinal overflow",
            ))?;
    }
    canonicalize_v2_operand_order(prepared.beginning_owner(), &mut operands).map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
    })?;
    let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
        prepared.beginning_owner(),
        &beginning.shadow.inner.lse_configuration,
        operands,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
    })?;
    let trial = openwepp_land_surface_energy::advance_soil_thermal_trial_v2(
        &prepared,
        expected.accepted_operands(),
        expected.temperature_projections(),
    )
    .map_err(|_| DirectV11RealConsumerError::Identity("unpublished V2 soil trial"))?;
    DirectSoilThermalCandidate::from_v2(trial).map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
    })
}

impl DirectV11SnowCoveredRealConsumerStack<'_> {
    /// Construct the actual V11/LSE/precipitation/snow--soil carrier for one
    /// immutable terminal trial and stop before Stage-3 evaluation.
    pub(crate) fn execute_covered_carrier_phase_v1(
        &self,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        request: &CoveredTerminalTrialRequestV1,
        child: CoveredProbeChildIdentityV1,
    ) -> Result<CoveredCarrierPhaseResultV1, DirectV11RealConsumerError> {
        self.execute_shared_covered_carrier_engine_v1(
            beginning,
            request,
            CoveredSnowBoundaryStateV1::TerminalTrial {
                lane_id: request.lane_id,
                ice_kg_m2: request.ice_kg_m2,
                liquid_kg_m2: request.liquid_kg_m2,
                cold_content_j_m2: request.cold_content_j_m2,
                surface_temperature_k: request.surface_temperature_c + 273.15,
                depth_m: request.snow_depth_m,
                density_kg_m3: request.snow_density_kg_m3,
            },
            CoveredCarrierExecutionIdentityV1::Probe(child),
        )
    }

    /// Construct one carrier candidate for every active lane in a batch.
    /// All lane temperatures are installed in the lower-boundary set before
    /// the single carrier envelope and six shared owners are evaluated.
    pub(crate) fn execute_covered_carrier_batch_phase_v2(
        &self,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        request: &CoveredTerminalBatchTrialRequestV2,
        child: CoveredProbeChildIdentityV1,
    ) -> Result<CoveredCarrierPhaseResultV1, DirectV11RealConsumerError> {
        let (&leader_id, leader) =
            request
                .lanes
                .first_key_value()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered carrier empty terminal batch",
                ))?;
        let expected_active_lanes = beginning
            .stage3_by_lane()
            .iter()
            .filter_map(|(lane_id, state)| {
                (crate::hydrology::stage3_is_resolved_thermal_domain(state)
                    || crate::hydrology::stage3_is_terminal_event_domain(state))
                .then_some(*lane_id)
            })
            .collect::<BTreeSet<_>>();
        if request.beginning_joint != *beginning.joint()
            || request.lanes.keys().copied().collect::<BTreeSet<_>>() != expected_active_lanes
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier batch beginning topology",
            ));
        }
        let beginning_stage3_state = beginning.stage3_by_lane().get(&leader_id).cloned().ok_or(
            DirectV11RealConsumerError::Identity("covered carrier batch leader state"),
        )?;
        let leader_request = CoveredTerminalTrialRequestV1 {
            lane_id: leader_id,
            support: request.support,
            role: request.role,
            attempt_ordinal: request.attempt_ordinal,
            coupling_iteration: 0,
            ice_kg_m2: leader.ice_kg_m2,
            liquid_kg_m2: leader.liquid_kg_m2,
            cold_content_j_m2: leader.cold_content_j_m2,
            surface_temperature_c: leader.surface_temperature_c,
            snow_depth_m: leader.snow_depth_m,
            snow_density_kg_m3: leader.snow_density_kg_m3,
            beginning_stage3_state: Box::new(beginning_stage3_state),
            ending_snow_hint: None,
            beginning_joint: request.beginning_joint.clone(),
        };
        self.execute_shared_covered_carrier_engine_v1(
            beginning,
            &leader_request,
            CoveredSnowBoundaryStateV1::BatchTerminalTrial {
                lanes: request.lanes.clone(),
            },
            CoveredCarrierExecutionIdentityV1::Probe(child),
        )
    }

    /// Execute the value-returning covered carrier engine without adopting a
    /// slab or publishing any receipt. Both persistent and terminal callers
    /// use this mapping; execution identity controls only lineage, never
    /// physical operands.
    fn execute_shared_covered_carrier_engine_v1(
        &self,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        request: &CoveredTerminalTrialRequestV1,
        snow_boundary_state: CoveredSnowBoundaryStateV1,
        execution_identity: CoveredCarrierExecutionIdentityV1,
    ) -> Result<CoveredCarrierPhaseResultV1, DirectV11RealConsumerError> {
        audit_covered_carrier_support(request.support);
        let CoveredCarrierExecutionIdentityV1::Probe(child) = execution_identity;
        if child.trial_support != request.support
            || child.role != request.role
            || child.attempt_ordinal != request.attempt_ordinal
            || child.beginning_joint_sha256 != beginning.joint.receipt_sha256()
            || request.beginning_joint != beginning.joint
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier probe-child join",
            ));
        }
        let interval_s = f64::from_bits(request.support.duration_s_bits());
        if interval_s <= 0.0 || !interval_s.is_finite() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier positive trial support",
            ));
        }
        for forcing in self.stage3_forcing_by_lane.values() {
            if forcing.duration_seconds.to_bits() != interval_s.to_bits() {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered carrier exact projected forcing duration",
                ));
            }
        }
        let projected_vegetation = covered_boxed_execution_v1(|| {
            project_v9_runtime_to_v8(
                &beginning.shadow.inner.vegetation_configuration,
                &beginning.shadow.inner.vegetation_state,
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                    DirectV9RealConsumerError::V9(error),
                ))
            })
        })?;
        let carrier_receipts = self.carrier_receipts_by_destination(
            interval_s,
            &projected_vegetation.1,
            &beginning.stage3_by_lane,
            self.stage3_forcing_by_lane,
        )?;
        let prepared = self.execute_shared_covered_carrier_physical_v1(
            beginning,
            request,
            snow_boundary_state,
            &child,
            interval_s,
            Box::new(carrier_receipts),
        )?;
        let boundary_prepared =
            self.complete_shared_covered_carrier_physical_v1(interval_s, prepared)?;
        let physical = self.build_shared_covered_carrier_boundaries_v1(
            beginning,
            request,
            interval_s,
            boundary_prepared,
        )?;
        self.finalize_shared_covered_carrier_engine_v1(beginning, request, child, physical)
    }

    #[inline(never)]
    fn execute_shared_covered_carrier_physical_v1(
        &self,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        request: &CoveredTerminalTrialRequestV1,
        snow_boundary_state: CoveredSnowBoundaryStateV1,
        child: &CoveredProbeChildIdentityV1,
        interval_s: f64,
        carrier_receipts: Box<BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>>,
    ) -> Result<Box<CoveredCarrierPreparedTrialV1>, DirectV11RealConsumerError> {
        let seed = self.stage3_lower_boundaries_by_destination(
            &carrier_receipts,
            self.stage3_inputs_by_lane,
            self.stage3_forcing_by_lane,
        )?;
        let mut seed = self.merge_latest_stage3_state_operands(&seed, &beginning.stage3_by_lane)?;
        // Stage-3 lower-boundary construction, rather than the broader
        // carrier-diagnostic receipt topology, is the authority for which
        // destinations enter the covered LSE branch. Ordinary canopy/open
        // destinations remain in the complete envelope below, but must not
        // be reclassified as snow-covered merely because they have a carrier
        // diagnostic receipt.
        let covered_destinations = seed.keys().cloned().collect::<BTreeSet<_>>();
        if covered_destinations != self.covered_expected_destinations() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier Stage-3 lower-boundary membership",
            ));
        }
        let trial_stage3_by_lane =
            snow_boundary_state.project_trial_stage3_states(request, &beginning.stage3_by_lane)?;
        let (open_diagnostics, mut open_boundaries, open_snow_candidates) = self
            .open_snow_boundaries_by_destination_with_beginning(
                &trial_stage3_by_lane,
                &beginning.stage3_by_lane,
            )?;
        snow_boundary_state.apply_to_boundary_sets(
            &beginning.shadow.inner.surface_configuration.ofe_bindings,
            &mut seed,
            &mut open_boundaries,
        )?;
        if covered_destinations
            .iter()
            .any(|destination| open_boundaries.contains_key(destination))
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier Stage-3/open destination membership",
            ));
        }
        // One provider call is one joint carrier mapping. The hydrology
        // terminal solver owns the outer fixed-point replay and returns the
        // preceding snow estimate through `ending_snow_hint`; iterating only
        // the carrier here would omit snow and soil from convergence.
        let envelope = covered_boxed_execution_v1(|| {
            self.build_covered_carrier_envelope_value_v1(CoveredCarrierEnvelopeBuildV1 {
                candidate: &beginning.shadow,
                interval_s,
                duration_s_bits: request.support.duration_s_bits(),
                covered_destinations: &covered_destinations,
                covered_boundaries: &seed,
                open_boundaries: &open_boundaries,
                // Every provider replay starts from an unsealed carrier
                // operand. Coupling iteration is joint-solver chronology, not
                // authority to reinterpret that operand as a final optical
                // boundary.
                provisional: true,
                finalize_wb14_parent_interval: self.finalize_wb14_parent_interval,
            })
        })?;
        let (corrected, lse_states) = self.rebuild_covered_lse_carrier_value_v1(
            &seed,
            &envelope,
            &beginning.stage3_by_lane,
            snow_boundary_state.clone(),
        )?;
        let precipitation_sets = self.precipitation_parcel_sets(request.support, &envelope)?;
        let lane_states = snow_boundary_state.lane_states(request);
        if lane_states.is_empty() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier terminal lane set",
            ));
        }
        let lane_to_ofe = self.covered_lane_to_ofe(&beginning.stage3_by_lane)?;
        let mut terminal_soil_trials = BTreeMap::new();
        let mut terminal_soil_credits = BTreeMap::new();
        for (lane_id, lane) in &lane_states {
            let ofe_id =
                lane_to_ofe
                    .get(lane_id)
                    .cloned()
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered carrier terminal snow-soil OFE",
                    ))?;
            let configured_ofe = beginning
                .shadow
                .inner
                .lse_configuration
                .ofes
                .iter()
                .find(|value| value.ofe_id == ofe_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered carrier terminal configured OFE",
                ))?;
            let configured_top = configured_ofe.soil_interface_layers.first().ok_or(
                DirectV11RealConsumerError::Identity(
                    "covered carrier terminal configured soil top",
                ),
            )?;
            let beginning_soil_ofe = beginning
                .shadow
                .inner
                .soil_thermal
                .read_view()
                .ordered_ofes()
                .into_iter()
                .find(|value| value.ofe_id() == &ofe_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered carrier terminal beginning soil OFE",
                ))?;
            let beginning_soil_top = beginning_soil_ofe
                .ordered_layers()
                .into_iter()
                .next()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered carrier terminal beginning soil top",
                ))?;
            let stage3_inputs = self.stage3_inputs_by_lane.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("covered carrier terminal Stage-3 inputs"),
            )?;
            let terminal_soil_trial =
                physical_outcome_ledger::evaluate_terminal_snow_bottom_soil_trial_v1(
                    &physical_outcome_ledger::TerminalSnowBottomSoilTrialInputsV1 {
                        support: request.support,
                        lane_id: *lane_id,
                        ofe_id: &ofe_id,
                        canonical_source_sha256: child.receipt_sha256,
                        ice_kg_m2: lane.ice_kg_m2,
                        liquid_kg_m2: lane.liquid_kg_m2,
                        cold_content_j_m2: lane.cold_content_j_m2,
                        depth_m: lane.snow_depth_m,
                        density_kg_m3: lane.snow_density_kg_m3,
                        temperature_k: lane.surface_temperature_c + 273.15,
                        atmospheric_pressure_pa: stage3_inputs
                            .surface_energy_options
                            .atmospheric_pressure_pa,
                        first_soil_configuration: configured_top,
                        beginning_soil_owner_id: beginning.shadow.inner.soil_thermal.owner_id(),
                        beginning_soil_state_sha256: beginning
                            .shadow
                            .inner
                            .soil_thermal
                            .state_sha256(),
                        transaction_id: envelope.transaction_id(),
                        beginning_first_soil: beginning_soil_top,
                    },
                )
                .map_err(|_| {
                    DirectV11RealConsumerError::Identity("covered carrier terminal snow-soil trial")
                })?;
            let terminal_soil_credit = SoilThermalTopBoundaryCreditV1 {
                lane_id: *lane_id,
                ofe_id,
                first_layer_id: configured_top.layer_id.clone(),
                beginning_owner_id: beginning.shadow.inner.soil_thermal.owner_id().clone(),
                beginning_configuration_sha256: beginning
                    .shadow
                    .inner
                    .soil_thermal
                    .configuration_sha256()
                    .clone(),
                beginning_state_sha256: beginning.shadow.inner.soil_thermal.state_sha256().clone(),
                support_start_ns: i64::try_from(request.support.start_ns().get()).map_err(
                    |_| DirectV11RealConsumerError::Identity("terminal soil credit support start"),
                )?,
                support_end_ns: i64::try_from(request.support.end_ns().get()).map_err(|_| {
                    DirectV11RealConsumerError::Identity("terminal soil credit support end")
                })?,
                accepted_positive_downward_j_m2_ofe_ground: terminal_soil_trial.soil_heat_j_m2,
                soil_thermal_credit_j_m2_ofe_ground: terminal_soil_trial.soil_heat_j_m2,
                snow_soil_heat_receipt_sha256: Sha256Digest::try_new(digest32_hex(
                    terminal_soil_trial.receipt.receipt_sha256,
                ))
                .map_err(|_| DirectV11RealConsumerError::Identity("terminal soil credit digest"))?,
            };
            terminal_soil_credits.insert(*lane_id, terminal_soil_credit);
            terminal_soil_trials.insert(*lane_id, terminal_soil_trial);
        }
        let destination_receipts = carrier_receipts
            .iter()
            .map(|(key, value)| (key.clone(), value.diagnostic_sha256))
            .chain(open_diagnostics)
            .collect::<BTreeMap<_, _>>();
        let mut corrected = corrected;
        for (destination, boundary) in open_boundaries {
            corrected.insert(destination, boundary);
        }
        covered_boxed_execution_v1(|| {
            Ok::<_, DirectV11RealConsumerError>(CoveredCarrierPreparedTrialV1 {
                envelope,
                carrier_receipts: *carrier_receipts,
                corrected,
                lse_states,
                precipitation_sets,
                open_snow_candidates,
                terminal_soil_trials,
                terminal_soil_credits,
                destination_receipts,
                lane_states,
            })
        })
    }

    #[inline(never)]
    fn complete_shared_covered_carrier_physical_v1(
        &self,
        interval_s: f64,
        prepared: Box<CoveredCarrierPreparedTrialV1>,
    ) -> Result<Box<CoveredCarrierBoundaryPreparedTrialV1>, DirectV11RealConsumerError> {
        let CoveredCarrierPreparedTrialV1 {
            envelope,
            carrier_receipts,
            corrected,
            lse_states,
            precipitation_sets,
            open_snow_candidates,
            terminal_soil_trials,
            terminal_soil_credits,
            destination_receipts,
            lane_states,
        } = *prepared;
        let terms =
            self.lane_stage3_terms_from_boundaries(&destination_receipts, &corrected, interval_s)?;
        covered_boxed_execution_v1(|| {
            Ok::<_, DirectV11RealConsumerError>(CoveredCarrierBoundaryPreparedTrialV1 {
                envelope,
                carrier_receipts,
                corrected,
                lse_states,
                precipitation_sets,
                open_snow_candidates,
                terminal_soil_trials,
                terminal_soil_credits,
                lane_states,
                terms,
            })
        })
    }

    #[inline(never)]
    fn build_shared_covered_carrier_boundaries_v1(
        &self,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        request: &CoveredTerminalTrialRequestV1,
        interval_s: f64,
        prepared: Box<CoveredCarrierBoundaryPreparedTrialV1>,
    ) -> Result<Box<CoveredCarrierPhysicalTrialV1>, DirectV11RealConsumerError> {
        let CoveredCarrierBoundaryPreparedTrialV1 {
            envelope,
            carrier_receipts,
            corrected,
            lse_states,
            precipitation_sets,
            open_snow_candidates,
            terminal_soil_trials,
            terminal_soil_credits,
            lane_states,
            terms,
        } = *prepared;
        let mut boundaries_by_lane = BTreeMap::new();
        for lane_id in lane_states.keys() {
            let lane_terms = terms
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered carrier active lane",
                ))?;
            let precipitation =
                precipitation_sets
                    .get(lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered carrier precipitation lane",
                    ))?;
            let (_, advection) = reconstruct_precipitation_mass_and_advected_heat(precipitation)
                .map_err(|error| {
                    DirectV11RealConsumerError::from_stage3_physical_custody(&error)
                })?;
            let snow = beginning.stage3_by_lane.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("covered carrier snow lane"),
            )?;
            let snow_digest = if crate::hydrology::stage3_is_terminal_event_domain(snow) {
                Wb11HydrologyKernel::project_stage3_terminal_surface_state_v1(snow)
            } else {
                Wb11HydrologyKernel::project_stage3_surface_state_v1(snow)
            }
            .map_err(|_| DirectV11RealConsumerError::Identity("covered carrier snow projection"))?
            .beginning_stage3_state_sha256;
            let (sensible, vapor, latent) = outward_snow_fluxes_to_stage3(
                lane_terms.sensible_to_canopy_air_w_m2,
                lane_terms.vapor_to_canopy_air_kg_m2_s,
                lane_terms.latent_energy_to_canopy_air_j_m2,
                interval_s,
            );
            let terminal_soil_trial =
                terminal_soil_trials
                    .get(lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered carrier terminal trial join",
                    ))?;
            let boundary = covered_boxed_execution_v1(|| {
                Stage3SnowSurfaceBoundaryReceiptV1::try_new(
                    Stage3SnowSurfaceBoundaryReceiptInputs {
                        support: request.support,
                        sensible_energy_j_m2: sensible,
                        vapor_mass_kg_m2: vapor,
                        latent_energy_j_m2: latent,
                        shortwave_energy_j_m2: lane_terms.snow_absorbed_shortwave_w_m2 * interval_s,
                        net_longwave_energy_j_m2: lane_terms.snow_net_longwave_w_m2 * interval_s,
                        precipitation_advection_j_m2: advection,
                        snow_soil_heat_j_m2: terminal_soil_trial.snow_heat_j_m2,
                        latent_heat_j_kg: lane_terms.latent_heat_j_kg,
                        beginning_stage3_state_sha256: snow_digest,
                        identity: Stage3BoundaryIdentity::Provisional {
                            carrier_receipt_sha256: lane_terms.provisional_carrier_receipt_sha256,
                        },
                    },
                )
            })?;
            boundaries_by_lane.insert(*lane_id, *boundary);
        }
        covered_boxed_execution_v1(|| {
            Ok::<_, DirectV11RealConsumerError>(CoveredCarrierPhysicalTrialV1 {
                envelope,
                carrier_receipts,
                corrected,
                lse_states,
                precipitation_sets,
                open_snow_candidates,
                terminal_soil_trials,
                terminal_soil_credits,
                boundaries_by_lane,
            })
        })
    }

    #[inline(never)]
    fn finalize_shared_covered_carrier_engine_v1(
        &self,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        request: &CoveredTerminalTrialRequestV1,
        child: CoveredProbeChildIdentityV1,
        physical: Box<CoveredCarrierPhysicalTrialV1>,
    ) -> Result<CoveredCarrierPhaseResultV1, DirectV11RealConsumerError> {
        let CoveredCarrierPhysicalTrialV1 {
            envelope,
            carrier_receipts,
            corrected,
            lse_states,
            precipitation_sets,
            open_snow_candidates,
            terminal_soil_trials,
            terminal_soil_credits,
            boundaries_by_lane,
        } = *physical;
        let boundary = Box::new(*boundaries_by_lane.get(&request.lane_id).ok_or(
            DirectV11RealConsumerError::Identity("covered carrier leader boundary"),
        )?);
        #[cfg(test)]
        let terminal_soil_credit = terminal_soil_credits.get(&request.lane_id).cloned().ok_or(
            DirectV11RealConsumerError::Identity("covered carrier leader soil credit"),
        )?;
        let terminal_soil_trial = terminal_soil_trials.get(&request.lane_id).ok_or(
            DirectV11RealConsumerError::Identity("covered carrier leader soil trial"),
        )?;

        // Adopt only into the unpublished clone. This evolves the six
        // carrier-owned typed candidates without accepting a slab, publishing
        // a receipt, or mutating the owning stack. Hydrology seals the seventh
        // (snow) candidate after applying this boundary.
        let mut candidate = covered_boxed_execution_v1(|| {
            Ok::<_, DirectV11RealConsumerError>(beginning.shadow.clone())
        })?;
        candidate.inner.authority = CoveredColumnAuthority::V11SnowCovered;
        let ordered_soil_credits = terminal_soil_credits.values().cloned().collect::<Vec<_>>();
        let soil_candidate =
            covered_boxed_execution_v1(|| match &beginning.shadow.inner.soil_thermal {
                DirectSoilThermalResident::V1(_) => {
                    candidate
                        .inner
                        .accept_envelope_with_soil_top_boundary_credits(
                            envelope.transaction_id(),
                            &envelope,
                            &ordered_soil_credits,
                        )
                        .map_err(|error| {
                            DirectV11RealConsumerError::Runtime(
                                DirectV10RealConsumerError::Runtime(error),
                            )
                        })?;
                    DirectSoilThermalCandidate::from_v1(
                        candidate
                            .inner
                            .soil_thermal
                            .v1()
                            .map_err(|error| {
                                DirectV11RealConsumerError::Runtime(
                                    DirectV10RealConsumerError::Runtime(error),
                                )
                            })?
                            .clone(),
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })
                }
                DirectSoilThermalResident::V2(_) => {
                    stage_unpublished_v2_carrier_owners(&mut candidate, &envelope)?;
                    unpublished_v2_soil_trial(
                        beginning,
                        &envelope,
                        request.support,
                        &ordered_soil_credits,
                    )
                }
            })?;
        // A trial is a complete unpublished owner candidate, not merely the
        // inner V9 carrier state. Apply the same V10 projections and parent
        // lineage normalization used by accepted segment finalization so a
        // composed child can begin from the exact installable owner set.
        let vegetation_state = covered_boxed_execution_v1(|| {
            project_v9_runtime_to_v10(
                candidate.inner.vegetation_state(),
                &candidate.vegetation_configuration,
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::V10(error))
            })
        })?;
        candidate.vegetation_state = *vegetation_state;
        let lse_state = covered_boxed_execution_v1(|| {
            project_validated_v1_runtime_to_v2(
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
            })
        })?;
        candidate.lse_state = *lse_state;
        normalize_v11_staged_parent_lineage(
            &mut candidate,
            beginning.shadow.vegetation_state.0.last_transaction_id,
        )?;
        let wb14_child_receipt_set_sha256 = envelope
            .hydrology()
            .surface_ingress()
            .wb14_child_receipt_set_sha256()
            .to_string();
        let wb14_parent_receipt_set_sha256 = envelope
            .hydrology()
            .surface_ingress()
            .wb14_parent_receipt_set_sha256()
            .map(ToString::to_string);
        let wb14_child_replay_bytes = envelope
            .hydrology()
            .surface_ingress()
            .wb14_child_replay_bytes()
            .to_vec();
        let wb14_parent_replay_bytes = envelope
            .hydrology()
            .surface_ingress()
            .wb14_parent_replay_bytes()
            .map(ToOwned::to_owned);
        let mut ending_owner_bytes = candidate.canonical_owner_state_bytes()?;
        if let DirectSoilThermalCandidate::V2(trial) = &*soil_candidate {
            ending_owner_bytes.insert(
                "soil_thermal".to_owned(),
                serde_json::to_vec(trial.ending_state()).map_err(|_| {
                    DirectV11RealConsumerError::Identity("unpublished V2 soil trial owner bytes")
                })?,
            );
        }
        let trial_snow = request
            .beginning_joint
            .owner_bytes()
            .get("snow")
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered carrier trial snow owner",
            ))?
            .clone();
        ending_owner_bytes.insert("snow".to_owned(), trial_snow);
        let ending_joint = covered_boxed_execution_v1(|| {
            CoveredTerminalJointTrialStateV1::try_new(
                beginning.joint.authority().clone(),
                ending_owner_bytes,
            )
            .map_err(|_| DirectV11RealConsumerError::Identity("covered carrier ending joint"))
        })?;
        let mut ending_candidates = covered_boxed_execution_v1(|| {
            CoveredCarrierEphemeralCandidatesV1::try_new_with_soil_candidate(
                *ending_joint,
                *candidate,
                beginning.stage3_by_lane.clone(),
                matches!(&*soil_candidate, DirectSoilThermalCandidate::V2(_))
                    .then(|| soil_candidate.as_ref().clone()),
            )
        })?;
        ending_candidates.terminal_snow_soil_trial_receipt =
            Some(terminal_soil_trial.receipt.clone());
        let transition = Box::new(CoveredTerminalTrialTransitionV1 {
            boundary: *boundary,
            beginning_joint: beginning.joint.clone(),
            ending_joint: ending_candidates.joint.clone(),
            probe_child_identity: child,
            trial_snow_soil_receipt: Some(terminal_soil_trial.receipt.clone()),
        });
        let result = covered_boxed_execution_v1(|| {
            Ok::<_, DirectV11RealConsumerError>(CoveredCarrierPhaseResultV1 {
                transition: *transition,
                beginning_candidates: beginning.clone(),
                ending_candidates: *ending_candidates,
                beginning_stage3_by_lane: beginning.stage3_by_lane.clone(),
                precipitation_sets,
                carrier_envelope: *envelope,
                complete_lower_boundaries: corrected,
                carrier_source_receipts: carrier_receipts,
                open_snow_candidates,
                covered_lse_states: lse_states,
                soil_candidate: *soil_candidate,
                #[cfg(test)]
                soil_top_boundary_credit: terminal_soil_credit,
                batch_boundaries_by_lane: boundaries_by_lane,
                batch_terminal_snow_soil_trial_receipts_by_lane: terminal_soil_trials
                    .iter()
                    .map(|(lane_id, trial)| (*lane_id, trial.receipt.clone()))
                    .collect(),
                batch_soil_top_boundary_credits_by_lane: terminal_soil_credits,
                wb14_child_receipt_set_sha256,
                wb14_parent_receipt_set_sha256,
                wb14_child_replay_bytes,
                wb14_parent_replay_bytes,
            })
        })?;
        Ok(*result)
    }
}

#[cfg(test)]
mod covered_carrier_phase_tests {
    use super::*;
    use crate::hydrology::JointTrialAuthorityV1;
    use openwepp_coupled_time::ModelTimeNs;

    fn test_sha256(character: char) -> Sha256Digest {
        Sha256Digest::try_new(character.to_string().repeat(64)).expect("test digest")
    }

    fn native_v2_shadow_and_trials() -> (
        DirectV10RealConsumerShadow,
        DirectSoilThermalCandidate,
        DirectSoilThermalCandidate,
    ) {
        let (v1_shadow, _) = crate::v9_real_consumer_shadow::tests::v10_shadow_fixture();
        let current_transaction = TransactionId(v1_shadow.vegetation_state.0.last_transaction_id);
        let support_transaction = TransactionId(current_transaction.0 + 1);
        let migrated = openwepp_land_surface_energy::migrate_soil_thermal_v1_to_v2(
            v1_shadow
                .inner
                .soil_thermal
                .v1()
                .expect("V1 fixture resident"),
            openwepp_land_surface_energy::SoilThermalV2MigrationIdentity {
                model_version: v1_shadow
                    .inner
                    .lse_configuration
                    .soil_thermal_configuration
                    .model_version
                    .clone(),
                model_definition_sha256: v1_shadow
                    .inner
                    .lse_configuration
                    .soil_thermal_configuration
                    .model_definition_sha256
                    .clone(),
                run_id: "covered-carrier-native-v2".to_owned(),
                transaction_id: current_transaction,
                support_start_ns: 0,
                support_end_ns: 60_000_000_000,
                receipt_chain_sha256: test_sha256('a'),
            },
        )
        .expect("checked V2 migration");
        let prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
            &migrated,
            support_transaction,
            60_000_000_000,
            120_000_000_000,
        )
        .expect("prepared V2 carrier support");
        let receipt_free_seals =
            openwepp_land_surface_energy::seal_soil_thermal_receipt_free_owner_v2(&prepared)
                .expect("receipt-free V2 seals");
        let v2_shadow = DirectV10RealConsumerShadow::try_new_v2(
            v1_shadow.vegetation_configuration.clone(),
            v1_shadow.vegetation_state.clone(),
            v1_shadow.inner.vegetation_owner_id.clone(),
            v1_shadow.lse_configuration.clone(),
            v1_shadow.lse_state.clone(),
            v1_shadow.inner.surface_configuration.clone(),
            v1_shadow.inner.layer_maps.clone(),
            prepared.clone(),
            receipt_free_seals,
            v1_shadow.inner.biogeochemistry.clone(),
            v1_shadow.inner.hydrology_frame.clone(),
            v1_shadow.inner.next_day_index,
            v1_shadow.gsi_owner_configuration.clone(),
            v1_shadow.gsi_state.clone(),
            v1_shadow.provider_static_configuration.clone(),
            v1_shadow.provider_cursor.clone(),
            v1_shadow.root_zone_hydraulic_configuration.clone(),
        )
        .expect("native V2 shadow");
        let trial = |energy_j_m2_ofe_ground: f64, identity: char| {
            let beginning = prepared.beginning_owner();
            let operand = openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2 {
                ofe_id: beginning.state.ofes[0].ofe_id.clone(),
                layer_id: beginning.state.ofes[0].ordered_layers[0].layer_id.clone(),
                source_kind:
                    openwepp_land_surface_energy::SoilThermalEnergyOperandKindV2::SoilInternal,
                source_owner_id: ResourceOwnerId::try_new("carrier-v2-test-source")
                    .expect("source owner"),
                debit_credit_identity_sha256: test_sha256(identity),
                ordinal: 0,
                units: "J m^-2 OFE-ground".to_owned(),
                basis: "ofe_ground".to_owned(),
                energy_j_m2_ofe_ground,
            };
            let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
                beginning,
                &v2_shadow.inner.lse_configuration,
                vec![operand],
            )
            .expect("expected V2 operands");
            let trial = openwepp_land_surface_energy::advance_soil_thermal_trial_v2(
                &prepared,
                expected.accepted_operands(),
                expected.temperature_projections(),
            )
            .expect("unpublished V2 trial");
            DirectSoilThermalCandidate::from_v2(trial).expect("typed V2 candidate")
        };
        let candidate = trial(0.25, 'b');
        let stale_candidate = trial(0.5, 'c');
        (v2_shadow, candidate, stale_candidate)
    }

    fn carrier_joint(
        shadow: &DirectV10RealConsumerShadow,
        soil_candidate: Option<&DirectSoilThermalCandidate>,
    ) -> CoveredTerminalJointTrialStateV1 {
        let posture = if soil_candidate.is_some() {
            CoveredCarrierTypedJointPostureV1::CandidateEnding
        } else {
            CoveredCarrierTypedJointPostureV1::ResidentBeginning
        };
        let mut owner_bytes = covered_carrier_typed_owner_bytes_v1(shadow, soil_candidate, posture)
            .expect("typed owner bytes");
        let snow = vec![7, 11, 13];
        owner_bytes.insert("snow".to_owned(), snow.clone());
        CoveredTerminalJointTrialStateV1::try_new(
            JointTrialAuthorityV1 {
                source_owner_set_sha256: Digest32::from_bytes([17; 32]),
                lane_id: 1,
                source_snow_owner_sha256: digest_bytes(&snow),
                interval_index: 0,
                state_support: TimeSupport::new(
                    ModelTimeNs::new(60_000_000_000),
                    ModelTimeNs::new(120_000_000_000),
                )
                .expect("support"),
                accepted_predecessors: Vec::new(),
            },
            owner_bytes,
        )
        .expect("carrier joint")
    }

    fn test_boundary(latent_heat_j_kg: f64) -> Stage3SnowCoveredLowerBoundary {
        let digest = Sha256Digest::try_new("11".repeat(32)).expect("digest");
        Stage3SnowCoveredLowerBoundary {
            snow_temperature_k: 273.15,
            latent_heat_j_kg,
            sensible_to_canopy_air_w_m2: 0.0,
            vapor_to_canopy_air_kg_m2_s: 0.0,
            net_longwave_w_m2: 0.0,
            shortwave_absorbed_w_m2: 0.0,
            precipitation_advection_w_m2: 0.0,
            carrier_receipt_id: digest.clone(),
            snow_vis_albedo: 0.8,
            snow_nir_albedo: 0.8,
            stage3_albedo_state_sha256: digest.clone(),
            forcing_receipt_sha256: digest,
            optical_receipt_sha256: None,
            reciprocal_longwave_receipt_sha256: None,
            final_canopy_boundary_receipt_sha256: None,
        }
    }

    #[test]
    fn native_v2_selected_joint_binds_resident_beginning_and_trial_ending_exactly() {
        let (shadow, candidate, _) = native_v2_shadow_and_trials();
        let beginning_joint = carrier_joint(&shadow, None);
        let ending_joint = carrier_joint(&shadow, Some(&candidate));
        let ending = CoveredCarrierEphemeralCandidatesV1::try_new_with_soil_candidate(
            ending_joint.clone(),
            shadow,
            BTreeMap::new(),
            Some(candidate.clone()),
        )
        .expect("typed V2 ending");

        let selected_beginning = ending
            .try_with_selected_stage3_by_lane(beginning_joint.clone(), BTreeMap::new())
            .expect("resident beginning joint");
        assert_eq!(selected_beginning.joint(), &beginning_joint);
        assert_eq!(
            selected_beginning
                .soil_candidate
                .as_ref()
                .expect("retained V2 carry")
                .state_sha256(),
            candidate.state_sha256(),
        );

        let selected_ending = ending
            .try_with_selected_stage3_by_lane(ending_joint.clone(), BTreeMap::new())
            .expect("candidate ending joint");
        assert_eq!(selected_ending.joint(), &ending_joint);
        assert_eq!(
            selected_ending
                .soil_candidate
                .as_ref()
                .expect("selected V2 trial")
                .state_sha256(),
            candidate.state_sha256(),
        );
    }

    #[test]
    fn native_v2_selected_joint_rejects_stale_carry_and_substituted_owner() {
        let (shadow, candidate, stale_candidate) = native_v2_shadow_and_trials();
        let ending_joint = carrier_joint(&shadow, Some(&candidate));
        let ending = CoveredCarrierEphemeralCandidatesV1::try_new_with_soil_candidate(
            ending_joint.clone(),
            shadow.clone(),
            BTreeMap::new(),
            Some(candidate),
        )
        .expect("typed V2 ending");

        let mut stale = ending.clone();
        stale.soil_candidate = Some(stale_candidate);
        assert!(
            stale
                .try_with_selected_stage3_by_lane(ending_joint, BTreeMap::new())
                .is_err(),
            "a selected ending joint must reject a substituted exact carry",
        );

        let mut substituted_bytes = carrier_joint(&shadow, None).owner_bytes().clone();
        substituted_bytes
            .get_mut("hydrology")
            .expect("hydrology owner")
            .push(0xff);
        let substituted_joint = CoveredTerminalJointTrialStateV1::try_new(
            JointTrialAuthorityV1 {
                source_owner_set_sha256: Digest32::from_bytes([17; 32]),
                lane_id: 1,
                source_snow_owner_sha256: digest_bytes(
                    substituted_bytes.get("snow").expect("snow owner"),
                ),
                interval_index: 0,
                state_support: TimeSupport::new(
                    ModelTimeNs::new(60_000_000_000),
                    ModelTimeNs::new(120_000_000_000),
                )
                .expect("support"),
                accepted_predecessors: Vec::new(),
            },
            substituted_bytes,
        )
        .expect("sealed substituted joint");
        assert!(
            ending
                .try_with_selected_stage3_by_lane(substituted_joint, BTreeMap::new())
                .is_err(),
            "a selected joint must reject any substituted non-soil owner",
        );
    }

    #[test]
    fn phase_has_no_stage3_evaluation_or_publication_surface() {
        let source = include_str!("carrier_phase.rs");
        let implementation = source
            .split("#[cfg(test)]")
            .next()
            .expect("implementation source");
        for forbidden in [
            "evaluate_stage3_persistent_support",
            "evaluate_stage3_terminal_support",
            "accept_slab(",
            "finalize_v11_imported_segment",
            "last_support_receipt =",
            "last_final_boundary_receipts =",
            "last_wb14_",
            "self.ending =",
        ] {
            assert!(
                !implementation.contains(forbidden),
                "carrier phase reached forbidden publication/evaluation surface: {forbidden}"
            );
        }
    }

    #[test]
    fn v2_carrier_composition_is_trial_only_and_receipt_free() {
        let source = include_str!("carrier_phase.rs");
        let body = source
            .split("fn unpublished_v2_soil_trial(")
            .nth(1)
            .expect("V2 unpublished trial")
            .split("impl DirectV11SnowCoveredRealConsumerStack")
            .next()
            .expect("V2 unpublished trial body");
        assert!(body.contains("advance_soil_thermal_trial_v2("));
        for forbidden in [
            "apply_soil_thermal_energy_credit_v2(",
            "aggregate_soil_thermal_ending_v2(",
            "seal_soil_thermal_accepted_candidate_v2(",
            "install_soil_thermal_accepted_v2(",
        ] {
            assert!(
                !body.contains(forbidden),
                "unpublished V2 carrier emitted accepted custody: {forbidden}"
            );
        }
    }

    #[test]
    fn phase_receiver_is_immutable_and_trial_identity_is_exact() {
        let source = include_str!("carrier_phase.rs");
        let implementation = source
            .split("#[cfg(test)]")
            .next()
            .expect("implementation source");
        assert!(source.contains("execute_covered_carrier_phase_v1(\n        &self,"));
        assert!(source.contains("child.trial_support != request.support"));
        assert!(
            source.contains("child.beginning_joint_sha256 != beginning.joint.receipt_sha256()")
        );
        assert!(source.contains("forcing.duration_seconds.to_bits() != interval_s.to_bits()"));
        assert!(source.contains("boundary.snow_temperature_k = surface_temperature_k"));
        assert!(source.contains("boundary.latent_heat_j_kg = latent_heat_j_kg"));
        assert!(source.contains("One provider call is one joint carrier mapping"));
        assert!(source.contains("provisional: true"));
        assert!(source.contains("accept_envelope(envelope.transaction_id(), &envelope)"));
        assert!(!implementation.contains("let ending_candidates = beginning.clone()"));
    }

    #[test]
    fn batch_phase_enters_the_shared_engine_once_with_complete_lane_state() {
        let source = include_str!("carrier_phase.rs");
        let body = source
            .split("pub(crate) fn execute_covered_carrier_batch_phase_v2")
            .nth(1)
            .expect("batch carrier entry")
            .split("fn execute_shared_covered_carrier_engine_v1")
            .next()
            .expect("batch carrier body");
        assert_eq!(
            body.matches("self.execute_shared_covered_carrier_engine_v1(")
                .count(),
            1,
            "one batch candidate must advance the shared carrier once",
        );
        assert!(
            body.contains("BatchTerminalTrial {\n                lanes: request.lanes.clone(),")
        );

        let engine = source
            .split("fn execute_shared_covered_carrier_engine_v1")
            .nth(1)
            .expect("shared carrier engine");
        assert!(engine.contains("let lane_states = snow_boundary_state.lane_states(request);"));
        assert!(engine.contains("for (lane_id, lane) in &lane_states"));
        assert!(engine.contains("for lane_id in lane_states.keys()"));
        assert!(engine.contains("batch_terminal_snow_soil_trial_receipts_by_lane"));
    }

    #[test]
    fn terminal_trial_rebinds_common_temperature_and_latent_heat_together() {
        let ofe_id = OfeId::try_new("ofe-1").expect("OFE");
        let covered_key = (ofe_id.clone(), TileId::try_new("covered").expect("tile"));
        let open_key = (ofe_id.clone(), TileId::try_new("open").expect("tile"));
        let top_layer = SoilLayerId::try_new("soil-1").expect("soil layer");
        let bindings = vec![crate::direct_runtime::DirectSurfaceLiquidOfeBinding {
            ofe_id,
            production_lane_index: 0,
            production_lane_id: 1,
            ordered_soil_layer_ids: vec![top_layer.clone()],
            infiltration_soil_thermal_layer_id: top_layer,
        }];
        let mut covered_boundaries =
            BTreeMap::from([(covered_key.clone(), test_boundary(2_500_000.0))]);
        let mut open_boundaries = BTreeMap::from([(open_key.clone(), test_boundary(2_900_000.0))]);
        let temperature_c = -12.345_678_9;
        CoveredSnowBoundaryStateV1::TerminalTrial {
            lane_id: 1,
            ice_kg_m2: 0.25,
            liquid_kg_m2: 0.0,
            cold_content_j_m2: 6_481.481_422_5,
            surface_temperature_k: temperature_c + 273.15,
            depth_m: 0.0025,
            density_kg_m3: 100.0,
        }
        .apply_to_boundary_sets(&bindings, &mut covered_boundaries, &mut open_boundaries)
        .expect("terminal trial boundary rebind");

        let covered = covered_boundaries
            .get(&covered_key)
            .expect("covered boundary");
        let open = open_boundaries.get(&open_key).expect("open boundary");
        let expected_temperature_k = temperature_c + 273.15;
        let expected_latent =
            openwepp_meteorology::surface_energy::latent_heat_for_surface_temperature(
                TemperatureCelsius::try_new(temperature_c).expect("temperature"),
            )
            .expect("latent heat")
            .as_joules_per_kilogram();
        assert_eq!(
            covered.snow_temperature_k.to_bits(),
            expected_temperature_k.to_bits()
        );
        assert_eq!(
            open.snow_temperature_k.to_bits(),
            expected_temperature_k.to_bits()
        );
        assert_eq!(
            covered.latent_heat_j_kg.to_bits(),
            expected_latent.to_bits()
        );
        assert_eq!(open.latent_heat_j_kg.to_bits(), expected_latent.to_bits());
    }
}
