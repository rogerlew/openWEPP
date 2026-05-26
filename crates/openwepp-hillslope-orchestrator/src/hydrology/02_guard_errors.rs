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
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation => ("WB14", "RUNOFF"),
            HillslopeKernelPhaseClass::HydrologyStorageReconciliation => ("WB12", "STORAGE"),
            HillslopeKernelPhaseClass::HydrologyPeakRunoff => ("WB16", "PEAK"),
            _ => ("WB11", "GEN"),
        };

        format!("HKERNEL-{kernel_family}-{phase_prefix}-E-{suffix}")
    }
}

impl fmt::Display for Wb11HydrologyKernelGuardError {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredStateSymbol {
                phase_class,
                symbol,
            } => write!(
                f,
                "{}: phase class {} missing required state symbol {}",
                self.code(),
                phase_class.as_str(),
                symbol
            ),
            Self::MissingRequiredFluxSymbol {
                phase_class,
                symbol,
            } => write!(
                f,
                "{}: phase class {} missing required flux symbol {}",
                self.code(),
                phase_class.as_str(),
                symbol
            ),
            Self::NonFiniteStateSymbol {
                phase_class,
                symbol,
                value,
            } => write!(
                f,
                "{}: phase class {} state symbol {} is non-finite ({})",
                self.code(),
                phase_class.as_str(),
                symbol,
                value
            ),
            Self::NonFiniteFluxSymbol {
                phase_class,
                symbol,
                value,
            } => write!(
                f,
                "{}: phase class {} flux symbol {} is non-finite ({})",
                self.code(),
                phase_class.as_str(),
                symbol,
                value
            ),
            Self::StateSymbolOutOfRange {
                phase_class,
                symbol,
                value,
                minimum,
                maximum,
            } => write!(
                f,
                "{}: phase class {} state symbol {}={} outside [{:?}, {:?}]",
                self.code(),
                phase_class.as_str(),
                symbol,
                value,
                minimum,
                maximum
            ),
            Self::FluxSymbolOutOfRange {
                phase_class,
                symbol,
                value,
                minimum,
                maximum,
            } => write!(
                f,
                "{}: phase class {} flux symbol {}={} outside [{:?}, {:?}]",
                self.code(),
                phase_class.as_str(),
                symbol,
                value,
                minimum,
                maximum
            ),
            Self::Erod13MissingRequiredSymbol { symbol } => write!(
                f,
                "{}: missing required EROD13 Wave-1 symbol {}",
                self.code(),
                symbol
            ),
            Self::Erod13NonFiniteSymbol { symbol, value } => write!(
                f,
                "{}: non-finite EROD13 Wave-1 symbol {} ({})",
                self.code(),
                symbol,
                value
            ),
            Self::Erod13DomainViolation {
                symbol,
                value,
                minimum,
                maximum,
            } => write!(
                f,
                "{}: EROD13 Wave-1 symbol {}={} outside [{:?}, {:?}]",
                self.code(),
                symbol,
                value,
                minimum,
                maximum
            ),
            Self::Erod14MissingRequiredSymbol { symbol } => write!(
                f,
                "{}: missing required EROD14 Wave-2 symbol {}",
                self.code(),
                symbol
            ),
            Self::Erod14NonFiniteSymbol { symbol, value } => write!(
                f,
                "{}: non-finite EROD14 Wave-2 symbol {} ({})",
                self.code(),
                symbol,
                value
            ),
            Self::Erod14DomainViolation {
                symbol,
                value,
                minimum,
                maximum,
            } => write!(
                f,
                "{}: EROD14 Wave-2 symbol {}={} outside [{:?}, {:?}]",
                self.code(),
                symbol,
                value,
                minimum,
                maximum
            ),
            Self::Erod18MissingRequiredSymbol { symbol } => write!(
                f,
                "{}: missing required EROD18 route topology symbol {}",
                self.code(),
                symbol
            ),
            Self::Erod18NonFiniteSymbol { symbol, value } => write!(
                f,
                "{}: non-finite EROD18 route topology symbol {} ({})",
                self.code(),
                symbol,
                value
            ),
            Self::Erod18DomainViolation {
                symbol,
                value,
                minimum,
                maximum,
            } => write!(
                f,
                "{}: EROD18 route topology symbol {}={} outside [{:?}, {:?}]",
                self.code(),
                symbol,
                value,
                minimum,
                maximum
            ),
        }
    }
}

impl Error for Wb11HydrologyKernelGuardError {}
