#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct DirectExecutionCounters {
    spans: u64,
    entries: u64,
    computes: u64,
    mutations: u64,
    downstream_operands: u64,
    shadows: u64,
    compatibility_edges: u64,
    day_frame_commits: u64,
    phase_executed_counts: [u64; DIRECT_PHASE_COUNT],
    phase_hold_counts: [u64; DIRECT_PHASE_COUNT],
}

impl DirectExecutionCounters {
    fn record_span(
        &mut self,
        phase_entry_count: u64,
        direct_compute_count: u64,
        state_mutation_count: u64,
        downstream_operand_count: u64,
        shadow_projection_count: u64,
        compatibility_edge_invocation_count: u64,
    ) {
        self.spans += 1;
        self.entries += phase_entry_count;
        self.computes += direct_compute_count;
        self.mutations += state_mutation_count;
        self.downstream_operands += downstream_operand_count;
        self.shadows += shadow_projection_count;
        self.compatibility_edges += compatibility_edge_invocation_count;
    }

    fn record_phase_status(&mut self, phase: DirectPhaseKind, status: DirectPhaseLifecycleStatus) {
        match status {
            DirectPhaseLifecycleStatus::Executed => {
                self.phase_executed_counts[phase.rank()] += 1;
            }
            DirectPhaseLifecycleStatus::Hold => {
                self.phase_hold_counts[phase.rank()] += 1;
            }
        }
    }

    fn record_day_frame_commit(&mut self) {
        self.day_frame_commits += 1;
    }

    fn record_dynamic_transfer_publication(&mut self) {
        self.mutations += 1;
        self.downstream_operands += 1;
    }

    fn phase_status_counts(&self) -> Vec<DirectPhaseStatusCount> {
        let mut counts = Vec::with_capacity(DIRECT_PHASE_COUNT);
        for phase in DirectPhaseKind::ORDERED {
            let executed_count = self.phase_executed_counts[phase.rank()];
            let hold_count = self.phase_hold_counts[phase.rank()];
            if executed_count > 0 {
                counts.push(DirectPhaseStatusCount {
                    phase,
                    status: DirectPhaseLifecycleStatus::Executed,
                    count: executed_count,
                });
            }
            if hold_count > 0 {
                counts.push(DirectPhaseStatusCount {
                    phase,
                    status: DirectPhaseLifecycleStatus::Hold,
                    count: hold_count,
                });
            }
        }
        counts
    }
}

macro_rules! record_direct_span_report {
    ($counters:expr, $span:expr) => {{
        let report = $span?;
        $counters.record_span(
            report.phase_entry_count,
            report.direct_compute_count,
            report.state_mutation_count,
            report.downstream_operand_count,
            report.shadow_projection_count,
            report.compatibility_edge_invocation_count,
        );
    }};
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectFrameExecutor {
    mode: DirectExecutorMode,
}

impl DirectFrameExecutor {
    #[must_use]
    pub fn new(mode: DirectExecutorMode) -> Self {
        DIRECT_AUDIT.record_executor_construction();
        Self { mode }
    }

    #[must_use]
    pub const fn mode(&self) -> DirectExecutorMode {
        self.mode
    }

    pub fn run_skeleton(
        &self,
        frame: &mut DirectRunFrame,
    ) -> Result<DirectExecutionReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_skeleton_run();
        // D15A: the active owner is a publication-stream integration; a
        // skeleton run with the selector present would silently not route,
        // which is exactly the shadow-only-activation shape rev 27 forbids.
        if frame.laned_active.is_some() {
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "laned_active_selector",
                detail: "the skeleton executor does not support the active routing owner; use the publication stream".to_string(),
            });
        }
        let mut phase_view_count = 0_u64;
        let mut counters = DirectExecutionCounters::default();
        let phase_plan = *frame.phase_plan.phases();

        let transfer_span_report = frame.run_r3c_lane_transfer_span()?;
        counters.record_span(
            transfer_span_report.phase_entry_count,
            transfer_span_report.direct_compute_count,
            transfer_span_report.state_mutation_count,
            transfer_span_report.downstream_operand_count,
            transfer_span_report.shadow_projection_count,
            transfer_span_report.compatibility_edge_invocation_count,
        );

        for day_index in 0..frame.identity.day_count {
            for lane_index in 0..frame.identity.lane_count {
                let winter_frost_compute_inputs = frame
                    .lanes
                    .get(lane_index)
                    .and_then(|lane| lane.day_inputs.get(day_index))
                    .and_then(|day_inputs| day_inputs.winter_frost_compute_inputs.clone());
                let mut day_frame = frame.seed_day_frame(lane_index, day_index)?;
                Self::run_day_spans(
                    &mut day_frame,
                    &mut counters,
                    winter_frost_compute_inputs.as_ref(),
                )
                .map_err(|source| {
                    Self::day_execution_failure(&day_frame, lane_index, day_index, &source)
                })?;
                for phase in phase_plan {
                    let view = day_frame.phase_view(phase);
                    let _phase = view.phase();
                    phase_view_count += 1;
                    counters.record_phase_status(phase, Self::phase_lifecycle_status(phase));
                }
                if Self::publish_dynamic_transfer_to_downstream(frame, &day_frame)? {
                    counters.record_dynamic_transfer_publication();
                }
                if Self::publish_erosion_inflow_to_downstream(frame, &day_frame)? {
                    counters.record_dynamic_transfer_publication();
                }
                frame.commit_day_frame(&day_frame)?;
                counters.record_day_frame_commit();
            }
        }

        Ok(DirectExecutionReport {
            mode: self.mode,
            lane_count: frame.lanes.len(),
            day_count: frame.identity.day_count,
            planned_phase_count: frame.phase_plan.len(),
            canonical_phase_entry_count: phase_view_count,
            phase_view_count,
            phase_status_counts: counters.phase_status_counts(),
            phase_span_run_count: counters.spans,
            direct_phase_entry_count: counters.entries,
            direct_compute_count: counters.computes,
            state_mutation_count: counters.mutations,
            downstream_operand_count: counters.downstream_operands,
            shadow_projection_count: counters.shadows,
            compatibility_edge_invocation_count: counters.compatibility_edges,
            day_frame_commit_count: counters.day_frame_commits,
        })
    }

    pub fn run_publication_capture(
        &self,
        frame: &mut DirectRunFrame,
        metadata: DirectPublicationRunMetadata,
        calendar_days: &[DirectPublicationCalendarDay],
    ) -> Result<DirectPublicationExecution, DirectRuntimeError> {
        let day_inputs = calendar_days
            .iter()
            .copied()
            .map(DirectPublicationDayInput::calendar_only)
            .collect::<Vec<_>>();
        self.run_publication_capture_with_day_inputs(frame, metadata, &day_inputs)
    }

    pub fn run_publication_capture_with_day_inputs(
        &self,
        frame: &mut DirectRunFrame,
        metadata: DirectPublicationRunMetadata,
        day_inputs: &[DirectPublicationDayInput],
    ) -> Result<DirectPublicationExecution, DirectRuntimeError> {
        if day_inputs.len() != frame.identity.day_count {
            return Err(DirectRuntimeError::CalendarDayCountMismatch {
                identity_day_count: frame.identity.day_count,
                calendar_day_count: day_inputs.len(),
            });
        }
        let identity_day_count = frame.identity.day_count;
        let calendar_day_count = day_inputs.len();
        self.run_publication_capture_with_interleaved_day_inputs(
            frame,
            metadata,
            |_frame, day_index, _lane_index| {
                day_inputs.get(day_index).cloned().ok_or(
                    DirectRuntimeError::CalendarDayCountMismatch {
                        identity_day_count,
                        calendar_day_count,
                    },
                )
            },
        )
    }

    pub fn run_publication_capture_with_interleaved_day_inputs<F>(
        &self,
        frame: &mut DirectRunFrame,
        metadata: DirectPublicationRunMetadata,
        build_day_input: F,
    ) -> Result<DirectPublicationExecution, DirectRuntimeError>
    where
        F: FnMut(
            &DirectRunFrame,
            usize,
            usize,
        ) -> Result<DirectPublicationDayInput, DirectRuntimeError>,
    {
        let expected_row_count = frame
            .identity
            .lane_count
            .checked_mul(frame.identity.day_count)
            .ok_or(DirectRuntimeError::DirectDomainViolation {
                field: "publication.expected_row_count",
            })?;
        let mut publication_frame =
            DirectRunPublicationFrame::new(frame.identity, metadata.clone(), expected_row_count);
        let execution = self.run_publication_stream_with_interleaved_day_inputs(
            frame,
            metadata,
            build_day_input,
            |row| {
                publication_frame.rows.push(row.clone());
                Ok(())
            },
        )?;
        publication_frame.validate_complete()?;

        Ok(DirectPublicationExecution {
            report: execution.report,
            publication_frame,
        })
    }

    pub fn run_publication_stream_with_interleaved_day_inputs<F, S>(
        &self,
        frame: &mut DirectRunFrame,
        metadata: DirectPublicationRunMetadata,
        build_day_input: F,
        mut consume_row: S,
    ) -> Result<DirectStreamingPublicationExecution, DirectRuntimeError>
    where
        F: FnMut(
            &DirectRunFrame,
            usize,
            usize,
        ) -> Result<DirectPublicationDayInput, DirectRuntimeError>,
        S: FnMut(&DirectPublicationDayRow) -> Result<(), DirectRuntimeError>,
    {
        self.run_publication_stream_with_interleaved_day_inputs_and_day_frames(
            frame,
            metadata,
            build_day_input,
            |row, _day_frame| consume_row(row),
        )
    }

    #[allow(clippy::too_many_lines)]
    pub fn run_publication_stream_with_interleaved_day_inputs_and_day_frames<F, S>(
        &self,
        frame: &mut DirectRunFrame,
        metadata: DirectPublicationRunMetadata,
        mut build_day_input: F,
        mut consume_row: S,
    ) -> Result<DirectStreamingPublicationExecution, DirectRuntimeError>
    where
        F: FnMut(
            &DirectRunFrame,
            usize,
            usize,
        ) -> Result<DirectPublicationDayInput, DirectRuntimeError>,
        S: FnMut(&DirectPublicationDayRow, &DirectDayFrame) -> Result<(), DirectRuntimeError>,
    {
        // D15A (rev 27): the opt-in ACTIVE owner takes the two-phase day
        // loop; the default path below is textually untouched
        // (`INV-OFEROUTE-010`).
        if frame.laned_active.is_some() {
            return self.run_laned_active_publication_stream(
                frame,
                metadata,
                build_day_input,
                consume_row,
            );
        }
        DIRECT_AUDIT.record_publication_capture_run();
        let expected_row_count = frame
            .identity
            .lane_count
            .checked_mul(frame.identity.day_count)
            .ok_or(DirectRuntimeError::DirectDomainViolation {
                field: "publication.expected_row_count",
            })?;
        let identity = frame.identity;
        let mut phase_view_count = 0_u64;
        let mut counters = DirectExecutionCounters::default();
        let phase_plan = *frame.phase_plan.phases();
        let mut row_count = 0_usize;

        let transfer_span_report = frame.run_r3c_lane_transfer_span()?;
        counters.record_span(
            transfer_span_report.phase_entry_count,
            transfer_span_report.direct_compute_count,
            transfer_span_report.state_mutation_count,
            transfer_span_report.downstream_operand_count,
            transfer_span_report.shadow_projection_count,
            transfer_span_report.compatibility_edge_invocation_count,
        );

        for day_index in 0..frame.identity.day_count {
            for lane_index in 0..frame.identity.lane_count {
                let day_input = build_day_input(frame, day_index, lane_index)?;
                let mut day_frame = frame.seed_day_frame(lane_index, day_index)?;
                Self::apply_publication_day_input(&mut day_frame, &day_input)?;
                Self::run_day_spans(
                    &mut day_frame,
                    &mut counters,
                    day_input.winter_frost_compute_inputs.as_ref(),
                )
                .map_err(|source| {
                    Self::day_execution_failure(&day_frame, lane_index, day_index, &source)
                })?;
                for phase in phase_plan {
                    let view = day_frame.phase_view(phase);
                    let _phase = view.phase();
                    phase_view_count += 1;
                    counters.record_phase_status(phase, Self::phase_lifecycle_status(phase));
                }
                let lane =
                    frame
                        .lanes
                        .get(lane_index)
                        .ok_or(DirectRuntimeError::LaneIndexOutOfRange {
                            lane_index,
                            lane_count: frame.lanes.len(),
                        })?;
                let row = DirectPublicationDayRow::from_day_frame(&day_frame, &day_input, lane)?;
                consume_row(&row, &day_frame)?;
                row_count = row_count.checked_add(1).ok_or(
                    DirectRuntimeError::DirectDomainViolation {
                        field: "publication.row_count",
                    },
                )?;
                if Self::publish_dynamic_transfer_to_downstream(frame, &day_frame)? {
                    counters.record_dynamic_transfer_publication();
                }
                if Self::publish_erosion_inflow_to_downstream(frame, &day_frame)? {
                    counters.record_dynamic_transfer_publication();
                }
                frame.commit_day_frame(&day_frame)?;
                counters.record_day_frame_commit();
            }
        }
        if row_count != expected_row_count {
            return Err(DirectRuntimeError::PublicationRowCountMismatch {
                expected_row_count,
                actual_row_count: row_count,
            });
        }

        Ok(DirectStreamingPublicationExecution {
            report: DirectExecutionReport {
                mode: self.mode,
                lane_count: frame.lanes.len(),
                day_count: frame.identity.day_count,
                planned_phase_count: frame.phase_plan.len(),
                canonical_phase_entry_count: phase_view_count,
                phase_view_count,
                phase_status_counts: counters.phase_status_counts(),
                phase_span_run_count: counters.spans,
                direct_phase_entry_count: counters.entries,
                direct_compute_count: counters.computes,
                state_mutation_count: counters.mutations,
                downstream_operand_count: counters.downstream_operands,
                shadow_projection_count: counters.shadows,
                compatibility_edge_invocation_count: counters.compatibility_edges,
                day_frame_commit_count: counters.day_frame_commits,
            },
            identity,
            metadata,
            row_count,
        })
    }

    /// D15A (rev 27): the ACTIVE publication stream — the two-phase day loop
    /// in which Lane D routing OWNS the surface-water path. Phase 1 runs
    /// every lane's hydrology (with the surface transfer suppressed — DC01
    /// disable) so the shared day window is derivable; phase 2 routes each
    /// lane in cascade order, flips the D13 erosion authority to the routed
    /// shape, runs the erosion/ledger tail, publishes rows, and enforces the
    /// day-closure hard-fails.
    #[allow(clippy::too_many_lines)]
    fn run_laned_active_publication_stream<F, S>(
        &self,
        frame: &mut DirectRunFrame,
        metadata: DirectPublicationRunMetadata,
        mut build_day_input: F,
        mut consume_row: S,
    ) -> Result<DirectStreamingPublicationExecution, DirectRuntimeError>
    where
        F: FnMut(
            &DirectRunFrame,
            usize,
            usize,
        ) -> Result<DirectPublicationDayInput, DirectRuntimeError>,
        S: FnMut(&DirectPublicationDayRow, &DirectDayFrame) -> Result<(), DirectRuntimeError>,
    {
        DIRECT_AUDIT.record_publication_capture_run();
        let config = frame
            .laned_active
            .clone()
            .ok_or(DirectRuntimeError::MissingDirectUpstream {
                upstream: "laned_active configuration",
            })?;
        config.validate(frame.identity.lane_count)?;
        // Active routing requires the linear MOFE chain in lane-index order
        // (the cascade handoff and the closure books assume it).
        for (lane_index, lane) in frame.lanes.iter().enumerate() {
            let expected_downstream = if lane_index + 1 < frame.lanes.len() {
                u32::try_from(lane_index + 2).map_err(|_| {
                    DirectRuntimeError::LaneIdOverflow { lane_index }
                })?
            } else {
                0
            };
            if lane.downstream_lane_id != expected_downstream {
                return Err(DirectRuntimeError::InvalidLaneTopology {
                    lane_index,
                    lane_id: lane.lane_id,
                    upstream_lane_id: lane.upstream_lane_id,
                    downstream_lane_id: lane.downstream_lane_id,
                });
            }
        }
        let expected_row_count = frame
            .identity
            .lane_count
            .checked_mul(frame.identity.day_count)
            .ok_or(DirectRuntimeError::DirectDomainViolation {
                field: "publication.expected_row_count",
            })?;
        let identity = frame.identity;
        let mut phase_view_count = 0_u64;
        let mut counters = DirectExecutionCounters::default();
        let phase_plan = *frame.phase_plan.phases();
        let mut row_count = 0_usize;
        let mut summary = laned_active::DirectLanedActiveRunSummary::default();

        let transfer_span_report = frame.run_r3c_lane_transfer_span()?;
        counters.record_span(
            transfer_span_report.phase_entry_count,
            transfer_span_report.direct_compute_count,
            transfer_span_report.state_mutation_count,
            transfer_span_report.downstream_operand_count,
            transfer_span_report.shadow_projection_count,
            transfer_span_report.compatibility_edge_invocation_count,
        );

        let lane_count = frame.identity.lane_count;
        for day_index in 0..frame.identity.day_count {
            // Phase 1: hydrology for every lane, in lane order, with the
            // SURFACE transfer suppressed (router ownership) and the LATERAL
            // transfer published unchanged.
            let mut day_frames: Vec<DirectDayFrame> = Vec::with_capacity(lane_count);
            let mut day_inputs: Vec<DirectPublicationDayInput> = Vec::with_capacity(lane_count);
            for lane_index in 0..lane_count {
                let day_input = build_day_input(frame, day_index, lane_index)?;
                let mut day_frame = frame.seed_day_frame(lane_index, day_index)?;
                Self::apply_publication_day_input(&mut day_frame, &day_input)?;
                Self::run_day_spans_hydrology(
                    &mut day_frame,
                    &mut counters,
                    day_input.winter_frost_compute_inputs.as_ref(),
                )
                .map_err(|source| {
                    Self::day_execution_failure(&day_frame, lane_index, day_index, &source)
                })?;
                if Self::publish_dynamic_transfer_to_downstream_with_ownership(
                    frame, &day_frame, true,
                )? {
                    counters.record_dynamic_transfer_publication();
                }
                day_inputs.push(day_input);
                day_frames.push(day_frame);
            }

            // The shared day window (rev-27 window row): last active source
            // hour over ALL lanes + the drain tail; `None` = zero-source day.
            let mut last_active_hour: Option<usize> = None;
            let mut lane_sources = Vec::with_capacity(lane_count);
            for (lane_index, day_frame) in day_frames.iter().enumerate() {
                let source =
                    laned_active::laned_active_lane_source(day_frame).map_err(|source_error| {
                        Self::day_execution_failure(
                            day_frame,
                            lane_index,
                            day_index,
                            &source_error,
                        )
                    })?;
                for (hour, depth) in source.depths_m.iter().enumerate() {
                    if *depth > 0.0 {
                        last_active_hour =
                            Some(last_active_hour.map_or(hour, |current| current.max(hour)));
                    }
                }
                lane_sources.push(source);
            }
            let window_s = last_active_hour.map(laned_active::laned_active_window_s);

            // Phase 2: route -> erosion -> ledger -> row -> publishers ->
            // commit, lane order preserved.
            let mut books = laned_active::DirectLanedActiveDayBooks::default();
            let mut upstream: Option<crate::ofe_routing::cascade::UpstreamHandoff> = None;
            for lane_index in 0..lane_count {
                let area_m2 = frame
                    .lanes
                    .get(lane_index)
                    .ok_or(DirectRuntimeError::LaneIndexOutOfRange {
                        lane_index,
                        lane_count: frame.lanes.len(),
                    })?
                    .area_m2;
                let day_frame = &mut day_frames[lane_index];
                // The upstream lane's phase 2 published this lane's erosion
                // inflow intake AFTER this frame was seeded; refresh it so
                // the erosion span sees the same intake the DC01 loop would.
                day_frame
                    .erosion_inflow_intake
                    .clone_from(&frame.lanes[lane_index].erosion_inflow_intake);
                if let Some(window_s) = window_s {
                    let handoff = laned_active::laned_active_route_lane(
                        day_frame,
                        &config.lanes[lane_index],
                        area_m2,
                        upstream.as_ref(),
                        window_s,
                        &mut books,
                        &lane_sources[lane_index],
                    )
                    .map_err(|source| {
                        Self::day_execution_failure(day_frame, lane_index, day_index, &source)
                    })?;
                    upstream = Some(handoff);
                } else {
                    // Zero-source day: nothing to route, but the erosion
                    // authority still owns the (all-zero) shape so no DC01
                    // shape is consumed on an active lane, and the
                    // double-feed guard still runs.
                    laned_active::laned_active_assert_no_dc01_surface_feed(day_frame).map_err(
                        |source| {
                            Self::day_execution_failure(day_frame, lane_index, day_index, &source)
                        },
                    )?;
                    day_frame.erosion_inputs.hydrograph_shape_authority =
                        DirectErosionHydrographShapeAuthority::RoutedHydrograph;
                    day_frame.erosion_inputs.routed_hydrograph_runoff_fraction =
                        Some(Box::new([0.0; 24]));
                    // CR-L1: the INV-OFEROUTE-012 latqcc bypass exists on
                    // zero-source days too; record the terminal lane's
                    // lateral export so the manifest total covers ALL days.
                    if lane_index + 1 == lane_count {
                        books.latqcc_outlet_m3 = day_frame
                            .subsurface_compute_shadow_projection
                            .as_ref()
                            .map_or(0.0, |subsurface| subsurface.lateral_flow_m * area_m2);
                    }
                }
                Self::run_day_spans_erosion_and_ledger(day_frame, &mut counters).map_err(
                    |source| {
                        Self::day_execution_failure(day_frame, lane_index, day_index, &source)
                    },
                )?;
                for phase in phase_plan {
                    let view = day_frame.phase_view(phase);
                    let _phase = view.phase();
                    phase_view_count += 1;
                    counters.record_phase_status(phase, Self::phase_lifecycle_status(phase));
                }
                let lane =
                    frame
                        .lanes
                        .get(lane_index)
                        .ok_or(DirectRuntimeError::LaneIndexOutOfRange {
                            lane_index,
                            lane_count: frame.lanes.len(),
                        })?;
                let row =
                    DirectPublicationDayRow::from_day_frame(day_frame, &day_inputs[lane_index], lane)?;
                consume_row(&row, day_frame)?;
                row_count =
                    row_count
                        .checked_add(1)
                        .ok_or(DirectRuntimeError::DirectDomainViolation {
                            field: "publication.row_count",
                        })?;
                if Self::publish_erosion_inflow_to_downstream(frame, &day_frames[lane_index])? {
                    counters.record_dynamic_transfer_publication();
                }
                frame.commit_day_frame(&day_frames[lane_index])?;
                counters.record_day_frame_commit();
            }
            laned_active::laned_active_enforce_day_closure(day_index, &books, &mut summary)?;
        }
        if row_count != expected_row_count {
            return Err(DirectRuntimeError::PublicationRowCountMismatch {
                expected_row_count,
                actual_row_count: row_count,
            });
        }
        frame.laned_active_summary = Some(Box::new(summary));

        Ok(DirectStreamingPublicationExecution {
            report: DirectExecutionReport {
                mode: self.mode,
                lane_count: frame.lanes.len(),
                day_count: frame.identity.day_count,
                planned_phase_count: frame.phase_plan.len(),
                canonical_phase_entry_count: phase_view_count,
                phase_view_count,
                phase_status_counts: counters.phase_status_counts(),
                phase_span_run_count: counters.spans,
                direct_phase_entry_count: counters.entries,
                direct_compute_count: counters.computes,
                state_mutation_count: counters.mutations,
                downstream_operand_count: counters.downstream_operands,
                shadow_projection_count: counters.shadows,
                compatibility_edge_invocation_count: counters.compatibility_edges,
                day_frame_commit_count: counters.day_frame_commits,
            },
            identity,
            metadata,
            row_count,
        })
    }

    fn apply_publication_day_input(
        day_frame: &mut DirectDayFrame,
        day_input: &DirectPublicationDayInput,
    ) -> Result<(), DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "publication_input.precipitation_m",
            day_input.precipitation_m,
        )?;
        validate_finite(
            "publication_input.effective_temperature_c",
            day_input.effective_temperature_c,
        )?;
        if let Some(canopy_cover_fraction) = day_input.canopy_cover_fraction {
            validate_finite(
                "publication_input.canopy_cover_fraction",
                canopy_cover_fraction,
            )?;
            if !(0.0..=1.0).contains(&canopy_cover_fraction) {
                return Err(DirectRuntimeError::DirectDomainViolation {
                    field: "publication_input.canopy_cover_fraction",
                });
            }
        }
        day_frame.forcing.precipitation_m = day_input.precipitation_m;
        day_frame.forcing.effective_temperature_c = day_input.effective_temperature_c;
        day_frame.interception_m = day_input.interception_m;
        day_frame.storage_reconciliation_inputs.interception_m = day_input.interception_m;
        if let Some(initial_soil_water_m) = day_input.initial_soil_water_m {
            validate_nonnegative_direct_m(
                "publication_input.initial_soil_water_m",
                initial_soil_water_m,
            )?;
            day_frame.water.soil_water_m = initial_soil_water_m;
        }
        if let Some(storage_input_inputs) = day_input.storage_input_inputs {
            day_frame.storage_input_inputs = storage_input_inputs;
        }
        if let Some(liquid_input_inputs) = day_input.liquid_input_inputs {
            day_frame.liquid_input_inputs = liquid_input_inputs;
        } else {
            day_frame.liquid_input_inputs.liquid_input_handoff_m = day_input.precipitation_m;
        }
        Self::apply_publication_percolation_input(day_frame, day_input)?;
        if let Some(infiltration_depression_inputs) = &day_input.infiltration_depression_inputs {
            day_frame.infiltration_depression_inputs = infiltration_depression_inputs.clone();
        }
        if let Some(peak_runoff_inputs) = &day_input.peak_runoff_inputs {
            day_frame.peak_runoff_inputs = peak_runoff_inputs.clone();
        }
        Self::apply_publication_subsurface_input(day_frame, day_input)?;
        if let Some(evapotranspiration_compute_inputs) =
            day_input.evapotranspiration_compute_inputs.clone()
        {
            let mut evapotranspiration_compute_inputs = evapotranspiration_compute_inputs;
            if evapotranspiration_compute_inputs.stage_state.is_none() {
                evapotranspiration_compute_inputs.stage_state =
                    day_frame.evapotranspiration_compute_inputs.stage_state;
            }
            day_frame.evapotranspiration_compute_inputs = evapotranspiration_compute_inputs;
        }
        if let Some(decomposition_inputs) = day_input.decomposition_inputs {
            day_frame.decomposition_inputs = decomposition_inputs;
        }
        if let Some(residue_partition_inputs) = day_input.residue_partition_inputs {
            day_frame.residue_partition_inputs = residue_partition_inputs;
        }
        if let Some(annual_growth_inputs) = day_input.annual_growth_inputs {
            day_frame.annual_growth_inputs = annual_growth_inputs;
        }
        if let Some(perennial_growth_inputs) = day_input.perennial_growth_inputs {
            day_frame.perennial_growth_inputs = perennial_growth_inputs;
        }
        if let Some(snow_coupling_inputs) = &day_input.snow_coupling_inputs {
            day_frame.snow_coupling_inputs = snow_coupling_inputs.clone();
        }
        if let Some(hydrology_projection_inputs) = day_input.hydrology_projection_inputs {
            day_frame.hydrology_projection_inputs = hydrology_projection_inputs;
        }
        if let Some(erosion_inputs) = &day_input.erosion_inputs {
            day_frame.erosion_inputs = erosion_inputs.clone();
        }
        day_frame
            .winter_frost_outcome
            .clone_from(&day_input.winter_frost_outcome);
        if let Some(frost_storage_liquid_delta_m) = day_input.frost_storage_liquid_delta_m {
            validate_finite(
                "publication_input.frost_storage_liquid_delta_m",
                frost_storage_liquid_delta_m,
            )?;
            day_frame.frost_storage_liquid_delta_m = Some(frost_storage_liquid_delta_m);
        }
        day_frame
            .frost_layer_carry_projection
            .clone_from(&day_input.frost_layer_carry_projection);
        if day_input.frost_runtime_carry.is_some() {
            day_frame
                .frost_runtime_carry
                .clone_from(&day_input.frost_runtime_carry);
        }
        Ok(())
    }

    fn apply_publication_percolation_input(
        day_frame: &mut DirectDayFrame,
        day_input: &DirectPublicationDayInput,
    ) -> Result<(), DirectRuntimeError> {
        let Some(percolation_inputs) = &day_input.percolation_inputs else {
            return Ok(());
        };
        let mut percolation_inputs = percolation_inputs.clone();
        if day_frame.day_index > 0 && !day_frame.percolation_inputs.layers.is_empty() {
            if percolation_inputs.layers.is_empty() {
                percolation_inputs
                    .layers
                    .clone_from(&day_frame.percolation_inputs.layers);
                percolation_inputs.soil_water_initial_m = day_frame.water.soil_water_m;
            } else if percolation_inputs.layers.len() != day_frame.percolation_inputs.layers.len() {
                return Err(DirectRuntimeError::DirectDomainViolation {
                    field: "publication_input.percolation_layers",
                });
            }
        } else if percolation_inputs.layers.is_empty() {
            percolation_inputs
                .layers
                .clone_from(&day_frame.percolation_inputs.layers);
            percolation_inputs.soil_water_initial_m = day_frame.water.soil_water_m;
        }
        day_frame.percolation_inputs = percolation_inputs;
        Ok(())
    }

    fn apply_publication_subsurface_input(
        day_frame: &mut DirectDayFrame,
        day_input: &DirectPublicationDayInput,
    ) -> Result<(), DirectRuntimeError> {
        let Some(subsurface_compute_inputs) = &day_input.subsurface_compute_inputs else {
            return Ok(());
        };
        let mut subsurface_compute_inputs = subsurface_compute_inputs.clone();
        if day_frame.day_index > 0 && !day_frame.subsurface_compute_inputs.layers.is_empty() {
            if subsurface_compute_inputs.layers.is_empty() {
                subsurface_compute_inputs
                    .layers
                    .clone_from(&day_frame.subsurface_compute_inputs.layers);
            } else if subsurface_compute_inputs.layers.len()
                != day_frame.subsurface_compute_inputs.layers.len()
            {
                return Err(DirectRuntimeError::DirectDomainViolation {
                    field: "publication_input.subsurface_layers",
                });
            }
        } else if subsurface_compute_inputs.layers.is_empty() {
            subsurface_compute_inputs
                .layers
                .clone_from(&day_frame.subsurface_compute_inputs.layers);
        }
        day_frame.subsurface_compute_inputs = subsurface_compute_inputs;
        Ok(())
    }

    /// DC01: unit-normalized hourly distribution of the day's surface runoff
    /// (WB14 excess profile + saturation carry). Uniform fallback when the day
    /// has runoff without a profile shape; all-zero when there is no runoff.
    /// Delegates to the shared shape authority
    /// (`runoff::dc01_surface_runoff_hourly_weights`, ADR-0036
    /// `REF-SED-DC01-SHAPE`) so the transfer publication and the
    /// hydrograph-resolved erosion substrate consume ONE hourly shape.
    fn dc01_surface_transfer_weights(
        q_runoff_m: f64,
        wb14_hourly_excess_m: &[f64; DIRECT_TRANSFER_HOUR_COUNT],
        hourly_saturation_carry_m: &[f64; DIRECT_TRANSFER_HOUR_COUNT],
        hourly_routed_melt_m: &[f64; DIRECT_TRANSFER_HOUR_COUNT],
    ) -> Result<[f64; DIRECT_TRANSFER_HOUR_COUNT], DirectRuntimeError> {
        crate::direct_runtime::runoff::dc01_surface_runoff_hourly_weights(
            q_runoff_m,
            wb14_hourly_excess_m,
            hourly_saturation_carry_m,
            hourly_routed_melt_m,
        )
    }

    /// E.3 (INV-SED-012): publish the inter-OFE erosion inflow to the
    /// downstream lane — the prior lane's hourly outflow discharge and
    /// sediment discharge (EROSION lineage: the Wave-1 solve's own
    /// surfaces, never water-transfer substitutes), its static slopes,
    /// solve-final coefficient sets, and exiting class fractions.
    fn publish_erosion_inflow_to_downstream(
        frame: &mut DirectRunFrame,
        day_frame: &DirectDayFrame,
    ) -> Result<bool, DirectRuntimeError> {
        if !day_frame.erosion_inputs.wave1_operand_seed.enabled
            || day_frame.wave1_hourly_plan.is_empty()
        {
            return Ok(false);
        }
        let (_, _, downstream_lane_id) = {
            let lane =
                frame
                    .lanes
                    .get(day_frame.lane_index)
                    .ok_or(DirectRuntimeError::LaneIndexOutOfRange {
                        lane_index: day_frame.lane_index,
                        lane_count: frame.lanes.len(),
                    })?;
            (lane.lane_id, lane.upstream_lane_id, lane.downstream_lane_id)
        };
        if downstream_lane_id == 0 {
            return Ok(false);
        }
        let downstream_index = usize::try_from(downstream_lane_id - 1).map_err(|_| {
            DirectRuntimeError::LaneIdOverflow {
                lane_index: day_frame.lane_index,
            }
        })?;

        let peak = day_frame.peak_runoff_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "E.3 erosion inflow publisher peak-runoff producer",
            },
        )?;
        let continuity = day_frame.erosion.wave1_continuity.as_deref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "E.3 erosion inflow publisher continuity state",
            },
        )?;
        let publication = &day_frame.erosion_downstream_operands.publication;
        let hourly_sediment_kg = publication.hourly_sediment_mass_kg.ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "E.3 erosion inflow publisher hourly sediment surface",
            },
        )?;
        let seed = &day_frame.erosion_inputs.wave1_operand_seed;
        if seed.field_width_m <= 0.0 || seed.efflen_m <= 0.0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "erosion.inflow_publisher.geometry",
            });
        }

        let mut hourly_qout_m2_s = [0.0_f64; 24];
        let mut hourly_qsout_kg_m_s = [0.0_f64; 24];
        for hour in 0..24 {
            hourly_qout_m2_s[hour] =
                peak.q_runoff_m * day_frame.wave1_hourly_weights[hour] / 3600.0 * seed.efflen_m;
            hourly_qsout_kg_m_s[hour] =
                hourly_sediment_kg[hour] / seed.field_width_m / 3600.0;
        }
        let sedcon = publication.sediment_concentration_kg_m3.unwrap_or([0.0; 5]);
        let sedcon_total: f64 = sedcon.iter().sum();
        let exit_fractions = if sedcon_total > 0.0 {
            core::array::from_fn(|index| sedcon[index] / sedcon_total)
        } else {
            // Legacy `route.for:158`: no exiting flow-load => zero fractions.
            [0.0; 5]
        };

        let intake = DirectErosionInflowIntake {
            hourly_qout_m2_s,
            hourly_qsout_kg_m_s,
            prior_slpend: seed.slpend,
            prior_cnslp: seed.avg_slope,
            prior_end_shear: continuity.end_shear_coefficients,
            prior_end_transport: continuity.end_transport_coefficients,
            exit_fractions,
        };
        let downstream_lane_count = frame.lanes.len();
        let downstream_lane =
            frame
                .lanes
                .get_mut(downstream_index)
                .ok_or(DirectRuntimeError::LaneIndexOutOfRange {
                    lane_index: downstream_index,
                    lane_count: downstream_lane_count,
                })?;
        downstream_lane.erosion_inflow_intake = Some(Box::new(intake));
        Ok(true)
    }

    fn publish_dynamic_transfer_to_downstream(
        frame: &mut DirectRunFrame,
        day_frame: &DirectDayFrame,
    ) -> Result<bool, DirectRuntimeError> {
        Self::publish_dynamic_transfer_to_downstream_with_ownership(frame, day_frame, false)
    }

    /// D15A (rev 27, `INV-OFEROUTE-009`): when `router_owns_surface` the
    /// SURFACE portion of the dynamic transfer is NOT published — the active
    /// router carries the inter-OFE surface water as the routed hydrograph
    /// handoff, and DC01's daily-lump surface admission must see zero (the
    /// double-feed guard in the routing step enforces it). The LATERAL
    /// (`ui_LfCrf`-lineage) carry is published unchanged: the router
    /// supersedes surface runon only (`GAP-OFEROUTE-006`).
    fn publish_dynamic_transfer_to_downstream_with_ownership(
        frame: &mut DirectRunFrame,
        day_frame: &DirectDayFrame,
        router_owns_surface: bool,
    ) -> Result<bool, DirectRuntimeError> {
        let (lane_id, upstream_lane_id, downstream_lane_id) = {
            let lane =
                frame
                    .lanes
                    .get(day_frame.lane_index)
                    .ok_or(DirectRuntimeError::LaneIndexOutOfRange {
                        lane_index: day_frame.lane_index,
                        lane_count: frame.lanes.len(),
                    })?;
            (lane.lane_id, lane.upstream_lane_id, lane.downstream_lane_id)
        };
        if downstream_lane_id == 0 {
            return Ok(false);
        }
        let downstream_index = usize::try_from(downstream_lane_id - 1).map_err(|_| {
            DirectRuntimeError::LaneIdOverflow {
                lane_index: day_frame.lane_index,
            }
        })?;
        let downstream_lane_count = frame.lanes.len();
        let downstream_lane =
            frame
                .lanes
                .get_mut(downstream_index)
                .ok_or(DirectRuntimeError::LaneIndexOutOfRange {
                    lane_index: downstream_index,
                    lane_count: downstream_lane_count,
                })?;
        if downstream_lane.upstream_lane_id != lane_id {
            return Err(DirectRuntimeError::InvalidLaneTopology {
                lane_index: day_frame.lane_index,
                lane_id,
                upstream_lane_id,
                downstream_lane_id,
            });
        }
        validate_nonnegative_direct_m(
            "dynamic_transfer.upstream_area_ratio",
            downstream_lane.upstream_area_ratio,
        )?;
        let subsurface = day_frame.subsurface_compute_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4O subsurface compute producer",
            },
        )?;
        let runoff = day_frame.runoff_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4A runoff partition producer",
            },
        )?;
        let mut surface_carry_m = [0.0; DIRECT_TRANSFER_HOUR_COUNT];
        let mut lateral_carry_m = [0.0; DIRECT_TRANSFER_HOUR_COUNT];
        validate_nonnegative_direct_m("dynamic_transfer.surface_runoff_m", runoff.q_runoff_m)?;
        // DC01 (INV-RUNOFFPART-031 / M2): the transferred TOTAL stays the exact
        // slot-0 lump (bitwise-identical R4J totals); the hourly DISTRIBUTION
        // rides separately as unit-normalized weights shaped by the WB14
        // infiltration-excess profile plus the hourly saturation carry. Nothing
        // consumes the weights until M3 supply admission.
        // D15A: with `router_owns_surface` the surface lump and weights stay
        // zero — the routed hydrograph handoff is the surface carrier.
        let surface_hourly_weights = if router_owns_surface {
            [0.0; DIRECT_TRANSFER_HOUR_COUNT]
        } else {
            surface_carry_m[0] = runoff.q_runoff_m;
            Self::dc01_surface_transfer_weights(
                runoff.q_runoff_m,
                &day_frame.wb14_hourly_excess_m,
                &subsurface.hourly_saturation_carry_m,
                day_frame
                    .snow_coupling_downstream_operands
                    .hourly_routed_melt_m
                    .as_ref(),
            )?
        };
        for (target, source) in lateral_carry_m
            .iter_mut()
            .zip(subsurface.hourly_lateral_carry_m.iter())
        {
            validate_nonnegative_direct_m(
                "dynamic_transfer.lateral_carry_m",
                *source,
            )?;
            *target = *source;
        }
        downstream_lane.transfer.surface_carry_m = surface_carry_m;
        downstream_lane.transfer.surface_hourly_weights = surface_hourly_weights;
        downstream_lane.transfer.lateral_carry_m = lateral_carry_m;
        downstream_lane.transfer.upstream_flow_m = 0.0;
        downstream_lane.transfer.subsurface_input_m = 0.0;
        DIRECT_AUDIT.record_downstream_operand_production();
        DIRECT_AUDIT.record_direct_state_mutation();
        Ok(true)
    }

    fn day_execution_failure(
        day_frame: &DirectDayFrame,
        lane_index: usize,
        day_index: usize,
        source: &DirectRuntimeError,
    ) -> DirectRuntimeError {
        let mut detail = source.to_string();
        if matches!(
            source,
            DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "hydrology_projection.aggregate_storage_delta_m"
                    | "hydrology_projection.frozen_layer_storage_m"
            }
        ) {
            if let (Some(storage), Some(evapotranspiration)) = (
                day_frame.storage_shadow_projection.as_ref(),
                day_frame
                    .evapotranspiration_compute_shadow_projection
                    .as_ref(),
            ) {
                if let Ok((aggregate_storage_from_layers_m, frozen_layer_storage_m)) =
                    projection::aggregate_storage_from_layers(
                        &evapotranspiration.layer_state_after_root_uptake,
                    )
                {
                    let aggregate_storage_delta_m =
                        aggregate_storage_from_layers_m - storage.storage_reconciled_m;
                    detail = format!(
                        "{detail}; aggregate_storage_from_layers_m={aggregate_storage_from_layers_m}; storage_reconciled_m={}; aggregate_storage_delta_m={aggregate_storage_delta_m}; frozen_layer_storage_m={frozen_layer_storage_m}; projected_frozen_soil_water_m={}; tolerance_m={}; storage_initial_m={}; precip_input_m={}; q_runoff_m={}; evapotranspiration_m={}; deep_seepage_m={}; subsurface_loss_m={}; frost_liquid_delta_m={}; liquid_input_m={}; cumulative_infiltration_m={}; depression_storage_delta_m={}; surface_saturation_runoff_m={}",
                        storage.storage_reconciled_m,
                        day_frame.hydrology_projection_inputs.frozen_soil_water_m,
                        day_frame.hydrology_projection_inputs.aggregate_storage_tolerance_m,
                        storage.storage_initial_m,
                        storage.precip_input_m,
                        storage.q_runoff_m,
                        storage.evapotranspiration_m,
                        storage.deep_seepage_m,
                        storage.subsurface_loss_m,
                        storage.frost_liquid_delta_m,
                        day_frame.liquid_input.liquid_input_m,
                        day_frame.infiltration_depression.cumulative_infiltration_m,
                        day_frame
                            .infiltration_depression
                            .depression_storage_delta_m,
                        day_frame.saturation_addback.surface_saturation_runoff_m
                    );
                }
            }
        } else if matches!(
            source,
            DirectRuntimeError::NegativeDirectValue {
                field: "runoff_partition.partition_runoff_m"
            }
        ) {
            detail = format!(
                "{detail}; liquid_input_m={}; runon_input_m={}; cumulative_infiltration_m={}; depression_storage_delta_m={}; surface_saturation_runoff_m={}; interception_m={}; storage_precip_input_m={}",
                day_frame.runoff_partition_inputs.liquid_input_m,
                day_frame.runoff_partition_inputs.runon_input_m,
                day_frame.runoff_partition_inputs.cumulative_infiltration_m,
                day_frame
                    .runoff_partition_inputs
                    .depression_storage_delta_m,
                day_frame
                    .runoff_partition_inputs
                    .surface_saturation_runoff_m,
                day_frame.interception_m,
                day_frame.storage_reconciliation_inputs.precip_input_m
            );
        }
        DirectRuntimeError::DirectDayExecutionFailure {
            lane_index,
            day_index,
            detail,
        }
    }

    #[must_use]
    const fn phase_lifecycle_status(phase: DirectPhaseKind) -> DirectPhaseLifecycleStatus {
        match phase {
            DirectPhaseKind::Normalization
            | DirectPhaseKind::StorageBounds
            | DirectPhaseKind::DecompositionTransition
            | DirectPhaseKind::ResiduePartitionTransition
            | DirectPhaseKind::AnnualGrowthTransition
            | DirectPhaseKind::PerennialGrowthTransition
            | DirectPhaseKind::PercolationDeepSeepage
            | DirectPhaseKind::Evapotranspiration
            | DirectPhaseKind::Drainage
            | DirectPhaseKind::LateralTransfer
            | DirectPhaseKind::PlantRootUptake
            | DirectPhaseKind::RunoffReconciliation
            | DirectPhaseKind::StorageReconciliation
            | DirectPhaseKind::ClosureDiagnostics => DirectPhaseLifecycleStatus::Executed,
        }
    }

    fn run_day_spans(
        day_frame: &mut DirectDayFrame,
        counters: &mut DirectExecutionCounters,
        winter_frost_compute_inputs: Option<&crate::hydrology::DirectWinterFrostComputeInputs>,
    ) -> Result<(), DirectRuntimeError> {
        Self::run_day_spans_hydrology(day_frame, counters, winter_frost_compute_inputs)?;
        Self::run_day_spans_erosion_and_ledger(day_frame, counters)
    }

    /// The day pipeline through the hydrology projection (everything before
    /// the erosion span). D15A (rev 27): split point for the active owner —
    /// the routed source operands (`wb14_hourly_excess`, the R4O hourly
    /// carries, routed melt, `q_runoff`) are all committed here, and the
    /// routing step must run after this half and before the erosion half so
    /// the D13 consumer sees the routed shape. The default path calls both
    /// halves back-to-back (identical span sequence).
    fn run_day_spans_hydrology(
        day_frame: &mut DirectDayFrame,
        counters: &mut DirectExecutionCounters,
        winter_frost_compute_inputs: Option<&crate::hydrology::DirectWinterFrostComputeInputs>,
    ) -> Result<(), DirectRuntimeError> {
        record_direct_span_report!(counters, day_frame.run_r5b_normalization_phase());
        record_direct_span_report!(counters, day_frame.run_r5b_storage_bounds_phase());
        record_direct_span_report!(counters, day_frame.run_r5c_decomposition_phase());
        record_direct_span_report!(counters, day_frame.run_r5c_residue_partition_phase());
        record_direct_span_report!(counters, day_frame.run_r5d_annual_growth_phase());
        record_direct_span_report!(counters, day_frame.run_r5d_perennial_growth_phase());
        record_direct_span_report!(counters, day_frame.run_r4c_storage_input_span());
        if day_frame.apply_r4w_winter_frost_ingress()? {
            counters.record_dynamic_transfer_publication();
        }
        record_direct_span_report!(counters, day_frame.run_r4i_liquid_input_span());
        record_direct_span_report!(counters, day_frame.run_r4j_runon_carry_span());
        record_direct_span_report!(counters, day_frame.run_r4k_infiltration_depression_span());
        record_direct_span_report!(counters, day_frame.run_r4m_percolation_span());
        record_direct_span_report!(counters, day_frame.run_r4n_surface_et_span());
        day_frame
            .project_r4x_winter_local_liquid_before_saturation(winter_frost_compute_inputs)?;
        record_direct_span_report!(counters, day_frame.run_r4o_subsurface_compute_span());
        record_direct_span_report!(counters, day_frame.run_r4n_root_uptake_span());
        record_direct_span_report!(counters, day_frame.run_r4g_snow_coupling_span());
        record_direct_span_report!(counters, day_frame.run_r4l_saturation_addback_span());
        record_direct_span_report!(
            counters,
            day_frame.run_r4a_runoff_partition_span_with_winter_frost(
                winter_frost_compute_inputs
            )
        );
        record_direct_span_report!(counters, day_frame.run_r7d6_peak_runoff_span());
        record_direct_span_report!(counters, day_frame.run_r4b_storage_reconciliation_span());
        record_direct_span_report!(counters, day_frame.run_r4pqz_hydrology_projection_span());

        Ok(())
    }

    /// The erosion span + the water-ledger span (the day pipeline tail after
    /// the D15A routing split point).
    fn run_day_spans_erosion_and_ledger(
        day_frame: &mut DirectDayFrame,
        counters: &mut DirectExecutionCounters,
    ) -> Result<(), DirectRuntimeError> {
        record_direct_span_report!(counters, day_frame.run_r7d6_erosion_span());
        record_direct_span_report!(counters, day_frame.run_r3b_water_ledger_span());

        Ok(())
    }
}
