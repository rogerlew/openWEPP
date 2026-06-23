const DIRECT_PUBLICATION_EROSION_MIN_POST_INTERCEPTION_RAINFALL_M: f64 = 0.001;

struct RetainedDirectPublicationRequest<'a> {
    runtime_selection: HillslopeRuntimeSelection,
    run_name: &'a str,
    output_hillslope_id: u32,
    execution_lane: ExecutionLane,
    lane_areas_m2: &'a [f64],
    climate_request: &'a HillslopeClimateRuntimeRequest,
    climate_span: &'a ClimateRunSpanSummary,
    static_runtime_surface: &'a HillslopeWritebackSurface,
}

fn build_retained_direct_publication_frame(
    request: &RetainedDirectPublicationRequest<'_>,
) -> Result<Option<DirectPublicationExecution>, HillslopeCliError> {
    if request.runtime_selection != HillslopeRuntimeSelection::DirectPublicationFrameShadow {
        return Ok(None);
    }
    let identity = DirectRunIdentity::new(
        u64::from(request.output_hillslope_id),
        request.output_hillslope_id,
        request.lane_areas_m2.len(),
        request.climate_span.days.len(),
    )
    .map_err(|source| direct_publication_runtime_error(&source))?;
    let mut frame = DirectRunFrame::skeleton(identity)
        .map_err(|source| direct_publication_runtime_error(&source))?;
    seed_direct_publication_lane_area_geometry(&mut frame, request.lane_areas_m2)?;
    let day_input_builder = DirectPublicationDayInputBuilder::new(
        request.climate_request,
        request.climate_span,
        request.static_runtime_surface,
        request.execution_lane,
    )?;
    let metadata = DirectPublicationRunMetadata {
        run_name: request.run_name.to_string(),
        runtime_selection: request.runtime_selection.as_str().to_string(),
        output_policy: direct_publication_output_policy(request.runtime_selection).to_string(),
    };
    DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_capture_with_interleaved_day_inputs(
            &mut frame,
            metadata,
            |frame, day_index, lane_index| {
                day_input_builder
                    .build(frame, day_index, lane_index)
                    .map_err(|error| direct_publication_day_input_build_error(&error))
            },
        )
        .map(Some)
        .map_err(|source| direct_publication_runtime_error(&source))
}

fn annotate_day_runtime_error(
    error: HillslopeCliError,
    day_index: usize,
    day_projection: &ClimateDayProjection,
) -> HillslopeCliError {
    match error {
        HillslopeCliError::RuntimeSurfaceFailure { surface, detail } => {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface,
                detail: format!(
                    "{detail} [sim_day_index={}, calendar_year={}, julian_day={}]",
                    day_index + 1,
                    day_projection.year,
                    day_projection.julian_day
                ),
            }
        }
        other => other,
    }
}

fn hillslope_id_for_pass_output(output_hillslope_id: u32) -> Result<i32, HillslopeCliError> {
    i32::try_from(output_hillslope_id).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "outputs.pass_parquet",
        detail: format!("{SIMOUT_GUARD_ID} hillslope id {output_hillslope_id} exceeds i32 range"),
    })
}

fn build_direct_publication_artifacts(
    runtime_selection: HillslopeRuntimeSelection,
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
    sidecars: &HillslopeSidecarResolution,
    execution: &HillslopeClimateExecution,
) -> Result<Option<DirectPublicationArtifacts>, HillslopeCliError> {
    if !matches!(
        runtime_selection,
        HillslopeRuntimeSelection::DirectPublicationFrameShadow
            | HillslopeRuntimeSelection::DirectPublicationFrameCutover
            | HillslopeRuntimeSelection::DirectProductionExecutor
    ) {
        return Ok(None);
    }
    let direct_execution = match runtime_selection {
        HillslopeRuntimeSelection::DirectProductionExecutor => {
            let direct_execution = execution.retained_direct_publication.clone().ok_or_else(|| {
                direct_production_executor_blocked(
                    "direct production executor requires retained direct execution artifacts",
                )
            })?;
            validate_retained_direct_publication_frame(&direct_execution.publication_frame)?;
            direct_execution
        }
        HillslopeRuntimeSelection::DirectPublicationFrameCutover => {
            let direct_execution = build_direct_publication_execution_from_simulation_outputs(
                runtime_selection,
                &inputs.runfile.run_name,
                targets.output_hillslope_id,
                execution,
            )?;
            validate_retained_direct_publication_frame(&direct_execution.publication_frame)?;
            direct_execution
        }
        HillslopeRuntimeSelection::DirectPublicationFrameShadow => {
            if let Some(direct_execution) = execution.retained_direct_publication.clone() {
                validate_retained_direct_publication_frame(&direct_execution.publication_frame)?;
                direct_execution
            } else {
                let identity = DirectRunIdentity::new(
                    u64::from(targets.output_hillslope_id),
                    targets.output_hillslope_id,
                    inputs.slope.ofe_count,
                    execution.climate_span.days.len(),
                )
                .map_err(|source| direct_publication_runtime_error(&source))?;
                let mut frame = DirectRunFrame::skeleton(identity)
                    .map_err(|source| direct_publication_runtime_error(&source))?;
                seed_direct_publication_lane_geometry(&mut frame, &inputs.slope)?;
                let calendar_days = direct_publication_calendar_days(&execution.climate_span)?;
                let metadata = DirectPublicationRunMetadata {
                    run_name: inputs.runfile.run_name.clone(),
                    runtime_selection: runtime_selection.as_str().to_string(),
                    output_policy: direct_publication_output_policy(runtime_selection).to_string(),
                };
                DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
                    .run_publication_capture(&mut frame, metadata, &calendar_days)
                    .map_err(|source| direct_publication_runtime_error(&source))?
            }
        }
        HillslopeRuntimeSelection::Compatibility
        | HillslopeRuntimeSelection::DirectSkeletonNoop
        | HillslopeRuntimeSelection::DirectSkeletonShadowOnly => return Ok(None),
    };
    let publication_frame = &direct_execution.publication_frame;
    let hbp_bytes =
        build_hbp_output_from_direct_publication(&targets.output_pass, publication_frame)?;
    let wat_rows = build_hillslope_wat_rows_from_direct_publication(publication_frame)?;
    let pass_projection_rows = build_hillslope_pass_rows_from_direct_publication(publication_frame)?;
    let loss_text = build_loss_output_json_from_direct_publication(
        publication_frame,
        inputs.soil.ofes.len(),
        sidecars.snow.sidecar_present,
        sidecars.frost.wint_red,
    )?;
    let manifest_text = build_manifest_text_from_direct_publication(publication_frame)?;
    let artifacts = DirectPublicationArtifacts {
        execution: direct_execution,
        hbp_bytes,
        wat_rows,
        pass_projection_rows,
        loss_text,
        manifest_text,
    };
    validate_direct_publication_artifacts(&artifacts)?;
    Ok(Some(artifacts))
}

fn build_direct_publication_execution_from_simulation_outputs(
    runtime_selection: HillslopeRuntimeSelection,
    run_name: &str,
    output_hillslope_id: u32,
    execution: &HillslopeClimateExecution,
) -> Result<DirectPublicationExecution, HillslopeCliError> {
    let identity = DirectRunIdentity::new(
        u64::from(output_hillslope_id),
        output_hillslope_id,
        execution.contributor_ofe_count,
        execution.climate_span.days.len(),
    )
    .map_err(|source| direct_publication_runtime_error(&source))?;
    let expected_rows = direct_publication_expected_row_count(&identity)?;
    if execution.wb13_rows.len() != expected_rows {
        return Err(direct_publication_output_failure(format!(
            "simulation-owned WB13 row count {} does not match direct publication identity expected row count {expected_rows}",
            execution.wb13_rows.len()
        )));
    }

    let metadata = DirectPublicationRunMetadata {
        run_name: run_name.to_string(),
        runtime_selection: runtime_selection.as_str().to_string(),
        output_policy: direct_publication_output_policy(runtime_selection).to_string(),
    };
    let publication_scalars =
        DirectPublicationSimulationScalarIndex::from_execution_outputs(execution)?;
    let rows = execution
        .wb13_rows
        .iter()
        .map(|row| {
            direct_publication_row_from_simulation_owned_wb13(
                row,
                output_hillslope_id,
                execution,
                &publication_scalars,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let publication_frame = DirectRunPublicationFrame {
        identity,
        metadata,
        rows,
    };
    Ok(DirectPublicationExecution {
        report: direct_publication_adapter_execution_report(
            execution.contributor_ofe_count,
            execution.climate_span.days.len(),
        ),
        publication_frame,
    })
}

fn direct_publication_adapter_execution_report(
    lane_count: usize,
    day_count: usize,
) -> DirectExecutionReport {
    DirectExecutionReport {
        mode: DirectExecutorMode::ShadowOnly,
        lane_count,
        day_count,
        planned_phase_count: 0,
        canonical_phase_entry_count: 0,
        phase_view_count: 0,
        phase_status_counts: Vec::new(),
        phase_span_run_count: 0,
        direct_phase_entry_count: 0,
        direct_compute_count: 0,
        state_mutation_count: 0,
        downstream_operand_count: 0,
        shadow_projection_count: 0,
        compatibility_edge_invocation_count: 0,
        day_frame_commit_count: 0,
    }
}

fn direct_publication_row_from_simulation_owned_wb13(
    row: &SimulationOwnedWb13Row,
    output_hillslope_id: u32,
    execution: &HillslopeClimateExecution,
    publication_scalars: &DirectPublicationSimulationScalarIndex,
) -> Result<DirectPublicationDayRow, HillslopeCliError> {
    let day_index = usize::try_from(row.sim_day_index - 1).map_err(|_| {
        direct_publication_output_failure(format!(
            "simulation-owned WB13 sim_day_index must be >= 1, observed {}",
            row.sim_day_index
        ))
    })?;
    let calendar_day = execution
        .climate_span
        .days
        .get(day_index)
        .ok_or_else(|| {
            direct_publication_output_failure(format!(
                "simulation-owned WB13 sim_day_index {} exceeds climate span {}",
                row.sim_day_index,
                execution.climate_span.days.len()
            ))
        })?;
    let ofe_id = u32::from(row.wb13_row.ofe);
    let lane_index = usize::from(row.wb13_row.ofe.saturating_sub(1));
    let area_m2 = row.wb13_row.area;
    if !area_m2.is_finite() || area_m2 <= 0.0 {
        return Err(direct_publication_output_failure(format!(
            "simulation-owned WB13 row area must be finite and > 0.0, observed {area_m2}"
        )));
    }
    let runvol_m3 = depth_mm_to_volume_m3("publication.runoff.runvol_m3", row.wb13_row.qofe, area_m2)?;
    let sbrunv_m3 =
        depth_mm_to_volume_m3("publication.subsurface.sbrunv_m3", row.wb13_row.latqcc, area_m2)?;
    let runtime_scalars = direct_publication_runtime_scalars(execution)?;
    let peak_runoff_m3_s = publication_scalars.peak_runoff_m3_s(row.sim_day_index)?;
    Ok(DirectPublicationDayRow {
        run_id: u64::from(output_hillslope_id),
        hillslope_id: output_hillslope_id,
        lane_id: ofe_id,
        ofe_id,
        lane_index,
        day_index,
        sim_day_index: row.sim_day_index,
        calendar: DirectPublicationCalendarDay {
            year: calendar_day.year,
            julian_day: calendar_day.julian_day,
            month: row.month,
            day_of_month: row.day_of_month,
            water_year: row.water_year,
        },
        area_m2,
        climate: DirectPublicationClimateOperands {
            precipitation_mm: row.wb13_row.p,
        },
        liquid_input: DirectPublicationLiquidInputOperands {
            rm_mm: row.wb13_row.rm,
            irrigation_mm: row.wb13_row.irr,
        },
        runoff: DirectPublicationRunoffOperands {
            q_mm: row.wb13_row.q,
            qofe_mm: row.wb13_row.qofe,
            runvol_m3,
            peak_runoff_m3_s: Some(peak_runoff_m3_s),
            runoff_duration_s: Some(runtime_scalars.runoff_duration_s),
        },
        evaporation: DirectPublicationEvaporationOperands {
            ep_mm: row.wb13_row.ep,
            es_mm: row.wb13_row.es,
            er_mm: row.wb13_row.er,
            total_evapotranspiration_mm: row.wb13_row.ep + row.wb13_row.es + row.wb13_row.er,
        },
        subsurface: DirectPublicationSubsurfaceOperands {
            dp_mm: row.wb13_row.dp,
            latqcc_mm: row.wb13_row.latqcc,
            tile_mm: row.wb13_row.tile,
            sbrunv_m3,
        },
        transfer: DirectPublicationTransferOperands {
            upstream_surface_mm: row.wb13_row.upstrmq,
            upstream_lateral_mm: row.wb13_row.subrin,
        },
        storage: DirectPublicationStorageOperands {
            total_soil_mm: row.wb13_row.total_soil,
            soil_water_total_mm: row.wb13_row.soil_water_total,
            frozwt_mm: row.wb13_row.frozwt,
            frdp_mm: Some(row.frdp_mm),
            snow_water_mm: row.wb13_row.snow_water,
        },
        profile: DirectPublicationProfileOperands {
            depth_mm: Some(row.wb13_row.profile_depth),
            porosity_cap_mm: Some(row.wb13_row.profile_porosity_cap),
            fc_store_mm: Some(row.wb13_row.profile_fc_store),
            wp_store_mm: Some(row.wb13_row.profile_wp_store),
        },
        interception: DirectPublicationInterceptionOperands {
            interception_mm: row.interception_mm,
            interception_storage_mm: None,
        },
        erosion: direct_publication_erosion_operands_from_runtime(&runtime_scalars),
    })
}

struct DirectPublicationRuntimeScalars {
    runoff_duration_s: f64,
    hbp_total_detachment_kg: f64,
    hbp_total_deposition_kg: f64,
    hbp_sediment_concentration_kg_m3: f64,
}

fn direct_publication_runtime_scalars(
    execution: &HillslopeClimateExecution,
) -> Result<DirectPublicationRuntimeScalars, HillslopeCliError> {
    Ok(DirectPublicationRuntimeScalars {
        runoff_duration_s: optional_non_negative_runtime_scalar(
            &execution.runtime_surface,
            "watdur",
            0.0,
        )?,
        hbp_total_detachment_kg: optional_non_negative_runtime_scalar(
            &execution.runtime_surface,
            "total_detachment_kg",
            0.0,
        )?,
        hbp_total_deposition_kg: optional_non_negative_runtime_scalar(
            &execution.runtime_surface,
            "total_deposition_kg",
            0.0,
        )?,
        hbp_sediment_concentration_kg_m3: optional_non_negative_runtime_scalar(
            &execution.runtime_surface,
            "sediment_concentration_kg_m3_0001",
            0.0,
        )?,
    })
}

struct DirectPublicationSimulationScalarIndex {
    peak_runoff_m3_s_by_sim_day:
        std::collections::BTreeMap<i32, f64>,
}

impl DirectPublicationSimulationScalarIndex {
    fn from_execution_outputs(
        execution: &HillslopeClimateExecution,
    ) -> Result<Self, HillslopeCliError> {
        let mut peak_runoff_m3_s_by_sim_day =
            std::collections::BTreeMap::<i32, f64>::new();
        for row in &execution.pass_rows {
            if row.sim_day_index <= 0 {
                return Err(direct_publication_output_failure(format!(
                    "simulation-owned PASS row sim_day_index must be >= 1, observed {}",
                    row.sim_day_index
                )));
            }
            if !row.peakro_m3_s.is_finite() || row.peakro_m3_s < 0.0 {
                return Err(direct_publication_output_failure(format!(
                    "simulation-owned PASS peakro must be finite and >= 0.0 for sim day {}, observed {}",
                    row.sim_day_index, row.peakro_m3_s
                )));
            }
            if peak_runoff_m3_s_by_sim_day
                .insert(row.sim_day_index, row.peakro_m3_s)
                .is_some()
            {
                return Err(direct_publication_output_failure(format!(
                    "simulation-owned PASS emitted duplicate peakro rows for sim day {}",
                    row.sim_day_index
                )));
            }
        }
        Ok(Self {
            peak_runoff_m3_s_by_sim_day,
        })
    }

    fn peak_runoff_m3_s(&self, sim_day_index: i32) -> Result<f64, HillslopeCliError> {
        self.peak_runoff_m3_s_by_sim_day
            .get(&sim_day_index)
            .copied()
            .ok_or_else(|| {
                direct_publication_output_failure(format!(
                    "simulation-owned PASS has no peakro row for WB13 sim day {sim_day_index}"
                ))
            })
    }
}

fn direct_publication_erosion_operands_from_runtime(
    runtime_scalars: &DirectPublicationRuntimeScalars,
) -> DirectPublicationErosionOperands {
    DirectPublicationErosionOperands {
        hbp_total_detachment_kg: Some(runtime_scalars.hbp_total_detachment_kg),
        hbp_total_deposition_kg: Some(runtime_scalars.hbp_total_deposition_kg),
        hbp_sediment_concentration_kg_m3: Some(runtime_scalars.hbp_sediment_concentration_kg_m3),
        ..DirectPublicationErosionOperands::zero_authority()
    }
}

fn depth_mm_to_volume_m3(
    field: &'static str,
    depth_mm: f64,
    area_m2: f64,
) -> Result<f64, HillslopeCliError> {
    if !depth_mm.is_finite() || depth_mm < 0.0 {
        return Err(direct_publication_output_failure(format!(
            "{field} depth must be finite and >= 0.0, observed {depth_mm}"
        )));
    }
    let volume = depth_mm * area_m2 / 1_000.0;
    if volume.is_finite() && volume >= 0.0 {
        return Ok(volume);
    }
    Err(direct_publication_output_failure(format!(
        "{field} volume is invalid for depth {depth_mm} mm and area {area_m2} m2"
    )))
}

#[cfg(test)]
fn reduced_wat_mismatch_fields(
    direct_rows: &[HillslopeWatRow],
    compatibility_rows: &[HillslopeWatRow],
) -> Vec<&'static str> {
    let mut mismatches = BTreeSet::new();
    if direct_rows.len() != compatibility_rows.len() {
        mismatches.insert("row_count");
    }
    for (direct, compatibility) in direct_rows.iter().zip(compatibility_rows) {
        collect_wat_identity_mismatch_fields(&mut mismatches, direct, compatibility);
        collect_wat_required_scalar_mismatch_fields(&mut mismatches, direct, compatibility);
        collect_wat_optional_scalar_mismatch_fields(&mut mismatches, direct, compatibility);
    }
    wat_mismatch_field_order()
        .iter()
        .copied()
        .filter(|field| mismatches.contains(field))
        .collect()
}

#[cfg(test)]
fn collect_wat_identity_mismatch_fields(
    mismatches: &mut BTreeSet<&'static str>,
    direct: &HillslopeWatRow,
    compatibility: &HillslopeWatRow,
) {
    insert_mismatch_if(mismatches, "wepp_id", direct.wepp_id != compatibility.wepp_id);
    insert_mismatch_if(mismatches, "ofe_id", direct.ofe_id != compatibility.ofe_id);
    insert_mismatch_if(mismatches, "year", direct.year != compatibility.year);
    insert_mismatch_if(
        mismatches,
        "sim_day_index",
        direct.sim_day_index != compatibility.sim_day_index,
    );
    insert_mismatch_if(mismatches, "julian", direct.julian != compatibility.julian);
    insert_mismatch_if(mismatches, "month", direct.month != compatibility.month);
    insert_mismatch_if(
        mismatches,
        "day_of_month",
        direct.day_of_month != compatibility.day_of_month,
    );
    insert_mismatch_if(
        mismatches,
        "water_year",
        direct.water_year != compatibility.water_year,
    );
    insert_mismatch_if(mismatches, "ofe", direct.ofe != compatibility.ofe);
}

#[cfg(test)]
fn collect_wat_required_scalar_mismatch_fields(
    mismatches: &mut BTreeSet<&'static str>,
    direct: &HillslopeWatRow,
    compatibility: &HillslopeWatRow,
) {
    insert_float_mismatch(mismatches, "P", direct.p, compatibility.p);
    insert_float_mismatch(mismatches, "RM", direct.rm, compatibility.rm);
    insert_float_mismatch(mismatches, "Q", direct.q, compatibility.q);
    insert_float_mismatch(mismatches, "Ep", direct.ep, compatibility.ep);
    insert_float_mismatch(mismatches, "Es", direct.es, compatibility.es);
    insert_float_mismatch(mismatches, "Er", direct.er, compatibility.er);
    insert_float_mismatch(mismatches, "Dp", direct.dp, compatibility.dp);
    insert_float_mismatch(
        mismatches,
        "UpStrmQ",
        direct.up_strm_q,
        compatibility.up_strm_q,
    );
    insert_float_mismatch(mismatches, "SubRIn", direct.sub_r_in, compatibility.sub_r_in);
    insert_float_mismatch(mismatches, "latqcc", direct.latqcc, compatibility.latqcc);
    insert_float_mismatch(
        mismatches,
        "Total-Soil",
        direct.total_soil_water,
        compatibility.total_soil_water,
    );
    insert_float_mismatch(mismatches, "frozwt", direct.frozwt, compatibility.frozwt);
    insert_float_mismatch(mismatches, "frdp", direct.frdp, compatibility.frdp);
    insert_float_mismatch(
        mismatches,
        "Snow-Water",
        direct.snow_water,
        compatibility.snow_water,
    );
    insert_float_mismatch(mismatches, "QOFE", direct.qofe, compatibility.qofe);
    insert_float_mismatch(mismatches, "Tile", direct.tile, compatibility.tile);
    insert_float_mismatch(mismatches, "Irr", direct.irr, compatibility.irr);
    insert_float_mismatch(mismatches, "Area", direct.area, compatibility.area);
}

#[cfg(test)]
fn collect_wat_optional_scalar_mismatch_fields(
    mismatches: &mut BTreeSet<&'static str>,
    direct: &HillslopeWatRow,
    compatibility: &HillslopeWatRow,
) {
    insert_option_float_mismatch(
        mismatches,
        "SoilWaterTotal",
        direct.soil_water_total,
        compatibility.soil_water_total,
    );
    insert_option_float_mismatch(
        mismatches,
        "ProfileDepth",
        direct.profile_depth,
        compatibility.profile_depth,
    );
    insert_option_float_mismatch(
        mismatches,
        "ProfilePorosityCap",
        direct.profile_porosity_cap,
        compatibility.profile_porosity_cap,
    );
    insert_option_float_mismatch(
        mismatches,
        "ProfileFCStore",
        direct.profile_fc_store,
        compatibility.profile_fc_store,
    );
    insert_option_float_mismatch(
        mismatches,
        "ProfileWPStore",
        direct.profile_wp_store,
        compatibility.profile_wp_store,
    );
    insert_option_float_mismatch(
        mismatches,
        "Interception",
        direct.interception,
        compatibility.interception,
    );
    insert_option_float_mismatch(
        mismatches,
        "InterceptionStorage",
        direct.interception_storage,
        compatibility.interception_storage,
    );
}

#[cfg(test)]
fn insert_float_mismatch(
    mismatches: &mut BTreeSet<&'static str>,
    field: &'static str,
    direct: f64,
    compatibility: f64,
) {
    insert_mismatch_if(mismatches, field, direct.to_bits() != compatibility.to_bits());
}

#[cfg(test)]
fn insert_option_float_mismatch(
    mismatches: &mut BTreeSet<&'static str>,
    field: &'static str,
    direct: Option<f64>,
    compatibility: Option<f64>,
) {
    insert_mismatch_if(
        mismatches,
        field,
        direct.map(f64::to_bits) != compatibility.map(f64::to_bits),
    );
}

#[cfg(test)]
fn insert_mismatch_if(
    mismatches: &mut BTreeSet<&'static str>,
    field: &'static str,
    is_different: bool,
) {
    if is_different {
        mismatches.insert(field);
    }
}

#[cfg(test)]
fn r6f_wat_direct_process_producer_authority_gap(fields: &[&str]) -> bool {
    let expected = [
        "wepp_id",
        "year",
        "Es",
        "Total-Soil",
        "SoilWaterTotal",
        "ProfileDepth",
        "ProfilePorosityCap",
        "ProfileFCStore",
        "ProfileWPStore",
    ];
    fields.len() == expected.len() && expected.iter().all(|field| fields.contains(field))
}

#[cfg(test)]
fn r6g_wat_direct_et_storage_producer_gap(fields: &[&str]) -> bool {
    let expected = ["Es", "Total-Soil", "SoilWaterTotal"];
    fields.len() == expected.len() && expected.iter().all(|field| fields.contains(field))
}

#[cfg(test)]
fn r6g_wat_pmet_day_state_carry_gap(
    direct_rows: &[HillslopeWatRow],
    compatibility_rows: &[HillslopeWatRow],
    fields: &[&str],
) -> bool {
    if !r6g_wat_direct_et_storage_producer_gap(fields)
        || direct_rows.len() != compatibility_rows.len()
        || direct_rows.len() < 2
    {
        return false;
    }
    if direct_rows.first() != compatibility_rows.first() {
        return false;
    }
    direct_rows
        .iter()
        .zip(compatibility_rows)
        .skip(1)
        .any(|(direct, compatibility)| {
            direct.es.to_bits() != compatibility.es.to_bits()
                && direct.total_soil_water.to_bits() != compatibility.total_soil_water.to_bits()
                && direct.soil_water_total.map(f64::to_bits)
                    != compatibility.soil_water_total.map(f64::to_bits)
        })
}

#[cfg(test)]
fn r6h_wat_pmet_layer_carry_ulp_gap(
    direct_rows: &[HillslopeWatRow],
    compatibility_rows: &[HillslopeWatRow],
    fields: &[&str],
) -> bool {
    if fields.len() != 1
        || fields[0] != "Es"
        || direct_rows.len() != compatibility_rows.len()
        || direct_rows.len() < 2
    {
        return false;
    }
    if direct_rows.first() != compatibility_rows.first() {
        return false;
    }
    let mut found_ulp_mismatch = false;
    for (direct, compatibility) in direct_rows.iter().zip(compatibility_rows).skip(1) {
        if direct.es.to_bits() == compatibility.es.to_bits() {
            continue;
        }
        if (direct.es - compatibility.es).abs() > 1.0e-12 {
            return false;
        }
        found_ulp_mismatch = true;
    }
    found_ulp_mismatch
}

#[cfg(test)]
fn wat_mismatch_field_order() -> &'static [&'static str] {
    &[
        "row_count",
        "wepp_id",
        "ofe_id",
        "year",
        "sim_day_index",
        "julian",
        "month",
        "day_of_month",
        "water_year",
        "ofe",
        "P",
        "RM",
        "Q",
        "Ep",
        "Es",
        "Er",
        "Dp",
        "UpStrmQ",
        "SubRIn",
        "latqcc",
        "Total-Soil",
        "frozwt",
        "frdp",
        "Snow-Water",
        "QOFE",
        "Tile",
        "Irr",
        "Area",
        "SoilWaterTotal",
        "ProfileDepth",
        "ProfilePorosityCap",
        "ProfileFCStore",
        "ProfileWPStore",
        "Interception",
        "InterceptionStorage",
    ]
}

fn direct_publication_output_policy(runtime_selection: HillslopeRuntimeSelection) -> &'static str {
    match runtime_selection {
        HillslopeRuntimeSelection::DirectProductionExecutor => {
            "direct-production-executor/direct-publication-frame"
        }
        HillslopeRuntimeSelection::DirectPublicationFrameShadow => {
            "compatibility-public-output/direct-publication-shadow"
        }
        HillslopeRuntimeSelection::DirectPublicationFrameCutover => {
            "direct-publication-frame-cutover-candidate/fail-closed-parity"
        }
        HillslopeRuntimeSelection::Compatibility
        | HillslopeRuntimeSelection::DirectSkeletonNoop
        | HillslopeRuntimeSelection::DirectSkeletonShadowOnly => "compatibility-public-output",
    }
}

fn direct_publication_expected_row_count(
    identity: &DirectRunIdentity,
) -> Result<usize, HillslopeCliError> {
    identity
        .lane_count
        .checked_mul(identity.day_count)
        .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!("{SIMOUT_GUARD_ID} direct publication expected row count overflow"),
        })
}

fn validate_retained_direct_publication_frame(
    publication_frame: &DirectRunPublicationFrame,
) -> Result<(), HillslopeCliError> {
    let expected_row_count = direct_publication_expected_row_count(&publication_frame.identity)?;
    if publication_frame.rows().len() != expected_row_count {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} retained direct publication row count mismatch: expected {expected_row_count}, actual {}",
                publication_frame.rows().len()
            ),
        });
    }
    for row in publication_frame.rows() {
        if row.run_id != publication_frame.identity.run_id
            || row.hillslope_id != publication_frame.identity.hillslope_id
            || row.lane_index >= publication_frame.identity.lane_count
            || row.day_index >= publication_frame.identity.day_count
        {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} retained direct publication row identity is inconsistent"
                ),
            });
        }
    }
    Ok(())
}

fn seed_direct_publication_lane_geometry(
    frame: &mut DirectRunFrame,
    slope: &SlopeProfile,
) -> Result<(), HillslopeCliError> {
    if frame.lanes.len() != slope.ofes.len() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication lane count {} does not match slope OFE count {}",
                frame.lanes.len(),
                slope.ofes.len()
            ),
        });
    }
    let mut cumulative_length_m = 0.0_f64;
    for (lane, ofe) in frame.lanes.iter_mut().zip(&slope.ofes) {
        let area_m2 = ofe.fwidth * ofe.slplen;
        if !area_m2.is_finite() || area_m2 <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} OFE {} direct publication area must be finite and > 0.0, observed {area_m2}",
                    ofe.index
                ),
            });
        }
        cumulative_length_m += ofe.slplen;
        if !cumulative_length_m.is_finite() || cumulative_length_m <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} cumulative direct publication length is invalid for OFE {}: {cumulative_length_m}",
                    ofe.index
                ),
            });
        }
        lane.area_m2 = area_m2;
        lane.upstream_area_ratio = 1.0;
        lane.runoff_publication_q_scale = ofe.slplen / cumulative_length_m;
        lane.runoff_publication_qofe_scale = 1.0;
        lane.runoff_publication_efflen_m = ofe.slplen;
        lane.runoff_publication_cumulative_length_m = cumulative_length_m;
        lane.runoff_publication_ofe_length_m = ofe.slplen;
    }
    Ok(())
}

fn seed_direct_publication_lane_area_geometry(
    frame: &mut DirectRunFrame,
    lane_areas_m2: &[f64],
) -> Result<(), HillslopeCliError> {
    if frame.lanes.len() != lane_areas_m2.len() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication lane count {} does not match lane area count {}",
                frame.lanes.len(),
                lane_areas_m2.len()
            ),
        });
    }
    let mut cumulative_area_m2 = 0.0_f64;
    for (lane_index, (lane, area_m2)) in frame
        .lanes
        .iter_mut()
        .zip(lane_areas_m2.iter().copied())
        .enumerate()
    {
        if !area_m2.is_finite() || area_m2 <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct publication lane {} area must be finite and > 0.0, observed {area_m2}",
                    lane_index + 1
                ),
            });
        }
        cumulative_area_m2 += area_m2;
        if !cumulative_area_m2.is_finite() || cumulative_area_m2 <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} cumulative direct publication area is invalid for lane {}: {cumulative_area_m2}",
                    lane_index + 1
                ),
            });
        }
        lane.area_m2 = area_m2;
        lane.upstream_area_ratio = if lane_index == 0 {
            1.0
        } else {
            lane_areas_m2[lane_index - 1] / area_m2
        };
        lane.runoff_publication_q_scale = area_m2 / cumulative_area_m2;
        lane.runoff_publication_qofe_scale = 1.0;
        lane.runoff_publication_efflen_m = area_m2;
        lane.runoff_publication_cumulative_length_m = cumulative_area_m2;
        lane.runoff_publication_ofe_length_m = area_m2;
    }
    Ok(())
}


include!("direct_publication/day_input_and_helpers.rs");
