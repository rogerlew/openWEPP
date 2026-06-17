use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use openwepp_sim_contract::status::SimulationStatus;
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
        let symbol = Self(symbol.into());
        record_constructed_boundary_symbol(&symbol);
        symbol
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

/// Dense identifier assigned by a frozen [`SymbolRegistry`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolId(u32);

impl SymbolId {
    /// Return the raw registry id.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    /// Return the id as a vector index.
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for SymbolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[allow(clippy::cast_possible_truncation)]
fn symbol_registry_supported_max_usize() -> usize {
    u32::MAX as usize
}

#[allow(clippy::cast_possible_truncation)]
fn symbol_id_from_registry_index(index: usize) -> SymbolId {
    debug_assert!(index <= symbol_registry_supported_max_usize());
    SymbolId(index as u32)
}

/// Frozen run-scoped mapping from logical boundary symbols to dense ids.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolRegistry {
    symbols_by_id: Vec<BoundarySymbol>,
    ids_by_symbol: BTreeMap<BoundarySymbol, SymbolId>,
}

impl SymbolRegistry {
    /// Build a frozen registry. Ids are assigned in sorted symbol order.
    pub fn from_symbols<I, S>(symbols: I) -> Result<Self, SymbolRegistryError>
    where
        I: IntoIterator<Item = S>,
        S: Into<BoundarySymbol>,
    {
        let mut symbols_by_id = symbols.into_iter().map(Into::into).collect::<Vec<_>>();
        symbols_by_id.sort();
        symbols_by_id.dedup();

        if symbols_by_id.len() > symbol_registry_supported_max_usize() {
            return Err(SymbolRegistryError::TooManySymbols {
                count: symbols_by_id.len(),
                supported_max: u32::MAX,
            });
        }

        let mut ids_by_symbol = BTreeMap::new();
        for (index, symbol) in symbols_by_id.iter().enumerate() {
            ids_by_symbol.insert(symbol.clone(), symbol_id_from_registry_index(index));
        }

        Ok(Self {
            symbols_by_id,
            ids_by_symbol,
        })
    }

    /// Build a registry from the union of state and flux surface keys.
    pub fn from_surfaces(
        state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Result<Self, SymbolRegistryError> {
        Self::from_symbols(
            state_surface
                .keys()
                .chain(flux_surface.keys())
                .cloned()
                .collect::<Vec<_>>(),
        )
    }

    /// Return the number of symbols in this registry.
    #[must_use]
    pub fn len(&self) -> usize {
        self.symbols_by_id.len()
    }

    /// Return whether this registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.symbols_by_id.is_empty()
    }

    /// Resolve a logical symbol to its dense id.
    pub fn id_of(&self, symbol: &BoundarySymbol) -> Result<SymbolId, SymbolRegistryError> {
        self.ids_by_symbol
            .get(symbol)
            .copied()
            .ok_or_else(|| SymbolRegistryError::UnknownSymbol {
                symbol: symbol.clone(),
            })
    }

    /// Return the symbol for a dense id.
    #[must_use]
    pub fn symbol(&self, id: SymbolId) -> Option<&BoundarySymbol> {
        self.symbols_by_id.get(id.as_usize())
    }

    /// Return true when this registry contains the logical symbol.
    #[must_use]
    pub fn contains_symbol(&self, symbol: &BoundarySymbol) -> bool {
        self.ids_by_symbol.contains_key(symbol)
    }

    /// Return symbols in id order.
    #[must_use]
    pub fn symbols(&self) -> &[BoundarySymbol] {
        &self.symbols_by_id
    }

    /// Iterate through `(SymbolId, BoundarySymbol)` pairs in id order.
    pub fn iter(&self) -> impl Iterator<Item = (SymbolId, &BoundarySymbol)> {
        self.symbols_by_id
            .iter()
            .enumerate()
            .map(|(index, symbol)| {
                let id = symbol_id_from_registry_index(index);
                (id, symbol)
            })
    }

    /// Return surface entries in registry id order.
    ///
    /// # Errors
    ///
    /// Returns [`SymbolRegistryError::UnknownSymbol`] when the surface contains
    /// a key that was not pre-registered.
    pub fn export_surface_in_id_order(
        &self,
        surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Result<Vec<(SymbolId, BoundarySymbol, BoundaryValue)>, SymbolRegistryError> {
        let mut exported = Vec::with_capacity(surface.len());
        for (symbol, value) in surface {
            exported.push((self.id_of(symbol)?, symbol.clone(), *value));
        }
        exported.sort_by_key(|(id, _, _)| *id);
        Ok(exported)
    }

    /// Return surface keys missing from this registry.
    #[must_use]
    pub fn surface_unknown_symbols(
        &self,
        surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Vec<BoundarySymbol> {
        surface
            .keys()
            .filter(|symbol| !self.contains_symbol(symbol))
            .cloned()
            .collect()
    }
}

/// Working-set-sized sparse indexed boundary surface.
///
/// Entries are stored in sorted [`SymbolId`] order. The surface therefore keeps
/// the current sorted-symbol export order without requiring a dense global-id
/// vector.
#[derive(Debug, Clone, PartialEq)]
pub struct IndexedSurface {
    entries: Vec<(SymbolId, BoundaryValue)>,
}

/// Logical symbol-keyed boundary surface map.
pub type BoundarySurfaceMap = BTreeMap<BoundarySymbol, BoundaryValue>;

/// Logical state and flux surface pair.
pub type BoundarySurfacePair = (BoundarySurfaceMap, BoundarySurfaceMap);

impl IndexedSurface {
    /// Build an indexed surface from a logical `BTreeMap` surface.
    ///
    /// # Errors
    ///
    /// Returns [`SymbolRegistryError::UnknownSymbol`] when any surface key is not
    /// present in the frozen registry.
    pub fn from_btreemap(
        registry: &SymbolRegistry,
        surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Result<Self, SymbolRegistryError> {
        let mut entries = Vec::with_capacity(surface.len());
        let mut registry_entries = registry.iter();
        for (symbol, value) in surface {
            loop {
                let Some((id, registry_symbol)) = registry_entries.next() else {
                    return Err(SymbolRegistryError::UnknownSymbol {
                        symbol: symbol.clone(),
                    });
                };

                match registry_symbol.cmp(symbol) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        entries.push((id, *value));
                        break;
                    }
                    Ordering::Greater => {
                        return Err(SymbolRegistryError::UnknownSymbol {
                            symbol: symbol.clone(),
                        });
                    }
                }
            }
        }
        Ok(Self { entries })
    }

    /// Return the number of present entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return whether this indexed surface has no present entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Return entries in sorted id order.
    #[must_use]
    pub fn entries(&self) -> &[(SymbolId, BoundaryValue)] {
        &self.entries
    }

    /// Lookup a present value by id.
    #[must_use]
    pub fn get(&self, id: SymbolId) -> Option<BoundaryValue> {
        self.entries
            .binary_search_by_key(&id, |(entry_id, _)| *entry_id)
            .ok()
            .map(|index| self.entries[index].1)
    }

    /// Export this indexed surface to the logical `BTreeMap` representation.
    ///
    /// # Errors
    ///
    /// Returns [`SymbolRegistryError::UnknownSymbolId`] if an entry id is not
    /// present in the registry used for export.
    pub fn export_btreemap(
        &self,
        registry: &SymbolRegistry,
    ) -> Result<BTreeMap<BoundarySymbol, BoundaryValue>, SymbolRegistryError> {
        let mut exported = BTreeMap::new();
        for (id, value) in &self.entries {
            let Some(symbol) = registry.symbol(*id) else {
                return Err(SymbolRegistryError::UnknownSymbolId { id: *id });
            };
            exported.insert(symbol.clone(), *value);
        }
        Ok(exported)
    }
}

/// Non-authoritative indexed state/flux shadow surface.
#[derive(Debug, Clone, PartialEq)]
pub struct IndexedWritebackSurface {
    state_surface: IndexedSurface,
    flux_surface: IndexedSurface,
}

impl IndexedWritebackSurface {
    /// Build an indexed shadow from the current logical state and flux maps.
    ///
    /// # Errors
    ///
    /// Returns [`SymbolRegistryError::UnknownSymbol`] when any state or flux key
    /// is not present in the frozen registry.
    pub fn from_btreemap_surfaces(
        registry: &SymbolRegistry,
        state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Result<Self, SymbolRegistryError> {
        Ok(Self {
            state_surface: IndexedSurface::from_btreemap(registry, state_surface)?,
            flux_surface: IndexedSurface::from_btreemap(registry, flux_surface)?,
        })
    }

    /// Return the indexed state surface.
    #[must_use]
    pub const fn state_surface(&self) -> &IndexedSurface {
        &self.state_surface
    }

    /// Return the indexed flux surface.
    #[must_use]
    pub const fn flux_surface(&self) -> &IndexedSurface {
        &self.flux_surface
    }

    /// Export the indexed state and flux surfaces back to logical `BTreeMap`s.
    ///
    /// # Errors
    ///
    /// Returns [`SymbolRegistryError::UnknownSymbolId`] if an entry id is not
    /// present in the registry used for export.
    pub fn export_btreemap_surfaces(
        &self,
        registry: &SymbolRegistry,
    ) -> Result<BoundarySurfacePair, SymbolRegistryError> {
        Ok((
            self.state_surface.export_btreemap(registry)?,
            self.flux_surface.export_btreemap(registry)?,
        ))
    }
}

/// Registry construction and lookup errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolRegistryError {
    TooManySymbols { count: usize, supported_max: u32 },
    UnknownSymbol { symbol: BoundarySymbol },
    UnknownSymbolId { id: SymbolId },
}

impl fmt::Display for SymbolRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManySymbols {
                count,
                supported_max,
            } => write!(
                f,
                "symbol registry has {count} symbols, exceeding supported maximum {supported_max}"
            ),
            Self::UnknownSymbol { symbol } => {
                write!(f, "symbol {symbol} is not present in the frozen registry")
            }
            Self::UnknownSymbolId { id } => {
                write!(f, "symbol id {id} is not present in the frozen registry")
            }
        }
    }
}

impl std::error::Error for SymbolRegistryError {}

/// Runtime audit report for a frozen symbol registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolRegistryAuditReport {
    registry_symbol_count: usize,
    constructed_symbol_count: usize,
    unknown_symbols: Vec<BoundarySymbol>,
}

impl SymbolRegistryAuditReport {
    #[must_use]
    pub const fn registry_symbol_count(&self) -> usize {
        self.registry_symbol_count
    }

    #[must_use]
    pub const fn constructed_symbol_count(&self) -> usize {
        self.constructed_symbol_count
    }

    #[must_use]
    pub fn unknown_symbols(&self) -> &[BoundarySymbol] {
        &self.unknown_symbols
    }

    #[must_use]
    pub fn unknown_symbol_count(&self) -> usize {
        self.unknown_symbols.len()
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.unknown_symbols.is_empty()
    }
}

/// Audit lifecycle errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolRegistryAuditError {
    AlreadyActive,
}

impl fmt::Display for SymbolRegistryAuditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyActive => write!(f, "symbol registry audit is already active"),
        }
    }
}

impl std::error::Error for SymbolRegistryAuditError {}

#[derive(Debug, Clone)]
struct SymbolRegistryAuditState {
    registry: SymbolRegistry,
    constructed_symbols: BTreeSet<BoundarySymbol>,
    unknown_symbols: BTreeSet<BoundarySymbol>,
}

thread_local! {
    static SYMBOL_REGISTRY_AUDIT: RefCell<Option<SymbolRegistryAuditState>> =
        const { RefCell::new(None) };
}

/// Begin a thread-local frozen-registry audit.
///
/// # Errors
///
/// Returns [`SymbolRegistryAuditError::AlreadyActive`] when an audit is already
/// active on this thread.
pub fn begin_symbol_registry_audit(
    registry: SymbolRegistry,
) -> Result<(), SymbolRegistryAuditError> {
    SYMBOL_REGISTRY_AUDIT.with(|cell| {
        let mut state = cell.borrow_mut();
        if state.is_some() {
            return Err(SymbolRegistryAuditError::AlreadyActive);
        }
        *state = Some(SymbolRegistryAuditState {
            registry,
            constructed_symbols: BTreeSet::new(),
            unknown_symbols: BTreeSet::new(),
        });
        Ok(())
    })
}

/// Finish a thread-local frozen-registry audit and return its report.
#[must_use]
pub fn finish_symbol_registry_audit() -> Option<SymbolRegistryAuditReport> {
    SYMBOL_REGISTRY_AUDIT.with(|cell| {
        let state = cell.borrow_mut().take()?;
        Some(SymbolRegistryAuditReport {
            registry_symbol_count: state.registry.len(),
            constructed_symbol_count: state.constructed_symbols.len(),
            unknown_symbols: state.unknown_symbols.into_iter().collect(),
        })
    })
}

fn record_constructed_boundary_symbol(symbol: &BoundarySymbol) {
    SYMBOL_REGISTRY_AUDIT.with(|cell| {
        let mut state = cell.borrow_mut();
        let Some(state) = state.as_mut() else {
            return;
        };
        state.constructed_symbols.insert(symbol.clone());
        if !state.registry.contains_symbol(symbol) {
            state.unknown_symbols.insert(symbol.clone());
        }
    });
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
