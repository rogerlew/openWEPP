# HPHYS0298 Paired Snow/RM Lineage Partition

Status: historical/superseded

Evidence mode: ran + static supersession

Static:

- Retrospective review `artifacts/review_claude_hrsnow_unit_artifact.md`
  found the HPHYS0298 `hrsnow` row paired baseline snowfall depth against
  openWEPP `snow_hourly_snowfall_water_equiv_sum_m`, a water-equivalent
  accounting surface.
- HPHYS0299 supersedes the HPHYS0298 all-window `hourly-forcing` migration
  inference with corrected depth-vs-depth `hrsnow` provenance.
- The ledger below is retained as historical output, not production migration
  authority.

Ran:

- Run root: `/tmp/hphys0298_full_20260605T000000Z`
- Baseline source: `/workdir/wepp-forest_260430_baseline` at `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- Baseline observe worktree: `/tmp/hphys0298_wepp_forest_obs`
- Candidate HEAD: `2e626969f7d0789ed80b2a3b4666fb6dc7689de8`
- Target windows: H1/H7/H39 spring snow/RM windows from SC-SNOWFREEZE-001#INV-SNOWFREEZE-029.

## Baseline Observe Identity

- H1: pass=`True`, bit-identical=`True`, H298 records=`48102`
- H7: pass=`True`, bit-identical=`True`, H298 records=`48387`
- H39: pass=`True`, bit-identical=`True`, H298 records=`48102`

## Verdict Counts

- `OPENWEPP-DEFECTIVE`: `9` windows

## First Cut-Point Counts

- `hourly-forcing`: `9` windows

## Ledger

| Hill | Window | Days | Verdict | First Cut-Point | First Symbols | Baseline RM | Candidate RM | Baseline-Open RM | Raw Snow Δ | Raw Melt Δ | Routed Melt Δ | Q Δ | Total-Soil Δ |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | first-abs-storage-ge-10mm | 2013 112-127 | OPENWEPP-DEFECTIVE | hourly-forcing | hrsnow | 176.290000 | 161.617424 | 14.672576 | 61.712030 | 2.296420 | 14.945848 | -0.000000 | 207.526646 |
| H1 | spring-2014 | 2014 120-146 | OPENWEPP-DEFECTIVE | hourly-forcing | hrsnow | 550.900000 | 487.337417 | 63.562583 | 72.932589 | 45.939224 | 67.473661 | -0.000000 | -1546.726174 |
| H1 | spring-2016 | 2016 104-111 | OPENWEPP-DEFECTIVE | hourly-forcing | hrsnow | 90.920000 | 75.643593 | 15.276407 | 14.985000 | 12.878990 | 15.269307 | 0.000000 | -857.235502 |
| H7 | first-abs-storage-ge-10mm | 2013 112-127 | OPENWEPP-DEFECTIVE | hourly-forcing | hrsnow | 194.240000 | 182.812732 | 11.427268 | 64.423280 | -0.165206 | 11.698650 | -0.000000 | 186.596716 |
| H7 | spring-2014 | 2014 120-146 | OPENWEPP-DEFECTIVE | hourly-forcing | hrsnow | 577.160000 | 515.360976 | 61.799024 | 72.436782 | 44.101662 | 65.157003 | -0.000000 | 294.808536 |
| H7 | spring-2016 | 2016 104-111 | OPENWEPP-DEFECTIVE | hourly-forcing | hrsnow | 152.610000 | 135.724574 | 16.885426 | 14.985000 | 14.431077 | 16.883726 | -0.000000 | -46.727989 |
| H39 | first-abs-storage-ge-10mm | 2013 97-112 | OPENWEPP-DEFECTIVE | hourly-forcing | hrrain,hrsnow | 52.280000 | 41.590702 | 10.689298 | 198.825733 | -16.144365 | 11.370185 | -0.000000 | 163.029542 |
| H39 | spring-2014 | 2014 120-146 | OPENWEPP-DEFECTIVE | hourly-forcing | hrsnow | 549.600000 | 483.844778 | 65.755222 | 72.436782 | 48.072940 | 69.768900 | -0.000000 | 973.943367 |
| H39 | spring-2016 | 2016 104-111 | OPENWEPP-DEFECTIVE | hourly-forcing | hrsnow | 99.230000 | 83.289837 | 15.940163 | 14.985000 | 13.551643 | 15.955163 | -0.000000 | 49.593766 |


## Source Provenance Payload

- Full per-window source provenance is embedded in `artifacts/paired-lineage-ledger.json` under `source_provenance`.
- Each provenance row records canonical symbol, openWEPP symbol, unit, baseline value, openWEPP value, delta, and source path/line reference.


## Interpretation

- The `OPENWEPP-DEFECTIVE` / `hourly-forcing` rows are historical HPHYS0298
  classifier output and are superseded for production migration purposes.
- The `hrsnow` deltas are non-authoritative because the openWEPP side used
  `snow_hourly_snowfall_water_equiv_sum_m`; canonical `hrsnow` parity requires
  an openWEPP snowfall-depth surface, as corrected in HPHYS0299.
- HPHYS0298 still records real residual windows, but it does not prove winter
  hourly precipitation-phase migration authority by itself.
- `LEGACY-DEFECTIVE` is reserved for the signed negative-melt correction case; it must show raw-lineage closure and material negative raw melt before the correction cut-point.
- `UNRESOLVED` remains a hold and cannot be converted into closure by downstream metric improvement.
