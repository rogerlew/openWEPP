# Line Count Governance Checklist

Ran: touched Rust file line counts after refactor:

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs` | 1255 | below 2000 |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs` | 10 | below 2000 |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/core_types.rs` | 588 | below 2000 |

Static: no touched Rust file reaches the 2000-line WARN threshold.

Disposition: line-count governance passed.
