# Line-count Governance

Status: `PASS`

Evidence class: `Ran`

Terminal counts for new/refactored files:

| File | Lines | Disposition |
| --- | ---: | --- |
| `openwepp-management-schema/src/forest_litter.rs` | 948 | PASS |
| `openwepp-input-contract/.../management/forest_data.rs` | 25 | PASS |
| `tests/integration/canopy_litter_external_boundary_contract.rs` | 397 | PASS |
| `openwepp-input-contract/src/parsers/management.rs` | 2,984 | PASS after extraction |
| `openwepp-runner/src/hillslope/03_tests.rs` | 2,890 | WARN / PASS after extraction |
| `openwepp-runner/.../00_builders_and_authority.rs` | 2,844 | WARN / PASS after extraction |
| `openwepp-runner/.../canopy_litter_boundary_helpers.rs` | 110 | PASS |
| `openwepp-runner/.../00e_native_canopy_trace.rs` | 164 | PASS |

Several touched runner/orchestrator files remain above the 2,000-line warning
threshold and receive `WARN`. The input parser and two runner files crossed
3,000 during implementation, so forest data, forcing-fixture helpers, and
native trace support were extracted before closure. No touched nonexempt Rust
file is 3,000 lines or longer.
