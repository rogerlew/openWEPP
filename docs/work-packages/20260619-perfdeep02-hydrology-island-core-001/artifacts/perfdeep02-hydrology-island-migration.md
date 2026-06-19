# PERFDEEP02 Hydrology Island Migration

Evidence class: Static + Ran.

## Implementation Summary

PERFDEEP02 implemented the Stage-1 hydrology island mechanics over the
slot-backed `HillslopeDayFrame` for the contiguous hydrology cluster, but did
not leave the island enabled by default because the H2637 endpoint gate failed.
The production opt-in is `OPENWEPP_PERFDEEP02_FRAME_ISLAND=1`.

- `HillslopeKernelRequest` now accepts optional dense state/flux slots and
  resolves `indexed_state_value` / `indexed_flux_value` from dense slots before
  falling back to the indexed mirror.
- `HotSymbolTables::from_registry` remains limited to canonical hot symbols;
  the temporary full-registry exact-scalar expansion was removed because it
  added hot-path lookup cost without producing a measured endpoint win.
- `HillslopeDayFrame` can apply logical and id-backed kernel writeback payloads
  directly into dense slots and flush dirty `SymbolId`s back to the logical
  compatibility surface.
- `HillslopePhaseScheduler::execute_with_kernel_indexed` can seed a frame at
  the first PERFDEEP02 island phase, preferring direct seeding from the existing
  `IndexedWritebackSurface`, lend dense slot slices to island kernel requests,
  apply accepted island writebacks into the frame, and flush dirty ids at the
  island boundary.
- Hydrology scalar access now routes through dense-first helpers before logical
  map fallback. A static scan leaves only the intentional fallback reads in
  `state_access.rs`.

## Island Phases

The frame-authoritative island covers:

- `PercolationDeepSeepage`
- `Evapotranspiration`
- `Drainage`
- `LateralTransfer`
- `PlantRootUptake`
- `RunoffReconciliation`
- `StorageReconciliation`
- `ClosureDiagnostics`

Non-island phases continue to use the existing logical/indexed request surface.
Outside tests, this island is disabled unless
`OPENWEPP_PERFDEEP02_FRAME_ISLAND=1` is set.

## Identity Coverage

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator perfdeep0 -- --nocapture
```

Result: passed, 5 tests.

The new scheduler regression
`perfdeep02_scheduler_runs_hydrology_island_through_dense_slots_without_indexed_mirror`
executes `execute_with_kernel_indexed` with no `IndexedWritebackSurface`,
asserts non-island requests receive no dense slots, asserts all 8 island phases
receive dense slots and no indexed mirror, applies an id-backed writeback in
`PercolationDeepSeepage`, observes it from dense slots in `Evapotranspiration`,
and verifies the final flushed logical surface.

Production endpoint evidence did not support enabling this path:

- A pre-final dense island run with whole-frame logical flush measured
  `2417.14 s`, `235700 KB` and was replaced.
- A deferred dirty-id flush attempt was terminated after more than `23:36`
  elapsed, already more than 2x the PERFDEEP01 `669.97 s` endpoint.
- A direct indexed-frame seeding attempt was terminated after `25:27`, also
  more than 2x the PERFDEEP01 endpoint.

The blocker is frame lifecycle cost in the per-day/per-OFE loop. A follow-on
must carry dense frame state persistently across lane days, or otherwise avoid
re-seeding/copying full frame surfaces before production activation.

Static review:

```text
rg -n "request\.(state_surface|flux_surface)\.(get|contains_key)" crates/openwepp-hillslope-orchestrator/src/hydrology
```

Result: only the intentional dense-first fallback reads in
`support_helpers_mod/state_access.rs` remain.

## Non-Goals Preserved

- No output schema changes.
- No typed-field frame promotion.
- No fallback wrapper that masks missing dependencies.
- No science/numeric formula changes were intended; changes are routing and
  authority-path changes for existing boundary values and writebacks.
