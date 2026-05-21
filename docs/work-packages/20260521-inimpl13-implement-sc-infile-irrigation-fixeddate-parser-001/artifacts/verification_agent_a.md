# Verification Agent A — INIMPL13

Evidence: `Ran` + `Static`

## Per-Finding Verification

| finding_id | source | severity | disposition_decision | verifier_verdict | evidence | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `FDIR-A-001` | `review_agent_a` | high | `open_hold` | `open_confirmed` | [`irrigation_fixeddate.rs:304`](/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate/crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs:304) | No run-context coupling inputs exist in parser API. |
| `FDIR-A-002` | `review_agent_a` | medium | `open_followup` | `open_confirmed` | [`irrigation_fixeddate.rs:40`](/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate/crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs:40) | Warning code is declared but unused in current execution path. |
| `FDIR-B-001` | `review_agent_b` | high | `open_hold` | `open_confirmed` | [`irrigation_fixeddate.rs:440`](/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate/crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs:440) | Furrow disallow context policy not enforced. |
| `FDIR-B-002` | `review_agent_b` | medium | `open_followup` | `open_confirmed` | [`infile_irrigation_fixeddate_parser_contract.rs:12`](/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate/tests/integration/infile_irrigation_fixeddate_parser_contract.rs:12) | Tests validate parser-local branches only. |

## Package Verdict

`HOLD`
