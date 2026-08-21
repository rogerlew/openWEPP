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
  sealed provider-day binding, exact-one Stage-3 snow owner, and owner-chain
  projection.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/09_snow_free_half_hour_forcing.rs`:
  read-only provider cursor accessors and GSI receipt digest projection.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/snow_stage3_v11_scheduler.rs`:
  runner-facing provider-day installation seam.
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs`:
  ending Stage-3 owner injection into the existing snow-free owner envelope
  and read-only owner configuration accessors. The snow-present guard remains
  unchanged.
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_tests.rs`:
  two-day provider-bound capability and GSI/cursor sequence poison regressions.
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`: public typed exports.

`Static:` No covered lower-boundary physics implementation is admitted through
the existing snow-free LSE call. A distinct covered adopter remains the next
source boundary and is not claimed closed by these edits.
