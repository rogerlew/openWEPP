# INIMPL04 Worker Handoff

Evidence: Mixed (`Ran` + `Static`)

## Scope Completed

Implemented `SC-INFILE-SOIL-001` parser surface in owned write set:
- strict/compat parse modes
- datver allowlist + explicit compatibility alias gate
- datver-specific OFE policy row parsing (`9002`, `9003`, `9005`)
- datver-specific layer arity parsing (`97.5`, `2006.2`, `7777`, `7778`, `9002+`)
- restrictive-layer footer parsing for datvers that require it
- invariant/guard checks mapped to `SOL-E-*` typed errors
- integration tests + fixtures for success/failure branches

## Files Implemented

- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/crates/openwepp-input-contract/src/parsers/soil.rs`
- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/tests/integration/infile_soil_parser_contract.rs`
- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/tests/fixtures/infile/soil/valid_97_5.sol`
- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/tests/fixtures/infile/soil/valid_9002.sol`
- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/tests/fixtures/infile/soil/unknown_datver.sol`
- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/tests/fixtures/infile/soil/invalid_layer_arity_9002.sol`
- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/tests/fixtures/infile/soil/invalid_non_monotone_depth.sol`
- `/home/workdir/openWEPP/.worktrees/inimpl04-soil/tests/fixtures/infile/soil/alias_97_0.sol`

## Check Evidence

### Ran

1. `rustfmt crates/openwepp-input-contract/src/parsers/soil.rs tests/integration/infile_soil_parser_contract.rs`
- Result: pass.

2. `rustc --edition=2024 --test tests/integration/infile_soil_parser_contract.rs -o /tmp/infile_soil_parser_contract && /tmp/infile_soil_parser_contract`
- Result: pass (`7` tests passed).

3. `cargo test --workspace`
- Result: blocked.
- Output: `manifest ... contains no package: The manifest is virtual, and the workspace has no members.`

### Static

- Contract alignment cross-check performed against:
  - `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
  - `docs/specifications/wepp-input-files/specs/soil-file.spec.md`

## Blockers

1. Workspace bootstrap gap (external to owned write set): root workspace has no members, so standard workspace gates cannot run yet.
- Impact: `cargo test --workspace` and related workspace-level gates are not executable in this package.
- Suggested follow-on: integration package (`INIMPL07`) should add/activate workspace members and run full cargo gates.

## Integration Notes for INIMPL07

1. Wire crate/module ownership into a concrete workspace member and expose `soil.rs` through crate API.
2. Re-run canonical gates once workspace members exist:
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
3. Reconcile warning-surface expectations once the parser module is compiled in its final crate context.
