# HPHYS0206 Verification Agent B

Status: completed  
Evidence mode: Ran

## Verification checks
1. Recomputed FC/WP fail-hillslope counts from
   `/tmp/hphys0206_20260530T032538Z/parity/reports/semantic/H*.semantic.json`.
2. Revalidated row-closure presence (`common=1461`, no unmatched rows) in
   summary rows for all 39 hillslopes.
3. Compared FC/WP fail-count and residual-magnitude metrics against:
   - `/tmp/hphys0205_20260530T022235Z/parity/reports/semantic/H*.semantic.json`
   - `/tmp/hparity02_20260529T204555Z/parity/reports/semantic/H*.semantic.json`

## Confirmed outcomes
- `ProfileFCStore` fail hillslopes: `39`.
- `ProfileWPStore` fail hillslopes: `39`.
- FC/WP mean abs diff averages (H1..H39):
  - FC: `7.2212` (HPHYS0206) vs `6.4922` (HPHYS0205).
  - WP: `2.2445` (HPHYS0206) vs `1.8894` (HPHYS0205).

## Verdict
- Required rerun evidence exists and is internally consistent.
- Hold-lift criteria are not met; `HOLD` disposition verified.
