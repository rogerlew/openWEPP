# Correctness Review - Faraday

Static: read root/work-package/science-contract/crate/test instructions, `SC-OFEROUTE-001`, the package `package.md`, all package-local artifacts present at review time, the touched solver diff, active clamp-source guard, and D10B/Case-4 tests.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator ofe_routing::kinematic_wave -- --nocapture` -> 16 passed.
- `cargo test -p openwepp-hillslope-orchestrator ofe_routing::d10b_reconciliation_tests -- --nocapture` -> 11 passed.
- `cargo test -p openwepp-hillslope-orchestrator direct_runtime::laned_active::tests::day_closure_enforces_cascade_and_identity_tolerances -- --nocapture` -> 1 passed.

## Findings

1. Medium - Final TVD positivity scaling is contract-bound but not directly exercised by package evidence.

   `SC-OFEROUTE-001` rev 41 requires the final TVD face correction to be scaled uniformly when the full correction would make a cell negative, preserving conservation because the correction is face-based and telescoping (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:192`). The implementation follows that algebra by computing one global `tvd_scale` and applying it to every face-derived cell correction (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:910`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:925`). However, the new focused regression constructs flat depths and zeros `alpha`, so the TVD term is zero and only the stage-face limiter is exercised (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:1474`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:1481`). The package gate table records broad D10B/Case-4 coverage but no branch-level evidence that `tvd_scale < 1` occurred or that such a branch preserves non-negativity and raw closure (`docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/artifacts/gate-results.md:11`, `docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/artifacts/gate-results.md:12`). For a conservation-sensitive, contract-amended solver branch, add a focused vector or recorded counter proving a full-strength TVD correction would go negative, the scaled correction stays non-negative, and `residual_m2()` remains machine-scale without clamp mass.

## Residual Risk And Missing Tests

- The stage-face limiter is conservative by static inspection: each limited outgoing face is bounded by `h*dx + incoming*dt + source*dx*dt`, and the same limited boundary faces are booked in `outflow_m2`/`scheme_outflow_m2` (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:718`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:1001`). The focused stage over-drain test passed.
- Booked outflow and storage closure remain coherent: run-level storage is computed after the final committed state (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:1203`), and active day closure still uses injected + clamp - terminal outlet - mesh storage (`crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs:922`).
- D10B/Case-4 authority was not visibly violated in focused local validation; the Iwagaki oracle ladder and D10B ledger/handoff tests passed.
- Rev-40 active clamp-source guard remains meaningful. The guard is still before cascade/identity acceptance (`crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs:897`), and the focused guard test still rejects excessive and zero-source clamp books.
- I did not rerun the package WA harness or full workspace gates; I relied on `gate-results.md` for those recorded runs.

Approval status: no production arithmetic blocker found for the stage limiter, ledger booking, active guard, or D10B surfaces. Closure should address the missing direct final-TVD-scaling evidence before treating rev 41 as fully validated.
