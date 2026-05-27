# WSHEDIMPL10 Implementation and Test Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Implemented parser model and parsing-flow updates in
  `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`:
  - Added typed payload structures and export surfaces for active outlet
    branches:
    - drop spillway,
    - culvert 1 and 2,
    - rockfill,
    - emergency spillway (open channel/rating curve),
    - filter barrier,
    - perforated riser.
  - Preserved existing fail-closed parsing/validation posture.
- Updated watershed runtime seam test and error rule text in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`:
  - Error rule now truthfully states runtime active-coefficient projection is
    unimplemented (rather than parser payloads missing).
  - Active-structure gap test now uses real active fixture input.
- Added fixture and contract-derived test coverage for active payload export in
  `tests/fixtures/.../strict_valid_active_payloads.imp` and
  `tests/integration/infile_watershed_impoundment_parser_contract.rs`.
- Updated canonical contract and registry documentation:
  - `SC-IMPOUND-001` (v9),
  - `SC-SYSTEM-001` (v34),
  - `science-contracts/index.md`.

## Ran
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
