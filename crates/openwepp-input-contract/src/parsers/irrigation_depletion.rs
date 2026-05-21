#![allow(clippy::missing_errors_doc, clippy::too_many_lines)]

use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const DATVER_CANONICAL: f64 = 95.7;
const DATVER_SPRINKLER_COMPAT_MIN: f64 = 94.21;
const DATVER_FURROW_COMPAT_MIN: f64 = 91.5;
const FLOAT_TOLERANCE: f64 = 1e-9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrrigationSystemType {
    Sprinkler,
    Furrow,
}

impl IrrigationSystemType {
    fn from_raw(raw: i32) -> Result<Self, &'static str> {
        match raw {
            1 => Ok(Self::Sprinkler),
            2 => Ok(Self::Furrow),
            _ => Err("expected 1 (sprinkler) or 2 (furrow)"),
        }
    }

    fn raw(self) -> i32 {
        match self {
            Self::Sprinkler => 1,
            Self::Furrow => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZeroStartTransition {
    None,
    ThreeToTwo,
    OneToZero,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrrigationDepletionWarningCode {
    IrdW001,
    IrdW002,
    IrdW003,
    IrdW004,
    IrdW005,
    IrdW006,
}

impl IrrigationDepletionWarningCode {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IrdW001 => "IRD-W-001",
            Self::IrdW002 => "IRD-W-002",
            Self::IrdW003 => "IRD-W-003",
            Self::IrdW004 => "IRD-W-004",
            Self::IrdW005 => "IRD-W-005",
            Self::IrdW006 => "IRD-W-006",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrrigationDepletionWarning {
    pub code: IrrigationDepletionWarningCode,
    pub line: usize,
    pub message: String,
}

impl IrrigationDepletionWarning {
    fn new(code: IrrigationDepletionWarningCode, line: usize, message: impl Into<String>) -> Self {
        Self {
            code,
            line,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IrrigationDepletionParserOptions {
    pub mode: ParseMode,
    pub expected_element_count: Option<usize>,
    pub expected_system_type: Option<IrrigationSystemType>,
    pub expected_irrigation_option: Option<i32>,
    pub furrow_disallowed_context: bool,
    pub irschd_on_entry: Option<i32>,
    pub enforce_continuation_order: bool,
}

impl IrrigationDepletionParserOptions {
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            mode: ParseMode::Strict,
            expected_element_count: None,
            expected_system_type: None,
            expected_irrigation_option: None,
            furrow_disallowed_context: false,
            irschd_on_entry: None,
            enforce_continuation_order: true,
        }
    }

    #[must_use]
    pub const fn compatibility() -> Self {
        Self {
            mode: ParseMode::Compatibility,
            expected_element_count: None,
            expected_system_type: None,
            expected_irrigation_option: None,
            furrow_disallowed_context: false,
            irschd_on_entry: None,
            enforce_continuation_order: true,
        }
    }
}

impl Default for IrrigationDepletionParserOptions {
    fn default() -> Self {
        Self::strict()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IrrigationDepletionTopologyContext {
    pub allowed_element_ids: Option<HashSet<usize>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrrigationDepletionFile {
    pub datver: Option<f64>,
    pub datver_explicit: bool,
    pub element_count: usize,
    pub system_type: IrrigationSystemType,
    pub schedule_type: i32,
    pub min_depth_m: f64,
    pub max_depth_m: Option<f64>,
    pub periods: Vec<IrrigationPeriodRecord>,
    pub initialization_complete: bool,
    pub warnings: Vec<IrrigationDepletionWarning>,
}

impl IrrigationDepletionFile {
    #[must_use]
    pub fn continuation_rows(&self) -> &[IrrigationPeriodRecord] {
        &self.periods[self.element_count..]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrrigationPeriodRecord {
    pub line: usize,
    pub element_id: usize,
    pub depletion_trigger_ratio: f64,
    pub start_doy: i32,
    pub start_year: i32,
    pub end_doy: i32,
    pub end_year: i32,
    pub continuation_order_key: (i32, i32, usize),
    pub zero_start_transition: ZeroStartTransition,
    pub furrow_disabled_by_landuse: bool,
    pub data: IrrigationPeriodData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrrigationPeriodData {
    Sprinkler(SprinklerPeriodData),
    Furrow(FurrowPeriodData),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SprinklerPeriodData {
    pub rate_m_per_s: f64,
    pub depth_ratio: f64,
    pub nozzle_factor: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FurrowPeriodData {
    pub end_element_id: usize,
    pub supply_rate_m3_per_s: f64,
    pub supply_duration_s: f64,
    pub surge_code: i32,
    pub fill_ratio: f64,
}

#[derive(Debug)]
pub enum IrrigationDepletionParseError {
    InputOpenError {
        path: PathBuf,
        source: io::Error,
    },
    MissingRecord {
        field: &'static str,
    },
    TokenParseError {
        line: usize,
        field: &'static str,
        token: String,
    },
    RecordArityError {
        line: usize,
        context: &'static str,
        expected: &'static str,
        found: usize,
    },
    UnsupportedDatver {
        line: usize,
        observed: Option<f64>,
        reason: &'static str,
    },
    InvalidHeaderDomain {
        line: usize,
        field: &'static str,
        value: i32,
        allowed: &'static str,
    },
    FieldRangeError {
        line: usize,
        field: &'static str,
        value: f64,
        expected: &'static str,
    },
    CrossFileMismatch {
        line: usize,
        field: &'static str,
        expected: String,
        observed: String,
    },
    InvalidElementId {
        line: usize,
        field: &'static str,
        value: i32,
    },
    ContinuationOrderingError {
        line: usize,
        previous: (i32, i32, usize),
        current: (i32, i32, usize),
    },
    FurrowDisallowed {
        line: usize,
        reason: &'static str,
    },
}

impl IrrigationDepletionParseError {
    #[must_use]
    pub fn contract_error_id(&self) -> &'static str {
        match self {
            Self::InputOpenError { .. } => "IRD-E-000",
            Self::TokenParseError { .. } => "IRD-E-001",
            Self::MissingRecord { .. } | Self::RecordArityError { .. } => "IRD-E-002",
            Self::UnsupportedDatver { .. } => "IRD-E-003",
            Self::InvalidHeaderDomain { .. } => "IRD-E-004",
            Self::FieldRangeError { .. } => "IRD-E-005",
            Self::CrossFileMismatch { .. } => "IRD-E-006",
            Self::InvalidElementId { .. } => "IRD-E-007",
            Self::ContinuationOrderingError { .. } => "IRD-E-008",
            Self::FurrowDisallowed { .. } => "IRD-E-009",
        }
    }
}

impl fmt::Display for IrrigationDepletionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOpenError { path, source } => {
                write!(
                    f,
                    "{}: could not open {} ({source})",
                    self.contract_error_id(),
                    path.display()
                )
            }
            Self::MissingRecord { field } => {
                write!(
                    f,
                    "{}: missing required record: {field}",
                    self.contract_error_id()
                )
            }
            Self::TokenParseError { line, field, token } => write!(
                f,
                "{}: line {line} could not parse field '{field}' from token '{token}'",
                self.contract_error_id()
            ),
            Self::RecordArityError {
                line,
                context,
                expected,
                found,
            } => write!(
                f,
                "{}: line {line} {context} expects {expected} token(s), found {found}",
                self.contract_error_id()
            ),
            Self::UnsupportedDatver {
                line,
                observed,
                reason,
            } => {
                if let Some(datver) = observed {
                    write!(
                        f,
                        "{}: line {line} datver {datver} unsupported ({reason})",
                        self.contract_error_id()
                    )
                } else {
                    write!(
                        f,
                        "{}: line {line} unsupported datver/header branch ({reason})",
                        self.contract_error_id()
                    )
                }
            }
            Self::InvalidHeaderDomain {
                line,
                field,
                value,
                allowed,
            } => write!(
                f,
                "{}: line {line} invalid header field '{field}' value {value}; expected {allowed}",
                self.contract_error_id()
            ),
            Self::FieldRangeError {
                line,
                field,
                value,
                expected,
            } => write!(
                f,
                "{}: line {line} field '{field}' value {value} violates {expected}",
                self.contract_error_id()
            ),
            Self::CrossFileMismatch {
                line,
                field,
                expected,
                observed,
            } => write!(
                f,
                "{}: line {line} cross-file mismatch for '{field}' (expected {expected}, observed {observed})",
                self.contract_error_id()
            ),
            Self::InvalidElementId { line, field, value } => write!(
                f,
                "{}: line {line} invalid element id for '{field}': {value}",
                self.contract_error_id()
            ),
            Self::ContinuationOrderingError {
                line,
                previous,
                current,
            } => write!(
                f,
                "{}: line {line} continuation ordering violation prev={previous:?} current={current:?}",
                self.contract_error_id()
            ),
            Self::FurrowDisallowed { line, reason } => write!(
                f,
                "{}: line {line} furrow irrigation disallowed ({reason})",
                self.contract_error_id()
            ),
        }
    }
}

impl std::error::Error for IrrigationDepletionParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InputOpenError { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn parse_irrigation_depletion_from_path(
    path: impl AsRef<Path>,
    options: IrrigationDepletionParserOptions,
    topology: &IrrigationDepletionTopologyContext,
) -> Result<IrrigationDepletionFile, IrrigationDepletionParseError> {
    let path = path.as_ref();
    let content = fs::read_to_string(path).map_err(|source| {
        IrrigationDepletionParseError::InputOpenError {
            path: path.to_path_buf(),
            source,
        }
    })?;
    parse_irrigation_depletion_from_str(&content, options, topology)
}

pub fn parse_irrigation_depletion_from_str(
    input: &str,
    options: IrrigationDepletionParserOptions,
    topology: &IrrigationDepletionTopologyContext,
) -> Result<IrrigationDepletionFile, IrrigationDepletionParseError> {
    let lines = collect_non_empty_lines(input);
    if lines.is_empty() {
        return Err(IrrigationDepletionParseError::MissingRecord {
            field: "datver_or_header",
        });
    }

    let mut cursor = 0usize;
    let mut warnings = Vec::new();

    let first_line = lines[0];
    let first_tokens = tokenize(first_line.text);

    let mut datver = None;
    let mut datver_explicit = false;

    if first_tokens.len() == 1 {
        let value = parse_f64(first_tokens[0], first_line.number, "datver")?;
        datver = Some(value);
        datver_explicit = true;
        cursor += 1;
    } else if options.mode == ParseMode::Compatibility {
        warnings.push(IrrigationDepletionWarning::new(
            IrrigationDepletionWarningCode::IrdW001,
            first_line.number,
            "compatibility path accepted legacy no-datver header branch",
        ));
    } else {
        return Err(IrrigationDepletionParseError::UnsupportedDatver {
            line: first_line.number,
            observed: None,
            reason: "strict mode requires explicit datver header",
        });
    }

    let header_line = require_line(&lines, &mut cursor, "header_line")?;
    let header_tokens = tokenize(header_line.text);
    if header_tokens.len() != 3 {
        return Err(IrrigationDepletionParseError::RecordArityError {
            line: header_line.number,
            context: "header_line",
            expected: "3",
            found: header_tokens.len(),
        });
    }

    let itemp_raw = parse_i32(header_tokens[0], header_line.number, "itemp")?;
    let jtemp_raw = parse_i32(header_tokens[1], header_line.number, "jtemp")?;
    let ktemp = parse_i32(header_tokens[2], header_line.number, "ktemp")?;

    if itemp_raw <= 0 {
        return Err(IrrigationDepletionParseError::InvalidHeaderDomain {
            line: header_line.number,
            field: "itemp",
            value: itemp_raw,
            allowed: "> 0",
        });
    }

    let system_type = IrrigationSystemType::from_raw(jtemp_raw).map_err(|allowed| {
        IrrigationDepletionParseError::InvalidHeaderDomain {
            line: header_line.number,
            field: "jtemp",
            value: jtemp_raw,
            allowed,
        }
    })?;

    if ktemp != 1 {
        return Err(IrrigationDepletionParseError::InvalidHeaderDomain {
            line: header_line.number,
            field: "ktemp",
            value: ktemp,
            allowed: "1 (depletion schedule)",
        });
    }

    validate_datver_policy(
        header_line.number,
        datver,
        datver_explicit,
        system_type,
        options.mode,
        &mut warnings,
    )?;

    validate_cross_file_constraints(
        header_line.number,
        usize::try_from(itemp_raw).map_err(|_| {
            IrrigationDepletionParseError::InvalidHeaderDomain {
                line: header_line.number,
                field: "itemp",
                value: itemp_raw,
                allowed: "> 0",
            }
        })?,
        system_type,
        options,
    )?;

    let static_line = require_line(&lines, &mut cursor, "static_line")?;
    let static_tokens = tokenize(static_line.text);

    let min_depth_m = parse_f64(
        static_tokens
            .first()
            .copied()
            .ok_or(IrrigationDepletionParseError::RecordArityError {
                line: static_line.number,
                context: "static_line",
                expected: "1 or 2",
                found: static_tokens.len(),
            })?,
        static_line.number,
        "irdmin",
    )?;
    validate_nonnegative(static_line.number, "irdmin", min_depth_m)?;

    let max_depth_m = match system_type {
        IrrigationSystemType::Sprinkler => {
            if static_tokens.len() != 2 {
                return Err(IrrigationDepletionParseError::RecordArityError {
                    line: static_line.number,
                    context: "sprinkler_static",
                    expected: "2",
                    found: static_tokens.len(),
                });
            }
            let parsed = parse_f64(static_tokens[1], static_line.number, "irdmax")?;
            validate_nonnegative(static_line.number, "irdmax", parsed)?;
            Some(parsed)
        }
        IrrigationSystemType::Furrow => {
            if static_tokens.len() != 1 {
                return Err(IrrigationDepletionParseError::RecordArityError {
                    line: static_line.number,
                    context: "furrow_static",
                    expected: "1",
                    found: static_tokens.len(),
                });
            }
            None
        }
    };

    let itemp = usize::try_from(itemp_raw).map_err(|_| {
        IrrigationDepletionParseError::InvalidHeaderDomain {
            line: header_line.number,
            field: "itemp",
            value: itemp_raw,
            allowed: "> 0",
        }
    })?;

    let mut periods = Vec::new();
    for index in 0..itemp {
        let row = require_line(&lines, &mut cursor, "period_line")?;
        let record = parse_period_row(
            row,
            index,
            system_type,
            datver,
            options,
            topology,
            &mut warnings,
        )?;
        periods.push(record);
    }

    while let Some(row) = lines.get(cursor).copied() {
        cursor += 1;
        let record = parse_period_row(
            row,
            periods.len(),
            system_type,
            datver,
            options,
            topology,
            &mut warnings,
        )?;
        periods.push(record);
    }

    validate_initialization_rows(itemp, &periods)?;

    if options.enforce_continuation_order {
        validate_continuation_ordering(itemp, &periods)?;
    }

    Ok(IrrigationDepletionFile {
        datver,
        datver_explicit,
        element_count: itemp,
        system_type,
        schedule_type: ktemp,
        min_depth_m,
        max_depth_m,
        periods,
        initialization_complete: true,
        warnings,
    })
}

fn validate_datver_policy(
    line: usize,
    datver: Option<f64>,
    datver_explicit: bool,
    system_type: IrrigationSystemType,
    mode: ParseMode,
    warnings: &mut Vec<IrrigationDepletionWarning>,
) -> Result<(), IrrigationDepletionParseError> {
    if !datver_explicit {
        if mode == ParseMode::Strict {
            return Err(IrrigationDepletionParseError::UnsupportedDatver {
                line,
                observed: None,
                reason: "strict mode disallows legacy no-datver probe branch",
            });
        }
        return Ok(());
    }

    let observed = datver.ok_or(IrrigationDepletionParseError::UnsupportedDatver {
        line,
        observed: None,
        reason: "explicit datver header expected",
    })?;

    if approx_eq(observed, DATVER_CANONICAL) {
        return Ok(());
    }

    if mode == ParseMode::Compatibility {
        let accepted = match system_type {
            IrrigationSystemType::Sprinkler => {
                (DATVER_SPRINKLER_COMPAT_MIN..DATVER_CANONICAL).contains(&observed)
            }
            IrrigationSystemType::Furrow => {
                (DATVER_FURROW_COMPAT_MIN..DATVER_CANONICAL).contains(&observed)
            }
        };
        if accepted {
            warnings.push(IrrigationDepletionWarning::new(
                IrrigationDepletionWarningCode::IrdW006,
                line,
                format!("compatibility accepted legacy datver {observed}"),
            ));
            return Ok(());
        }
    }

    Err(IrrigationDepletionParseError::UnsupportedDatver {
        line,
        observed: Some(observed),
        reason: "unsupported datver for selected mode/system",
    })
}

fn validate_cross_file_constraints(
    line: usize,
    element_count: usize,
    system_type: IrrigationSystemType,
    options: IrrigationDepletionParserOptions,
) -> Result<(), IrrigationDepletionParseError> {
    if let Some(expected) = options.expected_element_count {
        if expected != element_count {
            return Err(IrrigationDepletionParseError::CrossFileMismatch {
                line,
                field: "itemp",
                expected: expected.to_string(),
                observed: element_count.to_string(),
            });
        }
    }

    if let Some(expected_system) = options.expected_system_type {
        if expected_system != system_type {
            return Err(IrrigationDepletionParseError::CrossFileMismatch {
                line,
                field: "jtemp",
                expected: expected_system.raw().to_string(),
                observed: system_type.raw().to_string(),
            });
        }
    }

    if let Some(ir_option) = options.expected_irrigation_option {
        let depletion_options = [2, 3, 5, 6];
        if !depletion_options.contains(&ir_option) {
            return Err(IrrigationDepletionParseError::CrossFileMismatch {
                line,
                field: "irrigation_option",
                expected: "2|3|5|6 (depletion scheduling enabled)".to_string(),
                observed: ir_option.to_string(),
            });
        }
    }

    Ok(())
}

fn parse_period_row(
    row: LocatedLine<'_>,
    row_index: usize,
    system_type: IrrigationSystemType,
    datver: Option<f64>,
    options: IrrigationDepletionParserOptions,
    topology: &IrrigationDepletionTopologyContext,
    warnings: &mut Vec<IrrigationDepletionWarning>,
) -> Result<IrrigationPeriodRecord, IrrigationDepletionParseError> {
    let tokens = tokenize(row.text);

    match system_type {
        IrrigationSystemType::Sprinkler => {
            let parsed = parse_sprinkler_row(row, &tokens, datver, options.mode, warnings)?;
            validate_allowed_element_id(row.number, "ofeflg", parsed.element_id, topology)?;
            Ok(parsed)
        }
        IrrigationSystemType::Furrow => {
            let parsed = parse_furrow_row(row, &tokens, options, warnings)?;
            validate_allowed_element_id(row.number, "ofeflg", parsed.element_id, topology)?;
            if let IrrigationPeriodData::Furrow(data) = &parsed.data {
                validate_allowed_element_id(row.number, "endpln", data.end_element_id, topology)?;
            }
            if options.furrow_disallowed_context && options.mode == ParseMode::Strict {
                return Err(IrrigationDepletionParseError::FurrowDisallowed {
                    line: row.number,
                    reason: "furrow irrigation disallowed under contour/non-cropland strict policy",
                });
            }

            let mut parsed = parsed;
            if options.furrow_disallowed_context {
                parsed.furrow_disabled_by_landuse = true;
                warnings.push(IrrigationDepletionWarning::new(
                    IrrigationDepletionWarningCode::IrdW005,
                    row.number,
                    "compatibility disabled furrow irrigation for contour/non-cropland context",
                ));
            }

            let _ = row_index;
            Ok(parsed)
        }
    }
}

fn parse_sprinkler_row(
    row: LocatedLine<'_>,
    tokens: &[&str],
    datver: Option<f64>,
    mode: ParseMode,
    warnings: &mut Vec<IrrigationDepletionWarning>,
) -> Result<IrrigationPeriodRecord, IrrigationDepletionParseError> {
    let has_nozzle = match tokens.len() {
        9 => true,
        8 if mode == ParseMode::Compatibility => false,
        _ => {
            return Err(IrrigationDepletionParseError::RecordArityError {
                line: row.number,
                context: "sprinkler_period",
                expected: "9 (or 8 in compatibility for legacy nozzle omission)",
                found: tokens.len(),
            });
        }
    };

    if !has_nozzle && mode == ParseMode::Compatibility {
        if let Some(value) = datver {
            if value >= DATVER_SPRINKLER_COMPAT_MIN {
                warnings.push(IrrigationDepletionWarning::new(
                    IrrigationDepletionWarningCode::IrdW002,
                    row.number,
                    "compatibility injected legacy default nozzle=1.0",
                ));
            }
        } else {
            warnings.push(IrrigationDepletionWarning::new(
                IrrigationDepletionWarningCode::IrdW002,
                row.number,
                "compatibility injected legacy default nozzle=1.0",
            ));
        }
    }

    let element_id = parse_positive_i32(tokens[0], row.number, "ofeflg")?;
    let rate_m_per_s = parse_f64(tokens[1], row.number, "irrate")?;
    validate_nonnegative(row.number, "irrate", rate_m_per_s)?;

    let depth_ratio = parse_f64(tokens[2], row.number, "aprati")?;
    validate_nonnegative(row.number, "aprati", depth_ratio)?;

    let depletion_trigger_ratio = parse_f64(tokens[3], row.number, "deplev")?;
    validate_nonnegative(row.number, "deplev", depletion_trigger_ratio)?;

    let nozzle_factor = if has_nozzle {
        let parsed = parse_f64(tokens[4], row.number, "nozzle")?;
        validate_positive(row.number, "nozzle", parsed)?;
        parsed
    } else {
        1.0
    };

    let date_offset = if has_nozzle { 5 } else { 4 };
    let start_doy = parse_i32(tokens[date_offset], row.number, "irbeg")?;
    let start_year = parse_i32(tokens[date_offset + 1], row.number, "yrbeg")?;
    let end_doy = parse_i32(tokens[date_offset + 2], row.number, "irend")?;
    let end_year = parse_i32(tokens[date_offset + 3], row.number, "yrend")?;

    validate_day_year_tuple(row.number, "irbeg/yrbeg", start_doy, start_year)?;
    validate_day_year_tuple(row.number, "irend/yrend", end_doy, end_year)?;
    validate_period_bounds(row.number, start_doy, start_year, end_doy, end_year)?;

    let zero_start_transition =
        derive_zero_start_transition(start_doy, options_for_zero(mode), row.number, warnings);

    Ok(IrrigationPeriodRecord {
        line: row.number,
        element_id: usize::try_from(element_id).map_err(|_| {
            IrrigationDepletionParseError::InvalidElementId {
                line: row.number,
                field: "ofeflg",
                value: element_id,
            }
        })?,
        depletion_trigger_ratio,
        start_doy,
        start_year,
        end_doy,
        end_year,
        continuation_order_key: (
            end_year,
            end_doy,
            usize::try_from(element_id).unwrap_or_default(),
        ),
        zero_start_transition,
        furrow_disabled_by_landuse: false,
        data: IrrigationPeriodData::Sprinkler(SprinklerPeriodData {
            rate_m_per_s,
            depth_ratio,
            nozzle_factor,
        }),
    })
}

fn parse_furrow_row(
    row: LocatedLine<'_>,
    tokens: &[&str],
    options: IrrigationDepletionParserOptions,
    warnings: &mut Vec<IrrigationDepletionWarning>,
) -> Result<IrrigationPeriodRecord, IrrigationDepletionParseError> {
    if tokens.len() != 11 {
        return Err(IrrigationDepletionParseError::RecordArityError {
            line: row.number,
            context: "furrow_period",
            expected: "11",
            found: tokens.len(),
        });
    }

    let element_id = parse_positive_i32(tokens[0], row.number, "ofeflg")?;
    let end_element_id = parse_positive_i32(tokens[1], row.number, "endpln")?;

    let supply_rate_m3_per_s = parse_f64(tokens[2], row.number, "florat")?;
    validate_nonnegative(row.number, "florat", supply_rate_m3_per_s)?;

    let supply_duration_s = parse_f64(tokens[3], row.number, "timest")?;
    validate_nonnegative(row.number, "timest", supply_duration_s)?;

    let mut surge_code = parse_i32(tokens[4], row.number, "depsrg")?;
    if options.mode == ParseMode::Strict {
        if !matches!(surge_code, 1 | 2 | 4 | 5 | 6) {
            return Err(IrrigationDepletionParseError::FieldRangeError {
                line: row.number,
                field: "depsrg",
                value: f64::from(surge_code),
                expected: "1, 2, 4, 5, or 6",
            });
        }
    } else if surge_code == 3 {
        warnings.push(IrrigationDepletionWarning::new(
            IrrigationDepletionWarningCode::IrdW003,
            row.number,
            "compatibility normalized depsrg 3 -> 4",
        ));
        surge_code = 4;
    } else if surge_code > 6 {
        warnings.push(IrrigationDepletionWarning::new(
            IrrigationDepletionWarningCode::IrdW003,
            row.number,
            format!("compatibility normalized depsrg {surge_code} -> 6"),
        ));
        surge_code = 6;
    } else if surge_code <= 0 {
        return Err(IrrigationDepletionParseError::FieldRangeError {
            line: row.number,
            field: "depsrg",
            value: f64::from(surge_code),
            expected: "> 0",
        });
    }

    let fill_ratio = parse_f64(tokens[5], row.number, "filrat")?;
    validate_nonnegative(row.number, "filrat", fill_ratio)?;

    let depletion_trigger_ratio = parse_f64(tokens[6], row.number, "deplev")?;
    validate_nonnegative(row.number, "deplev", depletion_trigger_ratio)?;

    let start_doy = parse_i32(tokens[7], row.number, "irbeg")?;
    let start_year = parse_i32(tokens[8], row.number, "yrbeg")?;
    let end_doy = parse_i32(tokens[9], row.number, "irend")?;
    let end_year = parse_i32(tokens[10], row.number, "yrend")?;

    validate_day_year_tuple(row.number, "irbeg/yrbeg", start_doy, start_year)?;
    validate_day_year_tuple(row.number, "irend/yrend", end_doy, end_year)?;
    validate_period_bounds(row.number, start_doy, start_year, end_doy, end_year)?;

    let zero_start_transition =
        derive_zero_start_transition(start_doy, options.irschd_on_entry, row.number, warnings);

    Ok(IrrigationPeriodRecord {
        line: row.number,
        element_id: usize::try_from(element_id).map_err(|_| {
            IrrigationDepletionParseError::InvalidElementId {
                line: row.number,
                field: "ofeflg",
                value: element_id,
            }
        })?,
        depletion_trigger_ratio,
        start_doy,
        start_year,
        end_doy,
        end_year,
        continuation_order_key: (
            end_year,
            end_doy,
            usize::try_from(element_id).unwrap_or_default(),
        ),
        zero_start_transition,
        furrow_disabled_by_landuse: false,
        data: IrrigationPeriodData::Furrow(FurrowPeriodData {
            end_element_id: usize::try_from(end_element_id).map_err(|_| {
                IrrigationDepletionParseError::InvalidElementId {
                    line: row.number,
                    field: "endpln",
                    value: end_element_id,
                }
            })?,
            supply_rate_m3_per_s,
            supply_duration_s,
            surge_code,
            fill_ratio,
        }),
    })
}

fn validate_initialization_rows(
    element_count: usize,
    periods: &[IrrigationPeriodRecord],
) -> Result<(), IrrigationDepletionParseError> {
    for (index, row) in periods.iter().take(element_count).enumerate() {
        let expected = index + 1;
        if row.element_id != expected {
            return Err(IrrigationDepletionParseError::ContinuationOrderingError {
                line: row.line,
                previous: (0, 0, expected),
                current: (0, 0, row.element_id),
            });
        }
    }
    Ok(())
}

fn validate_continuation_ordering(
    element_count: usize,
    periods: &[IrrigationPeriodRecord],
) -> Result<(), IrrigationDepletionParseError> {
    if periods.len() <= element_count + 1 {
        return Ok(());
    }

    let continuation = &periods[element_count..];
    for pair in continuation.windows(2) {
        let previous = pair[0].continuation_order_key;
        let current = pair[1].continuation_order_key;
        if current < previous {
            return Err(IrrigationDepletionParseError::ContinuationOrderingError {
                line: pair[1].line,
                previous,
                current,
            });
        }
    }

    Ok(())
}

fn validate_allowed_element_id(
    line: usize,
    field: &'static str,
    value: usize,
    topology: &IrrigationDepletionTopologyContext,
) -> Result<(), IrrigationDepletionParseError> {
    if let Some(ids) = &topology.allowed_element_ids {
        if !ids.contains(&value) {
            return Err(IrrigationDepletionParseError::InvalidElementId {
                line,
                field,
                value: i32::try_from(value).unwrap_or(i32::MAX),
            });
        }
    }
    Ok(())
}

fn derive_zero_start_transition(
    start_doy: i32,
    irschd_on_entry: Option<i32>,
    line: usize,
    warnings: &mut Vec<IrrigationDepletionWarning>,
) -> ZeroStartTransition {
    if start_doy != 0 {
        return ZeroStartTransition::None;
    }

    let transition = match irschd_on_entry {
        Some(3) => ZeroStartTransition::ThreeToTwo,
        Some(1) => ZeroStartTransition::OneToZero,
        _ => ZeroStartTransition::Unknown,
    };

    warnings.push(IrrigationDepletionWarning::new(
        IrrigationDepletionWarningCode::IrdW004,
        line,
        "irbeg==0 transition branch encountered",
    ));

    transition
}

fn options_for_zero(mode: ParseMode) -> Option<i32> {
    match mode {
        ParseMode::Strict => Some(1),
        ParseMode::Compatibility => None,
    }
}

#[derive(Clone, Copy)]
struct LocatedLine<'a> {
    number: usize,
    text: &'a str,
}

fn collect_non_empty_lines(input: &str) -> Vec<LocatedLine<'_>> {
    input
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let trimmed = line.trim();
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
    field: &'static str,
) -> Result<LocatedLine<'a>, IrrigationDepletionParseError> {
    let line = lines
        .get(*cursor)
        .copied()
        .ok_or(IrrigationDepletionParseError::MissingRecord { field })?;
    *cursor += 1;
    Ok(line)
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

fn parse_i32(
    token: &str,
    line: usize,
    field: &'static str,
) -> Result<i32, IrrigationDepletionParseError> {
    if let Ok(value) = token.parse::<i32>() {
        return Ok(value);
    }

    let float_value = parse_f64(token, line, field)?;
    let rounded = float_value.round();
    if (rounded - float_value).abs() > FLOAT_TOLERANCE {
        return Err(IrrigationDepletionParseError::TokenParseError {
            line,
            field,
            token: token.to_string(),
        });
    }
    if !(f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&rounded) {
        return Err(IrrigationDepletionParseError::TokenParseError {
            line,
            field,
            token: token.to_string(),
        });
    }

    let rounded_i64 = format!("{rounded:.0}").parse::<i64>().map_err(|_| {
        IrrigationDepletionParseError::TokenParseError {
            line,
            field,
            token: token.to_string(),
        }
    })?;
    i32::try_from(rounded_i64).map_err(|_| IrrigationDepletionParseError::TokenParseError {
        line,
        field,
        token: token.to_string(),
    })
}

fn parse_positive_i32(
    token: &str,
    line: usize,
    field: &'static str,
) -> Result<i32, IrrigationDepletionParseError> {
    let value = parse_i32(token, line, field)?;
    if value <= 0 {
        return Err(IrrigationDepletionParseError::InvalidElementId { line, field, value });
    }
    Ok(value)
}

fn parse_f64(
    token: &str,
    line: usize,
    field: &'static str,
) -> Result<f64, IrrigationDepletionParseError> {
    token
        .parse::<f64>()
        .map_err(|_| IrrigationDepletionParseError::TokenParseError {
            line,
            field,
            token: token.to_string(),
        })
}

fn validate_nonnegative(
    line: usize,
    field: &'static str,
    value: f64,
) -> Result<(), IrrigationDepletionParseError> {
    if !value.is_finite() || value < 0.0 {
        return Err(IrrigationDepletionParseError::FieldRangeError {
            line,
            field,
            value,
            expected: "finite and >= 0",
        });
    }
    Ok(())
}

fn validate_positive(
    line: usize,
    field: &'static str,
    value: f64,
) -> Result<(), IrrigationDepletionParseError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(IrrigationDepletionParseError::FieldRangeError {
            line,
            field,
            value,
            expected: "finite and > 0",
        });
    }
    Ok(())
}

fn validate_day_year_tuple(
    line: usize,
    field: &'static str,
    day: i32,
    year: i32,
) -> Result<(), IrrigationDepletionParseError> {
    if !(0..=366).contains(&day) {
        return Err(IrrigationDepletionParseError::FieldRangeError {
            line,
            field,
            value: f64::from(day),
            expected: "day in [0, 366]",
        });
    }
    if day == 0 {
        return Ok(());
    }
    if year <= 0 {
        return Err(IrrigationDepletionParseError::FieldRangeError {
            line,
            field,
            value: f64::from(year),
            expected: "year > 0 when day > 0",
        });
    }
    Ok(())
}

fn validate_period_bounds(
    line: usize,
    start_doy: i32,
    start_year: i32,
    end_doy: i32,
    end_year: i32,
) -> Result<(), IrrigationDepletionParseError> {
    if start_doy == 0 {
        return Ok(());
    }

    let start = (start_year, start_doy);
    let end = (end_year, end_doy);
    if end < start {
        return Err(IrrigationDepletionParseError::FieldRangeError {
            line,
            field: "irend/yrend",
            value: f64::from(end_year),
            expected: "end date must be >= start date",
        });
    }

    Ok(())
}

fn approx_eq(left: f64, right: f64) -> bool {
    (left - right).abs() <= FLOAT_TOLERANCE
}
