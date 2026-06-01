# HPHYS0235 Disposition

Status: completed  
Evidence mode: mixed (`Ran` + `Static`)

## Decision

- **HOLD**

## Closure Measure Adjudication

1. `MEASURE-HP235-001` (contract reanchoring): **satisfied**.
2. `MEASURE-HP235-002` (hourly vs daily A/B evidence): **satisfied**.
3. `MEASURE-HP235-003` (root-cause mapping): **satisfied**.
4. `MEASURE-HP235-004` (implementation-ready handoff): **satisfied**.

## Rationale

1. `Dp ~7x` mismatch is reproduced and attributed to hourly-lane execution
   shape mismatch, not WB13 publication lineage.
2. Baseline `ui_run=1` authority is iterative (`watbal_hourly` + `purk`),
   while current openWEPP hourly lane is divisor-only single-pass.
3. Package objective (root-cause closure) is complete; production fix is
   intentionally deferred to follow-on implementation package.

## Stream-Level Outcome

HPHYS stream remains in `HOLD` pending hourly WB18/WB11 iterative-lane
implementation and rerun adjudication.
