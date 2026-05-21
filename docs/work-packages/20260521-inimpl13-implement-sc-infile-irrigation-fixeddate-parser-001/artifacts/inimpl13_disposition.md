# Disposition — INIMPL13

Evidence mode: `Ran` + `Static`

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `FDIR-A-001` | `review_agent_a` | high | `open_followup` | No API broadening in this package; parser remains file-local by write-set scope. | [`irrigation_fixeddate.rs:304`](/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate/crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs:304) | Closure for `FDIR-E-006` is assigned to downstream cross-validation surface per contract ownership split. |
| `FDIR-A-002` | `review_agent_a` | medium | `open_followup` | `FDIR-W-005` code retained for taxonomy continuity; no emission path added. | [`irrigation_fixeddate.rs:40`](/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate/crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs:40) | Follow-on branch should add context-gated furrow disable behavior. |
| `FDIR-B-001` | `review_agent_b` | high | `open_followup` | Furrow disallow policy is intentionally deferred because contour/non-cropland context is not available to this parser function surface. | [`irrigation_fixeddate.rs:440`](/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate/crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs:440) | Contract guard `G-FDIR-013` is assigned to downstream cross-validation surface. |
| `FDIR-B-002` | `review_agent_b` | medium | `open_followup` | Existing tests are scoped to parser-local behavior only. | [`infile_irrigation_fixeddate_parser_contract.rs:12`](/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate/tests/integration/infile_irrigation_fixeddate_parser_contract.rs:12) | Add integration-context fixtures when coupling interface lands. |

## Status
- Closed findings: none.
- Open high-severity follow-ups: `FDIR-A-001`, `FDIR-B-001` (cross-validation surface ownership).
- Package recommendation: `GO-WITH-AMENDMENTS` (parser-local surface complete; downstream cross-validation follow-up required).

## Decision Update (2026-05-21)
- Validation ownership decision ratified: run-context and cross-file gates for fixed-date irrigation
  (`G-FDIR-009`, `G-FDIR-010`, `G-FDIR-013`) are implemented in a downstream cross-validation surface,
  not by broadening the file-local parser API.
