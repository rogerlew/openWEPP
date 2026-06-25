# Pre-Implementation Evidence

Evidence mode: Static.

## Existing Benchmark Surface

The CLIM06 frost integration suite already publishes the state needed for
no-migration heat-flow checks:

- `frost.hourly.qsrf_w_m2_####`
- `frost.hourly.quf_w_m2_####`
- `frost.hourly.ksrf_w_m_k_####`
- `frost.hourly.surface_temp_c_####`
- `frost.hourly.snow_depth_m_####`
- `frost.hourly.residue_depth_m_####`
- `frost.hourly.tilled_frozen_depth_m_####`
- `frost.hourly.untilled_frozen_depth_m_####`
- `frost.runtime_slfsd_m_*`
- `frost.runtime_slsic_m_*`
- `frost.runtime_watbtm_m`

## No-Qwet Baseline

`rg -n "qwet|Qwet|frzftp" crates tests/integration tools` before production
edits found no production Rust implementation of `qwet` or `frzftp`; only docs
and the A classifier mention `Qwet` as non-authorized future work. B therefore
benchmarks the current no-migration heat-flow column.

## Anti-Tuning Constraint

SNOWFROST-FIDELITY-A classified zero sites as eligible for frost-model defect
attribution. This package may add benchmark tests only. It cannot use field
residuals to tune heat-flow, frozen conductivity, SFCC/impedance, or migration
heat.
