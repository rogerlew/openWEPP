# Review Agent A — INIMPL13

Evidence: `Ran` + `Static`

## Findings (Severity-Ranked)

### FDIR-A-001 — High
- File: [`irrigation_fixeddate.rs:304`](/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate/crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs:304)
- Issue: Cross-file coupling guards (`FDIR-E-006`, `G-FDIR-009`, `G-FDIR-010`) are not enforced because parser APIs accept only file content + mode, not run-context metadata.
- Why it matters: Contract requires validation against topology/system/schedule context (`itemp`, `jtemp`, `ktemp` couplings).
- Proposed disposition: `HOLD` pending parser context interface extension.

### FDIR-A-002 — Medium
- File: [`irrigation_fixeddate.rs:40`](/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate/crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs:40)
- Issue: `FDIR-W-005` warning code is declared for continuity but has no emission path.
- Why it matters: Furrow disable compatibility behavior remains implicit/unimplemented.
- Proposed disposition: `amend` in follow-on package with context-aware furrow-policy branch.

Final recommendation: `HOLD`
