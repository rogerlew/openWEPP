#[allow(clippy::wildcard_imports)]
use super::*;

impl Wb11HydrologyKernel {
    pub(crate) fn covered_terminal_batch_common_earliest_lanes_v2(
        request: &CoveredTerminalBatchTrialRequestV2,
    ) -> Option<(ModelTimeNs, Vec<u32>)> {
        let tick = request
            .lanes
            .values()
            .filter_map(|lane| lane.candidate_event_tick)
            .min()?;
        let lanes = request
            .lanes
            .iter()
            .filter_map(|(lane_id, lane)| (lane.candidate_event_tick == Some(tick)).then_some(*lane_id))
            .collect();
        Some((tick, lanes))
    }

    pub(crate) fn join_covered_terminal_batch_prefix_v2(
        request: &CoveredTerminalBatchTrialRequestV2,
        hydrology_endings_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
        provider: &mut CoveredTerminalBatchTrialProviderV2<'_>,
        join_hydrology: &mut CoveredTerminalBatchHydrologyJoinV2<'_>,
    ) -> Result<CoveredTerminalBatchTrialResultV2, DirectSnowStage3EvaluationError> {
        if request.lanes.is_empty()
            || request
                .lanes
                .iter()
                .any(|(lane_id, lane)| lane.lane_id != *lane_id)
            || request.lanes.values().any(|lane| {
                !lane.ice_kg_m2.is_finite()
                    || !lane.liquid_kg_m2.is_finite()
                    || !lane.cold_content_j_m2.is_finite()
                    || !lane.surface_temperature_c.is_finite()
                    || !lane.snow_depth_m.is_finite()
                    || !lane.snow_density_kg_m3.is_finite()
                    || lane.ice_kg_m2 < 0.0
                    || lane.liquid_kg_m2 < 0.0
                    || lane.cold_content_j_m2 < 0.0
                    || lane.snow_depth_m < 0.0
                    || lane.snow_density_kg_m3 <= 0.0
                    || (lane.resolved_beginning && lane.snow_depth_m <= 0.0)
            })
            || hydrology_endings_by_lane.keys().ne(request.lanes.keys())
        {
            return Err(DirectSnowStage3EvaluationError::TerminalCustody(
                "covered terminal batch lane topology",
            ));
        }
        let candidates = provider(request)?;
        if candidates.support != request.support
            || candidates.beginning_joint_sha256 != request.beginning_joint.receipt_sha256()
            || candidates.boundaries_by_lane.keys().ne(request.lanes.keys())
            || candidates
                .boundaries_by_lane
                .values()
                .any(|boundary| boundary.support != request.support)
            || candidates
                .ordered_q_ss_receipts_by_lane
                .keys()
                .any(|lane_id| !request.lanes.contains_key(lane_id))
        {
            return Err(DirectSnowStage3EvaluationError::TerminalCustody(
                "covered terminal batch carrier join",
            ));
        }
        for (lane_id, ending) in &hydrology_endings_by_lane {
            if ending.lane_id != *lane_id {
                return Err(DirectSnowStage3EvaluationError::TerminalCustody(
                    "covered terminal batch hydrology ending identity",
                ));
            }
        }
        let ending_joint = join_hydrology(
            request,
            &candidates,
            &hydrology_endings_by_lane,
        )?;
        Ok(CoveredTerminalBatchTrialResultV2 {
            support: request.support,
            hydrology_endings_by_lane,
            carrier_candidates: candidates,
            ending_joint,
        })
    }

    #[allow(clippy::too_many_lines)]
    pub fn evaluate_stage3_persistent_day(
        inputs: &DirectActiveSnowPartitionInputs,
        state: &DirectSnowStage3PersistentState,
        lane_id: u32,
        interval_index: u64,
    ) -> Result<DirectSnowStage3PersistentDayResult, DirectSnowStage3EvaluationError> {
        if state.schema_version != 1 {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_persistent_request_state_mismatch",
                f64::from(state.schema_version),
                Some(1.0),
                Some(1.0),
            )
            .into());
        }
        let supports = inputs
            .hourly
            .iter()
            .copied()
            .map(|forcing| DirectSnowStage3SupportInput {
                forcing,
                duration_seconds: STAGE3_SECONDS_PER_HOUR,
            })
            .collect::<Vec<_>>();
        Self::evaluate_stage3_persistent_day_internal(
            inputs,
            state,
            lane_id,
            interval_index,
            &supports,
            None,
            None,
            None,
        )
    }

    pub fn evaluate_stage3_persistent_day_with_terminal_event(
        inputs: &DirectActiveSnowPartitionInputs,
        state: &DirectSnowStage3PersistentState,
        lane_id: u32,
        interval_index: u64,
        request: DirectSnowTerminalEventRequest,
    ) -> Result<DirectSnowStage3PersistentDayResult, DirectSnowStage3EvaluationError> {
        if state.schema_version != 2 || state.terminal_event_model != Some(request.model) {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_persistent_request_state_mismatch",
                f64::from(state.schema_version),
                Some(2.0),
                Some(2.0),
            )
            .into());
        }
        Self::evaluate_stage3_persistent_day_internal(
            inputs,
            state,
            lane_id,
            interval_index,
            &inputs
                .hourly
                .iter()
                .copied()
                .map(|forcing| DirectSnowStage3SupportInput {
                    forcing,
                    duration_seconds: STAGE3_SECONDS_PER_HOUR,
                })
                .collect::<Vec<_>>(),
            Some(request),
            None,
            None,
        )
    }

    /// Evaluate one actual coupled-time support with the same sequential and
    /// adaptive terminal equations used by the day wrapper. The support is
    /// not expanded, duplicated, scaled, or interpolated.
    pub fn evaluate_stage3_persistent_support(
        inputs: &DirectActiveSnowPartitionInputs,
        state: &DirectSnowStage3PersistentState,
        lane_id: u32,
        interval_index: u64,
        support: DirectSnowStage3SupportInput,
        request: DirectSnowTerminalEventRequest,
    ) -> Result<DirectSnowStage3PersistentDayResult, DirectSnowStage3EvaluationError> {
        if !support.duration_seconds.is_finite() || support.duration_seconds <= 0.0 {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_support_duration_seconds",
                support.duration_seconds,
                Some(f64::MIN_POSITIVE),
                None,
            )
            .into());
        }
        if state.schema_version != 2 || state.terminal_event_model != Some(request.model) {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_persistent_request_state_mismatch",
                f64::from(state.schema_version),
                Some(2.0),
                Some(2.0),
            )
            .into());
        }
        Self::evaluate_stage3_persistent_day_internal(
            inputs,
            state,
            lane_id,
            interval_index,
            &[support],
            Some(request),
            None,
            None,
        )
    }

    /// Evaluate one admitted coupled-time support without enabling terminal
    /// event localization. This is the ordinary 1,800-second Stage-3
    /// transition used before a terminal request is introduced by the parent
    /// controller.
    pub fn evaluate_stage3_persistent_support_without_terminal_event(
        inputs: &DirectActiveSnowPartitionInputs,
        state: &DirectSnowStage3PersistentState,
        lane_id: u32,
        interval_index: u64,
        support: DirectSnowStage3SupportInput,
    ) -> Result<DirectSnowStage3PersistentDayResult, DirectSnowStage3EvaluationError> {
        if !support.duration_seconds.is_finite() || support.duration_seconds <= 0.0 {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_support_duration_seconds",
                support.duration_seconds,
                Some(f64::MIN_POSITIVE),
                None,
            )
            .into());
        }
        Self::validate_stage3_persistent_state(state)?;
        if state.lane_id != lane_id || state.next_interval_index != interval_index {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_persistent_identity_or_order",
                1.0,
                Some(0.0),
                Some(0.0),
            )
            .into());
        }
        Self::evaluate_stage3_persistent_day_internal(
            inputs,
            state,
            lane_id,
            interval_index,
            &[support],
            None,
            None,
            None,
        )
    }

    /// Evaluate one covered support using the exact sensible, vapor, latent,
    /// longwave, and precipitation boundary supplied by the shared carrier.
    /// The ordinary snow surface operator is not re-run for this path.
    pub fn evaluate_stage3_persistent_support_with_boundary(
        inputs: &DirectActiveSnowPartitionInputs,
        state: &DirectSnowStage3PersistentState,
        lane_id: u32,
        interval_index: u64,
        support: DirectSnowStage3SupportInput,
        boundary: Stage3SnowSurfaceBoundaryReceiptV1,
    ) -> Result<DirectSnowStage3PersistentDayResult, DirectSnowStage3EvaluationError> {
        if !support.duration_seconds.is_finite()
            || support.duration_seconds <= 0.0
            || boundary.support.duration_ns() != duration_seconds_to_ns(support.duration_seconds)?
        {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_covered_boundary_support_join",
                support.duration_seconds,
                Some(f64::MIN_POSITIVE),
                None,
            )
            .into());
        }
        Self::validate_stage3_persistent_state(state)?;
        if state.lane_id != lane_id || state.next_interval_index != interval_index {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_persistent_identity_or_order",
                1.0,
                Some(0.0),
                Some(0.0),
            )
            .into());
        }
        Self::evaluate_stage3_persistent_day_internal(
            inputs,
            state,
            lane_id,
            interval_index,
            &[support],
            None,
            Some(boundary),
            None,
        )
    }

    /// Evaluate one covered support with the terminal enthalpy-event operator
    /// enabled and the exact sealed surface boundary used by the coupled solve.
    pub fn evaluate_stage3_terminal_support_with_boundary_v1(
        inputs: &DirectActiveSnowPartitionInputs,
        state: &DirectSnowStage3PersistentState,
        lane_id: u32,
        interval_index: u64,
        support: DirectSnowStage3SupportInput,
        boundary: Stage3SnowSurfaceBoundaryReceiptV1,
    ) -> Result<DirectSnowStage3PersistentDayResult, DirectSnowStage3EvaluationError> {
        if !support.duration_seconds.is_finite()
            || support.duration_seconds <= 0.0
            || boundary.support.duration_ns() != duration_seconds_to_ns(support.duration_seconds)?
        {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_terminal_covered_boundary_support_join",
                support.duration_seconds,
                Some(f64::MIN_POSITIVE),
                None,
            )
            .into());
        }
        Self::validate_stage3_persistent_state(state)?;
        if state.lane_id != lane_id
            || state.next_interval_index != interval_index
            || !stage3_is_terminal_event_domain(state)
        {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_terminal_persistent_identity_or_order",
                1.0,
                Some(0.0),
                Some(0.0),
            )
            .into());
        }
        let _ = (inputs, state, lane_id, interval_index, boundary);
        Err(Self::stage3_domain_error(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            "snow.stage3_terminal_covered_trial_provider_required",
            support.duration_seconds,
            Some(f64::MIN_POSITIVE),
            None,
        )
        .into())
    }

    /// Evaluate terminal chronology through a pure covered-carrier provider.
    /// The provider is invoked independently for every adaptive full/half
    /// trial and every event-root trial with its exact absolute support.
    pub(crate) fn evaluate_stage3_terminal_support_with_trial_provider_v1(
        inputs: &DirectActiveSnowPartitionInputs,
        state: &DirectSnowStage3PersistentState,
        lane_id: u32,
        interval_index: u64,
        support_input: DirectSnowStage3SupportInput,
        support: TimeSupport,
        mode: CoveredTerminalExecutionMode,
        initial_joint: CoveredTerminalJointTrialStateV1,
        provider: &mut CoveredTerminalTrialProviderV1<'_>,
    ) -> Result<DirectSnowStage3PersistentDayResult, DirectSnowStage3EvaluationError> {
        let mut evidence = <NoEvidence as TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>>>::new_state();
        Self::evaluate_stage3_terminal_support_with_trial_provider_and_evidence_v1::<NoEvidence>(
            inputs, state, lane_id, interval_index, support_input, support, mode,
            initial_joint, provider, &mut evidence,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn evaluate_stage3_terminal_support_with_trial_provider_and_evidence_v1<M: TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>>>(
        inputs: &DirectActiveSnowPartitionInputs,
        state: &DirectSnowStage3PersistentState,
        lane_id: u32,
        interval_index: u64,
        support_input: DirectSnowStage3SupportInput,
        support: TimeSupport,
        mode: CoveredTerminalExecutionMode,
        initial_joint: CoveredTerminalJointTrialStateV1,
        provider: &mut CoveredTerminalTrialProviderV1<'_>,
        evidence: &mut M::State,
    ) -> Result<DirectSnowStage3PersistentDayResult, DirectSnowStage3EvaluationError> {
        if mode == CoveredTerminalExecutionMode::PersistentReject
            || !support_input.duration_seconds.is_finite()
            || support_input.duration_seconds <= 0.0
            || support.duration_ns() != duration_seconds_to_ns(support_input.duration_seconds)?
        {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_terminal_covered_execution_mode_or_support",
                support_input.duration_seconds,
                Some(f64::MIN_POSITIVE),
                None,
            )
            .into());
        }
        Self::validate_stage3_persistent_state(state)?;
        if state.lane_id != lane_id
            || state.next_interval_index != interval_index
            || state.schema_version != 2
            || state.terminal_event_model != Some(DirectSnowTerminalEventModel::EnthalpyEventV1)
        {
            return Err(Self::stage3_domain_error(
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "snow.stage3_terminal_persistent_identity_or_model",
                1.0,
                Some(0.0),
                Some(0.0),
            )
            .into());
        }
        let result = Self::evaluate_stage3_persistent_day_internal_with_evidence::<M>(
            inputs,
            state,
            lane_id,
            interval_index,
            &[support_input],
            Some(DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1),
            None,
            Some((support, mode, initial_joint, provider)),
            evidence,
        )?;
        if let CoveredTerminalExecutionMode::ExactEndpoint { expected_tick } = mode {
            let exact = result.terminal_event.as_ref().is_some_and(|event| {
                event.event_occurred
                    && event.unevaluated_seconds.abs() <= 1.0e-6
                    && support.end_ns() == expected_tick
                    && (event.hour_offset_seconds - support_input.duration_seconds).abs() <= 1.0e-6
            });
            if !exact {
                return Err(Self::stage3_domain_error(
                    HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                    "snow.stage3_terminal_exact_endpoint",
                    result.evaluation.evaluated_seconds,
                    Some(support_input.duration_seconds),
                    Some(support_input.duration_seconds),
                )
                .into());
            }
        }
        Ok(result)
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn duration_seconds_to_ns(seconds: f64) -> Result<u128, DirectSnowStage3EvaluationError> {
    let nanos = seconds * 1_000_000_000.0;
    if !nanos.is_finite() || nanos < 0.0 || nanos.fract() != 0.0 {
        return Err(Wb11HydrologyKernel::stage3_domain_error(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            "snow.stage3_covered_boundary_support_nanoseconds",
            seconds,
            Some(f64::MIN_POSITIVE),
            None,
        )
        .into());
    }
    Ok(nanos as u128)
}

#[cfg(test)]
mod batch_prefix_tests {
    use super::*;

    fn owners() -> BTreeMap<String, Vec<u8>> {
        BTreeMap::from([
            ("vegetation".to_owned(), vec![1]),
            ("snow".to_owned(), vec![2]),
            ("land_surface_energy".to_owned(), vec![3]),
            ("hydrology".to_owned(), vec![4]),
            ("bgc".to_owned(), vec![5]),
            ("soil_thermal".to_owned(), vec![6]),
            ("surface_liquid".to_owned(), vec![7]),
        ])
    }

    fn authority() -> JointTrialAuthorityV1 {
        JointTrialAuthorityV1 {
            source_owner_set_sha256: Digest32::from_bytes([8; 32]),
            lane_id: 1,
            source_snow_owner_sha256: Digest32::from_bytes([9; 32]),
            interval_index: 0,
            state_support: TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(10)).unwrap(),
            accepted_predecessors: Vec::new(),
        }
    }

    fn joint() -> CoveredTerminalJointTrialStateV1 {
        CoveredTerminalJointTrialStateV1::try_new(authority(), owners()).unwrap()
    }

    fn request(ticks: [Option<u128>; 3]) -> CoveredTerminalBatchTrialRequestV2 {
        CoveredTerminalBatchTrialRequestV2 {
            support: TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(10)).unwrap(),
            role: CoveredTerminalTrialRoleV1::Root,
            attempt_ordinal: 0,
            lanes: ticks.into_iter().enumerate().map(|(index, tick)| {
                let lane_id = index as u32 + 1;
                (lane_id, CoveredTerminalLaneTrialStateV2 {
                    lane_id,
                    ice_kg_m2: 1.0,
                    liquid_kg_m2: 0.0,
                    cold_content_j_m2: 0.0,
                    surface_temperature_c: 0.0,
                    snow_depth_m: 0.01,
                    snow_density_kg_m3: 100.0,
                    resolved_beginning: index == 2,
                    candidate_event_tick: tick.map(ModelTimeNs::new),
                })
            }).collect(),
            beginning_joint: joint(),
        }
    }

    #[test]
    fn joint_authority_poisoning_changes_wire_identity_and_duplicate_predecessors_fail() {
        let baseline = joint();
        assert_eq!(baseline.receipt_sha256(), joint().receipt_sha256());
        let mut poisons = Vec::new();
        let mut poison = authority();
        poison.source_owner_set_sha256 = Digest32::from_bytes([10; 32]);
        poisons.push(poison);
        let mut poison = authority();
        poison.lane_id = 2;
        poisons.push(poison);
        let mut poison = authority();
        poison.source_snow_owner_sha256 = Digest32::from_bytes([11; 32]);
        poisons.push(poison);
        let mut poison = authority();
        poison.interval_index = 1;
        poisons.push(poison);
        let mut poison = authority();
        poison.state_support =
            TimeSupport::new(ModelTimeNs::new(1), ModelTimeNs::new(10)).unwrap();
        poisons.push(poison);
        let mut poison = authority();
        poison.accepted_predecessors = vec![Digest32::from_bytes([12; 32])];
        poisons.push(poison);
        for poison in poisons {
            let poisoned =
                CoveredTerminalJointTrialStateV1::try_new(poison, owners()).unwrap();
            assert_ne!(baseline.receipt_sha256(), poisoned.receipt_sha256());
        }
        let mut poisoned_owners = owners();
        poisoned_owners.insert("hydrology".to_owned(), vec![44]);
        assert_ne!(
            baseline.receipt_sha256(),
            CoveredTerminalJointTrialStateV1::try_new(authority(), poisoned_owners)
                .unwrap()
                .receipt_sha256()
        );

        let duplicate = Digest32::from_bytes([3; 32]);
        let mut duplicate_authority = authority();
        duplicate_authority.accepted_predecessors = vec![duplicate, duplicate];
        assert!(CoveredTerminalJointTrialStateV1::try_new(duplicate_authority, owners()).is_err());
    }

    #[test]
    fn accepted_hydrology_state_preserves_authority_and_appends_one_predecessor() {
        let beginning = joint();
        let ending = beginning
            .with_terminal_hydrology_state(1, 0.5, 0.1, 12.0)
            .unwrap();
        assert_eq!(
            ending.authority().source_owner_set_sha256,
            beginning.authority().source_owner_set_sha256
        );
        assert_eq!(ending.authority().lane_id, beginning.authority().lane_id);
        assert_eq!(
            ending.authority().source_snow_owner_sha256,
            beginning.authority().source_snow_owner_sha256
        );
        assert_eq!(
            ending.authority().state_support,
            beginning.authority().state_support
        );
        assert_eq!(ending.authority().accepted_predecessors.len(), 1);
    }

    #[test]
    fn probe_child_identity_binds_every_authority_operand_and_support() {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(10)).unwrap();
        let trial = TimeSupport::new(ModelTimeNs::new(2), ModelTimeNs::new(8)).unwrap();
        let base = ProbeChildAuthorityV1 {
            parent_transaction_sha256: Digest32::from_bytes([1; 32]),
            enclosing_parent_support: support,
            trial_support: trial,
            physical_child_ordinal: 2,
            attempt_ordinal: 3,
            role: CoveredTerminalTrialRoleV1::Root,
            beginning_joint_sha256: Digest32::from_bytes([2; 32]),
            beginning_owner_set_sha256: Digest32::from_bytes([3; 32]),
            complete_forcing_sha256: Digest32::from_bytes([4; 32]),
            topology_sha256: Digest32::from_bytes([5; 32]),
        };
        let baseline = CoveredProbeChildIdentityV1::try_new(base).unwrap();
        assert_eq!(
            baseline.receipt_sha256,
            CoveredProbeChildIdentityV1::try_new(base)
                .unwrap()
                .receipt_sha256
        );
        let mut poisons = Vec::new();
        let mut poison = base;
        poison.parent_transaction_sha256 = Digest32::from_bytes([6; 32]);
        poisons.push(poison);
        let mut poison = base;
        poison.physical_child_ordinal += 1;
        poisons.push(poison);
        let mut poison = base;
        poison.attempt_ordinal += 1;
        poisons.push(poison);
        let mut poison = base;
        poison.role = CoveredTerminalTrialRoleV1::BracketUpper;
        poisons.push(poison);
        let mut poison = base;
        poison.beginning_joint_sha256 = Digest32::from_bytes([7; 32]);
        poisons.push(poison);
        let mut poison = base;
        poison.beginning_owner_set_sha256 = Digest32::from_bytes([8; 32]);
        poisons.push(poison);
        let mut poison = base;
        poison.complete_forcing_sha256 = Digest32::from_bytes([9; 32]);
        poisons.push(poison);
        let mut poison = base;
        poison.topology_sha256 = Digest32::from_bytes([10; 32]);
        poisons.push(poison);
        for poison in poisons {
            assert_ne!(
                baseline.receipt_sha256,
                CoveredProbeChildIdentityV1::try_new(poison)
                    .unwrap()
                    .receipt_sha256
            );
        }
        let mut outside = base;
        outside.trial_support =
            TimeSupport::new(ModelTimeNs::new(9), ModelTimeNs::new(11)).unwrap();
        assert!(CoveredProbeChildIdentityV1::try_new(outside).is_err());
    }

    #[test]
    fn same_tick_lanes_are_one_ordered_common_group() {
        let request = request([Some(6), Some(6), None]);
        assert_eq!(
            Wb11HydrologyKernel::covered_terminal_batch_common_earliest_lanes_v2(&request),
            Some((ModelTimeNs::new(6), vec![1, 2]))
        );
    }

    #[test]
    fn different_ticks_select_only_common_earliest_and_preserve_survivors() {
        let request = request([Some(8), Some(5), None]);
        assert_eq!(
            Wb11HydrologyKernel::covered_terminal_batch_common_earliest_lanes_v2(&request),
            Some((ModelTimeNs::new(5), vec![2]))
        );
        assert!(request.lanes[&3].resolved_beginning);
        assert_eq!(request.beginning_joint.owner_bytes()["snow"], vec![2]);
    }
}
