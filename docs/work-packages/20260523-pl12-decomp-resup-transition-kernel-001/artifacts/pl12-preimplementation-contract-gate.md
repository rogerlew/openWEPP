# PL12 Pre-Implementation Contract Gate

Status: `complete`
Evidence mode: `Ran`

## Gate Objective

Run contract-derived PL12 decomposition conformance tests before production
kernel edits and record the failing baseline.

## Command

```bash
cargo test -p openwepp-hillslope-orchestrator pl12_contract_conformance -- --nocapture
```

## Result (Pre-Implementation)

`FAILED` with `2` expected failing tests:

1. `pl12_contract_conformance_rejects_missing_perennial_cutday_payload`
2. `pl12_contract_conformance_rejects_invalid_perennial_grazing_window`

Observed failure posture (pre-edit baseline): scheduler decomposition dispatch
had not yet implemented full PL12 typed transition payload guard handling.

This gate run occurred before PL12 production decomposition-dispatch code edits.
