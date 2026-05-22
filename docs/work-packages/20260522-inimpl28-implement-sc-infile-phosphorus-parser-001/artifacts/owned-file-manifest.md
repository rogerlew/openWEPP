# Owned File Manifest — INIMPL28

Evidence mode: Direct listing

## Parser Implementation
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/crates/openwepp-input-contract/src/parsers/phosphorus.rs`
  - Status: created
  - Purpose: `SC-INFILE-PHOSPHORUS-001` parser implementation with strict/compat policy branches, typed error/warning taxonomy, and closure guards.

## Integration Test
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/tests/integration/infile_phosphorus_parser_contract.rs`
  - Status: created
  - Purpose: strict/compat parser behavior validation, malformed-input errors, missing-sidecar policy, fanout closure, and W4DR-009 evidence test.

## Surface Fixtures
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/tests/fixtures/infile/phosphorus/strict_valid_canonical.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/tests/fixtures/infile/phosphorus/strict_valid_trailing_tokens.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/tests/fixtures/infile/phosphorus/compat_header_variant.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/tests/fixtures/infile/phosphorus/malformed_short_record_count.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/tests/fixtures/infile/phosphorus/malformed_non_numeric.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/tests/fixtures/infile/phosphorus/malformed_negative.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/tests/fixtures/infile/phosphorus/malformed_non_finite.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/tests/fixtures/infile/phosphorus/large_non_negative_values.txt`
  - Status: created
  - Purpose: canonical success, compatibility warning branches, and malformed-input error coverage.

## Package Artifacts
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/docs/work-packages/20260522-inimpl28-implement-sc-infile-phosphorus-parser-001/artifacts/worker-handoff.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/docs/work-packages/20260522-inimpl28-implement-sc-infile-phosphorus-parser-001/artifacts/owned-file-manifest.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/docs/work-packages/20260522-inimpl28-implement-sc-infile-phosphorus-parser-001/artifacts/inimpl28_disposition.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/docs/work-packages/20260522-inimpl28-implement-sc-infile-phosphorus-parser-001/artifacts/review_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/docs/work-packages/20260522-inimpl28-implement-sc-infile-phosphorus-parser-001/artifacts/review_agent_b.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/docs/work-packages/20260522-inimpl28-implement-sc-infile-phosphorus-parser-001/artifacts/verification_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus/docs/work-packages/20260522-inimpl28-implement-sc-infile-phosphorus-parser-001/artifacts/verification_agent_b.md`
  - Status: created
  - Purpose: package closeout, review, and verification records.
