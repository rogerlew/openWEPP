# Line-Count Governance

Evidence mode: Static + Ran.

Ran:

- `wc -l crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/03_tests.rs crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs`

Result:

| File | Lines | Status |
|---|---:|---|
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | `3234` | FAIL: above 3000-line governance threshold. |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | `1040` | PASS. |
| `crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs` | `81` | PASS. |

Disposition:

- R6D is executed-held, not complete.
- The touched runner module was already in the high-risk monolith category, and
  R6D added additional direct-publication helpers there to preserve minimal
  behavioral scope.
- Next follow-on should split the direct publication retention/cutover helpers
  out of `00_runner_intake_and_lane_setup.rs` before any package claims full R6
  closure.
- This line-count failure is a closure blocker for complete status, not a
  blocker for the R6D executed-held handoff because public output cutover remains
  fail-closed.
