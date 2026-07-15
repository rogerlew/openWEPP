# ASSURE-03 Line-Count Governance

Status: PASS

Evidence class: Ran

| Touched Rust file | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-assurance/src/cli.rs` | 184 | PASS |
| `crates/openwepp-assurance/src/engine.rs` | 632 | PASS |
| `crates/openwepp-assurance/src/error.rs` | 55 | PASS |
| `crates/openwepp-assurance/src/hash.rs` | 33 | PASS |
| `crates/openwepp-assurance/src/lib.rs` | 15 | PASS |
| `tests/integration/assurance_dossier_build_contract.rs` | 686 | PASS |

`main.rs` is unchanged at 9 lines. Eight v1 compiler modules were deleted.
No touched or retained Rust file is at the 2,000-line warning threshold; no
3,000-line exception exists or is required.
