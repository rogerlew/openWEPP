# Line-Count Governance

Evidence mode: Static + Ran.

Status: R6E touched hard-threshold issue resolved.

Known starting issue:

- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  was `3234` lines at R6D closure.

R6E split direct-publication helpers into:

- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`;
- same-module `include!("04_direct_publication.rs")` from
  `crates/openwepp-runner/src/hillslope/mod.rs`.

Ran:

```bash
wc -l \
  crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  crates/openwepp-runner/src/hillslope/04_direct_publication.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs \
  crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs
```

Result:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 2787 | PASS: touched hard-threshold file is now below 3000. |
| `crates/openwepp-runner/src/hillslope/04_direct_publication.rs` | 376 | PASS. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | 2660 | WARN band; expanded for typed direct publication input API, below hard threshold. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs` | 803 | PASS. |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | 2322 | WARN band; expanded by one parsed climate projection field, below hard threshold. |

A complete R6 cutover claim still cannot defer line-count governance for any
future touched 3000+ file.
