/// Typed guard failures for WB11 hydrology production kernels.
#[derive(Debug, Clone, PartialEq)]
pub enum Wb11HydrologyKernelGuardError {
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
            | Self::Erod13DomainViolation { .. }
            | Self::Erod14DomainViolation { .. }
            | Self::Erod18DomainViolation { .. } => BoundaryClass::DomainViolation,
        }
    }

    #[must_use]
    pub fn code(&self) -> String {
        match self {
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
            Self::Erod13MissingRequiredSymbol { .. }
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
        self.display_parts().fmt_with_code(&self.code(), f)
    }
}

impl Error for Wb11HydrologyKernelGuardError {}
