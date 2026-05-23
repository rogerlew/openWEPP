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
    HillslopePerennialGrowthControl, KernelRunResponse, KernelWritebackApplyResult,
    KernelWritebackPayload, MAX_CLIMATE_FORCING_SERIES_POINTS, WritebackDecisionOutcome,
    WritebackError, WritebackField, apply_kernel_writeback, evaluate_kernel_writeback,
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
const PL_GROWTH_CLIMATE_TMAX_SYMBOL: &str = "tmax";
const PL_GROWTH_CLIMATE_TMIN_SYMBOL: &str = "tmin";
const PL_GROWTH_CLIMATE_RAD_SYMBOL: &str = "rad";
const PL_DECOMP_CLIMATE_TMAX_SYMBOL: &str = "tmax";
const PL_DECOMP_CLIMATE_TMIN_SYMBOL: &str = "tmin";
const PL_DECOMP_CLIMATE_PRCP_SYMBOL: &str = "prcp";
const PL_GROWTH_SOIL_DEPTH_SYMBOL: &str = "solthk";
const PL_GROWTH_WATER_STRESS_SYMBOL: &str = "Ws";
const PL_DECOMP_PARAM_ORATEA_ROOT: &str = "oratea";
const PL_DECOMP_PARAM_ORATER_ROOT: &str = "orater";
const PL_DECOMP_TEMP_ATEMP: f64 = 6.1;
const PL_DECOMP_TEMP_ACTIVE_UPPER: f64 = 49.2;
const PL_DECOMP_TEMP_T2: f64 = 1528.81;
const PL_DECOMP_STANDING_RAIN_SATURATION: f64 = 0.004;
const PL_GROWTH_PARAM_BTEMP_ROOT: &str = "btemp";
const PL_GROWTH_PARAM_OTEMP_ROOT: &str = "otemp";
const PL_GROWTH_PARAM_GDDMAX_ROOT: &str = "gddmax";
const PL_GROWTH_PARAM_DLAI_ROOT: &str = "dlai";
const PL_GROWTH_PARAM_DROPFC_ROOT: &str = "dropfc";
const PL_GROWTH_PARAM_DECFCT_ROOT: &str = "decfct";
const PL_GROWTH_PARAM_SPRIOD_ROOT: &str = "spriod";
const PL_GROWTH_PARAM_BB_ROOT: &str = "bb";
const PL_GROWTH_PARAM_BEINP_ROOT: &str = "beinp";
const PL_GROWTH_PARAM_EXTNCT_ROOT: &str = "extnct";
const PL_GROWTH_PARAM_HI_ROOT: &str = "hi";
const PL_GROWTH_PARAM_XMXLAI_ROOT: &str = "xmxlai";
const PL_GROWTH_PARAM_RSR_ROOT: &str = "rsr";
const PL_GROWTH_PARAM_RTMMAX_ROOT: &str = "rtmmax";
const PL_GROWTH_PARAM_RDMAX_ROOT: &str = "rdmax";
const PL_GROWTH_PAR_RAD_SCALE: f64 = 0.02092;
const PL_GROWTH_PAR_LAI_OFFSET: f64 = 0.05;
const PL_GROWTH_DDM_SCALE: f64 = 0.0001;
const PL_GROWTH_ANNUAL_LAI_A: f64 = 0.5512;
const PL_GROWTH_ANNUAL_LAI_B: f64 = 6.8;
const PL_GROWTH_PERENNIAL_LAI_A: f64 = 0.2756;
const PL_GROWTH_PERENNIAL_LAI_B: f64 = 13.6;
const PL_GROWTH_ROOT_DEPTH_CURVE_A: f64 = 3.03;
const PL_GROWTH_ROOT_DEPTH_CURVE_B: f64 = 1.47;
const PL_GROWTH_CANCOV_MAX: f64 = 0.999;
const ORDER_FLAG_EPSILON: f64 = 1.0e-12;
const MANAGEMENT_CLASS_EPSILON: f64 = 1.0e-9;
const WB11_ZERO_THRESHOLD: f64 = 1.0e-12;
const WB11_SYMBOL_SOIL_WATER: &str = "wb11_soil_water";
const WB11_SYMBOL_ET_DEMAND: &str = "wb11_et_demand";
const WB11_SYMBOL_FIELD_CAPACITY: &str = "wb11_field_capacity";
const WB11_SYMBOL_PERC_FRACTION: &str = "wb11_perc_fraction";
const WB11_SYMBOL_LATERAL_FRACTION: &str = "wb11_lateral_fraction";
const WB11_SYMBOL_DRAINAGE_FRACTION: &str = "wb11_drainage_fraction";
const WB11_SYMBOL_DRAINAGE_COEFFICIENT: &str = "wb11_drainage_coefficient";
const WB11_SYMBOL_DRAINABLE_STORAGE: &str = "wb11_drainable_storage";
const WB11_SYMBOL_ET: &str = "ET";
const WB11_SYMBOL_WS: &str = "Ws";
const WB11_SYMBOL_PERC_LOSS_D: &str = "D";
const WB11_SYMBOL_PERC_RECHARGE_PE: &str = "Pe";
const WB11_SYMBOL_LATERAL_Q: &str = "q";
const WB11_SYMBOL_DRAINAGE_QDD: &str = "Qdd";
const WB11_SYMBOL_SUBHYD_QD: &str = "Qd";
const WB12_SYMBOL_RAINFALL_INPUT: &str = "wb12_rainfall_input";
const WB12_SYMBOL_RUNON_INPUT: &str = "wb12_runon_input";
const WB12_SYMBOL_INFILTRATION: &str = "wb12_infiltration";
const WB12_SYMBOL_DEPRESSION_STORAGE_DELTA: &str = "wb12_depression_storage_delta";
const WB12_SYMBOL_RUNOFF_OBSERVED: &str = "wb12_runoff_observed";
const WB12_SYMBOL_RUNOFF_CLOSURE_TOLERANCE: &str = "wb12_runoff_closure_tolerance";
const WB12_SYMBOL_RUNOFF_CLOSURE_DELTA: &str = "wb12_runoff_closure_delta";
const WB12_SYMBOL_RUNOFF_RECONCILED: &str = "wb12_runoff_reconciled";
const WB12_SYMBOL_STORAGE_INITIAL: &str = "wb12_storage_initial";
const WB12_SYMBOL_STORAGE_OBSERVED: &str = "wb12_storage_observed";
const WB12_SYMBOL_STORAGE_CLOSURE_TOLERANCE: &str = "wb12_storage_closure_tolerance";
const WB12_SYMBOL_PRECIP_INPUT: &str = "wb12_precip_input";
const WB12_SYMBOL_STORAGE_CLOSURE_DELTA: &str = "wb12_storage_closure_delta";
const WB12_SYMBOL_STORAGE_RECONCILED: &str = "wb12_storage_reconciled";
const WB12_SYMBOL_RUNOFF_Q: &str = "Q";
const WB12_SYMBOL_SNOW_COUPLING_S: &str = "S";
const IRRIG_SYMBOL_DAILY_IRRIGATION: &str = "Irr";
const IRRIG_SYMBOL_RUNTIME_SOURCE: &str = "irrigation.runtime_schedule_source";
const IRRIG_SYMBOL_RUNTIME_DEPTH_M: &str = "irrigation.runtime_depth_m";
const IRRIG_SYMBOL_RUNTIME_DURATION_S: &str = "irrigation.runtime_duration_s";
const IRRIG_SYMBOL_RUNTIME_RATE_MPS: &str = "irrigation.runtime_rate_m_per_s";
const IRRIG_SYMBOL_RUNTIME_EVENT_INDEX: &str = "irrigation.runtime_event_index";
const IRRIG_SYMBOL_RUNTIME_SYSTEM_TYPE: &str = "irrigation.runtime_system_type";
const IRRIG_SYMBOL_DEPLETION_ENABLED: &str = "irrigation.depletion.enabled";
const IRRIG_SYMBOL_DEPLETION_SYSTEM_TYPE: &str = "irrigation.depletion.system_type";
const IRRIG_SYMBOL_DEPLETION_MIN_DEPTH_M: &str = "irrigation.depletion.min_depth_m";
const IRRIG_SYMBOL_DEPLETION_MAX_DEPTH_M: &str = "irrigation.depletion.max_depth_m";
const IRRIG_SYMBOL_DEPLETION_PERIOD_COUNT: &str = "irrigation.depletion.period_count";
const IRRIG_SYMBOL_FIXEDDATE_ENABLED: &str = "irrigation.fixeddate.enabled";
const IRRIG_SYMBOL_FIXEDDATE_SYSTEM_TYPE: &str = "irrigation.fixeddate.system_type";
const IRRIG_SYMBOL_FIXEDDATE_EVENT_COUNT: &str = "irrigation.fixeddate.event_count";
const WB15_SYMBOL_INTERCEPTION_I: &str = "I";
const WB15_SYMBOL_PLANT_CANCOV: &str = "cancov";
const WB15_SYMBOL_PLANT_LAI: &str = "lai";
const WB15_SYMBOL_PLANT_VDMT: &str = "vdmt";
const WB15_CANCOV_MAX: f64 = 0.999;
const WB15_VDMT_MAX: f64 = 0.8;
const WB15_BIOMASS_TO_KG_HA: f64 = 10_000.0;
const WB15_INTERCEPT_LINEAR_COEFF: f64 = 0.000_627;
const WB15_INTERCEPT_QUADRATIC_COEFF: f64 = 3.733_49e-8;
const WB15_INTERCEPT_MM_TO_M: f64 = 1000.0;
const WB14_SYMBOL_HYETOGRAPH_NINTEN: &str = "ninten";
const WB14_SYMBOL_HYETOGRAPH_NBRKPT: &str = "nbrkpt";
const WB14_SYMBOL_SOIL_CONDUCTIVITY: &str = "ssc";
const WB14_SYMBOL_SOIL_LAYER_DEPTH: &str = "dg";
const WB14_SYMBOL_SOIL_THETA_RESIDUAL: &str = "thetdr";
const WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY: &str = "thetfc";
const WB14_SYMBOL_SNOW_FILE_PRESENT: &str = "snow.options.snow_file_present";
const WB14_SYMBOL_SNOW_RST: &str = "snow.options.rst";
const WB14_SYMBOL_SNOW_NEWSNW: &str = "snow.options.newsnw";
const WB14_SYMBOL_SNOW_SSD: &str = "snow.options.ssd";
const WB14_SYMBOL_SNOW_RUNTIME_SWE: &str = "snow.runtime_swe";
const WB14_SYMBOL_FROST_FILE_PRESENT: &str = "frost.options.frost_file_present";
const WB14_SYMBOL_FROST_WINT_RED: &str = "frost.options.wintRed";
const WB14_SYMBOL_FROST_FINE_TOP: &str = "frost.options.fineTop";
const WB14_SYMBOL_FROST_FINE_BOT: &str = "frost.options.fineBot";
const WB14_SYMBOL_FROST_KSNOWF: &str = "frost.options.ksnowf";
const WB14_SYMBOL_FROST_KRESF: &str = "frost.options.kresf";
const WB14_SYMBOL_FROST_KSOILF: &str = "frost.options.ksoilf";
const WB14_SYMBOL_FROST_KFACTOR1: &str = "frost.options.kfactor1";
const WB14_SYMBOL_FROST_KFACTOR2: &str = "frost.options.kfactor2";
const WB14_SYMBOL_FROST_KFACTOR3: &str = "frost.options.kfactor3";
const WB14_SYMBOL_FROST_RUNTIME_DFROST: &str = "frost.runtime_dfrost";
const WB14_SYMBOL_FROST_RUNTIME_DTHAW: &str = "frost.runtime_dthaw";
const WB14_SYMBOL_FROST_RUNTIME_NFT: &str = "frost.runtime_nft";
const WB14_SYMBOL_FROST_RUNTIME_WS_FRZ: &str = "frost.runtime_ws_frz";
const WB14_SYMBOL_FROST_RUNTIME_INFCAP_FRZ: &str = "frost.runtime_infcap_frz";
const WB14_SYMBOL_TMAX: &str = "tmax";
const WB14_SYMBOL_TMIN: &str = "tmin";
const WB14_FROST_MAX_DEPTH_M: f64 = 0.20;
const WB16_SYMBOL_TIMEP: &str = "timep";
const WB16_SYMBOL_EFFLEN: &str = "efflen";
const WB16_SYMBOL_EALPHA: &str = "ealpha";
const WB16_SYMBOL_EXPONENT_M: &str = "m";
const WB16_SYMBOL_PEAKRO: &str = "peakro";
const WB16_SYMBOL_WATDUR: &str = "watdur";
const WB16_SYMBOL_METHOD_BRANCH: &str = "wb16_peak_method_branch";
const WB16_SYMBOL_TSTAR: &str = "wb16_tstar";
const WB16_SYMBOL_QPSTAR: &str = "wb16_qpstar";
const WB16_SYMBOL_VSTAR: &str = "wb16_vstar";
const WB16_PEAKRO_FLOOR: f64 = 3.63e-8;
const WB16_MAX_DURATION_S: f64 = 86_400.0;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HydrologyPhaseDispatch {
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

fn hydrology_phase_dispatch_for_phase(
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
}

impl Wb11HydrologyKernelGuardError {
    #[must_use]
    pub const fn boundary_class(&self) -> BoundaryClass {
        match self {
            Self::MissingRequiredStateSymbol { .. } | Self::MissingRequiredFluxSymbol { .. } => {
                BoundaryClass::MissingRequiredInput
            }
            Self::NonFiniteStateSymbol { .. } | Self::NonFiniteFluxSymbol { .. } => {
                BoundaryClass::NonFinite
            }
            Self::StateSymbolOutOfRange { .. } | Self::FluxSymbolOutOfRange { .. } => {
                BoundaryClass::DomainViolation
            }
        }
    }

    #[must_use]
    pub fn code(&self) -> String {
        let (phase_class, suffix) = match self {
            Self::MissingRequiredStateSymbol { phase_class, .. }
            | Self::MissingRequiredFluxSymbol { phase_class, .. } => (phase_class, "001"),
            Self::NonFiniteStateSymbol { phase_class, .. }
            | Self::NonFiniteFluxSymbol { phase_class, .. } => (phase_class, "002"),
            Self::StateSymbolOutOfRange { phase_class, .. }
            | Self::FluxSymbolOutOfRange { phase_class, .. } => (phase_class, "003"),
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
        symbol: &'static str,
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
                symbol: BoundarySymbol::from(symbol),
                value: scalar,
            });
        }
        Ok(scalar)
    }

    fn require_flux_scalar(
        request: &HillslopeKernelRequest<'_>,
        phase_class: HillslopeKernelPhaseClass,
        symbol: &'static str,
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
        symbol: &'static str,
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
        symbol: &'static str,
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

    fn require_state_range(
        phase_class: HillslopeKernelPhaseClass,
        symbol: &'static str,
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

    fn require_flux_range(
        phase_class: HillslopeKernelPhaseClass,
        symbol: &'static str,
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
        symbol: &'static str,
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

    fn irrigation_depletion_period_symbol(period_index: usize, field: &str) -> BoundarySymbol {
        BoundarySymbol::from(format!(
            "irrigation.depletion.period_{period_index:04}.{field}"
        ))
    }

    fn irrigation_fixeddate_event_symbol(event_index: usize, field: &str) -> BoundarySymbol {
        BoundarySymbol::from(format!(
            "irrigation.fixeddate.event_{event_index:04}.{field}"
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
            let ofe_symbol = Self::irrigation_fixeddate_event_symbol(event_index, "ofe_id");
            let event_ofe = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &ofe_symbol,
            )?;
            if event_ofe != 1 {
                continue;
            }

            let day_symbol = Self::irrigation_fixeddate_event_symbol(event_index, "day");
            let event_day = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &day_symbol,
            )?;
            let year_symbol = Self::irrigation_fixeddate_event_symbol(event_index, "year");
            let event_year = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &year_symbol,
            )?;

            let termination_symbol =
                Self::irrigation_fixeddate_event_symbol(event_index, "schedule_termination_flag");
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

            let depth_symbol =
                Self::irrigation_fixeddate_event_symbol(event_index, "sprinkler_depth_m");
            let depth_m =
                Self::require_state_scalar_for_symbol(request, phase_class, &depth_symbol)?;
            let rate_symbol =
                Self::irrigation_fixeddate_event_symbol(event_index, "sprinkler_rate_m_per_s");
            let base_rate =
                Self::require_state_scalar_for_symbol(request, phase_class, &rate_symbol)?;
            let nozzle_symbol =
                Self::irrigation_fixeddate_event_symbol(event_index, "sprinkler_nozzle_factor");
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
            let element_symbol =
                Self::irrigation_depletion_period_symbol(period_index, "element_id");
            let element_id = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &element_symbol,
            )?;
            if element_id != 1 {
                continue;
            }

            let start_day_symbol =
                Self::irrigation_depletion_period_symbol(period_index, "start_doy");
            let start_day = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &start_day_symbol,
            )?;
            let start_year_symbol =
                Self::irrigation_depletion_period_symbol(period_index, "start_year");
            let start_year = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &start_year_symbol,
            )?;
            let end_day_symbol = Self::irrigation_depletion_period_symbol(period_index, "end_doy");
            let end_day = Self::require_non_negative_integral_state_symbol(
                request,
                phase_class,
                &end_day_symbol,
            )?;
            let end_year_symbol =
                Self::irrigation_depletion_period_symbol(period_index, "end_year");
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

            let threshold_symbol =
                Self::irrigation_depletion_period_symbol(period_index, "depletion_trigger_ratio");
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

            let depth_ratio_symbol =
                Self::irrigation_depletion_period_symbol(period_index, "sprinkler_depth_ratio");
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

            let rate_symbol =
                Self::irrigation_depletion_period_symbol(period_index, "sprinkler_rate_m_per_s");
            let base_rate =
                Self::require_state_scalar_for_symbol(request, phase_class, &rate_symbol)?;
            let nozzle_symbol =
                Self::irrigation_depletion_period_symbol(period_index, "sprinkler_nozzle_factor");
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
        Self::require_state_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            potential_interception,
            Some(0.0),
            None,
        )?;

        let interception = potential_interception.min(hyetograph_rainfall);
        Self::require_state_range(
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
        interception: f64,
    ) -> Result<(f64, f64), Wb11HydrologyKernelGuardError> {
        let liquid_after_interception = hyetograph_rainfall - interception;
        Self::require_state_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            liquid_after_interception,
            Some(0.0),
            Some(hyetograph_rainfall),
        )?;

        if hyetograph_rainfall <= WB11_ZERO_THRESHOLD {
            return Ok((liquid_after_interception, 0.0));
        }

        let rainfall_scale = liquid_after_interception / hyetograph_rainfall;
        Self::require_state_range(
            phase_class,
            WB15_SYMBOL_INTERCEPTION_I,
            rainfall_scale,
            Some(0.0),
            Some(1.0),
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

        let actual_et = soil_water.min(et_demand);
        let soil_water_after = soil_water - actual_et;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_SOIL_WATER,
            soil_water_after,
            Some(0.0),
            None,
        )?;

        let ws = if et_demand <= WB11_ZERO_THRESHOLD {
            1.0
        } else {
            actual_et / et_demand
        };
        Self::require_flux_range(phase_class, WB11_SYMBOL_ET, actual_et, Some(0.0), None)?;
        Self::require_flux_range(phase_class, WB11_SYMBOL_WS, ws, Some(0.0), Some(1.0))?;

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB11-ET-OK-001")
        else {
            unreachable!("status message ids are non-empty WB11 constants")
        };
        let writeback = KernelWritebackPayload::with_updates(
            vec![WritebackField::bounded(
                WB11_SYMBOL_SOIL_WATER,
                soil_water_after,
                Some(0.0),
                None,
            )],
            vec![
                WritebackField::bounded(WB11_SYMBOL_ET, actual_et, Some(0.0), None),
                WritebackField::bounded(WB11_SYMBOL_WS, ws, Some(0.0), Some(1.0)),
            ],
        );
        Ok(KernelRunResponse::new(status, writeback))
    }

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

        let field_capacity =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_FIELD_CAPACITY)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_FIELD_CAPACITY,
            field_capacity,
            Some(0.0),
            None,
        )?;

        let perc_fraction =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_PERC_FRACTION)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_PERC_FRACTION,
            perc_fraction,
            Some(0.0),
            Some(1.0),
        )?;

        let excess = if soil_water > field_capacity {
            soil_water - field_capacity
        } else {
            0.0
        };
        let percolation_loss = excess * perc_fraction;
        let soil_water_after = soil_water - percolation_loss;
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
            Some(excess),
        )?;

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB11-PERC-OK-001")
        else {
            unreachable!("status message ids are non-empty WB11 constants")
        };
        let writeback = KernelWritebackPayload::with_updates(
            vec![WritebackField::bounded(
                WB11_SYMBOL_SOIL_WATER,
                soil_water_after,
                Some(0.0),
                None,
            )],
            vec![
                WritebackField::bounded(WB11_SYMBOL_PERC_LOSS_D, percolation_loss, Some(0.0), None),
                WritebackField::bounded(
                    WB11_SYMBOL_PERC_RECHARGE_PE,
                    percolation_loss,
                    Some(0.0),
                    None,
                ),
            ],
        );
        Ok(KernelRunResponse::new(status, writeback))
    }

    fn run_lateral_transfer(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyLateralTransfer;
        let drainable_storage =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_DRAINABLE_STORAGE)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_storage,
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

        let lateral_fraction =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_LATERAL_FRACTION)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_LATERAL_FRACTION,
            lateral_fraction,
            Some(0.0),
            Some(1.0),
        )?;

        let available = drainable_storage + recharge_pe;
        let q_lateral = available * lateral_fraction;
        let drainable_after = available - q_lateral;
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
            Some(available),
        )?;

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB11-LAT-OK-001")
        else {
            unreachable!("status message ids are non-empty WB11 constants")
        };
        let writeback = KernelWritebackPayload::with_updates(
            vec![WritebackField::bounded(
                WB11_SYMBOL_DRAINABLE_STORAGE,
                drainable_after,
                Some(0.0),
                None,
            )],
            vec![WritebackField::bounded(
                WB11_SYMBOL_LATERAL_Q,
                q_lateral,
                Some(0.0),
                None,
            )],
        );
        Ok(KernelRunResponse::new(status, writeback))
    }

    fn run_drainage(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyDrainage;
        let drainable_storage =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_DRAINABLE_STORAGE)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINABLE_STORAGE,
            drainable_storage,
            Some(0.0),
            None,
        )?;

        let drainage_fraction =
            Self::require_state_scalar(request, phase_class, WB11_SYMBOL_DRAINAGE_FRACTION)?;
        Self::require_state_range(
            phase_class,
            WB11_SYMBOL_DRAINAGE_FRACTION,
            drainage_fraction,
            Some(0.0),
            Some(1.0),
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

        let uncapped_drainage = drainable_storage * drainage_fraction;
        let q_drainage = uncapped_drainage.min(drainage_capacity);
        let drainable_after = drainable_storage - q_drainage;
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
        let writeback = KernelWritebackPayload::with_updates(
            vec![WritebackField::bounded(
                WB11_SYMBOL_DRAINABLE_STORAGE,
                drainable_after,
                Some(0.0),
                None,
            )],
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

        let interception =
            Self::compute_canopy_interception_depth(request, phase_class, hyetograph_rainfall)?;
        let (hyetograph_liquid_after_interception, rainfall_scale) =
            Self::resolve_interception_rainfall_scale(
                phase_class,
                hyetograph_rainfall,
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

        let active_snow_coupling = Self::resolve_active_snow_coupling(request, phase_class)?;
        let snow_coupling = if active_snow_coupling {
            Self::compute_active_snow_coupling(request, phase_class, hyetograph_rainfall)?
        } else {
            SnowCouplingOutcome {
                signed_s: 0.0,
                runtime_swe: 0.0,
            }
        };

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

        let runoff_observed =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_RUNOFF_OBSERVED)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RUNOFF_OBSERVED,
            runoff_observed,
            Some(0.0),
            None,
        )?;

        let q_runoff = Self::compute_runoff_after_interception(
            phase_class,
            liquid_after_interception,
            snow_coupling.signed_s,
            runon_input,
            cumulative_infiltration,
            depression_storage_delta,
        )?;

        let closure_delta = q_runoff - runoff_observed;
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

        let storage_observed =
            Self::require_state_scalar(request, phase_class, WB12_SYMBOL_STORAGE_OBSERVED)?;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_STORAGE_OBSERVED,
            storage_observed,
            Some(0.0),
            None,
        )?;

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

        let closure_delta = storage_reconciled - storage_observed;
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

    #[allow(clippy::too_many_lines)]
    fn run_peak_runoff(
        request: &HillslopeKernelRequest<'_>,
    ) -> Result<KernelRunResponse, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyPeakRunoff;

        let q_runoff = Self::require_flux_scalar(request, phase_class, WB12_SYMBOL_RUNOFF_Q)?;
        Self::require_flux_range(phase_class, WB12_SYMBOL_RUNOFF_Q, q_runoff, Some(0.0), None)?;
        if q_runoff <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::FluxSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(WB12_SYMBOL_RUNOFF_Q),
                value: q_runoff,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
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
        if !remax.is_finite() || remax <= WB11_ZERO_THRESHOLD {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from("intsty_0001"),
                value: remax,
                minimum: Some(WB11_ZERO_THRESHOLD),
                maximum: None,
            });
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

        let Ok(status) =
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HKERNEL-WB16-PEAK-OK-001")
        else {
            unreachable!("status message ids are non-empty WB16 constants")
        };

        let writeback = KernelWritebackPayload::with_updates(
            vec![
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
                WritebackField::bounded(
                    WB16_SYMBOL_QPSTAR,
                    qpstar,
                    Some(WB11_ZERO_THRESHOLD),
                    None,
                ),
                WritebackField::bounded(
                    WB16_SYMBOL_VSTAR,
                    vstar,
                    Some(WB11_ZERO_THRESHOLD),
                    Some(1.0),
                ),
            ],
            Vec::new(),
        );
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
            } else {
                if let Err(source) = hydrology_phase_dispatch_for_phase(phase, phase_class) {
                    let boundary_status = match SimulationStatus::failure(
                        SimulationPhase::HillslopeKernel,
                        true,
                        false,
                        source.boundary_class(),
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

                if let Err(source) =
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
        for (root, value) in [
            ("btemp", 10.0),
            ("otemp", 25.0),
            ("gddmax", 1700.0),
            ("dlai", 0.85),
            ("dropfc", 0.98),
            ("decfct", 0.65),
            ("spriod", 30.0),
            ("bb", 3.6),
            ("beinp", 35.00196),
            ("extnct", 0.65),
            ("hi", 0.5),
            ("xmxlai", 3.5),
            ("rsr", 0.25),
            ("rtmmax", 3.0),
            ("rdmax", 1.51995),
        ] {
            state_surface.insert(
                BoundarySymbol::from(format!("pl_growth_slot_0001_crop_0001_{root}")),
                BoundaryValue::scalar(value),
            );
        }
        state_surface.insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(25.0));
        state_surface.insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(13.0));
        state_surface.insert(BoundarySymbol::from("rad"), BoundaryValue::scalar(210.0));
        state_surface.insert(BoundarySymbol::from("prcp"), BoundaryValue::scalar(0.003));
        state_surface.insert(BoundarySymbol::from("Ws"), BoundaryValue::scalar(0.8));
        state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
        state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(2.0));
        state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(1.0));
        state_surface.insert(BoundarySymbol::from("thetdr"), BoundaryValue::scalar(0.15));
        state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(0.35));
        state_surface.insert(BoundarySymbol::from("ssc"), BoundaryValue::scalar(0.2));
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
        for (root, value) in [("oratea", 0.0065), ("orater", 0.0065)] {
            state_surface.insert(
                BoundarySymbol::from(format!("pl_decomp_slot_0001_crop_0001_{root}")),
                BoundaryValue::scalar(value),
            );
            state_surface.insert(BoundarySymbol::from(root), BoundaryValue::scalar(value));
        }

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
        for slot_index in 1..=6 {
            for (root, value) in [
                ("btemp", 10.0),
                ("otemp", 25.0),
                ("gddmax", 1700.0),
                ("dlai", 0.85),
                ("dropfc", 0.98),
                ("decfct", 0.65),
                ("spriod", 30.0),
                ("bb", 3.6),
                ("beinp", 35.00196),
                ("extnct", 0.65),
                ("hi", 0.5),
                ("xmxlai", 3.5),
                ("rsr", 0.25),
                ("rtmmax", 3.0),
                ("rdmax", 1.51995),
            ] {
                state.insert(
                    BoundarySymbol::from(format!(
                        "pl_growth_slot_{slot_index:04}_crop_0001_{root}"
                    )),
                    BoundaryValue::scalar(value),
                );
            }
            for (root, value) in [("oratea", 0.0065), ("orater", 0.0065)] {
                state.insert(
                    BoundarySymbol::from(format!(
                        "pl_decomp_slot_{slot_index:04}_crop_0001_{root}"
                    )),
                    BoundaryValue::scalar(value),
                );
            }
        }

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
    fn wb10_contract_conformance_hydrology_phase_classes_are_not_generic() {
        #[derive(Default)]
        struct ProbeKernel {
            observed_phase_classes: BTreeMap<String, String>,
        }

        impl HillslopeKernel for ProbeKernel {
            fn run_hillslope_phase(
                &mut self,
                request: &HillslopeKernelRequest<'_>,
            ) -> KernelRunResponse {
                if matches!(
                    request.phase_name,
                    "evapotranspiration"
                        | "percolation_deep_seepage"
                        | "lateral_transfer"
                        | "drainage"
                        | "runoff_reconciliation"
                        | "storage_reconciliation"
                        | "closure_diagnostics"
                ) {
                    self.observed_phase_classes.insert(
                        request.phase_name.to_owned(),
                        request.phase_class.as_str().to_owned(),
                    );
                }

                let status = openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::HillslopeKernel,
                    "HSCHED-TEST-WB10-PHASE-CLASS",
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
            .expect("wb10 phase-class conformance probe should execute");

        assert!(report.scheduler_report.is_success());
        assert_eq!(
            kernel.observed_phase_classes.get("evapotranspiration"),
            Some(&"hydrology_evapotranspiration".to_owned())
        );
        assert_eq!(
            kernel
                .observed_phase_classes
                .get("percolation_deep_seepage"),
            Some(&"hydrology_percolation_deep_seepage".to_owned())
        );
        assert_eq!(
            kernel.observed_phase_classes.get("lateral_transfer"),
            Some(&"hydrology_lateral_transfer".to_owned())
        );
        assert_eq!(
            kernel.observed_phase_classes.get("drainage"),
            Some(&"hydrology_drainage".to_owned())
        );
        assert_eq!(
            kernel.observed_phase_classes.get("runoff_reconciliation"),
            Some(&"hydrology_runoff_reconciliation".to_owned())
        );
        assert_eq!(
            kernel.observed_phase_classes.get("storage_reconciliation"),
            Some(&"hydrology_storage_reconciliation".to_owned())
        );
        assert_eq!(
            kernel.observed_phase_classes.get("closure_diagnostics"),
            Some(&"hydrology_peak_runoff".to_owned())
        );
    }

    #[test]
    fn wb10_contract_conformance_rejects_unsupported_hydrology_phase_class() {
        let error = super::hydrology_phase_dispatch_for_phase(
            HillslopePhase::Evapotranspiration,
            HillslopeKernelPhaseClass::Hydrology,
        )
        .expect_err("evapotranspiration must not accept generic hydrology class");

        assert_eq!(error.code(), "HS-HYDRO-E-001");
        assert_eq!(error.boundary_class(), BoundaryClass::DomainViolation);
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
                    phase_class if phase_class.is_hydrology_phase() => {
                        assert!(request.growth_context.is_none());
                        assert!(request.decomposition_context.is_none());
                    }
                    _ => unreachable!("unexpected phase class for annual growth test"),
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
                    phase_class if phase_class.is_hydrology_phase() => {
                        assert!(request.growth_context.is_none());
                        assert!(request.decomposition_context.is_none());
                    }
                    _ => unreachable!("unexpected phase class for perennial growth test"),
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
                    phase_class if phase_class.is_hydrology_phase() => {}
                    _ => unreachable!("unexpected phase class for active-slot perennial test"),
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
                    phase_class if phase_class.is_hydrology_phase() => {}
                    _ => unreachable!("unexpected phase class for active-slot annual test"),
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
