# PERFDEEP04 Profile Results

Evidence class: Ran + Static.

## Verdict

The PERFDEEP03 no-go is dominated by daily lane-dense resynchronization and
compatibility-edge work, not by dense array arithmetic.

`HillslopeLaneDenseState::sync_from_writeback_surface` is the leading
PERFDEEP03-specific hotspot:

```text
33.49% children, 14.19% self
```

That symbol is absent from the default-disabled comparison profile.

## Commands

Release binary:

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result: passed.

Opt-in PERFDEEP03 lane-dense H2637 profile:

```text
/usr/bin/time -f "perfdeep04_h2637_optin_perf_record\t%e\t%M" \
  perf record -F 49 --call-graph dwarf,8192 \
  -o /tmp/perfdeep04/profile/perfdeep04-h2637-optin.data \
  -- env OPENWEPP_PERFDEEP03_LANE_DENSE_STATE=1 \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /home/workdir/openWEPP/docs/work-packages/20260619-perfdeep04-profile-perfdeep03-lane-dense-no-go-001/artifacts/runfiles/perfdeep04-h2637.run \
  --output-dir /tmp/perfdeep04/current/h2637_same_manifest \
  --policy compat \
  --legacy-sidecar-discovery
```

Default-disabled comparison profile:

```text
/usr/bin/time -f "perfdeep04_h2637_default_perf_record\t%e\t%M" \
  perf record -F 49 --call-graph dwarf,8192 \
  -o /tmp/perfdeep04/profile/perfdeep04-h2637-default.data \
  -- target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /home/workdir/openWEPP/docs/work-packages/20260619-perfdeep04-profile-perfdeep03-lane-dense-no-go-001/artifacts/runfiles/perfdeep04-h2637-default.run \
  --output-dir /tmp/perfdeep04/default/h2637_same_manifest \
  --policy compat \
  --legacy-sidecar-discovery
```

Both runs emitted the existing scoped sidecar warning:

```text
MOFE01-MG-W-001 EROD14 Wave-2 qin is seeded from water-transfer provenance only; true sediment-coupled qin/qout and particle-fraction handoff remains MOFE01 M-G follow-on scope.
```

## Run Summary

| Profile | Elapsed s | Max RSS KB | Samples | Lost samples | perf.data |
|---|---:|---:|---:|---:|---:|
| PERFDEEP03 opt-in | 1164.31 | 519160 | 61248 | 0 | 492.932 MB |
| Default disabled | 704.82 | 320640 | 37051 | 0 | 298.207 MB |

Matched profiling overhead preserves the PERFDEEP03 no-go shape:

```text
opt-in / default = 1.652x
opt-in - default = 459.49 s
```

The binary `perf.data` files are in `/tmp/perfdeep04/profile/`. Repository
artifacts keep text reports only:

- `raw/perfdeep04-h2637-optin-flat-top.txt`
- `raw/perfdeep04-h2637-optin-children-top.txt`
- `raw/perfdeep04-h2637-optin-children-report.txt`
- `raw/perfdeep04-h2637-optin-header.txt`
- `raw/perfdeep04-h2637-default-flat-top.txt`
- `raw/perfdeep04-h2637-default-children-top.txt`
- `raw/perfdeep04-h2637-default-children-report.txt`
- `raw/perfdeep04-h2637-default-header.txt`

Kernel symbols are partially unresolved because `/proc/kallsyms` is restricted,
but user-space Rust symbols resolved and are sufficient for this package.

## Opt-In Top Costs

Top inclusive symbols from `raw/perfdeep04-h2637-optin-children-top.txt`:

| Children | Self | Symbol |
|---:|---:|---|
| 51.05% | 0.15% | `execute_persistent_scheduler_kernel_lifecycle` |
| 44.24% | 0.00% | `ClimateExecutionAccumulator::apply_hillslope_day` |
| 33.49% | 14.19% | `HillslopeLaneDenseState::sync_from_writeback_surface` |
| 29.03% | 0.02% | `HillslopePhaseScheduler::execute_with_kernel_indexed_internal` |
| 21.84% | 19.94% | `__memcmp_sse2` |
| 8.09% | 0.27% | `HillslopeLaneDenseState::apply_kernel_writeback_payload` |
| 8.08% | 0.17% | `run_runoff_reconciliation` |
| 7.34% | 0.01% | `decomposition_phase_dispatch_for_state_indexed` |
| 5.86% | 3.43% | `SymbolRegistry::id_of` |
| 5.67% | 0.38% | `HillslopeLaneDenseState::flush_dirty_to_writeback_surface` |
| 4.67% | 0.14% | `HotSymbolTables::hot_state_symbols` |
| 4.32% | 0.16% | `perfdeep02_apply_logical_frame_writeback` |
| 3.80% | 1.10% | `state_value_for_symbol` |

Flat top costs from `raw/perfdeep04-h2637-optin-flat-top.txt`:

| Self | Symbol |
|---:|---|
| 19.94% | `__memcmp_sse2` |
| 14.19% | `HillslopeLaneDenseState::sync_from_writeback_surface` |
| 4.87% | `ensure_no_overflow_indexed_symbols_for_decomposition` |
| 3.94% | `_int_malloc` |
| 3.43% | `SymbolRegistry::id_of` |
| 3.01% | `_int_free` |
| 3.00% | `malloc` |
| 2.61% | `BTreeMap::insert` |
| 2.28% | `HotSymbolTables::state_grid_symbol` |

## Default Top Costs

Top inclusive symbols from `raw/perfdeep04-h2637-default-children-top.txt`:

| Children | Self | Symbol |
|---:|---:|---|
| 44.15% | 0.13% | scheduler phase closure |
| 34.95% | 0.08% | `HillslopePhaseScheduler::execute_with_kernel_indexed_internal` |
| 20.61% | 0.52% | `compute_active_frost_coupling` |
| 20.18% | 0.23% | `execute_persistent_scheduler_kernel_lifecycle` |
| 19.29% | 0.00% | `ClimateExecutionAccumulator::apply_hillslope_day` |
| 18.60% | 17.10% | `__memcmp_sse2` |
| 17.15% | 0.26% | `run_runoff_reconciliation` |
| 15.63% | 0.68% | `apply_kernel_writeback` |
| 14.83% | 8.85% | `state_value_for_symbol` |
| 11.52% | 0.02% | `decomposition_phase_dispatch_for_state_indexed` |
| 9.18% | 7.72% | `ensure_no_overflow_indexed_symbols_for_decomposition` |
| 7.95% | 3.93% | `BTreeMap::insert` |

Default confirms the baseline is still map/string/symbol heavy, but it does not
have the lane-dense resync hotspot.

## Mechanism Classification

1. **PERFDEEP03-specific resync dominates.** `sync_from_writeback_surface`
   accounts for one third of opt-in inclusive cycles. The code path is called
   after transfer input is first applied to logical/indexed surfaces and then
   re-copied into dense slots.
2. **Hot-symbol metadata is rebuilt in the hot loop.** `hot_state_symbols`
   appears inside the sync call. The current helper returns a fresh sorted
   vector, causing allocation and sort/dedup work during H2637 lane execution.
3. **Symbol lookups survived in dense writeback apply.** The opt-in path still
   spends `5.86%` inclusive in `SymbolRegistry::id_of`, largely under
   `HillslopeLaneDenseState::apply_kernel_writeback_payload`. That is not a
   dense-id application path.
4. **Boundary BTreeMap work remains material.** `flush_dirty_to_writeback_surface`
   and `perfdeep02_apply_logical_frame_writeback` still insert/sort through
   logical surfaces in the lane-dense opt-in path.
5. **Dense reads helped, but not enough.** Default `state_value_for_symbol` is
   `14.83%` inclusive; opt-in falls to `3.80%`. The dense read win is real, but
   the sync/apply/flush compatibility edge more than consumes it.

## Static Pointers

Relevant current code:

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:2092` calls
  `sync_from_writeback_surface` after `apply_transfer_input_to_lane_surface`.
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs:197` implements
  `sync_from_writeback_surface`.
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs:204` calls
  `hot_symbol_tables.hot_state_symbols()` in the sync path.
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs:217` clones
  `state_slot_ids` before repopulating slots.
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs:270` calls
  `symbol_registry.id_of` for logical writeback payloads.
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:1990` flushes dirty
  dense slots back to logical/indexed surfaces at scheduler exit.

## Interpretation

PERFDEEP03 did not fail because dense arrays are intrinsically impractical. It
failed because the lane-dense state is not yet authoritative at the transfer and
publication compatibility edges. The implementation updates logical/indexed
surfaces, then re-synchronizes dense state from them, then flushes back to those
surfaces. That is a second compatibility boundary wrapped around the new dense
state.

The immediate fix is not a broader whole-simulation dense array. The next
package should remove the measured resync path first and prove whether the
partial island can become endpoint-flat or positive once dense state is actually
the carried authority.
