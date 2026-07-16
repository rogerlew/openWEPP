# ASSURE-04D Line-Count Governance

Status: focused closure PASS

Evidence class: Ran

| Production Rust file | Intake lines | Intake disposition |
| --- | ---: | --- |
| `crates/openwepp-assurance/src/cli.rs` | 352 | PASS |
| `crates/openwepp-assurance/src/error.rs` | 72 | PASS |
| `crates/openwepp-assurance/src/lib.rs` | 21 | PASS |
| `crates/openwepp-assurance/src/v2.rs` | 2,436 | WARN — split new publication logic into `v2/publication.rs` |
| `crates/openwepp-assurance/src/v2/assembly.rs` | 1,732 | PASS — expose only the minimum internal rendering/check seam |
| `crates/openwepp-assurance/src/v2/confined.rs` | 889 | PASS |

No intake production Rust file reaches the 3,000-line closure block. Terminal
counts and touched-file CRAP maxima replace this baseline after implementation.

## Focused Closure Counts

| Production Rust file | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-assurance/src/cli.rs` | 508 | PASS |
| `crates/openwepp-assurance/src/error.rs` | 72 | PASS |
| `crates/openwepp-assurance/src/lib.rs` | 24 | PASS |
| `crates/openwepp-assurance/src/v2.rs` | 2,984 | WARN — below the block; lifecycle/identity helpers were split out |
| `crates/openwepp-assurance/src/v2/assembly.rs` | 1,747 | PASS |
| `crates/openwepp-assurance/src/v2/confined.rs` | 1,262 | PASS |
| `crates/openwepp-assurance/src/v2/lifecycle.rs` | 146 | PASS |
| `crates/openwepp-assurance/src/v2/planner.rs` | 1,182 | PASS — unchanged |
| `crates/openwepp-assurance/src/v2/publication.rs` | 2,903 | WARN — below the block; decompose before further lifecycle expansion |

No nonexempt production Rust file reaches 3,000 lines. Fresh adjudicated CRAP
remains a Phase 5 closure gate and is not inferred from line count.

## Post-CRAP-HOLD Remediation Counts

| Production Rust file | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-assurance/src/cli.rs` | 661 | PASS |
| `crates/openwepp-assurance/src/error.rs` | 72 | PASS |
| `crates/openwepp-assurance/src/lib.rs` | 24 | PASS |
| `crates/openwepp-assurance/src/v2.rs` | 2,821 | PASS — review lifecycle validation moved to its owned module |
| `crates/openwepp-assurance/src/v2/assembly.rs` | 1,747 | PASS |
| `crates/openwepp-assurance/src/v2/confined.rs` | 1,293 | PASS |
| `crates/openwepp-assurance/src/v2/lifecycle.rs` | 349 | PASS |
| `crates/openwepp-assurance/src/v2/planner.rs` | 1,182 | PASS — unchanged |
| `crates/openwepp-assurance/src/v2/publication.rs` | 2,982 | WARN — below the block; no further growth is permitted in this package |

No nonexempt production Rust file reaches 3,000 lines after the CRAP
remediation. The independent heavy runner must renew these counts.
