const SIMIMPL28_SUNMAP_SOLCON: f64 = 1.94;
const SIMIMPL28_RADCUR_SOLCON: f64 = 0.082;
const SIMIMPL28_PI: f64 = std::f64::consts::PI;
const SIMIMPL28_DOMAIN_EPS: f64 = 1e-12;
const SIMIMPL28_WINTER_HOURS_PER_DAY: usize = 24;
pub const DIRECT_WINTER_HOURLY_FORCING_COUNT: usize = SIMIMPL28_WINTER_HOURS_PER_DAY;
const SIMIMPL28_HOURLY_RADIATION_BOUND_REL_TOLERANCE: f64 = 1.0e-9;
const SIMIMPL28_HOURLY_RADIATION_BOUND_ABS_TOLERANCE_MJ_M2: f64 = 1.0e-12;
const SIMIMPL28_DAILY_RADIATION_BOUND_ALLOWED: &str =
    "0 <= radly <= baseline sunmap horizontal daily potential (rpoth/r3)";
const SIMIMPL28_HOURLY_RADIATION_BOUND_ALLOWED: &str =
    "0 <= hradmj <= physical hourly extraterrestrial radiation bound from radcur solar-constant lineage";
// UNIT-CONVERSION-ALLOW: mm_m_scale SIMIMPL28 legacy random-seed scaling, not dimensional conversion.
const SIMIMPL28_RANDOM_THOUSANDTH_SCALE: f64 = 0.001;
// UNIT-CONVERSION-ALLOW: cm_m_scale SIMIMPL28 legacy random-seed scaling, not dimensional conversion.
const SIMIMPL28_RANDOM_HUNDREDTH_SCALE: f64 = 0.01;

use openwepp_meteorology::phase::{PhaseTimescale, harder_pomeroy_phase_from_relative_humidity};
use openwepp_meteorology::psychrometrics::saturation_vapor_pressure_water_kpa;
use openwepp_unit_boundary::{FractionUnitInterval, TemperatureCelsius};

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
    rain_fraction: f64,
    snow_fraction: f64,
    phase_model: SnowPhasePartitionModel,
    hydrometeor_temperature_c: Option<f64>,
    relative_humidity: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SnowPhasePartitionModel {
    #[default]
    LegacyRst,
    HarderPomeroyHourly,
}

impl SnowPhasePartitionModel {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::LegacyRst => "legacy_rst",
            Self::HarderPomeroyHourly => "harder_pomeroy_hourly",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectWinterHourlyForcing {
    pub rain_m: f64,
    pub snowfall_m: f64,
    pub radiation_mj_m2: f64,
    pub air_temperature_c: f64,
    pub cloud_fraction: f64,
    pub daily_solar_radiation_mj_m2: f64,
    pub daily_extraterrestrial_radiation_mj_m2: f64,
    pub daylight: bool,
    pub phase_model: SnowPhasePartitionModel,
    pub rain_fraction: f64,
    pub snow_fraction: f64,
    pub hydrometeor_temperature_c: Option<f64>,
    pub relative_humidity: Option<f64>,
}

impl DirectWinterHourlyForcing {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            rain_m: 0.0,
            snowfall_m: 0.0,
            radiation_mj_m2: 0.0,
            air_temperature_c: 0.0,
            cloud_fraction: 0.0,
            daily_solar_radiation_mj_m2: 0.0,
            daily_extraterrestrial_radiation_mj_m2: 0.0,
            daylight: false,
            phase_model: SnowPhasePartitionModel::LegacyRst,
            rain_fraction: 0.0,
            snow_fraction: 0.0,
            hydrometeor_temperature_c: None,
            relative_humidity: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectWinterHourlyContext {
    pub snow_runtime_swe_m: f64,
    pub frost_runtime_depth_m: f64,
    pub frost_runtime_frozen_water_m: f64,
    pub frost_file_present: bool,
    pub frost_wint_red_enabled: bool,
    pub avg_slope: f64,
    pub azimuth: f64,
    pub snow_rst_c: f64,
    pub snow_phase_model: SnowPhasePartitionModel,
}

impl HillslopeClimateRuntimeRequest {
    /// Build typed hourly winter forcing for production direct mode without
    /// materializing runtime symbols.
    ///
    /// Returns `Ok(None)` when the existing SIMIMPL28 winter trigger is inactive.
    pub fn direct_winter_hourly_forcing(
        &self,
        day_index: usize,
        context: DirectWinterHourlyContext,
    ) -> Result<Option<[DirectWinterHourlyForcing; SIMIMPL28_WINTER_HOURS_PER_DAY]>, ClimateRuntimeInputError>
    {
        let forcing = select_day_forcing(&self.shared, day_index)?;
        build_simimpl28_hourly_winter_forcing_typed(
            forcing,
            &self.metadata,
            context,
            Simimpl28WinterExportMode::ProductionTrigger,
        )
    }

    /// Build complete diagnostic hourly winter forcing rows for external
    /// snow-model input export. This uses the same SIMIMPL28 hourly radiation,
    /// temperature, and rain/snow partition calculations as production direct
    /// mode, but deliberately bypasses the production winter-trigger
    /// suppression so warm/no-snow days remain present in the exported series.
    ///
    /// This method is diagnostic-only and must not be used to alter production
    /// winter branch activation.
    pub fn diagnostic_winter_hourly_forcing(
        &self,
        day_index: usize,
        context: DirectWinterHourlyContext,
    ) -> Result<[DirectWinterHourlyForcing; SIMIMPL28_WINTER_HOURS_PER_DAY], ClimateRuntimeInputError>
    {
        let forcing = select_day_forcing(&self.shared, day_index)?;
        match build_simimpl28_hourly_winter_forcing_typed(
            forcing,
            &self.metadata,
            context,
            Simimpl28WinterExportMode::ForceCompleteDiagnosticRows,
        )? {
            Some(forcing) => Ok(forcing),
            None => Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
                symbol: "diagnostic_winter_hourly_forcing".to_string(),
                value: f64::NAN,
                allowed: "forced diagnostic export must return hourly rows",
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Simimpl28WinterExportMode {
    ProductionTrigger,
    ForceCompleteDiagnosticRows,
}

#[allow(clippy::too_many_lines)]
fn build_simimpl28_hourly_winter_forcing_typed(
    forcing: &HillslopeClimateDailyForcing,
    metadata: &ClimateMetadata,
    context: DirectWinterHourlyContext,
    export_mode: Simimpl28WinterExportMode,
) -> Result<Option<[DirectWinterHourlyForcing; SIMIMPL28_WINTER_HOURS_PER_DAY]>, ClimateRuntimeInputError>
{
    if context.snow_runtime_swe_m < 0.0 {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "snow.runtime_swe".to_string(),
            value: context.snow_runtime_swe_m,
            allowed: ">= 0",
        });
    }
    if context.frost_runtime_depth_m < 0.0 {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "frost.runtime_dfrost".to_string(),
            value: context.frost_runtime_depth_m,
            allowed: ">= 0",
        });
    }
    if context.frost_runtime_frozen_water_m < 0.0 {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "frost.runtime_ws_frz".to_string(),
            value: context.frost_runtime_frozen_water_m,
            allowed: ">= 0",
        });
    }
    let (day, mon, year, rain_m, stmdur_s, tmax, tmin, radly, tdpt, wnttim) = match forcing {
        HillslopeClimateDailyForcing::NoBreakpoint(day) => (
            day.day,
            day.mon,
            day.year,
            day.prcp,
            day.stmdur,
            day.tmax,
            day.tmin,
            day.rad,
            day.tdpt,
            simimpl28_winter_random_start_hour(simimpl28_day_of_year(day.day, day.mon, day.year)?),
        ),
        HillslopeClimateDailyForcing::Breakpoint(day) => (
            day.day,
            day.mon,
            day.year,
            day.prcp,
            day.stmdur,
            day.tmax,
            day.tmin,
            day.rad,
            day.tdpt,
            day.stmstr,
        ),
    };
    let winter_trigger_active = context.snow_runtime_swe_m > SIMIMPL28_DOMAIN_EPS
        || context.frost_runtime_depth_m > SIMIMPL28_DOMAIN_EPS
        || context.frost_runtime_frozen_water_m > SIMIMPL28_DOMAIN_EPS
        || context.frost_file_present
        || context.frost_wint_red_enabled
        || f64::midpoint(tmax, tmin) < 0.0;
    if !winter_trigger_active
        && export_mode == Simimpl28WinterExportMode::ProductionTrigger
    {
        return Ok(None);
    }
    if context.avg_slope <= 0.0 {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "avgslp".to_string(),
            value: context.avg_slope,
            allowed: "> 0",
        });
    }

    let sdate = simimpl28_day_of_year(day, mon, year)?;
    let geometry = simimpl28_aspect_geometry(metadata.deglat, context.avg_slope, context.azimuth)?;
    let radmj = simimpl28_langleys_to_mj_m2("rad", radly)?;
    let sunmap = simimpl28_sunmap(radly, sdate, geometry)?;
    let itflag = (tmax - tmin) <= 1.0;
    let mut hourly =
        [DirectWinterHourlyForcing::zero(); SIMIMPL28_WINTER_HOURS_PER_DAY];
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
        let partition = simimpl28_stmtim_hourly_partition_with_model(
            rain_m,
            stmdur_s,
            f64::from(hour_u32),
            wnttim,
            context.snow_rst_c,
            hrtemp_c,
            tdpt,
            context.snow_phase_model,
        )?;
        hourly[hour - 1] = DirectWinterHourlyForcing {
            rain_m: partition.hrrain_m,
            snowfall_m: partition.hrsnow_m,
            radiation_mj_m2: hrrad_mj_m2,
            air_temperature_c: hrtemp_c,
            cloud_fraction: sunmap.cloud_fraction,
            daily_solar_radiation_mj_m2: radmj,
            daily_extraterrestrial_radiation_mj_m2: sunmap.rpoth_mj_m2,
            daylight: sunmap.rpoth_mj_m2 > SIMIMPL28_DOMAIN_EPS,
            phase_model: partition.phase_model,
            rain_fraction: partition.rain_fraction,
            snow_fraction: partition.snow_fraction,
            hydrometeor_temperature_c: partition.hydrometeor_temperature_c,
            relative_humidity: partition.relative_humidity,
        };
    }
    Ok(Some(hourly))
}


fn simimpl28_hourly_symbol(root: &str, hour: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("{root}_{hour:04}"))
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

#[allow(clippy::too_many_arguments)]
fn simimpl28_sunmap_slope_radiation_langleys(
    r1: f64,
    declination: f64,
    geometry: Simimpl28AspectGeometry,
    t3: f64,
    t2_effective: f64,
    mut t6: f64,
    mut t7: f64,
    t1: f64,
    t0: f64,
) -> f64 {
    t6 += 2.0 * SIMIMPL28_PI;
    if t6 < t1 {
        let t8 = t6;
        let t9 = t1;
        return r1
            * (simimpl28_psolr(
                declination,
                geometry.delong,
                geometry.eqlat,
                t3,
                t2_effective,
            ) + simimpl28_psolr(
                declination,
                geometry.delong,
                geometry.eqlat,
                t9,
                t8,
            ));
    }

    t7 -= 2.0 * SIMIMPL28_PI;
    if t7 > t0 {
        let t8 = t0;
        let t9 = t7;
        return r1
            * (simimpl28_psolr(
                declination,
                geometry.delong,
                geometry.eqlat,
                t3,
                t2_effective,
            ) + simimpl28_psolr(
                declination,
                geometry.delong,
                geometry.eqlat,
                t9,
                t8,
            ));
    }

    r1 * simimpl28_psolr(
        declination,
        geometry.delong,
        geometry.eqlat,
        t3,
        t2_effective,
    )
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
    let t7 = t - geometry.delong;
    let t6 = -t - geometry.delong;

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
    let r4_ly =
        simimpl28_sunmap_slope_radiation_langleys(r1, d, geometry, t3, t2_effective, t6, t7, t1, t0);
    let r4 = simimpl28_langleys_to_mj_m2("sunmap.r4", r4_ly)?;

    let t4 = t2_effective * 12.0 / SIMIMPL28_PI;
    let halfdy = t4.abs();

    let r3 = r1
        * ((d.sin() * geometry.radlat.sin() * (t1 - t0) * 12.0 / SIMIMPL28_PI)
            + (d.cos() * geometry.radlat.cos() * (t1.sin() - t0.sin()) * 12.0 / SIMIMPL28_PI));
    let hourly_radiation_upper_bound_mj_m2 =
        simimpl28_hourly_extraterrestrial_radiation_upper_bound(sdate)?;
    if !r3.is_finite() || r3 <= SIMIMPL28_DOMAIN_EPS {
        if r3.is_finite() && radly <= SIMIMPL28_DOMAIN_EPS {
            return Ok(Simimpl28SunmapResult {
                halfdy: 0.0,
                dsunmp: d,
                estrad_mj_m2: 0.0,
                rpoth_mj_m2: 0.0,
                hourly_radiation_upper_bound_mj_m2,
                cloud_fraction: 0.0,
            });
        }
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "sunmap.r3".to_string(),
            value: r3,
            allowed: "> 0 and finite",
        });
    }
    if radly > r3 {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "radly".to_string(),
            value: radly,
            allowed: SIMIMPL28_DAILY_RADIATION_BOUND_ALLOWED,
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
    let (hrrad_mj_m2, hrtemp_c) = if sunmap.rpoth_mj_m2 <= SIMIMPL28_DOMAIN_EPS {
        (0.0, f64::midpoint(tmax, tmin))
    } else if itflag {
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

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn simimpl28_stmtim_hourly_partition_with_model(
    rain_m: f64,
    stmdur_s: f64,
    hour: f64,
    wnttim: f64,
    rst: f64,
    hrtemp_c: f64,
    dew_point_c: f64,
    phase_model: SnowPhasePartitionModel,
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
    let wnttim = simimpl28_stmtim_start_time(wnttim)?;
    if rain_m <= 0.0001 {
        return Ok(simimpl28_partition_result(
            0.0,
            0.0,
            0.0,
            0.0,
            phase_model,
            None,
            None,
        ));
    }
    let (wnttim, wntdur) = simimpl28_normalized_winter_precipitation_window(stmdur_s, wnttim)?;

    let active_interval = (hour >= wnttim) && (hour < (wnttim + wntdur));
    if !active_interval {
        return Ok(simimpl28_partition_result(
            0.0,
            0.0,
            0.0,
            0.0,
            phase_model,
            None,
            None,
        ));
    }

    let active_precip_m = rain_m / wntdur;
    let (rain_fraction, snow_fraction, hydrometeor_temperature_c, relative_humidity) =
        match phase_model {
            SnowPhasePartitionModel::LegacyRst => {
                if hrtemp_c > rst {
                    (1.0, 0.0, None, None)
                } else {
                    (0.0, 1.0, None, None)
                }
            }
            SnowPhasePartitionModel::HarderPomeroyHourly => {
                let air_temperature = simimpl28_temperature("winter.hourly.air_temp_c", hrtemp_c)?;
                let dew_point = simimpl28_temperature("tdpt", dew_point_c)?;
                let relative_humidity =
                    simimpl28_relative_humidity_from_dewpoint_saturated(air_temperature, dew_point)?;
                let estimate = harder_pomeroy_phase_from_relative_humidity(
                    air_temperature,
                    relative_humidity,
                    PhaseTimescale::Hourly,
                )
                .map_err(|_| ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
                    symbol: "snow.phase.harder_pomeroy_hourly".to_string(),
                    value: f64::NAN,
                    allowed: "openwepp-meteorology Harder-Pomeroy hourly solver must converge within domain",
                })?;
                (
                    estimate.fractions.rain_fraction.as_fraction(),
                    estimate.fractions.snow_fraction.as_fraction(),
                    Some(estimate.hydrometeor_temperature.temperature.as_celsius()),
                    Some(relative_humidity.as_fraction()),
                )
            }
        };

    let hrrain_m = active_precip_m * rain_fraction;
    let hrsnow_m = match phase_model {
        SnowPhasePartitionModel::LegacyRst if snow_fraction > 0.0 => {
            simimpl28_legacy_stmtim_snowfall_depth_m(rain_m, wntdur)
        }
        _ => active_precip_m * snow_fraction * 10.0,
    };
    let reconstruction_m = hrrain_m + hrsnow_m / 10.0;
    if (reconstruction_m - active_precip_m).abs() > 1.0e-12 {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "snow.hourly.stmtim.partition_reconstruction".to_string(),
            value: reconstruction_m - active_precip_m,
            allowed: "abs(hrrain + hrsnow / 10 - active_precip) <= 1e-12",
        });
    }

    Ok(simimpl28_partition_result(
        hrrain_m,
        hrsnow_m,
        rain_fraction,
        snow_fraction,
        phase_model,
        hydrometeor_temperature_c,
        relative_humidity,
    ))
}

fn simimpl28_normalized_winter_precipitation_window(
    stmdur_s: f64,
    wnttim: f64,
) -> Result<(f64, f64), ClimateRuntimeInputError> {
    let mut wnttim = simimpl28_stmtim_start_time(wnttim)?;
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
    Ok((wnttim, wntdur))
}

fn simimpl28_legacy_stmtim_snowfall_depth_m(rain_m: f64, wntdur: f64) -> f64 {
    rain_m / wntdur * 10.0
}

#[allow(clippy::too_many_arguments)]
const fn simimpl28_partition_result(
    hrrain_m: f64,
    hrsnow_m: f64,
    rain_fraction: f64,
    snow_fraction: f64,
    phase_model: SnowPhasePartitionModel,
    hydrometeor_temperature_c: Option<f64>,
    relative_humidity: Option<f64>,
) -> Simimpl28StmtimHourlyPartition {
    Simimpl28StmtimHourlyPartition {
        hrrain_m,
        hrsnow_m,
        rain_fraction,
        snow_fraction,
        phase_model,
        hydrometeor_temperature_c,
        relative_humidity,
    }
}

fn simimpl28_temperature(
    symbol: &'static str,
    value_c: f64,
) -> Result<TemperatureCelsius, ClimateRuntimeInputError> {
    TemperatureCelsius::try_new(value_c).map_err(|error| match error {
        BoundaryError::NonFinite { value, .. } => ClimateRuntimeInputError::NonFiniteField {
            field: symbol,
            value,
        },
        BoundaryError::BelowMinimum { value, .. } | BoundaryError::AboveMaximum { value, .. } => {
            ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
                symbol: symbol.to_string(),
                value,
                allowed: "finite temperature",
            }
        }
    })
}

fn simimpl28_relative_humidity_from_dewpoint_saturated(
    air_temperature: TemperatureCelsius,
    dew_point: TemperatureCelsius,
) -> Result<FractionUnitInterval, ClimateRuntimeInputError> {
    let actual = saturation_vapor_pressure_water_kpa(dew_point).map_err(|_| {
        ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "tdpt".to_string(),
            value: dew_point.as_celsius(),
            allowed: "finite dew point with valid water saturation vapor pressure",
        }
    })?;
    let saturation = saturation_vapor_pressure_water_kpa(air_temperature).map_err(|_| {
        ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "winter.hourly.air_temp_c".to_string(),
            value: air_temperature.as_celsius(),
            allowed: "finite air temperature with valid water saturation vapor pressure",
        }
    })?;
    let saturation_kpa = saturation.as_kilopascals();
    if saturation_kpa <= 0.0 {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "snow.phase.relative_humidity.saturation_vapor_pressure".to_string(),
            value: saturation_kpa,
            allowed: "> 0",
        });
    }
    let ratio = actual.as_kilopascals() / saturation_kpa;
    if !ratio.is_finite() || ratio < 0.0 {
        return Err(ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
            symbol: "snow.phase.relative_humidity".to_string(),
            value: ratio,
            allowed: "finite and >= 0",
        });
    }
    let normalized = ratio.min(1.0);
    FractionUnitInterval::try_new(normalized).map_err(|error| match error {
        BoundaryError::NonFinite { value, .. } => ClimateRuntimeInputError::NonFiniteField {
            field: "snow.phase.relative_humidity",
            value,
        },
        BoundaryError::BelowMinimum { value, .. } | BoundaryError::AboveMaximum { value, .. } => {
            ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange {
                symbol: "snow.phase.relative_humidity".to_string(),
                value,
                allowed: "0..=1 after exact-saturation normalization",
            }
        }
    })
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

#[cfg(test)]
mod cqr_row4_simimpl28_hourly_forcing_tests {
    use super::*;
    use openwepp_climate_runtime_adapter::{
        SharedBreakpointForcing, SharedNoBreakpointForcing,
    };

    fn metadata() -> ClimateMetadata {
        ClimateMetadata {
            deglat: 45.0,
            deglon: -116.0,
            elev: 1_500.0,
            obsyrs: 1,
            ibyear: 2020,
            numyr: 1,
            generator_cmd: None,
        }
    }

    fn no_breakpoint_forcing(tmax: f64, tmin: f64) -> HillslopeClimateDailyForcing {
        HillslopeClimateDailyForcing::NoBreakpoint(SharedNoBreakpointForcing {
            day: 15,
            mon: 1,
            year: 2020,
            prcp: 0.012,
            stmdur: 3_600.0,
            timep: 0.5,
            ip: 10.0,
            ninten: 1,
            avrint: 10.0,
            mxint: 10.0,
            timem: vec![0.0],
            intsty: vec![10.0],
            tmax,
            tmin,
            rad: 180.0,
            vwind: 1.0,
            wind: 1.0,
            tdpt: tmin,
        })
    }

    fn breakpoint_forcing() -> HillslopeClimateDailyForcing {
        HillslopeClimateDailyForcing::Breakpoint(SharedBreakpointForcing {
            day: 15,
            mon: 1,
            year: 2020,
            nbrkpt: 1,
            stmstr: 1.0,
            prcp: 0.012,
            stmdur: 3_600.0,
            mxint: 10.0,
            timem: vec![0.0],
            intsty: vec![10.0],
            tmax: -1.0,
            tmin: -5.0,
            rad: 180.0,
            vwind: 1.0,
            wind: 1.0,
            tdpt: -6.0,
        })
    }

    fn warm_context() -> DirectWinterHourlyContext {
        DirectWinterHourlyContext {
            snow_runtime_swe_m: 0.0,
            frost_runtime_depth_m: 0.0,
            frost_runtime_frozen_water_m: 0.0,
            frost_file_present: false,
            frost_wint_red_enabled: false,
            avg_slope: 0.1,
            azimuth: 180.0,
            snow_rst_c: 0.0,
            snow_phase_model: SnowPhasePartitionModel::LegacyRst,
        }
    }

    fn assert_exact_f64(actual: f64, expected: f64) {
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "expected exact branch constant {expected}, got {actual}"
        );
    }

    #[test]
    fn cqr_row4_build_forcing_suppresses_production_and_forces_diagnostic_rows() {
        let forcing = no_breakpoint_forcing(10.0, 4.0);
        let suppressed = build_simimpl28_hourly_winter_forcing_typed(
            &forcing,
            &metadata(),
            warm_context(),
            Simimpl28WinterExportMode::ProductionTrigger,
        )
        .expect("warm no-snow production day should evaluate trigger");
        assert!(suppressed.is_none());

        let diagnostic = build_simimpl28_hourly_winter_forcing_typed(
            &forcing,
            &metadata(),
            warm_context(),
            Simimpl28WinterExportMode::ForceCompleteDiagnosticRows,
        )
        .expect("forced diagnostic rows should build")
        .expect("forced diagnostic mode should return rows");
        assert_eq!(diagnostic.len(), SIMIMPL28_WINTER_HOURS_PER_DAY);
        assert!(diagnostic.iter().all(|row| row.radiation_mj_m2 >= 0.0));
    }

    #[test]
    fn cqr_row4_build_forcing_covers_context_guards_and_breakpoint_trigger() {
        let forcing = no_breakpoint_forcing(2.0, -2.0);
        for (mut context, symbol) in [
            (
                DirectWinterHourlyContext {
                    snow_runtime_swe_m: -1.0,
                    ..warm_context()
                },
                "snow.runtime_swe",
            ),
            (
                DirectWinterHourlyContext {
                    frost_runtime_depth_m: -1.0,
                    ..warm_context()
                },
                "frost.runtime_dfrost",
            ),
            (
                DirectWinterHourlyContext {
                    frost_runtime_frozen_water_m: -1.0,
                    ..warm_context()
                },
                "frost.runtime_ws_frz",
            ),
            (
                DirectWinterHourlyContext {
                    avg_slope: 0.0,
                    ..warm_context()
                },
                "avgslp",
            ),
        ] {
            context.frost_file_present = true;
            let error = build_simimpl28_hourly_winter_forcing_typed(
                &forcing,
                &metadata(),
                context,
                Simimpl28WinterExportMode::ForceCompleteDiagnosticRows,
            )
            .expect_err("invalid context should fail closed");
            assert!(format!("{error:?}").contains(symbol));
        }

        let rows = build_simimpl28_hourly_winter_forcing_typed(
            &breakpoint_forcing(),
            &metadata(),
            warm_context(),
            Simimpl28WinterExportMode::ProductionTrigger,
        )
        .expect("cold breakpoint day should build")
        .expect("cold breakpoint day should trigger production rows");
        assert!(rows.iter().any(|row| row.snowfall_m > 0.0));
    }

    #[test]
    fn cqr_row4_sunmap_covers_success_and_domain_errors() {
        let geometry =
            simimpl28_aspect_geometry(45.0, 0.1, 180.0).expect("geometry should be valid");
        let sunmap = simimpl28_sunmap(180.0, 15, geometry).expect("sunmap should build");
        assert!(sunmap.halfdy.is_finite());
        assert!(sunmap.rpoth_mj_m2 > 0.0);
        assert!((0.0..=1.0).contains(&sunmap.cloud_fraction));

        let error = simimpl28_sunmap(f64::NAN, 15, geometry)
            .expect_err("non-finite radiation should fail closed");
        assert!(format!("{error:?}").contains("radly"));

        let error = simimpl28_sunmap(100_000.0, 15, geometry)
            .expect_err("excess daily radiation should fail closed");
        assert!(format!("{error:?}").contains("radly"));
    }

    #[test]
    fn eb03_polar_night_publishes_explicit_unavailable_solar_state() {
        let geometry =
            simimpl28_aspect_geometry(89.0, 0.1, 180.0).expect("geometry should be valid");
        let sunmap = simimpl28_sunmap(0.0, 355, geometry).expect("polar night is a valid state");
        assert!(sunmap.rpoth_mj_m2.abs() <= f64::EPSILON);
        assert!(sunmap.estrad_mj_m2.abs() <= f64::EPSILON);
        assert!(sunmap.halfdy.abs() <= f64::EPSILON);
    }

    #[test]
    fn cqr_row4_stmtim_partition_covers_legacy_and_harder_pomeroy_branches() {
        let zero = simimpl28_stmtim_hourly_partition_with_model(
            0.0,
            3_600.0,
            1.0,
            1.0,
            0.0,
            -2.0,
            -3.0,
            SnowPhasePartitionModel::LegacyRst,
        )
        .expect("zero precipitation should partition");
        assert_exact_f64(zero.hrrain_m, 0.0);
        assert_exact_f64(zero.hrsnow_m, 0.0);

        let inactive = simimpl28_stmtim_hourly_partition_with_model(
            0.012,
            3_600.0,
            4.0,
            1.0,
            0.0,
            -2.0,
            -3.0,
            SnowPhasePartitionModel::LegacyRst,
        )
        .expect("inactive hour should partition");
        assert_exact_f64(inactive.hrrain_m, 0.0);
        assert_exact_f64(inactive.hrsnow_m, 0.0);

        let legacy_snow = simimpl28_stmtim_hourly_partition_with_model(
            0.012,
            3_600.0,
            1.0,
            1.0,
            0.0,
            -2.0,
            -3.0,
            SnowPhasePartitionModel::LegacyRst,
        )
        .expect("cold legacy hour should partition as snow");
        assert_exact_f64(legacy_snow.rain_fraction, 0.0);
        assert_exact_f64(legacy_snow.snow_fraction, 1.0);
        assert!(legacy_snow.hrsnow_m > 0.0);

        let legacy_rain = simimpl28_stmtim_hourly_partition_with_model(
            0.012,
            3_600.0,
            1.0,
            1.0,
            0.0,
            2.0,
            -1.0,
            SnowPhasePartitionModel::LegacyRst,
        )
        .expect("warm legacy hour should partition as rain");
        assert_exact_f64(legacy_rain.rain_fraction, 1.0);
        assert_exact_f64(legacy_rain.snow_fraction, 0.0);
        assert!(legacy_rain.hrrain_m > 0.0);

        let harder_pomeroy = simimpl28_stmtim_hourly_partition_with_model(
            0.012,
            3_600.0,
            1.0,
            1.0,
            0.0,
            -1.0,
            -2.0,
            SnowPhasePartitionModel::HarderPomeroyHourly,
        )
        .expect("Harder-Pomeroy hourly branch should partition");
        assert_eq!(
            harder_pomeroy.phase_model,
            SnowPhasePartitionModel::HarderPomeroyHourly
        );
        assert!(harder_pomeroy.hydrometeor_temperature_c.is_some());
        assert!(harder_pomeroy.relative_humidity.is_some());
    }
}
