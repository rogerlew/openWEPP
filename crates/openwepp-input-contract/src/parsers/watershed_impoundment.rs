#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::too_many_lines
)]

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

const DATVER_MIN: f64 = 94.301;
const LEGACY_DATVER_THRESHOLD: f64 = 10.0;
const DEFAULT_MAX_IMPOUNDMENTS: usize = 25;
const FLOAT_TOLERANCE: f64 = 1e-9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WatershedImpoundmentParseOptions {
    pub mode: ParseMode,
    pub expected_structural_count: Option<usize>,
    pub max_impoundments: usize,
}

impl WatershedImpoundmentParseOptions {
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            mode: ParseMode::Strict,
            expected_structural_count: None,
            max_impoundments: DEFAULT_MAX_IMPOUNDMENTS,
        }
    }

    #[must_use]
    pub const fn compatibility() -> Self {
        Self {
            mode: ParseMode::Compatibility,
            expected_structural_count: None,
            max_impoundments: DEFAULT_MAX_IMPOUNDMENTS,
        }
    }
}

impl Default for WatershedImpoundmentParseOptions {
    fn default() -> Self {
        Self::strict()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImpWarningCode {
    ImpW001,
    ImpW002,
}

impl ImpWarningCode {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ImpW001 => "IMP-W-001",
            Self::ImpW002 => "IMP-W-002",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImpWarning {
    pub code: ImpWarningCode,
    pub line: usize,
    pub message: String,
}

impl ImpWarning {
    fn new(code: ImpWarningCode, line: usize, message: impl Into<String>) -> Self {
        Self {
            code,
            line,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructureFlags {
    pub has_drop_spillway: bool,
    pub has_culvert_1: bool,
    pub has_culvert_2: bool,
    pub has_rockfill: bool,
    pub has_emergency_spillway: bool,
    pub has_filter_barrier: bool,
    pub has_perforated_riser: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DropSpillwayIds1Payload {
    pub diars: f64,
    pub hrs: f64,
    pub coefw: f64,
    pub coefo: f64,
    pub diabl: f64,
    pub hrh: f64,
    pub lbl: f64,
    pub sbl: f64,
    pub hblot: f64,
    pub ke: f64,
    pub kb: f64,
    pub kc: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DropSpillwayIds2Payload {
    pub lenrs: f64,
    pub widrs: f64,
    pub hrs: f64,
    pub coefw: f64,
    pub coefo: f64,
    pub diabl: f64,
    pub hrh: f64,
    pub lbl: f64,
    pub sbl: f64,
    pub hblot: f64,
    pub ke: f64,
    pub kb: f64,
    pub kc: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DropSpillwayIds3Payload {
    pub lenrs: f64,
    pub widrs: f64,
    pub hrs: f64,
    pub coefw: f64,
    pub coefo: f64,
    pub hitbl: f64,
    pub wdbl: f64,
    pub hrh: f64,
    pub lbl: f64,
    pub sbl: f64,
    pub hblot: f64,
    pub ke: f64,
    pub kb: f64,
    pub kc: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DropSpillwayPayload {
    None,
    Ids1 {
        comment: String,
        payload: DropSpillwayIds1Payload,
    },
    Ids2 {
        comment: String,
        payload: DropSpillwayIds2Payload,
    },
    Ids3 {
        comment: String,
        payload: DropSpillwayIds3Payload,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct CulvertParameters {
    pub arcv: f64,
    pub hitcv: f64,
    pub hcv: f64,
    pub lcv: f64,
    pub scv: f64,
    pub hcvot: f64,
    pub ke: f64,
    pub kb: f64,
    pub kc: f64,
    pub kus: f64,
    pub mus: f64,
    pub cs: f64,
    pub ys: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CulvertPayload {
    pub icv: i32,
    pub ncv: i32,
    pub comment: Option<String>,
    pub parameters: Option<CulvertParameters>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RockfillPayload {
    pub comment: String,
    pub lnrf: f64,
    pub hrf: f64,
    pub hotrf: f64,
    pub wdrf: f64,
    pub diarf: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmergencyOpenChannelPayload {
    pub bwes: f64,
    pub sses: f64,
    pub nes: f64,
    pub hes: f64,
    pub hmxes: f64,
    pub ses1: f64,
    pub les1: f64,
    pub ses2: f64,
    pub les2: f64,
    pub ses3: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmergencyRatingCurvePayload {
    pub hes: f64,
    pub hest: Vec<f64>,
    pub qes: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencySpillwayPayload {
    None,
    OpenChannel {
        comment: String,
        payload: EmergencyOpenChannelPayload,
    },
    RatingCurve {
        comment: String,
        payload: EmergencyRatingCurvePayload,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FilterBarrierPayload {
    pub comment: String,
    pub vsl: f64,
    pub wdff: f64,
    pub hff: f64,
    pub hotff: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PerforatedRiserPayload {
    pub comment: String,
    pub hr: f64,
    pub hb: f64,
    pub hs: f64,
    pub hd: f64,
    pub diar: f64,
    pub as_slot: f64,
    pub diab: f64,
    pub hrh: f64,
    pub lbl: f64,
    pub sbl: f64,
    pub diabl: f64,
    pub cb: f64,
    pub coefw: f64,
    pub coefo: f64,
    pub cs: f64,
    pub ke: f64,
    pub kb: f64,
    pub kc: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImpoundmentRecord {
    pub description: [String; 3],
    pub branch_comments: Vec<String>,
    pub ids: i32,
    pub culvert_icv: [i32; 2],
    pub rockfill_code: i32,
    pub emergency_code: i32,
    pub filter_code: i32,
    pub riser_code: i32,
    pub hot: f64,
    pub hfull: f64,
    pub h: f64,
    pub deltat: f64,
    pub qinf: f64,
    pub isize: i32,
    pub ndiv: i32,
    pub nalpts: usize,
    pub hmin: f64,
    pub a0: f64,
    pub l0: f64,
    pub stage: Vec<f64>,
    pub area: Vec<f64>,
    pub length: Vec<f64>,
    pub drop_spillway: DropSpillwayPayload,
    pub culverts: [CulvertPayload; 2],
    pub rockfill: Option<RockfillPayload>,
    pub emergency_spillway: EmergencySpillwayPayload,
    pub filter_barrier: Option<FilterBarrierPayload>,
    pub perforated_riser: Option<PerforatedRiserPayload>,
    pub structure_flags: StructureFlags,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WatershedImpoundmentFile {
    pub datver: Option<f64>,
    pub datver_explicit: bool,
    pub declared_count: usize,
    pub expected_structural_count: Option<usize>,
    pub parsed_count: usize,
    pub surplus_ignored_count: usize,
    pub items: Vec<ImpoundmentRecord>,
    pub warnings: Vec<ImpWarning>,
}

#[derive(Debug)]
pub enum WatershedImpoundmentParseError {
    InputOpenError {
        path: PathBuf,
        source: std::io::Error,
    },
    TokenParseError {
        line: usize,
        field: &'static str,
        token: String,
    },
    UnexpectedEof {
        context: &'static str,
    },
    UnsupportedDatver {
        line: usize,
        observed: Option<f64>,
        reason: &'static str,
    },
    DomainError {
        line: usize,
        field: &'static str,
        value: String,
        allowed: &'static str,
    },
    BranchArityError {
        line: usize,
        context: &'static str,
        expected: usize,
        found: usize,
    },
    PhysicalDomainError {
        line: usize,
        field: &'static str,
        value: f64,
        expected: &'static str,
    },
    CountMismatch {
        line: usize,
        declared_jpond: usize,
        expected_npond: usize,
        reason: &'static str,
    },
    InvariantViolation {
        line: usize,
        context: &'static str,
    },
    OrderingMismatch {
        line: usize,
        context: &'static str,
    },
}

impl WatershedImpoundmentParseError {
    #[must_use]
    pub fn contract_error_id(&self) -> &'static str {
        match self {
            Self::InputOpenError { .. } => "IMP-E-000",
            Self::TokenParseError { .. } => "IMP-E-001",
            Self::UnexpectedEof { .. } => "IMP-E-002",
            Self::UnsupportedDatver { .. } => "IMP-E-003",
            Self::DomainError { .. } => "IMP-E-004",
            Self::BranchArityError { .. } => "IMP-E-005",
            Self::PhysicalDomainError { .. } => "IMP-E-006",
            Self::CountMismatch { .. } => "IMP-E-007",
            Self::InvariantViolation { .. } => "IMP-E-008",
            Self::OrderingMismatch { .. } => "IMP-E-009",
        }
    }
}

impl fmt::Display for WatershedImpoundmentParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOpenError { path, source } => write!(
                f,
                "{}: could not open/read '{}': {source}",
                self.contract_error_id(),
                path.display()
            ),
            Self::TokenParseError { line, field, token } => write!(
                f,
                "{}: line {line} token parse error for {field} from '{token}'",
                self.contract_error_id()
            ),
            Self::UnexpectedEof { context } => write!(
                f,
                "{}: unexpected end of file while parsing {context}",
                self.contract_error_id()
            ),
            Self::UnsupportedDatver {
                line,
                observed,
                reason,
            } => {
                if let Some(value) = observed {
                    write!(
                        f,
                        "{}: line {line} unsupported datver {value} ({reason})",
                        self.contract_error_id()
                    )
                } else {
                    write!(
                        f,
                        "{}: line {line} unsupported legacy no-datver preamble ({reason})",
                        self.contract_error_id()
                    )
                }
            }
            Self::DomainError {
                line,
                field,
                value,
                allowed,
            } => write!(
                f,
                "{}: line {line} invalid domain value '{value}' for {field}; expected {allowed}",
                self.contract_error_id()
            ),
            Self::BranchArityError {
                line,
                context,
                expected,
                found,
            } => write!(
                f,
                "{}: line {line} arity mismatch in {context}; expected {expected}, found {found}",
                self.contract_error_id()
            ),
            Self::PhysicalDomainError {
                line,
                field,
                value,
                expected,
            } => write!(
                f,
                "{}: line {line} physical-domain violation for {field}: {value} ({expected})",
                self.contract_error_id()
            ),
            Self::CountMismatch {
                line,
                declared_jpond,
                expected_npond,
                reason,
            } => write!(
                f,
                "{}: line {line} count mismatch jpond={declared_jpond} vs npond={expected_npond} ({reason})",
                self.contract_error_id()
            ),
            Self::InvariantViolation { line, context } => write!(
                f,
                "{}: line {line} invariant violation ({context})",
                self.contract_error_id()
            ),
            Self::OrderingMismatch { line, context } => write!(
                f,
                "{}: line {line} ordering mismatch ({context})",
                self.contract_error_id()
            ),
        }
    }
}

impl std::error::Error for WatershedImpoundmentParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InputOpenError { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn parse_watershed_impoundment_from_path(
    path: impl AsRef<Path>,
    options: WatershedImpoundmentParseOptions,
) -> Result<WatershedImpoundmentFile, WatershedImpoundmentParseError> {
    let path = path.as_ref();
    let input = fs::read_to_string(path).map_err(|source| {
        WatershedImpoundmentParseError::InputOpenError {
            path: path.to_path_buf(),
            source,
        }
    })?;

    parse_watershed_impoundment_from_str(&input, options)
}

pub fn parse_watershed_impoundment_from_str(
    input: &str,
    options: WatershedImpoundmentParseOptions,
) -> Result<WatershedImpoundmentFile, WatershedImpoundmentParseError> {
    let mut reader = LineReader::new(input);
    let mut warnings = Vec::new();

    let first = reader
        .next_data_line()
        .ok_or(WatershedImpoundmentParseError::UnexpectedEof {
            context: "preamble_first_line",
        })?;
    let first_tokens = tokenize(&first.text);
    if first_tokens.len() != 1 {
        return Err(WatershedImpoundmentParseError::BranchArityError {
            line: first.number,
            context: "preamble_first_line",
            expected: 1,
            found: first_tokens.len(),
        });
    }

    let first_value = parse_f64(first_tokens[0], first.number, "datver_or_jpond")?;

    let (datver, datver_explicit, declared_count, jpond_line) =
        if first_value > LEGACY_DATVER_THRESHOLD + FLOAT_TOLERANCE {
            if first_value + FLOAT_TOLERANCE < DATVER_MIN {
                return Err(WatershedImpoundmentParseError::UnsupportedDatver {
                    line: first.number,
                    observed: Some(first_value),
                    reason: "datver below minimum supported 94.301",
                });
            }

            let jpond_line =
                reader
                    .next_data_line()
                    .ok_or(WatershedImpoundmentParseError::UnexpectedEof {
                        context: "jpond_line",
                    })?;
            let jpond_tokens = tokenize(&jpond_line.text);
            if jpond_tokens.len() != 1 {
                return Err(WatershedImpoundmentParseError::BranchArityError {
                    line: jpond_line.number,
                    context: "jpond_line",
                    expected: 1,
                    found: jpond_tokens.len(),
                });
            }

            (
                Some(first_value),
                true,
                parse_usize_i32_like(jpond_tokens[0], jpond_line.number, "jpond")?,
                jpond_line.number,
            )
        } else {
            if options.mode == ParseMode::Strict {
                return Err(WatershedImpoundmentParseError::UnsupportedDatver {
                    line: first.number,
                    observed: None,
                    reason: "strict mode disallows legacy no-datver preamble",
                });
            }

            warnings.push(ImpWarning::new(
                ImpWarningCode::ImpW001,
                first.number,
                "compatibility accepted legacy no-datver preamble",
            ));

            (
                None,
                false,
                parse_usize_i32_like(first_tokens[0], first.number, "jpond")?,
                first.number,
            )
        };

    if declared_count > options.max_impoundments {
        return Err(WatershedImpoundmentParseError::DomainError {
            line: jpond_line,
            field: "jpond",
            value: declared_count.to_string(),
            allowed: "<= max_impoundments",
        });
    }

    let mut target_count = declared_count;
    if let Some(expected_npond) = options.expected_structural_count {
        if options.mode == ParseMode::Strict && declared_count != expected_npond {
            return Err(WatershedImpoundmentParseError::CountMismatch {
                line: jpond_line,
                declared_jpond: declared_count,
                expected_npond,
                reason: "strict mode requires exact jpond == npond",
            });
        }

        if options.mode == ParseMode::Compatibility {
            if declared_count < expected_npond {
                return Err(WatershedImpoundmentParseError::CountMismatch {
                    line: jpond_line,
                    declared_jpond: declared_count,
                    expected_npond,
                    reason: "compatibility still requires jpond >= npond",
                });
            }
            if declared_count > expected_npond {
                target_count = expected_npond;
                warnings.push(ImpWarning::new(
                    ImpWarningCode::ImpW002,
                    jpond_line,
                    format!(
                        "compatibility ignored surplus impoundments: jpond={declared_count} > npond={expected_npond}"
                    ),
                ));
            }
        }
    }

    if declared_count == 0 && options.expected_structural_count != Some(0) {
        return Err(WatershedImpoundmentParseError::DomainError {
            line: jpond_line,
            field: "jpond",
            value: declared_count.to_string(),
            allowed: ">= 1 unless watershed structure declares npond=0",
        });
    }

    let mut items = Vec::with_capacity(declared_count);
    for impoundment_index in 0..declared_count {
        let item = parse_impoundment(&mut reader, impoundment_index + 1)?;
        items.push(item);
    }

    let surplus_ignored_count = declared_count.saturating_sub(target_count);
    if surplus_ignored_count > 0 {
        items.truncate(target_count);
    }

    Ok(WatershedImpoundmentFile {
        datver,
        datver_explicit,
        declared_count,
        expected_structural_count: options.expected_structural_count,
        parsed_count: items.len(),
        surplus_ignored_count,
        items,
        warnings,
    })
}

fn parse_impoundment(
    reader: &mut LineReader,
    _item_index: usize,
) -> Result<ImpoundmentRecord, WatershedImpoundmentParseError> {
    let mut branch_comments = Vec::new();

    let desc1 =
        reader
            .next_nonempty_line()
            .ok_or(WatershedImpoundmentParseError::UnexpectedEof {
                context: "impdes_1",
            })?;
    let desc2 =
        reader
            .next_nonempty_line()
            .ok_or(WatershedImpoundmentParseError::UnexpectedEof {
                context: "impdes_2",
            })?;
    let desc3 =
        reader
            .next_nonempty_line()
            .ok_or(WatershedImpoundmentParseError::UnexpectedEof {
                context: "impdes_3",
            })?;

    let ids_line = consume_i32_line(reader, "ids", 1)?;
    let ids = ids_line.values[0];
    validate_enum(ids_line.number, "ids", ids, &[0, 1, 2, 3])?;
    let mut drop_spillway = DropSpillwayPayload::None;

    if ids != 0 {
        let strdes =
            reader
                .next_nonempty_line()
                .ok_or(WatershedImpoundmentParseError::UnexpectedEof {
                    context: "drop_spillway.strdes",
                })?;
        let comment = strdes.text;
        branch_comments.push(comment.clone());

        drop_spillway = match ids {
            1 => {
                let line1 = consume_f64_line(reader, "drop.ids1.line1", 4)?;
                let line2 = consume_f64_line(reader, "drop.ids1.line2", 5)?;
                let line3 = consume_f64_line(reader, "drop.ids1.line3", 3)?;
                DropSpillwayPayload::Ids1 {
                    comment,
                    payload: DropSpillwayIds1Payload {
                        diars: line1.values[0],
                        hrs: line1.values[1],
                        coefw: line1.values[2],
                        coefo: line1.values[3],
                        diabl: line2.values[0],
                        hrh: line2.values[1],
                        lbl: line2.values[2],
                        sbl: line2.values[3],
                        hblot: line2.values[4],
                        ke: line3.values[0],
                        kb: line3.values[1],
                        kc: line3.values[2],
                    },
                }
            }
            2 => {
                let line1 = consume_f64_line(reader, "drop.ids2.line1", 5)?;
                let line2 = consume_f64_line(reader, "drop.ids2.line2", 5)?;
                let line3 = consume_f64_line(reader, "drop.ids2.line3", 3)?;
                DropSpillwayPayload::Ids2 {
                    comment,
                    payload: DropSpillwayIds2Payload {
                        lenrs: line1.values[0],
                        widrs: line1.values[1],
                        hrs: line1.values[2],
                        coefw: line1.values[3],
                        coefo: line1.values[4],
                        diabl: line2.values[0],
                        hrh: line2.values[1],
                        lbl: line2.values[2],
                        sbl: line2.values[3],
                        hblot: line2.values[4],
                        ke: line3.values[0],
                        kb: line3.values[1],
                        kc: line3.values[2],
                    },
                }
            }
            3 => {
                let line1 = consume_f64_line(reader, "drop.ids3.line1", 5)?;
                let line2 = consume_f64_line(reader, "drop.ids3.line2", 6)?;
                let line3 = consume_f64_line(reader, "drop.ids3.line3", 3)?;
                DropSpillwayPayload::Ids3 {
                    comment,
                    payload: DropSpillwayIds3Payload {
                        lenrs: line1.values[0],
                        widrs: line1.values[1],
                        hrs: line1.values[2],
                        coefw: line1.values[3],
                        coefo: line1.values[4],
                        hitbl: line2.values[0],
                        wdbl: line2.values[1],
                        hrh: line2.values[2],
                        lbl: line2.values[3],
                        sbl: line2.values[4],
                        hblot: line2.values[5],
                        ke: line3.values[0],
                        kb: line3.values[1],
                        kc: line3.values[2],
                    },
                }
            }
            _ => unreachable!(),
        };
    }

    let culvert1 = parse_culvert(reader, 1, &mut branch_comments)?;
    let culvert2 = parse_culvert(reader, 2, &mut branch_comments)?;

    let irf_line = consume_i32_line(reader, "irf", 1)?;
    let rockfill_code = irf_line.values[0];
    validate_enum(irf_line.number, "irf", rockfill_code, &[0, 1])?;
    let mut rockfill = None;
    if rockfill_code != 0 {
        let strdes =
            reader
                .next_nonempty_line()
                .ok_or(WatershedImpoundmentParseError::UnexpectedEof {
                    context: "rockfill.strdes",
                })?;
        let comment = strdes.text;
        branch_comments.push(comment.clone());
        let payload = consume_f64_line(reader, "rockfill.payload", 5)?;
        rockfill = Some(RockfillPayload {
            comment,
            lnrf: payload.values[0],
            hrf: payload.values[1],
            hotrf: payload.values[2],
            wdrf: payload.values[3],
            diarf: payload.values[4],
        });
    }

    let ies_line = consume_i32_line(reader, "ies", 1)?;
    let emergency_code = ies_line.values[0];
    validate_enum(ies_line.number, "ies", emergency_code, &[0, 1, 2])?;
    let mut emergency_spillway = EmergencySpillwayPayload::None;
    match emergency_code {
        0 => {}
        1 => {
            let strdes = reader.next_nonempty_line().ok_or(
                WatershedImpoundmentParseError::UnexpectedEof {
                    context: "emergency.strdes",
                },
            )?;
            let comment = strdes.text;
            branch_comments.push(comment.clone());
            let line1 = consume_f64_line(reader, "emergency.open_channel.line1", 5)?;
            let line2 = consume_f64_line(reader, "emergency.open_channel.line2", 5)?;
            emergency_spillway = EmergencySpillwayPayload::OpenChannel {
                comment,
                payload: EmergencyOpenChannelPayload {
                    bwes: line1.values[0],
                    sses: line1.values[1],
                    nes: line1.values[2],
                    hes: line1.values[3],
                    hmxes: line1.values[4],
                    ses1: line2.values[0],
                    les1: line2.values[1],
                    ses2: line2.values[2],
                    les2: line2.values[3],
                    ses3: line2.values[4],
                },
            };
        }
        2 => {
            let strdes = reader.next_nonempty_line().ok_or(
                WatershedImpoundmentParseError::UnexpectedEof {
                    context: "emergency.rating.strdes",
                },
            )?;
            let comment = strdes.text;
            branch_comments.push(comment.clone());

            let npts_line = consume_i32_line(reader, "npts", 1)?;
            let npts_i32 = npts_line.values[0];
            if npts_i32 <= 0 {
                return Err(WatershedImpoundmentParseError::DomainError {
                    line: npts_line.number,
                    field: "npts",
                    value: npts_i32.to_string(),
                    allowed: ">= 1",
                });
            }
            let npts = usize::try_from(npts_i32).map_err(|_| {
                WatershedImpoundmentParseError::DomainError {
                    line: npts_line.number,
                    field: "npts",
                    value: npts_i32.to_string(),
                    allowed: "positive integer",
                }
            })?;

            let hes = consume_f64_line(reader, "emergency.rating.hes", 1)?;
            let hest = consume_f64_vector(reader, npts, "hest")?;
            let qes = consume_f64_vector(reader, npts, "qes")?;
            emergency_spillway = EmergencySpillwayPayload::RatingCurve {
                comment,
                payload: EmergencyRatingCurvePayload {
                    hes: hes.values[0],
                    hest,
                    qes,
                },
            };
        }
        _ => unreachable!(),
    }

    let iff_line = consume_i32_line(reader, "iff", 1)?;
    let filter_code = iff_line.values[0];
    validate_enum(iff_line.number, "iff", filter_code, &[0, 1])?;
    let mut filter_barrier = None;
    if filter_code != 0 {
        let strdes =
            reader
                .next_nonempty_line()
                .ok_or(WatershedImpoundmentParseError::UnexpectedEof {
                    context: "filter.strdes",
                })?;
        let comment = strdes.text;
        branch_comments.push(comment.clone());
        let payload = consume_f64_line(reader, "filter.payload", 4)?;
        filter_barrier = Some(FilterBarrierPayload {
            comment,
            vsl: payload.values[0],
            wdff: payload.values[1],
            hff: payload.values[2],
            hotff: payload.values[3],
        });
    }

    let ipr_line = consume_i32_line(reader, "ipr", 1)?;
    let riser_code = ipr_line.values[0];
    validate_enum(ipr_line.number, "ipr", riser_code, &[0, 1])?;
    let mut perforated_riser = None;
    if riser_code != 0 {
        let strdes =
            reader
                .next_nonempty_line()
                .ok_or(WatershedImpoundmentParseError::UnexpectedEof {
                    context: "riser.strdes",
                })?;
        let comment = strdes.text;
        branch_comments.push(comment.clone());
        let line1 = consume_f64_line(reader, "riser.payload.line1", 7)?;
        let line2 = consume_f64_line(reader, "riser.payload.line2", 4)?;
        let line3 = consume_f64_line(reader, "riser.payload.line3", 4)?;
        let line4 = consume_f64_line(reader, "riser.payload.line4", 3)?;
        perforated_riser = Some(PerforatedRiserPayload {
            comment,
            hr: line1.values[0],
            hb: line1.values[1],
            hs: line1.values[2],
            hd: line1.values[3],
            diar: line1.values[4],
            as_slot: line1.values[5],
            diab: line1.values[6],
            hrh: line2.values[0],
            lbl: line2.values[1],
            sbl: line2.values[2],
            diabl: line2.values[3],
            cb: line3.values[0],
            coefw: line3.values[1],
            coefo: line3.values[2],
            cs: line3.values[3],
            ke: line4.values[0],
            kb: line4.values[1],
            kc: line4.values[2],
        });
    }

    let misc_line = consume_f64_line(reader, "misc", 5)?;
    let hot = misc_line.values[0];
    let hfull = misc_line.values[1];
    let h = misc_line.values[2];
    let deltat = misc_line.values[3];
    let qinf = misc_line.values[4];

    if deltat <= 0.0 {
        return Err(WatershedImpoundmentParseError::PhysicalDomainError {
            line: misc_line.number,
            field: "deltat",
            value: deltat,
            expected: "> 0",
        });
    }
    if qinf < 0.0 {
        return Err(WatershedImpoundmentParseError::PhysicalDomainError {
            line: misc_line.number,
            field: "qinf",
            value: qinf,
            expected: ">= 0",
        });
    }
    if hot + FLOAT_TOLERANCE < h {
        return Err(WatershedImpoundmentParseError::InvariantViolation {
            line: misc_line.number,
            context: "hot must be >= h",
        });
    }

    let size_line = consume_i32_line(reader, "isize_ndiv", 2)?;
    let isize = size_line.values[0];
    let ndiv = size_line.values[1];
    if ndiv < 1 {
        return Err(WatershedImpoundmentParseError::DomainError {
            line: size_line.number,
            field: "ndiv",
            value: ndiv.to_string(),
            allowed: ">= 1",
        });
    }

    let nalpts_line = consume_i32_line(reader, "nalpts", 1)?;
    let nalpts_i32 = nalpts_line.values[0];
    if nalpts_i32 < 1 {
        return Err(WatershedImpoundmentParseError::DomainError {
            line: nalpts_line.number,
            field: "nalpts",
            value: nalpts_i32.to_string(),
            allowed: ">= 1",
        });
    }
    let nalpts =
        usize::try_from(nalpts_i32).map_err(|_| WatershedImpoundmentParseError::DomainError {
            line: nalpts_line.number,
            field: "nalpts",
            value: nalpts_i32.to_string(),
            allowed: "positive integer",
        })?;

    let curve0 = consume_f64_line(reader, "curve_baseline", 3)?;
    let hmin = curve0.values[0];
    let a0 = curve0.values[1];
    let l0 = curve0.values[2];

    let stage = consume_f64_vector(reader, nalpts, "hal")?;
    let area = consume_f64_vector(reader, nalpts, "area")?;
    let length = consume_f64_vector(reader, nalpts, "length")?;

    if hfull + FLOAT_TOLERANCE < hmin {
        return Err(WatershedImpoundmentParseError::InvariantViolation {
            line: curve0.number,
            context: "hfull must be >= hmin",
        });
    }

    for pair in stage.windows(2) {
        if pair[1] + FLOAT_TOLERANCE < pair[0] {
            return Err(WatershedImpoundmentParseError::InvariantViolation {
                line: nalpts_line.number,
                context: "stage array must be monotone non-decreasing",
            });
        }
    }

    let has_culvert_1 = culvert1.icv != 0;
    let has_culvert_2 = culvert2.icv != 0;

    Ok(ImpoundmentRecord {
        description: [desc1.text, desc2.text, desc3.text],
        branch_comments,
        ids,
        culvert_icv: [culvert1.icv, culvert2.icv],
        rockfill_code,
        emergency_code,
        filter_code,
        riser_code,
        hot,
        hfull,
        h,
        deltat,
        qinf,
        isize,
        ndiv,
        nalpts,
        hmin,
        a0,
        l0,
        stage,
        area,
        length,
        drop_spillway,
        culverts: [culvert1, culvert2],
        rockfill,
        emergency_spillway,
        filter_barrier,
        perforated_riser,
        structure_flags: StructureFlags {
            has_drop_spillway: ids != 0,
            has_culvert_1,
            has_culvert_2,
            has_rockfill: rockfill_code != 0,
            has_emergency_spillway: emergency_code != 0,
            has_filter_barrier: filter_code != 0,
            has_perforated_riser: riser_code != 0,
        },
    })
}

fn parse_culvert(
    reader: &mut LineReader,
    culvert_index: usize,
    branch_comments: &mut Vec<String>,
) -> Result<CulvertPayload, WatershedImpoundmentParseError> {
    let header_ctx = if culvert_index == 1 {
        "culvert1_header"
    } else {
        "culvert2_header"
    };
    let payload1_ctx = if culvert_index == 1 {
        "culvert1.payload.line1"
    } else {
        "culvert2.payload.line1"
    };
    let payload2_ctx = if culvert_index == 1 {
        "culvert1.payload.line2"
    } else {
        "culvert2.payload.line2"
    };

    let culvert_line = consume_i32_line(reader, header_ctx, 2)?;
    let icv = culvert_line.values[0];
    let ncv = culvert_line.values[1];

    validate_enum(culvert_line.number, "icv", icv, &[0, 1])?;
    if ncv < 0 {
        return Err(WatershedImpoundmentParseError::DomainError {
            line: culvert_line.number,
            field: "ncv",
            value: ncv.to_string(),
            allowed: ">= 0",
        });
    }

    let mut comment = None;
    let mut parameters = None;
    if icv >= 1 {
        let strdes =
            reader
                .next_nonempty_line()
                .ok_or(WatershedImpoundmentParseError::UnexpectedEof {
                    context: "culvert.strdes",
                })?;
        let strdes_text = strdes.text;
        branch_comments.push(strdes_text.clone());
        comment = Some(strdes_text);
        let line1 = consume_f64_line(reader, payload1_ctx, 6)?;
        let line2 = consume_f64_line(reader, payload2_ctx, 7)?;
        parameters = Some(CulvertParameters {
            arcv: line1.values[0],
            hitcv: line1.values[1],
            hcv: line1.values[2],
            lcv: line1.values[3],
            scv: line1.values[4],
            hcvot: line1.values[5],
            ke: line2.values[0],
            kb: line2.values[1],
            kc: line2.values[2],
            kus: line2.values[3],
            mus: line2.values[4],
            cs: line2.values[5],
            ys: line2.values[6],
        });
    }

    Ok(CulvertPayload {
        icv,
        ncv,
        comment,
        parameters,
    })
}

#[derive(Debug, Clone)]
struct IntLine {
    number: usize,
    values: Vec<i32>,
}

#[derive(Debug, Clone)]
struct FloatLine {
    number: usize,
    values: Vec<f64>,
}

fn consume_i32_line(
    reader: &mut LineReader,
    context: &'static str,
    expected: usize,
) -> Result<IntLine, WatershedImpoundmentParseError> {
    let line = reader
        .next_data_line()
        .ok_or(WatershedImpoundmentParseError::UnexpectedEof { context })?;
    let tokens = tokenize(&line.text);
    if tokens.len() != expected {
        return Err(WatershedImpoundmentParseError::BranchArityError {
            line: line.number,
            context,
            expected,
            found: tokens.len(),
        });
    }

    let mut values = Vec::with_capacity(expected);
    for token in &tokens {
        values.push(parse_i32(token, line.number, context)?);
    }

    Ok(IntLine {
        number: line.number,
        values,
    })
}

fn consume_f64_line(
    reader: &mut LineReader,
    context: &'static str,
    expected: usize,
) -> Result<FloatLine, WatershedImpoundmentParseError> {
    let line = reader
        .next_data_line()
        .ok_or(WatershedImpoundmentParseError::UnexpectedEof { context })?;
    let tokens = tokenize(&line.text);
    if tokens.len() != expected {
        return Err(WatershedImpoundmentParseError::BranchArityError {
            line: line.number,
            context,
            expected,
            found: tokens.len(),
        });
    }

    let mut values = Vec::with_capacity(expected);
    for token in &tokens {
        values.push(parse_f64(token, line.number, context)?);
    }

    Ok(FloatLine {
        number: line.number,
        values,
    })
}

fn consume_f64_vector(
    reader: &mut LineReader,
    required: usize,
    field: &'static str,
) -> Result<Vec<f64>, WatershedImpoundmentParseError> {
    let mut values = Vec::with_capacity(required);
    let mut last_line = 0usize;

    while values.len() < required {
        let line =
            reader
                .next_data_line()
                .ok_or(WatershedImpoundmentParseError::UnexpectedEof {
                    context: "vector_payload",
                })?;
        last_line = line.number;
        let tokens = tokenize(&line.text);
        for token in tokens {
            values.push(parse_f64(token, line.number, field)?);
        }
    }

    if values.len() > required {
        return Err(WatershedImpoundmentParseError::BranchArityError {
            line: last_line,
            context: field,
            expected: required,
            found: values.len(),
        });
    }

    Ok(values)
}

fn validate_enum(
    line: usize,
    field: &'static str,
    value: i32,
    allowed: &[i32],
) -> Result<(), WatershedImpoundmentParseError> {
    if allowed.contains(&value) {
        return Ok(());
    }

    let allowed_str = match field {
        "ids" => "0|1|2|3",
        "icv" | "irf" | "iff" | "ipr" => "0|1",
        "ies" => "0|1|2",
        _ => "valid enum domain",
    };

    Err(WatershedImpoundmentParseError::DomainError {
        line,
        field,
        value: value.to_string(),
        allowed: allowed_str,
    })
}

fn parse_f64(
    token: &str,
    line: usize,
    field: &'static str,
) -> Result<f64, WatershedImpoundmentParseError> {
    let parsed =
        token
            .parse::<f64>()
            .map_err(|_| WatershedImpoundmentParseError::TokenParseError {
                line,
                field,
                token: token.to_string(),
            })?;

    if !parsed.is_finite() {
        return Err(WatershedImpoundmentParseError::PhysicalDomainError {
            line,
            field,
            value: parsed,
            expected: "finite real number",
        });
    }

    Ok(parsed)
}

fn parse_i32(
    token: &str,
    line: usize,
    field: &'static str,
) -> Result<i32, WatershedImpoundmentParseError> {
    if let Ok(value) = token.parse::<i32>() {
        return Ok(value);
    }

    let value = parse_f64(token, line, field)?;
    let rounded = value.round();
    if (rounded - value).abs() > FLOAT_TOLERANCE {
        return Err(WatershedImpoundmentParseError::TokenParseError {
            line,
            field,
            token: token.to_string(),
        });
    }
    if !(f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&rounded) {
        return Err(WatershedImpoundmentParseError::TokenParseError {
            line,
            field,
            token: token.to_string(),
        });
    }

    let rounded_str = format!("{rounded:.0}");
    rounded_str
        .parse::<i32>()
        .map_err(|_| WatershedImpoundmentParseError::TokenParseError {
            line,
            field,
            token: token.to_string(),
        })
}

fn parse_usize_i32_like(
    token: &str,
    line: usize,
    field: &'static str,
) -> Result<usize, WatershedImpoundmentParseError> {
    let parsed = parse_i32(token, line, field)?;
    usize::try_from(parsed).map_err(|_| WatershedImpoundmentParseError::DomainError {
        line,
        field,
        value: parsed.to_string(),
        allowed: ">= 0",
    })
}

#[derive(Debug, Clone)]
struct Line {
    number: usize,
    text: String,
}

struct LineReader {
    lines: Vec<Line>,
    cursor: usize,
}

impl LineReader {
    fn new(input: &str) -> Self {
        let lines = input
            .lines()
            .enumerate()
            .map(|(idx, line)| Line {
                number: idx + 1,
                text: line.trim().to_string(),
            })
            .collect();

        Self { lines, cursor: 0 }
    }

    fn next_nonempty_line(&mut self) -> Option<Line> {
        while let Some(line) = self.lines.get(self.cursor) {
            self.cursor += 1;
            if !line.text.is_empty() {
                return Some(line.clone());
            }
        }
        None
    }

    fn next_data_line(&mut self) -> Option<Line> {
        while let Some(line) = self.next_nonempty_line() {
            if line.text.starts_with('#') || line.text.starts_with('!') {
                continue;
            }
            return Some(line);
        }
        None
    }
}

fn tokenize(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}
