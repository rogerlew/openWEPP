# Line Count Governance

Status: `exact code-count HOLD snapshot at 1d1bb33d3`

Exact counts after the focused implementation increment:

| Rust file | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs` | 2,975 | WARN; established facade below hard stop |
| `crates/openwepp-land-surface-energy/src/solver.rs` | 2,607 | WARN; established solver facade below hard stop |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` | 1,905 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs` | 1,363 | PASS |
| `crates/openwepp-biogeochemistry/src/lib.rs` | 875 | PASS |
| `crates/openwepp-land-surface-energy/src/solver_tests.rs` | 817 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/strict_v8_endpoint.rs` | 804 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/strict_v8_endpoint_tests.rs` | 712 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs` | 968 | PASS |
| `crates/openwepp-vegetation/src/v9_state.rs` | 525 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | 229 | PASS |

No affected Rust file reaches the 3,000-line hard stop. The two WARN files are
existing facades. Follow-on split intent, before either file next grows: split
the LSE facade by open/covered public boundary modules while preserving reexports;
split covered constitutive/evaluation from Newton transaction assembly in the
solver facade (solver tests are already separate). Child 4 does not expand
either facade and does not perform that unrelated-risk refactor while held.
