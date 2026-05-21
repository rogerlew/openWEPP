# Owned File Manifest — INIMPL20

Evidence mode: Direct listing

## Parser Implementation
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/crates/openwepp-input-contract/src/parsers/watershed_channel.rs`
  - Status: created
  - Purpose: `SC-INFILE-WATERSHED-CHANNEL-001` strict/compat parser implementation with typed errors/warnings and invariant guards.

## Integration Test
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/tests/integration/infile_watershed_channel_parser_contract.rs`
  - Status: created
  - Purpose: fixture-driven strict/compat contract test suite for watershed channel parse surface.

## Surface Fixtures
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/tests/fixtures/infile/watershed_channel/strict_valid_single_channel.chn`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/tests/fixtures/infile/watershed_channel/strict_invalid_datver.chn`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/tests/fixtures/infile/watershed_channel/compat_legacy_datver.chn`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/tests/fixtures/infile/watershed_channel/strict_missing_rating_curve.chn`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/tests/fixtures/infile/watershed_channel/strict_ishape_out_of_domain.chn`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/tests/fixtures/infile/watershed_channel/compat_ishape_normalized.chn`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/tests/fixtures/infile/watershed_channel/strict_sidecar_required.chn`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/tests/fixtures/infile/watershed_channel/compat_sidecar_missing_warn.chn`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/tests/fixtures/infile/watershed_channel/strict_icntrl0_requires_slplst.chn`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/tests/fixtures/infile/watershed_channel/strict_extra_record.chn`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/tests/fixtures/infile/watershed_channel/strict_non_numeric.chn`
  - Status: created
  - Purpose: canonical success + strict failures + compatibility warnings across contract guard matrix.

## Package Artifact Outputs
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/docs/work-packages/20260521-inimpl20-implement-sc-infile-watershed-channel-parser-001/artifacts/worker-handoff.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/docs/work-packages/20260521-inimpl20-implement-sc-infile-watershed-channel-parser-001/artifacts/owned-file-manifest.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/docs/work-packages/20260521-inimpl20-implement-sc-infile-watershed-channel-parser-001/artifacts/inimpl20_disposition.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/docs/work-packages/20260521-inimpl20-implement-sc-infile-watershed-channel-parser-001/artifacts/review_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/docs/work-packages/20260521-inimpl20-implement-sc-infile-watershed-channel-parser-001/artifacts/review_agent_b.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/docs/work-packages/20260521-inimpl20-implement-sc-infile-watershed-channel-parser-001/artifacts/verification_agent_a.md`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/docs/work-packages/20260521-inimpl20-implement-sc-infile-watershed-channel-parser-001/artifacts/verification_agent_b.md`
