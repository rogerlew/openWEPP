# HPHYS0207 Verification Agent B

Status: completed  
Evidence mode: Ran

## Verification checks
1. Recomputed FC/WP fail-hillslope counts from
   `/tmp/hphys0207_20260530T042607Z/parity/reports/semantic/H*.semantic.json`.
2. Revalidated row-closure presence (`common=1461`, no unmatched rows) in all
   39 summary rows.
3. Compared FC/WP fail-count and residual-magnitude metrics against:
   - `/tmp/hphys0206_20260530T032538Z/parity/reports/semantic/H*.semantic.json`
   - `/tmp/hphys0205_20260530T022235Z/parity/reports/semantic/H*.semantic.json`
   - `/tmp/hparity02_20260529T204555Z/parity/reports/semantic/H*.semantic.json`

## Confirmed outcomes
- `ProfileFCStore` fail hillslopes: `27`.
- `ProfileWPStore` fail hillslopes: `1`.
- FC/WP mean abs diff averages (H1..H39):
  - FC: `2.0527` (HPHYS0207) vs `7.2212` (HPHYS0206), vs `6.4922` (HPHYS0205).
  - WP: `0.0573` (HPHYS0207) vs `2.2445` (HPHYS0206), vs `1.8894` (HPHYS0205).
- Residual direction vs HPARITY02 baseline:
  - fail counts match (`27`, `1`);
  - mean abs diffs are equal within floating-point noise.

## Verdict
- Required rerun evidence exists and is internally consistent.
- HPHYS0207 no-regression criterion is satisfied.
- Hold-lift criteria are not met; `HOLD` disposition verified.
