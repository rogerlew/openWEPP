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
