# HPHYS0233 Disposition

Status: completed  
Evidence mode: mixed (`Ran` + `Static`)

## Decision

- **HOLD**

## Closure Measure Adjudication

1. `MEASURE-HP233-001` (SC-PERC authority amendment for restrictive branch and
   WB13 anti-shadow lineage): **satisfied**.
2. `MEASURE-HP233-002` (contract-derived restrictive/guard tests): **satisfied**.
3. `MEASURE-HP233-003` (runtime projection + WB18/WB13 production
   implementation with typed guards): **satisfied**.
4. `MEASURE-HP233-004` (`H1..H39` rerun + semantic coverage): **satisfied**
   (`39/39` execution and comparator reports, all `rc=0`).
5. `MEASURE-HP233-005` (H1 day-1..7 transient readjudication): **satisfied**.
6. `MEASURE-HP233-006` (required gates + disposition/handoff): **satisfied**.

## Rationale

1. Contract-first daily restrictive conductivity lineage closure is now
   implemented and regression-guarded.
2. Cohort-level residuals improved materially for `Dp`, `Total-Soil`, and
   `SoilWaterTotal`.
3. Stream-level hold columns still fail on all 39 reports for `Dp`, `latqcc`,
   `Total-Soil`, and `SoilWaterTotal`, and `ProfileFCStore` remains `27/39`.
4. `latqcc` regressed relative to HPHYS0232, so coupled closure remains open.

## Stream-Level Outcome

HPHYS0233 objective is complete; HPHYS stream remains in `HOLD` pending
follow-on closure for coupled WB18/WB19 residual families.
