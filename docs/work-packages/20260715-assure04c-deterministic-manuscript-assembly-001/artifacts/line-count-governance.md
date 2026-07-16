# ASSURE-04C Line-Count Governance

Status: PASS — terminal focused measurement

Evidence class: Ran when measured

No touched production Rust file reaches the 3,000-line closure block:

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-assurance/src/lib.rs` | 21 | PASS |
| `crates/openwepp-assurance/src/error.rs` | 72 | PASS; dedicated recovery-error typing remains compact |
| `crates/openwepp-assurance/src/cli.rs` | 352 | PASS |
| `crates/openwepp-assurance/src/v2.rs` | 2,436 | WARN accepted; typed source admission remains cohesive and is 564 lines below the required split threshold |
| `crates/openwepp-assurance/src/v2/assembly.rs` | 1,732 | PASS; new assembly behavior is isolated from source admission |
| `crates/openwepp-assurance/src/v2/confined.rs` | 889 | PASS; descriptor-relative no-follow staging operations are isolated here |
| `crates/openwepp-assurance/src/v2/planner.rs` | 1,182 | PASS |

The `v2.rs` warning does not waive the 3,000-line block. ASSURE-04C added its
substantial new behavior in `v2/assembly.rs` and decomposed high-CRAP table and
binding functions without inflating the admission module toward the threshold.
