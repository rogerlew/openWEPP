# WSHEDIMPL10 Watershed Validation and Comparator Rerun Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- WSHED10 validation scope was parser/runtime-seam focused:
  - active impoundment branch payload export test vector passes,
  - runtime active-structure projection gap remains explicit fail-closed.
- Workspace-wide test gate was rerun and passed, covering WS10/WS11/WS12,
  watershed CLI, and comparator-tier metadata tests as part of
  `cargo test --workspace`.
- This package does not create a new baseline-authoritative end-to-end
  watershed comparator fixture lane; `GAP-SYSTEM-005` remains open.

## Ran
- `cargo test -p openwepp --test infile_watershed_impoundment_parser_contract strict_mode_parses_active_structure_payload_exports`
- `cargo test -p openwepp-watershed-orchestrator watershed_impoundment_runtime_seed_rejects_active_structure_projection_gap`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
