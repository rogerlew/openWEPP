#[allow(clippy::wildcard_imports)]
use super::*;

pub(crate) const PHASE_COUNT: usize = 13;
pub(crate) const RUNOFF_SLOPE_REQUIRED_STATE_SYMBOLS: &[&str] =
    &["nslpts", "slplen", "avgslp", "xinput_0001", "slpinp_0001"];
pub(crate) const RUNOFF_SOIL_REQUIRED_STATE_SYMBOLS: &[&str] =
    &["nsl", "solthk", "thetdr", "thetfc", "ssc"];
pub(crate) const SOIL_REQUIRED_STATE_SYMBOLS: &[&str] =
    &["nsl", "solthk", "dg", "thetdr", "thetfc", "ssc"];
pub(crate) const WATBAL_REQUIRED_STATE_SYMBOLS: &[&str] =
    &["nsl", "solthk", "thetdr", "thetfc", "ssc"];
pub(crate) const PERC_REQUIRED_STATE_SYMBOLS: &[&str] = &["nsl", "thetdr", "thetfc", "ssc"];
pub(crate) const SLOPE_FAMILY_SENTINELS: &[&str] =
    &["nelem", "nwsofe", "nslpts", "slplen", "avgslp"];
pub(crate) const SOIL_FAMILY_SENTINELS: &[&str] =
    &["nsl", "solthk", "dg", "thetdr", "thetfc", "ssc"];
pub(crate) const PL_GROWTH_RUNTIME_SENTINEL: &str = "pl_schedule_slot_count";
pub(crate) const PL_DECOMP_RUNTIME_SENTINEL: &str = "pl_schedule_slot_count";
pub(crate) const PL_SCHEDULE_SLOT_COUNT_SYMBOL: &str = "pl_schedule_slot_count";
pub(crate) const PL_SCHEDULE_ROTATION_REPEATS_SYMBOL: &str = "pl_schedule_rotation_repeats";
pub(crate) const PL_SCHEDULE_ROTATION_YEARS_SYMBOL: &str = "pl_schedule_rotation_years";
pub(crate) const PL_SCHEDULE_SLOT_ROTATION_INDEX_ROOT: &str = "rotation_index";
pub(crate) const PL_SCHEDULE_SLOT_OFE_INDEX_ROOT: &str = "ofe_index";
pub(crate) const PL_SCHEDULE_SLOT_YEAR_IN_ROTATION_ROOT: &str = "year_in_rotation";
pub(crate) const PL_SCHEDULE_SLOT_CROP_SLOTS_ROOT: &str = "crop_slots";
pub(crate) const PL_SCHEDULE_SLOT_CROP_IMNGMT_ROOT: &str = "imngmt";
pub(crate) const PL_RUNTIME_DAY_SYMBOL: &str = "day";
pub(crate) const PL_RUNTIME_YEAR_SYMBOL: &str = "year";
pub(crate) const PL_PRIMARY_OFE_INDEX: usize = 1;
pub(crate) const PL_DECOMP_IRESD_SEED_SYMBOL: &str = "iresd_seed";
pub(crate) const PL_DECOMP_SUMRTM_SEED_SYMBOL: &str = "sumrtm_seed";
pub(crate) const PL_DECOMP_SUMSRM_SEED_SYMBOL: &str = "sumsrm_seed";
pub(crate) const PL_ORDER_DECOMP_BEFORE_SOIL_SYMBOL: &str = "pl_order_decomp_before_soil";
pub(crate) const PL_ORDER_GROWTH_AFTER_DECOMP_SYMBOL: &str = "pl_order_growth_after_decomp";
pub(crate) const PL_ORDER_WATBAL_AFTER_GROWTH_SYMBOL: &str = "pl_order_watbal_after_growth";
pub(crate) const PL_GROWTH_STATE_SUMGDD_SYMBOL: &str = "sumgdd";
pub(crate) const PL_GROWTH_STATE_VDMT_SYMBOL: &str = "vdmt";
pub(crate) const PL_GROWTH_STATE_CANCOV_SYMBOL: &str = "cancov";
pub(crate) const PL_GROWTH_STATE_LAI_SYMBOL: &str = "lai";
pub(crate) const PL_GROWTH_STATE_RTMASS_SYMBOL: &str = "rtmass";
pub(crate) const PL_GROWTH_STATE_RTD_SYMBOL: &str = "rtd";
pub(crate) const PL_GROWTH_STATE_HIA_SYMBOL: &str = "hia";
pub(crate) const PL_GROWTH_CLIMATE_TMAX_SYMBOL: &str = "tmax";
pub(crate) const PL_GROWTH_CLIMATE_TMIN_SYMBOL: &str = "tmin";
pub(crate) const PL_GROWTH_CLIMATE_RAD_SYMBOL: &str = "rad";
pub(crate) const PL_GROWTH_CLIMATE_OBMAX_ROOT: &str = "obmaxt";
pub(crate) const PL_GROWTH_CLIMATE_OBMIN_ROOT: &str = "obmint";
pub(crate) const PL_DECOMP_CLIMATE_TMAX_SYMBOL: &str = "tmax";
pub(crate) const PL_DECOMP_CLIMATE_TMIN_SYMBOL: &str = "tmin";
pub(crate) const PL_DECOMP_CLIMATE_PRCP_SYMBOL: &str = "prcp";
pub(crate) const PL_GROWTH_SOIL_DEPTH_SYMBOL: &str = "solthk";
pub(crate) const PL_GROWTH_WATER_STRESS_SYMBOL: &str = "Ws";
pub(crate) const PL_DECOMP_PARAM_ORATEA_ROOT: &str = "oratea";
pub(crate) const PL_DECOMP_PARAM_ORATER_ROOT: &str = "orater";
pub(crate) const PL_DECOMP_TEMP_ATEMP: f64 = 6.1;
pub(crate) const PL_DECOMP_TEMP_ACTIVE_UPPER: f64 = 49.2;
pub(crate) const PL_DECOMP_TEMP_T2: f64 = 1528.81;
pub(crate) const PL_DECOMP_STANDING_RAIN_SATURATION: f64 = 0.004;
pub(crate) const PL_GROWTH_PARAM_BTEMP_ROOT: &str = "btemp";
pub(crate) const PL_GROWTH_PARAM_OTEMP_ROOT: &str = "otemp";
pub(crate) const PL_GROWTH_PARAM_GDDMAX_ROOT: &str = "gddmax";
pub(crate) const PL_GROWTH_PARAM_DLAI_ROOT: &str = "dlai";
pub(crate) const PL_GROWTH_PARAM_DROPFC_ROOT: &str = "dropfc";
pub(crate) const PL_GROWTH_PARAM_DECFCT_ROOT: &str = "decfct";
pub(crate) const PL_GROWTH_PARAM_SPRIOD_ROOT: &str = "spriod";
pub(crate) const PL_GROWTH_PARAM_BB_ROOT: &str = "bb";
pub(crate) const PL_GROWTH_PARAM_BEINP_ROOT: &str = "beinp";
pub(crate) const PL_GROWTH_PARAM_EXTNCT_ROOT: &str = "extnct";
pub(crate) const PL_GROWTH_PARAM_HI_ROOT: &str = "hi";
pub(crate) const PL_GROWTH_PARAM_XMXLAI_ROOT: &str = "xmxlai";
pub(crate) const PL_GROWTH_PARAM_RSR_ROOT: &str = "rsr";
pub(crate) const PL_GROWTH_PARAM_RTMMAX_ROOT: &str = "rtmmax";
pub(crate) const PL_GROWTH_PARAM_RDMAX_ROOT: &str = "rdmax";
pub(crate) const PL_GROWTH_PAR_RAD_SCALE: f64 = 0.02092;
pub(crate) const PL_GROWTH_PAR_LAI_OFFSET: f64 = 0.05;
pub(crate) const PL_GROWTH_DDM_SCALE: f64 = 0.0001;
pub(crate) const PL_GROWTH_ANNUAL_LAI_A: f64 = 0.5512;
pub(crate) const PL_GROWTH_ANNUAL_LAI_B: f64 = 6.8;
pub(crate) const PL_GROWTH_PERENNIAL_LAI_A: f64 = 0.2756;
pub(crate) const PL_GROWTH_PERENNIAL_LAI_B: f64 = 13.6;
pub(crate) const PL_GROWTH_ROOT_DEPTH_CURVE_A: f64 = 3.03;
pub(crate) const PL_GROWTH_ROOT_DEPTH_CURVE_B: f64 = 1.47;
pub(crate) const PL_GROWTH_CANCOV_MAX: f64 = 0.999;
pub(crate) const PL_GROWTH_GDMAX_YEAR_END_DAY: usize = 365;
pub(crate) const PL_GROWTH_GDMAX_MONTH_DAY_STARTS: [usize; 13] =
    [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366];
pub(crate) const PL_GROWTH_GDMAX_MONTH_LENGTHS: [usize; 12] =
    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub(crate) const ORDER_FLAG_EPSILON: f64 = 1.0e-12;
pub(crate) const MANAGEMENT_CLASS_EPSILON: f64 = 1.0e-9;
pub(crate) const WB11_ZERO_THRESHOLD: f64 = 1.0e-12;
pub(crate) const WB11_SYMBOL_SOIL_WATER: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb11SoilWater;
pub(crate) const WB11_SYMBOL_ET_DEMAND: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb11EtDemand;
pub(crate) const WB17_SYMBOL_RESIDUE_INTERCEPTION: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb17ResidueInterception;
pub(crate) const WB11_SYMBOL_FIELD_CAPACITY: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb11FieldCapacity;
pub(crate) const WB11_SYMBOL_PERC_FRACTION: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb11PercFraction;
pub(crate) const WB11_SYMBOL_DRAINAGE_COEFFICIENT: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb11DrainageCoefficient;
pub(crate) const WB11_SYMBOL_DRAINABLE_STORAGE: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb11DrainableStorage;
pub(crate) const WB11_SYMBOL_ET: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb11Et;
pub(crate) const WB11_SYMBOL_WS: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb11Ws;
pub(crate) const WB17_SYMBOL_EP: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb17PlantTranspirationEp;
pub(crate) const WB17_SYMBOL_ES: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb17SoilEvaporationEs;
pub(crate) const WB17_SYMBOL_ER: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb17ResidueEvaporationEr;
pub(crate) const WB17_STAGE_SYMBOL_S1: &str = "s1";
pub(crate) const WB17_STAGE_SYMBOL_S2: &str = "s2";
pub(crate) const WB17_STAGE_SYMBOL_TU: &str = "tu";
pub(crate) const WB17_STAGE_SYMBOL_TV: &str = "tv";
pub(crate) const WB17_FLUX_SYMBOL_ETP: &str = "Etp";
pub(crate) const WB17_FLUX_SYMBOL_UPI: &str = "UPi";
pub(crate) const WB17_FLUX_SYMBOL_UI: &str = "Ui";
pub(crate) const WB13_STATE_SYMBOL_WATCON: &str = "watcon";
pub(crate) const WB13_STATE_SYMBOL_TOTAL_SOIL: &str = "Total-Soil";
pub(crate) const WB13_STATE_SYMBOL_SOIL_WATER_TOTAL: &str = "SoilWaterTotal";
pub(crate) const WB17_STAGE_ONE_DEFICIT_SCALE: f64 = 0.4;
pub(crate) const WB17_STAGE_TWO_DEFICIT_SCALE: f64 = 0.6;
pub(crate) const WB17_STAGE_TWO_DENOMINATOR: f64 = 0.0035;
pub(crate) const WB13_DEPTH_TO_MM: f64 = 1000.0;
pub(crate) const WB11_SYMBOL_PERC_LOSS_D: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb11PercLossD;
pub(crate) const WB11_SYMBOL_PERC_RECHARGE_PE: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb11PercRechargePe;
pub(crate) const WB11_SYMBOL_LATERAL_Q: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb11LateralQ;
pub(crate) const WB11_SYMBOL_DRAINAGE_QDD: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb11DrainageQdd;
pub(crate) const WB11_SYMBOL_SUBHYD_QD: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb11SubhydQd;
pub(crate) const WB19_SYMBOL_AVG_SLOPE: &str = "avgslp";
pub(crate) const WB19_SYMBOL_SLOPE_LENGTH: &str = "slplen";
pub(crate) const WB19_SYMBOL_LATERAL_ANISOTROPY_RATIO: &str = "wb19_lateral_anisotropy_ratio";
pub(crate) const WB19_SYMBOL_DRAIN_ENABLED: &str = "wb19_drain_enabled";
pub(crate) const WB19_SYMBOL_DRAIN_DEPTH: &str = "wb19_drain_depth";
pub(crate) const WB19_SYMBOL_DRAIN_SPACING: &str = "wb19_drain_spacing";
pub(crate) const WB19_SYMBOL_DRAIN_DIAMETER: &str = "wb19_drain_diameter";
pub(crate) const WB19_SYMBOL_SATURATED_DEPTH_FCDEP: &str = "wb19_fcdep";
pub(crate) const WB19_SYMBOL_UNSATURATED_DEPTH_UNSDEP: &str = "wb19_unsdep";
pub(crate) const WB19_SYMBOL_WATER_YIELD_WATYLD: &str = "wb19_watyld";
pub(crate) const WB20_SYMBOL_FORWARD_SOLVER_LANE_ENABLED: &str = "wb20_forward_solver_lane_enabled";
pub(crate) const WB19_DRAIN_ALPHA: f64 = 3.4;
pub(crate) const WB19_DRAIN_HOURS_PER_DAY: f64 = 24.0;
pub(crate) const WB12_SYMBOL_RAINFALL_INPUT: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb12RainfallInput;
pub(crate) const WB12_SYMBOL_RUNON_INPUT: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb12RunonInput;
pub(crate) const WB12_SYMBOL_INFILTRATION: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb12Infiltration;
pub(crate) const WB12_SYMBOL_DEPRESSION_STORAGE_DELTA: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb12DepressionStorageDelta;
pub(crate) const WB12_SYMBOL_RUNOFF_OBSERVED: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb12RunoffObserved;
pub(crate) const WB12_SYMBOL_RUNOFF_CLOSURE_TOLERANCE: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb12RunoffClosureTolerance;
pub(crate) const WB12_SYMBOL_RUNOFF_CLOSURE_DELTA: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb12RunoffClosureDelta;
pub(crate) const WB12_SYMBOL_RUNOFF_RECONCILED: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb12RunoffReconciled;
pub(crate) const WB12_SYMBOL_STORAGE_INITIAL: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb12StorageInitial;
pub(crate) const WB12_SYMBOL_STORAGE_OBSERVED: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb12StorageObserved;
pub(crate) const WB12_SYMBOL_STORAGE_CLOSURE_TOLERANCE: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb12StorageClosureTolerance;
pub(crate) const WB12_SYMBOL_PRECIP_INPUT: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb12PrecipInput;
pub(crate) const WB12_SYMBOL_STORAGE_CLOSURE_DELTA: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb12StorageClosureDelta;
pub(crate) const WB12_SYMBOL_STORAGE_RECONCILED: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb12StorageReconciled;
pub(crate) const WB12_SYMBOL_RUNOFF_Q: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb12RunoffQ;
pub(crate) const WB12_SYMBOL_SNOW_COUPLING_S: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb12SnowCouplingS;
pub(crate) const IRRIG_SYMBOL_DAILY_IRRIGATION: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::IrrigDailyIrrigation;
pub(crate) const IRRIG_SYMBOL_RUNTIME_SOURCE: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::IrrigRuntimeSource;
pub(crate) const IRRIG_SYMBOL_RUNTIME_DEPTH_M: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::IrrigRuntimeDepthMeters;
pub(crate) const IRRIG_SYMBOL_RUNTIME_DURATION_S: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::IrrigRuntimeDurationSeconds;
pub(crate) const IRRIG_SYMBOL_RUNTIME_RATE_MPS: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::IrrigRuntimeRateMetersPerSecond;
pub(crate) const IRRIG_SYMBOL_RUNTIME_EVENT_INDEX: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::IrrigRuntimeEventIndex;
pub(crate) const IRRIG_SYMBOL_RUNTIME_SYSTEM_TYPE: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::IrrigRuntimeSystemType;
pub(crate) const IRRIG_SYMBOL_DEPLETION_ENABLED: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::IrrigDepletionEnabled;
pub(crate) const IRRIG_SYMBOL_DEPLETION_SYSTEM_TYPE: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::IrrigDepletionSystemType;
pub(crate) const IRRIG_SYMBOL_DEPLETION_MIN_DEPTH_M: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::IrrigDepletionMinDepthMeters;
pub(crate) const IRRIG_SYMBOL_DEPLETION_MAX_DEPTH_M: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::IrrigDepletionMaxDepthMeters;
pub(crate) const IRRIG_SYMBOL_DEPLETION_PERIOD_COUNT: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::IrrigDepletionPeriodCount;
pub(crate) const IRRIG_SYMBOL_FIXEDDATE_ENABLED: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::IrrigFixedDateEnabled;
pub(crate) const IRRIG_SYMBOL_FIXEDDATE_SYSTEM_TYPE: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::IrrigFixedDateSystemType;
pub(crate) const IRRIG_SYMBOL_FIXEDDATE_EVENT_COUNT: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::IrrigFixedDateEventCount;
pub(crate) const WB15_SYMBOL_INTERCEPTION_I: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb15InterceptionI;
pub(crate) const WB15_SYMBOL_PLANT_CANCOV: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb15PlantCancov;
pub(crate) const WB15_SYMBOL_PLANT_LAI: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb15PlantLai;
pub(crate) const WB15_SYMBOL_PLANT_VDMT: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb15PlantVdmt;
pub(crate) const WB15_CANCOV_MAX: f64 = 0.999;
pub(crate) const WB15_VDMT_MAX: f64 = 0.8;
pub(crate) const WB15_BIOMASS_TO_KG_HA: f64 = 10_000.0;
pub(crate) const WB15_INTERCEPT_LINEAR_COEFF: f64 = 0.000_627;
pub(crate) const WB15_INTERCEPT_QUADRATIC_COEFF: f64 = 3.733_49e-8;
pub(crate) const WB15_INTERCEPT_MM_TO_M: f64 = 1000.0;
pub(crate) const WB14_SYMBOL_HYETOGRAPH_NINTEN: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14HyetographNinten;
pub(crate) const WB14_SYMBOL_HYETOGRAPH_NBRKPT: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14HyetographNbrkpt;
pub(crate) const WB14_SYMBOL_SOIL_CONDUCTIVITY: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SoilConductivity;
pub(crate) const WB14_SYMBOL_SOIL_LAYER_DEPTH: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SoilLayerDepth;
pub(crate) const WB14_SYMBOL_SOIL_THETA_RESIDUAL: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SoilThetaResidual;
pub(crate) const WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SoilThetaFieldCapacity;
pub(crate) const WB14_SYMBOL_SNOW_FILE_PRESENT: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SnowFilePresent;
pub(crate) const WB14_SYMBOL_SNOW_RST: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SnowRst;
pub(crate) const WB14_SYMBOL_SNOW_NEWSNW: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SnowNewsnw;
pub(crate) const WB14_SYMBOL_SNOW_SSD: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SnowSsd;
pub(crate) const WB14_SYMBOL_SNOW_RUNTIME_SWE: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SnowRuntimeSwe;
pub(crate) const WB14_SYMBOL_FROST_FILE_PRESENT: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostFilePresent;
pub(crate) const WB14_SYMBOL_FROST_WINT_RED: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostWintRed;
pub(crate) const WB14_SYMBOL_FROST_FINE_TOP: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostFineTop;
pub(crate) const WB14_SYMBOL_FROST_FINE_BOT: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostFineBot;
pub(crate) const WB14_SYMBOL_FROST_KSNOWF: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostKsnowf;
pub(crate) const WB14_SYMBOL_FROST_KRESF: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostKresf;
pub(crate) const WB14_SYMBOL_FROST_KSOILF: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostKsoilf;
pub(crate) const WB14_SYMBOL_FROST_KFACTOR1: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostKfactor1;
pub(crate) const WB14_SYMBOL_FROST_KFACTOR2: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostKfactor2;
pub(crate) const WB14_SYMBOL_FROST_KFACTOR3: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostKfactor3;
pub(crate) const WB14_SYMBOL_FROST_RUNTIME_DFROST: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostRuntimeDfrost;
pub(crate) const WB14_SYMBOL_FROST_RUNTIME_DTHAW: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostRuntimeDthaw;
pub(crate) const WB14_SYMBOL_FROST_RUNTIME_NFT: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostRuntimeNft;
pub(crate) const WB14_SYMBOL_FROST_RUNTIME_WS_FRZ: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostRuntimeWsFrz;
pub(crate) const WB14_SYMBOL_FROST_RUNTIME_INFCAP_FRZ: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14FrostRuntimeInfcapFrz;
pub(crate) const WB14_SYMBOL_TMAX: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14Tmax;
pub(crate) const WB14_SYMBOL_TMIN: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14Tmin;
pub(crate) const WB14_FROST_MAX_DEPTH_M: f64 = 0.20;
pub(crate) const WB16_SYMBOL_EFFLEN: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb16Efflen;
pub(crate) const WB16_SYMBOL_EALPHA: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb16Ealpha;
pub(crate) const WB16_SYMBOL_EXPONENT_M: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb16ExponentM;
pub(crate) const WB16_SYMBOL_PEAKRO: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb16Peakro;
pub(crate) const WB16_SYMBOL_WATDUR: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb16Watdur;
pub(crate) const WB16_SYMBOL_METHOD_BRANCH: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb16MethodBranch;
pub(crate) const WB16_SYMBOL_TSTAR: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb16Tstar;
pub(crate) const WB16_SYMBOL_QPSTAR: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb16Qpstar;
pub(crate) const WB16_SYMBOL_VSTAR: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb16Vstar;
pub(crate) const WB16_RUNOFF_NEAR_ZERO_THRESHOLD: f64 = 1.0e-8;
pub(crate) const WB16_PEAKRO_FLOOR: f64 = 3.63e-8;
pub(crate) const WB16_MAX_DURATION_S: f64 = 86_400.0;
pub(crate) const EROD13_SYMBOL_CORE_ENABLED: &str = "erod13_core_enabled";
pub(crate) const EROD13_SYMBOL_IE: &str = "Ie";
pub(crate) const EROD13_SYMBOL_TE: &str = "te";
pub(crate) const EROD13_SYMBOL_FS: &str = "fs";
pub(crate) const EROD13_SYMBOL_FT: &str = "ft";
pub(crate) const EROD13_SYMBOL_TAUFE: &str = "taufe";
pub(crate) const EROD13_SYMBOL_Q: &str = "q";
pub(crate) const EROD13_SYMBOL_G: &str = "G";
pub(crate) const EROD13_SYMBOL_DI: &str = "Di";
pub(crate) const EROD13_SYMBOL_BETA: &str = "beta";
pub(crate) const EROD13_SYMBOL_VF: &str = "vf";
pub(crate) const EROD13_SYMBOL_DGDX: &str = "dGdx";
pub(crate) const EROD13_SYMBOL_CNTLEN: &str = "cntlen";
pub(crate) const EROD13_SYMBOL_KR: &str = "kr";
pub(crate) const EROD13_SYMBOL_KRADJF: &str = "kradjf";
pub(crate) const EROD13_SYMBOL_TCADJF: &str = "tcadjf";
pub(crate) const EROD13_SYMBOL_SHRSOL: &str = "shrsol";
pub(crate) const EROD13_SYMBOL_TCEND: &str = "tcend";
pub(crate) const EROD13_SYMBOL_SHCRIT: &str = "shcrit";
pub(crate) const EROD13_SYMBOL_DETINR: &str = "detinr";
pub(crate) const EROD13_SYMBOL_EFFDRR: &str = "effdrr";
pub(crate) const EROD13_SYMBOL_EFFDRN: &str = "effdrn";
pub(crate) const EROD13_SYMBOL_VELEFF: &str = "veleff";
pub(crate) const EROD13_SYMBOL_PKRO: &str = "pkro";
pub(crate) const EROD13_SYMBOL_TC_K: &str = "erod13_tc_k";
pub(crate) const EROD13_SYMBOL_TC_M: &str = "erod13_tc_m";
pub(crate) const EROD13_SYMBOL_DC: &str = "Dc";
pub(crate) const EROD13_SYMBOL_TC: &str = "Tc";
pub(crate) const EROD13_SYMBOL_DF: &str = "Df";
pub(crate) const EROD13_SYMBOL_ETA: &str = "eta";
pub(crate) const EROD13_SYMBOL_TAUCN: &str = "taucn";
pub(crate) const EROD13_SYMBOL_THETA: &str = "theta";
pub(crate) const EROD13_SYMBOL_PHI: &str = "phi";
pub(crate) const EROD13_CONTINUITY_TOLERANCE: f64 = 1.0e-9;
pub(crate) const EROD13_MIN_TCADJF: f64 = 0.30;
pub(crate) const EROD14_SYMBOL_WAVE2_ENABLED: &str = "erod14_wave2_enabled";
pub(crate) const EROD14_SYMBOL_CLASS_COUNT: &str = "erod14_class_count";
pub(crate) const EROD14_SYMBOL_XTOP: &str = "erod14_xtop";
pub(crate) const EROD14_SYMBOL_XBOT: &str = "erod14_xbot";
pub(crate) const EROD14_SYMBOL_XDETST: &str = "erod14_xdetst";
pub(crate) const EROD14_SYMBOL_LDTOP: &str = "erod14_ldtop";
pub(crate) const EROD14_SYMBOL_LDBOT: &str = "erod14_ldbot";
pub(crate) const EROD14_SYMBOL_LDDEND: &str = "erod14_lddend";
pub(crate) const EROD14_SYMBOL_QOUT: &str = "erod14_qout";
pub(crate) const EROD14_SYMBOL_QIN: &str = "erod14_qin";
pub(crate) const EROD14_SYMBOL_QOSTAR: &str = "erod14_qostar";
pub(crate) const EROD14_SYMBOL_SLP_LEN: &str = "erod14_slplen";
pub(crate) const EROD14_SYMBOL_KTRATO: &str = "erod14_ktrato";
pub(crate) const EROD14_SYMBOL_AINTC: &str = "erod14_ainftc";
pub(crate) const EROD14_SYMBOL_BINTC: &str = "erod14_binftc";
pub(crate) const EROD14_SYMBOL_CINTC: &str = "erod14_cinftc";
pub(crate) const EROD14_SYMBOL_BETA: &str = "erod14_beta";
pub(crate) const EROD14_SYMBOL_QJ_MINUS_1: &str = "erod14_Qj_minus_1";
pub(crate) const EROD14_SYMBOL_VJ: &str = "erod14_Vj";
pub(crate) const EROD14_SYMBOL_QJ: &str = "erod14_Qj";
pub(crate) const EROD14_SYMBOL_FH: &str = "erod14_Fh";
pub(crate) const EROD14_SYMBOL_FP: &str = "erod14_Fp";
pub(crate) const EROD14_SYMBOL_CASE: &str = "erod14_case";
pub(crate) const EROD14_SYMBOL_SUMG: &str = "erod14_sumg";
pub(crate) const EROD14_SYMBOL_ER: &str = "ER";
pub(crate) const EROD14_SYMBOL_SSA_SOIL: &str = "erod14_ssa_soil";
pub(crate) const EROD14_ROOT_FALL: &str = "erod14_fall";
pub(crate) const EROD14_ROOT_FRCFLW: &str = "erod14_frcflw";
pub(crate) const EROD14_ROOT_FRAC: &str = "erod14_frac";
pub(crate) const EROD14_ROOT_FIDEL: &str = "erod14_fidel";
pub(crate) const EROD14_ROOT_TCF1: &str = "erod14_tcf1";
pub(crate) const EROD14_ROOT_SSA_CLASS: &str = "erod14_ssa_class";
pub(crate) const EROD14_ROOT_GEND: &str = "erod14_gend";
pub(crate) const EROD14_ROOT_SEDMAX: &str = "erod14_sedmax";
pub(crate) const EROD14_ROOT_SED_FRAC: &str = "sed_frac";
pub(crate) const EROD15_SYMBOL_TOTAL_DETACHMENT_KG: &str = "total_detachment_kg";
pub(crate) const EROD15_SYMBOL_TOTAL_DEPOSITION_KG: &str = "total_deposition_kg";
pub(crate) const EROD15_SYMBOL_PARTICLE_CLASS_COUNT: &str = "particle_class_count";
pub(crate) const EROD15_ROOT_SEDIMENT_CONCENTRATION_KG_M3: &str = "sediment_concentration_kg_m3";
pub(crate) const EROD15_ROOT_PARTICLE_FLOW_FRACTION: &str = "particle_flow_fraction";
pub(crate) const EROD18_SYMBOL_NSLPTS: &str = "nslpts";
pub(crate) const EROD18_ROUTE_SEGMENT_INDEX: usize = 2;
pub(crate) const EROD18_SYMBOL_QOSTAR: &str = "qostar";
pub(crate) const EROD18_SYMBOL_XDBEG: &str = "xdbeg";
pub(crate) const EROD18_SYMBOL_XDEND: &str = "xdend";
pub(crate) const EROD18_SYMBOL_XDETST: &str = "xdetst";
pub(crate) const EROD18_SYMBOL_LDLAST: &str = "ldlast";
pub(crate) const EROD18_SYMBOL_LDDEND: &str = "lddend";
pub(crate) const EROD18_SYMBOL_DU: &str = "du";
pub(crate) const EROD18_SYMBOL_DL: &str = "dl";
pub(crate) const EROD18_SYMBOL_NDEP: &str = "ndep";
pub(crate) const EROD18_SYMBOL_MSHEAR: &str = "mshear";
pub(crate) const EROD18_SYMBOL_XC1: &str = "xc1";
pub(crate) const EROD18_SYMBOL_XC2: &str = "xc2";
pub(crate) const EROD18_ROOT_XU: &str = "xu";
pub(crate) const EROD18_ROOT_XL: &str = "xl";
pub(crate) const EROD18_ROOT_AINF: &str = "ainf";
pub(crate) const EROD18_ROOT_BINF: &str = "binf";
pub(crate) const EROD18_ROOT_CINF: &str = "cinf";
pub(crate) const EROD18_ROOT_AINTC: &str = "ainftc";
pub(crate) const EROD18_ROOT_BINTC: &str = "binftc";
pub(crate) const EROD18_ROOT_CINTC: &str = "cinftc";
pub(crate) const EROD14_PKRO_ZERO_THRESHOLD: f64 = 1.0e-15;
pub(crate) const EROD14_CLASS_FLOOR: f64 = 1.0e-15;
pub(crate) const EROD14_CASE_TOLERANCE: f64 = 1.0e-12;
pub(crate) const EROD14_CASE_MIN: i32 = 1;
pub(crate) const EROD14_CASE_MAX: i32 = 4;
pub(crate) const EROD14_BASE_UPDATE_FIELD_COUNT: usize = 5;
pub(crate) const EROD14_CLASS_UPDATE_FIELD_COUNT: usize = 6;
pub(crate) const EROD14_ATTENUATION_FLOOR: f64 = 1.0e-8;
pub(crate) const EROD14_ENRICHMENT_RATIO_OFFSET: f64 = 0.005;
pub(crate) const EROD14_MAX_PHI: f64 = 100_000.0;
pub(crate) const EROD14_MAX_REPROPORTION_ITERS: usize = 64;
pub(crate) const EROD18_MSHEAR_MIN: f64 = 1.0;
pub(crate) const EROD18_MSHEAR_MAX: f64 = 5.0;
pub(crate) const EROD19_QOSTAR_NEAR_ZERO_THRESHOLD: f64 = 0.0011;
pub(crate) const EROD19_DEPC_QOSTAR_XU_EPSILON: f64 = 1.0e-7;
pub(crate) const EROD19_SHEAR_FLOOR: f64 = 0.0001;
pub(crate) const EROD19_UNIFORM_XC_SENTINEL: f64 = 1000.0;
pub(crate) const EROD19_DEPEND_INITIAL_STEP_POSITIVE: f64 = 0.01;
pub(crate) const EROD19_DEPEND_INITIAL_STEP_NEGATIVE: f64 = 0.0001;
pub(crate) const EROD19_DEPEND_XU_QOSTAR_NEAR_ZERO: f64 = 0.0001;
pub(crate) const EROD19_DEPEND_NEWTON_RESIDUAL_TOLERANCE: f64 = 0.001;
pub(crate) const EROD19_DEPEND_NEWTON_MAX_ITERS: usize = 10;
pub(crate) const EROD19_TAUC_FALLBACK_SCALE: f64 = 0.2;
pub(crate) const WB17_LAI_PARTITION_COEFFICIENT: f64 = 0.4;
pub(crate) const WB18_PERC_SATURATION_THRESHOLD: f64 = 0.95;
pub(crate) const WB18_PERC_MIN_FX: f64 = 0.002;
pub(crate) const WB18_PERC_SHAPE_EXPONENT: f64 = 1.0;
pub(crate) const WB18_PERC_TIMESTEP_S: f64 = 86_400.0;
