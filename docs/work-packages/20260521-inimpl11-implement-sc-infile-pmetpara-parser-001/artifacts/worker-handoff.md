# INIMPL11 Worker Handoff

Evidence: Mixed (`Ran` + `Static`)

## Scope Completed

Implemented `SC-INFILE-PMETPARA-001` parser surface in assigned write-set:

1. Strict/compat parse modes for `pmetpara.txt`.
2. Typed `PMET-E-*` error mapping and `PMET-W-*` warning surfaces.
3. Optional-sidecar branch modeling (`sidecar_present`, `iflget=1/2`).
4. Deterministic crop-key normalization policy with strict width guard and compatibility truncation.
5. Explicit strict no-hit lookup error vs compatibility first-row fallback behavior.
6. Fixtures and integration tests for valid, malformed, and policy-branch cases.

## Files Implemented

- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/crates/openwepp-input-contract/src/parsers/pmetpara.rs`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/crates/openwepp-input-contract/src/parsers/mod.rs`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/integration/infile_pmetpara_parser_contract.rs`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/strict_valid.txt`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/invalid_header_datver_variant.txt`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/invalid_record_count_mismatch.txt`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/invalid_duplicate_keys.txt`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/invalid_row_arity.txt`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/strict_multitoken_actlnam.txt`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/compat_multitoken_actlnam.txt`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/compat_long_crop_name.txt`

## Check Evidence

### Ran

1. `rustfmt crates/openwepp-input-contract/src/parsers/pmetpara.rs tests/integration/infile_pmetpara_parser_contract.rs`
- Result: pass.

2. `rustc --edition=2024 --test tests/integration/infile_pmetpara_parser_contract.rs -o /tmp/infile_pmetpara_parser_contract && /tmp/infile_pmetpara_parser_contract`
- Result: pass (`13` tests).

3. `cargo fmt --check`
- Result: pass.

4. `cargo clippy --workspace --all-targets -- -D warnings`
- Result: pass.

5. `cargo test --workspace`
- Result: pass (existing registered integration suites + unit/doc tests).

### Static

- Contract/spec alignment checked against:
  - `docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md`
  - `docs/specifications/wepp-input-files/specs/pmetpara.spec.md`

## Follow-On Note for INIMPL17

- New PMETPARA integration test file is executed via direct `rustc` in this package because root test-target registration currently enumerates named targets in `Cargo.toml` and does not include this new file yet.
- If desired, integration package can register this target in root test metadata when cross-branch integrations are merged.
