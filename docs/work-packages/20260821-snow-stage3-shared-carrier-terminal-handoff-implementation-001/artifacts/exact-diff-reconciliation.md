# Exact Diff Reconciliation

Status: `RECONCILED / EXECUTED HOLD`

Static: the exact changed paths are the frozen write set plus package artifacts:

- `Cargo.toml`
- `crates/openwepp-coupled-time/src/error.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- `crates/openwepp-hillslope-orchestrator/src/v11_vegetation_consumer.rs`
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs`
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_tests.rs`
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs`
- `crates/openwepp-persisted-restart-v1/src/lib.rs`
- `crates/openwepp-persisted-restart-v1/src/snow_stage3_handoff.rs`
- `tests/integration/snow_stage3_shared_carrier_terminal_handoff_implementation.rs`
- package artifacts under this directory

Ran: `git diff --check` passed. No generated or unrelated path was found in
`git status --short`; source changes are the initially frozen package write set
plus the amended owner-wiring surfaces, identity/restart hardening, and
package-owned tests. Inside
`nix develop`, formatting, workspace check,
the focused typed endpoint tests, and package integration target passed. The
final critical frost profile is 390/391; its one failure is outside this
write set (the unchanged contract-document guard). No
commit was created because no commit was authorized; commit binding remains
pending by design.
