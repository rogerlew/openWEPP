# simimpl04 contract derived test plan

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- Test requirements were extracted from SIMIMPL03 authority closure artifacts and canonical contracts:
  - `SC-WATBAL-001` (`INV-WATBAL-018`, `INV-WATBAL-019`, `INV-WATBAL-020`)
  - `SC-SYSTEM-001` (`INV-SYSTEM-018`, `INV-SYSTEM-019`, `INV-SYSTEM-020`)
  - `SC-INFILE-WEPPUI-001` (`D-WUI-005`, `G-WUI-008`, `G-WUI-009`, `WUI-E-005`)
- SIMIMPL04 package is tests-and-gates only; no production path edits are permitted.

## Contract-derived test files and obligations
| Test file | Primary obligations | Expected current state |
|---|---|---|
| `crates/openwepp-runner/tests/simimpl04_runner_kernel_execution_contract.rs` | Enforce SIMPIPE execution-ownership manifest closure and typed guard linkage (`HS-SIMPIPE-E-001`, `WS-SIMPIPE-E-001`). | expected fail |
| `crates/openwepp-runner/tests/simimpl04_wepp_ui_mode_closure_contract.rs` | Enforce requested/effective/selected-lane closure and `WUI-E-005` linkage (`D-WUI-005`, `G-WUI-008`). | expected fail |
| `crates/openwepp-runner/tests/simimpl04_wb13_publication_contract.rs` | Enforce simulation-owned WB13 publication provenance and no projection fallback (`HS-SIMOUT-E-001`, `WS-SIMOUT-E-001`, `G-WUI-009`). | expected fail |

## Execution posture
- Tests are implemented as integration tests and marked `#[ignore]` with explicit expected-fail rationale so default package test runs remain stable.
- SIMIMPL04 executes ignored tests explicitly to record fail-state evidence before SIMIMPL05 code integration.

## Ran
- Test files created and formatted.
- Compilation verified with `--no-run`.
