# R6F No-Compatibility Proof

Status: executed-held.

## Forbidden Authority Sources

The direct cutover path must not treat these as direct publication authority:

- compatibility WB13 rows;
- compatibility runtime surfaces;
- writeback payloads;
- stale logical state;
- skeleton direct frame capture;
- output writer self-consistency values;
- wrappers around compatibility structures with direct names.

## Static Findings

| Path | Finding | Disposition |
|---|---|---|
| `build_direct_publication_artifacts` | Cutover consumes retained `DirectPublicationExecution` and builds direct HBP/WAT/PASS/loss/manifest candidates from `DirectRunPublicationFrame`. | Acceptable direct consumer path. |
| `require_direct_publication_cutover_gates` | Reads compatibility HBP/WAT/loss/PASS only for parity comparison before public writes. | Acceptable gate comparison, not direct authority. |
| R6F rejected shortcut | Remaining WAT fields could be copied from `execution.wb13_rows`/`execution.runtime_surface`, but that would violate architecture section 5.2.1. | Rejected; hold rather than alias. |
| R6F direct runtime | `DirectPublicationDayInput` now exposes optional typed process inputs and lane-carried layers. | Direct receiving surface exists. |
| Production runner | `direct_publication_day_inputs` still fills only calendar, precipitation, and effective temperature. | Current blocker; R6G must add parsed-input producer. |

## Commands

| Command | Result | Notes |
|---|---|---|
| `rg -n "A compatibility WB13 row|runtime symbol|not a valid direct source|from compatibility WB13 rows or runtime surfaces" docs/architecture/array-native-runtime-specification.md` | Ran | Confirmed section 5.2.1 forbids compatibility structures as direct authority. |
| `cargo test -p openwepp-runner r6f_cutover_candidate_hbp_identity_exposes_wat_producer_gap -- --nocapture` | Passed | HBP direct bytes equal compatibility; WAT mismatch remains reduced. |
| `cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract -- --nocapture` | Passed | Public outputs are not written while WAT producer authority is absent. |

## Residual Risk

This is not a complete R6 no-compatibility proof because WAT parity is not
closed. It is sufficient for the R6F hold: the remaining viable implementation
path is a typed parsed-input producer, not a compatibility wrapper.
