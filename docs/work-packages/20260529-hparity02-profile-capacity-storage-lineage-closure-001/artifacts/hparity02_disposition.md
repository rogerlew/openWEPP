# HPARITY02 Disposition

Status: completed  
Evidence mode: Static + Ran

## Decision
- `HOLD`

## Closure measure evaluation
1. `MEASURE-HP02-001` (`ProfileDepth` fail count 39 -> 0): **pass**
2. `MEASURE-HP02-002` (`ProfilePorosityCap`, `ProfileFCStore`,
   `ProfileWPStore` fail counts 39 -> 0 each): **fail**
   - `ProfilePorosityCap`: `0` fail hillslopes (pass)
   - `ProfileFCStore`: `27` fail hillslopes (open)
   - `ProfileWPStore`: `1` fail hillslope (open)
3. `MEASURE-HP02-003` row-presence integrity (`1461` common rows, no
   baseline-only/candidate-only rows): **pass**
4. `MEASURE-HP02-004` control columns remain passing: **fail**
   - `Q`: `39` fail hillslopes
   - `QOFE`: `39` fail hillslopes

## Quantitative evidence
- Ran summary:
  `/tmp/hparity02_20260529T204555Z/parity/reports/hillslope_semantic_summary.json`
- Ran per-hillslope reports:
  `/tmp/hparity02_20260529T204555Z/parity/reports/semantic/H*.semantic.json`

## Interpretation
- HPARITY02 runtime projection closure materially reduced profile-family
  residuals from HPARITY01 baseline (`39` failures each) but did not close the
  family completely.
- `ProfileFCStore` remains the dominant open residual and must be resolved
  before HPARITY02 can lift `HOLD`.
