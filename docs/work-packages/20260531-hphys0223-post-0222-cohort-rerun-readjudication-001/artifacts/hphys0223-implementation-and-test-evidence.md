# HPHYS0223 Implementation and Test Evidence

Status: completed
Evidence mode: Static + Ran

## Scope executed
- Diagnostics-only rerun/readjudication package execution.
- No production code edits.
- No science-contract or governance edits.

## Ran evidence
- Run root: `/tmp/hphys0223_20260531T201410Z/parity`
- Hillslope status: `39/39` success
- Semantic status: `39/39` success (valid rerun settings)
- Summary:
  `/tmp/hphys0223_20260531T201410Z/parity/reports/hillslope_semantic_summary.json`

## Residual-family readjudication
- `Dp`: `39/39`, mean abs diff `0.32491959785932106`
- `latqcc`: `39/39`, mean abs diff `0.7517675547493208`
- `Total-Soil`: `39/39`, mean abs diff `140.7089613428272`
- `SoilWaterTotal`: `39/39`, mean abs diff `140.7089613428272`
- `ProfileFCStore`: `27/39`, mean abs diff `2.0526911601041165`
- `ProfileWPStore`: `1/39`, mean abs diff `0.05729745831355476`

## Delta vs HPHYS0221 (Ran)
Reference:
`/tmp/hphys0221_20260531T141839Z/parity/reports/hillslope_semantic_summary.json`

All monitored-family metrics are unchanged (`delta = 0`) versus HPHYS0221.
