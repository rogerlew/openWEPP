# Line-Count Governance Checklist

Status: `EXECUTED-PASS`

Evidence mode: `Ran` after formatting.

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-input-contract/src/parsers/chaninp.rs` | 930 | PASS |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/hourly.rs` | 2,064 | WARN accepted; existing cohesive WS11 interval owner, and splitting the recurrence/storage helpers in a numerical defect package would enlarge review surface without crossing the mandatory 3,000-line refactor threshold |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/hourly_tests.rs` | 1,379 | PASS |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs` | 1,457 | PASS; includes bounded terminal-publication unit tests |
| `crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs` | 1,541 | PASS |
| `tests/integration/infile_chaninp_parser_contract.rs` | 431 | PASS |
| `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` | 1,167 | PASS |

No touched Rust file reaches 3,000 lines. The implementation remains in the
existing parser, hourly-kernel, and network-publication owners; no new wrapper,
shadow path, or decomposition follow-up is required.
