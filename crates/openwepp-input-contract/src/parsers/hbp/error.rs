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

#[cfg(test)]
mod m04_tests {
    use std::collections::HashSet;
    use std::error::Error;

    use super::*;

    #[test]
    fn all_format_error_code_mappings_are_exact_and_unique() {
        let cases = [
            (HbpFormatErrorCode::HbpE002, "HBP-E-002"),
            (HbpFormatErrorCode::HbpE003, "HBP-E-003"),
            (HbpFormatErrorCode::HbpE004, "HBP-E-004"),
            (HbpFormatErrorCode::HbpE005, "HBP-E-005"),
            (HbpFormatErrorCode::HbpE006, "HBP-E-006"),
            (HbpFormatErrorCode::HbpE007, "HBP-E-007"),
            (HbpFormatErrorCode::HbpE008, "HBP-E-008"),
            (HbpFormatErrorCode::HbpE009, "HBP-E-009"),
            (HbpFormatErrorCode::HbpE010, "HBP-E-010"),
            (HbpFormatErrorCode::HbpE011, "HBP-E-011"),
            (HbpFormatErrorCode::HbpE012, "HBP-E-012"),
            (HbpFormatErrorCode::HbpE013, "HBP-E-013"),
            (HbpFormatErrorCode::HbpE014, "HBP-E-014"),
            (HbpFormatErrorCode::HbpE015, "HBP-E-015"),
        ];
        let mut unique = HashSet::new();
        for (code, expected) in cases {
            assert_eq!(code.as_str(), expected);
            assert!(unique.insert(code.as_str()));
        }
        assert_eq!(unique.len(), 14);
    }

    #[test]
    fn parse_error_ids_display_and_sources_are_exact() {
        let errors = [
            HbpParseError::InputOpenError {
                path: PathBuf::from("H1.hbp"),
                source: io::Error::new(io::ErrorKind::PermissionDenied, "denied"),
            },
            HbpParseError::InvalidProcessHbpName {
                input_path: PathBuf::from("H1.pass.hbp"),
                reason: "reserved suffix".to_string(),
            },
            HbpParseError::HillslopeIdMismatch {
                expected: 7,
                found: 8,
            },
            HbpParseError::FormatViolation {
                code: HbpFormatErrorCode::HbpE015,
                detail: "latest event payload mismatch".to_string(),
            },
        ];
        let expected_ids = ["HBP-E-000", "HBP-E-001", "HBP-E-014", "HBP-E-015"];
        let expected_display = [
            "HBP-E-000: failed to open/read HBP shard 'H1.hbp': denied",
            "HBP-E-001: invalid process HBP name 'H1.pass.hbp': reserved suffix",
            "HBP-E-014: hillslope id mismatch (expected 7, found 8)",
            "HBP-E-015: latest event payload mismatch",
        ];
        for ((error, expected_id), expected_display) in
            errors.iter().zip(expected_ids).zip(expected_display)
        {
            assert_eq!(error.contract_error_id(), expected_id);
            assert_eq!(error.to_string(), expected_display);
        }
        assert!(errors[0].source().is_some());
        for error in &errors[1..] {
            assert!(error.source().is_none());
        }
    }
}
