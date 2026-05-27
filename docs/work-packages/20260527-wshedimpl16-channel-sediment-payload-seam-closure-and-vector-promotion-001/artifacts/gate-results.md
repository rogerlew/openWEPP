# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-27

## Static
- none

## Ran
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings`
  - first run: fail (`clippy::uninlined_format_args` in
    `openwepp-cli-watershed.rs`)
  - fix applied
  - second run: pass
- `cargo test --workspace --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract --test erod15_wave3_contract_authority_closure_contract --test watershed_cli_behavior_contract` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` (final) -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass (warnings only: duplicate crates and
  license-not-encountered policy entries)
