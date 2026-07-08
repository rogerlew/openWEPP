# Implementation

Evidence mode: Static plus Ran.

## Summary

This package added a narrow diagnostic trace-detail surface so the active
router can emit day/lane outlet bins and raw hydrograph samples for one
selected row. The surface is opt-in, trace-gated, and absent from default/off
execution.

No production routing physics, mesh policy, shape tolerance, or contract text
was changed.

## Code Changes

### Runner selector

`crates/openwepp-runner/src/hillslope/laned_active.rs` now accepts
`OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=sim_day:lane`, using one-based values for
the external selector. It rejects the selector unless active routing and
active trace output are both enabled.

### Runtime config

`DirectLanedActiveConfig` now carries an optional
`DirectLanedActiveTraceDetailFilter`. Config validation rejects a detail filter
when trace output is disabled.

### Active route detail

For the selected day/lane only, `laned_active_route_lane` records:

- outlet bin mass in cubic metres
- outlet bin spans in seconds
- raw hydrograph time in seconds
- raw outlet flow in cubic metres per second
- raw outlet depth in metres

The executor threads the boolean selector into the route call. Zero-source
trace rows and non-selected rows carry no detail payload.

### Trace JSONL publication

`write_laned_active_trace_output` writes a `trace_detail` object only when the
selected row contains detail data. The schema tag is
`openwepp-laned-active-trace-detail-v1`.

## Code Gates

Ran:

```bash
cargo fmt --check
cargo nextest run -p openwepp-runner --lib \
  trace_detail_filter_parses_one_based_day_lane \
  trace_selector_requires_explicit_one \
  mesh_policy_parser_defaults_parses_and_rejects_invalid_target_dx
cargo nextest run -p openwepp-hillslope-orchestrator --lib \
  active_route_uses_post_growth_canhgt_not_static_lane_config \
  mesh_policy_resolves_fixed_target_floor_and_cap \
  day_closure_enforces_cascade_and_identity_tolerances \
  stage_flux_limiter_prevents_positive_clamp_injection \
  final_tvd_scaling_preserves_positivity_and_total
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo deny check
```

Focused tests and the full Rust closure loop passed. Final gate results are
recorded in `gate-results.md`.
