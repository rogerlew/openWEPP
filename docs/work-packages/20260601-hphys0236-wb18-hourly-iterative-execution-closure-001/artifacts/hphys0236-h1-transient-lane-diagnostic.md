# HPHYS0236 H1 Transient Lane Diagnostic

Status: completed  
Evidence mode: mixed (`Ran` + `Static`)

## Diagnostic Question

After migrating WB18 hourly iterative execution shape, what remains unresolved
in `H1` early-transient `Dp` behavior?

## Static Attribution

1. Baseline authority (`watbal_hourly` + `purk`) remains iterative `ui_LFtstp`
   per-day percolation execution.
2. HPHYS0236 landed hourly iterative recompute in openWEPP WB18 production
   kernel (`run_percolation`) using `wb18_perc_lane_substeps`.
3. The migration slice is intentionally WB18-local; full hourly substep forcing
   lineage across coupled families (for example WB14/WB11 distribution effects)
   was out of scope for this package.

## Ran Evidence

Joined from:
- candidate:
  `/tmp/hphys0236_20260601T230600Z/parity/hillslope_output/H1.wat.parquet`
- baseline:
  `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H1.parquet`

`H1` day-1..7 `Dp` (`mm/day`), year-aligned with candidate year offset `+2012`:

| day | candidate | baseline | ratio |
| ---: | ---: | ---: | ---: |
| 1 | `44.24439883715175` | `0.24` | `184.35166182146563` |
| 2 | `40.54547616416101` | `0.24` | `168.93948401733755` |
| 3 | `30.670601834316557` | `0.24` | `127.79417430965232` |
| 4 | `21.991933296858946` | `0.24` | `91.63305540357895` |
| 5 | `15.767278033768376` | `0.24` | `65.69699180736824` |
| 6 | `11.473462607675053` | `0.24` | `47.80609419864606` |
| 7 | `8.571260655197706` | `0.24` | `35.71358606332378` |

Day-1..7 mean ratio: `103.13357823162465`.

## Root Cause

Inference:
1. WB18 iterative migration alone does not close `H1` transient `Dp`.
2. Remaining mismatch is likely in coupled hourly forcing/distribution lineage
   outside this WB18-local slice.

## Closure Implication

Next hold-lift slice should target coupled hourly pathway authority (WB11/WB14
interaction surfaces) with rerun readjudication.
