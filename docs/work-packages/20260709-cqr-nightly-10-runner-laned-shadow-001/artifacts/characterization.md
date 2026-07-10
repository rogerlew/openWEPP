# Characterization

Evidence label: Static/Ran.

Status: `EXECUTED`

Characterization targets:

- `LanedShadowCollector::observe_row`
- `LanedShadowCollector::validate_lane_day_operands`
- `LanedShadowCollector::commit_day`
- `LanedShadowCollector::finalize` as a threshold-adjacent row
- diagnostic helpers `env_enabled`, `record_operand_build`, and
  `emit_profile_report`

Behavior oracle:

- Added module-local test helpers that construct small `DirectPublicationDayRow`
  values without fixture mutation.
- Added validation tests for non-finite/negative hourly rainfall, routed melt,
  LAI, canopy height, and the positive-LAI/missing-height hard failure.
- Added `observe_row` domain tests for out-of-range lanes, non-positive area,
  and negative runoff volume.
- Added zero-source day-change/finalize coverage for the non-routing
  `commit_day` branch.
- Added positive uniform-shape routed day coverage for source reconstruction,
  routed-melt uniform-class accounting, `route_buffered_day` handoff, and
  finalize aggregate calculation.
- Added positive uniform-shape coverage without routed melt to bind the
  lump-only classification path.
- Added direct missing-dynamic-operands fail-closed coverage for cascade segment
  and rate-series construction.
- Added a diagnostic helper test for env/profile/reporting surfaces without
  asserting inherited environment state.

Commands:

- `cargo test -p openwepp-runner laned_shadow --lib -- --nocapture`
  - PASS, `15` passed, `0` failed, `83` filtered.
- `OPENWEPP_LANED_SHADOW_PROFILE=1 cargo test -p openwepp-runner diagnostic_profile_helpers_cover_opt_in_surfaces_without_public_outputs --lib -- --nocapture`
  - PASS, `1` passed, `97` filtered; review env-profile reliability check.
- `cargo nextest run -p openwepp-runner laned_shadow`
  - PASS, `15` tests run, `15` passed, `133` skipped.

Disposition:

- Characterization-only implementation. No production logic changed.
- The behavior oracle is direct unit coverage over the collector branches plus
  the existing source guard and H2637 integration surfaces recorded in
  `coverage-before.md`.
