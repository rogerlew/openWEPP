# PERFHO02 Verification

Status: LOCAL VERIFICATION COMPLETE 2026-06-16
Evidence mode: **Ran** + **Static**

## Verification Result

PASS.

## Verified Evidence

- `perf` availability probe was initially run and blocked by kernel policy.
- After `kernel.perf_event_paranoid=0` became visible in this session, `perf stat` on the CLI help path passed.
- Bounded H2637 `perf record` captured 9,586 samples at `/tmp/perfho02/perf-h2637-post-perfopt.data`.
- Bounded H2637 `perf stat` produced cycles/instructions/branch/cache counters for a 30-second window.
- Release binary was built with frame-pointer/debug-symbol flags.
- GDB sampling produced a raw log at `/tmp/perfho02/gdb-h2637-post-perfopt.txt`.
- The recorded table classifies 20 backtrace samples and excludes the final kill interrupt.
- The verdict follows from the GDB sample table and the `perf report` supplement: dominant residual is hydrology symbol/guard work; secondary residual is writeback application sort/alloc/insert; no output-writer samples.
- `git diff --check` passed after artifact and catalog edits.

## Gate Evidence Non-Deferral Check

PASS. PERFHO02 closes only characterization gates. The recommended optimization is explicitly assigned to follow-on `PERFOPT02` and is not a hidden current-scope acceptance criterion.

## Caveat

This is local verification by the primary agent, not an independent delegated verifier.
