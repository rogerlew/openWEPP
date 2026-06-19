# PERFDEEP06 Layout and Allocation Ledger

Status: complete 2026-06-19.
Evidence class: Static + Ran.

## Current Layout Risks

- Current `HillslopeDayFrame` has registry-sized
  `Vec<Option<BoundaryValue>>` state/flux slots plus a cloned `SymbolRegistry`.
  This is transition-only.
- Current `HillslopeLaneDenseState` is compact but still stores
  `Option<BoundaryValue>`, `Vec<Option<usize>>`, `Vec<SymbolId>`, and
  `BTreeSet<SymbolId>` dirty ids.
- `BoundaryValue` is a 17-variant enum, not a guaranteed niche-optimized scalar.
- `HotSymbolTables::hot_state_symbols()` and `hot_flux_symbols()` allocate fresh
  vectors and sort/dedup.
- Several current hydrology phase helpers allocate per-run vectors from
  `layer_count`; PERFDEEP07 should move those into reusable frame/worker
  buffers or typed SoA columns.
- PERFDEEP05 default-disabled H2637 measured `701.95 s` versus `669.97 s`,
  and PERFDEEP03 default-disabled measured `697-708 s`. Any dense/indexed
  compatibility object, view, branch chain, or symbol table built when opt-ins
  are disabled is now a default-path allocation/layout risk, not merely an
  opt-in risk.

Ran:

- `wc -l` on inspected files.
- `rg` over symbol/writeback/runtime-surface sites.

## Proposed Frame Families

| Family | Shape | Allocation rule | Validity rule |
|---|---|---|---|
| Core scalar fields | named unit wrappers or `f64` | inline struct fields | no option when required |
| MOFE carry | `[f64; 24]` for upstream/current surface/lateral arrays | inline fixed arrays | all entries valid; zero is physical zero |
| Soil layers | `SoilLayerColumns { theta, thetdr, thetfc, ssc, dg, solthk, ul, fc, ... }` | one pre-sized `Box<[f64]>`/`Vec<f64>` per column at lane setup | active layer count plus bounds, not per-element `Option` |
| Frost fine layers | `FrostColumns` SoA | pre-sized once per lane | active count/bitset |
| Snow hourly state | borrowed slice or fixed hourly struct array where bounded | no per-phase copy | active snow flag and explicit count |
| Publication projection | typed struct per executed row | appended outside phase loop | optional only for schema-nullable output fields |
| Dirty tracking | compact bitset or small `Vec<FieldId>` | preallocated and cleared | field id enum, not `SymbolId` |

## Normal Success Path Prohibitions

PERFDEEP07 must prove that the migrated phase loop does not retain:

- `format!` / `String` construction for hot field names;
- `BoundarySymbol` construction;
- `SymbolRegistry::id_of`;
- `BTreeMap` lookup/insert/remove;
- `KernelWritebackPayload` or `IndexedKernelWritebackPayload`;
- `Vec<WritebackField>` construction;
- `HotSymbolTables::hot_state_symbols()` / `hot_flux_symbols()`;
- per-phase layer work-vector allocation where frame-owned reusable buffers can
  serve the same purpose.
- dense-first frame/view construction or indexed shadow setup when all PERFDEEP
  opt-ins are disabled.

## Measurement Plan for PERFDEEP07

- Record `std::mem::size_of::<HillslopeDayFrame>()` and key subtypes in a
  focused test or diagnostic.
- Record field/array counts and lane-preallocated heap allocations.
- Add an allocation audit command or static proof showing no normal success path
  `format!`, owned symbol clone, collection rebuild, or map write remains in the
  migrated loop.
- Measure H2637 endpoint and RSS against `669.97 s` activation reference and
  final default-disabled comparison.
- Measure the default-disabled path first and prove the dense-first tax is gone
  or explicitly attributed. Require at least three clean H2637 no-UI runs with
  all PERFDEEP opt-ins disabled; record min/median/max/RSS; require median
  `<= 676.67 s` (`669.97 s + 1%`). Run a same-machine control in the same
  harness/session where feasible; if the historical reference cannot be rerun,
  report a pre-cleanup control and require hard attribution for any candidate
  above `676.67 s`. The direct-frame opt-in endpoint is not enough if `main`
  still pays about `+4.7%` when the opt-in is off.

## Gate

PASS. The follow-on implementation has concrete layout/allocation evidence to
collect and explicit mechanisms to exclude.
