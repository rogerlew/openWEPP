# No-Compatibility Proof

Evidence class: Static plus Ran.

Status: executed-held.

## Runtime Counters

- Direct H2637 modes produced no manifest because both failed closed before
  output creation at active snow partition authority.
- Focused R7 test
  `r7c_direct_production_executor_reports_no_day_input_compatibility_edges`
  passed in `cargo test -p openwepp-runner r7 -- --nocapture`.
- Focused R7E manifest tests passed and still prove default-disabled
  compatibility has no direct counters while activated default candidate uses
  the direct runtime route.

## Source Scans

- Ran: `cargo test -p openwepp-runner r7 -- --nocapture`; included passing
  source-scan tests:
  - `r7c_direct_production_source_excludes_compatibility_entrypoints`
  - `r7f_production_direct_uses_typed_day_input_builder`
  - `r7f_typed_day_input_hot_loop_excludes_runtime_surface_reads`
- Static scan:
  `DirectProductionDayInputBuilder::reject_unsupported_active_snow_frost`
  fails before using the existing map-backed
  `direct_publication_snow_liquid_partition` helper.
- Static scan:
  `direct_publication_snow_liquid_partition` still calls
  `Wb11HydrologyKernel::compute_direct_snow_liquid_partition` with
  `HillslopeWritebackSurface` state/flux maps. That helper is valid for
  compatibility/shadow/cutover evidence, not production direct hot-loop
  authority.

## Profile Scan

Not run. Direct H2637 did not reach the production hot loop, so profile output
would not satisfy the R7G performance-profile gate.

## Allowlist

Allowed edge-only compatibility paths remain explicit compatibility,
rollback, replay, diagnostics, shadow validation, and non-production setup
adapters documented by the package.

The active-snow follow-up must not resolve this hold by calling the existing
map-backed snow helper from `DirectProductionDayInputBuilder`.
