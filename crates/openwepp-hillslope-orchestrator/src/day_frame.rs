use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, DenseBoundarySlotView, HotSymbolTables,
    IndexedKernelWritebackPayload, IndexedWritebackSurface, KernelWritebackPayload, SymbolId,
    SymbolRegistry, SymbolRegistryError,
};

use crate::constants::{
    MOFE_HOURLY_CARRY_ARRAY_COUNT, MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT,
    MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT, MOFE_HOURLY_UPSTREAM_LATERAL_RUNOFF_ROOT,
    MOFE_HOURLY_UPSTREAM_SATURATION_RUNOFF_ROOT, PL_RUNTIME_DAY_SYMBOL, PL_RUNTIME_YEAR_SYMBOL,
};
use crate::scheduler::HillslopeWritebackSurface;

const HBP_PEAKRO_SYMBOL: &str = "peakro";
const HBP_WATDUR_SYMBOL: &str = "watdur";
const HBP_TOTAL_DETACHMENT_SYMBOL: &str = "total_detachment_kg";
const HBP_TOTAL_DEPOSITION_SYMBOL: &str = "total_deposition_kg";
const HBP_SEDIMENT_CONCENTRATION_SYMBOL: &str = "sediment_concentration_kg_m3_0001";

type HillslopeSurfacePair = (
    BTreeMap<BoundarySymbol, BoundaryValue>,
    BTreeMap<BoundarySymbol, BoundaryValue>,
);

#[derive(Debug, Clone, PartialEq)]
pub struct HillslopeDayFrame<'a> {
    symbol_registry: SymbolRegistry,
    state_slots: Vec<Option<BoundaryValue>>,
    flux_slots: Vec<Option<BoundaryValue>>,
    pub mofe_hourly_upstream_saturation_runoff:
        [Option<BoundaryValue>; MOFE_HOURLY_CARRY_ARRAY_COUNT],
    pub mofe_hourly_current_saturation_runoff:
        [Option<BoundaryValue>; MOFE_HOURLY_CARRY_ARRAY_COUNT],
    pub mofe_hourly_upstream_lateral_runoff: [Option<BoundaryValue>; MOFE_HOURLY_CARRY_ARRAY_COUNT],
    pub mofe_hourly_current_lateral_runoff: [Option<BoundaryValue>; MOFE_HOURLY_CARRY_ARRAY_COUNT],
    io_edge_scalars: HillslopeDayFrameIoEdgeScalars,
    climate_forcing_series: Option<&'a [f64]>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HillslopeDayFrameIoEdgeScalars {
    pub peakro: Option<BoundaryValue>,
    pub watdur: Option<BoundaryValue>,
    pub total_detachment_kg: Option<BoundaryValue>,
    pub total_deposition_kg: Option<BoundaryValue>,
    pub sediment_concentration_kg_m3_0001: Option<BoundaryValue>,
    pub runtime_day: Option<BoundaryValue>,
    pub runtime_year: Option<BoundaryValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HillslopeDayFrameMismatch {
    pub surface: &'static str,
    pub symbol: BoundarySymbol,
    pub expected_bits: u64,
    pub observed_bits: u64,
    pub expected_unit: &'static str,
    pub observed_unit: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HillslopeDayFrameShadowReport {
    pub state_symbol_count: usize,
    pub flux_symbol_count: usize,
    pub state_mismatch_count: usize,
    pub flux_mismatch_count: usize,
    pub first_mismatch: Option<HillslopeDayFrameMismatch>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HillslopeDayFrameDirtyIds {
    pub state_ids: Vec<SymbolId>,
    pub flux_ids: Vec<SymbolId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HillslopeLaneDenseState {
    state_id_to_slot: Vec<Option<usize>>,
    flux_id_to_slot: Vec<Option<usize>>,
    state_slot_ids: Vec<SymbolId>,
    flux_slot_ids: Vec<SymbolId>,
    state_slots: Vec<Option<BoundaryValue>>,
    flux_slots: Vec<Option<BoundaryValue>>,
    dirty_state_ids: BTreeSet<SymbolId>,
    dirty_flux_ids: BTreeSet<SymbolId>,
}

impl HillslopeDayFrameShadowReport {
    #[must_use]
    pub fn is_bit_identical(&self) -> bool {
        self.state_mismatch_count == 0 && self.flux_mismatch_count == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HillslopeDayFrameError {
    SymbolRegistry(SymbolRegistryError),
    MissingRoundtripSymbol {
        surface: &'static str,
        symbol: BoundarySymbol,
    },
    SymbolCardinalityMismatch {
        surface: &'static str,
        expected_count: usize,
        observed_count: usize,
    },
    RoundtripMismatch(HillslopeDayFrameMismatch),
}

impl fmt::Display for HillslopeDayFrameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SymbolRegistry(source) => {
                write!(formatter, "frame symbol-registry error: {source}")
            }
            Self::MissingRoundtripSymbol { surface, symbol } => {
                write!(
                    formatter,
                    "frame roundtrip {surface} symbol {symbol} missing after flush"
                )
            }
            Self::SymbolCardinalityMismatch {
                surface,
                expected_count,
                observed_count,
            } => write!(
                formatter,
                "frame roundtrip {surface} symbol count mismatch: expected {expected_count}, observed {observed_count}"
            ),
            Self::RoundtripMismatch(mismatch) => write!(
                formatter,
                "frame roundtrip mismatch on {} symbol {}: expected bits {} ({}), observed bits {} ({})",
                mismatch.surface,
                mismatch.symbol,
                mismatch.expected_bits,
                mismatch.expected_unit,
                mismatch.observed_bits,
                mismatch.observed_unit
            ),
        }
    }
}

impl Error for HillslopeDayFrameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SymbolRegistry(source) => Some(source),
            _ => None,
        }
    }
}

impl From<SymbolRegistryError> for HillslopeDayFrameError {
    fn from(value: SymbolRegistryError) -> Self {
        Self::SymbolRegistry(value)
    }
}

impl HillslopeLaneDenseState {
    pub fn seed_hot_from_writeback_surface(
        writeback_surface: &HillslopeWritebackSurface,
        indexed_writeback_surface: Option<&IndexedWritebackSurface>,
        symbol_registry: &SymbolRegistry,
        hot_symbol_tables: &HotSymbolTables,
    ) -> Result<Self, SymbolRegistryError> {
        let mut state = Self {
            state_id_to_slot: vec![None; symbol_registry.len()],
            flux_id_to_slot: vec![None; symbol_registry.len()],
            state_slot_ids: Vec::new(),
            flux_slot_ids: Vec::new(),
            state_slots: Vec::new(),
            flux_slots: Vec::new(),
            dirty_state_ids: BTreeSet::new(),
            dirty_flux_ids: BTreeSet::new(),
        };

        for symbol in hot_symbol_tables.hot_state_symbols() {
            let value = indexed_writeback_surface
                .and_then(|surface| surface.state_value(symbol.id))
                .or_else(|| writeback_surface.state_surface.get(&symbol.symbol).copied());
            state.ensure_state_slot(symbol.id, value)?;
        }
        for symbol in hot_symbol_tables.hot_flux_symbols() {
            let value = indexed_writeback_surface
                .and_then(|surface| surface.flux_value(symbol.id))
                .or_else(|| writeback_surface.flux_surface.get(&symbol.symbol).copied());
            state.ensure_flux_slot(symbol.id, value)?;
        }

        Ok(state)
    }

    pub fn sync_from_writeback_surface(
        &mut self,
        writeback_surface: &HillslopeWritebackSurface,
        indexed_writeback_surface: Option<&IndexedWritebackSurface>,
        symbol_registry: &SymbolRegistry,
        hot_symbol_tables: &HotSymbolTables,
    ) -> Result<(), SymbolRegistryError> {
        for symbol in hot_symbol_tables.hot_state_symbols() {
            let value = indexed_writeback_surface
                .and_then(|surface| surface.state_value(symbol.id))
                .or_else(|| writeback_surface.state_surface.get(&symbol.symbol).copied());
            self.ensure_state_slot(symbol.id, value)?;
        }
        for symbol in hot_symbol_tables.hot_flux_symbols() {
            let value = indexed_writeback_surface
                .and_then(|surface| surface.flux_value(symbol.id))
                .or_else(|| writeback_surface.flux_surface.get(&symbol.symbol).copied());
            self.ensure_flux_slot(symbol.id, value)?;
        }

        let state_ids = self.state_slot_ids.clone();
        for id in state_ids {
            let Some(symbol) = symbol_registry.symbol(id) else {
                return Err(SymbolRegistryError::UnknownSymbolId { id });
            };
            let value = indexed_writeback_surface
                .and_then(|surface| surface.state_value(id))
                .or_else(|| writeback_surface.state_surface.get(symbol).copied());
            self.set_state_slot(id, value)?;
        }

        let flux_ids = self.flux_slot_ids.clone();
        for id in flux_ids {
            let Some(symbol) = symbol_registry.symbol(id) else {
                return Err(SymbolRegistryError::UnknownSymbolId { id });
            };
            let value = indexed_writeback_surface
                .and_then(|surface| surface.flux_value(id))
                .or_else(|| writeback_surface.flux_surface.get(symbol).copied());
            self.set_flux_slot(id, value)?;
        }

        self.dirty_state_ids.clear();
        self.dirty_flux_ids.clear();
        Ok(())
    }

    #[must_use]
    pub fn state_slot_view(&self) -> DenseBoundarySlotView<'_> {
        DenseBoundarySlotView::new(&self.state_id_to_slot, &self.state_slots)
    }

    #[must_use]
    pub fn flux_slot_view(&self) -> DenseBoundarySlotView<'_> {
        DenseBoundarySlotView::new(&self.flux_id_to_slot, &self.flux_slots)
    }

    #[must_use]
    pub fn state_slot_count(&self) -> usize {
        self.state_slots.len()
    }

    #[must_use]
    pub fn flux_slot_count(&self) -> usize {
        self.flux_slots.len()
    }

    pub fn apply_kernel_writeback_payload(
        &mut self,
        payload: &KernelWritebackPayload,
        symbol_registry: &SymbolRegistry,
    ) -> Result<HillslopeDayFrameDirtyIds, SymbolRegistryError> {
        let mut dirty = HillslopeDayFrameDirtyIds::default();
        for field in &payload.state_updates {
            let id = symbol_registry.id_of(&field.symbol)?;
            self.set_state_slot(id, Some(field.value))?;
            self.dirty_state_ids.insert(id);
            dirty.state_ids.push(id);
        }
        for field in &payload.flux_updates {
            let id = symbol_registry.id_of(&field.symbol)?;
            self.set_flux_slot(id, Some(field.value))?;
            self.dirty_flux_ids.insert(id);
            dirty.flux_ids.push(id);
        }
        Ok(dirty)
    }

    pub fn apply_indexed_kernel_writeback_payload(
        &mut self,
        payload: &IndexedKernelWritebackPayload,
    ) -> Result<HillslopeDayFrameDirtyIds, SymbolRegistryError> {
        let mut dirty = HillslopeDayFrameDirtyIds::default();
        for field in &payload.state_updates {
            self.set_state_slot(field.id, Some(field.value))?;
            self.dirty_state_ids.insert(field.id);
            dirty.state_ids.push(field.id);
        }
        for field in &payload.flux_updates {
            self.set_flux_slot(field.id, Some(field.value))?;
            self.dirty_flux_ids.insert(field.id);
            dirty.flux_ids.push(field.id);
        }
        Ok(dirty)
    }

    pub fn flush_dirty_to_writeback_surface(
        &mut self,
        symbol_registry: &SymbolRegistry,
        writeback_surface: &mut HillslopeWritebackSurface,
        indexed_writeback_surface: Option<&mut IndexedWritebackSurface>,
    ) -> Result<(), SymbolRegistryError> {
        let mut indexed_writeback_surface = indexed_writeback_surface;

        for id in &self.dirty_state_ids {
            let Some(symbol) = symbol_registry.symbol(*id) else {
                return Err(SymbolRegistryError::UnknownSymbolId { id: *id });
            };
            let value = self.state_value(*id);
            if let Some(value) = value {
                writeback_surface
                    .state_surface
                    .insert(symbol.clone(), value);
            } else {
                writeback_surface.state_surface.remove(symbol);
            }
            if let Some(indexed_surface) = indexed_writeback_surface.as_deref_mut() {
                indexed_surface.set_state_value(*id, value);
            }
        }

        for id in &self.dirty_flux_ids {
            let Some(symbol) = symbol_registry.symbol(*id) else {
                return Err(SymbolRegistryError::UnknownSymbolId { id: *id });
            };
            let value = self.flux_value(*id);
            if let Some(value) = value {
                writeback_surface.flux_surface.insert(symbol.clone(), value);
            } else {
                writeback_surface.flux_surface.remove(symbol);
            }
            if let Some(indexed_surface) = indexed_writeback_surface.as_deref_mut() {
                indexed_surface.set_flux_value(*id, value);
            }
        }

        self.dirty_state_ids.clear();
        self.dirty_flux_ids.clear();
        Ok(())
    }

    fn ensure_state_slot(
        &mut self,
        id: SymbolId,
        value: Option<BoundaryValue>,
    ) -> Result<(), SymbolRegistryError> {
        if id.as_usize() >= self.state_id_to_slot.len() {
            return Err(SymbolRegistryError::UnknownSymbolId { id });
        }
        if self.state_id_to_slot[id.as_usize()].is_none() {
            let slot = self.state_slots.len();
            self.state_id_to_slot[id.as_usize()] = Some(slot);
            self.state_slot_ids.push(id);
            self.state_slots.push(None);
        }
        self.set_state_slot(id, value)
    }

    fn ensure_flux_slot(
        &mut self,
        id: SymbolId,
        value: Option<BoundaryValue>,
    ) -> Result<(), SymbolRegistryError> {
        if id.as_usize() >= self.flux_id_to_slot.len() {
            return Err(SymbolRegistryError::UnknownSymbolId { id });
        }
        if self.flux_id_to_slot[id.as_usize()].is_none() {
            let slot = self.flux_slots.len();
            self.flux_id_to_slot[id.as_usize()] = Some(slot);
            self.flux_slot_ids.push(id);
            self.flux_slots.push(None);
        }
        self.set_flux_slot(id, value)
    }

    fn set_state_slot(
        &mut self,
        id: SymbolId,
        value: Option<BoundaryValue>,
    ) -> Result<(), SymbolRegistryError> {
        let Some(slot) = self.state_id_to_slot.get(id.as_usize()).copied().flatten() else {
            return self.ensure_state_slot(id, value);
        };
        self.state_slots[slot] = value;
        Ok(())
    }

    fn set_flux_slot(
        &mut self,
        id: SymbolId,
        value: Option<BoundaryValue>,
    ) -> Result<(), SymbolRegistryError> {
        let Some(slot) = self.flux_id_to_slot.get(id.as_usize()).copied().flatten() else {
            return self.ensure_flux_slot(id, value);
        };
        self.flux_slots[slot] = value;
        Ok(())
    }

    fn state_value(&self, id: SymbolId) -> Option<BoundaryValue> {
        self.state_id_to_slot
            .get(id.as_usize())
            .copied()
            .flatten()
            .and_then(|slot| self.state_slots.get(slot))
            .copied()
            .flatten()
    }

    fn flux_value(&self, id: SymbolId) -> Option<BoundaryValue> {
        self.flux_id_to_slot
            .get(id.as_usize())
            .copied()
            .flatten()
            .and_then(|slot| self.flux_slots.get(slot))
            .copied()
            .flatten()
    }
}

impl<'a> HillslopeDayFrame<'a> {
    pub fn seed_from_writeback_surface(
        writeback_surface: &HillslopeWritebackSurface,
        symbol_registry: &SymbolRegistry,
        climate_forcing_series: Option<&'a [f64]>,
    ) -> Result<Self, HillslopeDayFrameError> {
        Self::seed_from_surfaces(
            &writeback_surface.state_surface,
            &writeback_surface.flux_surface,
            symbol_registry,
            climate_forcing_series,
        )
    }

    pub fn seed_from_indexed_writeback_surface(
        indexed_writeback_surface: &IndexedWritebackSurface,
        symbol_registry: &SymbolRegistry,
        climate_forcing_series: Option<&'a [f64]>,
    ) -> Result<Self, HillslopeDayFrameError> {
        let mut state_slots = vec![None; symbol_registry.len()];
        let mut flux_slots = vec![None; symbol_registry.len()];

        for (id, value) in indexed_writeback_surface.state_surface().entries() {
            let Some(slot) = state_slots.get_mut(id.as_usize()) else {
                return Err(SymbolRegistryError::UnknownSymbolId { id: *id }.into());
            };
            *slot = Some(*value);
        }

        for (id, value) in indexed_writeback_surface.flux_surface().entries() {
            let Some(slot) = flux_slots.get_mut(id.as_usize()) else {
                return Err(SymbolRegistryError::UnknownSymbolId { id: *id }.into());
            };
            *slot = Some(*value);
        }

        Ok(Self::from_seeded_slots(
            state_slots,
            flux_slots,
            symbol_registry,
            climate_forcing_series,
        ))
    }

    pub fn seed_from_surfaces(
        state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
        flux_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
        symbol_registry: &SymbolRegistry,
        climate_forcing_series: Option<&'a [f64]>,
    ) -> Result<Self, HillslopeDayFrameError> {
        let mut state_slots = vec![None; symbol_registry.len()];
        let mut flux_slots = vec![None; symbol_registry.len()];

        for (symbol, value) in state_surface {
            let id = symbol_registry.id_of(symbol)?;
            state_slots[id.as_usize()] = Some(*value);
        }

        for (symbol, value) in flux_surface {
            let id = symbol_registry.id_of(symbol)?;
            flux_slots[id.as_usize()] = Some(*value);
        }

        Ok(Self::from_seeded_slots(
            state_slots,
            flux_slots,
            symbol_registry,
            climate_forcing_series,
        ))
    }

    fn from_seeded_slots(
        state_slots: Vec<Option<BoundaryValue>>,
        flux_slots: Vec<Option<BoundaryValue>>,
        symbol_registry: &SymbolRegistry,
        climate_forcing_series: Option<&'a [f64]>,
    ) -> Self {
        let mut frame = Self {
            symbol_registry: symbol_registry.clone(),
            state_slots,
            flux_slots,
            mofe_hourly_upstream_saturation_runoff: [None; MOFE_HOURLY_CARRY_ARRAY_COUNT],
            mofe_hourly_current_saturation_runoff: [None; MOFE_HOURLY_CARRY_ARRAY_COUNT],
            mofe_hourly_upstream_lateral_runoff: [None; MOFE_HOURLY_CARRY_ARRAY_COUNT],
            mofe_hourly_current_lateral_runoff: [None; MOFE_HOURLY_CARRY_ARRAY_COUNT],
            io_edge_scalars: HillslopeDayFrameIoEdgeScalars::default(),
            climate_forcing_series,
        };

        frame.mofe_hourly_upstream_saturation_runoff =
            frame.capture_hourly_state_family(MOFE_HOURLY_UPSTREAM_SATURATION_RUNOFF_ROOT);
        frame.mofe_hourly_current_saturation_runoff =
            frame.capture_hourly_state_family(MOFE_HOURLY_CURRENT_SATURATION_RUNOFF_ROOT);
        frame.mofe_hourly_upstream_lateral_runoff =
            frame.capture_hourly_state_family(MOFE_HOURLY_UPSTREAM_LATERAL_RUNOFF_ROOT);
        frame.mofe_hourly_current_lateral_runoff =
            frame.capture_hourly_state_family(MOFE_HOURLY_CURRENT_LATERAL_RUNOFF_ROOT);

        frame.io_edge_scalars = HillslopeDayFrameIoEdgeScalars {
            peakro: frame.capture_symbol_from_any_surface(HBP_PEAKRO_SYMBOL),
            watdur: frame.capture_symbol_from_any_surface(HBP_WATDUR_SYMBOL),
            total_detachment_kg: frame.capture_symbol_from_any_surface(HBP_TOTAL_DETACHMENT_SYMBOL),
            total_deposition_kg: frame.capture_symbol_from_any_surface(HBP_TOTAL_DEPOSITION_SYMBOL),
            sediment_concentration_kg_m3_0001: frame
                .capture_symbol_from_any_surface(HBP_SEDIMENT_CONCENTRATION_SYMBOL),
            runtime_day: frame.capture_symbol_from_any_surface(PL_RUNTIME_DAY_SYMBOL),
            runtime_year: frame.capture_symbol_from_any_surface(PL_RUNTIME_YEAR_SYMBOL),
        };

        frame
    }

    #[must_use]
    pub fn symbol_registry(&self) -> &SymbolRegistry {
        &self.symbol_registry
    }

    #[must_use]
    pub fn state_slots(&self) -> &[Option<BoundaryValue>] {
        &self.state_slots
    }

    #[must_use]
    pub fn flux_slots(&self) -> &[Option<BoundaryValue>] {
        &self.flux_slots
    }

    pub fn set_state_slot(
        &mut self,
        id: SymbolId,
        value: Option<BoundaryValue>,
    ) -> Result<(), SymbolRegistryError> {
        let Some(slot) = self.state_slots.get_mut(id.as_usize()) else {
            return Err(SymbolRegistryError::UnknownSymbolId { id });
        };
        *slot = value;
        Ok(())
    }

    pub fn set_flux_slot(
        &mut self,
        id: SymbolId,
        value: Option<BoundaryValue>,
    ) -> Result<(), SymbolRegistryError> {
        let Some(slot) = self.flux_slots.get_mut(id.as_usize()) else {
            return Err(SymbolRegistryError::UnknownSymbolId { id });
        };
        *slot = value;
        Ok(())
    }

    pub fn apply_kernel_writeback_payload(
        &mut self,
        payload: &KernelWritebackPayload,
    ) -> Result<HillslopeDayFrameDirtyIds, SymbolRegistryError> {
        let mut dirty = HillslopeDayFrameDirtyIds::default();
        for field in &payload.state_updates {
            let id = self.symbol_registry.id_of(&field.symbol)?;
            self.set_state_slot(id, Some(field.value))?;
            dirty.state_ids.push(id);
        }
        for field in &payload.flux_updates {
            let id = self.symbol_registry.id_of(&field.symbol)?;
            self.set_flux_slot(id, Some(field.value))?;
            dirty.flux_ids.push(id);
        }
        Ok(dirty)
    }

    pub fn apply_indexed_kernel_writeback_payload(
        &mut self,
        payload: &IndexedKernelWritebackPayload,
    ) -> Result<HillslopeDayFrameDirtyIds, SymbolRegistryError> {
        let mut dirty = HillslopeDayFrameDirtyIds::default();
        for field in &payload.state_updates {
            self.set_state_slot(field.id, Some(field.value))?;
            dirty.state_ids.push(field.id);
        }
        for field in &payload.flux_updates {
            self.set_flux_slot(field.id, Some(field.value))?;
            dirty.flux_ids.push(field.id);
        }
        Ok(dirty)
    }

    pub fn flush_dirty_ids_to_writeback_surface(
        &self,
        state_ids: &[SymbolId],
        flux_ids: &[SymbolId],
        writeback_surface: &mut HillslopeWritebackSurface,
    ) -> Result<(), SymbolRegistryError> {
        for id in state_ids {
            let symbol = self
                .symbol_registry
                .symbol(*id)
                .ok_or(SymbolRegistryError::UnknownSymbolId { id: *id })?;
            if let Some(value) = self.state_slots[id.as_usize()] {
                writeback_surface
                    .state_surface
                    .insert(symbol.clone(), value);
            } else {
                writeback_surface.state_surface.remove(symbol);
            }
        }
        for id in flux_ids {
            let symbol = self
                .symbol_registry
                .symbol(*id)
                .ok_or(SymbolRegistryError::UnknownSymbolId { id: *id })?;
            if let Some(value) = self.flux_slots[id.as_usize()] {
                writeback_surface.flux_surface.insert(symbol.clone(), value);
            } else {
                writeback_surface.flux_surface.remove(symbol);
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn io_edge_scalars(&self) -> &HillslopeDayFrameIoEdgeScalars {
        &self.io_edge_scalars
    }

    #[must_use]
    pub fn climate_forcing_series(&self) -> Option<&[f64]> {
        self.climate_forcing_series
    }

    pub fn flush_to_writeback_surface(
        &self,
    ) -> Result<HillslopeWritebackSurface, HillslopeDayFrameError> {
        let (state_surface, flux_surface) = self.flush_to_surfaces()?;
        Ok(HillslopeWritebackSurface {
            state_surface,
            flux_surface,
        })
    }

    pub fn flush_to_surfaces(&self) -> Result<HillslopeSurfacePair, HillslopeDayFrameError> {
        let mut state_surface = BTreeMap::new();
        let mut flux_surface = BTreeMap::new();

        for (id, symbol) in self.symbol_registry.iter() {
            if let Some(value) = self.state_slots[id.as_usize()] {
                state_surface.insert(symbol.clone(), value);
            }
            if let Some(value) = self.flux_slots[id.as_usize()] {
                flux_surface.insert(symbol.clone(), value);
            }
        }

        Ok((state_surface, flux_surface))
    }

    pub fn shadow_roundtrip_report(
        &self,
        original_state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
        original_flux_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Result<HillslopeDayFrameShadowReport, HillslopeDayFrameError> {
        let (roundtrip_state_surface, roundtrip_flux_surface) = self.flush_to_surfaces()?;

        let (state_mismatch_count, state_first_mismatch) =
            compare_surface_bits("state", original_state_surface, &roundtrip_state_surface)?;
        let (flux_mismatch_count, flux_first_mismatch) =
            compare_surface_bits("flux", original_flux_surface, &roundtrip_flux_surface)?;

        Ok(HillslopeDayFrameShadowReport {
            state_symbol_count: original_state_surface.len(),
            flux_symbol_count: original_flux_surface.len(),
            state_mismatch_count,
            flux_mismatch_count,
            first_mismatch: state_first_mismatch.or(flux_first_mismatch),
        })
    }

    pub fn assert_shadow_roundtrip_bits(
        &self,
        original_state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
        original_flux_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Result<HillslopeDayFrameShadowReport, HillslopeDayFrameError> {
        let report = self.shadow_roundtrip_report(original_state_surface, original_flux_surface)?;
        if report.is_bit_identical() {
            return Ok(report);
        }

        if let Some(first_mismatch) = report.first_mismatch.clone() {
            return Err(HillslopeDayFrameError::RoundtripMismatch(first_mismatch));
        }

        Err(HillslopeDayFrameError::SymbolCardinalityMismatch {
            surface: "state+flux",
            expected_count: report.state_symbol_count + report.flux_symbol_count,
            observed_count: report.state_symbol_count + report.flux_symbol_count
                - report.state_mismatch_count
                - report.flux_mismatch_count,
        })
    }

    fn capture_hourly_state_family(
        &self,
        symbol_root: &str,
    ) -> [Option<BoundaryValue>; MOFE_HOURLY_CARRY_ARRAY_COUNT] {
        let mut series = [None; MOFE_HOURLY_CARRY_ARRAY_COUNT];
        for index in 1..=MOFE_HOURLY_CARRY_ARRAY_COUNT {
            let symbol = BoundarySymbol::from(format!("{symbol_root}_{index:04}"));
            if let Ok(id) = self.symbol_registry.id_of(&symbol) {
                series[index - 1] = self.state_slots[id.as_usize()];
            }
        }
        series
    }

    fn capture_symbol_from_any_surface(&self, symbol_name: &str) -> Option<BoundaryValue> {
        let symbol = BoundarySymbol::from(symbol_name);
        let id = self.symbol_registry.id_of(&symbol).ok()?;
        self.state_slots[id.as_usize()].or(self.flux_slots[id.as_usize()])
    }
}

fn compare_surface_bits(
    surface_name: &'static str,
    expected_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    observed_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<(usize, Option<HillslopeDayFrameMismatch>), HillslopeDayFrameError> {
    if expected_surface.len() != observed_surface.len() {
        return Err(HillslopeDayFrameError::SymbolCardinalityMismatch {
            surface: surface_name,
            expected_count: expected_surface.len(),
            observed_count: observed_surface.len(),
        });
    }

    let mut mismatch_count = 0;
    let mut first_mismatch = None;

    for (symbol, expected_value) in expected_surface {
        let Some(observed_value) = observed_surface.get(symbol).copied() else {
            return Err(HillslopeDayFrameError::MissingRoundtripSymbol {
                surface: surface_name,
                symbol: symbol.clone(),
            });
        };

        let expected_bits = expected_value.as_f64().to_bits();
        let observed_bits = observed_value.as_f64().to_bits();
        let expected_unit = expected_value.unit_label();
        let observed_unit = observed_value.unit_label();

        if expected_bits != observed_bits || expected_unit != observed_unit {
            mismatch_count += 1;
            if first_mismatch.is_none() {
                first_mismatch = Some(HillslopeDayFrameMismatch {
                    surface: surface_name,
                    symbol: symbol.clone(),
                    expected_bits,
                    observed_bits,
                    expected_unit,
                    observed_unit,
                });
            }
        }
    }

    Ok((mismatch_count, first_mismatch))
}
