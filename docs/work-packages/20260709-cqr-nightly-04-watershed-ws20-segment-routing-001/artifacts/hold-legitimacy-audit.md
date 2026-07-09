# Hold Legitimacy Audit

Evidence label: Static/Ran.

Status: `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-ADR0021-COVERAGE-BLOCKER`

## Blocker

Accepted dual-review findings block completion:

- ADR-0021 science-tier coverage closure is not met after adding/materially
  changing tests for this target.
- Region coverage and per-function region-floor evidence are unavailable in the
  focused LCOV artifact.
- Provisional characterization did not cover key refactored case34/case4 paths
  before decomposition closeout.

## Evidence

- Provisional focused LCOV:
  `LF:1364`, `LH:817`, `59.89736070381232%`.
- Production-line subset from the same LCOV:
  `LF:1024`, `LH:478`, `46.6797%`.
- Provisional target CRAP after decomposition:
  max `30.0`, rows above `30`: `0`.
- Review Agent A and Review Agent B both rejected completion on coverage/gate
  evidence grounds.
- Full nextest rerun was interrupted after the hold decision:
  `command-07-nextest-rerun.exit` = `130`.

## Attempted In-Envelope Route

The package attempted behavior-preserving helper extraction plus module-local
characterization tests for WS20 flow partitioning, class transport preparation,
segment hydraulics, transport snapshots, case12 update branches, transition
eligibility, no-transition diagnostics, and no-segment core identity.

This reduced provisional target CRAP to the threshold but did not provide
coverage closure for the science-tier target.

## Why CQR Cannot Close Safely Here

Closing this package would require converting the CQR slice into a broader
WS20/WS21 module test-enhancement package:

- cover hundreds of additional production lines;
- cover case34/case4/case3 route paths and branch families;
- produce branch/region evidence, not just LCOV line evidence;
- bind coverage to `SC-ROUTE-001` and `SC-SED-001` obligations;
- rerun full closure gates after the expanded tests.

That work exceeds this narrow nightly CQR target and must be planned as a
dedicated test-enhancement or combined test-enhancement-plus-CQR package.

## Rollback Proof

- Target file restored to scaffold state:
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs`.
- Current line count: `1078`.
- `git diff -- crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs` is empty after rollback.

## First Actionable Follow-On

Create and execute a dedicated WS20/WS21 channel sediment routing
test-enhancement package before retrying CQR on this module. The package should
add branch-sensitive coverage for case34/case4/case3 segment routing,
document region/per-function floor evidence, and only then reattempt CRAP
decomposition.
