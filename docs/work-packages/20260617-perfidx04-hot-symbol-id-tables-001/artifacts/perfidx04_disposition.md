# PERFIDX04 Disposition

Status: complete 2026-06-17.

Ran:
- Final anchor identity: pass at required level for OFE1-OFE5, H2637 no-UI, and H2637 with UI.
- Final profiler evidence: pass; user-space `perf` captured 9,495 samples and direct hot `hourly_symbol` formatting is no longer material in named hot paths.
- Final gates: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`, and `git diff --check` all passed.

Outcome:
- PERFIDX04 realized a 24.26% H2637 no-UI speedup and 25.17% H2637 UI speedup against the pre-PERFIDX04 baseline.
- The package is closed with residual Stage-5 symbol-writeback/guard work noted but not blocking this Stage-4 objective.
