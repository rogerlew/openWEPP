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
pub fn seed_hillslope_runtime_surface_from_irrigation_depletion(
    runtime_surface: &mut HillslopeWritebackSurface,
    depletion: &IrrigationDepletionFile,
) -> Result<(), HillslopeRuntimeInputError> {
    let state_surface = &mut runtime_surface.state_surface;
    let system_type = irrigation_depletion_system_type_value(depletion.system_type);
    seed_irrigation_depletion_header_symbols(state_surface, depletion, system_type)?;
    seed_irrigation_depletion_periods(state_surface, &depletion.periods)
}

fn irrigation_depletion_system_type_value(system_type: IrrigationSystemType) -> f64 {
    match system_type {
        IrrigationSystemType::Sprinkler => 1.0,
        IrrigationSystemType::Furrow => 2.0,
    }
}

fn seed_irrigation_depletion_header_symbols(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    depletion: &IrrigationDepletionFile,
    system_type: f64,
) -> Result<(), HillslopeRuntimeInputError> {
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

    Ok(())
}

fn seed_irrigation_depletion_periods(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    periods: &[IrrigationPeriodRecord],
) -> Result<(), HillslopeRuntimeInputError> {
    for (period_position, period) in periods.iter().enumerate() {
        seed_irrigation_depletion_period(state_surface, period_position + 1, period)?;
    }
    Ok(())
}

fn seed_irrigation_depletion_period(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    period_index: usize,
    period: &IrrigationPeriodRecord,
) -> Result<(), HillslopeRuntimeInputError> {
    seed_irrigation_depletion_period_header_symbols(state_surface, period_index, period)?;
    match &period.data {
        IrrigationPeriodData::Sprinkler(record) => {
            seed_irrigation_depletion_sprinkler_period(state_surface, period_index, record)
        }
        IrrigationPeriodData::Furrow(record) => {
            seed_irrigation_depletion_furrow_period(state_surface, period_index, record)
        }
    }
}

fn seed_irrigation_depletion_period_header_symbols(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    period_index: usize,
    period: &IrrigationPeriodRecord,
) -> Result<(), HillslopeRuntimeInputError> {
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

    seed_irrigation_depletion_trigger_symbol(state_surface, period_index, period)?;
    seed_irrigation_depletion_date_symbols(state_surface, period_index, period)
}

fn seed_irrigation_depletion_trigger_symbol(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    period_index: usize,
    period: &IrrigationPeriodRecord,
) -> Result<(), HillslopeRuntimeInputError> {
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
    Ok(())
}

fn seed_irrigation_depletion_date_symbols(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    period_index: usize,
    period: &IrrigationPeriodRecord,
) -> Result<(), HillslopeRuntimeInputError> {
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
    Ok(())
}

fn seed_irrigation_depletion_sprinkler_period(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    period_index: usize,
    record: &SprinklerPeriodData,
) -> Result<(), HillslopeRuntimeInputError> {
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
    Ok(())
}

fn seed_irrigation_depletion_furrow_period(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    period_index: usize,
    record: &FurrowPeriodData,
) -> Result<(), HillslopeRuntimeInputError> {
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
pub fn seed_hillslope_runtime_surface_from_irrigation_fixeddate(
    runtime_surface: &mut HillslopeWritebackSurface,
    fixeddate: &FixedDateIrrigationFile,
) -> Result<(), HillslopeRuntimeInputError> {
    let datver = validate_fixeddate_irrigation_header(fixeddate)?;
    let state_surface = &mut runtime_surface.state_surface;
    seed_fixeddate_irrigation_header_symbols(state_surface, fixeddate, datver)?;
    seed_fixeddate_irrigation_events(state_surface, fixeddate)?;

    Ok(())
}

#[derive(Debug, Clone)]
struct FixedDateProjectionState {
    expected_ofe: usize,
    active_dates: Vec<Line3Record>,
}

impl FixedDateProjectionState {
    fn new(fixeddate: &FixedDateIrrigationFile) -> Self {
        Self {
            expected_ofe: 1,
            active_dates: fixeddate.initial_records.clone(),
        }
    }

    fn advance(&mut self, event: &FixedDateEvent, ofe_count: usize) {
        self.active_dates[self.expected_ofe - 1] = fixeddate_event_next_record(event).clone();
        self.expected_ofe += 1;
        if self.expected_ofe > ofe_count {
            self.expected_ofe = 1;
        }
    }
}

fn validate_fixeddate_irrigation_header(
    fixeddate: &FixedDateIrrigationFile,
) -> Result<f64, HillslopeRuntimeInputError> {
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
    Ok(datver)
}

fn seed_fixeddate_irrigation_header_symbols(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    fixeddate: &FixedDateIrrigationFile,
    datver: f64,
) -> Result<(), HillslopeRuntimeInputError> {
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
    Ok(())
}

fn seed_fixeddate_irrigation_events(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    fixeddate: &FixedDateIrrigationFile,
) -> Result<(), HillslopeRuntimeInputError> {
    let mut projection = FixedDateProjectionState::new(fixeddate);
    for (event_position, event) in fixeddate.events.iter().enumerate() {
        let event_index = event_position + 1;
        seed_fixeddate_irrigation_event(state_surface, event_index, event, &projection)?;
        projection.advance(event, fixeddate.itemp);
    }

    Ok(())
}

fn seed_fixeddate_irrigation_event(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    event_index: usize,
    event: &FixedDateEvent,
    projection: &FixedDateProjectionState,
) -> Result<(), HillslopeRuntimeInputError> {
    seed_fixeddate_irrigation_event_schedule(state_surface, event_index, projection)?;
    match event {
        FixedDateEvent::Sprinkler(sprinkler) => {
            seed_fixeddate_irrigation_sprinkler_event(state_surface, event_index, sprinkler)
        }
        FixedDateEvent::Furrow(furrow) => {
            seed_fixeddate_irrigation_furrow_event(state_surface, event_index, furrow)
        }
    }
}

fn seed_fixeddate_irrigation_event_schedule(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    event_index: usize,
    projection: &FixedDateProjectionState,
) -> Result<(), HillslopeRuntimeInputError> {
    let expected_ofe_f64 = irrigation_count_to_f64(
        "irrigation.fixeddate.event_####.ofe_id",
        projection.expected_ofe,
    )?;
    let schedule = projection.active_dates.get(projection.expected_ofe - 1).ok_or(
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

    Ok(())
}

fn seed_fixeddate_irrigation_sprinkler_event(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    event_index: usize,
    sprinkler: &SprinklerEvent,
) -> Result<(), HillslopeRuntimeInputError> {
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
    Ok(())
}

fn seed_fixeddate_irrigation_furrow_event(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    event_index: usize,
    furrow: &FurrowEvent,
) -> Result<(), HillslopeRuntimeInputError> {
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
    Ok(())
}

fn fixeddate_event_next_record(event: &FixedDateEvent) -> &Line3Record {
    match event {
        FixedDateEvent::Sprinkler(event) => &event.next_record,
        FixedDateEvent::Furrow(event) => &event.next_record,
    }
}

/// Typed parsed snow-control seed projection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypedSnowRuntimeProjection {
    pub rst_c: f64,
    pub newsnw_kg_m3: f64,
    pub ssd_kg_m3: f64,
    pub snow_file_present: bool,
    pub runtime_swe_m: f64,
    pub runtime_depth_m: f64,
    pub runtime_density_kg_m3: f64,
    pub runtime_settle_day_count: f64,
}

/// Typed parsed frost-control seed projection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypedFrostRuntimeProjection {
    pub wint_red: bool,
    pub fine_top: i32,
    pub fine_bot: i32,
    pub ksnowf: f64,
    pub kresf: f64,
    pub ksoilf: f64,
    pub kfactor1: f64,
    pub kfactor2: f64,
    pub kfactor3: f64,
    pub frost_file_present: bool,
    pub dfrost_m: f64,
    pub dthaw_m: f64,
    pub nft: f64,
    pub ws_frz_m: f64,
    pub frwatc_soil_water_before_m: f64,
    pub frwatc_soil_water_after_m: f64,
    pub frwatc_frozen_water_before_m: f64,
    pub frwatc_frozen_water_after_m: f64,
    pub frwatc_freeze_debit_m: f64,
    pub frwatc_thaw_credit_m: f64,
    pub frwatc_net_liquid_delta_m: f64,
    pub infcap_frz_m_s: f64,
    pub frdp_m: f64,
    pub thdp_m: f64,
    pub tfrdp_m: f64,
    pub tthawd_m: f64,
    pub fgthwd_flag: f64,
    pub total_fine_layer_count: f64,
    pub kftill_w_m_k: f64,
    pub kfutil_w_m_k: f64,
    pub kres_w_m_k: f64,
    pub residue_depth_m: f64,
}

/// Project typed snow controls from parsed snow input.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when controls violate CLIM05 domains.
pub fn project_typed_snow_runtime(
    snow: &SnowParseOutput,
) -> Result<TypedSnowRuntimeProjection, HillslopeRuntimeInputError> {
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

    Ok(TypedSnowRuntimeProjection {
        rst_c: rst,
        newsnw_kg_m3: newsnw,
        ssd_kg_m3: ssd,
        snow_file_present: snow.sidecar_present,
        runtime_swe_m: 0.0,
        runtime_depth_m: 0.0,
        runtime_density_kg_m3: 0.0,
        runtime_settle_day_count: 0.0,
    })
}

/// Project typed frost controls from parsed frost input.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when controls violate CLIM06 domains.
pub fn project_typed_frost_runtime(
    frost: &FrostParseOutput,
) -> Result<TypedFrostRuntimeProjection, HillslopeRuntimeInputError> {
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

    Ok(TypedFrostRuntimeProjection {
        wint_red: frost.wint_red == 1,
        fine_top: frost.fine_top,
        fine_bot: frost.fine_bot,
        ksnowf,
        kresf,
        ksoilf,
        kfactor1,
        kfactor2,
        kfactor3,
        frost_file_present: frost.frost_file_present,
        dfrost_m: 0.0,
        dthaw_m: 0.0,
        nft: 0.0,
        ws_frz_m: 0.0,
        frwatc_soil_water_before_m: 0.0,
        frwatc_soil_water_after_m: 0.0,
        frwatc_frozen_water_before_m: 0.0,
        frwatc_frozen_water_after_m: 0.0,
        frwatc_freeze_debit_m: 0.0,
        frwatc_thaw_credit_m: 0.0,
        frwatc_net_liquid_delta_m: 0.0,
        infcap_frz_m_s: 0.0,
        frdp_m: 0.0,
        thdp_m: 0.0,
        tfrdp_m: 0.0,
        tthawd_m: 0.0,
        fgthwd_flag: 0.0,
        total_fine_layer_count: 0.0,
        kftill_w_m_k: 1.75,
        kfutil_w_m_k: 2.1,
        kres_w_m_k: 0.05 * kresf,
        residue_depth_m: 0.0,
    })
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
    let projection = project_typed_snow_runtime(snow)?;

    let state_surface = &mut runtime_surface.state_surface;
    state_surface.insert(
        BoundarySymbol::from("snow.options.rst"),
        BoundaryValue::scalar(projection.rst_c),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.options.newsnw"),
        BoundaryValue::scalar(projection.newsnw_kg_m3),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.options.ssd"),
        BoundaryValue::scalar(projection.ssd_kg_m3),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.options.snow_file_present"),
        BoundaryValue::scalar(if projection.snow_file_present { 1.0 } else { 0.0 }),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_swe"),
        snow_runtime_boundary_value(
            "snow.runtime_swe",
            projection.runtime_swe_m,
            ">= 0.0",
            BoundaryValue::water_depth_meters,
        )?,
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_depth_m"),
        snow_runtime_boundary_value(
            "snow.runtime_depth_m",
            projection.runtime_depth_m,
            ">= 0.0",
            BoundaryValue::water_depth_meters,
        )?,
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_density_kg_m3"),
        snow_runtime_boundary_value(
            "snow.runtime_density_kg_m3",
            projection.runtime_density_kg_m3,
            ">= 0.0",
            BoundaryValue::density_kilograms_per_cubic_meter,
        )?,
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_settle_day_count"),
        BoundaryValue::scalar(projection.runtime_settle_day_count),
    );

    Ok(())
}

fn snow_runtime_boundary_value(
    field: &'static str,
    value: f64,
    allowed: &'static str,
    constructor: fn(f64) -> Result<BoundaryValue, BoundaryError>,
) -> Result<BoundaryValue, HillslopeRuntimeInputError> {
    constructor(value).map_err(|error| match error {
        BoundaryError::NonFinite { value, .. } => HillslopeRuntimeInputError::NonFiniteSnowControl {
            field,
            value,
        },
        BoundaryError::BelowMinimum { value, .. } | BoundaryError::AboveMaximum { value, .. } => {
            HillslopeRuntimeInputError::SnowControlOutOfDomain {
                field,
                value,
                allowed,
            }
        }
    })
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
    let projection = project_typed_frost_runtime(frost)?;

    let state_surface = &mut runtime_surface.state_surface;
    state_surface.insert(
        BoundarySymbol::from("frost.options.wintRed"),
        BoundaryValue::scalar(if projection.wint_red { 1.0 } else { 0.0 }),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.fineTop"),
        BoundaryValue::scalar(f64::from(projection.fine_top)),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.fineBot"),
        BoundaryValue::scalar(f64::from(projection.fine_bot)),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.ksnowf"),
        BoundaryValue::scalar(projection.ksnowf),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kresf"),
        BoundaryValue::scalar(projection.kresf),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.ksoilf"),
        BoundaryValue::scalar(projection.ksoilf),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor1"),
        BoundaryValue::scalar(projection.kfactor1),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor2"),
        BoundaryValue::scalar(projection.kfactor2),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor3"),
        BoundaryValue::scalar(projection.kfactor3),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.frost_file_present"),
        BoundaryValue::scalar(if projection.frost_file_present {
            1.0
        } else {
            0.0
        }),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_dfrost"),
        BoundaryValue::scalar(projection.dfrost_m),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_dthaw"),
        BoundaryValue::scalar(projection.dthaw_m),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_nft"),
        BoundaryValue::scalar(projection.nft),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_ws_frz"),
        BoundaryValue::scalar(projection.ws_frz_m),
    );
    for (symbol, value) in [
        (
            "frost.runtime_frwatc_soil_water_before_m",
            projection.frwatc_soil_water_before_m,
        ),
        (
            "frost.runtime_frwatc_soil_water_after_m",
            projection.frwatc_soil_water_after_m,
        ),
        (
            "frost.runtime_frwatc_frozen_water_before_m",
            projection.frwatc_frozen_water_before_m,
        ),
        (
            "frost.runtime_frwatc_frozen_water_after_m",
            projection.frwatc_frozen_water_after_m,
        ),
        (
            "frost.runtime_frwatc_freeze_debit_m",
            projection.frwatc_freeze_debit_m,
        ),
        (
            "frost.runtime_frwatc_thaw_credit_m",
            projection.frwatc_thaw_credit_m,
        ),
        (
            "frost.runtime_frwatc_net_liquid_delta_m",
            projection.frwatc_net_liquid_delta_m,
        ),
    ] {
        state_surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_infcap_frz"),
        BoundaryValue::scalar(projection.infcap_frz_m_s),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_frdp_m"),
        BoundaryValue::scalar(projection.frdp_m),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_thdp_m"),
        BoundaryValue::scalar(projection.thdp_m),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_tfrdp_m"),
        BoundaryValue::scalar(projection.tfrdp_m),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_tthawd_m"),
        BoundaryValue::scalar(projection.tthawd_m),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_fgthwd_flag"),
        BoundaryValue::scalar(projection.fgthwd_flag),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_total_fine_layer_count"),
        BoundaryValue::scalar(projection.total_fine_layer_count),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_kftill_w_m_k"),
        BoundaryValue::scalar(projection.kftill_w_m_k),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_kfutil_w_m_k"),
        BoundaryValue::scalar(projection.kfutil_w_m_k),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_kres_w_m_k"),
        BoundaryValue::scalar(projection.kres_w_m_k),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_residue_depth_m"),
        BoundaryValue::scalar(projection.residue_depth_m),
    );

    Ok(())
}
