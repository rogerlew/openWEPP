# Line Count Governance

Status: `focused snapshot / terminal recount required`

Exact counts after the focused implementation increment:

| Rust file | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs` | 2,975 | WARN; established facade below hard stop |
| `crates/openwepp-land-surface-energy/src/solver.rs` | 2,607 | WARN; established solver facade below hard stop |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` | 1,995 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs` | 1,326 | PASS |
| `crates/openwepp-biogeochemistry/src/lib.rs` | 875 | PASS |
| `crates/openwepp-land-surface-energy/src/solver_tests.rs` | 817 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/strict_v8_endpoint.rs` | 804 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/strict_v8_endpoint_tests.rs` | 712 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs` | 961 | PASS |
| `crates/openwepp-vegetation/src/v9_state.rs` | 500 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 229 | PASS |

No affected Rust file reaches the 3,000-line hard stop. The two WARN files are
existing facades; solver tests are already split and the shadow module remains
separate from the direct scheduler. The bounded inactive-equation and test-only
visibility changes do not justify destabilizing those facades in this package.
