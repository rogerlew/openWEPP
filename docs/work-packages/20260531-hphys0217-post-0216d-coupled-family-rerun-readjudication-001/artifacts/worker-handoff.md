# HPHYS0217 Worker Handoff

Status: completed
Evidence mode: Static + Ran

## Execution result
- Fresh post-HPHYS0216D rerun completed (`39/39` hillslope + semantic rc=0).
- `ProfileFCStore` regression is reduced to pre-HPHYS0216D posture
  (`27/39`, `2.052691160104116`).
- Coupled families remain open with unchanged saturation:
  - `Dp` `39/39`
  - `latqcc` `39/39`
  - `Total-Soil` `39/39`
  - `SoilWaterTotal` `39/39`

## Immediate next actions
1. Prepare and execute `HPHYS0218` to remediate coupled `Dp`/`latqcc`
   process-authority lineage (contract-first: SC updates -> tests -> gate ->
   code).
2. Prepare and execute `HPHYS0219` to remediate `Total-Soil` /
   `SoilWaterTotal` continuity/publication lineage (contract-first sequence).
3. Run post-remediation integrated rerun/readjudication package (`HPHYS0220`)
   to decide hold-lift.
