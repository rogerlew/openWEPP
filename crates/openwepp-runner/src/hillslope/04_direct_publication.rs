const DIRECT_PUBLICATION_EROSION_MIN_POST_INTERCEPTION_RAINFALL_M: f64 = 0.001;

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
    execution: &mut HillslopeClimateExecution,
) -> Result<Option<DirectPublicationArtifacts>, HillslopeCliError> {
    if runtime_selection != HillslopeRuntimeSelection::DirectProductionExecutor {
        return Ok(None);
    }
    let direct_execution = execution.retained_direct_publication.take().ok_or_else(|| {
        direct_production_executor_blocked(
            "direct production executor requires retained direct execution artifacts",
        )
    })?;
    validate_retained_direct_publication_frame(&direct_execution.publication_frame)?;
    let publication_frame = &direct_execution.publication_frame;
    let hbp_bytes =
        build_hbp_output_from_direct_publication(&targets.output_pass, publication_frame)?;
    let wat_rows = inputs
        .runfile
        .output_config
        .wat
        .is_some()
        .then(|| build_hillslope_wat_rows_from_direct_publication(publication_frame))
        .transpose()?;
    let pass_projection_rows = inputs
        .runfile
        .output_config
        .pass_parquet
        .is_some()
        .then(|| build_hillslope_pass_rows_from_direct_publication(publication_frame))
        .transpose()?;
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
        "Snow-Depth",
        direct.snow_depth,
        compatibility.snow_depth,
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
        "Snow-Depth",
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
        HillslopeRuntimeSelection::DefaultCandidate | HillslopeRuntimeSelection::Compatibility => {
            "compatibility-public-output"
        }
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

include!("direct_publication/day_input_and_helpers.rs");
