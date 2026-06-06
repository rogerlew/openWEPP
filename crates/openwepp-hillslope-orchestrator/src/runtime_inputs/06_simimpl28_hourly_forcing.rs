const SIMIMPL28_SUNMAP_SOLCON: f64 = 1.94;
const SIMIMPL28_RADCUR_SOLCON: f64 = 0.082;
const SIMIMPL28_PI: f64 = std::f64::consts::PI;
const SIMIMPL28_DOMAIN_EPS: f64 = 1e-12;
const SIMIMPL28_WINTER_HOURS_PER_DAY: usize = 24;
const SIMIMPL28_HOURLY_RADIATION_BOUND_REL_TOLERANCE: f64 = 1.0e-9;
const SIMIMPL28_HOURLY_RADIATION_BOUND_ABS_TOLERANCE_MJ_M2: f64 = 1.0e-12;
const SIMIMPL28_HOURLY_RADIATION_BOUND_ALLOWED: &str =
    "0 <= hradmj <= physical hourly extraterrestrial radiation bound from radcur solar-constant lineage";
// UNIT-CONVERSION-ALLOW: mm_m_scale SIMIMPL28 legacy random-seed scaling, not dimensional conversion.
const SIMIMPL28_RANDOM_THOUSANDTH_SCALE: f64 = 0.001;
// UNIT-CONVERSION-ALLOW: cm_m_scale SIMIMPL28 legacy random-seed scaling, not dimensional conversion.
const SIMIMPL28_RANDOM_HUNDREDTH_SCALE: f64 = 0.01;

#[derive(Debug, Clone, Copy)]
struct Simimpl28AspectGeometry {
    radlat: f64,
    eqlat: f64,
    delong: f64,
}

#[derive(Debug, Clone, Copy)]
struct Simimpl28SunmapResult {
    halfdy: f64,
    dsunmp: f64,
    estrad_mj_m2: f64,
    rpoth_mj_m2: f64,
    hourly_radiation_upper_bound_mj_m2: f64,
    cloud_fraction: f64,
}

#[derive(Debug, Clone, Copy)]
struct Simimpl28StmtimHourlyPartition {
    hrrain_m: f64,
    hrsnow_m: f64,
    wntdur_h: f64,
    wnttim_h: f64,
    active_interval: bool,
    rain_branch: bool,
    snow_branch: bool,
}

#[allow(clippy::too_many_lines)]
fn build_simimpl28_hourly_winter_forcing_symbols(
    forcing: &HillslopeClimateDailyForcing,
    metadata: &ClimateMetadata,
    winter_context_state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<BTreeMap<BoundarySymbol, BoundaryValue>, ClimateRuntimeInputError> {
    let snow_file_present = optional_runtime_context_scalar(
        winter_context_state_surface,
        "snow.options.snow_file_present",
    )?
    .unwrap_or(0.0);
    let frost_file_present = optional_runtime_context_scalar(
        winter_context_state_surface,
        "frost.options.frost_file_present",
    )?
    .unwrap_or(0.0);

    if !(is_binary_flag(snow_file_present) && is_binary_flag(frost_file_present)) {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "snow.options.snow_file_present / frost.options.frost_file_present".to_string(),
            value: snow_file_present.max(frost_file_present),
            allowed: "{0,1}",
        });
    }

    let (day, mon, year, rain_m, stmdur_s, tmax, tmin, radly, wnttim) = match forcing {
        HillslopeClimateDailyForcing::NoBreakpoint(day) => (
            day.day,
            day.mon,
            day.year,
            day.prcp,
            day.stmdur,
            day.tmax,
            day.tmin,
            day.rad,
            simimpl28_winter_random_start_hour(simimpl28_day_of_year(day.day, day.mon, day.year)?),
        ),
        HillslopeClimateDailyForcing::Breakpoint(day) => (
            day.day, day.mon, day.year, day.prcp, day.stmdur, day.tmax, day.tmin, day.rad,
            day.stmstr,
        ),
    };
    let runtime_swe = optional_runtime_context_scalar(
        winter_context_state_surface,
        "snow.runtime_swe",
    )?
    .unwrap_or(0.0);
    if runtime_swe < 0.0 {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "snow.runtime_swe".to_string(),
            value: runtime_swe,
            allowed: ">= 0",
        });
    }
    let frost_depth = optional_runtime_context_scalar(
        winter_context_state_surface,
        "frost.runtime_dfrost",
    )?
    .unwrap_or(0.0);
    if frost_depth < 0.0 {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "frost.runtime_dfrost".to_string(),
            value: frost_depth,
            allowed: ">= 0",
        });
    }
    let frozen_water = optional_runtime_context_scalar(
        winter_context_state_surface,
        "frost.runtime_ws_frz",
    )?
    .unwrap_or(0.0);
    if frozen_water < 0.0 {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "frost.runtime_ws_frz".to_string(),
            value: frozen_water,
            allowed: ">= 0",
        });
    }
    let winter_trigger_active = runtime_swe > SIMIMPL28_DOMAIN_EPS
        || frost_depth > SIMIMPL28_DOMAIN_EPS
        || frozen_water > SIMIMPL28_DOMAIN_EPS
        || f64::midpoint(tmax, tmin) < 0.0;

    if !winter_trigger_active {
        return Ok(BTreeMap::new());
    }

    let avgslp = require_runtime_context_scalar(winter_context_state_surface, "avgslp")?;
    if avgslp <= 0.0 {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "avgslp".to_string(),
            value: avgslp,
            allowed: "> 0",
        });
    }
    let azm = require_runtime_context_scalar(winter_context_state_surface, "azm")?;
    let rst = require_runtime_context_scalar(winter_context_state_surface, "snow.options.rst")?;

    let sdate = simimpl28_day_of_year(day, mon, year)?;
    let geometry = simimpl28_aspect_geometry(metadata.deglat, avgslp, azm)?;
    let radmj = simimpl28_langleys_to_mj_m2("rad", radly)?;
    let sunmap = simimpl28_sunmap(radly, sdate, geometry)?;
    let itflag = (tmax - tmin) <= 1.0;

    let mut symbols = BTreeMap::new();
    for hour in 1..=SIMIMPL28_WINTER_HOURS_PER_DAY {
        let cratio = simimpl28_radcur(sdate, hour, geometry.radlat, sunmap.dsunmp)?;
        let (hrrad_mj_m2, hrtemp_c) =
            simimpl28_hr_tmp_hour(itflag, hour, sunmap, cratio, radmj, tmax, tmin)?;
        let hour_u32 = u32::try_from(hour).map_err(|_| {
            ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
                symbol: "hour".to_string(),
                value: f64::from(u32::MAX),
                allowed: "1..=24",
            }
        })?;
        let partition = simimpl28_stmtim_hourly_partition(
            rain_m,
            stmdur_s,
            f64::from(hour_u32),
            wnttim,
            rst,
            hrtemp_c,
        )?;

        symbols.insert(
            simimpl28_hourly_symbol("winter.hourly.rad_mj_m2", hour),
            climate_boundary_value(
                "winter.hourly.rad_mj_m2",
                ">= 0",
                BoundaryValue::solar_radiation_megajoules_per_square_meter_per_hour(hrrad_mj_m2),
            )?,
        );
        symbols.insert(
            simimpl28_hourly_symbol("winter.hourly.air_temp_c", hour),
            climate_boundary_value(
                "winter.hourly.air_temp_c",
                "finite",
                BoundaryValue::temperature_celsius(hrtemp_c),
            )?,
        );
        symbols.insert(
            simimpl28_hourly_symbol("winter.hourly.cloud_fraction", hour),
            climate_boundary_value(
                "winter.hourly.cloud_fraction",
                "0..=1",
                BoundaryValue::fraction_unit_interval(sunmap.cloud_fraction),
            )?,
        );
        symbols.insert(
            simimpl28_hourly_symbol("snow.hourly.rain_m", hour),
            climate_boundary_value(
                "snow.hourly.rain_m",
                ">= 0",
                BoundaryValue::water_depth_meters(partition.hrrain_m),
            )?,
        );
        symbols.insert(
            simimpl28_hourly_symbol("snow.hourly.snowfall_m", hour),
            climate_boundary_value(
                "snow.hourly.snowfall_m",
                ">= 0",
                BoundaryValue::water_depth_meters(partition.hrsnow_m),
            )?,
        );
        symbols.insert(
            simimpl28_hourly_symbol("snow.hourly.stmtim.rain_m", hour),
            climate_boundary_value(
                "snow.hourly.stmtim.rain_m",
                ">= 0",
                BoundaryValue::water_depth_meters(rain_m),
            )?,
        );
        symbols.insert(
            simimpl28_hourly_symbol("snow.hourly.stmtim.stmdur_s", hour),
            climate_boundary_value(
                "snow.hourly.stmtim.stmdur_s",
                ">= 0",
                BoundaryValue::elapsed_time_seconds(stmdur_s),
            )?,
        );
        symbols.insert(
            simimpl28_hourly_symbol("snow.hourly.stmtim.wntdur_h", hour),
            BoundaryValue::scalar(partition.wntdur_h),
        );
        symbols.insert(
            simimpl28_hourly_symbol("snow.hourly.stmtim.wnttim_h", hour),
            BoundaryValue::scalar(partition.wnttim_h),
        );
        symbols.insert(
            simimpl28_hourly_symbol("snow.hourly.stmtim.hrtemp_c", hour),
            climate_boundary_value(
                "snow.hourly.stmtim.hrtemp_c",
                "finite",
                BoundaryValue::temperature_celsius(hrtemp_c),
            )?,
        );
        symbols.insert(
            simimpl28_hourly_symbol("snow.hourly.stmtim.rst_c", hour),
            climate_boundary_value(
                "snow.hourly.stmtim.rst_c",
                "finite",
                BoundaryValue::temperature_celsius(rst),
            )?,
        );
        symbols.insert(
            simimpl28_hourly_symbol("snow.hourly.stmtim.hrrain_m", hour),
            climate_boundary_value(
                "snow.hourly.stmtim.hrrain_m",
                ">= 0",
                BoundaryValue::water_depth_meters(partition.hrrain_m),
            )?,
        );
        symbols.insert(
            simimpl28_hourly_symbol("snow.hourly.stmtim.hrsnow_m", hour),
            climate_boundary_value(
                "snow.hourly.stmtim.hrsnow_m",
                ">= 0",
                BoundaryValue::water_depth_meters(partition.hrsnow_m),
            )?,
        );
        symbols.insert(
            simimpl28_hourly_symbol("snow.hourly.stmtim.active_interval", hour),
            climate_boundary_value(
                "snow.hourly.stmtim.active_interval",
                "0..=1",
                BoundaryValue::fraction_unit_interval(simimpl28_bool_flag(
                    partition.active_interval,
                )),
            )?,
        );
        symbols.insert(
            simimpl28_hourly_symbol("snow.hourly.stmtim.rain_branch", hour),
            climate_boundary_value(
                "snow.hourly.stmtim.rain_branch",
                "0..=1",
                BoundaryValue::fraction_unit_interval(simimpl28_bool_flag(partition.rain_branch)),
            )?,
        );
        symbols.insert(
            simimpl28_hourly_symbol("snow.hourly.stmtim.snow_branch", hour),
            climate_boundary_value(
                "snow.hourly.stmtim.snow_branch",
                "0..=1",
                BoundaryValue::fraction_unit_interval(simimpl28_bool_flag(partition.snow_branch)),
            )?,
        );
    }

    Ok(symbols)
}

fn optional_runtime_context_scalar(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &'static str,
) -> Result<Option<f64>, ClimateRuntimeInputError> {
    let value = surface.get(&BoundarySymbol::from(symbol));
    match value {
        Some(value) => {
            let scalar = value.as_f64();
            if !scalar.is_finite() {
                return Err(ClimateRuntimeInputError::NonFiniteField {
                    field: symbol,
                    value: scalar,
                });
            }
            Ok(Some(scalar))
        }
        None => Ok(None),
    }
}

fn require_runtime_context_scalar(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &'static str,
) -> Result<f64, ClimateRuntimeInputError> {
    let value = optional_runtime_context_scalar(surface, symbol)?.ok_or_else(|| {
        ClimateRuntimeInputError::MissingRuntimeContextSymbol {
            symbol: symbol.to_string(),
        }
    })?;
    Ok(value)
}

fn is_binary_flag(value: f64) -> bool {
    (value - 0.0).abs() <= SIMIMPL28_DOMAIN_EPS || (value - 1.0).abs() <= SIMIMPL28_DOMAIN_EPS
}

fn simimpl28_hourly_symbol(root: &str, hour: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("{root}_{hour:04}"))
}

const fn simimpl28_bool_flag(value: bool) -> f64 {
    if value { 1.0 } else { 0.0 }
}

fn simimpl28_day_of_year(day: i32, mon: i32, year: i32) -> Result<i32, ClimateRuntimeInputError> {
    if !(1..=12).contains(&mon) || day < 1 {
        return Err(ClimateRuntimeInputError::InvalidCalendarDate { day, mon, year });
    }
    let leap = is_leap_year(year);
    let month_lengths = if leap {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let month_index = usize::try_from(mon - 1)
        .map_err(|_| ClimateRuntimeInputError::InvalidCalendarDate { day, mon, year })?;
    let max_day = month_lengths[month_index];
    if day > max_day {
        return Err(ClimateRuntimeInputError::InvalidCalendarDate { day, mon, year });
    }
    let ordinal = month_lengths[..month_index].iter().sum::<i32>() + day;
    Ok(ordinal)
}

const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn simimpl28_aspect_geometry(
    deglat: f64,
    avgslp: f64,
    azm: f64,
) -> Result<Simimpl28AspectGeometry, ClimateRuntimeInputError> {
    if !deglat.is_finite() {
        return Err(ClimateRuntimeInputError::NonFiniteField {
            field: "deglat",
            value: deglat,
        });
    }
    if !avgslp.is_finite() {
        return Err(ClimateRuntimeInputError::NonFiniteField {
            field: "avgslp",
            value: avgslp,
        });
    }
    if !azm.is_finite() {
        return Err(ClimateRuntimeInputError::NonFiniteField {
            field: "azm",
            value: azm,
        });
    }

    let radinc = avgslp.atan();
    let rdaz = azm * SIMIMPL28_PI / 180.0;
    let radlat = deglat * SIMIMPL28_PI / 180.0;
    let eqlat = (radinc.cos() * radlat.sin() + (radinc.sin() * radlat.cos() * rdaz.cos())).asin();

    let mut d1 = radinc.cos() * radlat.cos() - radinc.sin() * radlat.sin() * rdaz.cos();
    if d1 < 1e-10 {
        d1 = 1e-10;
    }
    let mut delong = ((radinc.sin() * rdaz.sin()) / d1).atan();
    if d1 < 0.0 {
        delong += SIMIMPL28_PI;
    }

    if !(radlat.is_finite() && eqlat.is_finite() && delong.is_finite()) {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "aspect_geometry".to_string(),
            value: f64::NAN,
            allowed: "finite geometry",
        });
    }

    Ok(Simimpl28AspectGeometry {
        radlat,
        eqlat,
        delong,
    })
}

fn simimpl28_langleys_to_mj_m2(
    symbol: &'static str,
    value_ly: f64,
) -> Result<f64, ClimateRuntimeInputError> {
    openwepp_unit_boundary::conversions::langleys_per_day_to_megajoules_per_square_meter_per_day(
        value_ly,
    )
    .map_err(|_| ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
        symbol: symbol.to_string(),
        value: value_ly,
        allowed: "finite and >= 0",
    })
}

fn simimpl28_psolr(sol_d: f64, v: f64, w: f64, x: f64, y: f64) -> f64 {
    (sol_d.sin() * w.sin() * (x - y) * 12.0 / SIMIMPL28_PI)
        + (sol_d.cos() * w.cos() * ((x + v).sin() - (y + v).sin()) * 12.0 / SIMIMPL28_PI)
}

#[allow(clippy::too_many_lines, clippy::many_single_char_names)]
fn simimpl28_sunmap(
    radly: f64,
    sdate: i32,
    geometry: Simimpl28AspectGeometry,
) -> Result<Simimpl28SunmapResult, ClimateRuntimeInputError> {
    if !radly.is_finite() {
        return Err(ClimateRuntimeInputError::NonFiniteField {
            field: "radly",
            value: radly,
        });
    }

    let sdate_f = f64::from(sdate);
    let d = 0.00698 - 0.4067 * ((sdate_f + 10.0) * 0.0172).cos();
    let e = 1.0 - 0.0167 * ((sdate_f - 3.0) * 0.0172).cos();
    if e.abs() <= SIMIMPL28_DOMAIN_EPS {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "sunmap.eccentricity".to_string(),
            value: e,
            allowed: "abs(e) > 0",
        });
    }
    let r1 = (60.0 * SIMIMPL28_SUNMAP_SOLCON) / (e * e);

    let mut x = -((geometry.eqlat.sin() / geometry.eqlat.cos()) * (d.sin() / d.cos()));
    x = x.clamp(-1.0, 1.0);
    let t = x.acos();
    let mut t7 = t - geometry.delong;
    let mut t6 = -t - geometry.delong;

    x = -(geometry.radlat.tan() * d.tan());
    x = x.clamp(-1.0, 1.0);
    let t = x.acos();
    let t1 = t;
    let t0 = -t;

    let t3 = t7.min(t1);
    let mut t2 = t6;
    if t6 <= t0 {
        t2 = t0;
    }

    let mut t2_effective = t2;
    if t3 < t2_effective {
        t2_effective = t3;
    }
    t6 += 2.0 * SIMIMPL28_PI;

    let r4_ly = if t6 < t1 {
        let t8 = t6;
        let t9 = t1;
        r1 * (simimpl28_psolr(d, geometry.delong, geometry.eqlat, t3, t2_effective)
            + simimpl28_psolr(d, geometry.delong, geometry.eqlat, t9, t8))
    } else {
        t7 -= 2.0 * SIMIMPL28_PI;
        if t7 > t0 {
            let t8 = t0;
            let t9 = t7;
            r1 * (simimpl28_psolr(d, geometry.delong, geometry.eqlat, t3, t2_effective)
                + simimpl28_psolr(d, geometry.delong, geometry.eqlat, t9, t8))
        } else {
            r1 * simimpl28_psolr(d, geometry.delong, geometry.eqlat, t3, t2_effective)
        }
    };
    let r4 = simimpl28_langleys_to_mj_m2("sunmap.r4", r4_ly)?;

    let t4 = t2_effective * 12.0 / SIMIMPL28_PI;
    let halfdy = t4.abs();

    let r3 = r1
        * ((d.sin() * geometry.radlat.sin() * (t1 - t0) * 12.0 / SIMIMPL28_PI)
            + (d.cos() * geometry.radlat.cos() * (t1.sin() - t0.sin()) * 12.0 / SIMIMPL28_PI));
    let hourly_radiation_upper_bound_mj_m2 =
        simimpl28_hourly_extraterrestrial_radiation_upper_bound(sdate)?;

    if r3.abs() <= SIMIMPL28_DOMAIN_EPS {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "sunmap.r3".to_string(),
            value: r3,
            allowed: "abs(r3) > 0",
        });
    }

    let sindlt = 0.39785
        * ((SIMIMPL28_PI / 180.0)
            * (278.97
                + 0.9856 * sdate_f
                + 1.9165 * ((SIMIMPL28_PI / 180.0) * (356.6 + 0.9856 * sdate_f)).sin()))
        .sin();
    let cosdlt = (1.0 - sindlt.powi(2)).sqrt();
    let cosfi = sindlt * geometry.radlat.sin() + cosdlt * geometry.radlat.cos();
    if cosfi.abs() <= SIMIMPL28_DOMAIN_EPS {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "sunmap.cosfi".to_string(),
            value: cosfi,
            allowed: "abs(cosfi) > 0",
        });
    }

    let fortao = radly - 0.3 * r3;
    let mut tao = if fortao > 0.0 {
        (fortao / (r3 * 0.7)).powf(cosfi)
    } else {
        0.4
    };
    tao = tao.clamp(0.4, 0.75);

    let ms = 1.0 / cosfi;
    let mut cloud_fraction = (0.3 + 0.7 * 0.75_f64.powf(ms) - radly / r3)
        / (0.7 * (0.75_f64.powf(ms) - 0.4_f64.powf(ms)));
    cloud_fraction = cloud_fraction.clamp(0.0, 1.0);

    let rpoth_mj_m2 = simimpl28_langleys_to_mj_m2("sunmap.r3", r3)?;
    let sb = rpoth_mj_m2 * tao.powf(ms);
    let sd = 0.3 * (rpoth_mj_m2 - sb);
    let denominator = sb + sd;
    if denominator.abs() <= SIMIMPL28_DOMAIN_EPS {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "sunmap.sb_plus_sd".to_string(),
            value: denominator,
            allowed: "abs(sb+sd) > 0",
        });
    }
    let f = (r4 * tao.powf(ms) + sd) / denominator;
    let estrad_mj_m2 = simimpl28_langleys_to_mj_m2("sunmap.estrad", f * radly)?;

    if !(halfdy.is_finite()
        && d.is_finite()
        && estrad_mj_m2.is_finite()
        && rpoth_mj_m2.is_finite()
        && hourly_radiation_upper_bound_mj_m2.is_finite()
        && cloud_fraction.is_finite())
    {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "sunmap.outputs".to_string(),
            value: f64::NAN,
            allowed: "finite",
        });
    }

    Ok(Simimpl28SunmapResult {
        halfdy,
        dsunmp: d,
        estrad_mj_m2,
        rpoth_mj_m2,
        hourly_radiation_upper_bound_mj_m2,
        cloud_fraction,
    })
}

fn simimpl28_hourly_extraterrestrial_radiation_upper_bound(
    sdate: i32,
) -> Result<f64, ClimateRuntimeInputError> {
    let day = f64::from(sdate);
    let rdsun = 1.0 + 0.033 * ((2.0 * SIMIMPL28_PI * day) / 365.0).cos();
    let hour_angle_integral = 2.0 * (SIMIMPL28_PI / 24.0).sin();
    let bound = ((12.0 * 60.0) / SIMIMPL28_PI)
        * SIMIMPL28_RADCUR_SOLCON
        * rdsun
        * hour_angle_integral;
    if !bound.is_finite() || bound <= 0.0 {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "winter.hourly.rad_mj_m2.physical_upper_bound".to_string(),
            value: bound,
            allowed: "> 0 and finite",
        });
    }
    Ok(bound)
}

fn simimpl28_radcur(
    sdate: i32,
    hour: usize,
    radlat: f64,
    decl: f64,
) -> Result<f64, ClimateRuntimeInputError> {
    let day = f64::from(sdate);
    let hour_f = f64::from(u32::try_from(hour).map_err(|_| {
        ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "hour".to_string(),
            value: f64::from(u32::MAX),
            allowed: "1..=24",
        }
    })?);

    let dfact = ((2.0 * SIMIMPL28_PI) * (day - 81.0)) / 365.0;
    let slrtm = (0.1645 * (2.0 * dfact).sin()) - (0.1255 * dfact.cos()) - (0.025 * dfact.sin());
    let hasun = ((hour_f + slrtm) - 12.0) * (SIMIMPL28_PI / 12.0);
    let haset = hasun - (SIMIMPL28_PI / 24.0);
    let harise = hasun + (SIMIMPL28_PI / 24.0);
    let rdsun = 1.0 + 0.033 * ((2.0 * SIMIMPL28_PI * day) / 365.0).cos();

    let mut ratio = ((12.0 * 60.0) / SIMIMPL28_PI)
        * SIMIMPL28_RADCUR_SOLCON
        * rdsun
        * (radlat.cos() * decl.cos() * (harise.sin() - haset.sin())
            + (harise - haset) * radlat.sin() * decl.sin());
    if ratio < 0.0 {
        ratio = 0.0;
    }
    if !ratio.is_finite() {
        return Err(ClimateRuntimeInputError::NonFiniteField {
            field: "radcur.ratio",
            value: ratio,
        });
    }
    Ok(ratio)
}

fn simimpl28_hrtmp(hour: usize, halfdy: f64, tmax: f64, tmin: f64) -> f64 {
    let sunris = 12.0 - halfdy;
    let tave = f64::midpoint(tmax, tmin);
    let amp = (tmax - tmin) / 2.0;
    let hour_u32 = u32::try_from(hour).unwrap_or(u32::MAX);
    let hour_f = f64::from(hour_u32);
    if hour_f < sunris || 14.0 < hour_f {
        let adjhr = if hour_f < sunris {
            (hour_f - 0.5) + 10.0
        } else {
            (hour_f - 0.5) - 14.0
        };
        tave + amp * ((SIMIMPL28_PI * adjhr) / (10.0 + sunris)).cos()
    } else {
        tave - amp * (SIMIMPL28_PI * (hour_f - 0.5 - sunris) / (14.0 - sunris)).cos()
    }
}

fn simimpl28_hr_tmp_hour(
    itflag: bool,
    hour: usize,
    sunmap: Simimpl28SunmapResult,
    cratio: f64,
    radmj: f64,
    tmax: f64,
    tmin: f64,
) -> Result<(f64, f64), ClimateRuntimeInputError> {
    let (hrrad_mj_m2, hrtemp_c) = if itflag {
        (
            openwepp_unit_boundary::conversions::megajoules_per_square_meter_per_day_to_uniform_hourly(
                radmj,
            )
            .map_err(|_| ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
                symbol: "rad".to_string(),
                value: radmj,
                allowed: "finite and >= 0",
            })?,
            f64::midpoint(tmax, tmin),
        )
    } else {
        if sunmap.rpoth_mj_m2.abs() <= SIMIMPL28_DOMAIN_EPS {
            return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
                symbol: "rpoth".to_string(),
                value: sunmap.rpoth_mj_m2,
                allowed: "abs(rpoth) > 0",
            });
        }
        (
            sunmap.estrad_mj_m2 * cratio / sunmap.rpoth_mj_m2,
            simimpl28_hrtmp(hour, sunmap.halfdy, tmax, tmin),
        )
    };

    if !hrrad_mj_m2.is_finite() {
        return Err(ClimateRuntimeInputError::NonFiniteField {
            field: "winter.hourly.rad_mj_m2",
            value: hrrad_mj_m2,
        });
    }
    let allowed_upper = sunmap.hourly_radiation_upper_bound_mj_m2
        * (1.0 + SIMIMPL28_HOURLY_RADIATION_BOUND_REL_TOLERANCE)
        + SIMIMPL28_HOURLY_RADIATION_BOUND_ABS_TOLERANCE_MJ_M2;
    if hrrad_mj_m2 < 0.0 || hrrad_mj_m2 > allowed_upper {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: simimpl28_hourly_symbol("winter.hourly.rad_mj_m2", hour).to_string(),
            value: hrrad_mj_m2,
            allowed: SIMIMPL28_HOURLY_RADIATION_BOUND_ALLOWED,
        });
    }
    if !hrtemp_c.is_finite() {
        return Err(ClimateRuntimeInputError::NonFiniteField {
            field: "winter.hourly.air_temp_c",
            value: hrtemp_c,
        });
    }

    Ok((hrrad_mj_m2, hrtemp_c))
}

fn simimpl28_stmtim_hourly_partition(
    rain_m: f64,
    stmdur_s: f64,
    hour: f64,
    mut wnttim: f64,
    rst: f64,
    hrtemp_c: f64,
) -> Result<Simimpl28StmtimHourlyPartition, ClimateRuntimeInputError> {
    if !rain_m.is_finite() {
        return Err(ClimateRuntimeInputError::NonFiniteField {
            field: "prcp",
            value: rain_m,
        });
    }
    if !stmdur_s.is_finite() {
        return Err(ClimateRuntimeInputError::NonFiniteField {
            field: "stmdur",
            value: stmdur_s,
        });
    }
    wnttim = simimpl28_stmtim_start_time(wnttim)?;
    if rain_m <= 0.0001 {
        return Ok(Simimpl28StmtimHourlyPartition {
            hrrain_m: 0.0,
            hrsnow_m: 0.0,
            wntdur_h: 0.0,
            wnttim_h: wnttim,
            active_interval: false,
            rain_branch: false,
            snow_branch: false,
        });
    }
    let tmpvr3 = openwepp_unit_boundary::conversions::seconds_to_legacy_stmtim_hours(stmdur_s)
        .map_err(|error| match error {
            openwepp_unit_boundary::BoundaryError::NonFinite { value, .. } => {
                ClimateRuntimeInputError::NonFiniteField {
                    field: "stmdur",
                    value,
                }
            }
            openwepp_unit_boundary::BoundaryError::BelowMinimum { value, .. }
            | openwepp_unit_boundary::BoundaryError::AboveMaximum { value, .. } => {
                ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
                    symbol: "stmdur".to_string(),
                    value,
                    allowed: "finite and >= 0",
                }
            }
        })?;
    let mut wntdur = tmpvr3.floor();
    if (tmpvr3 - wntdur) >= 0.5 {
        wntdur += 1.0;
    }
    if wntdur < 0.0001 {
        wntdur = 1.0;
    }
    if (wnttim + wntdur) > 24.0 {
        wnttim = 24.0 - wntdur;
    }
    if (wntdur - 24.0).abs() <= SIMIMPL28_DOMAIN_EPS {
        wnttim = 1.0;
    }
    if wntdur <= SIMIMPL28_DOMAIN_EPS {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "wntdur".to_string(),
            value: wntdur,
            allowed: "> 0",
        });
    }

    let active_interval = (hour >= wnttim) && (hour < (wnttim + wntdur));
    if active_interval {
        if hrtemp_c > rst {
            Ok(Simimpl28StmtimHourlyPartition {
                hrrain_m: rain_m / wntdur,
                hrsnow_m: 0.0,
                wntdur_h: wntdur,
                wnttim_h: wnttim,
                active_interval,
                rain_branch: true,
                snow_branch: false,
            })
        } else {
            Ok(Simimpl28StmtimHourlyPartition {
                hrrain_m: 0.0,
                hrsnow_m: rain_m / wntdur * 10.0,
                wntdur_h: wntdur,
                wnttim_h: wnttim,
                active_interval,
                rain_branch: false,
                snow_branch: true,
            })
        }
    } else {
        Ok(Simimpl28StmtimHourlyPartition {
            hrrain_m: 0.0,
            hrsnow_m: 0.0,
            wntdur_h: wntdur,
            wnttim_h: wnttim,
            active_interval,
            rain_branch: false,
            snow_branch: false,
        })
    }
}

fn simimpl28_stmtim_start_time(mut wnttim: f64) -> Result<f64, ClimateRuntimeInputError> {
    if !wnttim.is_finite() {
        return Err(ClimateRuntimeInputError::NonFiniteField {
            field: "wnttim",
            value: wnttim,
        });
    }
    if wnttim < 1.0 {
        wnttim = 1.0;
    }
    Ok(wnttim)
}

fn simimpl28_winter_random_start_hour(sdate: i32) -> f64 {
    let mut i = sdate;
    let mut k1 = i;
    let mut k2 = i;
    let mut k3 = i;
    let mut k4 = i;
    k4 = 3 * k4 + k2;
    k3 = 3 * k3 + k1;
    k2 *= 3;
    k1 *= 3;
    i = k1 / 1000;
    k1 -= i * 1000;
    k2 += i;
    i = k2 / 100;
    k2 -= 100 * i;
    k3 += i;
    i = k3 / 1000;
    k3 -= i * 1000;
    k4 += i;
    i = k4 / 100;
    k4 -= 100 * i;
    let randn_seed =
        ((f64::from(k1) * SIMIMPL28_RANDOM_THOUSANDTH_SCALE + f64::from(k2))
            * SIMIMPL28_RANDOM_HUNDREDTH_SCALE
            + f64::from(k3))
            * SIMIMPL28_RANDOM_THOUSANDTH_SCALE
            + f64::from(k4);
    let randn = (randn_seed * SIMIMPL28_RANDOM_HUNDREDTH_SCALE) * 24.0;
    let mut wnttim = randn.floor();
    if wnttim < 1.0 {
        wnttim = 1.0;
    }
    wnttim
}
