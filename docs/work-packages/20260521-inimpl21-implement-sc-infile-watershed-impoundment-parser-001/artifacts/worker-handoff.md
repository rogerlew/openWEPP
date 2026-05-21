# Worker Handoff — INIMPL21 Watershed Impoundment Parser

Evidence mode: `Ran` + `Static`

## Scope Delivered
- Added parser module: `/home/workdir/openWEPP/.worktrees/inimpl21-watershed-impoundment/crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`.
- Added integration tests: `/home/workdir/openWEPP/.worktrees/inimpl21-watershed-impoundment/tests/integration/infile_watershed_impoundment_parser_contract.rs`.
- Added fixtures under: `/home/workdir/openWEPP/.worktrees/inimpl21-watershed-impoundment/tests/fixtures/infile/watershed_impoundment/`.

## Contract-Critical Coverage (`SC-INFILE-WATERSHED-IMPOUNDMENT-001`)
- Strict vs compatibility preamble/datver handling, including legacy no-datver compatibility warning path (`IMP-W-001`) and strict rejection (`IMP-E-003`). [DIRECT]
- Typed error taxonomy mapping via `contract_error_id()`:
  - `IMP-E-000` input open error
  - `IMP-E-001` token parse error
  - `IMP-E-002` unexpected EOF / closure underflow
  - `IMP-E-003` unsupported datver/preamble policy
  - `IMP-E-004` domain violation (branch selector/value domains)
  - `IMP-E-005` branch arity mismatch
  - `IMP-E-006` physical-domain violation (`deltat`, `qinf`)
  - `IMP-E-007` declared-vs-expected impoundment count closure
  - `IMP-E-008` invariant violation (e.g., stage monotonicity, level constraints)
  - `IMP-E-009` ordering mismatch surface
- Cross-file count policy implemented through parse options (`expected_structural_count`) with strict equality and compatibility deterministic truncation warning (`IMP-W-002`). [DIRECT]
- Branch-structured payload closure for drop spillway, culverts, rockfill, emergency spillway, filter, riser, size, and area/stage/length curves. [DIRECT]

## Execution Evidence
- `Ran`: `cargo fmt --check` (pass).
- `Ran`: `cargo check --workspace` (pass).
- `Ran`: `cargo clippy --workspace --all-targets -- -D warnings` (pass).
- `Ran`: `cargo test --workspace` (pass for currently registered test targets).
- `Ran`: `cargo deny check` (pass; non-fatal `license-not-encountered` warnings only).
- `Ran`: direct execution of new impoundment tests:
  - `rustc --edition=2021 --test tests/integration/infile_watershed_impoundment_parser_contract.rs -o /tmp/inimpl21_impoundment_test`
  - `/tmp/inimpl21_impoundment_test`
  - result: `13 passed`.

## Shared-File Quarantine Requests (for INIMPL22 integration)
1. Add parser export in shared file:
   - `/home/workdir/openWEPP/.worktrees/inimpl21-watershed-impoundment/crates/openwepp-input-contract/src/parsers/mod.rs`
   - requested line: `pub mod watershed_impoundment;`
2. Register integration test target in shared file:
   - `/home/workdir/openWEPP/.worktrees/inimpl21-watershed-impoundment/Cargo.toml`
   - requested block:
     - `[[test]]`
     - `name = "infile_watershed_impoundment_parser_contract"`
     - `path = "tests/integration/infile_watershed_impoundment_parser_contract.rs"`

## Known Follow-Up / HOLD Context
- Per Wave 3 ownership manifest, this worker intentionally did not edit quarantine-owned shared files. [DIRECT]
- Until integration applies the two shared-file requests above, workspace gate execution will not exercise this new parser through the canonical module/test wiring path. [INFERENCE]
