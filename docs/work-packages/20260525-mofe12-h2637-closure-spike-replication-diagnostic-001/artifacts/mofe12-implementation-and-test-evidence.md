# MOFE12 Implementation and Test Evidence

Status: complete
Evidence mode: Ran

Ran:
1. Incident baseline metric reconstruction checks from raw legacy output:
- Parsed `/home/workdir/wepp-forest/docs/ablation/20260430_uncapped-spectacular_h2637_hillslope_closure-spike/artifacts/repro/C000_baseline_output/H2637.wat.dat`.
- Verified incident-aligned legacy formula:
  - `err_legacy = (RM + Irr + UpStrmQ + SubRIn) - (QOFE + latqcc + Ep + Es + Er + Dp + Tile) - Δ(Total-Soil Water)`
- Verified known day-44 values:
  - `day44_hillslope_error_mm_legacy = -180.31779`
  - `day44_ofe19_err_legacy = -180.4590`

2. openWEPP candidate execution attempt (full staged climate):
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe2637_defect_diag/runs --run-file p2637.run --output-dir /tmp/openwepp_mofe2637_defect_diag/output_meta --policy compat`
- Result: execution exceeded 10 minutes at sustained CPU with no emitted outputs;
  lane was stopped for bounded diagnostics.

3. openWEPP bounded candidate execution (60-day climate subset):
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe2637_defect_diag/runs --run-file p2637.run --output-dir /tmp/openwepp_mofe2637_defect_diag/output_meta_60d --policy compat`
- Result: pass.
- Emitted outputs:
  - `/tmp/openwepp_mofe2637_defect_diag/output/H2637.wat.parquet`
  - `/tmp/openwepp_mofe2637_defect_diag/output/H2637.hbp`
  - `/tmp/openwepp_mofe2637_defect_diag/output/H2637.loss.json`
  - `/tmp/openwepp_mofe2637_defect_diag/output/H2637.plot.parquet`

4. Candidate metric extraction (duckdb + Python):
- Candidate manifest confirms WB13 publication policy:
  - `single-row-canonicalized-hillslope-aggregate`
  - `contributor_ofe_count=19`
  - `row_count=60`
- Candidate diagnostic values (same legacy formula on published row):
  - `day44_err_legacy = -194.75053419004115`
  - `day45_err_legacy = -215.80856517617104`
  - `max_abs_err_day (first 60 days) = julian 38, -229.8203322371234`

Static:
- No Rust source files were edited.
