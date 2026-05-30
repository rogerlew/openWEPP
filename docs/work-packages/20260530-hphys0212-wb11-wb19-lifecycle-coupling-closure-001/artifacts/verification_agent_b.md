# HPHYS0212 Verification Agent B

Status: completed  
Evidence mode: Static + Ran

## Verification checks
1. Revalidated `hillslope_batch_status.tsv` and confirmed single execution
   failure at hillslope `H5`.
2. Revalidated `hillslope_semantic_summary.json` and confirmed monitored-family
   fail counts and mean absolute deltas.
3. Revalidated WB11/WB19/WB13 code-path anchors cited in package evidence:
   - WB11 carry-state seed gating
   - WB19 runtime-source symbol guards/projections
   - WB13 `Qd` decomposition guard/publication path

## Confirmed outcomes
- `semantic_report_count = 38`, `missing_semantic_reports = [5]`.
- `ProfileFCStore`: `26` fail hillslopes.
- `Dp`: `38` fail hillslopes.
- `latqcc`: `38` fail hillslopes.
- `Total-Soil`: `38` fail hillslopes.
- `SoilWaterTotal`: `38` fail hillslopes.

## Verdict
- Evidence is internally consistent.
- Follow-on remediation is required before hold-lift.
