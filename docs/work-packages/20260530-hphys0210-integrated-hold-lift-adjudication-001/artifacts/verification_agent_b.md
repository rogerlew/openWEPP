# HPHYS0210 Verification Agent B

Status: completed  
Evidence mode: Ran

## Verification checks
1. Recomputed integrated focus metrics from source semantic reports:
   - `/tmp/hphys0208_20260530T155837Z/parity/reports/semantic/H*.semantic.json`
   - `/tmp/hphys0207_20260530T042607Z/parity/reports/semantic/H*.semantic.json`
2. Revalidated generated integrated summary:
   `/tmp/hphys0210_20260530T194829Z/diagnostics/hphys0210_integrated_family_summary.json`.
3. Confirmed workspace gate logs and targeted integration logs exist in run
   root `/tmp/hphys0210_20260530T194829Z/`.

## Confirmed outcomes
- `ProfileDepth`: `0/39` fail hillslopes.
- `ProfilePorosityCap`: `0/39` fail hillslopes.
- `ProfileWPStore`: `1/39` fail hillslopes.
- `ProfileFCStore`: `27/39` fail hillslopes.
- `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`: `39/39` fail hillslopes.
- `Dp` and `latqcc` mean-abs-diff deltas vs HPHYS0207 are positive and large.

## Verdict
- Integrated residual matrix is reproducible from source evidence.
- Final `HOLD` disposition is verified.
