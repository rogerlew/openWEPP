use crate::constants::PHASE_COUNT;

/// Deterministic hillslope scheduler phases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum HillslopePhase {
    Normalization,
    StorageBounds,
    DecompositionTransition,
    ResiduePartitionTransition,
    AnnualGrowthTransition,
    PerennialGrowthTransition,
    Evapotranspiration,
    PercolationDeepSeepage,
    LateralTransfer,
    Drainage,
    RunoffReconciliation,
    StorageReconciliation,
    ClosureDiagnostics,
}

impl HillslopePhase {
    pub(crate) const ORDERED: [Self; PHASE_COUNT] = [
        Self::Normalization,
        Self::StorageBounds,
        Self::DecompositionTransition,
        Self::ResiduePartitionTransition,
        Self::AnnualGrowthTransition,
        Self::PerennialGrowthTransition,
        Self::PercolationDeepSeepage,
        Self::Evapotranspiration,
        Self::LateralTransfer,
        Self::Drainage,
        Self::RunoffReconciliation,
        Self::StorageReconciliation,
        Self::ClosureDiagnostics,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normalization => "normalization",
            Self::StorageBounds => "storage_bounds",
            Self::DecompositionTransition => "decomposition_transition",
            Self::ResiduePartitionTransition => "residue_partition_transition",
            Self::AnnualGrowthTransition => "annual_growth_transition",
            Self::PerennialGrowthTransition => "perennial_growth_transition",
            Self::Evapotranspiration => "evapotranspiration",
            Self::PercolationDeepSeepage => "percolation_deep_seepage",
            Self::LateralTransfer => "lateral_transfer",
            Self::Drainage => "drainage",
            Self::RunoffReconciliation => "runoff_reconciliation",
            Self::StorageReconciliation => "storage_reconciliation",
            Self::ClosureDiagnostics => "closure_diagnostics",
        }
    }

    #[must_use]
    pub const fn rank(self) -> usize {
        match self {
            Self::Normalization => 0,
            Self::StorageBounds => 1,
            Self::DecompositionTransition => 2,
            Self::ResiduePartitionTransition => 3,
            Self::AnnualGrowthTransition => 4,
            Self::PerennialGrowthTransition => 5,
            Self::PercolationDeepSeepage => 6,
            Self::Evapotranspiration => 7,
            Self::LateralTransfer => 8,
            Self::Drainage => 9,
            Self::RunoffReconciliation => 10,
            Self::StorageReconciliation => 11,
            Self::ClosureDiagnostics => 12,
        }
    }

    #[must_use]
    pub const fn ok_message_id(self) -> &'static str {
        match self {
            Self::Normalization => "HSCHED-PHASE-OK-001",
            Self::StorageBounds => "HSCHED-PHASE-OK-002",
            Self::DecompositionTransition => "HSCHED-PHASE-OK-012",
            Self::ResiduePartitionTransition => "HSCHED-PHASE-OK-013",
            Self::AnnualGrowthTransition => "HSCHED-PHASE-OK-010",
            Self::PerennialGrowthTransition => "HSCHED-PHASE-OK-011",
            Self::Evapotranspiration => "HSCHED-PHASE-OK-003",
            Self::PercolationDeepSeepage => "HSCHED-PHASE-OK-004",
            Self::LateralTransfer => "HSCHED-PHASE-OK-005",
            Self::Drainage => "HSCHED-PHASE-OK-006",
            Self::RunoffReconciliation => "HSCHED-PHASE-OK-007",
            Self::StorageReconciliation => "HSCHED-PHASE-OK-008",
            Self::ClosureDiagnostics => "HSCHED-PHASE-OK-009",
        }
    }
}
