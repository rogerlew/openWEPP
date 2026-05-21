# Disposition — INIMPL13

Evidence mode: `Ran` + `Static`

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `FDIR-A-001` | `review_agent_a` | high | `open_hold` | No API broadening in this package; parser remains file-local by write-set scope. | [`irrigation_fixeddate.rs:304`](/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate/crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs:304) | Requires context-bearing parse interface for `FDIR-E-006` closure. |
| `FDIR-A-002` | `review_agent_a` | medium | `open_followup` | `FDIR-W-005` code retained for taxonomy continuity; no emission path added. | [`irrigation_fixeddate.rs:40`](/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate/crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs:40) | Follow-on branch should add context-gated furrow disable behavior. |
| `FDIR-B-001` | `review_agent_b` | high | `open_hold` | Furrow disallow policy not implemented because contour/non-cropland context is not available to this parser function surface. | [`irrigation_fixeddate.rs:440`](/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate/crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs:440) | Contract guard `G-FDIR-013` remains open. |
| `FDIR-B-002` | `review_agent_b` | medium | `open_followup` | Existing tests are scoped to parser-local behavior only. | [`infile_irrigation_fixeddate_parser_contract.rs:12`](/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate/tests/integration/infile_irrigation_fixeddate_parser_contract.rs:12) | Add integration-context fixtures when coupling interface lands. |

## Status
- Closed findings: none.
- Open high-severity findings: `FDIR-A-001`, `FDIR-B-001`.
- Package recommendation: `HOLD` (parser surface delivered with explicit unresolved contract-coupling gaps).
