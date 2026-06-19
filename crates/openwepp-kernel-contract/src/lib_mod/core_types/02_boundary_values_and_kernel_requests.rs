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

/// One id-backed scalar writeback field proposed by a migrated kernel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IndexedWritebackField {
    pub id: SymbolId,
    pub value: BoundaryValue,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
}

impl IndexedWritebackField {
    #[must_use]
    pub const fn unbounded(id: SymbolId, value: BoundaryValue) -> Self {
        Self {
            id,
            value,
            minimum: None,
            maximum: None,
        }
    }

    #[must_use]
    pub const fn bounded(
        id: SymbolId,
        value: BoundaryValue,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Self {
        Self {
            id,
            value,
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

/// Kernel-proposed id-backed writeback payload.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct IndexedKernelWritebackPayload {
    pub state_updates: Vec<IndexedWritebackField>,
    pub flux_updates: Vec<IndexedWritebackField>,
}

impl IndexedKernelWritebackPayload {
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_updates(
        state_updates: Vec<IndexedWritebackField>,
        flux_updates: Vec<IndexedWritebackField>,
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
    pub indexed_writeback: Option<IndexedKernelWritebackPayload>,
}

impl KernelRunResponse {
    #[must_use]
    pub const fn new(status: SimulationStatus, writeback: KernelWritebackPayload) -> Self {
        Self {
            status,
            writeback,
            indexed_writeback: None,
        }
    }

    #[must_use]
    pub const fn with_indexed_writeback(
        status: SimulationStatus,
        indexed_writeback: IndexedKernelWritebackPayload,
    ) -> Self {
        Self {
            status,
            writeback: KernelWritebackPayload {
                state_updates: Vec::new(),
                flux_updates: Vec::new(),
            },
            indexed_writeback: Some(indexed_writeback),
        }
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
    pub indexed_state_surface: Option<&'a IndexedSurface>,
    pub indexed_flux_surface: Option<&'a IndexedSurface>,
    pub hot_symbol_tables: Option<&'a HotSymbolTables>,
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
        Self::with_transition_context_and_indexed(
            phase_name,
            phase_class,
            consumer_adapter,
            decomposition_context,
            growth_context,
            state_surface,
            flux_surface,
            None,
            None,
        )
    }

    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn with_transition_context_and_indexed(
        phase_name: &'a str,
        phase_class: HillslopeKernelPhaseClass,
        consumer_adapter: HillslopeConsumerAdapter,
        decomposition_context: Option<HillslopeDecompositionKernelContext>,
        growth_context: Option<HillslopeGrowthKernelContext>,
        state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
        indexed_writeback_surface: Option<&'a IndexedWritebackSurface>,
        hot_symbol_tables: Option<&'a HotSymbolTables>,
    ) -> Self {
        Self {
            phase_name,
            phase_class,
            consumer_adapter,
            decomposition_context,
            growth_context,
            state_surface,
            flux_surface,
            indexed_state_surface: indexed_writeback_surface
                .map(IndexedWritebackSurface::state_surface),
            indexed_flux_surface: indexed_writeback_surface
                .map(IndexedWritebackSurface::flux_surface),
            hot_symbol_tables,
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

    #[must_use]
    pub fn indexed_state_value(&self, symbol: &IndexedBoundarySymbol) -> Option<BoundaryValue> {
        self.indexed_state_surface
            .and_then(|surface| surface.get(symbol.id))
    }

    #[must_use]
    pub fn indexed_flux_value(&self, symbol: &IndexedBoundarySymbol) -> Option<BoundaryValue> {
        self.indexed_flux_surface
            .and_then(|surface| surface.get(symbol.id))
    }

    #[must_use]
    pub fn has_indexed_state_surface(&self) -> bool {
        self.indexed_state_surface.is_some()
    }

    #[must_use]
    pub fn has_indexed_flux_surface(&self) -> bool {
        self.indexed_flux_surface.is_some()
    }

    #[must_use]
    pub fn hot_state_scalar(&self, symbol: &str) -> Option<&IndexedBoundarySymbol> {
        self.hot_symbol_tables
            .and_then(|tables| tables.state_scalar(symbol))
    }

    #[must_use]
    pub fn hot_flux_scalar(&self, symbol: &str) -> Option<&IndexedBoundarySymbol> {
        self.hot_symbol_tables
            .and_then(|tables| tables.flux_scalar(symbol))
    }

    #[must_use]
    pub fn hot_state_series_symbol(
        &self,
        root: &str,
        one_based_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.hot_symbol_tables
            .and_then(|tables| tables.state_series_symbol(root, one_based_index))
    }

    #[must_use]
    pub fn hot_flux_series_symbol(
        &self,
        root: &str,
        one_based_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.hot_symbol_tables
            .and_then(|tables| tables.flux_series_symbol(root, one_based_index))
    }

    #[must_use]
    pub fn hot_state_grid_symbol(
        &self,
        root: &str,
        first_index: usize,
        second_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.hot_symbol_tables
            .and_then(|tables| tables.state_grid_symbol(root, first_index, second_index))
    }

    #[must_use]
    pub fn hot_flux_grid_symbol(
        &self,
        root: &str,
        first_index: usize,
        second_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.hot_symbol_tables
            .and_then(|tables| tables.flux_grid_symbol(root, first_index, second_index))
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
