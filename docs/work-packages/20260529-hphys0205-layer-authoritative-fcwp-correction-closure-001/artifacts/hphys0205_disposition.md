# HPHYS0205 Disposition

Status: completed  
Evidence mode: Static + Ran

## Decision
- `HOLD`

## Closure measure evaluation
1. `MEASURE-HP205-001` (corrected-layer authority text in canonical contracts):
   **pass**.
2. `MEASURE-HP205-002` (contract-derived corrected-layer lineage tests):
   **pass**.
3. `MEASURE-HP205-003` (workspace validation gates): **pass**.
4. `MEASURE-HP205-004` (39-hillslope rerun + predecessor deltas): **pass**.

## Residual blocker for GO disposition
- Ran: FC/WP fail-count closure did not improve after HPHYS0205 rerun:
  - `ProfileFCStore`: `39/39` fail hillslopes.
  - `ProfileWPStore`: `39/39` fail hillslopes.
- Ran + Static predecessor deltas:
  - vs HPHYS0202 (`/tmp/hphys0202_20260530T003833Z/parity/...summary.json`):
    - `ProfileFCStore`: `39 -> 39` (no change)
    - `ProfileWPStore`: `39 -> 39` (no change)
  - vs HPARITY02 disposition baseline:
    - `ProfileFCStore`: `27 -> 39`
    - `ProfileWPStore`: `1 -> 39`
- Ran + Static (residual-magnitude context):
  - HPHYS0205 did materially reduce FC/WP residual magnitudes (example H1 from
    `artifacts/claude-code-review-findings.md`):
    - `ProfileFCStore` mean abs diff: ~`206.7 mm` (HPHYS0202) -> `6.18 mm`
      (HPHYS0205)
    - `ProfileWPStore` mean abs diff: ~`87.2 mm` (HPHYS0202) -> `1.74 mm`
      (HPHYS0205)
  - Hold remains because tolerance gating is still unmet despite this
    magnitude reduction.

## Interpretation
- HPHYS0205 contract authority, corrected-layer projection wiring, and test/gate
  closure are complete.
- End-to-end FC/WP parity residual remains open; fail-count closure is unchanged
  even though residual magnitudes improved substantially. Additional follow-on
  investigation/closure work is required before hold-lift.

## Evidence
- Static: canonical contract and implementation/test edits listed in package
  artifacts.
- Ran: workspace gates and 39-hillslope rerun evidence under
  `/tmp/hphys0205_20260530T022235Z/parity/`.
