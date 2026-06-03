# H1 Day-36 Melt-Forcing Diagnostics

Status: completed/HOLD
Evidence mode: ran

Ran:

- Script: `docs/work-packages/20260603-hphys0271-day36-melt-forcing-lineage-closure-001/artifacts/hphys0271_diagnostics.py`
- Run root: `/tmp/hphys0271_full_20260603T212901Z`
- Classification report: `/tmp/hphys0271_full_20260603T212901Z/reports/hphys0271_day36_melt_forcing_classification.md`
- Classification JSON: `/tmp/hphys0271_full_20260603T212901Z/reports/hphys0271_day36_melt_forcing_classification.json`

## H1 Day-36 Result

| Classification | Cand RM | Base RM | RM Diff | Cand Snow-Water | Base Snow-Water | Snow-Water Diff | Σ Raw Melt | Redistributed Melt | Reconstruction Error |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `DAY36_MELT_TERMS_RECONSTRUCT_RAW_MELT_WITH_WAT_DIVERGENCE` | 28.175296 | 0.000000 | 28.175296 | 52.455535 | 81.360000 | -28.904465 | 0.053975 | 0.027105 | 0.000000 |

## H1 Highest-Magnitude Hour

| Hour | Raw Reconstructed | Raw Trace | Redistributed | amelt | bmelt | cmelt | dmelt | Air C | Dewpoint C | Rad MJ/m2 | Cloud | Wind m/s | Branch |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0012 | 0.009198 | 0.009198 | 0.002374 | 0.359696 | -0.000543 | 0.002860 | 0.000093 | 1.577290 | -1.500000 | 59.258047 | 0.000000 | 2.800000 | 1.000000 |

Interpretation: H1 day-36 raw melt is internally closed to the `melt.for` term sum. The next correction target is baseline-vs-openWEPP hourly forcing lineage, especially radiation/cloud/branch timing, not WB13 publication or WB17 `Ep`.
