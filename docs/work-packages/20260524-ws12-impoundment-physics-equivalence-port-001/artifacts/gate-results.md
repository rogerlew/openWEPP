# WS12 Gate Results

Status: `completed-with-hold`
Evidence mode: `Ran`

## Pre-Implementation Contract Gate
- `cargo test --test ws12_impoundment_physics_equivalence_contract`
  - result: **fail** (`0 passed; 4 failed`)
  - purpose: prove contract/test authority landed before WS12 production
    impoundment-kernel implementation.

## Post-Implementation Targeted Gates
- `cargo test --test ws10_watershed_kernel_contract`
  - result: pass (`4 passed`).
- `cargo test --test ws12_impoundment_physics_equivalence_contract`
  - result: pass (`4 passed`).

## Final Gates (Closeout Run)
- `cargo fmt --check`
  - result: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - result: pass.
- `cargo test --workspace`
  - result: **fail** at
    `cli01_contract_conformance_hillslope_run_emits_required_outputs_and_manifest`
    (`tests/integration/cli01_runner_hillslope_integration.rs`) with
    `ReleaseMetadata ... JSON parse EOF`.
- `cargo deny check`
  - result: **fail**
  - errors recorded:
    - advisory `RUSTSEC-2025-0038` (`arrow2`)
    - rejected `BSL-1.0` license for `xxhash-rust`
  - warnings recorded:
    - duplicate lockfile entries (`getrandom`, `hashbrown`)
    - license-not-encountered entries in `deny.toml`.

## Hold Decision
- Final gate sweep is recorded but not fully passing.
- Package disposition remains `completed-with-hold` pending parity traces and
  hold-lift conditions in `ws12_disposition.md`.
