use openwepp_unit_boundary::TemperatureCelsius;
use std::error::Error;
use std::fmt;

use super::super::{DirectSnowAccumulationMeltDiagnostics, DirectSnowStage3Diagnostics};
use crate::constants::WB11_ZERO_THRESHOLD;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DirectSnowDiagnosticCapture {
    #[default]
    Disabled,
    Verbose,
}

impl DirectSnowDiagnosticCapture {
    #[must_use]
    pub const fn is_verbose(self) -> bool {
        matches!(self, Self::Verbose)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DirectSnowSolidToLiquidLedger {
    pub raw_signed_melt_m: f64,
    pub redistributed_positive_melt_m: f64,
    pub snowpack_swe_loss_m: f64,
    pub rain_released_m: f64,
    pub liquid_handoff_m: f64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DirectSnowLiquidDispositionLedger {
    pub incoming_liquid_m: f64,
    pub routed_liquid_m: f64,
    pub retained_liquid_delta_m: f64,
    pub refrozen_liquid_m: f64,
    pub liquid_closure_residual_m: f64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DirectSnowStage3Outcome {
    pub enabled: bool,
    pub meltwater_temperature_c: Option<TemperatureCelsius>,
    pub sublimation_m: f64,
}

/// Immutable, linked view of the two durable snow mass-transition ledgers and
/// the Stage-3 production outcome that governs their linkage semantics.
///
/// The component records remain public value types for diagnostic consumers,
/// but the persisted bundle exposes no mutable access. This prevents the
/// duplicated upstream handoff / downstream incoming operands from diverging
/// after the authoritative solve.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DirectSnowMassTransitionLedgers {
    solid_to_liquid: DirectSnowSolidToLiquidLedger,
    liquid_disposition: DirectSnowLiquidDispositionLedger,
    stage3_outcome: DirectSnowStage3Outcome,
}

pub(super) const SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M: f64 = 1.0e-9;
pub(super) const SNOW_STAGE3_LIQUID_CLOSURE_TOLERANCE_M: f64 = 1.0e-9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectSnowMassTransitionLedgerError {
    NonFinite { field: &'static str },
    Negative { field: &'static str },
    UpstreamClosure,
    Stage3HandoffLink,
    Stage3Closure,
    Stage3Outcome,
    DisabledStage3Ledger,
    DisabledStage3Outcome,
}

impl fmt::Display for DirectSnowMassTransitionLedgerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { field } => write!(formatter, "non-finite snow ledger field: {field}"),
            Self::Negative { field } => write!(formatter, "negative snow ledger field: {field}"),
            Self::UpstreamClosure => formatter.write_str("solid-to-liquid ledger does not close"),
            Self::Stage3HandoffLink => {
                formatter.write_str("Stage-3 incoming liquid does not match upstream handoff")
            }
            Self::Stage3Closure => formatter.write_str("Stage-3 liquid ledger does not close"),
            Self::Stage3Outcome => {
                formatter.write_str("Stage-3 outcome is inconsistent with routed liquid")
            }
            Self::DisabledStage3Ledger => {
                formatter.write_str("disabled Stage-3 outcome carries a nonzero liquid ledger")
            }
            Self::DisabledStage3Outcome => {
                formatter.write_str("disabled Stage-3 outcome carries production values")
            }
        }
    }
}

impl Error for DirectSnowMassTransitionLedgerError {}

impl DirectSnowMassTransitionLedgers {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            solid_to_liquid: DirectSnowSolidToLiquidLedger {
                raw_signed_melt_m: 0.0,
                redistributed_positive_melt_m: 0.0,
                snowpack_swe_loss_m: 0.0,
                rain_released_m: 0.0,
                liquid_handoff_m: 0.0,
            },
            liquid_disposition: DirectSnowLiquidDispositionLedger {
                incoming_liquid_m: 0.0,
                routed_liquid_m: 0.0,
                retained_liquid_delta_m: 0.0,
                refrozen_liquid_m: 0.0,
                liquid_closure_residual_m: 0.0,
            },
            stage3_outcome: DirectSnowStage3Outcome {
                enabled: false,
                meltwater_temperature_c: None,
                sublimation_m: 0.0,
            },
        }
    }

    #[must_use]
    pub(in crate::hydrology) const fn from_authoritative_parts(
        solid_to_liquid: DirectSnowSolidToLiquidLedger,
        liquid_disposition: DirectSnowLiquidDispositionLedger,
        stage3_outcome: DirectSnowStage3Outcome,
    ) -> Self {
        Self {
            solid_to_liquid,
            liquid_disposition,
            stage3_outcome,
        }
    }

    pub fn try_from_parts(
        solid_to_liquid: DirectSnowSolidToLiquidLedger,
        liquid_disposition: DirectSnowLiquidDispositionLedger,
        stage3_outcome: DirectSnowStage3Outcome,
    ) -> Result<Self, DirectSnowMassTransitionLedgerError> {
        let candidate = Self::from_authoritative_parts(
            solid_to_liquid,
            liquid_disposition,
            stage3_outcome,
        );
        candidate.validate()?;
        Ok(candidate)
    }

    pub fn validate(self) -> Result<(), DirectSnowMassTransitionLedgerError> {
        let solid = self.solid_to_liquid;
        for (field, value) in [
            ("raw_signed_melt_m", solid.raw_signed_melt_m),
            (
                "redistributed_positive_melt_m",
                solid.redistributed_positive_melt_m,
            ),
            ("snowpack_swe_loss_m", solid.snowpack_swe_loss_m),
            ("rain_released_m", solid.rain_released_m),
            ("liquid_handoff_m", solid.liquid_handoff_m),
        ] {
            if !value.is_finite() {
                return Err(DirectSnowMassTransitionLedgerError::NonFinite { field });
            }
        }
        for (field, value) in [
            (
                "redistributed_positive_melt_m",
                solid.redistributed_positive_melt_m,
            ),
            ("snowpack_swe_loss_m", solid.snowpack_swe_loss_m),
            ("rain_released_m", solid.rain_released_m),
            ("liquid_handoff_m", solid.liquid_handoff_m),
        ] {
            if value < 0.0 {
                return Err(DirectSnowMassTransitionLedgerError::Negative { field });
            }
        }
        if (solid.liquid_handoff_m - solid.snowpack_swe_loss_m - solid.rain_released_m).abs()
            > SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M
        {
            return Err(DirectSnowMassTransitionLedgerError::UpstreamClosure);
        }

        let disposition = self.liquid_disposition;
        for (field, value) in [
            ("incoming_liquid_m", disposition.incoming_liquid_m),
            ("routed_liquid_m", disposition.routed_liquid_m),
            (
                "retained_liquid_delta_m",
                disposition.retained_liquid_delta_m,
            ),
            ("refrozen_liquid_m", disposition.refrozen_liquid_m),
            (
                "liquid_closure_residual_m",
                disposition.liquid_closure_residual_m,
            ),
            ("stage3_sublimation_m", self.stage3_outcome.sublimation_m),
        ] {
            if !value.is_finite() {
                return Err(DirectSnowMassTransitionLedgerError::NonFinite { field });
            }
        }
        for (field, value) in [
            ("incoming_liquid_m", disposition.incoming_liquid_m),
            ("routed_liquid_m", disposition.routed_liquid_m),
            ("refrozen_liquid_m", disposition.refrozen_liquid_m),
            ("stage3_sublimation_m", self.stage3_outcome.sublimation_m),
        ] {
            if value < 0.0 {
                return Err(DirectSnowMassTransitionLedgerError::Negative { field });
            }
        }

        if !self.stage3_outcome.enabled {
            if disposition != DirectSnowLiquidDispositionLedger::default() {
                return Err(DirectSnowMassTransitionLedgerError::DisabledStage3Ledger);
            }
            if self.stage3_outcome.meltwater_temperature_c.is_some()
                || self.stage3_outcome.sublimation_m != 0.0
            {
                return Err(DirectSnowMassTransitionLedgerError::DisabledStage3Outcome);
            }
            return Ok(());
        }
        if (disposition.incoming_liquid_m - solid.liquid_handoff_m).abs()
            > SNOW_STAGE3_LIQUID_CLOSURE_TOLERANCE_M
        {
            return Err(DirectSnowMassTransitionLedgerError::Stage3HandoffLink);
        }
        let reconstructed_residual_m = disposition.incoming_liquid_m
            - disposition.routed_liquid_m
            - disposition.retained_liquid_delta_m
            - disposition.refrozen_liquid_m;
        if disposition.liquid_closure_residual_m.abs()
            > SNOW_STAGE3_LIQUID_CLOSURE_TOLERANCE_M
            || (reconstructed_residual_m - disposition.liquid_closure_residual_m).abs()
                > SNOW_STAGE3_LIQUID_CLOSURE_TOLERANCE_M
        {
            return Err(DirectSnowMassTransitionLedgerError::Stage3Closure);
        }
        if (disposition.routed_liquid_m > WB11_ZERO_THRESHOLD)
            != self.stage3_outcome.meltwater_temperature_c.is_some()
            || self
                .stage3_outcome
                .meltwater_temperature_c
                .is_some_and(|temperature| temperature.as_celsius() > 0.0)
        {
            return Err(DirectSnowMassTransitionLedgerError::Stage3Outcome);
        }
        Ok(())
    }

    #[must_use]
    pub const fn solid_to_liquid(self) -> DirectSnowSolidToLiquidLedger {
        self.solid_to_liquid
    }

    #[must_use]
    pub const fn liquid_disposition(self) -> DirectSnowLiquidDispositionLedger {
        self.liquid_disposition
    }

    #[must_use]
    pub const fn stage3_outcome(self) -> DirectSnowStage3Outcome {
        self.stage3_outcome
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSnowVerboseDiagnostics {
    pub accumulation_melt: DirectSnowAccumulationMeltDiagnostics,
    pub stage3: DirectSnowStage3Diagnostics,
}

#[derive(Debug, Clone)]
pub(crate) struct DirectSnowStage3Resolution {
    pub outcome: DirectSnowStage3Outcome,
    pub liquid_disposition_ledger: DirectSnowLiquidDispositionLedger,
    pub diagnostics: Option<DirectSnowStage3Diagnostics>,
}

impl DirectSnowStage3Resolution {
    #[must_use]
    pub(crate) fn disabled(capture: DirectSnowDiagnosticCapture) -> Self {
        Self {
            outcome: DirectSnowStage3Outcome::default(),
            liquid_disposition_ledger: DirectSnowLiquidDispositionLedger::default(),
            diagnostics: capture
                .is_verbose()
                .then(DirectSnowStage3Diagnostics::disabled),
        }
    }
}
