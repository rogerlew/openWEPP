# Line-Count Governance Checklist

Status: `EXECUTED-WARN-NO-BLOCK`

Evidence mode: `Ran` `wc -l` on every touched/new Rust file after formatting.

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 2,350 | WARN; pre-existing large CLI, bounded 82-line authority binding |
| `crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs` | 817 | PASS |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | 2,971 | WARN; pre-existing 2,969-line test owner changed by two lines only |
| `crates/openwepp-watershed-orchestrator/src/lib.rs` | 476 | PASS |
| `.../kernel/diagnostics.rs` | 620 | PASS |
| `.../kernel/direct.rs` | 2,325 | WARN; pre-existing 2,310-line owner grew by 15 lines only |
| `.../kernel/direct_tests.rs` | 1,951 | PASS |
| `.../kernel/helpers.rs` | 1,067 | PASS |
| `.../kernel/hourly.rs` | 1,734 | PASS; new cohesive owner |
| `.../kernel/hourly_tests.rs` | 985 | PASS; new focused tests |
| `.../kernel/kernel_core.rs` | 22 | PASS |
| `.../kernel/routing.rs` | 214 | PASS |
| `.../kernel/routing/01_ws22_ws23_ws26_detachment.rs` | 1,923 | PASS |
| `.../kernel/routing/02_ws20_segment_routing.rs` | 1,285 | PASS |
| `.../kernel/types.rs` | 302 | PASS |
| `.../lib_mod/mod.rs` | 26 | PASS |
| `.../network_frame.rs` | 987 | PASS |
| `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` | 1,163 | PASS |

No touched/new file reaches 3,000 lines. Warnings are the pre-existing
`direct.rs`, runner CLI, and runner behavior-test owners. W11B deliberately
placed the interval implementation in `hourly.rs` rather than expanding
`direct.rs`; runner changes are bounded. Owner: watershed and runner
maintainers.
Decomposition plan/sunset: move the remaining legacy event-lane assembly out of
`direct.rs` when its next substantive package touches that lane; W11B has
already stopped growth by leaving only the activation call and module include.
