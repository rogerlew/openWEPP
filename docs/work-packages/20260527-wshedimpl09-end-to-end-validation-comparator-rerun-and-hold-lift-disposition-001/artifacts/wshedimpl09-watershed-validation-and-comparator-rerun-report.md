# WSHEDIMPL09 Watershed Validation and Comparator Rerun Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- WSHED09 validation rerun scope executed across:
  - watershed WS10/WS11/WS12 contract-derived vector suites,
  - watershed CLI end-to-end publication/guard behavior suite,
  - workspace-wide repository gates.
- Comparator evidence rerun scope executed for confidence-tier routing
  classification of watershed surfaces.
- Legacy comparison tooling limitation remains explicit:
  `tools/legacy_comparison_suite` currently does not provide watershed closure
  parity gates and therefore cannot close `GAP-SYSTEM-005` on its own.

## Ran
- `cargo test -p openwepp --test ws10_watershed_kernel_contract --test ws11_channel_routing_physics_equivalence_contract --test ws12_impoundment_physics_equivalence_contract`
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract`
- `cargo test -p openwepp --test comparator_tier_routing_metadata`
- `cargo test -p openwepp --test clim07_climate_comparator_and_closure_contract`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `rg -n "does not claim watershed or hourly parity closure gates" tools/legacy_comparison_suite/README.md`
