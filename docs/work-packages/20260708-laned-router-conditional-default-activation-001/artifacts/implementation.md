# Implementation

Status: `COMPLETE`
Evidence mode: Static + Ran.

## Code Changes

`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`

- Added `DirectLanedActiveDefaultEligibility`.
- Added `DirectProductionDayInputBuilder::laned_active_default_eligibility()`.
- The resolver counts scheduled lanes whose projected authority has
  `ofe_routing.is_some()`, the same authority surface consumed by
  `laned_active_config()`.
- Tightened `direct_production_optional_lane_routing_coefficient_authority()`
  so only truly all-absent scheduled crop-slot authority returns `None`.
  Schedule-incomplete route authority and inconsistent per-slot authority fail
  closed during typed seed construction instead of being collapsed into
  fallback eligibility.

`crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`

- Replaced `OPENWEPP_LANED_ACTIVE=1` as the only active selector with the rev
  46 resolver:
  - explicit active -> active;
  - explicit disable -> off;
  - default/no-env complete -> active;
  - default/no-env absent -> off;
  - default/no-env mixed -> typed fail-closed.
- Added fail-closed explicit active+disable conflict.
- Kept active+shadow mutual exclusion for both explicit and default active.
- Missing active summary remains a hard failure whenever active ownership is
  selected.

`crates/openwepp-runner/src/hillslope/laned_active.rs`

- Added `ACTIVE_DISABLE_ENV = "OPENWEPP_LANED_ACTIVE_DISABLE"`.
- Added `disable_enabled()`.
- Updated module authority text to rev 46.

`tests/integration/laned_shadow_h2637.rs`

- Clears `OPENWEPP_LANED_ACTIVE_DISABLE` in all selector helpers.
- Shadow diagnostic helper now sets `OPENWEPP_LANED_ACTIVE_DISABLE=1` for
  native coefficient fixtures so rev 46 default active does not conflict with
  diagnostic shadow runs.
- Added mixed coefficient fail-closed coverage.
- Added active+disable conflict coverage.
- Existing explicit active missing-coefficient and active+shadow conflict
  coverage retained.
- Updated the ignored H2637 acceptance vector to prove explicit disable,
  default active, and explicit active.

## Runtime Evidence

Focused non-ignored integration:

```text
cargo nextest run --workspace --profile full --test laned_shadow_h2637
Summary [ 38.125s] 8 tests run: 8 passed, 2 skipped
```

Ignored acceptance vector:

```text
cargo nextest run --workspace --profile full --test laned_shadow_h2637 \
  h2637_native_active_owner_routes_and_closes --run-ignored ignored-only
Summary [563.620s] 1 test run: 1 passed (1 slow), 9 skipped
```
