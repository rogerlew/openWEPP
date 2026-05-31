# HPHYS0214 Verification Agent B

Status: completed  
Evidence mode: Ran

## Verification checks
1. Revalidated integrated summary recomputation inputs:
   - `/tmp/hphys0212_20260530T221447Z/parity/reports/hillslope_semantic_summary.json`
   - `/tmp/hphys0213_20260530T233248Z/parity/reports/hillslope_semantic_summary.json`
2. Revalidated generated integrated summary:
   `/tmp/hphys0214_20260531T004200Z/diagnostics/hphys0214_integrated_family_summary.json`.
3. Confirmed H5 guard transition:
   - HPHYS0212 log contains `HKERNEL-WB12-STORAGE-E-003`.
   - HPHYS0213 log does not contain `HKERNEL-WB12-STORAGE-E-003`.
4. Confirmed workspace gate logs and targeted-test logs exist under run root
   `/tmp/hphys0214_20260531T004200Z/`.

## Confirmed outcomes
- `ProfileFCStore`: `27/39` fail hillslopes.
- `Dp`: `39/39` fail hillslopes.
- `latqcc`: `39/39` fail hillslopes.
- `Total-Soil`: `39/39` fail hillslopes.
- `SoilWaterTotal`: `39/39` fail hillslopes.

## Verdict
- Integrated residual matrix is reproducible from source evidence.
- Final `HOLD` disposition is verified.
