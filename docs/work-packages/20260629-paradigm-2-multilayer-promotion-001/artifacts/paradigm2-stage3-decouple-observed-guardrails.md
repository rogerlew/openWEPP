# PARADIGM-2 Stage 3-Decouple Observed Guardrails

Schema: `paradigm2-stage3-decouple-water-temperature-gates-v1`
Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-081 INV-SNOWFREEZE-050 ADR-0029`
Evidence: `Static + Ran`

## Summary

- Observed gates passed: `True`
- Current default robust profile: `15` fails / `179` score
- Decoupled robust profile: `15` fails / `179` score
- Decoupled vs default better robust cells: `0`
- Decoupled vs default worse robust cells: `0`
- Runoff/timing worse robust cells: `0`
- Real-run elapsed seconds: `111.857`

## Gates

- `candidate_availability`: `PASS` - all direct-runtime model runs completed
- `snow_guardrail_equals_default`: `PASS` - default/candidate robust profile=15/179 vs 15/179; better/equal/worse=0/90/0; fail_delta=0; score_delta=0
- `runoff_timing_guardrail_vs_default`: `PASS` - decoupled-vs-default timing/runoff cells scored=40; better/equal/worse=0/40/0

## Boundary

- The decoupled arm is compared directly to the current no-env bulk default.
- `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL` is not set for the candidate.
- `event_rain_on_snow_response` remains unavailable in the daily observed corpus.
- No default activation or full in-stream temperature routing is authorized.

## Raw Outputs

- Output directory: `target/paradigm2_multilayer_promotion/observed`
- JSON artifact: `docs/work-packages/20260629-paradigm-2-multilayer-promotion-001/artifacts/paradigm2-stage3-decouple-observed-guardrails.json`
