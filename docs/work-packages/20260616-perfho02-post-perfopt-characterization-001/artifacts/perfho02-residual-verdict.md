# PERFHO02 Residual Verdict

Status: COMPLETE 2026-06-16
Evidence mode: **Ran** (GDB samples, `perf record`, `perf stat`) + **Static** (source-path interpretation)

## Verdict

Post-PERFOPT01 H2637 execution remains CPU-bound in per-OFE-day metadata/control work. The dominant sampled residual is no longer writeback diagnostic detail construction. It is repeated typed-symbol lookup, dynamic symbol construction, and guard scans inside hydrology/frost/decomposition/plant scheduling, with a secondary residual in `apply_kernel_writeback` sorting/allocation/insertion.

PERFHO02 found no samples in Parquet or output writers. After `perf_event_paranoid` was lowered to `0`, `perf record` confirmed the same direction with 9,586 hardware samples: `execute_persistent_scheduler_kernel_lifecycle` accounts for `96.24%` children, `run_hillslope_phase` for `41.14%`, `run_runoff_reconciliation` for `22.40%`, `apply_kernel_writeback` for `12.46%`, `compute_active_frost_coupling` for `12.35%`, and `ensure_no_overflow_indexed_symbols_for_decomposition` for `7.48%`.

## Residual Hot Path

Primary residual:

- `Wb11HydrologyKernel` symbol access and dynamic symbol construction:
  - `wb19_load_layer_state`
  - `frost_fine_layer_symbol`
  - `compute_active_frost_coupling`
  - `require_shadow_fine_state_domains`
  - `resolve_erod14_wave2_enabled`
- Plant/decomposition dispatch guard paths:
  - `resolve_active_pl_slot_selection`
  - `require_integral_pl_dispatch_symbol_in_range`
  - `ensure_no_overflow_indexed_symbols_for_decomposition`
- Scheduler/consumer validation:
  - `validate_hillslope_consumer_boundary`
  - remaining `BTreeMap::insert` under `execute_persistent_scheduler_kernel_lifecycle`

Secondary residual:

- `openwepp_kernel_contract::lib_mod::writeback::apply_kernel_writeback` still sorts applied symbol lists and inserts string-keyed `BoundarySymbol` updates into `BTreeMap`s on the success path.

## Recommended Follow-On

Open `PERFOPT02-symbol-access-and-writeback-application` as a behavior-preserving optimization package.

Suggested scope:

1. Add narrow, deterministic symbol-key reuse in the hot hydrology paths that repeatedly build indexed `BoundarySymbol`s on every OFE-day. Candidate families: frost fine-layer symbols, PL dispatch symbols, EROD14 Wave-2 enablement symbols, and WB19 layer-state symbols.
2. Avoid success-path guard scans where an equivalent bounded indexed-symbol registry or cached per-schedule/per-layer shape can prove absence of overflow without scanning all state keys every phase/day.
3. Revisit `apply_kernel_writeback` success path: preserve public applied-symbol ordering, but avoid repeated allocation/sort where payload order is already canonical or where canonical symbols can be produced without sorting borrowed fields.
4. Keep the same hard constraints as PERFOPT01: bit-identical outputs, no FP reduction reorder, no fail-closed behavior drift, no science-contract change.

## Not Recommended For PERFOPT02

- Do not target output writers first; PERFHO01, PERFOPT01, and PERFHO02 all found no output-writer samples in the steady-state windows.
- Do not replace the whole runtime-surface contract in one step. The sample set points to narrower symbol-family and writeback-application wins that can be tested under the same bit-identity gates.
- Do not relax or remove guards. Any guard optimization must preserve typed failure behavior and detail on the failure path.

## Residual Risk

The original GDB window is coarse, but it has now been supplemented by `perf record` and `perf stat`. Kernel symbol resolution remains limited by `kernel.kptr_restrict=1`, but the target attribution is in openWEPP user-space symbols and is not blocked.
