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
    static COVERED_CARRIER_ENDPOINT_OWNER_PROJECTION_AUDIT_V1: std::cell::Cell<Option<u32>> = const { std::cell::Cell::new(None) };
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

#[cfg(test)]
pub(crate) fn begin_covered_carrier_endpoint_owner_projection_audit_v1() {
    COVERED_CARRIER_ENDPOINT_OWNER_PROJECTION_AUDIT_V1.with(|audit| audit.set(Some(0)));
}

#[cfg(test)]
pub(crate) fn take_covered_carrier_endpoint_owner_projection_audit_v1() -> u32 {
    COVERED_CARRIER_ENDPOINT_OWNER_PROJECTION_AUDIT_V1
        .with(|audit| audit.take().unwrap_or_default())
}

#[cfg(test)]
fn audit_covered_carrier_endpoint_owner_projection_v1() {
    COVERED_CARRIER_ENDPOINT_OWNER_PROJECTION_AUDIT_V1.with(|audit| {
        if let Some(count) = audit.get() {
            audit.set(Some(count.saturating_add(1)));
        }
    });
}

#[cfg(not(test))]
fn audit_covered_carrier_endpoint_owner_projection_v1() {}

include!("carrier_phase/snow_boundary.rs");

/// Typed companions for opaque canonical joint-owner bytes.
///
/// The probe therefore retains the typed, unpublished candidates beside their
/// canonical joint identity and validates the pair before every trial.
#[derive(Clone)]
pub(crate) struct CoveredCarrierEphemeralCandidatesV1 {
    joint: CoveredTerminalJointTrialStateV1,
    shadow: DirectV10RealConsumerShadow,
    stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    soil_candidate: Option<DirectSoilThermalCandidate>,
    soil_continuation: Option<DirectSoilThermalUnpublishedContinuationResultV2>,
    validated_owner_bytes: std::sync::Arc<ValidatedCoveredCarrierOwnerBytesV1>,
    terminal_snow_soil_trial_receipt:
        Option<physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1>,
}

struct ValidatedCoveredCarrierOwnerBytesV1 {
    resident: BTreeMap<String, Vec<u8>>,
    candidate: BTreeMap<String, Vec<u8>>,
    soil_candidate_seal: CoveredCarrierSoilCandidateSealV1,
}

enum CoveredCarrierSoilCandidateSealV1 {
    None,
    V1(SoilThermalSnapshot),
    V2(openwepp_land_surface_energy::Sha256Digest),
}

impl CoveredCarrierSoilCandidateSealV1 {
    fn from_candidate(candidate: Option<&DirectSoilThermalCandidate>) -> Self {
        match candidate {
            None => Self::None,
            Some(DirectSoilThermalCandidate::V1(snapshot)) => Self::V1(snapshot.clone()),
            Some(DirectSoilThermalCandidate::V2(trial)) => {
                Self::V2(trial.unpublished_trial_sha256().clone())
            }
        }
    }

    fn matches(&self, candidate: Option<&DirectSoilThermalCandidate>) -> bool {
        match (self, candidate) {
            (Self::None, None) => true,
            (Self::V1(sealed), Some(DirectSoilThermalCandidate::V1(actual))) => sealed == actual,
            (Self::V2(sealed), Some(DirectSoilThermalCandidate::V2(actual))) => {
                sealed == actual.unpublished_trial_sha256()
            }
            _ => false,
        }
    }
}

/// Borrowed proof that the exact immutable carrier soil candidate still
/// matches the seal minted after its constructor validation. The borrow keeps
/// that candidate immutable until its read view has been consumed by the
/// immediately adjacent physical preparation.
pub(crate) struct ValidatedCarrierSoilReadV1<'a> {
    candidate: &'a DirectSoilThermalCandidate,
    read_view: DirectSoilThermalReadView<'a>,
}

impl ValidatedCarrierSoilReadV1<'_> {
    pub(crate) fn read_view_for(
        &self,
        candidate: Option<&DirectSoilThermalCandidate>,
    ) -> Result<DirectSoilThermalReadView<'_>, DirectV9RealConsumerError> {
        let candidate = candidate.ok_or(DirectV9RealConsumerError::OwnerClosure(
            "validated carrier soil read without candidate",
        ))?;
        if !std::ptr::eq(self.candidate, candidate) {
            return Err(DirectV9RealConsumerError::OwnerClosure(
                "validated carrier soil read candidate identity",
            ));
        }
        Ok(self.read_view)
    }
}

/// Move-only owner projection for one completed covered-map endpoint.
///
/// The candidate and its canonical bytes enter this token together and cannot
/// be separated or reused across a mutation/restart boundary.  Consumption
/// validates the exact joint and soil custody before minting the trusted
/// owner-byte proof retained by `CoveredCarrierEphemeralCandidatesV1`.
struct CoveredCarrierEndpointOwnerProjectionV1 {
    shadow: DirectV10RealConsumerShadow,
    resident_owner_bytes: BTreeMap<String, Vec<u8>>,
}

/// Borrowed proof for the exact normalized V10 vegetation owner used by one
/// covered-carrier endpoint. Private fields prevent construction outside this
/// module; the borrow prevents mutation until canonical owner bytes exist.
pub(crate) struct ValidatedCarrierVegetationOwnerV10V1<'a> {
    state: &'a openwepp_vegetation::V10CoupledOwnedState,
    configuration: &'a VegetationConfiguration,
}

impl ValidatedCarrierVegetationOwnerV10V1<'_> {
    pub(crate) fn matches(
        &self,
        state: &openwepp_vegetation::V10CoupledOwnedState,
        configuration: &VegetationConfiguration,
    ) -> bool {
        std::ptr::eq(self.state, state) && std::ptr::eq(self.configuration, configuration)
    }
}

fn validated_normalized_carrier_vegetation_v10_v1(
    shadow: &DirectV10RealConsumerShadow,
    expected_parent_transaction: u128,
) -> Result<ValidatedCarrierVegetationOwnerV10V1<'_>, DirectV11RealConsumerError> {
    let state = &shadow.vegetation_state;
    let configuration = &shadow.vegetation_configuration;
    let expected_accepted_transaction =
        (expected_parent_transaction != 0).then_some(expected_parent_transaction);
    if configuration.model_definition_sha256 != openwepp_vegetation::V10_MODEL_SHA256
        || state.0.model_definition_sha256 != openwepp_vegetation::V10_MODEL_SHA256
        || state.0.configuration_sha256 != configuration.configuration_sha256
        || state.0.last_transaction_id != expected_parent_transaction
        || state
            .0
            .strata
            .values()
            .any(|stratum| stratum.last_transaction_id != expected_parent_transaction)
        || state.0.occupancies.values().any(|occupancy| {
            occupancy.last_accepted_transaction_id != expected_accepted_transaction
        })
        || state.0.state_sha256 != state.0.canonical_sha256()
        || (expected_parent_transaction == 0
            && configuration.initial_state_sha256 != state.0.state_sha256)
    {
        return Err(DirectV11RealConsumerError::Identity(
            "normalized carrier V10 vegetation owner proof",
        ));
    }
    Ok(ValidatedCarrierVegetationOwnerV10V1 {
        state,
        configuration,
    })
}

impl CoveredCarrierEndpointOwnerProjectionV1 {
    fn from_validated_owner_bytes(
        shadow: DirectV10RealConsumerShadow,
        resident_owner_bytes: BTreeMap<String, Vec<u8>>,
    ) -> Self {
        audit_covered_carrier_endpoint_owner_projection_v1();
        Self {
            shadow,
            resident_owner_bytes,
        }
    }

    fn joint_owner_bytes_with_snow_v1(&self, snow: Vec<u8>) -> BTreeMap<String, Vec<u8>> {
        let mut owner_bytes = self.resident_owner_bytes.clone();
        owner_bytes.insert("snow".to_owned(), snow);
        owner_bytes
    }

    fn try_into_ephemeral_candidates_v1(
        self,
        joint: CoveredTerminalJointTrialStateV1,
        stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
        validated_soil_ending: ValidatedCoveredCarrierSoilEndingV1,
    ) -> Result<
        (
            CoveredCarrierEphemeralCandidatesV1,
            DirectSoilThermalCandidate,
            Option<DirectSoilThermalUnpublishedContinuationResultV2>,
        ),
        DirectV11RealConsumerError,
    > {
        let custody_profile = CarrierProfileScopeV1::begin("carrier owner soil custody");
        let (soil_candidate, soil_continuation) = validated_soil_ending.into_parts();
        drop(custody_profile);
        let candidate_bytes_profile = CarrierProfileScopeV1::begin("carrier owner candidate bytes");
        let candidate_owner_bytes = self.resident_owner_bytes.clone();
        drop(candidate_bytes_profile);
        let assembly_profile = CarrierProfileScopeV1::begin("carrier owner ephemeral assembly");
        validate_covered_carrier_typed_joint_v1(
            &joint,
            &candidate_owner_bytes,
            CoveredCarrierTypedJointPostureV1::CandidateEnding,
        )?;
        let ending_soil_candidate = matches!(&soil_candidate, DirectSoilThermalCandidate::V2(_))
            .then(|| soil_candidate.clone());
        let ending_soil_continuation = soil_continuation.clone();
        let validated_owner_bytes = std::sync::Arc::new(ValidatedCoveredCarrierOwnerBytesV1 {
            resident: self.resident_owner_bytes,
            candidate: candidate_owner_bytes,
            soil_candidate_seal: CoveredCarrierSoilCandidateSealV1::from_candidate(
                ending_soil_candidate.as_ref(),
            ),
        });
        let value = CoveredCarrierEphemeralCandidatesV1 {
            joint,
            shadow: self.shadow,
            stage3_by_lane,
            soil_candidate: ending_soil_candidate,
            soil_continuation: ending_soil_continuation,
            validated_owner_bytes,
            terminal_snow_soil_trial_receipt: None,
        };
        drop(assembly_profile);
        Ok((value, soil_candidate, soil_continuation))
    }
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
            None,
            CoveredCarrierTypedJointPostureV1::ResidentBeginning,
        )
    }

    pub(crate) fn try_new_with_deferred_native_v2_soil_custody(
        joint: CoveredTerminalJointTrialStateV1,
        shadow: DirectV10RealConsumerShadow,
        stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
        custody: &DeferredNativeV2SoilCustodyV1,
    ) -> Result<Self, DirectV11RealConsumerError> {
        let authenticated = DeferredNativeV2SoilCustodyV1::try_new(
            &shadow,
            custody.candidate().clone(),
            custody.continuation().cloned(),
        )?;
        if &authenticated != custody {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier deferred native V2 soil custody",
            ));
        }
        Self::try_new_with_joint_posture(
            joint,
            shadow,
            stage3_by_lane,
            Some(authenticated.candidate().clone()),
            authenticated.continuation().cloned(),
            CoveredCarrierTypedJointPostureV1::CandidateEnding,
        )
    }

    #[cfg(test)]
    fn try_new_with_soil_candidate(
        joint: CoveredTerminalJointTrialStateV1,
        shadow: DirectV10RealConsumerShadow,
        stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
        soil_candidate: Option<DirectSoilThermalCandidate>,
        soil_continuation: Option<DirectSoilThermalUnpublishedContinuationResultV2>,
    ) -> Result<Self, DirectV11RealConsumerError> {
        Self::try_new_with_joint_posture(
            joint,
            shadow,
            stage3_by_lane,
            soil_candidate,
            soil_continuation,
            CoveredCarrierTypedJointPostureV1::CandidateEnding,
        )
    }

    fn try_new_with_joint_posture(
        joint: CoveredTerminalJointTrialStateV1,
        shadow: DirectV10RealConsumerShadow,
        stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
        soil_candidate: Option<DirectSoilThermalCandidate>,
        soil_continuation: Option<DirectSoilThermalUnpublishedContinuationResultV2>,
        posture: CoveredCarrierTypedJointPostureV1,
    ) -> Result<Self, DirectV11RealConsumerError> {
        let resident_owner_bytes = covered_carrier_typed_owner_bytes_v1(
            &shadow,
            soil_candidate.as_ref(),
            CoveredCarrierTypedJointPostureV1::ResidentBeginning,
        )?;
        let candidate_owner_bytes = covered_carrier_candidate_owner_bytes_from_resident_v1(
            &resident_owner_bytes,
            soil_candidate.as_ref(),
        )?;
        let actual = match posture {
            CoveredCarrierTypedJointPostureV1::ResidentBeginning => &resident_owner_bytes,
            CoveredCarrierTypedJointPostureV1::CandidateEnding => &candidate_owner_bytes,
        };
        validate_covered_carrier_typed_joint_v1(&joint, &actual, posture)?;
        let validated_owner_bytes = std::sync::Arc::new(ValidatedCoveredCarrierOwnerBytesV1 {
            resident: resident_owner_bytes,
            candidate: candidate_owner_bytes,
            soil_candidate_seal: CoveredCarrierSoilCandidateSealV1::from_candidate(
                soil_candidate.as_ref(),
            ),
        });
        Ok(Self {
            joint,
            shadow,
            stage3_by_lane,
            soil_candidate,
            soil_continuation,
            validated_owner_bytes,
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

    pub(crate) const fn soil_continuation(
        &self,
    ) -> Option<&DirectSoilThermalUnpublishedContinuationResultV2> {
        self.soil_continuation.as_ref()
    }

    fn validated_soil_read_v1(
        &self,
    ) -> Result<Option<ValidatedCarrierSoilReadV1<'_>>, DirectV11RealConsumerError> {
        if !self
            .validated_owner_bytes
            .soil_candidate_seal
            .matches(self.soil_candidate.as_ref())
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier soil read candidate seal",
            ));
        }
        Ok(self
            .soil_candidate
            .as_ref()
            .map(|candidate| ValidatedCarrierSoilReadV1 {
                candidate,
                read_view: candidate.read_view(),
            }))
    }

    pub(crate) fn try_with_selected_stage3_by_lane(
        &self,
        joint: CoveredTerminalJointTrialStateV1,
        stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
    ) -> Result<Self, DirectV11RealConsumerError> {
        if !self
            .validated_owner_bytes
            .soil_candidate_seal
            .matches(self.soil_candidate.as_ref())
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier selected soil candidate seal",
            ));
        }
        let beginning = &self.validated_owner_bytes.resident;
        let ending = &self.validated_owner_bytes.candidate;
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
        validate_covered_carrier_typed_joint_v1(
            &joint,
            match posture {
                CoveredCarrierTypedJointPostureV1::ResidentBeginning => beginning,
                CoveredCarrierTypedJointPostureV1::CandidateEnding => ending,
            },
            posture,
        )?;
        Ok(Self {
            joint,
            shadow: self.shadow.clone(),
            stage3_by_lane,
            soil_candidate: self.soil_candidate.clone(),
            soil_continuation: self.soil_continuation.clone(),
            validated_owner_bytes: std::sync::Arc::clone(&self.validated_owner_bytes),
            terminal_snow_soil_trial_receipt: self.terminal_snow_soil_trial_receipt.clone(),
        })
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
    let resident = shadow.canonical_owner_state_bytes()?;
    match posture {
        CoveredCarrierTypedJointPostureV1::ResidentBeginning => Ok(resident),
        CoveredCarrierTypedJointPostureV1::CandidateEnding => {
            covered_carrier_candidate_owner_bytes_from_resident_v1(&resident, soil_candidate)
        }
    }
}

fn covered_carrier_candidate_owner_bytes_from_resident_v1(
    resident: &BTreeMap<String, Vec<u8>>,
    soil_candidate: Option<&DirectSoilThermalCandidate>,
) -> Result<BTreeMap<String, Vec<u8>>, DirectV11RealConsumerError> {
    // A native V2 trial ending is a typed unpublished continuation, not a
    // complete resource owner. Its identity is retained by `soil_candidate`
    // and the sealed snow/soil trial receipt; projecting `ending_state()`
    // here would leak a bare owner envelope without its authenticated
    // resident custody into the V11 complete-owner join. Until final
    // acceptance consumes the continuation, the joint therefore retains the
    // exact resident soil owner bytes.
    if let Some(candidate) = soil_candidate {
        candidate.read_view().validate().map_err(|_| {
            DirectV11RealConsumerError::Identity("covered carrier typed soil candidate")
        })?;
    }
    Ok(resident.clone())
}

pub(crate) fn covered_carrier_initial_owner_bytes_with_deferred_native_v2_soil_custody_v1(
    shadow: &DirectV10RealConsumerShadow,
    custody: Option<&DeferredNativeV2SoilCustodyV1>,
) -> Result<BTreeMap<String, Vec<u8>>, DirectV11RealConsumerError> {
    let Some(custody) = custody else {
        return covered_carrier_typed_owner_bytes_v1(
            shadow,
            None,
            CoveredCarrierTypedJointPostureV1::ResidentBeginning,
        );
    };
    let authenticated = DeferredNativeV2SoilCustodyV1::try_new(
        shadow,
        custody.candidate().clone(),
        custody.continuation().cloned(),
    )?;
    if &authenticated != custody {
        return Err(DirectV11RealConsumerError::Identity(
            "covered carrier initial deferred native V2 soil custody",
        ));
    }
    covered_carrier_typed_owner_bytes_v1(
        shadow,
        Some(authenticated.candidate()),
        CoveredCarrierTypedJointPostureV1::CandidateEnding,
    )
}

fn validate_covered_stack_deferred_native_v2_soil_beginning_v1(
    stack: &DirectV11SnowCoveredRealConsumerStack<'_>,
    beginning: &CoveredCarrierEphemeralCandidatesV1,
) -> Result<(), DirectV11RealConsumerError> {
    let Some(custody) = stack.deferred_native_v2_soil_custody.as_ref() else {
        return Ok(());
    };
    let authenticated = DeferredNativeV2SoilCustodyV1::try_new(
        &beginning.shadow,
        custody.candidate().clone(),
        custody.continuation().cloned(),
    )?;
    if &authenticated != custody {
        return Err(DirectV11RealConsumerError::Identity(
            "covered stack deferred native V2 soil beginning",
        ));
    }
    Ok(())
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

/// Private physical endpoint for a nonfinal canonical covered map.  It owns
/// only values required by the Stage-3 evaluation and convergence read view;
/// it cannot be converted into a complete owner envelope or publication.
struct CoveredCarrierPhysicalPhaseResultV1 {
    physical: crate::land_surface_energy_shadow::ProvisionalCoveredV8PhysicalEvaluationV1,
    projected_vegetation: (VegetationConfiguration, V8CoupledOwnedState),
    native_finalization_posture: CoveredNativeFinalizationPostureV1,
    precipitation_sets: BTreeMap<u32, Stage3PrecipitationPhaseParcelSetV1>,
    complete_lower_boundaries: BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    covered_lse_states: BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
    validated_soil_ending: ValidatedCoveredCarrierSoilEndingV1,
    batch_boundaries_by_lane: BTreeMap<u32, Stage3SnowSurfaceBoundaryReceiptV1>,
    carrier_source_receipts: BTreeMap<(OfeId, TileId), CoveredCarrierInitialGuessV1>,
    open_snow_candidates: BTreeMap<(OfeId, TileId), OpenSnowTileBoundaryCandidateV1>,
    terminal_soil_trials:
        BTreeMap<u32, physical_outcome_ledger::TerminalSnowBottomSoilTrialResultV1>,
    terminal_soil_credits: BTreeMap<u32, SoilThermalTopBoundaryCreditV1>,
    surface_custody: CoveredPhysicalSurfaceCustodyV1,
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
    validated_soil_ending: ValidatedCoveredCarrierSoilEndingV1,
}

/// Exact unpublished soil ending authenticated by the physical producer.
/// This token is move-only and has no serialized form, so the complete
/// endpoint cannot separate or substitute its candidate and continuation.
struct ValidatedCoveredCarrierSoilEndingV1 {
    candidate: DirectSoilThermalCandidate,
    continuation: Option<DirectSoilThermalUnpublishedContinuationResultV2>,
}

impl ValidatedCoveredCarrierSoilEndingV1 {
    fn after_physical_custody_validation(
        candidate: DirectSoilThermalCandidate,
        continuation: Option<DirectSoilThermalUnpublishedContinuationResultV2>,
    ) -> Self {
        Self {
            candidate,
            continuation,
        }
    }

    const fn candidate(&self) -> &DirectSoilThermalCandidate {
        &self.candidate
    }

    #[cfg(test)]
    fn continuation(&self) -> Option<&DirectSoilThermalUnpublishedContinuationResultV2> {
        self.continuation.as_ref()
    }

    fn into_parts(
        self,
    ) -> (
        DirectSoilThermalCandidate,
        Option<DirectSoilThermalUnpublishedContinuationResultV2>,
    ) {
        (self.candidate, self.continuation)
    }
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

#[derive(Clone, Copy)]
struct CoveredCarrierSoilBeginningIdentityV1<'a> {
    read_view: crate::v9_real_consumer_shadow::DirectSoilThermalReadView<'a>,
    owner_id: &'a ResourceOwnerId,
    configuration_sha256: &'a Sha256Digest,
    state_sha256: &'a Sha256Digest,
}

fn covered_carrier_soil_beginning_identity_v1(
    beginning: &CoveredCarrierEphemeralCandidatesV1,
) -> Result<CoveredCarrierSoilBeginningIdentityV1<'_>, DirectV11RealConsumerError> {
    if let Some(candidate) = beginning.soil_candidate.as_ref() {
        let ending = candidate
            .v2()
            .map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "covered carrier successor soil candidate posture",
                )
            })?
            .ending_state();
        return Ok(CoveredCarrierSoilBeginningIdentityV1 {
            read_view: candidate.read_view(),
            owner_id: &ending.owner_id,
            configuration_sha256: &ending.configuration_sha256,
            state_sha256: &ending.state_sha256,
        });
    }
    let resident = &beginning.shadow.inner.soil_thermal;
    Ok(CoveredCarrierSoilBeginningIdentityV1 {
        read_view: resident.read_view(),
        owner_id: resident.owner_id(),
        configuration_sha256: resident.configuration_sha256(),
        state_sha256: resident.state_sha256(),
    })
}

pub(crate) fn stage_unpublished_v2_carrier_owners(
    candidate: &mut DirectV10RealConsumerShadow,
    envelope: &UncommittedCoveredV8OwnerEnvelope,
) -> Result<(), DirectV11RealConsumerError> {
    let authenticated_lse_beginning = candidate.inner.lse_state.clone();
    stage_unpublished_v2_carrier_owner_values_v1(
        candidate,
        &authenticated_lse_beginning,
        envelope,
        true,
    )
}

pub(crate) fn restage_unpublished_v2_carrier_owners_without_acceptance_count_v1(
    candidate: &mut DirectV10RealConsumerShadow,
    authenticated_lse_beginning: &openwepp_land_surface_energy::LandSurfaceEnergyState,
    envelope: &UncommittedCoveredV8OwnerEnvelope,
) -> Result<(), DirectV11RealConsumerError> {
    stage_unpublished_v2_carrier_owner_values_v1(
        candidate,
        authenticated_lse_beginning,
        envelope,
        false,
    )
}

fn stage_unpublished_v2_carrier_owner_values_v1(
    candidate: &mut DirectV10RealConsumerShadow,
    authenticated_lse_beginning: &openwepp_land_surface_energy::LandSurfaceEnergyState,
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    increment_accepted_interval_count: bool,
) -> Result<(), DirectV11RealConsumerError> {
    envelope.validate().map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
    })?;
    candidate.inner.authority = CoveredColumnAuthority::V11SnowCovered;
    candidate.inner.vegetation_state = project_v8_runtime_to_v9(
        envelope.vegetation().ending_state(),
        &candidate.inner.vegetation_configuration,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
    })?;
    candidate.inner.lse_state = build_lse_ending_state(
        authenticated_lse_beginning,
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
    match (
        candidate.frozen_litter_v3_resident().is_some(),
        candidate.frozen_litter_v4_resident().is_some(),
    ) {
        // A represented-snow V11 interval executes current surface ingress
        // through the active inner V1 owner. Keep the installed native V3/V4
        // successor's nested WB14 arithmetic synchronized with that accepted
        // owner so a later snow-free retry cannot revive a stale parent.
        (true, true) => candidate
            .stage_frozen_litter_wb14_parent_from_inner_v1()
            .map_err(DirectV11RealConsumerError::Runtime)?,
        (false, false) => {}
        _ => {
            return Err(DirectV11RealConsumerError::Identity(
                "unpublished V2 carrier half-native frozen-litter posture",
            ));
        }
    }
    if increment_accepted_interval_count
        && envelope
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UnpublishedV2SoilTemporalPostureV1 {
    SameSupportReplay,
    ContiguousSuccessor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UnpublishedV2SoilBeginningSourceV1 {
    AuthenticatedResident,
    RawContiguousCandidate,
    DeferredPriorChildCustody,
}

fn unpublished_v2_soil_temporal_posture_v1(
    retained_start_ns: u128,
    retained_end_ns: u128,
    requested_start_ns: u128,
    requested_end_ns: u128,
) -> Result<UnpublishedV2SoilTemporalPostureV1, DirectV11RealConsumerError> {
    if retained_start_ns == requested_start_ns && retained_end_ns == requested_end_ns {
        Ok(UnpublishedV2SoilTemporalPostureV1::SameSupportReplay)
    } else if retained_end_ns == requested_start_ns && requested_start_ns < requested_end_ns {
        Ok(UnpublishedV2SoilTemporalPostureV1::ContiguousSuccessor)
    } else {
        Err(DirectV11RealConsumerError::Identity(
            "unpublished V2 soil temporal overlap or gap",
        ))
    }
}

fn unpublished_v2_soil_beginning_source_v1(
    raw_candidate_support: Option<(u128, u128)>,
    deferred_custody_support: Option<(u128, u128)>,
    requested_start_ns: u128,
    requested_end_ns: u128,
) -> Result<UnpublishedV2SoilBeginningSourceV1, DirectV11RealConsumerError> {
    let Some((raw_start_ns, raw_end_ns)) = raw_candidate_support else {
        return Ok(UnpublishedV2SoilBeginningSourceV1::AuthenticatedResident);
    };
    match unpublished_v2_soil_temporal_posture_v1(
        raw_start_ns,
        raw_end_ns,
        requested_start_ns,
        requested_end_ns,
    )? {
        UnpublishedV2SoilTemporalPostureV1::ContiguousSuccessor => {
            Ok(UnpublishedV2SoilBeginningSourceV1::RawContiguousCandidate)
        }
        UnpublishedV2SoilTemporalPostureV1::SameSupportReplay => {
            let Some((custody_start_ns, custody_end_ns)) = deferred_custody_support else {
                return Ok(UnpublishedV2SoilBeginningSourceV1::AuthenticatedResident);
            };
            if unpublished_v2_soil_temporal_posture_v1(
                custody_start_ns,
                custody_end_ns,
                requested_start_ns,
                requested_end_ns,
            )? != UnpublishedV2SoilTemporalPostureV1::ContiguousSuccessor
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "unpublished V2 deferred soil beginning chronology",
                ));
            }
            Ok(UnpublishedV2SoilBeginningSourceV1::DeferredPriorChildCustody)
        }
    }
}

fn unpublished_v2_original_support_start_ns_v1(
    _current_resident_support_start_ns: u128,
    retained_trial_support_start_ns: u128,
    retained_continuation_original_support_start_ns: Option<u128>,
) -> u128 {
    retained_continuation_original_support_start_ns.unwrap_or(retained_trial_support_start_ns)
}

fn unpublished_v2_soil_trial(
    beginning: &CoveredCarrierEphemeralCandidatesV1,
    immutable_deferred_native_v2_soil_custody: Option<&DeferredNativeV2SoilCustodyV1>,
    transaction_authority:
        crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2,
    hydrology: &crate::land_surface_energy_shadow::UnifiedRealHydrologyCandidate,
    support: TimeSupport,
    credits: &[SoilThermalTopBoundaryCreditV1],
) -> Result<
    (
        DirectSoilThermalCandidate,
        Option<DirectSoilThermalUnpublishedContinuationResultV2>,
    ),
    DirectV11RealConsumerError,
> {
    let source_owner_id = ResourceOwnerId::try_new("snow").map_err(|_| {
        DirectV11RealConsumerError::Identity("unpublished V2 terminal soil source owner")
    })?;
    let raw_candidate_support = beginning
        .soil_candidate
        .as_ref()
        .map(|candidate| {
            candidate
                .v2()
                .map(|trial| (trial.support_start_ns(), trial.support_end_ns()))
        })
        .transpose()
        .map_err(|_| {
            DirectV11RealConsumerError::Identity(
                "unpublished V2 soil continuation candidate posture",
            )
        })?;
    let deferred_custody_support = immutable_deferred_native_v2_soil_custody
        .map(DeferredNativeV2SoilCustodyV1::candidate)
        .map(|candidate| {
            candidate
                .v2()
                .map(|trial| (trial.support_start_ns(), trial.support_end_ns()))
        })
        .transpose()
        .map_err(|_| {
            DirectV11RealConsumerError::Identity("unpublished V2 deferred soil candidate posture")
        })?;
    let beginning_source = unpublished_v2_soil_beginning_source_v1(
        raw_candidate_support,
        deferred_custody_support,
        support.start_ns().get(),
        support.end_ns().get(),
    )?;
    let (retained_candidate, retained_continuation) = match beginning_source {
        UnpublishedV2SoilBeginningSourceV1::AuthenticatedResident => (None, None),
        UnpublishedV2SoilBeginningSourceV1::RawContiguousCandidate => (
            beginning.soil_candidate.as_ref(),
            beginning.soil_continuation.as_ref(),
        ),
        UnpublishedV2SoilBeginningSourceV1::DeferredPriorChildCustody => (
            immutable_deferred_native_v2_soil_custody.map(DeferredNativeV2SoilCustodyV1::candidate),
            immutable_deferred_native_v2_soil_custody
                .and_then(DeferredNativeV2SoilCustodyV1::continuation),
        ),
    };
    if let Some(candidate) = retained_candidate {
        let retained_trial = candidate.v2().map_err(|_| {
            DirectV11RealConsumerError::Identity("unpublished V2 immutable soil beginning posture")
        })?;
        if unpublished_v2_soil_temporal_posture_v1(
            retained_trial.support_start_ns(),
            retained_trial.support_end_ns(),
            support.start_ns().get(),
            support.end_ns().get(),
        )? != UnpublishedV2SoilTemporalPostureV1::ContiguousSuccessor
        {
            return Err(DirectV11RealConsumerError::Identity(
                "unpublished V2 immutable soil beginning chronology",
            ));
        }
        let active_owner = beginning
            .shadow
            .soil_thermal_v2()
            .map_err(DirectV11RealConsumerError::Runtime)?
            .owner();
        let original_support_start_ns = unpublished_v2_original_support_start_ns_v1(
            active_owner.support_start_ns,
            retained_trial.support_start_ns(),
            retained_continuation.map(|continuation| {
                continuation
                    .original_prepared()
                    .beginning_owner()
                    .support_start_ns
            }),
        );
        let original_prepared = beginning
            .shadow
            .prepare_next_soil_thermal_support_v2(original_support_start_ns, support.end_ns().get())
            .map_err(DirectV11RealConsumerError::Runtime)?;
        let continuation_result = if let Some(prior) = retained_continuation {
            beginning
                .shadow
                .prepare_next_soil_thermal_unpublished_continuation_v2(
                    &original_prepared,
                    prior,
                    candidate.state_sha256(),
                    support.start_ns().get(),
                    support.end_ns().get(),
                )
        } else {
            beginning
                .shadow
                .prepare_soil_thermal_base_unpublished_continuation_v2(
                    &original_prepared,
                    retained_trial,
                    candidate.state_sha256(),
                    support.start_ns().get(),
                    support.end_ns().get(),
                )
        };
        let continuation = continuation_result.map_err(DirectV11RealConsumerError::Runtime)?;
        let mut child_operands =
            crate::land_surface_energy_shadow::physical_soil_energy_operands_v2(
                transaction_authority,
                support.start_ns().get(),
                support.end_ns().get(),
                &beginning.shadow.inner.lse_configuration.owner_id,
                &beginning.shadow.inner.surface_configuration.owner_id,
                hydrology.pre_ingress_soil_thermal_candidates(),
                hydrology.surface_ingress(),
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                    DirectV9RealConsumerError::LandSurfaceShadow(error),
                ))
            })?;
        child_operands.extend(
            continuation
                .child_top_boundary_operands_v2(credits, &source_owner_id)
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })?,
        );
        let result = beginning
            .shadow
            .advance_soil_thermal_unpublished_continuation_v2(&continuation, &child_operands)
            .map_err(DirectV11RealConsumerError::Runtime)?;
        let physical = result.clone().into_physical_candidate().map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
        })?;
        return Ok((physical, Some(result)));
    }

    let prepared = beginning
        .shadow
        .prepare_next_soil_thermal_support_v2(support.start_ns().get(), support.end_ns().get())
        .map_err(DirectV11RealConsumerError::Runtime)?;
    let mut operands = Vec::new();
    operands.extend(
        crate::land_surface_energy_shadow::physical_soil_energy_operands_v2(
            transaction_authority,
            support.start_ns().get(),
            support.end_ns().get(),
            &beginning.shadow.inner.lse_configuration.owner_id,
            &beginning.shadow.inner.surface_configuration.owner_id,
            hydrology.pre_ingress_soil_thermal_candidates(),
            hydrology.surface_ingress(),
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
    let continuation = beginning
        .shadow
        .authenticate_soil_thermal_base_unpublished_result_v2(
            &prepared,
            &trial,
            expected.accepted_operands(),
        )
        .map_err(DirectV11RealConsumerError::Runtime)?;
    let physical = continuation
        .clone()
        .into_physical_candidate()
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
        })?;
    Ok((physical, Some(continuation)))
}

struct CarrierProfileScopeV1 {
    phase: &'static str,
    started: Option<std::time::Instant>,
}

impl CarrierProfileScopeV1 {
    fn begin(phase: &'static str) -> Self {
        Self {
            phase,
            started: crate::snow_stage3_v11_attachment::begin_adaptive_parent_fixed_point_phase_v1(
            ),
        }
    }
}

impl Drop for CarrierProfileScopeV1 {
    fn drop(&mut self) {
        crate::snow_stage3_v11_attachment::record_adaptive_parent_profile_detail_v1(
            self.phase,
            self.started.take(),
        );
    }
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
        let physical_started =
            crate::snow_stage3_v11_attachment::begin_adaptive_parent_fixed_point_phase_v1();
        validate_covered_stack_deferred_native_v2_soil_beginning_v1(self, beginning)?;
        let physical = self.execute_shared_covered_carrier_physical_phase_v1(
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
            child.clone(),
        );
        crate::snow_stage3_v11_attachment::record_adaptive_parent_profile_detail_v1(
            "carrier physical phase",
            physical_started,
        );
        let physical = physical?;
        let complete_started =
            crate::snow_stage3_v11_attachment::begin_adaptive_parent_fixed_point_phase_v1();
        let result =
            self.complete_covered_carrier_physical_result_v1(beginning, request, child, physical);
        crate::snow_stage3_v11_attachment::record_adaptive_parent_profile_detail_v1(
            "carrier complete phase",
            complete_started,
        );
        result
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
        validate_covered_stack_deferred_native_v2_soil_beginning_v1(self, beginning)?;
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
        let physical = self.execute_shared_covered_carrier_physical_phase_v1(
            beginning,
            &leader_request,
            CoveredSnowBoundaryStateV1::BatchTerminalTrial {
                lanes: request.lanes.clone(),
            },
            child.clone(),
        )?;
        self.complete_covered_carrier_physical_result_v1(
            beginning,
            &leader_request,
            child,
            physical,
        )
    }

    /// Evaluate exactly the physical prefix used by a canonical covered map.
    /// This function has no complete-envelope return type and performs no
    /// vegetation-persistent, material, BGC, joint-owner, restart, or
    /// publication construction.
    fn execute_covered_carrier_physical_phase_v1(
        &self,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        request: &CoveredTerminalBatchTrialRequestV2,
        child: CoveredProbeChildIdentityV1,
    ) -> Result<CoveredCarrierPhysicalPhaseResultV1, DirectV11RealConsumerError> {
        validate_covered_stack_deferred_native_v2_soil_beginning_v1(self, beginning)?;
        let (&leader_id, leader) =
            request
                .lanes
                .first_key_value()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered physical empty terminal batch",
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
                "covered physical beginning topology",
            ));
        }
        let beginning_stage3_state = beginning.stage3_by_lane().get(&leader_id).cloned().ok_or(
            DirectV11RealConsumerError::Identity("covered physical leader state"),
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
        self.execute_shared_covered_carrier_physical_phase_v1(
            beginning,
            &leader_request,
            CoveredSnowBoundaryStateV1::BatchTerminalTrial {
                lanes: request.lanes.clone(),
            },
            child,
        )
    }

    #[allow(clippy::too_many_lines)]
    fn execute_shared_covered_carrier_physical_phase_v1(
        &self,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        request: &CoveredTerminalTrialRequestV1,
        snow_boundary_state: CoveredSnowBoundaryStateV1,
        child: CoveredProbeChildIdentityV1,
    ) -> Result<CoveredCarrierPhysicalPhaseResultV1, DirectV11RealConsumerError> {
        let setup_profile = CarrierProfileScopeV1::begin("carrier physical setup");
        audit_covered_carrier_support(request.support);
        if child.trial_support != request.support
            || child.role != request.role
            || child.attempt_ordinal != request.attempt_ordinal
            || child.beginning_joint_sha256 != beginning.joint.receipt_sha256()
            || request.beginning_joint != beginning.joint
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered physical probe-child join",
            ));
        }
        let interval_s = f64::from_bits(request.support.duration_s_bits());
        if interval_s <= 0.0 || !interval_s.is_finite() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered physical positive trial support",
            ));
        }
        for forcing in self.stage3_forcing_by_lane.values() {
            if forcing.duration_seconds.to_bits() != interval_s.to_bits() {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered physical exact projected forcing duration",
                ));
            }
        }

        let projected_vegetation = covered_boxed_execution_v1(|| {
            beginning
                .shadow
                .validated_v9_to_v8_projection_v1()
                .map_err(DirectV11RealConsumerError::Runtime)
        })?;
        let carrier_receipts = self.carrier_receipts_by_destination(
            interval_s,
            projected_vegetation.state(),
            &beginning.stage3_by_lane,
            self.stage3_forcing_by_lane,
        )?;
        let seed = self.stage3_lower_boundaries_by_destination(
            &carrier_receipts,
            self.stage3_inputs_by_lane,
            self.stage3_forcing_by_lane,
        )?;
        let mut seed = self.merge_latest_stage3_state_operands(&seed, &beginning.stage3_by_lane)?;
        let covered_destinations = seed.keys().cloned().collect::<BTreeSet<_>>();
        if covered_destinations != self.covered_expected_destinations() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered physical Stage-3 lower-boundary membership",
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
                "covered physical Stage-3/open destination membership",
            ));
        }
        let prepared = DirectV9RealConsumerShadow::prepare_covered_canopy_soil_input(self.interval)
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
            })?;
        let validated_soil_read = beginning.validated_soil_read_v1()?;
        drop(setup_profile);
        let evidence_profile = CarrierProfileScopeV1::begin("carrier physical evidence");
        let evidence = self.build_provisional_covered_iteration_evidence_v1(
            request.support,
            &seed,
            &prepared,
            CoveredCarrierEnvelopeBuildV1 {
                candidate: &beginning.shadow,
                validated_v8_projection: &projected_vegetation,
                validated_soil_read: validated_soil_read.as_ref(),
                interval_s,
                duration_s_bits: request.support.duration_s_bits(),
                covered_destinations: &covered_destinations,
                covered_boundaries: &seed,
                open_boundaries: &open_boundaries,
                provisional: true,
                finalize_wb14_parent_interval: self.finalize_wb14_parent_interval,
            },
            beginning.soil_candidate.as_ref(),
            beginning.soil_continuation.as_ref(),
        )?;
        drop(evidence_profile);
        let completion_profile = CarrierProfileScopeV1::begin("carrier physical completion");
        #[cfg(test)]
        let mut evidence = evidence;
        #[cfg(test)]
        if canonical_covered_parity_poison_selected_for_current_map_v1(
            CanonicalCoveredPhysicalParityPoisonV1::Precipitation,
        ) {
            if let Some((_, set)) = evidence.precipitation_sets.iter_mut().next() {
                set.receipt_sha256 = Digest32::zero();
            }
        }
        for set in evidence.precipitation_sets.values() {
            validate_precipitation_phase_parcel_set(set).map_err(|error| {
                DirectV11RealConsumerError::from_stage3_physical_custody(&error)
            })?;
        }
        let lane_states = snow_boundary_state.lane_states(request);
        if lane_states.is_empty() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered physical terminal lane set",
            ));
        }
        let lane_to_ofe = self.covered_lane_to_ofe(&beginning.stage3_by_lane)?;
        let beginning_soil = covered_carrier_soil_beginning_identity_v1(beginning)?;
        let mut terminal_soil_trials = BTreeMap::new();
        let mut terminal_soil_credits = BTreeMap::new();
        for (lane_id, lane) in &lane_states {
            let ofe_id =
                lane_to_ofe
                    .get(lane_id)
                    .cloned()
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered physical terminal snow-soil OFE",
                    ))?;
            let configured_top = beginning
                .shadow
                .inner
                .lse_configuration
                .ofes
                .iter()
                .find(|value| value.ofe_id == ofe_id)
                .and_then(|value| value.soil_interface_layers.first())
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered physical configured soil top",
                ))?;
            let beginning_soil_top = beginning_soil
                .read_view
                .ordered_ofes()
                .into_iter()
                .find(|value| value.ofe_id() == &ofe_id)
                .and_then(|value| value.ordered_layers().into_iter().next())
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered physical beginning soil top",
                ))?;
            let stage3_inputs = self.stage3_inputs_by_lane.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("covered physical Stage-3 inputs"),
            )?;
            let trial = physical_outcome_ledger::evaluate_terminal_snow_bottom_soil_trial_v1(
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
                    beginning_soil_owner_id: beginning_soil.owner_id,
                    beginning_soil_state_sha256: beginning_soil.state_sha256,
                    transaction_id: evidence.transaction_id,
                    beginning_first_soil: beginning_soil_top,
                },
            )
            .map_err(|_| {
                DirectV11RealConsumerError::Identity("covered physical terminal snow-soil trial")
            })?;
            #[cfg(test)]
            let mut trial = trial;
            #[cfg(test)]
            if canonical_covered_parity_poison_selected_for_current_map_v1(
                CanonicalCoveredPhysicalParityPoisonV1::SoilCandidate,
            ) {
                trial.receipt.receipt_sha256 = Digest32::zero();
            }
            trial.receipt.validate().map_err(|_| {
                DirectV11RealConsumerError::Stage3SnowSoilHeatCustody(
                    "terminal snow-soil trial receipt",
                )
            })?;
            let credit = SoilThermalTopBoundaryCreditV1 {
                lane_id: *lane_id,
                ofe_id,
                first_layer_id: configured_top.layer_id.clone(),
                beginning_owner_id: beginning_soil.owner_id.clone(),
                beginning_configuration_sha256: beginning_soil.configuration_sha256.clone(),
                beginning_state_sha256: beginning_soil.state_sha256.clone(),
                support_start_ns: i64::try_from(request.support.start_ns().get()).map_err(
                    |_| DirectV11RealConsumerError::Identity("physical soil credit support start"),
                )?,
                support_end_ns: i64::try_from(request.support.end_ns().get()).map_err(|_| {
                    DirectV11RealConsumerError::Identity("physical soil credit support end")
                })?,
                accepted_positive_downward_j_m2_ofe_ground: trial.soil_heat_j_m2,
                soil_thermal_credit_j_m2_ofe_ground: trial.soil_heat_j_m2,
                snow_soil_heat_receipt_sha256: Sha256Digest::try_new(digest32_hex(
                    trial.receipt.receipt_sha256,
                ))
                .map_err(|_| DirectV11RealConsumerError::Identity("physical soil credit digest"))?,
            };
            terminal_soil_credits.insert(*lane_id, credit);
            terminal_soil_trials.insert(*lane_id, trial);
        }
        let ordered_credits = terminal_soil_credits.values().cloned().collect::<Vec<_>>();
        let (soil_candidate, soil_continuation, v2_transaction_authority) = match &beginning
            .shadow
            .inner
            .soil_thermal
        {
            DirectSoilThermalResident::V1(_) => (
                self.unpublished_soil_candidate_for_covered_iteration_v1(
                    request.support,
                    evidence.transaction_id,
                    &evidence.soil_candidates,
                    &evidence.soil_energy_operands_v2,
                    &ordered_credits,
                )?,
                None,
                None,
            ),
            DirectSoilThermalResident::V2(_) => {
                let authority = crate::land_surface_energy_shadow::PhysicalSoilEnergyTransactionAuthorityV2::try_from_pre_ingress_candidates(
                        evidence.physical.transaction_id(),
                        request.support.start_ns().get(),
                        request.support.end_ns().get(),
                        evidence
                            .physical
                            .hydrology()
                            .pre_ingress_soil_thermal_candidates(),
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            DirectV9RealConsumerError::LandSurfaceShadow(error),
                        ))
                    })?;
                let (candidate, continuation) = unpublished_v2_soil_trial(
                    beginning,
                    self.deferred_native_v2_soil_custody.as_ref(),
                    authority,
                    evidence.physical.hydrology(),
                    request.support,
                    &ordered_credits,
                )?;
                (candidate, continuation, Some(authority))
            }
        };
        evidence.surface_custody.validate(evidence.transaction_id)?;
        match v2_transaction_authority {
            None if soil_candidate.transaction_id() != Some(evidence.transaction_id) => {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered physical legacy soil/source transaction custody",
                ));
            }
            Some(authority)
                if authority.source_transaction_id != evidence.transaction_id
                    || soil_continuation.as_ref().is_none_or(|continuation| {
                        let original = continuation.original_prepared().beginning_owner();
                        let trial = continuation.physical_trial();
                        let base_custody = trial.unpublished_predecessor_trial_sha256().is_none()
                            && trial.accepted_predecessor_receipt_chain_sha256()
                                == Some(&original.receipt_chain_sha256)
                            && trial.beginning_state_sha256() == &original.state.state_sha256
                            && trial.predecessor_transaction_id()
                                == original.expected_predecessor_transaction_id;
                        let continued_custody =
                            trial.unpublished_predecessor_trial_sha256().is_some()
                                && trial.accepted_predecessor_receipt_chain_sha256().is_none();
                        original.transaction_id != authority.soil_thermal_transaction_id
                            || soil_candidate.transaction_id() != Some(trial.transaction_id())
                            || !(base_custody || continued_custody)
                            || original.support_start_ns > trial.support_start_ns()
                            || original.support_end_ns < trial.support_end_ns()
                    }) =>
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered physical native soil source/target transaction custody",
                ));
            }
            _ => {}
        }
        let validated_soil_ending =
            ValidatedCoveredCarrierSoilEndingV1::after_physical_custody_validation(
                soil_candidate,
                soil_continuation,
            );
        let destination_receipts = carrier_receipts
            .iter()
            .map(|(key, value)| (key.clone(), value.diagnostic_sha256))
            .chain(open_diagnostics)
            .collect::<BTreeMap<_, _>>();
        let mut corrected = evidence.corrected_boundaries;
        for (destination, boundary) in open_boundaries {
            corrected.insert(destination, boundary);
        }
        #[cfg(test)]
        if canonical_covered_parity_poison_selected_for_current_map_v1(
            CanonicalCoveredPhysicalParityPoisonV1::LowerBoundary,
        ) || canonical_covered_parity_poison_selected_for_current_map_v1(
            CanonicalCoveredPhysicalParityPoisonV1::LowerBoundaryAndV8Persistent,
        ) {
            for boundary in corrected.values_mut() {
                boundary.latent_heat_j_kg = f64::NAN;
            }
        }
        let terms =
            self.lane_stage3_terms_from_boundaries(&destination_receipts, &corrected, interval_s)?;
        let mut boundaries_by_lane = BTreeMap::new();
        for lane_id in lane_states.keys() {
            let lane_terms = terms
                .get(lane_id)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered physical active lane",
                ))?;
            let precipitation = evidence.precipitation_sets.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("covered physical precipitation lane"),
            )?;
            let (_, advection) = reconstruct_precipitation_mass_and_advected_heat(precipitation)
                .map_err(|error| {
                    DirectV11RealConsumerError::from_stage3_physical_custody(&error)
                })?;
            let snow = beginning.stage3_by_lane.get(lane_id).ok_or(
                DirectV11RealConsumerError::Identity("covered physical snow lane"),
            )?;
            let snow_digest = if crate::hydrology::stage3_is_terminal_event_domain(snow) {
                Wb11HydrologyKernel::project_stage3_terminal_surface_state_v1(snow)
            } else {
                Wb11HydrologyKernel::project_stage3_surface_state_v1(snow)
            }
            .map_err(|_| DirectV11RealConsumerError::Identity("covered physical snow projection"))?
            .beginning_stage3_state_sha256;
            let (sensible, vapor, latent) = outward_snow_fluxes_to_stage3(
                lane_terms.sensible_to_canopy_air_w_m2,
                lane_terms.vapor_to_canopy_air_kg_m2_s,
                lane_terms.latent_energy_to_canopy_air_j_m2,
                interval_s,
            );
            let soil_trial =
                terminal_soil_trials
                    .get(lane_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "covered physical terminal trial join",
                    ))?;
            let boundary = Stage3SnowSurfaceBoundaryReceiptV1::try_new(
                Stage3SnowSurfaceBoundaryReceiptInputs {
                    support: request.support,
                    sensible_energy_j_m2: sensible,
                    vapor_mass_kg_m2: vapor,
                    latent_energy_j_m2: latent,
                    shortwave_energy_j_m2: lane_terms.snow_absorbed_shortwave_w_m2 * interval_s,
                    net_longwave_energy_j_m2: lane_terms.snow_net_longwave_w_m2 * interval_s,
                    precipitation_advection_j_m2: advection,
                    snow_soil_heat_j_m2: soil_trial.snow_heat_j_m2,
                    latent_heat_j_kg: lane_terms.latent_heat_j_kg,
                    beginning_stage3_state_sha256: snow_digest,
                    identity: Stage3BoundaryIdentity::Provisional {
                        carrier_receipt_sha256: lane_terms.provisional_carrier_receipt_sha256,
                    },
                },
            )?;
            boundaries_by_lane.insert(*lane_id, boundary);
        }
        let result = CoveredCarrierPhysicalPhaseResultV1 {
            physical: evidence.physical,
            projected_vegetation: (*projected_vegetation).into_values(),
            native_finalization_posture: evidence.native_finalization_posture,
            precipitation_sets: evidence.precipitation_sets,
            complete_lower_boundaries: corrected,
            covered_lse_states: evidence.lse_states,
            validated_soil_ending,
            batch_boundaries_by_lane: boundaries_by_lane,
            carrier_source_receipts: carrier_receipts,
            open_snow_candidates,
            terminal_soil_trials,
            terminal_soil_credits,
            surface_custody: evidence.surface_custody,
        };
        #[cfg(test)]
        record_covered_physical_endpoint_audit_v1(&result);
        drop(completion_profile);
        Ok(result)
    }

    /// Consume the exact retained physical prefix for the accepted map and
    /// continue through the one complete unpublished owner construction.
    /// No LSE, surface-liquid, WB14, precipitation, or snow--soil physical
    /// evaluation is reachable from this continuation.
    fn complete_covered_carrier_physical_phase_v1(
        &self,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        request: &CoveredTerminalBatchTrialRequestV2,
        child: CoveredProbeChildIdentityV1,
        physical: CoveredCarrierPhysicalPhaseResultV1,
    ) -> Result<CoveredCarrierPhaseResultV1, DirectV11RealConsumerError> {
        let (&leader_id, leader) =
            request
                .lanes
                .first_key_value()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "covered final empty terminal batch",
                ))?;
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
            beginning_stage3_state: Box::new(
                beginning.stage3_by_lane().get(&leader_id).cloned().ok_or(
                    DirectV11RealConsumerError::Identity("covered final leader state"),
                )?,
            ),
            ending_snow_hint: None,
            beginning_joint: request.beginning_joint.clone(),
        };
        self.complete_covered_carrier_physical_result_v1(
            beginning,
            &leader_request,
            child,
            physical,
        )
    }

    fn complete_covered_carrier_physical_result_v1(
        &self,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        request: &CoveredTerminalTrialRequestV1,
        child: CoveredProbeChildIdentityV1,
        physical: CoveredCarrierPhysicalPhaseResultV1,
    ) -> Result<CoveredCarrierPhaseResultV1, DirectV11RealConsumerError> {
        let envelope_profile = CarrierProfileScopeV1::begin("carrier complete envelope");
        physical
            .surface_custody
            .validate(physical.physical.transaction_id())?;
        let projected_vegetation = physical.projected_vegetation;
        let configured_root_layers = beginning
            .shadow
            .inner
            .vegetation_configuration
            .strata
            .iter()
            .flat_map(|stratum| stratum.root_layers.iter().map(|root| root.layer_id.clone()))
            .collect::<BTreeSet<_>>();
        let vegetation_bindings = projected_vegetation
            .0
            .strata
            .iter()
            .flat_map(|stratum| {
                stratum.tile_ids.iter().map(|tile_id| {
                    let occupancy_id = openwepp_kernel_contract::OccupancyId {
                        stratum_id: stratum.stratum_id.clone(),
                        tile_id: tile_id.clone(),
                    };
                    openwepp_vegetation::V8LseComponentId::try_new(format!(
                        "{}::{}",
                        occupancy_id.stratum_id.as_str(),
                        occupancy_id.tile_id.as_str(),
                    ))
                    .map(|component_id| {
                        openwepp_vegetation::V8ComponentOccupancyBinding {
                            component_id,
                            occupancy_id,
                            vertical_rank: stratum.vertical_rank,
                        }
                    })
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                    DirectV9RealConsumerError::Vegetation(error),
                ))
            })?;
        let persistent_forcing = openwepp_vegetation::V8PersistentForcingReceipt {
            model_definition_sha256: projected_vegetation.0.model_definition_sha256.clone(),
            configuration_sha256: projected_vegetation.0.configuration_sha256.clone(),
            transaction_id: physical.physical.transaction_id(),
            vegetation_beginning_state_sha256: projected_vegetation.1.state_sha256.clone(),
            air_temperature_k: self.interval.vegetation_forcing.air_temperature_k,
            gsi: self.interval.vegetation_forcing.gsi,
            soil_temperature_k_by_layer: self
                .interval
                .vegetation_forcing
                .soil_layers
                .iter()
                .filter(|layer| configured_root_layers.contains(&layer.layer_id))
                .map(|layer| (layer.layer_id.clone(), layer.temperature_k))
                .collect(),
        };
        let nitrogen =
            BiogeochemistryNitrogenArbiter::try_new(&beginning.shadow.inner.biogeochemistry)
                .map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
                })?;
        let envelope = physical
            .physical
            .into_complete_owner_envelope_v11(
                &vegetation_bindings,
                &projected_vegetation.0,
                &projected_vegetation.1,
                &persistent_forcing,
                &nitrogen,
                &beginning.shadow.inner.biogeochemistry,
                request.support.duration_s_bits(),
            )
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                    DirectV9RealConsumerError::OwnerEnvelope(error),
                ))
            })?;
        // The physical posture remains part of the physical endpoint and its
        // solver/admission evidence. Both admitted postures continue through
        // the same complete-owner mutation below; neither may deep-clone the
        // immutable beginning before physical success.
        let _native_finalization_posture = physical.native_finalization_posture;
        drop(envelope_profile);
        self.finalize_shared_covered_carrier_engine_v1(
            beginning,
            request,
            child,
            Box::new(CoveredCarrierPhysicalTrialV1 {
                envelope: Box::new(envelope),
                carrier_receipts: physical.carrier_source_receipts,
                corrected: physical.complete_lower_boundaries,
                lse_states: physical.covered_lse_states,
                precipitation_sets: physical.precipitation_sets,
                open_snow_candidates: physical.open_snow_candidates,
                terminal_soil_trials: physical.terminal_soil_trials,
                terminal_soil_credits: physical.terminal_soil_credits,
                boundaries_by_lane: physical.batch_boundaries_by_lane,
                validated_soil_ending: physical.validated_soil_ending,
            }),
        )
    }

    #[inline(never)]
    fn finalize_shared_covered_carrier_engine_v1(
        &self,
        beginning: &CoveredCarrierEphemeralCandidatesV1,
        request: &CoveredTerminalTrialRequestV1,
        child: CoveredProbeChildIdentityV1,
        physical: Box<CoveredCarrierPhysicalTrialV1>,
    ) -> Result<CoveredCarrierPhaseResultV1, DirectV11RealConsumerError> {
        let adoption_profile = CarrierProfileScopeV1::begin("carrier complete adoption");
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
            validated_soil_ending: precomputed_soil_ending,
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
        let soil_trial =
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
                    if candidate.inner.soil_thermal.v1().map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })? != precomputed_soil_ending.candidate().v1().map_err(|error| {
                        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                            error,
                        ))
                    })? {
                        return Err(DirectV11RealConsumerError::Identity(
                            "final covered retained soil physical prefix",
                        ));
                    }
                    Ok(precomputed_soil_ending)
                }
                DirectSoilThermalResident::V2(_) => {
                    stage_unpublished_v2_carrier_owners(&mut candidate, &envelope)?;
                    Ok(precomputed_soil_ending)
                }
            })?;
        drop(adoption_profile);
        let projection_profile = CarrierProfileScopeV1::begin("carrier complete projection");
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
        let authenticated_final_parent = self.wb14_coupled_child_binding.child_support_end_ns
            == self.wb14_coupled_child_binding.parent_support_end_ns;
        if authenticated_final_parent != self.finalize_wb14_parent_interval {
            return Err(DirectV11RealConsumerError::Identity(
                "covered carrier parent-final posture",
            ));
        }
        let authenticated_predecessor_transaction =
            beginning.shadow.vegetation_state.0.last_transaction_id;
        let staged_parent_transaction = if authenticated_final_parent {
            envelope.transaction_id().0
        } else {
            authenticated_predecessor_transaction
        };
        let inactive_native_surface = envelope
            .hydrology()
            .surface_ingress()
            .is_stage3_covered_native_inactive()
            .then(|| {
                beginning
                    .shadow
                    .inner
                    .hydrology_frame
                    .surface_liquid_shadow
                    .clone()
            });
        normalize_v11_staged_parent_lineage(&mut candidate, staged_parent_transaction)?;
        if let Some(surface) = inactive_native_surface {
            candidate.inner.hydrology_frame.surface_liquid_shadow = surface;
        }
        drop(projection_profile);
        let owner_profile = CarrierProfileScopeV1::begin("carrier complete owner");
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
        // The V2 trial stays in typed unpublished custody below. A joint is
        // a V11 owner join, so its soil member remains the complete resident
        // owner until the final continuation is consumed exactly once.
        let trial_snow = request
            .beginning_joint
            .owner_bytes()
            .get("snow")
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered carrier trial snow owner",
            ))?
            .clone();
        let vegetation_validation_profile =
            CarrierProfileScopeV1::begin("carrier owner vegetation validation");
        let validated_vegetation = validated_normalized_carrier_vegetation_v10_v1(
            candidate.as_ref(),
            staged_parent_transaction,
        )?;
        drop(vegetation_validation_profile);
        let resident_owner_bytes =
            candidate.canonical_owner_state_bytes_for_carrier_endpoint_v1(&validated_vegetation)?;
        drop(validated_vegetation);
        let ending_owner_projection =
            CoveredCarrierEndpointOwnerProjectionV1::from_validated_owner_bytes(
                *candidate,
                resident_owner_bytes,
            );
        let joint_map_profile = CarrierProfileScopeV1::begin("carrier owner joint map");
        let ending_owner_bytes = ending_owner_projection.joint_owner_bytes_with_snow_v1(trial_snow);
        drop(joint_map_profile);
        #[cfg(test)]
        let mut ending_owner_bytes = ending_owner_bytes;
        #[cfg(test)]
        if matches!(
            canonical_covered_parity_poison_v1(),
            Some(CanonicalCoveredPhysicalParityPoisonV1::EndingJoint)
        ) {
            ending_owner_bytes.remove("vegetation");
        }
        #[cfg(test)]
        canonical_covered_final_constructor_boundary_v1(
            CanonicalCoveredFinalConstructorStageV1::EndingJoint,
        );
        let joint_seal_profile = CarrierProfileScopeV1::begin("carrier owner joint seal");
        let ending_joint = covered_boxed_execution_v1(|| {
            CoveredTerminalJointTrialStateV1::try_new(
                beginning.joint.authority().clone(),
                ending_owner_bytes,
            )
            .map_err(DirectV11RealConsumerError::Stage3)
        })?;
        drop(joint_seal_profile);
        let endpoint = covered_boxed_execution_v1(|| {
            ending_owner_projection.try_into_ephemeral_candidates_v1(
                *ending_joint,
                beginning.stage3_by_lane.clone(),
                *soil_trial,
            )
        })?;
        let (mut ending_candidates, result_soil_candidate, _result_soil_continuation) = *endpoint;
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
                ending_candidates,
                beginning_stage3_by_lane: beginning.stage3_by_lane.clone(),
                precipitation_sets,
                carrier_envelope: *envelope,
                complete_lower_boundaries: corrected,
                carrier_source_receipts: carrier_receipts,
                open_snow_candidates,
                covered_lse_states: lse_states,
                soil_candidate: result_soil_candidate,
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
        drop(owner_profile);
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
        DirectSoilThermalUnpublishedContinuationResultV2,
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
            let continuation = v2_shadow
                .authenticate_soil_thermal_base_unpublished_result_v2(
                    &prepared,
                    &trial,
                    expected.accepted_operands(),
                )
                .expect("authenticated unpublished V2 continuation");
            (
                DirectSoilThermalCandidate::from_v2(trial).expect("typed V2 candidate"),
                continuation,
            )
        };
        let (candidate, continuation) = trial(0.25, 'b');
        let (stale_candidate, _) = trial(0.5, 'c');
        (v2_shadow, candidate, stale_candidate, continuation)
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

    #[test]
    fn normalized_v10_owner_proof_is_bound_to_the_exact_borrowed_revision() {
        let (mut shadow, _, _, _) = native_v2_shadow_and_trials();
        let parent_transaction = shadow.vegetation_state.0.last_transaction_id;
        normalize_v11_staged_parent_lineage(&mut shadow, parent_transaction)
            .expect("normalized carrier parent lineage");
        let proof = validated_normalized_carrier_vegetation_v10_v1(&shadow, parent_transaction)
            .expect("validated normalized V10 proof");
        assert!(proof.matches(&shadow.vegetation_state, &shadow.vegetation_configuration));

        let equal_but_distinct_revision = shadow.clone();
        assert!(
            !proof.matches(
                &equal_but_distinct_revision.vegetation_state,
                &equal_but_distinct_revision.vegetation_configuration,
            ),
            "the borrowed proof must not authorize an equal clone",
        );
    }

    #[test]
    fn native_v2_soil_read_proof_is_seal_and_pointer_bound() {
        let (shadow, candidate, stale_candidate, _) = native_v2_shadow_and_trials();
        let ending_joint = carrier_joint(&shadow, Some(&candidate));
        let ending = CoveredCarrierEphemeralCandidatesV1::try_new_with_soil_candidate(
            ending_joint,
            shadow,
            BTreeMap::new(),
            Some(candidate.clone()),
            None,
        )
        .expect("typed V2 ending");
        let proof = ending
            .validated_soil_read_v1()
            .expect("sealed carrier soil read")
            .expect("candidate-backed proof");

        assert_eq!(
            proof.read_view_for(None),
            Err(DirectV9RealConsumerError::OwnerClosure(
                "validated carrier soil read without candidate"
            )),
        );
        assert_eq!(
            proof.read_view_for(Some(&candidate)),
            Err(DirectV9RealConsumerError::OwnerClosure(
                "validated carrier soil read candidate identity"
            )),
            "an equal but separately owned candidate cannot consume the proof",
        );
        proof
            .read_view_for(ending.soil_candidate.as_ref())
            .expect("exact borrowed candidate consumes proof")
            .validate()
            .expect("proof carries a validated read view");

        let mut substituted = ending.clone();
        substituted.soil_candidate = Some(stale_candidate);
        assert!(matches!(
            substituted.validated_soil_read_v1(),
            Err(DirectV11RealConsumerError::Identity(
                "covered carrier soil read candidate seal"
            )),
        ));
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
    fn native_v2_selected_joint_retains_resident_owner_and_binds_typed_trial_exactly() {
        let (shadow, candidate, _, _) = native_v2_shadow_and_trials();
        let beginning_joint = carrier_joint(&shadow, None);
        let ending_joint = carrier_joint(&shadow, Some(&candidate));
        assert_eq!(
            beginning_joint.owner_bytes().get("soil_thermal"),
            ending_joint.owner_bytes().get("soil_thermal"),
            "an unpublished V2 trial cannot replace the complete resident owner bytes",
        );
        assert_ne!(
            candidate.state_sha256(),
            shadow.inner.soil_thermal.state_sha256(),
            "the typed trial seal, not the retained resident bytes, binds physical advancement",
        );
        let ending = CoveredCarrierEphemeralCandidatesV1::try_new_with_soil_candidate(
            ending_joint.clone(),
            shadow,
            BTreeMap::new(),
            Some(candidate.clone()),
            None,
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
    fn native_v2_second_child_beginning_rejects_sealed_typed_carry_substitution() {
        let (shadow, candidate, stale_candidate, _) = native_v2_shadow_and_trials();
        let parent_start_state = shadow.inner.soil_thermal.state_sha256().clone();
        assert_ne!(candidate.state_sha256(), &parent_start_state);
        let ending_joint = carrier_joint(&shadow, Some(&candidate));
        let ending = CoveredCarrierEphemeralCandidatesV1::try_new_with_soil_candidate(
            ending_joint.clone(),
            shadow.clone(),
            BTreeMap::new(),
            Some(candidate.clone()),
            None,
        )
        .expect("first-child ending becomes second-child beginning");
        let second_child = covered_carrier_soil_beginning_identity_v1(&ending)
            .expect("typed second-child beginning");
        assert_eq!(second_child.state_sha256, candidate.state_sha256());
        assert_ne!(second_child.state_sha256, &parent_start_state);

        let mut substituted = ending.clone();
        substituted.soil_candidate = Some(stale_candidate);
        assert!(
            substituted
                .try_with_selected_stage3_by_lane(ending_joint, BTreeMap::new())
                .is_err(),
            "a substituted predecessor carry must not become the next child beginning",
        );
    }

    #[test]
    fn native_v2_deferred_custody_seeds_joint_and_seals_exact_typed_trial() {
        let (shadow, candidate, stale_candidate, _) = native_v2_shadow_and_trials();
        let custody = DeferredNativeV2SoilCustodyV1::try_new(&shadow, candidate.clone(), None)
            .expect("authenticated deferred native V2 custody");
        let joint = carrier_joint(&shadow, Some(&candidate));
        let initial =
            CoveredCarrierEphemeralCandidatesV1::try_new_with_deferred_native_v2_soil_custody(
                joint.clone(),
                shadow.clone(),
                BTreeMap::new(),
                &custody,
            )
            .expect("deferred custody initial candidate");
        assert_eq!(initial.joint(), &joint);
        assert_eq!(initial.soil_candidate.as_ref(), Some(&candidate));
        assert_eq!(initial.soil_continuation(), None);

        let mut substituted = initial.clone();
        substituted.soil_candidate = Some(stale_candidate);
        assert!(
            substituted
                .try_with_selected_stage3_by_lane(joint, BTreeMap::new())
                .is_err(),
            "a substituted trial digest must fail before carrier evaluation",
        );
    }

    #[test]
    fn native_v2_selected_joint_rejects_stale_carry_and_substituted_owner() {
        let (shadow, candidate, stale_candidate, _) = native_v2_shadow_and_trials();
        let ending_joint = carrier_joint(&shadow, Some(&candidate));
        let ending = CoveredCarrierEphemeralCandidatesV1::try_new_with_soil_candidate(
            ending_joint.clone(),
            shadow.clone(),
            BTreeMap::new(),
            Some(candidate),
            None,
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
        assert!(body.contains("prepare_soil_thermal_base_unpublished_continuation_v2("));
        assert!(body.contains("prepare_next_soil_thermal_unpublished_continuation_v2("));
        assert!(body.contains("advance_soil_thermal_unpublished_continuation_v2("));
        for forbidden in [
            "apply_soil_thermal_energy_credit_v2(",
            "aggregate_soil_thermal_ending_v2(",
            "seal_soil_thermal_accepted_candidate_v2(",
            "install_soil_thermal_accepted_v2(",
            "OPENWEPP_V2_CARRIER_TOP_CAPTURE",
            "eprintln!(",
        ] {
            assert!(
                !body.contains(forbidden),
                "unpublished V2 carrier emitted accepted custody: {forbidden}"
            );
        }
    }

    #[test]
    fn represented_snow_carrier_synchronizes_the_native_wb14_successor_only_when_present() {
        let source = include_str!("carrier_phase.rs");
        let body = source
            .split("match (\n        candidate.frozen_litter_v3_resident().is_some()")
            .nth(1)
            .expect("native frozen-litter carrier posture")
            .split("if increment_accepted_interval_count")
            .next()
            .expect("native frozen-litter carrier match");
        let native = body
            .split("(true, true) =>")
            .nth(1)
            .expect("complete native successor branch")
            .split("(false, false) =>")
            .next()
            .expect("complete native successor branch body");
        assert!(native.contains("stage_frozen_litter_wb14_parent_from_inner_v1"));
        let absent = body
            .split("(false, false) =>")
            .nth(1)
            .expect("absent native successor branch")
            .split("_ =>")
            .next()
            .expect("absent native successor branch body");
        assert!(!absent.contains("stage_frozen_litter_wb14_parent_from_inner_v1"));
    }

    #[test]
    fn v2_soil_temporal_posture_separates_replay_successor_overlap_and_gap() {
        assert_eq!(
            unpublished_v2_soil_temporal_posture_v1(0, 900, 0, 900)
                .expect("same-support coupled replay"),
            UnpublishedV2SoilTemporalPostureV1::SameSupportReplay
        );
        assert_eq!(
            unpublished_v2_soil_temporal_posture_v1(0, 60, 60, 900)
                .expect("strictly contiguous successor"),
            UnpublishedV2SoilTemporalPostureV1::ContiguousSuccessor
        );
        for (retained_start, retained_end, requested_start, requested_end) in [
            (0, 900, 0, 1_800),
            (0, 60, 120, 900),
            (0, 60, 60, 60),
            (60, 120, 0, 60),
        ] {
            assert!(
                unpublished_v2_soil_temporal_posture_v1(
                    retained_start,
                    retained_end,
                    requested_start,
                    requested_end,
                )
                .is_err()
            );
        }
    }

    #[test]
    fn same_support_raw_iterate_reuses_immutable_prior_child_custody() {
        assert_eq!(
            unpublished_v2_soil_beginning_source_v1(
                Some((900, 1_320)),
                Some((0, 900)),
                900,
                1_320,
            )
            .expect("same-support iterate with contiguous immutable custody"),
            UnpublishedV2SoilBeginningSourceV1::DeferredPriorChildCustody,
        );
        assert_eq!(
            unpublished_v2_soil_beginning_source_v1(Some((0, 900)), Some((0, 900)), 900, 1_320,)
                .expect("raw first successor iterate"),
            UnpublishedV2SoilBeginningSourceV1::RawContiguousCandidate,
        );
        assert_eq!(
            unpublished_v2_soil_beginning_source_v1(Some((900, 1_320)), None, 900, 1_320,)
                .expect("first-child same-support replay"),
            UnpublishedV2SoilBeginningSourceV1::AuthenticatedResident,
        );
        assert!(
            unpublished_v2_soil_beginning_source_v1(
                Some((900, 1_320)),
                Some((0, 800)),
                900,
                1_320,
            )
            .is_err(),
            "a gapped immutable custody cannot seed a same-support iterate",
        );
    }

    #[test]
    fn retained_custody_original_start_outlives_current_carrier_resident_start() {
        assert_eq!(
            unpublished_v2_original_support_start_ns_v1(900, 0, None),
            0,
            "a reauthenticated base trial retains its original outer support start",
        );
        assert_eq!(
            unpublished_v2_original_support_start_ns_v1(1_320, 900, Some(0)),
            0,
            "a chained continuation retains its authenticated original-prepared start",
        );
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
        assert!(
            source.contains("Evaluate exactly the physical prefix used by a canonical covered map")
        );
        assert!(source.contains("provisional: true"));
        assert!(source.contains("accept_envelope(envelope.transaction_id(), &envelope)"));
        assert!(!implementation.contains("let ending_candidates = beginning.clone()"));
    }

    #[test]
    fn batch_phase_enters_the_shared_physical_prefix_once_with_complete_lane_state() {
        let source = include_str!("carrier_phase.rs");
        let body = source
            .split("pub(crate) fn execute_covered_carrier_batch_phase_v2")
            .nth(1)
            .expect("batch carrier entry")
            .split("fn execute_covered_carrier_physical_phase_v1")
            .next()
            .expect("batch carrier body");
        assert_eq!(
            body.matches("self.execute_shared_covered_carrier_physical_phase_v1(")
                .count(),
            1,
            "one batch candidate must advance the shared physical prefix once",
        );
        assert_eq!(
            body.matches("self.complete_covered_carrier_physical_result_v1(")
                .count(),
            1,
            "one batch candidate must consume that physical result once",
        );
        assert!(
            body.contains("BatchTerminalTrial {\n                lanes: request.lanes.clone(),")
        );

        let engine = source
            .split("fn execute_shared_covered_carrier_physical_phase_v1")
            .nth(1)
            .expect("shared carrier physical prefix");
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
