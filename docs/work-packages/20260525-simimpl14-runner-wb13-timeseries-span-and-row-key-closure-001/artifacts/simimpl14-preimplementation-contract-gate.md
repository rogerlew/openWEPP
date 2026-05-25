# simimpl14-preimplementation-contract-gate

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Gate objective: confirm newly added SIMIMPL14 contract-derived tests fail against pre-implementation single-day runner behavior.

## Ran
- Command: `cargo test -p openwepp-runner --lib simimpl14_contract_gate_ -- --nocapture`
- Result: failed as required before production edits.
- Failure evidence:
- `simimpl14_contract_gate_continuous_wb13_span_and_keys`: WB13 numeric row count was `1` instead of expected `2`.
- `simimpl14_contract_gate_loss_output_is_run_span_truthful`: missing `/climate_day_count` in loss JSON.
- Interpretation: pre-implementation runner path collapsed span to first day and did not expose run-span continuity metadata, matching SIMIMPL14 gap hypothesis.
