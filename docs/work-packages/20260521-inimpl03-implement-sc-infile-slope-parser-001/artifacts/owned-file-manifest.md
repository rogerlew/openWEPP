# INIMPL03 Owned File Manifest

Evidence: Ran + Static

## Owned Write Set Files Changed

| file | status | purpose |
| --- | --- | --- |
| `crates/openwepp-input-contract/src/parsers/slope.rs` | created | `SC-INFILE-SLOPE-001` parser implementation with strict/compat behavior and typed guard errors |
| `tests/integration/infile_slope_parser_contract.rs` | created | parser contract integration tests for success/failure branches |
| `tests/fixtures/infile/slope/strict_valid_canonical.slp` | created | canonical strict-mode success fixture |
| `tests/fixtures/infile/slope/compat_legacy_no_datver.slp` | created | compatibility no-datver success fixture |
| `tests/fixtures/infile/slope/invalid_mixed_distance_mode.slp` | created | mixed-distance-mode rejection fixture |
| `tests/fixtures/infile/slope/invalid_missing_endpoint.slp` | created | endpoint constraint rejection fixture |
| `tests/fixtures/infile/slope/invalid_cross_ofe_boundary.slp` | created | cross-OFE continuity rejection fixture |
| `tests/fixtures/infile/slope/invalid_nslpts_lt2.slp` | created | `nslpts>=2` guard rejection fixture |
| `tests/fixtures/infile/slope/invalid_token.slp` | created | token parse error fixture |
| `docs/work-packages/20260521-inimpl03-implement-sc-infile-slope-parser-001/artifacts/worker-handoff.md` | created | worker handoff summary for INIMPL07 integration |
| `docs/work-packages/20260521-inimpl03-implement-sc-infile-slope-parser-001/artifacts/owned-file-manifest.md` | created | owned-path change ledger |
| `docs/work-packages/20260521-inimpl03-implement-sc-infile-slope-parser-001/artifacts/review_agent_a.md` | created | review gate A artifact |
| `docs/work-packages/20260521-inimpl03-implement-sc-infile-slope-parser-001/artifacts/review_agent_b.md` | created | review gate B artifact |
| `docs/work-packages/20260521-inimpl03-implement-sc-infile-slope-parser-001/artifacts/inimpl03_disposition.md` | created | review disposition ledger |
| `docs/work-packages/20260521-inimpl03-implement-sc-infile-slope-parser-001/artifacts/verification_agent_a.md` | created | verification gate A artifact |
| `docs/work-packages/20260521-inimpl03-implement-sc-infile-slope-parser-001/artifacts/verification_agent_b.md` | created | verification gate B artifact |
