fn execute_hillslope_direct_production_days(
    run_name: &str,
    output_hillslope_id: u32,
    inputs: &ParsedHillslopeRunInputs,
    sidecars: &HillslopeSidecarResolution,
    state: HillslopeClimateExecutionState,
    climate: &ClimateFile,
    streaming_targets: &DirectPublicationStreamingTargets,
) -> Result<HillslopeClimateExecution, HillslopeCliError> {
    let climate_request = build_hillslope_climate_runtime_request(climate).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "climate",
            detail: error.to_string(),
        }
    })?;
    let HillslopeClimateExecutionState {
        per_ofe_lane_areas_m2,
        per_ofe_runoff_publication_geometries,
        lane_context,
        climate_span,
    } = state;
    let seed_authority = DirectProductionSeedAuthority::from_typed_inputs(
        &climate_request,
        inputs,
        sidecars,
        per_ofe_lane_areas_m2.len(),
        lane_context.lane,
    )?;
    let mut frame = build_direct_production_run_frame(&DirectProductionRunFrameBuildInputs {
        output_hillslope_id,
        lane_areas_m2: &per_ofe_lane_areas_m2,
        runoff_publication_geometries: &per_ofe_runoff_publication_geometries,
        day_count: climate_span.days.len(),
        seed_authority: &seed_authority,
    })?;
    let day_input_builder = DirectProductionDayInputBuilder::new(
        &climate_request,
        &climate_span,
        &seed_authority,
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
    let retained_direct_publication = execute_direct_publication_stream(
        &mut frame,
        metadata,
        &day_input_builder,
        streaming_targets,
    )?;
    let coupling_vectors = build_direct_production_coupling_vector_provenance(
        &seed_authority,
        &frame,
        &retained_direct_publication.stream.summary,
    )?;
    let executed_day_count = climate_span.days.len();
    let erod14_wave2_enabled = seed_authority.erod14_wave2_enabled;

    Ok(HillslopeClimateExecution {
        selected_lane: lane_context.lane,
        climate_span,
        coupling_vectors,
        erod14_wave2_kernel_status_seen: erod14_wave2_enabled,
        scheduler_outcome_class: "completed",
        scheduler_status_message_id: "R7C-DIRECT-PRODUCTION-EXECUTOR".to_string(),
        kernel_phase_message_ids: Vec::new(),
        executed_day_count,
        retained_direct_publication: Some(retained_direct_publication),
        direct_publication: None,
    })
}

fn execute_direct_publication_stream(
    frame: &mut DirectRunFrame,
    metadata: DirectPublicationRunMetadata,
    day_input_builder: &DirectProductionDayInputBuilder<'_>,
    streaming_targets: &DirectPublicationStreamingTargets,
) -> Result<RetainedDirectPublication, HillslopeCliError> {
    let mut stream_sink =
        DirectPublicationStreamingSink::create(frame.identity, metadata.clone(), streaming_targets)?;
    let execution = DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect)
        .run_publication_stream_with_interleaved_day_inputs(
            frame,
            metadata,
            |frame, day_index, lane_index| {
                day_input_builder
                    .build(frame, day_index, lane_index)
                    .map_err(|error| direct_publication_day_input_build_error(&error))
            },
            |row| {
                stream_sink.observe_row(row).map_err(|error| {
                    DirectRuntimeError::PublicationSinkFailure {
                        detail: error.to_string(),
                    }
                })
            },
        )
        .map_err(|source| direct_production_runtime_error(&source))?;
    let stream = stream_sink.finish()?;
    Ok(RetainedDirectPublication { execution, stream })
}

struct DirectProductionRunFrameBuildInputs<'a> {
    output_hillslope_id: u32,
    lane_areas_m2: &'a [f64],
    runoff_publication_geometries: &'a [Wb13RunoffPublicationGeometry],
    day_count: usize,
    seed_authority: &'a DirectProductionSeedAuthority,
}

fn build_direct_production_run_frame(
    inputs: &DirectProductionRunFrameBuildInputs<'_>,
) -> Result<DirectRunFrame, HillslopeCliError> {
    let output_hillslope_id = inputs.output_hillslope_id;
    let lane_areas_m2 = inputs.lane_areas_m2;
    let runoff_publication_geometries = inputs.runoff_publication_geometries;
    let day_count = inputs.day_count;
    let seed_authority = inputs.seed_authority;
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
            let mut lane_inputs = DirectLaneConstructorInputs::from_topology_with_dynamic_day_inputs(
                lane_index,
                lane_areas_m2.len(),
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
            let runoff_publication_geometry = direct_production_runoff_publication_geometry(
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
                seed_authority,
            )?;
            Ok(lane_inputs)
        })
        .collect::<Result<Vec<_>, HillslopeCliError>>()?;
    DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(identity, lanes))
        .map_err(|source| direct_production_runtime_error(&source))
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
    geometry: Wb13RunoffPublicationGeometry,
    lane_index: usize,
) -> Result<DirectProductionRunoffPublicationGeometry, HillslopeCliError> {
    let efflen_m = geometry.ofe_length_m;
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

fn seed_direct_production_lane_constructor_inputs(
    lane_inputs: &mut DirectLaneConstructorInputs,
    lane_index: usize,
    seed_authority: &DirectProductionSeedAuthority,
) -> Result<(), HillslopeCliError> {
    seed_authority
        .lane(lane_index)?
        .constructor
        .apply_to_lane_constructor(lane_inputs);
    Ok(())
}

fn build_direct_production_coupling_vector_provenance(
    seed_authority: &DirectProductionSeedAuthority,
    frame: &DirectRunFrame,
    publication: &DirectPublicationOutputSummary,
) -> Result<HillslopeCouplingVectorProvenance, HillslopeCliError> {
    let row = publication.last_row()?;
    let snow_frost_authority = seed_authority.outlet_snow_frost()?;
    let snow_file_present = snow_frost_authority.snow_file_present;
    let rst = snow_frost_authority.snow_rst_c;
    let newsnw = snow_frost_authority.snow_newsnw_kg_m3;
    let ssd = snow_frost_authority.snow_ssd_kg_m3;
    let runtime_swe = row.storage.snow_water_mm / 1_000.0;
    let frost_file_present = snow_frost_authority.frost_file_present;
    let wint_red_enabled = snow_frost_authority.frost_wint_red_enabled;
    let outlet_frost_carry = frame
        .lanes
        .last()
        .and_then(|lane| {
            direct_publication_frost_runtime_carry_from_lane_state(&lane.winter_column.frost)
        });
    let dfrost = outlet_frost_carry.as_ref().map_or(0.0, |carry| carry.dfrost_m);
    let dthaw = outlet_frost_carry.as_ref().map_or(0.0, |carry| carry.dthaw_m);
    let nft = outlet_frost_carry.as_ref().map_or(0.0, |carry| carry.nft);
    let ws_frz = outlet_frost_carry.as_ref().map_or(0.0, |carry| carry.ws_frz_m);
    let infcap_frz = outlet_frost_carry
        .as_ref()
        .map_or(0.0, |carry| carry.infcap_frz_m_s);
    let ssc = seed_authority.outlet_top_soil_conductivity_m_s()?;
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

fn build_hillslope_execution_provenance(
    execution: &HillslopeClimateExecution,
    runtime_selection: HillslopeRuntimeSelection,
    sidecar_warnings: &mut Vec<String>,
) -> HillslopeExecutionProvenance {
    debug_assert_eq!(
        runtime_selection,
        HillslopeRuntimeSelection::DirectProductionExecutor
    );
    let wb16_ealpha_compatibility_seed_used = false;
    let erod14_wave2_enabled = execution.erod14_wave2_kernel_status_seen;
    let erod14_qin_source_policy = erod14_qin_source_policy(erod14_wave2_enabled, sidecar_warnings);
    HillslopeExecutionProvenance {
        scheduler_kernel_executed: false,
        publication_source: DIRECT_PUBLICATION_FRAME_PUBLICATION_SOURCE.to_string(),
        simpipe_guard_id: SIMPIPE_GUARD_ID.to_string(),
        selected_lane: execution.selected_lane.as_str().to_string(),
        scheduler_outcome_class: execution.scheduler_outcome_class.to_string(),
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
    }
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
    debug_assert!(!wb16_ealpha_compatibility_seed_used);
    WB16_EALPHA_SEED_POLICY_RUNTIME_PROVIDED.to_string()
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
    debug_assert_eq!(
        runtime_selection,
        HillslopeRuntimeSelection::DirectProductionExecutor
    );
    let artifacts = execution.direct_publication.as_ref().ok_or_else(|| {
        direct_publication_cutover_blocked(
            "direct production publication requires retained direct publication artifacts",
        )
    })?;
    build_streamed_direct_publication_manifest_provenance(&artifacts.summary)
}

fn write_hillslope_run_outputs(
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
    sidecars: &HillslopeSidecarResolution,
    execution: &HillslopeClimateExecution,
    runtime_selection: HillslopeRuntimeSelection,
) -> Result<(), HillslopeCliError> {
    debug_assert_eq!(
        runtime_selection,
        HillslopeRuntimeSelection::DirectProductionExecutor
    );
    let _ = sidecars;
    write_hillslope_direct_publication_outputs(inputs, targets, execution)
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
    if !artifacts.summary.parity_grade_row_seen {
        return Err(direct_publication_cutover_blocked(
            "HOLD-R6E-PRODUCTION-DIRECT-RUNTIME-INPUT-BINDING-ABSENT \
             retained direct publication contains parsed climate/calendar/geometry rows, \
             but production direct runtime input/state binding for hydrology, storage, \
             subsurface, evaporation, PASS, loss, manifest, and erosion publication \
             producers is absent; refusing to treat compatibility scheduler output as \
             direct publication authority",
        ));
    }
    let direct_row_count = artifacts.execution.row_count;
    if direct_row_count == 0 {
        return Err(direct_publication_cutover_blocked(
            "direct publication cutover requires at least one typed direct row",
        ));
    }
    if artifacts.hbp_bytes.is_empty() || artifacts.loss_text.is_empty() {
        return Err(direct_publication_cutover_blocked(
            "direct publication cutover requires non-empty direct HBP and loss artifacts",
        ));
    }
    if inputs.runfile.output_config.wat.is_some()
        && artifacts
            .wat_rows_written
            .is_none_or(|rows_written| rows_written != direct_row_count)
    {
        return Err(direct_publication_cutover_blocked(format!(
            "direct WAT projection row-count mismatch: direct_rows={} projection_rows={}",
            direct_row_count,
            artifacts.wat_rows_written.unwrap_or(0)
        )));
    }
    let direct_pass_row_count = artifacts.execution.identity.day_count;
    if inputs.runfile.output_config.pass_parquet.is_some()
        && artifacts
            .pass_projection_rows_written
            .is_none_or(|rows_written| rows_written != direct_pass_row_count)
    {
        return Err(direct_publication_cutover_blocked(format!(
            "direct PASS projection row-count mismatch: direct_days={} projection_rows={}",
            direct_pass_row_count,
            artifacts.pass_projection_rows_written.unwrap_or(0)
        )));
    }
    Ok(())
}

fn require_direct_publication_output_family_authority_row(
    row: &DirectPublicationDayRow,
) -> Result<(), HillslopeCliError> {
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
    let sediment = row.erosion.sediment_concentration_kg_m3.ok_or_else(|| {
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
        let rows_written = artifacts.wat_rows_written.ok_or_else(|| {
            direct_publication_cutover_blocked(
                "direct WAT output requested but direct WAT projection rows were not streamed",
            )
        })?;
        if rows_written != artifacts.execution.row_count || !wat_output.is_file() {
            return Err(direct_publication_cutover_blocked(format!(
                "direct WAT streamed row-count mismatch: direct_rows={} streamed_rows={} path={}",
                artifacts.execution.row_count,
                rows_written,
                wat_output.display()
            )));
        }
    }
    if let Some(pass_parquet_output) = inputs.runfile.output_config.pass_parquet.as_ref() {
        let rows_written = artifacts.pass_projection_rows_written.ok_or_else(|| {
            direct_publication_cutover_blocked(
                "direct PASS output requested but direct PASS projection rows were not streamed",
            )
        })?;
        if rows_written != artifacts.execution.identity.day_count || !pass_parquet_output.is_file() {
            return Err(direct_publication_cutover_blocked(format!(
                "direct PASS streamed row-count mismatch: direct_days={} streamed_rows={} path={}",
                artifacts.execution.identity.day_count,
                rows_written,
                pass_parquet_output.display()
            )));
        }
    }
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

fn direct_publication_row_lacks_parity_grade_output_producers(
    row: &DirectPublicationDayRow,
) -> bool {
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
}

#[cfg(test)]
fn direct_publication_lacks_parity_grade_output_producers(
    publication: &DirectRunPublicationFrame,
) -> bool {
    publication
        .rows()
        .iter()
        .all(direct_publication_row_lacks_parity_grade_output_producers)
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
    if runtime_selection != HillslopeRuntimeSelection::DirectProductionExecutor {
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
        erod14_qin_clamped_events: snapshot.erod14_qin_clamped_events,
        phase_span_runs: snapshot.phase_span_runs,
        direct_phase_entries: snapshot.direct_phase_entries,
        direct_compute_operations: snapshot.direct_compute_operations,
        direct_state_mutations: snapshot.direct_state_mutations,
        downstream_operand_productions: snapshot.downstream_operand_productions,
        shadow_projections: snapshot.shadow_projections,
        compatibility_edge_invocations: snapshot.compatibility_edge_invocations,
        ksatadj_effective_conductivity_evaluations: snapshot
            .ksatadj_effective_conductivity_evaluations,
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
        erod14_qin_clamped_events: current
            .erod14_qin_clamped_events
            .saturating_sub(baseline.erod14_qin_clamped_events),
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
        ksatadj_effective_conductivity_evaluations: current
            .ksatadj_effective_conductivity_evaluations
            .saturating_sub(baseline.ksatadj_effective_conductivity_evaluations),
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

#[allow(clippy::too_many_arguments)]
fn execute_selected_hillslope_days(
    run_name: &str,
    output_hillslope_id: u32,
    runtime_selection: HillslopeRuntimeSelection,
    inputs: &ParsedHillslopeRunInputs,
    sidecars: &HillslopeSidecarResolution,
    state: HillslopeClimateExecutionState,
    climate: &ClimateFile,
    streaming_targets: &DirectPublicationStreamingTargets,
) -> Result<HillslopeClimateExecution, HillslopeCliError> {
    debug_assert_eq!(
        runtime_selection,
        HillslopeRuntimeSelection::DirectProductionExecutor
    );
    execute_hillslope_direct_production_days(
        run_name,
        output_hillslope_id,
        inputs,
        sidecars,
        state,
        climate,
        streaming_targets,
    )
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
            HillslopeDefaultRuntimeActivation::default(),
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
    ensure_hillslope_output_parent_directories(&targets)?;
    let streaming_targets = DirectPublicationStreamingTargets {
        wat: inputs.runfile.output_config.wat.clone(),
        pass_parquet: inputs.runfile.output_config.pass_parquet.clone(),
    };
    let runtime_resolution = runtime_policy.resolve();
    let runtime_selection = runtime_resolution.selected();
    let direct_runtime_counter_baseline = direct_runtime_audit_snapshot();
    let mut sidecars = resolve_hillslope_sidecars(request, &inputs, &targets)?;
    let runtime_setup = build_static_hillslope_runtime_setup(request, &inputs, &sidecars, runtime_selection)?;
    let StaticHillslopeRuntimeSetup {
        timestep_policy,
        adapter_boundary,
        execution_state,
    } = runtime_setup;
    let mut execution = execute_selected_hillslope_days(
        &inputs.runfile.run_name,
        targets.output_hillslope_id,
        runtime_selection,
        &inputs,
        &sidecars,
        execution_state,
        &inputs.climate,
        &streaming_targets,
    )?;
    execution.direct_publication =
        build_direct_publication_artifacts(runtime_selection, &inputs, &targets, &sidecars, &mut execution)?;
    let direct_runtime_counters = direct_runtime_counters_for_manifest(
        runtime_selection,
        direct_runtime_counter_baseline,
        direct_runtime_audit_snapshot(),
    );
    let execution_provenance = build_hillslope_execution_provenance(
        &execution,
        runtime_selection,
        &mut sidecars.sidecar_warnings,
    );
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
        rollback_runtime: "none".to_string(),
        compatibility_rollback_available: false,
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
