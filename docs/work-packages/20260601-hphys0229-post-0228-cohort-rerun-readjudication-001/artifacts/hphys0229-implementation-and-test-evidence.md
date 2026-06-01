# HPHYS0229 Implementation and Test Evidence

Status: completed  
Evidence mode: Ran

## Implementation Summary

1. Prepared fresh run root:
   - `/tmp/hphys0229_20260601T175346Z/parity/`
2. Executed `openwepp-cli-hill` across `H1..H39` for `unpalatable-rind`.
3. Ran semantic comparator (`pl14s` tolerances, `--candidate-year-offset 2012`)
   for all 39 hillslopes.
4. Aggregated semantic summary artifacts and computed monitored-family deltas
   versus HPHYS0224 baseline summary.

## Rerun and Comparator Evidence

- Ran:
  - `cat /tmp/hphys0229_20260601T175346Z/parity/reports/hillslope_batch_status.tsv`
  - `cat /tmp/hphys0229_20260601T175346Z/parity/reports/semantic_status.tsv`
  - `jq -r '.comparison.common_row_count' /tmp/hphys0229_20260601T175346Z/parity/reports/semantic/H*.semantic.json | awk 'BEGIN{min=1e18;max=0;count=0} {if($1<min)min=$1; if($1>max)max=$1; count++} END{print "count="count" min="min" max="max}'`
- Result:
  - Hillslope rerun status: `39/39` succeeded (`rc=0`).
  - Comparator status: `39/39` succeeded (`rc=0`).
  - Row overlap check: `count=39 min=1461 max=1461`.

## Monitored-Family Delta Results (vs HPHYS0224)

- Compared:
  - `/tmp/hphys0224_20260601T054337Z/parity/reports/hillslope_semantic_summary.json`
  - `/tmp/hphys0229_20260601T175346Z/parity/reports/hillslope_semantic_summary.json`
- Outcome for all monitored families (`Dp`, `latqcc`, `Total-Soil`,
  `SoilWaterTotal`, `ProfileFCStore`):
  - `delta_fail_count = 0`
  - `delta_mean_abs_diff_mean = 0`
  - `delta_mean_abs_diff_max = 0`

## Closure Measure Mapping

- `MEASURE-HP229-001`: satisfied.  
- `MEASURE-HP229-002`: satisfied.  
- `MEASURE-HP229-003`: satisfied.  
- `MEASURE-HP229-004`: satisfied.  
- `MEASURE-HP229-005`: satisfied (see `gate-results.md`).
