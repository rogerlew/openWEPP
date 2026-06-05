# HPHYS0300 Raw/Post-Raw Melt Lineage Summary

Ran:

- Run root: `/tmp/hphys0300_full_20260605T155527Z`
- Candidate HEAD: `ab0801b58a4a038eda780ce5a108c27ea263a5d6`
- Corrected partition source: `docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/artifacts/corrected-partition-ledger.json`
- Contract authority: `SC-SNOWFREEZE-001#INV-SNOWFREEZE-031` and `SC-WATBAL-001#INV-WATBAL-075`.
- Scope: all nine H1/H7/H39 target windows plus same-HEAD full H1..H39 metrics.

## Route Counts

- `corrected-depth-hourly-forcing-hold`: `1` windows
- `post-raw-routing-without-baseline-negative-melt-hold`: `1` windows
- `raw-hourly-melt-term-state-hold`: `7` windows

## HPHYS0299 Cut-Point Counts

- `hourly-forcing`: `1` windows
- `negative-melt-correction`: `1` windows
- `raw-hourly-melt`: `7` windows

## Ledger

| Hill | Window | Days | HPHYS0299 Cut | HPHYS0300 Route | Raw Δ | Post Δ | Post-Raw Δ | Base Neg | Open Neg | Edit? |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | first-abs-storage-ge-10mm | 2013 112-127 | raw-hourly-melt | raw-hourly-melt-term-state-hold | 2.296420 | 14.945848 | 12.649427 | 0.000000 | -11.533388 | False |
| H1 | spring-2014 | 2014 120-146 | raw-hourly-melt | raw-hourly-melt-term-state-hold | 45.939224 | 67.473661 | 21.534437 | 0.000000 | -28.308587 | False |
| H1 | spring-2016 | 2016 104-111 | raw-hourly-melt | raw-hourly-melt-term-state-hold | 12.878990 | 15.269307 | 2.390317 | 0.000000 | -0.224814 | False |
| H7 | first-abs-storage-ge-10mm | 2013 112-127 | negative-melt-correction | post-raw-routing-without-baseline-negative-melt-hold | -0.165206 | 11.698650 | 11.863857 | 0.000000 | -12.330078 | False |
| H7 | spring-2014 | 2014 120-146 | raw-hourly-melt | raw-hourly-melt-term-state-hold | 44.101662 | 65.157003 | 21.055340 | 0.000000 | -30.016628 | False |
| H7 | spring-2016 | 2016 104-111 | raw-hourly-melt | raw-hourly-melt-term-state-hold | 14.431077 | 16.883726 | 2.452650 | 0.000000 | -0.255930 | False |
| H39 | first-abs-storage-ge-10mm | 2013 97-112 | hourly-forcing | corrected-depth-hourly-forcing-hold | -16.144365 | 11.370185 | 27.514550 | 0.000000 | -10.581207 | False |
| H39 | spring-2014 | 2014 120-146 | raw-hourly-melt | raw-hourly-melt-term-state-hold | 48.072940 | 69.768900 | 21.695961 | 0.000000 | -29.319266 | False |
| H39 | spring-2016 | 2016 104-111 | raw-hourly-melt | raw-hourly-melt-term-state-hold | 13.551643 | 15.955163 | 2.403520 | 0.000000 | -0.243386 | False |


## Disposition

- Production edits are not authorized by this run because raw/post-raw rows still have `term_state_evidence_status = aggregate-only`.
- Seven windows require paired `melt.for` term/state evidence before raw-hourly-melt migration or correction.
- H7 first-2013 remains a post-raw routed-melt hold, not legacy-defective acceptance, because `baseline_negative_raw_melt_sum_mm = 0.0`.
- H39 first-2013 remains a corrected-depth hourly-forcing seam and must be handled separately from raw/post-raw melt closure.
- WB17/WB18/WB19/WB13 compensation remains prohibited by `INV-WATBAL-075`.
