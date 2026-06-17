#[allow(clippy::wildcard_imports)]
use super::*;
#[allow(clippy::wildcard_imports)]
use crate::constants::*;
use crate::consumer_boundary::ActivePlSlotSelection;

fn pl_schedule_slot_symbol(root: &str, slot_index: usize) -> String {
    format!("pl_schedule_slot_{slot_index:04}_{root}")
}

fn pl_schedule_slot_crop_symbol(root: &str, slot_index: usize, crop_slot_index: usize) -> String {
    format!("pl_schedule_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn pl_growth_slot_crop_symbol(root: &str, slot_index: usize, crop_slot_index: usize) -> String {
    format!("pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn pl_decomp_slot_crop_symbol(root: &str, slot_index: usize, crop_slot_index: usize) -> String {
    format!("pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}")
}

fn pl_decomp_slot_crop_indexed_symbol(
    root: &str,
    slot_index: usize,
    crop_slot_index: usize,
    index: usize,
) -> String {
    format!("pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}_{index:04}")
}

#[derive(Clone, Copy)]
struct PlDispatchContext<'a> {
    state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
    indexed_writeback_surface: Option<&'a IndexedWritebackSurface>,
    hot_symbol_tables: Option<&'a HotSymbolTables>,
}

impl<'a> PlDispatchContext<'a> {
    #[cfg(test)]
    fn logical(state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>) -> Self {
        Self {
            state_surface,
            indexed_writeback_surface: None,
            hot_symbol_tables: None,
        }
    }

    fn indexed(
        state_surface: &'a BTreeMap<BoundarySymbol, BoundaryValue>,
        indexed_writeback_surface: Option<&'a IndexedWritebackSurface>,
        hot_symbol_tables: Option<&'a HotSymbolTables>,
    ) -> Self {
        Self {
            state_surface,
            indexed_writeback_surface,
            hot_symbol_tables,
        }
    }

    fn state_scalar_symbol(&self, symbol: &'static str) -> PlDispatchSymbolRef<'a> {
        if let Some(indexed_symbol) =
            self.hot_symbol_tables.and_then(|tables| tables.state_scalar(symbol))
        {
            PlDispatchSymbolRef::Indexed(indexed_symbol)
        } else {
            PlDispatchSymbolRef::Owned(BoundarySymbol::from(symbol))
        }
    }

    fn schedule_slot_symbol(&self, root: &str, slot_index: usize) -> PlDispatchSymbolRef<'a> {
        if let Some(indexed_symbol) = self
            .hot_symbol_tables
            .and_then(|tables| tables.pl_schedule_slot_state_symbol(root, slot_index))
        {
            PlDispatchSymbolRef::Indexed(indexed_symbol)
        } else {
            PlDispatchSymbolRef::Owned(BoundarySymbol::from(pl_schedule_slot_symbol(
                root, slot_index,
            )))
        }
    }

    fn schedule_slot_crop_symbol(
        &self,
        root: &str,
        slot_index: usize,
        crop_slot_index: usize,
    ) -> PlDispatchSymbolRef<'a> {
        if let Some(indexed_symbol) = self.hot_symbol_tables.and_then(|tables| {
            tables.pl_schedule_slot_crop_state_symbol(root, slot_index, crop_slot_index)
        }) {
            PlDispatchSymbolRef::Indexed(indexed_symbol)
        } else {
            PlDispatchSymbolRef::Owned(BoundarySymbol::from(pl_schedule_slot_crop_symbol(
                root,
                slot_index,
                crop_slot_index,
            )))
        }
    }

    fn growth_slot_crop_symbol(
        &self,
        root: &str,
        slot_index: usize,
        crop_slot_index: usize,
    ) -> PlDispatchSymbolRef<'a> {
        if let Some(indexed_symbol) = self.hot_symbol_tables.and_then(|tables| {
            tables.pl_growth_slot_crop_state_symbol(root, slot_index, crop_slot_index)
        }) {
            PlDispatchSymbolRef::Indexed(indexed_symbol)
        } else {
            PlDispatchSymbolRef::Owned(BoundarySymbol::from(pl_growth_slot_crop_symbol(
                root,
                slot_index,
                crop_slot_index,
            )))
        }
    }
}

enum PlDispatchSymbolRef<'a> {
    Indexed(&'a IndexedBoundarySymbol),
    Owned(BoundarySymbol),
}

impl PlDispatchSymbolRef<'_> {
    fn symbol(&self) -> &BoundarySymbol {
        match self {
            Self::Indexed(symbol) => &symbol.symbol,
            Self::Owned(symbol) => symbol,
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
fn require_finite_pl_dispatch_symbol(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
) -> Result<f64, HillslopePlActiveSlotResolutionError> {
    require_finite_pl_dispatch_symbol_ref(
        PlDispatchContext::logical(state_surface),
        PlDispatchSymbolRef::Owned(BoundarySymbol::from(symbol)),
    )
}

#[allow(clippy::needless_pass_by_value)]
fn require_finite_pl_dispatch_symbol_ref(
    context: PlDispatchContext<'_>,
    symbol_ref: PlDispatchSymbolRef<'_>,
) -> Result<f64, HillslopePlActiveSlotResolutionError> {
    let symbol = symbol_ref.symbol();
    let value = match &symbol_ref {
        PlDispatchSymbolRef::Indexed(indexed_symbol) => context
            .indexed_writeback_surface
            .and_then(|surface| surface.state_value(indexed_symbol.id))
            .or_else(|| context.state_surface.get(&indexed_symbol.symbol).copied()),
        PlDispatchSymbolRef::Owned(symbol) => context.state_surface.get(symbol).copied(),
    };
    let value = value
        .ok_or_else(
            || HillslopePlActiveSlotResolutionError::MissingRequiredStateSymbol {
                symbol: symbol.clone(),
            },
        )?
        .as_f64();

    if !value.is_finite() {
        return Err(
            HillslopePlActiveSlotResolutionError::NonFiniteRequiredStateSymbol {
                symbol: symbol.clone(),
                value,
            },
        );
    }

    Ok(value)
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
#[cfg(test)]
#[allow(dead_code)]
fn require_integral_pl_dispatch_symbol_in_range(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopePlActiveSlotResolutionError> {
    require_integral_pl_dispatch_symbol_ref_in_range(
        PlDispatchContext::logical(state_surface),
        PlDispatchSymbolRef::Owned(BoundarySymbol::from(symbol)),
        min_allowed,
        max_allowed,
    )
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn require_integral_pl_dispatch_symbol_ref_in_range(
    context: PlDispatchContext<'_>,
    symbol_ref: PlDispatchSymbolRef<'_>,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopePlActiveSlotResolutionError> {
    let symbol = symbol_ref.symbol().clone();
    let value = require_finite_pl_dispatch_symbol_ref(context, symbol_ref)?;
    let rounded = value.round();
    if (value - rounded).abs() > MANAGEMENT_CLASS_EPSILON {
        return Err(
            HillslopePlActiveSlotResolutionError::NonIntegralRequiredStateSymbol {
                symbol,
                value,
            },
        );
    }

    let min_f64 = min_allowed as f64;
    let max_f64 = max_allowed as f64;
    if rounded < min_f64 || rounded > max_f64 {
        return Err(
            HillslopePlActiveSlotResolutionError::StateSymbolValueOutOfRange {
                symbol,
                value: rounded as usize,
                min_allowed,
                max_allowed,
            },
        );
    }

    Ok(rounded as usize)
}

fn day_is_within_julian_window(day_of_year: usize, start_day: usize, end_day: usize) -> bool {
    if start_day <= end_day {
        day_of_year >= start_day && day_of_year <= end_day
    } else {
        day_of_year >= start_day || day_of_year <= end_day
    }
}

fn select_active_crop_slot_for_day(
    context: PlDispatchContext<'_>,
    slot_index: usize,
    crop_slots: usize,
    day_of_year: usize,
) -> Result<usize, HillslopePlActiveSlotResolutionError> {
    let mut candidates = Vec::new();

    for crop_slot_index in 1..=crop_slots {
        let imngmt_symbol = context.schedule_slot_crop_symbol(
            PL_SCHEDULE_SLOT_CROP_IMNGMT_ROOT,
            slot_index,
            crop_slot_index,
        );
        let imngmt = require_integral_pl_dispatch_symbol_ref_in_range(
            context,
            imngmt_symbol,
            1,
            3,
        )?;

        let growth_imngmt_symbol = context.growth_slot_crop_symbol("imngmt", slot_index, crop_slot_index);
        let _ = require_integral_pl_dispatch_symbol_ref_in_range(
            context,
            growth_imngmt_symbol,
            1,
            3,
        )?;

        let jdplt_symbol = context.growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index);
        let jdplt = require_integral_pl_dispatch_symbol_ref_in_range(
            context,
            jdplt_symbol,
            usize::from(imngmt != 2),
            366,
        )?;
        let jdharv_symbol = context.growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index);
        let jdharv = require_integral_pl_dispatch_symbol_ref_in_range(
            context,
            jdharv_symbol,
            0,
            366,
        )?;

        let is_active = if imngmt == 2 {
            // PL11+ carries full perennial event payloads; PL10 keeps slot
            // selection bounded to existing day-window symbols.
            let jdstop_symbol = context.growth_slot_crop_symbol("jdstop", slot_index, crop_slot_index);
            let jdstop = require_integral_pl_dispatch_symbol_ref_in_range(
                context,
                jdstop_symbol,
                0,
                366,
            )?;
            if jdplt == 0 {
                jdstop == 0 || day_of_year <= jdstop
            } else if jdstop == 0 {
                day_is_within_julian_window(day_of_year, jdplt, jdharv.max(1))
            } else {
                day_is_within_julian_window(day_of_year, jdplt, jdstop)
            }
        } else {
            day_is_within_julian_window(day_of_year, jdplt, jdharv.max(1))
        };

        if is_active {
            candidates.push(crop_slot_index);
        }
    }

    match candidates.as_slice() {
        [crop_slot_index] => Ok(*crop_slot_index),
        [] => Err(
            HillslopePlActiveSlotResolutionError::MissingActiveCropForDay {
                slot_index,
                day_of_year,
            },
        ),
        _ => Err(
            HillslopePlActiveSlotResolutionError::AmbiguousActiveCropForDay {
                slot_index,
                day_of_year,
                crop_slot_indexes: candidates,
            },
        ),
    }
}

#[allow(clippy::too_many_lines)]
#[cfg(test)]
#[allow(dead_code)]
fn resolve_active_pl_slot_selection(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<ActivePlSlotSelection, HillslopePlActiveSlotResolutionError> {
    resolve_active_pl_slot_selection_with_context(PlDispatchContext::logical(state_surface))
}

#[allow(clippy::too_many_lines)]
fn resolve_active_pl_slot_selection_with_context(
    context: PlDispatchContext<'_>,
) -> Result<ActivePlSlotSelection, HillslopePlActiveSlotResolutionError> {
    let slot_count = require_integral_pl_dispatch_symbol_ref_in_range(
        context,
        context.state_scalar_symbol(PL_SCHEDULE_SLOT_COUNT_SYMBOL),
        1,
        usize::MAX,
    )?;
    let rotation_years = require_integral_pl_dispatch_symbol_ref_in_range(
        context,
        context.state_scalar_symbol(PL_SCHEDULE_ROTATION_YEARS_SYMBOL),
        1,
        usize::MAX,
    )?;
    let rotation_repeats = require_integral_pl_dispatch_symbol_ref_in_range(
        context,
        context.state_scalar_symbol(PL_SCHEDULE_ROTATION_REPEATS_SYMBOL),
        1,
        usize::MAX,
    )?;
    let runtime_year = require_integral_pl_dispatch_symbol_ref_in_range(
        context,
        context.state_scalar_symbol(PL_RUNTIME_YEAR_SYMBOL),
        1,
        usize::MAX,
    )?;
    let max_runtime_year = rotation_repeats.saturating_mul(rotation_years);
    if runtime_year > max_runtime_year {
        return Err(
            HillslopePlActiveSlotResolutionError::StateSymbolValueOutOfRange {
                symbol: BoundarySymbol::from(PL_RUNTIME_YEAR_SYMBOL),
                value: runtime_year,
                min_allowed: 1,
                max_allowed: max_runtime_year,
            },
        );
    }
    let day_of_year = require_integral_pl_dispatch_symbol_ref_in_range(
        context,
        context.state_scalar_symbol(PL_RUNTIME_DAY_SYMBOL),
        1,
        366,
    )?;
    let rotation_index = ((runtime_year - 1) / rotation_years) + 1;
    let year_in_rotation = ((runtime_year - 1) % rotation_years) + 1;

    let mut slot_candidates = Vec::new();
    for slot_index in 1..=slot_count {
        let slot_ofe_symbol =
            context.schedule_slot_symbol(PL_SCHEDULE_SLOT_OFE_INDEX_ROOT, slot_index);
        let ofe_index = require_integral_pl_dispatch_symbol_ref_in_range(
            context,
            slot_ofe_symbol,
            1,
            usize::MAX,
        )?;
        if ofe_index != PL_PRIMARY_OFE_INDEX {
            continue;
        }

        let slot_year_symbol =
            context.schedule_slot_symbol(PL_SCHEDULE_SLOT_YEAR_IN_ROTATION_ROOT, slot_index);
        let slot_year_in_rotation = require_integral_pl_dispatch_symbol_ref_in_range(
            context,
            slot_year_symbol,
            1,
            rotation_years,
        )?;
        let slot_rotation_symbol =
            context.schedule_slot_symbol(PL_SCHEDULE_SLOT_ROTATION_INDEX_ROOT, slot_index);
        let slot_rotation_index = require_integral_pl_dispatch_symbol_ref_in_range(
            context,
            slot_rotation_symbol,
            1,
            rotation_repeats,
        )?;
        if slot_year_in_rotation == year_in_rotation && slot_rotation_index == rotation_index {
            slot_candidates.push(slot_index);
        }
    }

    let slot_index = match slot_candidates.as_slice() {
        [slot_index] => *slot_index,
        [] => {
            return Err(
                HillslopePlActiveSlotResolutionError::MissingActiveSlotForOfeYear {
                    ofe_index: PL_PRIMARY_OFE_INDEX,
                    year_in_rotation,
                },
            );
        }
        _ => {
            return Err(
                HillslopePlActiveSlotResolutionError::AmbiguousActiveSlotForOfeYear {
                    ofe_index: PL_PRIMARY_OFE_INDEX,
                    year_in_rotation,
                    slot_indexes: slot_candidates,
                },
            );
        }
    };

    let crop_slots_symbol =
        context.schedule_slot_symbol(PL_SCHEDULE_SLOT_CROP_SLOTS_ROOT, slot_index);
    let crop_slots = require_integral_pl_dispatch_symbol_ref_in_range(
        context,
        crop_slots_symbol,
        0,
        usize::MAX,
    )?;
    if crop_slots == 0 {
        return Err(HillslopePlActiveSlotResolutionError::InvalidCropSlotCount {
            slot_index,
            crop_slots,
        });
    }

    let crop_slot_index =
        select_active_crop_slot_for_day(context, slot_index, crop_slots, day_of_year)?;
    Ok(ActivePlSlotSelection {
        slot_index,
        crop_slot_index,
    })
}
