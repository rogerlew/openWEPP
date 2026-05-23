# Gate Results

Status: `completed`
Evidence mode: `Ran`

## Pre-Implementation Contract Gate
- `cargo test --test arch22_typed_state_surface_contract`
  - result: **fail** at compile stage (`E0432` unresolved ARCH22 typed symbol
    imports)
  - purpose: prove contract/test authority existed before production migration.

## Final Gates
1. `cargo fmt --check`
- result: pass.
- log: `artifacts/gate-logs/01-cargo-fmt-check.log`

2. `cargo clippy --workspace --all-targets -- -D warnings`
- result: pass.
- log: `artifacts/gate-logs/02-cargo-clippy-workspace.log`

3. `cargo test --workspace`
- result: pass.
- log: `artifacts/gate-logs/03-cargo-test-workspace.log`

4. `cargo deny check`
- result: pass-with-warnings.
- log: `artifacts/gate-logs/04-cargo-deny-check.log`
- warnings: `license-not-encountered` allowlist entries; summary reports
  `advisories ok, bans ok, licenses ok, sources ok`.

## Targeted ARCH22 Validation
- `cargo test --test arch22_typed_state_surface_contract`
  - result: pass (`6 passed`).
  - log: `artifacts/test-logs/01-arch22-typed-state-surface-contract.log`
- `cargo test --test wb11_hydrology_kernel_contract`
  - result: pass (`3 passed`).
  - log: `artifacts/test-logs/02-wb11-hydrology-kernel-contract.log`
- `cargo test --test ws10_watershed_kernel_contract`
  - result: pass (`4 passed`).
  - log: `artifacts/test-logs/03-ws10-watershed-kernel-contract.log`
- `cargo test --test parser_runtime_seam_integration`
  - result: pass (`45 passed`).
  - log: `artifacts/test-logs/04-parser-runtime-seam-integration.log`
