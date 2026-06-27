# Line-Count Governance Checklist

Evidence mode: Ran.

Command:

`wc -l crates/openwepp-meteorology/src/phase.rs tools/snowfreeze_observed/phase_partition_snowdepth_adjudication.py tests/integration/snowdensity10_3_5c_phase_partition_snowdepth_impact.rs`

Result:

- `crates/openwepp-meteorology/src/phase.rs`: `558`
- `tools/snowfreeze_observed/phase_partition_snowdepth_adjudication.py`: `668`
- `tests/integration/snowdensity10_3_5c_phase_partition_snowdepth_impact.rs`: `104`

Disposition: PASS. No touched `.rs` file is near the 2000-line warning threshold.
