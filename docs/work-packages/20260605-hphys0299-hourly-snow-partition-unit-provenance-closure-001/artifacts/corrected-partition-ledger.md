# HPHYS0299 Corrected Hourly Snow Partition Ledger

Ran:

- Run root: `/tmp/hphys0299_full_20260605T101220Z`
- Baseline source: `/workdir/wepp-forest_260430_baseline` at `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- Baseline observe worktree: `/tmp/hphys0298_wepp_forest_obs`
- Candidate HEAD: `bcbec46ccff8f16d38c2ff18b5dda98631d83009`
- Canonical `hrsnow` comparison: baseline `stmtim.for` snow depth vs openWEPP `snow_hourly_snowfall_depth_sum_m`.
- Rejected HPHYS0298 seam: `snow_hourly_snowfall_water_equiv_sum_m` is water equivalent, not canonical `hrsnow` depth.

## Baseline Observe Identity

- H1: pass=`True`, bit-identical=`True`, H298 records=`48102`
- H7: pass=`True`, bit-identical=`True`, H298 records=`48387`
- H39: pass=`True`, bit-identical=`True`, H298 records=`48102`

## Verdict Counts

- `OPENWEPP-DEFECTIVE`: `9` windows

## First Cut-Point Counts

- `hourly-forcing`: `1` windows
- `negative-melt-correction`: `1` windows
- `raw-hourly-melt`: `7` windows

## Ledger

| Hill | Window | Days | Verdict | First Cut-Point | First Symbols | Baseline RM | Candidate RM | Baseline-Open RM | Raw Snow Depth Δ | Raw Melt Δ | Routed Melt Δ | Q Δ | Total-Soil Δ |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | first-abs-storage-ge-10mm | 2013 112-127 | OPENWEPP-DEFECTIVE | raw-hourly-melt | hrmlt | 176.290000 | 161.617424 | 14.672576 | -0.000697 | 2.296420 | 14.945848 | -0.000000 | 207.526646 |
| H1 | spring-2014 | 2014 120-146 | OPENWEPP-DEFECTIVE | raw-hourly-melt | hrmlt | 550.900000 | 487.337417 | 63.562583 | -1.670312 | 45.939224 | 67.473661 | -0.000000 | -1546.726174 |
| H1 | spring-2016 | 2016 104-111 | OPENWEPP-DEFECTIVE | raw-hourly-melt | hrmlt | 90.920000 | 75.643593 | 15.276407 | -0.000000 | 12.878990 | 15.269307 | 0.000000 | -857.235502 |
| H7 | first-abs-storage-ge-10mm | 2013 112-127 | OPENWEPP-DEFECTIVE | negative-melt-correction | pstvML,ngtvML,wmelt | 194.240000 | 182.812732 | 11.427268 | -0.000697 | -0.165206 | 11.698650 | -0.000000 | 186.596716 |
| H7 | spring-2014 | 2014 120-146 | OPENWEPP-DEFECTIVE | raw-hourly-melt | hrmlt | 577.160000 | 515.360976 | 61.799024 | -1.670280 | 44.101662 | 65.157003 | -0.000000 | 294.808536 |
| H7 | spring-2016 | 2016 104-111 | OPENWEPP-DEFECTIVE | raw-hourly-melt | hrmlt | 152.610000 | 135.724574 | 16.885426 | -0.000000 | 14.431077 | 16.883726 | -0.000000 | -46.727989 |
| H39 | first-abs-storage-ge-10mm | 2013 97-112 | OPENWEPP-DEFECTIVE | hourly-forcing | hrrain,hrsnow | 52.280000 | 41.590702 | 10.689298 | 3.390758 | -16.144365 | 11.370185 | -0.000000 | 163.029542 |
| H39 | spring-2014 | 2014 120-146 | OPENWEPP-DEFECTIVE | raw-hourly-melt | hrmlt | 549.600000 | 483.844778 | 65.755222 | -1.670280 | 48.072940 | 69.768900 | -0.000000 | 973.943367 |
| H39 | spring-2016 | 2016 104-111 | OPENWEPP-DEFECTIVE | raw-hourly-melt | hrmlt | 99.230000 | 83.289837 | 15.940163 | -0.000000 | 13.551643 | 15.955163 | -0.000000 | 49.593766 |


## Interpretation

- If `hourly-forcing` remains after this corrected run, the residual is a true depth-vs-depth precipitation-partition difference and can authorize a focused producer package.
- If `hourly-forcing` closes, HPHYS0298's production-migration recommendation was a diagnostic unit/provenance defect and must not drive production code edits.
- Downstream WB17/WB18/WB19/WB13 compensation remains prohibited in both branches.
