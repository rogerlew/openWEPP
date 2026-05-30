# HPHYS0211 Verification Agent B

Status: completed  
Evidence mode: Static + Ran

## Verification checks
1. Re-validated family fail counts against
   `/tmp/hphys0208_20260530T155837Z/parity/reports/hillslope_semantic_summary.json`.
2. Recomputed per-hillslope row-failure/magnitude extracts from
   `/tmp/hphys0208_20260530T155837Z/parity/reports/semantic/H*.semantic.json`
   into `/tmp/hphys0211_20260530T203603Z/analysis/`.
3. Verified code-path anchors cited in the root-cause ledger:
   - runner daily lifecycle seed invocation,
   - WB18/WB19 state mutation/publication paths,
   - WB13 column publication paths.

## Confirmed outcomes
- `ProfileFCStore`: `27/39` fail hillslopes, all-row structural split.
- `Dp`: `39/39` fail hillslopes, full-row fail saturation.
- `latqcc`: `39/39` fail hillslopes.
- `Total-Soil`/`SoilWaterTotal`: `39/39` fail hillslopes with shared ranges.

## Verdict
- HPHYS0211 root-cause decomposition is internally consistent with evidence.
- Follow-on remediation should begin with HPHYS0212.
