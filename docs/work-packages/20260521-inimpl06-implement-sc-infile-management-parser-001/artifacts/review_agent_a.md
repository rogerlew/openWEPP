# Review Agent A — INIMPL06

Evidence: `Ran` + `Static`

## Findings (Severity-Ranked)

### MAN-A-001 — High
- File: `/home/workdir/openWEPP/.worktrees/inimpl06-management/crates/openwepp-input-contract/src/parsers/management.rs:331`
- Issue: Parser rejects any non-zero scenario section count (`ncrop`, `nop`, `nini`, `nseq`, `ncnt`, `ndrain`, `nscen`) with `NonZeroScenarioSectionUnsupported`.
- Why it matters: This leaves the contract surface incomplete for canonical `.man` files that contain real scenario payloads.
- Proposed disposition: `HOLD` until full section grammar/loop parsing is implemented.

### MAN-A-002 — Medium
- File: `/home/workdir/openWEPP/.worktrees/inimpl06-management/crates/openwepp-input-contract/src/parsers/management.rs:203`, `:218`, `:266`
- Issue: Implemented guards are currently control-surface only (header/count/schedule closure); section payload field-level domains are not covered.
- Why it matters: Parser currently enforces only a subset of `SC-INFILE-MANAGEMENT-001` field taxonomy.
- Proposed disposition: `amend` in follow-on package once shared scaffold exists.

Final recommendation: `HOLD`
