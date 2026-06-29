# PARADIGM-2 Stage 3 Observed Guardrails

Schema: `paradigm2-stage3-liquid-routing-meltwater-temperature-gates-v1`
Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-080 INV-SNOWFREEZE-050 ADR-0029`
Evidence: `Static + Ran`

## Summary

- Deferred observed gates passed: `True`
- Current default robust profile: `15` fails / `179` score
- Stage 1 rollback robust profile: `16` fails / `177` score
- Stage 3 robust profile: `16` fails / `177` score
- Stage 3 vs Stage 1 worse robust cells: `0`
- Runoff/timing worse robust cells: `0`
- Real-run elapsed seconds: `167.666`

## Gates

- `candidate_availability`: `PASS` - all direct-runtime model runs completed
- `snow_guardrail_vs_stage1`: `PASS` - stage3-vs-stage1 robust cells better/equal/worse=0/90/0; fail_delta=0; score_delta=0
- `runoff_timing_guardrail_vs_stage1`: `PASS` - stage3-vs-stage1 timing/runoff cells scored=40; better/equal/worse=0/40/0

## Boundary

- Stage 3 is compared to the Stage 1 rollback baseline because it requires the Stage 1 layer stack.
- The current default profile is reported as reference, not as Stage 3 isolation evidence.
- `event_rain_on_snow_response` remains unavailable in the daily observed corpus.
- No promotion/default activation decision is made by this diagnostic.

## Raw Outputs

- Output directory: `target/paradigm2_stage3_liquid_routing`
- JSON artifact: `docs/work-packages/20260629-paradigm-2-stage-3-liquid-routing-meltwater-temperature-001/artifacts/paradigm2-stage3-observed-guardrails.json`
