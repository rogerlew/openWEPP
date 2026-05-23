#![allow(clippy::missing_errors_doc)]

pub mod runtime_inputs;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeDecompositionKernelContext,
    HillslopeDecompositionManagementClass, HillslopeGrowthKernelContext,
    HillslopeGrowthManagementClass, HillslopeKernel, HillslopeKernelPhaseClass,
    HillslopeKernelRequest, KernelWritebackApplyResult, WritebackDecisionOutcome, WritebackError,
    apply_kernel_writeback, evaluate_kernel_writeback,
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
const PL_DECOMP_IMNGMT_SYMBOL: &str = "pl_growth_slot_0001_crop_0001_imngmt";
const PL_DECOMP_RESMGT_SYMBOL: &str = "pl_decomp_slot_0001_crop_0001_resmgt";
const PL_DECOMP_MGTOPT_SYMBOL: &str = "pl_decomp_slot_0001_crop_0001_mgtopt";
const PL_DECOMP_NCUT_SYMBOL: &str = "pl_decomp_slot_0001_crop_0001_ncut";
const PL_DECOMP_NCYCLE_SYMBOL: &str = "pl_decomp_slot_0001_crop_0001_ncycle";
const PL_DECOMP_IRESD_SEED_SYMBOL: &str = "iresd_seed";
const PL_DECOMP_SUMRTM_SEED_SYMBOL: &str = "sumrtm_seed";
const PL_DECOMP_SUMSRM_SEED_SYMBOL: &str = "sumsrm_seed";
const PL_ORDER_DECOMP_BEFORE_SOIL_SYMBOL: &str = "pl_order_decomp_before_soil";
const PL_GROWTH_IMNGMT_SYMBOL: &str = "pl_growth_slot_0001_crop_0001_imngmt";
const PL_GROWTH_JDHARV_SYMBOL: &str = "pl_growth_slot_0001_crop_0001_jdharv";
const PL_GROWTH_JDPLT_SYMBOL: &str = "pl_growth_slot_0001_crop_0001_jdplt";
const PL_GROWTH_RW_SYMBOL: &str = "pl_growth_slot_0001_crop_0001_rw";
const PL_GROWTH_JDSTOP_SYMBOL: &str = "pl_growth_slot_0001_crop_0001_jdstop";
const PL_GROWTH_MGTOPT_SYMBOL: &str = "pl_growth_slot_0001_crop_0001_mgtopt";
const PL_ORDER_GROWTH_AFTER_DECOMP_SYMBOL: &str = "pl_order_growth_after_decomp";
const PL_ORDER_WATBAL_AFTER_GROWTH_SYMBOL: &str = "pl_order_watbal_after_growth";
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
}

impl HillslopeGrowthBoundaryError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingRequiredStateSymbol { .. } => "HS-GROWTH-E-001",
            Self::NonFiniteRequiredStateSymbol { .. } => "HS-GROWTH-E-002",
            Self::InvalidOrderingFlagValue { .. } => "HS-GROWTH-E-003",
            Self::UnsupportedManagementClass { .. } => "HS-GROWTH-E-004",
        }
    }

    #[must_use]
    pub const fn boundary_class(&self) -> BoundaryClass {
        match self {
            Self::MissingRequiredStateSymbol { .. } => BoundaryClass::MissingRequiredInput,
            Self::NonFiniteRequiredStateSymbol { .. } => BoundaryClass::NonFinite,
            Self::InvalidOrderingFlagValue { .. } | Self::UnsupportedManagementClass { .. } => {
                BoundaryClass::DomainViolation
            }
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
}

impl HillslopeDecompositionBoundaryError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingRequiredStateSymbol { .. } => "HS-DECOMP-E-001",
            Self::NonFiniteRequiredStateSymbol { .. } => "HS-DECOMP-E-002",
            Self::InvalidOrderingFlagValue { .. } => "HS-DECOMP-E-003",
            Self::UnsupportedManagementClass { .. } => "HS-DECOMP-E-004",
        }
    }

    #[must_use]
    pub const fn boundary_class(&self) -> BoundaryClass {
        match self {
            Self::MissingRequiredStateSymbol { .. } => BoundaryClass::MissingRequiredInput,
            Self::NonFiniteRequiredStateSymbol { .. } => BoundaryClass::NonFinite,
            Self::InvalidOrderingFlagValue { .. } | Self::UnsupportedManagementClass { .. } => {
                BoundaryClass::DomainViolation
            }
        }
    }
}

impl fmt::Display for HillslopeDecompositionBoundaryError {
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

fn decomposition_phase_dispatch_for_state(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<DecompositionPhaseDispatch, HillslopeDecompositionBoundaryError> {
    if !state_surface.contains_key(&BoundarySymbol::from(PL_DECOMP_RUNTIME_SENTINEL)) {
        return Ok(DecompositionPhaseDispatch::Skip);
    }

    let imngmt = require_finite_state_value_for_decomposition(
        phase,
        state_surface,
        PL_DECOMP_IMNGMT_SYMBOL,
    )?;
    let management_class = normalize_management_class_for_decomposition(phase, imngmt)?;
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

    for symbol in [
        PL_DECOMP_IRESD_SEED_SYMBOL,
        PL_DECOMP_SUMRTM_SEED_SYMBOL,
        PL_DECOMP_SUMSRM_SEED_SYMBOL,
    ] {
        let _ = require_finite_state_value_for_decomposition(phase, state_surface, symbol)?;
    }

    match management_class {
        1 | 3 => {
            let _ = require_finite_state_value_for_decomposition(
                phase,
                state_surface,
                PL_DECOMP_RESMGT_SYMBOL,
            )?;
        }
        2 => {
            for symbol in [
                PL_DECOMP_MGTOPT_SYMBOL,
                PL_DECOMP_NCUT_SYMBOL,
                PL_DECOMP_NCYCLE_SYMBOL,
            ] {
                let _ = require_finite_state_value_for_decomposition(phase, state_surface, symbol)?;
            }
        }
        _ => {
            return Err(
                HillslopeDecompositionBoundaryError::UnsupportedManagementClass {
                    phase,
                    symbol: BoundarySymbol::from(PL_DECOMP_IMNGMT_SYMBOL),
                    value: imngmt,
                },
            );
        }
    }

    let management_class = if management_class == 2 {
        HillslopeDecompositionManagementClass::Perennial
    } else {
        HillslopeDecompositionManagementClass::AnnualOrFallow
    };

    Ok(DecompositionPhaseDispatch::Execute(
        HillslopeDecompositionKernelContext::new(
            management_class,
            order_decomp_before_soil,
            order_growth_after_decomp,
        ),
    ))
}

fn growth_phase_dispatch_for_state(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<GrowthPhaseDispatch, HillslopeGrowthBoundaryError> {
    if !state_surface.contains_key(&BoundarySymbol::from(PL_GROWTH_RUNTIME_SENTINEL)) {
        return Ok(GrowthPhaseDispatch::Skip);
    }

    let imngmt = require_finite_state_value(phase, state_surface, PL_GROWTH_IMNGMT_SYMBOL)?;
    let management_class = normalize_management_class(phase, imngmt)?;
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

    match phase {
        HillslopePhase::AnnualGrowthTransition => {
            if management_class == 2 {
                return Ok(GrowthPhaseDispatch::Skip);
            }
            debug_assert!(management_class == 1 || management_class == 3);

            for symbol in [
                PL_GROWTH_JDHARV_SYMBOL,
                PL_GROWTH_JDPLT_SYMBOL,
                PL_GROWTH_RW_SYMBOL,
                PL_DECOMP_RESMGT_SYMBOL,
            ] {
                let _ = require_finite_state_value(phase, state_surface, symbol)?;
            }

            Ok(GrowthPhaseDispatch::Execute(
                HillslopeGrowthKernelContext::new(
                    HillslopeGrowthManagementClass::AnnualOrFallow,
                    order_growth_after_decomp,
                    order_watbal_after_growth,
                ),
            ))
        }
        HillslopePhase::PerennialGrowthTransition => {
            if management_class == 1 || management_class == 3 {
                return Ok(GrowthPhaseDispatch::Skip);
            }
            debug_assert!(management_class == 2);

            for symbol in [
                PL_GROWTH_JDHARV_SYMBOL,
                PL_GROWTH_JDPLT_SYMBOL,
                PL_GROWTH_RW_SYMBOL,
                PL_GROWTH_JDSTOP_SYMBOL,
                PL_GROWTH_MGTOPT_SYMBOL,
            ] {
                let _ = require_finite_state_value(phase, state_surface, symbol)?;
            }

            Ok(GrowthPhaseDispatch::Execute(
                HillslopeGrowthKernelContext::new(
                    HillslopeGrowthManagementClass::Perennial,
                    order_growth_after_decomp,
                    order_watbal_after_growth,
                ),
            ))
        }
        _ => Ok(GrowthPhaseDispatch::Skip),
    }
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
) -> Result<u8, HillslopeGrowthBoundaryError> {
    let rounded = value.round();
    if (value - rounded).abs() > MANAGEMENT_CLASS_EPSILON {
        return Err(HillslopeGrowthBoundaryError::UnsupportedManagementClass {
            phase,
            symbol: BoundarySymbol::from(PL_GROWTH_IMNGMT_SYMBOL),
            value,
        });
    }
    if !(1.0..=3.0).contains(&rounded) {
        return Err(HillslopeGrowthBoundaryError::UnsupportedManagementClass {
            phase,
            symbol: BoundarySymbol::from(PL_GROWTH_IMNGMT_SYMBOL),
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
        symbol: BoundarySymbol::from(PL_GROWTH_IMNGMT_SYMBOL),
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
) -> Result<u8, HillslopeDecompositionBoundaryError> {
    let rounded = value.round();
    if (value - rounded).abs() > MANAGEMENT_CLASS_EPSILON {
        return Err(
            HillslopeDecompositionBoundaryError::UnsupportedManagementClass {
                phase,
                symbol: BoundarySymbol::from(PL_DECOMP_IMNGMT_SYMBOL),
                value,
            },
        );
    }
    if !(1.0..=3.0).contains(&rounded) {
        return Err(
            HillslopeDecompositionBoundaryError::UnsupportedManagementClass {
                phase,
                symbol: BoundarySymbol::from(PL_DECOMP_IMNGMT_SYMBOL),
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
            symbol: BoundarySymbol::from(PL_DECOMP_IMNGMT_SYMBOL),
            value,
        },
    )
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
        BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter,
        HillslopeDecompositionManagementClass, HillslopeGrowthManagementClass, HillslopeKernel,
        HillslopeKernelPhaseClass, HillslopeKernelRequest, KernelRunResponse,
        KernelWritebackPayload, WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID, WritebackDecisionOutcome,
        WritebackField,
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

    fn seeded_growth_runtime_surface(imngmt: f64) -> HillslopeWritebackSurface {
        let mut state_surface = BTreeMap::new();
        state_surface.insert(
            BoundarySymbol::from("pl_schedule_slot_count"),
            BoundaryValue::scalar(1.0),
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
                BoundaryValue::scalar(1.0),
            );
            state_surface.insert(
                BoundarySymbol::from("pl_decomp_slot_0001_crop_0001_ncycle"),
                BoundaryValue::scalar(1.0),
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
        }

        HillslopeWritebackSurface {
            state_surface,
            flux_surface: BTreeMap::new(),
        }
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
            .remove(&BoundarySymbol::from("pl_growth_slot_0001_crop_0001_jdplt"));

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
