# Review Agent A — INIMPL25

Evidence: `Ran` + `Static`

## Findings (Severity-Ranked)

### INIMPL25-A-001 — High
- File: `/home/workdir/openWEPP/.worktrees/inimpl25-tc/crates/openwepp-input-contract/src/parsers/tc.rs`
- Issue: None found. Contract-critical strict/compat branches, typed IDs (`TC-E-000`, `TC-E-001`, `TC-W-001..003`), and watershed-only guard are implemented and covered by tests.
- Why it matters: `SC-INFILE-TC-001` requires deterministic sentinel-mode behavior without silent strict-mode IO masking.
- Proposed disposition: `close`.

### INIMPL25-A-002 — Medium
- File: `/home/workdir/openWEPP/.worktrees/inimpl25-tc/Cargo.toml`
- Issue: New TC integration test target is not registered in shared test-target list from this worker stream.
- Why it matters: `cargo test --workspace` does not include `infile_tc_parser_contract` until shared-file integration applies registration.
- Proposed disposition: `amend` (handoff request to integration stream; retain direct `rustc --test` run evidence).

## Final Recommendation

`GO-WITH-AMENDMENTS`
