# HPHYS0209 Disposition

Status: completed  
Evidence mode: Static + Ran

## Decision
- `HOLD`

## Closure measure evaluation
1. `MEASURE-HP209-001` (`ProfileWPStore` `1/39 -> 0/39` or expected process-correct
   adjudication): **pass**.
   - Ran: focused summary shows `ProfileWPStore` fail-hillslope count remains
     `1/39` and is isolated to `H7`.
   - Static + Ran: HPHYS0209 contract addenda explicitly authorize bounded
     expected process-correct classification when non-regression conditions are
     satisfied.
2. `MEASURE-HP209-002` (`ProfileDepth` and `ProfilePorosityCap`
   non-regressing): **pass**.
   - Ran: `ProfileDepth` `0/39`; `ProfilePorosityCap` `0/39`.
3. `MEASURE-HP209-003` (contract-derived lane-specific tests): **pass**.
4. `MEASURE-HP209-004` (workspace validation gates): **pass**.

## Adjudication result
- Static + Ran: near-closed `ProfileWPStore` residual is adjudicated as
  bounded expected process-correct diagnostic evidence for HPHYS0209 scope.
- Ran: residual remains isolated to `H7` with stable profile-geometry
  non-regression (`ProfileDepth`, `ProfilePorosityCap` both `0/39` fail
  hillslopes).
- Static: fail-closed WB13 guard posture remains intact; no fallback or
  surrogate reprojection path was introduced.

## Why disposition remains HOLD
- Static: HPHYS0209 objective is complete, but integrated hold-lift adjudication
  for all active residual families is explicitly assigned to HPHYS0210.
- Static + Ran: coupled-family blockers from HPHYS0208 remain unresolved at
  integrated wave level (`ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`,
  `SoilWaterTotal`).

## Evidence
- Static: contract + test changes in HPHYS0209 write set.
- Ran: gate/test logs under `/tmp/hphys0209_20260530T171007Z/`.
- Ran: focused residual summary:
  `/tmp/hphys0209_20260530T171007Z/parity/reports/hphys0209_profilewp_focus_summary.json`.
