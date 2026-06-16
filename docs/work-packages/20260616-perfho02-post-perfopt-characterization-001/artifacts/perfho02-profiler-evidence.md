# PERFHO02 Profiler Evidence

Status: COMPLETE 2026-06-16
Evidence mode: **Ran** (release build, profiler probe, `perf` sampling/stat, GDB stack sampling) + **Static** (source-path attribution)

## Build And Environment

Ran:

```text
RUSTFLAGS='-C force-frame-pointers=yes -C debuginfo=1' cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result:

```text
Finished release profile [optimized] target(s) in 0.98s
```

H2637 staged inputs were present under `/tmp/perfho01/run-dirs/h2637`.

## Profiler Availability

Initial scaffold probe:

```text
perf stat -e task-clock,cycles,instructions -- target/release/openwepp-cli-hill --help
```

Result at first execution: blocked with exit code `255` by `perf_event_paranoid=4`; PERFHO02 used the same GDB user-space sampling fallback as PERFHO01 and PERFOPT01.

After the host sysctl was changed, this session saw:

```text
cat /proc/sys/kernel/perf_event_paranoid
0
```

Ran:

```text
perf stat -e task-clock,cycles,instructions -- target/release/openwepp-cli-hill --help
```

Result:

```text
code=0
1.34 msec task-clock
3009899 cycles
1994835 instructions
```

`perf` is now available for user-space profiling in this session. Kernel address maps remain restricted by `kernel.kptr_restrict=1`, so kernel samples may appear unresolved, but openWEPP user-space symbol attribution is available.

## Perf Sampling Supplement

Ran:

```text
perf record -F 99 --call-graph fp -o /tmp/perfho02/perf-h2637-post-perfopt.data -- timeout --signal=INT 90s \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /home/workdir/openWEPP/docs/work-packages/20260616-perfho02-post-perfopt-characterization-001/artifacts/runfiles/perfho02-h2637.run \
  --output-dir /tmp/perfho02/outputs/h2637_perf \
  --policy compat --legacy-sidecar-discovery
```

Result: timeout exit `124` expected for the bounded sample window; `perf record` captured and wrote `9,586` samples.

Artifacts:

```text
/tmp/perfho02/perf-h2637-post-perfopt.data
/tmp/perfho02/perf-h2637-report-nochildren.txt
/tmp/perfho02/perf-h2637-report-children.txt
```

No lost samples were reported.

Top no-children user-space rows:

| Overhead | Symbol | Interpretation |
|---:|---|---|
| 11.55% | `__memcmp_sse2` | string-key/BTreeMap comparison work across writeback, scheduler, decomposition, runoff reconciliation, and PL selection |
| 7.11% | `ensure_no_overflow_indexed_symbols_for_decomposition` | decomposition guard scan over symbol-keyed state |
| 5.46% | `_int_free` | allocator/free cost, including `HillslopeWritebackSurface` drop |
| 5.28% | `_int_malloc` | allocation cost, including `BTreeMap::clone_subtree`, writeback, and string formatting |

Top children rows from `perf report --children`:

| Children | Self | Symbol / path | Interpretation |
|---:|---:|---|---|
| 96.24% | 0.00% | `execute_persistent_scheduler_kernel_lifecycle` | H2637 remains dominated by per-OFE-day scheduler lifecycle |
| 69.67% | 0.00% | `HillslopePhaseScheduler::execute_with_kernel` | kernel phase scheduler dominates inside lifecycle |
| 41.14% | 0.00% | `Wb11HydrologyKernel::run_hillslope_phase` | hydrology kernel phases dominate scheduler work |
| 22.40% | 0.00% | `run_runoff_reconciliation` | runoff reconciliation plus frost coupling is the largest hydrology subpath in this window |
| 12.46% | 0.00% | `apply_kernel_writeback` | writeback application sort/insert/allocation remains significant |
| 12.35% | 0.00% | `compute_active_frost_coupling` | frost coupling symbol access/formatting remains significant |
| 11.02% | 0.00% | `decomposition_phase_dispatch_for_state` | decomposition dispatch and guard scans remain significant |
| 7.48% | 0.00% | `ensure_no_overflow_indexed_symbols_for_decomposition` | direct guard-scan target |
| 5.51% | 0.00% | `BTreeMap::clone_subtree` | remaining lane-surface clone residual |
| 4.46% | 0.00% | `seed_wb11_runtime_surface_inputs` | scheduler seeding symbol writes remain visible |
| 3.80% | 0.00% | `drop_in_place<HillslopeWritebackSurface>` | remaining surface drop cost |

Ran bounded hardware-counter stat:

```text
perf stat -e task-clock,cycles,instructions,branches,branch-misses,cache-references,cache-misses -- timeout --signal=INT 30s target/release/openwepp-cli-hill ...
```

Result:

```text
29996.74 msec task-clock # 1.000 CPUs utilized
96864877021 cycles
186027365950 instructions # 1.92 insn per cycle
35637583422 branches
673027617 branch-misses # 1.89% of all branches
939886047 cache-references
28841012 cache-misses # 3.07% of all cache refs
30.006451816 seconds time elapsed
29.957934000 seconds user
0.043879000 seconds sys
```

## GDB Sampling Command Shape

Ran:

```text
gdb -q target/release/openwepp-cli-hill
set pagination off
set confirm off
set print frame-arguments none
set backtrace limit 20
set logging file /tmp/perfho02/gdb-h2637-post-perfopt.txt
set logging overwrite on
set logging enabled on
set args --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /home/workdir/openWEPP/docs/work-packages/20260616-perfho02-post-perfopt-characterization-001/artifacts/runfiles/perfho02-h2637.run \
  --output-dir /tmp/perfho02/outputs/h2637_gdb \
  --policy compat --legacy-sidecar-discovery
run
```

Raw log:

```text
/tmp/perfho02/gdb-h2637-post-perfopt.txt
```

GDB transcript size: `408` lines, `36K`. The log contains 20 `thread apply all bt 20` backtrace samples plus one final interrupt used only to kill the inferior. The final kill interrupt is excluded from the table below.

## Sample Attribution

Sample window: 20 manual samples on optimized H2637 steady-state execution. Shares are stack-sample percentages, not hardware-cycle percentages.

| Category | Samples | Share | Representative stacks |
|---|---:|---:|---|
| Hydrology typed-symbol lookup, dynamic symbol formatting, frost/decomposition/PL guards | 13/20 | 65.0% | `wb19_load_layer_state`, `compute_active_frost_coupling`, `frost_fine_layer_symbol`, `require_shadow_fine_state_domains`, `resolve_active_pl_slot_selection`, `require_integral_pl_dispatch_symbol_in_range`, `ensure_no_overflow_indexed_symbols_for_decomposition`, `resolve_erod14_wave2_enabled` |
| Kernel writeback application sort/alloc/insert | 4/20 | 20.0% | `apply_kernel_writeback` via `BTreeMap::insert`, `malloc`, and `core::slice::sort::stable::quicksort` |
| Scheduler/runtime-surface insertion and outer daily-loop allocation | 2/20 | 10.0% | `BTreeMap::insert` under `execute_persistent_scheduler_kernel_lifecycle`; `malloc` under `execute_hillslope_climate_days` |
| Consumer-boundary validation | 1/20 | 5.0% | `validate_hillslope_consumer_boundary` under `HillslopePhaseScheduler::execute_with_kernel` |
| Output writers / Parquet | 0/20 | 0.0% | No writer sample |

## Raw Sample Index

| Sample | Top useful frame(s) | Classification |
|---:|---|---|
| 1 | `wb19_load_layer_state` | Hydrology typed-symbol lookup |
| 2 | `compute_active_frost_coupling` -> `compute_same_pass_wb14_infiltration_lineage` | Frost coupling symbol access |
| 3 | `BTreeMap::insert` -> `apply_kernel_writeback` | Writeback application insertion |
| 4 | `validate_hillslope_consumer_boundary` | Consumer-boundary validation |
| 5 | `malloc` -> `apply_kernel_writeback` | Writeback allocation |
| 6 | `require_integral_pl_dispatch_symbol_in_range` -> `resolve_active_pl_slot_selection` | PL dispatch symbol lookup |
| 7 | `format` -> `frost_fine_layer_symbol` -> `run_runoff_reconciliation` | Frost fine-layer symbol formatting |
| 8 | `ensure_no_overflow_indexed_symbols_for_decomposition` | Decomposition guard scan |
| 9 | `require_integral_pl_dispatch_symbol_in_range` -> `resolve_active_pl_slot_selection` | PL dispatch symbol lookup |
| 10 | `format` -> `compute_active_frost_coupling` -> `run_runoff_reconciliation` | Frost coupling symbol formatting |
| 11 | `Vec::push_mut` -> `run_runoff_reconciliation` | Hydrology runoff reconciliation allocation |
| 12 | `realloc` -> `format` -> `require_shadow_fine_state_domains` -> `compute_active_frost_coupling` | Frost guard symbol formatting |
| 13 | `malloc` -> `format` -> `resolve_active_pl_slot_selection` | PL dispatch formatting/allocation |
| 14 | `malloc` -> `execute_hillslope_climate_days` | Outer daily-loop allocation |
| 15 | `malloc` -> `resolve_erod14_wave2_enabled` -> `run_erod19_route_segment_migration` | EROD14 Wave-2 enablement lookup/allocation |
| 16 | `compute_active_frost_coupling` -> `run_runoff_reconciliation` | Frost coupling symbol access |
| 17 | `realloc` -> `format` -> `frost_fine_layer_symbol` -> `run_runoff_reconciliation` | Frost fine-layer symbol formatting |
| 18 | `core::slice::sort::stable::quicksort` -> `apply_kernel_writeback` | Writeback sorting |
| 19 | `BTreeMap::insert` -> `execute_persistent_scheduler_kernel_lifecycle` | Runtime-surface insertion |
| 20 | `realloc` -> `format` -> `require_shadow_fine_state_domains` -> `compute_active_frost_coupling` | Frost guard symbol formatting |

## Source Attribution

Static source locations checked:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/00_pl_slot_resolution.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`
- `crates/openwepp-hillslope-orchestrator/src/consumer_boundary.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/writeback.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`

PERFHO02 confirms with both GDB and `perf` evidence that the dominant residual remains CPU-bound symbol-keyed metadata/control work, not output writing.
