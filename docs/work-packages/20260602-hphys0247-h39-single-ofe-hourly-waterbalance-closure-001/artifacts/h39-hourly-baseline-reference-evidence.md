# H39 Hourly Baseline Reference Evidence

Status: updated

Evidence mode: static + ran

Static:
- Physics authority: `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Baseline WAT partition:
  `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H39.parquet`.
- Baseline total soil alias is `Total-Soil Water`; semantic comparator maps it
  to `Total-Soil`.

Ran:
- Local parquet inspection confirmed baseline columns include `P`, `RM`, `Q`,
  `Ep`, `Es`, `Er`, `Dp`, `latqcc`, `Total-Soil Water`, `frozwt`,
  `Snow-Water`, `SoilWaterTotal`, profile stores, and
  `InterceptionStorage`.
- Comparator report confirmed `common_row_count=1461` and no baseline-only or
  candidate-only columns after alias normalization.
