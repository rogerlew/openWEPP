# Line-Count Governance

Status: `executed`

Evidence mode: `static`

Post-implementation line counts for touched Rust files sampled on the W5 path:

| File | Lines | Status |
| --- | ---: | --- |
| `crates/openwepp-watershed-orchestrator/src/lib.rs` | 377 | `PASS` |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/dispatch.rs` | 418 | `PASS` |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs` | 709 | `PASS` |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs` | 611 | `PASS` |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs` | 542 | `PASS` |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/validation.rs` | 14 | `PASS` |
| `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs` | 1479 | `PASS` |
| `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/types.rs` | 54 | `PASS` |
| `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` | 533 | `PASS` |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | 2465 | `WARN existing test file` |

The only file above 2000 lines is an existing runner integration test file
outside W5 production code. No touched file exceeds the 3000-line closure block.
