# ASSURE-04B Line-Count Governance

Status: reviewed before independent review

Evidence class: Ran

| Rust file | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-assurance/src/v2.rs` | 2,064 | WARN; preexisting 2,042-line source-admission module, touched to factor structural validation and expose planner APIs |
| `crates/openwepp-assurance/src/v2/confined.rs` | 256 | PASS |
| `crates/openwepp-assurance/src/v2/planner.rs` | 1,114 | PASS |
| `crates/openwepp-assurance/src/engine.rs` | 622 | PASS |
| `crates/openwepp-assurance/src/cli.rs` | 285 | PASS |
| `crates/openwepp-assurance/src/lib.rs` | 20 | PASS |
| `tests/integration/assurance_v2_source_contract.rs` | 709 | PASS |
| `tests/integration/assurance_v2_planner_contract.rs` | 515 | PASS |

No touched Rust file reaches the 3,000-line closure block.

The planner was placed in its own module rather than appended to `v2.rs`.
`v2.rs` remains above the 2,000-line warning because its strict source-admission
types and validators are still one 04A unit. ASSURE-04C must begin by assessing
a behavior-preserving split between schema/source types, structural admission,
and content readers before adding assembly logic; the intent is to reduce
`v2.rs` below 2,000 without mixing that refactor into the new planner.
