# simimpl05 contract implementation evidence

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- `SC-WATBAL-001` `INV-WATBAL-018` and `SC-SYSTEM-001` `INV-SYSTEM-018`
  closure was implemented in runner production flow by adding an explicit
  scheduler/kernel lifecycle gate before publication.
- `crates/openwepp-runner/src/lib.rs` now computes and writes
  `/execution_provenance/*` manifest fields:
  - `scheduler_kernel_executed=true`
  - `publication_source="scheduler-kernel"`
  - `simpipe_guard_id="HS-SIMPIPE-E-001"`
- Publication is now execution-owned for SIMIMPL05 scope: the runner returns a
  typed hard-fail (`RuntimeSurfaceFailure` detail prefixed with
  `HS-SIMPIPE-E-001`) when scheduler/kernel lifecycle does not complete.
- Daily-lane scope is explicit (`selected_lane="daily"`) and unchanged
  out-of-scope closures remain deferred:
  - `GAP-SIMMODE-001` (SIMIMPL07)
  - `GAP-SIMOUT-001` (SIMIMPL06)

## Ran
- Verified pre-change SIMPIPE failure baseline:
  - `cargo test -p openwepp-runner --test simimpl04_runner_kernel_execution_contract -- --ignored`
  - Observed missing `/execution_provenance/scheduler_kernel_executed`.
- Verified post-change SIMPIPE closure:
  - `cargo test -p openwepp-runner --test simimpl04_runner_kernel_execution_contract`
  - Result: pass.
