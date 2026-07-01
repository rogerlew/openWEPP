const DIRECT_PUBLICATION_EROSION_MIN_POST_INTERCEPTION_RAINFALL_M: f64 = 0.001;
const DIRECT_PUBLICATION_PARQUET_ROW_GROUP_ROWS: usize = 8192;

struct DirectPublicationStreamingSink {
    summary: DirectPublicationOutputSummary,
    expected_row_count: usize,
    outlet_ofe_id: u32,
    simulation_start_year: Option<i32>,
    wat_writer: Option<HillslopeWatParquetRowGroupWriter>,
    wat_chunk: Vec<HillslopeWatRow>,
    wat_rows_written: usize,
    pass_writer: Option<HillslopePassParquetRowGroupWriter>,
    pass_chunk: Vec<HillslopePassRow>,
    pass_projection_rows_written: usize,
}

impl DirectPublicationStreamingSink {
    fn create(
        identity: DirectRunIdentity,
        metadata: DirectPublicationRunMetadata,
        targets: &DirectPublicationStreamingTargets,
    ) -> Result<Self, HillslopeCliError> {
        let expected_row_count =
            identity
                .lane_count
                .checked_mul(identity.day_count)
                .ok_or_else(|| {
                    direct_publication_output_failure(
                        "direct publication expected row count overflow",
                    )
                })?;
        let outlet_ofe_id = u32::try_from(identity.lane_count).map_err(|_| {
            direct_publication_output_failure(format!(
                "direct publication lane count out of u32 range: {}",
                identity.lane_count
            ))
        })?;
        let wat_writer = targets
            .wat
            .as_deref()
            .map(|path| {
                HillslopeWatParquetRowGroupWriter::create(path, InterchangeVersion::default())
                    .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                        surface: "outputs.wat",
                        detail: error.to_string(),
                    })
            })
            .transpose()?;
        let pass_writer = targets
            .pass_parquet
            .as_deref()
            .map(|path| {
                HillslopePassParquetRowGroupWriter::create(path, InterchangeVersion::default())
                    .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                        surface: "outputs.pass_parquet",
                        detail: error.to_string(),
                    })
            })
            .transpose()?;
        Ok(Self {
            summary: DirectPublicationOutputSummary::new(identity, metadata),
            expected_row_count,
            outlet_ofe_id,
            simulation_start_year: None,
            wat_writer,
            wat_chunk: Vec::with_capacity(DIRECT_PUBLICATION_PARQUET_ROW_GROUP_ROWS),
            wat_rows_written: 0,
            pass_writer,
            pass_chunk: Vec::with_capacity(DIRECT_PUBLICATION_PARQUET_ROW_GROUP_ROWS),
            pass_projection_rows_written: 0,
        })
    }

    fn observe_row(
        &mut self,
        row: &DirectPublicationDayRow,
    ) -> Result<(), HillslopeCliError> {
        require_direct_publication_output_family_authority_row(row)?;
        let simulation_start_year = *self
            .simulation_start_year
            .get_or_insert(row.calendar.year);
        self.summary.observe(row)?;
        if self.wat_writer.is_some() {
            let wat_row =
                build_hillslope_wat_row_from_direct_publication(row, simulation_start_year)?;
            self.wat_chunk.push(wat_row);
            if self.wat_chunk.len() >= DIRECT_PUBLICATION_PARQUET_ROW_GROUP_ROWS {
                self.flush_wat_chunk()?;
            }
        }
        if self.pass_writer.is_some() && row.ofe_id == self.outlet_ofe_id {
            let pass_row =
                build_hillslope_pass_row_from_direct_publication(row, simulation_start_year)?;
            self.pass_chunk.push(pass_row);
            if self.pass_chunk.len() >= DIRECT_PUBLICATION_PARQUET_ROW_GROUP_ROWS {
                self.flush_pass_chunk()?;
            }
        }
        Ok(())
    }

    fn finish(mut self) -> Result<DirectPublicationStreamResult, HillslopeCliError> {
        self.summary.validate_complete(self.expected_row_count)?;
        self.flush_wat_chunk()?;
        self.flush_pass_chunk()?;
        let wat_rows_written = self
            .wat_writer
            .take()
            .map(|writer| {
                writer.close().map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "outputs.wat",
                    detail: error.to_string(),
                })
            })
            .transpose()?
            .map(|summary| summary.rows_written);
        let pass_projection_rows_written = self
            .pass_writer
            .take()
            .map(|writer| {
                writer.close().map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                    surface: "outputs.pass_parquet",
                    detail: error.to_string(),
                })
            })
            .transpose()?
            .map(|summary| summary.rows_written);
        Ok(DirectPublicationStreamResult {
            summary: self.summary,
            wat_rows_written,
            pass_projection_rows_written,
        })
    }

    fn flush_wat_chunk(&mut self) -> Result<(), HillslopeCliError> {
        if self.wat_chunk.is_empty() {
            return Ok(());
        }
        let writer = self.wat_writer.as_mut().ok_or_else(|| {
            direct_publication_output_failure("direct WAT chunk exists without a WAT writer")
        })?;
        writer
            .write_rows(&self.wat_chunk)
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.wat",
                detail: error.to_string(),
            })?;
        self.wat_rows_written =
            self.wat_rows_written
                .checked_add(self.wat_chunk.len())
                .ok_or_else(|| {
                    direct_publication_output_failure("direct WAT row count overflow")
                })?;
        self.wat_chunk.clear();
        Ok(())
    }

    fn flush_pass_chunk(&mut self) -> Result<(), HillslopeCliError> {
        if self.pass_chunk.is_empty() {
            return Ok(());
        }
        let writer = self.pass_writer.as_mut().ok_or_else(|| {
            direct_publication_output_failure("direct PASS chunk exists without a PASS writer")
        })?;
        writer
            .write_rows(&self.pass_chunk)
            .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "outputs.pass_parquet",
                detail: error.to_string(),
            })?;
        self.pass_projection_rows_written = self
            .pass_projection_rows_written
            .checked_add(self.pass_chunk.len())
            .ok_or_else(|| direct_publication_output_failure("direct PASS row count overflow"))?;
        self.pass_chunk.clear();
        Ok(())
    }
}

impl DirectPublicationOutputSummary {
    fn new(identity: DirectRunIdentity, metadata: DirectPublicationRunMetadata) -> Self {
        Self {
            identity,
            metadata,
            row_count: 0,
            first_row: None,
            last_row: None,
            hbp_sediment_row: None,
            parity_grade_row_seen: false,
            area_by_ofe: BTreeMap::new(),
            sim_day_index_monotonic: true,
            previous_sim_day_index: None,
            upstream_carry_total_mm: 0.0,
        }
    }

    fn observe(&mut self, row: &DirectPublicationDayRow) -> Result<(), HillslopeCliError> {
        self.observe_manifest_aggregates(row)?;
        if self.row_count == 0 {
            self.first_row = Some(row.clone());
        }
        if direct_publication_row_has_hbp_sediment(row) {
            self.hbp_sediment_row = Some(row.clone());
        }
        if !direct_publication_row_lacks_parity_grade_output_producers(row) {
            self.parity_grade_row_seen = true;
        }
        self.last_row = Some(row.clone());
        self.row_count = self.row_count.checked_add(1).ok_or_else(|| {
            direct_publication_output_failure("direct publication summary row count overflow")
        })?;
        Ok(())
    }

    fn observe_manifest_aggregates(
        &mut self,
        row: &DirectPublicationDayRow,
    ) -> Result<(), HillslopeCliError> {
        if !row.area_m2.is_finite() || row.area_m2 <= 0.0 {
            return Err(direct_publication_cutover_blocked(format!(
                "direct publication manifest row area must be finite and > 0.0, observed {}",
                row.area_m2
            )));
        }
        if let Some(existing) = self.area_by_ofe.insert(row.ofe_id, row.area_m2) {
            if existing.to_bits() != row.area_m2.to_bits() {
                return Err(direct_publication_cutover_blocked(format!(
                    "direct publication manifest area changed for OFE {}: first={}, observed={}",
                    row.ofe_id, existing, row.area_m2
                )));
            }
        }
        if self
            .previous_sim_day_index
            .is_some_and(|previous| previous > row.sim_day_index)
        {
            self.sim_day_index_monotonic = false;
        }
        self.previous_sim_day_index = Some(row.sim_day_index);
        let upstream_surface_mm = row.transfer.upstream_surface_mm;
        let upstream_lateral_mm = row.transfer.upstream_lateral_mm;
        if !upstream_surface_mm.is_finite()
            || upstream_surface_mm < 0.0
            || !upstream_lateral_mm.is_finite()
            || upstream_lateral_mm < 0.0
        {
            return Err(direct_publication_cutover_blocked(format!(
                "direct publication manifest carry totals require finite nonnegative transfer operands, observed upstream_surface_mm={} upstream_lateral_mm={} for OFE {} sim day {}",
                upstream_surface_mm, upstream_lateral_mm, row.ofe_id, row.sim_day_index
            )));
        }
        self.upstream_carry_total_mm += upstream_surface_mm + upstream_lateral_mm;
        if !self.upstream_carry_total_mm.is_finite() || self.upstream_carry_total_mm < 0.0 {
            return Err(direct_publication_cutover_blocked(
                "direct publication manifest carry total is invalid",
            ));
        }
        Ok(())
    }

    fn validate_complete(&self, expected_row_count: usize) -> Result<(), HillslopeCliError> {
        if self.row_count != expected_row_count {
            return Err(direct_publication_output_failure(format!(
                "direct publication row count {} does not match expected row count {expected_row_count}",
                self.row_count
            )));
        }
        if self.first_row.is_none() || self.last_row.is_none() {
            return Err(direct_publication_output_failure(
                "direct publication summary requires first and last rows",
            ));
        }
        if self.area_by_ofe.len() != self.identity.lane_count {
            return Err(direct_publication_cutover_blocked(format!(
                "direct publication manifest area lane count mismatch: expected {}, observed {}",
                self.identity.lane_count,
                self.area_by_ofe.len()
            )));
        }
        Ok(())
    }

    fn first_row(&self) -> Result<&DirectPublicationDayRow, HillslopeCliError> {
        self.first_row.as_ref().ok_or_else(|| {
            direct_publication_output_failure("missing first direct publication row")
        })
    }

    fn last_row(&self) -> Result<&DirectPublicationDayRow, HillslopeCliError> {
        self.last_row.as_ref().ok_or_else(|| {
            direct_publication_output_failure("missing last direct publication row")
        })
    }

    fn hbp_sediment_row(&self) -> Option<&DirectPublicationDayRow> {
        self.hbp_sediment_row.as_ref()
    }

    fn publication_area_m2(&self) -> f64 {
        self.area_by_ofe.values().sum()
    }

    fn upstream_carry_total_m(&self) -> f64 {
        self.upstream_carry_total_mm / 1_000.0
    }
}

fn build_hbp_output_from_direct_publication_summary(
    output_pass: &Path,
    summary: &DirectPublicationOutputSummary,
) -> Result<Vec<u8>, HillslopeCliError> {
    let latest_row = summary.last_row()?;
    let sediment_row = summary.hbp_sediment_row().unwrap_or(latest_row);
    let nofe = u16::try_from(summary.identity.lane_count).map_err(|_| {
        direct_publication_output_failure(format!(
            "direct publication lane count out of u16 range: {}",
            summary.identity.lane_count
        ))
    })?;
    if nofe == 0 {
        return Err(direct_publication_output_failure(
            "direct publication lane count must be >= 1",
        ));
    }
    let sediment_concentration_kg_m3 = sediment_row
        .erosion
        .hbp_sediment_concentration_kg_m3
        .map_or_else(
            || {
                direct_publication_required_sediment_concentration(
                    sediment_row.erosion.sediment_concentration_kg_m3,
                )
                .map(|values| values[0])
            },
            |value| {
                direct_publication_required_erosion_scalar(
                    "erosion.hbp_sediment_concentration_kg_m3",
                    Some(value),
                )
            },
        )?;

    build_schema1_hbp_event_fixture(HbpEventFixtureInput {
        hillslope_id: parse_hillslope_id_from_output_pass_path(output_pass)?,
        nofe,
        julian_day: latest_row.calendar.julian_day,
        peak_runoff_m3_s: direct_publication_required_erosion_scalar(
            "runoff.peak_runoff_m3_s or erosion.peak_runoff_m3_s",
            latest_row
                .runoff
                .peak_runoff_m3_s
                .or(latest_row.erosion.peak_runoff_m3_s),
        )?,
        duration_seconds: direct_publication_required_erosion_scalar(
            "runoff.runoff_duration_s or erosion.runoff_duration_s",
            latest_row
                .runoff
                .runoff_duration_s
                .or(latest_row.erosion.runoff_duration_s),
        )?,
        total_detachment_kg: direct_publication_required_erosion_scalar(
            "erosion.hbp_total_detachment_kg or erosion.total_detachment_kg",
            sediment_row
                .erosion
                .hbp_total_detachment_kg
                .or(sediment_row.erosion.total_detachment_kg),
        )?,
        total_deposition_kg: direct_publication_required_erosion_scalar(
            "erosion.hbp_total_deposition_kg or erosion.total_deposition_kg",
            sediment_row
                .erosion
                .hbp_total_deposition_kg
                .or(sediment_row.erosion.total_deposition_kg),
        )?,
        sediment_concentration_kg_m3,
        particle_flow_fraction: 1.0,
        particle_diameter_m: HBP_DEFAULT_PARTICLE_DIAMETER_M,
    })
}

fn build_loss_output_json_from_direct_publication_summary(
    summary: &DirectPublicationOutputSummary,
    ofe_count: usize,
    snow_override_applied: bool,
    frost_wint_red: i32,
) -> Result<String, HillslopeCliError> {
    let first_day = summary.first_row()?;
    let last_day = summary.last_row()?;
    let payload = serde_json::json!({
        "schema": "openwepp-hillslope-loss-v1",
        "run_name": summary.metadata.run_name,
        "first_day_year": first_day.calendar.year,
        "first_day_julian": first_day.calendar.julian_day,
        "last_day_year": last_day.calendar.year,
        "last_day_julian": last_day.calendar.julian_day,
        "precipitation_mm": first_day.climate.precipitation_mm,
        "climate_day_count": summary.identity.day_count,
        "executed_day_count": summary.identity.day_count,
        "ofe_count": ofe_count,
        "snow_override_applied": snow_override_applied,
        "frost_wint_red": frost_wint_red,
    });

    serde_json::to_string_pretty(&payload)
        .map_err(|source| HillslopeCliError::ManifestSerialize { source })
}

fn build_manifest_text_from_direct_publication_summary(
    summary: &DirectPublicationOutputSummary,
) -> Result<String, HillslopeCliError> {
    summary.validate_complete(summary.row_count)?;
    Ok(format!(
        "direct_publication_frame_v1\nrun_name={}\nruntime_selection={}\noutput_policy={}\nrow_count={}\nlane_count={}\nday_count={}\n",
        summary.metadata.run_name,
        summary.metadata.runtime_selection,
        summary.metadata.output_policy,
        summary.row_count,
        summary.identity.lane_count,
        summary.identity.day_count
    ))
}

fn build_direct_publication_artifacts(
    runtime_selection: HillslopeRuntimeSelection,
    inputs: &ParsedHillslopeRunInputs,
    targets: &HillslopeOutputTargets,
    sidecars: &HillslopeSidecarResolution,
    execution: &mut HillslopeClimateExecution,
) -> Result<Option<DirectPublicationArtifacts>, HillslopeCliError> {
    debug_assert_eq!(
        runtime_selection,
        HillslopeRuntimeSelection::DirectProductionExecutor
    );
    let retained = execution.retained_direct_publication.take().ok_or_else(|| {
        direct_production_executor_blocked(
            "direct production executor requires retained direct execution artifacts",
        )
    })?;
    validate_streamed_direct_publication(&retained.execution, &retained.stream)?;
    let summary = retained.stream.summary;
    let hbp_bytes = build_hbp_output_from_direct_publication_summary(&targets.output_pass, &summary)?;
    let loss_text = build_loss_output_json_from_direct_publication_summary(
        &summary,
        inputs.soil.ofes.len(),
        sidecars.snow.sidecar_present,
        sidecars.frost.wint_red,
    )?;
    let manifest_text = build_manifest_text_from_direct_publication_summary(&summary)?;
    let artifacts = DirectPublicationArtifacts {
        execution: retained.execution,
        summary,
        hbp_bytes,
        wat_rows_written: retained.stream.wat_rows_written,
        pass_projection_rows_written: retained.stream.pass_projection_rows_written,
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
        HillslopeRuntimeSelection::DirectProductionExecutor
        | HillslopeRuntimeSelection::DefaultCandidate => {
            "direct-production-executor/direct-publication-frame"
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

#[cfg(test)]
#[allow(dead_code)]
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

fn validate_streamed_direct_publication(
    execution: &DirectStreamingPublicationExecution,
    stream: &DirectPublicationStreamResult,
) -> Result<(), HillslopeCliError> {
    let expected_row_count = direct_publication_expected_row_count(&execution.identity)?;
    if execution.row_count != expected_row_count || stream.summary.row_count != expected_row_count {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} streamed direct publication row count mismatch: expected {expected_row_count}, execution {}, summary {}",
                execution.row_count, stream.summary.row_count
            ),
        });
    }
    if execution.identity != stream.summary.identity || execution.metadata != stream.summary.metadata
    {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} streamed direct publication summary identity is inconsistent"
            ),
        });
    }
    for row in [
        stream.summary.first_row.as_ref(),
        stream.summary.last_row.as_ref(),
        stream.summary.hbp_sediment_row.as_ref(),
    ]
    .into_iter()
    .flatten()
    {
        if row.run_id != execution.identity.run_id
            || row.hillslope_id != execution.identity.hillslope_id
            || row.lane_index >= execution.identity.lane_count
            || row.day_index >= execution.identity.day_count
        {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_publication_frame",
                detail: format!(
                    "{SIMOUT_GUARD_ID} streamed direct publication row identity is inconsistent"
                ),
            });
        }
    }
    Ok(())
}

include!("direct_publication/day_input_and_helpers.rs");
