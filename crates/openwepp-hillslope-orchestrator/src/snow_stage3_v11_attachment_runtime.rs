#[inline(never)]
fn stage3_boxed_execution_v1<T, E>(execute: impl FnOnce() -> Result<T, E>) -> Result<Box<T>, E> {
    execute().map(Box::new)
}

impl DirectSnowStage3V11ShadowAttachment {
    pub fn restart_authority_snow_enthalpy_material_residents_v1(
        &self,
    ) -> Result<
        SnowStage3V11SnowEnthalpyMaterialResidentSetV1,
        DirectSnowStage3V11AttachmentError,
    > {
        Ok(SnowStage3V11SnowEnthalpyMaterialResidentSetV1 {
            committed: snow_enthalpy_material_resident_from_committed_v1(&self.committed)?,
            pending_candidate: self
                .pending_candidate
                .as_ref()
                .map(|candidate| {
                    snow_enthalpy_material_resident_from_committed_v1(&candidate.ending_state)
                })
                .transpose()?,
            in_progress_day_candidate: self
                .in_progress_execution
                .as_ref()
                .map(|execution| {
                    snow_enthalpy_material_resident_from_committed_v1(&execution.day_candidate)
                })
                .transpose()?,
            in_progress_support_current: self
                .in_progress_execution
                .as_ref()
                .and_then(|execution| execution.support_current.as_ref())
                .map(snow_enthalpy_material_resident_from_committed_v1)
                .transpose()?,
        })
    }

    pub fn restart_authority_install_snow_enthalpy_material_residents_v1(
        &mut self,
        residents: SnowStage3V11SnowEnthalpyMaterialResidentSetV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if residents.pending_candidate.is_some() != self.pending_candidate.is_some()
            || residents.in_progress_day_candidate.is_some()
                != self.in_progress_execution.is_some()
            || residents.in_progress_support_current.is_some()
                != self
                    .in_progress_execution
                    .as_ref()
                    .is_some_and(|execution| execution.support_current.is_some())
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "V56 restart material resident posture",
            ));
        }
        let mut committed = self.committed.clone();
        install_snow_enthalpy_material_resident_into_committed_v1(
            &mut committed,
            residents.committed,
        )?;
        let mut pending_candidate = self.pending_candidate.clone();
        if let (Some(candidate), Some(resident)) =
            (pending_candidate.as_mut(), residents.pending_candidate)
        {
            install_snow_enthalpy_material_resident_into_committed_v1(
                &mut candidate.ending_state,
                resident,
            )?;
        }
        let mut in_progress_execution = self.in_progress_execution.clone();
        if let Some(execution) = in_progress_execution.as_mut() {
            install_snow_enthalpy_material_resident_into_committed_v1(
                &mut execution.day_candidate,
                residents.in_progress_day_candidate.ok_or(
                    DirectSnowStage3V11AttachmentError::Identity(
                        "V56 restart in-progress day resident",
                    ),
                )?,
            )?;
            if let Some(current) = execution.support_current.as_mut() {
                install_snow_enthalpy_material_resident_into_committed_v1(
                    current,
                    residents.in_progress_support_current.ok_or(
                        DirectSnowStage3V11AttachmentError::Identity(
                            "V56 restart in-progress current resident",
                        ),
                    )?,
                )?;
            }
        }
        self.committed = committed;
        self.pending_candidate = pending_candidate;
        self.in_progress_execution = in_progress_execution;
        Ok(())
    }

    /// Restart projection guard: exact V4 custody anywhere in the retained
    /// Stage-3 transaction graph prohibits the legacy hydrology wire.
    #[cfg(feature = "persisted-restart-v1")]
    #[must_use]
    pub fn restart_authority_contains_frozen_litter_v4(&self) -> bool {
        self.committed
            .real_consumer
            .frozen_litter_v4_resident()
            .is_some()
            || self.pending_candidate.as_ref().is_some_and(|candidate| {
                candidate
                    .ending_state
                    .real_consumer
                    .frozen_litter_v4_resident()
                    .is_some()
            })
            || self.in_progress_execution.as_deref().is_some_and(|execution| {
                execution
                    .day_candidate()
                    .real_consumer
                    .frozen_litter_v4_resident()
                    .is_some()
                    || execution.support_current().is_some_and(|current| {
                        current.real_consumer.frozen_litter_v4_resident().is_some()
                    })
            })
    }

    /// Prepare one repository-provider day from the live owners retained by
    /// the committed V10 consumer. This is deliberately attachment-owned:
    /// after production initialization the runner no longer owns the GSI
    /// state or forcing cursor and cannot truthfully reconstruct either one.
    pub fn prepare_repository_provider_day(
        &self,
        climate: &HillslopeClimateRuntimeRequest,
        day_index: usize,
    ) -> Result<PreparedSnowFreeGsiDayV1, DirectSnowStage3V11AttachmentError> {
        let consumer = &self.committed.real_consumer;
        Ok(climate.prepare_snow_free_gsi_day_from_repository(
            day_index,
            consumer.provider_static_configuration(),
            consumer.gsi_owner_configuration(),
            consumer.gsi_state(),
            consumer.provider_cursor(),
        )?)
    }

    pub(crate) fn prepare_repository_v11_intervals(
        &self,
        provider: &PreparedSnowFreeGsiDayV1,
        template: &DirectV9ShadowIntervalInput,
    ) -> Result<
        Vec<(
            DirectV9ShadowIntervalInput,
            DirectV11SnowCoveredSegmentInput,
        )>,
        DirectSnowStage3V11AttachmentError,
    > {
        self.committed
            .real_consumer
            .prepare_v11_intervals_from_repository(provider, template)
            .map_err(DirectSnowStage3V11AttachmentError::Owner)
    }

    /// Construct the production attachment from physical beginning owners.
    /// The caller supplies no V11 parent transaction, coupled clock, owner
    /// envelope, participant list, or sequence graph; all are derived here
    /// from the live V10 consumer and canonical Stage-3 state.
    pub fn new_production(
        configuration: DirectSnowStage3V11ProductionConfigurationV1,
        stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
        real_consumer: DirectV10RealConsumerShadow,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        if configuration.run_identity == Digest32::zero()
            || configuration.topology_identity == Digest32::zero()
            || configuration.calendar_receipt == Digest32::zero()
            || configuration.controller_policy == Digest32::zero()
            || stage3_by_lane.is_empty()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "production configuration authority or Stage-3 lane set",
            ));
        }
        let lane_ids = stage3_by_lane.keys().copied().collect::<Vec<_>>();
        if lane_ids.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "production Stage-3 lane order",
            ));
        }
        let migration = migrate_v10_runtime_to_v11(
            real_consumer.vegetation_configuration(),
            real_consumer.vegetation_state(),
        )?;
        let mut owner_bytes = real_consumer.canonical_v11_parent_owner_state_bytes()?;
        owner_bytes.insert(
            "snow".to_owned(),
            canonical_stage3_snow_owner_bytes(&stage3_by_lane)?,
        );
        let manifest = openwepp_vegetation::v11::V11_COMPLETE_OWNER_MANIFEST;
        if owner_bytes.len() != manifest.len()
            || owner_bytes
                .keys()
                .any(|owner| !manifest.contains(&owner.as_str()))
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "production complete owner manifest",
            ));
        }
        let owner_envelopes = manifest
            .iter()
            .map(|owner_id| {
                let bytes = owner_bytes.remove(*owner_id).ok_or(
                    DirectSnowStage3V11AttachmentError::Identity("production complete owner bytes"),
                )?;
                let envelope = if *owner_id == "vegetation" {
                    let _ = bytes;
                    v11_vegetation_owner_envelope(&migration.state)?
                } else {
                    V11OwnerEnvelope::try_new((*owner_id).to_owned(), bytes)?
                };
                Ok(((*owner_id).to_owned(), envelope))
            })
            .collect::<Result<BTreeMap<_, _>, DirectSnowStage3V11AttachmentError>>()?;
        let owner_states = owner_states_from_envelopes(&owner_envelopes)?;
        let beginning_owner_digest = complete_owner_set_digest(&owner_states)?;
        let day_index = real_consumer.v11_next_day_index();
        let day_start = day_start_ns(day_index)?;
        let support = TimeSupport::new(
            ModelTimeNs::new(day_start),
            ModelTimeNs::new(day_start.checked_add(STAGE3_V11_PARENT_SUPPORT_NS).ok_or(
                DirectSnowStage3V11AttachmentError::Support("production initial support overflow"),
            )?),
        )?;
        let initial_forcing_receipt = framed_sha256(
            "stage3-v11-production-initial-owner-v1",
            &[
                FramedField {
                    tag: "run",
                    value: configuration.run_identity.as_bytes(),
                },
                FramedField {
                    tag: "topology",
                    value: configuration.topology_identity.as_bytes(),
                },
                FramedField {
                    tag: "owners",
                    value: beginning_owner_digest.as_bytes(),
                },
            ],
        )?;
        let next_parent_sequence = u128::try_from(day_index)
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "production beginning day sequence width",
                )
            })?
            .checked_mul(
                u128::try_from(STAGE3_V11_PARENT_SUPPORT_COUNT).map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity("production support count width")
                })?,
            )
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "production beginning parent sequence overflow",
            ))?;
        let authority = ParentAuthorityV1::new(
            configuration.run_identity,
            configuration.calendar_receipt,
            initial_forcing_receipt,
            next_parent_sequence,
            support,
            beginning_owner_digest,
        )?;
        let mut participants = manifest
            .iter()
            .map(|owner| (*owner).to_owned())
            .collect::<Vec<_>>();
        participants.sort();
        let clock = CoupledClockStateV1::new(
            authority,
            owner_states,
            "snow-stage3-v11".to_owned(),
            participants,
            configuration.controller_policy,
            Vec::new(),
        )?;
        let parent = V11ParentTransaction::new_with_complete_owners(
            &migration.configuration,
            &migration.state,
            clock.parent_transaction_id(),
            support.start_ns(),
            owner_envelopes,
        )?;
        let static_context = DirectSnowStage3V11StaticContext {
            run_identity: configuration.run_identity,
            topology_identity: configuration.topology_identity,
            parent_duration_ns: STAGE3_V11_PARENT_SUPPORT_NS,
            minimum_support_ns: STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS,
            calendar_receipt: configuration.calendar_receipt,
            controller_policy: configuration.controller_policy,
            parent_sequence: next_parent_sequence,
            lane_ids,
            vegetation_configuration: migration.configuration,
            surface_liquid_configuration: configuration.surface_liquid_configuration,
            wb14_parameters: configuration.wb14_parameters,
        };
        Self::new(
            static_context,
            DirectSnowStage3V11CommittedState {
                stage3_by_lane,
                real_consumer,
                v11_parent_state: parent,
                coupled_clock: clock,
                next_parent_sequence,
                last_v11_parent_candidate: None,
                terminal_parcels: BTreeMap::new(),
                receipt_chain: Vec::new(),
                snow_enthalpy_material_owner: None,
                snow_enthalpy_material_owner_chronology: Vec::new(),
            },
        )
    }

    pub fn new(
        static_context: DirectSnowStage3V11StaticContext,
        committed: DirectSnowStage3V11CommittedState,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        static_context.validate()?;
        if committed.stage3_by_lane.len() != static_context.lane_ids.len()
            || committed
                .stage3_by_lane
                .keys()
                .copied()
                .collect::<BTreeSet<_>>()
                != static_context
                    .lane_ids
                    .iter()
                    .copied()
                    .collect::<BTreeSet<_>>()
            || committed.v11_parent_state.parent_transaction_id().digest() == Digest32::zero()
            || committed.coupled_clock.parent_transaction_id()
                != committed.v11_parent_state.parent_transaction_id()
            || committed.coupled_clock.parent_support().duration_ns()
                != static_context.parent_duration_ns
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "complete committed lane and V11 parent state",
            ));
        }
        let canonical_snow = canonical_stage3_snow_owner_bytes_with_pending(
            &committed.stage3_by_lane,
            &committed.terminal_parcels,
        )?;
        let parent_snow = committed
            .v11_parent_state
            .staged_resource_owners()
            .get("snow")
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "missing canonical Stage-3 snow owner",
            ))?;
        let clock_snow = committed
            .coupled_clock
            .owners()
            .iter()
            .find(|owner| owner.owner_id() == "snow")
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "missing coupled-time Stage-3 snow owner",
            ))?;
        if parent_snow.state_bytes != canonical_snow
            || clock_snow.state_bytes() != canonical_snow.as_slice()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "hydrology and Stage-3 snow owners are not exact-one custody",
            ));
        }
        let archived_receipt_prefix = Stage3ArchivedReceiptPrefixV1::empty(
            static_context.run_identity,
            static_context.topology_identity,
            committed.next_parent_sequence,
        )?;
        Ok(Self {
            static_context,
            committed,
            archived_receipt_prefix,
            pending_committed_day_evidence: None,
            pending_candidate: None,
            pending_publication_day: None,
            committed_publication_day: None,
            in_progress_execution: None,
            failure_injection: None,
        })
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn inject_failure_after_subslab(&mut self, ordinal: usize) {
        self.failure_injection = Some(Stage3V11FailureInjection::SubslabAccepted(ordinal));
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn inject_failure_after_outcome_ledger(&mut self, ordinal: usize) {
        self.failure_injection = Some(Stage3V11FailureInjection::OutcomeLedgerBuilt(ordinal));
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn inject_failure_after_final_owner_join(&mut self) {
        self.failure_injection = Some(Stage3V11FailureInjection::FinalOwnerJoinCompleted);
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) const fn pending_candidate_is_none(&self) -> bool {
        self.pending_candidate.is_none()
    }

    pub fn stage_prepared_day(
        &mut self,
        prepared: &ValidatedPreparedStage3V11DayV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.pending_candidate.is_some()
            || self.pending_publication_day.is_some()
            || self.pending_committed_day_evidence.is_some()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "duplicate uncommitted or unarchived Stage-3/V11 parent",
            ));
        }
        if self.in_progress_execution.is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restartable Stage-3/V11 parent is already in progress",
            ));
        }
        let candidate = self.execute_prepared_day(prepared)?;
        self.pending_candidate = Some(candidate);
        Ok(())
    }

    pub fn commit_staged_day(&mut self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.in_progress_execution.is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restartable Stage-3/V11 parent is still in progress",
            ));
        }
        let candidate =
            self.pending_candidate
                .take()
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "missing staged Stage-3/V11 parent",
                ))?;
        let publication_day = self.pending_publication_day.take().ok_or(
            DirectSnowStage3V11AttachmentError::Identity(
                "missing completed Stage-3 publication day",
            ),
        )?;
        self.install_candidate_with_publication(candidate, publication_day)
    }

    pub(crate) fn pending_publication_completion_inputs(
        &self,
        day_index: usize,
    ) -> Result<
        (
            crate::direct_runtime::DirectRunFrame,
            Vec<crate::v9_real_consumer_shadow::Stage3AcceptedPublicationSupportV1>,
            Vec<openwepp_coupled_time::AcceptedEventReceiptV1>,
            Vec<Stage3V11TerminalEventGroupV1>,
            Vec<Stage3CoupledSubslabReceiptV1>,
            BTreeMap<u32, DirectSnowStage3PersistentState>,
            BTreeMap<u32, DirectSnowStage3PersistentState>,
            DirectSurfaceLiquidConfiguration,
        ),
        DirectSnowStage3V11AttachmentError,
    > {
        let candidate =
            self.pending_candidate
                .as_ref()
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "missing staged candidate for publication completion",
                ))?;
        if candidate.parent_receipt.day_index != day_index || self.pending_publication_day.is_some()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "pending publication completion identity",
            ));
        }
        let supports = candidate
            .ending_state
            .real_consumer
            .accepted_publication_supports_for_day(day_index)?
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();
        let day_start_ns = u128::try_from(day_index)
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "publication completion day-index width",
                )
            })?
            .checked_mul(STAGE3_V11_DAY_NS)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "publication completion day-start overflow",
            ))?;
        let day_end_ns = day_start_ns.checked_add(STAGE3_V11_DAY_NS).ok_or(
            DirectSnowStage3V11AttachmentError::Identity(
                "publication completion day-end overflow",
            ),
        )?;
        let first_parent = supports
            .first()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "missing accepted publication support for event selection",
            ))?
            .parent_transaction_id();
        let event_handoffs = candidate
            .ending_state
            .real_consumer
            .accepted_publication_event_handoffs()
            .iter()
            .filter(|event| {
                let tick = event.tick().get();
                (tick > day_start_ns && tick <= day_end_ns)
                    || (tick == day_start_ns && event.parent_transaction_id() == first_parent)
            })
            .cloned()
            .collect();
        Ok((
            candidate
                .ending_state
                .real_consumer
                .hydrology_frame()
                .clone(),
            supports,
            event_handoffs,
            candidate.parent_receipt.terminal_event_groups.clone(),
            candidate.parent_receipt.coupled_subslabs.clone(),
            self.committed.stage3_by_lane.clone(),
            candidate.ending_state.stage3_by_lane.clone(),
            self.static_context.surface_liquid_configuration.clone(),
        ))
    }

    pub(crate) fn complete_pending_publication_day(
        &mut self,
        publication_day: crate::direct_runtime::Stage3AcceptedPublicationDayV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let candidate =
            self.pending_candidate
                .as_mut()
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "missing staged candidate for publication installation",
                ))?;
        publication_day
            .validate_for_install(
                candidate.parent_receipt.day_index,
                self.static_context.lane_ids.len(),
                candidate.parent_receipt.ending_coupled_owner_set_sha256,
            )
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "pending publication capability installation",
                )
            })?;
        self.pending_publication_day = Some(publication_day);
        Ok(())
    }

    pub(crate) fn committed_publication_day(
        &self,
        day_index: usize,
    ) -> Result<
        &crate::direct_runtime::Stage3AcceptedPublicationDayV1,
        DirectSnowStage3V11AttachmentError,
    > {
        let publication = self.committed_publication_day.as_ref().ok_or(
            DirectSnowStage3V11AttachmentError::Identity(
                "missing committed publication capability",
            ),
        )?;
        if publication.day_index() != day_index {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "committed publication day identity",
            ));
        }
        Ok(publication)
    }

    /// Execute all 48 actual Stage-3 transitions atomically.  Terminal
    /// candidates are rerun against the actual Stage-3 support evaluator; no
    /// rate projection or completed production day frame is consulted.
    #[allow(clippy::too_many_lines)]
    pub fn execute_prepared_day(
        &self,
        prepared: &ValidatedPreparedStage3V11DayV1,
    ) -> Result<DirectSnowStage3V11ParentCandidate, DirectSnowStage3V11AttachmentError> {
        let PreparedDayExecutionOutcomeV2::Complete(candidate) =
            self.execute_prepared_day_resumable_v2(prepared, None, None)?
        else {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "unexpected prepared-day restart interruption",
            ));
        };
        Ok(candidate)
    }

    #[allow(clippy::too_many_lines)]
    fn execute_prepared_day_resumable_v2(
        &self,
        prepared: &ValidatedPreparedStage3V11DayV1,
        restart: Option<Box<DirectSnowStage3V11InProgressExecutionV2>>,
        interrupt_at: Option<DirectSnowStage3V11InterruptionPostureV2>,
    ) -> Result<PreparedDayExecutionOutcomeV2, DirectSnowStage3V11AttachmentError> {
        prepared.validate(&self.static_context, day_start_ns(prepared.day_index())?)?;
        validate_prepared_day_against_committed_provider(&self.committed, prepared)?;
        let mut restart = restart;
        if let Some(checkpoint) = restart.as_ref() {
            checkpoint.validate(&self.static_context, &self.committed)?;
            checkpoint.validate_prepared_day(prepared)?;
        }
        let (
            mut candidate,
            mut terminal_events,
            mut terminal_event_groups,
            mut covered_owner_joins,
            mut coupled_subslabs,
            mut adaptive_support_receipts,
            mut snow_free_successor_receipts,
            mut support_index,
        ) = restart.as_ref().map_or_else(
            || {
                (
                    self.committed.clone(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    0,
                )
            },
            |checkpoint| {
                (
                    checkpoint.day_candidate.clone(),
                    checkpoint.terminal_events.clone(),
                    checkpoint.terminal_event_groups.clone(),
                    checkpoint.covered_owner_joins.clone(),
                    checkpoint.coupled_subslabs.clone(),
                    checkpoint.adaptive_support_receipts.clone(),
                    checkpoint.snow_free_successor_receipts.clone(),
                    checkpoint.support_index,
                )
            },
        );
        while support_index < prepared.supports().len() {
            let mut live_support = prepared.supports()[support_index].clone();
            live_support
                .bind_live_owner_transaction(candidate.real_consumer.next_transaction_id()?)?;
            let support = &live_support;
            let beginning_stage3 = candidate.stage3_by_lane.clone();
            let active_snow_lanes = support.state_derived_active_snow_lanes(
                &beginning_stage3,
                &candidate.terminal_parcels,
            )?;
            let covered_support = !active_snow_lanes.is_empty();
            if !covered_support {
                for lane_id in &self.static_context.lane_ids {
                    let inputs = support.snow_inputs_by_lane.get(lane_id).ok_or(
                        DirectSnowStage3V11AttachmentError::Support("missing lane support input"),
                    )?;
                    let support_forcing = support
                        .support_forcing_by_lane
                        .get(lane_id)
                        .copied()
                        .ok_or(DirectSnowStage3V11AttachmentError::Support(
                            "missing sealed support forcing",
                        ))?;
                    let state = candidate.stage3_by_lane.get(lane_id).ok_or(
                        DirectSnowStage3V11AttachmentError::Identity(
                            "missing committed Stage-3 lane",
                        ),
                    )?;
                    let result = Wb11HydrologyKernel::evaluate_stage3_persistent_support(
                        inputs,
                        state,
                        *lane_id,
                        state.next_interval_index,
                        support_forcing,
                        DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
                    )?;
                    let (ending, event) = if let Some(event) = result.terminal_event {
                        let selected = select_actual_terminal_candidate(
                            inputs,
                            state,
                            *lane_id,
                            state.next_interval_index,
                            support,
                            support_forcing,
                            event,
                            self.static_context.minimum_support_ns,
                        )?;
                        let ending = selected.1.state.clone();
                        (ending, Some(selected.0))
                    } else {
                        (result.state, None)
                    };
                    candidate.stage3_by_lane.insert(*lane_id, ending.clone());
                    if let Some(event) = event {
                        terminal_events.push(event);
                    }
                }
            }

            let forcing_receipt = canonical_parent_forcing_digest(
                prepared.day_index(),
                support_index,
                prepared.accepted_gsi_receipt(),
                support,
            )?;
            let (beginning_parent, beginning_clock) = begin_v11_parent_for_support(
                &self.static_context,
                &candidate,
                support,
                forcing_receipt,
                candidate.next_parent_sequence,
            )?;
            let (
                parent,
                consumer,
                clock,
                finalized,
                covered_stage3,
                covered_snow_enthalpy_material_owner,
                covered_snow_enthalpy_material_owner_chronology,
            ) = if covered_support {
                let support_restart = if restart.is_some() {
                    restart.take()
                } else if let Some(posture) = interrupt_at {
                    let prepared_supports = (0..prepared.supports().len())
                        .map(|index| {
                            DirectSnowStage3V11PreparedSupportRestartV2::project(prepared, index)
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    Some(Box::new(DirectSnowStage3V11InProgressExecutionV2 {
                        day_candidate: candidate.clone(),
                        support_current: Some(DirectSnowStage3V11CommittedState {
                            stage3_by_lane: beginning_stage3.clone(),
                            real_consumer: candidate.real_consumer.clone(),
                            v11_parent_state: beginning_parent.clone(),
                            coupled_clock: beginning_clock.clone(),
                            next_parent_sequence: candidate.next_parent_sequence,
                            last_v11_parent_candidate: candidate.last_v11_parent_candidate.clone(),
                            terminal_parcels: candidate.terminal_parcels.clone(),
                            receipt_chain: candidate.receipt_chain.clone(),
                            snow_enthalpy_material_owner:
                                candidate.snow_enthalpy_material_owner.clone(),
                            snow_enthalpy_material_owner_chronology:
                                candidate.snow_enthalpy_material_owner_chronology.clone(),
                        }),
                        day_index: prepared.day_index(),
                        support_index,
                        prepared_supports,
                        terminal_events: terminal_events.clone(),
                        terminal_event_groups: terminal_event_groups.clone(),
                        covered_owner_joins: covered_owner_joins.clone(),
                        coupled_subslabs: coupled_subslabs.clone(),
                        adaptive_support_receipts: adaptive_support_receipts.clone(),
                        snow_free_successor_receipts: snow_free_successor_receipts.clone(),
                        posture,
                        support_owner_joins: Vec::new(),
                        support_event_groups: Vec::new(),
                        support_terminal_parcels: Vec::new(),
                        expected_child_beginning: complete_owner_set_digest(
                            beginning_clock.owners(),
                        )?,
                        pending_adaptive_request: None,
                        adaptive_receipts: AdaptiveReceiptAccumulatorV1::default(),
                        support_snow_free_successor_receipts: Vec::new(),
                        adaptive_trial_quanta: adaptive_test_initial_quanta(
                            support.support.duration_ns() / STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS,
                        ),
                    }))
                } else {
                    None
                };
                let mut evidence =
                    <crate::hydrology::NoEvidence as crate::hydrology::TerminalEvidenceMode<
                        Option<CoveredTerminalJointTrialStateV1>,
                    >>::new_state();
                let support_outcome =
                    execute_covered_real_v11_parent_with_evidence::<crate::hydrology::NoEvidence>(
                        &self.static_context,
                        &beginning_parent,
                        &candidate.real_consumer,
                        &beginning_clock,
                        support,
                        prepared.day_index(),
                        support_index,
                        forcing_receipt,
                        beginning_stage3,
                        candidate.snow_enthalpy_material_owner.clone(),
                        candidate.snow_enthalpy_material_owner_chronology.clone(),
                        candidate.terminal_parcels.clone(),
                        self.failure_injection,
                        &mut evidence,
                        support_restart,
                        interrupt_at,
                    )?;
                let (
                    parent,
                    consumer,
                    clock,
                    finalized,
                    ending_stage3,
                    ending_snow_enthalpy_material_owner,
                    ending_snow_enthalpy_material_owner_chronology,
                    owner_joins,
                    support_event_groups,
                    support_terminal_parcels,
                    adaptive_support_receipt,
                    support_snow_free_successor_receipts,
                ) = match support_outcome {
                    AdaptiveSupportExecutionOutcomeV2::Complete(complete) => complete,
                    AdaptiveSupportExecutionOutcomeV2::Paused(checkpoint) => {
                        return Ok(PreparedDayExecutionOutcomeV2::Paused(checkpoint));
                    }
                };
                let mut same_support_produced = BTreeMap::new();
                for group in &support_event_groups {
                    let accepted = group.accepted_event_receipt.as_ref().ok_or(
                        DirectSnowStage3V11AttachmentError::Identity(
                            "terminal parcel predecessor accepted event",
                        ),
                    )?;
                    let proposal_core = group.proposal_core_sha256.ok_or(
                        DirectSnowStage3V11AttachmentError::Identity(
                            "terminal parcel predecessor proposal core",
                        ),
                    )?;
                    for digest in &group.produced_unconsumed_parcel_digests {
                        if same_support_produced
                            .insert(*digest, (accepted.ordinal(), proposal_core, group))
                            .is_some()
                        {
                            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                                "duplicate same-support produced parcel identity",
                            ));
                        }
                    }
                }
                let mut same_support_consumed = BTreeSet::new();
                for parcel in support_terminal_parcels {
                    match parcel.posture {
                        DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed => {
                            if candidate
                                .terminal_parcels
                                .insert(parcel.parcel_digest, parcel)
                                .is_some()
                            {
                                return Err(DirectSnowStage3V11AttachmentError::Terminal(
                                    "duplicate terminal parcel identity",
                                ));
                            }
                        }
                        DirectSnowStage3V11TerminalParcelPosture::Consumed => {
                            if candidate
                                .terminal_parcels
                                .remove(&parcel.parcel_digest)
                                .is_none()
                            {
                                let (event_ordinal, proposal_core, group) =
                                    same_support_produced.get(&parcel.parcel_digest).ok_or(
                                        DirectSnowStage3V11AttachmentError::Terminal(
                                            "terminal parcel consumed without pending predecessor",
                                        ),
                                    )?;
                                let mut produced_predecessor = parcel.clone();
                                produced_predecessor.posture =
                                    DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed;
                                if parcel.event_ordinal != *event_ordinal
                                    || parcel.terminal_event_proposal_core_id != *proposal_core
                                    || crate::snow_owner_v4::canonical_terminal_parcel_digest(
                                        &produced_predecessor,
                                    )
                                    .ok()
                                        != Some(parcel.parcel_digest)
                                    || !group.candidates.iter().any(|candidate| {
                                        candidate.lane_id == parcel.source_lane_id
                                            && candidate.event_result_digest
                                                == parcel.event_result_digest
                                    })
                                    || !same_support_consumed.insert(parcel.parcel_digest)
                                {
                                    return Err(DirectSnowStage3V11AttachmentError::Terminal(
                                        "same-support terminal parcel predecessor substitution",
                                    ));
                                }
                            }
                        }
                    }
                }
                if same_support_produced.keys().any(|digest| {
                    !same_support_consumed.contains(digest)
                        && !candidate.terminal_parcels.contains_key(digest)
                }) {
                    return Err(DirectSnowStage3V11AttachmentError::Terminal(
                        "same-support terminal parcel predecessor disposition",
                    ));
                }
                terminal_event_groups.extend(support_event_groups);
                covered_owner_joins
                    .extend(owner_joins.iter().map(|receipt| receipt.owner_join.clone()));
                coupled_subslabs.extend(owner_joins);
                adaptive_support_receipts.push(adaptive_support_receipt);
                snow_free_successor_receipts.extend(support_snow_free_successor_receipts);
                (
                    parent,
                    consumer,
                    clock,
                    finalized,
                    Some(ending_stage3),
                    ending_snow_enthalpy_material_owner,
                    ending_snow_enthalpy_material_owner_chronology,
                )
            } else {
                let beginning_pending_terminal_parcels = candidate.terminal_parcels.clone();
                let (parent, consumer, clock, finalized, accepted_support) = execute_real_v11_parent(
                    &self.static_context,
                    &beginning_parent,
                    &candidate.real_consumer,
                    &beginning_clock,
                    support,
                    prepared.day_index(),
                    support_index,
                    forcing_receipt,
                    if candidate.terminal_parcels.is_empty() {
                        canonical_stage3_snow_owner_bytes_with_pending(
                            &candidate.stage3_by_lane,
                            &candidate.terminal_parcels,
                        )?
                    } else {
                        candidate
                            .coupled_clock
                            .owners()
                            .iter()
                            .find(|owner| owner.owner_id() == "snow")
                            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                                "retained terminal V4 snow owner",
                            ))?
                            .state_bytes()
                            .to_vec()
                    },
                    None,
                    true,
                )?;
                let finalized = finalized.ok_or(
                    DirectSnowStage3V11AttachmentError::Identity(
                        "snow-free V11 parent missing endpoint finalization",
                    ),
                )?;
                snow_free_successor_receipts.push(Stage3SnowFreeSuccessorReceiptV1::seal(
                    support,
                    prepared.day_index(),
                    support_index,
                    beginning_parent.parent_transaction_id(),
                    0,
                    forcing_receipt,
                    &beginning_pending_terminal_parcels,
                    &candidate.terminal_parcels,
                    accepted_support,
                )?);
                (
                    parent,
                    consumer,
                    clock,
                    finalized,
                    None,
                    candidate.snow_enthalpy_material_owner.clone(),
                    candidate.snow_enthalpy_material_owner_chronology.clone(),
                )
            };
            candidate.v11_parent_state = parent;
            candidate.real_consumer = consumer;
            candidate.coupled_clock = clock;
            candidate.last_v11_parent_candidate = Some(finalized);
            if let Some(ending_stage3) = covered_stage3 {
                candidate.stage3_by_lane = ending_stage3;
            }
            install_snow_enthalpy_material_resident_into_committed_v1(
                &mut candidate,
                SnowStage3V11SnowEnthalpyMaterialResidentV1 {
                    current_owner: covered_snow_enthalpy_material_owner,
                    accepted_owner_chronology:
                        covered_snow_enthalpy_material_owner_chronology,
                },
            )?;
            candidate.next_parent_sequence = candidate.next_parent_sequence.checked_add(1).ok_or(
                DirectSnowStage3V11AttachmentError::Identity("V11 parent sequence overflow"),
            )?;
            support_index += 1;
        }
        candidate
            .real_consumer
            .commit_prepared_provider_day(prepared.clone().into_provider_day())?;
        let stage3_digests = candidate
            .stage3_by_lane
            .iter()
            .map(|(lane, state)| {
                let bytes = Wb11HydrologyKernel::serialize_stage3_persistent_state(state).map_err(
                    |_| DirectSnowStage3V11AttachmentError::Identity("Stage-3 restart bytes"),
                )?;
                Ok((*lane, openwepp_coupled_time::digest_bytes(&bytes)))
            })
            .collect::<Result<BTreeMap<_, _>, DirectSnowStage3V11AttachmentError>>()?;
        let complete_owner_bytes = candidate
            .real_consumer
            .canonical_owner_state_bytes()
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("canonical V11 owner bytes")
            })?;
        let mut complete_owner_bytes = complete_owner_bytes;
        bind_parent_receipt_snow_owner_bytes_v1(
            &mut complete_owner_bytes,
            candidate.coupled_clock.owners(),
        )?;
        let integrated_boundary_ledger = reconstruct_integrated_boundary_ledger(&coupled_subslabs);
        let receipt = DirectSnowStage3V11ParentReceipt {
            day_index: prepared.day_index(),
            support_count: prepared.supports().len(),
            terminal_events,
            terminal_event_groups,
            ending_stage3_state_digests: stage3_digests,
            complete_owner_bytes,
            covered_owner_joins,
            coupled_subslabs,
            adaptive_support_receipts,
            snow_free_successor_receipts,
            integrated_boundary_ledger,
            ending_coupled_owner_set_sha256: complete_owner_set_digest(
                candidate.coupled_clock.owners(),
            )?,
            ending_coupled_accepted_until_ns: candidate.coupled_clock.accepted_until(),
            ending_next_parent_sequence: candidate.next_parent_sequence,
            ending_v11_parent_state: candidate.v11_parent_state.clone(),
            ending_last_v11_parent_candidate: candidate.last_v11_parent_candidate.clone(),
        };
        candidate.receipt_chain.push(receipt.clone());
        Ok(PreparedDayExecutionOutcomeV2::Complete(
            DirectSnowStage3V11ParentCandidate {
                ending_state: candidate,
                parent_receipt: receipt,
            },
        ))
    }

    /// The only installation point.  Every owner and receipt check happens
    /// before this non-fallible replacement, preserving rollback on failure.
    pub fn install_candidate(
        &mut self,
        candidate: DirectSnowStage3V11ParentCandidate,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.install_candidate_inner(candidate, None)
    }

    fn install_candidate_with_publication(
        &mut self,
        candidate: DirectSnowStage3V11ParentCandidate,
        publication_day: crate::direct_runtime::Stage3AcceptedPublicationDayV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.install_candidate_inner(candidate, Some(publication_day))
    }

    fn install_candidate_inner(
        &mut self,
        candidate: DirectSnowStage3V11ParentCandidate,
        publication_day: Option<crate::direct_runtime::Stage3AcceptedPublicationDayV1>,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if candidate.ending_state.receipt_chain.len() != self.committed.receipt_chain.len() + 1
            || candidate.ending_state.receipt_chain[..self.committed.receipt_chain.len()]
                != self.committed.receipt_chain
            || candidate.ending_state.receipt_chain.last() != Some(&candidate.parent_receipt)
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt chain installation",
            ));
        }
        candidate
            .parent_receipt
            .validate_against_ending(&candidate.ending_state)?;
        if let Some(publication_day) = &publication_day {
            publication_day
                .validate_for_install(
                    candidate.parent_receipt.day_index,
                    self.static_context.lane_ids.len(),
                    candidate.parent_receipt.ending_coupled_owner_set_sha256,
                )
                .map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "committed publication capability installation",
                    )
                })?;
        }
        let mut accepted_subslabs = candidate
            .parent_receipt
            .coupled_subslabs
            .iter()
            .map(|subslab| {
                (
                    subslab.support,
                    subslab.accepted_slab_sha256,
                    subslab.owner_join.parent_transaction_sha256,
                    subslab.owner_join.beginning_complete_owner_set_sha256,
                    subslab.owner_join.ending_complete_owner_set_sha256,
                )
            })
            .collect::<Vec<_>>();
        accepted_subslabs.extend(
            candidate
                .parent_receipt
                .snow_free_successor_receipts
                .iter()
                .map(|receipt| {
                    (
                        receipt.support,
                        receipt.accepted_slab_sha256,
                        receipt.parent_transaction_id.digest(),
                        receipt.beginning_complete_owner_set_sha256,
                        receipt.ending_complete_owner_set_sha256,
                    )
                }),
        );
        accepted_subslabs.sort_by_key(|receipt| receipt.0.start_ns());
        let final_publication_handoff_valid = candidate
            .ending_state
            .real_consumer
            .validate_accepted_publication_final_handoff(
                &accepted_subslabs,
                complete_owner_set_digest(self.committed.coupled_clock.owners())?,
                candidate.parent_receipt.ending_coupled_owner_set_sha256,
            )
            .is_ok();
        if candidate.parent_receipt.day_index != self.committed.real_consumer.v11_next_day_index() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt installation day-index join",
            ));
        }
        let installed_parent_count = candidate
            .ending_state
            .next_parent_sequence
            .checked_sub(self.committed.next_parent_sequence)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "parent sequence installation",
            ))?;
        if candidate.parent_receipt.support_count as u128 != installed_parent_count {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt installation support-count join",
            ));
        }
        if !final_publication_handoff_valid {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt installation final-publication join",
            ));
        }
        if candidate
            .parent_receipt
            .terminal_events
            .iter()
            .any(|event| {
                !event.candidate_ticks.contains(&event.accepted_event_tick)
                    || !self.static_context.lane_ids.contains(&event.lane_id)
            })
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt installation terminal-event membership join",
            ));
        }
        self.committed = candidate.ending_state;
        self.committed_publication_day = publication_day;
        Ok(())
    }

    /// Seal one fully installed day for the runner's transaction-private,
    /// content-addressed archive. Full evidence remains resident until an
    /// exact archive acknowledgement advances the prefix.
    pub(crate) fn stage_committed_day_archive_v1(
        &mut self,
        day_delta: SnowStage3V11QualificationDayDeltaV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.pending_candidate.is_some()
            || self.pending_publication_day.is_some()
            || self.in_progress_execution.is_some()
            || self.pending_committed_day_evidence.is_some()
            || self.committed.receipt_chain.len() != 1
        {
            return Err(archive_error("archive staging posture or resident-day bound"));
        }
        let parent_receipt = self
            .committed
            .receipt_chain
            .last()
            .ok_or_else(|| archive_error("archive missing committed parent receipt"))?;
        let publication_day = self
            .committed_publication_day
            .as_ref()
            .ok_or_else(|| archive_error("archive missing committed publication day"))?;
        let publication_evidence = self
            .committed
            .real_consumer
            .seal_accepted_publication_day_evidence_v1(parent_receipt.day_index)
            .map_err(|_| archive_error("archive publication-day evidence seal"))?;
        let pending = Stage3PendingCommittedDayEvidenceV1::try_new(
            &self.static_context,
            &self.archived_receipt_prefix,
            parent_receipt,
            publication_day,
            publication_evidence,
            day_delta,
        )?;
        self.pending_committed_day_evidence = Some(pending);
        Ok(())
    }

    #[must_use]
    pub fn archived_receipt_prefix_v1(&self) -> &Stage3ArchivedReceiptPrefixV1 {
        &self.archived_receipt_prefix
    }

    #[must_use]
    pub fn pending_committed_day_evidence_v1(
        &self,
    ) -> Option<&Stage3PendingCommittedDayEvidenceV1> {
        self.pending_committed_day_evidence.as_ref()
    }

    /// Stream the exact staged record without materializing its canonical
    /// representation. The resident committed receipt remains authoritative
    /// until the runner durably persists and acknowledges this record.
    pub fn write_pending_committed_day_evidence_v1(
        &self,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let pending = self
            .pending_committed_day_evidence
            .as_ref()
            .ok_or_else(|| archive_error("archive canonical write without pending day"))?;
        let parent_receipt = self
            .committed
            .receipt_chain
            .last()
            .ok_or_else(|| archive_error("archive canonical write missing parent receipt"))?;
        let publication_day = self
            .committed_publication_day
            .as_ref()
            .ok_or_else(|| archive_error("archive canonical write missing publication day"))?;
        if parent_receipt.day_index != pending.entry.day_index
            || publication_day.day_index() != pending.entry.day_index
        {
            return Err(archive_error("archive canonical write day join"));
        }
        pending.write_canonical_uncompressed(
            &self.static_context,
            &self.archived_receipt_prefix,
            parent_receipt,
            publication_day,
            writer,
        )
    }

    /// Accept the runner spool's exact content-addressed record and release
    /// the now-archived day. Rotation is transactional and the next day stays
    /// fail-closed until this acknowledgement succeeds.
    pub fn acknowledge_committed_day_archive_v1(
        &mut self,
        record_sha256: Digest32,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let pending = self
            .pending_committed_day_evidence
            .as_ref()
            .ok_or_else(|| archive_error("archive acknowledgement without pending day"))?;
        if record_sha256 != pending.entry.record_sha256 {
            return Err(archive_error("archive acknowledgement record substitution"));
        }
        let mut next_prefix = self.archived_receipt_prefix.clone();
        next_prefix.append_day(&pending.entry, &pending.day_delta)?;
        let mut next_real_consumer = self.committed.real_consumer.clone();
        next_real_consumer
            .rotate_accepted_publication_day_v1(&pending.publication_evidence)
            .map_err(|_| archive_error("archive publication-history rotation"))?;
        let retention = next_real_consumer.accepted_publication_retention_state_v1();
        if retention.resident_support_count() != 0
            || retention.resident_event_count() != 0
            || retention.sealed_support_count()
                != usize::try_from(next_prefix.qualification_accumulator.accepted_publication_support_count)
                    .map_err(|_| archive_error("archive support-count width"))?
            || retention.sealed_event_count()
                != usize::try_from(next_prefix.qualification_accumulator.publication_event_count)
                    .map_err(|_| archive_error("archive event-count width"))?
        {
            return Err(archive_error("archive rotation bounded residency or count join"));
        }
        self.committed.real_consumer = next_real_consumer;
        self.committed.receipt_chain.clear();
        self.archived_receipt_prefix = next_prefix;
        self.pending_committed_day_evidence = None;
        Ok(())
    }
}
