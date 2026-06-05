# Reconstruction Evidence

Status: executed-hold
Evidence mode: Static + Ran

Static:
- Baseline branch source:
  `/workdir/wepp-forest_260430_baseline/src/winter.for:434-448`.
- Corrected openWEPP source:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4231-4276`.
- Baseline reconstruction applies the pinned branch scale
  `1 - ngtvML/pstvML` to signed hourly melt and sums reconstructed melt plus
  post-winter rain.
- Corrected openWEPP routes net positive melt and preserves separate snowpack
  state-loss lineage.

Ran:
- H1/H7/H39 targeted traces completed with `rc=0`.
- Reconstruction tolerance was `2.000 mm` absolute window-sum residual.

Result:
- No target window reconstructed baseline `RM` to tolerance.
- First-2013 reconstruction residuals: H1 `-7.591490 mm`, H7 `-4.997207 mm`,
  H39 `-7.393538 mm`.
- Spring-2014 reconstruction residuals: H1 `-43.685550 mm`, H7
  `-44.617950 mm`, H39 `-45.281781 mm`.
- Spring-2016 negative raw melt is immaterial and reconstruction residuals are
  H1 `-21.556542 mm`, H7 `-23.074032 mm`, H39 `-22.188750 mm`.

Interpretation:
- The rejected pinned-baseline negative-melt branch is a real contributor but
  is not a complete root-cause reconstruction for the target windows.
- HPHYS0297 therefore leaves all target windows in `UNRESOLVED` and points the
  next package to missing winter producer term/timing lineage rather than
  downstream compensation.
