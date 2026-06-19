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

    /// Insert, update, or remove a present value by id while preserving id order.
    pub fn set(&mut self, id: SymbolId, value: Option<BoundaryValue>) {
        match self
            .entries
            .binary_search_by_key(&id, |(entry_id, _)| *entry_id)
        {
            Ok(index) => {
                if let Some(value) = value {
                    self.entries[index].1 = value;
                } else {
                    self.entries.remove(index);
                }
            }
            Err(index) => {
                if let Some(value) = value {
                    self.entries.insert(index, (id, value));
                }
            }
        }
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

    /// Lookup an indexed state value by id.
    #[must_use]
    pub fn state_value(&self, id: SymbolId) -> Option<BoundaryValue> {
        self.state_surface.get(id)
    }

    /// Lookup an indexed flux value by id.
    #[must_use]
    pub fn flux_value(&self, id: SymbolId) -> Option<BoundaryValue> {
        self.flux_surface.get(id)
    }

    /// Insert, update, or remove an indexed state value.
    pub fn set_state_value(&mut self, id: SymbolId, value: Option<BoundaryValue>) {
        self.state_surface.set(id, value);
    }

    /// Insert, update, or remove an indexed flux value.
    pub fn set_flux_value(&mut self, id: SymbolId, value: Option<BoundaryValue>) {
        self.flux_surface.set(id, value);
    }

    /// Insert, update, or remove a state value by logical symbol.
    ///
    /// # Errors
    ///
    /// Returns [`SymbolRegistryError::UnknownSymbol`] when the logical symbol is
    /// not present in the frozen registry.
    pub fn set_state_symbol(
        &mut self,
        registry: &SymbolRegistry,
        symbol: &BoundarySymbol,
        value: Option<BoundaryValue>,
    ) -> Result<(), SymbolRegistryError> {
        let id = registry.id_of(symbol)?;
        self.set_state_value(id, value);
        Ok(())
    }

    /// Insert, update, or remove a flux value by logical symbol.
    ///
    /// # Errors
    ///
    /// Returns [`SymbolRegistryError::UnknownSymbol`] when the logical symbol is
    /// not present in the frozen registry.
    pub fn set_flux_symbol(
        &mut self,
        registry: &SymbolRegistry,
        symbol: &BoundarySymbol,
        value: Option<BoundaryValue>,
    ) -> Result<(), SymbolRegistryError> {
        let id = registry.id_of(symbol)?;
        self.set_flux_value(id, value);
        Ok(())
    }

    /// Apply an accepted logical writeback payload to the indexed mirror.
    ///
    /// This keeps Stage-4 read mirrors synchronized while preserving the
    /// existing logical writeback payload shape.
    ///
    /// # Errors
    ///
    /// Returns [`SymbolRegistryError::UnknownSymbol`] when a writeback field
    /// names a symbol outside the frozen registry.
    pub fn apply_writeback_payload(
        &mut self,
        registry: &SymbolRegistry,
        payload: &KernelWritebackPayload,
    ) -> Result<(), SymbolRegistryError> {
        for field in &payload.state_updates {
            self.set_state_symbol(registry, &field.symbol, Some(field.value))?;
        }
        for field in &payload.flux_updates {
            self.set_flux_symbol(registry, &field.symbol, Some(field.value))?;
        }
        Ok(())
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

/// Logical symbol paired with its frozen registry id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedBoundarySymbol {
    pub symbol: BoundarySymbol,
    pub id: SymbolId,
}

impl IndexedBoundarySymbol {
    #[must_use]
    pub const fn new(symbol: BoundarySymbol, id: SymbolId) -> Self {
        Self { symbol, id }
    }
}

/// One-indexed sparse symbol series for canonical `root_0001` families.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedSymbolSeries {
    symbols_by_one_based_index: Vec<Option<IndexedBoundarySymbol>>,
}

impl IndexedSymbolSeries {
    #[must_use]
    fn from_pairs(pairs: Vec<(usize, IndexedBoundarySymbol)>) -> Self {
        let max_index = pairs.iter().map(|(index, _)| *index).max().unwrap_or(0);
        let mut symbols_by_one_based_index = vec![None; max_index];
        for (index, symbol) in pairs {
            if index > 0 {
                symbols_by_one_based_index[index - 1] = Some(symbol);
            }
        }
        Self {
            symbols_by_one_based_index,
        }
    }

    #[must_use]
    pub fn get(&self, one_based_index: usize) -> Option<&IndexedBoundarySymbol> {
        if one_based_index == 0 {
            return None;
        }
        self.symbols_by_one_based_index
            .get(one_based_index - 1)
            .and_then(Option::as_ref)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.symbols_by_one_based_index.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.symbols_by_one_based_index.is_empty()
    }

    pub fn symbols(&self) -> impl Iterator<Item = &IndexedBoundarySymbol> {
        self.symbols_by_one_based_index
            .iter()
            .filter_map(Option::as_ref)
    }
}

/// Sparse two-index symbol grid for canonical `root_0001_0001` families.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedSymbolGrid {
    symbols_by_index: BTreeMap<(usize, usize), IndexedBoundarySymbol>,
}

impl IndexedSymbolGrid {
    #[must_use]
    fn from_pairs(pairs: Vec<((usize, usize), IndexedBoundarySymbol)>) -> Self {
        Self {
            symbols_by_index: pairs.into_iter().collect(),
        }
    }

    #[must_use]
    pub fn get(&self, first_index: usize, second_index: usize) -> Option<&IndexedBoundarySymbol> {
        self.symbols_by_index.get(&(first_index, second_index))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.symbols_by_index.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.symbols_by_index.is_empty()
    }

    pub fn symbols(&self) -> impl Iterator<Item = &IndexedBoundarySymbol> {
        self.symbols_by_index.values()
    }
}

/// Resolve-once hot symbol id tables for indexed execution reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotSymbolTables {
    state_scalars: HashMap<String, IndexedBoundarySymbol>,
    flux_scalars: HashMap<String, IndexedBoundarySymbol>,
    state_series: HashMap<String, IndexedSymbolSeries>,
    flux_series: HashMap<String, IndexedSymbolSeries>,
    state_grids: HashMap<String, IndexedSymbolGrid>,
    flux_grids: HashMap<String, IndexedSymbolGrid>,
    pl_state_symbols: IndexedPlSymbolTables,
}

impl HotSymbolTables {
    /// Build hot symbol tables by scanning the frozen registry once.
    #[must_use]
    pub fn from_registry(
        registry: &SymbolRegistry,
        state_scalar_symbols: &[&str],
        flux_scalar_symbols: &[&str],
        state_series_roots: &[&str],
        flux_series_roots: &[&str],
        state_grid_roots: &[&str],
        flux_grid_roots: &[&str],
    ) -> Self {
        Self {
            state_scalars: collect_scalar_symbols(registry, state_scalar_symbols, false),
            flux_scalars: collect_scalar_symbols(registry, flux_scalar_symbols, false),
            state_series: collect_series_roots(registry, state_series_roots),
            flux_series: collect_series_roots(registry, flux_series_roots),
            state_grids: collect_grid_roots(registry, state_grid_roots),
            flux_grids: collect_grid_roots(registry, flux_grid_roots),
            pl_state_symbols: IndexedPlSymbolTables::from_registry(registry),
        }
    }

    #[must_use]
    pub fn state_scalar(&self, symbol: &str) -> Option<&IndexedBoundarySymbol> {
        self.state_scalars.get(symbol)
    }

    #[must_use]
    pub fn flux_scalar(&self, symbol: &str) -> Option<&IndexedBoundarySymbol> {
        self.flux_scalars.get(symbol)
    }

    #[must_use]
    pub fn state_series_symbol(
        &self,
        root: &str,
        one_based_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.state_series
            .get(root)
            .and_then(|series| series.get(one_based_index))
    }

    #[must_use]
    pub fn flux_series_symbol(
        &self,
        root: &str,
        one_based_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.flux_series
            .get(root)
            .and_then(|series| series.get(one_based_index))
    }

    #[must_use]
    pub fn state_grid_symbol(
        &self,
        root: &str,
        first_index: usize,
        second_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.state_grids
            .get(root)
            .and_then(|grid| grid.get(first_index, second_index))
    }

    #[must_use]
    pub fn flux_grid_symbol(
        &self,
        root: &str,
        first_index: usize,
        second_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.flux_grids
            .get(root)
            .and_then(|grid| grid.get(first_index, second_index))
    }

    #[must_use]
    pub fn state_series_count(&self) -> usize {
        self.state_series.len()
    }

    #[must_use]
    pub fn state_grid_count(&self) -> usize {
        self.state_grids.len()
    }

    #[must_use]
    pub fn state_scalar_count(&self) -> usize {
        self.state_scalars.len()
    }

    #[must_use]
    pub fn hot_state_symbols(&self) -> Vec<IndexedBoundarySymbol> {
        let mut symbols = Vec::new();
        symbols.extend(self.state_scalars.values().cloned());
        for series in self.state_series.values() {
            symbols.extend(series.symbols().cloned());
        }
        for grid in self.state_grids.values() {
            symbols.extend(grid.symbols().cloned());
        }
        sort_and_dedup_indexed_symbols(symbols)
    }

    #[must_use]
    pub fn hot_flux_symbols(&self) -> Vec<IndexedBoundarySymbol> {
        let mut symbols = Vec::new();
        symbols.extend(self.flux_scalars.values().cloned());
        for series in self.flux_series.values() {
            symbols.extend(series.symbols().cloned());
        }
        for grid in self.flux_grids.values() {
            symbols.extend(grid.symbols().cloned());
        }
        sort_and_dedup_indexed_symbols(symbols)
    }

    #[must_use]
    pub fn pl_schedule_slot_state_symbol(
        &self,
        root: &str,
        slot_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.pl_state_symbols.schedule_slot(root, slot_index)
    }

    #[must_use]
    pub fn pl_schedule_slot_crop_state_symbol(
        &self,
        root: &str,
        slot_index: usize,
        crop_slot_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.pl_state_symbols
            .schedule_slot_crop(root, slot_index, crop_slot_index)
    }

    #[must_use]
    pub fn pl_growth_slot_crop_state_symbol(
        &self,
        root: &str,
        slot_index: usize,
        crop_slot_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.pl_state_symbols
            .growth_slot_crop(root, slot_index, crop_slot_index)
    }

    #[must_use]
    pub fn pl_decomp_slot_crop_state_symbol(
        &self,
        root: &str,
        slot_index: usize,
        crop_slot_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.pl_state_symbols
            .decomp_slot_crop(root, slot_index, crop_slot_index)
    }

    #[must_use]
    pub fn pl_decomp_slot_crop_indexed_state_symbol(
        &self,
        root: &str,
        slot_index: usize,
        crop_slot_index: usize,
        index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.pl_state_symbols
            .decomp_slot_crop_indexed(root, slot_index, crop_slot_index, index)
    }
}

/// Resolve-once PL slot symbol ids for hot management dispatch.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IndexedPlSymbolTables {
    schedule_slots: BTreeMap<usize, BTreeMap<String, IndexedBoundarySymbol>>,
    schedule_slot_crops: BTreeMap<(usize, usize), BTreeMap<String, IndexedBoundarySymbol>>,
    growth_slot_crops: BTreeMap<(usize, usize), BTreeMap<String, IndexedBoundarySymbol>>,
    decomp_slot_crops: BTreeMap<(usize, usize), BTreeMap<String, IndexedBoundarySymbol>>,
    decomp_slot_crop_indexes:
        BTreeMap<(usize, usize, usize), BTreeMap<String, IndexedBoundarySymbol>>,
}

impl IndexedPlSymbolTables {
    #[must_use]
    pub fn from_registry(registry: &SymbolRegistry) -> Self {
        let mut tables = Self::default();
        for (id, symbol) in registry.iter() {
            if let Some((slot_index, root)) = parse_pl_schedule_slot_symbol(symbol.as_str()) {
                tables.schedule_slots.entry(slot_index).or_default().insert(
                    root.to_owned(),
                    IndexedBoundarySymbol::new(symbol.clone(), id),
                );
            } else if let Some((slot_index, crop_slot_index, root)) =
                parse_pl_schedule_slot_crop_symbol(symbol.as_str())
            {
                tables
                    .schedule_slot_crops
                    .entry((slot_index, crop_slot_index))
                    .or_default()
                    .insert(
                        root.to_owned(),
                        IndexedBoundarySymbol::new(symbol.clone(), id),
                    );
            } else if let Some((slot_index, crop_slot_index, root)) =
                parse_pl_growth_slot_crop_symbol(symbol.as_str())
            {
                tables
                    .growth_slot_crops
                    .entry((slot_index, crop_slot_index))
                    .or_default()
                    .insert(
                        root.to_owned(),
                        IndexedBoundarySymbol::new(symbol.clone(), id),
                    );
            } else if let Some((slot_index, crop_slot_index, root, index)) =
                parse_pl_decomp_slot_crop_indexed_symbol(symbol.as_str())
            {
                tables
                    .decomp_slot_crop_indexes
                    .entry((slot_index, crop_slot_index, index))
                    .or_default()
                    .insert(
                        root.to_owned(),
                        IndexedBoundarySymbol::new(symbol.clone(), id),
                    );
            } else if let Some((slot_index, crop_slot_index, root)) =
                parse_pl_decomp_slot_crop_symbol(symbol.as_str())
            {
                tables
                    .decomp_slot_crops
                    .entry((slot_index, crop_slot_index))
                    .or_default()
                    .insert(
                        root.to_owned(),
                        IndexedBoundarySymbol::new(symbol.clone(), id),
                    );
            }
        }
        tables
    }

    #[must_use]
    pub fn schedule_slot(&self, root: &str, slot_index: usize) -> Option<&IndexedBoundarySymbol> {
        self.schedule_slots
            .get(&slot_index)
            .and_then(|symbols| symbols.get(root))
    }

    #[must_use]
    pub fn schedule_slot_crop(
        &self,
        root: &str,
        slot_index: usize,
        crop_slot_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.schedule_slot_crops
            .get(&(slot_index, crop_slot_index))
            .and_then(|symbols| symbols.get(root))
    }

    #[must_use]
    pub fn growth_slot_crop(
        &self,
        root: &str,
        slot_index: usize,
        crop_slot_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.growth_slot_crops
            .get(&(slot_index, crop_slot_index))
            .and_then(|symbols| symbols.get(root))
    }

    #[must_use]
    pub fn decomp_slot_crop(
        &self,
        root: &str,
        slot_index: usize,
        crop_slot_index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.decomp_slot_crops
            .get(&(slot_index, crop_slot_index))
            .and_then(|symbols| symbols.get(root))
    }

    #[must_use]
    pub fn decomp_slot_crop_indexed(
        &self,
        root: &str,
        slot_index: usize,
        crop_slot_index: usize,
        index: usize,
    ) -> Option<&IndexedBoundarySymbol> {
        self.decomp_slot_crop_indexes
            .get(&(slot_index, crop_slot_index, index))
            .and_then(|symbols| symbols.get(root))
    }
}

fn sort_and_dedup_indexed_symbols(
    mut symbols: Vec<IndexedBoundarySymbol>,
) -> Vec<IndexedBoundarySymbol> {
    symbols.sort_by_key(|symbol| symbol.id);
    symbols.dedup_by_key(|symbol| symbol.id);
    symbols
}

fn collect_scalar_symbols(
    registry: &SymbolRegistry,
    symbols: &[&str],
    include_all_registry_symbols: bool,
) -> HashMap<String, IndexedBoundarySymbol> {
    let mut collected = HashMap::new();
    if include_all_registry_symbols {
        for (id, symbol) in registry.iter() {
            collected.insert(
                symbol.as_str().to_owned(),
                IndexedBoundarySymbol::new(symbol.clone(), id),
            );
        }
    }
    for symbol in symbols {
        let boundary_symbol = BoundarySymbol::from(*symbol);
        if let Ok(id) = registry.id_of(&boundary_symbol) {
            collected.insert(
                (*symbol).to_owned(),
                IndexedBoundarySymbol::new(boundary_symbol, id),
            );
        }
    }
    collected
}

fn collect_series_roots(
    registry: &SymbolRegistry,
    roots: &[&str],
) -> HashMap<String, IndexedSymbolSeries> {
    let mut collected = HashMap::new();
    for root in roots {
        let mut pairs = Vec::new();
        for (id, symbol) in registry.iter() {
            if let Some(index) = parse_series_symbol_index(symbol.as_str(), root) {
                pairs.push((index, IndexedBoundarySymbol::new(symbol.clone(), id)));
            }
        }
        if !pairs.is_empty() {
            collected.insert((*root).to_owned(), IndexedSymbolSeries::from_pairs(pairs));
        }
    }
    collected
}

fn collect_grid_roots(
    registry: &SymbolRegistry,
    roots: &[&str],
) -> HashMap<String, IndexedSymbolGrid> {
    let mut collected = HashMap::new();
    for root in roots {
        let mut pairs = Vec::new();
        for (id, symbol) in registry.iter() {
            if let Some(indexes) = parse_grid_symbol_indexes(symbol.as_str(), root) {
                pairs.push((indexes, IndexedBoundarySymbol::new(symbol.clone(), id)));
            }
        }
        if !pairs.is_empty() {
            collected.insert((*root).to_owned(), IndexedSymbolGrid::from_pairs(pairs));
        }
    }
    collected
}

fn parse_series_symbol_index(symbol: &str, root: &str) -> Option<usize> {
    let suffix = symbol.strip_prefix(root)?.strip_prefix('_')?;
    parse_padded_index(suffix)
}

fn parse_grid_symbol_indexes(symbol: &str, root: &str) -> Option<(usize, usize)> {
    let suffix = symbol.strip_prefix(root)?.strip_prefix('_')?;
    let (first, second) = suffix.split_once('_')?;
    Some((parse_padded_index(first)?, parse_padded_index(second)?))
}

fn parse_pl_schedule_slot_symbol(symbol: &str) -> Option<(usize, &str)> {
    let suffix = symbol.strip_prefix("pl_schedule_slot_")?;
    let (slot, root) = parse_pl_slot_prefix(suffix)?;
    (!root.starts_with("crop_")).then_some((slot, root))
}

fn parse_pl_schedule_slot_crop_symbol(symbol: &str) -> Option<(usize, usize, &str)> {
    parse_pl_slot_crop_symbol(symbol, "pl_schedule_slot_")
}

fn parse_pl_growth_slot_crop_symbol(symbol: &str) -> Option<(usize, usize, &str)> {
    parse_pl_slot_crop_symbol(symbol, "pl_growth_slot_")
}

fn parse_pl_decomp_slot_crop_symbol(symbol: &str) -> Option<(usize, usize, &str)> {
    let (slot, crop, root) = parse_pl_slot_crop_symbol(symbol, "pl_decomp_slot_")?;
    split_trailing_padded_index(root)
        .is_none()
        .then_some((slot, crop, root))
}

fn parse_pl_decomp_slot_crop_indexed_symbol(symbol: &str) -> Option<(usize, usize, &str, usize)> {
    let (slot, crop, root_with_index) = parse_pl_slot_crop_symbol(symbol, "pl_decomp_slot_")?;
    let (root, index) = split_trailing_padded_index(root_with_index)?;
    Some((slot, crop, root, index))
}

fn parse_pl_slot_crop_symbol<'a>(symbol: &'a str, prefix: &str) -> Option<(usize, usize, &'a str)> {
    let suffix = symbol.strip_prefix(prefix)?;
    let (slot, rest) = parse_pl_slot_prefix(suffix)?;
    let crop_suffix = rest.strip_prefix("crop_")?;
    let (crop_text, root) = crop_suffix.split_once('_')?;
    Some((slot, parse_padded_index(crop_text)?, root))
}

fn parse_pl_slot_prefix(suffix: &str) -> Option<(usize, &str)> {
    let (slot_text, rest) = suffix.split_once('_')?;
    Some((parse_padded_index(slot_text)?, rest))
}

fn split_trailing_padded_index(value: &str) -> Option<(&str, usize)> {
    let (root, index_text) = value.rsplit_once('_')?;
    Some((root, parse_padded_index(index_text)?))
}

fn parse_padded_index(value: &str) -> Option<usize> {
    if value.len() != 4 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let parsed = value.parse::<usize>().ok()?;
    (parsed > 0).then_some(parsed)
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
