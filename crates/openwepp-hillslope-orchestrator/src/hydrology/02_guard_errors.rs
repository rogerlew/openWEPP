use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct SnowStage3ConductivityError {
    pub phase_class: HillslopeKernelPhaseClass,
    pub source: openwepp_meteorology::MeteorologyError,
    pub layer_index: usize,
    pub layer: DirectSnowLayerState,
    pub control_volume_layers: Vec<DirectSnowLayerState>,
    pub control_volume_temperature: openwepp_unit_boundary::TemperatureCelsius,
    pub atmospheric_pressure_pa: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SnowLayerAggregateMismatchError {
    pub phase_class: HillslopeKernelPhaseClass,
    pub symbol: &'static str,
    pub value: f64,
    pub expected: f64,
    pub prior_swe_m: f64,
    pub prior_depth_m: f64,
    pub prior_layers: Vec<DirectSnowLayerState>,
}

impl SnowLayerAggregateMismatchError {
    #[must_use]
    pub fn replay_value(&self) -> f64 {
        let retained_layers = self
            .prior_layers
            .iter()
            .filter(|layer| snow_density_layer_has_resolved_mass(layer.mass_swe_m));
        match self.symbol {
            "prior_layers.mass_swe_m" => retained_layers.map(|layer| layer.mass_swe_m).sum(),
            "prior_layers.thickness_m" => {
                retained_layers.map(|layer| layer.thickness_m).sum()
            }
            _ => f64::NAN,
        }
    }
}

/// Typed guard failures for WB11 hydrology production kernels.
#[derive(Debug, Clone, PartialEq)]
pub enum Wb11HydrologyKernelGuardError {
    SnowStage3Conductivity(Box<SnowStage3ConductivityError>),
    SnowLayerAggregateMismatch(Box<SnowLayerAggregateMismatchError>),
    MissingRequiredStateSymbol {
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
    },
    MissingRequiredFluxSymbol {
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
    },
    NonFiniteStateSymbol {
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        value: f64,
    },
    NonFiniteFluxSymbol {
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        value: f64,
    },
    StateSymbolOutOfRange {
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    },
    FluxSymbolOutOfRange {
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    },
    Erod13MissingRequiredSymbol {
        symbol: BoundarySymbol,
    },
    Erod13NonFiniteSymbol {
        symbol: BoundarySymbol,
        value: f64,
    },
    Erod13DomainViolation {
        symbol: BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    },
    Erod14MissingRequiredSymbol {
        symbol: BoundarySymbol,
    },
    Erod14NonFiniteSymbol {
        symbol: BoundarySymbol,
        value: f64,
    },
    Erod14DomainViolation {
        symbol: BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    },
    Erod18MissingRequiredSymbol {
        symbol: BoundarySymbol,
    },
    Erod18NonFiniteSymbol {
        symbol: BoundarySymbol,
        value: f64,
    },
    Erod18DomainViolation {
        symbol: BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    },
}

impl Wb11HydrologyKernelGuardError {
    #[must_use]
    pub const fn boundary_class(&self) -> BoundaryClass {
        match self {
            Self::MissingRequiredStateSymbol { .. }
            | Self::MissingRequiredFluxSymbol { .. }
            | Self::Erod13MissingRequiredSymbol { .. }
            | Self::Erod14MissingRequiredSymbol { .. }
            | Self::Erod18MissingRequiredSymbol { .. } => BoundaryClass::MissingRequiredInput,
            Self::NonFiniteStateSymbol { .. }
            | Self::NonFiniteFluxSymbol { .. }
            | Self::Erod13NonFiniteSymbol { .. }
            | Self::Erod14NonFiniteSymbol { .. }
            | Self::Erod18NonFiniteSymbol { .. } => BoundaryClass::NonFinite,
            Self::StateSymbolOutOfRange { .. }
            | Self::FluxSymbolOutOfRange { .. }
            | Self::SnowStage3Conductivity(_)
            | Self::SnowLayerAggregateMismatch(_)
            | Self::Erod13DomainViolation { .. }
            | Self::Erod14DomainViolation { .. }
            | Self::Erod18DomainViolation { .. } => BoundaryClass::DomainViolation,
        }
    }

    #[must_use]
    pub fn code(&self) -> String {
        match self {
            Self::SnowStage3Conductivity(_) | Self::SnowLayerAggregateMismatch(_) => {
                return String::from("HKERNEL-WB14-RUNOFF-E-003");
            }
            Self::Erod13MissingRequiredSymbol { .. } => {
                return String::from("HKERNEL-EROD13-CORE-E-001");
            }
            Self::Erod13NonFiniteSymbol { .. } => {
                return String::from("HKERNEL-EROD13-CORE-E-002");
            }
            Self::Erod13DomainViolation { .. } => {
                return String::from("HKERNEL-EROD13-CORE-E-003");
            }
            Self::Erod14MissingRequiredSymbol { .. } => {
                return String::from("HKERNEL-EROD14-WAVE2-E-001");
            }
            Self::Erod14NonFiniteSymbol { .. } => {
                return String::from("HKERNEL-EROD14-WAVE2-E-002");
            }
            Self::Erod14DomainViolation { .. } => {
                return String::from("HKERNEL-EROD14-WAVE2-E-003");
            }
            Self::Erod18MissingRequiredSymbol { .. } => {
                return String::from("HKERNEL-EROD18-ROUTE-E-001");
            }
            Self::Erod18NonFiniteSymbol { .. } => {
                return String::from("HKERNEL-EROD18-ROUTE-E-002");
            }
            Self::Erod18DomainViolation { .. } => {
                return String::from("HKERNEL-EROD18-ROUTE-E-003");
            }
            _ => {}
        }
        let (phase_class, suffix) = match self {
            Self::MissingRequiredStateSymbol { phase_class, .. }
            | Self::MissingRequiredFluxSymbol { phase_class, .. } => (phase_class, "001"),
            Self::NonFiniteStateSymbol { phase_class, .. }
            | Self::NonFiniteFluxSymbol { phase_class, .. } => (phase_class, "002"),
            Self::StateSymbolOutOfRange { phase_class, .. }
            | Self::FluxSymbolOutOfRange { phase_class, .. } => (phase_class, "003"),
            Self::SnowStage3Conductivity(_)
            | Self::SnowLayerAggregateMismatch(_)
            | Self::Erod13MissingRequiredSymbol { .. }
            | Self::Erod13NonFiniteSymbol { .. }
            | Self::Erod13DomainViolation { .. }
            | Self::Erod14MissingRequiredSymbol { .. }
            | Self::Erod14NonFiniteSymbol { .. }
            | Self::Erod14DomainViolation { .. }
            | Self::Erod18MissingRequiredSymbol { .. }
            | Self::Erod18NonFiniteSymbol { .. }
            | Self::Erod18DomainViolation { .. } => unreachable!(),
        };

        let (kernel_family, phase_prefix) = match phase_class {
            HillslopeKernelPhaseClass::HydrologyEvapotranspiration => ("WB11", "ET"),
            HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage => ("WB11", "PERC"),
            HillslopeKernelPhaseClass::HydrologyLateralTransfer => ("WB11", "LAT"),
            HillslopeKernelPhaseClass::HydrologyDrainage => ("WB11", "DRAIN"),
            HillslopeKernelPhaseClass::HydrologyPlantRootUptake => ("WB17", "SWU"),
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation => ("WB14", "RUNOFF"),
            HillslopeKernelPhaseClass::HydrologyStorageReconciliation => ("WB12", "STORAGE"),
            HillslopeKernelPhaseClass::HydrologyPeakRunoff => ("WB16", "PEAK"),
            _ => ("WB11", "GEN"),
        };

        format!("HKERNEL-{kernel_family}-{phase_prefix}-E-{suffix}")
    }

    fn display_parts(&self) -> HydrologyGuardErrorDisplayParts<'_> {
        match self {
            Self::SnowStage3Conductivity(_) | Self::SnowLayerAggregateMismatch(_) => {
                unreachable!("snow diagnostic errors use their typed display")
            }
            Self::MissingRequiredStateSymbol { .. }
            | Self::MissingRequiredFluxSymbol { .. }
            | Self::NonFiniteStateSymbol { .. }
            | Self::NonFiniteFluxSymbol { .. }
            | Self::StateSymbolOutOfRange { .. }
            | Self::FluxSymbolOutOfRange { .. } => self.phase_display_parts(),
            Self::Erod13MissingRequiredSymbol { .. }
            | Self::Erod13NonFiniteSymbol { .. }
            | Self::Erod13DomainViolation { .. } => self.erod13_display_parts(),
            Self::Erod14MissingRequiredSymbol { .. }
            | Self::Erod14NonFiniteSymbol { .. }
            | Self::Erod14DomainViolation { .. } => self.erod14_display_parts(),
            Self::Erod18MissingRequiredSymbol { .. }
            | Self::Erod18NonFiniteSymbol { .. }
            | Self::Erod18DomainViolation { .. } => self.erod18_display_parts(),
        }
    }

    fn phase_display_parts(&self) -> HydrologyGuardErrorDisplayParts<'_> {
        match self {
            Self::MissingRequiredStateSymbol {
                phase_class,
                symbol,
            } => HydrologyGuardErrorDisplayParts::PhaseMissing(phase_class, symbol, "state"),
            Self::MissingRequiredFluxSymbol {
                phase_class,
                symbol,
            } => HydrologyGuardErrorDisplayParts::PhaseMissing(phase_class, symbol, "flux"),
            Self::NonFiniteStateSymbol {
                phase_class,
                symbol,
                value,
            } => HydrologyGuardErrorDisplayParts::PhaseNonFinite(phase_class, symbol, *value, "state"),
            Self::NonFiniteFluxSymbol {
                phase_class,
                symbol,
                value,
            } => HydrologyGuardErrorDisplayParts::PhaseNonFinite(phase_class, symbol, *value, "flux"),
            Self::StateSymbolOutOfRange {
                phase_class,
                symbol,
                value,
                minimum,
                maximum,
            } => HydrologyGuardErrorDisplayParts::PhaseOutOfRange(
                phase_class,
                symbol,
                *value,
                *minimum,
                *maximum,
                "state",
            ),
            Self::FluxSymbolOutOfRange {
                phase_class,
                symbol,
                value,
                minimum,
                maximum,
            } => HydrologyGuardErrorDisplayParts::PhaseOutOfRange(
                phase_class,
                symbol,
                *value,
                *minimum,
                *maximum,
                "flux",
            ),
            _ => unreachable!("phase display mapper received erosion guard error"),
        }
    }

    fn erod13_display_parts(&self) -> HydrologyGuardErrorDisplayParts<'_> {
        match self {
            Self::Erod13MissingRequiredSymbol { symbol } => {
                HydrologyGuardErrorDisplayParts::ErodMissing(symbol, "EROD13 Wave-1")
            }
            Self::Erod13NonFiniteSymbol { symbol, value } => {
                HydrologyGuardErrorDisplayParts::ErodNonFinite(symbol, *value, "EROD13 Wave-1")
            }
            Self::Erod13DomainViolation {
                symbol,
                value,
                minimum,
                maximum,
            } => HydrologyGuardErrorDisplayParts::ErodOutOfRange(
                symbol,
                *value,
                *minimum,
                *maximum,
                "EROD13 Wave-1",
            ),
            _ => unreachable!("EROD13 display mapper received non-EROD13 guard error"),
        }
    }

    fn erod14_display_parts(&self) -> HydrologyGuardErrorDisplayParts<'_> {
        match self {
            Self::Erod14MissingRequiredSymbol { symbol } => {
                HydrologyGuardErrorDisplayParts::ErodMissing(symbol, "EROD14 Wave-2")
            }
            Self::Erod14NonFiniteSymbol { symbol, value } => {
                HydrologyGuardErrorDisplayParts::ErodNonFinite(symbol, *value, "EROD14 Wave-2")
            }
            Self::Erod14DomainViolation {
                symbol,
                value,
                minimum,
                maximum,
            } => HydrologyGuardErrorDisplayParts::ErodOutOfRange(
                symbol,
                *value,
                *minimum,
                *maximum,
                "EROD14 Wave-2",
            ),
            _ => unreachable!("EROD14 display mapper received non-EROD14 guard error"),
        }
    }

    fn erod18_display_parts(&self) -> HydrologyGuardErrorDisplayParts<'_> {
        match self {
            Self::Erod18MissingRequiredSymbol { symbol } => {
                HydrologyGuardErrorDisplayParts::ErodMissing(symbol, "EROD18 route topology")
            }
            Self::Erod18NonFiniteSymbol { symbol, value } => {
                HydrologyGuardErrorDisplayParts::ErodNonFinite(
                    symbol,
                    *value,
                    "EROD18 route topology",
                )
            }
            Self::Erod18DomainViolation {
                symbol,
                value,
                minimum,
                maximum,
            } => HydrologyGuardErrorDisplayParts::ErodOutOfRange(
                symbol,
                *value,
                *minimum,
                *maximum,
                "EROD18 route topology",
            ),
            _ => unreachable!("EROD18 display mapper received non-EROD18 guard error"),
        }
    }
}

enum HydrologyGuardErrorDisplayParts<'a> {
    PhaseMissing(&'a HillslopeKernelPhaseClass, &'a BoundarySymbol, &'static str),
    PhaseNonFinite(&'a HillslopeKernelPhaseClass, &'a BoundarySymbol, f64, &'static str),
    PhaseOutOfRange(
        &'a HillslopeKernelPhaseClass,
        &'a BoundarySymbol,
        f64,
        Option<f64>,
        Option<f64>,
        &'static str,
    ),
    ErodMissing(&'a BoundarySymbol, &'static str),
    ErodNonFinite(&'a BoundarySymbol, f64, &'static str),
    ErodOutOfRange(&'a BoundarySymbol, f64, Option<f64>, Option<f64>, &'static str),
}

impl HydrologyGuardErrorDisplayParts<'_> {
    fn fmt_with_code(&self, code: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PhaseMissing(
                phase_class,
                symbol,
                symbol_kind,
            ) => write!(
                f,
                "{code}: phase class {} missing required {symbol_kind} symbol {symbol}",
                phase_class.as_str()
            ),
            Self::PhaseNonFinite(
                phase_class,
                symbol,
                value,
                symbol_kind,
            ) => write!(
                f,
                "{code}: phase class {} {symbol_kind} symbol {symbol} is non-finite ({value})",
                phase_class.as_str()
            ),
            Self::PhaseOutOfRange(
                phase_class,
                symbol,
                value,
                minimum,
                maximum,
                symbol_kind,
            ) => write!(
                f,
                "{code}: phase class {} {symbol_kind} symbol {symbol}={value} outside [{minimum:?}, {maximum:?}]",
                phase_class.as_str()
            ),
            Self::ErodMissing(symbol, label) => {
                write!(f, "{code}: missing required {label} symbol {symbol}")
            }
            Self::ErodNonFinite(symbol, value, label) => {
                write!(f, "{code}: non-finite {label} symbol {symbol} ({value})")
            }
            Self::ErodOutOfRange(symbol, value, minimum, maximum, label) => write!(
                f,
                "{code}: {label} symbol {symbol}={value} outside [{minimum:?}, {maximum:?}]"
            ),
        }
    }
}

impl fmt::Display for Wb11HydrologyKernelGuardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.code();
        match self {
            Self::SnowStage3Conductivity(snapshot) => {
                let SnowStage3ConductivityError {
                phase_class,
                source,
                layer_index,
                layer,
                control_volume_layers,
                control_volume_temperature,
                atmospheric_pressure_pa,
            } = snapshot.as_ref();
                write!(
                f,
                "{code}: phase class {} snow Stage 3 conductivity evaluation failed: {source}; \
                 layer_index={layer_index}, layer_mass_swe_m={}, layer_thickness_m={}, \
                 layer_density_kg_m3={}, layer_temperature_c={}, layer_cold_content_j_m2={}, \
                 control_volume_temperature_c={}, atmospheric_pressure_pa={}, \
                 control_volume_layers={control_volume_layers:?}",
                phase_class.as_str(),
                layer.mass_swe_m,
                layer.thickness_m,
                layer.density_kg_m3,
                layer.temperature_c,
                layer.cold_content_j_m2,
                control_volume_temperature.as_celsius(),
                atmospheric_pressure_pa,
            )
            }
            Self::SnowLayerAggregateMismatch(snapshot) => {
                let SnowLayerAggregateMismatchError {
                phase_class,
                symbol,
                value,
                expected,
                prior_swe_m,
                prior_depth_m,
                prior_layers,
                } = snapshot.as_ref();
                write!(
                f,
                "{code}: phase class {} snow layer aggregate {symbol}={value} does not match \
                 expected {expected}; prior_swe_m={prior_swe_m}, \
                 prior_depth_m={prior_depth_m}, prior_layers={prior_layers:?}",
                phase_class.as_str(),
                )
            }
            _ => self.display_parts().fmt_with_code(&code, f),
        }
    }
}

impl Error for Wb11HydrologyKernelGuardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SnowStage3Conductivity(snapshot) => Some(&snapshot.source),
            _ => None,
        }
    }
}

#[cfg(test)]
mod cqr_row5_guard_error_tests {
    use super::*;

    fn symbol() -> BoundarySymbol {
        BoundarySymbol::from("row5.symbol")
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn guard_error_codes_cover_phase_and_erosion_families() {
        let phase_cases = [
            (
                HillslopeKernelPhaseClass::HydrologyEvapotranspiration,
                "HKERNEL-WB11-ET-E-001",
            ),
            (
                HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage,
                "HKERNEL-WB11-PERC-E-001",
            ),
            (
                HillslopeKernelPhaseClass::HydrologyLateralTransfer,
                "HKERNEL-WB11-LAT-E-001",
            ),
            (
                HillslopeKernelPhaseClass::HydrologyDrainage,
                "HKERNEL-WB11-DRAIN-E-001",
            ),
            (
                HillslopeKernelPhaseClass::HydrologyPlantRootUptake,
                "HKERNEL-WB17-SWU-E-001",
            ),
            (
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "HKERNEL-WB14-RUNOFF-E-001",
            ),
            (
                HillslopeKernelPhaseClass::HydrologyStorageReconciliation,
                "HKERNEL-WB12-STORAGE-E-001",
            ),
            (
                HillslopeKernelPhaseClass::HydrologyPeakRunoff,
                "HKERNEL-WB16-PEAK-E-001",
            ),
            (
                HillslopeKernelPhaseClass::GrowthAnnualTransition,
                "HKERNEL-WB11-GEN-E-001",
            ),
        ];
        for (phase_class, expected_code) in phase_cases {
            assert_eq!(
                Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: symbol(),
                }
                .code(),
                expected_code
            );
        }

        assert_eq!(
            Wb11HydrologyKernelGuardError::NonFiniteFluxSymbol {
                phase_class: HillslopeKernelPhaseClass::Hydrology,
                symbol: symbol(),
                value: f64::NAN,
            }
            .code(),
            "HKERNEL-WB11-GEN-E-002"
        );
        assert_eq!(
            Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                phase_class: HillslopeKernelPhaseClass::Hydrology,
                symbol: symbol(),
                value: 4.0,
                minimum: Some(0.0),
                maximum: Some(1.0),
            }
            .code(),
            "HKERNEL-WB11-GEN-E-003"
        );

        let erosion_cases = [
            (
                Wb11HydrologyKernelGuardError::Erod13MissingRequiredSymbol { symbol: symbol() },
                "HKERNEL-EROD13-CORE-E-001",
            ),
            (
                Wb11HydrologyKernelGuardError::Erod13NonFiniteSymbol {
                    symbol: symbol(),
                    value: f64::NAN,
                },
                "HKERNEL-EROD13-CORE-E-002",
            ),
            (
                Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: symbol(),
                    value: -1.0,
                    minimum: Some(0.0),
                    maximum: None,
                },
                "HKERNEL-EROD13-CORE-E-003",
            ),
            (
                Wb11HydrologyKernelGuardError::Erod14MissingRequiredSymbol { symbol: symbol() },
                "HKERNEL-EROD14-WAVE2-E-001",
            ),
            (
                Wb11HydrologyKernelGuardError::Erod14NonFiniteSymbol {
                    symbol: symbol(),
                    value: f64::NAN,
                },
                "HKERNEL-EROD14-WAVE2-E-002",
            ),
            (
                Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: symbol(),
                    value: -1.0,
                    minimum: Some(0.0),
                    maximum: None,
                },
                "HKERNEL-EROD14-WAVE2-E-003",
            ),
            (
                Wb11HydrologyKernelGuardError::Erod18MissingRequiredSymbol { symbol: symbol() },
                "HKERNEL-EROD18-ROUTE-E-001",
            ),
            (
                Wb11HydrologyKernelGuardError::Erod18NonFiniteSymbol {
                    symbol: symbol(),
                    value: f64::NAN,
                },
                "HKERNEL-EROD18-ROUTE-E-002",
            ),
            (
                Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                    symbol: symbol(),
                    value: -1.0,
                    minimum: Some(0.0),
                    maximum: None,
                },
                "HKERNEL-EROD18-ROUTE-E-003",
            ),
        ];
        for (error, expected_code) in erosion_cases {
            assert_eq!(error.code(), expected_code);
        }
    }

    #[test]
    fn guard_error_display_covers_symbol_kinds_and_range_shapes() {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        let cases = [
            (
                Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: symbol(),
                },
                "missing required state symbol",
            ),
            (
                Wb11HydrologyKernelGuardError::MissingRequiredFluxSymbol {
                    phase_class,
                    symbol: symbol(),
                },
                "missing required flux symbol",
            ),
            (
                Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol: symbol(),
                    value: f64::NAN,
                },
                "state symbol row5.symbol is non-finite",
            ),
            (
                Wb11HydrologyKernelGuardError::NonFiniteFluxSymbol {
                    phase_class,
                    symbol: symbol(),
                    value: f64::NAN,
                },
                "flux symbol row5.symbol is non-finite",
            ),
            (
                Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: symbol(),
                    value: 3.0,
                    minimum: Some(0.0),
                    maximum: Some(2.0),
                },
                "state symbol row5.symbol=3 outside",
            ),
            (
                Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                    phase_class,
                    symbol: symbol(),
                    value: 3.0,
                    minimum: Some(0.0),
                    maximum: None,
                },
                "flux symbol row5.symbol=3 outside",
            ),
            (
                Wb11HydrologyKernelGuardError::Erod13MissingRequiredSymbol { symbol: symbol() },
                "missing required EROD13 Wave-1 symbol",
            ),
            (
                Wb11HydrologyKernelGuardError::Erod14NonFiniteSymbol {
                    symbol: symbol(),
                    value: f64::NAN,
                },
                "non-finite EROD14 Wave-2 symbol",
            ),
            (
                Wb11HydrologyKernelGuardError::Erod18DomainViolation {
                    symbol: symbol(),
                    value: -1.0,
                    minimum: Some(0.0),
                    maximum: Some(1.0),
                },
                "EROD18 route topology symbol row5.symbol=-1 outside",
            ),
        ];

        for (error, expected_fragment) in cases {
            assert!(
                error.to_string().contains(expected_fragment),
                "{error:?} rendered as {error}"
            );
        }
    }
}
