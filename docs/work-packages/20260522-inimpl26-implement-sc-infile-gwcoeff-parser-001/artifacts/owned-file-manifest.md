# Owned File Manifest — INIMPL26

Evidence mode: Direct listing

## Parser Implementation
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/crates/openwepp-input-contract/src/parsers/gwcoeff.rs`
  - Status: created
  - Purpose: `SC-INFILE-GWCOEFF-001` strict/compat parser with typed error taxonomy (`GW-E-000..007`), warnings (`GW-W-001`), branch invariants, and namespace-separation guard.

## Integration Test
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/tests/integration/infile_gwcoeff_parser_contract.rs`
  - Status: created
  - Purpose: contract behavior tests for strict/compat parsing, open-error policy, absence semantics, and W4DR branches.

## Surface Fixtures
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/tests/fixtures/infile/gwcoeff/strict_valid_with_trailing_text.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/tests/fixtures/infile/gwcoeff/strict_valid_numeric_only.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/tests/fixtures/infile/gwcoeff/invalid_missing_line4.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/tests/fixtures/infile/gwcoeff/invalid_non_numeric_line2.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/tests/fixtures/infile/gwcoeff/invalid_negative_bftharea.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/tests/fixtures/infile/gwcoeff/invalid_nonfinite_line3.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/tests/fixtures/infile/gwcoeff/invalid_prefixed_variant.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/tests/fixtures/infile/gwcoeff/compat_extra_line_record_count_error.txt`
  - Status: created
  - Purpose: canonical success, malformed-present failures, and compatibility/open-error decision branches.

## Package Artifact Outputs
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/docs/work-packages/20260522-inimpl26-implement-sc-infile-gwcoeff-parser-001/artifacts/worker-handoff.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/docs/work-packages/20260522-inimpl26-implement-sc-infile-gwcoeff-parser-001/artifacts/owned-file-manifest.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/docs/work-packages/20260522-inimpl26-implement-sc-infile-gwcoeff-parser-001/artifacts/inimpl26_disposition.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/docs/work-packages/20260522-inimpl26-implement-sc-infile-gwcoeff-parser-001/artifacts/review_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/docs/work-packages/20260522-inimpl26-implement-sc-infile-gwcoeff-parser-001/artifacts/review_agent_b.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/docs/work-packages/20260522-inimpl26-implement-sc-infile-gwcoeff-parser-001/artifacts/verification_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff/docs/work-packages/20260522-inimpl26-implement-sc-infile-gwcoeff-parser-001/artifacts/verification_agent_b.md`
