# Line-Count Governance

Static: pre-review inventory.

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-gate-planner/src/planner.rs` | 2,780 | WARN: below blocker; assurance construction/reconciliation was extracted into the new 553-line `assurance.rs`; continue moving root/context assembly in the next planner package that touches it. |
| `crates/openwepp-gate-planner/src/executor.rs` | 2,353 | WARN: pre-existing executor; this package changes only isolated test-fixture setup. Continue adapter/fixture decomposition in the next executor package. |
| `crates/openwepp-gate-planner/src/verifier.rs` | 2,305 | WARN: pre-existing and byte-unchanged; continue receipt/artifact verifier decomposition in the next verifier package. |

No touched or adjacent non-generated Rust file is at or above the 3,000-line
closure blocker. New production code is confined to `assurance.rs`; the new
integration contract is 311 lines.
