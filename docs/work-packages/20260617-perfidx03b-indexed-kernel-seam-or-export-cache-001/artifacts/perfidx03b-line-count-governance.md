# PERFIDX03B Line-Count Governance

Ran: `wc -l` on touched Rust files after implementation.

| File | Lines | Status | Disposition |
| --- | ---: | --- | --- |
| `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs` | 1968 | PASS | Below WARN threshold. |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 66 | PASS | Below WARN threshold. |
| `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` | 2062 | WARN | Existing large scheduler surface; PERFIDX03B adds a narrow persistent-state seam. Refactor not required because below 3000. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs` | 57 | PASS | Below WARN threshold. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs` | 958 | PASS | Below WARN threshold. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 2396 | WARN | Existing large runner setup surface; PERFIDX03B adds registry plumbing only. Refactor follow-on remains advisable, not blocking. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 577 | PASS | Below WARN threshold. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs` | 2648 | WARN | Existing large scheduler trace include; PERFIDX03B changes are localized to persistent lane preparation/replacement. Refactor not required because below 3000. |
| `crates/openwepp-runner/src/hillslope/symbol_registry_audit.rs` | 1013 | PASS | Below WARN threshold. |
| `crates/openwepp-runner/src/hillslope/tests03/publication/publication_scheduler_pl_activation.rs` | 155 | PASS | Below WARN threshold. |

No touched Rust file is at or above 3000 lines.

