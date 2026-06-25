# Line-Count Governance

Status: executed-hold

Evidence mode: Ran.

Ran:

```text
1090 crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs
16 crates/openwepp-runner/src/hillslope/mod.rs
1006 crates/openwepp-runner/src/hillslope/snowbench.rs
78 crates/openwepp-runner/src/bin/openwepp-snowbench.rs
39 crates/openwepp-runner/src/lib.rs
188 tests/integration/snowfrost_fidelity_g0_pysnobal_bridge_contract.rs
508 tools/snowfreeze_observed/pysnobal_compare.py
```

Disposition: no touched Rust file is at or above the 2000-line warning
threshold, and no file is at or above the 3000-line blocking threshold.
