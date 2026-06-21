fn build_retained_direct_publication_frame(
    runtime_selection: HillslopeRuntimeSelection,
    run_name: &str,
    output_hillslope_id: u32,
    lane_areas_m2: &[f64],
    climate_span: &ClimateRunSpanSummary,
) -> Result<Option<DirectPublicationExecution>, HillslopeCliError> {
    if runtime_selection != HillslopeRuntimeSelection::DirectPublicationFrameCutover {
        return Ok(None);
    }
    let identity = DirectRunIdentity::new(
        u64::from(output_hillslope_id),
        output_hillslope_id,
        lane_areas_m2.len(),
        climate_span.days.len(),
    )
    .map_err(|source| direct_publication_runtime_error(&source))?;
    let mut frame = DirectRunFrame::skeleton(identity)
        .map_err(|source| direct_publication_runtime_error(&source))?;
    seed_direct_publication_lane_area_geometry(&mut frame, lane_areas_m2)?;
    let day_inputs = direct_publication_day_inputs(climate_span)?;
    let metadata = DirectPublicationRunMetadata {
        run_name: run_name.to_string(),
        runtime_selection: runtime_selection.as_str().to_string(),
        output_policy: direct_publication_output_policy(runtime_selection).to_string(),
    };
    DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
        .run_publication_capture_with_day_inputs(&mut frame, metadata, &day_inputs)
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

fn direct_publication_day_inputs(
    climate_span: &ClimateRunSpanSummary,
) -> Result<Vec<DirectPublicationDayInput>, HillslopeCliError> {
    let mut day_inputs = Vec::with_capacity(climate_span.days.len());
    for day in &climate_span.days {
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
        day_inputs.push(DirectPublicationDayInput {
            calendar: direct_publication_calendar_day(day)?,
            precipitation_m: day.precipitation_mm / 1_000.0,
            effective_temperature_c: day.effective_temperature_c,
        });
    }
    Ok(day_inputs)
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

fn direct_publication_typed_bridge_blocked() -> HillslopeCliError {
    direct_publication_cutover_blocked(
        "HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT \
         production direct publication producers are not retained by the runner; \
         refusing to build cutover artifacts from a skeleton direct frame or from \
         compatibility WB13/runtime/writeback surfaces",
    )
}
