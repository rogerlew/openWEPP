use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const ALLOWED_DATVERS: &[&str] = &["95.7", "98.4", "2016.3", "2017.1"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagementSectionCounts {
    pub ncrop: usize,
    pub nop: usize,
    pub nini: usize,
    pub nseq: usize,
    pub ncnt: usize,
    pub ndrain: usize,
    pub nscen: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagementScheduleSlot {
    pub rotation_index: usize,
    pub year_in_rotation: usize,
    pub ofe_index: usize,
    pub crop_slots: usize,
    pub yearly_refs: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagementSchedule {
    pub ofe_initial_refs: Vec<usize>,
    pub rotation_repeats: usize,
    pub rotation_years: usize,
    pub slots: Vec<ManagementScheduleSlot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagementParseOutput {
    pub datver: String,
    pub topology_count: usize,
    pub declared_total_years: usize,
    pub section_counts: ManagementSectionCounts,
    pub schedule: ManagementSchedule,
}

#[derive(Debug)]
pub enum ManagementParseError {
    InputOpenError {
        path: PathBuf,
        source: io::Error,
    },
    MissingRecord {
        field: &'static str,
    },
    TokenParseError {
        field: &'static str,
        value: String,
    },
    UnsupportedDatver {
        datver: String,
    },
    InvalidCount {
        field: &'static str,
        value: i64,
    },
    NonZeroScenarioSectionUnsupported {
        section: &'static str,
        count: usize,
    },
    DanglingScenarioReference {
        field: &'static str,
        value: usize,
        max_allowed: usize,
    },
    TotalYearMismatch {
        declared_total_years: usize,
        derived_total_years: usize,
    },
    TrailingInput {
        first_unconsumed_line: usize,
    },
}

impl ManagementParseError {
    pub fn contract_error_id(&self) -> &'static str {
        match self {
            Self::InputOpenError { .. } => "MAN-E-002",
            Self::MissingRecord { .. } => "MAN-E-002",
            Self::TokenParseError { .. } => "MAN-E-001",
            Self::UnsupportedDatver { .. } => "MAN-E-003",
            Self::InvalidCount { .. } => "MAN-E-005",
            Self::NonZeroScenarioSectionUnsupported { .. } => "MAN-E-002",
            Self::DanglingScenarioReference { .. } => "MAN-E-009",
            Self::TotalYearMismatch { .. } => "MAN-E-008",
            Self::TrailingInput { .. } => "MAN-E-006",
        }
    }
}

impl fmt::Display for ManagementParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOpenError { path, source } => write!(
                f,
                "{}: could not open {} ({source})",
                self.contract_error_id(),
                path.display()
            ),
            Self::MissingRecord { field } => write!(
                f,
                "{}: missing required record for {field}",
                self.contract_error_id()
            ),
            Self::TokenParseError { field, value } => write!(
                f,
                "{}: invalid token for {field}: {value}",
                self.contract_error_id()
            ),
            Self::UnsupportedDatver { datver } => write!(
                f,
                "{}: unsupported datver {datver}",
                self.contract_error_id()
            ),
            Self::InvalidCount { field, value } => write!(
                f,
                "{}: invalid count for {field}: {value}",
                self.contract_error_id()
            ),
            Self::NonZeroScenarioSectionUnsupported { section, count } => write!(
                f,
                "{}: non-zero {section} count ({count}) is not implemented in this worker package",
                self.contract_error_id()
            ),
            Self::DanglingScenarioReference {
                field,
                value,
                max_allowed,
            } => write!(
                f,
                "{}: {field} reference {value} exceeds max allowed {max_allowed}",
                self.contract_error_id()
            ),
            Self::TotalYearMismatch {
                declared_total_years,
                derived_total_years,
            } => write!(
                f,
                "{}: declared total years ({declared_total_years}) do not match derived schedule years ({derived_total_years})",
                self.contract_error_id()
            ),
            Self::TrailingInput {
                first_unconsumed_line,
            } => write!(
                f,
                "{}: trailing input starts at normalized line {}",
                self.contract_error_id(),
                first_unconsumed_line
            ),
        }
    }
}

impl std::error::Error for ManagementParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InputOpenError { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn parse_management_from_path(
    path: impl AsRef<Path>,
    mode: ParseMode,
) -> Result<ManagementParseOutput, ManagementParseError> {
    let path_ref = path.as_ref();
    let content =
        fs::read_to_string(path_ref).map_err(|source| ManagementParseError::InputOpenError {
            path: path_ref.to_path_buf(),
            source,
        })?;
    parse_management_from_str(&content, mode)
}

pub fn parse_management_from_str(
    input: &str,
    mode: ParseMode,
) -> Result<ManagementParseOutput, ManagementParseError> {
    let lines = normalize_lines(input);
    let mut cursor = Cursor::new(lines.as_slice(), mode);

    let datver_raw = cursor.next_required("datver")?;
    let datver = cursor.parse_token("datver", datver_raw)?;
    if !ALLOWED_DATVERS.contains(&datver.as_str()) {
        return Err(ManagementParseError::UnsupportedDatver { datver });
    }

    let topology_count = cursor.parse_non_negative_required("nofe_or_nchan")?;
    if topology_count == 0 {
        return Err(ManagementParseError::InvalidCount {
            field: "nofe_or_nchan",
            value: 0,
        });
    }
    let declared_total_years = cursor.parse_non_negative_required("total_years")?;
    if declared_total_years == 0 {
        return Err(ManagementParseError::InvalidCount {
            field: "total_years",
            value: 0,
        });
    }

    let section_counts = ManagementSectionCounts {
        ncrop: cursor.parse_non_negative_required("ncrop")?,
        nop: cursor.parse_non_negative_required("nop")?,
        nini: cursor.parse_non_negative_required("nini")?,
        nseq: cursor.parse_non_negative_required("nseq")?,
        ncnt: cursor.parse_non_negative_required("ncnt")?,
        ndrain: cursor.parse_non_negative_required("ndrain")?,
        nscen: cursor.parse_non_negative_required("nscen")?,
    };

    reject_nonzero_sections(&section_counts)?;

    let mut ofe_initial_refs = Vec::with_capacity(topology_count);
    for _ in 0..topology_count {
        let ofe_ref = cursor.parse_non_negative_required("ofeindx")?;
        if section_counts.nini == 0 {
            if ofe_ref != 0 {
                return Err(ManagementParseError::DanglingScenarioReference {
                    field: "ofeindx",
                    value: ofe_ref,
                    max_allowed: 0,
                });
            }
        } else if !(1..=section_counts.nini).contains(&ofe_ref) {
            return Err(ManagementParseError::DanglingScenarioReference {
                field: "ofeindx",
                value: ofe_ref,
                max_allowed: section_counts.nini,
            });
        }
        ofe_initial_refs.push(ofe_ref);
    }

    let rotation_repeats = cursor.parse_non_negative_required("nrots")?;
    if rotation_repeats == 0 {
        return Err(ManagementParseError::InvalidCount {
            field: "nrots",
            value: 0,
        });
    }
    let rotation_years = cursor.parse_non_negative_required("nyears")?;
    if rotation_years == 0 {
        return Err(ManagementParseError::InvalidCount {
            field: "nyears",
            value: 0,
        });
    }

    let derived_total_years = rotation_repeats.saturating_mul(rotation_years);
    if declared_total_years != derived_total_years {
        return Err(ManagementParseError::TotalYearMismatch {
            declared_total_years,
            derived_total_years,
        });
    }

    let mut slots = Vec::new();
    for rotation_index in 0..rotation_repeats {
        for year_in_rotation in 0..rotation_years {
            for ofe_index in 0..topology_count {
                let crop_slots = cursor.parse_non_negative_required("nycrop")?;
                let mut yearly_refs = Vec::with_capacity(crop_slots);
                for _ in 0..crop_slots {
                    let manindx = cursor.parse_non_negative_required("manindx")?;
                    if section_counts.nscen == 0 {
                        if manindx != 0 {
                            return Err(ManagementParseError::DanglingScenarioReference {
                                field: "manindx",
                                value: manindx,
                                max_allowed: 0,
                            });
                        }
                    } else if !(1..=section_counts.nscen).contains(&manindx) {
                        return Err(ManagementParseError::DanglingScenarioReference {
                            field: "manindx",
                            value: manindx,
                            max_allowed: section_counts.nscen,
                        });
                    }
                    yearly_refs.push(manindx);
                }

                slots.push(ManagementScheduleSlot {
                    rotation_index,
                    year_in_rotation,
                    ofe_index,
                    crop_slots,
                    yearly_refs,
                });
            }
        }
    }

    if let Some(first_unconsumed_line) = cursor.first_unconsumed_line_number() {
        return Err(ManagementParseError::TrailingInput {
            first_unconsumed_line,
        });
    }

    Ok(ManagementParseOutput {
        datver,
        topology_count,
        declared_total_years,
        section_counts,
        schedule: ManagementSchedule {
            ofe_initial_refs,
            rotation_repeats,
            rotation_years,
            slots,
        },
    })
}

fn reject_nonzero_sections(counts: &ManagementSectionCounts) -> Result<(), ManagementParseError> {
    let checks = [
        ("ncrop", counts.ncrop),
        ("nop", counts.nop),
        ("nini", counts.nini),
        ("nseq", counts.nseq),
        ("ncnt", counts.ncnt),
        ("ndrain", counts.ndrain),
        ("nscen", counts.nscen),
    ];

    for (section, count) in checks {
        if count != 0 {
            return Err(ManagementParseError::NonZeroScenarioSectionUnsupported { section, count });
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct NormalizedLine {
    line_no: usize,
    value: String,
}

fn normalize_lines(input: &str) -> Vec<NormalizedLine> {
    let mut output = Vec::new();
    for (line_idx, raw_line) in input.lines().enumerate() {
        let without_comment = match raw_line.split_once('#') {
            Some((prefix, _)) => prefix,
            None => raw_line,
        };
        let trimmed = without_comment.trim();
        if !trimmed.is_empty() {
            output.push(NormalizedLine {
                line_no: line_idx + 1,
                value: trimmed.to_string(),
            });
        }
    }
    output
}

struct Cursor<'a> {
    lines: &'a [NormalizedLine],
    mode: ParseMode,
    index: usize,
}

impl<'a> Cursor<'a> {
    fn new(lines: &'a [NormalizedLine], mode: ParseMode) -> Self {
        Self {
            lines,
            mode,
            index: 0,
        }
    }

    fn next_required(&mut self, field: &'static str) -> Result<&'a str, ManagementParseError> {
        let value = self
            .lines
            .get(self.index)
            .map(|line| line.value.as_str())
            .ok_or(ManagementParseError::MissingRecord { field })?;
        self.index += 1;
        Ok(value)
    }

    fn parse_token(&self, field: &'static str, raw: &str) -> Result<String, ManagementParseError> {
        let mut parts = raw.split_whitespace();
        let first = parts
            .next()
            .ok_or_else(|| ManagementParseError::TokenParseError {
                field,
                value: raw.to_string(),
            })?;
        if matches!(self.mode, ParseMode::Strict) && parts.next().is_some() {
            return Err(ManagementParseError::TokenParseError {
                field,
                value: raw.to_string(),
            });
        }
        Ok(first.to_string())
    }

    fn parse_non_negative_required(
        &mut self,
        field: &'static str,
    ) -> Result<usize, ManagementParseError> {
        let raw = self.next_required(field)?;
        let token = self.parse_token(field, raw)?;
        let value = token
            .parse::<i64>()
            .map_err(|_| ManagementParseError::TokenParseError {
                field,
                value: raw.to_string(),
            })?;
        if value < 0 {
            return Err(ManagementParseError::InvalidCount { field, value });
        }
        Ok(value as usize)
    }

    fn first_unconsumed_line_number(&self) -> Option<usize> {
        self.lines.get(self.index).map(|line| line.line_no)
    }
}
