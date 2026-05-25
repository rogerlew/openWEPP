# simimpl18-first-day-rain-snow-partition-diagnostic

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Diagnostic compares first shared key (`OFE=1`, `J=1`, `Y=1`) under identical
  shared fixture inputs with baseline-year policy materialization enabled.

## Ran
- Evidence inputs:
  - `artifacts/replay-run-20260525T132822Z/suite_parquet/investigation/baseline_wat_year_policy.dat`
  - `artifacts/replay-run-20260525T132822Z/candidate/H5.wat.dat`
- First-key values:
  - `P`: baseline `4.4`, candidate `4.4` (match)
  - `RM`: baseline `0.0`, candidate `4.4` (delta `+4.4`)
  - `Snow-Water`: baseline `4.4`, candidate `250.0` (delta `+245.6`)
  - `Total-Soil`: baseline `102.7`, candidate `76.0` (delta `-26.7`)
  - `frozwt`: baseline `1.22`, candidate `0.0` (delta `-1.22`)
  - `SoilWaterTotal`: baseline `103.92`, candidate `76.0` (delta `-27.92`)
  - `Ep`: baseline `0.0`, candidate `0.65`
  - `Es`: baseline `0.83`, candidate `0.352`
- Comparator lane corroboration:
  - `suite_parquet/investigation/h5_wat_semantic_comparator.json`
  - `suite_dat/investigation/h5_wat_semantic_comparator.json`

## Interpretation
- Day-1 partition/publication mismatch remains open in current production
  physics path.
