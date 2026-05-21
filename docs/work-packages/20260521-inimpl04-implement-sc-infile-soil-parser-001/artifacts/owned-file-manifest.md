# INIMPL04 Owned File Manifest

Evidence: `Static`

## Code and Test Write Set

| Path | Change type | Purpose |
| --- | --- | --- |
| `/home/workdir/openWEPP/.worktrees/inimpl04-soil/crates/openwepp-input-contract/src/parsers/soil.rs` | created | `SC-INFILE-SOIL-001` parser implementation with typed data model and `SOL-E-*` taxonomy. |
| `/home/workdir/openWEPP/.worktrees/inimpl04-soil/tests/integration/infile_soil_parser_contract.rs` | created | Integration-style parser contract tests for strict/compat and failure branches. |
| `/home/workdir/openWEPP/.worktrees/inimpl04-soil/tests/fixtures/infile/soil/valid_97_5.sol` | created | Strict base-profile success fixture. |
| `/home/workdir/openWEPP/.worktrees/inimpl04-soil/tests/fixtures/infile/soil/valid_9002.sol` | created | Disturbed-land + restrictive-layer success fixture. |
| `/home/workdir/openWEPP/.worktrees/inimpl04-soil/tests/fixtures/infile/soil/unknown_datver.sol` | created | Unsupported datver negative fixture (`SOL-E-003`). |
| `/home/workdir/openWEPP/.worktrees/inimpl04-soil/tests/fixtures/infile/soil/invalid_layer_arity_9002.sol` | created | Variant arity mismatch negative fixture (`SOL-E-006`). |
| `/home/workdir/openWEPP/.worktrees/inimpl04-soil/tests/fixtures/infile/soil/invalid_non_monotone_depth.sol` | created | Monotonic depth closure negative fixture (`SOL-E-009`). |
| `/home/workdir/openWEPP/.worktrees/inimpl04-soil/tests/fixtures/infile/soil/alias_97_0.sol` | created | Compatibility alias-gating fixture (`97.0 -> 97.5`). |

## Package Artifacts Produced

- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/docs/work-packages/20260521-inimpl04-implement-sc-infile-soil-parser-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/docs/work-packages/20260521-inimpl04-implement-sc-infile-soil-parser-001/artifacts/owned-file-manifest.md`
- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/docs/work-packages/20260521-inimpl04-implement-sc-infile-soil-parser-001/artifacts/review_agent_a.md`
- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/docs/work-packages/20260521-inimpl04-implement-sc-infile-soil-parser-001/artifacts/review_agent_b.md`
- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/docs/work-packages/20260521-inimpl04-implement-sc-infile-soil-parser-001/artifacts/inimpl04_disposition.md`
- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/docs/work-packages/20260521-inimpl04-implement-sc-infile-soil-parser-001/artifacts/verification_agent_a.md`
- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/docs/work-packages/20260521-inimpl04-implement-sc-infile-soil-parser-001/artifacts/verification_agent_b.md`
