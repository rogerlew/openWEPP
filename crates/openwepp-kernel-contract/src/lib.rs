#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]

//! Kernel invocation and writeback contract boundaries for openWEPP.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use openwepp_sim_contract::closure::{
    ClosureViolation, ClosureViolationKind, check_finite, check_max, check_min, check_range,
};
use openwepp_sim_contract::status::{
    BoundaryClass, SimulationPhase, SimulationStatus, StatusError,
};
pub use openwepp_unit_boundary::BoundaryError;
use openwepp_unit_boundary::{
    DensityKilogramsPerCubicMeter, DirectionDegrees, ElapsedTimeSeconds,
    FlowRateCubicMetersPerSecond, FractionUnitInterval, HourOfDay, LinearRateMetersPerSecond,
    ProcessRateMillimetersPerHour, RunoffDepthMillimeters, SolarRadiationLangleysPerDay,
    SolarRadiationMegajoulesPerSquareMeterPerDay, SolarRadiationMegajoulesPerSquareMeterPerHour,
    StorageVolumeCubicMeters, SurfaceAreaSquareMeters, TemperatureCelsius, WaterDepthMeters,
};

/// Message id emitted when writeback payload evaluation accepts all fields.
pub const WRITEBACK_ACCEPT_MESSAGE_ID: &str = "KWRITEBACK-ACCEPT-001";
/// Message id emitted when accepted writeback is applied by orchestrator.
pub const WRITEBACK_APPLY_MESSAGE_ID: &str = "KWRITEBACK-APPLY-001";
/// Message id emitted when writeback is rejected for non-finite values.
pub const WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID: &str = "KWRITEBACK-E-NON-FINITE";
/// Message id emitted when writeback is rejected for domain/range violations.
pub const WRITEBACK_REJECT_DOMAIN_MESSAGE_ID: &str = "KWRITEBACK-E-DOMAIN-VIOLATION";

/// Type-safe state/flux symbol key for kernel seam maps.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundarySymbol(String);

impl BoundarySymbol {
    #[must_use]
    pub fn new(symbol: impl Into<String>) -> Self {
        Self(symbol.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for BoundarySymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for BoundarySymbol {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for BoundarySymbol {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Maximum supported climate forcing series points for runtime symbol
/// projection.
pub const MAX_CLIMATE_FORCING_SERIES_POINTS: usize = 1_500;

/// Typed climate forcing symbol projection surface for `timem_*` and
/// `intsty_*` boundary aliases.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClimateForcingSymbolSurface {
    timem_symbols: Vec<BoundarySymbol>,
    intsty_symbols: Vec<BoundarySymbol>,
}

impl ClimateForcingSymbolSurface {
    /// Build canonical hillslope series symbols (`timem_XXXX`, `intsty_XXXX`).
    ///
    /// # Errors
    ///
    /// Returns `ClimateForcingSymbolSurfaceError` when point cardinality
    /// exceeds supported runtime bounds.
    pub fn hillslope(point_count: usize) -> Result<Self, ClimateForcingSymbolSurfaceError> {
        Self::build(None, point_count)
    }

    /// Build canonical watershed-hillslope scoped series symbols
    /// (`hs{id}_timem_XXXX`, `hs{id}_intsty_XXXX`).
    ///
    /// # Errors
    ///
    /// Returns `ClimateForcingSymbolSurfaceError` when point cardinality
    /// exceeds supported runtime bounds.
    pub fn watershed_hillslope(
        hillslope_id: u32,
        point_count: usize,
    ) -> Result<Self, ClimateForcingSymbolSurfaceError> {
        Self::build(Some(hillslope_id), point_count)
    }

    #[must_use]
    pub fn timem_symbols(&self) -> &[BoundarySymbol] {
        &self.timem_symbols
    }

    #[must_use]
    pub fn intsty_symbols(&self) -> &[BoundarySymbol] {
        &self.intsty_symbols
    }

    #[must_use]
    pub fn point_count(&self) -> usize {
        self.timem_symbols.len()
    }

    fn build(
        hillslope_id: Option<u32>,
        point_count: usize,
    ) -> Result<Self, ClimateForcingSymbolSurfaceError> {
        if point_count > MAX_CLIMATE_FORCING_SERIES_POINTS {
            return Err(ClimateForcingSymbolSurfaceError::PointCountOutOfRange {
                count: point_count,
                supported_max: MAX_CLIMATE_FORCING_SERIES_POINTS,
            });
        }

        let mut timem_symbols = Vec::with_capacity(point_count);
        let mut intsty_symbols = Vec::with_capacity(point_count);
        for index in 1..=point_count {
            timem_symbols.push(build_series_symbol(hillslope_id, "timem", index));
            intsty_symbols.push(build_series_symbol(hillslope_id, "intsty", index));
        }

        Ok(Self {
            timem_symbols,
            intsty_symbols,
        })
    }
}

/// Typed failure for climate forcing symbol-surface construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClimateForcingSymbolSurfaceError {
    PointCountOutOfRange { count: usize, supported_max: usize },
}

impl fmt::Display for ClimateForcingSymbolSurfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PointCountOutOfRange {
                count,
                supported_max,
            } => write!(
                f,
                "climate forcing point count {count} exceeds supported maximum {supported_max}"
            ),
        }
    }
}

impl Error for ClimateForcingSymbolSurfaceError {}

fn build_series_symbol(
    hillslope_id: Option<u32>,
    series: &str,
    one_based_index: usize,
) -> BoundarySymbol {
    match hillslope_id {
        Some(id) => BoundarySymbol::from(format!("hs{id}_{series}_{one_based_index:04}")),
        None => BoundarySymbol::from(format!("{series}_{one_based_index:04}")),
    }
}

/// Indexed irrigation depletion-period fields used by ARCH22 typed symbol
/// projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HillslopeIrrigationDepletionPeriodField {
    ElementId,
    StartDoy,
    StartYear,
    EndDoy,
    EndYear,
    DepletionTriggerRatio,
    SprinklerDepthRatio,
    SprinklerRateMetersPerSecond,
    SprinklerNozzleFactor,
}

impl HillslopeIrrigationDepletionPeriodField {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ElementId => "element_id",
            Self::StartDoy => "start_doy",
            Self::StartYear => "start_year",
            Self::EndDoy => "end_doy",
            Self::EndYear => "end_year",
            Self::DepletionTriggerRatio => "depletion_trigger_ratio",
            Self::SprinklerDepthRatio => "sprinkler_depth_ratio",
            Self::SprinklerRateMetersPerSecond => "sprinkler_rate_m_per_s",
            Self::SprinklerNozzleFactor => "sprinkler_nozzle_factor",
        }
    }
}

/// Indexed irrigation fixed-date event fields used by ARCH22 typed symbol
/// projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HillslopeIrrigationFixedDateEventField {
    OfeId,
    Day,
    Year,
    ScheduleTerminationFlag,
    SprinklerDepthMeters,
    SprinklerRateMetersPerSecond,
    SprinklerNozzleFactor,
}

impl HillslopeIrrigationFixedDateEventField {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfeId => "ofe_id",
            Self::Day => "day",
            Self::Year => "year",
            Self::ScheduleTerminationFlag => "schedule_termination_flag",
            Self::SprinklerDepthMeters => "sprinkler_depth_m",
            Self::SprinklerRateMetersPerSecond => "sprinkler_rate_m_per_s",
            Self::SprinklerNozzleFactor => "sprinkler_nozzle_factor",
        }
    }
}

/// Typed ARCH22 boundary-state symbols for covered production hillslope
/// kernels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HillslopeProductionStateSymbol {
    Wb11SoilWater,
    Wb11EtDemand,
    Wb17ResidueInterception,
    Wb11FieldCapacity,
    Wb11PercFraction,
    Wb11LateralFraction,
    Wb11DrainageFraction,
    Wb11DrainageCoefficient,
    Wb11DrainableStorage,
    Wb12RainfallInput,
    Wb12RunonInput,
    Wb12Infiltration,
    Wb12DepressionStorageDelta,
    Wb12RunoffObserved,
    Wb12RunoffClosureTolerance,
    Wb12RunoffReconciled,
    Wb12StorageInitial,
    Wb12StorageObserved,
    Wb12StorageClosureTolerance,
    Wb12PrecipInput,
    Wb12StorageReconciled,
    IrrigRuntimeSource,
    IrrigRuntimeDepthMeters,
    IrrigRuntimeDurationSeconds,
    IrrigRuntimeRateMetersPerSecond,
    IrrigRuntimeEventIndex,
    IrrigRuntimeSystemType,
    IrrigDepletionEnabled,
    IrrigDepletionSystemType,
    IrrigDepletionMinDepthMeters,
    IrrigDepletionMaxDepthMeters,
    IrrigDepletionPeriodCount,
    IrrigFixedDateEnabled,
    IrrigFixedDateSystemType,
    IrrigFixedDateEventCount,
    Wb15PlantCancov,
    Wb15PlantLai,
    Wb15PlantVdmt,
    Wb14HyetographNinten,
    Wb14HyetographNbrkpt,
    Wb14SoilConductivity,
    Wb14SoilLayerDepth,
    Wb14SoilThetaResidual,
    Wb14SoilThetaFieldCapacity,
    Wb14SnowFilePresent,
    Wb14SnowRst,
    Wb14SnowNewsnw,
    Wb14SnowSsd,
    Wb14SnowRuntimeSwe,
    Wb14FrostFilePresent,
    Wb14FrostWintRed,
    Wb14FrostFineTop,
    Wb14FrostFineBot,
    Wb14FrostKsnowf,
    Wb14FrostKresf,
    Wb14FrostKsoilf,
    Wb14FrostKfactor1,
    Wb14FrostKfactor2,
    Wb14FrostKfactor3,
    Wb14FrostRuntimeDfrost,
    Wb14FrostRuntimeDthaw,
    Wb14FrostRuntimeNft,
    Wb14FrostRuntimeWsFrz,
    Wb14FrostRuntimeInfcapFrz,
    Wb14Tmax,
    Wb14Tmin,
    Wb16Timep,
    Wb16Efflen,
    Wb16Ealpha,
    Wb16ExponentM,
    Wb16Peakro,
    Wb16Watdur,
    Wb16MethodBranch,
    Wb16Tstar,
    Wb16Qpstar,
    Wb16Vstar,
    IrrigationDepletionPeriod {
        period_index: usize,
        field: HillslopeIrrigationDepletionPeriodField,
    },
    IrrigationFixedDateEvent {
        event_index: usize,
        field: HillslopeIrrigationFixedDateEventField,
    },
}

impl From<HillslopeProductionStateSymbol> for BoundarySymbol {
    #[allow(clippy::too_many_lines)]
    fn from(value: HillslopeProductionStateSymbol) -> Self {
        match value {
            HillslopeProductionStateSymbol::Wb11SoilWater => Self::from("wb11_soil_water"),
            HillslopeProductionStateSymbol::Wb11EtDemand => Self::from("wb11_et_demand"),
            HillslopeProductionStateSymbol::Wb17ResidueInterception => {
                Self::from("wb17_residue_interception")
            }
            HillslopeProductionStateSymbol::Wb11FieldCapacity => Self::from("wb11_field_capacity"),
            HillslopeProductionStateSymbol::Wb11PercFraction => Self::from("wb11_perc_fraction"),
            HillslopeProductionStateSymbol::Wb11LateralFraction => {
                Self::from("wb11_lateral_fraction")
            }
            HillslopeProductionStateSymbol::Wb11DrainageFraction => {
                Self::from("wb11_drainage_fraction")
            }
            HillslopeProductionStateSymbol::Wb11DrainageCoefficient => {
                Self::from("wb11_drainage_coefficient")
            }
            HillslopeProductionStateSymbol::Wb11DrainableStorage => {
                Self::from("wb11_drainable_storage")
            }
            HillslopeProductionStateSymbol::Wb12RainfallInput => Self::from("wb12_rainfall_input"),
            HillslopeProductionStateSymbol::Wb12RunonInput => Self::from("wb12_runon_input"),
            HillslopeProductionStateSymbol::Wb12Infiltration => Self::from("wb12_infiltration"),
            HillslopeProductionStateSymbol::Wb12DepressionStorageDelta => {
                Self::from("wb12_depression_storage_delta")
            }
            HillslopeProductionStateSymbol::Wb12RunoffObserved => {
                Self::from("wb12_runoff_observed")
            }
            HillslopeProductionStateSymbol::Wb12RunoffClosureTolerance => {
                Self::from("wb12_runoff_closure_tolerance")
            }
            HillslopeProductionStateSymbol::Wb12RunoffReconciled => {
                Self::from("wb12_runoff_reconciled")
            }
            HillslopeProductionStateSymbol::Wb12StorageInitial => {
                Self::from("wb12_storage_initial")
            }
            HillslopeProductionStateSymbol::Wb12StorageObserved => {
                Self::from("wb12_storage_observed")
            }
            HillslopeProductionStateSymbol::Wb12StorageClosureTolerance => {
                Self::from("wb12_storage_closure_tolerance")
            }
            HillslopeProductionStateSymbol::Wb12PrecipInput => Self::from("wb12_precip_input"),
            HillslopeProductionStateSymbol::Wb12StorageReconciled => {
                Self::from("wb12_storage_reconciled")
            }
            HillslopeProductionStateSymbol::IrrigRuntimeSource => {
                Self::from("irrigation.runtime_schedule_source")
            }
            HillslopeProductionStateSymbol::IrrigRuntimeDepthMeters => {
                Self::from("irrigation.runtime_depth_m")
            }
            HillslopeProductionStateSymbol::IrrigRuntimeDurationSeconds => {
                Self::from("irrigation.runtime_duration_s")
            }
            HillslopeProductionStateSymbol::IrrigRuntimeRateMetersPerSecond => {
                Self::from("irrigation.runtime_rate_m_per_s")
            }
            HillslopeProductionStateSymbol::IrrigRuntimeEventIndex => {
                Self::from("irrigation.runtime_event_index")
            }
            HillslopeProductionStateSymbol::IrrigRuntimeSystemType => {
                Self::from("irrigation.runtime_system_type")
            }
            HillslopeProductionStateSymbol::IrrigDepletionEnabled => {
                Self::from("irrigation.depletion.enabled")
            }
            HillslopeProductionStateSymbol::IrrigDepletionSystemType => {
                Self::from("irrigation.depletion.system_type")
            }
            HillslopeProductionStateSymbol::IrrigDepletionMinDepthMeters => {
                Self::from("irrigation.depletion.min_depth_m")
            }
            HillslopeProductionStateSymbol::IrrigDepletionMaxDepthMeters => {
                Self::from("irrigation.depletion.max_depth_m")
            }
            HillslopeProductionStateSymbol::IrrigDepletionPeriodCount => {
                Self::from("irrigation.depletion.period_count")
            }
            HillslopeProductionStateSymbol::IrrigFixedDateEnabled => {
                Self::from("irrigation.fixeddate.enabled")
            }
            HillslopeProductionStateSymbol::IrrigFixedDateSystemType => {
                Self::from("irrigation.fixeddate.system_type")
            }
            HillslopeProductionStateSymbol::IrrigFixedDateEventCount => {
                Self::from("irrigation.fixeddate.event_count")
            }
            HillslopeProductionStateSymbol::Wb15PlantCancov => Self::from("cancov"),
            HillslopeProductionStateSymbol::Wb15PlantLai => Self::from("lai"),
            HillslopeProductionStateSymbol::Wb15PlantVdmt => Self::from("vdmt"),
            HillslopeProductionStateSymbol::Wb14HyetographNinten => Self::from("ninten"),
            HillslopeProductionStateSymbol::Wb14HyetographNbrkpt => Self::from("nbrkpt"),
            HillslopeProductionStateSymbol::Wb14SoilConductivity => Self::from("ssc"),
            HillslopeProductionStateSymbol::Wb14SoilLayerDepth => Self::from("dg"),
            HillslopeProductionStateSymbol::Wb14SoilThetaResidual => Self::from("thetdr"),
            HillslopeProductionStateSymbol::Wb14SoilThetaFieldCapacity => Self::from("thetfc"),
            HillslopeProductionStateSymbol::Wb14SnowFilePresent => {
                Self::from("snow.options.snow_file_present")
            }
            HillslopeProductionStateSymbol::Wb14SnowRst => Self::from("snow.options.rst"),
            HillslopeProductionStateSymbol::Wb14SnowNewsnw => Self::from("snow.options.newsnw"),
            HillslopeProductionStateSymbol::Wb14SnowSsd => Self::from("snow.options.ssd"),
            HillslopeProductionStateSymbol::Wb14SnowRuntimeSwe => Self::from("snow.runtime_swe"),
            HillslopeProductionStateSymbol::Wb14FrostFilePresent => {
                Self::from("frost.options.frost_file_present")
            }
            HillslopeProductionStateSymbol::Wb14FrostWintRed => Self::from("frost.options.wintRed"),
            HillslopeProductionStateSymbol::Wb14FrostFineTop => Self::from("frost.options.fineTop"),
            HillslopeProductionStateSymbol::Wb14FrostFineBot => Self::from("frost.options.fineBot"),
            HillslopeProductionStateSymbol::Wb14FrostKsnowf => Self::from("frost.options.ksnowf"),
            HillslopeProductionStateSymbol::Wb14FrostKresf => Self::from("frost.options.kresf"),
            HillslopeProductionStateSymbol::Wb14FrostKsoilf => Self::from("frost.options.ksoilf"),
            HillslopeProductionStateSymbol::Wb14FrostKfactor1 => {
                Self::from("frost.options.kfactor1")
            }
            HillslopeProductionStateSymbol::Wb14FrostKfactor2 => {
                Self::from("frost.options.kfactor2")
            }
            HillslopeProductionStateSymbol::Wb14FrostKfactor3 => {
                Self::from("frost.options.kfactor3")
            }
            HillslopeProductionStateSymbol::Wb14FrostRuntimeDfrost => {
                Self::from("frost.runtime_dfrost")
            }
            HillslopeProductionStateSymbol::Wb14FrostRuntimeDthaw => {
                Self::from("frost.runtime_dthaw")
            }
            HillslopeProductionStateSymbol::Wb14FrostRuntimeNft => Self::from("frost.runtime_nft"),
            HillslopeProductionStateSymbol::Wb14FrostRuntimeWsFrz => {
                Self::from("frost.runtime_ws_frz")
            }
            HillslopeProductionStateSymbol::Wb14FrostRuntimeInfcapFrz => {
                Self::from("frost.runtime_infcap_frz")
            }
            HillslopeProductionStateSymbol::Wb14Tmax => Self::from("tmax"),
            HillslopeProductionStateSymbol::Wb14Tmin => Self::from("tmin"),
            HillslopeProductionStateSymbol::Wb16Timep => Self::from("timep"),
            HillslopeProductionStateSymbol::Wb16Efflen => Self::from("efflen"),
            HillslopeProductionStateSymbol::Wb16Ealpha => Self::from("ealpha"),
            HillslopeProductionStateSymbol::Wb16ExponentM => Self::from("m"),
            HillslopeProductionStateSymbol::Wb16Peakro => Self::from("peakro"),
            HillslopeProductionStateSymbol::Wb16Watdur => Self::from("watdur"),
            HillslopeProductionStateSymbol::Wb16MethodBranch => {
                Self::from("wb16_peak_method_branch")
            }
            HillslopeProductionStateSymbol::Wb16Tstar => Self::from("wb16_tstar"),
            HillslopeProductionStateSymbol::Wb16Qpstar => Self::from("wb16_qpstar"),
            HillslopeProductionStateSymbol::Wb16Vstar => Self::from("wb16_vstar"),
            HillslopeProductionStateSymbol::IrrigationDepletionPeriod {
                period_index,
                field,
            } => Self::from(format!(
                "irrigation.depletion.period_{period_index:04}.{}",
                field.as_str()
            )),
            HillslopeProductionStateSymbol::IrrigationFixedDateEvent { event_index, field } => {
                Self::from(format!(
                    "irrigation.fixeddate.event_{event_index:04}.{}",
                    field.as_str()
                ))
            }
        }
    }
}

/// Typed ARCH22 boundary-flux symbols for covered production hillslope
/// kernels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HillslopeProductionFluxSymbol {
    Wb11Et,
    Wb11Ws,
    Wb17PlantTranspirationEp,
    Wb17SoilEvaporationEs,
    Wb17ResidueEvaporationEr,
    Wb11PercLossD,
    Wb11PercRechargePe,
    Wb11LateralQ,
    Wb11DrainageQdd,
    Wb11SubhydQd,
    Wb12RunoffClosureDelta,
    Wb12RunoffQ,
    Wb12SnowCouplingS,
    Wb12StorageClosureDelta,
    Wb15InterceptionI,
    IrrigDailyIrrigation,
}

impl From<HillslopeProductionFluxSymbol> for BoundarySymbol {
    fn from(value: HillslopeProductionFluxSymbol) -> Self {
        match value {
            HillslopeProductionFluxSymbol::Wb11Et => Self::from("ET"),
            HillslopeProductionFluxSymbol::Wb11Ws => Self::from("Ws"),
            HillslopeProductionFluxSymbol::Wb17PlantTranspirationEp => Self::from("Ep"),
            HillslopeProductionFluxSymbol::Wb17SoilEvaporationEs => Self::from("Es"),
            HillslopeProductionFluxSymbol::Wb17ResidueEvaporationEr => Self::from("Er"),
            HillslopeProductionFluxSymbol::Wb11PercLossD => Self::from("D"),
            HillslopeProductionFluxSymbol::Wb11PercRechargePe => Self::from("Pe"),
            HillslopeProductionFluxSymbol::Wb11LateralQ => Self::from("q"),
            HillslopeProductionFluxSymbol::Wb11DrainageQdd => Self::from("Qdd"),
            HillslopeProductionFluxSymbol::Wb11SubhydQd => Self::from("Qd"),
            HillslopeProductionFluxSymbol::Wb12RunoffClosureDelta => {
                Self::from("wb12_runoff_closure_delta")
            }
            HillslopeProductionFluxSymbol::Wb12RunoffQ => Self::from("Q"),
            HillslopeProductionFluxSymbol::Wb12SnowCouplingS => Self::from("S"),
            HillslopeProductionFluxSymbol::Wb12StorageClosureDelta => {
                Self::from("wb12_storage_closure_delta")
            }
            HillslopeProductionFluxSymbol::Wb15InterceptionI => Self::from("I"),
            HillslopeProductionFluxSymbol::IrrigDailyIrrigation => Self::from("Irr"),
        }
    }
}

/// Node-scoped channel state fields for ARCH22 watershed typed symbol
/// projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WatershedChannelStateField {
    Chnn,
    Ctlslp,
    Chnk,
    Qpo,
    Durrof,
}

impl WatershedChannelStateField {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Chnn => "chnn",
            Self::Ctlslp => "ctlslp",
            Self::Chnk => "chnk",
            Self::Qpo => "qpo",
            Self::Durrof => "durrof",
        }
    }
}

/// Node-scoped channel flux fields for ARCH22 watershed typed symbol
/// projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WatershedChannelFluxField {
    Roff,
}

impl WatershedChannelFluxField {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Roff => "roff",
        }
    }
}

/// Node-scoped impoundment state fields for ARCH22 watershed typed symbol
/// projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WatershedImpoundmentStateField {
    H,
    Hfull,
    Deltat,
    Qinf,
    Qo,
    Durout,
    Hnext,
}

impl WatershedImpoundmentStateField {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::H => "h",
            Self::Hfull => "hfull",
            Self::Deltat => "deltat",
            Self::Qinf => "qinf",
            Self::Qo => "qo",
            Self::Durout => "durout",
            Self::Hnext => "hnext",
        }
    }
}

/// Node-scoped impoundment flux fields for ARCH22 watershed typed symbol
/// projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WatershedImpoundmentFluxField {
    OutflowVolume,
}

impl WatershedImpoundmentFluxField {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OutflowVolume => "outflow_volume",
        }
    }
}

/// Typed ARCH22 boundary-state symbols for covered production watershed
/// kernels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WatershedProductionStateSymbol {
    Dtchr,
    Nchnum,
    Ipeak,
    ChannelNode {
        node_id: u32,
        field: WatershedChannelStateField,
    },
    ImpoundmentNode {
        node_id: u32,
        field: WatershedImpoundmentStateField,
    },
    HillslopeContributorPeak {
        hillslope_id: u32,
    },
    HillslopeContributorDuration {
        hillslope_id: u32,
    },
    HillslopeContributorTotalDetachmentKg {
        hillslope_id: u32,
    },
    HillslopeContributorTotalDepositionKg {
        hillslope_id: u32,
    },
    HillslopeContributorParticleClassCount {
        hillslope_id: u32,
    },
    HillslopeContributorSedimentConcentrationKgM3 {
        hillslope_id: u32,
        class_index: usize,
    },
    HillslopeContributorParticleDiameterMeters {
        hillslope_id: u32,
        class_index: usize,
    },
    HillslopeContributorParticleFlowFraction {
        hillslope_id: u32,
        class_index: usize,
    },
}

impl From<WatershedProductionStateSymbol> for BoundarySymbol {
    fn from(value: WatershedProductionStateSymbol) -> Self {
        match value {
            WatershedProductionStateSymbol::Dtchr => Self::from("dtchr"),
            WatershedProductionStateSymbol::Nchnum => Self::from("nchnum"),
            WatershedProductionStateSymbol::Ipeak => Self::from("ipeak"),
            WatershedProductionStateSymbol::ChannelNode { node_id, field } => {
                Self::from(format!("ws10_channel_{node_id}_{}", field.as_str()))
            }
            WatershedProductionStateSymbol::ImpoundmentNode { node_id, field } => {
                Self::from(format!("ws10_impoundment_{node_id}_{}", field.as_str()))
            }
            WatershedProductionStateSymbol::HillslopeContributorPeak { hillslope_id } => {
                Self::from(format!("hs{hillslope_id}_peakro"))
            }
            WatershedProductionStateSymbol::HillslopeContributorDuration { hillslope_id } => {
                Self::from(format!("hs{hillslope_id}_watdur"))
            }
            WatershedProductionStateSymbol::HillslopeContributorTotalDetachmentKg {
                hillslope_id,
            } => Self::from(format!("hs{hillslope_id}_total_detachment_kg")),
            WatershedProductionStateSymbol::HillslopeContributorTotalDepositionKg {
                hillslope_id,
            } => Self::from(format!("hs{hillslope_id}_total_deposition_kg")),
            WatershedProductionStateSymbol::HillslopeContributorParticleClassCount {
                hillslope_id,
            } => Self::from(format!("hs{hillslope_id}_particle_class_count")),
            WatershedProductionStateSymbol::HillslopeContributorSedimentConcentrationKgM3 {
                hillslope_id,
                class_index,
            } => Self::from(format!(
                "hs{hillslope_id}_sediment_concentration_kg_m3_{class_index:04}"
            )),
            WatershedProductionStateSymbol::HillslopeContributorParticleDiameterMeters {
                hillslope_id,
                class_index,
            } => Self::from(format!(
                "hs{hillslope_id}_particle_diameter_m_{class_index:04}"
            )),
            WatershedProductionStateSymbol::HillslopeContributorParticleFlowFraction {
                hillslope_id,
                class_index,
            } => Self::from(format!(
                "hs{hillslope_id}_particle_flow_fraction_{class_index:04}"
            )),
        }
    }
}

/// Typed ARCH22 boundary-flux symbols for covered production watershed
/// kernels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WatershedProductionFluxSymbol {
    Cbase,
    ChannelNode {
        node_id: u32,
        field: WatershedChannelFluxField,
    },
    ImpoundmentNode {
        node_id: u32,
        field: WatershedImpoundmentFluxField,
    },
}

impl From<WatershedProductionFluxSymbol> for BoundarySymbol {
    fn from(value: WatershedProductionFluxSymbol) -> Self {
        match value {
            WatershedProductionFluxSymbol::Cbase => Self::from("cbase"),
            WatershedProductionFluxSymbol::ChannelNode { node_id, field } => {
                Self::from(format!("ws10_channel_{node_id}_{}", field.as_str()))
            }
            WatershedProductionFluxSymbol::ImpoundmentNode { node_id, field } => {
                Self::from(format!("ws10_impoundment_{node_id}_{}", field.as_str()))
            }
        }
    }
}

/// Unit-aware scalar value for kernel seam maps.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoundaryValue {
    Scalar(f64),
    RunoffDepthMillimeters(RunoffDepthMillimeters),
    FlowRateCubicMetersPerSecond(FlowRateCubicMetersPerSecond),
    StorageVolumeCubicMeters(StorageVolumeCubicMeters),
    ProcessRateMillimetersPerHour(ProcessRateMillimetersPerHour),
    SurfaceAreaSquareMeters(SurfaceAreaSquareMeters),
    WaterDepthMeters(WaterDepthMeters),
    ElapsedTimeSeconds(ElapsedTimeSeconds),
    HourOfDay(HourOfDay),
    LinearRateMetersPerSecond(LinearRateMetersPerSecond),
    SolarRadiationLangleysPerDay(SolarRadiationLangleysPerDay),
    SolarRadiationMegajoulesPerSquareMeterPerDay(SolarRadiationMegajoulesPerSquareMeterPerDay),
    SolarRadiationMegajoulesPerSquareMeterPerHour(SolarRadiationMegajoulesPerSquareMeterPerHour),
    TemperatureCelsius(TemperatureCelsius),
    DirectionDegrees(DirectionDegrees),
    DensityKilogramsPerCubicMeter(DensityKilogramsPerCubicMeter),
    FractionUnitInterval(FractionUnitInterval),
}

impl BoundaryValue {
    #[must_use]
    pub const fn scalar(value: f64) -> Self {
        Self::Scalar(value)
    }

    pub fn water_depth_meters(value: f64) -> Result<Self, BoundaryError> {
        WaterDepthMeters::try_new(value).map(Self::WaterDepthMeters)
    }

    pub fn elapsed_time_seconds(value: f64) -> Result<Self, BoundaryError> {
        ElapsedTimeSeconds::try_new(value).map(Self::ElapsedTimeSeconds)
    }

    pub fn hour_of_day(value: f64) -> Result<Self, BoundaryError> {
        HourOfDay::try_new(value).map(Self::HourOfDay)
    }

    pub fn linear_rate_meters_per_second(value: f64) -> Result<Self, BoundaryError> {
        LinearRateMetersPerSecond::try_new(value).map(Self::LinearRateMetersPerSecond)
    }

    pub fn solar_radiation_langleys_per_day(value: f64) -> Result<Self, BoundaryError> {
        SolarRadiationLangleysPerDay::try_new(value).map(Self::SolarRadiationLangleysPerDay)
    }

    pub fn solar_radiation_megajoules_per_square_meter_per_day(
        value: f64,
    ) -> Result<Self, BoundaryError> {
        SolarRadiationMegajoulesPerSquareMeterPerDay::try_new(value)
            .map(Self::SolarRadiationMegajoulesPerSquareMeterPerDay)
    }

    pub fn solar_radiation_megajoules_per_square_meter_per_hour(
        value: f64,
    ) -> Result<Self, BoundaryError> {
        SolarRadiationMegajoulesPerSquareMeterPerHour::try_new(value)
            .map(Self::SolarRadiationMegajoulesPerSquareMeterPerHour)
    }

    pub fn temperature_celsius(value: f64) -> Result<Self, BoundaryError> {
        TemperatureCelsius::try_new(value).map(Self::TemperatureCelsius)
    }

    pub fn direction_degrees(value: f64) -> Result<Self, BoundaryError> {
        DirectionDegrees::try_new(value).map(Self::DirectionDegrees)
    }

    pub fn density_kilograms_per_cubic_meter(value: f64) -> Result<Self, BoundaryError> {
        DensityKilogramsPerCubicMeter::try_new(value).map(Self::DensityKilogramsPerCubicMeter)
    }

    pub fn fraction_unit_interval(value: f64) -> Result<Self, BoundaryError> {
        FractionUnitInterval::try_new(value).map(Self::FractionUnitInterval)
    }

    #[must_use]
    pub fn as_f64(self) -> f64 {
        match self {
            Self::Scalar(value) => value,
            Self::RunoffDepthMillimeters(value) => value.as_millimeters(),
            Self::FlowRateCubicMetersPerSecond(value) => value.as_cubic_meters_per_second(),
            Self::StorageVolumeCubicMeters(value) => value.as_cubic_meters(),
            Self::ProcessRateMillimetersPerHour(value) => value.as_millimeters_per_hour(),
            Self::SurfaceAreaSquareMeters(value) => value.as_square_meters(),
            Self::WaterDepthMeters(value) => value.as_meters(),
            Self::ElapsedTimeSeconds(value) => value.as_seconds(),
            Self::HourOfDay(value) => value.as_hours(),
            Self::LinearRateMetersPerSecond(value) => value.as_meters_per_second(),
            Self::SolarRadiationLangleysPerDay(value) => value.as_langleys_per_day(),
            Self::SolarRadiationMegajoulesPerSquareMeterPerDay(value) => {
                value.as_megajoules_per_square_meter_per_day()
            }
            Self::SolarRadiationMegajoulesPerSquareMeterPerHour(value) => {
                value.as_megajoules_per_square_meter_per_hour()
            }
            Self::TemperatureCelsius(value) => value.as_celsius(),
            Self::DirectionDegrees(value) => value.as_degrees(),
            Self::DensityKilogramsPerCubicMeter(value) => value.as_kilograms_per_cubic_meter(),
            Self::FractionUnitInterval(value) => value.as_fraction(),
        }
    }

    #[must_use]
    pub const fn unit_label(self) -> &'static str {
        match self {
            Self::Scalar(_) => "scalar",
            Self::RunoffDepthMillimeters(_) => "mm",
            Self::FlowRateCubicMetersPerSecond(_) => "m3/s",
            Self::StorageVolumeCubicMeters(_) => "m3",
            Self::ProcessRateMillimetersPerHour(_) => "mm/hr",
            Self::SurfaceAreaSquareMeters(_) => "m2",
            Self::WaterDepthMeters(_) => "m",
            Self::ElapsedTimeSeconds(_) => "s",
            Self::HourOfDay(_) => "h",
            Self::LinearRateMetersPerSecond(_) => "m s^-1",
            Self::SolarRadiationLangleysPerDay(_) => "Ly d^-1",
            Self::SolarRadiationMegajoulesPerSquareMeterPerDay(_) => "MJ m^-2 d^-1",
            Self::SolarRadiationMegajoulesPerSquareMeterPerHour(_) => "MJ m^-2 h^-1",
            Self::TemperatureCelsius(_) => "degC",
            Self::DirectionDegrees(_) => "deg",
            Self::DensityKilogramsPerCubicMeter(_) => "kg m^-3",
            Self::FractionUnitInterval(_) => "dimensionless",
        }
    }
}

impl From<f64> for BoundaryValue {
    fn from(value: f64) -> Self {
        Self::Scalar(value)
    }
}

impl From<RunoffDepthMillimeters> for BoundaryValue {
    fn from(value: RunoffDepthMillimeters) -> Self {
        Self::RunoffDepthMillimeters(value)
    }
}

impl From<FlowRateCubicMetersPerSecond> for BoundaryValue {
    fn from(value: FlowRateCubicMetersPerSecond) -> Self {
        Self::FlowRateCubicMetersPerSecond(value)
    }
}

impl From<StorageVolumeCubicMeters> for BoundaryValue {
    fn from(value: StorageVolumeCubicMeters) -> Self {
        Self::StorageVolumeCubicMeters(value)
    }
}

impl From<ProcessRateMillimetersPerHour> for BoundaryValue {
    fn from(value: ProcessRateMillimetersPerHour) -> Self {
        Self::ProcessRateMillimetersPerHour(value)
    }
}

impl From<SurfaceAreaSquareMeters> for BoundaryValue {
    fn from(value: SurfaceAreaSquareMeters) -> Self {
        Self::SurfaceAreaSquareMeters(value)
    }
}

impl From<WaterDepthMeters> for BoundaryValue {
    fn from(value: WaterDepthMeters) -> Self {
        Self::WaterDepthMeters(value)
    }
}

impl From<ElapsedTimeSeconds> for BoundaryValue {
    fn from(value: ElapsedTimeSeconds) -> Self {
        Self::ElapsedTimeSeconds(value)
    }
}

impl From<HourOfDay> for BoundaryValue {
    fn from(value: HourOfDay) -> Self {
        Self::HourOfDay(value)
    }
}

impl From<LinearRateMetersPerSecond> for BoundaryValue {
    fn from(value: LinearRateMetersPerSecond) -> Self {
        Self::LinearRateMetersPerSecond(value)
    }
}

impl From<SolarRadiationLangleysPerDay> for BoundaryValue {
    fn from(value: SolarRadiationLangleysPerDay) -> Self {
        Self::SolarRadiationLangleysPerDay(value)
    }
}

impl From<SolarRadiationMegajoulesPerSquareMeterPerDay> for BoundaryValue {
    fn from(value: SolarRadiationMegajoulesPerSquareMeterPerDay) -> Self {
        Self::SolarRadiationMegajoulesPerSquareMeterPerDay(value)
    }
}

impl From<SolarRadiationMegajoulesPerSquareMeterPerHour> for BoundaryValue {
    fn from(value: SolarRadiationMegajoulesPerSquareMeterPerHour) -> Self {
        Self::SolarRadiationMegajoulesPerSquareMeterPerHour(value)
    }
}

impl From<TemperatureCelsius> for BoundaryValue {
    fn from(value: TemperatureCelsius) -> Self {
        Self::TemperatureCelsius(value)
    }
}

impl From<DirectionDegrees> for BoundaryValue {
    fn from(value: DirectionDegrees) -> Self {
        Self::DirectionDegrees(value)
    }
}

impl From<DensityKilogramsPerCubicMeter> for BoundaryValue {
    fn from(value: DensityKilogramsPerCubicMeter) -> Self {
        Self::DensityKilogramsPerCubicMeter(value)
    }
}

impl From<FractionUnitInterval> for BoundaryValue {
    fn from(value: FractionUnitInterval) -> Self {
        Self::FractionUnitInterval(value)
    }
}

/// Outcome class for orchestrator writeback decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WritebackDecisionOutcome {
    Accept,
    Reject,
    Apply,
}

/// One scalar writeback field proposed by a kernel.
#[derive(Debug, Clone, PartialEq)]
pub struct WritebackField {
    pub symbol: BoundarySymbol,
    pub value: BoundaryValue,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
}

impl WritebackField {
    #[must_use]
    pub fn unbounded(symbol: impl Into<BoundarySymbol>, value: impl Into<BoundaryValue>) -> Self {
        Self {
            symbol: symbol.into(),
            value: value.into(),
            minimum: None,
            maximum: None,
        }
    }

    #[must_use]
    pub fn bounded(
        symbol: impl Into<BoundarySymbol>,
        value: impl Into<BoundaryValue>,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            value: value.into(),
            minimum,
            maximum,
        }
    }
}

/// Kernel-proposed writeback payload.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct KernelWritebackPayload {
    pub state_updates: Vec<WritebackField>,
    pub flux_updates: Vec<WritebackField>,
}

impl KernelWritebackPayload {
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_updates(
        state_updates: Vec<WritebackField>,
        flux_updates: Vec<WritebackField>,
    ) -> Self {
        Self {
            state_updates,
            flux_updates,
        }
    }
}

/// Kernel response surface for hillslope and watershed invocations.
#[derive(Debug, Clone, PartialEq)]
pub struct KernelRunResponse {
    pub status: SimulationStatus,
    pub writeback: KernelWritebackPayload,
}

impl KernelRunResponse {
    #[must_use]
    pub const fn new(status: SimulationStatus, writeback: KernelWritebackPayload) -> Self {
        Self { status, writeback }
    }
}

/// Management class for growth-phase scheduler dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HillslopeGrowthManagementClass {
    AnnualOrFallow,
    Perennial,
}

/// Management class for decomposition/resup phase scheduler dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HillslopeDecompositionManagementClass {
    AnnualOrFallow,
    Perennial,
}

/// Active annual decomposition/residue transition selected for runtime day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HillslopeAnnualDecompositionAction {
    None,
    Herbicide,
    Burn,
    Silage,
    Cut,
    Remove,
}

/// Annual transition-control payload consumed by decomposition dispatch.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HillslopeAnnualDecompositionControl {
    pub resmgt: u8,
    pub jdherb: u16,
    pub jdburn: u16,
    pub jdslge: u16,
    pub jdcut: u16,
    pub jdmove: u16,
    pub fbrnag: f64,
    pub fbrnog: f64,
    pub frcut: f64,
    pub frmove: f64,
    pub active_action: HillslopeAnnualDecompositionAction,
}

/// Active perennial decomposition/residue transition selected for runtime day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HillslopePerennialDecompositionAction {
    None,
    Cut { event_index: u16 },
    Grazing { cycle_index: u16 },
}

/// Active grazing payload selected for runtime day.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HillslopeActiveGrazingCycle {
    pub cycle_index: u16,
    pub gday: u16,
    pub gend: u16,
    pub animal: f64,
    pub bodywt: f64,
    pub area: f64,
    pub digest: f64,
}

/// Perennial transition-control payload consumed by decomposition dispatch.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HillslopePerennialDecompositionControl {
    pub mgtopt: u8,
    pub ncut: u16,
    pub ncycle: u16,
    pub active_action: HillslopePerennialDecompositionAction,
    pub active_grazing_cycle: Option<HillslopeActiveGrazingCycle>,
}

/// Typed transition-control payload consumed by decomposition dispatch.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HillslopeDecompositionTransitionControl {
    Annual(HillslopeAnnualDecompositionControl),
    Perennial(HillslopePerennialDecompositionControl),
}

/// Typed decomposition transition payload assembled by scheduler dispatch.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HillslopeDecompositionTransitionPayload {
    pub active_slot_index: usize,
    pub active_crop_slot_index: usize,
    pub runtime_day_of_year: u16,
    pub iresd_seed: f64,
    pub sumrtm_seed: f64,
    pub sumsrm_seed: f64,
    pub control: HillslopeDecompositionTransitionControl,
}

/// Growth state surface consumed and updated by growth transition controls.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HillslopeGrowthStateSurface {
    pub sumgdd: f64,
    pub vdmt: f64,
    pub cancov: f64,
    pub lai: f64,
    pub rtmass: f64,
    pub rtd: f64,
    pub hia: f64,
}

/// Active annual growth transition selected for runtime day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HillslopeAnnualGrowthAction {
    None,
    PlantingReset,
    HarvestReset,
    SenescenceReset,
}

/// Annual growth transition-control payload consumed by growth dispatch.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HillslopeAnnualGrowthControl {
    pub jdharv: u16,
    pub jdplt: u16,
    pub rw: f64,
    pub active_action: HillslopeAnnualGrowthAction,
}

/// Active perennial growth transition selected for runtime day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HillslopePerennialGrowthAction {
    None,
    PlantingReset,
    StopReset,
}

/// Perennial growth transition-control payload consumed by growth dispatch.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HillslopePerennialGrowthControl {
    pub jdharv: u16,
    pub jdplt: u16,
    pub jdstop: u16,
    pub mgtopt: u8,
    pub rw: f64,
    pub active_action: HillslopePerennialGrowthAction,
}

/// Typed transition-control payload consumed by growth dispatch.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HillslopeGrowthTransitionControl {
    Annual(HillslopeAnnualGrowthControl),
    Perennial(HillslopePerennialGrowthControl),
}

/// Typed growth transition payload assembled by scheduler dispatch.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HillslopeGrowthTransitionPayload {
    pub active_slot_index: usize,
    pub active_crop_slot_index: usize,
    pub runtime_day_of_year: u16,
    pub state_before: HillslopeGrowthStateSurface,
    pub state_after: HillslopeGrowthStateSurface,
    pub control: HillslopeGrowthTransitionControl,
}

/// Typed phase class for hillslope kernel dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HillslopeKernelPhaseClass {
    Hydrology,
    HydrologyEvapotranspiration,
    HydrologyPercolationDeepSeepage,
    HydrologyLateralTransfer,
    HydrologyDrainage,
    HydrologyPlantRootUptake,
    HydrologyRunoffReconciliation,
    HydrologyStorageReconciliation,
    HydrologyPeakRunoff,
    DecompositionTransition,
    ResiduePartitionTransition,
    GrowthAnnualTransition,
    GrowthPerennialTransition,
}

impl HillslopeKernelPhaseClass {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hydrology => "hydrology",
            Self::HydrologyEvapotranspiration => "hydrology_evapotranspiration",
            Self::HydrologyPercolationDeepSeepage => "hydrology_percolation_deep_seepage",
            Self::HydrologyLateralTransfer => "hydrology_lateral_transfer",
            Self::HydrologyDrainage => "hydrology_drainage",
            Self::HydrologyPlantRootUptake => "hydrology_plant_root_uptake",
            Self::HydrologyRunoffReconciliation => "hydrology_runoff_reconciliation",
            Self::HydrologyStorageReconciliation => "hydrology_storage_reconciliation",
            Self::HydrologyPeakRunoff => "hydrology_peak_runoff",
            Self::DecompositionTransition => "decomposition_transition",
            Self::ResiduePartitionTransition => "residue_partition_transition",
            Self::GrowthAnnualTransition => "growth_annual_transition",
            Self::GrowthPerennialTransition => "growth_perennial_transition",
        }
    }

    #[must_use]
    pub const fn is_hydrology_phase(self) -> bool {
        matches!(
            self,
            Self::Hydrology
                | Self::HydrologyEvapotranspiration
                | Self::HydrologyPercolationDeepSeepage
                | Self::HydrologyLateralTransfer
                | Self::HydrologyDrainage
                | Self::HydrologyPlantRootUptake
                | Self::HydrologyRunoffReconciliation
                | Self::HydrologyStorageReconciliation
                | Self::HydrologyPeakRunoff
        )
    }

    #[must_use]
    pub const fn is_growth_transition(self) -> bool {
        matches!(
            self,
            Self::GrowthAnnualTransition | Self::GrowthPerennialTransition
        )
    }

    #[must_use]
    pub const fn is_decomposition_transition(self) -> bool {
        matches!(
            self,
            Self::DecompositionTransition | Self::ResiduePartitionTransition
        )
    }
}

/// Typed growth scheduler context carried on hillslope kernel requests.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HillslopeGrowthKernelContext {
    pub management_class: HillslopeGrowthManagementClass,
    pub order_growth_after_decomp: f64,
    pub order_watbal_after_growth: f64,
    pub transition_payload: Option<HillslopeGrowthTransitionPayload>,
}

impl HillslopeGrowthKernelContext {
    #[must_use]
    pub const fn new(
        management_class: HillslopeGrowthManagementClass,
        order_growth_after_decomp: f64,
        order_watbal_after_growth: f64,
    ) -> Self {
        Self {
            management_class,
            order_growth_after_decomp,
            order_watbal_after_growth,
            transition_payload: None,
        }
    }

    #[must_use]
    pub const fn with_transition_payload(
        mut self,
        transition_payload: HillslopeGrowthTransitionPayload,
    ) -> Self {
        self.transition_payload = Some(transition_payload);
        self
    }
}

/// Typed decomposition/resup scheduler context carried on hillslope kernel
/// requests.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HillslopeDecompositionKernelContext {
    pub management_class: HillslopeDecompositionManagementClass,
    pub order_decomp_before_soil: f64,
    pub order_growth_after_decomp: f64,
    pub transition_payload: Option<HillslopeDecompositionTransitionPayload>,
}

impl HillslopeDecompositionKernelContext {
    #[must_use]
    pub const fn new(
        management_class: HillslopeDecompositionManagementClass,
        order_decomp_before_soil: f64,
        order_growth_after_decomp: f64,
    ) -> Self {
        Self {
            management_class,
            order_decomp_before_soil,
            order_growth_after_decomp,
            transition_payload: None,
        }
    }

    #[must_use]
    pub const fn with_transition_payload(
        mut self,
        transition_payload: HillslopeDecompositionTransitionPayload,
    ) -> Self {
        self.transition_payload = Some(transition_payload);
        self
    }
}

/// Consumer adapter class selected for the current hillslope phase invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HillslopeConsumerAdapter {
    Runoff,
    Soil,
    Watbal,
    Perc,
    Decomposition,
    Growth,
}

impl HillslopeConsumerAdapter {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Runoff => "runoff",
            Self::Soil => "soil",
            Self::Watbal => "watbal",
            Self::Perc => "perc",
            Self::Decomposition => "decomposition",
            Self::Growth => "growth",
        }
    }
}

/// Hillslope kernel invocation request.
///
/// Scheduler execution keeps state/flux ownership and lends immutable views to
/// kernels to avoid full-surface cloning in hot paths.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HillslopeKernelRequest<'a> {
    pub phase_name: &'a str,
    pub phase_class: HillslopeKernelPhaseClass,
    pub consumer_adapter: HillslopeConsumerAdapter,
    pub decomposition_context: Option<HillslopeDecompositionKernelContext>,
    pub growth_context: Option<HillslopeGrowthKernelContext>,
    pub state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
    pub flux_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
}

impl<'a> HillslopeKernelRequest<'a> {
    #[must_use]
    pub fn new(
        phase_name: &'a str,
        consumer_adapter: HillslopeConsumerAdapter,
        state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Self {
        Self::with_transition_context(
            phase_name,
            HillslopeKernelPhaseClass::Hydrology,
            consumer_adapter,
            None,
            None,
            state_surface,
            flux_surface,
        )
    }

    #[must_use]
    pub fn with_transition_context(
        phase_name: &'a str,
        phase_class: HillslopeKernelPhaseClass,
        consumer_adapter: HillslopeConsumerAdapter,
        decomposition_context: Option<HillslopeDecompositionKernelContext>,
        growth_context: Option<HillslopeGrowthKernelContext>,
        state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Self {
        Self {
            phase_name,
            phase_class,
            consumer_adapter,
            decomposition_context,
            growth_context,
            state_surface,
            flux_surface,
        }
    }

    #[must_use]
    pub fn with_phase_context(
        phase_name: &'a str,
        phase_class: HillslopeKernelPhaseClass,
        consumer_adapter: HillslopeConsumerAdapter,
        growth_context: Option<HillslopeGrowthKernelContext>,
        state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Self {
        Self::with_transition_context(
            phase_name,
            phase_class,
            consumer_adapter,
            None,
            growth_context,
            state_surface,
            flux_surface,
        )
    }
}

/// Watershed kernel invocation request.
///
/// State/flux surfaces are borrowed from orchestrator-owned writeback maps to
/// reduce scheduler hot-path allocation pressure.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedKernelRequest<'a> {
    pub node_kind: &'a str,
    pub node_id: u32,
    pub dependency_nodes: Vec<String>,
    pub contributor_hillslopes: &'a [u32],
    pub state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
    pub flux_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
}

impl<'a> WatershedKernelRequest<'a> {
    #[must_use]
    pub fn new(
        node_kind: &'a str,
        node_id: u32,
        dependency_nodes: Vec<String>,
        contributor_hillslopes: &'a [u32],
        state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Self {
        Self {
            node_kind,
            node_id,
            dependency_nodes,
            contributor_hillslopes,
            state_surface,
            flux_surface,
        }
    }
}

/// Hillslope kernel trait boundary.
pub trait HillslopeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse;
}

/// Watershed kernel trait boundary.
pub trait WatershedKernel {
    fn run_watershed_node(&mut self, request: &WatershedKernelRequest<'_>) -> KernelRunResponse;
}

/// Outcome surface for writeback evaluation.
#[derive(Debug, Clone, PartialEq)]
pub struct KernelWritebackDecision {
    pub outcome: WritebackDecisionOutcome,
    pub status: SimulationStatus,
    pub violations: Vec<ClosureViolation>,
}

/// Outcome surface for accepted writeback application.
#[derive(Debug, Clone, PartialEq)]
pub struct KernelWritebackApplyResult {
    pub outcome: WritebackDecisionOutcome,
    pub status: SimulationStatus,
    pub applied_state_symbols: Vec<BoundarySymbol>,
    pub applied_flux_symbols: Vec<BoundarySymbol>,
}

/// Writeback-application errors.
#[derive(Debug)]
pub enum WritebackError {
    Status(StatusError),
    DecisionNotAccept { outcome: WritebackDecisionOutcome },
}

impl fmt::Display for WritebackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status(source) => write!(f, "failed constructing writeback status: {source}"),
            Self::DecisionNotAccept { outcome } => {
                write!(
                    f,
                    "cannot apply writeback for non-accept outcome: {outcome:?}"
                )
            }
        }
    }
}

impl Error for WritebackError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Status(source) => Some(source),
            Self::DecisionNotAccept { .. } => None,
        }
    }
}

impl From<StatusError> for WritebackError {
    fn from(value: StatusError) -> Self {
        Self::Status(value)
    }
}

/// Evaluate a kernel writeback payload into deterministic accept/reject status.
pub fn evaluate_kernel_writeback(
    phase: SimulationPhase,
    payload: &KernelWritebackPayload,
) -> Result<KernelWritebackDecision, StatusError> {
    let mut violations = Vec::new();

    for field in &payload.state_updates {
        collect_field_violations("state", field, &mut violations);
    }

    for field in &payload.flux_updates {
        collect_field_violations("flux", field, &mut violations);
    }

    if violations.is_empty() {
        let status = SimulationStatus::ok(phase, WRITEBACK_ACCEPT_MESSAGE_ID)?;
        Ok(KernelWritebackDecision {
            outcome: WritebackDecisionOutcome::Accept,
            status,
            violations,
        })
    } else {
        let has_non_finite = violations
            .iter()
            .any(|violation| violation.kind == ClosureViolationKind::NonFinite);

        let status = if has_non_finite {
            SimulationStatus::non_finite_failure(phase, WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID)?
        } else {
            SimulationStatus::domain_failure(
                phase,
                BoundaryClass::DomainViolation,
                WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
            )?
        };

        Ok(KernelWritebackDecision {
            outcome: WritebackDecisionOutcome::Reject,
            status,
            violations,
        })
    }
}

/// Apply an accepted writeback payload to orchestrator-owned state/flux maps.
pub fn apply_kernel_writeback(
    phase: SimulationPhase,
    decision: &KernelWritebackDecision,
    payload: &KernelWritebackPayload,
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    flux_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<KernelWritebackApplyResult, WritebackError> {
    if decision.outcome != WritebackDecisionOutcome::Accept {
        return Err(WritebackError::DecisionNotAccept {
            outcome: decision.outcome,
        });
    }

    let mut state_updates: Vec<&WritebackField> = payload.state_updates.iter().collect();
    state_updates.sort_by_key(|field| field.symbol.as_str());

    let mut flux_updates: Vec<&WritebackField> = payload.flux_updates.iter().collect();
    flux_updates.sort_by_key(|field| field.symbol.as_str());

    for field in &state_updates {
        state_surface.insert(field.symbol.clone(), field.value);
    }

    for field in &flux_updates {
        flux_surface.insert(field.symbol.clone(), field.value);
    }

    let status = SimulationStatus::ok(phase, WRITEBACK_APPLY_MESSAGE_ID)?;

    Ok(KernelWritebackApplyResult {
        outcome: WritebackDecisionOutcome::Apply,
        status,
        applied_state_symbols: state_updates
            .iter()
            .map(|field| field.symbol.clone())
            .collect(),
        applied_flux_symbols: flux_updates
            .iter()
            .map(|field| field.symbol.clone())
            .collect(),
    })
}

fn collect_field_violations(
    scope: &str,
    field: &WritebackField,
    output: &mut Vec<ClosureViolation>,
) {
    let subject = format!("{scope}:{}[{}]", field.symbol, field.value.unit_label());
    let value = field.value.as_f64();

    collect_check(
        check_finite(
            "INV-WRITEBACK-001",
            WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID,
            subject.as_str(),
            value,
        ),
        output,
    );

    match (field.minimum, field.maximum) {
        (Some(minimum), Some(maximum)) => collect_check(
            check_range(
                "INV-WRITEBACK-002",
                WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                subject.as_str(),
                value,
                minimum,
                maximum,
            ),
            output,
        ),
        (Some(minimum), None) => collect_check(
            check_min(
                "INV-WRITEBACK-003",
                WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                subject.as_str(),
                value,
                minimum,
            ),
            output,
        ),
        (None, Some(maximum)) => collect_check(
            check_max(
                "INV-WRITEBACK-004",
                WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
                subject.as_str(),
                value,
                maximum,
            ),
            output,
        ),
        (None, None) => {}
    }
}

fn collect_check(result: Result<(), Box<ClosureViolation>>, output: &mut Vec<ClosureViolation>) {
    if let Err(violation) = result {
        output.push(*violation);
    }
}

#[cfg(test)]
mod tests {
    use openwepp_sim_contract::status::{SimulationPhase, StatusClassification};

    use super::*;

    #[test]
    fn accepts_finite_domain_valid_payload() {
        let payload = KernelWritebackPayload::with_updates(
            vec![WritebackField::bounded("st", 10.0, Some(0.0), None)],
            vec![WritebackField::unbounded("runoff", 1.5)],
        );

        let decision = evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, &payload)
            .expect("decision should construct");

        assert_eq!(decision.outcome, WritebackDecisionOutcome::Accept);
        assert_eq!(
            decision.status.classification(),
            StatusClassification::Nominal
        );
        assert!(decision.violations.is_empty());
    }

    #[test]
    fn accepts_unit_boundary_typed_values() {
        let storage = StorageVolumeCubicMeters::try_new(12.0).expect("storage should construct");
        let flow = FlowRateCubicMetersPerSecond::try_new(0.25).expect("flow should construct");
        let payload = KernelWritebackPayload::with_updates(
            vec![WritebackField::bounded("st", storage, Some(0.0), None)],
            vec![WritebackField::bounded("qout", flow, Some(0.0), None)],
        );

        let decision = evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, &payload)
            .expect("decision should construct");

        assert_eq!(decision.outcome, WritebackDecisionOutcome::Accept);
        assert!(decision.violations.is_empty());
    }

    #[test]
    fn rejects_non_finite_payload_with_typed_status() {
        let payload = KernelWritebackPayload::with_updates(
            vec![WritebackField::unbounded("st", f64::NAN)],
            Vec::new(),
        );

        let decision = evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, &payload)
            .expect("decision should construct");

        assert_eq!(decision.outcome, WritebackDecisionOutcome::Reject);
        assert_eq!(
            decision.status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            decision.status.message_id(),
            WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID
        );
        assert_eq!(decision.violations.len(), 1);
    }

    #[test]
    fn apply_requires_accept_outcome() {
        let payload = KernelWritebackPayload::empty();
        let reject_decision = KernelWritebackDecision {
            outcome: WritebackDecisionOutcome::Reject,
            status: SimulationStatus::domain_failure(
                SimulationPhase::WatershedKernel,
                BoundaryClass::DomainViolation,
                WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
            )
            .expect("status should construct"),
            violations: Vec::new(),
        };
        let mut state = BTreeMap::new();
        let mut flux = BTreeMap::new();

        let error = apply_kernel_writeback(
            SimulationPhase::WatershedKernel,
            &reject_decision,
            &payload,
            &mut state,
            &mut flux,
        )
        .expect_err("reject decision should not apply");

        assert!(matches!(
            error,
            WritebackError::DecisionNotAccept {
                outcome: WritebackDecisionOutcome::Reject
            }
        ));
    }

    #[test]
    fn climate_forcing_symbol_surface_hillslope_uses_canonical_aliases() {
        let surface = ClimateForcingSymbolSurface::hillslope(3)
            .expect("hillslope symbol surface should build");

        assert_eq!(surface.point_count(), 3);
        assert_eq!(surface.timem_symbols()[0].as_str(), "timem_0001");
        assert_eq!(surface.timem_symbols()[2].as_str(), "timem_0003");
        assert_eq!(surface.intsty_symbols()[0].as_str(), "intsty_0001");
        assert_eq!(surface.intsty_symbols()[2].as_str(), "intsty_0003");
    }

    #[test]
    fn climate_forcing_symbol_surface_watershed_scope_uses_canonical_aliases() {
        let surface = ClimateForcingSymbolSurface::watershed_hillslope(42, 2)
            .expect("watershed symbol surface should build");

        assert_eq!(surface.point_count(), 2);
        assert_eq!(surface.timem_symbols()[0].as_str(), "hs42_timem_0001");
        assert_eq!(surface.timem_symbols()[1].as_str(), "hs42_timem_0002");
        assert_eq!(surface.intsty_symbols()[0].as_str(), "hs42_intsty_0001");
        assert_eq!(surface.intsty_symbols()[1].as_str(), "hs42_intsty_0002");
    }

    #[test]
    fn climate_forcing_symbol_surface_rejects_unsupported_point_count() {
        let error = ClimateForcingSymbolSurface::hillslope(MAX_CLIMATE_FORCING_SERIES_POINTS + 1)
            .expect_err("point count above supported maximum should fail");

        assert!(matches!(
            error,
            ClimateForcingSymbolSurfaceError::PointCountOutOfRange {
                count,
                supported_max
            } if count == MAX_CLIMATE_FORCING_SERIES_POINTS + 1
                && supported_max == MAX_CLIMATE_FORCING_SERIES_POINTS
        ));
    }

    #[test]
    fn phase_class_growth_predicate_matches_contract() {
        assert!(!HillslopeKernelPhaseClass::Hydrology.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyEvapotranspiration.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyLateralTransfer.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyDrainage.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyPlantRootUptake.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyRunoffReconciliation.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyStorageReconciliation.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyPeakRunoff.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::DecompositionTransition.is_growth_transition());
        assert!(!HillslopeKernelPhaseClass::ResiduePartitionTransition.is_growth_transition());
        assert!(HillslopeKernelPhaseClass::GrowthAnnualTransition.is_growth_transition());
        assert!(HillslopeKernelPhaseClass::GrowthPerennialTransition.is_growth_transition());
    }

    #[test]
    fn phase_class_decomposition_predicate_matches_contract() {
        assert!(!HillslopeKernelPhaseClass::Hydrology.is_decomposition_transition());
        assert!(
            !HillslopeKernelPhaseClass::HydrologyEvapotranspiration.is_decomposition_transition()
        );
        assert!(
            !HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage
                .is_decomposition_transition()
        );
        assert!(!HillslopeKernelPhaseClass::HydrologyLateralTransfer.is_decomposition_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyDrainage.is_decomposition_transition());
        assert!(!HillslopeKernelPhaseClass::HydrologyPlantRootUptake.is_decomposition_transition());
        assert!(
            !HillslopeKernelPhaseClass::HydrologyRunoffReconciliation.is_decomposition_transition()
        );
        assert!(
            !HillslopeKernelPhaseClass::HydrologyStorageReconciliation
                .is_decomposition_transition()
        );
        assert!(!HillslopeKernelPhaseClass::HydrologyPeakRunoff.is_decomposition_transition());
        assert!(HillslopeKernelPhaseClass::DecompositionTransition.is_decomposition_transition());
        assert!(
            HillslopeKernelPhaseClass::ResiduePartitionTransition.is_decomposition_transition()
        );
        assert!(!HillslopeKernelPhaseClass::GrowthAnnualTransition.is_decomposition_transition());
        assert!(
            !HillslopeKernelPhaseClass::GrowthPerennialTransition.is_decomposition_transition()
        );
    }

    #[test]
    fn phase_class_hydrology_predicate_matches_contract() {
        assert!(HillslopeKernelPhaseClass::Hydrology.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyEvapotranspiration.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyLateralTransfer.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyDrainage.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyPlantRootUptake.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyRunoffReconciliation.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyStorageReconciliation.is_hydrology_phase());
        assert!(HillslopeKernelPhaseClass::HydrologyPeakRunoff.is_hydrology_phase());
        assert!(!HillslopeKernelPhaseClass::DecompositionTransition.is_hydrology_phase());
        assert!(!HillslopeKernelPhaseClass::ResiduePartitionTransition.is_hydrology_phase());
        assert!(!HillslopeKernelPhaseClass::GrowthAnnualTransition.is_hydrology_phase());
        assert!(!HillslopeKernelPhaseClass::GrowthPerennialTransition.is_hydrology_phase());
    }

    #[test]
    fn request_with_growth_context_preserves_typed_phase_metadata() {
        let state_surface = BTreeMap::new();
        let flux_surface = BTreeMap::new();
        let growth_context =
            HillslopeGrowthKernelContext::new(HillslopeGrowthManagementClass::Perennial, 1.0, 1.0);

        let request = HillslopeKernelRequest::with_phase_context(
            "perennial_growth_transition",
            HillslopeKernelPhaseClass::GrowthPerennialTransition,
            HillslopeConsumerAdapter::Growth,
            Some(growth_context),
            &state_surface,
            &flux_surface,
        );

        assert_eq!(
            request.phase_class,
            HillslopeKernelPhaseClass::GrowthPerennialTransition
        );
        assert_eq!(request.consumer_adapter, HillslopeConsumerAdapter::Growth);
        assert_eq!(request.decomposition_context, None);
        assert_eq!(request.growth_context, Some(growth_context));
    }

    #[test]
    fn request_with_decomposition_context_preserves_typed_phase_metadata() {
        let state_surface = BTreeMap::new();
        let flux_surface = BTreeMap::new();
        let decomposition_context = HillslopeDecompositionKernelContext::new(
            HillslopeDecompositionManagementClass::AnnualOrFallow,
            1.0,
            1.0,
        );

        let request = HillslopeKernelRequest::with_transition_context(
            "decomposition_transition",
            HillslopeKernelPhaseClass::DecompositionTransition,
            HillslopeConsumerAdapter::Decomposition,
            Some(decomposition_context),
            None,
            &state_surface,
            &flux_surface,
        );

        assert_eq!(
            request.phase_class,
            HillslopeKernelPhaseClass::DecompositionTransition
        );
        assert_eq!(
            request.consumer_adapter,
            HillslopeConsumerAdapter::Decomposition
        );
        assert_eq!(request.decomposition_context, Some(decomposition_context));
        assert_eq!(request.growth_context, None);
    }

    #[test]
    fn decomposition_context_can_carry_typed_transition_payload() {
        let payload = HillslopeDecompositionTransitionPayload {
            active_slot_index: 1,
            active_crop_slot_index: 1,
            runtime_day_of_year: 200,
            iresd_seed: 3.0,
            sumrtm_seed: 2.5,
            sumsrm_seed: 1.5,
            control: HillslopeDecompositionTransitionControl::Annual(
                HillslopeAnnualDecompositionControl {
                    resmgt: 1,
                    jdherb: 200,
                    jdburn: 0,
                    jdslge: 0,
                    jdcut: 0,
                    jdmove: 0,
                    fbrnag: 0.0,
                    fbrnog: 0.0,
                    frcut: 0.0,
                    frmove: 0.0,
                    active_action: HillslopeAnnualDecompositionAction::Herbicide,
                },
            ),
        };
        let context = HillslopeDecompositionKernelContext::new(
            HillslopeDecompositionManagementClass::AnnualOrFallow,
            1.0,
            1.0,
        )
        .with_transition_payload(payload);

        assert_eq!(context.transition_payload, Some(payload));
    }

    #[test]
    fn growth_context_can_carry_typed_transition_payload() {
        let payload = HillslopeGrowthTransitionPayload {
            active_slot_index: 1,
            active_crop_slot_index: 1,
            runtime_day_of_year: 200,
            state_before: HillslopeGrowthStateSurface {
                sumgdd: 800.0,
                vdmt: 2.4,
                cancov: 0.65,
                lai: 2.1,
                rtmass: 1.0,
                rtd: 0.35,
                hia: 0.45,
            },
            state_after: HillslopeGrowthStateSurface {
                sumgdd: 0.0,
                vdmt: 0.0,
                cancov: 0.0,
                lai: 0.0,
                rtmass: 0.0,
                rtd: 0.0,
                hia: 0.0,
            },
            control: HillslopeGrowthTransitionControl::Annual(HillslopeAnnualGrowthControl {
                jdharv: 240,
                jdplt: 120,
                rw: 1.3,
                active_action: HillslopeAnnualGrowthAction::HarvestReset,
            }),
        };
        let context = HillslopeGrowthKernelContext::new(
            HillslopeGrowthManagementClass::AnnualOrFallow,
            1.0,
            1.0,
        )
        .with_transition_payload(payload);

        assert_eq!(context.transition_payload, Some(payload));
    }
}
