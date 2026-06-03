# H1 Day-36 Radiation Diagnostics

Status: completed
Evidence mode: static + ran

Static:

- HPHYS0271 before-fix evidence reported H1 day-36 hour `0012` radiation as
  `59.258047 MJ m^-2 h^-1`, which is Langley-scale and physically impossible
  under the published unit.

Ran:

- Script:
  `docs/work-packages/20260603-hphys0272-hourly-radiation-unit-closure-001/artifacts/hphys0272_diagnostics.py`
- Run root: `/tmp/hphys0272_full_20260603T221209Z`
- Classification report:
  `/tmp/hphys0272_full_20260603T221209Z/reports/hphys0272_hourly_radiation_unit_classification.md`

## H1 Day-36 Before/After

| Metric | HPHYS0271 Before | HPHYS0272 After |
| --- | --- | --- |
| Max radiation hour | `0012` | `0012` |
| Max radiation `MJ m^-2 h^-1` | `59.258047` | `2.388678` |
| Hours `>=10 MJ m^-2 h^-1` | not recorded | `0` |
| Any hour `>=50 MJ m^-2 h^-1` | `true` | `false` |
| Candidate RM `mm` | `28.175296` | `0.000000` |
| Baseline RM `mm` | `0.000000` | `0.000000` |
| Snow-Water diff `mm` | `-28.904465` | `-0.729168` |
| Raw melt sum `m` | `0.053975` | `-0.000169` |
| Redistributed melt sum `m` | `0.027105` | `0.000000` |

## Interpretation

- The radiation-unit fix removes the day-36 false melt-release event without
  changing snowmelt equations or applying radiation clipping.
- Remaining H1 day-36 differences are much smaller and belong to residual
  snowpack state, ET, storage, or publication lineage, not the HPHYS0271
  radiation-unit defect.
