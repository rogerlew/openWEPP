# Worker Handoff — INIMPL24 (SC-INFILE-CHANINP-001)

Evidence mode: Ran + Static

## Scope Delivered
- [DIRECT] Implemented chaninp parser at `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/crates/openwepp-input-contract/src/parsers/chaninp.rs`.
- [DIRECT] Added integration contract harness at `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/integration/infile_chaninp_parser_contract.rs`.
- [DIRECT] Added surface fixture set under `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/`.
- [DIRECT] Produced required INIMPL24 artifact bundle in this package artifact directory.

## Implemented Contract Behaviors
- [DIRECT] Applicability branch (`ipeak<=2`) returns explicit non-applicable outcome without sidecar dependency.
- [DIRECT] Strict `ipeak>2` missing/open handling is typed (`CHN-E-009` missing, `CHN-E-000` non-ENOENT open error).
- [DIRECT] Compatibility missing/open collapse branches are explicit (`CHN-W-001`, `CHN-W-002`) with deterministic default export state.
- [DIRECT] Required 4-line grammar, token parsing, non-finite checks, and list-arity closure are typed (`CHN-E-001/002/003`).
- [DIRECT] Strict range/topology invariants are typed (`CHN-E-004/005`) and compatibility normalization/clamping emits `CHN-W-004`.
- [DIRECT] Unsupported prefixed/datver-style variant is rejected in both modes (`CHN-E-008`).
- [DIRECT] Unknown `ichnum` IDs are strict-typed failures and compatibility-retained warnings (`CHN-W-005`) with explicit provenance flag.

## W4DR Evidence (Required)
- [DIRECT] `W4DR-003` (ownership split): parser exports parse/provenance + normalized option state only; no output-row grammar/runtime-output contract logic introduced in parser surface.
- [DIRECT] `W4DR-004` (ichout policy): strict enforces canonical domain `0..3`; compatibility normalizes to writer-interoperable `{1,3}` subset with `CHN-W-004` evidence path in `compatibility_normalizes_ichout_to_writer_subset_with_warning`.
- [DIRECT] `W4DR-005` (dtchr semantics): strict rejects out-of-range timestep; compatibility normalizes/clamps and recomputes closure (`ntchr`, `dtchr_norm_s`) with warning evidence in `strict_rejects_dtchr_out_of_range_and_compatibility_normalizes`.
- [DIRECT] `W4DR-006` (cbase semantics/guards): strict finite+non-negative guard; compatibility clamps negative values to `0.0` with explicit warning in `strict_rejects_negative_cbase_and_compatibility_clamps`.
- [DIRECT] `W4DR-008` (namespace separation): parser and tests use explicit `cbase_m3_s_m2` field naming (no `gwcoeff` alias conflation), evidenced in `cbase_namespace_is_explicit_and_not_gwcoeff_alias`.

## Shared-File Quarantine Requests (No Direct Edit Per Ownership Manifest)
1. Request integration owner (`INIMPL30`) to export parser module in shared quarantine file:
   - `crates/openwepp-input-contract/src/parsers/mod.rs`
   - Requested line: `pub mod chaninp;`
2. Request integration owner (`INIMPL30`) to register integration test target in shared quarantine file:
   - `Cargo.toml`
   - Requested `[[test]]` target: `infile_chaninp_parser_contract`.

These shared-file edits were intentionally not made in this worker stream per explicit ownership constraints.

## Gate Evidence
- [RAN] `rustfmt crates/openwepp-input-contract/src/parsers/chaninp.rs tests/integration/infile_chaninp_parser_contract.rs`
- [RAN] `rustc --edition=2024 --test tests/integration/infile_chaninp_parser_contract.rs -o /tmp/infile_chaninp_parser_contract && /tmp/infile_chaninp_parser_contract`
  - [DIRECT] Result: 17 passed, 0 failed.
- [RAN] `cargo fmt --check`
- [RAN] `cargo clippy --workspace --all-targets -- -D warnings`
- [RAN] `cargo test --workspace`
- [RAN] `cargo deny check`
  - [DIRECT] Result: pass with non-failing `license-not-encountered` warnings from `deny.toml` allowlist.

## Open Findings / HOLD Conditions
- [DIRECT] No unresolved high-severity findings in implementation-owned files.
- [DIRECT] Contract-level HOLD items remain upstream in `SC-INFILE-CHANINP-001` gap register (`CHANINP-GAP-001..004`) and are not introduced by this parser package.
