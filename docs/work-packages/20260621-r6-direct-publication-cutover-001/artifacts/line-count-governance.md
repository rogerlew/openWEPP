# Line Count Governance

Status: executed-hold.
Evidence mode: Ran.

## Requirement

Before closure, measure touched `.rs` files. Files at or above 2000 lines
require WARN disposition. Non-exempt files at or above 3000 lines require
refactor or a package-authorized blocker before completion.

## Ran

```text
wc -l crates/openwepp-runner/src/api.rs \
  crates/openwepp-runner/src/bin/openwepp-cli-hill.rs \
  crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs \
  crates/openwepp-runner/src/hillslope/03_tests.rs \
  crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs
```

Result:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-runner/src/api.rs` | 61 | PASS |
| `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs` | 124 | PASS |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 2823 | WARN |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | 2319 | WARN |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | 913 | PASS |
| `crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs` | 77 | PASS |

## Disposition

No touched `.rs` file is at or above the 3000-line refactor blocker. The two
WARN-band runner helper files predate this package's narrow cutover-candidate
edit. R6 cannot close anyway due parity/manifest blockers; follow-on direct
publication work should split output-family cutover helpers before expanding
these files further.
