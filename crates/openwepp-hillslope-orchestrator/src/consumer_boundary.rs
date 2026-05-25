#[allow(clippy::wildcard_imports)]
use super::*;
#[allow(clippy::wildcard_imports)]
use crate::constants::*;

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
pub(crate) struct ActivePlSlotSelection {
    pub(crate) slot_index: usize,
    pub(crate) crop_slot_index: usize,
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
