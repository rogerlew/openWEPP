#![allow(clippy::missing_errors_doc, clippy::too_many_lines)]

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

const ALLOWED_DATVERS_EXACT: [f64; 3] = [0.0, 4.0, 4.3];
const DATVER_53_FAMILY_MIN: f64 = 5.3;
const DATVER_53_FAMILY_MAX_EXCLUSIVE: f64 = 5.4;
const MAX_BREAKPOINTS_PER_DAY: usize = 1_500;
const FLOAT_EQ_TOLERANCE: f64 = 1e-9;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserMode {
    Strict,
    Compatibility(CompatibilityOptions),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CompatibilityOptions {
    pub allow_single_storm: bool,
    pub allow_breakpoint_cardinality_override: bool,
    pub allow_legacy_zero_drain_non_positive_dtime: bool,
}

impl ParserMode {
    fn allow_single_storm(self) -> bool {
        match self {
            Self::Strict => false,
            Self::Compatibility(options) => options.allow_single_storm,
        }
    }

    fn allow_breakpoint_cardinality_override(self) -> bool {
        match self {
            Self::Strict => false,
            Self::Compatibility(options) => options.allow_breakpoint_cardinality_override,
        }
    }

    fn allow_legacy_zero_drain_non_positive_dtime(self) -> bool {
        match self {
            Self::Strict => false,
            Self::Compatibility(options) => options.allow_legacy_zero_drain_non_positive_dtime,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClimateFile {
    pub datver: f64,
    pub mode: ClimateModeFlags,
    pub station_id: String,
    pub metadata: ClimateMetadata,
    pub monthly: ClimateMonthlyStats,
    pub daily_records: Vec<ClimateDailyRecord>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClimateModeFlags {
    pub itemp: i32,
    pub breakpoint_enabled: bool,
    pub iwind: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClimateMetadata {
    pub deglat: f64,
    pub deglon: f64,
    pub elev: f64,
    pub obsyrs: i32,
    pub ibyear: i32,
    pub numyr: i32,
    pub generator_cmd: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClimateMonthlyStats {
    pub obmaxt: [f64; 12],
    pub obmint: [f64; 12],
    pub radave: [f64; 12],
    pub obrain: [f64; 12],
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClimateDailyRecord {
    NoBreakpoint(NoBreakpointDay),
    Breakpoint(BreakpointDay),
}

impl ClimateDailyRecord {
    fn year(&self) -> i32 {
        match self {
            Self::NoBreakpoint(day) => day.year,
            Self::Breakpoint(day) => day.year,
        }
    }

    fn month(&self) -> i32 {
        match self {
            Self::NoBreakpoint(day) => day.mon,
            Self::Breakpoint(day) => day.mon,
        }
    }

    fn day(&self) -> i32 {
        match self {
            Self::NoBreakpoint(day) => day.day,
            Self::Breakpoint(day) => day.day,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoBreakpointDay {
    pub day: i32,
    pub mon: i32,
    pub year: i32,
    pub prcp: f64,
    pub stmdur: f64,
    pub timep: f64,
    pub ip: f64,
    pub tmax: f64,
    pub tmin: f64,
    pub rad: f64,
    pub vwind: f64,
    pub wind: f64,
    pub tdpt: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakpointDay {
    pub day: i32,
    pub mon: i32,
    pub year: i32,
    pub nbrkpt: usize,
    pub tmax: f64,
    pub tmin: f64,
    pub rad: f64,
    pub vwind: f64,
    pub wind: f64,
    pub tdpt: f64,
    pub breakpoints: Vec<BreakpointPoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakpointPoint {
    pub timem: f64,
    pub pptcum: f64,
}

#[derive(Debug)]
pub enum ClimateParseError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    UnexpectedEof {
        context: &'static str,
    },
    RecordArity {
        line: usize,
        context: &'static str,
        expected: usize,
        found: usize,
    },
    TokenParse {
        line: usize,
        field: &'static str,
        token: String,
    },
    UnsupportedDatver {
        line: usize,
        value: f64,
    },
    EnumDomain {
        line: usize,
        field: &'static str,
        value: i32,
    },
    SingleStormUnsupported {
        line: usize,
    },
    FieldRange {
        line: usize,
        field: &'static str,
        value: f64,
    },
    DateDomain {
        line: usize,
        day: i32,
        month: i32,
        year: i32,
    },
    BreakpointCardinality {
        line: usize,
        nbrkpt: usize,
        max: usize,
    },
    BreakpointMonotonicity {
        line: usize,
        previous: f64,
        current: f64,
    },
    BreakpointTimeMonotonicity {
        line: usize,
        previous: f64,
        current: f64,
    },
    RecordCount {
        context: &'static str,
        expected: usize,
        found: usize,
    },
    InvariantViolation {
        line: usize,
        context: &'static str,
    },
}

impl Display for ClimateParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.write_display(f)
    }
}

impl ClimateParseError {
    fn write_display(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(
                    f,
                    "failed to read climate file '{}': {source}",
                    path.display()
                )
            }
            Self::UnexpectedEof { context } => {
                write!(f, "unexpected end of file while parsing {context}")
            }
            Self::RecordArity {
                line,
                context,
                expected,
                found,
            } => {
                write!(
                    f,
                    "line {line}: {context} expects {expected} token(s), found {found}"
                )
            }
            Self::TokenParse { line, field, token } => {
                write!(
                    f,
                    "line {line}: failed to parse field '{field}' from token '{token}'"
                )
            }
            Self::UnsupportedDatver { line, value } => {
                write!(f, "line {line}: unsupported datver '{value}'")
            }
            Self::EnumDomain { line, field, value } => {
                write!(
                    f,
                    "line {line}: value '{value}' is out of domain for '{field}'"
                )
            }
            Self::SingleStormUnsupported { line } => {
                write!(f, "line {line}: single-storm mode (itemp=2) is unsupported")
            }
            Self::FieldRange { line, field, value } => {
                write!(
                    f,
                    "line {line}: value '{value}' violates range for '{field}'"
                )
            }
            Self::DateDomain {
                line,
                day,
                month,
                year,
            } => {
                write!(
                    f,
                    "line {line}: invalid date tuple ({day}, {month}, {year})"
                )
            }
            Self::BreakpointCardinality { line, nbrkpt, max } => {
                write!(
                    f,
                    "line {line}: breakpoint count '{nbrkpt}' exceeds policy max '{max}'"
                )
            }
            Self::BreakpointMonotonicity {
                line,
                previous,
                current,
            } => {
                write!(
                    f,
                    "line {line}: cumulative breakpoint precipitation must be monotone: previous={previous}, current={current}"
                )
            }
            Self::BreakpointTimeMonotonicity {
                line,
                previous,
                current,
            } => {
                write!(
                    f,
                    "line {line}: breakpoint timem must be strictly increasing: previous={previous}, current={current}"
                )
            }
            Self::RecordCount {
                context,
                expected,
                found,
            } => {
                write!(f, "{context}: expected {expected}, found {found}")
            }
            Self::InvariantViolation { line, context } => {
                write!(f, "line {line}: invariant violation for {context}")
            }
        }
    }
}

impl Error for ClimateParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
struct LocatedLine<'a> {
    number: usize,
    text: &'a str,
}

pub fn parse_climate_file(
    path: impl AsRef<Path>,
    mode: ParserMode,
) -> Result<ClimateFile, ClimateParseError> {
    let path = path.as_ref();
    let content = fs::read_to_string(path).map_err(|source| ClimateParseError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    parse_climate_from_str(&content, mode)
}

pub fn parse_climate_from_str(
    input: &str,
    mode: ParserMode,
) -> Result<ClimateFile, ClimateParseError> {
    let lines = collect_non_empty_lines(input);
    let mut cursor = 0usize;

    let datver_line = require_line(&lines, &mut cursor, "datver line")?;
    let datver_tokens = tokenize(datver_line.text);
    expect_arity_exact(&datver_tokens, datver_line.number, "datver line", 1)?;
    let datver_raw = parse_f64(datver_tokens[0], datver_line.number, "datver")?;
    let Some(datver) = canonicalize_datver(datver_raw) else {
        return Err(ClimateParseError::UnsupportedDatver {
            line: datver_line.number,
            value: datver_raw,
        });
    };

    let flags_line = require_line(&lines, &mut cursor, "flags line")?;
    let flags_tokens = tokenize(flags_line.text);
    expect_arity_exact(&flags_tokens, flags_line.number, "flags line", 3)?;
    let itemp = parse_i32(flags_tokens[0], flags_line.number, "itemp")?;
    let ibrkpt = parse_i32(flags_tokens[1], flags_line.number, "ibrkpt")?;
    let iwind = parse_i32(flags_tokens[2], flags_line.number, "iwind")?;

    if itemp != 1 && itemp != 2 {
        return Err(ClimateParseError::EnumDomain {
            line: flags_line.number,
            field: "itemp",
            value: itemp,
        });
    }
    if itemp == 2 && !mode.allow_single_storm() {
        return Err(ClimateParseError::SingleStormUnsupported {
            line: flags_line.number,
        });
    }
    if ibrkpt != 0 && ibrkpt != 1 {
        return Err(ClimateParseError::EnumDomain {
            line: flags_line.number,
            field: "ibrkpt",
            value: ibrkpt,
        });
    }
    if iwind != 0 && iwind != 1 {
        return Err(ClimateParseError::EnumDomain {
            line: flags_line.number,
            field: "iwind",
            value: iwind,
        });
    }

    let station_line = require_line(&lines, &mut cursor, "station line")?;
    let station_id = station_line.text.to_string();

    require_line(&lines, &mut cursor, "line-4 variable headers")?;

    let metadata_line = require_line(&lines, &mut cursor, "metadata line")?;
    let metadata_tokens = tokenize(metadata_line.text);
    if metadata_tokens.len() < 6 {
        return Err(ClimateParseError::RecordArity {
            line: metadata_line.number,
            context: "metadata line",
            expected: 6,
            found: metadata_tokens.len(),
        });
    }
    let deglat = parse_f64(metadata_tokens[0], metadata_line.number, "deglat")?;
    let deglon = parse_f64(metadata_tokens[1], metadata_line.number, "deglon")?;
    let elev = parse_f64(metadata_tokens[2], metadata_line.number, "elev")?;
    let obsyrs = parse_i32(metadata_tokens[3], metadata_line.number, "obsyrs")?;
    let ibyear = parse_i32(metadata_tokens[4], metadata_line.number, "ibyear")?;
    let numyr = parse_i32(metadata_tokens[5], metadata_line.number, "numyr")?;
    if numyr <= 0 {
        return Err(ClimateParseError::FieldRange {
            line: metadata_line.number,
            field: "numyr",
            value: f64::from(numyr),
        });
    }
    let generator_cmd = (metadata_tokens.len() > 6).then(|| metadata_tokens[6..].join(" "));
    let metadata = ClimateMetadata {
        deglat,
        deglon,
        elev,
        obsyrs,
        ibyear,
        numyr,
        generator_cmd,
    };

    let obmaxt = parse_monthly_vector(
        &lines,
        &mut cursor,
        "monthly max temperature header",
        "monthly max temperature vector",
    )?;
    let obmint = parse_monthly_vector(
        &lines,
        &mut cursor,
        "monthly min temperature header",
        "monthly min temperature vector",
    )?;
    let radave = parse_monthly_vector(
        &lines,
        &mut cursor,
        "monthly radiation header",
        "monthly radiation vector",
    )?;
    let obrain = parse_monthly_vector(
        &lines,
        &mut cursor,
        "monthly precipitation header",
        "monthly precipitation vector",
    )?;

    require_line(&lines, &mut cursor, "daily variable names header")?;
    require_line(&lines, &mut cursor, "daily variable units header")?;

    let mode_flags = ClimateModeFlags {
        itemp,
        breakpoint_enabled: ibrkpt == 1,
        iwind,
    };

    let mut daily_records = Vec::new();
    while cursor < lines.len() {
        let day_record = if mode_flags.breakpoint_enabled {
            parse_breakpoint_day(&lines, &mut cursor, mode)?
        } else {
            parse_no_breakpoint_day(&lines, &mut cursor)?
        };
        daily_records.push(day_record);
    }

    if daily_records.is_empty() {
        return Err(ClimateParseError::RecordCount {
            context: "daily records",
            expected: 1,
            found: 0,
        });
    }

    validate_daily_sequence(&daily_records)?;
    validate_year_coverage(&daily_records, &metadata)?;

    Ok(ClimateFile {
        datver,
        mode: mode_flags,
        station_id,
        metadata,
        monthly: ClimateMonthlyStats {
            obmaxt,
            obmint,
            radave,
            obrain,
        },
        daily_records,
    })
}

fn canonicalize_datver(datver_raw: f64) -> Option<f64> {
    for allowed in ALLOWED_DATVERS_EXACT {
        if (datver_raw - allowed).abs() <= FLOAT_EQ_TOLERANCE {
            return Some(allowed);
        }
    }
    if (datver_raw - DATVER_53_FAMILY_MIN).abs() <= FLOAT_EQ_TOLERANCE
        || (datver_raw > DATVER_53_FAMILY_MIN && datver_raw < DATVER_53_FAMILY_MAX_EXCLUSIVE)
    {
        return Some(DATVER_53_FAMILY_MIN);
    }
    None
}

fn collect_non_empty_lines(input: &str) -> Vec<LocatedLine<'_>> {
    input
        .lines()
        .enumerate()
        .filter_map(|(index, raw_line)| {
            let trimmed = raw_line.trim();
            (!trimmed.is_empty()).then_some(LocatedLine {
                number: index + 1,
                text: trimmed,
            })
        })
        .collect()
}

fn require_line<'a>(
    lines: &'a [LocatedLine<'a>],
    cursor: &mut usize,
    context: &'static str,
) -> Result<LocatedLine<'a>, ClimateParseError> {
    let line = lines
        .get(*cursor)
        .copied()
        .ok_or(ClimateParseError::UnexpectedEof { context })?;
    *cursor += 1;
    Ok(line)
}

fn parse_monthly_vector(
    lines: &[LocatedLine<'_>],
    cursor: &mut usize,
    header_context: &'static str,
    values_context: &'static str,
) -> Result<[f64; 12], ClimateParseError> {
    require_line(lines, cursor, header_context)?;
    let values_line = require_line(lines, cursor, values_context)?;
    let tokens = tokenize(values_line.text);
    expect_arity_exact(&tokens, values_line.number, values_context, 12)?;

    let mut values = [0.0; 12];
    for (index, token) in tokens.iter().enumerate() {
        values[index] = parse_f64(token, values_line.number, values_context)?;
    }
    Ok(values)
}

fn parse_no_breakpoint_day(
    lines: &[LocatedLine<'_>],
    cursor: &mut usize,
) -> Result<ClimateDailyRecord, ClimateParseError> {
    let line = require_line(lines, cursor, "daily no-breakpoint record")?;
    let tokens = tokenize(line.text);
    expect_arity_exact(&tokens, line.number, "daily no-breakpoint record", 13)?;

    let day = parse_i32(tokens[0], line.number, "day")?;
    let mon = parse_i32(tokens[1], line.number, "mon")?;
    let year = parse_i32(tokens[2], line.number, "year")?;
    validate_date(day, mon, year, line.number)?;

    let prcp = parse_f64(tokens[3], line.number, "prcp")?;
    let stmdur = parse_f64(tokens[4], line.number, "stmdur")?;
    let timep = parse_f64(tokens[5], line.number, "timep")?;
    let ip = parse_f64(tokens[6], line.number, "ip")?;
    let tmax = parse_f64(tokens[7], line.number, "tmax")?;
    let tmin = parse_f64(tokens[8], line.number, "tmin")?;
    let rad = parse_f64(tokens[9], line.number, "rad")?;
    let vwind = parse_f64(tokens[10], line.number, "vwind")?;
    let wind = parse_f64(tokens[11], line.number, "wind")?;
    let tdpt = parse_f64(tokens[12], line.number, "tdpt")?;

    if prcp < 0.0 {
        return Err(ClimateParseError::FieldRange {
            line: line.number,
            field: "prcp",
            value: prcp,
        });
    }
    if stmdur < 0.0 {
        return Err(ClimateParseError::FieldRange {
            line: line.number,
            field: "stmdur",
            value: stmdur,
        });
    }
    if !(0.0..=1.0).contains(&timep) {
        return Err(ClimateParseError::FieldRange {
            line: line.number,
            field: "timep",
            value: timep,
        });
    }
    if ip < 0.0 {
        return Err(ClimateParseError::FieldRange {
            line: line.number,
            field: "ip",
            value: ip,
        });
    }

    Ok(ClimateDailyRecord::NoBreakpoint(NoBreakpointDay {
        day,
        mon,
        year,
        prcp,
        stmdur,
        timep,
        ip,
        tmax,
        tmin,
        rad,
        vwind,
        wind,
        tdpt,
    }))
}

fn parse_breakpoint_day(
    lines: &[LocatedLine<'_>],
    cursor: &mut usize,
    mode: ParserMode,
) -> Result<ClimateDailyRecord, ClimateParseError> {
    let line = require_line(lines, cursor, "daily breakpoint record")?;
    let tokens = tokenize(line.text);
    expect_arity_exact(&tokens, line.number, "daily breakpoint record", 10)?;

    let day = parse_i32(tokens[0], line.number, "day")?;
    let mon = parse_i32(tokens[1], line.number, "mon")?;
    let year = parse_i32(tokens[2], line.number, "year")?;
    validate_date(day, mon, year, line.number)?;

    let nbrkpt_raw = parse_i32(tokens[3], line.number, "nbrkpt")?;
    if nbrkpt_raw < 0 {
        return Err(ClimateParseError::FieldRange {
            line: line.number,
            field: "nbrkpt",
            value: f64::from(nbrkpt_raw),
        });
    }
    let nbrkpt = usize::try_from(nbrkpt_raw).map_err(|_| ClimateParseError::FieldRange {
        line: line.number,
        field: "nbrkpt",
        value: f64::from(nbrkpt_raw),
    })?;
    if nbrkpt > MAX_BREAKPOINTS_PER_DAY && !mode.allow_breakpoint_cardinality_override() {
        return Err(ClimateParseError::BreakpointCardinality {
            line: line.number,
            nbrkpt,
            max: MAX_BREAKPOINTS_PER_DAY,
        });
    }

    let tmax = parse_f64(tokens[4], line.number, "tmax")?;
    let tmin = parse_f64(tokens[5], line.number, "tmin")?;
    let rad = parse_f64(tokens[6], line.number, "rad")?;
    let vwind = parse_f64(tokens[7], line.number, "vwind")?;
    let wind = parse_f64(tokens[8], line.number, "wind")?;
    let tdpt = parse_f64(tokens[9], line.number, "tdpt")?;

    let mut breakpoints = Vec::with_capacity(nbrkpt);
    let mut previous_timem: Option<f64> = None;
    let mut previous_pptcum = 0.0;
    for index in 0..nbrkpt {
        let point_line = require_line(lines, cursor, "breakpoint pair record")?;
        let point_tokens = tokenize(point_line.text);
        expect_arity_exact(
            &point_tokens,
            point_line.number,
            "breakpoint pair record",
            2,
        )?;

        let timem = parse_f64(point_tokens[0], point_line.number, "timem")?;
        let pptcum = parse_f64(point_tokens[1], point_line.number, "pptcum")?;
        if !(0.0..=24.0).contains(&timem) {
            return Err(ClimateParseError::FieldRange {
                line: point_line.number,
                field: "timem",
                value: timem,
            });
        }
        if pptcum < 0.0 {
            return Err(ClimateParseError::FieldRange {
                line: point_line.number,
                field: "pptcum",
                value: pptcum,
            });
        }
        if let Some(previous_timem_value) = previous_timem {
            let delta_time_h = timem - previous_timem_value;
            if delta_time_h <= FLOAT_EQ_TOLERANCE {
                let drain = pptcum - previous_pptcum;
                let allow_legacy_bug = mode.allow_legacy_zero_drain_non_positive_dtime()
                    && drain.abs() <= FLOAT_EQ_TOLERANCE;
                if !allow_legacy_bug {
                    return Err(ClimateParseError::BreakpointTimeMonotonicity {
                        line: point_line.number,
                        previous: previous_timem_value,
                        current: timem,
                    });
                }
            }
        }
        if index > 0 && pptcum + FLOAT_EQ_TOLERANCE < previous_pptcum {
            return Err(ClimateParseError::BreakpointMonotonicity {
                line: point_line.number,
                previous: previous_pptcum,
                current: pptcum,
            });
        }

        previous_timem = Some(timem);
        previous_pptcum = pptcum;
        breakpoints.push(BreakpointPoint { timem, pptcum });
    }

    Ok(ClimateDailyRecord::Breakpoint(BreakpointDay {
        day,
        mon,
        year,
        nbrkpt,
        tmax,
        tmin,
        rad,
        vwind,
        wind,
        tdpt,
        breakpoints,
    }))
}

fn validate_date(day: i32, month: i32, year: i32, line: usize) -> Result<(), ClimateParseError> {
    if year <= 0 || !(1..=12).contains(&month) {
        return Err(ClimateParseError::DateDomain {
            line,
            day,
            month,
            year,
        });
    }
    let max_day = days_in_month(year, month);
    if day <= 0 || day > max_day {
        return Err(ClimateParseError::DateDomain {
            line,
            day,
            month,
            year,
        });
    }
    Ok(())
}

fn validate_daily_sequence(records: &[ClimateDailyRecord]) -> Result<(), ClimateParseError> {
    for window in records.windows(2) {
        let previous = &window[0];
        let current = &window[1];
        let previous_tuple = (previous.year(), previous.month(), previous.day());
        let current_tuple = (current.year(), current.month(), current.day());
        if current_tuple <= previous_tuple {
            return Err(ClimateParseError::InvariantViolation {
                line: 0,
                context: "daily date sequence must be strictly increasing",
            });
        }
    }
    Ok(())
}

fn validate_year_coverage(
    records: &[ClimateDailyRecord],
    metadata: &ClimateMetadata,
) -> Result<(), ClimateParseError> {
    let first_year = records
        .first()
        .map(ClimateDailyRecord::year)
        .unwrap_or_default();
    let last_year = records
        .last()
        .map(ClimateDailyRecord::year)
        .unwrap_or_default();

    let expected_start = metadata.ibyear;
    let expected_end = metadata.ibyear + metadata.numyr - 1;
    if first_year < expected_start || last_year > expected_end {
        let expected_span = usize::try_from(metadata.numyr).unwrap_or_default();
        let found_span = usize::try_from(last_year - first_year + 1).unwrap_or_default();
        return Err(ClimateParseError::RecordCount {
            context: "daily year span must fit [ibyear, ibyear+numyr-1]",
            expected: expected_span,
            found: found_span,
        });
    }

    Ok(())
}

fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn tokenize(line: &str) -> Vec<&str> {
    line.split(|c: char| c == ',' || c.is_whitespace())
        .filter(|token| !token.is_empty())
        .collect()
}

fn expect_arity_exact(
    tokens: &[&str],
    line: usize,
    context: &'static str,
    expected: usize,
) -> Result<(), ClimateParseError> {
    if tokens.len() != expected {
        return Err(ClimateParseError::RecordArity {
            line,
            context,
            expected,
            found: tokens.len(),
        });
    }
    Ok(())
}

fn parse_i32(token: &str, line: usize, field: &'static str) -> Result<i32, ClimateParseError> {
    token
        .parse::<i32>()
        .map_err(|_| ClimateParseError::TokenParse {
            line,
            field,
            token: token.to_string(),
        })
}

fn parse_f64(token: &str, line: usize, field: &'static str) -> Result<f64, ClimateParseError> {
    token
        .parse::<f64>()
        .map_err(|_| ClimateParseError::TokenParse {
            line,
            field,
            token: token.to_string(),
        })
}
