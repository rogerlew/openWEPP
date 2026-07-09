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

impl std::error::Error for ClimateForcingSymbolSurfaceError {}

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
    fn from(value: HillslopeProductionStateSymbol) -> Self {
        use HillslopeProductionStateSymbol as H;

        match value {
            H::Wb11SoilWater
            | H::Wb11EtDemand
            | H::Wb17ResidueInterception
            | H::Wb11FieldCapacity
            | H::Wb11PercFraction
            | H::Wb11LateralFraction
            | H::Wb11DrainageFraction
            | H::Wb11DrainageCoefficient
            | H::Wb11DrainableStorage => Self::from(hillslope_wb11_state_symbol(value)),
            H::Wb12RainfallInput
            | H::Wb12RunonInput
            | H::Wb12Infiltration
            | H::Wb12DepressionStorageDelta
            | H::Wb12RunoffObserved
            | H::Wb12RunoffClosureTolerance
            | H::Wb12RunoffReconciled
            | H::Wb12StorageInitial
            | H::Wb12StorageObserved
            | H::Wb12StorageClosureTolerance
            | H::Wb12PrecipInput
            | H::Wb12StorageReconciled => Self::from(hillslope_wb12_state_symbol(value)),
            H::IrrigRuntimeSource
            | H::IrrigRuntimeDepthMeters
            | H::IrrigRuntimeDurationSeconds
            | H::IrrigRuntimeRateMetersPerSecond
            | H::IrrigRuntimeEventIndex
            | H::IrrigRuntimeSystemType
            | H::IrrigDepletionEnabled
            | H::IrrigDepletionSystemType
            | H::IrrigDepletionMinDepthMeters
            | H::IrrigDepletionMaxDepthMeters
            | H::IrrigDepletionPeriodCount
            | H::IrrigFixedDateEnabled
            | H::IrrigFixedDateSystemType
            | H::IrrigFixedDateEventCount => {
                Self::from(hillslope_irrigation_scalar_state_symbol(value))
            }
            H::Wb15PlantCancov
            | H::Wb15PlantLai
            | H::Wb15PlantVdmt
            | H::Wb14HyetographNinten
            | H::Wb14HyetographNbrkpt
            | H::Wb14SoilConductivity
            | H::Wb14SoilLayerDepth
            | H::Wb14SoilThetaResidual
            | H::Wb14SoilThetaFieldCapacity => {
                Self::from(hillslope_plant_hyetograph_soil_state_symbol(value))
            }
            H::Wb14SnowFilePresent
            | H::Wb14SnowRst
            | H::Wb14SnowNewsnw
            | H::Wb14SnowSsd
            | H::Wb14SnowRuntimeSwe
            | H::Wb14FrostFilePresent
            | H::Wb14FrostWintRed
            | H::Wb14FrostFineTop
            | H::Wb14FrostFineBot
            | H::Wb14FrostKsnowf
            | H::Wb14FrostKresf
            | H::Wb14FrostKsoilf
            | H::Wb14FrostKfactor1
            | H::Wb14FrostKfactor2
            | H::Wb14FrostKfactor3
            | H::Wb14FrostRuntimeDfrost
            | H::Wb14FrostRuntimeDthaw
            | H::Wb14FrostRuntimeNft
            | H::Wb14FrostRuntimeWsFrz
            | H::Wb14FrostRuntimeInfcapFrz => {
                Self::from(hillslope_snow_frost_state_symbol(value))
            }
            H::Wb14Tmax
            | H::Wb14Tmin
            | H::Wb16Timep
            | H::Wb16Efflen
            | H::Wb16Ealpha
            | H::Wb16ExponentM
            | H::Wb16Peakro
            | H::Wb16Watdur
            | H::Wb16MethodBranch
            | H::Wb16Tstar
            | H::Wb16Qpstar
            | H::Wb16Vstar => Self::from(hillslope_peak_method_state_symbol(value)),
            H::IrrigationDepletionPeriod {
                period_index,
                field,
            } => Self::from(format!(
                "irrigation.depletion.period_{period_index:04}.{}",
                field.as_str()
            )),
            H::IrrigationFixedDateEvent { event_index, field } => Self::from(format!(
                "irrigation.fixeddate.event_{event_index:04}.{}",
                field.as_str()
            )),
        }
    }
}

fn hillslope_wb11_state_symbol(value: HillslopeProductionStateSymbol) -> &'static str {
    use HillslopeProductionStateSymbol as H;

    match value {
        H::Wb11SoilWater => "wb11_soil_water",
        H::Wb11EtDemand => "wb11_et_demand",
        H::Wb17ResidueInterception => "wb17_residue_interception",
        H::Wb11FieldCapacity => "wb11_field_capacity",
        H::Wb11PercFraction => "wb11_perc_fraction",
        H::Wb11LateralFraction => "wb11_lateral_fraction",
        H::Wb11DrainageFraction => "wb11_drainage_fraction",
        H::Wb11DrainageCoefficient => "wb11_drainage_coefficient",
        H::Wb11DrainableStorage => "wb11_drainable_storage",
        // The exhaustive caller match only routes WB11 variants here.
        _ => unreachable!("caller must pass a WB11 state symbol"),
    }
}

fn hillslope_wb12_state_symbol(value: HillslopeProductionStateSymbol) -> &'static str {
    use HillslopeProductionStateSymbol as H;

    match value {
        H::Wb12RainfallInput => "wb12_rainfall_input",
        H::Wb12RunonInput => "wb12_runon_input",
        H::Wb12Infiltration => "wb12_infiltration",
        H::Wb12DepressionStorageDelta => "wb12_depression_storage_delta",
        H::Wb12RunoffObserved => "wb12_runoff_observed",
        H::Wb12RunoffClosureTolerance => "wb12_runoff_closure_tolerance",
        H::Wb12RunoffReconciled => "wb12_runoff_reconciled",
        H::Wb12StorageInitial => "wb12_storage_initial",
        H::Wb12StorageObserved => "wb12_storage_observed",
        H::Wb12StorageClosureTolerance => "wb12_storage_closure_tolerance",
        H::Wb12PrecipInput => "wb12_precip_input",
        H::Wb12StorageReconciled => "wb12_storage_reconciled",
        // The exhaustive caller match only routes WB12 variants here.
        _ => unreachable!("caller must pass a WB12 state symbol"),
    }
}

fn hillslope_irrigation_scalar_state_symbol(
    value: HillslopeProductionStateSymbol,
) -> &'static str {
    use HillslopeProductionStateSymbol as H;

    match value {
        H::IrrigRuntimeSource => "irrigation.runtime_schedule_source",
        H::IrrigRuntimeDepthMeters => "irrigation.runtime_depth_m",
        H::IrrigRuntimeDurationSeconds => "irrigation.runtime_duration_s",
        H::IrrigRuntimeRateMetersPerSecond => "irrigation.runtime_rate_m_per_s",
        H::IrrigRuntimeEventIndex => "irrigation.runtime_event_index",
        H::IrrigRuntimeSystemType => "irrigation.runtime_system_type",
        H::IrrigDepletionEnabled => "irrigation.depletion.enabled",
        H::IrrigDepletionSystemType => "irrigation.depletion.system_type",
        H::IrrigDepletionMinDepthMeters => "irrigation.depletion.min_depth_m",
        H::IrrigDepletionMaxDepthMeters => "irrigation.depletion.max_depth_m",
        H::IrrigDepletionPeriodCount => "irrigation.depletion.period_count",
        H::IrrigFixedDateEnabled => "irrigation.fixeddate.enabled",
        H::IrrigFixedDateSystemType => "irrigation.fixeddate.system_type",
        H::IrrigFixedDateEventCount => "irrigation.fixeddate.event_count",
        // The exhaustive caller match only routes irrigation scalar variants here.
        _ => unreachable!("caller must pass an irrigation scalar state symbol"),
    }
}

fn hillslope_plant_hyetograph_soil_state_symbol(
    value: HillslopeProductionStateSymbol,
) -> &'static str {
    use HillslopeProductionStateSymbol as H;

    match value {
        H::Wb15PlantCancov => "cancov",
        H::Wb15PlantLai => "lai",
        H::Wb15PlantVdmt => "vdmt",
        H::Wb14HyetographNinten => "ninten",
        H::Wb14HyetographNbrkpt => "nbrkpt",
        H::Wb14SoilConductivity => "ssc",
        H::Wb14SoilLayerDepth => "dg",
        H::Wb14SoilThetaResidual => "thetdr",
        H::Wb14SoilThetaFieldCapacity => "thetfc",
        // The exhaustive caller match only routes plant, hyetograph, and soil variants here.
        _ => unreachable!("caller must pass a plant, hyetograph, or soil state symbol"),
    }
}

fn hillslope_snow_frost_state_symbol(value: HillslopeProductionStateSymbol) -> &'static str {
    use HillslopeProductionStateSymbol as H;

    match value {
        H::Wb14SnowFilePresent => "snow.options.snow_file_present",
        H::Wb14SnowRst => "snow.options.rst",
        H::Wb14SnowNewsnw => "snow.options.newsnw",
        H::Wb14SnowSsd => "snow.options.ssd",
        H::Wb14SnowRuntimeSwe => "snow.runtime_swe",
        H::Wb14FrostFilePresent => "frost.options.frost_file_present",
        H::Wb14FrostWintRed => "frost.options.wintRed",
        H::Wb14FrostFineTop => "frost.options.fineTop",
        H::Wb14FrostFineBot => "frost.options.fineBot",
        H::Wb14FrostKsnowf => "frost.options.ksnowf",
        H::Wb14FrostKresf => "frost.options.kresf",
        H::Wb14FrostKsoilf => "frost.options.ksoilf",
        H::Wb14FrostKfactor1 => "frost.options.kfactor1",
        H::Wb14FrostKfactor2 => "frost.options.kfactor2",
        H::Wb14FrostKfactor3 => "frost.options.kfactor3",
        H::Wb14FrostRuntimeDfrost => "frost.runtime_dfrost",
        H::Wb14FrostRuntimeDthaw => "frost.runtime_dthaw",
        H::Wb14FrostRuntimeNft => "frost.runtime_nft",
        H::Wb14FrostRuntimeWsFrz => "frost.runtime_ws_frz",
        H::Wb14FrostRuntimeInfcapFrz => "frost.runtime_infcap_frz",
        // The exhaustive caller match only routes snow and frost variants here.
        _ => unreachable!("caller must pass a snow or frost state symbol"),
    }
}

fn hillslope_peak_method_state_symbol(value: HillslopeProductionStateSymbol) -> &'static str {
    use HillslopeProductionStateSymbol as H;

    match value {
        H::Wb14Tmax => "tmax",
        H::Wb14Tmin => "tmin",
        H::Wb16Timep => "timep",
        H::Wb16Efflen => "efflen",
        H::Wb16Ealpha => "ealpha",
        H::Wb16ExponentM => "m",
        H::Wb16Peakro => "peakro",
        H::Wb16Watdur => "watdur",
        H::Wb16MethodBranch => "wb16_peak_method_branch",
        H::Wb16Tstar => "wb16_tstar",
        H::Wb16Qpstar => "wb16_qpstar",
        H::Wb16Vstar => "wb16_vstar",
        // The exhaustive caller match only routes temperature and peak-method variants here.
        _ => unreachable!("caller must pass a temperature or peak-method state symbol"),
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
