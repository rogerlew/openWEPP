use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HbpFormatErrorCode {
    HbpE002,
    HbpE003,
    HbpE004,
    HbpE005,
    HbpE006,
    HbpE007,
    HbpE008,
    HbpE009,
    HbpE010,
    HbpE011,
    HbpE012,
    HbpE013,
    HbpE014,
    HbpE015,
}

impl HbpFormatErrorCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HbpE002 => "HBP-E-002",
            Self::HbpE003 => "HBP-E-003",
            Self::HbpE004 => "HBP-E-004",
            Self::HbpE005 => "HBP-E-005",
            Self::HbpE006 => "HBP-E-006",
            Self::HbpE007 => "HBP-E-007",
            Self::HbpE008 => "HBP-E-008",
            Self::HbpE009 => "HBP-E-009",
            Self::HbpE010 => "HBP-E-010",
            Self::HbpE011 => "HBP-E-011",
            Self::HbpE012 => "HBP-E-012",
            Self::HbpE013 => "HBP-E-013",
            Self::HbpE014 => "HBP-E-014",
            Self::HbpE015 => "HBP-E-015",
        }
    }
}

#[derive(Debug)]
pub enum HbpParseError {
    InputOpenError {
        path: PathBuf,
        source: io::Error,
    },
    InvalidProcessHbpName {
        input_path: PathBuf,
        reason: String,
    },
    HillslopeIdMismatch {
        expected: u32,
        found: u32,
    },
    FormatViolation {
        code: HbpFormatErrorCode,
        detail: String,
    },
}

impl HbpParseError {
    #[must_use]
    pub const fn contract_error_id(&self) -> &'static str {
        match self {
            Self::InputOpenError { .. } => "HBP-E-000",
            Self::InvalidProcessHbpName { .. } => "HBP-E-001",
            Self::HillslopeIdMismatch { .. } => "HBP-E-014",
            Self::FormatViolation { code, .. } => code.as_str(),
        }
    }
}

impl fmt::Display for HbpParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOpenError { path, source } => write!(
                f,
                "{}: failed to open/read HBP shard '{}': {}",
                self.contract_error_id(),
                path.display(),
                source
            ),
            Self::InvalidProcessHbpName { input_path, reason } => write!(
                f,
                "{}: invalid process HBP name '{}': {}",
                self.contract_error_id(),
                input_path.display(),
                reason
            ),
            Self::HillslopeIdMismatch { expected, found } => write!(
                f,
                "{}: hillslope id mismatch (expected {}, found {})",
                self.contract_error_id(),
                expected,
                found
            ),
            Self::FormatViolation { code, detail } => {
                write!(f, "{}: {}", code.as_str(), detail)
            }
        }
    }
}

impl std::error::Error for HbpParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InputOpenError { source, .. } => Some(source),
            Self::InvalidProcessHbpName { .. }
            | Self::HillslopeIdMismatch { .. }
            | Self::FormatViolation { .. } => None,
        }
    }
}
