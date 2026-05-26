/// Build a hillslope runtime surface from parsed snow-control input.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when snow controls are non-finite or
/// outside required CLIM05 domains.
pub fn build_hillslope_runtime_surface_from_snow(
    snow: &SnowParseOutput,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    let mut surface = HillslopeWritebackSurface::default();
    seed_hillslope_runtime_surface_from_snow(&mut surface, snow)?;
    Ok(surface)
}

/// Build a hillslope runtime surface from parsed frost-control input.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when frost controls are non-finite or
/// outside required CLIM06 domains.
pub fn build_hillslope_runtime_surface_from_frost(
    frost: &FrostParseOutput,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    let mut surface = HillslopeWritebackSurface::default();
    seed_hillslope_runtime_surface_from_frost(&mut surface, frost)?;
    Ok(surface)
}

/// Build a hillslope runtime surface from parsed depletion-scheduled irrigation
/// sidecar input.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when required depletion scheduling
/// surfaces are malformed, non-finite, or out-of-domain.
pub fn build_hillslope_runtime_surface_from_irrigation_depletion(
    depletion: &IrrigationDepletionFile,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    let mut surface = HillslopeWritebackSurface::default();
    seed_hillslope_runtime_surface_from_irrigation_depletion(&mut surface, depletion)?;
    Ok(surface)
}

/// Seed parsed depletion-scheduled irrigation symbols into an existing
/// hillslope runtime surface.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when required depletion scheduling
/// surfaces are malformed, non-finite, or out-of-domain.
#[allow(clippy::too_many_lines)]
pub fn seed_hillslope_runtime_surface_from_irrigation_depletion(
    runtime_surface: &mut HillslopeWritebackSurface,
    depletion: &IrrigationDepletionFile,
) -> Result<(), HillslopeRuntimeInputError> {
    let system_type = match depletion.system_type {
        openwepp_input_contract::parsers::irrigation_depletion::IrrigationSystemType::Sprinkler => {
            1.0
        }
        openwepp_input_contract::parsers::irrigation_depletion::IrrigationSystemType::Furrow => 2.0,
    };

    let state_surface = &mut runtime_surface.state_surface;
    state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.enabled"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.element_count"),
        BoundaryValue::scalar(irrigation_count_to_f64(
            "irrigation.depletion.element_count",
            depletion.element_count,
        )?),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.system_type"),
        BoundaryValue::scalar(system_type),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.schedule_type"),
        BoundaryValue::scalar(f64::from(depletion.schedule_type)),
    );

    let min_depth =
        validate_irrigation_finite("irrigation.depletion.min_depth_m", depletion.min_depth_m)?;
    if min_depth < 0.0 {
        return Err(
            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                field: "irrigation.depletion.min_depth_m",
                value: min_depth,
                allowed: ">= 0.0",
            },
        );
    }
    state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.min_depth_m"),
        BoundaryValue::scalar(min_depth),
    );
    if let Some(max_depth_m) = depletion.max_depth_m {
        let max_depth =
            validate_irrigation_finite("irrigation.depletion.max_depth_m", max_depth_m)?;
        if max_depth < min_depth {
            return Err(
                HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                    field: "irrigation.depletion.max_depth_m",
                    value: max_depth,
                    allowed: ">= irrigation.depletion.min_depth_m",
                },
            );
        }
        state_surface.insert(
            BoundarySymbol::from("irrigation.depletion.max_depth_m"),
            BoundaryValue::scalar(max_depth),
        );
    }

    state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.period_count"),
        BoundaryValue::scalar(irrigation_count_to_f64(
            "irrigation.depletion.period_count",
            depletion.periods.len(),
        )?),
    );

    for (period_position, period) in depletion.periods.iter().enumerate() {
        let period_index = period_position + 1;
        if period.element_id == 0 {
            return Err(
                HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                    field: "irrigation.depletion.period_####.element_id",
                    value: 0.0,
                    allowed: ">= 1",
                },
            );
        }
        state_surface.insert(
            irrigation_depletion_period_symbol(period_index, "element_id"),
            BoundaryValue::scalar(irrigation_count_to_f64(
                "irrigation.depletion.period_####.element_id",
                period.element_id,
            )?),
        );

        let depletion_trigger = validate_irrigation_finite(
            "irrigation.depletion.period_####.depletion_trigger_ratio",
            period.depletion_trigger_ratio,
        )?;
        if !(0.0..=1.0).contains(&depletion_trigger) {
            return Err(
                HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                    field: "irrigation.depletion.period_####.depletion_trigger_ratio",
                    value: depletion_trigger,
                    allowed: "[0.0,1.0]",
                },
            );
        }
        state_surface.insert(
            irrigation_depletion_period_symbol(period_index, "depletion_trigger_ratio"),
            BoundaryValue::scalar(depletion_trigger),
        );

        for (field, value) in [
            ("start_doy", period.start_doy),
            ("start_year", period.start_year),
            ("end_doy", period.end_doy),
            ("end_year", period.end_year),
        ] {
            if value < 0 {
                return Err(
                    HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                        field: "irrigation.depletion.period_####.date",
                        value: f64::from(value),
                        allowed: ">= 0",
                    },
                );
            }
            state_surface.insert(
                irrigation_depletion_period_symbol(period_index, field),
                BoundaryValue::scalar(f64::from(value)),
            );
        }

        match &period.data {
            IrrigationPeriodData::Sprinkler(record) => {
                let rate = validate_irrigation_finite(
                    "irrigation.depletion.period_####.sprinkler_rate_m_per_s",
                    record.rate_m_per_s,
                )?;
                if rate <= 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.depletion.period_####.sprinkler_rate_m_per_s",
                            value: rate,
                            allowed: "> 0.0",
                        },
                    );
                }
                let depth_ratio = validate_irrigation_finite(
                    "irrigation.depletion.period_####.sprinkler_depth_ratio",
                    record.depth_ratio,
                )?;
                if depth_ratio < 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.depletion.period_####.sprinkler_depth_ratio",
                            value: depth_ratio,
                            allowed: ">= 0.0",
                        },
                    );
                }
                let nozzle = validate_irrigation_finite(
                    "irrigation.depletion.period_####.sprinkler_nozzle_factor",
                    record.nozzle_factor,
                )?;
                if nozzle <= 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.depletion.period_####.sprinkler_nozzle_factor",
                            value: nozzle,
                            allowed: "> 0.0",
                        },
                    );
                }

                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "sprinkler_rate_m_per_s"),
                    BoundaryValue::scalar(rate),
                );
                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "sprinkler_depth_ratio"),
                    BoundaryValue::scalar(depth_ratio),
                );
                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "sprinkler_nozzle_factor"),
                    BoundaryValue::scalar(nozzle),
                );
            }
            IrrigationPeriodData::Furrow(record) => {
                if record.end_element_id == 0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.depletion.period_####.furrow_end_element_id",
                            value: 0.0,
                            allowed: ">= 1",
                        },
                    );
                }
                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "furrow_end_element_id"),
                    BoundaryValue::scalar(irrigation_count_to_f64(
                        "irrigation.depletion.period_####.furrow_end_element_id",
                        record.end_element_id,
                    )?),
                );

                let supply_rate = validate_irrigation_finite(
                    "irrigation.depletion.period_####.furrow_supply_rate_m3_per_s",
                    record.supply_rate_m3_per_s,
                )?;
                if supply_rate <= 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.depletion.period_####.furrow_supply_rate_m3_per_s",
                            value: supply_rate,
                            allowed: "> 0.0",
                        },
                    );
                }
                let supply_duration = validate_irrigation_finite(
                    "irrigation.depletion.period_####.furrow_supply_duration_s",
                    record.supply_duration_s,
                )?;
                if supply_duration <= 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.depletion.period_####.furrow_supply_duration_s",
                            value: supply_duration,
                            allowed: "> 0.0",
                        },
                    );
                }
                let fill_ratio = validate_irrigation_finite(
                    "irrigation.depletion.period_####.furrow_fill_ratio",
                    record.fill_ratio,
                )?;
                if fill_ratio < 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.depletion.period_####.furrow_fill_ratio",
                            value: fill_ratio,
                            allowed: ">= 0.0",
                        },
                    );
                }

                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "furrow_supply_rate_m3_per_s"),
                    BoundaryValue::scalar(supply_rate),
                );
                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "furrow_supply_duration_s"),
                    BoundaryValue::scalar(supply_duration),
                );
                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "furrow_surge_code"),
                    BoundaryValue::scalar(f64::from(record.surge_code)),
                );
                state_surface.insert(
                    irrigation_depletion_period_symbol(period_index, "furrow_fill_ratio"),
                    BoundaryValue::scalar(fill_ratio),
                );
            }
        }
    }

    Ok(())
}

/// Build a hillslope runtime surface from parsed fixed-date irrigation sidecar
/// input.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when required fixed-date scheduling
/// surfaces are malformed, non-finite, or out-of-domain.
pub fn build_hillslope_runtime_surface_from_irrigation_fixeddate(
    fixeddate: &FixedDateIrrigationFile,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    let mut surface = HillslopeWritebackSurface::default();
    seed_hillslope_runtime_surface_from_irrigation_fixeddate(&mut surface, fixeddate)?;
    Ok(surface)
}

/// Seed parsed fixed-date irrigation symbols into an existing hillslope runtime
/// surface.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when required fixed-date scheduling
/// surfaces are malformed, non-finite, or out-of-domain.
#[allow(clippy::too_many_lines)]
pub fn seed_hillslope_runtime_surface_from_irrigation_fixeddate(
    runtime_surface: &mut HillslopeWritebackSurface,
    fixeddate: &FixedDateIrrigationFile,
) -> Result<(), HillslopeRuntimeInputError> {
    let datver = validate_irrigation_finite("irrigation.fixeddate.datver", fixeddate.datver)?;
    if datver <= 0.0 {
        return Err(
            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                field: "irrigation.fixeddate.datver",
                value: datver,
                allowed: "> 0.0",
            },
        );
    }

    if fixeddate.itemp == 0 {
        return Err(
            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                field: "irrigation.fixeddate.ofe_count",
                value: 0.0,
                allowed: ">= 1",
            },
        );
    }
    if fixeddate.initial_records.len() != fixeddate.itemp {
        let observed = irrigation_count_to_f64(
            "irrigation.fixeddate.initial_records",
            fixeddate.initial_records.len(),
        )?;
        return Err(
            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                field: "irrigation.fixeddate.initial_records",
                value: observed,
                allowed: "== irrigation.fixeddate.ofe_count",
            },
        );
    }

    let state_surface = &mut runtime_surface.state_surface;
    state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.enabled"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.datver"),
        BoundaryValue::scalar(datver),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.ofe_count"),
        BoundaryValue::scalar(irrigation_count_to_f64(
            "irrigation.fixeddate.ofe_count",
            fixeddate.itemp,
        )?),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.system_type"),
        BoundaryValue::scalar(irrigation_count_to_f64(
            "irrigation.fixeddate.system_type",
            fixeddate.jtemp,
        )?),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.schedule_type"),
        BoundaryValue::scalar(irrigation_count_to_f64(
            "irrigation.fixeddate.schedule_type",
            fixeddate.ktemp,
        )?),
    );
    state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_count"),
        BoundaryValue::scalar(irrigation_count_to_f64(
            "irrigation.fixeddate.event_count",
            fixeddate.events.len(),
        )?),
    );

    let mut expected_ofe = 1usize;
    let mut active_dates = fixeddate.initial_records.clone();
    for (event_position, event) in fixeddate.events.iter().enumerate() {
        let event_index = event_position + 1;
        let expected_ofe_f64 =
            irrigation_count_to_f64("irrigation.fixeddate.event_####.ofe_id", expected_ofe)?;
        let schedule = active_dates.get(expected_ofe - 1).ok_or(
            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                field: "irrigation.fixeddate.event_####.ofe_id",
                value: expected_ofe_f64,
                allowed: "1..=irrigation.fixeddate.ofe_count",
            },
        )?;

        state_surface.insert(
            irrigation_fixeddate_event_symbol(event_index, "ofe_id"),
            BoundaryValue::scalar(irrigation_count_to_f64(
                "irrigation.fixeddate.event_####.ofe_id",
                schedule.ofeflg,
            )?),
        );
        state_surface.insert(
            irrigation_fixeddate_event_symbol(event_index, "day"),
            BoundaryValue::scalar(irrigation_count_to_f64(
                "irrigation.fixeddate.event_####.day",
                schedule.irday,
            )?),
        );
        state_surface.insert(
            irrigation_fixeddate_event_symbol(event_index, "year"),
            BoundaryValue::scalar(irrigation_count_to_f64(
                "irrigation.fixeddate.event_####.year",
                schedule.iryr,
            )?),
        );
        state_surface.insert(
            irrigation_fixeddate_event_symbol(event_index, "schedule_termination_flag"),
            BoundaryValue::scalar(if schedule.schedule_termination_flag {
                1.0
            } else {
                0.0
            }),
        );

        match event {
            FixedDateEvent::Sprinkler(sprinkler) => {
                let rate = validate_irrigation_finite(
                    "irrigation.fixeddate.event_####.sprinkler_rate_m_per_s",
                    sprinkler.irint,
                )?;
                if rate <= 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.fixeddate.event_####.sprinkler_rate_m_per_s",
                            value: rate,
                            allowed: "> 0.0",
                        },
                    );
                }
                let depth = validate_irrigation_finite(
                    "irrigation.fixeddate.event_####.sprinkler_depth_m",
                    sprinkler.irdept,
                )?;
                if depth < 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.fixeddate.event_####.sprinkler_depth_m",
                            value: depth,
                            allowed: ">= 0.0",
                        },
                    );
                }
                let nozzle = validate_irrigation_finite(
                    "irrigation.fixeddate.event_####.sprinkler_nozzle_factor",
                    sprinkler.nozzle,
                )?;
                if nozzle <= 0.0 {
                    return Err(
                        HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                            field: "irrigation.fixeddate.event_####.sprinkler_nozzle_factor",
                            value: nozzle,
                            allowed: "> 0.0",
                        },
                    );
                }

                state_surface.insert(
                    irrigation_fixeddate_event_symbol(event_index, "sprinkler_rate_m_per_s"),
                    BoundaryValue::scalar(rate),
                );
                state_surface.insert(
                    irrigation_fixeddate_event_symbol(event_index, "sprinkler_depth_m"),
                    BoundaryValue::scalar(depth),
                );
                state_surface.insert(
                    irrigation_fixeddate_event_symbol(event_index, "sprinkler_nozzle_factor"),
                    BoundaryValue::scalar(nozzle),
                );
            }
            FixedDateEvent::Furrow(furrow) => {
                let surges = irrigation_count_to_f64(
                    "irrigation.fixeddate.event_####.furrow_surges",
                    furrow.surges,
                )?;
                let mut total_duration = 0.0_f64;
                let mut total_volume = 0.0_f64;
                for surge in &furrow.rows {
                    let supply_rate = validate_irrigation_finite(
                        "irrigation.fixeddate.event_####.furrow_supply_rate_m3_per_s",
                        surge.qspply,
                    )?;
                    if supply_rate <= 0.0 {
                        return Err(
                            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                                field: "irrigation.fixeddate.event_####.furrow_supply_rate_m3_per_s",
                                value: supply_rate,
                                allowed: "> 0.0",
                            },
                        );
                    }
                    let start_s = validate_irrigation_finite(
                        "irrigation.fixeddate.event_####.furrow_start_s",
                        surge.tstart,
                    )?;
                    let end_s = validate_irrigation_finite(
                        "irrigation.fixeddate.event_####.furrow_end_s",
                        surge.tend,
                    )?;
                    if end_s < start_s {
                        return Err(
                            HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                                field: "irrigation.fixeddate.event_####.furrow_end_s",
                                value: end_s,
                                allowed: ">= irrigation.fixeddate.event_####.furrow_start_s",
                            },
                        );
                    }
                    if let Some(tdepl) = surge.tdepl {
                        let depletion_tail = validate_irrigation_finite(
                            "irrigation.fixeddate.event_####.furrow_tdepl_s",
                            tdepl,
                        )?;
                        if depletion_tail < 0.0 {
                            return Err(
                                HillslopeRuntimeInputError::IrrigationScheduleFieldOutOfDomain {
                                    field: "irrigation.fixeddate.event_####.furrow_tdepl_s",
                                    value: depletion_tail,
                                    allowed: ">= 0.0",
                                },
                            );
                        }
                        total_duration += depletion_tail;
                    }
                    let active_duration = end_s - start_s;
                    total_duration += active_duration;
                    total_volume += supply_rate * active_duration;
                }

                state_surface.insert(
                    irrigation_fixeddate_event_symbol(event_index, "furrow_surges"),
                    BoundaryValue::scalar(surges),
                );
                state_surface.insert(
                    irrigation_fixeddate_event_symbol(event_index, "furrow_total_duration_s"),
                    BoundaryValue::scalar(total_duration),
                );
                state_surface.insert(
                    irrigation_fixeddate_event_symbol(event_index, "furrow_total_supply_volume_m3"),
                    BoundaryValue::scalar(total_volume),
                );
            }
        }

        match event {
            FixedDateEvent::Sprinkler(event) => {
                active_dates[expected_ofe - 1] = event.next_record.clone();
            }
            FixedDateEvent::Furrow(event) => {
                active_dates[expected_ofe - 1] = event.next_record.clone();
            }
        }
        expected_ofe += 1;
        if expected_ofe > fixeddate.itemp {
            expected_ofe = 1;
        }
    }

    Ok(())
}

/// Seed parsed snow-control runtime symbols into an existing hillslope runtime
/// surface.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when parsed snow controls are
/// non-finite or violate required CLIM05 domains.
pub fn seed_hillslope_runtime_surface_from_snow(
    runtime_surface: &mut HillslopeWritebackSurface,
    snow: &SnowParseOutput,
) -> Result<(), HillslopeRuntimeInputError> {
    let rst = validate_snow_control_finite("snow.options.rst", snow.rst)?;
    let newsnw = validate_snow_control_finite("snow.options.newsnw", snow.newsnw)?;
    let ssd = validate_snow_control_finite("snow.options.ssd", snow.ssd)?;

    if newsnw <= 0.0 {
        return Err(HillslopeRuntimeInputError::SnowControlOutOfDomain {
            field: "snow.options.newsnw",
            value: newsnw,
            allowed: "> 0.0",
        });
    }
    if ssd <= 0.0 {
        return Err(HillslopeRuntimeInputError::SnowControlOutOfDomain {
            field: "snow.options.ssd",
            value: ssd,
            allowed: "> 0.0",
        });
    }
    if newsnw > ssd {
        return Err(HillslopeRuntimeInputError::SnowControlOutOfDomain {
            field: "snow.options.newsnw",
            value: newsnw,
            allowed: "<= snow.options.ssd",
        });
    }

    let state_surface = &mut runtime_surface.state_surface;
    state_surface.insert(
        BoundarySymbol::from("snow.options.rst"),
        BoundaryValue::scalar(rst),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.options.newsnw"),
        BoundaryValue::scalar(newsnw),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.options.ssd"),
        BoundaryValue::scalar(ssd),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.options.snow_file_present"),
        BoundaryValue::scalar(if snow.sidecar_present { 1.0 } else { 0.0 }),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_swe"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_depth_m"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_density_kg_m3"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_settle_day_count"),
        BoundaryValue::scalar(0.0),
    );

    Ok(())
}

/// Seed parsed frost-control runtime symbols into an existing hillslope runtime
/// surface.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when parsed frost controls are
/// non-finite or violate required CLIM06 domains.
#[allow(clippy::too_many_lines)]
pub fn seed_hillslope_runtime_surface_from_frost(
    runtime_surface: &mut HillslopeWritebackSurface,
    frost: &FrostParseOutput,
) -> Result<(), HillslopeRuntimeInputError> {
    let wint_red = f64::from(frost.wint_red);
    let fine_top = f64::from(frost.fine_top);
    let fine_bot = f64::from(frost.fine_bot);
    let ksnowf = validate_frost_control_finite("frost.options.ksnowf", frost.ksnowf)?;
    let kresf = validate_frost_control_finite("frost.options.kresf", frost.kresf)?;
    let ksoilf = validate_frost_control_finite("frost.options.ksoilf", frost.ksoilf)?;
    let kfactor1 = validate_frost_control_finite("frost.options.kfactor1", frost.kfactor1)?;
    let kfactor2 = validate_frost_control_finite("frost.options.kfactor2", frost.kfactor2)?;
    let kfactor3 = validate_frost_control_finite("frost.options.kfactor3", frost.kfactor3)?;

    if frost.wint_red != 0 && frost.wint_red != 1 {
        return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
            field: "frost.options.wintRed",
            value: wint_red,
            allowed: "{0,1}",
        });
    }
    if !(1..=10).contains(&frost.fine_top) {
        return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
            field: "frost.options.fineTop",
            value: fine_top,
            allowed: "integer [1,10]",
        });
    }
    if !(1..=10).contains(&frost.fine_bot) {
        return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
            field: "frost.options.fineBot",
            value: fine_bot,
            allowed: "integer [1,10]",
        });
    }
    for (field, value) in [
        ("frost.options.ksnowf", ksnowf),
        ("frost.options.kresf", kresf),
        ("frost.options.ksoilf", ksoilf),
    ] {
        if !(0.1..=10.0).contains(&value) {
            return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
                field,
                value,
                allowed: "real [0.1,10.0]",
            });
        }
    }
    for (field, value) in [
        ("frost.options.kfactor1", kfactor1),
        ("frost.options.kfactor2", kfactor2),
        ("frost.options.kfactor3", kfactor3),
    ] {
        if !(value > 0.0 && value <= 1.0) {
            return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
                field,
                value,
                allowed: "real (0.0,1.0]",
            });
        }
    }

    let state_surface = &mut runtime_surface.state_surface;
    state_surface.insert(
        BoundarySymbol::from("frost.options.wintRed"),
        BoundaryValue::scalar(wint_red),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.fineTop"),
        BoundaryValue::scalar(fine_top),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.fineBot"),
        BoundaryValue::scalar(fine_bot),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.ksnowf"),
        BoundaryValue::scalar(ksnowf),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kresf"),
        BoundaryValue::scalar(kresf),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.ksoilf"),
        BoundaryValue::scalar(ksoilf),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor1"),
        BoundaryValue::scalar(kfactor1),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor2"),
        BoundaryValue::scalar(kfactor2),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor3"),
        BoundaryValue::scalar(kfactor3),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.frost_file_present"),
        BoundaryValue::scalar(if frost.frost_file_present { 1.0 } else { 0.0 }),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_dfrost"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_dthaw"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_nft"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_ws_frz"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_infcap_frz"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_frdp_m"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_thdp_m"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_tfrdp_m"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_tthawd_m"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_fgthwd_flag"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_total_fine_layer_count"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_kftill_w_m_k"),
        BoundaryValue::scalar(1.75),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_kfutil_w_m_k"),
        BoundaryValue::scalar(2.1),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_kres_w_m_k"),
        BoundaryValue::scalar(0.05 * kresf),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_residue_depth_m"),
        BoundaryValue::scalar(0.0),
    );

    Ok(())
}
