# Line Count Governance

Evidence class: Ran.

Command:

`wc -l crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs`

Result:

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs` | `2730` | warning, not touched by this package |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs` | `2124` | warning, touched; remains below 3000-line hard gate |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` | `651` | ok |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs` | `467` | ok |

The follow-up performance/parity package should not grow `runoff.rs` further
without splitting direct frost/no-material helpers into a smaller module.

