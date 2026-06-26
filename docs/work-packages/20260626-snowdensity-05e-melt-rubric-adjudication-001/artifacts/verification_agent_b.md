# Verification Agent B

Evidence mode: Ran.

## Evidence Artifacts

- `artifacts/snotel-adjudication.json`
- `artifacts/snotel-adjudication.md`
- `artifacts/non-snotel-baseline.json`
- `artifacts/non-snotel-baseline.md`

## Result Checks

- SNOTEL diagnostic legacy: `robust_fail_count=13`,
  `robust_ordinal_score=61`.
- SNOTEL opt-in: `robust_fail_count=10`, `robust_ordinal_score=84`.
- SNOTEL disposition: `PROMOTION-CANDIDATE`.
- Non-SNOTEL: `openwepp_defective_cells=0`.
- Non-SNOTEL snow control: `SNOW_CONTROL_FAILED=3`, no paired observed snow on
  two sites.

## Boundary Checks

- Static source scan found production direct day-input builder still selecting
  `SnowMeltModel::LegacyCoe`.
- `--model` is confined to `openwepp-snowbench`.
- `dense_slow_melt_v1` remains diagnostic/negative-benchmark only.
