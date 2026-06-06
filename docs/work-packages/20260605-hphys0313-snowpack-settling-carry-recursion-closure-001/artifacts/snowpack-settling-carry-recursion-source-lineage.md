# HPHYS0313 Source Lineage

Status: complete

Evidence mode: static

Static:

- Baseline settle-day count: `/workdir/wepp-forest_260430_baseline/src/snowd.for:61-65`.
- Baseline cold settling equation: `/workdir/wepp-forest_260430_baseline/src/snowd.for:122-139`.
- Baseline cold no-snow `driftg` final-depth addition: `/workdir/wepp-forest_260430_baseline/src/snowd.for:145-146`.
- Baseline carry writeback: `/workdir/wepp-forest_260430_baseline/src/snowd.for:310-312`.
- openWEPP settle-day count and settling equations: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3872-3924`.
- Temporary diagnostic instrumentation patch: `fixed-baseline-settling-instrumentation.patch`.
