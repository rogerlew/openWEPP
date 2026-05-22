# Hillslope Consumer Wiring Implementation Evidence (SR06)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Implemented explicit hillslope consumer adapter identity and request wiring.
- Implemented typed consumer-boundary required-symbol validation (`HS-CONSUMER-E-001`) with no silent defaults.
- Added dedicated SR06 integration tests for boundary wiring and typed failure behavior.

Ran:
- Full required SR06 gates passed after implementation.

## Code Changes

1. Kernel contract: add consumer adapter identity on hillslope requests.
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:335`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:361`

2. Hillslope orchestrator: encode consumer boundary contract and validators.
- Required symbol families and per-adapter required sets:
  - `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:21`
- Typed consumer boundary error:
  - `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:103`
- Phase->adapter mapping:
  - `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:143`
- Active required-symbol resolution:
  - `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:165`
- Boundary validation function:
  - `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:203`
- Scheduler wiring + typed status emission:
  - `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:731`

3. Integration coverage: new SR06 test module.
- `/home/workdir/openWEPP/tests/integration/hillslope_consumer_boundary_integration.rs:1`

4. Workspace test registration for explicit integration-test manifest.
- `/home/workdir/openWEPP/Cargo.toml:164`

## Behavioral Evidence

1. Consumer adapter identity is passed to kernel on each phase request.
- Verified via `consumer_adapter_boundaries_receive_runtime_seam_symbols`.

2. Combined slope+soil runtime seams satisfy all consumer boundary requirements.
- Verified via per-phase required-symbol assertions in the same test.

3. Missing required soil symbol yields typed phase failure before kernel invocation.
- Verified via `missing_soil_consumer_symbol_fails_with_typed_missing_input_status`.

4. Missing required runoff slope symbol yields typed failure at `runoff_reconciliation` boundary.
- Verified via `missing_runoff_slope_symbol_fails_at_runoff_reconciliation_boundary`.
