# Line-Count Governance Checklist

Status: **PASS**.

Ran:

`wc -l` on touched Rust files:

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-runner/src/hillslope/laned_shadow.rs` | 476 | PASS |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 2693 | WARN, below 3000-line mandatory refactor trigger; existing include-slice file. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs` | 1183 | PASS |
| `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | 1256 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` | 897 | PASS |
| `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs` | 526 | PASS |

Static: No touched `.rs` file exceeds the 3000-line refactor-before-closure
threshold. The existing `00_builders_and_authority.rs` include-slice remains
above the 2000-line warning threshold but below the mandatory split threshold.
