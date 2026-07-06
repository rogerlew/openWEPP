# H2637 Routed-Hydrograph Evidence

Status: **COMPLETE** (Ran).

## Active-Candidate Consumer Proof

Ran:
`cargo test -p openwepp-hillslope-orchestrator wave1_span_routed_hydrograph_shape -- --nocapture`

Result: pass, 3 passed.

Evidence:
- `wave1_span_routed_hydrograph_shape_supersedes_dc01_weights` proves the
  Wave-1 plan and published `hourly_runoff_fraction` consume supplied routed
  hydrograph weights instead of DC01 weights.
- The fixture includes DC01 excess/source hours not present in the routed
  candidate; a fallback to DC01 would publish nonzero hours `10`/`11` and fail.

## H2637 Default/Off Evidence

Ran:
`cargo test -p openwepp --test laned_shadow_h2637 h2637_native_shadow_classifies_uniform_shape_after_d12 -- --ignored --nocapture`

Result: pass, 1 passed, finished in `325.24s` on the final boxed-shape code.

Recorded assertions from the integration test:
- shadow-off manifest has no `laned_shadow` block;
- shadow-on HBP bytes equal shadow-off HBP bytes;
- shadow-on pass parquet bytes equal shadow-off pass parquet bytes;
- `days_seen = 731`;
- `days_routed = 622`;
- `days_uniform_shape = 6`;
- `days_uniform_shape_with_routed_melt = 0`;
- `days_uniform_shape_without_routed_melt = 6`.

D13 does not activate the routed-hydrograph candidate in H2637 production
paths; the real-H2637 evidence is default/off identity plus preservation of
the D12 Lane D shadow diagnostics.
