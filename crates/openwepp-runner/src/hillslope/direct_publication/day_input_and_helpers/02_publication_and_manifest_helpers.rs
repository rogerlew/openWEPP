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
    let lateral_conductivity_m_s =
        direct_publication_lateral_conductivity_m_s(runtime_surface, layer_index, conductivity_m_s)?;
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
            lateral_conductivity_m_s,
        },
    ))
}

fn direct_publication_lateral_conductivity_m_s(
    runtime_surface: &HillslopeWritebackSurface,
    layer_index: usize,
    vertical_conductivity_m_s: f64,
) -> Result<f64, HillslopeCliError> {
    let lateral_symbol = format!("wb19_lateral_ssh_{layer_index:04}");
    if let Some(value) = runtime_surface_symbol_value(runtime_surface, lateral_symbol.as_str()) {
        if value.is_finite() && value > 0.0 {
            return Ok(value);
        }
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} {lateral_symbol} must be finite and > 0.0 for direct hourly WB19 lateral conductivity, observed {value}"
            ),
        });
    }

    let lane_substeps =
        runtime_surface_symbol_value(runtime_surface, "wb19_lateral_drain_lane_substeps")
            .unwrap_or(1.0);
    let solwpv = runtime_surface_symbol_value(runtime_surface, "solwpv").unwrap_or(0.0);
    if lane_substeps > 1.0 && solwpv >= 7778.0 {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_publication_frame",
            detail: format!(
                "{SIMOUT_GUARD_ID} direct hourly WB19 lateral conductivity requires {lateral_symbol}; substituting wb18_perc_ssc_{layer_index:04} violates INV-SUBHYD-027"
            ),
        });
    }
    Ok(vertical_conductivity_m_s)
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
    direct_publication_parse_enabled_flag(symbol, value)
}

fn direct_publication_optional_enabled_flag(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &'static str,
) -> Result<Option<bool>, HillslopeCliError> {
    runtime_surface_symbol_value(runtime_surface, symbol)
        .map(|value| direct_publication_parse_enabled_flag(symbol, value))
        .transpose()
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

fn direct_publication_required_positive_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    symbol: &'static str,
) -> Result<f64, HillslopeCliError> {
    let value = require_runtime_surface_scalar(runtime_surface, symbol)?;
    if value.is_finite() && value > 0.0 {
        return Ok(value);
    }
    Err(HillslopeCliError::RuntimeSurfaceFailure {
        surface: "direct_publication_frame",
        detail: format!(
            "{SIMOUT_GUARD_ID} {symbol} must be finite and > 0.0 for direct publication, observed {value}"
        ),
    })
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
        build_direct_publication_mofe_hourly_carry_provenance(&facts)?,
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
) -> Result<HillslopeMofeHourlyCarryProvenance, HillslopeCliError> {
    let upstream_carry_total_m = if facts.publishes_per_ofe_records {
        sum_direct_publication_upstream_carry_m(facts.rows)?
    } else {
        0.0
    };
    let current_carry_total_m = upstream_carry_total_m;
    Ok(HillslopeMofeHourlyCarryProvenance {
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
        upstream_carry_total_m,
        current_carry_total_m,
    })
}

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
