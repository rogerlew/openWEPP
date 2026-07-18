use std::error::Error;
use std::fmt;

/// Stable failure categories emitted by the shadow planner and verifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorClass {
    Io,
    Json,
    Schema,
    Identity,
    GitState,
    CargoMetadata,
    Policy,
    Planning,
    Receipt,
    Trust,
    Ledger,
    Cli,
}

/// Typed, fail-closed gate-policy error.
#[derive(Debug)]
pub struct GatePolicyError {
    pub class: ErrorClass,
    pub code: &'static str,
    pub message: String,
}

impl GatePolicyError {
    #[must_use]
    pub fn new(class: ErrorClass, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            class,
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for GatePolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl Error for GatePolicyError {}

pub type Result<T> = std::result::Result<T, GatePolicyError>;
