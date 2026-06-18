# PERFIDX06 profiler evidence

Evidence: Ran.

## Command

Ran:

```text
perf record -F 99 --call-graph fp \
  -o /tmp/perfidx06/perf-h2637-endpoint.data \
  -- timeout --signal=INT 90s \
  /tmp/perfidx04/current/bin/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfidx06/runfiles/h2637_same_current.run \
  --output-dir /tmp/perfidx06/current/h2637_perf_manifest \
  --policy compat \
  --legacy-sidecar-discovery
```

The timeout exit code was `124`, expected for the profiling window. `perf` captured
`9,547` samples, `0` lost samples, and wrote
`/tmp/perfidx06/perf-h2637-endpoint.data`.

Kernel symbols were restricted by `kernel.kptr_restrict = 1`, but user-space Rust symbols
resolved. This matches the package need: the hot path is openWEPP user-space execution.

Reports:

```text
perf report --stdio -i /tmp/perfidx06/perf-h2637-endpoint.data --no-children \
  > /tmp/perfidx06/perf-h2637-endpoint-report-nochildren.txt
perf report --stdio -i /tmp/perfidx06/perf-h2637-endpoint.data --children \
  > /tmp/perfidx06/perf-h2637-endpoint-report-children.txt
```

## Direct Samples

Top no-children entries:

| Self | Symbol |
| ---: | --- |
| 17.07% | `__memcmp_sse2` |
| 7.70% | `ensure_no_overflow_indexed_symbols_for_decomposition` |
| 6.62% | unresolved kernel sample |
| 4.50% | `BTreeMap<K,V,A>::insert` under writeback |
| 4.25% | `HotSymbolTables::state_grid_symbol` |
| 3.75% | `malloc` |
| 3.72% | `_int_free` |
| 3.26% | `optional_state_scalar_for_indexed_symbol` |
| 3.06% | `_int_malloc` |
| 2.96% | `require_state_scalar_for_symbol` |
| 2.34% | `__memmove_sse2_unaligned_erms` |
| 2.33% | `BTreeMap<K,V,A>::insert` under scheduler/runtime surface |
| 2.22% | `compute_active_frost_hourly_state` |
| 2.03% | `core::slice::sort::stable::quicksort::quicksort` |
| 1.97% | `runtime_surface_symbol_value` |
| 1.95% | `BTreeMap<K,V,A>::remove` |

## Children Split

Key children rows:

| Children | Self | Symbol |
| ---: | ---: | --- |
| 98.40% | 0.29% | `execute_persistent_scheduler_kernel_lifecycle` |
| 76.17% | 0.10% | `HillslopePhaseScheduler::execute_with_kernel_indexed` |
| 41.62% | 0.16% | `Wb11HydrologyKernel::run_hillslope_phase` |
| 21.12% | 0.31% | `run_runoff_reconciliation` |
| 20.58% | 0.53% | `compute_active_frost_coupling` |
| 20.54% | 17.07% | `__memcmp_sse2` |
| 17.03% | 0.59% | `apply_kernel_writeback` |
| 11.73% | 0.01% | `decomposition_phase_dispatch_for_state_indexed` |
| 9.53% | 0.83% | `alloc::fmt::format::format_inner` |
| 9.21% | 0.00% | `build_perennial_decomposition_control` |
| 8.80% | 0.00% | `compute_same_pass_wb14_infiltration_lineage` |
| 8.15% | 7.70% | `ensure_no_overflow_indexed_symbols_for_decomposition` |
| 7.33% | 3.75% | `malloc` |
| 6.34% | 0.07% | `seed_wb11_runtime_surface_inputs` |
| 5.84% | 4.50% | `BTreeMap<K,V,A>::insert` |

## Interpretation

The PERFIDX04 read-side win held: hot-table reads removed the direct
`hourly_symbol`/string-lookup profile shape seen before PERFIDX04. The remaining profile is
not a single missed lookup. It is the cost of running the symbol-keyed runtime surface:
comparison-heavy map operations, writeback inserts, allocation/free, residual formatting,
decomposition overflow scans, and frost/hydrology state access.

Compared with PERFHO01's coarse split, the endpoint bottleneck moved from
"lookup/format dominates" to "runtime-surface architecture dominates across several
subtrees." That distinction matters: no one narrow id-table substitution is large enough to
move a 73x legacy ratio into a 10x target band.
