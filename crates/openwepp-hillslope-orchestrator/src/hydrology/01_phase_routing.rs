#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum GrowthPhaseDispatch {
    Skip,
    Execute(HillslopeGrowthKernelContext),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum DecompositionPhaseDispatch {
    Skip,
    Execute(HillslopeDecompositionKernelContext),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HydrologyPhaseDispatch {
    Generic,
    Evapotranspiration,
    PercolationDeepSeepage,
    LateralTransfer,
    Drainage,
    RunoffReconciliation,
    StorageReconciliation,
    PeakRunoff,
}

/// Typed failure surface for scheduler hydrology phase-class routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HillslopeHydrologyRoutingError {
    UnsupportedPhaseClass {
        phase: HillslopePhase,
        phase_class: HillslopeKernelPhaseClass,
    },
}

impl HillslopeHydrologyRoutingError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::UnsupportedPhaseClass { .. } => "HS-HYDRO-E-001",
        }
    }

    #[must_use]
    pub const fn boundary_class(&self) -> BoundaryClass {
        match self {
            Self::UnsupportedPhaseClass { .. } => BoundaryClass::DomainViolation,
        }
    }
}

impl fmt::Display for HillslopeHydrologyRoutingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPhaseClass { phase, phase_class } => write!(
                f,
                "{}: phase {} unsupported hydrology phase class {}",
                self.code(),
                phase.as_str(),
                phase_class.as_str()
            ),
        }
    }
}

impl Error for HillslopeHydrologyRoutingError {}

#[must_use]
pub(crate) const fn is_decomposition_phase(phase: HillslopePhase) -> bool {
    matches!(
        phase,
        HillslopePhase::DecompositionTransition | HillslopePhase::ResiduePartitionTransition
    )
}

#[must_use]
pub(crate) const fn is_growth_phase(phase: HillslopePhase) -> bool {
    matches!(
        phase,
        HillslopePhase::AnnualGrowthTransition | HillslopePhase::PerennialGrowthTransition
    )
}

#[must_use]
pub(crate) const fn hillslope_phase_class_for_phase(
    phase: HillslopePhase,
) -> HillslopeKernelPhaseClass {
    match phase {
        HillslopePhase::DecompositionTransition => {
            HillslopeKernelPhaseClass::DecompositionTransition
        }
        HillslopePhase::ResiduePartitionTransition => {
            HillslopeKernelPhaseClass::ResiduePartitionTransition
        }
        HillslopePhase::AnnualGrowthTransition => HillslopeKernelPhaseClass::GrowthAnnualTransition,
        HillslopePhase::PerennialGrowthTransition => {
            HillslopeKernelPhaseClass::GrowthPerennialTransition
        }
        HillslopePhase::Evapotranspiration => {
            HillslopeKernelPhaseClass::HydrologyEvapotranspiration
        }
        HillslopePhase::PercolationDeepSeepage => {
            HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage
        }
        HillslopePhase::LateralTransfer => HillslopeKernelPhaseClass::HydrologyLateralTransfer,
        HillslopePhase::Drainage => HillslopeKernelPhaseClass::HydrologyDrainage,
        HillslopePhase::RunoffReconciliation => {
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation
        }
        HillslopePhase::StorageReconciliation => {
            HillslopeKernelPhaseClass::HydrologyStorageReconciliation
        }
        HillslopePhase::ClosureDiagnostics => HillslopeKernelPhaseClass::HydrologyPeakRunoff,
        _ => HillslopeKernelPhaseClass::Hydrology,
    }
}

pub(crate) fn hydrology_phase_dispatch_for_phase(
    phase: HillslopePhase,
    phase_class: HillslopeKernelPhaseClass,
) -> Result<HydrologyPhaseDispatch, HillslopeHydrologyRoutingError> {
    match (phase, phase_class) {
        (
            HillslopePhase::Normalization | HillslopePhase::StorageBounds,
            HillslopeKernelPhaseClass::Hydrology,
        ) => Ok(HydrologyPhaseDispatch::Generic),
        (
            HillslopePhase::Evapotranspiration,
            HillslopeKernelPhaseClass::HydrologyEvapotranspiration,
        ) => Ok(HydrologyPhaseDispatch::Evapotranspiration),
        (
            HillslopePhase::PercolationDeepSeepage,
            HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage,
        ) => Ok(HydrologyPhaseDispatch::PercolationDeepSeepage),
        (HillslopePhase::LateralTransfer, HillslopeKernelPhaseClass::HydrologyLateralTransfer) => {
            Ok(HydrologyPhaseDispatch::LateralTransfer)
        }
        (HillslopePhase::Drainage, HillslopeKernelPhaseClass::HydrologyDrainage) => {
            Ok(HydrologyPhaseDispatch::Drainage)
        }
        (
            HillslopePhase::RunoffReconciliation,
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
        ) => Ok(HydrologyPhaseDispatch::RunoffReconciliation),
        (
            HillslopePhase::StorageReconciliation,
            HillslopeKernelPhaseClass::HydrologyStorageReconciliation,
        ) => Ok(HydrologyPhaseDispatch::StorageReconciliation),
        (HillslopePhase::ClosureDiagnostics, HillslopeKernelPhaseClass::HydrologyPeakRunoff) => {
            Ok(HydrologyPhaseDispatch::PeakRunoff)
        }
        _ => Err(HillslopeHydrologyRoutingError::UnsupportedPhaseClass { phase, phase_class }),
    }
}

