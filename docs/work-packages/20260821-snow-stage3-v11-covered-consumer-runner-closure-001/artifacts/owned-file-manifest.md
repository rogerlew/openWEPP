# Owned-file manifest

Initial declared write set:

- `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- affected files under `crates/openwepp-hillslope-orchestrator/`
- affected files under `crates/openwepp-vegetation/`
- affected files under `crates/openwepp-runner/`
- package-owned tests discovered during contract-first implementation

Exact source edits admitted in this increment:

- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_attachment.rs`:
  sealed provider-day binding, exact-one Stage-3 snow owner, covered-support
  routing, and per-support Stage-3/covered-V11/carrier forcing projections.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/09_snow_free_half_hour_forcing.rs`:
  read-only provider cursor accessors and GSI receipt digest projection.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/snow_stage3_v11_scheduler.rs`:
  runner-facing provider-day installation seam.
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs`:
  shared V11 resource/owner finalization, the unchanged snow-free guard, and
  the distinct typed `DirectV11SnowCoveredRealConsumerStack` with its
  `DirectV11SnowCoveredSegmentInput` and shared-carrier/Stage-3
  persistent-support boundary.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs`:
  support-duration-bound Stage-3 reconciliation validation for the admitted
  1,800-second persistent-support API.
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_tests.rs`:
  two-day provider-bound capability, GSI/cursor sequence poison regressions,
  and the persistent covered V11/Stage-3 shared-carrier test.
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`: public typed exports.

`Static:` Covered execution is admitted only through the distinct typed
adopter and covered segment input. It evaluates the shared carrier and
persistent Stage-3 support before using the explicitly named canopy/soil
continuation core; the existing snow-free adopter and its snow-present guard
remain unchanged. Terminal chronology and runner-owned physical support
construction remain outside this increment.
