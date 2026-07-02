# Line-Count Governance

Status: `EXECUTED`

Counts from `wc -l`:

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 2120 | Existing `WARN`; W4DC01 reduced production routing logic there. |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | 2405 | Existing `WARN`; W4DC01 added source guard coverage. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/dispatch.rs` | 565 | OK. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs` | 1417 | OK. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs` | 1117 | OK. |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs` | 1241 | OK. |

Governance:

- No touched Rust file exceeded the `3000+` refactor threshold.
- New production routing logic lives in orchestrator modules rather than
  expanding the public CLI binary.
- Existing warning-sized CLI/test files remain below the hard threshold.
