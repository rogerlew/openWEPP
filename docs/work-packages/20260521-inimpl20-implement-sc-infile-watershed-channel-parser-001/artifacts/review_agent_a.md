# Review Agent A — INIMPL20 Parser Implementation

Evidence: Ran + Static

## Findings (severity-ranked)

### INIMPL20-A-001
- Severity: medium
- File/line: `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/Cargo.toml:35`
- Issue: New integration test target `infile_watershed_channel_parser_contract.rs` is not registered in root cargo `[[test]]` entries.
- Why it matters: `cargo test --workspace` does not execute the new watershed-channel contract suite unless integration owner wires shared test registry.
- Proposed disposition: amend under `INIMPL22` shared-file ownership.

### INIMPL20-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel/crates/openwepp-input-contract/src/parsers/mod.rs:1`
- Issue: Shared parser module export for `watershed_channel` is absent in quarantine-owned `mod.rs`.
- Why it matters: Parser is implemented and testable via path harness, but not yet exported through crate parser surface for integration consumers.
- Proposed disposition: amend under `INIMPL22` shared-file ownership.

## Final recommendation
GO-WITH-AMENDMENTS
