# PERFHO02 Disposition

Status: COMPLETE 2026-06-16
Evidence mode: **Ran** + **Static**

## Outcome

PERFHO02 is complete as a post-PERFOPT01 characterization package.

It confirms the residual H2637 cost remains CPU-bound in symbol-keyed runtime metadata/control paths. The dominant sampled class was hydrology typed-symbol lookup, dynamic symbol formatting, frost/decomposition/PL guard work (`13/20`, `65%` in the GDB window). Writeback application sorting/allocation/insertion remains secondary (`4/20`, `20%`). Scheduler/daily-loop insertion/allocation and consumer-boundary validation accounted for the remaining GDB samples. After `perf_event_paranoid` was lowered to `0`, `perf record` captured 9,586 samples and confirmed the same direction: `execute_persistent_scheduler_kernel_lifecycle` `96.24%` children, `run_hillslope_phase` `41.14%`, `run_runoff_reconciliation` `22.40%`, `apply_kernel_writeback` `12.46%`, `compute_active_frost_coupling` `12.35%`, and `ensure_no_overflow_indexed_symbols_for_decomposition` `7.48%`. Output writers were not sampled.

## Deliverables

- `artifacts/perfho02-profiler-evidence.md`
- `artifacts/perfho02-residual-verdict.md`
- `artifacts/perfho02-gate-results.md`
- `artifacts/perfho02-review.md`
- `artifacts/perfho02-verification.md`
- `artifacts/perfho02-worker-handoff.md`

## Closure

No production Rust files were edited. No science contracts were edited. `git diff --check` passed. Rust closure gates were intentionally not run because PERFHO02 is documentation/profiling-only.

## Follow-On

Open `PERFOPT02-symbol-access-and-writeback-application` for a behavior-preserving optimization of the residual symbol-access and writeback-application paths.
