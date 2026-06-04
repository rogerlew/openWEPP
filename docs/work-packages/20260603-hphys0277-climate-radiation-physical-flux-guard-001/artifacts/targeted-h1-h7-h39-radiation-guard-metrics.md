# HPHYS0277 Hourly Radiation Physical Guard Classification

Ran:

- Root: `/tmp/openwepp-hphys0277-radiation-guard`
- Anchor day: simulation day `36`.
- Classification JSON: `/tmp/openwepp-hphys0277-radiation-guard/reports/hphys0272_hourly_radiation_unit_classification.json`.

## Targeted Radiation Metrics

| Hill | Classification | Max Hour | Max Rad MJ/m2/hr | Σ Rad MJ/m2/day | Hours >= 10 | Any >= 50 | Air C | Dewpoint C | Raw Melt m | Redistributed Melt m |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | HOURLY_RADIATION_MJ_SCALE_CONFIRMED | 0012 | 2.388678 | 14.879381 | 0 | False | 1.577290 | -1.500000 | 0.000449 | 0.000000 |
| H7 | HOURLY_RADIATION_MJ_SCALE_CONFIRMED | 0012 | 0.633677 | 3.947257 | 0 | False | 1.577290 | -1.500000 | 0.000179 | 0.000000 |
| H39 | HOURLY_RADIATION_MJ_SCALE_CONFIRMED | 0012 | 2.053842 | 12.793644 | 0 | False | 1.577290 | -1.500000 | 0.000398 | 0.000000 |

## Day-36 WAT/Snowpack Context

| Hill | Year | Julian | Cand RM | Base RM | RM Diff | Cand Snow-Water | Base Snow-Water | Snow-Water Diff | Ep Diff | Max Abs Ep Diff |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | 2013 | 36 | 0.000000 | 0.000000 | 0.000000 | 80.630832 | 81.360000 | -0.729168 | -0.134747 | 7.778764 |
| H7 | 2013 | 36 | 0.000000 | 0.000000 | 0.000000 | 80.967915 | 81.700000 | -0.732085 | -0.156863 | 7.590000 |
| H39 | 2013 | 36 | 0.000000 | 0.000000 | 0.000000 | 80.630832 | 81.360000 | -0.729168 | -0.065568 | 7.014496 |

## Interpretation

- `HOURLY_RADIATION_MJ_SCALE_CONFIRMED` means no day-36 targeted hour retains the pre-fix `59+ MJ m^-2 h^-1` Langley-scale artifact.
- Remaining WAT residuals after this classification belong to snowpack state, melt term, ET, storage, or publication lineage and should not be compensated by radiation clipping.

## Guard Compatibility

- Targeted H1/H7/H39 runs completed with `rc=0`; valid HPHYS0272 radiation traces did not trip the physical flux guard.
- No targeted hour reached `10 MJ m^-2 h^-1`, and no Langley-scale `>= 50 MJ m^-2 h^-1` artifact was present.
