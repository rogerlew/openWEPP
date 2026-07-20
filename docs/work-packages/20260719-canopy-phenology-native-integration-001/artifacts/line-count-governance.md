# Line-Count Governance

Evidence mode: `Ran`

Status: `PASS — no touched non-generated Rust file is 3,000 lines or larger`

Counts are from exact terminal head `7053a9b8` for the 17 touched production
paths authenticated by the global CRAP report.

| Lines | Status | Path |
|---:|---|---|
| 2,994 | WARN | `crates/openwepp-input-contract/src/parsers/management.rs` |
| 2,975 | WARN | `crates/openwepp-gate-planner/src/planner.rs` |
| 2,934 | WARN | `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` |
| 2,698 | WARN | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs` |
| 2,554 | WARN | `crates/openwepp-gate-planner/src/verifier.rs` |
| 2,332 | WARN | `crates/openwepp-runner/src/hillslope/03_tests.rs` |
| 1,886 | PASS | `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` |
| 1,771 | PASS | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` |
| 1,734 | PASS | `crates/openwepp-management-schema/src/lib.rs` |
| 1,709 | PASS | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs` |
| 1,519 | PASS | `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs` |
| 1,330 | PASS | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs` |
| 1,327 | PASS | `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00d_authority_runtime_impl.rs` |
| 1,253 | PASS | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs` |
| 1,199 | PASS | `crates/openwepp-plant-phenology/src/lib.rs` |
| 707 | PASS | `crates/openwepp-landuse-migrate/src/convert.rs` |
| 617 | PASS | `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs` |

The initial exact terminal head exposed two mandatory blockers: `planner.rs`
at 3,047 lines and `00_builders_and_authority.rs` at 3,012 lines. Commit
`14e470eb` moved the multi-package inventory test to the integration suite and
moved crop-schedule authority into the existing authority-runtime module.
Focused tests, strict Clippy, formatting, and independent review found no
behavior change.

The six WARN files are below the closure blocker but require decomposition
intent before growth:

- split management YAML parsing/validation from legacy management parsing;
- split planner request/reconciliation responsibilities from gate selection;
- continue moving typed runtime authorities out of the builder aggregation;
- divide direct runtime frames by hydrology, plant/residue, and publication
  ownership;
- separate verifier inventory reconstruction from receipt/artifact checks; and
- divide runner tests by native-canopy, runtime-selection, and publication
  concern.

Those splits belong to the next package that materially edits each file; this
package does not add speculative seams after exact terminal evidence.
