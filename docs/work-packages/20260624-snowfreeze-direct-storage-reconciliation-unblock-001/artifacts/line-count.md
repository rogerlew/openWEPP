# Line-Count Governance

Status: complete

Evidence mode: Ran.

Ran:

- `wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r3c_r4b.rs tests/integration/snowfreeze_observed_frost_depth_contract.rs`

Results:

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs` | 1523 | PASS, below 2000-line warning threshold. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r3c_r4b.rs` | 947 | PASS, below 2000-line warning threshold. |
| `tests/integration/snowfreeze_observed_frost_depth_contract.rs` | 225 | PASS, below 2000-line warning threshold. |

No 2000+ line warning or 3000+ line closure blocker applies.
