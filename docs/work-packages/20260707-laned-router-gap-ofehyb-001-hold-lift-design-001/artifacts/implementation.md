# Implementation Notes

Status: **EXECUTED**. Evidence mode: **Static**.

## Contract Amendment

`SC-OFEROUTE-002` rev 3 authorizes the source-memory cooldown predicate:

- Source-active bins route explicitly.
- After a contiguous source-active burst ends, the next
  `2 * burst_duration` source-free bins route explicitly.
- Later source-free bins are implicit-eligible.
- Upstream inflow still does not force explicit stepping after cooldown,
  because the implicit step books interval-mean upstream mass exactly.

The selector remains experimental/unpromoted; this package targets
`GAP-OFEHYB-001`, not default promotion.

`SC-OFEROUTE-001` rev 34 is a parent-pointer synchronization only: it points
the active hybrid rows at `SC-OFEROUTE-002` rev 3 and records that the Case-4
subgate is closed. It does not change routing ownership, tolerance, selector,
or default posture.

## Code Changes

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
  - Added shared `HYBRID_SOURCE_MEMORY_COOLDOWN_MULTIPLIER = 2`.
  - Added `hybrid_implicit_eligibility_mask`.
  - Replaced the old zero-source-only mask with the source-memory mask.
  - Added focused tests:
    - `hybrid_source_memory_cooldown_keeps_post_source_bins_explicit`
    - `hybrid_source_memory_allows_implicit_after_cooldown`
    - `hybrid_source_memory_resets_on_later_source_burst`
    - `hybrid_source_memory_routes_upstream_fed_zero_source_bins_implicitly`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
  - Updated the retained Case-4 hybrid harness to route source phase plus
    `2 * source_duration` cooldown explicitly before implicit recession.
  - Imports the shared production multiplier to avoid comparator drift.
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs`
  - Unignored `case4_hybrid_manning_ladder_meets_iwagaki_oracle`.
  - Updated the test comment to the rev-33 source-memory closure gate.

## Non-Changes

- No explicit TVD-MacCormack math changed.
- No implicit stepper math changed.
- No friction/source authority changed.
- No default/off selector behavior changed.
- No tolerance was weakened.
