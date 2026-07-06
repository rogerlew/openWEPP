# Implementation

Status: **EXECUTED-PARTIAL-HOLD**.

Evidence mode: Static + Ran.

## Changes

Implemented:

- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
  - Replaced the one-day route cap with explicit `24 h` source window plus
    `6 h` zero-source drain tail.
  - Kept `seam_rate_at` as the source/rainfall sampler, so no source depth is
    added after the 24 hourly bins.
  - Added `routing_window_s` and the regression
    `routing_window_keeps_drain_tail_for_hour_24_source`.

Not implemented:

- No active production selector.
- No DC01-disable in production.
- No active closure hard-fail.
- No routed-hydrograph-to-erosion producer wiring.
- No public schema/default behavior change.

Reason: after the terminal fix, release timing completes but is far above the
D14 budget, and static audit shows the active owner path requires production
phase-order integration. A post-hoc shadow/candidate bridge would violate the
consumer-path closure rule.
