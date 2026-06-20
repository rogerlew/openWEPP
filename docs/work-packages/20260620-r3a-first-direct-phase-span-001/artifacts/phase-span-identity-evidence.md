# R3A Phase-Span Identity Evidence

Status: complete.
Evidence mode: Ran.

Fixture:

- Direct unit fixture in
  `r3a_input_accounting_span_computes_mutates_downstream_and_shadow_projects`.
- Identity: `DirectRunIdentity::new(7, 2637, 1, 1)`.
- Direct frame: one lane, one day, signed temperature allowed.

Typed inputs:

- `precipitation_m = 0.125`
- `effective_temperature_c = -2.5`
- `surface_carry_m[0] = 0.25`
- `surface_carry_m[1] = 0.125`
- `lateral_carry_m[0] = 0.0625`
- `upstream_flow_m = 0.03125`
- `subsurface_input_m = 0.015625`

Expected exact results:

| Field | Expected |
|---|---:|
| surface transfer | `0.375` |
| lateral transfer | `0.0625` |
| upstream flow | `0.03125` |
| subsurface input | `0.015625` |
| transfer input | `0.484375` |
| total accounted input | `0.609375` |

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r3a_ -- --nocapture`: PASS.

Identity result:

- `DirectInputAccountingState` equals expected state.
- `DirectDownstreamOperands` equals expected projection from state.
- `DirectShadowProjection` equals expected lane/day/input projection.
- Span report counters: phase entries `2`, direct compute `1`, state mutation
  `1`, downstream operand `1`, shadow projection `1`, compatibility edge
  invocations `0`.

Tolerance: exact equality. All values are binary fractions.
