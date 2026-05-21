#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::manual_range_contains,
    clippy::missing_errors_doc,
    clippy::needless_borrow,
    clippy::too_many_lines
)]

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const DATVER_CANONICAL: f64 = 95.7;
const DATVER_LEGACY_MIN_SPRINKLER: f64 = 94.21;
const DATVER_LEGACY_MIN_FURROW: f64 = 91.5;
const DATVER_EPSILON: f64 = 1e-6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatverSource {
    ExplicitHeader,
    LegacyCompatNoDatver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IryrInterpretationMode {
    UnresolvedRequiresRuntimePolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixedDateWarningCode {
    FdirW001,
    FdirW002,
    FdirW003,
    FdirW004,
    FdirW005,
    FdirW006,
}

impl FixedDateWarningCode {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FdirW001 => "FDIR-W-001",
            Self::FdirW002 => "FDIR-W-002",
            Self::FdirW003 => "FDIR-W-003",
            Self::FdirW004 => "FDIR-W-004",
            Self::FdirW005 => "FDIR-W-005",
            Self::FdirW006 => "FDIR-W-006",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixedDateWarning {
    pub code: FixedDateWarningCode,
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line3Record {
    pub ofeflg: usize,
    pub irday: usize,
    pub iryr: usize,
    pub schedule_termination_flag: bool,
    pub legacy_ordering_warning_emitted: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SprinklerEvent {
    pub irint: f64,
    pub irdept: f64,
    pub nozzle: f64,
    pub legacy_nozzle_default_applied: bool,
    pub next_record: Line3Record,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FurrowSurge {
    pub qspply: f64,
    pub tstart: f64,
    pub tend: f64,
    pub tdepl: Option<f64>,
    pub legacy_line5_arity: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FurrowEvent {
    pub surges: usize,
    pub rows: Vec<FurrowSurge>,
    pub next_record: Line3Record,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FixedDateEvent {
    Sprinkler(SprinklerEvent),
    Furrow(FurrowEvent),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixedDateIrrigationFile {
    pub datver: f64,
    pub datver_source: DatverSource,
    pub itemp: usize,
    pub jtemp: usize,
    pub ktemp: usize,
    pub initial_records: Vec<Line3Record>,
    pub events: Vec<FixedDateEvent>,
    pub initial_dates_complete: bool,
    pub event_stream_complete: bool,
    pub iryr_interpretation_mode: IryrInterpretationMode,
    pub warnings: Vec<FixedDateWarning>,
}

#[derive(Debug)]
pub enum FixedDateParseError {
    InputOpenError {
        path: PathBuf,
        source: io::Error,
    },
    MissingRecord {
        line: usize,
        context: &'static str,
    },
    TokenParseError {
        line: usize,
        field: &'static str,
        value: String,
    },
    RecordArityError {
        line: usize,
        context: &'static str,
        expected: &'static str,
        observed: usize,
    },
    LegacyNoDatverDisallowed {
        line: usize,
    },
    UnsupportedDatver {
        line: usize,
        datver: f64,
        jtemp: usize,
    },
    HeaderDomainError {
        line: usize,
        field: &'static str,
        value: i64,
        expected: &'static str,
    },
    FieldRangeError {
        line: usize,
        field: &'static str,
        value: f64,
        expected: &'static str,
    },
    OrderingConstraintError {
        line: usize,
        phase: &'static str,
        expected_ofe: usize,
        observed_ofe: usize,
    },
    EventStreamClosureError {
        line: usize,
        context: &'static str,
    },
}

impl FixedDateParseError {
    #[must_use]
    pub fn contract_error_id(&self) -> &'static str {
        match self {
            Self::InputOpenError { .. } => "FDIR-E-000",
            Self::TokenParseError { .. } => "FDIR-E-001",
            Self::MissingRecord { .. } | Self::RecordArityError { .. } => "FDIR-E-002",
            Self::LegacyNoDatverDisallowed { .. } | Self::UnsupportedDatver { .. } => "FDIR-E-003",
            Self::HeaderDomainError { .. } => "FDIR-E-004",
            Self::FieldRangeError { .. } => "FDIR-E-005",
            Self::OrderingConstraintError { .. } => "FDIR-E-010",
            Self::EventStreamClosureError { .. } => "FDIR-E-008",
        }
    }
}

impl fmt::Display for FixedDateParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOpenError { path, source } => write!(
                f,
                "{}: could not open {} ({source})",
                self.contract_error_id(),
                path.display()
            ),
            Self::MissingRecord { line, context } => write!(
                f,
                "{}: missing record for {context} near line {line}",
                self.contract_error_id()
            ),
            Self::TokenParseError { line, field, value } => write!(
                f,
                "{}: token parse failure at line {line} for {field}: {value}",
                self.contract_error_id()
            ),
            Self::RecordArityError {
                line,
                context,
                expected,
                observed,
            } => write!(
                f,
                "{}: line {line} invalid {context} arity; expected {expected}, observed {observed}",
                self.contract_error_id()
            ),
            Self::LegacyNoDatverDisallowed { line } => write!(
                f,
                "{}: strict mode requires explicit datver header (line {line})",
                self.contract_error_id()
            ),
            Self::UnsupportedDatver {
                line,
                datver,
                jtemp,
            } => write!(
                f,
                "{}: line {line} unsupported datver {datver} for jtemp={jtemp}",
                self.contract_error_id()
            ),
            Self::HeaderDomainError {
                line,
                field,
                value,
                expected,
            } => write!(
                f,
                "{}: line {line} invalid header field {field}={value}, expected {expected}",
                self.contract_error_id()
            ),
            Self::FieldRangeError {
                line,
                field,
                value,
                expected,
            } => write!(
                f,
                "{}: line {line} invalid {field}={value}, expected {expected}",
                self.contract_error_id()
            ),
            Self::OrderingConstraintError {
                line,
                phase,
                expected_ofe,
                observed_ofe,
            } => write!(
                f,
                "{}: line {line} {phase} ordering mismatch, expected OFE {expected_ofe}, observed OFE {observed_ofe}",
                self.contract_error_id()
            ),
            Self::EventStreamClosureError { line, context } => write!(
                f,
                "{}: event-stream closure failure near line {line}: {context}",
                self.contract_error_id()
            ),
        }
    }
}

impl std::error::Error for FixedDateParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InputOpenError { source, .. } => Some(source),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct NormalizedLine {
    line_no: usize,
    text: String,
}

pub fn parse_fixeddate_file(
    path: impl AsRef<Path>,
    mode: ParseMode,
) -> Result<FixedDateIrrigationFile, FixedDateParseError> {
    let path_ref = path.as_ref();
    let input =
        fs::read_to_string(path_ref).map_err(|source| FixedDateParseError::InputOpenError {
            path: path_ref.to_path_buf(),
            source,
        })?;
    parse_fixeddate_str(&input, mode)
}

pub fn parse_fixeddate_str(
    input: &str,
    mode: ParseMode,
) -> Result<FixedDateIrrigationFile, FixedDateParseError> {
    let lines = normalize_lines(input);
    if lines.is_empty() {
        return Err(FixedDateParseError::MissingRecord {
            line: 1,
            context: "fixeddate preamble",
        });
    }

    let mut warnings = Vec::new();
    let mut idx = 0usize;
    let first_tokens = split_tokens(&lines[0]);

    let (datver, datver_source) = match mode {
        ParseMode::Strict => {
            if first_tokens.len() != 1 {
                return Err(FixedDateParseError::LegacyNoDatverDisallowed {
                    line: lines[0].line_no,
                });
            }
            let datver = parse_f64(&first_tokens[0], lines[0].line_no, "datver")?;
            idx += 1;
            (datver, DatverSource::ExplicitHeader)
        }
        ParseMode::Compatibility => {
            if first_tokens.len() == 1 {
                if let Ok(value) = first_tokens[0].parse::<f64>() {
                    if value > 2.0 {
                        idx += 1;
                        (value, DatverSource::ExplicitHeader)
                    } else {
                        warnings.push(FixedDateWarning {
                            code: FixedDateWarningCode::FdirW001,
                            line: lines[0].line_no,
                            message: "legacy no-datver branch accepted".to_string(),
                        });
                        (DATVER_CANONICAL, DatverSource::LegacyCompatNoDatver)
                    }
                } else {
                    return Err(FixedDateParseError::TokenParseError {
                        line: lines[0].line_no,
                        field: "datver_or_header",
                        value: first_tokens[0].to_string(),
                    });
                }
            } else {
                warnings.push(FixedDateWarning {
                    code: FixedDateWarningCode::FdirW001,
                    line: lines[0].line_no,
                    message: "legacy no-datver branch accepted".to_string(),
                });
                (DATVER_CANONICAL, DatverSource::LegacyCompatNoDatver)
            }
        }
    };

    let header = lines.get(idx).ok_or(FixedDateParseError::MissingRecord {
        line: lines.last().map_or(1, |line| line.line_no + 1),
        context: "header line (itemp jtemp ktemp)",
    })?;
    let header_tokens = split_tokens(header);
    if header_tokens.len() != 3 {
        return Err(FixedDateParseError::RecordArityError {
            line: header.line_no,
            context: "header line",
            expected: "3",
            observed: header_tokens.len(),
        });
    }
    let itemp = parse_i64(&header_tokens[0], header.line_no, "itemp")?;
    let jtemp = parse_i64(&header_tokens[1], header.line_no, "jtemp")?;
    let ktemp = parse_i64(&header_tokens[2], header.line_no, "ktemp")?;
    idx += 1;

    if itemp <= 0 {
        return Err(FixedDateParseError::HeaderDomainError {
            line: header.line_no,
            field: "itemp",
            value: itemp,
            expected: "> 0",
        });
    }
    if jtemp != 1 && jtemp != 2 {
        return Err(FixedDateParseError::HeaderDomainError {
            line: header.line_no,
            field: "jtemp",
            value: jtemp,
            expected: "{1,2}",
        });
    }
    if ktemp != 2 {
        return Err(FixedDateParseError::HeaderDomainError {
            line: header.line_no,
            field: "ktemp",
            value: ktemp,
            expected: "2",
        });
    }

    let itemp = itemp as usize;
    let jtemp = jtemp as usize;
    let ktemp = ktemp as usize;

    validate_datver_policy(
        datver,
        datver_source,
        jtemp,
        mode,
        header.line_no,
        &mut warnings,
    )?;

    let mut initial_records = Vec::with_capacity(itemp);
    for expected_ofe in 1..=itemp {
        let line = lines.get(idx).ok_or(FixedDateParseError::MissingRecord {
            line: lines.last().map_or(1, |tail| tail.line_no + 1),
            context: "initial line3 record",
        })?;
        let record = parse_line3_record(
            line,
            itemp,
            expected_ofe,
            "initialization",
            mode,
            &mut warnings,
        )?;
        initial_records.push(record);
        idx += 1;
    }

    let mut events = Vec::new();
    let mut expected_event_ofe = 1usize;
    while idx < lines.len() {
        match jtemp {
            1 => {
                let event_line4 =
                    lines
                        .get(idx)
                        .ok_or(FixedDateParseError::EventStreamClosureError {
                            line: lines.last().map_or(1, |tail| tail.line_no + 1),
                            context: "sprinkler line4",
                        })?;
                let event_line3 =
                    lines
                        .get(idx + 1)
                        .ok_or(FixedDateParseError::EventStreamClosureError {
                            line: event_line4.line_no + 1,
                            context: "sprinkler successor line3",
                        })?;
                let event = parse_sprinkler_event(
                    event_line4,
                    event_line3,
                    itemp,
                    expected_event_ofe,
                    mode,
                    &mut warnings,
                )?;
                events.push(FixedDateEvent::Sprinkler(event));
                idx += 2;
            }
            2 => {
                let event_line4 =
                    lines
                        .get(idx)
                        .ok_or(FixedDateParseError::EventStreamClosureError {
                            line: lines.last().map_or(1, |tail| tail.line_no + 1),
                            context: "furrow line4",
                        })?;
                let surges = parse_furrow_surges(event_line4)?;
                let first_row_idx = idx + 1;
                let line3_idx = first_row_idx + surges;
                let event_line3 =
                    lines
                        .get(line3_idx)
                        .ok_or(FixedDateParseError::EventStreamClosureError {
                            line: event_line4.line_no + 1,
                            context: "furrow successor line3",
                        })?;

                let mut rows = Vec::with_capacity(surges);
                for row_idx in 0..surges {
                    let row_line = lines.get(first_row_idx + row_idx).ok_or(
                        FixedDateParseError::EventStreamClosureError {
                            line: event_line4.line_no + 1 + row_idx,
                            context: "furrow line5",
                        },
                    )?;
                    rows.push(parse_furrow_row(row_line, mode, &mut warnings)?);
                }

                let next_record = parse_line3_record(
                    event_line3,
                    itemp,
                    expected_event_ofe,
                    "event",
                    mode,
                    &mut warnings,
                )?;

                events.push(FixedDateEvent::Furrow(FurrowEvent {
                    surges,
                    rows,
                    next_record,
                }));
                idx = line3_idx + 1;
            }
            _ => {
                return Err(FixedDateParseError::HeaderDomainError {
                    line: header.line_no,
                    field: "jtemp",
                    value: jtemp as i64,
                    expected: "{1,2}",
                });
            }
        }

        expected_event_ofe += 1;
        if expected_event_ofe > itemp {
            expected_event_ofe = 1;
        }
    }

    Ok(FixedDateIrrigationFile {
        datver,
        datver_source,
        itemp,
        jtemp,
        ktemp,
        initial_records,
        events,
        initial_dates_complete: true,
        event_stream_complete: true,
        iryr_interpretation_mode: IryrInterpretationMode::UnresolvedRequiresRuntimePolicy,
        warnings,
    })
}

fn validate_datver_policy(
    datver: f64,
    datver_source: DatverSource,
    jtemp: usize,
    mode: ParseMode,
    line: usize,
    warnings: &mut Vec<FixedDateWarning>,
) -> Result<(), FixedDateParseError> {
    match mode {
        ParseMode::Strict => {
            if !approx_eq(datver, DATVER_CANONICAL) {
                return Err(FixedDateParseError::UnsupportedDatver {
                    line,
                    datver,
                    jtemp,
                });
            }
            if !matches!(datver_source, DatverSource::ExplicitHeader) {
                return Err(FixedDateParseError::LegacyNoDatverDisallowed { line });
            }
        }
        ParseMode::Compatibility => {
            if matches!(datver_source, DatverSource::LegacyCompatNoDatver) {
                return Ok(());
            }

            if approx_eq(datver, DATVER_CANONICAL) {
                return Ok(());
            }

            let allowed = if jtemp == 1 {
                datver >= DATVER_LEGACY_MIN_SPRINKLER && datver < DATVER_CANONICAL
            } else {
                datver >= DATVER_LEGACY_MIN_FURROW && datver < DATVER_CANONICAL
            };

            if allowed {
                warnings.push(FixedDateWarning {
                    code: FixedDateWarningCode::FdirW002,
                    line,
                    message: format!("legacy explicit datver accepted: {datver}"),
                });
                return Ok(());
            }
            return Err(FixedDateParseError::UnsupportedDatver {
                line,
                datver,
                jtemp,
            });
        }
    }

    Ok(())
}

fn parse_sprinkler_event(
    line4: &NormalizedLine,
    line3: &NormalizedLine,
    itemp: usize,
    expected_ofe: usize,
    mode: ParseMode,
    warnings: &mut Vec<FixedDateWarning>,
) -> Result<SprinklerEvent, FixedDateParseError> {
    let tokens = split_tokens(line4);
    if tokens.len() != 3 && !(matches!(mode, ParseMode::Compatibility) && tokens.len() == 2) {
        return Err(FixedDateParseError::RecordArityError {
            line: line4.line_no,
            context: "sprinkler line4",
            expected: "3 (strict) or 2..3 (compatibility)",
            observed: tokens.len(),
        });
    }

    let irint = parse_f64(&tokens[0], line4.line_no, "irint")?;
    let irdept = parse_f64(&tokens[1], line4.line_no, "irdept")?;
    if irint <= 0.0 {
        return Err(FixedDateParseError::FieldRangeError {
            line: line4.line_no,
            field: "irint",
            value: irint,
            expected: "> 0",
        });
    }
    if irdept < 0.0 {
        return Err(FixedDateParseError::FieldRangeError {
            line: line4.line_no,
            field: "irdept",
            value: irdept,
            expected: ">= 0",
        });
    }

    let (nozzle, legacy_nozzle_default_applied) = if tokens.len() == 3 {
        let nozzle = parse_f64(&tokens[2], line4.line_no, "nozzle")?;
        if nozzle <= 0.0 {
            return Err(FixedDateParseError::FieldRangeError {
                line: line4.line_no,
                field: "nozzle",
                value: nozzle,
                expected: "> 0",
            });
        }
        (nozzle, false)
    } else {
        warnings.push(FixedDateWarning {
            code: FixedDateWarningCode::FdirW003,
            line: line4.line_no,
            message: "legacy sprinkler two-field row accepted; nozzle defaulted to 1.0".to_string(),
        });
        (1.0, true)
    };

    let next_record = parse_line3_record(line3, itemp, expected_ofe, "event", mode, warnings)?;
    Ok(SprinklerEvent {
        irint,
        irdept,
        nozzle,
        legacy_nozzle_default_applied,
        next_record,
    })
}

fn parse_furrow_surges(line4: &NormalizedLine) -> Result<usize, FixedDateParseError> {
    let tokens = split_tokens(line4);
    if tokens.len() != 1 {
        return Err(FixedDateParseError::RecordArityError {
            line: line4.line_no,
            context: "furrow line4",
            expected: "1",
            observed: tokens.len(),
        });
    }
    let surges = parse_i64(&tokens[0], line4.line_no, "surges")?;
    if !(1..=20).contains(&surges) {
        return Err(FixedDateParseError::FieldRangeError {
            line: line4.line_no,
            field: "surges",
            value: surges as f64,
            expected: "1..20",
        });
    }
    Ok(surges as usize)
}

fn parse_furrow_row(
    line: &NormalizedLine,
    mode: ParseMode,
    warnings: &mut Vec<FixedDateWarning>,
) -> Result<FurrowSurge, FixedDateParseError> {
    let tokens = split_tokens(line);
    match mode {
        ParseMode::Strict => {
            if tokens.len() != 4 {
                return Err(FixedDateParseError::RecordArityError {
                    line: line.line_no,
                    context: "furrow line5",
                    expected: "4",
                    observed: tokens.len(),
                });
            }
        }
        ParseMode::Compatibility => {
            if tokens.len() != 3 && tokens.len() != 4 {
                return Err(FixedDateParseError::RecordArityError {
                    line: line.line_no,
                    context: "furrow line5",
                    expected: "3 or 4",
                    observed: tokens.len(),
                });
            }
        }
    }

    let qspply = parse_f64(&tokens[0], line.line_no, "qspply")?;
    let tstart = parse_f64(&tokens[1], line.line_no, "tstart")?;
    let tend = parse_f64(&tokens[2], line.line_no, "tend")?;
    if qspply < 0.0 {
        return Err(FixedDateParseError::FieldRangeError {
            line: line.line_no,
            field: "qspply",
            value: qspply,
            expected: ">= 0",
        });
    }
    if tstart < 0.0 {
        return Err(FixedDateParseError::FieldRangeError {
            line: line.line_no,
            field: "tstart",
            value: tstart,
            expected: ">= 0",
        });
    }
    if tend < tstart {
        return Err(FixedDateParseError::FieldRangeError {
            line: line.line_no,
            field: "tend",
            value: tend,
            expected: ">= tstart",
        });
    }

    let (tdepl, legacy_line5_arity) = if tokens.len() == 4 {
        let value = parse_f64(&tokens[3], line.line_no, "tdepl")?;
        if value < 0.0 {
            return Err(FixedDateParseError::FieldRangeError {
                line: line.line_no,
                field: "tdepl",
                value,
                expected: ">= 0",
            });
        }
        (Some(value), 4usize)
    } else {
        warnings.push(FixedDateWarning {
            code: FixedDateWarningCode::FdirW004,
            line: line.line_no,
            message: "legacy furrow three-field line5 accepted; tdepl omitted".to_string(),
        });
        (None, 3usize)
    };

    Ok(FurrowSurge {
        qspply,
        tstart,
        tend,
        tdepl,
        legacy_line5_arity,
    })
}

fn parse_line3_record(
    line: &NormalizedLine,
    itemp: usize,
    expected_ofe: usize,
    phase: &'static str,
    mode: ParseMode,
    warnings: &mut Vec<FixedDateWarning>,
) -> Result<Line3Record, FixedDateParseError> {
    let tokens = split_tokens(line);
    if tokens.len() != 3 {
        return Err(FixedDateParseError::RecordArityError {
            line: line.line_no,
            context: "line3",
            expected: "3",
            observed: tokens.len(),
        });
    }

    let ofeflg = parse_i64(&tokens[0], line.line_no, "ofeflg")?;
    let irday = parse_i64(&tokens[1], line.line_no, "irday")?;
    let iryr = parse_i64(&tokens[2], line.line_no, "iryr")?;

    if ofeflg <= 0 || ofeflg as usize > itemp {
        return Err(FixedDateParseError::FieldRangeError {
            line: line.line_no,
            field: "ofeflg",
            value: ofeflg as f64,
            expected: "1..itemp",
        });
    }
    if !(0..=366).contains(&irday) {
        return Err(FixedDateParseError::FieldRangeError {
            line: line.line_no,
            field: "irday",
            value: irday as f64,
            expected: "0..366",
        });
    }
    if iryr < 0 {
        return Err(FixedDateParseError::FieldRangeError {
            line: line.line_no,
            field: "iryr",
            value: iryr as f64,
            expected: ">= 0",
        });
    }

    let observed_ofe = ofeflg as usize;
    let mut legacy_ordering_warning_emitted = false;
    if observed_ofe != expected_ofe {
        match mode {
            ParseMode::Strict => {
                return Err(FixedDateParseError::OrderingConstraintError {
                    line: line.line_no,
                    phase,
                    expected_ofe,
                    observed_ofe,
                });
            }
            ParseMode::Compatibility => {
                warnings.push(FixedDateWarning {
                    code: FixedDateWarningCode::FdirW006,
                    line: line.line_no,
                    message: format!(
                        "legacy ordering anomaly accepted in {phase}: expected OFE {expected_ofe}, observed OFE {observed_ofe}"
                    ),
                });
                legacy_ordering_warning_emitted = true;
            }
        }
    }

    Ok(Line3Record {
        ofeflg: observed_ofe,
        irday: irday as usize,
        iryr: iryr as usize,
        schedule_termination_flag: irday == 0,
        legacy_ordering_warning_emitted,
    })
}

fn normalize_lines(input: &str) -> Vec<NormalizedLine> {
    let mut lines = Vec::new();
    for (idx, raw) in input.lines().enumerate() {
        let no_comment = raw.split_once('#').map_or(raw, |(prefix, _)| prefix);
        let trimmed = no_comment.trim();
        if !trimmed.is_empty() {
            lines.push(NormalizedLine {
                line_no: idx + 1,
                text: trimmed.to_string(),
            });
        }
    }
    lines
}

fn split_tokens(line: &NormalizedLine) -> Vec<&str> {
    line.text.split_whitespace().collect()
}

fn parse_i64(token: &str, line: usize, field: &'static str) -> Result<i64, FixedDateParseError> {
    token
        .parse::<i64>()
        .map_err(|_| FixedDateParseError::TokenParseError {
            line,
            field,
            value: token.to_string(),
        })
}

fn parse_f64(token: &str, line: usize, field: &'static str) -> Result<f64, FixedDateParseError> {
    token
        .parse::<f64>()
        .map_err(|_| FixedDateParseError::TokenParseError {
            line,
            field,
            value: token.to_string(),
        })
}

fn approx_eq(left: f64, right: f64) -> bool {
    (left - right).abs() < DATVER_EPSILON
}
