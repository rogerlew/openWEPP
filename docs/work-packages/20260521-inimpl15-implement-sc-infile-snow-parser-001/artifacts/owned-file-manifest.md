# Owned File Manifest — INIMPL15

Evidence mode: Direct listing

## Parser Implementation
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/crates/openwepp-input-contract/src/parsers/snow.rs`
  - Status: created
  - Purpose: `SC-INFILE-SNOW-001` parser implementation, strict/compat policy handling, typed errors/warnings, and provenance markers.
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/crates/openwepp-input-contract/src/parsers/mod.rs`
  - Status: modified
  - Purpose: export snow parser module.

## Integration Test
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/tests/integration/infile_snow_parser_contract.rs`
  - Status: created
  - Purpose: contract behavior validation for strict/compat parser branches.

## Surface Fixtures
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/tests/fixtures/infile/snow/strict_valid.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/tests/fixtures/infile/snow/strict_trailing_tokens_invalid.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/tests/fixtures/infile/snow/compat_trailing_tokens.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/tests/fixtures/infile/snow/strict_surplus_records_invalid.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/tests/fixtures/infile/snow/compat_surplus_records.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/tests/fixtures/infile/snow/strict_missing_record_invalid.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/tests/fixtures/infile/snow/strict_nonfinite_invalid.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/tests/fixtures/infile/snow/strict_nonpositive_density_invalid.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/tests/fixtures/infile/snow/strict_non_numeric_invalid.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/tests/fixtures/infile/snow/prefixed_variant_rejected.txt`
  - Status: created
  - Purpose: strict-valid, compat-warning, and malformed-input coverage for snow parser contract.

## Package Artifacts
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/docs/work-packages/20260521-inimpl15-implement-sc-infile-snow-parser-001/artifacts/worker-handoff.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/docs/work-packages/20260521-inimpl15-implement-sc-infile-snow-parser-001/artifacts/owned-file-manifest.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/docs/work-packages/20260521-inimpl15-implement-sc-infile-snow-parser-001/artifacts/review_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/docs/work-packages/20260521-inimpl15-implement-sc-infile-snow-parser-001/artifacts/review_agent_b.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/docs/work-packages/20260521-inimpl15-implement-sc-infile-snow-parser-001/artifacts/inimpl15_disposition.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/docs/work-packages/20260521-inimpl15-implement-sc-infile-snow-parser-001/artifacts/verification_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl15-snow/docs/work-packages/20260521-inimpl15-implement-sc-infile-snow-parser-001/artifacts/verification_agent_b.md`
