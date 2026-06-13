#[allow(clippy::too_many_lines)]
pub(super) fn build_simimpl10_coupling_vector_provenance(
    runtime_surface: &HillslopeWritebackSurface,
    wb13_row: &SimulationOwnedWb13Row,
) -> Result<HillslopeCouplingVectorProvenance, HillslopeCliError> {
    let snow_file_present = parse_simimpl10_binary_flag(
        "snow.options.snow_file_present",
        require_simimpl10_coupling_scalar(runtime_surface, "snow.options.snow_file_present")?,
    )?;
    let rst = require_simimpl10_coupling_scalar(runtime_surface, "snow.options.rst")?;
    let newsnw = require_simimpl10_coupling_scalar(runtime_surface, "snow.options.newsnw")?;
    let ssd = require_simimpl10_coupling_scalar(runtime_surface, "snow.options.ssd")?;
    let runtime_swe = wb13_row.wb13_row.snow_water / 1_000.0;

    if newsnw <= 0.0 {
        return Err(simcoup_failure(format!(
            "snow.options.newsnw must be > 0.0, observed {newsnw}"
        )));
    }
    if ssd <= 0.0 {
        return Err(simcoup_failure(format!(
            "snow.options.ssd must be > 0.0, observed {ssd}"
        )));
    }
    if newsnw > ssd {
        return Err(simcoup_failure(format!(
            "snow.options.newsnw must be <= snow.options.ssd, observed {newsnw} > {ssd}"
        )));
    }
    if runtime_swe < 0.0 {
        return Err(simcoup_failure(format!(
            "snow.runtime_swe must be >= 0.0, observed {runtime_swe}"
        )));
    }

    let frost_file_present = parse_simimpl10_binary_flag(
        "frost.options.frost_file_present",
        require_simimpl10_coupling_scalar(runtime_surface, "frost.options.frost_file_present")?,
    )?;
    let wint_red_enabled = parse_simimpl10_binary_flag(
        "frost.options.wintRed",
        require_simimpl10_coupling_scalar(runtime_surface, "frost.options.wintRed")?,
    )?;
    let dfrost = require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_dfrost")?;
    let dthaw = require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_dthaw")?;
    let nft = require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_nft")?;
    let ws_frz = require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_ws_frz")?;
    let infcap_frz =
        require_simimpl10_coupling_scalar(runtime_surface, "frost.runtime_infcap_frz")?;
    let ssc = require_simimpl10_coupling_scalar(runtime_surface, "ssc")?;
    let profile_depth_m = require_simimpl10_coupling_scalar(runtime_surface, "solthk")?;
    let tmax = require_simimpl10_coupling_scalar(runtime_surface, "tmax")?;
    let tmin = require_simimpl10_coupling_scalar(runtime_surface, "tmin")?;
    let winter_active =
        runtime_swe > 0.0 || dfrost > 0.0 || ws_frz > 0.0 || f64::midpoint(tmax, tmin) < 0.0;

    let winter = HillslopeWinterCouplingProvenance {
        active: winter_active,
        snow_file_present,
        rst,
        newsnw,
        ssd,
        runtime_swe,
    };

    if profile_depth_m <= 0.0 {
        return Err(simcoup_failure(format!(
            "solthk must be > 0.0 for frozen-soil coupling depth bounds, observed {profile_depth_m}"
        )));
    }
    if !(0.0..=profile_depth_m).contains(&dfrost) {
        return Err(simcoup_failure(format!(
            "frost.runtime_dfrost must be within [0.0,{profile_depth_m}], observed {dfrost}"
        )));
    }
    if !(0.0..=profile_depth_m).contains(&dthaw) {
        return Err(simcoup_failure(format!(
            "frost.runtime_dthaw must be within [0.0,{profile_depth_m}], observed {dthaw}"
        )));
    }
    if nft < 0.0 {
        return Err(simcoup_failure(format!(
            "frost.runtime_nft must be >= 0.0, observed {nft}"
        )));
    }
    if ws_frz < 0.0 {
        return Err(simcoup_failure(format!(
            "frost.runtime_ws_frz must be >= 0.0, observed {ws_frz}"
        )));
    }
    if ssc < 0.0 {
        return Err(simcoup_failure(format!(
            "ssc must be >= 0.0 for frozen-soil coupling, observed {ssc}"
        )));
    }
    if infcap_frz < 0.0 || infcap_frz > ssc {
        return Err(simcoup_failure(format!(
            "frost.runtime_infcap_frz must be within [0.0,ssc], observed {infcap_frz} with ssc={ssc}"
        )));
    }

    let frsoil_active = wint_red_enabled;
    let frsoil = HillslopeFrozenSoilCouplingProvenance {
        active: frsoil_active,
        frost_file_present,
        wint_red_enabled,
        dfrost,
        dthaw,
        nft,
        ws_frz,
        infcap_frz,
    };
    let soil = HillslopeSoilCouplingProvenance {
        ssc,
        infiltration_capacity_frozen: infcap_frz,
        infcap_within_ssc: infcap_frz <= ssc,
    };

    let total_soil = wb13_row.wb13_row.total_soil;
    let frozwt = wb13_row.wb13_row.frozwt;
    let snow_water = wb13_row.wb13_row.snow_water;
    let soil_water_total = wb13_row.wb13_row.soil_water_total;
    let closure_delta = soil_water_total - total_soil;
    let closure_within_tolerance = closure_delta.abs() <= SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM;
    if !closure_within_tolerance {
        return Err(simcoup_failure(format!(
            "hydout-equivalent closure violated: SoilWaterTotal - Total-Soil = {closure_delta} exceeds tolerance {SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM}",
        )));
    }

    let hydout_equivalent = HillslopeHydoutEquivalentCouplingProvenance {
        source: WB13_PUBLICATION_SOURCE_SIMULATION_OWNED.to_string(),
        total_soil,
        frozwt,
        snow_water,
        soil_water_total,
        closure_delta,
        closure_tolerance: SIMIMPL10_SOIL_WATER_TOTAL_TOLERANCE_MM,
        closure_within_tolerance,
    };

    Ok(HillslopeCouplingVectorProvenance {
        guard_id: SIMCOUP_GUARD_ID.to_string(),
        winter,
        soil,
        frsoil,
        hydout_equivalent,
    })
}

pub(super) const MOFE04_PUBLICATION_OFE_POLICY: &str = "single-row-canonicalized-hillslope-aggregate";
pub(super) const MF_PUBLICATION_OFE_POLICY: &str = "per-ofe-dynamic-water-balance-state";
pub(super) const MOFE04_PUBLICATION_AREA_POLICY: &str = "sum-ofe-geometry-area";
pub(super) const HPHYS0255_STORAGE_LINEAGE_POLICY: &str = "single-runtime-wb11-state";
pub(super) const MF_STORAGE_LINEAGE_POLICY: &str = "per-ofe-dynamic-wb-state";
pub(super) const ME1_PER_OFE_STATE_POLICY: &str = "shadow-static-slices-only";
pub(super) const ME1_IDENTITY_STATUS: &str = "not-run-shadow-state-only";
pub(super) const ME3_PER_OFE_STATE_POLICY: &str = "persistent-dynamic-state-shadow";
pub(super) const ME3_IDENTITY_STATUS: &str = "not-run-dynamic-state-only";
pub(super) const ME4_PER_OFE_STATE_POLICY: &str = "internal-per-ofe-wb13-records";
pub(super) const ME4_IDENTITY_STATUS: &str = "pass-internal-wb13-records";
pub(super) const MF_PER_OFE_STATE_POLICY: &str = "published-per-ofe-wb13-records";
pub(super) const MF_IDENTITY_STATUS: &str = "pass-published-per-ofe-wb13-records";

fn validate_wb13_publication_common_inputs(
    rows: &[SimulationOwnedWb13Row],
    contributor_ofe_count: usize,
    static_per_ofe_slice_count: usize,
    publication_area_m2: f64,
) -> Result<(), HillslopeCliError> {
    if rows.is_empty() {
        return Err(wb13_simout_failure(
            "WB13 publication requires at least one executed-day row",
        ));
    }
    if rows.iter().any(|row| row.sim_day_index <= 0) {
        return Err(wb13_simout_failure(
            "sim_day_index must be positive for every WB13 publication row",
        ));
    }
    if contributor_ofe_count == 0 {
        return Err(wb13_simout_failure(
            "contributor_ofe_count must be >= 1 for WB13 publication provenance",
        ));
    }
    if static_per_ofe_slice_count != contributor_ofe_count {
        return Err(wb13_simout_failure(format!(
            "static_per_ofe_slice_count {static_per_ofe_slice_count} must equal contributor_ofe_count {contributor_ofe_count} during M-E1 shadow-state publication"
        )));
    }
    if !publication_area_m2.is_finite() || publication_area_m2 <= 0.0 {
        return Err(wb13_simout_failure(format!(
            "publication_area_m2 must be finite and > 0.0, observed {publication_area_m2}"
        )));
    }
    Ok(())
}

fn validate_per_ofe_wb13_publication_rows(
    rows: &[SimulationOwnedWb13Row],
    contributor_ofe_count: usize,
    summary: &PerOfeInternalWb13RunSummary,
) -> Result<(), HillslopeCliError> {
    if rows.len() != summary.record_count {
        return Err(wb13_simout_failure(format!(
            "per-OFE WB13 publication row_count {} must equal internal record_count {}",
            rows.len(),
            summary.record_count
        )));
    }
    if rows.len() != summary.expected_record_count {
        return Err(wb13_simout_failure(format!(
            "per-OFE WB13 publication row_count {} must equal expected_record_count {}",
            rows.len(),
            summary.expected_record_count
        )));
    }
    let expected_published_rows =
        summary
            .day_count
            .checked_mul(contributor_ofe_count)
            .ok_or_else(|| {
                wb13_simout_failure("per-OFE WB13 publication expected row count overflowed usize")
            })?;
    if rows.len() != expected_published_rows {
        return Err(wb13_simout_failure(format!(
            "per-OFE WB13 publication row_count {} must equal day_count {} * contributor_ofe_count {}",
            rows.len(),
            summary.day_count,
            contributor_ofe_count
        )));
    }
    validate_per_ofe_wb13_publication_chunks(rows, contributor_ofe_count)
}

fn validate_per_ofe_wb13_publication_chunks(
    rows: &[SimulationOwnedWb13Row],
    contributor_ofe_count: usize,
) -> Result<(), HillslopeCliError> {
    for (chunk_index, chunk) in rows.chunks(contributor_ofe_count).enumerate() {
        if chunk.len() != contributor_ofe_count {
            return Err(wb13_simout_failure(format!(
                "per-OFE WB13 publication chunk {chunk_index} has {} rows; expected {contributor_ofe_count}",
                chunk.len()
            )));
        }
        let sim_day_index = chunk[0].sim_day_index;
        for (ofe_index, row) in chunk.iter().enumerate() {
            let expected_ofe = u16::try_from(ofe_index + 1).map_err(|_| {
                wb13_simout_failure(format!(
                    "per-OFE WB13 expected OFE {} is outside u16 domain",
                    ofe_index + 1
                ))
            })?;
            if row.sim_day_index != sim_day_index {
                return Err(wb13_simout_failure(format!(
                    "per-OFE WB13 publication chunk {chunk_index} mixes sim_day_index {} and {}",
                    sim_day_index, row.sim_day_index
                )));
            }
            if row.wb13_row.ofe != expected_ofe {
                return Err(wb13_simout_failure(format!(
                    "per-OFE WB13 publication chunk {chunk_index} expected OFE {expected_ofe}, observed {}",
                    row.wb13_row.ofe
                )));
            }
        }
    }
    Ok(())
}

fn validate_aggregate_wb13_publication_rows(
    rows: &[SimulationOwnedWb13Row],
) -> Result<(), HillslopeCliError> {
    if rows.iter().any(|row| row.wb13_row.ofe != 1) {
        return Err(wb13_simout_failure(
            "MOFE04 canonicalized publication policy requires WB13 OFE key = 1 for all rows",
        ));
    }
    Ok(())
}

fn wb13_publication_sim_day_index_monotonic(
    rows: &[SimulationOwnedWb13Row],
    publishes_per_ofe_records: bool,
) -> bool {
    if publishes_per_ofe_records {
        rows.windows(2).all(|window| {
            window[1].sim_day_index > window[0].sim_day_index
                || (window[1].sim_day_index == window[0].sim_day_index
                    && window[1].wb13_row.ofe > window[0].wb13_row.ofe)
        })
    } else {
        rows.windows(2)
            .all(|window| window[1].sim_day_index > window[0].sim_day_index)
    }
}

fn wb13_per_ofe_state_policy(
    publishes_per_ofe_records: bool,
    per_ofe_dynamic_state_executed: bool,
    per_ofe_internal_wb13_summary: Option<&PerOfeInternalWb13RunSummary>,
) -> &'static str {
    if publishes_per_ofe_records {
        MF_PER_OFE_STATE_POLICY
    } else if per_ofe_internal_wb13_summary.is_some() {
        ME4_PER_OFE_STATE_POLICY
    } else if per_ofe_dynamic_state_executed {
        ME3_PER_OFE_STATE_POLICY
    } else {
        ME1_PER_OFE_STATE_POLICY
    }
}

fn wb13_identity_status(
    publishes_per_ofe_records: bool,
    per_ofe_dynamic_state_executed: bool,
    per_ofe_internal_wb13_summary: Option<&PerOfeInternalWb13RunSummary>,
) -> &'static str {
    if publishes_per_ofe_records {
        MF_IDENTITY_STATUS
    } else if per_ofe_internal_wb13_summary.is_some() {
        ME4_IDENTITY_STATUS
    } else if per_ofe_dynamic_state_executed {
        ME3_IDENTITY_STATUS
    } else {
        ME1_IDENTITY_STATUS
    }
}

pub(super) fn build_wb13_publication_provenance(
    rows: &[SimulationOwnedWb13Row],
    contributor_ofe_count: usize,
    static_per_ofe_slice_count: usize,
    publication_area_m2: f64,
    per_ofe_dynamic_state_executed: bool,
    per_ofe_internal_wb13_summary: Option<&PerOfeInternalWb13RunSummary>,
) -> Result<HillslopeWb13PublicationProvenance, HillslopeCliError> {
    validate_wb13_publication_common_inputs(
        rows,
        contributor_ofe_count,
        static_per_ofe_slice_count,
        publication_area_m2,
    )?;
    let first_row = &rows[0];
    let last_row = &rows[rows.len() - 1];
    let publishes_per_ofe_records =
        contributor_ofe_count > 1 && per_ofe_internal_wb13_summary.is_some();
    if publishes_per_ofe_records {
        let Some(summary) = per_ofe_internal_wb13_summary else {
            return Err(wb13_simout_failure(
                "per-OFE WB13 publication requires internal WB13 summary",
            ));
        };
        validate_per_ofe_wb13_publication_rows(rows, contributor_ofe_count, summary)?;
    } else {
        validate_aggregate_wb13_publication_rows(rows)?;
    }
    let sim_day_index_monotonic =
        wb13_publication_sim_day_index_monotonic(rows, publishes_per_ofe_records);
    let per_ofe_record_count =
        per_ofe_internal_wb13_summary.map_or(0usize, |summary| summary.record_count);
    let per_ofe_state_policy = wb13_per_ofe_state_policy(
        publishes_per_ofe_records,
        per_ofe_dynamic_state_executed,
        per_ofe_internal_wb13_summary,
    );
    let identity_status = wb13_identity_status(
        publishes_per_ofe_records,
        per_ofe_dynamic_state_executed,
        per_ofe_internal_wb13_summary,
    );
    let per_ofe_internal_day_count =
        per_ofe_internal_wb13_summary.map_or(0usize, |summary| summary.day_count);
    let per_ofe_expected_record_count =
        per_ofe_internal_wb13_summary.map_or(0usize, |summary| summary.expected_record_count);
    let transfer_identity_max_abs_mm = per_ofe_internal_wb13_summary
        .map_or(0.0, |summary| summary.transfer_identity_max_abs_mm);
    let per_element_identity_max_abs_mm = per_ofe_internal_wb13_summary
        .map_or(0.0, |summary| summary.per_element_identity_max_abs_mm);
    let aggregate_transfer_cancellation_max_abs_mm = per_ofe_internal_wb13_summary
        .map_or(0.0, |summary| summary.aggregate_transfer_cancellation_max_abs_mm);

    Ok(HillslopeWb13PublicationProvenance {
        source: WB13_PUBLICATION_SOURCE_SIMULATION_OWNED.to_string(),
        projection_fallback_used: false,
        guard_id: SIMOUT_GUARD_ID.to_string(),
        replay_candidate_surfaces: vec![
            WB13_REPLAY_CANDIDATE_SURFACE_WAT.to_string(),
            WB13_REPLAY_CANDIDATE_SURFACE_PASS.to_string(),
        ],
        publication_ofe_policy: if publishes_per_ofe_records {
            MF_PUBLICATION_OFE_POLICY
        } else {
            MOFE04_PUBLICATION_OFE_POLICY
        }
        .to_string(),
        contributor_ofe_count,
        static_per_ofe_slice_count,
        per_ofe_state_policy: per_ofe_state_policy.to_string(),
        per_ofe_dynamic_water_balance_state: per_ofe_dynamic_state_executed,
        per_ofe_dynamic_wb_state: per_ofe_dynamic_state_executed,
        per_ofe_record_count,
        transfer_identity_status: identity_status.to_string(),
        per_element_identity_status: identity_status.to_string(),
        aggregate_identity_status: identity_status.to_string(),
        area_policy: MOFE04_PUBLICATION_AREA_POLICY.to_string(),
        storage_lineage_policy: if publishes_per_ofe_records {
            MF_STORAGE_LINEAGE_POLICY
        } else {
            HPHYS0255_STORAGE_LINEAGE_POLICY
        }
        .to_string(),
        per_ofe_internal_day_count,
        per_ofe_expected_record_count,
        transfer_identity_max_abs_mm,
        per_element_identity_max_abs_mm,
        aggregate_transfer_cancellation_max_abs_mm,
        publication_area_m2,
        row_count: rows.len(),
        sim_day_index_monotonic,
        first_row_key: wb13_row_key_provenance(first_row),
        last_row_key: wb13_row_key_provenance(last_row),
    })
}

pub(super) fn build_mofe_hourly_carry_provenance(
    runtime_surface: &HillslopeWritebackSurface,
    contributor_ofe_count: usize,
) -> Result<HillslopeMofeHourlyCarryProvenance, HillslopeCliError> {
    let active = contributor_ofe_count > 1;
    let upstream_carry_total_m = if active {
        sum_mofe_hourly_carry_array(
            runtime_surface,
            MOFE_HOURLY_UPSTREAM_SATURATION_RUNOFF_ROOT,
            true,
        )? + sum_mofe_hourly_carry_array(
            runtime_surface,
            MOFE_HOURLY_UPSTREAM_LATERAL_RUNOFF_ROOT,
            true,
        )?
    } else {
        0.0
    };
    let current_carry_total_m = if active {
        sum_mofe_hourly_carry_array(
            runtime_surface,
            MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT,
            true,
        )? + sum_mofe_hourly_carry_array(
            runtime_surface,
            MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT,
            true,
        )?
    } else {
        0.0
    };

    Ok(HillslopeMofeHourlyCarryProvenance {
        policy: MOFE_HOURLY_CARRY_POLICY.to_string(),
        active,
        substep_count: MOFE_HOURLY_CARRY_ARRAY_COUNT,
        required_arrays: MOFE_HOURLY_REQUIRED_ARRAYS
            .iter()
            .map(|root| (*root).to_string())
            .collect(),
        upstream_carry_total_m,
        current_carry_total_m,
    })
}

pub(super) fn sum_mofe_hourly_carry_array(
    runtime_surface: &HillslopeWritebackSurface,
    root: &str,
    required: bool,
) -> Result<f64, HillslopeCliError> {
    let mut total = 0.0_f64;
    for hour in 1..=MOFE_HOURLY_CARRY_ARRAY_COUNT {
        let symbol = mofe_hourly_carry_hour_symbol(root, hour);
        let Some(value) = runtime_surface_symbol_value(runtime_surface, &symbol) else {
            if required {
                return Err(mofe_hourly_carry_failure(format!(
                    "missing required runtime symbol {symbol}"
                )));
            }
            continue;
        };
        require_mofe_hourly_carry_non_negative(value, &symbol)?;
        total += value;
    }
    require_mofe_hourly_carry_non_negative(total, root)?;
    Ok(total)
}

pub(super) fn scheduler_outcome_class_as_str(outcome_class: SchedulerOutcomeClass) -> &'static str {
    match outcome_class {
        SchedulerOutcomeClass::Completed => "completed",
        SchedulerOutcomeClass::TopologyPreconditionFailed => "topology_precondition_failed",
        SchedulerOutcomeClass::PhaseFailure => "phase_failure",
        SchedulerOutcomeClass::SchedulerInvariantFailure => "scheduler_invariant_failure",
    }
}

pub(super) fn wb13_row_key_provenance(row: &SimulationOwnedWb13Row) -> HillslopeWb13RowKeyProvenance {
    HillslopeWb13RowKeyProvenance {
        year: row.wb13_row.year,
        julian_day: row.wb13_row.julian_day,
        ofe: row.wb13_row.ofe,
        sim_day_index: row.sim_day_index,
    }
}

pub(super) const HBP_MAGIC: &[u8; 8] = b"WFPHBP01";
pub(super) const HBP_FOOTER_MAGIC: &[u8; 8] = b"ENDHBP01";
pub(super) const HBP_SUPPORTED_MAJOR_V1: u16 = 1;
pub(super) const HBP_DIM_SCALAR: u8 = 0;
pub(super) const HBP_DIM_NOFE: u8 = 1;
pub(super) const HBP_DIM_NOFE_LAYERS: u8 = 2;
pub(super) const HBP_DEFAULT_CALENDAR_YEAR: i32 = 2004;
pub(super) const HBP_DEFAULT_PARTICLE_DIAMETER_M: f64 = 0.001;
pub(super) const HBP_SCALE_INV_I64: f64 = 1.0e9;
pub(super) const HBP_I64_MIN_F64: f64 = -9_223_372_036_854_775_808.0;
pub(super) const HBP_I64_MAX_F64: f64 = 9_223_372_036_854_775_807.0;
pub(super) const HBP_REQUIRED_STATE_IDS: &[u16] = &[
    1, 2, 3, 4, 5, 6, 7, 100, 101, 102, 103, 104, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209,
    210, 300, 900, 901,
];
