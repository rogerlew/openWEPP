# H39 Hourly Semantic Parity Evidence

Status: hold

Evidence mode: ran

Static:
- Comparator tolerance config:
  `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`.

Ran:
- Command:
  `/workdir/wepppy/.venv/bin/python tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py --baseline-wat /tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/baseline_H39.parquet --candidate-wat /tmp/hphys0247_20260602T070132Z_final/hillslope_output/H39.wat.parquet --report-json /tmp/hphys0247_20260602T070132Z_final/reports/H39.semantic.json --tolerance-config tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json --candidate-year-offset 2012 --top-n 20`.
- Report:
  `/tmp/hphys0247_20260602T070132Z_final/reports/H39.semantic.json`.
- Result: `semantic_pass=false`, `common_row_count=1461`,
  `only_baseline_count=0`, `only_candidate_count=0`.
- Passing closure columns: `P`, `Er`, `frozwt`, `Area`,
  `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore`,
  `InterceptionStorage`.
- Failing closure columns after HPHYS0247 patch:
  - `Dp`: `fail_count=926`, `mean_abs=0.262718 mm`,
    `max_abs=23.809497 mm` at `[1,4,2013]`.
  - `Ep`: `fail_count=1460`, `mean_abs=1.388145 mm`,
    `max_abs=7.020000 mm` at `[1,183,2014]`.
  - `Es`: `fail_count=1461`, `mean_abs=3.499716 mm`,
    `max_abs=10.008919 mm` at `[1,179,2015]`.
  - `Q`: `fail_count=68`, `mean_abs=0.882300 mm`,
    `max_abs=77.220396 mm` at `[1,93,2014]`.
  - `QOFE`: `fail_count=68`, `mean_abs=0.882300 mm`,
    `max_abs=77.220396 mm` at `[1,93,2014]`.
  - `RM`: `fail_count=278`, `mean_abs=2.342016 mm`,
    `max_abs=78.410396 mm` at `[1,93,2014]`.
  - `Snow-Water`: `fail_count=635`, `mean_abs=63.849723 mm`,
    `max_abs=532.700000 mm` at `[1,108,2014]`.
  - `Total-Soil` and `SoilWaterTotal`: `fail_count=1453`,
    `mean_abs=76.572402 mm`, `max_abs=430.764834 mm` at `[1,145,2014]`.
  - `latqcc`: `fail_count=845`, `mean_abs=1.204451 mm`,
    `max_abs=8.130000 mm` at `[1,149,2014]`.
