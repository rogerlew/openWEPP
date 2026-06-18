# PERFARCH02 Contract Blast Radius

Evidence class: Static inventory from `rg` and source inspection on
2026-06-18. No production files were edited in this package.

## Contract Definitions

- `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs`
  - `SymbolId`, `SymbolRegistry`, `IndexedSurface`, and
    `IndexedWritebackSurface` are the ADR-0022 foundation.
  - `WritebackField`, `KernelWritebackPayload`, and `KernelRunResponse` still
    expose logical-symbol writeback payloads.
  - `KernelRequest` already has optional indexed read surfaces; the next design
    would need an array-authoritative mutable/writeback surface instead of an
    optional read mirror.
- `crates/openwepp-kernel-contract/src/lib_mod/writeback.rs`
  - `evaluate_kernel_writeback` and `apply_kernel_writeback` are the current
    logical payload evaluation and map application seam.
  - A migration needs id-backed equivalents with the same finite/domain status
    semantics and lazy logical subject resolution on failure.

## Scheduler State And Application

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  - `OfeLaneExecutionInput` and lane state carry both `writeback_surface` and
    `indexed_writeback_surface`.
  - `execute_with_kernel_indexed` still evaluates/apply logical writebacks and
    then refreshes the indexed mirror, which is the PERFIDX05 dual-write cost.
  - Transfer helpers insert/remove logical `BoundarySymbol` values and then
    update the indexed mirror.
  - Consumer-boundary validation reads logical maps.

The array-authoritative migration has to invert this ownership: lanes carry
dense hot state as the mutable authority, and logical maps are materialized only
when validation/publication requires them.

## Runner And Publication

- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  - builds the run-scoped registry and activates indexed writeback authority
    from logical runtime surfaces.
  - this is the natural seed point for `ArrayHotState`.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`
  - owns persistent scheduler lifecycle, clones/moves lane surfaces, refreshes
    indexed authority, and rebuilds outlet runtime surfaces.
  - this file likely needs the highest care in a staged migration.
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
  - many row builders accept `&HillslopeWritebackSurface`.
  - these should become publication-boundary consumers of a materialized view,
    not hot-loop readers.
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
  - watershed CLI reads `report.writeback_surface` for output/reporting.
  - this should stay a logical/reporting surface after materialization.

## Kernel Producers

The following production files produce `KernelWritebackPayload` or
`WritebackField` values that would need id-backed writeback equivalents or
direct array-slot writes:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/04_kernel_execution.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_infiltration_evap.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod13.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod14.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_peak_runoff.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_storage_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/validation.rs`

## Test Blast Radius

Tests that directly construct or inspect `HillslopeWritebackSurface`,
`KernelWritebackPayload`, or `WritebackField` remain valuable identity guards.
They should not all be rewritten at once. The staged migration should add
array-vs-logical identity tests beside existing logical tests, then narrow
logical tests to contract/publication surfaces after the flip.

High-signal suites include:

- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/hydrology.rs`
- `tests/integration/clim05_snow_runtime_kernel_contract.rs`
- `tests/integration/clim06_frost_frozen_soil_kernel_contract/`
- `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`
- `tests/integration/parser_runtime_seam_integration/`
- `tests/integration/ws10_watershed_kernel_contract.rs`

## Line-Count Governance

Files already above the 2,000-line caution threshold and likely in scope:

| File | Lines |
|---|---:|
| `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs` | 2,671 |
| `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` | 2,452 |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 2,410 |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs` | 2,672 |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | 2,095 |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 2,062 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs` | 2,219 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs` | 2,549 |

No listed file is above 3,000 lines, but any downstream implementation should
avoid expanding these files further without extraction.

## Required Migration Guardrails

- Keep logical output surfaces bit-identical at every staged flip.
- Preserve fail-closed finite/domain writeback semantics and message ids.
- Keep `SymbolRegistry` frozen; no lazy hot-loop symbol interning.
- Add source-level guards that prevent reintroducing per-day full-map export or
  logical + array dual-write in normal timing paths.
- Run H2637 timing and representative contract suites before each flip.
