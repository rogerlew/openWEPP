# Worker Handoff — INIMPL31 (SC-INFILE-HBP-001)

Evidence mode: Ran + Static

## Scope Delivered
- [DIRECT] Added canonical HBP parser specification:
  - `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/hbp-file.spec.md`
- [DIRECT] Added canonical parser contract:
  - `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- [DIRECT] Registered HBP in input-surface registry:
  - `/home/workdir/openWEPP/docs/specifications/wepp-input-files/input-surface-registry.md`
- [DIRECT] Integrated parser module export and dependency wiring:
  - `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/mod.rs`
  - `/home/workdir/openWEPP/crates/openwepp-input-contract/Cargo.toml`
- [DIRECT] Added integration test target wiring and HBP test suite:
  - `/home/workdir/openWEPP/Cargo.toml`
  - `/home/workdir/openWEPP/tests/integration/infile_hbp_parser_contract.rs`
- [DIRECT] Added INIMPL31 package scaffold and closeout artifacts under:
  - `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl31-implement-sc-infile-hbp-parser-001/`

## Implemented Contract Behaviors
- [DIRECT] Strict/compat path policy is explicit:
  - strict accepts canonical `H*.hbp` only;
  - compat can derive `.hbp` from `.pass.dat` with `HBP-W-001`.
- [DIRECT] Forbidden suffixes are rejected (`.pass.hbp`, `.pass.dat.hbp`) as typed naming failures.
- [DIRECT] No silent fallback to text pass files is implemented.
- [DIRECT] Schema `1.x` and `2.x` parse paths are both exercised by deterministic integration fixtures.
- [DIRECT] Typed failure IDs are exercised for open errors, malformed/truncated content, footer corruption, and hillslope-id mismatch.

## Gate Evidence
- [RAN] `cargo fmt --check` → pass
- [RAN] `cargo clippy --workspace --all-targets -- -D warnings` → pass
- [RAN] `cargo test --workspace` → pass (includes `infile_hbp_parser_contract` with 11 passing tests)
- [RAN] `cargo deny check` → pass (non-failing `license-not-encountered` warnings only)

## Notes
- [DIRECT] `Cargo.lock` updated due dependency wiring (`flate2`) and resolved graph updates.
- [DIRECT] Parser-local responsibilities remain separated from run-level shard-set orchestration invariants.
