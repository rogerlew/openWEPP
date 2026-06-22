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
    let rows = execution
        .wb13_rows
        .iter()
        .map(|row| {
            direct_publication_row_from_simulation_owned_wb13(row, output_hillslope_id, execution)
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
            peak_runoff_m3_s: Some(runtime_scalars.peak_runoff_m3_s),
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
    peak_runoff_m3_s: f64,
    runoff_duration_s: f64,
    hbp_total_detachment_kg: f64,
    hbp_total_deposition_kg: f64,
    hbp_sediment_concentration_kg_m3: f64,
}

fn direct_publication_runtime_scalars(
    execution: &HillslopeClimateExecution,
) -> Result<DirectPublicationRuntimeScalars, HillslopeCliError> {
    Ok(DirectPublicationRuntimeScalars {
        peak_runoff_m3_s: optional_non_negative_runtime_scalar(
            &execution.runtime_surface,
            "peakro",
            0.0,
        )?,
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
        lane.area_m2 = area_m2;
        lane.upstream_area_ratio = 1.0;
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
        lane.area_m2 = area_m2;
        lane.upstream_area_ratio = if lane_index == 0 {
            1.0
        } else {
            lane_areas_m2[lane_index - 1] / area_m2
        };
    }
    Ok(())
}

struct DirectPublicationDayInputBuilder<'a> {
    climate_request: &'a HillslopeClimateRuntimeRequest,
    climate_span: &'a ClimateRunSpanSummary,
    static_runtime_surface: &'a HillslopeWritebackSurface,
    execution_lane: ExecutionLane,
    profile_inputs: DirectHydrologyProjectionInputs,
}

impl<'a> DirectPublicationDayInputBuilder<'a> {
    fn new(
        climate_request: &'a HillslopeClimateRuntimeRequest,
        climate_span: &'a ClimateRunSpanSummary,
        static_runtime_surface: &'a HillslopeWritebackSurface,
        execution_lane: ExecutionLane,
    ) -> Result<Self, HillslopeCliError> {
        Ok(Self {
            climate_request,
            climate_span,
            static_runtime_surface,
            execution_lane,
            profile_inputs: direct_publication_profile_inputs(static_runtime_surface)?,
        })
    }

    fn build(
        &self,
        frame: &DirectRunFrame,
        day_index: usize,
        lane_index: usize,
    ) -> Result<DirectPublicationDayInput, HillslopeCliError> {
        self.build_with_seed_surface(frame, day_index, lane_index)
            .map(|(day_input, _seed_surface)| day_input)
    }

    fn build_with_seed_surface(
        &self,
        frame: &DirectRunFrame,
        day_index: usize,
        lane_index: usize,
    ) -> Result<(DirectPublicationDayInput, HillslopeWritebackSurface), HillslopeCliError> {
        let (seed_surface, day) = self.seed_surface(frame, day_index, lane_index)?;

        let precipitation_m = day.precipitation_mm / 1_000.0;
        let mut day_input =
            DirectPublicationDayInput::calendar_only(direct_publication_calendar_day(day)?);
        day_input.precipitation_m = precipitation_m;
        day_input.effective_temperature_c = day.effective_temperature_c;
        let mut percolation_inputs =
            direct_publication_percolation_inputs(&seed_surface, precipitation_m)?;
        let mut subsurface_inputs = direct_publication_subsurface_inputs(&seed_surface)?;
        if day_index == 0 {
            day_input.initial_soil_water_m =
                Some(require_runtime_surface_scalar(&seed_surface, "wb11_soil_water")?);
        } else {
            percolation_inputs.layers.clear();
            subsurface_inputs.layers.clear();
        }
        day_input.percolation_inputs = Some(percolation_inputs);
        day_input.subsurface_compute_inputs = Some(subsurface_inputs);
        day_input.evapotranspiration_compute_inputs =
            Some(direct_publication_evapotranspiration_inputs(
                &seed_surface,
                day_index == 0,
            )?);
        day_input.hydrology_projection_inputs = Some(self.profile_inputs);
        day_input.frost_layer_carry_projection =
            direct_publication_frost_layer_carry_projection(&seed_surface)?;
        Ok((day_input, seed_surface))
    }

    fn seed_surface(
        &self,
        frame: &DirectRunFrame,
        day_index: usize,
        lane_index: usize,
    ) -> Result<(HillslopeWritebackSurface, &ClimateDayProjection), HillslopeCliError> {
        let day = self.climate_span.days.get(day_index).ok_or_else(|| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct publication day index {} exceeds climate span {}",
                    day_index + 1,
                    self.climate_span.days.len()
                ),
            }
        })?;
        direct_publication_validate_day(day)?;
        let lane = frame
            .lanes
            .get(lane_index)
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} direct publication lane index {} exceeds frame lane count {}",
                    lane_index + 1,
                    frame.lanes.len()
                ),
            })?;

        let mut seed_surface = self.static_runtime_surface.clone();
        let mut climate_surface =
            build_day_climate_surface(self.climate_request, day_index, &seed_surface, day)?;
        seed_surface = crate::hillslope::intake_lane_setup::merge_runtime_surfaces(
            seed_surface,
            std::mem::take(&mut climate_surface),
        );
        overlay_direct_publication_lane_state(&mut seed_surface, day_index, lane_index, lane)?;
        seed_wb11_runtime_surface_inputs(&mut seed_surface, self.execution_lane)?;
        Ok((seed_surface, day))
    }
}

fn direct_publication_validate_day(day: &ClimateDayProjection) -> Result<(), HillslopeCliError> {
    if !day.precipitation_mm.is_finite() || day.precipitation_mm < 0.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication precipitation must be finite and >= 0.0, observed {}",
                day.precipitation_mm
            ),
        });
    }
    if !day.effective_temperature_c.is_finite() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication effective temperature must be finite, observed {}",
                day.effective_temperature_c
            ),
        });
    }
    Ok(())
}

fn overlay_direct_publication_lane_state(
    seed_surface: &mut HillslopeWritebackSurface,
    day_index: usize,
    lane_index: usize,
    lane: &openwepp_hillslope_orchestrator::DirectLaneFrame,
) -> Result<(), HillslopeCliError> {
    if lane.subsurface_layers.is_empty() {
        if day_index == 0 {
            return Ok(());
        }
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication day {} lane {} requires committed direct-carried layers before PMET construction",
                day_index + 1,
                lane_index + 1
            ),
        });
    }
    let nsl = lane.subsurface_layers.len();
    let nsl_u32 = u32::try_from(nsl).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!(
            "{SIMOUT_GUARD_ID} direct publication lane {} layer count {nsl} exceeds u32 range",
            lane_index + 1
        ),
    })?;
    let nsl_value = f64::from(nsl_u32);
    insert_direct_seed_scalar(seed_surface, "wb11_nsl", nsl_value, lane_index)?;
    insert_direct_seed_scalar(seed_surface, "nsl", nsl_value, lane_index)?;
    let mut soil_water_m = 0.0_f64;
    for (layer_offset, layer) in lane.subsurface_layers.iter().enumerate() {
        let layer_index = layer_offset + 1;
        let unfrozen_depth_m = (layer.depth_m - layer.frozen_depth_m).max(0.0);
        soil_water_m += layer.theta_m + layer.residual_theta * unfrozen_depth_m;
        for (symbol, value) in [
            (format!("wb18_perc_theta_{layer_index:04}"), layer.theta_m),
            (
                format!("wb18_perc_fc_{layer_index:04}"),
                layer.field_capacity_m,
            ),
            (
                format!("wb18_perc_ul_{layer_index:04}"),
                layer.upper_limit_m,
            ),
            (
                format!("wb18_perc_ssc_{layer_index:04}"),
                layer.conductivity_m_s,
            ),
            (format!("wb19_dg_{layer_index:04}"), layer.depth_m),
            (
                format!("wb19_thetdr_{layer_index:04}"),
                layer.residual_theta,
            ),
            (
                format!("wb18_perc_frozen_depth_{layer_index:04}"),
                layer.frozen_depth_m,
            ),
            (
                format!("wb18_perc_frzw_{layer_index:04}"),
                layer.frozen_water_m,
            ),
            (format!("wb19_por_{layer_index:04}"), layer.porosity),
            (
                format!("wb19_thetfc_{layer_index:04}"),
                layer.field_capacity_theta,
            ),
            (format!("wb19_coca_{layer_index:04}"), layer.coca),
            (format!("coca_{layer_index:04}"), layer.coca),
        ] {
            insert_direct_seed_scalar(seed_surface, symbol.as_str(), value, lane_index)?;
        }
    }
    insert_direct_seed_scalar(seed_surface, "wb11_soil_water", soil_water_m, lane_index)
}

fn insert_direct_seed_scalar(
    seed_surface: &mut HillslopeWritebackSurface,
    symbol: &str,
    value: f64,
    lane_index: usize,
) -> Result<(), HillslopeCliError> {
    if !value.is_finite() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication lane {} carried symbol {symbol} is non-finite ({value})",
                lane_index + 1
            ),
        });
    }
    seed_surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    Ok(())
}

fn direct_publication_percolation_inputs(
    runtime_surface: &HillslopeWritebackSurface,
    _precipitation_m: f64,
) -> Result<DirectPercolationInputs, HillslopeCliError> {
    let layers = direct_publication_layer_states(runtime_surface)?;
    let soil_water_initial_m = require_runtime_surface_scalar(runtime_surface, "wb11_soil_water")?;
    let lane_substeps = scalar_to_usize(
        "wb18_perc_lane_substeps",
        require_runtime_surface_scalar(runtime_surface, "wb18_perc_lane_substeps")?,
    )?;
    Ok(DirectPercolationInputs {
        soil_water_initial_m,
        reconcile_legacy_soil_water_from_layers: false,
        same_pass_infiltration_m: 0.0,
        same_pass_infiltration_lineage: false,
        tillage_depth_m: 0.0,
        lane_substeps,
        restrictive_layer_enabled: false,
        restrictive_layer_conductivity_m_s: 0.0,
        restrictive_layer_thickness_m: 0.0,
        layers,
    })
}

fn direct_publication_subsurface_inputs(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<DirectSubsurfaceComputeInputs, HillslopeCliError> {
    let layer_states = direct_publication_layer_states(runtime_surface)?;
    let soil_depth_m = layer_states.iter().map(|layer| layer.depth_m).sum::<f64>();
    let lane_substeps = scalar_to_usize(
        "wb19_lateral_drain_lane_substeps",
        require_runtime_surface_scalar(runtime_surface, "wb19_lateral_drain_lane_substeps")?,
    )?;
    let drain_enabled = direct_publication_enabled_flag(runtime_surface, "wb19_drain_enabled")?;
    let drain_depth_m = if drain_enabled {
        require_runtime_surface_scalar(runtime_surface, "wb19_drain_depth")?
    } else {
        0.5
    };
    let drain_spacing_m = if drain_enabled {
        require_runtime_surface_scalar(runtime_surface, "wb19_drain_spacing")?
    } else {
        1.0
    };
    let drain_diameter_m = if drain_enabled {
        require_runtime_surface_scalar(runtime_surface, "wb19_drain_diameter")?
    } else {
        0.1
    };
    Ok(DirectSubsurfaceComputeInputs {
        avg_slope: require_runtime_surface_scalar(runtime_surface, "avgslp")?,
        slope_length_m: require_runtime_surface_scalar(runtime_surface, "slplen")?,
        lateral_anisotropy_ratio: require_runtime_surface_scalar(
            runtime_surface,
            "wb19_lateral_anisotropy_ratio",
        )?,
        soil_depth_m,
        solwpv_mode: scalar_to_i32(
            "solwpv",
            require_runtime_surface_scalar(runtime_surface, "solwpv")?,
        )?,
        mofe_hourly_carry_arrays_enabled: lane_substeps == 24,
        lane_substeps,
        drainage_capacity_m: 0.0,
        drain_enabled,
        drain_depth_m,
        drain_spacing_m,
        drain_diameter_m,
        layers: layer_states.into_iter().map(Into::into).collect(),
    })
}

fn direct_publication_layer_states(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Vec<DirectSubsurfaceLayerState>, HillslopeCliError> {
    let nsl = direct_publication_layer_count(runtime_surface)?;
    let mut layers = Vec::with_capacity(nsl);
    for layer_index in 1..=nsl {
        layers.push(direct_publication_layer_state(
            runtime_surface,
            layer_index,
        )?);
    }
    Ok(layers)
}

fn direct_publication_layer_count(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<usize, HillslopeCliError> {
    let nsl_symbol = if runtime_surface_symbol_value(runtime_surface, "wb11_nsl").is_some() {
        "wb11_nsl"
    } else {
        "nsl"
    };
    scalar_to_usize(
        nsl_symbol,
        require_runtime_surface_scalar(runtime_surface, nsl_symbol)?,
    )
}

fn direct_publication_frost_layer_carry_projection(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Option<Vec<DirectFrostLayerCarryProjection>>, HillslopeCliError> {
    let Some(wint_red) = runtime_surface_symbol_value(runtime_surface, "frost.options.wintRed")
    else {
        return Ok(None);
    };
    if wint_red.abs() <= 1.0e-12 {
        return Ok(None);
    }
    if (wint_red - 1.0).abs() > 1.0e-12 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} frost.options.wintRed must be 0 or 1, observed {wint_red}"
            ),
        });
    }
    let layer_count = direct_publication_layer_count(runtime_surface)?;
    let fine_top_count =
        direct_publication_frost_fine_count(runtime_surface, "frost.options.fineTop")?;
    let fine_bot_count =
        direct_publication_frost_fine_count(runtime_surface, "frost.options.fineBot")?;
    let mut projection = Vec::with_capacity(layer_count);
    for layer_index in 1..=layer_count {
        let depth_m = require_runtime_surface_scalar(
            runtime_surface,
            format!("wb19_dg_{layer_index:04}").as_str(),
        )?;
        if !depth_m.is_finite() || depth_m <= 0.0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} wb19_dg_{layer_index:04} must be finite and > 0.0, observed {depth_m}"
                ),
            });
        }
        let fine_layer_count = direct_publication_frost_fine_layer_count(
            layer_index,
            layer_count,
            depth_m,
            fine_top_count,
            fine_bot_count,
        )?;
        let fine_layer_thickness_m =
            depth_m / usize_to_scalar("frost.runtime_nfine", fine_layer_count)?;
        projection.push(DirectFrostLayerCarryProjection {
            layer_index,
            fine_layer_count,
            fine_layer_thickness_m,
        });
    }
    Ok(Some(projection))
}

fn direct_publication_frost_fine_count(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &str,
) -> Result<usize, HillslopeCliError> {
    let value = require_runtime_surface_scalar(runtime_surface, symbol)?;
    let parsed = scalar_to_usize(symbol, value)?;
    if !(1..=10).contains(&parsed) {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} {symbol} must be an integer in [1,10], observed {value}"
            ),
        });
    }
    Ok(parsed)
}

fn direct_publication_frost_fine_layer_count(
    layer_index: usize,
    layer_count: usize,
    depth_m: f64,
    fine_top_count: usize,
    fine_bot_count: usize,
) -> Result<usize, HillslopeCliError> {
    if layer_index != layer_count {
        return Ok(if layer_index < 3 {
            fine_top_count
        } else {
            fine_bot_count
        });
    }
    let spacing_mm = if layer_index > 2 {
        200.0 / usize_to_scalar("frost.options.fineBot", fine_bot_count)?
    } else {
        100.0 / usize_to_scalar("frost.options.fineTop", fine_top_count)?
    };
    let depth_mm = depth_m * 1_000.0;
    let depth_mm_trunc = depth_mm.trunc();
    let ratio_trunc = (depth_mm / spacing_mm).trunc();
    let mut count = format!("{ratio_trunc:.0}")
        .parse::<usize>()
        .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} failed converting frost fine layer ratio {ratio_trunc} to usize: {error}"
            ),
        })?;
    let count_trunc_mm =
        (usize_to_scalar("frost.runtime_nfine", count)? * spacing_mm).trunc();
    if (count_trunc_mm - depth_mm_trunc).abs() > 1.0e-12 {
        count += 1;
    }
    Ok(count.max(1))
}

fn direct_publication_layer_state(
    runtime_surface: &HillslopeWritebackSurface,
    layer_index: usize,
) -> Result<DirectSubsurfaceLayerState, HillslopeCliError> {
    let theta_m = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb18_perc_theta_{layer_index:04}").as_str(),
    )?;
    let field_capacity_m = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb18_perc_fc_{layer_index:04}").as_str(),
    )?;
    let upper_limit_m = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb18_perc_ul_{layer_index:04}").as_str(),
    )?;
    let conductivity_m_s = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb18_perc_ssc_{layer_index:04}").as_str(),
    )?;
    let depth_m = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb19_dg_{layer_index:04}").as_str(),
    )?;
    let residual_theta = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb19_thetdr_{layer_index:04}").as_str(),
    )?;
    let frozen_depth_m = runtime_surface_symbol_value(
        runtime_surface,
        format!("wb18_perc_frozen_depth_{layer_index:04}").as_str(),
    )
    .unwrap_or(0.0);
    let frozen_water_m = runtime_surface_symbol_value(
        runtime_surface,
        format!("wb18_perc_frzw_{layer_index:04}").as_str(),
    )
    .unwrap_or(0.0);
    let porosity = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb19_por_{layer_index:04}").as_str(),
    )?;
    let field_capacity_theta = require_runtime_surface_scalar(
        runtime_surface,
        format!("wb19_thetfc_{layer_index:04}").as_str(),
    )?;
    let coca = require_preferred_or_legacy_runtime_surface_scalar(
        runtime_surface,
        format!("wb19_coca_{layer_index:04}").as_str(),
        format!("coca_{layer_index:04}").as_str(),
    )?;
    Ok(DirectSubsurfaceLayerState::from(
        DirectSubsurfaceLayerInputs {
            theta_m,
            field_capacity_m,
            upper_limit_m,
            conductivity_m_s,
            depth_m,
            residual_theta,
            frozen_depth_m,
            frozen_water_m,
            porosity,
            field_capacity_theta,
            coca,
            lateral_conductivity_m_s: conductivity_m_s,
        },
    ))
}

fn direct_publication_evapotranspiration_inputs(
    runtime_surface: &HillslopeWritebackSurface,
    include_stage_state: bool,
) -> Result<DirectEvapotranspirationComputeInputs, HillslopeCliError> {
    let pmet = if runtime_surface_symbol_value(runtime_surface, "wb11_et_seed_branch_evappm")
        .is_some_and(|value| value >= 0.5)
    {
        Some(DirectEvapotranspirationPmetInputs {
            soil_evaporation_m: require_runtime_surface_scalar(runtime_surface, "pmet.es_m")?,
            plant_transpiration_m: require_runtime_surface_scalar(runtime_surface, "pmet.ep_m")?,
            soil_evaporation_storage_return_m: runtime_surface_symbol_value(
                runtime_surface,
                "pmet.es_storage_return_m",
            )
            .unwrap_or(0.0),
        })
    } else {
        None
    };
    let stage_state = if pmet.is_some() || !include_stage_state {
        None
    } else {
        direct_publication_stage_state(runtime_surface)?
    };
    Ok(DirectEvapotranspirationComputeInputs {
        et_demand_m: require_runtime_surface_scalar(runtime_surface, "wb11_et_demand")?,
        leaf_area_index: require_runtime_surface_scalar(runtime_surface, "lai")?,
        canopy_cover_fraction: require_runtime_surface_scalar(runtime_surface, "cancov")?,
        residue_interception_m: require_runtime_surface_scalar(
            runtime_surface,
            "wb17_residue_interception",
        )?,
        same_pass_infiltration_m: 0.0,
        outside_water_depth_m: 0.0,
        root_depth_m: require_runtime_surface_scalar(runtime_surface, "rtd")?,
        plant_tolerance: require_preferred_or_legacy_runtime_surface_scalar(
            runtime_surface,
            "swu_effective_pltol",
            "pltol",
        )?,
        growth_context_required: false,
        stage_state,
        pmet,
    })
}

fn direct_publication_stage_state(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Option<DirectEvapotranspirationStageState>, HillslopeCliError> {
    let s1 = runtime_surface_symbol_value(runtime_surface, "s1");
    let s2 = runtime_surface_symbol_value(runtime_surface, "s2");
    let tu = runtime_surface_symbol_value(runtime_surface, "tu");
    let tv = runtime_surface_symbol_value(runtime_surface, "tv");
    match (s1, s2, tu, tv) {
        (None, None, None, None) => Ok(None),
        (Some(s1_m), Some(s2_m), Some(threshold_m), Some(counter)) => {
            Ok(Some(DirectEvapotranspirationStageState {
                s1_m,
                s2_m,
                threshold_m,
                counter,
            }))
        }
        _ => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication WB17 stage state requires complete s1/s2/tu/tv symbols"
            ),
        }),
    }
}

fn direct_publication_enabled_flag(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &'static str,
) -> Result<bool, HillslopeCliError> {
    let value = require_runtime_surface_scalar(runtime_surface, symbol)?;
    if value.abs() <= 1.0e-12 {
        Ok(false)
    } else if (value - 1.0).abs() <= 1.0e-12 {
        Ok(true)
    } else {
        Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!("{SIMOUT_GUARD_ID} {symbol} must be 0 or 1, observed {value}"),
        })
    }
}

fn require_preferred_or_legacy_runtime_surface_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    preferred_symbol: &str,
    legacy_symbol: &str,
) -> Result<f64, HillslopeCliError> {
    if let Some(value) = runtime_surface_symbol_value(runtime_surface, preferred_symbol) {
        return Ok(value);
    }
    require_runtime_surface_scalar(runtime_surface, legacy_symbol)
}

fn direct_publication_profile_inputs(
    static_runtime_surface: &HillslopeWritebackSurface,
) -> Result<DirectHydrologyProjectionInputs, HillslopeCliError> {
    let profile_depth_m = direct_publication_static_mm_to_m(
        static_runtime_surface,
        "wb13_profile_depth_mm",
        true,
    )?;
    let profile_porosity_cap_m = direct_publication_static_mm_to_m(
        static_runtime_surface,
        "wb13_profile_porosity_cap_mm",
        false,
    )?;
    let profile_field_capacity_m =
        derive_profile_fc_store_from_authoritative_layers(static_runtime_surface)? / 1_000.0;
    let profile_wilting_point_m = direct_publication_static_mm_to_m(
        static_runtime_surface,
        "wb13_profile_wp_store_mm",
        false,
    )?;
    if profile_porosity_cap_m < profile_field_capacity_m {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} parsed profile porosity cap must be >= field capacity store"
            ),
        });
    }
    if profile_field_capacity_m < profile_wilting_point_m {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} parsed profile field capacity store must be >= wilting point store"
            ),
        });
    }
    Ok(DirectHydrologyProjectionInputs {
        aggregate_storage_tolerance_m: 1.0e-9,
        profile_depth_m: Some(profile_depth_m),
        profile_porosity_cap_m: Some(profile_porosity_cap_m),
        profile_field_capacity_m: Some(profile_field_capacity_m),
        profile_wilting_point_m: Some(profile_wilting_point_m),
        ..DirectHydrologyProjectionInputs::zero()
    })
}

fn direct_publication_static_mm_to_m(
    static_runtime_surface: &HillslopeWritebackSurface,
    symbol: &'static str,
    require_positive: bool,
) -> Result<f64, HillslopeCliError> {
    let value_mm = require_runtime_surface_scalar(static_runtime_surface, symbol)?;
    if !value_mm.is_finite()
        || if require_positive {
            value_mm <= 0.0
        } else {
            value_mm < 0.0
        }
    {
        let comparator = if require_positive { "> 0.0" } else { ">= 0.0" };
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} parsed direct publication profile symbol {symbol} must be finite and {comparator}, observed {value_mm}"
            ),
        });
    }
    Ok(value_mm / 1_000.0)
}

fn direct_publication_calendar_days(
    climate_span: &ClimateRunSpanSummary,
) -> Result<Vec<DirectPublicationCalendarDay>, HillslopeCliError> {
    let mut calendar_days = Vec::with_capacity(climate_span.days.len());
    for day in &climate_span.days {
        calendar_days.push(direct_publication_calendar_day(day)?);
    }
    Ok(calendar_days)
}

fn direct_publication_calendar_day(
    day: &ClimateDayProjection,
) -> Result<DirectPublicationCalendarDay, HillslopeCliError> {
    let month = i8::try_from(day.month).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!(
            "{SIMOUT_GUARD_ID} direct publication month out of i8 range: {}",
            day.month
        ),
    })?;
    let day_of_month =
        i8::try_from(day.day_of_month).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication day-of-month out of i8 range: {}",
                day.day_of_month
            ),
        })?;
    let water_year = if day.month >= 10 {
        day.year + 1
    } else {
        day.year
    };
    let water_year =
        i16::try_from(water_year).map_err(|_| HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!("{SIMOUT_GUARD_ID} direct publication water-year out of i16 range"),
        })?;
    Ok(DirectPublicationCalendarDay {
        year: day.year,
        julian_day: day.julian_day,
        month,
        day_of_month,
        water_year,
    })
}

fn validate_direct_publication_artifacts(
    artifacts: &DirectPublicationArtifacts,
) -> Result<(), HillslopeCliError> {
    let frame = &artifacts.execution.publication_frame;
    let row_count = frame.rows().len();
    let pass_row_count = frame.identity.day_count;
    if row_count == 0
        || artifacts.hbp_bytes.is_empty()
        || artifacts.wat_rows.len() != row_count
        || artifacts.pass_projection_rows.len() != pass_row_count
        || artifacts.loss_text.is_empty()
        || artifacts.manifest_text.is_empty()
    {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct publication consumers failed frame row-count validation"
            ),
        });
    }
    Ok(())
}

fn build_direct_publication_manifest_provenance(
    publication: &DirectRunPublicationFrame,
) -> Result<
    (
        HillslopeWb13PublicationProvenance,
        HillslopeMofeHourlyCarryProvenance,
    ),
    HillslopeCliError,
> {
    let facts = direct_publication_manifest_facts(publication)?;
    Ok((
        build_direct_publication_wb13_manifest_provenance(&facts)?,
        build_direct_publication_mofe_hourly_carry_provenance(&facts),
    ))
}

struct DirectPublicationManifestFacts<'a> {
    rows: &'a [openwepp_hillslope_orchestrator::DirectPublicationDayRow],
    first_row: &'a openwepp_hillslope_orchestrator::DirectPublicationDayRow,
    last_row: &'a openwepp_hillslope_orchestrator::DirectPublicationDayRow,
    contributor_ofe_count: usize,
    expected_row_count: usize,
    publishes_per_ofe_records: bool,
    sim_day_index_monotonic: bool,
    publication_area_m2: f64,
}

fn direct_publication_manifest_facts(
    publication: &DirectRunPublicationFrame,
) -> Result<DirectPublicationManifestFacts<'_>, HillslopeCliError> {
    let rows = publication.rows();
    let first_row = rows.first().ok_or_else(|| {
        direct_publication_cutover_blocked(
            "direct publication manifest provenance requires at least one row",
        )
    })?;
    let last_row = rows.last().ok_or_else(|| {
        direct_publication_cutover_blocked(
            "direct publication manifest provenance requires at least one row",
        )
    })?;
    let contributor_ofe_count = publication.identity.lane_count;
    if contributor_ofe_count == 0 {
        return Err(direct_publication_cutover_blocked(
            "direct publication manifest provenance requires at least one lane",
        ));
    }
    let expected_row_count = publication
        .identity
        .lane_count
        .checked_mul(publication.identity.day_count)
        .ok_or_else(|| {
            direct_publication_cutover_blocked(
                "direct publication manifest expected row count overflowed",
            )
        })?;
    if rows.len() != expected_row_count {
        return Err(direct_publication_cutover_blocked(format!(
            "direct publication manifest row count mismatch: expected {expected_row_count}, actual {}",
            rows.len()
        )));
    }
    let publishes_per_ofe_records = contributor_ofe_count > 1;
    let sim_day_index_monotonic = rows
        .windows(2)
        .all(|pair| pair[0].sim_day_index <= pair[1].sim_day_index);
    let mut area_by_ofe = BTreeMap::new();
    for row in rows {
        if !row.area_m2.is_finite() || row.area_m2 <= 0.0 {
            return Err(direct_publication_cutover_blocked(format!(
                "direct publication manifest row area must be finite and > 0.0, observed {}",
                row.area_m2
            )));
        }
        if let Some(existing) = area_by_ofe.insert(row.ofe_id, row.area_m2) {
            if existing.to_bits() != row.area_m2.to_bits() {
                return Err(direct_publication_cutover_blocked(format!(
                    "direct publication manifest area changed for OFE {}: first={}, observed={}",
                    row.ofe_id, existing, row.area_m2
                )));
            }
        }
    }
    if area_by_ofe.len() != contributor_ofe_count {
        return Err(direct_publication_cutover_blocked(format!(
            "direct publication manifest area lane count mismatch: expected {contributor_ofe_count}, observed {}",
            area_by_ofe.len()
        )));
    }
    let publication_area_m2 = area_by_ofe.values().sum();
    Ok(DirectPublicationManifestFacts {
        rows,
        first_row,
        last_row,
        contributor_ofe_count,
        expected_row_count,
        publishes_per_ofe_records,
        sim_day_index_monotonic,
        publication_area_m2,
    })
}

fn build_direct_publication_wb13_manifest_provenance(
    facts: &DirectPublicationManifestFacts<'_>,
) -> Result<HillslopeWb13PublicationProvenance, HillslopeCliError> {
    let publishes_per_ofe_records = facts.publishes_per_ofe_records;
    let identity_status = if publishes_per_ofe_records {
        MF_IDENTITY_STATUS
    } else {
        "pass-direct-publication-frame"
    };
    Ok(HillslopeWb13PublicationProvenance {
        source: WB13_PUBLICATION_SOURCE_DIRECT_PUBLICATION_FRAME.to_string(),
        projection_fallback_used: false,
        guard_id: SIMOUT_GUARD_ID.to_string(),
        replay_candidate_surfaces: Vec::new(),
        publication_ofe_policy: if publishes_per_ofe_records {
            MF_PUBLICATION_OFE_POLICY
        } else {
            MOFE04_PUBLICATION_OFE_POLICY
        }
        .to_string(),
        contributor_ofe_count: facts.contributor_ofe_count,
        static_per_ofe_slice_count: facts.contributor_ofe_count,
        per_ofe_state_policy: if publishes_per_ofe_records {
            MF_PER_OFE_STATE_POLICY
        } else {
            "direct-publication-frame-state"
        }
        .to_string(),
        per_ofe_dynamic_water_balance_state: true,
        per_ofe_dynamic_wb_state: true,
        per_ofe_record_count: direct_manifest_per_ofe_value(publishes_per_ofe_records, facts.rows.len()),
        transfer_identity_status: identity_status.to_string(),
        per_element_identity_status: identity_status.to_string(),
        aggregate_identity_status: identity_status.to_string(),
        area_policy: MOFE04_PUBLICATION_AREA_POLICY.to_string(),
        storage_lineage_policy: if publishes_per_ofe_records {
            MF_STORAGE_LINEAGE_POLICY
        } else {
            "direct-publication-frame-state"
        }
        .to_string(),
        per_ofe_internal_day_count: direct_manifest_per_ofe_value(
            publishes_per_ofe_records,
            facts.expected_row_count / facts.contributor_ofe_count,
        ),
        per_ofe_expected_record_count: direct_manifest_per_ofe_value(
            publishes_per_ofe_records,
            facts.expected_row_count,
        ),
        transfer_identity_max_abs_mm: 0.0,
        per_element_identity_max_abs_mm: 0.0,
        aggregate_transfer_cancellation_max_abs_mm: 0.0,
        hillslope_total_identity_max_abs_mm: 0.0,
        publication_area_m2: facts.publication_area_m2,
        row_count: facts.rows.len(),
        sim_day_index_monotonic: facts.sim_day_index_monotonic,
        first_row_key: direct_publication_row_key_provenance(facts.first_row)?,
        last_row_key: direct_publication_row_key_provenance(facts.last_row)?,
    })
}

fn direct_manifest_per_ofe_value(active: bool, value: usize) -> usize {
    if active {
        value
    } else {
        0
    }
}

fn build_direct_publication_mofe_hourly_carry_provenance(
    facts: &DirectPublicationManifestFacts<'_>,
) -> HillslopeMofeHourlyCarryProvenance {
    HillslopeMofeHourlyCarryProvenance {
        policy: if facts.publishes_per_ofe_records {
            MOFE_HOURLY_CARRY_POLICY
        } else {
            "single-ofe-direct-publication-no-carry"
        }
        .to_string(),
        active: facts.publishes_per_ofe_records,
        substep_count: if facts.publishes_per_ofe_records {
            MOFE_HOURLY_CARRY_ARRAY_COUNT
        } else {
            0
        },
        required_arrays: if facts.publishes_per_ofe_records {
            MOFE_HOURLY_REQUIRED_ARRAYS
                .iter()
                .map(|root| (*root).to_string())
                .collect()
        } else {
            Vec::new()
        },
        upstream_carry_total_m: 0.0,
        current_carry_total_m: 0.0,
    }
}

fn direct_publication_row_key_provenance(
    row: &openwepp_hillslope_orchestrator::DirectPublicationDayRow,
) -> Result<HillslopeWb13RowKeyProvenance, HillslopeCliError> {
    Ok(HillslopeWb13RowKeyProvenance {
        year: row.calendar.year,
        julian_day: row.calendar.julian_day,
        ofe: u16::try_from(row.ofe_id).map_err(|_| {
            direct_publication_cutover_blocked(format!(
                "direct publication manifest OFE id {} exceeds u16 range",
                row.ofe_id
            ))
        })?,
        sim_day_index: row.sim_day_index,
    })
}

#[cfg(test)]
fn reduced_pass_mismatch_fields(
    direct_rows: &[HillslopePassRow],
    compatibility_rows: &[HillslopePassRow],
) -> Vec<&'static str> {
    let mut mismatches = BTreeSet::new();
    if direct_rows.len() != compatibility_rows.len() {
        mismatches.insert("row_count");
    }
    for (direct, compatibility) in direct_rows.iter().zip(compatibility_rows) {
        insert_mismatch_if(
            &mut mismatches,
            "wepp_id",
            direct.wepp_id != compatibility.wepp_id,
        );
        insert_mismatch_if(&mut mismatches, "year", direct.year != compatibility.year);
        insert_mismatch_if(
            &mut mismatches,
            "sim_day_index",
            direct.sim_day_index != compatibility.sim_day_index,
        );
        insert_mismatch_if(
            &mut mismatches,
            "julian",
            direct.julian != compatibility.julian,
        );
        insert_mismatch_if(&mut mismatches, "month", direct.month != compatibility.month);
        insert_mismatch_if(
            &mut mismatches,
            "day_of_month",
            direct.day_of_month != compatibility.day_of_month,
        );
        insert_mismatch_if(
            &mut mismatches,
            "water_year",
            direct.water_year != compatibility.water_year,
        );
        insert_float_mismatch(
            &mut mismatches,
            "runvol",
            direct.runvol_m3,
            compatibility.runvol_m3,
        );
        insert_float_mismatch(
            &mut mismatches,
            "sbrunv",
            direct.sbrunv_m3,
            compatibility.sbrunv_m3,
        );
        insert_float_mismatch(
            &mut mismatches,
            "peakro",
            direct.peakro_m3_s,
            compatibility.peakro_m3_s,
        );
        insert_float_mismatch(
            &mut mismatches,
            "total_detachment",
            direct.total_detachment_kg,
            compatibility.total_detachment_kg,
        );
        insert_float_mismatch(
            &mut mismatches,
            "total_deposition",
            direct.total_deposition_kg,
            compatibility.total_deposition_kg,
        );
        for (index, (direct_fraction, compatibility_fraction)) in direct
            .sediment_concentration_kg_m3
            .iter()
            .zip(compatibility.sediment_concentration_kg_m3)
            .enumerate()
        {
            insert_float_mismatch(
                &mut mismatches,
                match index {
                    0 => "sediment_concentration_1",
                    1 => "sediment_concentration_2",
                    2 => "sediment_concentration_3",
                    3 => "sediment_concentration_4",
                    _ => "sediment_concentration_5",
                },
                *direct_fraction,
                compatibility_fraction,
            );
        }
    }
    pass_mismatch_field_order()
        .iter()
        .copied()
        .filter(|field| mismatches.contains(field))
        .collect()
}

#[cfg(test)]
fn pass_mismatch_field_order() -> &'static [&'static str] {
    &[
        "row_count",
        "wepp_id",
        "year",
        "sim_day_index",
        "julian",
        "month",
        "day_of_month",
        "water_year",
        "runvol",
        "sbrunv",
        "peakro",
        "total_detachment",
        "total_deposition",
        "sediment_concentration_1",
        "sediment_concentration_2",
        "sediment_concentration_3",
        "sediment_concentration_4",
        "sediment_concentration_5",
    ]
}

fn direct_publication_runtime_error(
    source: &openwepp_hillslope_orchestrator::DirectRuntimeError,
) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: source.to_string(),
    }
}

fn direct_publication_day_input_build_error(error: &HillslopeCliError) -> DirectRuntimeError {
    DirectRuntimeError::PublicationDayInputBuildFailure {
        detail: error.to_string(),
    }
}
