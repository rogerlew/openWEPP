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
        self.scalar_value()
    }

    fn scalar_value(self) -> f64 {
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
        self.unit_label_for_variant()
    }

    const fn unit_label_for_variant(self) -> &'static str {
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
        self.phase_label()
    }

    const fn phase_label(self) -> &'static str {
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
        self.adapter_label()
    }

    const fn adapter_label(self) -> &'static str {
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
    pub dense_state_slot_view: Option<DenseBoundarySlotView<'a>>,
    pub dense_flux_slot_view: Option<DenseBoundarySlotView<'a>>,
    pub dense_state_slots: Option<&'a [Option<BoundaryValue>]>,
    pub dense_flux_slots: Option<&'a [Option<BoundaryValue>]>,
    pub symbol_registry: Option<&'a SymbolRegistry>,
    pub indexed_state_surface: Option<&'a IndexedSurface>,
    pub indexed_flux_surface: Option<&'a IndexedSurface>,
    pub hot_symbol_tables: Option<&'a HotSymbolTables>,
}

/// Compact dense slots addressed by global [`SymbolId`] through a borrowed
/// id-to-slot map.
///
/// PERFDEEP02 used registry-sized value slices. PERFDEEP03 keeps only the hot
/// carried values compact while this metadata view preserves existing indexed
/// kernel reads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DenseBoundarySlotView<'a> {
    id_to_slot: &'a [Option<usize>],
    slots: &'a [Option<BoundaryValue>],
}

impl<'a> DenseBoundarySlotView<'a> {
    #[must_use]
    pub const fn new(
        id_to_slot: &'a [Option<usize>],
        slots: &'a [Option<BoundaryValue>],
    ) -> Self {
        Self { id_to_slot, slots }
    }

    #[must_use]
    pub fn get(self, id: SymbolId) -> Option<BoundaryValue> {
        self.id_to_slot
            .get(id.as_usize())
            .copied()
            .flatten()
            .and_then(|slot| self.slots.get(slot))
            .copied()
            .flatten()
    }

    #[must_use]
    pub fn contains_id(self, id: SymbolId) -> bool {
        self.id_to_slot
            .get(id.as_usize())
            .copied()
            .flatten()
            .is_some()
    }

    #[must_use]
    pub fn slot_count(self) -> usize {
        self.slots.len()
    }
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
        Self::with_transition_context_and_dense_slots(
            phase_name,
            phase_class,
            consumer_adapter,
            decomposition_context,
            growth_context,
            state_surface,
            flux_surface,
            None,
            None,
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
        Self::with_transition_context_and_dense_slots(
            phase_name,
            phase_class,
            consumer_adapter,
            decomposition_context,
            growth_context,
            state_surface,
            flux_surface,
            None,
            None,
            indexed_writeback_surface,
            hot_symbol_tables,
        )
    }

    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn with_transition_context_and_dense_slots(
        phase_name: &'a str,
        phase_class: HillslopeKernelPhaseClass,
        consumer_adapter: HillslopeConsumerAdapter,
        decomposition_context: Option<HillslopeDecompositionKernelContext>,
        growth_context: Option<HillslopeGrowthKernelContext>,
        state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
        dense_state_slots: Option<&'a [Option<BoundaryValue>]>,
        dense_flux_slots: Option<&'a [Option<BoundaryValue>]>,
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
            dense_state_slot_view: None,
            dense_flux_slot_view: None,
            dense_state_slots,
            dense_flux_slots,
            symbol_registry: None,
            indexed_state_surface: indexed_writeback_surface
                .map(IndexedWritebackSurface::state_surface),
            indexed_flux_surface: indexed_writeback_surface
                .map(IndexedWritebackSurface::flux_surface),
            hot_symbol_tables,
        }
    }

    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn with_transition_context_and_dense_slot_views(
        phase_name: &'a str,
        phase_class: HillslopeKernelPhaseClass,
        consumer_adapter: HillslopeConsumerAdapter,
        decomposition_context: Option<HillslopeDecompositionKernelContext>,
        growth_context: Option<HillslopeGrowthKernelContext>,
        state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
        dense_state_slot_view: Option<DenseBoundarySlotView<'a>>,
        dense_flux_slot_view: Option<DenseBoundarySlotView<'a>>,
        symbol_registry: Option<&'a SymbolRegistry>,
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
            dense_state_slot_view,
            dense_flux_slot_view,
            dense_state_slots: None,
            dense_flux_slots: None,
            symbol_registry,
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
        if !self.has_dense_state_surface() {
            return self
                .indexed_state_surface
                .and_then(|surface| surface.get(symbol.id));
        }
        if let Some(value) = self
            .dense_state_slot_view
            .and_then(|view| view.get(symbol.id))
        {
            return Some(value);
        }
        if let Some(value) = self
            .dense_state_slots
            .and_then(|slots| slots.get(symbol.id.as_usize()))
            .copied()
            .flatten()
        {
            return Some(value);
        }
        self.indexed_state_surface
            .and_then(|surface| surface.get(symbol.id))
    }

    #[must_use]
    pub fn dense_state_value_for_symbol(
        &self,
        symbol: &BoundarySymbol,
    ) -> Option<BoundaryValue> {
        let id = self
            .symbol_registry
            .and_then(|registry| registry.id_of(symbol).ok())?;
        self.dense_state_slot_view
            .and_then(|view| view.get(id))
            .or_else(|| {
                self.dense_state_slots
                    .and_then(|slots| slots.get(id.as_usize()))
                    .copied()
                    .flatten()
            })
    }

    #[must_use]
    pub fn dense_flux_value_for_symbol(&self, symbol: &BoundarySymbol) -> Option<BoundaryValue> {
        let id = self
            .symbol_registry
            .and_then(|registry| registry.id_of(symbol).ok())?;
        self.dense_flux_slot_view
            .and_then(|view| view.get(id))
            .or_else(|| {
                self.dense_flux_slots
                    .and_then(|slots| slots.get(id.as_usize()))
                    .copied()
                    .flatten()
            })
    }

    #[must_use]
    pub fn indexed_flux_value(&self, symbol: &IndexedBoundarySymbol) -> Option<BoundaryValue> {
        if !self.has_dense_flux_surface() {
            return self
                .indexed_flux_surface
                .and_then(|surface| surface.get(symbol.id));
        }
        if let Some(value) = self
            .dense_flux_slot_view
            .and_then(|view| view.get(symbol.id))
        {
            return Some(value);
        }
        if let Some(value) = self
            .dense_flux_slots
            .and_then(|slots| slots.get(symbol.id.as_usize()))
            .copied()
            .flatten()
        {
            return Some(value);
        }
        self.indexed_flux_surface
            .and_then(|surface| surface.get(symbol.id))
    }

    #[must_use]
    pub fn has_indexed_state_surface(&self) -> bool {
        self.has_dense_state_surface() || self.indexed_state_surface.is_some()
    }

    #[must_use]
    pub fn has_indexed_flux_surface(&self) -> bool {
        self.has_dense_flux_surface() || self.indexed_flux_surface.is_some()
    }

    #[must_use]
    pub fn has_dense_state_surface(&self) -> bool {
        self.dense_state_slot_view.is_some() || self.dense_state_slots.is_some()
    }

    #[must_use]
    pub fn has_dense_flux_surface(&self) -> bool {
        self.dense_flux_slot_view.is_some() || self.dense_flux_slots.is_some()
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

/// Hillslope kernel trait boundary.
pub trait HillslopeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse;
}

#[cfg(test)]
mod cqr_boundary_request_tests {
    use std::collections::BTreeMap;

    use openwepp_sim_contract::status::SimulationPhase;
    use openwepp_unit_boundary::{
        DensityKilogramsPerCubicMeter, DirectionDegrees, ElapsedTimeSeconds,
        FlowRateCubicMetersPerSecond, FractionUnitInterval, HourOfDay,
        LinearRateMetersPerSecond, ProcessRateMillimetersPerHour,
        RunoffDepthMillimeters, SolarRadiationLangleysPerDay,
        SolarRadiationMegajoulesPerSquareMeterPerDay,
        SolarRadiationMegajoulesPerSquareMeterPerHour, StorageVolumeCubicMeters,
        SurfaceAreaSquareMeters, TemperatureCelsius, WaterDepthMeters,
    };

    use super::*;

    fn assert_boundary_value_cases(cases: &[(BoundaryValue, f64, &'static str)]) {
        for (value, expected_scalar, expected_unit) in cases {
            assert!((value.as_f64() - expected_scalar).abs() <= f64::EPSILON);
            assert_eq!(value.unit_label(), *expected_unit);
        }
    }

    #[test]
    fn boundary_value_units_and_scalars_cover_hydrologic_variants() {
        assert_boundary_value_cases(&[
            (BoundaryValue::scalar(1.0), 1.0, "scalar"),
            (
                RunoffDepthMillimeters::try_new(2.0).expect("valid runoff depth").into(),
                2.0,
                "mm",
            ),
            (
                FlowRateCubicMetersPerSecond::try_new(3.0)
                    .expect("valid flow rate")
                    .into(),
                3.0,
                "m3/s",
            ),
            (
                StorageVolumeCubicMeters::try_new(4.0)
                    .expect("valid storage volume")
                    .into(),
                4.0,
                "m3",
            ),
            (
                ProcessRateMillimetersPerHour::try_new(5.0)
                    .expect("valid process rate")
                    .into(),
                5.0,
                "mm/hr",
            ),
            (
                SurfaceAreaSquareMeters::try_new(6.0)
                    .expect("valid surface area")
                    .into(),
                6.0,
                "m2",
            ),
            (
                WaterDepthMeters::try_new(7.0).expect("valid water depth").into(),
                7.0,
                "m",
            ),
            (
                ElapsedTimeSeconds::try_new(8.0)
                    .expect("valid elapsed time")
                    .into(),
                8.0,
                "s",
            ),
            (
                HourOfDay::try_new(9.0).expect("valid hour").into(),
                9.0,
                "h",
            ),
            (
                LinearRateMetersPerSecond::try_new(10.0)
                    .expect("valid linear rate")
                    .into(),
                10.0,
                "m s^-1",
            ),
        ]);
    }

    #[test]
    fn boundary_value_units_and_scalars_cover_climate_variants() {
        assert_boundary_value_cases(&[
            (
                SolarRadiationLangleysPerDay::try_new(11.0)
                    .expect("valid daily langleys")
                    .into(),
                11.0,
                "Ly d^-1",
            ),
            (
                SolarRadiationMegajoulesPerSquareMeterPerDay::try_new(12.0)
                    .expect("valid daily radiation")
                    .into(),
                12.0,
                "MJ m^-2 d^-1",
            ),
            (
                SolarRadiationMegajoulesPerSquareMeterPerHour::try_new(13.0)
                    .expect("valid hourly radiation")
                    .into(),
                13.0,
                "MJ m^-2 h^-1",
            ),
            (
                TemperatureCelsius::try_new(14.0)
                    .expect("valid temperature")
                    .into(),
                14.0,
                "degC",
            ),
            (
                DirectionDegrees::try_new(15.0)
                    .expect("valid direction")
                    .into(),
                15.0,
                "deg",
            ),
            (
                DensityKilogramsPerCubicMeter::try_new(16.0)
                    .expect("valid density")
                    .into(),
                16.0,
                "kg m^-3",
            ),
            (
                FractionUnitInterval::try_new(0.5)
                    .expect("valid fraction")
                    .into(),
                0.5,
                "dimensionless",
            ),
        ]);
    }

    #[test]
    fn boundary_value_named_constructors_preserve_unit_validation() {
        let values = [
            BoundaryValue::water_depth_meters(1.0),
            BoundaryValue::elapsed_time_seconds(2.0),
            BoundaryValue::hour_of_day(3.0),
            BoundaryValue::linear_rate_meters_per_second(4.0),
            BoundaryValue::solar_radiation_langleys_per_day(5.0),
            BoundaryValue::solar_radiation_megajoules_per_square_meter_per_day(6.0),
            BoundaryValue::solar_radiation_megajoules_per_square_meter_per_hour(7.0),
            BoundaryValue::temperature_celsius(8.0),
            BoundaryValue::direction_degrees(9.0),
            BoundaryValue::density_kilograms_per_cubic_meter(10.0),
            BoundaryValue::fraction_unit_interval(0.25),
        ];

        assert!(values.into_iter().all(|value| value.is_ok()));
        assert!(matches!(
            BoundaryValue::water_depth_meters(-1.0),
            Err(BoundaryError::BelowMinimum { .. })
        ));
        assert!(matches!(
            BoundaryValue::fraction_unit_interval(1.1),
            Err(BoundaryError::AboveMaximum { .. })
        ));
        assert!(matches!(
            BoundaryValue::temperature_celsius(f64::NAN),
            Err(BoundaryError::NonFinite { .. })
        ));
    }

    #[test]
    fn writeback_payloads_and_responses_preserve_typed_fields() {
        let state = WritebackField::unbounded("state", BoundaryValue::scalar(1.0));
        let flux = WritebackField::bounded("flux", BoundaryValue::scalar(2.0), Some(0.0), None);
        let payload = KernelWritebackPayload::with_updates(vec![state.clone()], vec![flux.clone()]);
        assert_eq!(KernelWritebackPayload::empty(), KernelWritebackPayload::default());
        assert_eq!(payload.state_updates, vec![state]);
        assert_eq!(payload.flux_updates, vec![flux]);

        let status = SimulationStatus::ok(SimulationPhase::HillslopeKernel, "CQR-T01-OK")
            .expect("valid nominal status");
        let response = KernelRunResponse::new(status.clone(), payload.clone());
        assert_eq!(response.status, status);
        assert_eq!(response.writeback, payload);
        assert_eq!(response.indexed_writeback, None);

        let indexed_payload = IndexedKernelWritebackPayload::with_updates(
            vec![IndexedWritebackField::unbounded(
                SymbolRegistry::from_symbols(["state"])
                    .expect("registry")
                    .id_of(&BoundarySymbol::from("state"))
                    .expect("registered state"),
                BoundaryValue::scalar(3.0),
            )],
            vec![],
        );
        assert_eq!(IndexedKernelWritebackPayload::empty(), IndexedKernelWritebackPayload::default());
        assert_eq!(
            IndexedWritebackField::bounded(
                SymbolRegistry::from_symbols(["bounded"])
                    .expect("registry")
                    .id_of(&BoundarySymbol::from("bounded"))
                    .expect("registered bounded field"),
                BoundaryValue::scalar(4.0),
                Some(1.0),
                Some(5.0),
            )
            .minimum,
            Some(1.0)
        );
        assert!(KernelRunResponse::with_indexed_writeback(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "CQR-T01-INDEXED")
                .expect("valid nominal status"),
            indexed_payload
        )
        .indexed_writeback
        .is_some());
    }

    #[test]
    fn phase_and_consumer_labels_cover_all_dispatch_classes() {
        let phases = [
            (HillslopeKernelPhaseClass::Hydrology, "hydrology", true, false, false),
            (
                HillslopeKernelPhaseClass::HydrologyEvapotranspiration,
                "hydrology_evapotranspiration",
                true,
                false,
                false,
            ),
            (
                HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage,
                "hydrology_percolation_deep_seepage",
                true,
                false,
                false,
            ),
            (
                HillslopeKernelPhaseClass::HydrologyLateralTransfer,
                "hydrology_lateral_transfer",
                true,
                false,
                false,
            ),
            (HillslopeKernelPhaseClass::HydrologyDrainage, "hydrology_drainage", true, false, false),
            (
                HillslopeKernelPhaseClass::HydrologyPlantRootUptake,
                "hydrology_plant_root_uptake",
                true,
                false,
                false,
            ),
            (
                HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                "hydrology_runoff_reconciliation",
                true,
                false,
                false,
            ),
            (
                HillslopeKernelPhaseClass::HydrologyStorageReconciliation,
                "hydrology_storage_reconciliation",
                true,
                false,
                false,
            ),
            (
                HillslopeKernelPhaseClass::HydrologyPeakRunoff,
                "hydrology_peak_runoff",
                true,
                false,
                false,
            ),
            (
                HillslopeKernelPhaseClass::DecompositionTransition,
                "decomposition_transition",
                false,
                false,
                true,
            ),
            (
                HillslopeKernelPhaseClass::ResiduePartitionTransition,
                "residue_partition_transition",
                false,
                false,
                true,
            ),
            (
                HillslopeKernelPhaseClass::GrowthAnnualTransition,
                "growth_annual_transition",
                false,
                true,
                false,
            ),
            (
                HillslopeKernelPhaseClass::GrowthPerennialTransition,
                "growth_perennial_transition",
                false,
                true,
                false,
            ),
        ];

        for (phase, label, hydrology, growth, decomposition) in phases {
            assert_eq!(phase.as_str(), label);
            assert_eq!(phase.is_hydrology_phase(), hydrology);
            assert_eq!(phase.is_growth_transition(), growth);
            assert_eq!(phase.is_decomposition_transition(), decomposition);
        }

        let adapters = [
            (HillslopeConsumerAdapter::Runoff, "runoff"),
            (HillslopeConsumerAdapter::Soil, "soil"),
            (HillslopeConsumerAdapter::Watbal, "watbal"),
            (HillslopeConsumerAdapter::Perc, "perc"),
            (HillslopeConsumerAdapter::Decomposition, "decomposition"),
            (HillslopeConsumerAdapter::Growth, "growth"),
        ];
        for (adapter, label) in adapters {
            assert_eq!(adapter.as_str(), label);
        }
    }

    #[test]
    fn dense_slot_request_reads_dense_values_before_indexed_fallback() {
        let state_symbol = BoundarySymbol::from("state");
        let flux_symbol = BoundarySymbol::from("flux");
        let mut state_surface = BTreeMap::new();
        let mut flux_surface = BTreeMap::new();
        state_surface.insert(state_symbol.clone(), BoundaryValue::scalar(1.0));
        flux_surface.insert(flux_symbol.clone(), BoundaryValue::scalar(2.0));
        let registry = SymbolRegistry::from_surfaces(&state_surface, &flux_surface)
            .expect("registry from surfaces");
        let state_id = registry.id_of(&state_symbol).expect("state id");
        let flux_id = registry.id_of(&flux_symbol).expect("flux id");
        let mut state_slots = vec![None; registry.len()];
        let mut flux_slots = vec![None; registry.len()];
        state_slots[state_id.as_usize()] = Some(BoundaryValue::scalar(3.0));
        flux_slots[flux_id.as_usize()] = Some(BoundaryValue::scalar(4.0));
        let id_to_slot = (0..registry.len()).map(Some).collect::<Vec<_>>();
        let indexed_surface = IndexedWritebackSurface::from_btreemap_surfaces(
            &registry,
            &state_surface,
            &flux_surface,
        )
        .expect("indexed surface");
        let request = HillslopeKernelRequest::with_transition_context_and_dense_slot_views(
            "dense",
            HillslopeKernelPhaseClass::Hydrology,
            HillslopeConsumerAdapter::Runoff,
            None,
            None,
            &state_surface,
            &flux_surface,
            Some(DenseBoundarySlotView::new(&id_to_slot, &state_slots)),
            Some(DenseBoundarySlotView::new(&id_to_slot, &flux_slots)),
            Some(&registry),
            Some(&indexed_surface),
            None,
        );
        let indexed_state = IndexedBoundarySymbol::new(state_symbol.clone(), state_id);
        let indexed_flux = IndexedBoundarySymbol::new(flux_symbol.clone(), flux_id);

        assert!(request.has_dense_state_surface());
        assert!(request.has_dense_flux_surface());
        assert!(request.has_indexed_state_surface());
        assert!(request.has_indexed_flux_surface());
        assert_eq!(request.indexed_state_value(&indexed_state), Some(BoundaryValue::scalar(3.0)));
        assert_eq!(request.indexed_flux_value(&indexed_flux), Some(BoundaryValue::scalar(4.0)));
        assert_eq!(request.dense_state_value_for_symbol(&state_symbol), Some(BoundaryValue::scalar(3.0)));
        assert_eq!(request.dense_flux_value_for_symbol(&flux_symbol), Some(BoundaryValue::scalar(4.0)));
        assert!(DenseBoundarySlotView::new(&id_to_slot, &state_slots).contains_id(state_id));
        assert_eq!(DenseBoundarySlotView::new(&id_to_slot, &state_slots).slot_count(), registry.len());
    }

    fn assert_plain_and_indexed_fallback(
        state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
        indexed_surface: &IndexedWritebackSurface,
        indexed_state: &IndexedBoundarySymbol,
        indexed_flux: &IndexedBoundarySymbol,
    ) {
        let plain_request = HillslopeKernelRequest::new(
            "plain",
            HillslopeConsumerAdapter::Runoff,
            state_surface,
            flux_surface,
        );
        assert!(!plain_request.has_indexed_state_surface());
        assert!(!plain_request.has_indexed_flux_surface());
        assert_eq!(plain_request.indexed_state_value(indexed_state), None);
        assert_eq!(plain_request.indexed_flux_value(indexed_flux), None);

        let no_dense_request = HillslopeKernelRequest::with_transition_context_and_indexed(
            "indexed",
            HillslopeKernelPhaseClass::Hydrology,
            HillslopeConsumerAdapter::Runoff,
            None,
            None,
            state_surface,
            flux_surface,
            Some(indexed_surface),
            None,
        );
        assert_eq!(
            no_dense_request.indexed_state_value(indexed_state),
            Some(BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            no_dense_request.indexed_flux_value(indexed_flux),
            Some(BoundaryValue::scalar(2.0))
        );
    }

    #[test]
    fn request_lookup_preserves_legacy_slot_and_indexed_fallback_order() {
        let state_symbol = BoundarySymbol::from("state");
        let flux_symbol = BoundarySymbol::from("flux");
        let mut state_surface = BTreeMap::new();
        let mut flux_surface = BTreeMap::new();
        state_surface.insert(state_symbol.clone(), BoundaryValue::scalar(1.0));
        flux_surface.insert(flux_symbol.clone(), BoundaryValue::scalar(2.0));
        let registry = SymbolRegistry::from_surfaces(&state_surface, &flux_surface)
            .expect("registry from surfaces");
        let state_id = registry.id_of(&state_symbol).expect("state id");
        let flux_id = registry.id_of(&flux_symbol).expect("flux id");
        let indexed_surface = IndexedWritebackSurface::from_btreemap_surfaces(
            &registry,
            &state_surface,
            &flux_surface,
        )
        .expect("indexed surface");
        let indexed_state = IndexedBoundarySymbol::new(state_symbol.clone(), state_id);
        let indexed_flux = IndexedBoundarySymbol::new(flux_symbol.clone(), flux_id);

        assert_plain_and_indexed_fallback(
            &state_surface,
            &flux_surface,
            &indexed_surface,
            &indexed_state,
            &indexed_flux,
        );

        let mut legacy_state_slots = vec![None; registry.len()];
        let mut legacy_flux_slots = vec![None; registry.len()];
        legacy_state_slots[state_id.as_usize()] = Some(BoundaryValue::scalar(5.0));
        legacy_flux_slots[flux_id.as_usize()] = Some(BoundaryValue::scalar(6.0));
        let legacy_request = HillslopeKernelRequest::with_transition_context_and_dense_slots(
            "legacy",
            HillslopeKernelPhaseClass::Hydrology,
            HillslopeConsumerAdapter::Runoff,
            None,
            None,
            &state_surface,
            &flux_surface,
            Some(&legacy_state_slots),
            Some(&legacy_flux_slots),
            Some(&indexed_surface),
            None,
        );
        assert_eq!(
            legacy_request.indexed_state_value(&indexed_state),
            Some(BoundaryValue::scalar(5.0))
        );
        assert_eq!(
            legacy_request.indexed_flux_value(&indexed_flux),
            Some(BoundaryValue::scalar(6.0))
        );
        assert_eq!(legacy_request.dense_state_value_for_symbol(&state_symbol), None);
        assert_eq!(legacy_request.dense_flux_value_for_symbol(&flux_symbol), None);

        let missing_slots = vec![None; registry.len()];
        let fallback_request = HillslopeKernelRequest::with_transition_context_and_dense_slots(
            "fallback",
            HillslopeKernelPhaseClass::Hydrology,
            HillslopeConsumerAdapter::Runoff,
            None,
            None,
            &state_surface,
            &flux_surface,
            Some(&missing_slots),
            Some(&missing_slots),
            Some(&indexed_surface),
            None,
        );
        assert_eq!(
            fallback_request.indexed_state_value(&indexed_state),
            Some(BoundaryValue::scalar(1.0))
        );
        assert_eq!(
            fallback_request.indexed_flux_value(&indexed_flux),
            Some(BoundaryValue::scalar(2.0))
        );
        assert_eq!(fallback_request.dense_state_value_for_symbol(&state_symbol), None);
        assert_eq!(fallback_request.dense_flux_value_for_symbol(&flux_symbol), None);
        let missing_symbol = BoundarySymbol::from("missing");
        assert_eq!(fallback_request.dense_state_value_for_symbol(&missing_symbol), None);
        assert_eq!(fallback_request.dense_flux_value_for_symbol(&missing_symbol), None);

        let missing_slot_map = vec![None; registry.len()];
        let missing_view = DenseBoundarySlotView::new(&missing_slot_map, &missing_slots);
        assert_eq!(missing_view.get(state_id), None);
        assert!(!missing_view.contains_id(state_id));
    }

    #[test]
    fn dense_symbol_lookup_falls_back_from_empty_view_to_legacy_slots() {
        let state_symbol = BoundarySymbol::from("state");
        let flux_symbol = BoundarySymbol::from("flux");
        let mut state_surface = BTreeMap::new();
        let mut flux_surface = BTreeMap::new();
        state_surface.insert(state_symbol.clone(), BoundaryValue::scalar(1.0));
        flux_surface.insert(flux_symbol.clone(), BoundaryValue::scalar(2.0));
        let registry = SymbolRegistry::from_surfaces(&state_surface, &flux_surface)
            .expect("registry from surfaces");
        let state_id = registry.id_of(&state_symbol).expect("state id");
        let flux_id = registry.id_of(&flux_symbol).expect("flux id");
        let mut state_slots = vec![None; registry.len()];
        let mut flux_slots = vec![None; registry.len()];
        state_slots[state_id.as_usize()] = Some(BoundaryValue::scalar(7.0));
        flux_slots[flux_id.as_usize()] = Some(BoundaryValue::scalar(8.0));
        let missing_slots = vec![None; registry.len()];
        let missing_slot_map = vec![None; registry.len()];
        let mut request = HillslopeKernelRequest::with_transition_context_and_dense_slots(
            "fallback_view",
            HillslopeKernelPhaseClass::Hydrology,
            HillslopeConsumerAdapter::Runoff,
            None,
            None,
            &state_surface,
            &flux_surface,
            Some(&state_slots),
            Some(&flux_slots),
            None,
            None,
        );
        request.symbol_registry = Some(&registry);
        request.dense_state_slot_view = Some(DenseBoundarySlotView::new(
            &missing_slot_map,
            &missing_slots,
        ));
        request.dense_flux_slot_view = Some(DenseBoundarySlotView::new(
            &missing_slot_map,
            &missing_slots,
        ));

        assert_eq!(
            request.dense_state_value_for_symbol(&state_symbol),
            Some(BoundaryValue::scalar(7.0))
        );
        assert_eq!(
            request.dense_flux_value_for_symbol(&flux_symbol),
            Some(BoundaryValue::scalar(8.0))
        );
    }

    #[test]
    fn request_hot_symbol_accessors_preserve_state_and_flux_lookup_identity() {
        let state_surface = BTreeMap::new();
        let flux_surface = BTreeMap::new();
        let registry = SymbolRegistry::from_symbols([
            "state_scalar",
            "flux_scalar",
            "state_series_0001",
            "flux_series_0001",
            "state_grid_0001_0002",
            "flux_grid_0001_0002",
        ])
        .expect("registry");
        let hot_tables = HotSymbolTables::from_registry(
            &registry,
            &["state_scalar"],
            &["flux_scalar"],
            &["state_series"],
            &["flux_series"],
            &["state_grid"],
            &["flux_grid"],
        );
        let request = HillslopeKernelRequest::with_transition_context_and_indexed(
            "hot",
            HillslopeKernelPhaseClass::Hydrology,
            HillslopeConsumerAdapter::Runoff,
            None,
            None,
            &state_surface,
            &flux_surface,
            None,
            Some(&hot_tables),
        );

        assert_eq!(
            request.hot_state_scalar("state_scalar").map(|entry| entry.symbol.as_str()),
            Some("state_scalar")
        );
        assert_eq!(
            request.hot_flux_scalar("flux_scalar").map(|entry| entry.symbol.as_str()),
            Some("flux_scalar")
        );
        assert_eq!(
            request
                .hot_state_series_symbol("state_series", 1)
                .map(|entry| entry.symbol.as_str()),
            Some("state_series_0001")
        );
        assert_eq!(
            request
                .hot_flux_series_symbol("flux_series", 1)
                .map(|entry| entry.symbol.as_str()),
            Some("flux_series_0001")
        );
        assert_eq!(
            request
                .hot_state_grid_symbol("state_grid", 1, 2)
                .map(|entry| entry.symbol.as_str()),
            Some("state_grid_0001_0002")
        );
        assert_eq!(
            request
                .hot_flux_grid_symbol("flux_grid", 1, 2)
                .map(|entry| entry.symbol.as_str()),
            Some("flux_grid_0001_0002")
        );
        assert_eq!(request.hot_state_scalar("missing"), None);
        assert_eq!(request.hot_flux_scalar("missing"), None);
    }
}
