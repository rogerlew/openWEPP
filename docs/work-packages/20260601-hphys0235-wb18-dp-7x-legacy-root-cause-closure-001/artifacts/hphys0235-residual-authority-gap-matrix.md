# HPHYS0235 Residual Authority Gap Matrix

Status: completed  
Evidence mode: Ran

Reference files:
- Hourly candidate: `/tmp/hphys0234_20260601T215019Z/parity/hillslope_output/H1.wat.parquet`
- Daily candidate probe: `/tmp/hphys0235_probe/hillslope_output/H1.wat.parquet`
- Baseline: `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H1.parquet`

| Metric (`H1`) | Hourly lane | Daily lane | Baseline | Interpretation |
| --- | ---: | ---: | ---: | --- |
| `Dp` day-1..7 mean (`mm/day`) | `1.7423806497` | `0.2260067931` | `0.2400000000` | Hourly is ~7x legacy; daily is near legacy |
| `Dp` day-1..7 ratio vs baseline | `7.2599193737` | `0.9416949711` | `1.0` | Confirms lane-shaped mismatch |
| `Dp` day-1 min/max ratio vs baseline | `6.8620606454 / 7.5806146122` | `0.1421771895 / 1.4267928138` | `1.0 / 1.0` | Hourly ratio consistently elevated |
| `Total-Soil` day-7 (`mm`) | `242.1027692597` | `292.1733876895` | `338.22` | Hourly over-drains profile relative to baseline |

## Gap Decision

Dominant gap is unresolved WB18 hourly iterative semantics (`watbal_hourly`
shape). WB13 publication lineage is not the limiting surface for this
residual.
