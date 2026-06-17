use std::cell::RefCell;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::hint::black_box;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use openwepp_hillslope_orchestrator::HillslopeWritebackSurface;
use openwepp_input_contract::parsers::climate::ClimateFile;
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, IndexedWritebackSurface, SymbolId, SymbolRegistry,
};
use serde::Serialize;

use crate::errors::HillslopeCliError;

use super::{HillslopeClimateExecutionState, symbol_registry_audit};

const INDEXED_SHADOW_REPORT_PATH_ENV: &str = "OPENWEPP_INDEXED_SHADOW_REPORT_PATH";
const INDEXED_SHADOW_SCHEMA: &str = "openwepp.indexed_shadow_surface.v1";
const MAX_MISMATCHES_RECORDED: usize = 20;
const LOOKUP_OP_LIMIT: usize = 2_048;

#[derive(Debug)]
pub(super) struct IndexedShadowRun {
    report_path: PathBuf,
}

impl IndexedShadowRun {
    pub(super) fn finish(self) -> Result<(), HillslopeCliError> {
        let report = finish_report()?;
        write_report(&self.report_path, &report)?;
        if report.mismatch_count > 0 {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "indexed_shadow_surface",
                detail: format!(
                    "indexed shadow validation found {} mismatches; report written to {}",
                    report.mismatch_count,
                    self.report_path.display()
                ),
            });
        }
        if !report.clone_economics.sparse_clone_is_win {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "indexed_shadow_surface",
                detail: format!(
                    "sparse indexed clone is not a win against BTreeMap clone; report written to {}",
                    self.report_path.display()
                ),
            });
        }
        Ok(())
    }
}

pub(super) fn begin_if_requested(
    state: &HillslopeClimateExecutionState,
    climate: &ClimateFile,
) -> Result<Option<IndexedShadowRun>, HillslopeCliError> {
    let Some(path) = env::var_os(INDEXED_SHADOW_REPORT_PATH_ENV).map(PathBuf::from) else {
        return Ok(None);
    };
    if path.as_os_str().is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "indexed_shadow_surface",
            detail: format!("{INDEXED_SHADOW_REPORT_PATH_ENV} must not be empty"),
        });
    }

    let registry =
        symbol_registry_audit::build_registry_for_run(state, climate, "indexed_shadow_surface")?;
    INDEXED_SHADOW_STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        if state.is_some() {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "indexed_shadow_surface",
                detail: "indexed shadow report is already active".to_string(),
            });
        }
        *state = Some(IndexedShadowState::new(registry));
        Ok(())
    })?;

    Ok(Some(IndexedShadowRun { report_path: path }))
}

pub(super) fn observe_clone_source_surface(
    surface: &HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
    with_state_if_active(|state| {
        state.clone_source_observation_count += 1;
        let state_entries = surface.state_surface.len();
        let flux_entries = surface.flux_surface.len();
        let total_entries = state_entries + flux_entries;
        state.max_clone_source_state_entries =
            state.max_clone_source_state_entries.max(state_entries);
        state.max_clone_source_flux_entries = state.max_clone_source_flux_entries.max(flux_entries);
        state.max_clone_source_total_entries =
            state.max_clone_source_total_entries.max(total_entries);
        if state
            .largest_clone_source
            .as_ref()
            .is_none_or(|snapshot| total_entries > snapshot.total_entries())
        {
            state.largest_clone_source = Some(SurfaceSnapshot::from_surface(surface));
        }
        Ok(())
    })
}

pub(super) fn validate_shadow_surface(
    surface: &HillslopeWritebackSurface,
) -> Result<(), HillslopeCliError> {
    with_state_if_active(|state| {
        state.shadow_equality_checks += 1;
        state.state_entries_checked += surface.state_surface.len();
        state.flux_entries_checked += surface.flux_surface.len();

        let indexed = match IndexedWritebackSurface::from_btreemap_surfaces(
            &state.registry,
            &surface.state_surface,
            &surface.flux_surface,
        ) {
            Ok(indexed) => indexed,
            Err(error) => {
                state.record_mismatch(format!("index construction failed: {error}"));
                return Ok(());
            }
        };
        let (exported_state, exported_flux) =
            match indexed.export_btreemap_surfaces(&state.registry) {
                Ok(exported) => exported,
                Err(error) => {
                    state.record_mismatch(format!("index export failed: {error}"));
                    return Ok(());
                }
            };
        if exported_state != surface.state_surface {
            state.record_mismatch(format!(
                "state surface mismatch: expected {} entries, exported {} entries",
                surface.state_surface.len(),
                exported_state.len()
            ));
        }
        if exported_flux != surface.flux_surface {
            state.record_mismatch(format!(
                "flux surface mismatch: expected {} entries, exported {} entries",
                surface.flux_surface.len(),
                exported_flux.len()
            ));
        }
        Ok(())
    })
}

fn with_state_if_active<F>(work: F) -> Result<(), HillslopeCliError>
where
    F: FnOnce(&mut IndexedShadowState) -> Result<(), HillslopeCliError>,
{
    INDEXED_SHADOW_STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        let Some(state) = state.as_mut() else {
            return Ok(());
        };
        work(state)
    })
}

fn finish_report() -> Result<IndexedShadowReportJson, HillslopeCliError> {
    INDEXED_SHADOW_STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        let state = state
            .take()
            .ok_or_else(|| HillslopeCliError::RuntimeSurfaceFailure {
                surface: "indexed_shadow_surface",
                detail: "indexed shadow report was not active at finish".to_string(),
            })?;
        state.into_report()
    })
}

fn write_report(
    report_path: &Path,
    report: &IndexedShadowReportJson,
) -> Result<(), HillslopeCliError> {
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent).map_err(|source| HillslopeCliError::OutputWrite {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    let json = serde_json::to_string_pretty(report)
        .map_err(|source| HillslopeCliError::ManifestSerialize { source })?;
    fs::write(report_path, json).map_err(|source| HillslopeCliError::OutputWrite {
        path: report_path.to_path_buf(),
        source,
    })
}

thread_local! {
    static INDEXED_SHADOW_STATE: RefCell<Option<IndexedShadowState>> = const { RefCell::new(None) };
}

#[derive(Debug)]
struct IndexedShadowState {
    registry: SymbolRegistry,
    clone_source_observation_count: usize,
    shadow_equality_checks: usize,
    state_entries_checked: usize,
    flux_entries_checked: usize,
    max_clone_source_state_entries: usize,
    max_clone_source_flux_entries: usize,
    max_clone_source_total_entries: usize,
    largest_clone_source: Option<SurfaceSnapshot>,
    mismatch_count: usize,
    mismatches: Vec<String>,
}

impl IndexedShadowState {
    fn new(registry: SymbolRegistry) -> Self {
        Self {
            registry,
            clone_source_observation_count: 0,
            shadow_equality_checks: 0,
            state_entries_checked: 0,
            flux_entries_checked: 0,
            max_clone_source_state_entries: 0,
            max_clone_source_flux_entries: 0,
            max_clone_source_total_entries: 0,
            largest_clone_source: None,
            mismatch_count: 0,
            mismatches: Vec::new(),
        }
    }

    fn record_mismatch(&mut self, mismatch: String) {
        self.mismatch_count += 1;
        if self.mismatches.len() < MAX_MISMATCHES_RECORDED {
            self.mismatches.push(mismatch);
        }
    }

    fn into_report(self) -> Result<IndexedShadowReportJson, HillslopeCliError> {
        let Some(snapshot) = self.largest_clone_source.as_ref() else {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "indexed_shadow_surface",
                detail: "no clone source surfaces were observed".to_string(),
            });
        };
        let clone_economics = measure_clone_economics(&self.registry, snapshot)?;
        let rss = read_current_rss();

        Ok(IndexedShadowReportJson {
            schema: INDEXED_SHADOW_SCHEMA,
            chosen_representation: if clone_economics.sparse_clone_is_win {
                "sparse_sorted_vec"
            } else if clone_economics.compact_clone_is_win {
                "compact_local_dense_values"
            } else {
                "no_go"
            },
            registry_symbol_count: self.registry.len(),
            clone_source_observation_count: self.clone_source_observation_count,
            shadow_equality_checks: self.shadow_equality_checks,
            state_entries_checked: self.state_entries_checked,
            flux_entries_checked: self.flux_entries_checked,
            max_clone_source_state_entries: self.max_clone_source_state_entries,
            max_clone_source_flux_entries: self.max_clone_source_flux_entries,
            max_clone_source_total_entries: self.max_clone_source_total_entries,
            benchmark_surface_state_entries: snapshot.state_surface.len(),
            benchmark_surface_flux_entries: snapshot.flux_surface.len(),
            benchmark_surface_total_entries: snapshot.total_entries(),
            clone_economics,
            rss,
            mismatch_count: self.mismatch_count,
            mismatches: self.mismatches,
        })
    }
}

#[derive(Debug, Clone)]
struct SurfaceSnapshot {
    state_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
    flux_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
}

impl SurfaceSnapshot {
    fn from_surface(surface: &HillslopeWritebackSurface) -> Self {
        Self {
            state_surface: surface.state_surface.clone(),
            flux_surface: surface.flux_surface.clone(),
        }
    }

    fn total_entries(&self) -> usize {
        self.state_surface.len() + self.flux_surface.len()
    }

    fn clone_writeback_surface(&self) -> HillslopeWritebackSurface {
        HillslopeWritebackSurface {
            state_surface: self.state_surface.clone(),
            flux_surface: self.flux_surface.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct CompactDenseCandidate {
    state_values: Vec<BoundaryValue>,
    flux_values: Vec<BoundaryValue>,
}

impl CompactDenseCandidate {
    fn from_indexed(indexed: &IndexedWritebackSurface) -> Self {
        Self {
            state_values: indexed
                .state_surface()
                .entries()
                .iter()
                .map(|(_, value)| *value)
                .collect(),
            flux_values: indexed
                .flux_surface()
                .entries()
                .iter()
                .map(|(_, value)| *value)
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
struct IndexedShadowReportJson {
    schema: &'static str,
    chosen_representation: &'static str,
    registry_symbol_count: usize,
    clone_source_observation_count: usize,
    shadow_equality_checks: usize,
    state_entries_checked: usize,
    flux_entries_checked: usize,
    max_clone_source_state_entries: usize,
    max_clone_source_flux_entries: usize,
    max_clone_source_total_entries: usize,
    benchmark_surface_state_entries: usize,
    benchmark_surface_flux_entries: usize,
    benchmark_surface_total_entries: usize,
    clone_economics: CloneEconomicsJson,
    rss: RssJson,
    mismatch_count: usize,
    mismatches: Vec<String>,
}

#[derive(Debug, Serialize)]
struct CloneEconomicsJson {
    clone_repeats: usize,
    lookup_repeats: usize,
    lookup_op_count: usize,
    clone_btreemap_ns_per: f64,
    clone_sparse_ns_per: f64,
    clone_compact_dense_values_ns_per: f64,
    clone_sparse_speedup: f64,
    clone_compact_dense_values_speedup: f64,
    sparse_clone_is_win: bool,
    compact_clone_is_win: bool,
    lookup_btreemap_prebuilt_symbol_ns_per_op: f64,
    lookup_sparse_precomputed_id_ns_per_op: f64,
    lookup_compact_precomputed_local_ns_per_op: f64,
    lookup_sparse_speedup: f64,
    lookup_compact_speedup: f64,
}

#[derive(Debug, Default, Serialize)]
struct RssJson {
    vm_rss_kb: Option<u64>,
    vm_hwm_kb: Option<u64>,
}

fn measure_clone_economics(
    registry: &SymbolRegistry,
    snapshot: &SurfaceSnapshot,
) -> Result<CloneEconomicsJson, HillslopeCliError> {
    let indexed = IndexedWritebackSurface::from_btreemap_surfaces(
        registry,
        &snapshot.state_surface,
        &snapshot.flux_surface,
    )
    .map_err(|error| HillslopeCliError::RuntimeSurfaceFailure {
        surface: "indexed_shadow_surface",
        detail: error.to_string(),
    })?;
    let compact = CompactDenseCandidate::from_indexed(&indexed);
    let lookup_ops = build_lookup_ops(registry, snapshot, &indexed);
    let total_entries = snapshot.total_entries().max(1);
    let clone_repeats = (2_000_000 / total_entries).clamp(1_000, 10_000);
    let lookup_repeats = (500_000 / lookup_ops.len().max(1)).clamp(100, 10_000);

    let clone_btreemap_ns_per = bench_ns_per(clone_repeats, || {
        let clone = black_box(snapshot.clone_writeback_surface());
        black_box(clone.state_surface.len() + clone.flux_surface.len())
    });
    let clone_sparse_ns_per = bench_ns_per(clone_repeats, || {
        let clone = black_box(indexed.clone());
        black_box(clone.state_surface().len() + clone.flux_surface().len())
    });
    let clone_compact_dense_values_ns_per = bench_ns_per(clone_repeats, || {
        let clone = black_box(compact.clone());
        black_box(clone.state_values.len() + clone.flux_values.len())
    });

    let lookup_btreemap_prebuilt_symbol_ns_per_op =
        bench_lookup_ns_per(lookup_repeats, &lookup_ops, |op| match op.surface {
            LookupSurface::State => snapshot
                .state_surface
                .get(&op.symbol)
                .map_or(0.0, |value| value.as_f64()),
            LookupSurface::Flux => snapshot
                .flux_surface
                .get(&op.symbol)
                .map_or(0.0, |value| value.as_f64()),
        });
    let lookup_sparse_precomputed_id_ns_per_op =
        bench_lookup_ns_per(lookup_repeats, &lookup_ops, |op| match op.surface {
            LookupSurface::State => indexed
                .state_surface()
                .get(op.id)
                .map_or(0.0, BoundaryValue::as_f64),
            LookupSurface::Flux => indexed
                .flux_surface()
                .get(op.id)
                .map_or(0.0, BoundaryValue::as_f64),
        });
    let lookup_compact_precomputed_local_ns_per_op =
        bench_lookup_ns_per(lookup_repeats, &lookup_ops, |op| match op.surface {
            LookupSurface::State => compact
                .state_values
                .get(op.local_index)
                .map_or(0.0, |value| value.as_f64()),
            LookupSurface::Flux => compact
                .flux_values
                .get(op.local_index)
                .map_or(0.0, |value| value.as_f64()),
        });

    Ok(CloneEconomicsJson {
        clone_repeats,
        lookup_repeats,
        lookup_op_count: lookup_ops.len(),
        clone_btreemap_ns_per,
        clone_sparse_ns_per,
        clone_compact_dense_values_ns_per,
        clone_sparse_speedup: ratio(clone_btreemap_ns_per, clone_sparse_ns_per),
        clone_compact_dense_values_speedup: ratio(
            clone_btreemap_ns_per,
            clone_compact_dense_values_ns_per,
        ),
        sparse_clone_is_win: clone_sparse_ns_per < clone_btreemap_ns_per,
        compact_clone_is_win: clone_compact_dense_values_ns_per < clone_btreemap_ns_per,
        lookup_btreemap_prebuilt_symbol_ns_per_op,
        lookup_sparse_precomputed_id_ns_per_op,
        lookup_compact_precomputed_local_ns_per_op,
        lookup_sparse_speedup: ratio(
            lookup_btreemap_prebuilt_symbol_ns_per_op,
            lookup_sparse_precomputed_id_ns_per_op,
        ),
        lookup_compact_speedup: ratio(
            lookup_btreemap_prebuilt_symbol_ns_per_op,
            lookup_compact_precomputed_local_ns_per_op,
        ),
    })
}

#[derive(Debug, Clone, Copy)]
enum LookupSurface {
    State,
    Flux,
}

#[derive(Debug, Clone)]
struct LookupOp {
    surface: LookupSurface,
    symbol: BoundarySymbol,
    id: SymbolId,
    local_index: usize,
}

fn build_lookup_ops(
    registry: &SymbolRegistry,
    snapshot: &SurfaceSnapshot,
    indexed: &IndexedWritebackSurface,
) -> Vec<LookupOp> {
    let mut ops = Vec::new();
    push_lookup_ops(
        &mut ops,
        registry,
        LookupSurface::State,
        &snapshot.state_surface,
        indexed.state_surface().entries(),
    );
    push_lookup_ops(
        &mut ops,
        registry,
        LookupSurface::Flux,
        &snapshot.flux_surface,
        indexed.flux_surface().entries(),
    );
    ops.truncate(LOOKUP_OP_LIMIT);
    ops
}

fn push_lookup_ops(
    ops: &mut Vec<LookupOp>,
    registry: &SymbolRegistry,
    surface: LookupSurface,
    btree_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    indexed_entries: &[(SymbolId, BoundaryValue)],
) {
    let local_by_id = indexed_entries
        .iter()
        .enumerate()
        .map(|(index, (id, _))| (*id, index))
        .collect::<BTreeMap<_, _>>();
    for symbol in btree_surface.keys() {
        if let Ok(id) = registry.id_of(symbol)
            && let Some(local_index) = local_by_id.get(&id).copied()
        {
            ops.push(LookupOp {
                surface,
                symbol: symbol.clone(),
                id,
                local_index,
            });
        }
    }
}

fn bench_ns_per<F>(repeats: usize, mut work: F) -> f64
where
    F: FnMut() -> usize,
{
    let start = Instant::now();
    let mut sink = 0usize;
    for _ in 0..repeats {
        sink ^= work();
    }
    black_box(sink);
    ns_per(start.elapsed(), repeats)
}

fn bench_lookup_ns_per<F>(repeats: usize, ops: &[LookupOp], mut lookup: F) -> f64
where
    F: FnMut(&LookupOp) -> f64,
{
    let start = Instant::now();
    let mut sink = 0.0;
    for _ in 0..repeats {
        for op in ops {
            sink += lookup(op);
        }
    }
    black_box(sink);
    ns_per(start.elapsed(), repeats * ops.len().max(1))
}

fn ns_per(duration: Duration, repeats: usize) -> f64 {
    let repeats = u32::try_from(repeats).unwrap_or(u32::MAX);
    duration.as_secs_f64() * 1_000_000_000.0 / f64::from(repeats)
}

fn ratio(numerator: f64, denominator: f64) -> f64 {
    if denominator == 0.0 {
        return f64::INFINITY;
    }
    numerator / denominator
}

fn read_current_rss() -> RssJson {
    let Ok(status) = fs::read_to_string("/proc/self/status") else {
        return RssJson::default();
    };
    RssJson {
        vm_rss_kb: parse_status_kb(&status, "VmRSS:"),
        vm_hwm_kb: parse_status_kb(&status, "VmHWM:"),
    }
}

fn parse_status_kb(status: &str, field: &str) -> Option<u64> {
    status.lines().find_map(|line| {
        let rest = line.strip_prefix(field)?;
        rest.split_whitespace().next()?.parse::<u64>().ok()
    })
}
