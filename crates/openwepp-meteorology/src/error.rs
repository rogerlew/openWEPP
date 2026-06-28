use std::error::Error;
use std::fmt;

use openwepp_unit_boundary::BoundaryError;

/// Error raised by candidate meteorological primitive construction or solving.
#[derive(Debug, Clone, PartialEq)]
pub enum MeteorologyError {
    /// Unit-boundary wrapper rejected a value.
    Boundary(BoundaryError),
    /// A temperature would be at or below absolute zero.
    BelowAbsoluteZero {
        quantity: &'static str,
        value_c: f64,
    },
    /// A value expected to be strictly positive was zero or negative.
    NonPositive { quantity: &'static str, value: f64 },
    /// Solver options are outside the supported domain.
    InvalidSolverOptions { quantity: &'static str, value: f64 },
    /// The fixed-point hydrometeor-temperature solver did not converge.
    SolverDidNotConverge {
        iterations: usize,
        last_temperature_c: f64,
        last_delta_c: f64,
    },
    /// Monin-Obukhov turbulent-transfer iteration did not converge.
    TurbulentTransferDidNotConverge {
        iterations: usize,
        last_obukhov_length_m: f64,
        last_delta_m: f64,
    },
}

impl fmt::Display for MeteorologyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boundary(error) => write!(f, "{error}"),
            Self::BelowAbsoluteZero { quantity, value_c } => {
                write!(
                    f,
                    "{quantity} must be above absolute zero; received {value_c} degC"
                )
            }
            Self::NonPositive { quantity, value } => {
                write!(f, "{quantity} must be > 0; received {value}")
            }
            Self::InvalidSolverOptions { quantity, value } => {
                write!(
                    f,
                    "{quantity} is not a valid solver option; received {value}"
                )
            }
            Self::SolverDidNotConverge {
                iterations,
                last_temperature_c,
                last_delta_c,
            } => write!(
                f,
                "hydrometeor-temperature solver did not converge after {iterations} iterations; last temperature {last_temperature_c} degC, delta {last_delta_c} degC"
            ),
            Self::TurbulentTransferDidNotConverge {
                iterations,
                last_obukhov_length_m,
                last_delta_m,
            } => write!(
                f,
                "turbulent-transfer solver did not converge after {iterations} iterations; last Obukhov length {last_obukhov_length_m} m, delta {last_delta_m} m"
            ),
        }
    }
}

impl Error for MeteorologyError {}

impl From<BoundaryError> for MeteorologyError {
    fn from(error: BoundaryError) -> Self {
        Self::Boundary(error)
    }
}
