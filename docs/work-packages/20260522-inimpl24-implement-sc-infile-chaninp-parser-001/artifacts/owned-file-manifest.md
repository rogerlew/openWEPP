# Owned File Manifest — INIMPL24

Evidence mode: Direct listing

## Parser Implementation
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/crates/openwepp-input-contract/src/parsers/chaninp.rs`
  - Status: created
  - Purpose: `SC-INFILE-CHANINP-001` strict/compat parser implementation with typed CHN errors/warnings, applicability branching, and normalization/topology guards.

## Integration Test
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/integration/infile_chaninp_parser_contract.rs`
  - Status: created
  - Purpose: fixture-driven strict/compat contract and W4DR evidence suite for `chan.inp`.

## Surface Fixtures
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/strict_valid.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/strict_prefixed_variant.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/strict_invalid_ichout.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/strict_dtchr_out_of_range.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/strict_negative_cbase.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/strict_nchnum_exceeds_nchan.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/strict_unknown_ichnum.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/strict_line4_arity_mismatch.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/compat_parse_failure_defaults.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/compat_ichout_two_normalized.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/compat_ichout_zero_normalized.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/compat_dtchr_clamped.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/compat_cbase_negative_clamped.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/compat_nchnum_clamped.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/compat_unknown_ichnum_retained.chaninp`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/tests/fixtures/infile/chaninp/strict_trailing_tokens.chaninp`
  - Status: created
  - Purpose: canonical success + strict failures + compatibility fallback/normalization warnings across SC guard matrix.

## Package Artifact Outputs
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/docs/work-packages/20260522-inimpl24-implement-sc-infile-chaninp-parser-001/artifacts/worker-handoff.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/docs/work-packages/20260522-inimpl24-implement-sc-infile-chaninp-parser-001/artifacts/owned-file-manifest.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/docs/work-packages/20260522-inimpl24-implement-sc-infile-chaninp-parser-001/artifacts/inimpl24_disposition.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/docs/work-packages/20260522-inimpl24-implement-sc-infile-chaninp-parser-001/artifacts/review_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/docs/work-packages/20260522-inimpl24-implement-sc-infile-chaninp-parser-001/artifacts/review_agent_b.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/docs/work-packages/20260522-inimpl24-implement-sc-infile-chaninp-parser-001/artifacts/verification_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp/docs/work-packages/20260522-inimpl24-implement-sc-infile-chaninp-parser-001/artifacts/verification_agent_b.md`
