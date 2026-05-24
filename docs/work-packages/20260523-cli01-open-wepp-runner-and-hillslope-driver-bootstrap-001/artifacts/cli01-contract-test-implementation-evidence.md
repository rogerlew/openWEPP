# CLI01 Contract-Test Implementation Evidence

Status: complete
Evidence mode: Static

## Static
- Added integration test file:
  - `tests/integration/cli01_runner_contract_derived_tests.rs`
- Registered integration test target in workspace root `Cargo.toml`:
  - `[[test]] name = "cli01_runner_contract_derived_tests"`
- Implemented contract-derived checks that assert:
  - canonical runner contract advertises required command surface and failure IDs,
  - canonical hillslope CLI spec contains required sidecars + manifest schema id,
  - binary release contract contains required validation fields,
  - sidecar adapter enforces missing-required hard-fail in strict and compat,
  - sidecar adapter emits compat warning for unknown discovery.

## Ran
- None in this phase.
