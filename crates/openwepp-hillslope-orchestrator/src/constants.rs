#[allow(clippy::wildcard_imports)]
use super::*;
pub(crate) const PL_RUNTIME_DAY_SYMBOL: &str = "day";
pub(crate) const PL_GROWTH_SOIL_DEPTH_SYMBOL: &str = "solthk";
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
pub(crate) const WB11_ZERO_THRESHOLD: f64 = 1.0e-12;
pub(crate) const WB11_SYMBOL_SOIL_WATER: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb11SoilWater;
pub(crate) const WB17_STAGE_ONE_DEFICIT_SCALE: f64 = 0.4;
pub(crate) const WB17_STAGE_TWO_DEFICIT_SCALE: f64 = 0.6;
pub(crate) const WB17_STAGE_TWO_DENOMINATOR: f64 = 0.0035;
pub(crate) const WB17_CANOPY_EAJ_COEFFICIENT: f64 = 0.5;
pub(crate) const WB17_CANOPY_BARE_SOIL_OFFSET: f64 = 0.1;
pub(crate) const WB17_TRANSPIRATION_LAI_FULL_COVER: f64 = 3.0;
pub(crate) const WB17_SOIL_EVAPORATION_DEPTH_M: f64 = 0.10;
pub(crate) const WB17_SWU_UB: f64 = 3.065;
pub(crate) const WB17_SWU_UOB: f64 = 0.953_346;
pub(crate) const WB17_PLTOL_MIN: f64 = 0.1;
pub(crate) const WB17_PLTOL_MAX: f64 = 0.4;
pub(crate) const MOFE_HOURLY_CARRY_ARRAY_COUNT: usize = 24;
pub(crate) const WB19_DRAIN_ALPHA: f64 = 3.4;
pub(crate) const WB19_DRAIN_HOURS_PER_DAY: f64 = 24.0;
pub(crate) const WB12_SYMBOL_RAINFALL_INPUT: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb12RainfallInput;
pub(crate) const WB12_SYMBOL_SNOW_COUPLING_S: HillslopeProductionFluxSymbol =
    HillslopeProductionFluxSymbol::Wb12SnowCouplingS;
pub(crate) const WB15_SYMBOL_PLANT_CANCOV: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb15PlantCancov;
pub(crate) const WB15_CANCOV_MAX: f64 = 0.999;
pub(crate) const WB15_BIOMASS_TO_KG_HA: f64 = 10_000.0;
pub(crate) const WB15_INTERCEPT_BIOMASS_MAX_KG_HA: f64 = 8_000.0;
pub(crate) const WB15_INTERCEPT_LINEAR_COEFF: f64 = 0.000_627;
pub(crate) const WB15_INTERCEPT_QUADRATIC_COEFF: f64 = 3.733_49e-8;
pub(crate) const WB15_INTERCEPT_MM_TO_M: f64 = 1000.0;
pub(crate) const WB14_SYMBOL_SOIL_CONDUCTIVITY: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SoilConductivity;
pub(crate) const WB14_SYMBOL_SOIL_THETA_RESIDUAL: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SoilThetaResidual;
pub(crate) const WB14_SYMBOL_SOIL_THETA_FIELD_CAPACITY: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SoilThetaFieldCapacity;
pub(crate) const WB14_SYMBOL_SNOW_RST: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SnowRst;
pub(crate) const WB14_SYMBOL_SNOW_NEWSNW: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SnowNewsnw;
pub(crate) const WB14_SYMBOL_SNOW_SSD: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SnowSsd;
pub(crate) const WB14_SYMBOL_SNOW_RUNTIME_SWE: HillslopeProductionStateSymbol =
    HillslopeProductionStateSymbol::Wb14SnowRuntimeSwe;
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
pub(crate) const WB16_RUNOFF_NEAR_ZERO_THRESHOLD: f64 = 1.0e-8;
pub(crate) const WB16_PEAKRO_FLOOR: f64 = 3.63e-8;
pub(crate) const WB16_MAX_DURATION_S: f64 = 86_400.0;
pub(crate) const WB18_PERC_SATURATION_THRESHOLD: f64 = 0.95;
pub(crate) const WB18_PERC_MIN_FX: f64 = 0.002;
pub(crate) const WB18_PERC_BI_COEFFICIENT: f64 = 2.655;
pub(crate) const WB18_PERC_TIMESTEP_S: f64 = 86_400.0;
pub(crate) const WB18_DEEP_PERCOLATION_ROUNDOFF_TOLERANCE_M: f64 = 1.0e-11;
pub(crate) const WB18_STORAGE_ROUNDOFF_TOLERANCE_M: f64 = 2.0e-11;
