use std::error::Error;
use std::fmt;

/// Typed errors for parser-to-watershed runtime input adaptation.
#[derive(Debug, Clone, PartialEq)]
pub enum WatershedRuntimeInputError {
    ImpoundmentSymbolNonFinite {
        symbol: String,
        value: f64,
    },
    ImpoundmentSymbolOutOfDomain {
        symbol: String,
        value: f64,
        rule: &'static str,
    },
}

impl WatershedRuntimeInputError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::ImpoundmentSymbolNonFinite { .. } => "WS-RUNTIME-E-011",
            Self::ImpoundmentSymbolOutOfDomain { .. } => "WS-RUNTIME-E-012",
        }
    }
}

impl fmt::Display for WatershedRuntimeInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImpoundmentSymbolNonFinite { symbol, value } => write!(
                f,
                "{}: impoundment runtime symbol {} is non-finite ({})",
                self.code(),
                symbol,
                value
            ),
            Self::ImpoundmentSymbolOutOfDomain {
                symbol,
                value,
                rule,
            } => write!(
                f,
                "{}: impoundment runtime symbol {}={} violates {}",
                self.code(),
                symbol,
                value,
                rule
            ),
        }
    }
}

impl Error for WatershedRuntimeInputError {}
