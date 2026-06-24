#[allow(clippy::too_many_lines)]
pub(super) fn execute_scheduler_kernel_lifecycle(
    runtime_surface: HillslopeWritebackSurface,
    context: SchedulerLifecycleContext<'_>,
) -> Result<DailyExecutionResult, HillslopeCliError> {
    let mut runtime_surface = runtime_surface;
    seed_wb11_runtime_surface_inputs(&mut runtime_surface, context.execution_lane)?;
    seed_scheduler_calendar_symbols(&mut runtime_surface, &context);
    let pl_activation_sentinel = pl_runtime_activation_sentinel_value(&runtime_surface);
    prepare_pl_runtime_activation_for_scheduler(&mut runtime_surface)?;
    maybe_record_perfdeep02_frame_roundtrip(
        "single_pre_scheduler",
        None,
        &runtime_surface,
        &context,
    )?;
    let trace_day = context
        .hphys0245_trace_config
        .is_some_and(|config| config.includes_day(context.sim_day_index));
    let snow_runtime_before = trace_day.then(|| {
        Hphys0245SnowRuntimeBeforeState::from_surface(
            &runtime_surface,
            context.runtime_swe_before_m,
        )
    });
    let mut hphys0245_trace_rows = Vec::new();
    if trace_day {
        hphys0245_trace_rows.push(build_hphys0245_trace_row(
            context.run_name,
            context.simulation_year,
            context.sim_day_index,
            context.calendar_day.year,
            context.calendar_day.julian_day,
            "post_seed",
            None,
            &runtime_surface,
            None,
            snow_runtime_before,
        ));
    }

    let topology_graph = TopologyGraph::new(1, 0, 0, Vec::new());
    let topology_report = validate_pre_execution_topology(&topology_graph).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "execution_provenance",
            detail: format!(
                "{SIMPIPE_GUARD_ID} failed building topology precondition report: {error}"
            ),
        }
    })?;

    let scheduler = HillslopePhaseScheduler::canonical();
    let execution_report = if trace_day {
        let mut kernel = Hphys0245TelemetryKernel::new(
            context.run_name,
            context.simulation_year,
            context.sim_day_index,
            context.calendar_day.year,
            context.calendar_day.julian_day,
            snow_runtime_before,
        );
        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "execution_provenance",
                detail: format!("{SIMPIPE_GUARD_ID} scheduler/kernel lifecycle failed: {error}"),
            })?;
        hphys0245_trace_rows.extend(kernel.into_rows());
        report
    } else {
        let mut kernel = Wb11HydrologyKernel;
        scheduler
            .execute_with_kernel(&topology_report, &mut kernel, runtime_surface)
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "execution_provenance",
                detail: format!("{SIMPIPE_GUARD_ID} scheduler/kernel lifecycle failed: {error}"),
            })?
    };

    if !execution_report.scheduler_report.is_success() {
        let scheduler_status = &execution_report.scheduler_report.scheduler_status;
        let phase_context = execution_report
            .phase_reports
            .last()
            .map(|phase_report| {
                let mut context = format!(
                    ", last_phase={}, last_kernel_message_id={}, last_decision_outcome={:?}, last_decision_message_id={}",
                    phase_report.phase.as_str(),
                    phase_report.kernel_status.message_id(),
                    phase_report.decision_outcome,
                    phase_report.decision_status.message_id()
                );

                if !phase_report.decision_violations.is_empty() {
                    let violation_summary = phase_report
                        .decision_violations
                        .iter()
                        .take(3)
                        .map(|violation| {
                            format!(
                                "{}:{}:{:?}",
                                violation.check_id, violation.subject, violation.details
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(" | ");
                    context.push_str(", last_decision_violations=");
                    context.push_str(&violation_summary);
                }
                if phase_report.phase.as_str() == "storage_reconciliation" {
                    context.push_str(", wb12_terms=");
                    context.push_str(&format_wb12_storage_terms(
                        &execution_report.writeback_surface,
                    ));
                }
                if phase_report.phase.as_str() == "percolation_deep_seepage"
                    && phase_report.kernel_status.message_id() == "HKERNEL-WB11-PERC-E-003"
                {
                    context.push_str(", wb18_guard_terms=");
                    context.push_str(&format_wb18_perc_guard_terms(
                        &execution_report.writeback_surface,
                    ));
                }

                context
            })
            .unwrap_or_default();
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "execution_provenance",
            detail: format!(
                "{SIMPIPE_GUARD_ID} scheduler lifecycle did not complete successfully (outcome_class={}, status_class={:?}, boundary_class={}, message_id={}{})",
                scheduler_outcome_class_as_str(execution_report.scheduler_report.outcome_class),
                scheduler_status.classification(),
                scheduler_status.boundary_class().as_str(),
                scheduler_status.message_id(),
                phase_context
            ),
        });
    }

    let mut writeback_surface = execution_report.writeback_surface.clone();
    restore_pl_runtime_activation_sentinel_for_next_day(
        &mut writeback_surface,
        pl_activation_sentinel,
    );
    maybe_record_perfdeep02_frame_roundtrip(
        "single_post_scheduler",
        None,
        &writeback_surface,
        &context,
    )?;

    if trace_day {
        hphys0245_trace_rows.push(build_hphys0245_trace_row(
            context.run_name,
            context.simulation_year,
            context.sim_day_index,
            context.calendar_day.year,
            context.calendar_day.julian_day,
            "post_scheduler",
            None,
            &writeback_surface,
            None,
            snow_runtime_before,
        ));
    }

    let wb13_row = build_simulation_owned_wb13_row(
        &writeback_surface,
        context.publication_area_m2,
        context.simulation_year,
        context.sim_day_index,
        context.calendar_day,
        context.runtime_swe_before_m,
    )?;
    if trace_day {
        hphys0245_trace_rows.push(build_hphys0245_trace_row(
            context.run_name,
            context.simulation_year,
            context.sim_day_index,
            context.calendar_day.year,
            context.calendar_day.julian_day,
            "post_wb13",
            None,
            &writeback_surface,
            Some(&wb13_row),
            snow_runtime_before,
        ));
    }
    let coupling_vectors =
        build_simimpl10_coupling_vector_provenance(&writeback_surface, &wb13_row)?;
    let kernel_phase_message_ids = execution_report
        .phase_reports
        .iter()
        .map(|phase| phase.kernel_status.message_id().to_string())
        .collect::<Vec<_>>();
    let erod14_wave2_kernel_status_seen = execution_report.phase_reports.iter().any(|phase| {
        let message_id = phase.kernel_status.message_id();
        message_id.contains("EROD14-WAVE2")
            || message_id.contains("EROD18-ROUTE")
            || message_id.contains("EROD19-ROUTE")
    });

    Ok(DailyExecutionResult {
        scheduler_outcome_class: execution_report.scheduler_report.outcome_class,
        scheduler_status_message_id: execution_report
            .scheduler_report
            .scheduler_status
            .message_id()
            .to_string(),
        coupling_vectors,
        wb13_row,
        runtime_surface: writeback_surface,
        kernel_phase_message_ids,
        erod14_wave2_kernel_status_seen,
        hphys0245_trace_rows,
    })
}

struct PersistentLaneInputPreparation {
    lane_inputs: Vec<OfeLaneExecutionInput>,
    pl_activation_sentinels: Vec<Option<BoundaryValue>>,
    previous_storage_totals_mm: Vec<f64>,
}

fn prepare_persistent_lane_inputs(
    lane_state: &mut OfeLanePersistentStateSequence,
    climate_surface: &HillslopeWritebackSurface,
    stale_climate_symbols: &[BoundarySymbol],
    context: &SchedulerLifecycleContext<'_>,
) -> Result<PersistentLaneInputPreparation, HillslopeCliError> {
    let mut lane_inputs = Vec::with_capacity(lane_state.lane_states().len());
    let mut pl_activation_sentinels = Vec::with_capacity(lane_state.lane_states().len());
    let mut previous_storage_totals_mm = Vec::with_capacity(lane_state.lane_states().len());

    for lane in lane_state.lane_states_mut() {
        super::indexed_shadow_surface::observe_clone_source_surface(&lane.writeback_surface)?;
        let lane_ofe_id = lane.ofe_id;
        let upstream_area_ratio = lane.upstream_area_ratio;
        let mut lane_execution_input = lane.take_execution_input();
        let mut lane_surface = lane_execution_input.writeback_surface;
        for symbol in stale_climate_symbols {
            lane_surface.state_surface.remove(symbol);
            lane_surface.flux_surface.remove(symbol);
        }

        crate::hillslope::intake_lane_setup::extend_runtime_surface_from(
            &mut lane_surface,
            climate_surface,
        );
        seed_wb11_runtime_surface_inputs(&mut lane_surface, context.execution_lane)?;
        seed_scheduler_calendar_symbols(&mut lane_surface, context);
        previous_storage_totals_mm.push(internal_wb13_storage_total_mm_from_surface(
            &lane_surface,
        )?);
        pl_activation_sentinels.push(pl_runtime_activation_sentinel_value(&lane_surface));
        prepare_pl_runtime_activation_for_scheduler(&mut lane_surface)?;
        maybe_record_perfdeep02_frame_roundtrip(
            "mofe_pre_scheduler",
            Some(lane_ofe_id),
            &lane_surface,
            context,
        )?;
        if lane_execution_input.indexed_writeback_surface.is_some() {
            let symbol_registry =
                require_scheduler_symbol_registry(context, "indexed_runtime_surface")?;
            lane_execution_input.indexed_writeback_surface = Some(
                IndexedWritebackSurface::from_btreemap_surfaces(
                    symbol_registry,
                    &lane_surface.state_surface,
                    &lane_surface.flux_surface,
                )
                .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "indexed_runtime_surface",
                    detail: error.to_string(),
                })?,
            );
        }
        if let Some(lane_dense_state) = lane_execution_input.lane_dense_state.as_mut() {
            let symbol_registry =
                require_scheduler_symbol_registry(context, "perfdeep05_lane_dense_cached_slot_refresh")?;
            lane_dense_state
                .refresh_cached_slots_from_writeback_surface(
                    &lane_surface,
                    lane_execution_input.indexed_writeback_surface.as_ref(),
                    symbol_registry,
                )
                .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "perfdeep05_lane_dense_cached_slot_refresh",
                    detail: error.to_string(),
                })?;
        }
        let mut prepared_input = OfeLaneExecutionInput::with_upstream_area_ratio(
            lane_ofe_id,
            upstream_area_ratio,
            lane_surface,
        );
        prepared_input.indexed_writeback_surface =
            lane_execution_input.indexed_writeback_surface;
        prepared_input.lane_dense_state = lane_execution_input.lane_dense_state;
        lane_inputs.push(prepared_input);
    }

    Ok(PersistentLaneInputPreparation {
        lane_inputs,
        pl_activation_sentinels,
        previous_storage_totals_mm,
    })
}

fn execute_persistent_ofe_sequence(
    lane_inputs: Vec<OfeLaneExecutionInput>,
    context: &SchedulerLifecycleContext<'_>,
) -> Result<OfeLaneSequenceExecutionReport, HillslopeCliError> {
    let topology_graph = TopologyGraph::new(1, 0, 0, Vec::new());
    let topology_report = validate_pre_execution_topology(&topology_graph).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "per_ofe_dynamic_state",
            detail: format!(
                "{SIMPIPE_GUARD_ID} failed building persistent OFE topology precondition report: {error}"
            ),
        }
    })?;

    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;
    if context.indexed_scheduler_runtime_enabled {
        let (symbol_registry, hot_symbol_tables) =
            require_indexed_scheduler_resources(context, "per_ofe_dynamic_state")?;
        scheduler
            .execute_ofe_sequence_with_kernel_indexed(
                &topology_report,
                &mut kernel,
                lane_inputs,
                symbol_registry,
                hot_symbol_tables,
            )
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "per_ofe_dynamic_state",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} persistent OFE scheduler/kernel lifecycle failed: {error}"
                ),
            })
    } else {
        scheduler
            .execute_ofe_sequence_with_kernel(
            &topology_report,
            &mut kernel,
            lane_inputs,
            )
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "per_ofe_dynamic_state",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} persistent OFE scheduler/kernel lifecycle failed: {error}"
                ),
            })
    }
}

fn require_persistent_lane_sequence_success(
    sequence_report: &OfeLaneSequenceExecutionReport,
) -> Result<(), HillslopeCliError> {
    for lane_report in &sequence_report.lane_reports {
        if !lane_report.kernel_report.scheduler_report.is_success() {
            let scheduler_status = &lane_report.kernel_report.scheduler_report.scheduler_status;
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "per_ofe_dynamic_state",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} persistent OFE {} scheduler lifecycle did not complete successfully (outcome_class={}, status_class={:?}, boundary_class={}, message_id={})",
                    lane_report.ofe_id,
                    scheduler_outcome_class_as_str(
                        lane_report.kernel_report.scheduler_report.outcome_class
                    ),
                    scheduler_status.classification(),
                    scheduler_status.boundary_class().as_str(),
                    scheduler_status.message_id()
                ),
            });
        }
    }
    Ok(())
}

fn restore_persistent_lane_pl_sentinels(
    lane_state: &mut OfeLanePersistentStateSequence,
    pl_activation_sentinels: Vec<Option<BoundaryValue>>,
) {
    for (lane, sentinel) in lane_state
        .lane_states_mut()
        .iter_mut()
        .zip(pl_activation_sentinels)
    {
        restore_pl_runtime_activation_sentinel_for_next_day(&mut lane.writeback_surface, sentinel);
    }
}

fn persistent_outlet_runtime_surface(
    lane_state: &OfeLanePersistentStateSequence,
) -> Result<HillslopeWritebackSurface, HillslopeCliError> {
    lane_state
        .lane_states()
        .last()
        .map(|lane| lane.writeback_surface.clone())
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "per_ofe_dynamic_state",
            detail: format!("{SIMPIPE_GUARD_ID} persistent OFE state has no outlet lane"),
        })
}

fn persistent_kernel_phase_message_ids(
    sequence_report: &OfeLaneSequenceExecutionReport,
) -> Vec<String> {
    sequence_report
        .lane_reports
        .iter()
        .flat_map(|lane_report| {
            lane_report
                .kernel_report
                .phase_reports
                .iter()
                .map(|phase| phase.kernel_status.message_id().to_string())
        })
        .collect::<Vec<_>>()
}

fn persistent_erod14_wave2_kernel_status_seen(
    sequence_report: &OfeLaneSequenceExecutionReport,
) -> bool {
    sequence_report.lane_reports.iter().any(|lane_report| {
        lane_report
            .kernel_report
            .phase_reports
            .iter()
            .any(|phase| {
                let message_id = phase.kernel_status.message_id();
                message_id.contains("EROD14-WAVE2")
                    || message_id.contains("EROD18-ROUTE")
                    || message_id.contains("EROD19-ROUTE")
            })
    })
}

struct PersistentSequenceSummary {
    scheduler_outcome_class: SchedulerOutcomeClass,
    scheduler_status_message_id: String,
    kernel_phase_message_ids: Vec<String>,
    erod14_wave2_kernel_status_seen: bool,
}

fn persistent_sequence_summary(
    sequence_report: &OfeLaneSequenceExecutionReport,
) -> Result<PersistentSequenceSummary, HillslopeCliError> {
    let last_lane_report =
        sequence_report
            .lane_reports
            .last()
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "per_ofe_dynamic_state",
                detail: format!("{SIMPIPE_GUARD_ID} persistent OFE sequence produced no lanes"),
            })?;

    Ok(PersistentSequenceSummary {
        scheduler_outcome_class: last_lane_report.kernel_report.scheduler_report.outcome_class,
        scheduler_status_message_id: last_lane_report
            .kernel_report
            .scheduler_report
            .scheduler_status
            .message_id()
            .to_string(),
        kernel_phase_message_ids: persistent_kernel_phase_message_ids(sequence_report),
        erod14_wave2_kernel_status_seen: persistent_erod14_wave2_kernel_status_seen(
            sequence_report,
        ),
    })
}

fn replace_persistent_lane_state_from_report_moving(
    lane_state: &mut OfeLanePersistentStateSequence,
    sequence_report: OfeLaneSequenceExecutionReport,
) -> Result<(), HillslopeCliError> {
    let lane_reports = sequence_report.lane_reports;

    if lane_state.lane_states().len() != lane_reports.len() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "per_ofe_dynamic_state",
            detail: format!(
                "{SIMPIPE_GUARD_ID} persistent OFE state replacement failed: expected {} lanes, observed {} lanes",
                lane_state.lane_states().len(),
                lane_reports.len()
            ),
        });
    }

    for (state, lane_report) in lane_state.lane_states_mut().iter_mut().zip(lane_reports) {
        if state.ofe_id != lane_report.ofe_id {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "per_ofe_dynamic_state",
                detail: format!(
                    "{SIMPIPE_GUARD_ID} persistent OFE state replacement failed: expected OFE {}, observed OFE {}",
                    state.ofe_id,
                    lane_report.ofe_id
                ),
            });
        }

        state.writeback_surface = lane_report.kernel_report.writeback_surface;
        state.replace_lane_dense_state(lane_report.lane_dense_state);
    }

    Ok(())
}

fn refresh_persistent_lane_indexed_authority(
    lane_state: &mut OfeLanePersistentStateSequence,
    registry: &SymbolRegistry,
) -> Result<(), HillslopeCliError> {
    lane_state
        .refresh_indexed_writeback_authority(registry)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "indexed_runtime_surface",
            detail: error.to_string(),
        })
}

fn require_scheduler_symbol_registry<'a>(
    context: &SchedulerLifecycleContext<'a>,
    surface: &'static str,
) -> Result<&'a SymbolRegistry, HillslopeCliError> {
    context
        .symbol_registry
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface,
            detail: format!("{SIMPIPE_GUARD_ID} indexed scheduler symbol registry is not active"),
        })
}

fn require_indexed_scheduler_resources<'a>(
    context: &SchedulerLifecycleContext<'a>,
    surface: &'static str,
) -> Result<(&'a SymbolRegistry, &'a HotSymbolTables), HillslopeCliError> {
    let symbol_registry = require_scheduler_symbol_registry(context, surface)?;
    let hot_symbol_tables =
        context
            .hot_symbol_tables
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface,
                detail: format!("{SIMPIPE_GUARD_ID} indexed scheduler hot tables are not active"),
            })?;
    Ok((symbol_registry, hot_symbol_tables))
}

pub(super) fn execute_persistent_scheduler_kernel_lifecycle(
    lane_state: &mut OfeLanePersistentStateSequence,
    climate_surface: &HillslopeWritebackSurface,
    stale_climate_symbols: &[BoundarySymbol],
    lane_areas_m2: &[f64],
    runoff_publication_geometries: &[Wb13RunoffPublicationGeometry],
    context: SchedulerLifecycleContext<'_>,
) -> Result<PersistentDailyExecutionResult, HillslopeCliError> {
    let lane_preparation = prepare_persistent_lane_inputs(
        lane_state,
        climate_surface,
        stale_climate_symbols,
        &context,
    )?;
    let sequence_report = execute_persistent_ofe_sequence(lane_preparation.lane_inputs, &context)?;
    require_persistent_lane_sequence_success(&sequence_report)?;
    let internal_wb13_collection = DailyInternalPerOfeWb13Collection::from_sequence_report(
        &sequence_report,
        lane_areas_m2,
        runoff_publication_geometries,
        &lane_preparation.previous_storage_totals_mm,
        context,
    )?;
    let sequence_summary = persistent_sequence_summary(&sequence_report)?;
    maybe_record_r7h_compat_layer_trace(&sequence_report, &context)?;
    replace_persistent_lane_state_from_report_moving(lane_state, sequence_report)?;
    restore_persistent_lane_pl_sentinels(lane_state, lane_preparation.pl_activation_sentinels);
    if context.indexed_scheduler_runtime_enabled {
        let symbol_registry =
            require_scheduler_symbol_registry(&context, "indexed_runtime_surface")?;
        refresh_persistent_lane_indexed_authority(lane_state, symbol_registry)?;
    }

    let outlet_runtime_surface = persistent_outlet_runtime_surface(lane_state)?;
    super::indexed_shadow_surface::validate_shadow_surface(&outlet_runtime_surface)?;
    let outlet_row = internal_wb13_collection
        .outlet_row()
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "per_ofe_internal_wb13",
            detail: format!("{SIMPIPE_GUARD_ID} internal WB13 collection has no outlet row"),
        })?;
    let coupling_vectors =
        build_simimpl10_coupling_vector_provenance(&outlet_runtime_surface, outlet_row)?;

    Ok(PersistentDailyExecutionResult {
        scheduler_outcome_class: sequence_summary.scheduler_outcome_class,
        scheduler_status_message_id: sequence_summary.scheduler_status_message_id,
        coupling_vectors,
        runtime_surface: outlet_runtime_surface,
        internal_wb13_collection,
        kernel_phase_message_ids: sequence_summary.kernel_phase_message_ids,
        erod14_wave2_kernel_status_seen: sequence_summary.erod14_wave2_kernel_status_seen,
        hphys0245_trace_rows: Vec::new(),
    })
}

pub(super) fn pl_runtime_activation_sentinel_value(
    runtime_surface: &HillslopeWritebackSurface,
) -> Option<BoundaryValue> {
    runtime_surface
        .state_surface
        .get(&BoundarySymbol::from("pl_schedule_slot_count"))
        .copied()
}

#[allow(clippy::too_many_lines)]
fn maybe_record_r7h_compat_layer_trace(
    sequence_report: &OfeLaneSequenceExecutionReport,
    context: &SchedulerLifecycleContext<'_>,
) -> Result<(), HillslopeCliError> {
    let Some(trace_path) = std::env::var_os("OPENWEPP_R7H_COMPAT_LAYER_TRACE_PATH") else {
        return Ok(());
    };
    if r7h_optional_usize_env("OPENWEPP_R7H_COMPAT_LAYER_TRACE_DAY_INDEX")?
        .is_some_and(|day_index| day_index != context.sim_day_index)
    {
        return Ok(());
    }
    let lane_filter = r7h_optional_usize_env("OPENWEPP_R7H_COMPAT_LAYER_TRACE_LANE_INDEX")?;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&trace_path)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "r7h_compat_layer_trace",
            detail: format!(
                "{SIMPIPE_GUARD_ID} failed opening compatibility layer trace {}: {error}",
                std::path::PathBuf::from(&trace_path).display()
            ),
        })?;

    for (lane_index, lane_report) in sequence_report.lane_reports.iter().enumerate() {
        if lane_filter.is_some_and(|filter| filter != lane_index) {
            continue;
        }
        let surface = &lane_report.kernel_report.writeback_surface;
        let layer_count = r7h_trace_layer_count(surface);
        let mut row = serde_json::json!({
            "schema": "openwepp-r7h-compat-layer-trace-v1",
            "run_name": context.run_name,
            "simulation_year": context.simulation_year,
            "calendar_year": context.calendar_day.year,
            "julian_day": context.calendar_day.julian_day,
            "day_index": context.sim_day_index,
            "lane_index": lane_index,
            "ofe_id": lane_report.ofe_id,
            "layer_count": layer_count,
            "wb11_soil_water_m": r7h_state_scalar(surface, "wb11_soil_water"),
            "root_depth_m": r7h_state_scalar(surface, "rtd"),
            "leaf_area_index": r7h_state_scalar(surface, "lai"),
            "canopy_cover_fraction": r7h_state_scalar(surface, "cancov"),
            "residue_interception_m": r7h_state_scalar(surface, "wb17_residue_interception"),
            "plant_tolerance": r7h_state_scalar(surface, "pltol"),
            "effective_plant_tolerance": r7h_state_scalar(surface, "swu_effective_pltol"),
            "pmet_soil_evaporation_state_m": r7h_state_scalar(surface, "pmet.es_m"),
            "pmet_plant_transpiration_state_m": r7h_state_scalar(surface, "pmet.ep_m"),
            "pmet_soil_evaporation_flux_m": r7h_flux_scalar(surface, "pmet.es_m"),
            "pmet_plant_transpiration_flux_m": r7h_flux_scalar(surface, "pmet.ep_m"),
            "et_m": r7h_flux_scalar(surface, "ET"),
            "water_stress": r7h_flux_scalar(surface, "Ws"),
            "plant_transpiration_m": r7h_flux_scalar(surface, "Ep"),
            "soil_evaporation_m": r7h_flux_scalar(surface, "Es"),
            "residue_evaporation_m": r7h_flux_scalar(surface, "Er"),
            "uptake_potential_m": r7h_flux_scalar(surface, "UPi"),
            "uptake_actual_m": r7h_flux_scalar(surface, "Ui"),
            "runoff_m": r7h_flux_scalar(surface, "Q"),
            "subsurface_loss_m": r7h_flux_scalar(surface, "Qd"),
            "drainage_m": r7h_flux_scalar(surface, "Qdd"),
            "percolation_loss_m": r7h_flux_scalar(surface, "D"),
            "lateral_q_m": r7h_flux_scalar(surface, "q"),
            "layer_theta_m": r7h_layer_state_array(surface, "theta", layer_count),
            "layer_upper_limit_m": r7h_layer_state_array(surface, "ul", layer_count),
            "layer_field_capacity_m": r7h_layer_state_array(surface, "fc", layer_count),
            "layer_frozen_water_m": r7h_layer_state_array(surface, "frzw", layer_count),
            "layer_frozen_depth_m": r7h_layer_state_array(surface, "frozen_depth", layer_count),
            "layer_depth_m": r7h_layer_depth_array(surface, layer_count),
            "layer_uptake_potential_m": r7h_layer_flux_array(surface, "UPi", layer_count),
            "layer_uptake_actual_m": r7h_layer_flux_array(surface, "Ui", layer_count),
        });
        if let Some(row_object) = row.as_object_mut() {
            row_object.insert(
                "infiltration_m".to_string(),
                serde_json::json!(r7h_state_scalar(surface, "wb12_infiltration")),
            );
            row_object.insert(
                "same_pass_lineage".to_string(),
                serde_json::json!(r7h_state_scalar(
                    surface,
                    "wb12_infiltration_same_pass_lineage"
                )),
            );
            row_object.insert(
                "percolation_lane_substeps".to_string(),
                serde_json::json!(r7h_state_scalar(surface, "wb18_perc_lane_substeps")),
            );
            row_object.insert(
                "percolation_layer_flux_m".to_string(),
                serde_json::json!(r7h_layer_flux_array(
                    surface,
                    "wb18_perc_pei",
                    layer_count
                )),
            );
            r7h_insert_scalar(row_object, "storage_initial_m", r7h_state_scalar(surface, "wb12_storage_initial"));
            r7h_insert_scalar(row_object, "storage_reconciled_m", r7h_state_scalar(surface, "wb12_storage_reconciled"));
            r7h_insert_scalar(row_object, "precip_input_m", r7h_state_scalar(surface, "wb12_precip_input"));
            r7h_insert_scalar(row_object, "snow_coupling_s_m", r7h_surface_scalar_prefer_flux(surface, "S"));
            r7h_insert_scalar(row_object, "irrigation_input_m", r7h_surface_scalar_prefer_flux(surface, "Irr"));
            r7h_insert_scalar(
                row_object,
                "runon_input_m",
                r7h_surface_scalar_prefer_flux(surface, "wb12_runoff_carryover")
                    .or_else(|| r7h_state_scalar(surface, "wb12_runon_input")),
            );
            r7h_insert_scalar(row_object, "interception_m", r7h_surface_scalar_prefer_flux(surface, "I"));
            r7h_insert_scalar(
                row_object,
                "frost_frwatc_net_liquid_delta_m",
                r7h_surface_scalar_prefer_flux(surface, "frost.runtime_frwatc_net_liquid_delta_m"),
            );
            r7h_insert_scalar(
                row_object,
                "frost_frwatc_soil_water_before_m",
                r7h_surface_scalar_prefer_flux(surface, "frost.runtime_frwatc_soil_water_before_m"),
            );
            r7h_insert_scalar(
                row_object,
                "frost_frwatc_soil_water_after_m",
                r7h_surface_scalar_prefer_flux(surface, "frost.runtime_frwatc_soil_water_after_m"),
            );
            r7h_insert_scalar(
                row_object,
                "frost_frwatc_frozen_water_before_m",
                r7h_surface_scalar_prefer_flux(
                    surface,
                    "frost.runtime_frwatc_frozen_water_before_m",
                ),
            );
            r7h_insert_scalar(
                row_object,
                "frost_frwatc_frozen_water_after_m",
                r7h_surface_scalar_prefer_flux(
                    surface,
                    "frost.runtime_frwatc_frozen_water_after_m",
                ),
            );
            r7h_insert_scalar(
                row_object,
                "frost_frwatc_freeze_debit_m",
                r7h_surface_scalar_prefer_flux(surface, "frost.runtime_frwatc_freeze_debit_m"),
            );
            r7h_insert_scalar(
                row_object,
                "frost_frwatc_thaw_credit_m",
                r7h_surface_scalar_prefer_flux(surface, "frost.runtime_frwatc_thaw_credit_m"),
            );
            r7h_insert_scalar(
                row_object,
                "frost_shadow_frwatc_residual_m",
                r7h_surface_scalar_prefer_flux(surface, "frost.runtime_shadow_frwatc_residual_m"),
            );
            r7h_insert_scalar(row_object, "frost_watpdg_m", r7h_surface_scalar_prefer_flux(surface, "frost.runtime_watpdg_m"));
            r7h_insert_scalar(row_object, "frost_watbtm_m", r7h_surface_scalar_prefer_flux(surface, "frost.runtime_watbtm_m"));
        }
        let row_line = format!("{row}\n");
        std::io::Write::write_all(&mut file, row_line.as_bytes()).map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "r7h_compat_layer_trace",
            detail: format!(
                "{SIMPIPE_GUARD_ID} failed writing compatibility layer trace {}: {error}",
                std::path::PathBuf::from(&trace_path).display()
            ),
        })?;
    }

    Ok(())
}

fn r7h_optional_usize_env(name: &'static str) -> Result<Option<usize>, HillslopeCliError> {
    let Some(raw) = std::env::var_os(name) else {
        return Ok(None);
    };
    let raw = raw.to_string_lossy();
    raw.parse::<usize>()
        .map(Some)
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "r7h_compat_layer_trace",
            detail: format!("{SIMPIPE_GUARD_ID} invalid {name}={raw}: {error}"),
        })
}

fn r7h_trace_layer_count(surface: &HillslopeWritebackSurface) -> usize {
    let mut layer_count = 0;
    for layer_index in 1..=256 {
        if r7h_layer_has_any_state(surface, layer_index) {
            layer_count = layer_index;
        }
    }
    layer_count
}

fn r7h_layer_has_any_state(surface: &HillslopeWritebackSurface, layer_index: usize) -> bool {
    ["theta", "ul", "fc", "frzw", "frozen_depth"]
        .iter()
        .any(|field| {
            surface
                .state_surface
                .contains_key(&BoundarySymbol::from(format!(
                    "wb18_perc_{field}_{layer_index:04}"
                )))
        })
        || surface
            .state_surface
            .contains_key(&BoundarySymbol::from(format!("wb19_dg_{layer_index:04}")))
}

fn r7h_state_scalar(surface: &HillslopeWritebackSurface, symbol: &str) -> Option<f64> {
    surface
        .state_surface
        .get(&BoundarySymbol::from(symbol))
        .copied()
        .map(BoundaryValue::as_f64)
}

fn r7h_flux_scalar(surface: &HillslopeWritebackSurface, symbol: &str) -> Option<f64> {
    surface
        .flux_surface
        .get(&BoundarySymbol::from(symbol))
        .copied()
        .map(BoundaryValue::as_f64)
}

fn r7h_surface_scalar_prefer_flux(
    surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Option<f64> {
    r7h_flux_scalar(surface, symbol).or_else(|| r7h_state_scalar(surface, symbol))
}

fn r7h_insert_scalar(
    row_object: &mut serde_json::Map<String, serde_json::Value>,
    key: &str,
    value: Option<f64>,
) {
    row_object.insert(key.to_string(), serde_json::json!(value));
}

fn r7h_layer_state_array(
    surface: &HillslopeWritebackSurface,
    field: &str,
    layer_count: usize,
) -> Vec<Option<f64>> {
    (1..=layer_count)
        .map(|layer_index| {
            r7h_state_scalar(surface, &format!("wb18_perc_{field}_{layer_index:04}"))
        })
        .collect()
}

fn r7h_layer_depth_array(
    surface: &HillslopeWritebackSurface,
    layer_count: usize,
) -> Vec<Option<f64>> {
    (1..=layer_count)
        .map(|layer_index| r7h_state_scalar(surface, &format!("wb19_dg_{layer_index:04}")))
        .collect()
}

fn r7h_layer_flux_array(
    surface: &HillslopeWritebackSurface,
    root: &str,
    layer_count: usize,
) -> Vec<Option<f64>> {
    (1..=layer_count)
        .map(|layer_index| r7h_flux_scalar(surface, &format!("{root}_{layer_index:04}")))
        .collect()
}

pub(super) fn seed_scheduler_calendar_symbols(
    runtime_surface: &mut HillslopeWritebackSurface,
    context: &SchedulerLifecycleContext<'_>,
) {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("year"),
        BoundaryValue::scalar(f64::from(context.simulation_year)),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("day"),
        BoundaryValue::scalar(f64::from(context.calendar_day.julian_day)),
    );
}

pub(super) fn restore_pl_runtime_activation_sentinel_for_next_day(
    runtime_surface: &mut HillslopeWritebackSurface,
    sentinel_value: Option<BoundaryValue>,
) {
    if let Some(value) = sentinel_value {
        runtime_surface
            .state_surface
            .entry(BoundarySymbol::from("pl_schedule_slot_count"))
            .or_insert(value);
    }
}

pub(super) fn prepare_pl_runtime_activation_for_scheduler(
    runtime_surface: &mut HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
    const PL_SCHEDULE_SLOT_COUNT_SYMBOL: &str = "pl_schedule_slot_count";

    if runtime_surface_symbol_value(runtime_surface, PL_SCHEDULE_SLOT_COUNT_SYMBOL).is_none() {
        return Ok(());
    }

    if pl_runtime_has_active_crop_for_scheduler_day(runtime_surface)? {
        return Ok(());
    }

    runtime_surface
        .state_surface
        .remove(&BoundarySymbol::from(PL_SCHEDULE_SLOT_COUNT_SYMBOL));
    Ok(())
}

pub(super) fn pl_runtime_has_active_crop_for_scheduler_day(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<bool, HillslopeCliError> {
    let slot_count = require_runtime_usize_in_range(runtime_surface, "pl_schedule_slot_count", 1)?;
    let rotation_years =
        require_runtime_usize_in_range(runtime_surface, "pl_schedule_rotation_years", 1)?;
    let rotation_repeats =
        require_runtime_usize_in_range(runtime_surface, "pl_schedule_rotation_repeats", 1)?;
    let runtime_year = require_runtime_usize_in_range(runtime_surface, "year", 1)?;
    let day_of_year = require_runtime_usize_in_range(runtime_surface, "day", 1)?;
    if day_of_year > 366 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!("day must be in 1..=366 for PL activation, observed {day_of_year}"),
        });
    }

    let max_runtime_year = rotation_repeats
        .checked_mul(rotation_years)
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: "rotation_repeats * rotation_years overflowed".to_string(),
        })?;
    if runtime_year > max_runtime_year {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "year must be in 1..={max_runtime_year} for PL activation, observed {runtime_year}"
            ),
        });
    }

    let rotation_index = ((runtime_year - 1) / rotation_years) + 1;
    let year_in_rotation = ((runtime_year - 1) % rotation_years) + 1;
    let mut slot_candidates = Vec::new();
    for slot_index in 1..=slot_count {
        let ofe_index = require_runtime_usize_in_range(
            runtime_surface,
            &pl_schedule_slot_symbol("ofe_index", slot_index),
            1,
        )?;
        if ofe_index != 1 {
            continue;
        }
        let slot_year_in_rotation = require_runtime_usize_in_range(
            runtime_surface,
            &pl_schedule_slot_symbol("year_in_rotation", slot_index),
            1,
        )?;
        let slot_rotation_index = require_runtime_usize_in_range(
            runtime_surface,
            &pl_schedule_slot_symbol("rotation_index", slot_index),
            1,
        )?;
        if slot_year_in_rotation == year_in_rotation && slot_rotation_index == rotation_index {
            slot_candidates.push(slot_index);
        }
    }

    let [slot_index] = slot_candidates.as_slice() else {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "expected exactly one active PL slot for ofe=1 year_in_rotation={year_in_rotation} rotation_index={rotation_index}, observed {slot_candidates:?}"
            ),
        });
    };

    let crop_slots = require_runtime_usize_in_range(
        runtime_surface,
        &pl_schedule_slot_symbol("crop_slots", *slot_index),
        1,
    )?;
    let mut active_crop_count = 0usize;
    for crop_slot_index in 1..=crop_slots {
        if pl_crop_slot_is_active_for_day(
            runtime_surface,
            *slot_index,
            crop_slot_index,
            day_of_year,
        )? {
            active_crop_count += 1;
        }
    }

    match active_crop_count {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "expected at most one active PL crop for slot {slot_index} day {day_of_year}, observed {active_crop_count}"
            ),
        }),
    }
}

pub(super) fn pl_crop_slot_is_active_for_day(
    runtime_surface: &HillslopeWritebackSurface,
    slot_index: usize,
    crop_slot_index: usize,
    day_of_year: usize,
) -> Result<bool, HillslopeCliError> {
    let imngmt = require_runtime_usize_in_range(
        runtime_surface,
        &pl_schedule_slot_crop_symbol("imngmt", slot_index, crop_slot_index),
        1,
    )?;
    if imngmt > 3 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!("imngmt must be in 1..=3 for PL activation, observed {imngmt}"),
        });
    }

    let jdplt = require_runtime_usize_in_range(
        runtime_surface,
        &pl_growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index),
        0,
    )?;
    if jdplt > 366 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!("jdplt must be in 0..=366 for PL activation, observed {jdplt}"),
        });
    }
    let jdharv = require_runtime_usize_in_range(
        runtime_surface,
        &pl_growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index),
        0,
    )?;
    if jdharv > 366 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!("jdharv must be in 0..=366 for PL activation, observed {jdharv}"),
        });
    }

    let (active_end, jdstop) = if imngmt == 2 {
        let jdstop = require_runtime_usize_in_range(
            runtime_surface,
            &pl_growth_slot_crop_symbol("jdstop", slot_index, crop_slot_index),
            0,
        )?;
        if jdstop > 366 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "pl_runtime_activation",
                detail: format!("jdstop must be in 0..=366 for PL activation, observed {jdstop}"),
            });
        }
        if jdplt == 0 {
            return Ok(jdstop == 0 || day_of_year <= jdstop);
        }
        let active_end = if jdstop == 0 { jdharv.max(1) } else { jdstop };
        (active_end, jdstop)
    } else {
        (jdharv.max(1), 0)
    };

    if jdplt == 0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "jdplt must be in 1..=366 for non-perennial PL activation, observed jdplt={jdplt} jdharv={jdharv} jdstop={jdstop}"
            ),
        });
    }

    Ok(day_is_within_julian_window(day_of_year, jdplt, active_end))
}

pub(super) fn require_runtime_usize_in_range(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
    min_allowed: usize,
) -> Result<usize, HillslopeCliError> {
    let value = require_runtime_surface_scalar(runtime_surface, symbol)?;
    let value = scalar_to_usize(symbol, value)?;
    if value < min_allowed {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "pl_runtime_activation",
            detail: format!(
                "{symbol} must be >= {min_allowed} for PL activation, observed {value}"
            ),
        });
    }
    Ok(value)
}

pub(super) fn day_is_within_julian_window(day_of_year: usize, start_day: usize, end_day: usize) -> bool {
    if start_day <= end_day {
        day_of_year >= start_day && day_of_year <= end_day
    } else {
        day_of_year >= start_day || day_of_year <= end_day
    }
}
