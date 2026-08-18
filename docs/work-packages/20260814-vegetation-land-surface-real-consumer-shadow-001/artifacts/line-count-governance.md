# Line Count Governance

Status: `focused snapshot / terminal recount required`

Exact counts after the focused implementation increment:

| Rust file | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-land-surface-energy/src/solver.rs` | 2,597 | WARN; established solver facade below hard stop |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` | 1,849 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs` | 939 | PASS |
| `crates/openwepp-biogeochemistry/src/lib.rs` | 875 | PASS |
| `crates/openwepp-vegetation/src/v9_state.rs` | 467 | PASS |

No affected Rust file reaches the 3,000-line hard stop. The solver remains a
facade over its separately split test module; the Child-4 change is a bounded
inactive-equation branch and does not justify another production split in this
package.
