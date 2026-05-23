#![allow(clippy::missing_errors_doc)]

pub mod runtime_inputs;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeActiveGrazingCycle, HillslopeAnnualDecompositionAction,
    HillslopeAnnualDecompositionControl, HillslopeAnnualGrowthAction, HillslopeAnnualGrowthControl,
    HillslopeConsumerAdapter, HillslopeDecompositionKernelContext,
    HillslopeDecompositionManagementClass, HillslopeDecompositionTransitionControl,
    HillslopeDecompositionTransitionPayload, HillslopeGrowthKernelContext,
    HillslopeGrowthManagementClass, HillslopeGrowthStateSurface, HillslopeGrowthTransitionControl,
    HillslopeGrowthTransitionPayload, HillslopeKernel, HillslopeKernelPhaseClass,
    HillslopeKernelRequest, HillslopePerennialDecompositionAction,
    HillslopePerennialDecompositionControl, HillslopePerennialGrowthAction,
    HillslopePerennialGrowthControl, KernelWritebackApplyResult, WritebackDecisionOutcome,
    WritebackError, apply_kernel_writeback, evaluate_kernel_writeback,
};
use openwepp_sim_contract::closure::ClosureViolation;
use openwepp_sim_contract::status::{
    BoundaryClass, ClampClass, SimulationPhase, SimulationStatus, StatusClassification, StatusError,
};
use openwepp_topology::TopologyValidationReport;

const PHASE_COUNT: usize = 13;
const RUNOFF_SLOPE_REQUIRED_STATE_SYMBOLS: &[&str] =
    &["nslpts", "slplen", "avgslp", "xinput_0001", "slpinp_0001"];
const RUNOFF_SOIL_REQUIRED_STATE_SYMBOLS: &[&str] = &["nsl", "solthk", "thetdr", "thetfc", "ssc"];
const SOIL_REQUIRED_STATE_SYMBOLS: &[&str] = &["nsl", "solthk", "dg", "thetdr", "thetfc", "ssc"];
const WATBAL_REQUIRED_STATE_SYMBOLS: &[&str] = &["nsl", "solthk", "thetdr", "thetfc", "ssc"];
const PERC_REQUIRED_STATE_SYMBOLS: &[&str] = &["nsl", "thetdr", "thetfc", "ssc"];
const SLOPE_FAMILY_SENTINELS: &[&str] = &["nelem", "nwsofe", "nslpts", "slplen", "avgslp"];
const SOIL_FAMILY_SENTINELS: &[&str] = &["nsl", "solthk", "dg", "thetdr", "thetfc", "ssc"];
const PL_GROWTH_RUNTIME_SENTINEL: &str = "pl_schedule_slot_count";
const PL_DECOMP_RUNTIME_SENTINEL: &str = "pl_schedule_slot_count";
const PL_SCHEDULE_SLOT_COUNT_SYMBOL: &str = "pl_schedule_slot_count";
const PL_SCHEDULE_ROTATION_REPEATS_SYMBOL: &str = "pl_schedule_rotation_repeats";
const PL_SCHEDULE_ROTATION_YEARS_SYMBOL: &str = "pl_schedule_rotation_years";
const PL_SCHEDULE_SLOT_ROTATION_INDEX_ROOT: &str = "rotation_index";
const PL_SCHEDULE_SLOT_OFE_INDEX_ROOT: &str = "ofe_index";
const PL_SCHEDULE_SLOT_YEAR_IN_ROTATION_ROOT: &str = "year_in_rotation";
const PL_SCHEDULE_SLOT_CROP_SLOTS_ROOT: &str = "crop_slots";
const PL_SCHEDULE_SLOT_CROP_IMNGMT_ROOT: &str = "imngmt";
const PL_RUNTIME_DAY_SYMBOL: &str = "day";
const PL_RUNTIME_YEAR_SYMBOL: &str = "year";
const PL_PRIMARY_OFE_INDEX: usize = 1;
const PL_DECOMP_IRESD_SEED_SYMBOL: &str = "iresd_seed";
const PL_DECOMP_SUMRTM_SEED_SYMBOL: &str = "sumrtm_seed";
const PL_DECOMP_SUMSRM_SEED_SYMBOL: &str = "sumsrm_seed";
const PL_ORDER_DECOMP_BEFORE_SOIL_SYMBOL: &str = "pl_order_decomp_before_soil";
const PL_ORDER_GROWTH_AFTER_DECOMP_SYMBOL: &str = "pl_order_growth_after_decomp";
const PL_ORDER_WATBAL_AFTER_GROWTH_SYMBOL: &str = "pl_order_watbal_after_growth";
const PL_GROWTH_STATE_SUMGDD_SYMBOL: &str = "sumgdd";
const PL_GROWTH_STATE_VDMT_SYMBOL: &str = "vdmt";
const PL_GROWTH_STATE_CANCOV_SYMBOL: &str = "cancov";
const PL_GROWTH_STATE_LAI_SYMBOL: &str = "lai";
const PL_GROWTH_STATE_RTMASS_SYMBOL: &str = "rtmass";
const PL_GROWTH_STATE_RTD_SYMBOL: &str = "rtd";
const PL_GROWTH_STATE_HIA_SYMBOL: &str = "hia";
const ORDER_FLAG_EPSILON: f64 = 1.0e-12;
const MANAGEMENT_CLASS_EPSILON: f64 = 1.0e-9;

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
    const ORDERED: [Self; PHASE_COUNT] = [
        Self::Normalization,
        Self::StorageBounds,
        Self::DecompositionTransition,
        Self::ResiduePartitionTransition,
        Self::AnnualGrowthTransition,
        Self::PerennialGrowthTransition,
        Self::Evapotranspiration,
        Self::PercolationDeepSeepage,
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
            Self::Evapotranspiration => 6,
            Self::PercolationDeepSeepage => 7,
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

/// Typed failure surface for hillslope phase-consumer boundary validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HillslopeConsumerBoundaryError {
    MissingRequiredStateSymbol {
        phase: HillslopePhase,
        adapter: HillslopeConsumerAdapter,
        symbol: BoundarySymbol,
    },
}

impl HillslopeConsumerBoundaryError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingRequiredStateSymbol { .. } => "HS-CONSUMER-E-001",
        }
    }
}

impl fmt::Display for HillslopeConsumerBoundaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredStateSymbol {
                phase,
                adapter,
                symbol,
            } => write!(
                f,
                "{}: phase {} ({}) missing required state symbol {}",
                self.code(),
                phase.as_str(),
                adapter.as_str(),
                symbol
            ),
        }
    }
}

impl Error for HillslopeConsumerBoundaryError {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ActivePlSlotSelection {
    slot_index: usize,
    crop_slot_index: usize,
}

/// Typed failure surface for PL schedule slot/crop activation resolution.
#[derive(Debug, Clone, PartialEq)]
pub enum HillslopePlActiveSlotResolutionError {
    MissingRequiredStateSymbol {
        symbol: BoundarySymbol,
    },
    NonFiniteRequiredStateSymbol {
        symbol: BoundarySymbol,
        value: f64,
    },
    NonIntegralRequiredStateSymbol {
        symbol: BoundarySymbol,
        value: f64,
    },
    StateSymbolValueOutOfRange {
        symbol: BoundarySymbol,
        value: usize,
        min_allowed: usize,
        max_allowed: usize,
    },
    MissingActiveSlotForOfeYear {
        ofe_index: usize,
        year_in_rotation: usize,
    },
    AmbiguousActiveSlotForOfeYear {
        ofe_index: usize,
        year_in_rotation: usize,
        slot_indexes: Vec<usize>,
    },
    InvalidCropSlotCount {
        slot_index: usize,
        crop_slots: usize,
    },
    MissingActiveCropForDay {
        slot_index: usize,
        day_of_year: usize,
    },
    AmbiguousActiveCropForDay {
        slot_index: usize,
        day_of_year: usize,
        crop_slot_indexes: Vec<usize>,
    },
}

impl HillslopePlActiveSlotResolutionError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingRequiredStateSymbol { .. } => "HS-PLDISP-E-001",
            Self::NonFiniteRequiredStateSymbol { .. } => "HS-PLDISP-E-002",
            Self::NonIntegralRequiredStateSymbol { .. } => "HS-PLDISP-E-003",
            Self::StateSymbolValueOutOfRange { .. } => "HS-PLDISP-E-004",
            Self::MissingActiveSlotForOfeYear { .. } => "HS-PLDISP-E-005",
            Self::AmbiguousActiveSlotForOfeYear { .. } => "HS-PLDISP-E-006",
            Self::InvalidCropSlotCount { .. } => "HS-PLDISP-E-007",
            Self::MissingActiveCropForDay { .. } => "HS-PLDISP-E-008",
            Self::AmbiguousActiveCropForDay { .. } => "HS-PLDISP-E-009",
        }
    }

    #[must_use]
    pub const fn boundary_class(&self) -> BoundaryClass {
        match self {
            Self::MissingRequiredStateSymbol { .. } => BoundaryClass::MissingRequiredInput,
            Self::NonFiniteRequiredStateSymbol { .. } => BoundaryClass::NonFinite,
            Self::NonIntegralRequiredStateSymbol { .. }
            | Self::StateSymbolValueOutOfRange { .. }
            | Self::MissingActiveSlotForOfeYear { .. }
            | Self::AmbiguousActiveSlotForOfeYear { .. }
            | Self::InvalidCropSlotCount { .. }
            | Self::MissingActiveCropForDay { .. }
            | Self::AmbiguousActiveCropForDay { .. } => BoundaryClass::DomainViolation,
        }
    }
}

impl fmt::Display for HillslopePlActiveSlotResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredStateSymbol { symbol } => write!(
                f,
                "{}: missing required PL dispatch symbol {}",
                self.code(),
                symbol
            ),
            Self::NonFiniteRequiredStateSymbol { symbol, value } => write!(
                f,
                "{}: PL dispatch symbol {} is non-finite ({})",
                self.code(),
                symbol,
                value
            ),
            Self::NonIntegralRequiredStateSymbol { symbol, value } => write!(
                f,
                "{}: PL dispatch symbol {} must be integral but is {}",
                self.code(),
                symbol,
                value
            ),
            Self::StateSymbolValueOutOfRange {
                symbol,
                value,
                min_allowed,
                max_allowed,
            } => write!(
                f,
                "{}: PL dispatch symbol {}={} outside allowed [{}, {}]",
                self.code(),
                symbol,
                value,
                min_allowed,
                max_allowed
            ),
            Self::MissingActiveSlotForOfeYear {
                ofe_index,
                year_in_rotation,
            } => write!(
                f,
                "{}: no active schedule slot for ofe={} year_in_rotation={}",
                self.code(),
                ofe_index,
                year_in_rotation
            ),
            Self::AmbiguousActiveSlotForOfeYear {
                ofe_index,
                year_in_rotation,
                slot_indexes,
            } => write!(
                f,
                "{}: ambiguous schedule slot for ofe={} year_in_rotation={} candidates={:?}",
                self.code(),
                ofe_index,
                year_in_rotation,
                slot_indexes
            ),
            Self::InvalidCropSlotCount {
                slot_index,
                crop_slots,
            } => write!(
                f,
                "{}: slot {} has invalid crop_slots={} (must be >= 1)",
                self.code(),
                slot_index,
                crop_slots
            ),
            Self::MissingActiveCropForDay {
                slot_index,
                day_of_year,
            } => write!(
                f,
                "{}: slot {} has no active crop for day {}",
                self.code(),
                slot_index,
                day_of_year
            ),
            Self::AmbiguousActiveCropForDay {
                slot_index,
                day_of_year,
                crop_slot_indexes,
            } => write!(
                f,
                "{}: slot {} has ambiguous active crops for day {} candidates={:?}",
                self.code(),
                slot_index,
                day_of_year,
                crop_slot_indexes
            ),
        }
    }
}

impl Error for HillslopePlActiveSlotResolutionError {}

/// Typed failure surface for scheduler-to-growth kernel boundary validation.
#[derive(Debug, Clone, PartialEq)]
pub enum HillslopeGrowthBoundaryError {
    MissingRequiredStateSymbol {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
    },
    NonFiniteRequiredStateSymbol {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
        value: f64,
    },
    InvalidOrderingFlagValue {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
        observed: f64,
        expected: f64,
    },
    UnsupportedManagementClass {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
        value: f64,
    },
    NonIntegralRequiredStateSymbol {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
        value: f64,
    },
    StateSymbolValueOutOfRange {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
        value: usize,
        min_allowed: usize,
        max_allowed: usize,
    },
    InvalidTransitionPayloadState {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
        value: f64,
        reason: &'static str,
    },
    ActiveSlotResolution {
        phase: HillslopePhase,
        source: HillslopePlActiveSlotResolutionError,
    },
}

impl HillslopeGrowthBoundaryError {
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingRequiredStateSymbol { .. } => "HS-GROWTH-E-001",
            Self::NonFiniteRequiredStateSymbol { .. } => "HS-GROWTH-E-002",
            Self::InvalidOrderingFlagValue { .. } => "HS-GROWTH-E-003",
            Self::UnsupportedManagementClass { .. } => "HS-GROWTH-E-004",
            Self::NonIntegralRequiredStateSymbol { .. } => "HS-GROWTH-E-005",
            Self::StateSymbolValueOutOfRange { .. } => "HS-GROWTH-E-006",
            Self::InvalidTransitionPayloadState { .. } => "HS-GROWTH-E-007",
            Self::ActiveSlotResolution { source, .. } => source.code(),
        }
    }

    #[must_use]
    pub fn boundary_class(&self) -> BoundaryClass {
        match self {
            Self::MissingRequiredStateSymbol { .. } => BoundaryClass::MissingRequiredInput,
            Self::NonFiniteRequiredStateSymbol { .. } => BoundaryClass::NonFinite,
            Self::InvalidOrderingFlagValue { .. }
            | Self::UnsupportedManagementClass { .. }
            | Self::NonIntegralRequiredStateSymbol { .. }
            | Self::StateSymbolValueOutOfRange { .. }
            | Self::InvalidTransitionPayloadState { .. } => BoundaryClass::DomainViolation,
            Self::ActiveSlotResolution { source, .. } => source.boundary_class(),
        }
    }
}

impl fmt::Display for HillslopeGrowthBoundaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredStateSymbol { phase, symbol } => write!(
                f,
                "{}: phase {} missing required growth state symbol {}",
                self.code(),
                phase.as_str(),
                symbol
            ),
            Self::NonFiniteRequiredStateSymbol {
                phase,
                symbol,
                value,
            } => write!(
                f,
                "{}: phase {} growth state symbol {} is non-finite ({})",
                self.code(),
                phase.as_str(),
                symbol,
                value
            ),
            Self::InvalidOrderingFlagValue {
                phase,
                symbol,
                observed,
                expected,
            } => write!(
                f,
                "{}: phase {} growth ordering flag {}={} but expected {}",
                self.code(),
                phase.as_str(),
                symbol,
                observed,
                expected
            ),
            Self::UnsupportedManagementClass {
                phase,
                symbol,
                value,
            } => write!(
                f,
                "{}: phase {} unsupported management class {}={}",
                self.code(),
                phase.as_str(),
                symbol,
                value
            ),
            Self::NonIntegralRequiredStateSymbol {
                phase,
                symbol,
                value,
            } => write!(
                f,
                "{}: phase {} growth state symbol {} must be integral, observed {}",
                self.code(),
                phase.as_str(),
                symbol,
                value
            ),
            Self::StateSymbolValueOutOfRange {
                phase,
                symbol,
                value,
                min_allowed,
                max_allowed,
            } => write!(
                f,
                "{}: phase {} growth state symbol {}={} outside allowed range [{}..={}]",
                self.code(),
                phase.as_str(),
                symbol,
                value,
                min_allowed,
                max_allowed
            ),
            Self::InvalidTransitionPayloadState {
                phase,
                symbol,
                value,
                reason,
            } => write!(
                f,
                "{}: phase {} invalid growth transition payload {}={} ({})",
                self.code(),
                phase.as_str(),
                symbol,
                value,
                reason
            ),
            Self::ActiveSlotResolution { phase, source } => {
                write!(f, "{}: phase {} {}", self.code(), phase.as_str(), source)
            }
        }
    }
}

impl Error for HillslopeGrowthBoundaryError {}

/// Typed failure surface for scheduler-to-decomposition kernel boundary
/// validation.
#[derive(Debug, Clone, PartialEq)]
pub enum HillslopeDecompositionBoundaryError {
    MissingRequiredStateSymbol {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
    },
    NonFiniteRequiredStateSymbol {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
        value: f64,
    },
    InvalidOrderingFlagValue {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
        observed: f64,
        expected: f64,
    },
    UnsupportedManagementClass {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
        value: f64,
    },
    NonIntegralRequiredStateSymbol {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
        value: f64,
    },
    StateSymbolValueOutOfRange {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
        value: usize,
        min_allowed: usize,
        max_allowed: usize,
    },
    MissingIndexedStateSymbol {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
        index: usize,
    },
    UnexpectedIndexedStateSymbol {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
        index: usize,
        max_expected: usize,
    },
    InvalidGrazingWindow {
        phase: HillslopePhase,
        cycle_index: usize,
        gday_symbol: BoundarySymbol,
        gend_symbol: BoundarySymbol,
        gday: usize,
        gend: usize,
    },
    InvalidTransitionPayloadState {
        phase: HillslopePhase,
        symbol: BoundarySymbol,
        value: f64,
        reason: &'static str,
    },
    ActiveSlotResolution {
        phase: HillslopePhase,
        source: HillslopePlActiveSlotResolutionError,
    },
}

impl HillslopeDecompositionBoundaryError {
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::MissingRequiredStateSymbol { .. } => "HS-DECOMP-E-001",
            Self::NonFiniteRequiredStateSymbol { .. } => "HS-DECOMP-E-002",
            Self::InvalidOrderingFlagValue { .. } => "HS-DECOMP-E-003",
            Self::UnsupportedManagementClass { .. } => "HS-DECOMP-E-004",
            Self::NonIntegralRequiredStateSymbol { .. } => "HS-DECOMP-E-005",
            Self::StateSymbolValueOutOfRange { .. } => "HS-DECOMP-E-006",
            Self::MissingIndexedStateSymbol { .. } => "HS-DECOMP-E-007",
            Self::UnexpectedIndexedStateSymbol { .. } => "HS-DECOMP-E-008",
            Self::InvalidGrazingWindow { .. } => "HS-DECOMP-E-009",
            Self::InvalidTransitionPayloadState { .. } => "HS-DECOMP-E-010",
            Self::ActiveSlotResolution { source, .. } => source.code(),
        }
    }

    #[must_use]
    pub fn boundary_class(&self) -> BoundaryClass {
        match self {
            Self::MissingRequiredStateSymbol { .. } | Self::MissingIndexedStateSymbol { .. } => {
                BoundaryClass::MissingRequiredInput
            }
            Self::NonFiniteRequiredStateSymbol { .. } => BoundaryClass::NonFinite,
            Self::InvalidOrderingFlagValue { .. }
            | Self::UnsupportedManagementClass { .. }
            | Self::NonIntegralRequiredStateSymbol { .. }
            | Self::StateSymbolValueOutOfRange { .. }
            | Self::UnexpectedIndexedStateSymbol { .. }
            | Self::InvalidGrazingWindow { .. }
            | Self::InvalidTransitionPayloadState { .. } => BoundaryClass::DomainViolation,
            Self::ActiveSlotResolution { source, .. } => source.boundary_class(),
        }
    }
}

impl fmt::Display for HillslopeDecompositionBoundaryError {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRequiredStateSymbol { phase, symbol } => write!(
                f,
                "{}: phase {} missing required decomposition state symbol {}",
                self.code(),
                phase.as_str(),
                symbol
            ),
            Self::NonFiniteRequiredStateSymbol {
                phase,
                symbol,
                value,
            } => write!(
                f,
                "{}: phase {} decomposition state symbol {} is non-finite ({})",
                self.code(),
                phase.as_str(),
                symbol,
                value
            ),
            Self::InvalidOrderingFlagValue {
                phase,
                symbol,
                observed,
                expected,
            } => write!(
                f,
                "{}: phase {} decomposition ordering flag {}={} but expected {}",
                self.code(),
                phase.as_str(),
                symbol,
                observed,
                expected
            ),
            Self::UnsupportedManagementClass {
                phase,
                symbol,
                value,
            } => write!(
                f,
                "{}: phase {} unsupported decomposition management class {}={}",
                self.code(),
                phase.as_str(),
                symbol,
                value
            ),
            Self::NonIntegralRequiredStateSymbol {
                phase,
                symbol,
                value,
            } => write!(
                f,
                "{}: phase {} decomposition state symbol {} must be integral, observed {}",
                self.code(),
                phase.as_str(),
                symbol,
                value
            ),
            Self::StateSymbolValueOutOfRange {
                phase,
                symbol,
                value,
                min_allowed,
                max_allowed,
            } => write!(
                f,
                "{}: phase {} decomposition state symbol {}={} outside allowed range [{}..={}]",
                self.code(),
                phase.as_str(),
                symbol,
                value,
                min_allowed,
                max_allowed
            ),
            Self::MissingIndexedStateSymbol {
                phase,
                symbol,
                index,
            } => write!(
                f,
                "{}: phase {} missing indexed decomposition symbol {} for index {}",
                self.code(),
                phase.as_str(),
                symbol,
                index
            ),
            Self::UnexpectedIndexedStateSymbol {
                phase,
                symbol,
                index,
                max_expected,
            } => write!(
                f,
                "{}: phase {} unexpected indexed decomposition symbol {} with index {} above declared maximum {}",
                self.code(),
                phase.as_str(),
                symbol,
                index,
                max_expected
            ),
            Self::InvalidGrazingWindow {
                phase,
                cycle_index,
                gday_symbol,
                gend_symbol,
                gday,
                gend,
            } => write!(
                f,
                "{}: phase {} invalid grazing window at cycle {} ({}={}, {}={}); expected gday < gend",
                self.code(),
                phase.as_str(),
                cycle_index,
                gday_symbol,
                gday,
                gend_symbol,
                gend
            ),
            Self::InvalidTransitionPayloadState {
                phase,
                symbol,
                value,
                reason,
            } => write!(
                f,
                "{}: phase {} invalid transition payload {}={} ({})",
                self.code(),
                phase.as_str(),
                symbol,
                value,
                reason
            ),
            Self::ActiveSlotResolution { phase, source } => {
                write!(f, "{}: phase {} {}", self.code(), phase.as_str(), source)
            }
        }
    }
}

impl Error for HillslopeDecompositionBoundaryError {}

#[must_use]
pub const fn hillslope_consumer_adapter_for_phase(
    phase: HillslopePhase,
) -> HillslopeConsumerAdapter {
    match phase {
        HillslopePhase::Normalization | HillslopePhase::StorageBounds => {
            HillslopeConsumerAdapter::Soil
        }
        HillslopePhase::DecompositionTransition | HillslopePhase::ResiduePartitionTransition => {
            HillslopeConsumerAdapter::Decomposition
        }
        HillslopePhase::AnnualGrowthTransition | HillslopePhase::PerennialGrowthTransition => {
            HillslopeConsumerAdapter::Growth
        }
        HillslopePhase::Evapotranspiration
        | HillslopePhase::LateralTransfer
        | HillslopePhase::StorageReconciliation
        | HillslopePhase::ClosureDiagnostics => HillslopeConsumerAdapter::Watbal,
        HillslopePhase::PercolationDeepSeepage | HillslopePhase::Drainage => {
            HillslopeConsumerAdapter::Perc
        }
        HillslopePhase::RunoffReconciliation => HillslopeConsumerAdapter::Runoff,
    }
}

/// Resolve required consumer boundary state symbols for a phase against the
/// currently seeded runtime families.
#[must_use]
pub fn required_hillslope_consumer_state_symbols(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Vec<&'static str> {
    let adapter = hillslope_consumer_adapter_for_phase(phase);
    let slope_family_present = state_family_is_present(state_surface, SLOPE_FAMILY_SENTINELS);
    let soil_family_present = state_family_is_present(state_surface, SOIL_FAMILY_SENTINELS);
    let mut required = Vec::new();

    match adapter {
        HillslopeConsumerAdapter::Runoff => {
            if slope_family_present {
                required.extend(RUNOFF_SLOPE_REQUIRED_STATE_SYMBOLS);
            }
            if soil_family_present {
                required.extend(RUNOFF_SOIL_REQUIRED_STATE_SYMBOLS);
            }
        }
        HillslopeConsumerAdapter::Soil => {
            if soil_family_present {
                required.extend(SOIL_REQUIRED_STATE_SYMBOLS);
            }
        }
        HillslopeConsumerAdapter::Watbal => {
            if soil_family_present {
                required.extend(WATBAL_REQUIRED_STATE_SYMBOLS);
            }
        }
        HillslopeConsumerAdapter::Perc => {
            if soil_family_present {
                required.extend(PERC_REQUIRED_STATE_SYMBOLS);
            }
        }
        HillslopeConsumerAdapter::Decomposition | HillslopeConsumerAdapter::Growth => {}
    }

    required
}

/// Validate required state symbols for the selected phase consumer boundary.
pub fn validate_hillslope_consumer_boundary(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<(), HillslopeConsumerBoundaryError> {
    let adapter = hillslope_consumer_adapter_for_phase(phase);

    for symbol in required_hillslope_consumer_state_symbols(phase, state_surface) {
        if !state_surface.contains_key(&BoundarySymbol::from(symbol)) {
            return Err(HillslopeConsumerBoundaryError::MissingRequiredStateSymbol {
                phase,
                adapter,
                symbol: BoundarySymbol::from(symbol),
            });
        }
    }

    Ok(())
}

fn state_family_is_present(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    sentinels: &[&str],
) -> bool {
    sentinels
        .iter()
        .any(|symbol| state_surface.contains_key(&BoundarySymbol::from(*symbol)))
}

fn pl_schedule_slot_symbol(root: &str, slot_index: usize) -> String {
    format!("pl_schedule_slot_{slot_index:04}_{root}")
}

fn pl_schedule_slot_crop_symbol(root: &str, slot_index: usize, crop_slot_index: usize) -> String {
    format!("pl_schedule_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn pl_growth_slot_crop_symbol(root: &str, slot_index: usize, crop_slot_index: usize) -> String {
    format!("pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn pl_decomp_slot_crop_symbol(root: &str, slot_index: usize, crop_slot_index: usize) -> String {
    format!("pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn pl_decomp_slot_crop_indexed_symbol(
    root: &str,
    slot_index: usize,
    crop_slot_index: usize,
    index: usize,
) -> String {
    format!("pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}_{index:04}")
}

fn require_finite_pl_dispatch_symbol(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
) -> Result<f64, HillslopePlActiveSlotResolutionError> {
    let symbol_key = BoundarySymbol::from(symbol);
    let value = state_surface
        .get(&symbol_key)
        .ok_or_else(
            || HillslopePlActiveSlotResolutionError::MissingRequiredStateSymbol {
                symbol: symbol_key.clone(),
            },
        )?
        .as_f64();

    if !value.is_finite() {
        return Err(
            HillslopePlActiveSlotResolutionError::NonFiniteRequiredStateSymbol {
                symbol: symbol_key,
                value,
            },
        );
    }

    Ok(value)
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn require_integral_pl_dispatch_symbol_in_range(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopePlActiveSlotResolutionError> {
    let value = require_finite_pl_dispatch_symbol(state_surface, symbol)?;
    let rounded = value.round();
    if (value - rounded).abs() > MANAGEMENT_CLASS_EPSILON {
        return Err(
            HillslopePlActiveSlotResolutionError::NonIntegralRequiredStateSymbol {
                symbol: BoundarySymbol::from(symbol),
                value,
            },
        );
    }

    let min_f64 = min_allowed as f64;
    let max_f64 = max_allowed as f64;
    if rounded < min_f64 || rounded > max_f64 {
        return Err(
            HillslopePlActiveSlotResolutionError::StateSymbolValueOutOfRange {
                symbol: BoundarySymbol::from(symbol),
                value: rounded as usize,
                min_allowed,
                max_allowed,
            },
        );
    }

    Ok(rounded as usize)
}

fn day_is_within_julian_window(day_of_year: usize, start_day: usize, end_day: usize) -> bool {
    if start_day <= end_day {
        day_of_year >= start_day && day_of_year <= end_day
    } else {
        day_of_year >= start_day || day_of_year <= end_day
    }
}

fn select_active_crop_slot_for_day(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slots: usize,
    day_of_year: usize,
) -> Result<usize, HillslopePlActiveSlotResolutionError> {
    let mut candidates = Vec::new();

    for crop_slot_index in 1..=crop_slots {
        let imngmt_symbol = pl_schedule_slot_crop_symbol(
            PL_SCHEDULE_SLOT_CROP_IMNGMT_ROOT,
            slot_index,
            crop_slot_index,
        );
        let imngmt = require_integral_pl_dispatch_symbol_in_range(
            state_surface,
            imngmt_symbol.as_str(),
            1,
            3,
        )?;

        let growth_imngmt_symbol =
            pl_growth_slot_crop_symbol("imngmt", slot_index, crop_slot_index);
        let _ = require_integral_pl_dispatch_symbol_in_range(
            state_surface,
            growth_imngmt_symbol.as_str(),
            1,
            3,
        )?;

        let jdplt_symbol = pl_growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index);
        let jdplt = require_integral_pl_dispatch_symbol_in_range(
            state_surface,
            jdplt_symbol.as_str(),
            1,
            366,
        )?;
        let jdharv_symbol = pl_growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index);
        let jdharv = require_integral_pl_dispatch_symbol_in_range(
            state_surface,
            jdharv_symbol.as_str(),
            0,
            366,
        )?;

        let is_active = if imngmt == 2 {
            // PL11+ carries full perennial event payloads; PL10 keeps slot
            // selection bounded to existing day-window symbols.
            let jdstop_symbol = pl_growth_slot_crop_symbol("jdstop", slot_index, crop_slot_index);
            let jdstop = require_integral_pl_dispatch_symbol_in_range(
                state_surface,
                jdstop_symbol.as_str(),
                0,
                366,
            )?;
            if jdstop == 0 {
                day_is_within_julian_window(day_of_year, jdplt, jdharv.max(1))
            } else {
                day_is_within_julian_window(day_of_year, jdplt, jdstop)
            }
        } else {
            day_is_within_julian_window(day_of_year, jdplt, jdharv.max(1))
        };

        if is_active {
            candidates.push(crop_slot_index);
        }
    }

    match candidates.as_slice() {
        [crop_slot_index] => Ok(*crop_slot_index),
        [] => Err(
            HillslopePlActiveSlotResolutionError::MissingActiveCropForDay {
                slot_index,
                day_of_year,
            },
        ),
        _ => Err(
            HillslopePlActiveSlotResolutionError::AmbiguousActiveCropForDay {
                slot_index,
                day_of_year,
                crop_slot_indexes: candidates,
            },
        ),
    }
}

#[allow(clippy::too_many_lines)]
fn resolve_active_pl_slot_selection(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<ActivePlSlotSelection, HillslopePlActiveSlotResolutionError> {
    let slot_count = require_integral_pl_dispatch_symbol_in_range(
        state_surface,
        PL_SCHEDULE_SLOT_COUNT_SYMBOL,
        1,
        usize::MAX,
    )?;
    let rotation_years = require_integral_pl_dispatch_symbol_in_range(
        state_surface,
        PL_SCHEDULE_ROTATION_YEARS_SYMBOL,
        1,
        usize::MAX,
    )?;
    let rotation_repeats = require_integral_pl_dispatch_symbol_in_range(
        state_surface,
        PL_SCHEDULE_ROTATION_REPEATS_SYMBOL,
        1,
        usize::MAX,
    )?;
    let runtime_year = require_integral_pl_dispatch_symbol_in_range(
        state_surface,
        PL_RUNTIME_YEAR_SYMBOL,
        1,
        usize::MAX,
    )?;
    let max_runtime_year = rotation_repeats.saturating_mul(rotation_years);
    if runtime_year > max_runtime_year {
        return Err(
            HillslopePlActiveSlotResolutionError::StateSymbolValueOutOfRange {
                symbol: BoundarySymbol::from(PL_RUNTIME_YEAR_SYMBOL),
                value: runtime_year,
                min_allowed: 1,
                max_allowed: max_runtime_year,
            },
        );
    }
    let day_of_year =
        require_integral_pl_dispatch_symbol_in_range(state_surface, PL_RUNTIME_DAY_SYMBOL, 1, 366)?;
    let rotation_index = ((runtime_year - 1) / rotation_years) + 1;
    let year_in_rotation = ((runtime_year - 1) % rotation_years) + 1;

    let mut slot_candidates = Vec::new();
    for slot_index in 1..=slot_count {
        let slot_ofe_symbol = pl_schedule_slot_symbol(PL_SCHEDULE_SLOT_OFE_INDEX_ROOT, slot_index);
        let ofe_index = require_integral_pl_dispatch_symbol_in_range(
            state_surface,
            slot_ofe_symbol.as_str(),
            1,
            usize::MAX,
        )?;
        if ofe_index != PL_PRIMARY_OFE_INDEX {
            continue;
        }

        let slot_year_symbol =
            pl_schedule_slot_symbol(PL_SCHEDULE_SLOT_YEAR_IN_ROTATION_ROOT, slot_index);
        let slot_year_in_rotation = require_integral_pl_dispatch_symbol_in_range(
            state_surface,
            slot_year_symbol.as_str(),
            1,
            rotation_years,
        )?;
        let slot_rotation_symbol =
            pl_schedule_slot_symbol(PL_SCHEDULE_SLOT_ROTATION_INDEX_ROOT, slot_index);
        let slot_rotation_index = require_integral_pl_dispatch_symbol_in_range(
            state_surface,
            slot_rotation_symbol.as_str(),
            1,
            rotation_repeats,
        )?;
        if slot_year_in_rotation == year_in_rotation && slot_rotation_index == rotation_index {
            slot_candidates.push(slot_index);
        }
    }

    let slot_index = match slot_candidates.as_slice() {
        [slot_index] => *slot_index,
        [] => {
            return Err(
                HillslopePlActiveSlotResolutionError::MissingActiveSlotForOfeYear {
                    ofe_index: PL_PRIMARY_OFE_INDEX,
                    year_in_rotation,
                },
            );
        }
        _ => {
            return Err(
                HillslopePlActiveSlotResolutionError::AmbiguousActiveSlotForOfeYear {
                    ofe_index: PL_PRIMARY_OFE_INDEX,
                    year_in_rotation,
                    slot_indexes: slot_candidates,
                },
            );
        }
    };

    let crop_slots_symbol = pl_schedule_slot_symbol(PL_SCHEDULE_SLOT_CROP_SLOTS_ROOT, slot_index);
    let crop_slots = require_integral_pl_dispatch_symbol_in_range(
        state_surface,
        crop_slots_symbol.as_str(),
        0,
        usize::MAX,
    )?;
    if crop_slots == 0 {
        return Err(HillslopePlActiveSlotResolutionError::InvalidCropSlotCount {
            slot_index,
            crop_slots,
        });
    }

    let crop_slot_index =
        select_active_crop_slot_for_day(state_surface, slot_index, crop_slots, day_of_year)?;
    Ok(ActivePlSlotSelection {
        slot_index,
        crop_slot_index,
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GrowthPhaseDispatch {
    Skip,
    Execute(HillslopeGrowthKernelContext),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DecompositionPhaseDispatch {
    Skip,
    Execute(HillslopeDecompositionKernelContext),
}

#[must_use]
const fn is_decomposition_phase(phase: HillslopePhase) -> bool {
    matches!(
        phase,
        HillslopePhase::DecompositionTransition | HillslopePhase::ResiduePartitionTransition
    )
}

#[must_use]
const fn is_growth_phase(phase: HillslopePhase) -> bool {
    matches!(
        phase,
        HillslopePhase::AnnualGrowthTransition | HillslopePhase::PerennialGrowthTransition
    )
}

#[must_use]
const fn hillslope_phase_class_for_phase(phase: HillslopePhase) -> HillslopeKernelPhaseClass {
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
        _ => HillslopeKernelPhaseClass::Hydrology,
    }
}

#[allow(clippy::too_many_lines)]
fn decomposition_phase_dispatch_for_state(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<DecompositionPhaseDispatch, HillslopeDecompositionBoundaryError> {
    if !state_surface.contains_key(&BoundarySymbol::from(PL_DECOMP_RUNTIME_SENTINEL)) {
        return Ok(DecompositionPhaseDispatch::Skip);
    }

    let active_slot_selection =
        resolve_active_pl_slot_selection(state_surface).map_err(|source| {
            HillslopeDecompositionBoundaryError::ActiveSlotResolution { phase, source }
        })?;

    let runtime_day =
        require_integral_pl_dispatch_symbol_in_range(state_surface, PL_RUNTIME_DAY_SYMBOL, 1, 366)
            .map_err(
                |source| HillslopeDecompositionBoundaryError::ActiveSlotResolution {
                    phase,
                    source,
                },
            )?;

    let imngmt_symbol = pl_growth_slot_crop_symbol(
        "imngmt",
        active_slot_selection.slot_index,
        active_slot_selection.crop_slot_index,
    );
    let imngmt =
        require_finite_state_value_for_decomposition(phase, state_surface, imngmt_symbol.as_str())?;
    let management_class =
        normalize_management_class_for_decomposition(phase, imngmt, imngmt_symbol.as_str())?;
    let order_decomp_before_soil = require_ordering_flag_for_decomposition(
        phase,
        state_surface,
        PL_ORDER_DECOMP_BEFORE_SOIL_SYMBOL,
        1.0,
    )?;
    let order_growth_after_decomp = require_ordering_flag_for_decomposition(
        phase,
        state_surface,
        PL_ORDER_GROWTH_AFTER_DECOMP_SYMBOL,
        1.0,
    )?;

    let iresd_seed = require_finite_state_value_for_decomposition(
        phase,
        state_surface,
        PL_DECOMP_IRESD_SEED_SYMBOL,
    )?;
    let sumrtm_seed = require_finite_state_value_for_decomposition(
        phase,
        state_surface,
        PL_DECOMP_SUMRTM_SEED_SYMBOL,
    )?;
    let sumsrm_seed = require_finite_state_value_for_decomposition(
        phase,
        state_surface,
        PL_DECOMP_SUMSRM_SEED_SYMBOL,
    )?;

    let control = match management_class {
        1 | 3 => {
            HillslopeDecompositionTransitionControl::Annual(build_annual_decomposition_control(
                phase,
                state_surface,
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
                runtime_day,
            )?)
        }
        2 => HillslopeDecompositionTransitionControl::Perennial(
            build_perennial_decomposition_control(
                phase,
                state_surface,
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
                runtime_day,
            )?,
        ),
        _ => {
            return Err(
                HillslopeDecompositionBoundaryError::UnsupportedManagementClass {
                    phase,
                    symbol: BoundarySymbol::from(imngmt_symbol.as_str()),
                    value: imngmt,
                },
            );
        }
    };

    let management_class = if management_class == 2 {
        HillslopeDecompositionManagementClass::Perennial
    } else {
        HillslopeDecompositionManagementClass::AnnualOrFallow
    };

    let transition_payload = HillslopeDecompositionTransitionPayload {
        active_slot_index: active_slot_selection.slot_index,
        active_crop_slot_index: active_slot_selection.crop_slot_index,
        runtime_day_of_year: usize_to_u16_for_decomposition(
            phase,
            BoundarySymbol::from(PL_RUNTIME_DAY_SYMBOL),
            runtime_day,
        )?,
        iresd_seed,
        sumrtm_seed,
        sumsrm_seed,
        control,
    };

    Ok(DecompositionPhaseDispatch::Execute(
        HillslopeDecompositionKernelContext::new(
            management_class,
            order_decomp_before_soil,
            order_growth_after_decomp,
        )
        .with_transition_payload(transition_payload),
    ))
}

#[allow(clippy::too_many_lines)]
fn growth_phase_dispatch_for_state(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<GrowthPhaseDispatch, HillslopeGrowthBoundaryError> {
    if !state_surface.contains_key(&BoundarySymbol::from(PL_GROWTH_RUNTIME_SENTINEL)) {
        return Ok(GrowthPhaseDispatch::Skip);
    }

    let active_slot_selection = resolve_active_pl_slot_selection(state_surface)
        .map_err(|source| HillslopeGrowthBoundaryError::ActiveSlotResolution { phase, source })?;
    let imngmt_symbol = pl_growth_slot_crop_symbol(
        "imngmt",
        active_slot_selection.slot_index,
        active_slot_selection.crop_slot_index,
    );
    let imngmt = require_finite_state_value(phase, state_surface, imngmt_symbol.as_str())?;
    let management_class = normalize_management_class(phase, imngmt, imngmt_symbol.as_str())?;
    let runtime_day = require_integral_state_value_in_range_for_growth(
        phase,
        state_surface,
        PL_RUNTIME_DAY_SYMBOL,
        1,
        366,
    )?;
    let order_growth_after_decomp = require_ordering_flag(
        phase,
        state_surface,
        PL_ORDER_GROWTH_AFTER_DECOMP_SYMBOL,
        1.0,
    )?;
    let order_watbal_after_growth = require_ordering_flag(
        phase,
        state_surface,
        PL_ORDER_WATBAL_AFTER_GROWTH_SYMBOL,
        1.0,
    )?;
    let state_before = require_growth_state_surface(phase, state_surface)?;

    match phase {
        HillslopePhase::AnnualGrowthTransition => {
            if management_class == 2 {
                return Ok(GrowthPhaseDispatch::Skip);
            }
            debug_assert!(management_class == 1 || management_class == 3);

            let jdharv_symbol = pl_growth_slot_crop_symbol(
                "jdharv",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );
            let jdplt_symbol = pl_growth_slot_crop_symbol(
                "jdplt",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );
            let rw_symbol = pl_growth_slot_crop_symbol(
                "rw",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );
            let resmgt_symbol = pl_decomp_slot_crop_symbol(
                "resmgt",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );

            let jdharv = require_integral_state_value_in_range_for_growth(
                phase,
                state_surface,
                jdharv_symbol.as_str(),
                1,
                366,
            )?;
            let jdplt = require_integral_state_value_in_range_for_growth(
                phase,
                state_surface,
                jdplt_symbol.as_str(),
                1,
                366,
            )?;
            let rw = require_finite_state_value(phase, state_surface, rw_symbol.as_str())?;
            let _resmgt = require_integral_state_value_in_range_for_growth(
                phase,
                state_surface,
                resmgt_symbol.as_str(),
                1,
                6,
            )?;

            let active_action = if runtime_day == jdplt {
                HillslopeAnnualGrowthAction::PlantingReset
            } else if runtime_day == jdharv {
                HillslopeAnnualGrowthAction::HarvestReset
            } else if day_within_closed_window(runtime_day, jdplt, jdharv) {
                HillslopeAnnualGrowthAction::None
            } else {
                HillslopeAnnualGrowthAction::SenescenceReset
            };

            let state_after = match active_action {
                HillslopeAnnualGrowthAction::None => state_before,
                HillslopeAnnualGrowthAction::PlantingReset
                | HillslopeAnnualGrowthAction::HarvestReset
                | HillslopeAnnualGrowthAction::SenescenceReset => {
                    reset_growth_state_surface(state_before)
                }
            };

            let transition_payload = HillslopeGrowthTransitionPayload {
                active_slot_index: active_slot_selection.slot_index,
                active_crop_slot_index: active_slot_selection.crop_slot_index,
                runtime_day_of_year: usize_to_u16_for_growth(
                    phase,
                    BoundarySymbol::from(PL_RUNTIME_DAY_SYMBOL),
                    runtime_day,
                )?,
                state_before,
                state_after,
                control: HillslopeGrowthTransitionControl::Annual(HillslopeAnnualGrowthControl {
                    jdharv: usize_to_u16_for_growth(
                        phase,
                        BoundarySymbol::from(jdharv_symbol.as_str()),
                        jdharv,
                    )?,
                    jdplt: usize_to_u16_for_growth(
                        phase,
                        BoundarySymbol::from(jdplt_symbol.as_str()),
                        jdplt,
                    )?,
                    rw,
                    active_action,
                }),
            };

            Ok(GrowthPhaseDispatch::Execute(
                HillslopeGrowthKernelContext::new(
                    HillslopeGrowthManagementClass::AnnualOrFallow,
                    order_growth_after_decomp,
                    order_watbal_after_growth,
                )
                .with_transition_payload(transition_payload),
            ))
        }
        HillslopePhase::PerennialGrowthTransition => {
            if management_class == 1 || management_class == 3 {
                return Ok(GrowthPhaseDispatch::Skip);
            }
            debug_assert!(management_class == 2);

            let jdharv_symbol = pl_growth_slot_crop_symbol(
                "jdharv",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );
            let jdplt_symbol = pl_growth_slot_crop_symbol(
                "jdplt",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );
            let rw_symbol = pl_growth_slot_crop_symbol(
                "rw",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );
            let jdstop_symbol = pl_growth_slot_crop_symbol(
                "jdstop",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );
            let mgtopt_symbol = pl_growth_slot_crop_symbol(
                "mgtopt",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );

            let jdharv = require_integral_state_value_in_range_for_growth(
                phase,
                state_surface,
                jdharv_symbol.as_str(),
                0,
                366,
            )?;
            let jdplt = require_integral_state_value_in_range_for_growth(
                phase,
                state_surface,
                jdplt_symbol.as_str(),
                0,
                366,
            )?;
            let rw = require_finite_state_value(phase, state_surface, rw_symbol.as_str())?;
            let jdstop = require_integral_state_value_in_range_for_growth(
                phase,
                state_surface,
                jdstop_symbol.as_str(),
                0,
                366,
            )?;
            let mgtopt = require_integral_state_value_in_range_for_growth(
                phase,
                state_surface,
                mgtopt_symbol.as_str(),
                1,
                3,
            )?;

            let active_action = if runtime_day == jdplt {
                HillslopePerennialGrowthAction::PlantingReset
            } else if jdstop != 0 && runtime_day == jdstop {
                HillslopePerennialGrowthAction::StopReset
            } else {
                HillslopePerennialGrowthAction::None
            };

            let state_after = match active_action {
                HillslopePerennialGrowthAction::None => state_before,
                HillslopePerennialGrowthAction::PlantingReset
                | HillslopePerennialGrowthAction::StopReset => {
                    reset_growth_state_surface(state_before)
                }
            };

            let transition_payload = HillslopeGrowthTransitionPayload {
                active_slot_index: active_slot_selection.slot_index,
                active_crop_slot_index: active_slot_selection.crop_slot_index,
                runtime_day_of_year: usize_to_u16_for_growth(
                    phase,
                    BoundarySymbol::from(PL_RUNTIME_DAY_SYMBOL),
                    runtime_day,
                )?,
                state_before,
                state_after,
                control: HillslopeGrowthTransitionControl::Perennial(
                    HillslopePerennialGrowthControl {
                        jdharv: usize_to_u16_for_growth(
                            phase,
                            BoundarySymbol::from(jdharv_symbol.as_str()),
                            jdharv,
                        )?,
                        jdplt: usize_to_u16_for_growth(
                            phase,
                            BoundarySymbol::from(jdplt_symbol.as_str()),
                            jdplt,
                        )?,
                        jdstop: usize_to_u16_for_growth(
                            phase,
                            BoundarySymbol::from(jdstop_symbol.as_str()),
                            jdstop,
                        )?,
                        mgtopt: usize_to_u8_for_growth(
                            phase,
                            BoundarySymbol::from(mgtopt_symbol.as_str()),
                            mgtopt,
                        )?,
                        rw,
                        active_action,
                    },
                ),
            };

            Ok(GrowthPhaseDispatch::Execute(
                HillslopeGrowthKernelContext::new(
                    HillslopeGrowthManagementClass::Perennial,
                    order_growth_after_decomp,
                    order_watbal_after_growth,
                )
                .with_transition_payload(transition_payload),
            ))
        }
        _ => Ok(GrowthPhaseDispatch::Skip),
    }
}

fn require_growth_state_surface(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<HillslopeGrowthStateSurface, HillslopeGrowthBoundaryError> {
    let sumgdd = require_finite_state_value(phase, state_surface, PL_GROWTH_STATE_SUMGDD_SYMBOL)?;
    let vdmt = require_finite_state_value(phase, state_surface, PL_GROWTH_STATE_VDMT_SYMBOL)?;
    let cancov = require_finite_state_value(phase, state_surface, PL_GROWTH_STATE_CANCOV_SYMBOL)?;
    let lai = require_finite_state_value(phase, state_surface, PL_GROWTH_STATE_LAI_SYMBOL)?;
    let rtmass = require_finite_state_value(phase, state_surface, PL_GROWTH_STATE_RTMASS_SYMBOL)?;
    let rtd = require_finite_state_value(phase, state_surface, PL_GROWTH_STATE_RTD_SYMBOL)?;
    let hia = require_finite_state_value(phase, state_surface, PL_GROWTH_STATE_HIA_SYMBOL)?;

    for (symbol, value, minimum, maximum, reason) in [
        (
            PL_GROWTH_STATE_SUMGDD_SYMBOL,
            sumgdd,
            Some(0.0),
            None,
            "sumgdd must be non-negative",
        ),
        (
            PL_GROWTH_STATE_VDMT_SYMBOL,
            vdmt,
            Some(0.0),
            None,
            "vdmt must be non-negative",
        ),
        (
            PL_GROWTH_STATE_CANCOV_SYMBOL,
            cancov,
            Some(0.0),
            Some(0.999),
            "cancov must be within [0, 0.999]",
        ),
        (
            PL_GROWTH_STATE_LAI_SYMBOL,
            lai,
            Some(0.0),
            None,
            "lai must be non-negative",
        ),
        (
            PL_GROWTH_STATE_RTMASS_SYMBOL,
            rtmass,
            Some(0.0),
            None,
            "rtmass must be non-negative",
        ),
        (
            PL_GROWTH_STATE_RTD_SYMBOL,
            rtd,
            Some(0.0),
            None,
            "rtd must be non-negative",
        ),
        (
            PL_GROWTH_STATE_HIA_SYMBOL,
            hia,
            Some(0.0),
            Some(1.0),
            "hia must be within [0, 1]",
        ),
    ] {
        if let Some(minimum) = minimum {
            if value < minimum {
                return Err(
                    HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(symbol),
                        value,
                        reason,
                    },
                );
            }
        }
        if let Some(maximum) = maximum {
            if value > maximum {
                return Err(
                    HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(symbol),
                        value,
                        reason,
                    },
                );
            }
        }
    }

    Ok(HillslopeGrowthStateSurface {
        sumgdd,
        vdmt,
        cancov,
        lai,
        rtmass,
        rtd,
        hia,
    })
}

fn reset_growth_state_surface(_state: HillslopeGrowthStateSurface) -> HillslopeGrowthStateSurface {
    HillslopeGrowthStateSurface {
        sumgdd: 0.0,
        vdmt: 0.0,
        cancov: 0.0,
        lai: 0.0,
        rtmass: 0.0,
        rtd: 0.0,
        hia: 0.0,
    }
}

fn day_within_closed_window(day: usize, start: usize, end: usize) -> bool {
    if start <= end {
        day >= start && day <= end
    } else {
        day >= start || day <= end
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn require_integral_state_value_for_growth(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
) -> Result<usize, HillslopeGrowthBoundaryError> {
    let value = require_finite_state_value(phase, state_surface, symbol)?;
    let rounded = value.round();
    if (value - rounded).abs() > MANAGEMENT_CLASS_EPSILON {
        return Err(
            HillslopeGrowthBoundaryError::NonIntegralRequiredStateSymbol {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
            },
        );
    }
    if rounded < 0.0 {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
                reason: "integral growth symbol must be non-negative",
            },
        );
    }
    Ok(rounded as usize)
}

fn require_integral_state_value_in_range_for_growth(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeGrowthBoundaryError> {
    let value = require_integral_state_value_for_growth(phase, state_surface, symbol)?;
    if value < min_allowed || value > max_allowed {
        return Err(HillslopeGrowthBoundaryError::StateSymbolValueOutOfRange {
            phase,
            symbol: BoundarySymbol::from(symbol),
            value,
            min_allowed,
            max_allowed,
        });
    }
    Ok(value)
}

fn usize_to_u16_for_growth(
    phase: HillslopePhase,
    symbol: BoundarySymbol,
    value: usize,
) -> Result<u16, HillslopeGrowthBoundaryError> {
    u16::try_from(value).map_err(
        |_| HillslopeGrowthBoundaryError::StateSymbolValueOutOfRange {
            phase,
            symbol,
            value,
            min_allowed: 0,
            max_allowed: usize::from(u16::MAX),
        },
    )
}

fn usize_to_u8_for_growth(
    phase: HillslopePhase,
    symbol: BoundarySymbol,
    value: usize,
) -> Result<u8, HillslopeGrowthBoundaryError> {
    u8::try_from(value).map_err(
        |_| HillslopeGrowthBoundaryError::StateSymbolValueOutOfRange {
            phase,
            symbol,
            value,
            min_allowed: 0,
            max_allowed: usize::from(u8::MAX),
        },
    )
}

fn require_ordering_flag(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    expected: f64,
) -> Result<f64, HillslopeGrowthBoundaryError> {
    let observed = require_finite_state_value(phase, state_surface, symbol)?;
    if (observed - expected).abs() > ORDER_FLAG_EPSILON {
        return Err(HillslopeGrowthBoundaryError::InvalidOrderingFlagValue {
            phase,
            symbol: BoundarySymbol::from(symbol),
            observed,
            expected,
        });
    }

    Ok(observed)
}

fn require_finite_state_value(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
) -> Result<f64, HillslopeGrowthBoundaryError> {
    let symbol_key = BoundarySymbol::from(symbol);
    let value = state_surface
        .get(&symbol_key)
        .ok_or_else(
            || HillslopeGrowthBoundaryError::MissingRequiredStateSymbol {
                phase,
                symbol: symbol_key.clone(),
            },
        )?
        .as_f64();

    if !value.is_finite() {
        return Err(HillslopeGrowthBoundaryError::NonFiniteRequiredStateSymbol {
            phase,
            symbol: symbol_key,
            value,
        });
    }

    Ok(value)
}

fn normalize_management_class(
    phase: HillslopePhase,
    value: f64,
    symbol: &str,
) -> Result<u8, HillslopeGrowthBoundaryError> {
    let rounded = value.round();
    if (value - rounded).abs() > MANAGEMENT_CLASS_EPSILON {
        return Err(HillslopeGrowthBoundaryError::UnsupportedManagementClass {
            phase,
            symbol: BoundarySymbol::from(symbol),
            value,
        });
    }
    if !(1.0..=3.0).contains(&rounded) {
        return Err(HillslopeGrowthBoundaryError::UnsupportedManagementClass {
            phase,
            symbol: BoundarySymbol::from(symbol),
            value,
        });
    }
    if (rounded - 1.0).abs() <= MANAGEMENT_CLASS_EPSILON {
        return Ok(1);
    }
    if (rounded - 2.0).abs() <= MANAGEMENT_CLASS_EPSILON {
        return Ok(2);
    }
    if (rounded - 3.0).abs() <= MANAGEMENT_CLASS_EPSILON {
        return Ok(3);
    }

    Err(HillslopeGrowthBoundaryError::UnsupportedManagementClass {
        phase,
        symbol: BoundarySymbol::from(symbol),
        value,
    })
}

fn require_ordering_flag_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    expected: f64,
) -> Result<f64, HillslopeDecompositionBoundaryError> {
    let observed = require_finite_state_value_for_decomposition(phase, state_surface, symbol)?;
    if (observed - expected).abs() > ORDER_FLAG_EPSILON {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidOrderingFlagValue {
                phase,
                symbol: BoundarySymbol::from(symbol),
                observed,
                expected,
            },
        );
    }

    Ok(observed)
}

fn require_finite_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
) -> Result<f64, HillslopeDecompositionBoundaryError> {
    let symbol_key = BoundarySymbol::from(symbol);
    let value = state_surface
        .get(&symbol_key)
        .ok_or_else(
            || HillslopeDecompositionBoundaryError::MissingRequiredStateSymbol {
                phase,
                symbol: symbol_key.clone(),
            },
        )?
        .as_f64();

    if !value.is_finite() {
        return Err(
            HillslopeDecompositionBoundaryError::NonFiniteRequiredStateSymbol {
                phase,
                symbol: symbol_key,
                value,
            },
        );
    }

    Ok(value)
}

fn normalize_management_class_for_decomposition(
    phase: HillslopePhase,
    value: f64,
    symbol: &str,
) -> Result<u8, HillslopeDecompositionBoundaryError> {
    let rounded = value.round();
    if (value - rounded).abs() > MANAGEMENT_CLASS_EPSILON {
        return Err(
            HillslopeDecompositionBoundaryError::UnsupportedManagementClass {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
            },
        );
    }
    if !(1.0..=3.0).contains(&rounded) {
        return Err(
            HillslopeDecompositionBoundaryError::UnsupportedManagementClass {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
            },
        );
    }
    if (rounded - 1.0).abs() <= MANAGEMENT_CLASS_EPSILON {
        return Ok(1);
    }
    if (rounded - 2.0).abs() <= MANAGEMENT_CLASS_EPSILON {
        return Ok(2);
    }
    if (rounded - 3.0).abs() <= MANAGEMENT_CLASS_EPSILON {
        return Ok(3);
    }

    Err(
        HillslopeDecompositionBoundaryError::UnsupportedManagementClass {
            phase,
            symbol: BoundarySymbol::from(symbol),
            value,
        },
    )
}

fn usize_to_u16_for_decomposition(
    phase: HillslopePhase,
    symbol: BoundarySymbol,
    value: usize,
) -> Result<u16, HillslopeDecompositionBoundaryError> {
    u16::try_from(value).map_err(|_| {
        HillslopeDecompositionBoundaryError::StateSymbolValueOutOfRange {
            phase,
            symbol,
            value,
            min_allowed: 0,
            max_allowed: usize::from(u16::MAX),
        }
    })
}

fn usize_to_u8_for_decomposition(
    phase: HillslopePhase,
    symbol: BoundarySymbol,
    value: usize,
) -> Result<u8, HillslopeDecompositionBoundaryError> {
    u8::try_from(value).map_err(|_| {
        HillslopeDecompositionBoundaryError::StateSymbolValueOutOfRange {
            phase,
            symbol,
            value,
            min_allowed: 0,
            max_allowed: usize::from(u8::MAX),
        }
    })
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn require_integral_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeDecompositionBoundaryError> {
    let value = require_finite_state_value_for_decomposition(phase, state_surface, symbol)?;
    let rounded = value.round();
    if (value - rounded).abs() > MANAGEMENT_CLASS_EPSILON {
        return Err(
            HillslopeDecompositionBoundaryError::NonIntegralRequiredStateSymbol {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
            },
        );
    }

    let min_f64 = min_allowed as f64;
    let max_f64 = max_allowed as f64;
    if rounded < min_f64 || rounded > max_f64 {
        return Err(
            HillslopeDecompositionBoundaryError::StateSymbolValueOutOfRange {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value: rounded as usize,
                min_allowed,
                max_allowed,
            },
        );
    }

    Ok(rounded as usize)
}

fn require_day_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    allow_zero: bool,
) -> Result<usize, HillslopeDecompositionBoundaryError> {
    let min_allowed = usize::from(!allow_zero);
    require_integral_state_value_for_decomposition(phase, state_surface, symbol, min_allowed, 366)
}

fn require_fraction_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
) -> Result<f64, HillslopeDecompositionBoundaryError> {
    let value = require_finite_state_value_for_decomposition(phase, state_surface, symbol)?;
    if !(0.0..=1.0).contains(&value) {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
                reason: "expected fraction in [0,1]",
            },
        );
    }
    Ok(value)
}

fn require_zero_state_value_for_decomposition(
    phase: HillslopePhase,
    symbol: &str,
    value: f64,
    reason: &'static str,
) -> Result<(), HillslopeDecompositionBoundaryError> {
    if value.abs() > ORDER_FLAG_EPSILON {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
                reason,
            },
        );
    }
    Ok(())
}

fn parse_indexed_suffix_for_decomposition(suffix: &str) -> Option<usize> {
    if suffix.len() != 4 {
        return None;
    }
    if !suffix.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    suffix.parse::<usize>().ok()
}

fn ensure_no_overflow_indexed_symbols_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    root: &str,
    slot_index: usize,
    crop_slot_index: usize,
    max_expected: usize,
) -> Result<(), HillslopeDecompositionBoundaryError> {
    let prefix = format!("pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}_");
    for symbol in state_surface.keys() {
        if let Some(suffix) = symbol.as_str().strip_prefix(prefix.as_str())
            && let Some(index) = parse_indexed_suffix_for_decomposition(suffix)
            && (index == 0 || index > max_expected)
        {
            return Err(
                HillslopeDecompositionBoundaryError::UnexpectedIndexedStateSymbol {
                    phase,
                    symbol: symbol.clone(),
                    index,
                    max_expected,
                },
            );
        }
    }
    Ok(())
}

fn require_indexed_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    index: usize,
) -> Result<f64, HillslopeDecompositionBoundaryError> {
    if !state_surface.contains_key(&BoundarySymbol::from(symbol)) {
        return Err(
            HillslopeDecompositionBoundaryError::MissingIndexedStateSymbol {
                phase,
                symbol: BoundarySymbol::from(symbol),
                index,
            },
        );
    }
    require_finite_state_value_for_decomposition(phase, state_surface, symbol)
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn require_indexed_integral_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    index: usize,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeDecompositionBoundaryError> {
    let value = require_indexed_state_value_for_decomposition(phase, state_surface, symbol, index)?;
    let rounded = value.round();
    if (value - rounded).abs() > MANAGEMENT_CLASS_EPSILON {
        return Err(
            HillslopeDecompositionBoundaryError::NonIntegralRequiredStateSymbol {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
            },
        );
    }

    let min_f64 = min_allowed as f64;
    let max_f64 = max_allowed as f64;
    if rounded < min_f64 || rounded > max_f64 {
        return Err(
            HillslopeDecompositionBoundaryError::StateSymbolValueOutOfRange {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value: rounded as usize,
                min_allowed,
                max_allowed,
            },
        );
    }

    Ok(rounded as usize)
}

fn require_indexed_positive_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    index: usize,
) -> Result<f64, HillslopeDecompositionBoundaryError> {
    let value = require_indexed_state_value_for_decomposition(phase, state_surface, symbol, index)?;
    if value <= 0.0 {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
                reason: "expected positive value",
            },
        );
    }
    Ok(value)
}

fn require_indexed_fraction_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    index: usize,
) -> Result<f64, HillslopeDecompositionBoundaryError> {
    let value = require_indexed_state_value_for_decomposition(phase, state_surface, symbol, index)?;
    if !(0.0..=1.0).contains(&value) {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
                reason: "expected fraction in [0,1]",
            },
        );
    }
    Ok(value)
}

#[allow(
    clippy::too_many_lines,
    clippy::cast_precision_loss,
    clippy::similar_names
)]
fn build_annual_decomposition_control(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    runtime_day: usize,
) -> Result<HillslopeAnnualDecompositionControl, HillslopeDecompositionBoundaryError> {
    let resmgt_symbol = pl_decomp_slot_crop_symbol("resmgt", slot_index, crop_slot_index);
    let resmgt = require_integral_state_value_for_decomposition(
        phase,
        state_surface,
        resmgt_symbol.as_str(),
        1,
        6,
    )?;

    let jdherb_symbol = pl_decomp_slot_crop_symbol("jdherb", slot_index, crop_slot_index);
    let jdburn_symbol = pl_decomp_slot_crop_symbol("jdburn", slot_index, crop_slot_index);
    let jdslge_symbol = pl_decomp_slot_crop_symbol("jdslge", slot_index, crop_slot_index);
    let jdcut_symbol = pl_decomp_slot_crop_symbol("jdcut", slot_index, crop_slot_index);
    let jdmove_symbol = pl_decomp_slot_crop_symbol("jdmove", slot_index, crop_slot_index);
    let fbrnag_symbol = pl_decomp_slot_crop_symbol("fbrnag", slot_index, crop_slot_index);
    let fbrnog_symbol = pl_decomp_slot_crop_symbol("fbrnog", slot_index, crop_slot_index);
    let frcut_symbol = pl_decomp_slot_crop_symbol("frcut", slot_index, crop_slot_index);
    let frmove_symbol = pl_decomp_slot_crop_symbol("frmove", slot_index, crop_slot_index);

    let jdherb = require_day_state_value_for_decomposition(
        phase,
        state_surface,
        jdherb_symbol.as_str(),
        true,
    )?;
    let jdburn = require_day_state_value_for_decomposition(
        phase,
        state_surface,
        jdburn_symbol.as_str(),
        true,
    )?;
    let jdslge = require_day_state_value_for_decomposition(
        phase,
        state_surface,
        jdslge_symbol.as_str(),
        true,
    )?;
    let jdcut = require_day_state_value_for_decomposition(
        phase,
        state_surface,
        jdcut_symbol.as_str(),
        true,
    )?;
    let jdmove = require_day_state_value_for_decomposition(
        phase,
        state_surface,
        jdmove_symbol.as_str(),
        true,
    )?;
    let fbrnag = require_fraction_state_value_for_decomposition(
        phase,
        state_surface,
        fbrnag_symbol.as_str(),
    )?;
    let fbrnog = require_fraction_state_value_for_decomposition(
        phase,
        state_surface,
        fbrnog_symbol.as_str(),
    )?;
    let frcut = require_fraction_state_value_for_decomposition(
        phase,
        state_surface,
        frcut_symbol.as_str(),
    )?;
    let frmove = require_fraction_state_value_for_decomposition(
        phase,
        state_surface,
        frmove_symbol.as_str(),
    )?;

    let active_action = match resmgt {
        1 => {
            if jdherb == 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(jdherb_symbol.as_str()),
                        value: 0.0,
                        reason: "resmgt=1 requires jdherb in 1..366",
                    },
                );
            }
            require_zero_state_value_for_decomposition(
                phase,
                jdburn_symbol.as_str(),
                jdburn as f64,
                "resmgt=1 requires jdburn=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdslge_symbol.as_str(),
                jdslge as f64,
                "resmgt=1 requires jdslge=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdcut_symbol.as_str(),
                jdcut as f64,
                "resmgt=1 requires jdcut=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdmove_symbol.as_str(),
                jdmove as f64,
                "resmgt=1 requires jdmove=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                fbrnag_symbol.as_str(),
                fbrnag,
                "resmgt=1 requires fbrnag=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                fbrnog_symbol.as_str(),
                fbrnog,
                "resmgt=1 requires fbrnog=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                frcut_symbol.as_str(),
                frcut,
                "resmgt=1 requires frcut=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                frmove_symbol.as_str(),
                frmove,
                "resmgt=1 requires frmove=0",
            )?;
            if runtime_day == jdherb {
                HillslopeAnnualDecompositionAction::Herbicide
            } else {
                HillslopeAnnualDecompositionAction::None
            }
        }
        2 => {
            if jdburn == 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(jdburn_symbol.as_str()),
                        value: 0.0,
                        reason: "resmgt=2 requires jdburn in 1..366",
                    },
                );
            }
            require_zero_state_value_for_decomposition(
                phase,
                jdherb_symbol.as_str(),
                jdherb as f64,
                "resmgt=2 requires jdherb=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdslge_symbol.as_str(),
                jdslge as f64,
                "resmgt=2 requires jdslge=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdcut_symbol.as_str(),
                jdcut as f64,
                "resmgt=2 requires jdcut=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdmove_symbol.as_str(),
                jdmove as f64,
                "resmgt=2 requires jdmove=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                frcut_symbol.as_str(),
                frcut,
                "resmgt=2 requires frcut=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                frmove_symbol.as_str(),
                frmove,
                "resmgt=2 requires frmove=0",
            )?;
            if runtime_day == jdburn {
                HillslopeAnnualDecompositionAction::Burn
            } else {
                HillslopeAnnualDecompositionAction::None
            }
        }
        3 => {
            if jdslge == 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(jdslge_symbol.as_str()),
                        value: 0.0,
                        reason: "resmgt=3 requires jdslge in 1..366",
                    },
                );
            }
            require_zero_state_value_for_decomposition(
                phase,
                jdherb_symbol.as_str(),
                jdherb as f64,
                "resmgt=3 requires jdherb=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdburn_symbol.as_str(),
                jdburn as f64,
                "resmgt=3 requires jdburn=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdcut_symbol.as_str(),
                jdcut as f64,
                "resmgt=3 requires jdcut=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdmove_symbol.as_str(),
                jdmove as f64,
                "resmgt=3 requires jdmove=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                fbrnag_symbol.as_str(),
                fbrnag,
                "resmgt=3 requires fbrnag=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                fbrnog_symbol.as_str(),
                fbrnog,
                "resmgt=3 requires fbrnog=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                frcut_symbol.as_str(),
                frcut,
                "resmgt=3 requires frcut=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                frmove_symbol.as_str(),
                frmove,
                "resmgt=3 requires frmove=0",
            )?;
            if runtime_day == jdslge {
                HillslopeAnnualDecompositionAction::Silage
            } else {
                HillslopeAnnualDecompositionAction::None
            }
        }
        4 => {
            if jdcut == 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(jdcut_symbol.as_str()),
                        value: 0.0,
                        reason: "resmgt=4 requires jdcut in 1..366",
                    },
                );
            }
            require_zero_state_value_for_decomposition(
                phase,
                jdherb_symbol.as_str(),
                jdherb as f64,
                "resmgt=4 requires jdherb=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdburn_symbol.as_str(),
                jdburn as f64,
                "resmgt=4 requires jdburn=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdslge_symbol.as_str(),
                jdslge as f64,
                "resmgt=4 requires jdslge=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdmove_symbol.as_str(),
                jdmove as f64,
                "resmgt=4 requires jdmove=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                fbrnag_symbol.as_str(),
                fbrnag,
                "resmgt=4 requires fbrnag=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                fbrnog_symbol.as_str(),
                fbrnog,
                "resmgt=4 requires fbrnog=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                frmove_symbol.as_str(),
                frmove,
                "resmgt=4 requires frmove=0",
            )?;
            if runtime_day == jdcut {
                HillslopeAnnualDecompositionAction::Cut
            } else {
                HillslopeAnnualDecompositionAction::None
            }
        }
        5 => {
            if jdmove == 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(jdmove_symbol.as_str()),
                        value: 0.0,
                        reason: "resmgt=5 requires jdmove in 1..366",
                    },
                );
            }
            require_zero_state_value_for_decomposition(
                phase,
                jdherb_symbol.as_str(),
                jdherb as f64,
                "resmgt=5 requires jdherb=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdburn_symbol.as_str(),
                jdburn as f64,
                "resmgt=5 requires jdburn=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdslge_symbol.as_str(),
                jdslge as f64,
                "resmgt=5 requires jdslge=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdcut_symbol.as_str(),
                jdcut as f64,
                "resmgt=5 requires jdcut=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                fbrnag_symbol.as_str(),
                fbrnag,
                "resmgt=5 requires fbrnag=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                fbrnog_symbol.as_str(),
                fbrnog,
                "resmgt=5 requires fbrnog=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                frcut_symbol.as_str(),
                frcut,
                "resmgt=5 requires frcut=0",
            )?;
            if runtime_day == jdmove {
                HillslopeAnnualDecompositionAction::Remove
            } else {
                HillslopeAnnualDecompositionAction::None
            }
        }
        6 => {
            require_zero_state_value_for_decomposition(
                phase,
                jdherb_symbol.as_str(),
                jdherb as f64,
                "resmgt=6 requires jdherb=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdburn_symbol.as_str(),
                jdburn as f64,
                "resmgt=6 requires jdburn=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdslge_symbol.as_str(),
                jdslge as f64,
                "resmgt=6 requires jdslge=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdcut_symbol.as_str(),
                jdcut as f64,
                "resmgt=6 requires jdcut=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                jdmove_symbol.as_str(),
                jdmove as f64,
                "resmgt=6 requires jdmove=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                fbrnag_symbol.as_str(),
                fbrnag,
                "resmgt=6 requires fbrnag=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                fbrnog_symbol.as_str(),
                fbrnog,
                "resmgt=6 requires fbrnog=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                frcut_symbol.as_str(),
                frcut,
                "resmgt=6 requires frcut=0",
            )?;
            require_zero_state_value_for_decomposition(
                phase,
                frmove_symbol.as_str(),
                frmove,
                "resmgt=6 requires frmove=0",
            )?;
            HillslopeAnnualDecompositionAction::None
        }
        _ => unreachable!("resmgt domain is validated above"),
    };

    Ok(HillslopeAnnualDecompositionControl {
        resmgt: usize_to_u8_for_decomposition(phase, BoundarySymbol::from(resmgt_symbol), resmgt)?,
        jdherb: usize_to_u16_for_decomposition(phase, BoundarySymbol::from(jdherb_symbol), jdherb)?,
        jdburn: usize_to_u16_for_decomposition(phase, BoundarySymbol::from(jdburn_symbol), jdburn)?,
        jdslge: usize_to_u16_for_decomposition(phase, BoundarySymbol::from(jdslge_symbol), jdslge)?,
        jdcut: usize_to_u16_for_decomposition(phase, BoundarySymbol::from(jdcut_symbol), jdcut)?,
        jdmove: usize_to_u16_for_decomposition(phase, BoundarySymbol::from(jdmove_symbol), jdmove)?,
        fbrnag,
        fbrnog,
        frcut,
        frmove,
        active_action,
    })
}

#[allow(clippy::too_many_lines, clippy::cast_precision_loss)]
fn build_perennial_decomposition_control(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    runtime_day: usize,
) -> Result<HillslopePerennialDecompositionControl, HillslopeDecompositionBoundaryError> {
    let mgtopt_symbol = pl_decomp_slot_crop_symbol("mgtopt", slot_index, crop_slot_index);
    let ncut_symbol = pl_decomp_slot_crop_symbol("ncut", slot_index, crop_slot_index);
    let ncycle_symbol = pl_decomp_slot_crop_symbol("ncycle", slot_index, crop_slot_index);
    let mgtopt = require_integral_state_value_for_decomposition(
        phase,
        state_surface,
        mgtopt_symbol.as_str(),
        1,
        3,
    )?;
    let ncut = require_integral_state_value_for_decomposition(
        phase,
        state_surface,
        ncut_symbol.as_str(),
        0,
        usize::from(u16::MAX),
    )?;
    let ncycle = require_integral_state_value_for_decomposition(
        phase,
        state_surface,
        ncycle_symbol.as_str(),
        0,
        usize::from(u16::MAX),
    )?;

    let mut active_action = HillslopePerennialDecompositionAction::None;
    let mut active_grazing_cycle = None;

    match mgtopt {
        1 => {
            if ncut == 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(ncut_symbol.as_str()),
                        value: 0.0,
                        reason: "mgtopt=1 requires ncut>=1",
                    },
                );
            }
            if ncycle != 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(ncycle_symbol.as_str()),
                        value: ncycle as f64,
                        reason: "mgtopt=1 requires ncycle=0",
                    },
                );
            }

            ensure_no_overflow_indexed_symbols_for_decomposition(
                phase,
                state_surface,
                "cutday",
                slot_index,
                crop_slot_index,
                ncut,
            )?;
            for root in ["gday", "gend", "animal", "bodywt", "area", "digest"] {
                ensure_no_overflow_indexed_symbols_for_decomposition(
                    phase,
                    state_surface,
                    root,
                    slot_index,
                    crop_slot_index,
                    0,
                )?;
            }

            let mut active_cut_index = None;
            for event_index in 1..=ncut {
                let cut_symbol = pl_decomp_slot_crop_indexed_symbol(
                    "cutday",
                    slot_index,
                    crop_slot_index,
                    event_index,
                );
                let cutday = require_indexed_integral_state_value_for_decomposition(
                    phase,
                    state_surface,
                    cut_symbol.as_str(),
                    event_index,
                    1,
                    366,
                )?;
                if runtime_day == cutday {
                    if active_cut_index.is_some() {
                        return Err(
                            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                                phase,
                                symbol: BoundarySymbol::from(cut_symbol.as_str()),
                                value: cutday as f64,
                                reason: "multiple cutday entries active on runtime day",
                            },
                        );
                    }
                    active_cut_index = Some(event_index);
                }
            }

            if let Some(event_index) = active_cut_index {
                active_action = HillslopePerennialDecompositionAction::Cut {
                    event_index: usize_to_u16_for_decomposition(
                        phase,
                        BoundarySymbol::from("cutday"),
                        event_index,
                    )?,
                };
            }
        }
        2 => {
            if ncycle == 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(ncycle_symbol.as_str()),
                        value: 0.0,
                        reason: "mgtopt=2 requires ncycle>=1",
                    },
                );
            }
            if ncut != 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(ncut_symbol.as_str()),
                        value: ncut as f64,
                        reason: "mgtopt=2 requires ncut=0",
                    },
                );
            }

            ensure_no_overflow_indexed_symbols_for_decomposition(
                phase,
                state_surface,
                "cutday",
                slot_index,
                crop_slot_index,
                0,
            )?;
            for root in ["gday", "gend", "animal", "bodywt", "area", "digest"] {
                ensure_no_overflow_indexed_symbols_for_decomposition(
                    phase,
                    state_surface,
                    root,
                    slot_index,
                    crop_slot_index,
                    ncycle,
                )?;
            }

            for cycle_index in 1..=ncycle {
                let gday_symbol = pl_decomp_slot_crop_indexed_symbol(
                    "gday",
                    slot_index,
                    crop_slot_index,
                    cycle_index,
                );
                let gend_symbol = pl_decomp_slot_crop_indexed_symbol(
                    "gend",
                    slot_index,
                    crop_slot_index,
                    cycle_index,
                );
                let animal_symbol = pl_decomp_slot_crop_indexed_symbol(
                    "animal",
                    slot_index,
                    crop_slot_index,
                    cycle_index,
                );
                let bodywt_symbol = pl_decomp_slot_crop_indexed_symbol(
                    "bodywt",
                    slot_index,
                    crop_slot_index,
                    cycle_index,
                );
                let area_symbol = pl_decomp_slot_crop_indexed_symbol(
                    "area",
                    slot_index,
                    crop_slot_index,
                    cycle_index,
                );
                let digest_symbol = pl_decomp_slot_crop_indexed_symbol(
                    "digest",
                    slot_index,
                    crop_slot_index,
                    cycle_index,
                );

                let gday = require_indexed_integral_state_value_for_decomposition(
                    phase,
                    state_surface,
                    gday_symbol.as_str(),
                    cycle_index,
                    1,
                    366,
                )?;
                let gend = require_indexed_integral_state_value_for_decomposition(
                    phase,
                    state_surface,
                    gend_symbol.as_str(),
                    cycle_index,
                    1,
                    366,
                )?;
                if gday >= gend {
                    return Err(HillslopeDecompositionBoundaryError::InvalidGrazingWindow {
                        phase,
                        cycle_index,
                        gday_symbol: BoundarySymbol::from(gday_symbol.as_str()),
                        gend_symbol: BoundarySymbol::from(gend_symbol.as_str()),
                        gday,
                        gend,
                    });
                }

                let animal = require_indexed_positive_state_value_for_decomposition(
                    phase,
                    state_surface,
                    animal_symbol.as_str(),
                    cycle_index,
                )?;
                let bodywt = require_indexed_positive_state_value_for_decomposition(
                    phase,
                    state_surface,
                    bodywt_symbol.as_str(),
                    cycle_index,
                )?;
                let area = require_indexed_positive_state_value_for_decomposition(
                    phase,
                    state_surface,
                    area_symbol.as_str(),
                    cycle_index,
                )?;
                let digest = require_indexed_fraction_state_value_for_decomposition(
                    phase,
                    state_surface,
                    digest_symbol.as_str(),
                    cycle_index,
                )?;
                if digest == 0.0 {
                    return Err(
                        HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                            phase,
                            symbol: BoundarySymbol::from(digest_symbol.as_str()),
                            value: digest,
                            reason: "grazing digest must be positive",
                        },
                    );
                }

                let in_window = runtime_day >= gday && runtime_day < gend;
                if in_window {
                    if active_grazing_cycle.is_some() {
                        return Err(
                            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                                phase,
                                symbol: BoundarySymbol::from(gday_symbol.as_str()),
                                value: runtime_day as f64,
                                reason: "multiple grazing cycles active on runtime day",
                            },
                        );
                    }
                    active_grazing_cycle = Some(HillslopeActiveGrazingCycle {
                        cycle_index: usize_to_u16_for_decomposition(
                            phase,
                            BoundarySymbol::from("cycle_index"),
                            cycle_index,
                        )?,
                        gday: usize_to_u16_for_decomposition(
                            phase,
                            BoundarySymbol::from(gday_symbol.as_str()),
                            gday,
                        )?,
                        gend: usize_to_u16_for_decomposition(
                            phase,
                            BoundarySymbol::from(gend_symbol.as_str()),
                            gend,
                        )?,
                        animal,
                        bodywt,
                        area,
                        digest,
                    });
                }
            }

            if let Some(cycle) = active_grazing_cycle {
                active_action = HillslopePerennialDecompositionAction::Grazing {
                    cycle_index: cycle.cycle_index,
                };
            }
        }
        3 => {
            if ncut != 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(ncut_symbol.as_str()),
                        value: ncut as f64,
                        reason: "mgtopt=3 requires ncut=0",
                    },
                );
            }
            if ncycle != 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(ncycle_symbol.as_str()),
                        value: ncycle as f64,
                        reason: "mgtopt=3 requires ncycle=0",
                    },
                );
            }

            for root in [
                "cutday", "gday", "gend", "animal", "bodywt", "area", "digest",
            ] {
                ensure_no_overflow_indexed_symbols_for_decomposition(
                    phase,
                    state_surface,
                    root,
                    slot_index,
                    crop_slot_index,
                    0,
                )?;
            }
        }
        _ => unreachable!("mgtopt domain is validated above"),
    }

    Ok(HillslopePerennialDecompositionControl {
        mgtopt: usize_to_u8_for_decomposition(phase, BoundarySymbol::from(mgtopt_symbol), mgtopt)?,
        ncut: usize_to_u16_for_decomposition(phase, BoundarySymbol::from(ncut_symbol), ncut)?,
        ncycle: usize_to_u16_for_decomposition(phase, BoundarySymbol::from(ncycle_symbol), ncycle)?,
        active_action,
        active_grazing_cycle,
    })
}

/// Explicit scheduler dependency edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhaseDependency {
    pub phase: HillslopePhase,
    pub depends_on: HillslopePhase,
}

/// Deterministic dependency graph for hillslope phase ordering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HillslopePhaseGraph {
    dependencies: BTreeMap<HillslopePhase, Vec<HillslopePhase>>,
}

impl HillslopePhaseGraph {
    /// Build the canonical ARCH05 deterministic graph.
    #[must_use]
    pub fn canonical() -> Self {
        let mut dependencies: BTreeMap<HillslopePhase, Vec<HillslopePhase>> = BTreeMap::new();
        for phase in HillslopePhase::ORDERED {
            dependencies.insert(phase, Vec::new());
        }

        for edge in Self::canonical_dependencies() {
            dependencies
                .entry(edge.phase)
                .or_default()
                .push(edge.depends_on);
        }

        for deps in dependencies.values_mut() {
            deps.sort_by_key(|phase| phase.rank());
            deps.dedup();
        }

        Self { dependencies }
    }

    #[must_use]
    pub fn dependencies_for(&self, phase: HillslopePhase) -> &[HillslopePhase] {
        self.dependencies
            .get(&phase)
            .map_or(&[] as &[HillslopePhase], Vec::as_slice)
    }

    #[must_use]
    pub const fn canonical_order() -> [HillslopePhase; PHASE_COUNT] {
        HillslopePhase::ORDERED
    }

    #[must_use]
    pub fn dependency_edges(&self) -> Vec<PhaseDependency> {
        let mut edges = Vec::new();

        for phase in HillslopePhase::ORDERED {
            if let Some(deps) = self.dependencies.get(&phase) {
                for dependency in deps {
                    edges.push(PhaseDependency {
                        phase,
                        depends_on: *dependency,
                    });
                }
            }
        }

        edges
    }

    #[must_use]
    pub fn topological_order(&self) -> Option<Vec<HillslopePhase>> {
        let mut indegree: BTreeMap<HillslopePhase, usize> = BTreeMap::new();
        let mut adjacency: BTreeMap<HillslopePhase, BTreeSet<HillslopePhase>> = BTreeMap::new();

        for phase in HillslopePhase::ORDERED {
            indegree.insert(phase, 0);
            adjacency.insert(phase, BTreeSet::new());
        }

        for phase in HillslopePhase::ORDERED {
            for dependency in self.dependencies_for(phase) {
                let value = indegree.get_mut(&phase)?;
                *value += 1;

                adjacency.entry(*dependency).or_default().insert(phase);
            }
        }

        let mut ready: Vec<HillslopePhase> = HillslopePhase::ORDERED
            .iter()
            .copied()
            .filter(|phase| indegree.get(phase).copied().unwrap_or(0) == 0)
            .collect();
        let mut order = Vec::with_capacity(PHASE_COUNT);

        while !ready.is_empty() {
            ready.sort_by_key(|phase| phase.rank());
            let phase = ready.remove(0);
            order.push(phase);

            if let Some(neighbors) = adjacency.get(&phase) {
                for neighbor in neighbors {
                    let count = indegree.get_mut(neighbor)?;

                    if *count == 0 {
                        return None;
                    }

                    *count -= 1;
                    if *count == 0 {
                        ready.push(*neighbor);
                    }
                }
            }
        }

        if order.len() == PHASE_COUNT {
            Some(order)
        } else {
            None
        }
    }

    #[must_use]
    const fn canonical_dependencies() -> [PhaseDependency; PHASE_COUNT - 1] {
        [
            PhaseDependency {
                phase: HillslopePhase::StorageBounds,
                depends_on: HillslopePhase::Normalization,
            },
            PhaseDependency {
                phase: HillslopePhase::DecompositionTransition,
                depends_on: HillslopePhase::StorageBounds,
            },
            PhaseDependency {
                phase: HillslopePhase::ResiduePartitionTransition,
                depends_on: HillslopePhase::DecompositionTransition,
            },
            PhaseDependency {
                phase: HillslopePhase::AnnualGrowthTransition,
                depends_on: HillslopePhase::ResiduePartitionTransition,
            },
            PhaseDependency {
                phase: HillslopePhase::PerennialGrowthTransition,
                depends_on: HillslopePhase::AnnualGrowthTransition,
            },
            PhaseDependency {
                phase: HillslopePhase::Evapotranspiration,
                depends_on: HillslopePhase::PerennialGrowthTransition,
            },
            PhaseDependency {
                phase: HillslopePhase::PercolationDeepSeepage,
                depends_on: HillslopePhase::Evapotranspiration,
            },
            PhaseDependency {
                phase: HillslopePhase::LateralTransfer,
                depends_on: HillslopePhase::PercolationDeepSeepage,
            },
            PhaseDependency {
                phase: HillslopePhase::Drainage,
                depends_on: HillslopePhase::LateralTransfer,
            },
            PhaseDependency {
                phase: HillslopePhase::RunoffReconciliation,
                depends_on: HillslopePhase::Drainage,
            },
            PhaseDependency {
                phase: HillslopePhase::StorageReconciliation,
                depends_on: HillslopePhase::RunoffReconciliation,
            },
            PhaseDependency {
                phase: HillslopePhase::ClosureDiagnostics,
                depends_on: HillslopePhase::StorageReconciliation,
            },
        ]
    }
}

impl Default for HillslopePhaseGraph {
    fn default() -> Self {
        Self::canonical()
    }
}

/// One executed phase and its typed status surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HillslopePhaseOutcome {
    pub phase: HillslopePhase,
    pub status: SimulationStatus,
}

/// Coarse scheduler completion class for deterministic decision routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerOutcomeClass {
    Completed,
    TopologyPreconditionFailed,
    PhaseFailure,
    SchedulerInvariantFailure,
}

/// Scheduler execution report.
#[derive(Debug, Clone)]
pub struct HillslopeSchedulerReport {
    pub outcome_class: SchedulerOutcomeClass,
    pub topology_precondition_status: SimulationStatus,
    pub scheduler_status: SimulationStatus,
    pub ordered_phases: Vec<HillslopePhase>,
    pub outcomes: Vec<HillslopePhaseOutcome>,
    pub precondition_violations: Vec<ClosureViolation>,
    pub halted_phase: Option<HillslopePhase>,
}

impl HillslopeSchedulerReport {
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.outcome_class == SchedulerOutcomeClass::Completed
            && self.scheduler_status.classification() != StatusClassification::Failure
    }

    #[must_use]
    pub fn executed_phases(&self) -> Vec<HillslopePhase> {
        self.outcomes.iter().map(|outcome| outcome.phase).collect()
    }
}

/// Mutable state/flux maps owned by the hillslope orchestrator.
#[derive(Debug, Clone, Default)]
pub struct HillslopeWritebackSurface {
    pub state_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
    pub flux_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
}

/// Per-phase kernel/writeback execution evidence.
#[derive(Debug, Clone)]
pub struct HillslopeKernelPhaseReport {
    pub phase: HillslopePhase,
    pub kernel_status: SimulationStatus,
    pub decision_outcome: WritebackDecisionOutcome,
    pub decision_status: SimulationStatus,
    pub apply_result: Option<KernelWritebackApplyResult>,
}

/// Kernel-integrated hillslope execution report.
#[derive(Debug, Clone)]
pub struct HillslopeKernelExecutionReport {
    pub scheduler_report: HillslopeSchedulerReport,
    pub phase_reports: Vec<HillslopeKernelPhaseReport>,
    pub writeback_surface: HillslopeWritebackSurface,
}

/// Scheduler construction/operation error.
#[derive(Debug)]
pub enum HillslopeSchedulerError {
    Status(StatusError),
    Writeback(WritebackError),
}

impl fmt::Display for HillslopeSchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status(source) => write!(f, "status construction failed: {source}"),
            Self::Writeback(source) => write!(f, "writeback application failed: {source}"),
        }
    }
}

impl Error for HillslopeSchedulerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Status(source) => Some(source),
            Self::Writeback(source) => Some(source),
        }
    }
}

impl From<StatusError> for HillslopeSchedulerError {
    fn from(value: StatusError) -> Self {
        Self::Status(value)
    }
}

impl From<WritebackError> for HillslopeSchedulerError {
    fn from(value: WritebackError) -> Self {
        Self::Writeback(value)
    }
}

/// Deterministic hillslope scheduler.
#[derive(Debug, Clone)]
pub struct HillslopePhaseScheduler {
    graph: HillslopePhaseGraph,
}

impl HillslopePhaseScheduler {
    #[must_use]
    pub fn canonical() -> Self {
        Self {
            graph: HillslopePhaseGraph::canonical(),
        }
    }

    #[must_use]
    pub fn new(graph: HillslopePhaseGraph) -> Self {
        Self { graph }
    }

    #[must_use]
    pub fn graph(&self) -> &HillslopePhaseGraph {
        &self.graph
    }

    /// Build a nominal phase status for deterministic test/driver defaults.
    pub fn nominal_phase_status(phase: HillslopePhase) -> Result<SimulationStatus, StatusError> {
        SimulationStatus::ok(SimulationPhase::HillslopeKernel, phase.ok_message_id())
    }

    /// Execute deterministic phase scheduling with topology precondition gating.
    #[allow(clippy::too_many_lines)]
    pub fn execute_with<F>(
        &self,
        topology_report: &TopologyValidationReport,
        mut phase_executor: F,
    ) -> Result<HillslopeSchedulerReport, HillslopeSchedulerError>
    where
        F: FnMut(HillslopePhase) -> SimulationStatus,
    {
        if topology_report.status.classification() == StatusClassification::Failure {
            return Ok(HillslopeSchedulerReport {
                outcome_class: SchedulerOutcomeClass::TopologyPreconditionFailed,
                topology_precondition_status: topology_report.status.clone(),
                scheduler_status: topology_report.status.clone(),
                ordered_phases: Vec::new(),
                outcomes: Vec::new(),
                precondition_violations: topology_report.violations.clone(),
                halted_phase: None,
            });
        }

        if !topology_report.violations.is_empty() {
            let status = SimulationStatus::failure(
                SimulationPhase::PreExecutionValidation,
                true,
                false,
                BoundaryClass::TopologyInvalid,
                "HSCHED-E-TOPOLOGY-PRECONDITION",
            )?;

            return Ok(HillslopeSchedulerReport {
                outcome_class: SchedulerOutcomeClass::TopologyPreconditionFailed,
                topology_precondition_status: topology_report.status.clone(),
                scheduler_status: status,
                ordered_phases: Vec::new(),
                outcomes: Vec::new(),
                precondition_violations: topology_report.violations.clone(),
                halted_phase: None,
            });
        }

        let Some(order) = self.graph.topological_order() else {
            let status = SimulationStatus::failure(
                SimulationPhase::HillslopeKernel,
                true,
                false,
                BoundaryClass::ClosureViolation,
                "HSCHED-E-GRAPH-CYCLE",
            )?;

            return Ok(HillslopeSchedulerReport {
                outcome_class: SchedulerOutcomeClass::SchedulerInvariantFailure,
                topology_precondition_status: topology_report.status.clone(),
                scheduler_status: status,
                ordered_phases: Vec::new(),
                outcomes: Vec::new(),
                precondition_violations: Vec::new(),
                halted_phase: None,
            });
        };

        let mut outcomes = Vec::with_capacity(order.len());
        let mut completed: BTreeSet<HillslopePhase> = BTreeSet::new();

        for phase in order.clone() {
            let has_unsatisfied_dependency = self
                .graph
                .dependencies_for(phase)
                .iter()
                .any(|dependency| !completed.contains(dependency));

            if has_unsatisfied_dependency {
                let status = SimulationStatus::failure(
                    SimulationPhase::HillslopeKernel,
                    true,
                    false,
                    BoundaryClass::ClosureViolation,
                    "HSCHED-E-DEPENDENCY-CLOSURE",
                )?;

                return Ok(HillslopeSchedulerReport {
                    outcome_class: SchedulerOutcomeClass::SchedulerInvariantFailure,
                    topology_precondition_status: topology_report.status.clone(),
                    scheduler_status: status,
                    ordered_phases: order,
                    outcomes,
                    precondition_violations: Vec::new(),
                    halted_phase: Some(phase),
                });
            }

            let phase_status = phase_executor(phase);
            if phase_status.phase() != SimulationPhase::HillslopeKernel {
                let status = SimulationStatus::failure(
                    SimulationPhase::HillslopeKernel,
                    true,
                    false,
                    BoundaryClass::ModeMismatch,
                    "HSCHED-E-PHASE-STATUS-PHASE",
                )?;

                outcomes.push(HillslopePhaseOutcome {
                    phase,
                    status: status.clone(),
                });

                return Ok(HillslopeSchedulerReport {
                    outcome_class: SchedulerOutcomeClass::SchedulerInvariantFailure,
                    topology_precondition_status: topology_report.status.clone(),
                    scheduler_status: status,
                    ordered_phases: order,
                    outcomes,
                    precondition_violations: Vec::new(),
                    halted_phase: Some(phase),
                });
            }

            let is_failure = phase_status.classification() == StatusClassification::Failure;
            outcomes.push(HillslopePhaseOutcome {
                phase,
                status: phase_status.clone(),
            });
            completed.insert(phase);

            if is_failure {
                return Ok(HillslopeSchedulerReport {
                    outcome_class: SchedulerOutcomeClass::PhaseFailure,
                    topology_precondition_status: topology_report.status.clone(),
                    scheduler_status: phase_status,
                    ordered_phases: order,
                    outcomes,
                    precondition_violations: Vec::new(),
                    halted_phase: Some(phase),
                });
            }
        }

        let has_advisory = outcomes
            .iter()
            .any(|outcome| outcome.status.classification() == StatusClassification::Advisory);
        let scheduler_status = if has_advisory {
            SimulationStatus::advisory(
                SimulationPhase::HillslopeKernel,
                BoundaryClass::CapBinding,
                ClampClass::None,
                "HSCHED-W-ADVISORY",
            )?
        } else {
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HSCHED-OK-001")?
        };

        Ok(HillslopeSchedulerReport {
            outcome_class: SchedulerOutcomeClass::Completed,
            topology_precondition_status: topology_report.status.clone(),
            scheduler_status,
            ordered_phases: order,
            outcomes,
            precondition_violations: Vec::new(),
            halted_phase: None,
        })
    }

    /// Execute deterministic hillslope scheduling against a typed kernel
    /// boundary with explicit writeback accept/reject/apply handling.
    ///
    /// Kernel outputs are pure proposals; orchestrator-owned writeback surfaces
    /// are the only mutable commit authority.
    #[allow(clippy::too_many_lines)]
    pub fn execute_with_kernel<K>(
        &self,
        topology_report: &TopologyValidationReport,
        kernel: &mut K,
        mut writeback_surface: HillslopeWritebackSurface,
    ) -> Result<HillslopeKernelExecutionReport, HillslopeSchedulerError>
    where
        K: HillslopeKernel,
    {
        let mode_mismatch_status = SimulationStatus::failure(
            SimulationPhase::HillslopeKernel,
            true,
            false,
            BoundaryClass::ModeMismatch,
            "HKERNEL-E-STATUS-PHASE-MISMATCH",
        )?;
        let deferred_error_status = SimulationStatus::failure(
            SimulationPhase::HillslopeKernel,
            true,
            false,
            BoundaryClass::ClosureViolation,
            "HKERNEL-E-WRITEBACK-INTERNAL",
        )?;

        let mut phase_reports = Vec::new();
        let mut deferred_error: Option<HillslopeSchedulerError> = None;

        let scheduler_report = self.execute_with(topology_report, |phase| {
            if deferred_error.is_some() {
                return deferred_error_status.clone();
            }

            let consumer_adapter = hillslope_consumer_adapter_for_phase(phase);
            let phase_class = hillslope_phase_class_for_phase(phase);
            let mut decomposition_context = None;
            let mut growth_context = None;

            if is_decomposition_phase(phase) {
                let decomposition_dispatch = match decomposition_phase_dispatch_for_state(
                    phase,
                    &writeback_surface.state_surface,
                ) {
                    Ok(value) => value,
                    Err(source) => {
                        let boundary_status = match SimulationStatus::failure(
                            SimulationPhase::HillslopeKernel,
                            true,
                            false,
                            source.boundary_class(),
                            source.code(),
                        ) {
                            Ok(status) => status,
                            Err(status_error) => {
                                deferred_error =
                                    Some(HillslopeSchedulerError::Status(status_error));
                                phase_reports.push(HillslopeKernelPhaseReport {
                                    phase,
                                    kernel_status: deferred_error_status.clone(),
                                    decision_outcome: WritebackDecisionOutcome::Reject,
                                    decision_status: deferred_error_status.clone(),
                                    apply_result: None,
                                });
                                return deferred_error_status.clone();
                            }
                        };

                        phase_reports.push(HillslopeKernelPhaseReport {
                            phase,
                            kernel_status: boundary_status.clone(),
                            decision_outcome: WritebackDecisionOutcome::Reject,
                            decision_status: boundary_status.clone(),
                            apply_result: None,
                        });
                        return boundary_status;
                    }
                };

                if let DecompositionPhaseDispatch::Execute(context) = decomposition_dispatch {
                    decomposition_context = Some(context);
                }
            } else if is_growth_phase(phase) {
                let growth_dispatch = match growth_phase_dispatch_for_state(
                    phase,
                    &writeback_surface.state_surface,
                ) {
                    Ok(value) => value,
                    Err(source) => {
                        let boundary_status = match SimulationStatus::failure(
                            SimulationPhase::HillslopeKernel,
                            true,
                            false,
                            source.boundary_class(),
                            source.code(),
                        ) {
                            Ok(status) => status,
                            Err(status_error) => {
                                deferred_error =
                                    Some(HillslopeSchedulerError::Status(status_error));
                                phase_reports.push(HillslopeKernelPhaseReport {
                                    phase,
                                    kernel_status: deferred_error_status.clone(),
                                    decision_outcome: WritebackDecisionOutcome::Reject,
                                    decision_status: deferred_error_status.clone(),
                                    apply_result: None,
                                });
                                return deferred_error_status.clone();
                            }
                        };

                        phase_reports.push(HillslopeKernelPhaseReport {
                            phase,
                            kernel_status: boundary_status.clone(),
                            decision_outcome: WritebackDecisionOutcome::Reject,
                            decision_status: boundary_status.clone(),
                            apply_result: None,
                        });
                        return boundary_status;
                    }
                };

                if let GrowthPhaseDispatch::Execute(context) = growth_dispatch {
                    growth_context = Some(context);
                }
            } else if let Err(source) =
                validate_hillslope_consumer_boundary(phase, &writeback_surface.state_surface)
            {
                let boundary_status = match SimulationStatus::failure(
                    SimulationPhase::HillslopeKernel,
                    true,
                    false,
                    BoundaryClass::MissingRequiredInput,
                    source.code(),
                ) {
                    Ok(status) => status,
                    Err(status_error) => {
                        deferred_error = Some(HillslopeSchedulerError::Status(status_error));
                        phase_reports.push(HillslopeKernelPhaseReport {
                            phase,
                            kernel_status: deferred_error_status.clone(),
                            decision_outcome: WritebackDecisionOutcome::Reject,
                            decision_status: deferred_error_status.clone(),
                            apply_result: None,
                        });
                        return deferred_error_status.clone();
                    }
                };

                phase_reports.push(HillslopeKernelPhaseReport {
                    phase,
                    kernel_status: boundary_status.clone(),
                    decision_outcome: WritebackDecisionOutcome::Reject,
                    decision_status: boundary_status.clone(),
                    apply_result: None,
                });
                return boundary_status;
            }

            let response = {
                let request = HillslopeKernelRequest::with_transition_context(
                    phase.as_str(),
                    phase_class,
                    consumer_adapter,
                    decomposition_context,
                    growth_context,
                    &writeback_surface.state_surface,
                    &writeback_surface.flux_surface,
                );
                kernel.run_hillslope_phase(&request)
            };
            let kernel_status = response.status.clone();

            if kernel_status.phase() != SimulationPhase::HillslopeKernel {
                phase_reports.push(HillslopeKernelPhaseReport {
                    phase,
                    kernel_status,
                    decision_outcome: WritebackDecisionOutcome::Reject,
                    decision_status: mode_mismatch_status.clone(),
                    apply_result: None,
                });
                return mode_mismatch_status.clone();
            }

            if kernel_status.classification() == StatusClassification::Failure {
                phase_reports.push(HillslopeKernelPhaseReport {
                    phase,
                    kernel_status: kernel_status.clone(),
                    decision_outcome: WritebackDecisionOutcome::Reject,
                    decision_status: kernel_status.clone(),
                    apply_result: None,
                });
                return kernel_status;
            }

            let decision = match evaluate_kernel_writeback(
                SimulationPhase::HillslopeKernel,
                &response.writeback,
            ) {
                Ok(value) => value,
                Err(source) => {
                    deferred_error = Some(HillslopeSchedulerError::Status(source));
                    phase_reports.push(HillslopeKernelPhaseReport {
                        phase,
                        kernel_status,
                        decision_outcome: WritebackDecisionOutcome::Reject,
                        decision_status: deferred_error_status.clone(),
                        apply_result: None,
                    });
                    return deferred_error_status.clone();
                }
            };

            if decision.outcome == WritebackDecisionOutcome::Reject {
                phase_reports.push(HillslopeKernelPhaseReport {
                    phase,
                    kernel_status,
                    decision_outcome: WritebackDecisionOutcome::Reject,
                    decision_status: decision.status.clone(),
                    apply_result: None,
                });
                return decision.status;
            }

            let apply_result = match apply_kernel_writeback(
                SimulationPhase::HillslopeKernel,
                &decision,
                &response.writeback,
                &mut writeback_surface.state_surface,
                &mut writeback_surface.flux_surface,
            ) {
                Ok(value) => value,
                Err(source) => {
                    deferred_error = Some(HillslopeSchedulerError::Writeback(source));
                    phase_reports.push(HillslopeKernelPhaseReport {
                        phase,
                        kernel_status,
                        decision_outcome: WritebackDecisionOutcome::Reject,
                        decision_status: deferred_error_status.clone(),
                        apply_result: None,
                    });
                    return deferred_error_status.clone();
                }
            };

            phase_reports.push(HillslopeKernelPhaseReport {
                phase,
                kernel_status: kernel_status.clone(),
                decision_outcome: apply_result.outcome,
                decision_status: apply_result.status.clone(),
                apply_result: Some(apply_result),
            });

            kernel_status
        })?;

        if let Some(error) = deferred_error {
            return Err(error);
        }

        Ok(HillslopeKernelExecutionReport {
            scheduler_report,
            phase_reports,
            writeback_surface,
        })
    }
}

impl Default for HillslopePhaseScheduler {
    fn default() -> Self {
        Self::canonical()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::collections::BTreeMap;

    use openwepp_kernel_contract::{
        BoundarySymbol, BoundaryValue, HillslopeAnnualDecompositionAction,
        HillslopeAnnualDecompositionControl, HillslopeAnnualGrowthAction,
        HillslopeAnnualGrowthControl, HillslopeConsumerAdapter,
        HillslopeDecompositionManagementClass, HillslopeDecompositionTransitionControl,
        HillslopeGrowthManagementClass, HillslopeGrowthTransitionControl, HillslopeKernel,
        HillslopeKernelPhaseClass, HillslopeKernelRequest, HillslopePerennialDecompositionAction,
        HillslopePerennialDecompositionControl, HillslopePerennialGrowthAction,
        HillslopePerennialGrowthControl, KernelRunResponse, KernelWritebackPayload,
        WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID, WritebackDecisionOutcome, WritebackField,
    };
    use openwepp_sim_contract::status::{BoundaryClass, SimulationPhase, StatusClassification};
    use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};

    use super::{
        HillslopePhase, HillslopePhaseGraph, HillslopePhaseScheduler, HillslopeWritebackSurface,
        SchedulerOutcomeClass, hillslope_consumer_adapter_for_phase,
        required_hillslope_consumer_state_symbols, validate_hillslope_consumer_boundary,
    };

    const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 2 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

    const INVALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 0 0 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

    fn valid_topology_report() -> openwepp_topology::TopologyValidationReport {
        let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
        validate_pre_execution_topology(&graph).expect("topology report should build")
    }

    #[allow(clippy::too_many_lines)]
    fn seeded_growth_runtime_surface_for_day_year(
        imngmt: f64,
        day_of_year: f64,
        runtime_year: f64,
    ) -> HillslopeWritebackSurface {
        let mut state_surface = BTreeMap::new();
        state_surface.insert(
            BoundarySymbol::from("pl_schedule_slot_count"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_schedule_rotation_years"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_schedule_rotation_repeats"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("day"),
            BoundaryValue::scalar(day_of_year),
        );
        state_surface.insert(
            BoundarySymbol::from("year"),
            BoundaryValue::scalar(runtime_year),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_schedule_slot_0001_rotation_index"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_schedule_slot_0001_ofe_index"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_schedule_slot_0001_year_in_rotation"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_schedule_slot_0001_crop_slots"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_schedule_slot_0001_crop_0001_imngmt"),
            BoundaryValue::scalar(imngmt),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_order_decomp_before_soil"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_order_growth_after_decomp"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_order_watbal_after_growth"),
            BoundaryValue::scalar(1.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0001_imngmt"),
            BoundaryValue::scalar(imngmt),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdharv"),
            BoundaryValue::scalar(240.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"),
            BoundaryValue::scalar(120.0),
        );
        state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0001_rw"),
            BoundaryValue::scalar(1.3),
        );
        state_surface.insert(BoundarySymbol::from("sumgdd"), BoundaryValue::scalar(640.0));
        state_surface.insert(BoundarySymbol::from("vdmt"), BoundaryValue::scalar(2.4));
        state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.65));
        state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(2.1));
        state_surface.insert(BoundarySymbol::from("rtmass"), BoundaryValue::scalar(1.0));
        state_surface.insert(BoundarySymbol::from("rtd"), BoundaryValue::scalar(0.35));
        state_surface.insert(BoundarySymbol::from("hia"), BoundaryValue::scalar(0.45));
        state_surface.insert(
            BoundarySymbol::from("iresd_seed"),
            BoundaryValue::scalar(3.0),
        );
        state_surface.insert(
            BoundarySymbol::from("sumrtm_seed"),
            BoundaryValue::scalar(2.5),
        );
        state_surface.insert(
            BoundarySymbol::from("sumsrm_seed"),
            BoundaryValue::scalar(1.5),
        );

        if (imngmt - 2.0).abs() < f64::EPSILON {
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_mgtopt"),
                BoundaryValue::scalar(2.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncut"),
                BoundaryValue::scalar(0.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncycle"),
                BoundaryValue::scalar(1.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_gday_0001"),
                BoundaryValue::scalar(150.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_gend_0001"),
                BoundaryValue::scalar(250.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_animal_0001"),
                BoundaryValue::scalar(20.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_bodywt_0001"),
                BoundaryValue::scalar(450.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_area_0001"),
                BoundaryValue::scalar(1200.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_digest_0001"),
                BoundaryValue::scalar(0.62),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdstop"),
                BoundaryValue::scalar(310.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_growth_slot_0001_crop_0001_mgtopt"),
                BoundaryValue::scalar(2.0),
            );
        } else {
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_resmgt"),
                BoundaryValue::scalar(1.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdherb"),
                BoundaryValue::scalar(200.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdburn"),
                BoundaryValue::scalar(0.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdslge"),
                BoundaryValue::scalar(0.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdcut"),
                BoundaryValue::scalar(0.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdmove"),
                BoundaryValue::scalar(0.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_fbrnag"),
                BoundaryValue::scalar(0.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_fbrnog"),
                BoundaryValue::scalar(0.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_frcut"),
                BoundaryValue::scalar(0.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_frmove"),
                BoundaryValue::scalar(0.0),
            );
        }

        HillslopeWritebackSurface {
            state_surface,
            flux_surface: BTreeMap::new(),
        }
    }

    fn seeded_growth_runtime_surface(imngmt: f64) -> HillslopeWritebackSurface {
        seeded_growth_runtime_surface_for_day_year(imngmt, 200.0, 1.0)
    }

    #[allow(clippy::too_many_lines)]
    fn seeded_multislot_rotation_surface(
        runtime_year: f64,
        day_of_year: f64,
    ) -> HillslopeWritebackSurface {
        let mut surface =
            seeded_growth_runtime_surface_for_day_year(1.0, day_of_year, runtime_year);
        let state = &mut surface.state_surface;

        state.insert(
            BoundarySymbol::from("pl_schedule_slot_count"),
            BoundaryValue::scalar(6.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_rotation_years"),
            BoundaryValue::scalar(3.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_rotation_repeats"),
            BoundaryValue::scalar(2.0),
        );

        // Slot 1 / year 1 / annual.
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0001_ofe_index"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0001_rotation_index"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0001_year_in_rotation"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0001_crop_slots"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0001_crop_0001_imngmt"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0001_imngmt"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdharv"),
            BoundaryValue::scalar(240.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"),
            BoundaryValue::scalar(120.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0001_rw"),
            BoundaryValue::scalar(1.1),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_resmgt"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdherb"),
            BoundaryValue::scalar(200.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdburn"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdslge"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdcut"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_jdmove"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_fbrnag"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_fbrnog"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_frcut"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_frmove"),
            BoundaryValue::scalar(0.0),
        );

        // Slot 2 / year 2 / annual-fallow.
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0002_ofe_index"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0002_rotation_index"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0002_year_in_rotation"),
            BoundaryValue::scalar(2.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0002_crop_slots"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0002_crop_0001_imngmt"),
            BoundaryValue::scalar(3.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0002_crop_0001_imngmt"),
            BoundaryValue::scalar(3.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0002_crop_0001_jdharv"),
            BoundaryValue::scalar(365.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0002_crop_0001_jdplt"),
            BoundaryValue::scalar(228.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0002_crop_0001_rw"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_resmgt"),
            BoundaryValue::scalar(6.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_jdherb"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_jdburn"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_jdslge"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_jdcut"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_jdmove"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_fbrnag"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_fbrnog"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_frcut"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0002_crop_0001_frmove"),
            BoundaryValue::scalar(0.0),
        );

        // Slot 3 / year 3 / perennial.
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0003_ofe_index"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0003_rotation_index"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0003_year_in_rotation"),
            BoundaryValue::scalar(3.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0003_crop_slots"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0003_crop_0001_imngmt"),
            BoundaryValue::scalar(2.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0003_crop_0001_imngmt"),
            BoundaryValue::scalar(2.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0003_crop_0001_jdharv"),
            BoundaryValue::scalar(288.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0003_crop_0001_jdplt"),
            BoundaryValue::scalar(130.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0003_crop_0001_jdstop"),
            BoundaryValue::scalar(310.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0003_crop_0001_rw"),
            BoundaryValue::scalar(0.762),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0003_crop_0001_mgtopt"),
            BoundaryValue::scalar(2.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_mgtopt"),
            BoundaryValue::scalar(2.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_ncut"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_ncycle"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_gday_0001"),
            BoundaryValue::scalar(150.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_gend_0001"),
            BoundaryValue::scalar(220.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_animal_0001"),
            BoundaryValue::scalar(20.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_bodywt_0001"),
            BoundaryValue::scalar(450.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_area_0001"),
            BoundaryValue::scalar(1200.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0003_crop_0001_digest_0001"),
            BoundaryValue::scalar(0.62),
        );

        // Slot 4 / year 1 / annual (rotation repeat 2).
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0004_ofe_index"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0004_rotation_index"),
            BoundaryValue::scalar(2.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0004_year_in_rotation"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0004_crop_slots"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0004_crop_0001_imngmt"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0004_crop_0001_imngmt"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0004_crop_0001_jdharv"),
            BoundaryValue::scalar(240.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0004_crop_0001_jdplt"),
            BoundaryValue::scalar(120.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0004_crop_0001_rw"),
            BoundaryValue::scalar(1.1),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_resmgt"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_jdherb"),
            BoundaryValue::scalar(200.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_jdburn"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_jdslge"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_jdcut"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_jdmove"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_fbrnag"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_fbrnog"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_frcut"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0004_crop_0001_frmove"),
            BoundaryValue::scalar(0.0),
        );

        // Slot 5 / year 2 / annual-fallow (rotation repeat 2).
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0005_ofe_index"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0005_rotation_index"),
            BoundaryValue::scalar(2.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0005_year_in_rotation"),
            BoundaryValue::scalar(2.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0005_crop_slots"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0005_crop_0001_imngmt"),
            BoundaryValue::scalar(3.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0005_crop_0001_imngmt"),
            BoundaryValue::scalar(3.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0005_crop_0001_jdharv"),
            BoundaryValue::scalar(365.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0005_crop_0001_jdplt"),
            BoundaryValue::scalar(228.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0005_crop_0001_rw"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_resmgt"),
            BoundaryValue::scalar(6.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_jdherb"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_jdburn"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_jdslge"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_jdcut"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_jdmove"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_fbrnag"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_fbrnog"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_frcut"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0005_crop_0001_frmove"),
            BoundaryValue::scalar(0.0),
        );

        // Slot 6 / year 3 / perennial (rotation repeat 2).
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0006_ofe_index"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0006_rotation_index"),
            BoundaryValue::scalar(2.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0006_year_in_rotation"),
            BoundaryValue::scalar(3.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0006_crop_slots"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_schedule_slot_0006_crop_0001_imngmt"),
            BoundaryValue::scalar(2.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0006_crop_0001_imngmt"),
            BoundaryValue::scalar(2.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0006_crop_0001_jdharv"),
            BoundaryValue::scalar(288.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0006_crop_0001_jdplt"),
            BoundaryValue::scalar(130.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0006_crop_0001_jdstop"),
            BoundaryValue::scalar(310.0),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0006_crop_0001_rw"),
            BoundaryValue::scalar(0.762),
        );
        state.insert(
            BoundarySymbol::from("pl_growth_slot_0006_crop_0001_mgtopt"),
            BoundaryValue::scalar(2.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_mgtopt"),
            BoundaryValue::scalar(2.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_ncut"),
            BoundaryValue::scalar(0.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_ncycle"),
            BoundaryValue::scalar(1.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_gday_0001"),
            BoundaryValue::scalar(150.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_gend_0001"),
            BoundaryValue::scalar(220.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_animal_0001"),
            BoundaryValue::scalar(20.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_bodywt_0001"),
            BoundaryValue::scalar(450.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_area_0001"),
            BoundaryValue::scalar(1200.0),
        );
        state.insert(
            BoundarySymbol::from("pl_decomp_slot_0006_crop_0001_digest_0001"),
            BoundaryValue::scalar(0.62),
        );

        surface
    }

    #[test]
    fn canonical_graph_order_is_deterministic() {
        let graph = HillslopePhaseGraph::canonical();
        let order = graph
            .topological_order()
            .expect("canonical graph should always topologically sort");

        assert_eq!(
            order,
            Vec::from(HillslopePhaseGraph::canonical_order()),
            "ARCH05 requires explicit deterministic scheduler order"
        );
        assert_eq!(graph.dependency_edges().len(), 12);
    }

    #[test]
    fn topology_precondition_failure_blocks_phase_execution() {
        let graph = parse_topology_fixture_str(INVALID_TOPOLOGY).expect("fixture should parse");
        let topology_report =
            validate_pre_execution_topology(&graph).expect("topology report should build");
        assert_eq!(
            topology_report.status.classification(),
            StatusClassification::Failure
        );

        let scheduler = HillslopePhaseScheduler::canonical();
        let call_count = Cell::new(0_usize);

        let report = scheduler
            .execute_with(&topology_report, |_| {
                call_count.set(call_count.get() + 1);
                HillslopePhaseScheduler::nominal_phase_status(HillslopePhase::Normalization)
                    .expect("nominal status should build")
            })
            .expect("scheduler should not error");

        assert_eq!(call_count.get(), 0);
        assert_eq!(
            report.outcome_class,
            SchedulerOutcomeClass::TopologyPreconditionFailed
        );
        assert_eq!(
            report.scheduler_status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            report.scheduler_status.boundary_class(),
            BoundaryClass::TopologyInvalid
        );
    }

    #[test]
    fn phase_failure_is_typed_and_fail_fast() {
        let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
        let topology_report =
            validate_pre_execution_topology(&graph).expect("topology report should build");
        let scheduler = HillslopePhaseScheduler::canonical();

        let report = scheduler
            .execute_with(&topology_report, |phase| {
                if phase == HillslopePhase::PercolationDeepSeepage {
                    return openwepp_sim_contract::status::SimulationStatus::failure(
                        SimulationPhase::HillslopeKernel,
                        true,
                        false,
                        BoundaryClass::DomainViolation,
                        "HSCHED-PHASE-E-004",
                    )
                    .expect("failure status should build");
                }

                HillslopePhaseScheduler::nominal_phase_status(phase)
                    .expect("nominal status should build")
            })
            .expect("scheduler should not error");

        assert_eq!(report.outcome_class, SchedulerOutcomeClass::PhaseFailure);
        assert_eq!(
            report.scheduler_status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            report.scheduler_status.boundary_class(),
            BoundaryClass::DomainViolation
        );
        assert_eq!(
            report.executed_phases(),
            vec![
                HillslopePhase::Normalization,
                HillslopePhase::StorageBounds,
                HillslopePhase::DecompositionTransition,
                HillslopePhase::ResiduePartitionTransition,
                HillslopePhase::AnnualGrowthTransition,
                HillslopePhase::PerennialGrowthTransition,
                HillslopePhase::Evapotranspiration,
                HillslopePhase::PercolationDeepSeepage,
            ]
        );
        assert_eq!(
            report.halted_phase,
            Some(HillslopePhase::PercolationDeepSeepage)
        );
    }

    #[test]
    fn phase_status_phase_mismatch_returns_mode_mismatch_failure() {
        let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
        let topology_report =
            validate_pre_execution_topology(&graph).expect("topology report should build");
        let scheduler = HillslopePhaseScheduler::canonical();

        let report = scheduler
            .execute_with(&topology_report, |_| {
                openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::PreExecutionValidation,
                    "HSCHED-PHASE-INVALID-STATUS",
                )
                .expect("status should build")
            })
            .expect("scheduler should not error");

        assert_eq!(
            report.outcome_class,
            SchedulerOutcomeClass::SchedulerInvariantFailure
        );
        assert_eq!(
            report.scheduler_status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            report.scheduler_status.boundary_class(),
            BoundaryClass::ModeMismatch
        );
        assert_eq!(report.halted_phase, Some(HillslopePhase::Normalization));
    }

    #[test]
    fn nominal_execution_completes_in_canonical_order() {
        let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
        let topology_report =
            validate_pre_execution_topology(&graph).expect("topology report should build");
        let scheduler = HillslopePhaseScheduler::canonical();

        let report = scheduler
            .execute_with(&topology_report, |phase| {
                HillslopePhaseScheduler::nominal_phase_status(phase)
                    .expect("nominal status should build")
            })
            .expect("scheduler should not error");

        assert!(report.is_success());
        assert_eq!(report.outcome_class, SchedulerOutcomeClass::Completed);
        assert_eq!(report.halted_phase, None);
        assert_eq!(
            report.executed_phases(),
            Vec::from(HillslopePhaseGraph::canonical_order())
        );
        assert_eq!(
            report.scheduler_status.phase(),
            SimulationPhase::HillslopeKernel
        );
        assert_eq!(
            report.scheduler_status.classification(),
            StatusClassification::Nominal
        );
    }

    #[test]
    fn consumer_adapter_mapping_matches_phase_contract() {
        assert_eq!(
            hillslope_consumer_adapter_for_phase(HillslopePhase::Normalization),
            HillslopeConsumerAdapter::Soil
        );
        assert_eq!(
            hillslope_consumer_adapter_for_phase(HillslopePhase::StorageBounds),
            HillslopeConsumerAdapter::Soil
        );
        assert_eq!(
            hillslope_consumer_adapter_for_phase(HillslopePhase::DecompositionTransition),
            HillslopeConsumerAdapter::Decomposition
        );
        assert_eq!(
            hillslope_consumer_adapter_for_phase(HillslopePhase::ResiduePartitionTransition),
            HillslopeConsumerAdapter::Decomposition
        );
        assert_eq!(
            hillslope_consumer_adapter_for_phase(HillslopePhase::AnnualGrowthTransition),
            HillslopeConsumerAdapter::Growth
        );
        assert_eq!(
            hillslope_consumer_adapter_for_phase(HillslopePhase::PerennialGrowthTransition),
            HillslopeConsumerAdapter::Growth
        );
        assert_eq!(
            hillslope_consumer_adapter_for_phase(HillslopePhase::Evapotranspiration),
            HillslopeConsumerAdapter::Watbal
        );
        assert_eq!(
            hillslope_consumer_adapter_for_phase(HillslopePhase::PercolationDeepSeepage),
            HillslopeConsumerAdapter::Perc
        );
        assert_eq!(
            hillslope_consumer_adapter_for_phase(HillslopePhase::LateralTransfer),
            HillslopeConsumerAdapter::Watbal
        );
        assert_eq!(
            hillslope_consumer_adapter_for_phase(HillslopePhase::Drainage),
            HillslopeConsumerAdapter::Perc
        );
        assert_eq!(
            hillslope_consumer_adapter_for_phase(HillslopePhase::RunoffReconciliation),
            HillslopeConsumerAdapter::Runoff
        );
        assert_eq!(
            hillslope_consumer_adapter_for_phase(HillslopePhase::StorageReconciliation),
            HillslopeConsumerAdapter::Watbal
        );
        assert_eq!(
            hillslope_consumer_adapter_for_phase(HillslopePhase::ClosureDiagnostics),
            HillslopeConsumerAdapter::Watbal
        );
    }

    #[test]
    fn required_consumer_symbols_are_empty_without_slope_or_soil_families() {
        let empty_surface = BTreeMap::new();

        for phase in HillslopePhaseGraph::canonical_order() {
            let required = required_hillslope_consumer_state_symbols(phase, &empty_surface);
            assert!(
                required.is_empty(),
                "phase {} should not require slope/soil symbols when neither family is seeded",
                phase.as_str()
            );
            validate_hillslope_consumer_boundary(phase, &empty_surface)
                .expect("empty non-slope/non-soil surface should not trigger consumer guard");
        }
    }

    #[test]
    fn consumer_boundary_reports_typed_missing_symbol_for_seeded_family() {
        let mut state_surface = BTreeMap::new();
        state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
        state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.25));
        state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(0.1));
        state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(0.31));
        state_surface.insert(
            BoundarySymbol::from("ssc"),
            BoundaryValue::scalar(0.000_004),
        );

        let error =
            validate_hillslope_consumer_boundary(HillslopePhase::Normalization, &state_surface)
                .expect_err("missing thetdr must fail with typed consumer boundary error");
        assert_eq!(error.code(), "HS-CONSUMER-E-001");
        assert!(matches!(
            error,
            super::HillslopeConsumerBoundaryError::MissingRequiredStateSymbol {
                phase: HillslopePhase::Normalization,
                adapter: HillslopeConsumerAdapter::Soil,
                symbol,
            } if symbol.as_str() == "thetdr"
        ));
    }

    #[test]
    fn annual_growth_phase_emits_typed_growth_context() {
        #[derive(Default)]
        struct ProbeKernel {
            decomp: usize,
            annual: usize,
            perennial: usize,
        }

        impl HillslopeKernel for ProbeKernel {
            fn run_hillslope_phase(
                &mut self,
                request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                match request.phase_class {
                    HillslopeKernelPhaseClass::DecompositionTransition
                    | HillslopeKernelPhaseClass::ResiduePartitionTransition => {
                        let context = request
                            .decomposition_context
                            .expect("decomposition phases should carry decomposition context");
                        assert_eq!(
                            context.management_class,
                            HillslopeDecompositionManagementClass::AnnualOrFallow
                        );
                        let transition_payload = context
                            .transition_payload
                            .expect("decomposition context should carry transition payload");
                        assert!(matches!(
                            transition_payload.control,
                            HillslopeDecompositionTransitionControl::Annual(
                                HillslopeAnnualDecompositionControl {
                                    active_action: HillslopeAnnualDecompositionAction::Herbicide,
                                    ..
                                }
                            )
                        ));
                        assert!(request.growth_context.is_none());
                        self.decomp += 1;
                    }
                    HillslopeKernelPhaseClass::GrowthAnnualTransition => {
                        let context = request
                            .growth_context
                            .expect("annual growth phase should carry growth context");
                        assert_eq!(
                            context.management_class,
                            HillslopeGrowthManagementClass::AnnualOrFallow
                        );
                        let transition_payload = context
                            .transition_payload
                            .expect("annual growth context should carry transition payload");
                        assert!(matches!(
                            transition_payload.control,
                            HillslopeGrowthTransitionControl::Annual(
                                HillslopeAnnualGrowthControl {
                                    active_action: HillslopeAnnualGrowthAction::None,
                                    ..
                                }
                            )
                        ));
                        self.annual += 1;
                    }
                    HillslopeKernelPhaseClass::GrowthPerennialTransition => {
                        assert!(
                            request.growth_context.is_none(),
                            "perennial phase should skip context when annual branch is active"
                        );
                        self.perennial += 1;
                    }
                    HillslopeKernelPhaseClass::Hydrology => {
                        assert!(request.growth_context.is_none());
                        assert!(request.decomposition_context.is_none());
                    }
                }

                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-GROWTH-CONTEXT",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = ProbeKernel::default();
        let surface = seeded_growth_runtime_surface(1.0);

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("annual growth context execution should succeed");

        assert!(report.scheduler_report.is_success());
        assert_eq!(kernel.decomp, 2);
        assert_eq!(kernel.annual, 1);
        assert_eq!(kernel.perennial, 1);
    }

    #[test]
    fn perennial_growth_phase_emits_typed_growth_context() {
        #[derive(Default)]
        struct ProbeKernel {
            decomp: usize,
            annual: usize,
            perennial: usize,
        }

        impl HillslopeKernel for ProbeKernel {
            fn run_hillslope_phase(
                &mut self,
                request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                match request.phase_class {
                    HillslopeKernelPhaseClass::DecompositionTransition
                    | HillslopeKernelPhaseClass::ResiduePartitionTransition => {
                        let context = request
                            .decomposition_context
                            .expect("decomposition phases should carry decomposition context");
                        assert_eq!(
                            context.management_class,
                            HillslopeDecompositionManagementClass::Perennial
                        );
                        let transition_payload = context
                            .transition_payload
                            .expect("decomposition context should carry transition payload");
                        assert!(matches!(
                            transition_payload.control,
                            HillslopeDecompositionTransitionControl::Perennial(
                                HillslopePerennialDecompositionControl {
                                    active_action: HillslopePerennialDecompositionAction::Grazing {
                                        cycle_index: 1
                                    },
                                    ..
                                }
                            )
                        ));
                        assert!(request.growth_context.is_none());
                        self.decomp += 1;
                    }
                    HillslopeKernelPhaseClass::GrowthAnnualTransition => {
                        assert!(
                            request.growth_context.is_none(),
                            "annual phase should skip context when perennial branch is active"
                        );
                        self.annual += 1;
                    }
                    HillslopeKernelPhaseClass::GrowthPerennialTransition => {
                        let context = request
                            .growth_context
                            .expect("perennial growth phase should carry growth context");
                        assert_eq!(
                            context.management_class,
                            HillslopeGrowthManagementClass::Perennial
                        );
                        let transition_payload = context
                            .transition_payload
                            .expect("perennial growth context should carry transition payload");
                        assert!(matches!(
                            transition_payload.control,
                            HillslopeGrowthTransitionControl::Perennial(
                                HillslopePerennialGrowthControl {
                                    active_action: HillslopePerennialGrowthAction::None,
                                    ..
                                }
                            )
                        ));
                        self.perennial += 1;
                    }
                    HillslopeKernelPhaseClass::Hydrology => {
                        assert!(request.growth_context.is_none());
                        assert!(request.decomposition_context.is_none());
                    }
                }

                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-GROWTH-CONTEXT",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = ProbeKernel::default();
        let surface = seeded_growth_runtime_surface(2.0);

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("perennial growth context execution should succeed");

        assert!(report.scheduler_report.is_success());
        assert_eq!(kernel.decomp, 2);
        assert_eq!(kernel.annual, 1);
        assert_eq!(kernel.perennial, 1);
    }

    #[test]
    fn active_slot_resolution_uses_year_three_perennial_slot() {
        #[derive(Default)]
        struct ProbeKernel {
            saw_decomp_perennial: bool,
            saw_annual_context: bool,
            saw_perennial_context: bool,
        }

        impl HillslopeKernel for ProbeKernel {
            fn run_hillslope_phase(
                &mut self,
                request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                match request.phase_class {
                    HillslopeKernelPhaseClass::DecompositionTransition
                    | HillslopeKernelPhaseClass::ResiduePartitionTransition => {
                        let context = request
                            .decomposition_context
                            .expect("decomposition phases should carry decomposition context");
                        self.saw_decomp_perennial = context.management_class
                            == HillslopeDecompositionManagementClass::Perennial;
                    }
                    HillslopeKernelPhaseClass::GrowthAnnualTransition => {
                        self.saw_annual_context = request.growth_context.is_some();
                    }
                    HillslopeKernelPhaseClass::GrowthPerennialTransition => {
                        self.saw_perennial_context = request.growth_context.is_some();
                    }
                    HillslopeKernelPhaseClass::Hydrology => {}
                }

                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-ACTIVE-SLOT",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = ProbeKernel::default();
        let surface = seeded_multislot_rotation_surface(3.0, 200.0);

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("year-three slot resolution should succeed");

        assert!(report.scheduler_report.is_success());
        assert!(kernel.saw_decomp_perennial);
        assert!(!kernel.saw_annual_context);
        assert!(kernel.saw_perennial_context);
    }

    #[test]
    fn active_slot_resolution_wraps_rotation_boundary_to_year_one() {
        #[derive(Default)]
        struct ProbeKernel {
            saw_decomp_annual: bool,
            saw_annual_context: bool,
            saw_perennial_context: bool,
        }

        impl HillslopeKernel for ProbeKernel {
            fn run_hillslope_phase(
                &mut self,
                request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                match request.phase_class {
                    HillslopeKernelPhaseClass::DecompositionTransition
                    | HillslopeKernelPhaseClass::ResiduePartitionTransition => {
                        let context = request
                            .decomposition_context
                            .expect("decomposition phases should carry decomposition context");
                        self.saw_decomp_annual = context.management_class
                            == HillslopeDecompositionManagementClass::AnnualOrFallow;
                    }
                    HillslopeKernelPhaseClass::GrowthAnnualTransition => {
                        self.saw_annual_context = request.growth_context.is_some();
                    }
                    HillslopeKernelPhaseClass::GrowthPerennialTransition => {
                        self.saw_perennial_context = request.growth_context.is_some();
                    }
                    HillslopeKernelPhaseClass::Hydrology => {}
                }

                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-ACTIVE-SLOT",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = ProbeKernel::default();
        let surface = seeded_multislot_rotation_surface(4.0, 200.0);

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("rotation-boundary slot resolution should succeed");

        assert!(report.scheduler_report.is_success());
        assert!(kernel.saw_decomp_annual);
        assert!(kernel.saw_annual_context);
        assert!(!kernel.saw_perennial_context);
    }

    #[test]
    fn active_slot_resolution_rejects_ambiguous_slot_candidates() {
        #[derive(Default)]
        struct NoopKernel;

        impl HillslopeKernel for NoopKernel {
            fn run_hillslope_phase(
                &mut self,
                _request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-NOOP",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = NoopKernel;
        let mut surface = seeded_multislot_rotation_surface(1.0, 200.0);
        surface.state_surface.insert(
            BoundarySymbol::from("pl_schedule_slot_0002_year_in_rotation"),
            BoundaryValue::scalar(1.0),
        );

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("ambiguous slot candidate must return typed report");

        assert_eq!(
            report.scheduler_report.halted_phase,
            Some(HillslopePhase::DecompositionTransition)
        );
        assert_eq!(report.phase_reports.len(), 3);
        assert_eq!(
            report.phase_reports[2].decision_status.message_id(),
            "HS-PLDISP-E-006"
        );
        assert_eq!(
            report.phase_reports[2].decision_status.boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn active_slot_resolution_rejects_missing_active_crop_for_day() {
        #[derive(Default)]
        struct NoopKernel;

        impl HillslopeKernel for NoopKernel {
            fn run_hillslope_phase(
                &mut self,
                _request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-NOOP",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = NoopKernel;
        let mut surface = seeded_growth_runtime_surface_for_day_year(1.0, 30.0, 1.0);
        surface.state_surface.insert(
            BoundarySymbol::from("pl_schedule_slot_0001_crop_slots"),
            BoundaryValue::scalar(2.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_schedule_slot_0001_crop_0002_imngmt"),
            BoundaryValue::scalar(3.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0002_imngmt"),
            BoundaryValue::scalar(3.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"),
            BoundaryValue::scalar(120.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdharv"),
            BoundaryValue::scalar(150.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0002_jdplt"),
            BoundaryValue::scalar(200.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0002_jdharv"),
            BoundaryValue::scalar(240.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0002_rw"),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0002_resmgt"),
            BoundaryValue::scalar(6.0),
        );

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("missing active crop must return typed report");

        assert_eq!(
            report.scheduler_report.halted_phase,
            Some(HillslopePhase::DecompositionTransition)
        );
        assert_eq!(report.phase_reports.len(), 3);
        assert_eq!(
            report.phase_reports[2].decision_status.message_id(),
            "HS-PLDISP-E-008"
        );
        assert_eq!(
            report.phase_reports[2].decision_status.boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn active_slot_resolution_rejects_ambiguous_active_crops_for_day() {
        #[derive(Default)]
        struct NoopKernel;

        impl HillslopeKernel for NoopKernel {
            fn run_hillslope_phase(
                &mut self,
                _request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-NOOP",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = NoopKernel;
        let mut surface = seeded_growth_runtime_surface_for_day_year(1.0, 210.0, 1.0);
        surface.state_surface.insert(
            BoundarySymbol::from("pl_schedule_slot_0001_crop_slots"),
            BoundaryValue::scalar(2.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_schedule_slot_0001_crop_0002_imngmt"),
            BoundaryValue::scalar(3.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0002_imngmt"),
            BoundaryValue::scalar(3.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"),
            BoundaryValue::scalar(180.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdharv"),
            BoundaryValue::scalar(300.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0002_jdplt"),
            BoundaryValue::scalar(200.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0002_jdharv"),
            BoundaryValue::scalar(240.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_growth_slot_0001_crop_0002_rw"),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0002_resmgt"),
            BoundaryValue::scalar(6.0),
        );

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("ambiguous active crop must return typed report");

        assert_eq!(
            report.scheduler_report.halted_phase,
            Some(HillslopePhase::DecompositionTransition)
        );
        assert_eq!(report.phase_reports.len(), 3);
        assert_eq!(
            report.phase_reports[2].decision_status.message_id(),
            "HS-PLDISP-E-009"
        );
        assert_eq!(
            report.phase_reports[2].decision_status.boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn decomposition_boundary_missing_required_symbol_returns_typed_failure() {
        #[derive(Default)]
        struct NoopKernel {
            invocation_count: usize,
        }

        impl HillslopeKernel for NoopKernel {
            fn run_hillslope_phase(
                &mut self,
                _request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                self.invocation_count += 1;
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-NOOP",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = NoopKernel::default();
        let mut surface = seeded_growth_runtime_surface(1.0);
        surface.state_surface.remove(&BoundarySymbol::from(
            "pl_decomp_slot_0001_crop_0001_resmgt",
        ));

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("typed decomposition guard failure should produce report");

        assert_eq!(
            report.scheduler_report.halted_phase,
            Some(HillslopePhase::DecompositionTransition)
        );
        assert_eq!(kernel.invocation_count, 2);
        assert_eq!(report.phase_reports.len(), 3);
        assert_eq!(
            report.phase_reports[2].decision_status.message_id(),
            "HS-DECOMP-E-001"
        );
        assert_eq!(
            report.phase_reports[2].decision_status.boundary_class(),
            BoundaryClass::MissingRequiredInput
        );
    }

    #[test]
    fn decomposition_boundary_invalid_ordering_flag_returns_typed_failure() {
        #[derive(Default)]
        struct NoopKernel {
            invocation_count: usize,
        }

        impl HillslopeKernel for NoopKernel {
            fn run_hillslope_phase(
                &mut self,
                _request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                self.invocation_count += 1;
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-NOOP",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = NoopKernel::default();
        let mut surface = seeded_growth_runtime_surface(1.0);
        surface.state_surface.insert(
            BoundarySymbol::from("pl_order_decomp_before_soil"),
            BoundaryValue::scalar(0.0),
        );

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("typed decomposition guard failure should produce report");

        assert_eq!(
            report.scheduler_report.halted_phase,
            Some(HillslopePhase::DecompositionTransition)
        );
        assert_eq!(kernel.invocation_count, 2);
        assert_eq!(report.phase_reports.len(), 3);
        assert_eq!(
            report.phase_reports[2].decision_status.message_id(),
            "HS-DECOMP-E-003"
        );
        assert_eq!(
            report.phase_reports[2].decision_status.boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn pl12_contract_conformance_rejects_missing_perennial_cutday_payload() {
        #[derive(Default)]
        struct NoopKernel;

        impl HillslopeKernel for NoopKernel {
            fn run_hillslope_phase(
                &mut self,
                _request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-NOOP",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = NoopKernel;
        let mut surface = seeded_growth_runtime_surface(2.0);
        surface.state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_mgtopt"),
            BoundaryValue::scalar(1.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncut"),
            BoundaryValue::scalar(2.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncycle"),
            BoundaryValue::scalar(0.0),
        );
        for symbol in [
            "pl_decomp_slot_0001_crop_0001_gday_0001",
            "pl_decomp_slot_0001_crop_0001_gend_0001",
            "pl_decomp_slot_0001_crop_0001_animal_0001",
            "pl_decomp_slot_0001_crop_0001_bodywt_0001",
            "pl_decomp_slot_0001_crop_0001_area_0001",
            "pl_decomp_slot_0001_crop_0001_digest_0001",
        ] {
            surface.state_surface.remove(&BoundarySymbol::from(symbol));
        }

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("missing perennial cutday payload should return typed report");

        assert_eq!(
            report.scheduler_report.halted_phase,
            Some(HillslopePhase::DecompositionTransition)
        );
        assert_eq!(report.phase_reports.len(), 3);
        assert_eq!(
            report.phase_reports[2].decision_status.message_id(),
            "HS-DECOMP-E-007"
        );
        assert_eq!(
            report.phase_reports[2].decision_status.boundary_class(),
            BoundaryClass::MissingRequiredInput
        );
    }

    #[test]
    fn pl12_contract_conformance_rejects_invalid_perennial_grazing_window() {
        #[derive(Default)]
        struct NoopKernel;

        impl HillslopeKernel for NoopKernel {
            fn run_hillslope_phase(
                &mut self,
                _request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-NOOP",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = NoopKernel;
        let mut surface = seeded_growth_runtime_surface(2.0);
        surface.state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_mgtopt"),
            BoundaryValue::scalar(2.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncut"),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncycle"),
            BoundaryValue::scalar(1.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_gday_0001"),
            BoundaryValue::scalar(220.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_gend_0001"),
            BoundaryValue::scalar(200.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_animal_0001"),
            BoundaryValue::scalar(20.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_bodywt_0001"),
            BoundaryValue::scalar(450.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_area_0001"),
            BoundaryValue::scalar(1200.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_digest_0001"),
            BoundaryValue::scalar(0.62),
        );

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("invalid perennial grazing window should return typed report");

        assert_eq!(
            report.scheduler_report.halted_phase,
            Some(HillslopePhase::DecompositionTransition)
        );
        assert_eq!(report.phase_reports.len(), 3);
        assert_eq!(
            report.phase_reports[2].decision_status.message_id(),
            "HS-DECOMP-E-009"
        );
        assert_eq!(
            report.phase_reports[2].decision_status.boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn pl13_contract_conformance_rejects_missing_growth_state_surface() {
        #[derive(Default)]
        struct NoopKernel;

        impl HillslopeKernel for NoopKernel {
            fn run_hillslope_phase(
                &mut self,
                _request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-NOOP",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = NoopKernel;
        let mut surface = seeded_growth_runtime_surface(1.0);
        surface
            .state_surface
            .remove(&BoundarySymbol::from("sumgdd"));

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("missing growth transition state should return typed report");

        assert_eq!(
            report.scheduler_report.halted_phase,
            Some(HillslopePhase::AnnualGrowthTransition)
        );
        assert_eq!(report.phase_reports.len(), 5);
        assert_eq!(
            report.phase_reports[4].decision_status.message_id(),
            "HS-GROWTH-E-001"
        );
        assert_eq!(
            report.phase_reports[4].decision_status.boundary_class(),
            BoundaryClass::MissingRequiredInput
        );
    }

    #[test]
    fn pl13_contract_conformance_rejects_growth_state_domain_violation() {
        #[derive(Default)]
        struct NoopKernel;

        impl HillslopeKernel for NoopKernel {
            fn run_hillslope_phase(
                &mut self,
                _request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-NOOP",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = NoopKernel;
        let mut surface = seeded_growth_runtime_surface(1.0);
        surface
            .state_surface
            .insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(1.1));

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("invalid growth transition state should return typed report");

        assert_eq!(
            report.scheduler_report.halted_phase,
            Some(HillslopePhase::AnnualGrowthTransition)
        );
        assert_eq!(report.phase_reports.len(), 5);
        assert_eq!(
            report.phase_reports[4].decision_status.message_id(),
            "HS-GROWTH-E-007"
        );
        assert_eq!(
            report.phase_reports[4].decision_status.boundary_class(),
            BoundaryClass::DomainViolation
        );
    }

    #[test]
    fn growth_boundary_missing_required_symbol_returns_typed_failure() {
        #[derive(Default)]
        struct NoopKernel {
            invocation_count: usize,
        }

        impl HillslopeKernel for NoopKernel {
            fn run_hillslope_phase(
                &mut self,
                _request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                self.invocation_count += 1;
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-NOOP",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = NoopKernel::default();
        let mut surface = seeded_growth_runtime_surface(1.0);
        surface
            .state_surface
            .remove(&BoundarySymbol::from("pl_growth_slot_0001_crop_0001_rw"));

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("typed growth guard failure should produce report");

        assert_eq!(
            report.scheduler_report.halted_phase,
            Some(HillslopePhase::AnnualGrowthTransition)
        );
        assert_eq!(kernel.invocation_count, 4);
        assert_eq!(report.phase_reports.len(), 5);
        assert_eq!(
            report.phase_reports[4].decision_status.message_id(),
            "HS-GROWTH-E-001"
        );
        assert_eq!(
            report.phase_reports[4].decision_status.boundary_class(),
            BoundaryClass::MissingRequiredInput
        );
    }

    #[test]
    fn growth_boundary_non_finite_ordering_flag_returns_typed_failure() {
        #[derive(Default)]
        struct NoopKernel {
            invocation_count: usize,
        }

        impl HillslopeKernel for NoopKernel {
            fn run_hillslope_phase(
                &mut self,
                _request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                self.invocation_count += 1;
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-NOOP",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let topology_report = valid_topology_report();
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = NoopKernel::default();
        let mut surface = seeded_growth_runtime_surface(1.0);
        surface.state_surface.insert(
            BoundarySymbol::from("pl_order_watbal_after_growth"),
            BoundaryValue::scalar(f64::NAN),
        );

        let report = scheduler
            .execute_with_kernel(&topology_report, &mut kernel, surface)
            .expect("typed growth guard failure should produce report");

        assert_eq!(
            report.scheduler_report.halted_phase,
            Some(HillslopePhase::AnnualGrowthTransition)
        );
        assert_eq!(kernel.invocation_count, 4);
        assert_eq!(report.phase_reports.len(), 5);
        assert_eq!(
            report.phase_reports[4].decision_status.message_id(),
            "HS-GROWTH-E-002"
        );
        assert_eq!(
            report.phase_reports[4].decision_status.boundary_class(),
            BoundaryClass::NonFinite
        );
    }

    #[test]
    fn execute_with_kernel_applies_writeback_updates() {
        #[derive(Default)]
        struct NominalKernel {
            call_index: u32,
        }

        impl HillslopeKernel for NominalKernel {
            fn run_hillslope_phase(
                &mut self,
                _request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                self.call_index += 1;
                let call_value = f64::from(self.call_index);
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    format!("HKERNEL-PHASE-OK-{}", self.call_index),
                )
                .expect("status should construct");
                let writeback = KernelWritebackPayload::with_updates(
                    vec![WritebackField::bounded(
                        "soil_storage",
                        call_value,
                        Some(0.0),
                        Some(1000.0),
                    )],
                    vec![WritebackField::bounded(
                        "runoff_total",
                        call_value * 0.25,
                        Some(0.0),
                        None,
                    )],
                );

                KernelRunResponse::new(status, writeback)
            }
        }

        let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
        let topology_report =
            validate_pre_execution_topology(&graph).expect("topology report should build");
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = NominalKernel::default();

        let report = scheduler
            .execute_with_kernel(
                &topology_report,
                &mut kernel,
                HillslopeWritebackSurface::default(),
            )
            .expect("kernel execution should succeed");

        assert!(report.scheduler_report.is_success());
        assert_eq!(
            report.scheduler_report.executed_phases(),
            Vec::from(HillslopePhaseGraph::canonical_order())
        );
        assert_eq!(
            report.phase_reports.len(),
            HillslopePhaseGraph::canonical_order().len()
        );
        assert!(report.phase_reports.iter().all(|phase| {
            phase.decision_outcome == WritebackDecisionOutcome::Apply
                && phase.apply_result.is_some()
        }));
        assert_eq!(
            report
                .writeback_surface
                .state_surface
                .get(&BoundarySymbol::from("soil_storage"))
                .copied(),
            Some(BoundaryValue::from(13.0))
        );
        assert_eq!(
            report
                .writeback_surface
                .flux_surface
                .get(&BoundarySymbol::from("runoff_total"))
                .copied(),
            Some(BoundaryValue::from(3.25))
        );
    }

    #[test]
    fn execute_with_kernel_lends_stable_surface_references() {
        #[derive(Default)]
        struct PointerProbeKernel {
            call_index: u32,
            state_surface_ptrs: Vec<usize>,
            flux_surface_ptrs: Vec<usize>,
        }

        impl HillslopeKernel for PointerProbeKernel {
            fn run_hillslope_phase(
                &mut self,
                request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                self.call_index += 1;
                self.state_surface_ptrs
                    .push(std::ptr::from_ref(request.state_surface) as usize);
                self.flux_surface_ptrs
                    .push(std::ptr::from_ref(request.flux_surface) as usize);
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    format!("HKERNEL-PHASE-POINTER-{}", self.call_index),
                )
                .expect("status should construct");

                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
        let topology_report =
            validate_pre_execution_topology(&graph).expect("topology report should build");
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = PointerProbeKernel::default();

        let report = scheduler
            .execute_with_kernel(
                &topology_report,
                &mut kernel,
                HillslopeWritebackSurface::default(),
            )
            .expect("kernel execution should succeed");

        assert!(report.scheduler_report.is_success());
        assert_eq!(kernel.state_surface_ptrs.len(), 13);
        assert_eq!(kernel.flux_surface_ptrs.len(), 13);
        assert!(
            kernel
                .state_surface_ptrs
                .windows(2)
                .all(|pair| pair[0] == pair[1]),
            "state surface reference should remain stable across phase calls"
        );
        assert!(
            kernel
                .flux_surface_ptrs
                .windows(2)
                .all(|pair| pair[0] == pair[1]),
            "flux surface reference should remain stable across phase calls"
        );
    }

    #[test]
    fn execute_with_kernel_rejects_non_finite_writeback() {
        struct RejectKernel;

        impl HillslopeKernel for RejectKernel {
            fn run_hillslope_phase(
                &mut self,
                _request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HKERNEL-PHASE-OK-REJECT",
                )
                .expect("status should construct");
                let writeback = KernelWritebackPayload::with_updates(
                    vec![WritebackField::unbounded("soil_storage", f64::NAN)],
                    Vec::new(),
                );
                KernelRunResponse::new(status, writeback)
            }
        }

        let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
        let topology_report =
            validate_pre_execution_topology(&graph).expect("topology report should build");
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = RejectKernel;

        let report = scheduler
            .execute_with_kernel(
                &topology_report,
                &mut kernel,
                HillslopeWritebackSurface::default(),
            )
            .expect("execution should return typed report");

        assert_eq!(
            report.scheduler_report.outcome_class,
            SchedulerOutcomeClass::PhaseFailure
        );
        assert_eq!(report.phase_reports.len(), 1);
        assert_eq!(
            report.phase_reports[0].decision_outcome,
            WritebackDecisionOutcome::Reject
        );
        assert_eq!(
            report.phase_reports[0].decision_status.message_id(),
            WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID
        );
        assert!(
            !report
                .writeback_surface
                .state_surface
                .contains_key(&BoundarySymbol::from("soil_storage")),
            "rejected payload must not mutate orchestrator writeback state"
        );
    }

    #[test]
    fn execute_with_kernel_rejects_kernel_phase_mismatch() {
        struct PhaseMismatchKernel;

        impl HillslopeKernel for PhaseMismatchKernel {
            fn run_hillslope_phase(
                &mut self,
                _request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::PreExecutionValidation,
                    "HKERNEL-PHASE-INVALID",
                )
                .expect("status should construct");
                KernelRunResponse::new(status, KernelWritebackPayload::empty())
            }
        }

        let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
        let topology_report =
            validate_pre_execution_topology(&graph).expect("topology report should build");
        let scheduler = HillslopePhaseScheduler::canonical();
        let mut kernel = PhaseMismatchKernel;

        let report = scheduler
            .execute_with_kernel(
                &topology_report,
                &mut kernel,
                HillslopeWritebackSurface::default(),
            )
            .expect("execution should return typed report");

        assert_eq!(
            report.scheduler_report.outcome_class,
            SchedulerOutcomeClass::PhaseFailure
        );
        assert_eq!(
            report.scheduler_report.scheduler_status.boundary_class(),
            BoundaryClass::ModeMismatch
        );
        assert_eq!(report.phase_reports.len(), 1);
        assert_eq!(
            report.phase_reports[0].decision_outcome,
            WritebackDecisionOutcome::Reject
        );
    }
}
