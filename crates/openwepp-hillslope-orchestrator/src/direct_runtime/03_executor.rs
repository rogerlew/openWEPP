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
        mut build_day_input: F,
    ) -> Result<DirectPublicationExecution, DirectRuntimeError>
    where
        F: FnMut(
            &DirectRunFrame,
            usize,
            usize,
        ) -> Result<DirectPublicationDayInput, DirectRuntimeError>,
    {
        DIRECT_AUDIT.record_publication_capture_run();
        let expected_row_count = frame
            .identity
            .lane_count
            .checked_mul(frame.identity.day_count)
            .ok_or(DirectRuntimeError::DirectDomainViolation {
                field: "publication.expected_row_count",
            })?;
        let mut publication_frame =
            DirectRunPublicationFrame::new(frame.identity, metadata, expected_row_count);
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
                publication_frame.push_day_row(&day_frame, &day_input, lane)?;
                if Self::publish_dynamic_transfer_to_downstream(frame, &day_frame)? {
                    counters.record_dynamic_transfer_publication();
                }
                frame.commit_day_frame(&day_frame)?;
                counters.record_day_frame_commit();
            }
        }
        publication_frame.validate_complete()?;

        Ok(DirectPublicationExecution {
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
            publication_frame,
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

    fn publish_dynamic_transfer_to_downstream(
        frame: &mut DirectRunFrame,
        day_frame: &DirectDayFrame,
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
        surface_carry_m[0] = runoff.q_runoff_m;
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
        record_direct_span_report!(counters, day_frame.run_r5b_normalization_phase());
        record_direct_span_report!(counters, day_frame.run_r5b_storage_bounds_phase());
        record_direct_span_report!(counters, day_frame.run_r5c_decomposition_phase());
        record_direct_span_report!(counters, day_frame.run_r5c_residue_partition_phase());
        record_direct_span_report!(counters, day_frame.run_r5d_annual_growth_phase());
        record_direct_span_report!(counters, day_frame.run_r5d_perennial_growth_phase());
        record_direct_span_report!(counters, day_frame.run_r4c_storage_input_span());
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
        record_direct_span_report!(counters, day_frame.run_r7d6_erosion_span());
        record_direct_span_report!(counters, day_frame.run_r3b_water_ledger_span());

        Ok(())
    }
}
