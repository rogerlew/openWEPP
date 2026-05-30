# HPHYS0207 Disposition

Status: completed  
Evidence mode: Static + Ran

## Decision
- `HOLD`

## Closure measure evaluation
1. `MEASURE-HP207-001` (canonical FC/WP depth-authority + tail-policy
   authority): **pass**.
2. `MEASURE-HP207-002` (contract-derived depth-authority/tail-policy tests):
   **pass**.
3. `MEASURE-HP207-003` (workspace validation gates): **pass**.
4. `MEASURE-HP207-004` (39-hillslope rerun + predecessor deltas + no
   regression vs HPHYS0205): **pass**.

## Residual blocker for hold-lift
- Ran: FC/WP fail-hillslope counts are improved but non-zero:
  - `ProfileFCStore`: `27/39`
  - `ProfileWPStore`: `1/39`
- Ran + Static predecessor fail-count deltas:
  - vs HPHYS0206 (`/tmp/hphys0206_20260530T032538Z/parity/reports/hillslope_semantic_summary.json`)
    - `ProfileFCStore`: `39 -> 27` (improved)
    - `ProfileWPStore`: `39 -> 1` (improved)
  - vs HPHYS0205 (`/tmp/hphys0205_20260530T022235Z/parity/reports/hillslope_semantic_summary.json`)
    - `ProfileFCStore`: `39 -> 27` (improved)
    - `ProfileWPStore`: `39 -> 1` (improved)
  - vs HPARITY02 baseline (`/tmp/hparity02_20260529T204555Z/parity/reports/hillslope_semantic_summary.json`)
    - `ProfileFCStore`: `27 -> 27` (no change)
    - `ProfileWPStore`: `1 -> 1` (no change)
- Ran: FC/WP residual magnitudes improved and are non-regressing:
  - `ProfileFCStore` mean-abs-diff avg:
    - `6.4922` (HPHYS0205) -> `2.0527` (HPHYS0207)
    - `7.2212` (HPHYS0206) -> `2.0527` (HPHYS0207)
  - `ProfileWPStore` mean-abs-diff avg:
    - `1.8894` (HPHYS0205) -> `0.0573` (HPHYS0207)
    - `2.2445` (HPHYS0206) -> `0.0573` (HPHYS0207)

## Interpretation
- HPHYS0207 scope closure is complete:
  - contract authority,
  - contract-derived tests,
  - production depth-authority implementation,
  - workspace gates,
  - rerun/delta evidence.
- This package closes the HPHYS0206 regression signal and restores FC/WP
  residual posture to HPARITY02-level metrics.
- Package remains `HOLD` because FC/WP comparator residual is not yet zero.

## Evidence
- Static: canonical contract updates and implementation/test edits in the
  package write set.
- Ran: gate + rerun evidence under `/tmp/hphys0207_20260530T042607Z/parity/`.
