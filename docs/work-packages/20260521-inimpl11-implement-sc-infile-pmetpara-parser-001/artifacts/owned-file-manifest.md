# INIMPL11 Owned File Manifest

Evidence: `Static`

## Code and Test Write Set

| Path | Change type | Purpose |
| --- | --- | --- |
| `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/crates/openwepp-input-contract/src/parsers/pmetpara.rs` | created | PMETPARA parser implementation with strict/compat policy and typed errors/warnings. |
| `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/crates/openwepp-input-contract/src/parsers/mod.rs` | updated | Exposed `pmetpara` parser module. |
| `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/integration/infile_pmetpara_parser_contract.rs` | created | Parser-contract integration tests for `infile-pmetpara`. |
| `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/strict_valid.txt` | created | Canonical strict success fixture. |
| `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/invalid_header_datver_variant.txt` | created | Unsupported header variant fixture (`PMET-E-004`). |
| `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/invalid_record_count_mismatch.txt` | created | Row-count closure failure fixture (`PMET-E-002`). |
| `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/invalid_duplicate_keys.txt` | created | Strict duplicate key failure fixture (`PMET-E-003`). |
| `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/invalid_row_arity.txt` | created | Row arity failure fixture (`PMET-E-001`). |
| `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/strict_multitoken_actlnam.txt` | created | Strict tokenization failure fixture (`PMET-E-008`). |
| `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/compat_multitoken_actlnam.txt` | created | Compatibility `actlnam` normalization fixture (`PMET-W-004`). |
| `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/tests/fixtures/infile/pmetpara/compat_long_crop_name.txt` | created | Compatibility crop-key truncation fixture (`PMET-W-002`). |

## Package Artifact Outputs

- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/docs/work-packages/20260521-inimpl11-implement-sc-infile-pmetpara-parser-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/docs/work-packages/20260521-inimpl11-implement-sc-infile-pmetpara-parser-001/artifacts/owned-file-manifest.md`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/docs/work-packages/20260521-inimpl11-implement-sc-infile-pmetpara-parser-001/artifacts/inimpl11_disposition.md`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/docs/work-packages/20260521-inimpl11-implement-sc-infile-pmetpara-parser-001/artifacts/review_agent_a.md`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/docs/work-packages/20260521-inimpl11-implement-sc-infile-pmetpara-parser-001/artifacts/review_agent_b.md`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/docs/work-packages/20260521-inimpl11-implement-sc-infile-pmetpara-parser-001/artifacts/verification_agent_a.md`
- `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara/docs/work-packages/20260521-inimpl11-implement-sc-infile-pmetpara-parser-001/artifacts/verification_agent_b.md`
