# Owned File Manifest — INIMPL29

Evidence: Static + Direct file listing

## Parser Surface
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl29-lcwb/crates/openwepp-input-contract/src/parsers/lcwb.rs`
  - Status: created
  - Purpose: `SC-INFILE-LCWB-001` strict/compat parser behavior, typed errors/warnings, provenance fields.

## Integration Test
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl29-lcwb/tests/integration/infile_lcwb_parser_contract.rs`
  - Status: created
  - Purpose: fixture-backed strict/compat contract checks + W4DR evidence tests.

## Fixtures
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl29-lcwb/tests/fixtures/infile/lcwb/empty_lcwb.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl29-lcwb/tests/fixtures/infile/lcwb/whitespace_lcwb.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl29-lcwb/tests/fixtures/infile/lcwb/nonempty_payload_lcwb.txt`
  - Status: created
  - Purpose: strict empty/whitespace branches + compatibility non-empty payload branch.

## Package Directory
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl29-lcwb/docs/work-packages/20260522-inimpl29-implement-sc-infile-lcwb-parser-001/**`
  - Status: created in this worktree from authoritative kickoff package source, then populated with required artifacts.
  - Purpose: package execution envelope and closeout evidence.
