# HPHYS0208 Disposition

Status: completed  
Evidence mode: Static + Ran

## Decision
- `HOLD`

## Closure measure evaluation
1. `MEASURE-HP208-001` (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`
   fail-hillslope counts reduce `39 -> 0`): **fail**.
2. `MEASURE-HP208-002` (`ProfileFCStore` fail-hillslope count reduces `27 -> 0`):
   **fail**.
3. `MEASURE-HP208-003` (contract-derived coupled-lineage tests + fail-closed
   guards): **pass**.
4. `MEASURE-HP208-004` (workspace validation gates): **pass**.

## Residual blocker for hold-lift
- Ran: 39-hillslope rerun summary:
  `/tmp/hphys0208_20260530T155837Z/parity/reports/hillslope_semantic_summary.json`
  - `ProfileFCStore`: `27/39` fail hillslopes
  - `Dp`: `39/39` fail hillslopes
  - `latqcc`: `39/39` fail hillslopes
  - `Total-Soil`: `39/39` fail hillslopes
  - `SoilWaterTotal`: `39/39` fail hillslopes
- Ran + Static: fail-count deltas vs HPHYS0207 are unchanged (`0` delta for all
  monitored columns).
- Ran: residual magnitude regression vs HPHYS0207 on coupled columns:
  - `Dp` mean abs diff avg: `+39.9689`
  - `latqcc` mean abs diff avg: `+89.6728`

## Interpretation
- HPHYS0208 execution completed contract-first sequencing and implemented the
  intended coupled-threshold runtime projection/scaffolding changes.
- Residual closure objective was not achieved on the 39-hillslope cohort.
- Package remains `HOLD`; follow-on work is required before hold-lift.

## Evidence
- Static: contract, implementation, and test edits in package write set.
- Ran: gate and rerun evidence under `/tmp/hphys0208_20260530T155837Z/`.
