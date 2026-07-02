# Line-Count Governance

Status: `passed-for-hold`

Evidence mode: `Ran: wc -l`

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-runner/src/watershed_supervisor.rs` | `931` | OK |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | `2522` | WARN; existing broad watershed CLI behavior contract. W7 added one focused regression and did not push the file above the 3000-line closure block. Follow watershed CQR/test split queue for decomposition. |

No touched `.rs` file is at or above `3000` lines.
