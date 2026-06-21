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
    if request.runtime_selection != HillslopeRuntimeSelection::DirectPublicationFrameCutover {
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
    ) {
        return Ok(None);
    }
    let direct_execution = match runtime_selection {
        HillslopeRuntimeSelection::DirectPublicationFrameCutover => {
            let direct_execution = execution
                .retained_direct_publication
                .clone()
                .ok_or_else(direct_publication_typed_bridge_blocked)?;
            validate_retained_direct_publication_frame(&direct_execution.publication_frame)?;
            direct_execution
        }
        HillslopeRuntimeSelection::DirectPublicationFrameShadow => {
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

fn insert_float_mismatch(
    mismatches: &mut BTreeSet<&'static str>,
    field: &'static str,
    direct: f64,
    compatibility: f64,
) {
    insert_mismatch_if(mismatches, field, direct.to_bits() != compatibility.to_bits());
}

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

fn insert_mismatch_if(
    mismatches: &mut BTreeSet<&'static str>,
    field: &'static str,
    is_different: bool,
) {
    if is_different {
        mismatches.insert(field);
    }
}

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

fn r6g_wat_direct_et_storage_producer_gap(fields: &[&str]) -> bool {
    let expected = ["Es", "Total-Soil", "SoilWaterTotal"];
    fields.len() == expected.len() && expected.iter().all(|field| fields.contains(field))
}

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
        Ok(day_input)
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
    let nsl_symbol = if runtime_surface_symbol_value(runtime_surface, "wb11_nsl").is_some() {
        "wb11_nsl"
    } else {
        "nsl"
    };
    let nsl = scalar_to_usize(
        nsl_symbol,
        require_runtime_surface_scalar(runtime_surface, nsl_symbol)?,
    )?;
    let mut layers = Vec::with_capacity(nsl);
    for layer_index in 1..=nsl {
        layers.push(direct_publication_layer_state(
            runtime_surface,
            layer_index,
        )?);
    }
    Ok(layers)
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
    if row_count == 0
        || artifacts.hbp_bytes.is_empty()
        || artifacts.wat_rows.len() != row_count
        || artifacts.pass_projection_rows.len() != row_count
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

fn direct_publication_typed_bridge_blocked() -> HillslopeCliError {
    direct_publication_cutover_blocked(
        "HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT \
         production direct publication producers are not retained by the runner; \
         refusing to build cutover artifacts from a skeleton direct frame or from \
         compatibility WB13/runtime/writeback surfaces",
    )
}
