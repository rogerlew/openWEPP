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

fn direct_publication_parse_enabled_flag(
    symbol: &'static str,
    value: f64,
) -> Result<bool, HillslopeCliError> {
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
    let row_count = artifacts.execution.row_count;
    let pass_row_count = artifacts.execution.identity.day_count;
    let wat_row_count_valid = artifacts
        .wat_rows_written
        .is_none_or(|rows_written| rows_written == row_count);
    let pass_row_count_valid = artifacts
        .pass_projection_rows_written
        .is_none_or(|rows_written| rows_written == pass_row_count);
    if row_count == 0
        || artifacts.summary.row_count != row_count
        || artifacts.hbp_bytes.is_empty()
        || !wat_row_count_valid
        || !pass_row_count_valid
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

#[cfg(test)]
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

fn build_streamed_direct_publication_manifest_provenance(
    summary: &DirectPublicationOutputSummary,
) -> Result<
    (
        HillslopeWb13PublicationProvenance,
        HillslopeMofeHourlyCarryProvenance,
    ),
    HillslopeCliError,
> {
    let facts = streamed_direct_publication_manifest_facts(summary)?;
    Ok((
        build_direct_publication_wb13_manifest_provenance(&facts)?,
        build_direct_publication_mofe_hourly_carry_provenance(&facts),
    ))
}

struct DirectPublicationManifestFacts<'a> {
    first_row: &'a openwepp_hillslope_orchestrator::DirectPublicationDayRow,
    last_row: &'a openwepp_hillslope_orchestrator::DirectPublicationDayRow,
    contributor_ofe_count: usize,
    expected_row_count: usize,
    row_count: usize,
    publishes_per_ofe_records: bool,
    sim_day_index_monotonic: bool,
    publication_area_m2: f64,
    upstream_carry_total_m: f64,
}

#[cfg(test)]
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
    let upstream_carry_total_m = sum_direct_publication_upstream_carry_m(rows)?;
    Ok(DirectPublicationManifestFacts {
        first_row,
        last_row,
        contributor_ofe_count,
        expected_row_count,
        row_count: rows.len(),
        publishes_per_ofe_records,
        sim_day_index_monotonic,
        publication_area_m2,
        upstream_carry_total_m,
    })
}

fn streamed_direct_publication_manifest_facts(
    summary: &DirectPublicationOutputSummary,
) -> Result<DirectPublicationManifestFacts<'_>, HillslopeCliError> {
    let first_row = summary.first_row()?;
    let last_row = summary.last_row()?;
    let contributor_ofe_count = summary.identity.lane_count;
    if contributor_ofe_count == 0 {
        return Err(direct_publication_cutover_blocked(
            "direct publication manifest provenance requires at least one lane",
        ));
    }
    let expected_row_count = summary
        .identity
        .lane_count
        .checked_mul(summary.identity.day_count)
        .ok_or_else(|| {
            direct_publication_cutover_blocked(
                "direct publication manifest expected row count overflowed",
            )
        })?;
    if summary.row_count != expected_row_count {
        return Err(direct_publication_cutover_blocked(format!(
            "direct publication manifest row count mismatch: expected {expected_row_count}, actual {}",
            summary.row_count
        )));
    }
    if summary.area_by_ofe.len() != contributor_ofe_count {
        return Err(direct_publication_cutover_blocked(format!(
            "direct publication manifest area lane count mismatch: expected {contributor_ofe_count}, observed {}",
            summary.area_by_ofe.len()
        )));
    }
    let publishes_per_ofe_records = contributor_ofe_count > 1;
    Ok(DirectPublicationManifestFacts {
        first_row,
        last_row,
        contributor_ofe_count,
        expected_row_count,
        row_count: summary.row_count,
        publishes_per_ofe_records,
        sim_day_index_monotonic: summary.sim_day_index_monotonic,
        publication_area_m2: summary.publication_area_m2(),
        upstream_carry_total_m: summary.upstream_carry_total_m(),
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
        per_ofe_record_count: direct_manifest_per_ofe_value(publishes_per_ofe_records, facts.row_count),
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
        row_count: facts.row_count,
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
    let upstream_carry_total_m = if facts.publishes_per_ofe_records {
        facts.upstream_carry_total_m
    } else {
        0.0
    };
    let current_carry_total_m = upstream_carry_total_m;
    HillslopeMofeHourlyCarryProvenance {
        policy: MOFE_HOURLY_CARRY_POLICY.to_string(),
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
        upstream_carry_total_m,
        current_carry_total_m,
    }
}

#[cfg(test)]
fn sum_direct_publication_upstream_carry_m(
    rows: &[openwepp_hillslope_orchestrator::DirectPublicationDayRow],
) -> Result<f64, HillslopeCliError> {
    let mut total_mm = 0.0_f64;
    for row in rows {
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
        total_mm += upstream_surface_mm + upstream_lateral_mm;
    }
    let total_m = total_mm / 1000.0;
    if !total_m.is_finite() || total_m < 0.0 {
        return Err(direct_publication_cutover_blocked(format!(
            "direct publication manifest carry total is invalid: {total_m}"
        )));
    }
    Ok(total_m)
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
