# PERFHO01 Profiling Evidence

Status: COMPLETE 2026-06-16
Evidence mode: **Ran** (build, profiler availability probe, GDB stack sampling) + **Static** (source-path attribution)

## Build And Environment

- Repository: `/home/workdir/openWEPP`
- HEAD: `d6cb4ef9`
- Binary: `target/release/openwepp-cli-hill`
- Build command:
  `RUSTFLAGS='-C force-frame-pointers=yes -C debuginfo=1' cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
- Build result: `Finished release profile [optimized] target(s) in 1m 46s`
- Measurement staging root: `/tmp/perfho01`
- Package runfiles: `artifacts/runfiles/`

## Profiler Selection

`perf` is installed, but unusable in this environment:

```text
Ran: perf stat -e task-clock,cycles,instructions -- target/release/openwepp-cli-hill --help
Result: blocked by perf_event_paranoid=4; no CAP_PERFMON/CAP_SYS_ADMIN access.
```

`valgrind`, `cargo-flamegraph`, and related profilers were not available. GDB was
available and was used as a user-space statistical sampler: the H2637 19-OFE run
was started under GDB, interrupted at fixed intervals during steady-state
execution, backtraced with `thread apply all bt 18`, continued, then killed
before output publication. GDB reported no DWARF line info for the release
binary, but release symbol names were available and sufficient to attribute
module/function stacks.

Command shape:

```text
gdb -q target/release/openwepp-cli-hill
set pagination off
set confirm off
set print frame-arguments none
set backtrace limit 18
set args --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /home/workdir/openWEPP/docs/work-packages/20260616-perf-high-ofe-hillslope-characterization-001/artifacts/runfiles/perfho01-h2637.run \
  --output-dir /tmp/perfho01/outputs/h2637_gdb \
  --policy compat --legacy-sidecar-discovery
run
```

## GDB Sample Attribution

Sample window: 15 manual samples, approximately 5 seconds apart, on the H2637
19-OFE daily steady-state execution loop. Percentages below are stack-sample
percentages, not hardware-cycle percentages.

| Category | Samples | Share | Representative stacks |
|---|---:|---:|---|
| Symbol-keyed runtime-surface map lifecycle/access | 8/15 | 53.3% | `BTreeMap::clone_subtree`, `BTreeMap::insert`, `BTreeMap::remove`, `require_state_scalar_for_symbol`, `compute_active_frost_coupling`, `OfeLanePersistentStateSequence::replace_from_report`, all under `execute_persistent_scheduler_kernel_lifecycle` |
| Kernel writeback validation/sort/allocation/detail construction | 3/15 | 20.0% | `apply_kernel_writeback` via `core::slice::sort`, `apply_kernel_writeback` allocation, `collect_field_violations` / `evaluate_kernel_writeback` formatting |
| Hydrology/frost/decomposition guard and symbol formatting overhead outside writeback | 4/15 | 26.7% | `ensure_no_overflow_indexed_symbols_for_decomposition`, `require_shadow_fine_state_domains`, `hourly_symbol`, `compute_same_pass_wb14_infiltration_lineage` |

Raw sample classification:

| Sample | Top useful frame(s) | Classification |
|---:|---|---|
| 1 | `apply_kernel_writeback` -> `core::slice::sort::stable::*` -> `__memcmp_sse2` | Writeback validation/sort |
| 2 | `require_state_scalar_for_symbol` -> `wb19_load_layer_state` -> `__memcmp_sse2` | Symbol-keyed state lookup |
| 3 | `OfeLanePersistentStateSequence::replace_from_report` -> drop `HillslopeWritebackSurface` -> `free` | Runtime-surface lifecycle/drop |
| 4 | `ensure_no_overflow_indexed_symbols_for_decomposition` | Hydrology decomposition guard scan |
| 5 | `apply_kernel_writeback` -> `malloc` | Writeback allocation |
| 6 | `require_shadow_fine_state_domains` -> `format` -> `String::write_str` -> `realloc` | Frost guard string formatting |
| 7 | `BTreeMap::clone_subtree` -> `execute_persistent_scheduler_kernel_lifecycle` | Runtime-surface clone |
| 8 | `compute_active_frost_coupling` -> `__memcmp_sse2` | Symbol-keyed frost lookup |
| 9 | `hourly_symbol` -> `format` | Symbol string formatting |
| 10 | `collect_field_violations` -> `evaluate_kernel_writeback` -> `format` | Writeback validation detail construction |
| 11 | `compute_active_frost_coupling` -> `__memcmp_sse2` | Symbol-keyed frost lookup |
| 12 | `BTreeMap::remove` -> `execute_persistent_scheduler_kernel_lifecycle` | Runtime-surface map removal |
| 13 | `BTreeMap::clone_subtree` -> `malloc` -> `execute_persistent_scheduler_kernel_lifecycle` | Runtime-surface clone/allocation |
| 14 | `compute_same_pass_wb14_infiltration_lineage` -> `free` | Hydrology phase allocation/free |
| 15 | `BTreeMap::insert` -> `seed_mofe03_wave2_case_state` -> `seed_wb11_runtime_surface_inputs` | Runtime-surface seed insertion |

## Attribution Verdict

The H2637 hot path is not dominated by Parquet writers or filesystem I/O during
the steady-state daily loop. The full H2637 timing also shows user CPU dominance:
`elapsed_s=978.55`, `user_s=977.99`, `sys_s=0.42`.

Profiler-backed dominant cost:

1. Repeated symbol-keyed runtime-surface `BTreeMap` access, cloning, insertion,
   removal, and replacement inside the persistent per-OFE scheduler path.
2. Kernel writeback validation that sorts/allocates/constructs detail on the
   success path.
3. Hydrology/frost guard and symbol formatting work that is repeatedly performed
   during normal daily execution.

The package lead about eager per-OFE WB13 detail strings remains plausible but
was not the sampled dominant path in this H2637 window. No GDB sample stopped in
`DailyInternalPerOfeWb13Collection::from_sequence_report`,
`scan_internal_identity_terms`, or Parquet writing. The sampled bottleneck is
broader: string-keyed runtime-surface metadata/control overhead in the daily
per-OFE execution loop.
