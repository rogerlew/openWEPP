#[allow(clippy::wildcard_imports)]
use super::*;
#[allow(clippy::wildcard_imports)]
use crate::constants::*;
use crate::consumer_boundary::ActivePlSlotSelection;

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
}

impl Wb11HydrologyKernelGuardError {
    #[must_use]
    pub const fn boundary_class(&self) -> BoundaryClass {
        match self {
            Self::MissingRequiredStateSymbol { .. }
            | Self::MissingRequiredFluxSymbol { .. }
            | Self::Erod13MissingRequiredSymbol { .. }
            | Self::Erod14MissingRequiredSymbol { .. } => BoundaryClass::MissingRequiredInput,
            Self::NonFiniteStateSymbol { .. }
            | Self::NonFiniteFluxSymbol { .. }
            | Self::Erod13NonFiniteSymbol { .. }
            | Self::Erod14NonFiniteSymbol { .. } => BoundaryClass::NonFinite,
            Self::StateSymbolOutOfRange { .. }
            | Self::FluxSymbolOutOfRange { .. }
            | Self::Erod13DomainViolation { .. }
            | Self::Erod14DomainViolation { .. } => BoundaryClass::DomainViolation,
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
            | Self::Erod14DomainViolation { .. } => unreachable!(),
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
        }
    }
}

impl Error for Wb11HydrologyKernelGuardError {}

/// WB11 hydrology production kernel for ET/perc/lateral/drain lanes.
#[derive(Debug, Clone, Default)]
pub struct Wb11HydrologyKernel;

#[derive(Debug, Clone, Copy)]
struct SnowCouplingOutcome {
    signed_s: f64,
    accumulation: f64,
    runtime_swe: f64,
}

#[derive(Debug, Clone, Copy)]
struct FrostCouplingOutcome {
    dfrost: f64,
    dthaw: f64,
    nft: f64,
    ws_frz: f64,
    infcap_frz: f64,
}

#[derive(Debug, Clone, Copy)]
enum IrrigationScheduleSource {
    Depletion,
    FixedDate,
}

impl IrrigationScheduleSource {
    const fn as_scalar(self) -> f64 {
        match self {
            Self::Depletion => 1.0,
            Self::FixedDate => 2.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ActiveIrrigationEvent {
    source: IrrigationScheduleSource,
    event_index: usize,
    system_type: f64,
    depth_m: f64,
    duration_s: f64,
    rate_m_per_s: f64,
}

impl Wb11HydrologyKernel {
    fn require_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionStateSymbol,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(symbol);
        let Some(value) = request.state_surface.get(&key) else {
            return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                phase_class,
                symbol: key,
            });
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: key,
                value: scalar,
            });
        }
        Ok(scalar)
    }

    fn require_flux_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionFluxSymbol,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(symbol);
        let Some(value) = request.flux_surface.get(&key) else {
            return Err(Wb11HydrologyKernelGuardError::MissingRequiredFluxSymbol {
                phase_class,
                symbol: key,
            });
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteFluxSymbol {
                phase_class,
                symbol: BoundarySymbol::from(symbol),
                value: scalar,
            });
        }
        Ok(scalar)
    }

    fn optional_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionStateSymbol,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(symbol);
        let Some(value) = request.state_surface.get(&key) else {
            return Ok(None);
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: key,
                value: scalar,
            });
        }
        Ok(Some(scalar))
    }

    fn optional_flux_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionFluxSymbol,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(symbol);
        let Some(value) = request.flux_surface.get(&key) else {
            return Ok(None);
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteFluxSymbol {
                phase_class,
                symbol: key,
                value: scalar,
            });
        }
        Ok(Some(scalar))
    }

    fn optional_state_scalar_for_symbol(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.state_surface.get(symbol) else {
            return Ok(None);
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(Some(scalar))
    }

    fn require_state_scalar_for_symbol(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.state_surface.get(symbol) else {
            return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                phase_class,
                symbol: symbol.clone(),
            });
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(scalar)
    }

    fn resolve_wb20_forward_solver_lane_enabled(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let symbol = BoundarySymbol::from(WB20_SYMBOL_FORWARD_SOLVER_LANE_ENABLED);
        let Some(value) = Self::optional_state_scalar_for_symbol(request, phase_class, &symbol)?
        else {
            return Ok(false);
        };
        if value.abs() <= WB11_ZERO_THRESHOLD {
            return Ok(false);
        }
        if (value - 1.0).abs() <= WB11_ZERO_THRESHOLD {
            return Ok(true);
        }
        Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
            phase_class,
            symbol,
            value,
            minimum: Some(0.0),
            maximum: Some(1.0),
        })
    }

    fn require_state_non_negative_integral_for_symbol(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
    ) -> Result<usize, Wb11HydrologyKernelGuardError> {
        let scalar = Self::require_state_scalar_for_symbol(request, phase_class, symbol)?;
        if scalar < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let rounded = scalar.round();
        if (scalar - rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let rounded_text = format!("{rounded:.0}");
        let Ok(parsed_count) = rounded_text.parse::<usize>() else {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
                minimum: Some(0.0),
                maximum: Some(Self::diagnostic_count_to_f64(usize::MAX)),
            });
        };
        Ok(parsed_count)
    }

    fn require_state_range(
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionStateSymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum {
            if value < minimum_value - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        if let Some(maximum_value) = maximum {
            if value > maximum_value + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(())
    }

    fn require_state_range_for_symbol(
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum {
            if value < minimum_value - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        if let Some(maximum_value) = maximum {
            if value > maximum_value + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(())
    }

    fn require_flux_range(
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionFluxSymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum {
            if value < minimum_value - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        if let Some(maximum_value) = maximum {
            if value > maximum_value + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(())
    }

    fn require_flux_range_for_symbol(
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum {
            if value < minimum_value - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                    phase_class,
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        if let Some(maximum_value) = maximum {
            if value > maximum_value + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                    phase_class,
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(())
    }

    fn optional_erod13_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        symbol: &BoundarySymbol,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.state_surface.get(symbol) else {
            return Ok(None);
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod13NonFiniteSymbol {
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(Some(scalar))
    }

    fn require_erod13_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        symbol: &BoundarySymbol,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.state_surface.get(symbol) else {
            return Err(Wb11HydrologyKernelGuardError::Erod13MissingRequiredSymbol {
                symbol: symbol.clone(),
            });
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod13NonFiniteSymbol {
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(scalar)
    }

    fn require_erod13_domain(
        symbol: &BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum {
            if value < minimum_value - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        if let Some(maximum_value) = maximum {
            if value > maximum_value + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(())
    }

    fn resolve_erod13_core_enabled(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let enabled_symbol = BoundarySymbol::from(EROD13_SYMBOL_CORE_ENABLED);
        let Some(value) = Self::optional_erod13_state_scalar(request, &enabled_symbol)? else {
            return Ok(false);
        };
        if value.abs() <= WB11_ZERO_THRESHOLD {
            return Ok(false);
        }
        if (value - 1.0).abs() <= WB11_ZERO_THRESHOLD {
            return Ok(true);
        }
        Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
            symbol: enabled_symbol,
            value,
            minimum: Some(0.0),
            maximum: Some(1.0),
        })
    }

    fn optional_erod14_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        symbol: &BoundarySymbol,
    ) -> Result<Option<f64>, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.state_surface.get(symbol) else {
            return Ok(None);
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod14NonFiniteSymbol {
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(Some(scalar))
    }

    fn require_erod14_state_scalar(
        request: &HillslopeKernelRequest<'_>,
        symbol: &BoundarySymbol,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let Some(value) = request.state_surface.get(symbol) else {
            return Err(Wb11HydrologyKernelGuardError::Erod14MissingRequiredSymbol {
                symbol: symbol.clone(),
            });
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod14NonFiniteSymbol {
                symbol: symbol.clone(),
                value: scalar,
            });
        }
        Ok(scalar)
    }

    fn require_erod14_domain(
        symbol: &BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if let Some(minimum_value) = minimum {
            if value < minimum_value - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        if let Some(maximum_value) = maximum {
            if value > maximum_value + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: symbol.clone(),
                    value,
                    minimum,
                    maximum,
                });
            }
        }
        Ok(())
    }

    fn resolve_erod14_wave2_enabled(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let enabled_symbol = BoundarySymbol::from(EROD14_SYMBOL_WAVE2_ENABLED);
        let Some(value) = Self::optional_erod14_state_scalar(request, &enabled_symbol)? else {
            return Ok(false);
        };
        if value.abs() <= WB11_ZERO_THRESHOLD {
            return Ok(false);
        }
        if (value - 1.0).abs() <= WB11_ZERO_THRESHOLD {
            return Ok(true);
        }
        Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
            symbol: enabled_symbol,
            value,
            minimum: Some(0.0),
            maximum: Some(1.0),
        })
    }

    fn erod14_class_symbol(root: &str, class_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("{root}_{class_index:04}"))
    }

    fn extract_state_update_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
        let target = BoundarySymbol::from(symbol);
        fields.iter().find_map(|field| {
            if field.symbol == target {
                Some(field.value.as_f64())
            } else {
                None
            }
        })
    }

    fn wb18_perc_state_symbol(field: &str, layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb18_perc_{field}_{layer_index:04}"))
    }

    fn wb18_perc_flux_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("wb18_perc_pei_{layer_index:04}"))
    }

    fn wb19_dg_symbol(layer_index: usize) -> BoundarySymbol {
        BoundarySymbol::from(format!("dg_{layer_index:04}"))
    }

    #[allow(clippy::type_complexity)]
    fn wb19_load_layer_state(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), Wb11HydrologyKernelGuardError> {
        let nsl_symbol = BoundarySymbol::from("nsl");
        let layer_count = Self::require_state_non_negative_integral_for_symbol(
            request,
            phase_class,
            &nsl_symbol,
        )?;
        if layer_count == 0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: nsl_symbol,
                value: 0.0,
                minimum: Some(1.0),
                maximum: None,
            });
        }

        let mut theta = Vec::with_capacity(layer_count);
        let mut field_capacity = Vec::with_capacity(layer_count);
        let mut conductivity = Vec::with_capacity(layer_count);
        let mut thickness = Vec::with_capacity(layer_count);

        for layer_index in 1..=layer_count {
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
            let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index);
            let ssc_symbol = Self::wb18_perc_state_symbol("ssc", layer_index);
            let dg_symbol = Self::wb19_dg_symbol(layer_index);

            let layer_theta =
                Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &theta_symbol,
                layer_theta,
                Some(0.0),
                None,
            )?;

            let layer_fc = Self::require_state_scalar_for_symbol(request, phase_class, &fc_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &fc_symbol,
                layer_fc,
                Some(0.0),
                None,
            )?;

            let layer_ssc =
                Self::require_state_scalar_for_symbol(request, phase_class, &ssc_symbol)?;
            if layer_ssc <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: ssc_symbol,
                    value: layer_ssc,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }

            let layer_dg = Self::require_state_scalar_for_symbol(request, phase_class, &dg_symbol)?;
            if layer_dg <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: dg_symbol,
                    value: layer_dg,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }

            theta.push(layer_theta);
            field_capacity.push(layer_fc);
            conductivity.push(layer_ssc);
            thickness.push(layer_dg);
        }

        Ok((theta, field_capacity, conductivity, thickness))
    }

    fn wb19_drainable_storage(theta: &[f64], field_capacity: &[f64]) -> f64 {
        theta
            .iter()
            .zip(field_capacity.iter())
            .map(|(theta_i, fc_i)| (theta_i - fc_i).max(0.0))
            .sum()
    }

    fn wb19_withdraw_top_down(theta: &mut [f64], field_capacity: &[f64], amount: f64) -> f64 {
        let mut remaining = amount.max(0.0);
        for (theta_i, fc_i) in theta.iter_mut().zip(field_capacity.iter()) {
            if remaining <= WB11_ZERO_THRESHOLD {
                break;
            }
            let available = (*theta_i - *fc_i).max(0.0);
            if available <= WB11_ZERO_THRESHOLD {
                continue;
            }
            let withdrawn = available.min(remaining);
            *theta_i -= withdrawn;
            remaining -= withdrawn;
        }
        amount.max(0.0) - remaining.max(0.0)
    }

    fn wb19_withdraw_tile_to_surface(
        theta: &mut [f64],
        field_capacity: &[f64],
        tile_layer_index: usize,
        amount: f64,
    ) -> f64 {
        let mut remaining = amount.max(0.0);
        if theta.is_empty() {
            return 0.0;
        }
        let upper_layer = tile_layer_index.min(theta.len() - 1);
        for layer in (0..=upper_layer).rev() {
            if remaining <= WB11_ZERO_THRESHOLD {
                break;
            }
            let available = (theta[layer] - field_capacity[layer]).max(0.0);
            if available > WB11_ZERO_THRESHOLD {
                let withdrawn = available.min(remaining);
                theta[layer] -= withdrawn;
                remaining -= withdrawn;
            }
        }
        amount.max(0.0) - remaining.max(0.0)
    }

    fn diagnostic_count_to_f64(value: usize) -> f64 {
        value.to_string().parse::<f64>().unwrap_or(f64::INFINITY)
    }

    fn diagnostic_i64_to_f64(value: i64) -> f64 {
        value.to_string().parse::<f64>().unwrap_or_else(|_| {
            if value.is_negative() {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            }
        })
    }

    fn optional_state_non_negative_integral(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: HillslopeProductionStateSymbol,
    ) -> Result<Option<usize>, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(symbol);
        let Some(value) = request.state_surface.get(&key) else {
            return Ok(None);
        };
        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: key,
                value: scalar,
            });
        }
        if scalar < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: key,
                value: scalar,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let rounded = scalar.round();
        if (scalar - rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: key,
                value: scalar,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let rounded_text = format!("{rounded:.0}");
        let Ok(parsed_count) = rounded_text.parse::<usize>() else {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: key,
                value: scalar,
                minimum: Some(0.0),
                maximum: Some(Self::diagnostic_count_to_f64(usize::MAX)),
            });
        };

        Ok(Some(parsed_count))
    }

    fn resolve_hyetograph_point_count(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<usize, Wb11HydrologyKernelGuardError> {
        let ninten = Self::optional_state_non_negative_integral(
            request,
            phase_class,
            WB14_SYMBOL_HYETOGRAPH_NINTEN,
        )?;
        let nbrkpt = Self::optional_state_non_negative_integral(
            request,
            phase_class,
            WB14_SYMBOL_HYETOGRAPH_NBRKPT,
        )?;

        let point_count = match (ninten, nbrkpt) {
            (None, None) => {
                return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_HYETOGRAPH_NINTEN),
                });
            }
            (Some(ninten_points), Some(nbrkpt_points)) => {
                if ninten_points > 0 && nbrkpt_points > 0 && ninten_points != nbrkpt_points {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: BoundarySymbol::from(WB14_SYMBOL_HYETOGRAPH_NINTEN),
                        value: Self::diagnostic_count_to_f64(ninten_points),
                        minimum: Some(Self::diagnostic_count_to_f64(nbrkpt_points)),
                        maximum: Some(Self::diagnostic_count_to_f64(nbrkpt_points)),
                    });
                }
                ninten_points.max(nbrkpt_points)
            }
            (Some(ninten_points), None) => ninten_points,
            (None, Some(nbrkpt_points)) => nbrkpt_points,
        };

        if point_count > MAX_CLIMATE_FORCING_SERIES_POINTS {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_HYETOGRAPH_NINTEN),
                value: Self::diagnostic_count_to_f64(point_count),
                minimum: Some(0.0),
                maximum: Some(Self::diagnostic_count_to_f64(
                    MAX_CLIMATE_FORCING_SERIES_POINTS,
                )),
            });
        }

        Ok(point_count)
    }

    fn load_hyetograph_series(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        point_count: usize,
    ) -> Result<(Vec<f64>, Vec<f64>), Wb11HydrologyKernelGuardError> {
        if point_count == 0 {
            return Ok((Vec::new(), Vec::new()));
        }

        let mut times = Vec::with_capacity(point_count);
        let mut intensities = Vec::with_capacity(point_count);

        for index in 1..=point_count {
            let time_symbol = format!("timem_{index:04}");
            let intensity_symbol = format!("intsty_{index:04}");

            let time_key = BoundarySymbol::from(time_symbol.clone());
            let Some(time_value) = request.state_surface.get(&time_key) else {
                return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: time_key,
                });
            };
            let time_scalar = time_value.as_f64();
            if !time_scalar.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from(time_symbol.as_str()),
                    value: time_scalar,
                });
            }
            if time_scalar < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(time_symbol.as_str()),
                    value: time_scalar,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            times.push(if time_scalar < 0.0 { 0.0 } else { time_scalar });

            let intensity_key = BoundarySymbol::from(intensity_symbol.clone());
            let Some(intensity_value) = request.state_surface.get(&intensity_key) else {
                return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: intensity_key,
                });
            };
            let intensity_scalar = intensity_value.as_f64();
            if !intensity_scalar.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from(intensity_symbol.as_str()),
                    value: intensity_scalar,
                });
            }
            if intensity_scalar < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(intensity_symbol.as_str()),
                    value: intensity_scalar,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            intensities.push(if intensity_scalar < 0.0 {
                0.0
            } else {
                intensity_scalar
            });
        }

        if point_count == 1 && intensities[0] > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("intsty_0001"),
                value: intensities[0],
                minimum: Some(0.0),
                maximum: Some(0.0),
            });
        }

        for index in 1..point_count {
            let previous = times[index - 1];
            let current = times[index];
            if current <= previous + WB11_ZERO_THRESHOLD {
                let symbol = format!("timem_{:04}", index + 1);
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value: current,
                    minimum: Some(previous + WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
        }

        Ok((times, intensities))
    }

    fn irrigation_depletion_period_symbol(
        period_index: usize,
        field: HillslopeIrrigationDepletionPeriodField,
    ) -> BoundarySymbol {
        BoundarySymbol::from(format!(
            "irrigation.depletion.period_{period_index:04}.{}",
            field.as_str()
        ))
    }

    fn irrigation_fixeddate_event_symbol(
        event_index: usize,
        field: HillslopeIrrigationFixedDateEventField,
    ) -> BoundarySymbol {
        BoundarySymbol::from(format!(
            "irrigation.fixeddate.event_{event_index:04}.{}",
            field.as_str()
        ))
    }

    fn require_non_negative_integral_state_symbol(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: &BoundarySymbol,
    ) -> Result<usize, Wb11HydrologyKernelGuardError> {
        let scalar = Self::require_state_scalar_for_symbol(request, phase_class, symbol)?;
        if scalar < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let rounded = scalar.round();
        if (scalar - rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let rounded_text = format!("{rounded:.0}");
        let Ok(parsed) = rounded_text.parse::<usize>() else {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol.clone(),
                value: scalar,
                minimum: Some(0.0),
                maximum: Some(Self::diagnostic_count_to_f64(usize::MAX)),
            });
        };
        Ok(parsed)
    }

    fn normalize_irrigation_event(
        phase_class: HillslopeKernelPhaseClass,
        source: IrrigationScheduleSource,
        event_index: usize,
        system_type: f64,
        depth_m: f64,
        rate_m_per_s: f64,
        hyetograph_duration_s: f64,
    ) -> Result<ActiveIrrigationEvent, Wb11HydrologyKernelGuardError> {
        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_RUNTIME_DEPTH_M,
            depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_RUNTIME_RATE_MPS,
            rate_m_per_s,
            Some(0.0),
            None,
        )?;
        if depth_m <= WB11_ZERO_THRESHOLD {
            return Ok(ActiveIrrigationEvent {
                source,
                event_index,
                system_type,
                depth_m: 0.0,
                duration_s: 0.0,
                rate_m_per_s: 0.0,
            });
        }
        if rate_m_per_s <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(IRRIG_SYMBOL_RUNTIME_RATE_MPS),
                value: rate_m_per_s,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let raw_duration = depth_m / rate_m_per_s;
        if !raw_duration.is_finite() || raw_duration <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(IRRIG_SYMBOL_RUNTIME_DURATION_S),
                value: raw_duration,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        if hyetograph_duration_s <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(IRRIG_SYMBOL_RUNTIME_DURATION_S),
                value: hyetograph_duration_s,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let (duration_s, adjusted_rate) = if raw_duration > hyetograph_duration_s {
            (hyetograph_duration_s, depth_m / hyetograph_duration_s)
        } else {
            (raw_duration, rate_m_per_s)
        };

        Ok(ActiveIrrigationEvent {
            source,
            event_index,
            system_type,
            depth_m,
            duration_s,
            rate_m_per_s: adjusted_rate,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn resolve_fixeddate_irrigation_event(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        runtime_day: usize,
        runtime_year: usize,
        hyetograph_duration_s: f64,
    ) -> Result<Option<ActiveIrrigationEvent>, Wb11HydrologyKernelGuardError> {
        let event_count = Self::require_non_negative_integral_state_symbol(
            request,
            phase_class,
            &BoundarySymbol::from(IRRIG_SYMBOL_FIXEDDATE_EVENT_COUNT),
        )?;
        if event_count == 0 {
            return Ok(None);
        }

        let system_type =
            Self::require_state_scalar(request, phase_class, IRRIG_SYMBOL_FIXEDDATE_SYSTEM_TYPE)?;
        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_FIXEDDATE_SYSTEM_TYPE,
            system_type,
            Some(1.0),
            Some(2.0),
        )?;

        for event_index in 1..=event_count {
            let ofe_symbol = Self::irrigation_fixeddate_event_symbol(
                event_index,
                HillslopeIrrigationFixedDateEventField::OfeId,
            );
            let event_ofe = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &ofe_symbol,
            )?;
            if event_ofe != 1 {
                continue;
            }

            let day_symbol = Self::irrigation_fixeddate_event_symbol(
                event_index,
                HillslopeIrrigationFixedDateEventField::Day,
            );
            let event_day = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &day_symbol,
            )?;
            let year_symbol = Self::irrigation_fixeddate_event_symbol(
                event_index,
                HillslopeIrrigationFixedDateEventField::Year,
            );
            let event_year = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &year_symbol,
            )?;

            let termination_symbol = Self::irrigation_fixeddate_event_symbol(
                event_index,
                HillslopeIrrigationFixedDateEventField::ScheduleTerminationFlag,
            );
            let termination_flag =
                Self::require_state_scalar_for_symbol(request, phase_class, &termination_symbol)?;
            Self::require_state_range(
                phase_class,
                IRRIG_SYMBOL_RUNTIME_SOURCE,
                termination_flag,
                Some(0.0),
                Some(1.0),
            )?;
            if termination_flag >= 1.0 - WB11_ZERO_THRESHOLD {
                continue;
            }

            if event_day != runtime_day || event_year != runtime_year {
                continue;
            }

            if system_type >= 2.0 - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(IRRIG_SYMBOL_FIXEDDATE_SYSTEM_TYPE),
                    value: system_type,
                    minimum: Some(1.0),
                    maximum: Some(1.0),
                });
            }

            let depth_symbol = Self::irrigation_fixeddate_event_symbol(
                event_index,
                HillslopeIrrigationFixedDateEventField::SprinklerDepthMeters,
            );
            let depth_m =
                Self::require_state_scalar_for_symbol(request, phase_class, &depth_symbol)?;
            let rate_symbol = Self::irrigation_fixeddate_event_symbol(
                event_index,
                HillslopeIrrigationFixedDateEventField::SprinklerRateMetersPerSecond,
            );
            let base_rate =
                Self::require_state_scalar_for_symbol(request, phase_class, &rate_symbol)?;
            let nozzle_symbol = Self::irrigation_fixeddate_event_symbol(
                event_index,
                HillslopeIrrigationFixedDateEventField::SprinklerNozzleFactor,
            );
            let nozzle =
                Self::require_state_scalar_for_symbol(request, phase_class, &nozzle_symbol)?;
            Self::require_state_range(
                phase_class,
                IRRIG_SYMBOL_RUNTIME_RATE_MPS,
                nozzle,
                Some(0.0),
                None,
            )?;
            let rate_m_per_s = base_rate * nozzle;
            return Ok(Some(Self::normalize_irrigation_event(
                phase_class,
                IrrigationScheduleSource::FixedDate,
                event_index,
                system_type,
                depth_m,
                rate_m_per_s,
                hyetograph_duration_s,
            )?));
        }

        Ok(None)
    }

    #[allow(clippy::too_many_lines)]
    fn resolve_depletion_irrigation_event(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        runtime_day: usize,
        runtime_year: usize,
        hyetograph_duration_s: f64,
    ) -> Result<Option<ActiveIrrigationEvent>, Wb11HydrologyKernelGuardError> {
        let period_count = Self::require_non_negative_integral_state_symbol(
            request,
            phase_class,
            &BoundarySymbol::from(IRRIG_SYMBOL_DEPLETION_PERIOD_COUNT),
        )?;
        if period_count == 0 {
            return Ok(None);
        }

        let system_type =
            Self::require_state_scalar(request, phase_class, IRRIG_SYMBOL_DEPLETION_SYSTEM_TYPE)?;
        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_DEPLETION_SYSTEM_TYPE,
            system_type,
            Some(1.0),
            Some(2.0),
        )?;

        let min_depth =
            Self::require_state_scalar(request, phase_class, IRRIG_SYMBOL_DEPLETION_MIN_DEPTH_M)?;
        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_DEPLETION_MIN_DEPTH_M,
            min_depth,
            Some(0.0),
            None,
        )?;
        let max_depth =
            Self::optional_state_scalar(request, phase_class, IRRIG_SYMBOL_DEPLETION_MAX_DEPTH_M)?;
        if let Some(value) = max_depth {
            Self::require_state_range(
                phase_class,
                IRRIG_SYMBOL_DEPLETION_MAX_DEPTH_M,
                value,
                Some(min_depth),
                None,
            )?;
        }

        let soil_water = Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        let field_capacity =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_FIELD_CAPACITY)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_FIELD_CAPACITY,
            field_capacity,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        let depletion_ratio = soil_water / field_capacity;
        if !depletion_ratio.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("irrigation.depletion.trigger_ratio"),
                value: depletion_ratio,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let runtime_date_key = i64::try_from(runtime_year)
            .unwrap_or(i64::MAX)
            .saturating_mul(1000)
            .saturating_add(i64::try_from(runtime_day).unwrap_or(i64::MAX));

        for period_index in 1..=period_count {
            let element_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::ElementId,
            );
            let element_id = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &element_symbol,
            )?;
            if element_id != 1 {
                continue;
            }

            let start_day_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::StartDoy,
            );
            let start_day = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &start_day_symbol,
            )?;
            let start_year_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::StartYear,
            );
            let start_year = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &start_year_symbol,
            )?;
            let end_day_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::EndDoy,
            );
            let end_day = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &end_day_symbol,
            )?;
            let end_year_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::EndYear,
            );
            let end_year = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &end_year_symbol,
            )?;

            let start_key = i64::try_from(start_year)
                .unwrap_or(i64::MAX)
                .saturating_mul(1000)
                .saturating_add(i64::try_from(start_day).unwrap_or(i64::MAX));
            let end_key = i64::try_from(end_year)
                .unwrap_or(i64::MAX)
                .saturating_mul(1000)
                .saturating_add(i64::try_from(end_day).unwrap_or(i64::MAX));
            if end_key < start_key {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from("irrigation.depletion.period_window"),
                    value: Self::diagnostic_i64_to_f64(end_key),
                    minimum: Some(Self::diagnostic_i64_to_f64(start_key)),
                    maximum: None,
                });
            }
            if runtime_date_key < start_key || runtime_date_key > end_key {
                continue;
            }

            let threshold_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::DepletionTriggerRatio,
            );
            let threshold =
                Self::require_state_scalar_for_symbol(request, phase_class, &threshold_symbol)?;
            Self::require_state_range(
                phase_class,
                IRRIG_SYMBOL_RUNTIME_SOURCE,
                threshold,
                Some(0.0),
                Some(1.0),
            )?;
            if depletion_ratio > threshold + WB11_ZERO_THRESHOLD {
                continue;
            }

            if system_type >= 2.0 - WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(IRRIG_SYMBOL_DEPLETION_SYSTEM_TYPE),
                    value: system_type,
                    minimum: Some(1.0),
                    maximum: Some(1.0),
                });
            }

            let depth_ratio_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::SprinklerDepthRatio,
            );
            let depth_ratio =
                Self::require_state_scalar_for_symbol(request, phase_class, &depth_ratio_symbol)?;
            Self::require_state_range(
                phase_class,
                IRRIG_SYMBOL_RUNTIME_DEPTH_M,
                depth_ratio,
                Some(0.0),
                None,
            )?;
            let depth_cap = max_depth.unwrap_or(min_depth);
            let depth_from_ratio = depth_ratio * depth_cap;
            let depth_m = depth_from_ratio.max(min_depth);

            let rate_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::SprinklerRateMetersPerSecond,
            );
            let base_rate =
                Self::require_state_scalar_for_symbol(request, phase_class, &rate_symbol)?;
            let nozzle_symbol = Self::irrigation_depletion_period_symbol(
                period_index,
                HillslopeIrrigationDepletionPeriodField::SprinklerNozzleFactor,
            );
            let nozzle =
                Self::require_state_scalar_for_symbol(request, phase_class, &nozzle_symbol)?;
            Self::require_state_range(
                phase_class,
                IRRIG_SYMBOL_RUNTIME_RATE_MPS,
                nozzle,
                Some(0.0),
                None,
            )?;
            let rate_m_per_s = base_rate * nozzle;
            return Ok(Some(Self::normalize_irrigation_event(
                phase_class,
                IrrigationScheduleSource::Depletion,
                period_index,
                system_type,
                depth_m,
                rate_m_per_s,
                hyetograph_duration_s,
            )?));
        }

        Ok(None)
    }

    fn resolve_active_irrigation_event(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        hyetograph_duration_s: f64,
    ) -> Result<Option<ActiveIrrigationEvent>, Wb11HydrologyKernelGuardError> {
        let fixeddate_enabled =
            Self::optional_state_scalar(request, phase_class, IRRIG_SYMBOL_FIXEDDATE_ENABLED)?;
        let depletion_enabled =
            Self::optional_state_scalar(request, phase_class, IRRIG_SYMBOL_DEPLETION_ENABLED)?;

        if fixeddate_enabled.is_none() && depletion_enabled.is_none() {
            return Ok(None);
        }

        let runtime_day = Self::require_non_negative_integral_state_symbol(
            request,
            phase_class,
            &BoundarySymbol::from("day"),
        )?;
        if !(1..=366).contains(&runtime_day) {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("day"),
                value: Self::diagnostic_count_to_f64(runtime_day),
                minimum: Some(1.0),
                maximum: Some(366.0),
            });
        }
        let runtime_year = Self::require_non_negative_integral_state_symbol(
            request,
            phase_class,
            &BoundarySymbol::from("year"),
        )?;

        if fixeddate_enabled.unwrap_or(0.0) >= 1.0 - WB11_ZERO_THRESHOLD {
            if let Some(event) = Self::resolve_fixeddate_irrigation_event(
                request,
                phase_class,
                runtime_day,
                runtime_year,
                hyetograph_duration_s,
            )? {
                return Ok(Some(event));
            }
        }

        if depletion_enabled.unwrap_or(0.0) >= 1.0 - WB11_ZERO_THRESHOLD {
            if let Some(event) = Self::resolve_depletion_irrigation_event(
                request,
                phase_class,
                runtime_day,
                runtime_year,
                hyetograph_duration_s,
            )? {
                return Ok(Some(event));
            }
        }

        Ok(None)
    }

    fn interval_overlap_duration(
        interval_start: f64,
        interval_end: f64,
        active_duration: f64,
    ) -> f64 {
        if active_duration <= 0.0 {
            return 0.0;
        }
        let overlap_start = interval_start.max(0.0);
        let overlap_end = interval_end.min(active_duration);
        (overlap_end - overlap_start).max(0.0)
    }

    fn resolve_active_snow_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(WB14_SYMBOL_SNOW_FILE_PRESENT);
        let Some(value) = request.state_surface.get(&key) else {
            return Ok(false);
        };

        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: key,
                value: scalar,
            });
        }
        if !(-WB11_ZERO_THRESHOLD..=1.0 + WB11_ZERO_THRESHOLD).contains(&scalar) {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_FILE_PRESENT),
                value: scalar,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }

        let rounded = scalar.round();
        if (scalar - rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_FILE_PRESENT),
                value: scalar,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }

        Ok(rounded >= 1.0 - WB11_ZERO_THRESHOLD)
    }

    fn resolve_active_frost_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let key = BoundarySymbol::from(WB14_SYMBOL_FROST_FILE_PRESENT);
        let Some(value) = request.state_surface.get(&key) else {
            return Ok(false);
        };

        let scalar = value.as_f64();
        if !scalar.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: key,
                value: scalar,
            });
        }
        if !(-WB11_ZERO_THRESHOLD..=1.0 + WB11_ZERO_THRESHOLD).contains(&scalar) {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FILE_PRESENT),
                value: scalar,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }

        let rounded = scalar.round();
        if (scalar - rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_FILE_PRESENT),
                value: scalar,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }
        if rounded < 1.0 - WB11_ZERO_THRESHOLD {
            return Ok(false);
        }

        let wint_red =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_WINT_RED)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_WINT_RED,
            wint_red,
            Some(0.0),
            Some(1.0),
        )?;
        let wint_rounded = wint_red.round();
        if (wint_red - wint_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_WINT_RED),
                value: wint_red,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }

        Ok(wint_rounded >= 1.0 - WB11_ZERO_THRESHOLD)
    }

    #[allow(clippy::too_many_lines)]
    fn compute_active_frost_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        soil_conductivity: f64,
    ) -> Result<FrostCouplingOutcome, Wb11HydrologyKernelGuardError> {
        let wint_red =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_WINT_RED)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_WINT_RED,
            wint_red,
            Some(0.0),
            Some(1.0),
        )?;
        let wint_rounded = wint_red.round();
        if (wint_red - wint_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_WINT_RED),
                value: wint_red,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        }
        if wint_rounded < 1.0 - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_FROST_WINT_RED),
                value: wint_red,
                minimum: Some(1.0),
                maximum: Some(1.0),
            });
        }

        let fine_top =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_FINE_TOP)?;
        let fine_bot =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_FINE_BOT)?;
        for (symbol, value) in [
            (WB14_SYMBOL_FROST_FINE_TOP, fine_top),
            (WB14_SYMBOL_FROST_FINE_BOT, fine_bot),
        ] {
            Self::require_state_range(phase_class, symbol, value, Some(1.0), Some(10.0))?;
            let rounded = value.round();
            if (value - rounded).abs() > WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    minimum: Some(1.0),
                    maximum: Some(10.0),
                });
            }
        }

        let ksnowf = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KSNOWF)?;
        let kresf = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KRESF)?;
        let ksoilf = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KSOILF)?;
        for (symbol, value) in [
            (WB14_SYMBOL_FROST_KSNOWF, ksnowf),
            (WB14_SYMBOL_FROST_KRESF, kresf),
            (WB14_SYMBOL_FROST_KSOILF, ksoilf),
        ] {
            Self::require_state_range(phase_class, symbol, value, Some(0.1), Some(10.0))?;
        }

        let kfactor1 =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KFACTOR1)?;
        let kfactor2 =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KFACTOR2)?;
        let kfactor3 =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_FROST_KFACTOR3)?;
        for (symbol, value) in [
            (WB14_SYMBOL_FROST_KFACTOR1, kfactor1),
            (WB14_SYMBOL_FROST_KFACTOR2, kfactor2),
            (WB14_SYMBOL_FROST_KFACTOR3, kfactor3),
        ] {
            if value <= 0.0 + WB11_ZERO_THRESHOLD || value > 1.0 + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: Some(1.0),
                });
            }
        }

        let tmax = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_TMAX)?;
        let tmin = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_TMIN)?;
        if tmax < tmin - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_TMAX),
                value: tmax,
                minimum: Some(tmin),
                maximum: None,
            });
        }

        let freeze_active = tmin <= 0.0 + WB11_ZERO_THRESHOLD;
        let dfrost = if freeze_active {
            WB14_FROST_MAX_DEPTH_M
        } else {
            0.0
        };
        let dthaw = if freeze_active {
            0.0
        } else {
            WB14_FROST_MAX_DEPTH_M
        };
        let nft = if freeze_active { 1.0 } else { 0.0 };
        let conductivity_mean = (ksnowf + kresf + ksoilf) / 3.0;
        let fine_layer_scale = (fine_top + fine_bot) / 20.0;
        let ws_frz = dfrost * conductivity_mean * fine_layer_scale;
        let kfactor_floor = kfactor1.min(kfactor2.min(kfactor3));
        let freeze_fraction = (dfrost / WB14_FROST_MAX_DEPTH_M).clamp(0.0, 1.0);
        let infcap_frz =
            soil_conductivity * (1.0 - freeze_fraction + freeze_fraction * kfactor_floor);

        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_DFROST,
            dfrost,
            Some(0.0),
            Some(WB14_FROST_MAX_DEPTH_M),
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_DTHAW,
            dthaw,
            Some(0.0),
            Some(WB14_FROST_MAX_DEPTH_M),
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_NFT,
            nft,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_WS_FRZ,
            ws_frz,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_FROST_RUNTIME_INFCAP_FRZ,
            infcap_frz,
            Some(0.0),
            Some(soil_conductivity),
        )?;

        Ok(FrostCouplingOutcome {
            dfrost,
            dthaw,
            nft,
            ws_frz,
            infcap_frz,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn compute_active_snow_coupling(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        hyetograph_rainfall: f64,
    ) -> Result<SnowCouplingOutcome, Wb11HydrologyKernelGuardError> {
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            hyetograph_rainfall,
            Some(0.0),
            None,
        )?;

        let rst = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SNOW_RST)?;
        let newsnw = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SNOW_NEWSNW)?;
        let ssd = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SNOW_SSD)?;
        let runtime_swe =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SNOW_RUNTIME_SWE)?;
        let tmax = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_TMAX)?;
        let tmin = Self::require_state_scalar(request, phase_class, WB14_SYMBOL_TMIN)?;

        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SNOW_NEWSNW,
            newsnw,
            Some(0.0),
            None,
        )?;
        Self::require_state_range(phase_class, WB14_SYMBOL_SNOW_SSD, ssd, Some(0.0), None)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SNOW_RUNTIME_SWE,
            runtime_swe,
            Some(0.0),
            None,
        )?;

        if newsnw > ssd + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_NEWSNW),
                value: newsnw,
                minimum: Some(0.0),
                maximum: Some(ssd),
            });
        }

        let snow_fraction = if tmax <= rst + WB11_ZERO_THRESHOLD {
            1.0
        } else if tmin >= rst - WB11_ZERO_THRESHOLD {
            0.0
        } else {
            let span = tmax - tmin;
            if span <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_TMAX),
                    value: tmax,
                    minimum: Some(tmin + WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            let fraction = (rst - tmin) / span;
            if !(-WB11_ZERO_THRESHOLD..=1.0 + WB11_ZERO_THRESHOLD).contains(&fraction) {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB14_SYMBOL_SNOW_RST),
                    value: rst,
                    minimum: Some(tmin),
                    maximum: Some(tmax),
                });
            }
            fraction.clamp(0.0, 1.0)
        };

        let accumulation = hyetograph_rainfall * snow_fraction;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            accumulation,
            Some(0.0),
            Some(hyetograph_rainfall),
        )?;

        let available_swe = runtime_swe + accumulation;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SNOW_RUNTIME_SWE,
            available_swe,
            Some(0.0),
            None,
        )?;

        let temp_surplus = (tmax - rst).max(0.0);
        let melt_fraction = if temp_surplus <= WB11_ZERO_THRESHOLD {
            0.0
        } else {
            temp_surplus / (temp_surplus + 1.0)
        };
        let density_factor = newsnw / ssd;
        let melt = available_swe * melt_fraction * density_factor;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            melt,
            Some(0.0),
            Some(available_swe),
        )?;

        let runtime_swe_after = available_swe - melt;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SNOW_RUNTIME_SWE,
            runtime_swe_after,
            Some(0.0),
            None,
        )?;

        let signed_s = melt - accumulation;
        if !signed_s.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_SNOW_COUPLING_S),
                value: signed_s,
                minimum: None,
                maximum: None,
            });
        }

        Ok(SnowCouplingOutcome {
            signed_s,
            accumulation,
            runtime_swe: runtime_swe_after,
        })
    }

    fn compute_canopy_interception_depth(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        hyetograph_rainfall: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let cancov = Self::require_state_scalar(request, phase_class, WB15_SYMBOL_PLANT_CANCOV)?;
        Self::require_state_range(
            phase_class,
            WB15_SYMBOL_PLANT_CANCOV,
            cancov,
            Some(0.0),
            Some(WB15_CANCOV_MAX),
        )?;

        let lai = Self::require_state_scalar(request, phase_class, WB15_SYMBOL_PLANT_LAI)?;
        Self::require_state_range(phase_class, WB15_SYMBOL_PLANT_LAI, lai, Some(0.0), None)?;

        let vdmt = Self::require_state_scalar(request, phase_class, WB15_SYMBOL_PLANT_VDMT)?;
        Self::require_state_range(
            phase_class,
            WB15_SYMBOL_PLANT_VDMT,
            vdmt,
            Some(0.0),
            Some(WB15_VDMT_MAX),
        )?;

        if cancov <= WB11_ZERO_THRESHOLD || lai <= WB11_ZERO_THRESHOLD {
            return Ok(0.0);
        }

        let biomass_kg_ha = vdmt * WB15_BIOMASS_TO_KG_HA;
        if !biomass_kg_ha.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB15_SYMBOL_PLANT_VDMT),
                value: biomass_kg_ha,
                minimum: Some(0.0),
                maximum: Some(WB15_VDMT_MAX * WB15_BIOMASS_TO_KG_HA),
            });
        }

        let potential_interception = cancov
            * ((WB15_INTERCEPT_LINEAR_COEFF * biomass_kg_ha
                - WB15_INTERCEPT_QUADRATIC_COEFF * biomass_kg_ha.powi(2))
                / WB15_INTERCEPT_MM_TO_M);
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            potential_interception,
            Some(0.0),
            None,
        )?;

        let interception = potential_interception.min(hyetograph_rainfall);
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            interception,
            Some(0.0),
            Some(hyetograph_rainfall),
        )?;
        Ok(interception)
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_coupled_infiltration_depth(
        phase_class: HillslopeKernelPhaseClass,
        infiltration_conductivity: f64,
        matric_potential: f64,
        times: &[f64],
        intensities: &[f64],
        rainfall_scale: f64,
        irrigation_rate_m_per_s: f64,
        irrigation_duration_s: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let mut cumulative_infiltration = 0.0_f64;
        for index in 0..times.len().saturating_sub(1) {
            let interval_duration = times[index + 1] - times[index];
            let scaled_rainfall_rate = intensities[index] * rainfall_scale;
            let interval_rainfall = scaled_rainfall_rate * interval_duration;
            if !interval_rainfall.is_finite() || interval_rainfall < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                    value: interval_rainfall,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }

            let interval_irrigation_duration = Self::interval_overlap_duration(
                times[index],
                times[index + 1],
                irrigation_duration_s,
            );
            let interval_irrigation_depth = irrigation_rate_m_per_s * interval_irrigation_duration;
            if !interval_irrigation_depth.is_finite()
                || interval_irrigation_depth < -WB11_ZERO_THRESHOLD
            {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(IRRIG_SYMBOL_DAILY_IRRIGATION),
                    value: interval_irrigation_depth,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }

            let interval_liquid_depth = interval_rainfall + interval_irrigation_depth.max(0.0);
            if interval_duration <= WB11_ZERO_THRESHOLD {
                continue;
            }

            let rainfall_rate = interval_liquid_depth / interval_duration;
            if !rainfall_rate.is_finite() || rainfall_rate < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                    value: rainfall_rate,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }

            let interval_infiltration = Self::compute_interval_infiltration_depth(
                phase_class,
                infiltration_conductivity,
                matric_potential,
                cumulative_infiltration,
                rainfall_rate,
                interval_duration,
            )?;
            cumulative_infiltration += interval_infiltration;
        }

        if !cumulative_infiltration.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: cumulative_infiltration,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Ok(cumulative_infiltration)
    }

    fn resolve_interception_rainfall_scale(
        phase_class: HillslopeKernelPhaseClass,
        hyetograph_rainfall: f64,
        interception_rainfall_input: f64,
        interception: f64,
    ) -> Result<(f64, f64), Wb11HydrologyKernelGuardError> {
        let liquid_after_interception = interception_rainfall_input - interception;
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            liquid_after_interception,
            Some(0.0),
            Some(interception_rainfall_input),
        )?;

        if hyetograph_rainfall <= WB11_ZERO_THRESHOLD {
            return Ok((liquid_after_interception, 0.0));
        }

        let rainfall_scale = liquid_after_interception / hyetograph_rainfall;
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            rainfall_scale,
            Some(0.0),
            None,
        )?;
        Ok((liquid_after_interception, rainfall_scale))
    }

    fn require_infiltration_liquid_closure(
        phase_class: HillslopeKernelPhaseClass,
        cumulative_infiltration: f64,
        liquid_after_interception: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if cumulative_infiltration > liquid_after_interception + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: cumulative_infiltration,
                minimum: Some(0.0),
                maximum: Some(liquid_after_interception),
            });
        }

        Ok(())
    }

    fn require_non_negative_liquid_input(
        phase_class: HillslopeKernelPhaseClass,
        liquid_input: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            liquid_input,
            Some(0.0),
            None,
        )?;
        Ok(())
    }

    fn compute_runoff_after_interception(
        phase_class: HillslopeKernelPhaseClass,
        liquid_after_interception: f64,
        signed_s: f64,
        runon_input: f64,
        cumulative_infiltration: f64,
        depression_storage_delta: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let liquid_input = liquid_after_interception + signed_s;
        Self::require_non_negative_liquid_input(phase_class, liquid_input)?;

        let q_runoff =
            liquid_input + runon_input - cumulative_infiltration - depression_storage_delta;
        Self::require_flux_range(phase_class, WB12_SYMBOL_RUNOFF_Q, q_runoff, Some(0.0), None)?;
        Ok(q_runoff)
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_storage_reconciled_with_interception(
        phase_class: HillslopeKernelPhaseClass,
        storage_initial: f64,
        precip_input: f64,
        snow_coupling_s: f64,
        irrigation_input: f64,
        interception: f64,
        q_runoff: f64,
        et: f64,
        percolation_loss: f64,
        subsurface_loss: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let storage_reconciled =
            storage_initial + precip_input + snow_coupling_s + irrigation_input
                - interception
                - q_runoff
                - et
                - percolation_loss
                - subsurface_loss;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_STORAGE_RECONCILED,
            storage_reconciled,
            Some(0.0),
            None,
        )?;
        Ok(storage_reconciled)
    }
}

impl Wb11HydrologyKernel {
    #[allow(clippy::too_many_lines)]
    fn solve_ponded_cumulative_infiltration(
        phase_class: HillslopeKernelPhaseClass,
        conductivity: f64,
        matric_potential: f64,
        cumulative_start: f64,
        duration: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if duration <= WB11_ZERO_THRESHOLD {
            return Ok(cumulative_start);
        }
        if matric_potential <= WB11_ZERO_THRESHOLD {
            return Ok(cumulative_start + conductivity * duration);
        }

        let rhs = conductivity * duration;
        let start_plus_matric = cumulative_start + matric_potential;
        if start_plus_matric <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: cumulative_start,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let residual = |candidate: f64| {
            (candidate - cumulative_start)
                - matric_potential * ((candidate + matric_potential) / start_plus_matric).ln()
                - rhs
        };

        let mut lower = cumulative_start;
        let mut upper = cumulative_start + conductivity * duration + matric_potential;
        if upper <= lower {
            upper = lower + 1.0;
        }

        let mut upper_residual = residual(upper);
        if !upper_residual.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: upper,
                minimum: Some(cumulative_start),
                maximum: None,
            });
        }

        let mut expansion_steps = 0_usize;
        while upper_residual < 0.0 {
            upper = upper * 2.0 + 1.0;
            if !upper.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                    value: upper,
                    minimum: Some(cumulative_start),
                    maximum: None,
                });
            }
            upper_residual = residual(upper);
            if !upper_residual.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                    value: upper,
                    minimum: Some(cumulative_start),
                    maximum: None,
                });
            }
            expansion_steps += 1;
            if expansion_steps > 128 {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                    value: upper,
                    minimum: Some(cumulative_start),
                    maximum: None,
                });
            }
        }

        for _ in 0..128 {
            let midpoint = 0.5 * (lower + upper);
            let midpoint_residual = residual(midpoint);
            if !midpoint_residual.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                    value: midpoint,
                    minimum: Some(cumulative_start),
                    maximum: Some(upper),
                });
            }
            if midpoint_residual > 0.0 {
                upper = midpoint;
            } else {
                lower = midpoint;
            }

            let tolerance = 1.0e-10 * upper.max(1.0);
            if (upper - lower) <= tolerance {
                break;
            }
        }

        let solution = 0.5 * (lower + upper);
        if !solution.is_finite() || solution < cumulative_start - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: solution,
                minimum: Some(cumulative_start),
                maximum: None,
            });
        }

        Ok(solution)
    }

    fn compute_interval_infiltration_depth(
        phase_class: HillslopeKernelPhaseClass,
        conductivity: f64,
        matric_potential: f64,
        cumulative_infiltration_start: f64,
        rainfall_rate: f64,
        interval_duration: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        if interval_duration <= 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_HYETOGRAPH_NINTEN),
                value: interval_duration,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let interval_rainfall_depth = rainfall_rate * interval_duration;
        if !interval_rainfall_depth.is_finite() || interval_rainfall_depth < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                value: interval_rainfall_depth,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        if rainfall_rate <= conductivity + WB11_ZERO_THRESHOLD {
            return Ok(interval_rainfall_depth.max(0.0));
        }

        let interval_infiltration = if matric_potential <= WB11_ZERO_THRESHOLD {
            conductivity * interval_duration
        } else {
            let denominator = rainfall_rate - conductivity;
            if denominator <= WB11_ZERO_THRESHOLD {
                interval_rainfall_depth
            } else {
                let ponding_threshold = (conductivity * matric_potential) / denominator;
                if !ponding_threshold.is_finite() || ponding_threshold < 0.0 {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                        value: ponding_threshold,
                        minimum: Some(0.0),
                        maximum: None,
                    });
                }

                if cumulative_infiltration_start >= ponding_threshold - WB11_ZERO_THRESHOLD {
                    let cumulative_end = Self::solve_ponded_cumulative_infiltration(
                        phase_class,
                        conductivity,
                        matric_potential,
                        cumulative_infiltration_start,
                        interval_duration,
                    )?;
                    cumulative_end - cumulative_infiltration_start
                } else {
                    let infiltration_to_ponding =
                        (ponding_threshold - cumulative_infiltration_start).max(0.0);
                    let time_to_ponding = infiltration_to_ponding / rainfall_rate;

                    if time_to_ponding >= interval_duration - WB11_ZERO_THRESHOLD {
                        interval_rainfall_depth
                    } else {
                        let ponded_duration = interval_duration - time_to_ponding;
                        let cumulative_end = Self::solve_ponded_cumulative_infiltration(
                            phase_class,
                            conductivity,
                            matric_potential,
                            ponding_threshold,
                            ponded_duration,
                        )?;
                        infiltration_to_ponding + (cumulative_end - ponding_threshold)
                    }
                }
            }
        };

        if !interval_infiltration.is_finite()
            || interval_infiltration < -WB11_ZERO_THRESHOLD
            || interval_infiltration > interval_rainfall_depth + WB11_ZERO_THRESHOLD
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: interval_infiltration,
                minimum: Some(0.0),
                maximum: Some(interval_rainfall_depth),
            });
        }

        let non_negative_infiltration = if interval_infiltration < 0.0 {
            0.0
        } else {
            interval_infiltration
        };
        Ok(non_negative_infiltration.min(interval_rainfall_depth))
    }

    fn status_from_guard_error(error: &Wb11HydrologyKernelGuardError) -> SimulationStatus {
        let code = error.code();
        let status_result = match error.boundary_class() {
            BoundaryClass::NonFinite => {
                SimulationStatus::non_finite_failure(SimulationPhase::HillslopeKernel, code)
            }
            BoundaryClass::MissingRequiredInput | BoundaryClass::DomainViolation => {
                SimulationStatus::failure(
                    SimulationPhase::HillslopeKernel,
                    true,
                    false,
                    error.boundary_class(),
                    code,
                )
            }
            _ => SimulationStatus::failure(
                SimulationPhase::HillslopeKernel,
                true,
                false,
                BoundaryClass::DomainViolation,
                "HKERNEL-WB11-GEN-E-003",
            ),
        };

        match status_result {
            Ok(status) => status,
            Err(_) => unreachable!("status message ids are non-empty WB11 constants"),
        }
    }

    #[allow(clippy::too_many_lines)]
    fn run_evapotranspiration(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyEvapotranspiration;
        let soil_water = Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water,
            Some(0.0),
            None,
        )?;

        let et_demand = Self::require_state_scalar(request, phase_class, WB11_SYMBOL_ET_DEMAND)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_ET_DEMAND,
            et_demand,
            Some(0.0),
            None,
        )?;

        let lai = Self::require_state_scalar(request, phase_class, WB15_SYMBOL_PLANT_LAI)?;
        Self::require_state_range(phase_class, WB15_SYMBOL_PLANT_LAI, lai, Some(0.0), None)?;

        let residue_interception =
            Self::require_state_scalar(request, phase_class, WB17_SYMBOL_RESIDUE_INTERCEPTION)?;
        Self::require_state_range(
            phase_class,
            WB17_SYMBOL_RESIDUE_INTERCEPTION,
            residue_interception,
            Some(0.0),
            None,
        )?;

        let stage_s1_symbol = BoundarySymbol::from(WB17_STAGE_SYMBOL_S1);
        let stage_s2_symbol = BoundarySymbol::from(WB17_STAGE_SYMBOL_S2);
        let stage_threshold_symbol = BoundarySymbol::from(WB17_STAGE_SYMBOL_TU);
        let stage_counter_symbol = BoundarySymbol::from(WB17_STAGE_SYMBOL_TV);
        let stage_s1 =
            Self::optional_state_scalar_for_symbol(request, phase_class, &stage_s1_symbol)?;
        let stage_s2 =
            Self::optional_state_scalar_for_symbol(request, phase_class, &stage_s2_symbol)?;
        let stage_threshold =
            Self::optional_state_scalar_for_symbol(request, phase_class, &stage_threshold_symbol)?;
        let stage_counter =
            Self::optional_state_scalar_for_symbol(request, phase_class, &stage_counter_symbol)?;
        let stage_state = match (stage_s1, stage_s2, stage_threshold, stage_counter) {
            (None, None, None, None) => None,
            (Some(s1), Some(s2), Some(tu), Some(tv)) => {
                Self::require_state_range_for_symbol(
                    phase_class,
                    &stage_s1_symbol,
                    s1,
                    Some(0.0),
                    None,
                )?;
                Self::require_state_range_for_symbol(
                    phase_class,
                    &stage_s2_symbol,
                    s2,
                    Some(0.0),
                    None,
                )?;
                if tu <= WB11_ZERO_THRESHOLD {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: stage_threshold_symbol.clone(),
                        value: tu,
                        minimum: Some(WB11_ZERO_THRESHOLD),
                        maximum: None,
                    });
                }
                Self::require_state_range_for_symbol(
                    phase_class,
                    &stage_counter_symbol,
                    tv,
                    Some(0.0),
                    None,
                )?;
                Some((s1, s2, tu, tv))
            }
            _ => {
                let missing_symbol = if stage_s1.is_none() {
                    stage_s1_symbol.clone()
                } else if stage_s2.is_none() {
                    stage_s2_symbol.clone()
                } else if stage_threshold.is_none() {
                    stage_threshold_symbol.clone()
                } else {
                    stage_counter_symbol.clone()
                };
                return Err(Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: missing_symbol,
                });
            }
        };

        let soil_evaporation_partition_potential =
            et_demand * (-WB17_LAI_PARTITION_COEFFICIENT * lai).exp();
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_ES,
            soil_evaporation_partition_potential,
            Some(0.0),
            Some(et_demand),
        )?;

        let transpiration_partition_potential = et_demand - soil_evaporation_partition_potential;
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_EP,
            transpiration_partition_potential,
            Some(0.0),
            Some(et_demand),
        )?;

        let residue_evaporation = residue_interception.min(soil_evaporation_partition_potential);
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_ER,
            residue_evaporation,
            Some(0.0),
            Some(soil_evaporation_partition_potential),
        )?;

        let soil_evaporation_potential = soil_evaporation_partition_potential - residue_evaporation;
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_ES,
            soil_evaporation_potential,
            Some(0.0),
            Some(soil_evaporation_partition_potential),
        )?;

        let mut stage_state_updates = Vec::new();
        let soil_evaporation_demand = if let Some((mut s1, mut s2, tu, mut tv)) = stage_state {
            let infiltration =
                Self::optional_state_scalar(request, phase_class, WB12_SYMBOL_INFILTRATION)?
                    .unwrap_or(0.0);
            Self::require_state_range(
                phase_class,
                WB12_SYMBOL_INFILTRATION,
                infiltration,
                Some(0.0),
                None,
            )?;

            let mut es_stage = soil_evaporation_potential;
            if s1 < tu {
                s2 = 0.0;
                let sp = s1 - infiltration;
                s1 = if sp > 0.0 { sp } else { 0.0 };
                s1 += soil_evaporation_potential;
                let su = s1 - tu;
                if su > 0.0 {
                    es_stage = soil_evaporation_potential - WB17_STAGE_ONE_DEFICIT_SCALE * su;
                    s2 = WB17_STAGE_TWO_DEFICIT_SCALE * su;
                    tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
                }
            } else {
                let sb = infiltration - s2;
                if sb < 0.0 {
                    tv += 1.0;
                    es_stage = WB17_STAGE_TWO_DENOMINATOR * tv.sqrt() - s2;
                    if infiltration > 0.0 {
                        let mut esx = 0.8 * infiltration;
                        if es_stage > esx {
                            esx = es_stage + infiltration;
                        }
                        if esx > soil_evaporation_potential {
                            esx = soil_evaporation_potential;
                        }
                        es_stage = esx;
                    } else if es_stage > soil_evaporation_potential {
                        es_stage = soil_evaporation_potential;
                    }
                    s2 += es_stage - infiltration;
                    tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
                } else {
                    s1 = tu - sb;
                    tv = 0.0;
                    s2 = 0.0;
                    if s1 < 0.0 {
                        s1 = 0.0;
                    }
                    s1 += soil_evaporation_potential;
                    let su = s1 - tu;
                    if su > 0.0 {
                        es_stage = soil_evaporation_potential - WB17_STAGE_ONE_DEFICIT_SCALE * su;
                        s2 = WB17_STAGE_TWO_DEFICIT_SCALE * su;
                        tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
                    }
                }
            }

            Self::require_state_range_for_symbol(
                phase_class,
                &stage_s1_symbol,
                s1,
                Some(0.0),
                None,
            )?;
            Self::require_state_range_for_symbol(
                phase_class,
                &stage_s2_symbol,
                s2,
                Some(0.0),
                None,
            )?;
            Self::require_state_range_for_symbol(
                phase_class,
                &stage_threshold_symbol,
                tu,
                Some(WB11_ZERO_THRESHOLD),
                None,
            )?;
            Self::require_state_range_for_symbol(
                phase_class,
                &stage_counter_symbol,
                tv,
                Some(0.0),
                None,
            )?;
            Self::require_flux_range(
                phase_class,
                WB17_SYMBOL_ES,
                es_stage,
                Some(0.0),
                Some(soil_evaporation_potential),
            )?;

            stage_state_updates.extend([
                WritebackField::bounded(stage_s1_symbol.clone(), s1, Some(0.0), None),
                WritebackField::bounded(stage_s2_symbol.clone(), s2, Some(0.0), None),
                WritebackField::bounded(
                    stage_threshold_symbol.clone(),
                    tu,
                    Some(WB11_ZERO_THRESHOLD),
                    None,
                ),
                WritebackField::bounded(stage_counter_symbol.clone(), tv, Some(0.0), None),
            ]);
            es_stage
        } else {
            soil_evaporation_potential
        };

        let soil_evaporation_actual = soil_water.min(soil_evaporation_demand);
        let soil_after_evaporation = soil_water - soil_evaporation_actual;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_after_evaporation,
            Some(0.0),
            None,
        )?;

        let transpiration_actual = soil_after_evaporation.min(transpiration_partition_potential);
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_EP,
            transpiration_actual,
            Some(0.0),
            Some(transpiration_partition_potential),
        )?;

        let soil_water_after = soil_after_evaporation - transpiration_actual;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        )?;

        let actual_et = residue_evaporation + soil_evaporation_actual + transpiration_actual;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_ET,
            actual_et,
            Some(0.0),
            Some(et_demand),
        )?;

        let etp = transpiration_partition_potential;
        let upi = etp;
        let ui = transpiration_actual;
        let etp_symbol = BoundarySymbol::from(WB17_FLUX_SYMBOL_ETP);
        let uptake_potential_symbol = BoundarySymbol::from(WB17_FLUX_SYMBOL_UPI);
        let uptake_actual_symbol = BoundarySymbol::from(WB17_FLUX_SYMBOL_UI);
        Self::require_flux_range_for_symbol(
            phase_class,
            &etp_symbol,
            etp,
            Some(0.0),
            Some(et_demand),
        )?;
        Self::require_flux_range_for_symbol(
            phase_class,
            &uptake_potential_symbol,
            upi,
            Some(0.0),
            Some(et_demand),
        )?;
        Self::require_flux_range_for_symbol(
            phase_class,
            &uptake_actual_symbol,
            ui,
            Some(0.0),
            Some(upi),
        )?;

        let ws = if etp <= WB11_ZERO_THRESHOLD {
            1.0
        } else {
            ui / etp
        };
        Self::require_flux_range(phase_class, WB11_SYMBOL_WS, ws, Some(0.0), Some(1.0))?;
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_ES,
            soil_evaporation_actual,
            Some(0.0),
            Some(soil_evaporation_demand),
        )?;
        Self::require_flux_range(
            phase_class,
            WB17_SYMBOL_ER,
            residue_evaporation,
            Some(0.0),
            Some(residue_interception),
        )?;

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB11-ET-OK-001")
        else {
            unreachable!("status message ids are non-empty WB11 constants")
        };
        let mut state_updates = vec![WritebackField::bounded(
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        )];
        state_updates.extend(stage_state_updates);

        let writeback = KernelWritebackPayload::with_updates(
            state_updates,
            vec![
                WritebackField::bounded(WB11_SYMBOL_ET, actual_et, Some(0.0), None),
                WritebackField::bounded(WB11_SYMBOL_WS, ws, Some(0.0), Some(1.0)),
                WritebackField::bounded(WB17_SYMBOL_EP, transpiration_actual, Some(0.0), None),
                WritebackField::bounded(WB17_SYMBOL_ES, soil_evaporation_actual, Some(0.0), None),
                WritebackField::bounded(WB17_SYMBOL_ER, residue_evaporation, Some(0.0), None),
                WritebackField::bounded(etp_symbol, etp, Some(0.0), None),
                WritebackField::bounded(uptake_potential_symbol, upi, Some(0.0), None),
                WritebackField::bounded(uptake_actual_symbol, ui, Some(0.0), None),
            ],
        );
        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::too_many_lines)]
    fn run_percolation(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage;
        let soil_water = Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water,
            Some(0.0),
            None,
        )?;

        // Keep legacy WB11 symbol validation to preserve mixed-lane seam guard
        // posture while WB18 per-layer symbols carry the execution authority.
        let field_capacity_legacy =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_FIELD_CAPACITY)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_FIELD_CAPACITY,
            field_capacity_legacy,
            Some(0.0),
            None,
        )?;
        let perc_fraction_legacy =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_PERC_FRACTION)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_PERC_FRACTION,
            perc_fraction_legacy,
            Some(0.0),
            Some(1.0),
        )?;

        let nsl_symbol = BoundarySymbol::from("nsl");
        let layer_count = Self::require_state_non_negative_integral_for_symbol(
            request,
            phase_class,
            &nsl_symbol,
        )?;
        if layer_count == 0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: nsl_symbol,
                value: 0.0,
                minimum: Some(1.0),
                maximum: None,
            });
        }

        let mut theta = Vec::with_capacity(layer_count);
        let mut field_capacity = Vec::with_capacity(layer_count);
        let mut upper_limit = Vec::with_capacity(layer_count);
        let mut conductivity = Vec::with_capacity(layer_count);

        for layer_index in 1..=layer_count {
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index);
            let fc_symbol = Self::wb18_perc_state_symbol("fc", layer_index);
            let ul_symbol = Self::wb18_perc_state_symbol("ul", layer_index);
            let ssc_symbol = Self::wb18_perc_state_symbol("ssc", layer_index);

            let layer_theta =
                Self::require_state_scalar_for_symbol(request, phase_class, &theta_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &theta_symbol,
                layer_theta,
                Some(0.0),
                None,
            )?;

            let layer_fc = Self::require_state_scalar_for_symbol(request, phase_class, &fc_symbol)?;
            Self::require_state_range_for_symbol(
                phase_class,
                &fc_symbol,
                layer_fc,
                Some(0.0),
                None,
            )?;

            let layer_ul = Self::require_state_scalar_for_symbol(request, phase_class, &ul_symbol)?;
            if layer_ul <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: ul_symbol,
                    value: layer_ul,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            if layer_fc > layer_ul + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: fc_symbol,
                    value: layer_fc,
                    minimum: Some(0.0),
                    maximum: Some(layer_ul),
                });
            }

            let layer_ssc =
                Self::require_state_scalar_for_symbol(request, phase_class, &ssc_symbol)?;
            if layer_ssc <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: ssc_symbol,
                    value: layer_ssc,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }

            theta.push(layer_theta);
            field_capacity.push(layer_fc);
            upper_limit.push(layer_ul);
            conductivity.push(layer_ssc);
        }

        let mut per_layer_flux = vec![0.0_f64; layer_count];
        let mut percolation_loss = 0.0_f64;

        // Bottom-up routing mirrors legacy WEPP percolation ordering in PURK.
        for layer_index in (0..layer_count).rev() {
            let layer_theta = theta[layer_index];
            let layer_fc = field_capacity[layer_index];
            let layer_ul = upper_limit[layer_index];
            let layer_ssc = conductivity[layer_index];

            let excess = layer_theta - layer_fc;
            if excess <= WB11_ZERO_THRESHOLD {
                per_layer_flux[layer_index] = 0.0;
                continue;
            }

            let stz = layer_theta / layer_ul;
            if !stz.is_finite() || stz < 0.0 {
                let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index + 1);
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: theta_symbol,
                    value: stz,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }

            let fx = if stz < WB18_PERC_SATURATION_THRESHOLD {
                stz.powf(WB18_PERC_SHAPE_EXPONENT).max(WB18_PERC_MIN_FX)
            } else {
                1.0
            };
            if !fx.is_finite() || fx <= 0.0 {
                let ssc_symbol = Self::wb18_perc_state_symbol("ssc", layer_index + 1);
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: ssc_symbol,
                    value: fx,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }

            let ks_adjusted = layer_ssc * fx;
            let pei_pre = (WB18_PERC_TIMESTEP_S * ks_adjusted).min(excess);
            let pei = if layer_index < layer_count - 1 {
                let lower_ratio = theta[layer_index + 1] / upper_limit[layer_index + 1];
                let lower_radicand = 1.0 - lower_ratio;
                if lower_radicand < -WB11_ZERO_THRESHOLD {
                    let lower_theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index + 2);
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: lower_theta_symbol,
                        value: lower_ratio,
                        minimum: Some(0.0),
                        maximum: Some(1.0),
                    });
                }
                let lower_factor = if lower_radicand <= 0.0 {
                    0.0
                } else {
                    lower_radicand.sqrt()
                };
                pei_pre * lower_factor
            } else {
                pei_pre
            };

            let pei_symbol = Self::wb18_perc_flux_symbol(layer_index + 1);
            Self::require_flux_range_for_symbol(
                phase_class,
                &pei_symbol,
                pei,
                Some(0.0),
                Some(excess),
            )?;

            theta[layer_index] -= pei;
            let theta_symbol = Self::wb18_perc_state_symbol("theta", layer_index + 1);
            Self::require_state_range_for_symbol(
                phase_class,
                &theta_symbol,
                theta[layer_index],
                Some(0.0),
                None,
            )?;

            if layer_index < layer_count - 1 {
                theta[layer_index + 1] += pei;
            } else {
                percolation_loss = pei;
            }

            per_layer_flux[layer_index] = pei;
        }

        let soil_water_after: f64 = theta.iter().sum();
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        )?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_PERC_LOSS_D,
            percolation_loss,
            Some(0.0),
            None,
        )?;

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB11-PERC-OK-001")
        else {
            unreachable!("status message ids are non-empty WB11 constants")
        };
        let mut state_updates = Vec::with_capacity(layer_count + 1);
        state_updates.push(WritebackField::bounded(
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        ));
        for (index, value) in theta.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                Self::wb18_perc_state_symbol("theta", index + 1),
                *value,
                Some(0.0),
                None,
            ));
        }

        let mut flux_updates = Vec::with_capacity(layer_count + 2);
        for (index, value) in per_layer_flux.iter().enumerate() {
            flux_updates.push(WritebackField::bounded(
                Self::wb18_perc_flux_symbol(index + 1),
                *value,
                Some(0.0),
                None,
            ));
        }
        flux_updates.push(WritebackField::bounded(
            WB11_SYMBOL_PERC_LOSS_D,
            percolation_loss,
            Some(0.0),
            None,
        ));
        flux_updates.push(WritebackField::bounded(
            WB11_SYMBOL_PERC_RECHARGE_PE,
            percolation_loss,
            Some(0.0),
            None,
        ));

        let writeback = KernelWritebackPayload::with_updates(state_updates, flux_updates);
        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::too_many_lines)]
    fn run_lateral_transfer(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyLateralTransfer;
        let drainable_storage_legacy =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_DRAINABLE_STORAGE)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_storage_legacy,
            Some(0.0),
            None,
        )?;

        let recharge_pe =
            Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_PERC_RECHARGE_PE)?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_PERC_RECHARGE_PE,
            recharge_pe,
            Some(0.0),
            None,
        )?;

        let avgslp_symbol = BoundarySymbol::from(WB19_SYMBOL_AVG_SLOPE);
        let avgslp = Self::require_state_scalar_for_symbol(request, phase_class, &avgslp_symbol)?;
        Self::require_state_range_for_symbol(phase_class, &avgslp_symbol, avgslp, Some(0.0), None)?;

        let slplen_symbol = BoundarySymbol::from(WB19_SYMBOL_SLOPE_LENGTH);
        let slplen = Self::require_state_scalar_for_symbol(request, phase_class, &slplen_symbol)?;
        if slplen <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: slplen_symbol,
                value: slplen,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let anisotropy_symbol = BoundarySymbol::from(WB19_SYMBOL_LATERAL_ANISOTROPY_RATIO);
        let anisotropy =
            Self::require_state_scalar_for_symbol(request, phase_class, &anisotropy_symbol)?;
        if anisotropy <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: anisotropy_symbol,
                value: anisotropy,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let (mut theta, field_capacity, conductivity, thickness) =
            Self::wb19_load_layer_state(request, phase_class)?;

        let mut saturated_thickness = 0.0_f64;
        let mut conductivity_depth_sum = 0.0_f64;
        let mut saturated_depth_sum = 0.0_f64;
        for (((theta_i, fc_i), ssc_i), dg_i) in theta
            .iter()
            .zip(field_capacity.iter())
            .zip(conductivity.iter())
            .zip(thickness.iter())
        {
            if *theta_i + WB11_ZERO_THRESHOLD >= *fc_i {
                saturated_thickness += *dg_i;
                saturated_depth_sum += *dg_i;
                conductivity_depth_sum += *ssc_i * *dg_i;
            }
        }

        let q_lateral_potential = if saturated_thickness <= WB11_ZERO_THRESHOLD
            || saturated_depth_sum <= WB11_ZERO_THRESHOLD
        {
            0.0
        } else {
            let ke = 86_400.0 * (conductivity_depth_sum / saturated_depth_sum);
            if !ke.is_finite() || ke < 0.0 {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: avgslp_symbol.clone(),
                    value: ke,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }

            let slope_angle = avgslp.atan();
            let slope_factor = slope_angle.sin();
            if !slope_factor.is_finite() || slope_factor < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: avgslp_symbol.clone(),
                    value: slope_factor,
                    minimum: Some(0.0),
                    maximum: Some(1.0),
                });
            }

            (saturated_thickness * anisotropy * ke * slope_factor.max(0.0)) / slplen
        };

        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_LATERAL_Q,
            q_lateral_potential,
            Some(0.0),
            None,
        )?;

        let layer_pool = Self::wb19_drainable_storage(&theta, &field_capacity);
        let available_pool = layer_pool.max(drainable_storage_legacy + recharge_pe);
        let q_lateral = q_lateral_potential.min(available_pool);

        let _withdrawn = Self::wb19_withdraw_top_down(&mut theta, &field_capacity, q_lateral);

        let drainable_after = (available_pool - q_lateral).max(0.0);
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_after,
            Some(0.0),
            None,
        )?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_LATERAL_Q,
            q_lateral,
            Some(0.0),
            Some(available_pool),
        )?;

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB11-LAT-OK-001")
        else {
            unreachable!("status message ids are non-empty WB11 constants")
        };
        let mut state_updates = Vec::with_capacity(theta.len() + 1);
        state_updates.push(WritebackField::bounded(
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_after,
            Some(0.0),
            None,
        ));
        for (index, value) in theta.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                Self::wb18_perc_state_symbol("theta", index + 1),
                *value,
                Some(0.0),
                None,
            ));
        }
        let writeback = KernelWritebackPayload::with_updates(
            state_updates,
            vec![WritebackField::bounded(
                WB11_SYMBOL_LATERAL_Q,
                q_lateral,
                Some(0.0),
                None,
            )],
        );
        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::too_many_lines)]
    fn run_drainage(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyDrainage;
        let drainable_storage_legacy =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_DRAINABLE_STORAGE)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_storage_legacy,
            Some(0.0),
            None,
        )?;

        let drainage_capacity =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_DRAINAGE_COEFFICIENT)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINAGE_COEFFICIENT,
            drainage_capacity,
            Some(0.0),
            None,
        )?;

        let q_lateral = Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_LATERAL_Q)?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_LATERAL_Q,
            q_lateral,
            Some(0.0),
            None,
        )?;

        let drain_enabled_symbol = BoundarySymbol::from(WB19_SYMBOL_DRAIN_ENABLED);
        let drain_enabled_value =
            Self::require_state_scalar_for_symbol(request, phase_class, &drain_enabled_symbol)?;
        let drain_enabled = if (drain_enabled_value - 0.0).abs() <= WB11_ZERO_THRESHOLD {
            false
        } else if (drain_enabled_value - 1.0).abs() <= WB11_ZERO_THRESHOLD {
            true
        } else {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: drain_enabled_symbol,
                value: drain_enabled_value,
                minimum: Some(0.0),
                maximum: Some(1.0),
            });
        };

        let drain_depth_symbol = BoundarySymbol::from(WB19_SYMBOL_DRAIN_DEPTH);
        let drain_depth =
            Self::require_state_scalar_for_symbol(request, phase_class, &drain_depth_symbol)?;
        if drain_depth <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: drain_depth_symbol,
                value: drain_depth,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let drain_spacing_symbol = BoundarySymbol::from(WB19_SYMBOL_DRAIN_SPACING);
        let drain_spacing =
            Self::require_state_scalar_for_symbol(request, phase_class, &drain_spacing_symbol)?;
        if drain_spacing <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: drain_spacing_symbol,
                value: drain_spacing,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let drain_diameter_symbol = BoundarySymbol::from(WB19_SYMBOL_DRAIN_DIAMETER);
        let drain_diameter =
            Self::require_state_scalar_for_symbol(request, phase_class, &drain_diameter_symbol)?;
        if drain_diameter <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: drain_diameter_symbol,
                value: drain_diameter,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let soldep_symbol = BoundarySymbol::from("solthk");
        let soldep = Self::require_state_scalar_for_symbol(request, phase_class, &soldep_symbol)?;
        if soldep <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: soldep_symbol,
                value: soldep,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let (mut theta, field_capacity, conductivity, thickness) =
            Self::wb19_load_layer_state(request, phase_class)?;
        let layer_pool = Self::wb19_drainable_storage(&theta, &field_capacity);
        let available_pool = layer_pool.max(drainable_storage_legacy);

        let mut q_drainage_potential = 0.0_f64;
        let mut tile_layer_index = theta.len().saturating_sub(1);

        if drain_enabled {
            let mut watbl = 0.0_f64;
            let mut hit_unsat_zone = false;
            for idx in (0..theta.len()).rev() {
                if theta[idx] + WB11_ZERO_THRESHOLD >= field_capacity[idx] {
                    if !hit_unsat_zone {
                        watbl += thickness[idx];
                    }
                } else {
                    hit_unsat_zone = true;
                }
            }

            let dep2watbl = soldep - watbl;
            if !dep2watbl.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: soldep_symbol,
                    value: dep2watbl,
                    minimum: Some(0.0),
                    maximum: Some(soldep),
                });
            }

            if dep2watbl <= drain_depth + WB11_ZERO_THRESHOLD {
                let mut cumulative_depth = 0.0_f64;
                let mut tile_layer = 0usize;
                for (idx, dg) in thickness.iter().enumerate() {
                    cumulative_depth += *dg;
                    if cumulative_depth <= drain_depth + WB11_ZERO_THRESHOLD {
                        tile_layer = idx;
                    }
                }
                tile_layer_index = (tile_layer + 1).min(theta.len().saturating_sub(1));

                let mut cumulative_layer_depth = 0.0_f64;
                let mut conductivity_depth_sum = 0.0_f64;
                let mut saturated_depth_sum = 0.0_f64;
                for idx in 0..theta.len() {
                    cumulative_layer_depth += thickness[idx];
                    if cumulative_layer_depth + WB11_ZERO_THRESHOLD >= dep2watbl {
                        conductivity_depth_sum += conductivity[idx] * thickness[idx];
                        saturated_depth_sum += thickness[idx];
                    }
                }

                let dranks = if saturated_depth_sum > WB11_ZERO_THRESHOLD {
                    (conductivity_depth_sum / saturated_depth_sum) * 3600.0 * 100.0
                } else {
                    0.0
                };
                if !dranks.is_finite() || dranks < 0.0 {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: drain_spacing_symbol.clone(),
                        value: dranks,
                        minimum: Some(0.0),
                        maximum: None,
                    });
                }

                let mut drain_depth_cm = (soldep - drain_depth) * 100.0;
                if drain_depth_cm < 0.0 {
                    drain_depth_cm = 1.0;
                }
                let spacing_cm = drain_spacing * 100.0;
                let radius_cm = (drain_diameter / 2.0) * 100.0;

                let spacing_ratio = drain_depth_cm / spacing_cm;
                let equivalent_depth_cm = if spacing_ratio <= 0.3 && spacing_ratio > 0.0 {
                    let radius_ratio = drain_depth_cm / radius_cm;
                    if radius_ratio <= WB11_ZERO_THRESHOLD {
                        return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                            phase_class,
                            symbol: drain_diameter_symbol.clone(),
                            value: radius_ratio,
                            minimum: Some(WB11_ZERO_THRESHOLD),
                            maximum: None,
                        });
                    }
                    let denominator = 1.0
                        + spacing_ratio
                            * ((8.0 / std::f64::consts::PI) * radius_ratio.ln() - WB19_DRAIN_ALPHA);
                    if denominator <= WB11_ZERO_THRESHOLD {
                        return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                            phase_class,
                            symbol: drain_spacing_symbol.clone(),
                            value: denominator,
                            minimum: Some(WB11_ZERO_THRESHOLD),
                            maximum: None,
                        });
                    }
                    drain_depth_cm / denominator
                } else {
                    let log_term = (spacing_cm / radius_cm).ln() - 1.15;
                    if log_term <= WB11_ZERO_THRESHOLD {
                        return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                            phase_class,
                            symbol: drain_spacing_symbol.clone(),
                            value: log_term,
                            minimum: Some(WB11_ZERO_THRESHOLD),
                            maximum: None,
                        });
                    }
                    (spacing_cm * std::f64::consts::PI) / (8.0 * log_term)
                };
                if !equivalent_depth_cm.is_finite() || equivalent_depth_cm < 0.0 {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: drain_spacing_symbol.clone(),
                        value: equivalent_depth_cm,
                        minimum: Some(0.0),
                        maximum: None,
                    });
                }

                let water_table_cm = (drain_depth - dep2watbl).max(0.0) * 100.0;
                let drainage_cm_h = (8.0 * dranks * equivalent_depth_cm * water_table_cm
                    + 4.0 * dranks * water_table_cm.powi(2))
                    / spacing_cm.powi(2);
                if !drainage_cm_h.is_finite() || drainage_cm_h < -WB11_ZERO_THRESHOLD {
                    return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                        phase_class,
                        symbol: drain_depth_symbol.clone(),
                        value: drainage_cm_h,
                        minimum: Some(0.0),
                        maximum: None,
                    });
                }

                q_drainage_potential = (drainage_cm_h / 100.0) * WB19_DRAIN_HOURS_PER_DAY;
                Self::require_flux_range(
                    phase_class,
                    WB11_SYMBOL_DRAINAGE_QDD,
                    q_drainage_potential,
                    Some(0.0),
                    None,
                )?;
            }
        }

        let q_drainage = q_drainage_potential
            .min(drainage_capacity)
            .min(available_pool);
        let _withdrawn = Self::wb19_withdraw_tile_to_surface(
            &mut theta,
            &field_capacity,
            tile_layer_index,
            q_drainage,
        );

        let drainable_after = (available_pool - q_drainage).max(0.0);
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_after,
            Some(0.0),
            None,
        )?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_DRAINAGE_QDD,
            q_drainage,
            Some(0.0),
            Some(drainage_capacity),
        )?;

        let q_subhyd = q_lateral + q_drainage;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_SUBHYD_QD,
            q_subhyd,
            Some(0.0),
            None,
        )?;

        let Ok(status) = SimulationStatus::ok(
            SimulationPhase::HillslopeKernel,
            "HKERNEL-WB11-DRAIN-OK-001",
        ) else {
            unreachable!("status message ids are non-empty WB11 constants")
        };
        let mut state_updates = Vec::with_capacity(theta.len() + 1);
        state_updates.push(WritebackField::bounded(
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_after,
            Some(0.0),
            None,
        ));
        for (index, value) in theta.iter().enumerate() {
            state_updates.push(WritebackField::bounded(
                Self::wb18_perc_state_symbol("theta", index + 1),
                *value,
                Some(0.0),
                None,
            ));
        }
        let writeback = KernelWritebackPayload::with_updates(
            state_updates,
            vec![
                WritebackField::bounded(
                    WB11_SYMBOL_DRAINAGE_QDD,
                    q_drainage,
                    Some(0.0),
                    Some(drainage_capacity),
                ),
                WritebackField::bounded(WB11_SYMBOL_SUBHYD_QD, q_subhyd, Some(0.0), None),
            ],
        );
        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::too_many_lines)]
    fn run_runoff_reconciliation(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        let rainfall_input =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_RAINFALL_INPUT)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            rainfall_input,
            Some(0.0),
            None,
        )?;
        let closure_tolerance =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_RUNOFF_CLOSURE_TOLERANCE)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RUNOFF_CLOSURE_TOLERANCE,
            closure_tolerance,
            Some(0.0),
            None,
        )?;

        let soil_conductivity =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SOIL_CONDUCTIVITY)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_CONDUCTIVITY,
            soil_conductivity,
            Some(0.0),
            None,
        )?;
        let active_frost_coupling = Self::resolve_active_frost_coupling(request, phase_class)?;
        let frost_coupling = if active_frost_coupling {
            Some(Self::compute_active_frost_coupling(
                request,
                phase_class,
                soil_conductivity,
            )?)
        } else {
            None
        };
        let infiltration_conductivity =
            frost_coupling.map_or(soil_conductivity, |outcome| outcome.infcap_frz);

        let soil_layer_depth =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SOIL_LAYER_DEPTH)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_LAYER_DEPTH,
            soil_layer_depth,
            Some(0.0),
            None,
        )?;

        let theta_residual =
            Self::require_state_scalar(request, phase_class, WB14_SYMBOL_SOIL_THETA_RESIDUAL)?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_THETA_RESIDUAL,
            theta_residual,
            Some(0.0),
            None,
        )?;

        let theta_field_capacity = Self::require_state_scalar(
            request,
            phase_class,
            WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY,
        )?;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY,
            theta_field_capacity,
            Some(0.0),
            None,
        )?;

        let moisture_deficit = theta_field_capacity - theta_residual;
        if moisture_deficit < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY),
                value: theta_field_capacity,
                minimum: Some(theta_residual),
                maximum: None,
            });
        }
        let effective_moisture_deficit = if moisture_deficit < 0.0 {
            0.0
        } else {
            moisture_deficit
        };
        let matric_potential = soil_layer_depth * effective_moisture_deficit;
        if !matric_potential.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_INFILTRATION),
                value: matric_potential,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let hyetograph_point_count = Self::resolve_hyetograph_point_count(request, phase_class)?;
        let (times, intensities) =
            Self::load_hyetograph_series(request, phase_class, hyetograph_point_count)?;

        let mut hyetograph_rainfall = 0.0_f64;
        for index in 0..times.len().saturating_sub(1) {
            let interval_duration = times[index + 1] - times[index];
            let rainfall_rate = intensities[index];
            let interval_rainfall = rainfall_rate * interval_duration;
            if !interval_rainfall.is_finite() || interval_rainfall < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                    value: interval_rainfall,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            hyetograph_rainfall += interval_rainfall.max(0.0);
        }

        if !hyetograph_rainfall.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                value: hyetograph_rainfall,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let hyetograph_duration_s = if times.len() >= 2 {
            times[times.len() - 1] - times[0]
        } else {
            0.0
        };
        let active_irrigation_event =
            Self::resolve_active_irrigation_event(request, phase_class, hyetograph_duration_s)?;
        let irrigation_depth_m = active_irrigation_event.map_or(0.0, |event| event.depth_m);
        let irrigation_duration_s = active_irrigation_event.map_or(0.0, |event| event.duration_s);
        let irrigation_rate_m_per_s =
            active_irrigation_event.map_or(0.0, |event| event.rate_m_per_s);

        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_RUNTIME_DEPTH_M,
            irrigation_depth_m,
            Some(0.0),
            None,
        )?;

        let coupled_rainfall_input = hyetograph_rainfall + irrigation_depth_m;
        if !coupled_rainfall_input.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                value: coupled_rainfall_input,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        if (rainfall_input - coupled_rainfall_input).abs() > closure_tolerance + WB11_ZERO_THRESHOLD
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                value: rainfall_input - coupled_rainfall_input,
                minimum: Some(-closure_tolerance),
                maximum: Some(closure_tolerance),
            });
        }

        let active_snow_coupling = Self::resolve_active_snow_coupling(request, phase_class)?;
        let snow_coupling = if active_snow_coupling {
            Self::compute_active_snow_coupling(request, phase_class, hyetograph_rainfall)?
        } else {
            SnowCouplingOutcome {
                signed_s: 0.0,
                accumulation: 0.0,
                runtime_swe: 0.0,
            }
        };
        let hyetograph_liquid_input = hyetograph_rainfall - snow_coupling.accumulation;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            hyetograph_liquid_input,
            Some(0.0),
            None,
        )?;

        let interception =
            Self::compute_canopy_interception_depth(request, phase_class, hyetograph_liquid_input)?;
        let (hyetograph_liquid_after_interception, rainfall_scale) =
            Self::resolve_interception_rainfall_scale(
                phase_class,
                hyetograph_rainfall,
                hyetograph_liquid_input,
                interception,
            )?;
        let cumulative_infiltration = Self::compute_coupled_infiltration_depth(
            phase_class,
            infiltration_conductivity,
            matric_potential,
            &times,
            &intensities,
            rainfall_scale,
            irrigation_rate_m_per_s,
            irrigation_duration_s,
        )?;
        let liquid_after_interception = hyetograph_liquid_after_interception + irrigation_depth_m;
        if !liquid_after_interception.is_finite()
            || liquid_after_interception < -WB11_ZERO_THRESHOLD
        {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
                value: liquid_after_interception,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        Self::require_infiltration_liquid_closure(
            phase_class,
            cumulative_infiltration,
            liquid_after_interception,
        )?;

        let runon_input =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_RUNON_INPUT)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RUNON_INPUT,
            runon_input,
            Some(0.0),
            None,
        )?;

        let depression_storage_delta =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_DEPRESSION_STORAGE_DELTA)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_DEPRESSION_STORAGE_DELTA,
            depression_storage_delta,
            Some(0.0),
            None,
        )?;

        let forward_solver_lane =
            Self::resolve_wb20_forward_solver_lane_enabled(request, phase_class)?;
        let runoff_snow_term = snow_coupling.signed_s + snow_coupling.accumulation;

        let q_runoff = Self::compute_runoff_after_interception(
            phase_class,
            liquid_after_interception,
            runoff_snow_term,
            runon_input,
            cumulative_infiltration,
            depression_storage_delta,
        )?;

        let closure_delta = if forward_solver_lane {
            let solver_closure = liquid_after_interception + runon_input + runoff_snow_term
                - cumulative_infiltration
                - depression_storage_delta;
            solver_closure - q_runoff
        } else {
            let runoff_observed =
                Self::require_state_scalar(request, phase_class, WB12_SYMBOL_RUNOFF_OBSERVED)?;
            Self::require_state_range(
                phase_class,
                WB12_SYMBOL_RUNOFF_OBSERVED,
                runoff_observed,
                Some(0.0),
                None,
            )?;
            q_runoff - runoff_observed
        };
        if closure_delta.abs() > closure_tolerance + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RUNOFF_CLOSURE_DELTA),
                value: closure_delta,
                minimum: Some(-closure_tolerance),
                maximum: Some(closure_tolerance),
            });
        }

        let Ok(status) = SimulationStatus::ok(
            SimulationPhase::HillslopeKernel,
            "HKERNEL-WB14-RUNOFF-OK-001",
        ) else {
            unreachable!("status message ids are non-empty WB14 constants")
        };

        let mut state_updates = vec![
            WritebackField::bounded(
                WB12_SYMBOL_INFILTRATION,
                cumulative_infiltration,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(WB12_SYMBOL_RUNOFF_RECONCILED, q_runoff, Some(0.0), None),
            WritebackField::bounded(
                IRRIG_SYMBOL_RUNTIME_SOURCE,
                active_irrigation_event.map_or(0.0, |event| event.source.as_scalar()),
                Some(0.0),
                Some(2.0),
            ),
            WritebackField::bounded(
                IRRIG_SYMBOL_RUNTIME_DEPTH_M,
                irrigation_depth_m,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                IRRIG_SYMBOL_RUNTIME_DURATION_S,
                irrigation_duration_s,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                IRRIG_SYMBOL_RUNTIME_RATE_MPS,
                irrigation_rate_m_per_s,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                IRRIG_SYMBOL_RUNTIME_EVENT_INDEX,
                active_irrigation_event.map_or(0.0, |event| {
                    Self::diagnostic_count_to_f64(event.event_index)
                }),
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                IRRIG_SYMBOL_RUNTIME_SYSTEM_TYPE,
                active_irrigation_event.map_or(0.0, |event| event.system_type),
                Some(0.0),
                Some(2.0),
            ),
        ];
        if active_snow_coupling {
            state_updates.push(WritebackField::bounded(
                WB14_SYMBOL_SNOW_RUNTIME_SWE,
                snow_coupling.runtime_swe,
                Some(0.0),
                None,
            ));
        }
        if let Some(frost_outcome) = frost_coupling {
            state_updates.push(WritebackField::bounded(
                WB14_SYMBOL_FROST_RUNTIME_DFROST,
                frost_outcome.dfrost,
                Some(0.0),
                Some(WB14_FROST_MAX_DEPTH_M),
            ));
            state_updates.push(WritebackField::bounded(
                WB14_SYMBOL_FROST_RUNTIME_DTHAW,
                frost_outcome.dthaw,
                Some(0.0),
                Some(WB14_FROST_MAX_DEPTH_M),
            ));
            state_updates.push(WritebackField::bounded(
                WB14_SYMBOL_FROST_RUNTIME_NFT,
                frost_outcome.nft,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                WB14_SYMBOL_FROST_RUNTIME_WS_FRZ,
                frost_outcome.ws_frz,
                Some(0.0),
                None,
            ));
            state_updates.push(WritebackField::bounded(
                WB14_SYMBOL_FROST_RUNTIME_INFCAP_FRZ,
                frost_outcome.infcap_frz,
                Some(0.0),
                Some(soil_conductivity),
            ));
        }

        let flux_updates = vec![
            WritebackField::bounded(
                WB15_SYMBOL_INTERCEPTION_I,
                interception,
                Some(0.0),
                Some(hyetograph_rainfall),
            ),
            WritebackField::bounded(
                IRRIG_SYMBOL_DAILY_IRRIGATION,
                irrigation_depth_m,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(WB12_SYMBOL_RUNOFF_Q, q_runoff, Some(0.0), None),
            WritebackField::unbounded(WB12_SYMBOL_RUNOFF_CLOSURE_DELTA, closure_delta),
            WritebackField::unbounded(WB12_SYMBOL_SNOW_COUPLING_S, snow_coupling.signed_s),
        ];

        let writeback = KernelWritebackPayload::with_updates(state_updates, flux_updates);
        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::too_many_lines)]
    fn run_storage_reconciliation(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyStorageReconciliation;
        let storage_initial =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_STORAGE_INITIAL)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_STORAGE_INITIAL,
            storage_initial,
            Some(0.0),
            None,
        )?;

        let forward_solver_lane =
            Self::resolve_wb20_forward_solver_lane_enabled(request, phase_class)?;

        let closure_tolerance = Self::require_state_scalar(
            request,
            phase_class,
            WB12_SYMBOL_STORAGE_CLOSURE_TOLERANCE,
        )?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_STORAGE_CLOSURE_TOLERANCE,
            closure_tolerance,
            Some(0.0),
            None,
        )?;

        let precip_input =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_PRECIP_INPUT)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_PRECIP_INPUT,
            precip_input,
            Some(0.0),
            None,
        )?;

        let q_runoff = Self::require_flux_scalar(request, phase_class, WB12_SYMBOL_RUNOFF_Q)?;
        Self::require_flux_range(phase_class, WB12_SYMBOL_RUNOFF_Q, q_runoff, Some(0.0), None)?;

        let snow_coupling_s =
            Self::require_flux_scalar(request, phase_class, WB12_SYMBOL_SNOW_COUPLING_S)?;

        let interception_i =
            Self::require_flux_scalar(request, phase_class, WB15_SYMBOL_INTERCEPTION_I)?;
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            interception_i,
            Some(0.0),
            None,
        )?;
        let irrigation_input =
            Self::optional_flux_scalar(request, phase_class, IRRIG_SYMBOL_DAILY_IRRIGATION)?
                .unwrap_or(0.0);
        Self::require_flux_range(
            phase_class,
            IRRIG_SYMBOL_DAILY_IRRIGATION,
            irrigation_input,
            Some(0.0),
            None,
        )?;

        let et = Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_ET)?;
        Self::require_flux_range(phase_class, WB11_SYMBOL_ET, et, Some(0.0), None)?;

        let percolation_loss =
            Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_PERC_LOSS_D)?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_PERC_LOSS_D,
            percolation_loss,
            Some(0.0),
            None,
        )?;

        let subsurface_loss =
            Self::require_flux_scalar(request, phase_class, WB11_SYMBOL_SUBHYD_QD)?;
        Self::require_flux_range(
            phase_class,
            WB11_SYMBOL_SUBHYD_QD,
            subsurface_loss,
            Some(0.0),
            None,
        )?;

        let storage_reconciled = Self::compute_storage_reconciled_with_interception(
            phase_class,
            storage_initial,
            precip_input,
            snow_coupling_s,
            irrigation_input,
            interception_i,
            q_runoff,
            et,
            percolation_loss,
            subsurface_loss,
        )?;

        let closure_delta = if forward_solver_lane {
            let solver_closure =
                storage_initial + precip_input + snow_coupling_s + irrigation_input
                    - interception_i
                    - q_runoff
                    - et
                    - percolation_loss
                    - subsurface_loss;
            solver_closure - storage_reconciled
        } else {
            let storage_observed =
                Self::require_state_scalar(request, phase_class, WB12_SYMBOL_STORAGE_OBSERVED)?;
            Self::require_state_range(
                phase_class,
                WB12_SYMBOL_STORAGE_OBSERVED,
                storage_observed,
                Some(0.0),
                None,
            )?;
            storage_reconciled - storage_observed
        };
        if closure_delta.abs() > closure_tolerance + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_STORAGE_CLOSURE_DELTA),
                value: closure_delta,
                minimum: Some(-closure_tolerance),
                maximum: Some(closure_tolerance),
            });
        }

        let Ok(status) = SimulationStatus::ok(
            SimulationPhase::HillslopeKernel,
            "HKERNEL-WB12-STORAGE-OK-001",
        ) else {
            unreachable!("status message ids are non-empty WB12 constants")
        };
        let writeback = KernelWritebackPayload::with_updates(
            vec![WritebackField::bounded(
                WB12_SYMBOL_STORAGE_RECONCILED,
                storage_reconciled,
                Some(0.0),
                None,
            )],
            vec![WritebackField::unbounded(
                WB12_SYMBOL_STORAGE_CLOSURE_DELTA,
                closure_delta,
            )],
        );
        Ok(KernelRunResponse::new(status, writeback))
    }

    #[allow(clippy::similar_names, clippy::too_many_lines)]
    fn run_erod13_wave1_core(
        request: &HillslopeKernelRequest<'_>,
        q_runoff: f64,
        peakro: f64,
        watdur: f64,
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        if !Self::resolve_erod13_core_enabled(request)? {
            return Ok(Vec::new());
        }

        let ie_symbol = BoundarySymbol::from(EROD13_SYMBOL_IE);
        let te_symbol = BoundarySymbol::from(EROD13_SYMBOL_TE);
        let fs_symbol = BoundarySymbol::from(EROD13_SYMBOL_FS);
        let ft_symbol = BoundarySymbol::from(EROD13_SYMBOL_FT);
        let taufe_symbol = BoundarySymbol::from(EROD13_SYMBOL_TAUFE);
        let q_symbol = BoundarySymbol::from(EROD13_SYMBOL_Q);
        let g_symbol = BoundarySymbol::from(EROD13_SYMBOL_G);
        let di_symbol = BoundarySymbol::from(EROD13_SYMBOL_DI);
        let beta_symbol = BoundarySymbol::from(EROD13_SYMBOL_BETA);
        let vf_symbol = BoundarySymbol::from(EROD13_SYMBOL_VF);
        let dgdx_symbol = BoundarySymbol::from(EROD13_SYMBOL_DGDX);
        let cntlen_symbol = BoundarySymbol::from(EROD13_SYMBOL_CNTLEN);
        let kr_symbol = BoundarySymbol::from(EROD13_SYMBOL_KR);
        let kradjf_symbol = BoundarySymbol::from(EROD13_SYMBOL_KRADJF);
        let tcadjf_symbol = BoundarySymbol::from(EROD13_SYMBOL_TCADJF);
        let shrsol_symbol = BoundarySymbol::from(EROD13_SYMBOL_SHRSOL);
        let tcend_symbol = BoundarySymbol::from(EROD13_SYMBOL_TCEND);
        let shcrit_symbol = BoundarySymbol::from(EROD13_SYMBOL_SHCRIT);
        let detinr_symbol = BoundarySymbol::from(EROD13_SYMBOL_DETINR);
        let effdrr_symbol = BoundarySymbol::from(EROD13_SYMBOL_EFFDRR);
        let effdrn_symbol = BoundarySymbol::from(EROD13_SYMBOL_EFFDRN);
        let veleff_symbol = BoundarySymbol::from(EROD13_SYMBOL_VELEFF);
        let pkro_symbol = BoundarySymbol::from(EROD13_SYMBOL_PKRO);
        let tc_k_symbol = BoundarySymbol::from(EROD13_SYMBOL_TC_K);
        let tc_m_symbol = BoundarySymbol::from(EROD13_SYMBOL_TC_M);

        let ie = Self::require_erod13_state_scalar(request, &ie_symbol)?;
        Self::require_erod13_domain(&ie_symbol, ie, Some(0.0), None)?;
        let te = Self::require_erod13_state_scalar(request, &te_symbol)?;
        Self::require_erod13_domain(&te_symbol, te, Some(WB11_ZERO_THRESHOLD), None)?;
        let fs = Self::require_erod13_state_scalar(request, &fs_symbol)?;
        Self::require_erod13_domain(&fs_symbol, fs, Some(0.0), None)?;
        let ft = Self::require_erod13_state_scalar(request, &ft_symbol)?;
        Self::require_erod13_domain(&ft_symbol, ft, Some(WB11_ZERO_THRESHOLD), None)?;
        Self::require_erod13_domain(&fs_symbol, fs, Some(0.0), Some(ft))?;
        let taufe = Self::require_erod13_state_scalar(request, &taufe_symbol)?;
        Self::require_erod13_domain(&taufe_symbol, taufe, Some(0.0), None)?;
        let q = Self::require_erod13_state_scalar(request, &q_symbol)?;
        Self::require_erod13_domain(&q_symbol, q, Some(0.0), None)?;
        let g = Self::require_erod13_state_scalar(request, &g_symbol)?;
        Self::require_erod13_domain(&g_symbol, g, Some(0.0), None)?;
        let di = Self::require_erod13_state_scalar(request, &di_symbol)?;
        Self::require_erod13_domain(&di_symbol, di, Some(0.0), None)?;
        let beta = Self::require_erod13_state_scalar(request, &beta_symbol)?;
        Self::require_erod13_domain(&beta_symbol, beta, Some(0.0), None)?;
        let vf = Self::require_erod13_state_scalar(request, &vf_symbol)?;
        Self::require_erod13_domain(&vf_symbol, vf, Some(0.0), None)?;
        let dgdx = Self::require_erod13_state_scalar(request, &dgdx_symbol)?;

        let cntlen = Self::require_erod13_state_scalar(request, &cntlen_symbol)?;
        Self::require_erod13_domain(&cntlen_symbol, cntlen, Some(WB11_ZERO_THRESHOLD), None)?;
        let kr = Self::require_erod13_state_scalar(request, &kr_symbol)?;
        Self::require_erod13_domain(&kr_symbol, kr, Some(WB11_ZERO_THRESHOLD), None)?;
        let kradjf = Self::require_erod13_state_scalar(request, &kradjf_symbol)?;
        Self::require_erod13_domain(&kradjf_symbol, kradjf, Some(WB11_ZERO_THRESHOLD), None)?;
        let tcadjf = Self::require_erod13_state_scalar(request, &tcadjf_symbol)?;
        Self::require_erod13_domain(&tcadjf_symbol, tcadjf, Some(EROD13_MIN_TCADJF), None)?;
        let shrsol = Self::require_erod13_state_scalar(request, &shrsol_symbol)?;
        Self::require_erod13_domain(&shrsol_symbol, shrsol, Some(WB11_ZERO_THRESHOLD), None)?;
        let tcend = Self::require_erod13_state_scalar(request, &tcend_symbol)?;
        Self::require_erod13_domain(&tcend_symbol, tcend, Some(WB11_ZERO_THRESHOLD), None)?;
        let shcrit = Self::require_erod13_state_scalar(request, &shcrit_symbol)?;
        Self::require_erod13_domain(&shcrit_symbol, shcrit, Some(0.0), None)?;
        let detinr = Self::require_erod13_state_scalar(request, &detinr_symbol)?;
        Self::require_erod13_domain(&detinr_symbol, detinr, Some(0.0), None)?;
        let effdrr = Self::require_erod13_state_scalar(request, &effdrr_symbol)?;
        Self::require_erod13_domain(&effdrr_symbol, effdrr, Some(WB11_ZERO_THRESHOLD), None)?;
        let effdrn = Self::require_erod13_state_scalar(request, &effdrn_symbol)?;
        Self::require_erod13_domain(&effdrn_symbol, effdrn, Some(WB11_ZERO_THRESHOLD), None)?;
        let veleff = Self::require_erod13_state_scalar(request, &veleff_symbol)?;
        Self::require_erod13_domain(&veleff_symbol, veleff, Some(0.0), None)?;
        let pkro = Self::require_erod13_state_scalar(request, &pkro_symbol)?;
        Self::require_erod13_domain(&pkro_symbol, pkro, Some(WB11_ZERO_THRESHOLD), None)?;
        let tc_k = Self::require_erod13_state_scalar(request, &tc_k_symbol)?;
        Self::require_erod13_domain(&tc_k_symbol, tc_k, Some(WB11_ZERO_THRESHOLD), None)?;
        let tc_m = Self::require_erod13_state_scalar(request, &tc_m_symbol)?;
        Self::require_erod13_domain(&tc_m_symbol, tc_m, Some(WB11_ZERO_THRESHOLD), None)?;

        Self::require_erod13_domain(
            &BoundarySymbol::from(WB12_SYMBOL_RUNOFF_Q),
            q_runoff,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_erod13_domain(
            &BoundarySymbol::from(WB16_SYMBOL_PEAKRO),
            peakro,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_erod13_domain(
            &BoundarySymbol::from(WB16_SYMBOL_WATDUR),
            watdur,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        let expected_watdur = q_runoff / peakro;
        let continuity_residual = (watdur - expected_watdur).abs();
        if continuity_residual > EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(WB16_SYMBOL_WATDUR),
                value: watdur,
                minimum: Some(expected_watdur - EROD13_CONTINUITY_TOLERANCE),
                maximum: Some(expected_watdur + EROD13_CONTINUITY_TOLERANCE),
            });
        }

        let tau_f = taufe * (fs / ft);
        if !tau_f.is_finite() || tau_f < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: taufe_symbol.clone(),
                value: tau_f,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let eta = (cntlen * kr * kradjf * shrsol) / tcend;
        if !eta.is_finite() || eta < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_ETA),
                value: eta,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let taucn = (tcadjf * shcrit) / shrsol;
        if !taucn.is_finite() || taucn < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_TAUCN),
                value: taucn,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let theta = ((cntlen * detinr) / tcend) * (effdrr / effdrn);
        if !theta.is_finite() || theta < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_THETA),
                value: theta,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let phi = (beta * veleff) / pkro;
        if !phi.is_finite() || phi < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_PHI),
                value: phi,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let tc = tcadjf * tc_k * tau_f.powf(tc_m);
        if !tc.is_finite() || tc < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: BoundarySymbol::from(EROD13_SYMBOL_TC),
                value: tc,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let (dc, df) = if tau_f > taucn && g < tc {
            if tc <= WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: BoundarySymbol::from(EROD13_SYMBOL_TC),
                    value: tc,
                    minimum: Some(WB11_ZERO_THRESHOLD),
                    maximum: None,
                });
            }
            let dc = eta * (tau_f - taucn);
            if !dc.is_finite() || dc < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: BoundarySymbol::from(EROD13_SYMBOL_DC),
                    value: dc,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            let df = dc * ((tc - g) / tc);
            if !df.is_finite() || df < -WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: BoundarySymbol::from(EROD13_SYMBOL_DF),
                    value: df,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            (dc, df)
        } else if g > tc {
            Self::require_erod13_domain(&q_symbol, q, Some(WB11_ZERO_THRESHOLD), None)?;
            let df = -((beta * vf / q) * (g - tc));
            if !df.is_finite() || df > WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                    symbol: BoundarySymbol::from(EROD13_SYMBOL_DF),
                    value: df,
                    minimum: None,
                    maximum: Some(0.0),
                });
            }
            (0.0, df)
        } else {
            (0.0, 0.0)
        };

        let expected_dgdx = df + di;
        let dgdx_residual = (dgdx - expected_dgdx).abs();
        if dgdx_residual > EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod13DomainViolation {
                symbol: dgdx_symbol,
                value: dgdx,
                minimum: Some(expected_dgdx - EROD13_CONTINUITY_TOLERANCE),
                maximum: Some(expected_dgdx + EROD13_CONTINUITY_TOLERANCE),
            });
        }

        Ok(vec![
            WritebackField::bounded(EROD13_SYMBOL_DC, dc, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_TC, tc, Some(0.0), None),
            WritebackField::unbounded(EROD13_SYMBOL_DF, df),
            WritebackField::bounded(EROD13_SYMBOL_ETA, eta, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_TAUCN, taucn, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_THETA, theta, Some(0.0), None),
            WritebackField::bounded(EROD13_SYMBOL_PHI, phi, Some(0.0), None),
        ])
    }

    #[allow(clippy::too_many_lines)]
    fn run_erod14_wave2(
        request: &HillslopeKernelRequest<'_>,
        erod13_state_updates: &[WritebackField],
    ) -> Result<Vec<WritebackField>, Wb11HydrologyKernelGuardError> {
        if !Self::resolve_erod14_wave2_enabled(request)? {
            return Ok(Vec::new());
        }

        let class_count_symbol = BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT);
        let class_count_value = Self::require_erod14_state_scalar(request, &class_count_symbol)?;
        if class_count_value < 1.0 - WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: class_count_symbol,
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let class_count_rounded = class_count_value.round();
        if (class_count_value - class_count_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT),
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let class_count = format!("{class_count_rounded:.0}")
            .parse::<usize>()
            .map_err(|_| Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT),
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            })?;
        if class_count == 0 {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT),
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            });
        }
        let class_count_f64 = f64::from(u32::try_from(class_count).map_err(|_| {
            Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CLASS_COUNT),
                value: class_count_value,
                minimum: Some(1.0),
                maximum: None,
            }
        })?);

        let xtop_symbol = BoundarySymbol::from(EROD14_SYMBOL_XTOP);
        let xbot_symbol = BoundarySymbol::from(EROD14_SYMBOL_XBOT);
        let xdetst_symbol = BoundarySymbol::from(EROD14_SYMBOL_XDETST);
        let ldtop_symbol = BoundarySymbol::from(EROD14_SYMBOL_LDTOP);
        let ldbot_symbol = BoundarySymbol::from(EROD14_SYMBOL_LDBOT);
        let lddend_symbol = BoundarySymbol::from(EROD14_SYMBOL_LDDEND);
        let qout_symbol = BoundarySymbol::from(EROD14_SYMBOL_QOUT);
        let qin_symbol = BoundarySymbol::from(EROD14_SYMBOL_QIN);
        let qostar_symbol = BoundarySymbol::from(EROD14_SYMBOL_QOSTAR);
        let slplen_symbol = BoundarySymbol::from(EROD14_SYMBOL_SLP_LEN);
        let ktrato_symbol = BoundarySymbol::from(EROD14_SYMBOL_KTRATO);
        let aintc_symbol = BoundarySymbol::from(EROD14_SYMBOL_AINTC);
        let bintc_symbol = BoundarySymbol::from(EROD14_SYMBOL_BINTC);
        let cintc_symbol = BoundarySymbol::from(EROD14_SYMBOL_CINTC);
        let beta_symbol = BoundarySymbol::from(EROD14_SYMBOL_BETA);
        let qj_minus_1_symbol = BoundarySymbol::from(EROD14_SYMBOL_QJ_MINUS_1);
        let vj_symbol = BoundarySymbol::from(EROD14_SYMBOL_VJ);
        let qj_symbol = BoundarySymbol::from(EROD14_SYMBOL_QJ);
        let fh_runon_symbol = BoundarySymbol::from(EROD14_SYMBOL_FH);
        let fp_potential_symbol = BoundarySymbol::from(EROD14_SYMBOL_FP);
        let case_symbol = BoundarySymbol::from(EROD14_SYMBOL_CASE);
        let sumg_symbol = BoundarySymbol::from(EROD14_SYMBOL_SUMG);
        let er_symbol = BoundarySymbol::from(EROD14_SYMBOL_ER);
        let ssa_soil_symbol = BoundarySymbol::from(EROD14_SYMBOL_SSA_SOIL);

        let xtop = Self::require_erod14_state_scalar(request, &xtop_symbol)?;
        let xbot = Self::require_erod14_state_scalar(request, &xbot_symbol)?;
        let xdetst = Self::require_erod14_state_scalar(request, &xdetst_symbol)?;
        let ldtop = Self::require_erod14_state_scalar(request, &ldtop_symbol)?;
        let ldbot = Self::require_erod14_state_scalar(request, &ldbot_symbol)?;
        let lddend = Self::require_erod14_state_scalar(request, &lddend_symbol)?;
        let qout = Self::require_erod14_state_scalar(request, &qout_symbol)?;
        let qin = Self::require_erod14_state_scalar(request, &qin_symbol)?;
        let qostar = Self::require_erod14_state_scalar(request, &qostar_symbol)?;
        let slplen = Self::require_erod14_state_scalar(request, &slplen_symbol)?;
        let ktrato = Self::require_erod14_state_scalar(request, &ktrato_symbol)?;
        let aintc = Self::require_erod14_state_scalar(request, &aintc_symbol)?;
        let bintc = Self::require_erod14_state_scalar(request, &bintc_symbol)?;
        let cintc = Self::require_erod14_state_scalar(request, &cintc_symbol)?;
        let beta = Self::require_erod14_state_scalar(request, &beta_symbol)?;
        let qj_minus_1 = Self::require_erod14_state_scalar(request, &qj_minus_1_symbol)?;
        let vj = Self::require_erod14_state_scalar(request, &vj_symbol)?;
        let qj = Self::require_erod14_state_scalar(request, &qj_symbol)?;
        let fh = Self::require_erod14_state_scalar(request, &fh_runon_symbol)?;
        let fp = Self::require_erod14_state_scalar(request, &fp_potential_symbol)?;
        let case_value = Self::require_erod14_state_scalar(request, &case_symbol)?;
        let ssa_soil = Self::require_erod14_state_scalar(request, &ssa_soil_symbol)?;

        Self::require_erod14_domain(&xtop_symbol, xtop, Some(0.0), None)?;
        Self::require_erod14_domain(&xbot_symbol, xbot, Some(xtop), None)?;
        Self::require_erod14_domain(&xdetst_symbol, xdetst, Some(0.0), Some(xtop))?;
        Self::require_erod14_domain(&ldtop_symbol, ldtop, Some(0.0), None)?;
        Self::require_erod14_domain(&ldbot_symbol, ldbot, Some(0.0), None)?;
        Self::require_erod14_domain(&lddend_symbol, lddend, Some(0.0), Some(ldtop))?;
        Self::require_erod14_domain(&qout_symbol, qout, Some(0.0), None)?;
        Self::require_erod14_domain(&qin_symbol, qin, Some(0.0), None)?;
        Self::require_erod14_domain(&slplen_symbol, slplen, Some(WB11_ZERO_THRESHOLD), None)?;
        Self::require_erod14_domain(&ktrato_symbol, ktrato, Some(WB11_ZERO_THRESHOLD), None)?;
        Self::require_erod14_domain(&beta_symbol, beta, Some(0.0), None)?;
        Self::require_erod14_domain(&qj_minus_1_symbol, qj_minus_1, Some(0.0), None)?;
        Self::require_erod14_domain(&vj_symbol, vj, Some(0.0), None)?;
        Self::require_erod14_domain(&qj_symbol, qj, Some(0.0), None)?;
        Self::require_erod14_domain(&fh_runon_symbol, fh, Some(0.0), None)?;
        Self::require_erod14_domain(&fp_potential_symbol, fp, Some(0.0), None)?;
        Self::require_erod14_domain(&ssa_soil_symbol, ssa_soil, Some(WB11_ZERO_THRESHOLD), None)?;

        let case_rounded = case_value.round();
        if (case_value - case_rounded).abs() > WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: case_symbol,
                value: case_value,
                minimum: Some(1.0),
                maximum: Some(4.0),
            });
        }
        let case_number = format!("{case_rounded:.0}").parse::<i32>().map_err(|_| {
            Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CASE),
                value: case_value,
                minimum: Some(1.0),
                maximum: Some(4.0),
            }
        })?;
        if !(1..=4).contains(&case_number) {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CASE),
                value: case_value,
                minimum: Some(1.0),
                maximum: Some(4.0),
            });
        }

        let case_is_zero = |value: f64| value.abs() <= EROD14_CASE_TOLERANCE;
        let case_matches = match case_number {
            1 => case_is_zero(qj_minus_1) && case_is_zero(vj) && case_is_zero(qj),
            2 => {
                qj_minus_1 > EROD14_CASE_TOLERANCE
                    && vj > EROD14_CASE_TOLERANCE
                    && qj > EROD14_CASE_TOLERANCE
            }
            3 => {
                qj_minus_1 > EROD14_CASE_TOLERANCE
                    && case_is_zero(vj)
                    && (fh - fp) > EROD14_CASE_TOLERANCE
                    && qj > EROD14_CASE_TOLERANCE
            }
            4 => {
                qj_minus_1 > EROD14_CASE_TOLERANCE
                    && case_is_zero(vj)
                    && (fh - fp) <= EROD14_CASE_TOLERANCE
                    && case_is_zero(qj)
            }
            _ => false,
        };
        if !case_matches {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_CASE),
                value: case_value,
                minimum: Some(1.0),
                maximum: Some(4.0),
            });
        }

        let theta_symbol = BoundarySymbol::from(EROD13_SYMBOL_THETA);
        let theta = if let Some(value) =
            Self::extract_state_update_scalar(erod13_state_updates, EROD13_SYMBOL_THETA)
        {
            if !value.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod14NonFiniteSymbol {
                    symbol: theta_symbol,
                    value,
                });
            }
            value
        } else {
            Self::require_erod14_state_scalar(request, &theta_symbol)?
        };
        Self::require_erod14_domain(&theta_symbol, theta, Some(0.0), None)?;

        let mut fall = Vec::with_capacity(class_count);
        let mut frcflw = Vec::with_capacity(class_count);
        let mut fidel = Vec::with_capacity(class_count);
        let mut tcf1 = Vec::with_capacity(class_count);
        let mut ssa_class = Vec::with_capacity(class_count);
        let mut ftheta = Vec::with_capacity(class_count);
        let mut gu = Vec::with_capacity(class_count);
        let mut gend = vec![0.0_f64; class_count];
        let mut sedmax = vec![0.0_f64; class_count];
        let mut sed_frac = vec![0.0_f64; class_count];

        for class_index in 1..=class_count {
            let fall_symbol = Self::erod14_class_symbol(EROD14_ROOT_FALL, class_index);
            let frcflw_symbol = Self::erod14_class_symbol(EROD14_ROOT_FRCFLW, class_index);
            let frac_symbol = Self::erod14_class_symbol(EROD14_ROOT_FRAC, class_index);
            let fidel_symbol = Self::erod14_class_symbol(EROD14_ROOT_FIDEL, class_index);
            let tcf1_symbol = Self::erod14_class_symbol(EROD14_ROOT_TCF1, class_index);
            let ssa_class_symbol = Self::erod14_class_symbol(EROD14_ROOT_SSA_CLASS, class_index);

            let fall_value = Self::require_erod14_state_scalar(request, &fall_symbol)?;
            let frcflw_value = Self::require_erod14_state_scalar(request, &frcflw_symbol)?;
            let frac_value = Self::require_erod14_state_scalar(request, &frac_symbol)?;
            let fidel_value = Self::require_erod14_state_scalar(request, &fidel_symbol)?;
            let tcf1_value = Self::require_erod14_state_scalar(request, &tcf1_symbol)?;
            let ssa_class_value = Self::require_erod14_state_scalar(request, &ssa_class_symbol)?;

            Self::require_erod14_domain(&fall_symbol, fall_value, Some(0.0), None)?;
            Self::require_erod14_domain(&frcflw_symbol, frcflw_value, Some(0.0), Some(1.0))?;
            Self::require_erod14_domain(&frac_symbol, frac_value, Some(0.0), Some(1.0))?;
            Self::require_erod14_domain(&fidel_symbol, fidel_value, Some(0.0), Some(1.0))?;
            Self::require_erod14_domain(&tcf1_symbol, tcf1_value, Some(0.0), None)?;
            Self::require_erod14_domain(
                &ssa_class_symbol,
                ssa_class_value,
                Some(WB11_ZERO_THRESHOLD),
                None,
            )?;

            fall.push(fall_value);
            frcflw.push(frcflw_value);
            fidel.push(fidel_value);
            tcf1.push(tcf1_value);
            ssa_class.push(ssa_class_value);
            ftheta.push(fidel_value * theta);
            gu.push(frcflw_value * ldtop);
        }

        if qout <= WB11_ZERO_THRESHOLD {
            for i in 0..class_count {
                frcflw[i] = 0.0;
                sed_frac[i] = 0.0;
            }
            let mut updates = Vec::with_capacity(5 + (class_count * 6));
            updates.push(WritebackField::bounded(
                EROD14_SYMBOL_SUMG,
                0.0,
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                EROD14_SYMBOL_ER,
                0.0,
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                EROD15_SYMBOL_TOTAL_DETACHMENT_KG,
                0.0,
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                EROD15_SYMBOL_TOTAL_DEPOSITION_KG,
                lddend.max(0.0),
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                EROD15_SYMBOL_PARTICLE_CLASS_COUNT,
                class_count_f64,
                Some(1.0),
                None,
            ));
            for class_index in 1..=class_count {
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD14_ROOT_GEND, class_index),
                    0.0,
                    Some(0.0),
                    None,
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD14_ROOT_SEDMAX, class_index),
                    0.0,
                    Some(0.0),
                    None,
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD14_ROOT_FRCFLW, class_index),
                    0.0,
                    Some(0.0),
                    Some(1.0),
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD14_ROOT_SED_FRAC, class_index),
                    0.0,
                    Some(0.0),
                    Some(1.0),
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(
                        EROD15_ROOT_SEDIMENT_CONCENTRATION_KG_M3,
                        class_index,
                    ),
                    0.0,
                    Some(0.0),
                    None,
                ));
                updates.push(WritebackField::bounded(
                    Self::erod14_class_symbol(EROD15_ROOT_PARTICLE_FLOW_FRACTION, class_index),
                    0.0,
                    Some(0.0),
                    Some(1.0),
                ));
            }
            return Ok(updates);
        }

        let pkro = (qout - qin) / slplen;
        if !pkro.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: BoundarySymbol::from(EROD14_SYMBOL_QOUT),
                value: pkro,
                minimum: None,
                maximum: None,
            });
        }

        let tmpvr2 = xbot + qostar;
        let tmpvr3 = xtop + qostar;
        if tmpvr2.abs() <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: qostar_symbol,
                value: tmpvr2,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        let tmpvr4 = tmpvr2 * tmpvr2;
        let tmpvr5 = tmpvr3 * tmpvr3;

        let mut sumg = 0.0_f64;
        for i in 0..class_count {
            let tmpvr1 = ktrato * tcf1[i];
            let aa = tmpvr1 * aintc;
            let bb = tmpvr1 * bintc;
            let cc = tmpvr1 * cintc;

            let mut phi = if pkro.abs() > EROD14_PKRO_ZERO_THRESHOLD {
                (beta * fall[i]) / pkro
            } else if qostar >= 0.0 {
                EROD14_MAX_PHI
            } else {
                -EROD14_MAX_PHI
            };
            phi = phi.clamp(-EROD14_MAX_PHI, EROD14_MAX_PHI);

            let mut ratio = tmpvr3 / tmpvr2;
            if qostar >= 0.0 && ratio > 1.0 {
                ratio = 1.0;
            }
            if ratio < 0.0 {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: BoundarySymbol::from(EROD14_SYMBOL_QOSTAR),
                    value: ratio,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }

            let denom_coef1 = phi + 2.0;
            let denom_coef2 = phi + 1.0;
            if denom_coef1.abs() <= WB11_ZERO_THRESHOLD || denom_coef2.abs() <= WB11_ZERO_THRESHOLD
            {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(EROD14_ROOT_FALL, i + 1),
                    value: phi,
                    minimum: Some(-EROD14_MAX_PHI),
                    maximum: Some(EROD14_MAX_PHI),
                });
            }

            let mut attenuation_factor = ratio.powf(phi);
            if !attenuation_factor.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(EROD14_ROOT_FALL, i + 1),
                    value: attenuation_factor,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            if attenuation_factor < 1.0e-8 {
                attenuation_factor = 0.0;
            }

            let coef1 = phi * aa / denom_coef1;
            let coef2 = (phi * bb + ftheta[i] - 2.0 * aa * phi * qostar) / denom_coef2;
            let term1 = coef1 * tmpvr4;
            let term2 = coef2 * tmpvr2;
            let term3 = aa * qostar * qostar - bb * qostar + cc;
            let attenuation_tail = gu[i] - coef1 * tmpvr5 - coef2 * tmpvr3 - term3;
            let mut gend_i = term1 + term2 + term3 + attenuation_factor * attenuation_tail;
            if !gend_i.is_finite() {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(EROD14_ROOT_GEND, i + 1),
                    value: gend_i,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            if gend_i < 0.0 {
                gend_i = 0.0;
            }
            gend[i] = gend_i;
            sumg += gend_i;
        }

        if sumg > 0.0 {
            for i in 0..class_count {
                gend[i] = gend[i] * ldbot / sumg;
                sedmax[i] = gu[i] + ftheta[i] * (xbot - xtop);
                Self::require_erod14_domain(
                    &Self::erod14_class_symbol(EROD14_ROOT_SEDMAX, i + 1),
                    sedmax[i],
                    Some(0.0),
                    None,
                )?;
                if gend[i] < EROD14_CLASS_FLOOR {
                    gend[i] = EROD14_CLASS_FLOOR;
                }
            }

            let mut converged = false;
            for _ in 0..EROD14_MAX_REPROPORTION_ITERS {
                let mut ratbot = 0.0_f64;
                sumg = 0.0;
                let mut adjusted = false;

                for i in 0..class_count {
                    if gend[i] > sedmax[i] + WB11_ZERO_THRESHOLD {
                        gend[i] = sedmax[i];
                        adjusted = true;
                    } else if gend[i] < sedmax[i] - WB11_ZERO_THRESHOLD {
                        ratbot += gend[i];
                    }
                    sumg += gend[i];
                }

                if !adjusted {
                    converged = true;
                    break;
                }

                if ratbot <= WB11_ZERO_THRESHOLD {
                    return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                        symbol: BoundarySymbol::from(EROD14_SYMBOL_LDBOT),
                        value: ratbot,
                        minimum: Some(WB11_ZERO_THRESHOLD),
                        maximum: None,
                    });
                }

                let gdeficit = ldbot - sumg;
                for i in 0..class_count {
                    if gend[i] < sedmax[i] - WB11_ZERO_THRESHOLD {
                        let gadd = gdeficit * gend[i] / ratbot;
                        let updated = gend[i] + gadd;
                        if !updated.is_finite() {
                            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                                symbol: Self::erod14_class_symbol(EROD14_ROOT_GEND, i + 1),
                                value: updated,
                                minimum: Some(0.0),
                                maximum: None,
                            });
                        }
                        gend[i] = updated;
                    }
                }
            }

            if !converged {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: BoundarySymbol::from(EROD14_SYMBOL_LDBOT),
                    value: ldbot,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
        }

        sumg = gend.iter().sum();
        if !sumg.is_finite() || sumg < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: sumg_symbol,
                value: sumg,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        for i in 0..class_count {
            if gend[i] > sedmax[i] + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(EROD14_ROOT_GEND, i + 1),
                    value: gend[i],
                    minimum: Some(0.0),
                    maximum: Some(sedmax[i]),
                });
            }
        }

        if sumg > 0.0 {
            for i in 0..class_count {
                frcflw[i] = gend[i] / sumg;
                sed_frac[i] = frcflw[i];
            }
            let sed_frac_sum: f64 = sed_frac.iter().sum();
            if (sed_frac_sum - 1.0).abs() > EROD13_CONTINUITY_TOLERANCE + WB11_ZERO_THRESHOLD {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: BoundarySymbol::from(EROD14_ROOT_SED_FRAC),
                    value: sed_frac_sum,
                    minimum: Some(1.0 - EROD13_CONTINUITY_TOLERANCE),
                    maximum: Some(1.0 + EROD13_CONTINUITY_TOLERANCE),
                });
            }
        } else {
            for i in 0..class_count {
                frcflw[i] = 0.0;
                sed_frac[i] = 0.0;
            }
        }

        let mut sumssa = 0.0_f64;
        for i in 0..class_count {
            sumssa += sed_frac[i] * ssa_class[i];
        }
        let er = if sumg > 0.0 {
            (sumssa / ssa_soil) + 0.005
        } else {
            0.0
        };
        if !er.is_finite() || er < -WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                symbol: er_symbol,
                value: er,
                minimum: Some(0.0),
                maximum: None,
            });
        }

        let mut updates = Vec::with_capacity(5 + (class_count * 6));
        updates.push(WritebackField::bounded(
            EROD14_SYMBOL_SUMG,
            sumg.max(0.0),
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            EROD14_SYMBOL_ER,
            er,
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            EROD15_SYMBOL_TOTAL_DETACHMENT_KG,
            sumg.max(0.0),
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            EROD15_SYMBOL_TOTAL_DEPOSITION_KG,
            lddend.max(0.0),
            Some(0.0),
            None,
        ));
        updates.push(WritebackField::bounded(
            EROD15_SYMBOL_PARTICLE_CLASS_COUNT,
            class_count_f64,
            Some(1.0),
            None,
        ));

        for class_index in 1..=class_count {
            let i = class_index - 1;
            let concentration = if qout > WB11_ZERO_THRESHOLD {
                gend[i] / qout
            } else {
                0.0
            };
            if !concentration.is_finite() || concentration < 0.0 {
                return Err(Wb11HydrologyKernelGuardError::Erod14DomainViolation {
                    symbol: Self::erod14_class_symbol(
                        EROD15_ROOT_SEDIMENT_CONCENTRATION_KG_M3,
                        class_index,
                    ),
                    value: concentration,
                    minimum: Some(0.0),
                    maximum: None,
                });
            }
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD14_ROOT_GEND, class_index),
                gend[i],
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD14_ROOT_SEDMAX, class_index),
                sedmax[i],
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD14_ROOT_FRCFLW, class_index),
                frcflw[i],
                Some(0.0),
                Some(1.0),
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD14_ROOT_SED_FRAC, class_index),
                sed_frac[i],
                Some(0.0),
                Some(1.0),
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD15_ROOT_SEDIMENT_CONCENTRATION_KG_M3, class_index),
                concentration,
                Some(0.0),
                None,
            ));
            updates.push(WritebackField::bounded(
                Self::erod14_class_symbol(EROD15_ROOT_PARTICLE_FLOW_FRACTION, class_index),
                sed_frac[i],
                Some(0.0),
                Some(1.0),
            ));
        }

        Ok(updates)
    }

    #[allow(clippy::too_many_lines)]
    fn run_peak_runoff(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyPeakRunoff;

        let q_runoff = Self::require_flux_scalar(request, phase_class, WB12_SYMBOL_RUNOFF_Q)?;
        Self::require_flux_range(phase_class, WB12_SYMBOL_RUNOFF_Q, q_runoff, Some(0.0), None)?;
        if q_runoff <= WB11_ZERO_THRESHOLD {
            let wb11_soil_water =
                Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
            Self::require_state_range(
                phase_class,
                WB11_SYMBOL_SOIL_WATER,
                wb11_soil_water,
                Some(0.0),
                None,
            )?;
            let watcon = wb11_soil_water;
            let total_soil = watcon * WB13_DEPTH_TO_MM;
            let soil_water_total = total_soil;

            let Ok(status) = SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HKERNEL-WB16-PEAK-ZERO-001",
            ) else {
                unreachable!("status message ids are non-empty WB16 constants")
            };

            let writeback = KernelWritebackPayload::with_updates(
                vec![
                    WritebackField::bounded(
                        WB16_SYMBOL_PEAKRO,
                        WB16_PEAKRO_FLOOR,
                        Some(WB16_PEAKRO_FLOOR),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_WATDUR,
                        0.0,
                        Some(0.0),
                        Some(WB16_MAX_DURATION_S),
                    ),
                    WritebackField::bounded(WB16_SYMBOL_METHOD_BRANCH, 1.0, Some(1.0), Some(3.0)),
                    WritebackField::bounded(
                        WB16_SYMBOL_TSTAR,
                        WB11_ZERO_THRESHOLD,
                        Some(WB11_ZERO_THRESHOLD),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_QPSTAR,
                        WB11_ZERO_THRESHOLD,
                        Some(WB11_ZERO_THRESHOLD),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_VSTAR,
                        WB11_ZERO_THRESHOLD,
                        Some(WB11_ZERO_THRESHOLD),
                        Some(1.0),
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_WATCON),
                        watcon,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_TOTAL_SOIL),
                        total_soil,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_SOIL_WATER_TOTAL),
                        soil_water_total,
                        Some(0.0),
                        None,
                    ),
                ],
                Vec::new(),
            );
            return Ok(KernelRunResponse::new(status, writeback));
        }

        let hyetograph_point_count = Self::resolve_hyetograph_point_count(request, phase_class)?;
        let (hyetograph_times, hyetograph_intensities) =
            Self::load_hyetograph_series(request, phase_class, hyetograph_point_count)?;
        let effdrr = if hyetograph_times.len() >= 2 {
            hyetograph_times[hyetograph_times.len() - 1] - hyetograph_times[0]
        } else {
            0.0
        };
        if !effdrr.is_finite() || effdrr <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("timem_0001"),
                value: effdrr,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let vave = q_runoff / effdrr;
        if !vave.is_finite() || vave <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RUNOFF_Q),
                value: vave,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let irrigation_rate_m_per_s =
            Self::require_state_scalar(request, phase_class, IRRIG_SYMBOL_RUNTIME_RATE_MPS)?;
        Self::require_state_range(
            phase_class,
            IRRIG_SYMBOL_RUNTIME_RATE_MPS,
            irrigation_rate_m_per_s,
            Some(0.0),
            None,
        )?;

        let interception_i =
            Self::require_flux_scalar(request, phase_class, WB15_SYMBOL_INTERCEPTION_I)?;
        Self::require_flux_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            interception_i,
            Some(0.0),
            None,
        )?;

        let timep = Self::require_state_scalar(request, phase_class, WB16_SYMBOL_TIMEP)?;
        Self::require_state_range(phase_class, WB16_SYMBOL_TIMEP, timep, Some(0.0), Some(1.0))?;

        let efflen = Self::require_state_scalar(request, phase_class, WB16_SYMBOL_EFFLEN)?;
        if efflen <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_EFFLEN),
                value: efflen,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let ealpha = Self::require_state_scalar(request, phase_class, WB16_SYMBOL_EALPHA)?;
        if ealpha <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_EALPHA),
                value: ealpha,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let exponent_m = Self::require_state_scalar(request, phase_class, WB16_SYMBOL_EXPONENT_M)?;
        if exponent_m <= 1.0 + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_EXPONENT_M),
                value: exponent_m,
                minimum: Some(1.0 + WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let remax = hyetograph_intensities
            .iter()
            .copied()
            .fold(0.0_f64, f64::max)
            + irrigation_rate_m_per_s;
        if !remax.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("intsty_0001"),
                value: remax,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }
        if remax <= WB11_ZERO_THRESHOLD {
            let wb11_soil_water =
                Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
            Self::require_state_range(
                phase_class,
                WB11_SYMBOL_SOIL_WATER,
                wb11_soil_water,
                Some(0.0),
                None,
            )?;
            let watcon = wb11_soil_water;
            let total_soil = watcon * WB13_DEPTH_TO_MM;
            let soil_water_total = total_soil;

            let Ok(status) = SimulationStatus::ok(
                SimulationPhase::HillslopeKernel,
                "HKERNEL-WB16-PEAK-ZERO-002",
            ) else {
                unreachable!("status message ids are non-empty WB16 constants")
            };

            let writeback = KernelWritebackPayload::with_updates(
                vec![
                    WritebackField::bounded(
                        WB16_SYMBOL_PEAKRO,
                        WB16_PEAKRO_FLOOR,
                        Some(WB16_PEAKRO_FLOOR),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_WATDUR,
                        0.0,
                        Some(0.0),
                        Some(WB16_MAX_DURATION_S),
                    ),
                    WritebackField::bounded(WB16_SYMBOL_METHOD_BRANCH, 1.0, Some(1.0), Some(3.0)),
                    WritebackField::bounded(
                        WB16_SYMBOL_TSTAR,
                        WB11_ZERO_THRESHOLD,
                        Some(WB11_ZERO_THRESHOLD),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_QPSTAR,
                        WB11_ZERO_THRESHOLD,
                        Some(WB11_ZERO_THRESHOLD),
                        None,
                    ),
                    WritebackField::bounded(
                        WB16_SYMBOL_VSTAR,
                        WB11_ZERO_THRESHOLD,
                        Some(WB11_ZERO_THRESHOLD),
                        Some(1.0),
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_WATCON),
                        watcon,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_TOTAL_SOIL),
                        total_soil,
                        Some(0.0),
                        None,
                    ),
                    WritebackField::bounded(
                        BoundarySymbol::from(WB13_STATE_SYMBOL_SOIL_WATER_TOTAL),
                        soil_water_total,
                        Some(0.0),
                        None,
                    ),
                ],
                Vec::new(),
            );
            return Ok(KernelRunResponse::new(status, writeback));
        }

        let vstar = vave / remax;
        if !vstar.is_finite() || vstar <= WB11_ZERO_THRESHOLD || vstar > 1.0 + WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_VSTAR),
                value: vstar,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: Some(1.0),
            });
        }

        let vave_power = vave.powf(exponent_m - 1.0);
        let te_base = efflen / (ealpha * vave_power);
        if !te_base.is_finite() || te_base <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_EFFLEN),
                value: te_base,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let te = te_base.powf(1.0 / exponent_m);
        let tstar = te / effdrr;
        if !tstar.is_finite() || tstar <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_TSTAR),
                value: tstar,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let (method_branch, qpstar) = if tstar >= 1.0 {
            (1.0, 1.0 / tstar.powf(exponent_m))
        } else if tstar > timep {
            (2.0, 1.0 / tstar)
        } else {
            (3.0, (1.0 / vstar) - 0.6 * (((1.0 - vstar) / vstar) * tstar))
        };
        if !qpstar.is_finite() || qpstar <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_QPSTAR),
                value: qpstar,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let peakro_raw = vave * qpstar;
        if !peakro_raw.is_finite() || peakro_raw <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_PEAKRO),
                value: peakro_raw,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let peakro = peakro_raw.max(WB16_PEAKRO_FLOOR);
        if !peakro.is_finite() || peakro <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_PEAKRO),
                value: peakro,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
        }

        let watdur_raw = q_runoff / peakro;
        if !watdur_raw.is_finite() || watdur_raw < 0.0 {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB16_SYMBOL_WATDUR),
                value: watdur_raw,
                minimum: Some(0.0),
                maximum: None,
            });
        }
        let watdur = watdur_raw.min(WB16_MAX_DURATION_S);

        let wb11_soil_water =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_SOIL_WATER)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            wb11_soil_water,
            Some(0.0),
            None,
        )?;
        let watcon = wb11_soil_water;
        let total_soil = watcon * WB13_DEPTH_TO_MM;
        let soil_water_total = total_soil;

        let erod13_state_updates = Self::run_erod13_wave1_core(request, q_runoff, peakro, watdur)?;
        let erod14_state_updates = Self::run_erod14_wave2(request, &erod13_state_updates)?;
        let status_message_id = if !erod14_state_updates.is_empty() {
            "HKERNEL-EROD14-WAVE2-OK-001"
        } else if !erod13_state_updates.is_empty() {
            "HKERNEL-EROD13-CORE-OK-001"
        } else {
            "HKERNEL-WB16-PEAK-OK-001"
        };

        let Ok(status) = SimulationStatus::ok(SimulationPhase::HillslopeKernel, status_message_id)
        else {
            unreachable!("status message ids are non-empty WB16 constants")
        };

        let mut state_updates = vec![
            WritebackField::bounded(WB16_SYMBOL_PEAKRO, peakro, Some(WB16_PEAKRO_FLOOR), None),
            WritebackField::bounded(
                WB16_SYMBOL_WATDUR,
                watdur,
                Some(0.0),
                Some(WB16_MAX_DURATION_S),
            ),
            WritebackField::bounded(
                WB16_SYMBOL_METHOD_BRANCH,
                method_branch,
                Some(1.0),
                Some(3.0),
            ),
            WritebackField::bounded(WB16_SYMBOL_TSTAR, tstar, Some(WB11_ZERO_THRESHOLD), None),
            WritebackField::bounded(WB16_SYMBOL_QPSTAR, qpstar, Some(WB11_ZERO_THRESHOLD), None),
            WritebackField::bounded(
                WB16_SYMBOL_VSTAR,
                vstar,
                Some(WB11_ZERO_THRESHOLD),
                Some(1.0),
            ),
            WritebackField::bounded(
                BoundarySymbol::from(WB13_STATE_SYMBOL_WATCON),
                watcon,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                BoundarySymbol::from(WB13_STATE_SYMBOL_TOTAL_SOIL),
                total_soil,
                Some(0.0),
                None,
            ),
            WritebackField::bounded(
                BoundarySymbol::from(WB13_STATE_SYMBOL_SOIL_WATER_TOTAL),
                soil_water_total,
                Some(0.0),
                None,
            ),
        ];
        state_updates.extend(erod13_state_updates);
        state_updates.extend(erod14_state_updates);

        let writeback = KernelWritebackPayload::with_updates(state_updates, Vec::new());
        Ok(KernelRunResponse::new(status, writeback))
    }
}

impl HillslopeKernel for Wb11HydrologyKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        let response_result = match request.phase_class {
            HillslopeKernelPhaseClass::HydrologyEvapotranspiration => {
                Self::run_evapotranspiration(request)
            }
            HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage => {
                Self::run_percolation(request)
            }
            HillslopeKernelPhaseClass::HydrologyLateralTransfer => {
                Self::run_lateral_transfer(request)
            }
            HillslopeKernelPhaseClass::HydrologyDrainage => Self::run_drainage(request),
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation => {
                Self::run_runoff_reconciliation(request)
            }
            HillslopeKernelPhaseClass::HydrologyStorageReconciliation => {
                Self::run_storage_reconciliation(request)
            }
            HillslopeKernelPhaseClass::HydrologyPeakRunoff => Self::run_peak_runoff(request),
            _ => {
                let Ok(status) =
                    SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB11-NOP-001")
                else {
                    unreachable!("status message ids are non-empty WB11 constants")
                };
                Ok(KernelRunResponse::new(
                    status,
                    KernelWritebackPayload::empty(),
                ))
            }
        };

        match response_result {
            Ok(response) => response,
            Err(error) => KernelRunResponse::new(
                Self::status_from_guard_error(&error),
                KernelWritebackPayload::empty(),
            ),
        }
    }
}

#[allow(clippy::too_many_lines)]
pub(crate) fn decomposition_phase_dispatch_for_state(
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

    let (sumrtm_seed, sumsrm_seed) = compute_equation_decomposition_seed_surface(
        phase,
        state_surface,
        active_slot_selection.slot_index,
        active_slot_selection.crop_slot_index,
        control,
        sumrtm_seed,
        sumsrm_seed,
    )?;

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
pub(crate) fn growth_phase_dispatch_for_state(
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
            } else {
                HillslopeAnnualGrowthAction::None
            };

            let state_after = match active_action {
                HillslopeAnnualGrowthAction::None => compute_equation_growth_state_surface(
                    phase,
                    state_surface,
                    active_slot_selection.slot_index,
                    active_slot_selection.crop_slot_index,
                    management_class,
                    state_before,
                )?,
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
                HillslopePerennialGrowthAction::None => compute_equation_growth_state_surface(
                    phase,
                    state_surface,
                    active_slot_selection.slot_index,
                    active_slot_selection.crop_slot_index,
                    management_class,
                    state_before,
                )?,
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

#[derive(Debug, Clone, Copy)]
struct GrowthEquationInputs {
    ws: f64,
    tmax: f64,
    tmin: f64,
    rad: f64,
    solthk: f64,
    btemp: f64,
    otemp: f64,
    gddmax: f64,
    dlai: f64,
    dropfc: f64,
    decfct: f64,
    spriod: f64,
    bb: f64,
    beinp: f64,
    extnct: f64,
    hi: f64,
    xmxlai: f64,
    rsr: f64,
    rtmmax: f64,
    rdmax: f64,
}

#[allow(clippy::too_many_lines)]
fn compute_equation_growth_state_surface(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    management_class: u8,
    state_before: HillslopeGrowthStateSurface,
) -> Result<HillslopeGrowthStateSurface, HillslopeGrowthBoundaryError> {
    let inputs = require_growth_equation_inputs(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        management_class,
    )?;

    let tave = f64::midpoint(inputs.tmax, inputs.tmin);
    let gdd = (tave - inputs.btemp).max(0.0);
    let sumgdd_next = (state_before.sumgdd + gdd).min(inputs.gddmax);
    let fphu = (sumgdd_next / inputs.gddmax).clamp(0.0, 1.0);

    let temp_ratio = (gdd / (inputs.otemp - inputs.btemp)).min(1.0);
    let temstr = (std::f64::consts::FRAC_PI_2 * temp_ratio)
        .sin()
        .clamp(0.0, 1.0);
    let reg = inputs.ws.min(temstr);

    let par = PL_GROWTH_PAR_RAD_SCALE
        * inputs.rad
        * (1.0 - (-inputs.extnct * (state_before.lai + PL_GROWTH_PAR_LAI_OFFSET)).exp());
    if !par.is_finite() || par < 0.0 {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_CLIMATE_RAD_SYMBOL),
                value: par,
                reason: "PAR expression must be finite and non-negative",
            },
        );
    }

    let ddm = PL_GROWTH_DDM_SCALE * inputs.beinp * par;
    let vdmt_growth = state_before.vdmt + ddm * reg;
    let mut vdmt_next = vdmt_growth;
    if fphu >= inputs.dlai && inputs.spriod > 0.0 {
        let biomass_decline = (1.0 - inputs.dropfc) / inputs.spriod;
        let canopy_decline = (1.0 - inputs.decfct) / inputs.spriod;
        if !(0.0..=1.0).contains(&biomass_decline) {
            return Err(
                HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                    phase,
                    symbol: BoundarySymbol::from(PL_GROWTH_PARAM_DROPFC_ROOT),
                    value: biomass_decline,
                    reason: "daily biomass senescence decline must be within [0, 1]",
                },
            );
        }
        if !(0.0..=1.0).contains(&canopy_decline) {
            return Err(
                HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                    phase,
                    symbol: BoundarySymbol::from(PL_GROWTH_PARAM_DECFCT_ROOT),
                    value: canopy_decline,
                    reason: "daily canopy senescence decline must be within [0, 1]",
                },
            );
        }
        vdmt_next = vdmt_growth * (1.0 - biomass_decline);
    }
    vdmt_next = vdmt_next.max(0.0);

    let hufh_denom = fphu + (6.5 - 10.0 * fphu).exp();
    if hufh_denom <= 0.0 || !hufh_denom.is_finite() {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_PARAM_HI_ROOT),
                value: hufh_denom,
                reason: "harvest-index denominator must be positive and finite",
            },
        );
    }
    let mut hia_next = inputs.hi * (fphu / hufh_denom);
    let water_stress_adjustment = if (0.3..0.9).contains(&fphu) {
        (std::f64::consts::FRAC_PI_2 * (fphu - 0.3) / 0.3).sin()
    } else {
        0.0
    };
    hia_next -=
        inputs.hi * (1.0 - 1.0 / (1.0 + 0.01 * water_stress_adjustment * (0.9 - inputs.ws)));
    hia_next = hia_next.clamp(0.0, inputs.hi);

    let canopy_biomass = if management_class == 2 {
        vdmt_next
    } else {
        vdmt_next * (1.0 - hia_next)
    };
    let cancov_raw = 1.0 - (-inputs.bb * canopy_biomass).exp();
    if !cancov_raw.is_finite() {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_STATE_CANCOV_SYMBOL),
                value: cancov_raw,
                reason: "canopy-cover equation output must be finite",
            },
        );
    }
    let mut cancov_next = cancov_raw.clamp(0.0, PL_GROWTH_CANCOV_MAX);
    if fphu >= inputs.dlai && inputs.spriod > 0.0 {
        let canopy_decline = (1.0 - inputs.decfct) / inputs.spriod;
        cancov_next = (cancov_next * (1.0 - canopy_decline)).clamp(0.0, PL_GROWTH_CANCOV_MAX);
    }

    let lai_next = if management_class == 2 {
        let denom =
            vdmt_next + PL_GROWTH_PERENNIAL_LAI_A * (-PL_GROWTH_PERENNIAL_LAI_B * vdmt_next).exp();
        if denom <= 0.0 || !denom.is_finite() {
            return Err(
                HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                    phase,
                    symbol: BoundarySymbol::from(PL_GROWTH_STATE_LAI_SYMBOL),
                    value: denom,
                    reason: "perennial LAI denominator must be positive and finite",
                },
            );
        }
        inputs.xmxlai * vdmt_next / denom
    } else {
        let veg = vdmt_next * (1.0 - hia_next);
        let denom = veg + PL_GROWTH_ANNUAL_LAI_A * (-PL_GROWTH_ANNUAL_LAI_B * veg).exp();
        if denom <= 0.0 || !denom.is_finite() {
            return Err(
                HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                    phase,
                    symbol: BoundarySymbol::from(PL_GROWTH_STATE_LAI_SYMBOL),
                    value: denom,
                    reason: "annual LAI denominator must be positive and finite",
                },
            );
        }
        inputs.xmxlai * veg / denom
    };
    if !lai_next.is_finite() || lai_next < 0.0 {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_STATE_LAI_SYMBOL),
                value: lai_next,
                reason: "LAI must remain finite and non-negative",
            },
        );
    }

    let rtmass_unclamped = state_before.rtmass + (vdmt_next - state_before.vdmt) * inputs.rsr;
    let rtmass_next = if management_class == 2 {
        rtmass_unclamped.clamp(0.0, inputs.rtmmax)
    } else {
        rtmass_unclamped.max(0.0)
    };

    let rtd_floor = inputs.rdmax
        * 0.5
        * (1.0
            + (PL_GROWTH_ROOT_DEPTH_CURVE_A * fphu / inputs.dlai - PL_GROWTH_ROOT_DEPTH_CURVE_B)
                .sin());
    let rtd_upper = inputs.rdmax.min(inputs.solthk);
    if rtd_upper <= 0.0 {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_PARAM_RDMAX_ROOT),
                value: rtd_upper,
                reason: "root-depth upper bound must be positive",
            },
        );
    }

    let rtd_candidate = if management_class == 2 {
        let growth_increment = ((rtmass_next - state_before.rtmass) / inputs.rtmmax) * inputs.rdmax;
        (state_before.rtd + growth_increment).max(rtd_floor)
    } else {
        rtd_floor
    };
    if !rtd_candidate.is_finite() || rtd_candidate < 0.0 {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_STATE_RTD_SYMBOL),
                value: rtd_candidate,
                reason: "root depth must remain finite and non-negative",
            },
        );
    }
    let rtd_next = rtd_candidate.min(rtd_upper);

    let state_after = HillslopeGrowthStateSurface {
        sumgdd: sumgdd_next,
        vdmt: vdmt_next,
        cancov: cancov_next,
        lai: lai_next,
        rtmass: rtmass_next,
        rtd: rtd_next,
        hia: hia_next,
    };

    validate_growth_state_surface(phase, state_after)
}

#[allow(clippy::too_many_lines)]
fn require_growth_equation_inputs(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    management_class: u8,
) -> Result<GrowthEquationInputs, HillslopeGrowthBoundaryError> {
    let ws = require_finite_state_value(phase, state_surface, PL_GROWTH_WATER_STRESS_SYMBOL)?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_WATER_STRESS_SYMBOL,
        ws,
        Some(0.0),
        Some(1.0),
        "water-stress carryover must be within [0, 1]",
    )?;

    let tmax = require_finite_state_value(phase, state_surface, PL_GROWTH_CLIMATE_TMAX_SYMBOL)?;
    let tmin = require_finite_state_value(phase, state_surface, PL_GROWTH_CLIMATE_TMIN_SYMBOL)?;
    let rad = require_finite_state_value(phase, state_surface, PL_GROWTH_CLIMATE_RAD_SYMBOL)?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_CLIMATE_RAD_SYMBOL,
        rad,
        Some(0.0),
        None,
        "radiation forcing must be non-negative",
    )?;
    let solthk = require_finite_state_value(phase, state_surface, PL_GROWTH_SOIL_DEPTH_SYMBOL)?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_SOIL_DEPTH_SYMBOL,
        solthk,
        Some(f64::EPSILON),
        None,
        "soil-depth envelope must be positive",
    )?;

    let btemp = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_BTEMP_ROOT,
    )?;
    let otemp = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_OTEMP_ROOT,
    )?;
    if otemp <= btemp {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_PARAM_OTEMP_ROOT),
                value: otemp,
                reason: "otemp must be greater than btemp",
            },
        );
    }

    let gddmax = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_GDDMAX_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_GDDMAX_ROOT,
        gddmax,
        Some(f64::EPSILON),
        None,
        "gddmax must be positive",
    )?;

    let dlai = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_DLAI_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_DLAI_ROOT,
        dlai,
        Some(f64::EPSILON),
        Some(1.0),
        "dlai must be within (0, 1]",
    )?;

    let dropfc = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_DROPFC_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_DROPFC_ROOT,
        dropfc,
        Some(0.0),
        Some(1.0),
        "dropfc must be within [0, 1]",
    )?;
    let decfct = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_DECFCT_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_DECFCT_ROOT,
        decfct,
        Some(0.0),
        Some(1.0),
        "decfct must be within [0, 1]",
    )?;

    let spriod = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_SPRIOD_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_SPRIOD_ROOT,
        spriod,
        Some(0.0),
        None,
        "spriod must be non-negative",
    )?;

    let bb = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_BB_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_BB_ROOT,
        bb,
        Some(0.0),
        None,
        "bb must be non-negative",
    )?;

    let beinp = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_BEINP_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_BEINP_ROOT,
        beinp,
        Some(0.0),
        None,
        "beinp must be non-negative",
    )?;

    let extnct = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_EXTNCT_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_EXTNCT_ROOT,
        extnct,
        Some(0.0),
        None,
        "extnct must be non-negative",
    )?;

    let hi = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_HI_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_HI_ROOT,
        hi,
        Some(0.0),
        Some(1.0),
        "hi must be within [0, 1]",
    )?;

    let xmxlai = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_XMXLAI_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_XMXLAI_ROOT,
        xmxlai,
        Some(0.0),
        None,
        "xmxlai must be non-negative",
    )?;

    let rsr = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_RSR_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_RSR_ROOT,
        rsr,
        Some(0.0),
        None,
        "rsr must be non-negative",
    )?;

    let rtmmax = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_RTMMAX_ROOT,
    )?;
    if management_class == 2 {
        validate_growth_state_range(
            phase,
            PL_GROWTH_PARAM_RTMMAX_ROOT,
            rtmmax,
            Some(f64::EPSILON),
            None,
            "rtmmax must be positive for perennial growth",
        )?;
    }

    let rdmax = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_RDMAX_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_RDMAX_ROOT,
        rdmax,
        Some(f64::EPSILON),
        None,
        "rdmax must be positive",
    )?;

    Ok(GrowthEquationInputs {
        ws,
        tmax,
        tmin,
        rad,
        solthk,
        btemp,
        otemp,
        gddmax,
        dlai,
        dropfc,
        decfct,
        spriod,
        bb,
        beinp,
        extnct,
        hi,
        xmxlai,
        rsr,
        rtmmax,
        rdmax,
    })
}

fn require_slot_growth_value(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    root: &str,
) -> Result<f64, HillslopeGrowthBoundaryError> {
    let symbol = pl_growth_slot_crop_symbol(root, slot_index, crop_slot_index);
    require_finite_state_value(phase, state_surface, symbol.as_str())
}

fn validate_growth_state_surface(
    phase: HillslopePhase,
    state: HillslopeGrowthStateSurface,
) -> Result<HillslopeGrowthStateSurface, HillslopeGrowthBoundaryError> {
    for (symbol, value, minimum, maximum, reason) in [
        (
            PL_GROWTH_STATE_SUMGDD_SYMBOL,
            state.sumgdd,
            Some(0.0),
            None,
            "sumgdd must be non-negative",
        ),
        (
            PL_GROWTH_STATE_VDMT_SYMBOL,
            state.vdmt,
            Some(0.0),
            None,
            "vdmt must be non-negative",
        ),
        (
            PL_GROWTH_STATE_CANCOV_SYMBOL,
            state.cancov,
            Some(0.0),
            Some(PL_GROWTH_CANCOV_MAX),
            "cancov must be within [0, 0.999]",
        ),
        (
            PL_GROWTH_STATE_LAI_SYMBOL,
            state.lai,
            Some(0.0),
            None,
            "lai must be non-negative",
        ),
        (
            PL_GROWTH_STATE_RTMASS_SYMBOL,
            state.rtmass,
            Some(0.0),
            None,
            "rtmass must be non-negative",
        ),
        (
            PL_GROWTH_STATE_RTD_SYMBOL,
            state.rtd,
            Some(0.0),
            None,
            "rtd must be non-negative",
        ),
        (
            PL_GROWTH_STATE_HIA_SYMBOL,
            state.hia,
            Some(0.0),
            Some(1.0),
            "hia must be within [0, 1]",
        ),
    ] {
        validate_growth_state_range(phase, symbol, value, minimum, maximum, reason)?;
    }

    Ok(state)
}

fn validate_growth_state_range(
    phase: HillslopePhase,
    symbol: &str,
    value: f64,
    minimum: Option<f64>,
    maximum: Option<f64>,
    reason: &'static str,
) -> Result<(), HillslopeGrowthBoundaryError> {
    if !value.is_finite() {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
                reason: "state value must be finite",
            },
        );
    }
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
    Ok(())
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

#[derive(Debug, Clone, Copy)]
struct DecompositionEquationInputs {
    ws: f64,
    tmax: f64,
    tmin: f64,
    prcp: f64,
    oratea: f64,
    orater: f64,
}

#[allow(clippy::too_many_lines)]
fn compute_equation_decomposition_seed_surface(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    control: HillslopeDecompositionTransitionControl,
    sumrtm_seed: f64,
    sumsrm_seed: f64,
) -> Result<(f64, f64), HillslopeDecompositionBoundaryError> {
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_SUMRTM_SEED_SYMBOL,
        sumrtm_seed,
        Some(0.0),
        None,
        "sumrtm_seed must be non-negative",
    )?;
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_SUMSRM_SEED_SYMBOL,
        sumsrm_seed,
        Some(0.0),
        None,
        "sumsrm_seed must be non-negative",
    )?;

    let inputs =
        require_decomposition_equation_inputs(phase, state_surface, slot_index, crop_slot_index)?;
    let tave = f64::midpoint(inputs.tmax, inputs.tmin);

    let tmpfac = if tave <= -PL_DECOMP_TEMP_ATEMP || tave >= PL_DECOMP_TEMP_ACTIVE_UPPER {
        0.0
    } else {
        let t1 = (tave + PL_DECOMP_TEMP_ATEMP).powi(2);
        let numerator = t1 * (2.0 * PL_DECOMP_TEMP_T2 - t1);
        let denominator = PL_DECOMP_TEMP_T2.powi(2);
        if denominator <= 0.0 {
            return Err(
                HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                    phase,
                    symbol: BoundarySymbol::from(PL_DECOMP_CLIMATE_TMAX_SYMBOL),
                    value: denominator,
                    reason: "temperature-factor denominator must be positive",
                },
            );
        }
        numerator / denominator
    };
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_CLIMATE_TMAX_SYMBOL,
        tmpfac,
        Some(0.0),
        Some(1.0),
        "temperature decomposition factor must be within [0, 1]",
    )?;

    let swatfc = if tave <= 0.0 {
        0.0
    } else if inputs.prcp < PL_DECOMP_STANDING_RAIN_SATURATION {
        inputs.prcp / PL_DECOMP_STANDING_RAIN_SATURATION
    } else {
        1.0
    };
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_CLIMATE_PRCP_SYMBOL,
        swatfc,
        Some(0.0),
        Some(1.0),
        "standing-residue water factor must be within [0, 1]",
    )?;

    let fwatfc = inputs.ws.clamp(0.0, 1.0);
    let _senvin = tmpfac.min(swatfc);
    let envinx = tmpfac.min(fwatfc);
    validate_decomposition_state_range(
        phase,
        PL_GROWTH_WATER_STRESS_SYMBOL,
        envinx,
        Some(0.0),
        Some(1.0),
        "environmental decomposition factor must be within [0, 1]",
    )?;

    let surface_exponent = -envinx * inputs.oratea;
    let root_exponent = -envinx * inputs.orater;
    if !surface_exponent.is_finite() {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_DECOMP_PARAM_ORATEA_ROOT),
                value: surface_exponent,
                reason: "surface decomposition exponent must be finite",
            },
        );
    }
    if !root_exponent.is_finite() {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_DECOMP_PARAM_ORATER_ROOT),
                value: root_exponent,
                reason: "root decomposition exponent must be finite",
            },
        );
    }

    let surface_decay = surface_exponent.exp();
    let root_decay = root_exponent.exp();
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_PARAM_ORATEA_ROOT,
        surface_decay,
        Some(0.0),
        Some(1.0),
        "surface decomposition decay factor must be within [0, 1]",
    )?;
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_PARAM_ORATER_ROOT,
        root_decay,
        Some(0.0),
        Some(1.0),
        "root decomposition decay factor must be within [0, 1]",
    )?;

    let mut sumsrm_next = sumsrm_seed * surface_decay;
    let mut sumrtm_next = sumrtm_seed * root_decay;

    match control {
        HillslopeDecompositionTransitionControl::Annual(annual_control) => {
            match annual_control.active_action {
                HillslopeAnnualDecompositionAction::Burn => {
                    sumsrm_next *= 1.0 - annual_control.fbrnog;
                }
                HillslopeAnnualDecompositionAction::Remove => {
                    sumsrm_next *= 1.0 - annual_control.frmove;
                }
                HillslopeAnnualDecompositionAction::Cut => {
                    let transfer = sumsrm_next * annual_control.frcut;
                    sumsrm_next -= transfer;
                    sumrtm_next += transfer;
                }
                HillslopeAnnualDecompositionAction::None
                | HillslopeAnnualDecompositionAction::Herbicide
                | HillslopeAnnualDecompositionAction::Silage => {}
            }
        }
        HillslopeDecompositionTransitionControl::Perennial(perennial_control) => {
            if let HillslopePerennialDecompositionAction::Grazing { cycle_index } =
                perennial_control.active_action
            {
                let Some(active_cycle) = perennial_control.active_grazing_cycle else {
                    return Err(
                        HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                            phase,
                            symbol: BoundarySymbol::from("active_grazing_cycle"),
                            value: f64::from(cycle_index),
                            reason: "grazing action requires active_grazing_cycle payload instance",
                        },
                    );
                };
                if active_cycle.cycle_index != cycle_index {
                    return Err(
                        HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                            phase,
                            symbol: BoundarySymbol::from("active_grazing_cycle"),
                            value: f64::from(active_cycle.cycle_index),
                            reason: "active grazing cycle index must match active action",
                        },
                    );
                }
                validate_decomposition_state_range(
                    phase,
                    "digest",
                    active_cycle.digest,
                    Some(0.0),
                    Some(1.0),
                    "grazing digest fraction must be within [0, 1]",
                )?;
                sumsrm_next *= 1.0 - active_cycle.digest;
            }
        }
    }

    validate_decomposition_state_range(
        phase,
        PL_DECOMP_SUMRTM_SEED_SYMBOL,
        sumrtm_next,
        Some(0.0),
        None,
        "sumrtm_seed must remain non-negative",
    )?;
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_SUMSRM_SEED_SYMBOL,
        sumsrm_next,
        Some(0.0),
        None,
        "sumsrm_seed must remain non-negative",
    )?;

    Ok((sumrtm_next, sumsrm_next))
}

fn require_decomposition_equation_inputs(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
) -> Result<DecompositionEquationInputs, HillslopeDecompositionBoundaryError> {
    let ws = require_finite_state_value_for_decomposition(phase, state_surface, "Ws")?;
    validate_decomposition_state_range(
        phase,
        PL_GROWTH_WATER_STRESS_SYMBOL,
        ws,
        Some(0.0),
        Some(1.0),
        "water-stress carryover must be within [0, 1]",
    )?;

    let tmax = require_finite_state_value_for_decomposition(
        phase,
        state_surface,
        PL_DECOMP_CLIMATE_TMAX_SYMBOL,
    )?;
    let tmin = require_finite_state_value_for_decomposition(
        phase,
        state_surface,
        PL_DECOMP_CLIMATE_TMIN_SYMBOL,
    )?;
    let prcp = require_finite_state_value_for_decomposition(
        phase,
        state_surface,
        PL_DECOMP_CLIMATE_PRCP_SYMBOL,
    )?;
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_CLIMATE_PRCP_SYMBOL,
        prcp,
        Some(0.0),
        None,
        "precipitation forcing must be non-negative",
    )?;

    let annual_decay_rate = require_slot_decomposition_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_DECOMP_PARAM_ORATEA_ROOT,
    )?;
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_PARAM_ORATEA_ROOT,
        annual_decay_rate,
        Some(f64::EPSILON),
        None,
        "oratea must be positive",
    )?;

    let root_decay_rate = require_slot_decomposition_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_DECOMP_PARAM_ORATER_ROOT,
    )?;
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_PARAM_ORATER_ROOT,
        root_decay_rate,
        Some(f64::EPSILON),
        None,
        "orater must be positive",
    )?;

    Ok(DecompositionEquationInputs {
        ws,
        tmax,
        tmin,
        prcp,
        oratea: annual_decay_rate,
        orater: root_decay_rate,
    })
}

fn require_slot_decomposition_value(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    root: &str,
) -> Result<f64, HillslopeDecompositionBoundaryError> {
    let symbol = pl_decomp_slot_crop_symbol(root, slot_index, crop_slot_index);
    require_finite_state_value_for_decomposition(phase, state_surface, symbol.as_str())
}

fn validate_decomposition_state_range(
    phase: HillslopePhase,
    symbol: &str,
    value: f64,
    minimum: Option<f64>,
    maximum: Option<f64>,
    reason: &'static str,
) -> Result<(), HillslopeDecompositionBoundaryError> {
    if !value.is_finite() {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
                reason: "state value must be finite",
            },
        );
    }
    if let Some(minimum) = minimum {
        if value < minimum {
            return Err(
                HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
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
                HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                    phase,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    reason,
                },
            );
        }
    }
    Ok(())
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
