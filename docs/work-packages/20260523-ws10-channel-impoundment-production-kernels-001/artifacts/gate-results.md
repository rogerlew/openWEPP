# Gate Results

Status: `completed`
Evidence mode: `Ran`

## Pre-Implementation Contract Gate
- `cargo test --test ws10_watershed_kernel_contract`
  - result: **fail** (`0 passed; 4 failed`)
  - purpose: prove contract/test authority landed before production WS10 kernel
    behavior.

## Final Gates
- `cargo test --test ws10_watershed_kernel_contract`
  - result: pass (`4 passed`).
- `cargo test -p openwepp-watershed-orchestrator`
  - result: pass (`26 passed`).
- `cargo test --test parser_runtime_seam_integration`
  - result: pass (`45 passed`).
- `cargo fmt --check`
  - result: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - result: pass.
- `cargo test --workspace`
  - result: pass.
- `cargo deny check`
  - result: pass (`advisories ok, bans ok, licenses ok, sources ok`).
  - note: allowlist `license-not-encountered` warnings are non-fatal.
