# Source Scan Summary

Evidence class: Static source scans.

## Commands

Core carrier/runtime scan:

```bash
PAT='HillslopeWritebackSurface|HillslopeKernelRequest|KernelWritebackPayload|SymbolRegistry|HillslopePhaseScheduler|HillslopeDayFrame|HotSymbolTables|IndexedWritebackSurface|IndexedKernelWritebackPayload|execute_hillslope_climate_days|execute_persistent_scheduler_kernel_lifecycle|state_value_for_symbol|flux_value_for_symbol|dirty_state_ids|dirty_flux_ids'
rg -o "$PAT" crates/openwepp-runner/src crates/openwepp-hillslope-orchestrator/src crates/openwepp-kernel-contract/src --glob '*.rs' | wc -l
rg -l "$PAT" crates/openwepp-runner/src crates/openwepp-hillslope-orchestrator/src crates/openwepp-kernel-contract/src --glob '*.rs' | wc -l
```

Boundary serialization scan:

```bash
rg -o 'BoundarySymbol' crates/openwepp-runner/src crates/openwepp-hillslope-orchestrator/src crates/openwepp-kernel-contract/src --glob '*.rs' | wc -l
rg -o 'BoundaryValue' crates/openwepp-runner/src crates/openwepp-hillslope-orchestrator/src crates/openwepp-kernel-contract/src --glob '*.rs' | wc -l
rg -l 'BoundarySymbol|BoundaryValue' crates/openwepp-runner/src crates/openwepp-hillslope-orchestrator/src crates/openwepp-kernel-contract/src --glob '*.rs' | wc -l
```

Public selector absence scan:

```bash
rg -n "HillslopeRuntimeSelection::Compatibility|HillslopeDefaultRuntimeActivation::Disabled|--compatibility-runtime|default-candidate-disabled|explicit-deprecated-compatibility-selection" crates tools --glob '*.rs' --glob '*.py'
```

## Totals

| Surface | Count |
| --- | ---: |
| Core carrier/runtime matches | 1,284 |
| Core files with matches | 74 |
| `BoundarySymbol` matches | 2,557 |
| `BoundaryValue` matches | 1,580 |
| Boundary symbol/value files | 84 |
| Removed public selector tokens under `crates/` and `tools/` | 0 |

## Core Term Counts

| Term | Matches |
| --- | ---: |
| `HillslopeWritebackSurface` | 344 |
| `HillslopeKernelRequest` | 275 |
| `KernelWritebackPayload` | 114 |
| `SymbolRegistry` | 277 |
| `HillslopePhaseScheduler` | 56 |
| `HillslopeDayFrame` | 58 |
| `HotSymbolTables` | 32 |
| `IndexedWritebackSurface` | 56 |
| `IndexedKernelWritebackPayload` | 19 |
| `execute_hillslope_climate_days` | 3 |
| `execute_persistent_scheduler_kernel_lifecycle` | 2 |
| `state_value_for_symbol` | 27 |
| `flux_value_for_symbol` | 8 |
| `dirty_state_ids` | 16 |
| `dirty_flux_ids` | 16 |

## Matching Lines By Area

These are matching source lines, not distinct token occurrences.

| Area | Matching lines | Interpretation |
| --- | ---: | --- |
| Scheduler support | 381 | Compiled scheduler/day-frame/lifecycle support remains as a unit. |
| Tests | 295 | Tests still exercise symbol-boundary and scheduler-era behavior. |
| Hydrology boundary | 248 | Kernel phases and support helpers still use request/writeback carriers. |
| Kernel contract | 127 | Symbol-boundary API definitions and tests remain in `openwepp-kernel-contract`. |
| Runner setup/publication | 126 | Manifest, WB13, diagnostics, and publication support still import carriers. |
| Other | 41 | Audit, watershed CLI, or miscellaneous support. |
| Runtime inputs | 27 | Intake/projection adapters still use symbol-boundary helpers. |

## Highest-Count Core Files

| Matches | File |
| ---: | --- |
| 162 | `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` |
| 122 | `crates/openwepp-hillslope-orchestrator/src/day_frame.rs` |
| 111 | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs` |
| 72 | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs` |
| 62 | `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs` |
| 31 | `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` |
| 31 | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/phase.rs` |
| 30 | `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/boundaries.rs` |
| 27 | `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` |
| 27 | `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs` |
| 26 | `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/03_scheduler_lifecycle.rs` |
| 26 | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs` |

## Interpretation

The core surface is concentrated enough to sequence, but it spans more than the
obsolete scheduler executor. The highest production-impact groups are:

- scheduler/day-frame runtime support;
- hydrology phase request/writeback carriers;
- trace and WB13/publication helpers;
- tests that lock symbol-boundary behavior.

`BoundarySymbol` and `BoundaryValue` are lower-level serialization and guard
tokens. They should not be blanket-deleted in the first follow-on because some
are genuine intake/output or contract-test edges.
