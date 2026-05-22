# Worker Handoff — INIMPL29 (SC-INFILE-LCWB-001)

Evidence: Ran + Static

## Scope Completed
- [DIRECT] Implemented LCWB parser surface in `/home/workdir/openWEPP/.worktrees/inimpl29-lcwb/crates/openwepp-input-contract/src/parsers/lcwb.rs`.
- [DIRECT] Added LCWB contract integration tests in `/home/workdir/openWEPP/.worktrees/inimpl29-lcwb/tests/integration/infile_lcwb_parser_contract.rs`.
- [DIRECT] Added LCWB fixtures under `/home/workdir/openWEPP/.worktrees/inimpl29-lcwb/tests/fixtures/infile/lcwb/`.
- [DIRECT] Produced INIMPL29 package artifacts in this package directory.

## Implemented Strict/Compat Behavior
- [DIRECT] Watershed-only applicability:
  - strict mode rejects non-watershed context (`LCWB-E-002`)
  - compatibility mode emits typed not-applicable branch (`LCWB-W-004`).
- [DIRECT] Open-branch policy:
  - ENOENT -> missing branch (`lcwbflg=0`)
  - strict non-ENOENT open failure -> `LCWB-E-000`
  - compatibility collapse-with-warning -> `LCWB-W-003` (+ missing-branch warning `LCWB-W-001`).
- [DIRECT] Sentinel payload policy:
  - strict rejects non-whitespace payload (`LCWB-E-001`)
  - compatibility accepts ignored payload with warning (`LCWB-W-002`).
- [DIRECT] Requested/effective closure:
  - strict mismatch -> `LCWB-E-003`
  - compatibility preserves explicit divergence state (`mode_divergence`).
- [DIRECT] Exports required provenance fields: `lcwb_requested`, `lcwbflg`, `lcwb_file_present`, `payload_*`, `open_result`, `run_context`, `mode_divergence`, and `ofe_row_selection_policy_mode`.

## W4DR Evidence (Required)
- [DIRECT][W4DR-001] Legacy/current-source sentinel authority is fixture-backed by `w4dr_001_legacy_source_authority_presence_controls_lcwbflg` (presence -> `lcwbflg=1`, missing -> `lcwbflg=0`).
- [DIRECT][W4DR-003] Parser/output boundary is preserved: parser handles sentinel/provenance only; no output-row grammar ownership pulled into parser scope. Evidence in `w4dr_003_and_w4dr_011_policy_projection_is_derived_not_payload_parsed` and absence of output-row parsing fields in parser model.
- [DIRECT][W4DR-011] `lcwbflg` authority is aligned to current-source open-result semantics (`open success => 1`, otherwise `0`) and tested in strict/compat branches.

## Gates and Verification Executed
- [RAN] `cargo fmt --check` (pass)
- [RAN] `cargo clippy --workspace --all-targets -- -D warnings` (pass)
- [RAN] `cargo test --workspace` (pass)
- [RAN] `cargo deny check` (pass; non-failing `license-not-encountered` warnings)
- [RAN] LCWB-focused harness run:
  - `rustc --edition=2024 --test tests/integration/infile_lcwb_parser_contract.rs -o /tmp/infile_lcwb_parser_contract_test && /tmp/infile_lcwb_parser_contract_test`
  - result: 13 passed, 0 failed.

## Integration Notes for INIMPL30
- [DIRECT] `lcwb.rs` is not wired into `parsers/mod.rs` and `infile_lcwb_parser_contract.rs` is not registered as a cargo `[[test]]` target in `Cargo.toml` in this worker scope.
- [INFERENCE] INIMPL30 (shared/quarantine owner) should perform export/registration wiring so LCWB parser/tests participate in standard workspace test orchestration.
