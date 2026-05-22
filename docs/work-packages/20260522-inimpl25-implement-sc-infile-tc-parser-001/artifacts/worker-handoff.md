# Worker Handoff — INIMPL25 TC Parser

Evidence mode: `Ran` + `Static`

## Scope Delivered
- Added parser module: `/home/workdir/openWEPP/.worktrees/inimpl25-tc/crates/openwepp-input-contract/src/parsers/tc.rs`.
- Added integration test surface: `/home/workdir/openWEPP/.worktrees/inimpl25-tc/tests/integration/infile_tc_parser_contract.rs`.
- Added fixtures under: `/home/workdir/openWEPP/.worktrees/inimpl25-tc/tests/fixtures/infile/tc/`.

## Contract-Critical Coverage (`SC-INFILE-TC-001`)
- Watershed-only applicability guard (`TC-E-001`) implemented at parse entry. [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl25-tc/crates/openwepp-input-contract/src/parsers/tc.rs:180`
- Strict non-ENOENT open failure is typed (`TC-E-000`), compatibility collapses with warning (`TC-W-002`). [DIRECT] `.../tc.rs:200`
- Optional missing sentinel branch produces `luntc=0`; compatibility emits `TC-W-001`. [DIRECT] `.../tc.rs:191`
- Sentinel payload is content-insensitive; compatibility emits explicit ignore warning (`TC-W-003`) and marker. [DIRECT] `.../tc.rs:221`
- Derived mode/provenance surfaces are exported by parser result: `luntc_requested`, `luntc`, `tc_file_present`, `payload_*`, `open_result`, `mode_divergence`, `tc_out_expected`. [DIRECT] `.../tc.rs:83`

## W4DR Evidence Capture
- `W4DR-001` (source-authority policy):
  - [STATIC][DIRECT] Parser behavior is constrained to contract/spec-defined sentinel semantics and does not infer extra physics/grammar. `/home/workdir/openWEPP/.worktrees/inimpl25-tc/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:20`, `:61`, `:184`
- `W4DR-002` (strict fail vs compat collapse):
  - [RAN][DIRECT] `strict_non_enoent_open_error_is_typed_tc_e_000` and `compatibility_non_enoent_open_error_collapses_with_tc_w_002` passed. `/home/workdir/openWEPP/.worktrees/inimpl25-tc/tests/integration/infile_tc_parser_contract.rs:107`, `:123`
- `W4DR-003` (ownership boundary parser vs output semantics):
  - [STATIC][DIRECT] Parser owns sentinel/provenance and expected-output flag only; no `tc_out.txt` row parsing implemented. `/home/workdir/openWEPP/.worktrees/inimpl25-tc/crates/openwepp-input-contract/src/parsers/tc.rs:83`
- `W4DR-012` (`tc_out` grammar ownership boundary):
  - [STATIC][DIRECT] Contract keeps `tc_out.txt` row grammar out of parser scope (`runtime::watershed::outputs` ownership); implementation preserves this boundary. `/home/workdir/openWEPP/.worktrees/inimpl25-tc/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:61`, `:93`, `:145`

## Execution Evidence
- `Ran`: `rustfmt --check crates/openwepp-input-contract/src/parsers/tc.rs tests/integration/infile_tc_parser_contract.rs` (pass)
- `Ran`: `cargo fmt --check` (pass)
- `Ran`: `cargo clippy --workspace --all-targets -- -D warnings` (pass)
- `Ran`: `cargo test --workspace` (pass)
- `Ran`: `cargo deny check` (pass; non-fatal `license-not-encountered` warnings)
- `Ran`: direct execution of new TC test target
  - `rustc --edition=2021 --test tests/integration/infile_tc_parser_contract.rs -o /tmp/inimpl25_tc_test`
  - `/tmp/inimpl25_tc_test`
  - result: `8 passed`

## Shared-File Quarantine Requests (for INIMPL30)
1. Add parser export in shared file:
   - `/home/workdir/openWEPP/.worktrees/inimpl25-tc/crates/openwepp-input-contract/src/parsers/mod.rs`
   - requested line: `pub mod tc;`
2. Register integration test target in shared file:
   - `/home/workdir/openWEPP/.worktrees/inimpl25-tc/Cargo.toml`
   - requested block:
     - `[[test]]`
     - `name = "infile_tc_parser_contract"`
     - `path = "tests/integration/infile_tc_parser_contract.rs"`

## Known Follow-Up / HOLD Context
- `SC-INFILE-TC-001` still carries governance HOLD entries (`TC-GAP-001..003`) in this branch state; implementation is aligned to current contract text and does not resolve governance ratification in this worker package. [STATIC][DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl25-tc/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md:184`
