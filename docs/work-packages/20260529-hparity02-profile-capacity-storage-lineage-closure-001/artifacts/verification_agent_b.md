# HPARITY02 Verification Agent B

Status: completed  
Evidence mode: Ran

## Verification checks
1. Recomputed profile/control fail-hillslope counts from
   `/tmp/hparity02_20260529T204555Z/parity/reports/semantic/H*.semantic.json`.
2. Revalidated row-presence constraints (`common=1461`, no unmatched rows) for
   all 39 hillslopes.
3. Revalidated package summary rollup file:
   `/tmp/hparity02_20260529T204555Z/parity/reports/hillslope_semantic_summary.json`.

## Confirmed outcomes
- `ProfileDepth`: `0` fail hillslopes.
- `ProfilePorosityCap`: `0` fail hillslopes.
- `ProfileFCStore`: `27` fail hillslopes.
- `ProfileWPStore`: `1` fail hillslope.
- Control regression signal:
  - `Q`: `39` fail hillslopes.
  - `QOFE`: `39` fail hillslopes.

## Verdict
- `MEASURE-HP02-001` and `MEASURE-HP02-003` pass.
- `MEASURE-HP02-002` and `MEASURE-HP02-004` fail.
- Final package disposition `HOLD` verified.
