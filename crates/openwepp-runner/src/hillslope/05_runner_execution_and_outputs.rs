fn execute_hillslope_climate_days(
    run_name: &str,
    output_hillslope_id: u32,
    runtime_selection: HillslopeRuntimeSelection,
    state: HillslopeClimateExecutionState,
    climate: &ClimateFile,
) -> Result<HillslopeClimateExecution, HillslopeCliError> {
    let climate_request = build_hillslope_climate_runtime_request(climate).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: error.to_string(),
        }
    })?;
    let HillslopeClimateExecutionState {
        publication_area_m2,
        contributor_ofe_count,
        static_per_ofe_slice_count,
        per_ofe_lane_areas_m2,
        per_ofe_runoff_publication_geometries,
        runtime_surface,
        lane_context,
        climate_span,
        mut persistent_lane_state,
        symbol_registry,
        hot_symbol_tables,
    } = state;
    let symbol_registry = symbol_registry.as_ref();
    let hot_symbol_tables = hot_symbol_tables.as_ref();
    let indexed_scheduler_runtime_enabled = symbol_registry.is_some() && hot_symbol_tables.is_some();
    let persistent_lane_active = persistent_lane_state.is_some();
    let hphys0245_trace_config = hphys0245_trace_config_from_env()?;
    let retained_direct_publication = build_retained_direct_publication_frame(
        &RetainedDirectPublicationRequest {
            runtime_selection,
            run_name,
            output_hillslope_id,
            execution_lane: lane_context.lane,
            lane_areas_m2: &per_ofe_lane_areas_m2,
            climate_request: &climate_request,
            climate_span: &climate_span,
            static_runtime_surface: &runtime_surface,
        },
    )?;
    let context = ClimateExecutionContext {
        run_name,
        output_hillslope_id,
        lane: lane_context.lane,
        publication_area_m2,
        first_calendar_year: climate_span.first_day.year,
        hphys0245_trace_config: hphys0245_trace_config.as_ref(),
        symbol_registry,
        hot_symbol_tables,
        indexed_scheduler_runtime_enabled,
    };
    let mut accumulator = ClimateExecutionAccumulator::new(
        runtime_surface,
        climate_span.days.len(),
        contributor_ofe_count,
        retained_direct_publication,
    )?;

    for (day_index, day_projection) in climate_span.days.iter().enumerate() {
        let climate_surface = build_day_climate_surface(
            &climate_request,
            day_index,
            &accumulator.runtime_surface,
            day_projection,
        )?;
        let stale_climate_symbols = accumulator.previous_climate_symbols.clone();
        remove_stale_climate_symbols(&mut accumulator.runtime_surface, &stale_climate_symbols);
        let simulation_year =
            simulation_year_from_calendar_year(day_projection.year, context.first_calendar_year)?;
        accumulator.previous_climate_symbols.clear();
        accumulator
            .previous_climate_symbols
            .extend(climate_surface.state_surface.keys().cloned());
        let mut apply = HillslopeDayApply {
            persistent_lane_state: &mut persistent_lane_state,
            climate_surface,
            stale_climate_symbols: &stale_climate_symbols,
            per_ofe_lane_areas_m2: &per_ofe_lane_areas_m2,
            per_ofe_runoff_publication_geometries: &per_ofe_runoff_publication_geometries,
            context,
            day_index,
            day_projection,
            simulation_year,
            runtime_swe_before_m: accumulator.runtime_swe_publication_state_m,
        };
        accumulator.apply_hillslope_day(&mut apply)?;
    }

    let executed_day_count = climate_span.days.len();
    accumulator.finish(ClimateExecutionCompletion {
        selected_lane: lane_context.lane,
        publication_area_m2,
        contributor_ofe_count,
        static_per_ofe_slice_count,
        persistent_lane_active,
        climate_span,
        hphys0245_trace_config,
        executed_day_count,
    })
}

fn execute_hillslope_direct_production_days(
    run_name: &str,
    output_hillslope_id: u32,
    state: HillslopeClimateExecutionState,
    climate: &ClimateFile,
) -> Result<HillslopeClimateExecution, HillslopeCliError> {
    let climate_request = build_hillslope_climate_runtime_request(climate).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: error.to_string(),
        }
    })?;
    let HillslopeClimateExecutionState {
        publication_area_m2,
        contributor_ofe_count,
        static_per_ofe_slice_count,
        per_ofe_lane_areas_m2,
        per_ofe_runoff_publication_geometries,
        runtime_surface,
        lane_context,
        climate_span,
        persistent_lane_state,
        symbol_registry: _,
        hot_symbol_tables: _,
    } = state;
    let lane_seed_surfaces = direct_production_lane_seed_surfaces(
        &runtime_surface,
        persistent_lane_state.as_ref(),
        per_ofe_lane_areas_m2.len(),
    )?;
    let mut frame = build_direct_production_run_frame(&DirectProductionRunFrameBuildInputs {
        output_hillslope_id,
        lane_areas_m2: &per_ofe_lane_areas_m2,
        runoff_publication_geometries: &per_ofe_runoff_publication_geometries,
        day_count: climate_span.days.len(),
        climate_request: &climate_request,
        climate_span: &climate_span,
        climate_context_surface: &runtime_surface,
        lane_seed_surfaces: &lane_seed_surfaces,
        execution_lane: lane_context.lane,
    })?;
    let day_input_builder =
        DirectPublicationDayInputBuilder::new_with_seed_surfaces_and_erosion_guard(
        &climate_request,
        &climate_span,
        lane_seed_surfaces,
        &runtime_surface,
        lane_context.lane,
        true,
    )?;
    let metadata = DirectPublicationRunMetadata {
        run_name: run_name.to_string(),
        runtime_selection: HillslopeRuntimeSelection::DirectProductionExecutor
            .as_str()
            .to_string(),
        output_policy: direct_publication_output_policy(
            HillslopeRuntimeSelection::DirectProductionExecutor,
        )
        .to_string(),
    };
    let direct_execution = DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect)
        .run_publication_capture_with_interleaved_day_inputs(
            &mut frame,
            metadata,
            |frame, day_index, lane_index| {
                day_input_builder
                    .build(frame, day_index, lane_index)
                    .map_err(|error| direct_publication_day_input_build_error(&error))
            },
        )
        .map_err(|source| direct_production_runtime_error(&source))?;
    let coupling_vectors = build_direct_production_coupling_vector_provenance(
        &runtime_surface,
        &frame,
        &direct_execution.publication_frame,
    )?;
    let executed_day_count = climate_span.days.len();
    let persistent_lane_active = persistent_lane_state.is_some();

    Ok(HillslopeClimateExecution {
        selected_lane: lane_context.lane,
        publication_area_m2,
        contributor_ofe_count,
        static_per_ofe_slice_count,
        persistent_lane_active,
        runtime_surface,
        climate_span,
        wb13_rows: Vec::new(),
        pass_rows: Vec::new(),
        coupling_vectors,
        erod14_wave2_kernel_status_seen: false,
        scheduler_outcome_class: SchedulerOutcomeClass::Completed,
        scheduler_status_message_id: "R7C-DIRECT-PRODUCTION-EXECUTOR".to_string(),
        kernel_phase_message_ids: Vec::new(),
        hphys0245_trace_config: None,
        hphys0245_trace_rows: Vec::new(),
        per_ofe_internal_wb13_summary: PerOfeInternalWb13RunSummary::default(),
        executed_day_count,
        retained_direct_publication: Some(direct_execution),
        direct_publication: None,
    })
}

struct DirectProductionRunFrameBuildInputs<'a> {
    output_hillslope_id: u32,
    lane_areas_m2: &'a [f64],
    runoff_publication_geometries: &'a [Wb13RunoffPublicationGeometry],
    day_count: usize,
    climate_request: &'a HillslopeClimateRuntimeRequest,
    climate_span: &'a ClimateRunSpanSummary,
    climate_context_surface: &'a HillslopeWritebackSurface,
    lane_seed_surfaces: &'a [HillslopeWritebackSurface],
    execution_lane: ExecutionLane,
}

fn build_direct_production_run_frame(
    inputs: &DirectProductionRunFrameBuildInputs<'_>,
) -> Result<DirectRunFrame, HillslopeCliError> {
    let output_hillslope_id = inputs.output_hillslope_id;
    let lane_areas_m2 = inputs.lane_areas_m2;
    let runoff_publication_geometries = inputs.runoff_publication_geometries;
    let day_count = inputs.day_count;
    let climate_request = inputs.climate_request;
    let climate_span = inputs.climate_span;
    let climate_context_surface = inputs.climate_context_surface;
    let lane_seed_surfaces = inputs.lane_seed_surfaces;
    let execution_lane = inputs.execution_lane;
    let identity = DirectRunIdentity::new(
        u64::from(output_hillslope_id),
        output_hillslope_id,
        lane_areas_m2.len(),
        day_count,
    )
    .map_err(|source| direct_production_runtime_error(&source))?;
    if runoff_publication_geometries.len() != lane_areas_m2.len() {
        return Err(direct_production_executor_blocked(format!(
            "direct production runoff publication geometry count {} does not match lane count {}",
            runoff_publication_geometries.len(),
            lane_areas_m2.len()
        )));
    }
    let lanes = lane_areas_m2
        .iter()
        .copied()
        .enumerate()
        .map(|(lane_index, area_m2)| {
            let mut lane_inputs = DirectLaneConstructorInputs::from_topology(
                lane_index,
                lane_areas_m2.len(),
                day_count,
            )
            .map_err(|source| direct_production_runtime_error(&source))?;
            if !area_m2.is_finite() || area_m2 <= 0.0 {
                return Err(direct_production_executor_blocked(format!(
                    "direct production lane {} area must be finite and > 0.0, observed {area_m2}",
                    lane_index + 1
                )));
            }
            lane_inputs.area_m2 = area_m2;
            lane_inputs.upstream_area_ratio = if lane_index == 0 {
                1.0
            } else {
                lane_areas_m2[lane_index - 1] / area_m2
            };
            let seed_authority = direct_production_lane_seed_authority(
                lane_seed_surfaces,
                lane_index,
                lane_areas_m2.len(),
            )?;
            let runoff_publication_geometry = direct_production_runoff_publication_geometry(
                seed_authority,
                runoff_publication_geometries[lane_index],
                lane_index,
            )?;
            lane_inputs.runoff_publication_q_scale = runoff_publication_geometry.q_scale;
            lane_inputs.runoff_publication_qofe_scale = runoff_publication_geometry.qofe_scale;
            lane_inputs.runoff_publication_efflen_m = runoff_publication_geometry.efflen_m;
            lane_inputs.runoff_publication_cumulative_length_m =
                runoff_publication_geometry.cumulative_length_m;
            lane_inputs.runoff_publication_ofe_length_m =
                runoff_publication_geometry.ofe_length_m;
            seed_direct_production_lane_constructor_inputs(
                &mut lane_inputs,
                lane_index,
                climate_request,
                climate_span,
                climate_context_surface,
                lane_seed_surfaces,
                execution_lane,
            )?;
            Ok(lane_inputs)
        })
        .collect::<Result<Vec<_>, HillslopeCliError>>()?;
    DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(identity, lanes))
        .map_err(|source| direct_production_runtime_error(&source))
}

fn direct_production_lane_seed_authority(
    lane_seed_surfaces: &[HillslopeWritebackSurface],
    lane_index: usize,
    lane_count: usize,
) -> Result<&HillslopeWritebackSurface, HillslopeCliError> {
    if lane_seed_surfaces.len() == 1 {
        return Ok(&lane_seed_surfaces[0]);
    }
    lane_seed_surfaces.get(lane_index).ok_or_else(|| {
        direct_production_executor_blocked(format!(
            "direct production lane {} has no lane-indexed seed authority out of {} lanes",
            lane_index + 1,
            lane_count
        ))
    })
}

#[derive(Clone, Copy, Debug)]
struct DirectProductionRunoffPublicationGeometry {
    q_scale: f64,
    qofe_scale: f64,
    efflen_m: f64,
    cumulative_length_m: f64,
    ofe_length_m: f64,
}

fn direct_production_runoff_publication_geometry(
    seed_authority: &HillslopeWritebackSurface,
    geometry: Wb13RunoffPublicationGeometry,
    lane_index: usize,
) -> Result<DirectProductionRunoffPublicationGeometry, HillslopeCliError> {
    let efflen_m =
        runtime_surface_symbol_value(seed_authority, "efflen").unwrap_or(geometry.ofe_length_m);
    if !efflen_m.is_finite() || efflen_m <= 0.0 {
        return Err(direct_production_executor_blocked(format!(
            "direct production lane {} efflen must be finite and > 0.0 for WB13 runoff publication, observed {efflen_m}",
            lane_index + 1
        )));
    }
    if efflen_m > geometry.cumulative_length_m + 1.0e-9 {
        return Err(direct_production_executor_blocked(format!(
            "direct production lane {} efflen must not exceed cumulative runoff-publication length, observed efflen={} cumulative={}",
            lane_index + 1,
            efflen_m,
            geometry.cumulative_length_m
        )));
    }
    let q_scale = efflen_m / geometry.cumulative_length_m;
    let qofe_scale = efflen_m / geometry.ofe_length_m;
    if !q_scale.is_finite() || q_scale <= 0.0 || !qofe_scale.is_finite() || qofe_scale <= 0.0 {
        return Err(direct_production_executor_blocked(format!(
            "direct production lane {} invalid runoff publication scales q={} qofe={}",
            lane_index + 1,
            q_scale,
            qofe_scale
        )));
    }
    Ok(DirectProductionRunoffPublicationGeometry {
        q_scale,
        qofe_scale,
        efflen_m,
        cumulative_length_m: geometry.cumulative_length_m,
        ofe_length_m: geometry.ofe_length_m,
    })
}

fn direct_production_lane_seed_surfaces(
    runtime_surface: &HillslopeWritebackSurface,
    persistent_lane_state: Option<&OfeLanePersistentStateSequence>,
    lane_count: usize,
) -> Result<Vec<HillslopeWritebackSurface>, HillslopeCliError> {
    if let Some(persistent_lane_state) = persistent_lane_state {
        let lane_states = persistent_lane_state.lane_states();
        if lane_states.len() != lane_count {
            return Err(direct_production_executor_blocked(format!(
                "direct production lane seed authority count {} does not match lane count {lane_count}",
                lane_states.len()
            )));
        }
        return Ok(lane_states
            .iter()
            .map(|lane_state| lane_state.writeback_surface.clone())
            .collect());
    }
    if lane_count != 1 {
        return Err(direct_production_executor_blocked(format!(
            "direct production multi-OFE run requires lane-indexed seed authority, observed lane_count={lane_count} with no persistent lane state"
        )));
    }
    Ok(vec![runtime_surface.clone()])
}

fn seed_direct_production_lane_constructor_inputs(
    lane_inputs: &mut DirectLaneConstructorInputs,
    lane_index: usize,
    climate_request: &HillslopeClimateRuntimeRequest,
    climate_span: &ClimateRunSpanSummary,
    climate_context_surface: &HillslopeWritebackSurface,
    lane_seed_surfaces: &[HillslopeWritebackSurface],
    execution_lane: ExecutionLane,
) -> Result<(), HillslopeCliError> {
    let seed_authority = if lane_seed_surfaces.len() == 1 {
        &lane_seed_surfaces[0]
    } else {
        lane_seed_surfaces.get(lane_index).ok_or_else(|| {
            direct_production_executor_blocked(format!(
                "direct production lane {} has no lane-indexed seed authority",
                lane_index + 1
            ))
        })?
    };
    let day_zero_seed_surface = direct_publication_day_zero_seed_surface(
        climate_request,
        climate_span,
        seed_authority,
        climate_context_surface,
        execution_lane,
    )?;
    lane_inputs.water.soil_water_m =
        require_runtime_surface_scalar(&day_zero_seed_surface, "wb11_soil_water")?;
    lane_inputs.subsurface_layers = direct_publication_layer_states(&day_zero_seed_surface)?;
    lane_inputs.evapotranspiration_stage_state =
        direct_publication_stage_state(&day_zero_seed_surface)?;
    Ok(())
}

fn build_direct_production_coupling_vector_provenance(
    runtime_surface: &HillslopeWritebackSurface,
    frame: &DirectRunFrame,
    publication: &DirectRunPublicationFrame,
) -> Result<HillslopeCouplingVectorProvenance, HillslopeCliError> {
    let row = publication.last_day().ok_or_else(|| {
        direct_production_executor_blocked(
            "direct production executor requires at least one direct publication row",
        )
    })?;
    let snow_file_present = parse_simimpl10_binary_flag(
        "snow.options.snow_file_present",
        require_simimpl10_coupling_scalar(runtime_surface, "snow.options.snow_file_present")?,
    )?;
    let rst = require_simimpl10_coupling_scalar(runtime_surface, "snow.options.rst")?;
    let newsnw = require_simimpl10_coupling_scalar(runtime_surface, "snow.options.newsnw")?;
    let ssd = require_simimpl10_coupling_scalar(runtime_surface, "snow.options.ssd")?;
    let runtime_swe = row.storage.snow_water_mm / 1_000.0;
    let frost_file_present = parse_simimpl10_binary_flag(
        "frost.options.frost_file_present",
        require_simimpl10_coupling_scalar(runtime_surface, "frost.options.frost_file_present")?,
    )?;
    let wint_red_enabled = parse_simimpl10_binary_flag(
        "frost.options.wintRed",
        require_simimpl10_coupling_scalar(runtime_surface, "frost.options.wintRed")?,
    )?;
    let outlet_frost_carry = frame
        .lanes
        .last()
        .and_then(|lane| lane.frost_runtime_carry.as_ref());
    let dfrost = outlet_frost_carry.map_or_else(
        || require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_dfrost"),
        |carry| Ok(carry.dfrost_m),
    )?;
    let dthaw = outlet_frost_carry.map_or_else(
        || require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_dthaw"),
        |carry| Ok(carry.dthaw_m),
    )?;
    let nft = outlet_frost_carry.map_or_else(
        || require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_nft"),
        |carry| Ok(carry.nft),
    )?;
    let ws_frz = outlet_frost_carry.map_or_else(
        || require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_ws_frz"),
        |carry| Ok(carry.ws_frz_m),
    )?;
    let infcap_frz = outlet_frost_carry.map_or_else(
        || require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_infcap_frz"),
        |carry| Ok(carry.infcap_frz_m_s),
    )?;
    let ssc = require_simimpl10_coupling_scalar(runtime_surface, "ssc")?;
    let total_soil = row.storage.total_soil_mm;
    let frozwt = row.storage.frozwt_mm;
    let snow_water = row.storage.snow_water_mm;
    let soil_water_total = row.storage.soil_water_total_mm;
    let closure_delta = soil_water_total - total_soil;
    let closure_within_tolerance = closure_delta.abs() <= SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM;
    if !closure_within_tolerance {
        return Err(simcoup_failure(format!(
            "direct hydout-equivalent closure violated: SoilWaterTotal - Total-Soil = {closure_delta} exceeds tolerance {SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM}"
        )));
    }

    Ok(HillslopeCouplingVectorProvenance {
        guard_id: SIMCOUP_GUARD_ID.to_string(),
        winter: HillslopeWinterCouplingProvenance {
            active: runtime_swe > 0.0 || dfrost > 0.0 || ws_frz > 0.0,
            snow_file_present,
            rst,
            newsnw,
            ssd,
            runtime_swe,
        },
        soil: HillslopeSoilCouplingProvenance {
            ssc,
            infiltration_capacity_frozen: infcap_frz,
            infcap_within_ssc: infcap_frz <= ssc,
        },
        frsoil: HillslopeFrozenSoilCouplingProvenance {
            active: wint_red_enabled,
            frost_file_present,
            wint_red_enabled,
            dfrost,
            dthaw,
            nft,
            ws_frz,
            infcap_frz,
        },
        hydout_equivalent: HillslopeHydoutEquivalentCouplingProvenance {
            source: DIRECT_PUBLICATION_FRAME_PUBLICATION_SOURCE.to_string(),
            total_soil,
            frozwt,
            snow_water,
            soil_water_total,
            closure_delta,
            closure_tolerance: SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM,
            closure_within_tolerance,
        },
    })
}

impl ClimateExecutionAccumulator {
    fn new(
        runtime_surface: HillslopeWritebackSurface,
        day_count: usize,
        contributor_ofe_count: usize,
        retained_direct_publication: Option<DirectPublicationExecution>,
    ) -> Result<Self, HillslopeCliError> {
        let runtime_swe_publication_state_m =
            require_runtime_surface_scalar(&runtime_surface, "snow.runtime_swe")?;
        Ok(Self {
            runtime_surface,
            runtime_swe_publication_state_m,
            wb13_rows: Vec::with_capacity(day_count * contributor_ofe_count.max(1)),
            pass_rows: Vec::with_capacity(day_count),
            coupling_vectors: None,
            erod14_wave2_kernel_status_seen: false,
            scheduler_outcome_class: SchedulerOutcomeClass::Completed,
            scheduler_status_message_id: String::new(),
            previous_climate_symbols: Vec::new(),
            kernel_phase_message_ids: std::collections::BTreeSet::new(),
            hphys0245_trace_rows: Vec::new(),
            per_ofe_internal_wb13_summary: PerOfeInternalWb13RunSummary::default(),
            retained_direct_publication,
        })
    }

    fn finish(
        self,
        completion: ClimateExecutionCompletion,
    ) -> Result<HillslopeClimateExecution, HillslopeCliError> {
        let coupling_vectors =
            self.coupling_vectors
                .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "execution_provenance",
                    detail: format!(
                        "{SIMPIPE_GUARD_ID} climate span contained no executable days after parser validation"
                    ),
                })?;
        Ok(HillslopeClimateExecution {
            selected_lane: completion.selected_lane,
            publication_area_m2: completion.publication_area_m2,
            contributor_ofe_count: completion.contributor_ofe_count,
            static_per_ofe_slice_count: completion.static_per_ofe_slice_count,
            persistent_lane_active: completion.persistent_lane_active,
            runtime_surface: self.runtime_surface,
            climate_span: completion.climate_span,
            wb13_rows: self.wb13_rows,
            pass_rows: self.pass_rows,
            coupling_vectors,
            erod14_wave2_kernel_status_seen: self.erod14_wave2_kernel_status_seen,
            scheduler_outcome_class: self.scheduler_outcome_class,
            scheduler_status_message_id: self.scheduler_status_message_id,
            kernel_phase_message_ids: self.kernel_phase_message_ids.into_iter().collect(),
            hphys0245_trace_config: completion.hphys0245_trace_config,
            hphys0245_trace_rows: self.hphys0245_trace_rows,
            per_ofe_internal_wb13_summary: self.per_ofe_internal_wb13_summary,
            executed_day_count: completion.executed_day_count,
            retained_direct_publication: self.retained_direct_publication,
            direct_publication: None,
        })
    }

    fn apply_hillslope_day(
        &mut self,
        apply: &mut HillslopeDayApply<'_>,
    ) -> Result<(), HillslopeCliError> {
        let context = SchedulerLifecycleContext {
            run_name: apply.context.run_name,
            execution_lane: apply.context.lane,
            publication_area_m2: apply.context.publication_area_m2,
            simulation_year: apply.simulation_year,
            sim_day_index: apply.day_index + 1,
            calendar_day: apply.day_projection,
            runtime_swe_before_m: apply.runtime_swe_before_m,
            hphys0245_trace_config: apply.context.hphys0245_trace_config,
            symbol_registry: apply.context.symbol_registry,
            hot_symbol_tables: apply.context.hot_symbol_tables,
            indexed_scheduler_runtime_enabled: apply.context.indexed_scheduler_runtime_enabled,
        };
        if let Some(persistent_lane_state) = apply.persistent_lane_state.as_mut() {
            let persistent_result = execute_persistent_scheduler_kernel_lifecycle(
                persistent_lane_state,
                &apply.climate_surface,
                apply.stale_climate_symbols,
                apply.per_ofe_lane_areas_m2,
                apply.per_ofe_runoff_publication_geometries,
                context,
            )
            .map_err(|error| {
                annotate_day_runtime_error(error, apply.day_index, apply.day_projection)
            })?;
            self.publish_persistent_day_result(persistent_result, apply.context)?;
        } else {
            indexed_shadow_surface::observe_clone_source_surface(&self.runtime_surface)?;
            self.runtime_surface = crate::hillslope::intake_lane_setup::merge_runtime_surfaces(
                std::mem::take(&mut self.runtime_surface),
                std::mem::take(&mut apply.climate_surface),
            );
            let execution_result = execute_scheduler_kernel_lifecycle(
                std::mem::take(&mut self.runtime_surface),
                context,
            )
            .map_err(|error| {
                annotate_day_runtime_error(error, apply.day_index, apply.day_projection)
            })?;
            self.publish_single_lane_day_result(execution_result, apply.context)?;
        }
        Ok(())
    }

    fn publish_persistent_day_result(
        &mut self,
        persistent_result: PersistentDailyExecutionResult,
        context: ClimateExecutionContext<'_>,
    ) -> Result<(), HillslopeCliError> {
        self.per_ofe_internal_wb13_summary
            .observe_day(&persistent_result.internal_wb13_collection)?;
        self.runtime_swe_publication_state_m = persistent_result
            .internal_wb13_collection
            .outlet_row()
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "per_ofe_internal_wb13",
                detail: format!("{SIMPIPE_GUARD_ID} internal WB13 collection has no outlet row"),
            })?
            .wb13_row
            .snow_water
            / 1_000.0;
        persistent_result
            .internal_wb13_collection
            .append_publication_rows_to(&mut self.wb13_rows);
        persistent_result
            .internal_wb13_collection
            .append_runoff_delivery_rows_to(
                hillslope_id_for_pass_output(context.output_hillslope_id)?,
                context.publication_area_m2,
                HillslopePassPublicationScalars::from_runtime_surface(
                    &persistent_result.runtime_surface,
                )?,
                &mut self.pass_rows,
            )?;
        self.observe_persistent_day_result(persistent_result);
        Ok(())
    }

    fn publish_single_lane_day_result(
        &mut self,
        execution_result: DailyExecutionResult,
        context: ClimateExecutionContext<'_>,
    ) -> Result<(), HillslopeCliError> {
        self.runtime_swe_publication_state_m =
            execution_result.wb13_row.wb13_row.snow_water / 1_000.0;
        let publication_scalars =
            HillslopePassPublicationScalars::from_runtime_surface(&execution_result.runtime_surface)?;
        self.pass_rows.push(build_hillslope_pass_row(
            hillslope_id_for_pass_output(context.output_hillslope_id)?,
            &execution_result.wb13_row,
            publication_scalars,
        )?);
        self.wb13_rows.push(execution_result.wb13_row.clone());
        indexed_shadow_surface::validate_shadow_surface(&execution_result.runtime_surface)?;
        self.observe_single_lane_day_result(execution_result);
        Ok(())
    }

    fn observe_persistent_day_result(&mut self, result: PersistentDailyExecutionResult) {
        self.runtime_surface = result.runtime_surface;
        self.scheduler_outcome_class = result.scheduler_outcome_class;
        self.scheduler_status_message_id = result.scheduler_status_message_id;
        self.coupling_vectors = Some(result.coupling_vectors);
        self.kernel_phase_message_ids
            .extend(result.kernel_phase_message_ids);
        self.erod14_wave2_kernel_status_seen |= result.erod14_wave2_kernel_status_seen;
        self.hphys0245_trace_rows
            .extend(result.hphys0245_trace_rows);
    }

    fn observe_single_lane_day_result(&mut self, result: DailyExecutionResult) {
        self.runtime_surface = result.runtime_surface;
        self.scheduler_outcome_class = result.scheduler_outcome_class;
        self.scheduler_status_message_id = result.scheduler_status_message_id;
        self.coupling_vectors = Some(result.coupling_vectors);
        self.kernel_phase_message_ids
            .extend(result.kernel_phase_message_ids);
        self.erod14_wave2_kernel_status_seen |= result.erod14_wave2_kernel_status_seen;
        self.hphys0245_trace_rows
            .extend(result.hphys0245_trace_rows);
    }
}

fn build_day_climate_surface(
    climate_request: &HillslopeClimateRuntimeRequest,
    day_index: usize,
    runtime_surface: &HillslopeWritebackSurface,
    day_projection: &ClimateDayProjection,
) -> Result<HillslopeWritebackSurface, HillslopeCliError> {
    build_hillslope_runtime_surface_from_climate_request_with_context(
        climate_request,
        day_index,
        &runtime_surface.state_surface,
    )
    .map_err(|error| {
        annotate_day_runtime_error(
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "climate",
                detail: error.to_string(),
            },
            day_index,
            day_projection,
        )
    })
}

fn remove_stale_climate_symbols(
    runtime_surface: &mut HillslopeWritebackSurface,
    stale_climate_symbols: &[BoundarySymbol],
) {
    for symbol in stale_climate_symbols {
        runtime_surface.state_surface.remove(symbol);
        runtime_surface.flux_surface.remove(symbol);
    }
}

fn build_hillslope_execution_provenance(
    execution: &HillslopeClimateExecution,
    runtime_selection: HillslopeRuntimeSelection,
    sidecar_warnings: &mut Vec<String>,
) -> Result<HillslopeExecutionProvenance, HillslopeCliError> {
    let wb16_ealpha_compatibility_seed_used = parse_mofe03_binary_flag(
        WB16_EALPHA_COMPATIBILITY_SEED_FLAG_SYMBOL,
        runtime_surface_symbol_value(
            &execution.runtime_surface,
            WB16_EALPHA_COMPATIBILITY_SEED_FLAG_SYMBOL,
        )
        .unwrap_or(0.0),
    )?;
    if wb16_ealpha_compatibility_seed_used {
        sidecar_warnings.push(format!(
            "{WB16_EALPHA_SEED_WARNING_ID} WB16 ealpha seeded with compatibility constant 1.0 because no runtime producer was present; full baseline-authoritative ealpha producer-chain migration remains open."
        ));
    }
    let erod14_wave2_enabled = parse_mofe03_binary_flag(
        "erod14_wave2_enabled",
        runtime_surface_symbol_value(&execution.runtime_surface, "erod14_wave2_enabled")
            .unwrap_or(0.0),
    )?;
    let erod14_qin_source_policy = erod14_qin_source_policy(erod14_wave2_enabled, sidecar_warnings);
    Ok(HillslopeExecutionProvenance {
        scheduler_kernel_executed: runtime_selection != HillslopeRuntimeSelection::DirectProductionExecutor,
        publication_source: if matches!(
            runtime_selection,
            HillslopeRuntimeSelection::DirectPublicationFrameCutover
                | HillslopeRuntimeSelection::DirectProductionExecutor
        ) {
            DIRECT_PUBLICATION_FRAME_PUBLICATION_SOURCE
        } else {
            SCHEDULER_KERNEL_PUBLICATION_SOURCE
        }
        .to_string(),
        simpipe_guard_id: SIMPIPE_GUARD_ID.to_string(),
        selected_lane: execution.selected_lane.as_str().to_string(),
        scheduler_outcome_class: scheduler_outcome_class_as_str(execution.scheduler_outcome_class)
            .to_string(),
        scheduler_status_message_id: execution.scheduler_status_message_id.clone(),
        climate_day_count: execution.climate_span.days.len(),
        executed_day_count: execution.executed_day_count,
        kernel_phase_message_ids: execution.kernel_phase_message_ids.clone(),
        erod14_wave2_enabled,
        erod14_wave2_kernel_status_seen: execution.erod14_wave2_kernel_status_seen,
        erod14_qin_source_policy: erod14_qin_source_policy.to_string(),
        erod14_qin_sediment_coupled: false,
        wb16_ealpha_compatibility_seed_used,
        wb16_ealpha_seed_policy: wb16_ealpha_seed_policy(wb16_ealpha_compatibility_seed_used),
    })
}

fn erod14_qin_source_policy(
    erod14_wave2_enabled: bool,
    sidecar_warnings: &mut Vec<String>,
) -> &'static str {
    if erod14_wave2_enabled {
        sidecar_warnings.push(format!(
            "{EROD14_QIN_WARNING_ID} EROD14 Wave-2 qin is seeded from water-transfer provenance only; true sediment-coupled qin/qout and particle-fraction handoff remains MOFE01 M-G follow-on scope."
        ));
        EROD14_QIN_POLICY_WATER_TRANSFER_ONLY
    } else {
        EROD14_QIN_POLICY_WAVE2_DISABLED
    }
}

fn wb16_ealpha_seed_policy(wb16_ealpha_compatibility_seed_used: bool) -> String {
    if wb16_ealpha_compatibility_seed_used {
        WB16_EALPHA_SEED_POLICY_COMPATIBILITY.to_string()
    } else {
        WB16_EALPHA_SEED_POLICY_RUNTIME_PROVIDED.to_string()
    }
}

fn build_hillslope_publication_provenance(
    execution: &HillslopeClimateExecution,
    runtime_selection: HillslopeRuntimeSelection,
) -> Result<
    (
        HillslopeWb13PublicationProvenance,
        HillslopeMofeHourlyCarryProvenance,
    ),
    HillslopeCliError,
> {
    if matches!(
        runtime_selection,
        HillslopeRuntimeSelection::DirectPublicationFrameCutover
            | HillslopeRuntimeSelection::DirectProductionExecutor
    ) {
        let artifacts = execution.direct_publication.as_ref().ok_or_else(|| {
            direct_publication_cutover_blocked(
                "direct publication cutover requires retained direct publication artifacts",
            )
        })?;
        return build_direct_publication_manifest_provenance(
            &artifacts.execution.publication_frame,
        );
    }

    let per_ofe_summary = execution
        .persistent_lane_active
        .then_some(&execution.per_ofe_internal_wb13_summary);
    let wb13_publication = build_wb13_publication_provenance(
        &execution.wb13_rows,
        execution.contributor_ofe_count,
        execution.static_per_ofe_slice_count,
        execution.publication_area_m2,
        execution.persistent_lane_active,
        per_ofe_summary,
    )?;
    let mofe_hourly_carry = build_mofe_hourly_carry_provenance(
        &execution.runtime_surface,
        execution.contributor_ofe_count,
    )?;
    Ok((wb13_publication, mofe_hourly_carry))
}

fn write_hillslope_run_outputs(
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
    sidecars: &HillslopeSidecarResolution,
    execution: &HillslopeClimateExecution,
    runtime_selection: HillslopeRuntimeSelection,
) -> Result<(), HillslopeCliError> {
    if matches!(
        runtime_selection,
        HillslopeRuntimeSelection::DirectPublicationFrameCutover
            | HillslopeRuntimeSelection::DirectProductionExecutor
    ) {
        return write_hillslope_direct_publication_outputs(inputs, targets, execution);
    }

    let pass_bytes = build_hbp_output(
        &targets.output_pass,
        &execution.wb13_rows,
        &execution.runtime_surface,
        execution.contributor_ofe_count,
    )?;
    let loss_text = build_loss_output_json(
        &inputs.runfile.run_name,
        &inputs.soil,
        &sidecars.snow,
        &sidecars.frost,
        &execution.climate_span,
        execution.executed_day_count,
    )?;
    ensure_hillslope_output_parent_directories(targets)?;
    fs::write(&targets.output_pass, pass_bytes).map_err(|source| {
        HillslopeCliError::OutputWrite {
            path: targets.output_pass.clone(),
            source,
        }
    })?;
    fs::write(&targets.output_loss, loss_text).map_err(|source| {
        HillslopeCliError::OutputWrite {
            path: targets.output_loss.clone(),
            source,
        }
    })?;
    write_hillslope_optional_outputs(inputs, targets, execution)?;
    validate_required_hillslope_outputs(targets)
}

fn write_hillslope_direct_publication_outputs(
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
    execution: &HillslopeClimateExecution,
) -> Result<(), HillslopeCliError> {
    let artifacts = execution.direct_publication.as_ref().ok_or_else(|| {
        direct_publication_cutover_blocked(
            "direct publication frame was not built for cutover candidate",
        )
    })?;
    require_direct_publication_cutover_gates(inputs, artifacts)?;

    ensure_hillslope_output_parent_directories(targets)?;
    fs::write(&targets.output_pass, &artifacts.hbp_bytes).map_err(|source| {
        HillslopeCliError::OutputWrite {
            path: targets.output_pass.clone(),
            source,
        }
    })?;
    fs::write(&targets.output_loss, &artifacts.loss_text).map_err(|source| {
        HillslopeCliError::OutputWrite {
            path: targets.output_loss.clone(),
            source,
        }
    })?;
    write_hillslope_direct_publication_optional_outputs(inputs, targets, execution, artifacts)?;
    validate_required_hillslope_outputs(targets)
}

fn require_direct_publication_cutover_gates(
    inputs: &ParsedHillslopeRunInputs,
    artifacts: &DirectPublicationArtifacts,
) -> Result<(), HillslopeCliError> {
    if direct_publication_lacks_parity_grade_output_producers(
        &artifacts.execution.publication_frame,
    ) {
        return Err(direct_publication_cutover_blocked(
            "HOLD-R6E-PRODUCTION-DIRECT-RUNTIME-INPUT-BINDING-ABSENT \
             retained direct publication contains parsed climate/calendar/geometry rows, \
             but production direct runtime input/state binding for hydrology, storage, \
             subsurface, evaporation, PASS, loss, manifest, and erosion publication \
             producers is absent; refusing to treat compatibility scheduler output as \
             direct publication authority",
        ));
    }
    let direct_row_count = artifacts.execution.publication_frame.rows().len();
    if direct_row_count == 0 {
        return Err(direct_publication_cutover_blocked(
            "direct publication cutover requires at least one typed direct row",
        ));
    }
    require_direct_publication_output_family_authority(
        &artifacts.execution.publication_frame,
    )?;
    if artifacts.hbp_bytes.is_empty() || artifacts.loss_text.is_empty() {
        return Err(direct_publication_cutover_blocked(
            "direct publication cutover requires non-empty direct HBP and loss artifacts",
        ));
    }
    if inputs.runfile.output_config.wat.is_some() && artifacts.wat_rows.len() != direct_row_count {
        return Err(direct_publication_cutover_blocked(format!(
            "direct WAT projection row-count mismatch: direct_rows={} projection_rows={}",
            direct_row_count,
            artifacts.wat_rows.len()
        )));
    }
    let direct_pass_row_count = artifacts.execution.publication_frame.identity.day_count;
    if inputs.runfile.output_config.pass_parquet.is_some()
        && artifacts.pass_projection_rows.len() != direct_pass_row_count
    {
        return Err(direct_publication_cutover_blocked(format!(
            "direct PASS projection row-count mismatch: direct_days={} projection_rows={}",
            direct_pass_row_count,
            artifacts.pass_projection_rows.len()
        )));
    }
    Ok(())
}

fn require_direct_publication_output_family_authority(
    publication: &DirectRunPublicationFrame,
) -> Result<(), HillslopeCliError> {
    for row in publication.rows() {
        require_finite_nonnegative_direct_publication_scalar("area_m2", row.area_m2)?;
        require_finite_nonnegative_direct_publication_scalar(
            "climate.precipitation_mm",
            row.climate.precipitation_mm,
        )?;
        require_finite_nonnegative_direct_publication_scalar(
            "runoff.runvol_m3",
            row.runoff.runvol_m3,
        )?;
        require_direct_publication_option(
            "erosion.peak_runoff_m3_s",
            row.erosion.peak_runoff_m3_s,
        )?;
        require_direct_publication_option(
            "erosion.runoff_duration_s",
            row.erosion.runoff_duration_s,
        )?;
        require_direct_publication_option(
            "erosion.total_detachment_kg",
            row.erosion.total_detachment_kg,
        )?;
        require_direct_publication_option(
            "erosion.total_deposition_kg",
            row.erosion.total_deposition_kg,
        )?;
        require_direct_publication_option(
            "erosion.hbp_total_detachment_kg",
            row.erosion.hbp_total_detachment_kg,
        )?;
        require_direct_publication_option(
            "erosion.hbp_total_deposition_kg",
            row.erosion.hbp_total_deposition_kg,
        )?;
        require_direct_publication_option(
            "erosion.hbp_sediment_concentration_kg_m3",
            row.erosion.hbp_sediment_concentration_kg_m3,
        )?;
        let sediment = row
            .erosion
            .sediment_concentration_kg_m3
            .ok_or_else(|| {
                direct_publication_cutover_blocked(
                    "direct publication cutover requires producer-authoritative erosion.sediment_concentration_kg_m3",
                )
            })?;
        for (index, value) in sediment.iter().enumerate() {
            require_finite_nonnegative_direct_publication_scalar(
                &format!("erosion.sediment_concentration_kg_m3[{index}]"),
                *value,
            )?;
        }
    }
    Ok(())
}

fn require_direct_publication_option(
    field: &'static str,
    value: Option<f64>,
) -> Result<(), HillslopeCliError> {
    let value = value.ok_or_else(|| {
        direct_publication_cutover_blocked(format!(
            "direct publication cutover requires producer-authoritative {field}"
        ))
    })?;
    require_finite_nonnegative_direct_publication_scalar(field, value)
}

fn require_finite_nonnegative_direct_publication_scalar(
    field: &str,
    value: f64,
) -> Result<(), HillslopeCliError> {
    if value.is_finite() && value >= 0.0 {
        return Ok(());
    }
    Err(direct_publication_cutover_blocked(format!(
        "direct publication cutover requires finite non-negative {field}; observed {value}"
    )))
}

fn write_hillslope_direct_publication_optional_outputs(
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
    execution: &HillslopeClimateExecution,
    artifacts: &DirectPublicationArtifacts,
) -> Result<(), HillslopeCliError> {
    if let Some(wat_output) = inputs.runfile.output_config.wat.as_ref() {
        write_hillslope_wat_parquet(
            wat_output,
            &artifacts.wat_rows,
            InterchangeVersion::default(),
        )
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.wat",
            detail: error.to_string(),
        })?;
    }
    if let Some(pass_parquet_output) = inputs.runfile.output_config.pass_parquet.as_ref() {
        write_hillslope_pass_parquet(
            pass_parquet_output,
            &artifacts.pass_projection_rows,
            InterchangeVersion::default(),
        )
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass_parquet",
            detail: error.to_string(),
        })?;
    }
    write_hphys0245_trace_output(execution)?;
    write_generic_optional_outputs(inputs, targets, execution)
}

fn direct_publication_cutover_blocked(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_cutover",
        detail: format!(
            "{SIMOUT_GUARD_ID} R6-DIRECT-PUBLICATION-PARITY {}",
            detail.into()
        ),
    }
}

#[cfg(test)]
fn direct_publication_has_only_zero_or_absent_operands(
    publication: &DirectRunPublicationFrame,
) -> bool {
    publication.rows().iter().all(|row| {
        let scalar_operands = [
            row.climate.precipitation_mm,
            row.liquid_input.rm_mm,
            row.liquid_input.irrigation_mm,
            row.runoff.q_mm,
            row.runoff.qofe_mm,
            row.runoff.runvol_m3,
            row.evaporation.ep_mm,
            row.evaporation.es_mm,
            row.evaporation.er_mm,
            row.evaporation.total_evapotranspiration_mm,
            row.subsurface.dp_mm,
            row.subsurface.latqcc_mm,
            row.subsurface.tile_mm,
            row.subsurface.sbrunv_m3,
            row.transfer.upstream_surface_mm,
            row.transfer.upstream_lateral_mm,
            row.storage.total_soil_mm,
            row.storage.soil_water_total_mm,
            row.storage.frozwt_mm,
            row.storage.snow_water_mm,
            row.interception.interception_mm,
        ];
        let optional_operands = [
            row.runoff.peak_runoff_m3_s,
            row.runoff.runoff_duration_s,
            row.storage.frdp_mm,
            row.profile.depth_mm,
            row.profile.porosity_cap_mm,
            row.profile.fc_store_mm,
            row.profile.wp_store_mm,
            row.interception.interception_storage_mm,
            row.erosion.peak_runoff_m3_s,
            row.erosion.runoff_duration_s,
            row.erosion.total_detachment_kg,
            row.erosion.total_deposition_kg,
            row.erosion.hbp_total_detachment_kg,
            row.erosion.hbp_total_deposition_kg,
            row.erosion.hbp_sediment_concentration_kg_m3,
        ];
        let sediment_material = row
            .erosion
            .sediment_concentration_kg_m3
            .is_some_and(|fractions| fractions.iter().any(|value| *value != 0.0));

        scalar_operands.iter().all(|value| *value == 0.0)
            && optional_operands
                .iter()
                .all(|value| value.map(|value| value == 0.0).unwrap_or(true))
            && !sediment_material
    })
}

fn direct_publication_lacks_parity_grade_output_producers(
    publication: &DirectRunPublicationFrame,
) -> bool {
    publication.rows().iter().all(|row| {
        let hydrology_scalars = [
            row.liquid_input.rm_mm,
            row.liquid_input.irrigation_mm,
            row.runoff.q_mm,
            row.runoff.qofe_mm,
            row.runoff.runvol_m3,
            row.evaporation.ep_mm,
            row.evaporation.es_mm,
            row.evaporation.er_mm,
            row.evaporation.total_evapotranspiration_mm,
            row.subsurface.dp_mm,
            row.subsurface.latqcc_mm,
            row.subsurface.tile_mm,
            row.subsurface.sbrunv_m3,
            row.transfer.upstream_surface_mm,
            row.transfer.upstream_lateral_mm,
            row.storage.total_soil_mm,
            row.storage.soil_water_total_mm,
            row.storage.frozwt_mm,
            row.storage.snow_water_mm,
            row.interception.interception_mm,
        ];
        let optional_hydrology_scalars = [
            row.runoff.peak_runoff_m3_s,
            row.runoff.runoff_duration_s,
            row.storage.frdp_mm,
            row.profile.depth_mm,
            row.profile.porosity_cap_mm,
            row.profile.fc_store_mm,
            row.profile.wp_store_mm,
            row.interception.interception_storage_mm,
            row.erosion.peak_runoff_m3_s,
            row.erosion.runoff_duration_s,
            row.erosion.total_detachment_kg,
            row.erosion.total_deposition_kg,
            row.erosion.hbp_total_detachment_kg,
            row.erosion.hbp_total_deposition_kg,
            row.erosion.hbp_sediment_concentration_kg_m3,
        ];
        let erosion_material = row
            .erosion
            .sediment_concentration_kg_m3
            .is_some_and(|fractions| fractions.iter().any(|value| *value != 0.0));

        hydrology_scalars.iter().all(|value| *value == 0.0)
            && optional_hydrology_scalars
                .iter()
                .all(|value| value.map(|value| value == 0.0).unwrap_or(true))
            && !erosion_material
    })
}

fn ensure_hillslope_output_parent_directories(
    targets: &HillslopeOutputTargets,
) -> Result<(), HillslopeCliError> {
    for path in std::iter::once(&targets.output_pass)
        .chain(std::iter::once(&targets.output_loss))
        .chain(targets.optional_outputs.iter())
    {
        crate::hillslope::intake_lane_setup::ensure_output_parent_directory(path)?;
    }
    Ok(())
}

fn write_hillslope_optional_outputs(
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
    execution: &HillslopeClimateExecution,
) -> Result<(), HillslopeCliError> {
    if let Some(wat_output) = inputs.runfile.output_config.wat.as_ref() {
        let wat_rows = build_hillslope_wat_rows(&execution.wb13_rows)?;
        write_hillslope_wat_parquet(wat_output, &wat_rows, InterchangeVersion::default()).map_err(
            |error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.wat",
                detail: error.to_string(),
            },
        )?;
    }
    if let Some(pass_parquet_output) = inputs.runfile.output_config.pass_parquet.as_ref() {
        write_hillslope_pass_parquet(
            pass_parquet_output,
            &execution.pass_rows,
            InterchangeVersion::default(),
        )
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "outputs.pass_parquet",
            detail: error.to_string(),
        })?;
    }
    write_hphys0245_trace_output(execution)?;
    write_generic_optional_outputs(inputs, targets, execution)
}

fn write_hphys0245_trace_output(
    execution: &HillslopeClimateExecution,
) -> Result<(), HillslopeCliError> {
    if let Some(trace_config) = execution.hphys0245_trace_config.as_ref() {
        write_hphys0245_trace_jsonl(trace_config, &execution.hphys0245_trace_rows)?;
    }
    Ok(())
}

fn write_generic_optional_outputs(
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
    execution: &HillslopeClimateExecution,
) -> Result<(), HillslopeCliError> {
    for optional_output in targets
        .optional_outputs
        .iter()
        .filter(|path| Some(path.as_path()) != inputs.runfile.output_config.wat.as_deref())
        .filter(|path| Some(path.as_path()) != inputs.runfile.output_config.pass_parquet.as_deref())
    {
        let payload = build_optional_output_payload(
            &inputs.runfile.run_name,
            optional_output,
            &execution.climate_span,
            execution.executed_day_count,
        );
        fs::write(optional_output, payload).map_err(|source| HillslopeCliError::OutputWrite {
            path: optional_output.clone(),
            source,
        })?;
    }
    Ok(())
}

fn validate_required_hillslope_outputs(
    targets: &HillslopeOutputTargets,
) -> Result<(), HillslopeCliError> {
    if !targets.output_pass.is_file() {
        return Err(HillslopeCliError::MissingRequiredOutput {
            output_name: REQUIRED_RUN_OUTPUT_PASS,
        });
    }
    if !targets.output_loss.is_file() {
        return Err(HillslopeCliError::MissingRequiredOutput {
            output_name: REQUIRED_RUN_OUTPUT_LOSS,
        });
    }
    Ok(())
}

fn write_hillslope_run_manifest(
    publication: HillslopeManifestPublication<'_>,
) -> Result<PathBuf, HillslopeCliError> {
    let binary_path = std::env::current_exe().map_err(|source| HillslopeCliError::Io {
        path: PathBuf::from("<current_exe>"),
        source,
    })?;
    let binary_sidecar_path = write_release_sidecar_for_binary(&binary_path, BinaryRole::Hillslope)
        .map_err(|source| HillslopeCliError::ReleaseMetadata { source })?;
    let invoked_utc =
        utc_now_rfc3339().map_err(|detail| HillslopeCliError::TimeFormat { detail })?;
    let input_checksums =
        build_hillslope_input_checksums(publication.inputs, publication.sidecars.input_paths)?;
    let output_checksums = build_hillslope_output_checksums(publication.targets)?;
    let manifest_path = publication.request.manifest_path.clone().unwrap_or_else(|| {
        publication
            .request
            .output_dir
            .join("openwepp_hillslope_run_manifest.json")
    });
    let manifest = build_hillslope_run_manifest(
        publication,
        &binary_path,
        &binary_sidecar_path,
        invoked_utc,
        input_checksums,
        output_checksums,
    )?;
    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|source| HillslopeCliError::ManifestSerialize { source })?;
    fs::write(&manifest_path, manifest_json).map_err(|source| {
        HillslopeCliError::ManifestWrite {
            path: manifest_path.clone(),
            source,
        }
    })?;
    Ok(manifest_path)
}

fn build_hillslope_input_checksums(
    inputs: &ParsedHillslopeRunInputs,
    sidecar_input_paths: &HillslopeSidecarInputPaths,
) -> Result<BTreeMap<String, String>, HillslopeCliError> {
    let mut checksums = BTreeMap::new();
    let mut input_paths: Vec<&Path> = vec![
        inputs.run_file_path.as_path(),
        inputs.soil_path.as_path(),
        inputs.management_path.as_path(),
        inputs.slope_path.as_path(),
        inputs.climate_path.as_path(),
    ];
    input_paths.extend(optional_sidecar_input_paths(sidecar_input_paths));
    for path in input_paths {
        checksums.insert(
            path.display().to_string(),
            sha256_file_hex(path).map_err(|source| HillslopeCliError::Io {
                path: path.to_path_buf(),
                source,
            })?,
        );
    }
    Ok(checksums)
}

fn optional_sidecar_input_paths(input_paths: &HillslopeSidecarInputPaths) -> Vec<&Path> {
    [
        input_paths.snow.as_deref(),
        input_paths.frost.as_deref(),
        input_paths.wepp_ui.as_deref(),
        input_paths.pmetpara.as_deref(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn build_hillslope_output_checksums(
    targets: &HillslopeOutputTargets,
) -> Result<BTreeMap<String, String>, HillslopeCliError> {
    let mut output_checksum_entries = Vec::new();
    for path in std::iter::once(&targets.output_pass)
        .chain(std::iter::once(&targets.output_loss))
        .chain(targets.optional_outputs.iter())
    {
        output_checksum_entries.push(OutputChecksumEntry::new(
            path.display().to_string(),
            sha256_file_hex(path).map_err(|source| HillslopeCliError::Io {
                path: path.clone(),
                source,
            })?,
        ));
    }
    assemble_output_checksums(&output_checksum_entries).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "manifest_output_checksums",
            detail: error.to_string(),
        }
    })
}

fn build_hillslope_run_manifest(
    publication: HillslopeManifestPublication<'_>,
    binary_path: &Path,
    binary_sidecar_path: &Path,
    invoked_utc: String,
    input_checksums: BTreeMap<String, String>,
    output_checksums: BTreeMap<String, String>,
) -> Result<HillslopeRunManifest, HillslopeCliError> {
    Ok(HillslopeRunManifest {
        schema: HILLSLOPE_RUN_MANIFEST_SCHEMA_ID.to_string(),
        engine: "openwepp".to_string(),
        binary_path: binary_path.display().to_string(),
        binary_sha256: sha256_file_hex(binary_path).map_err(|source| HillslopeCliError::Io {
            path: binary_path.to_path_buf(),
            source,
        })?,
        binary_sidecar_path: binary_sidecar_path.display().to_string(),
        binary_sidecar_sha256: sha256_file_hex(binary_sidecar_path).map_err(|source| {
            HillslopeCliError::Io {
                path: binary_sidecar_path.to_path_buf(),
                source,
            }
        })?,
        source_commit: git_source_commit_or_unknown(),
        invoked_utc,
        argv: publication.argv.to_vec(),
        run_dir: publication.request.run_dir.display().to_string(),
        run_file: publication.inputs.run_file_path.display().to_string(),
        sidecar_policy: publication.request.sidecar_policy.as_str().to_string(),
        sidecar_discovery_mode: publication.sidecars.discovery_mode.to_string(),
        resolved_sidecars: publication.sidecars.resolved_sidecars,
        input_checksums,
        output_checksums,
        runtime_selection: publication.runtime_selection,
        mode_selection: publication.sidecars.mode_selection,
        timestep_policy: publication.timestep_policy,
        adapter_boundary: publication.adapter_boundary,
        execution_provenance: publication.execution_provenance,
        wb13_publication: publication.wb13_publication,
        mofe_hourly_carry: publication.mofe_hourly_carry,
        direct_runtime_counters: publication.direct_runtime_counters,
        coupling_vectors: publication.coupling_vectors,
    })
}

fn direct_runtime_counters_for_manifest(
    runtime_selection: HillslopeRuntimeSelection,
    baseline: DirectRuntimeAuditSnapshot,
    current: DirectRuntimeAuditSnapshot,
) -> Option<HillslopeDirectRuntimeCounterProvenance> {
    if !matches!(
        runtime_selection,
        HillslopeRuntimeSelection::DirectPublicationFrameCutover
            | HillslopeRuntimeSelection::DirectProductionExecutor
    ) {
        return None;
    }
    Some(direct_runtime_counter_provenance(
        direct_runtime_audit_delta(baseline, current),
    ))
}

fn direct_runtime_counter_provenance(
    snapshot: DirectRuntimeAuditSnapshot,
) -> HillslopeDirectRuntimeCounterProvenance {
    HillslopeDirectRuntimeCounterProvenance {
        run_frame_constructions: snapshot.run_frame_constructions,
        day_frame_constructions: snapshot.day_frame_constructions,
        day_frame_commits: snapshot.day_frame_commits,
        executor_constructions: snapshot.executor_constructions,
        skeleton_runs: snapshot.skeleton_runs,
        publication_capture_runs: snapshot.publication_capture_runs,
        phase_view_constructions: snapshot.phase_view_constructions,
        phase_span_runs: snapshot.phase_span_runs,
        direct_phase_entries: snapshot.direct_phase_entries,
        direct_compute_operations: snapshot.direct_compute_operations,
        direct_state_mutations: snapshot.direct_state_mutations,
        downstream_operand_productions: snapshot.downstream_operand_productions,
        shadow_projections: snapshot.shadow_projections,
        compatibility_edge_invocations: snapshot.compatibility_edge_invocations,
    }
}

fn direct_runtime_audit_delta(
    baseline: DirectRuntimeAuditSnapshot,
    current: DirectRuntimeAuditSnapshot,
) -> DirectRuntimeAuditSnapshot {
    DirectRuntimeAuditSnapshot {
        run_frame_constructions: current
            .run_frame_constructions
            .saturating_sub(baseline.run_frame_constructions),
        day_frame_constructions: current
            .day_frame_constructions
            .saturating_sub(baseline.day_frame_constructions),
        day_frame_commits: current
            .day_frame_commits
            .saturating_sub(baseline.day_frame_commits),
        executor_constructions: current
            .executor_constructions
            .saturating_sub(baseline.executor_constructions),
        skeleton_runs: current.skeleton_runs.saturating_sub(baseline.skeleton_runs),
        publication_capture_runs: current
            .publication_capture_runs
            .saturating_sub(baseline.publication_capture_runs),
        phase_view_constructions: current
            .phase_view_constructions
            .saturating_sub(baseline.phase_view_constructions),
        phase_span_runs: current
            .phase_span_runs
            .saturating_sub(baseline.phase_span_runs),
        direct_phase_entries: current
            .direct_phase_entries
            .saturating_sub(baseline.direct_phase_entries),
        direct_compute_operations: current
            .direct_compute_operations
            .saturating_sub(baseline.direct_compute_operations),
        direct_state_mutations: current
            .direct_state_mutations
            .saturating_sub(baseline.direct_state_mutations),
        downstream_operand_productions: current
            .downstream_operand_productions
            .saturating_sub(baseline.downstream_operand_productions),
        shadow_projections: current
            .shadow_projections
            .saturating_sub(baseline.shadow_projections),
        compatibility_edge_invocations: current
            .compatibility_edge_invocations
            .saturating_sub(baseline.compatibility_edge_invocations),
    }
}

pub fn execute_hillslope_run(
    request: &HillslopeRunRequest,
    argv: &[String],
) -> Result<HillslopeRunReport, HillslopeCliError> {
    execute_hillslope_run_with_runtime_policy(
        request,
        argv,
        HillslopeRuntimeSelectionPolicy::default(),
    )
}

fn execute_selected_hillslope_days(
    run_name: &str,
    output_hillslope_id: u32,
    runtime_selection: HillslopeRuntimeSelection,
    state: HillslopeClimateExecutionState,
    climate: &ClimateFile,
) -> Result<HillslopeClimateExecution, HillslopeCliError> {
    let production_direct_selected =
        runtime_selection == HillslopeRuntimeSelection::DirectProductionExecutor;
    let symbol_registry_audit = if production_direct_selected {
        None
    } else {
        symbol_registry_audit::begin_if_requested(&state, climate)?
    };
    let indexed_shadow = if production_direct_selected {
        None
    } else {
        indexed_shadow_surface::begin_if_requested(&state, climate)?
    };
    let execution_result = if production_direct_selected {
        execute_hillslope_direct_production_days(run_name, output_hillslope_id, state, climate)
    } else {
        execute_hillslope_climate_days(
            run_name,
            output_hillslope_id,
            runtime_selection,
            state,
            climate,
        )
    };
    if let Some(symbol_registry_audit) = symbol_registry_audit {
        symbol_registry_audit.finish()?;
    }
    if let Some(indexed_shadow) = indexed_shadow {
        indexed_shadow.finish()?;
    }
    execution_result
}

pub fn execute_hillslope_run_with_runtime_selection(
    request: &HillslopeRunRequest,
    argv: &[String],
    runtime_selection: HillslopeRuntimeSelection,
) -> Result<HillslopeRunReport, HillslopeCliError> {
    execute_hillslope_run_with_runtime_policy(
        request,
        argv,
        HillslopeRuntimeSelectionPolicy::new(
            runtime_selection,
            HillslopeDefaultRuntimeActivation::Disabled,
        ),
    )
}

pub fn execute_hillslope_run_with_runtime_policy(
    request: &HillslopeRunRequest,
    argv: &[String],
    runtime_policy: HillslopeRuntimeSelectionPolicy,
) -> Result<HillslopeRunReport, HillslopeCliError> {
    if !request.run_dir.is_dir() {
        return Err(HillslopeCliError::RunDirectoryMissing {
            path: request.run_dir.clone(),
        });
    }

    fs::create_dir_all(&request.output_dir).map_err(|source| {
        HillslopeCliError::OutputDirectoryCreate {
            path: request.output_dir.clone(),
            source,
        }
    })?;

    let inputs = load_hillslope_run_inputs(request)?;
    let targets = resolve_hillslope_output_targets(&inputs.runfile)?;
    let runtime_resolution = runtime_policy.resolve();
    let runtime_selection = runtime_resolution.selected();
    select_direct_runtime_skeleton_once(runtime_selection, &inputs, &targets)?;
    let direct_runtime_counter_baseline = direct_runtime_audit_snapshot();
    let mut sidecars = resolve_hillslope_sidecars(request, &inputs, &targets)?;
    let runtime_setup = build_static_hillslope_runtime_setup(request, &inputs, &mut sidecars)?;
    let StaticHillslopeRuntimeSetup {
        timestep_policy,
        adapter_boundary,
        execution_state,
    } = runtime_setup;
    let mut execution = execute_selected_hillslope_days(
        &inputs.runfile.run_name,
        targets.output_hillslope_id,
        runtime_selection,
        execution_state,
        &inputs.climate,
    )?;
    execution.direct_publication =
        build_direct_publication_artifacts(runtime_selection, &inputs, &targets, &sidecars, &execution)?;
    let direct_runtime_counters = direct_runtime_counters_for_manifest(
        runtime_selection,
        direct_runtime_counter_baseline,
        direct_runtime_audit_snapshot(),
    );
    let execution_provenance = build_hillslope_execution_provenance(
        &execution,
        runtime_selection,
        &mut sidecars.sidecar_warnings,
    )?;
    let (wb13_publication, mofe_hourly_carry) =
        build_hillslope_publication_provenance(&execution, runtime_selection)?;
    write_hillslope_run_outputs(&inputs, &targets, &sidecars, &execution, runtime_selection)?;
    let runtime_selection_provenance =
        build_hillslope_runtime_selection_provenance(runtime_resolution, runtime_selection);

    let HillslopeSidecarResolution {
        mode_selection,
        resolved_sidecars,
        sidecar_warnings,
        input_paths,
        discovery_mode,
        ..
    } = sidecars;
    let manifest_path = write_hillslope_run_manifest(HillslopeManifestPublication {
        request,
        argv,
        inputs: &inputs,
        targets: &targets,
        sidecars: HillslopeSidecarManifestInputs {
            discovery_mode,
            resolved_sidecars,
            input_paths: &input_paths,
            mode_selection,
        },
        timestep_policy,
        adapter_boundary,
        execution_provenance,
        wb13_publication,
        mofe_hourly_carry,
        runtime_selection: runtime_selection_provenance,
        direct_runtime_counters,
        coupling_vectors: execution.coupling_vectors,
    })?;

    Ok(HillslopeRunReport {
        output_pass: targets.output_pass,
        output_loss: targets.output_loss,
        optional_outputs: targets.optional_outputs,
        manifest_path,
        sidecar_warnings,
    })
}

fn build_hillslope_runtime_selection_provenance(
    resolution: HillslopeRuntimeSelectionResolution,
    selected_runtime: HillslopeRuntimeSelection,
) -> HillslopeRuntimeSelectionProvenance {
    HillslopeRuntimeSelectionProvenance {
        requested: resolution.requested().as_str().to_string(),
        selected: resolution.selected().as_str().to_string(),
        selection_reason: resolution.selection_reason().to_string(),
        default_activation_gate: resolution.default_activation().as_str().to_string(),
        fallback_reason: resolution.fallback_reason().map(str::to_string),
        output_policy: direct_publication_output_policy(selected_runtime).to_string(),
        rollback_runtime: HillslopeRuntimeSelection::Compatibility
            .as_str()
            .to_string(),
        compatibility_rollback_available: true,
    }
}

fn select_direct_runtime_skeleton_once(
    runtime_selection: HillslopeRuntimeSelection,
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
) -> Result<(), HillslopeCliError> {
    let mode = match runtime_selection {
        HillslopeRuntimeSelection::DefaultCandidate
        | HillslopeRuntimeSelection::Compatibility
        | HillslopeRuntimeSelection::DirectPublicationFrameShadow
        | HillslopeRuntimeSelection::DirectPublicationFrameCutover
        | HillslopeRuntimeSelection::DirectProductionExecutor => return Ok(()),
        HillslopeRuntimeSelection::DirectSkeletonNoop => DirectExecutorMode::Noop,
        HillslopeRuntimeSelection::DirectSkeletonShadowOnly => DirectExecutorMode::ShadowOnly,
    };

    let identity = DirectRunIdentity::new(
        u64::from(targets.output_hillslope_id),
        targets.output_hillslope_id,
        inputs.slope.ofe_count,
        inputs.climate.daily_records.len(),
    )
    .map_err(|source| direct_runtime_skeleton_error(&source))?;
    let mut frame =
        DirectRunFrame::skeleton(identity).map_err(|source| direct_runtime_skeleton_error(&source))?;
    let executor = DirectFrameExecutor::new(mode);
    let report = executor
        .run_skeleton(&mut frame)
        .map_err(|source| direct_runtime_skeleton_error(&source))?;
    debug_assert_eq!(report.mode.as_str(), mode.as_str());
    record_direct_runtime_compatibility_edge_invocation();

    Ok(())
}

fn direct_runtime_skeleton_error(
    source: &openwepp_hillslope_orchestrator::DirectRuntimeError,
) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "r2a_direct_runtime_skeleton",
        detail: source.to_string(),
    }
}

fn direct_production_runtime_error(
    source: &openwepp_hillslope_orchestrator::DirectRuntimeError,
) -> HillslopeCliError {
    direct_production_executor_blocked(source.to_string())
}

fn direct_production_executor_blocked(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "r7c_direct_production_executor",
        detail: format!("{SIMPIPE_GUARD_ID} {}", detail.into()),
    }
}
