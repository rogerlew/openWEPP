# Targeted H1/H7/H39 Radiation Metrics

Status: completed
Evidence mode: ran

Static: targeted diagnostics execute H1, H7, and H39 with HPHYS0245 trace
capture enabled through day 180.

Ran:

- Run root: `/tmp/hphys0272_full_20260603T221209Z`
- Trace status:
  `/tmp/hphys0272_full_20260603T221209Z/reports/targeted_trace_status.tsv`
- Classification JSON:
  `/tmp/hphys0272_full_20260603T221209Z/reports/hphys0272_hourly_radiation_unit_classification.json`

| Hill | Classification | Max Hour | Max Rad MJ/m2/hr | Σ Rad MJ/m2/day | Hours >= 10 | Any >= 50 | RM Diff mm | Snow-Water Diff mm | Ep Diff mm |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | `HOURLY_RADIATION_MJ_SCALE_CONFIRMED` | `0012` | `2.388678` | `14.879381` | `0` | `false` | `0.000000` | `-0.729168` | `-0.134747` |
| H7 | `HOURLY_RADIATION_MJ_SCALE_CONFIRMED` | `0012` | `0.633677` | `3.947257` | `0` | `false` | `0.000000` | `-0.732085` | `-0.156863` |
| H39 | `HOURLY_RADIATION_MJ_SCALE_CONFIRMED` | `0012` | `2.053842` | `12.793644` | `0` | `false` | `0.000000` | `-0.729168` | `-0.065568` |

Interpretation: all targeted day-36 radiation traces are on the MJ hourly
scale; no target retains the HPHYS0271 `59+ MJ m^-2 h^-1` artifact.
