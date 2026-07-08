# Line-Count Governance

Status: `passed`

Evidence mode: `Ran: wc -l`

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-runner/src/watershed_supervisor.rs` | `972` | OK |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | `2625` | WARN; existing broad watershed CLI behavior contract. W7R added one focused guard and did not push the file above the 3000-line closure block. |

No touched `.rs` file is at or above `3000` lines.
