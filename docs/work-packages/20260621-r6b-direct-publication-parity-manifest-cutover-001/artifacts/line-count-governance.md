# Line Count Governance

Status: executed-hold.
Evidence mode: Ran.

Before closure, measure every touched `.rs` file. Files at or above 2000 lines
require WARN disposition. Non-exempt files at or above 3000 lines require
refactor or a package-authorized hold before completion.

## Measurement

Ran:

```text
2884 crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
1025 crates/openwepp-runner/src/hillslope/03_tests.rs
  81 crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs
```

## Disposition

- `00_runner_intake_and_lane_setup.rs` is in WARN band above 2000 lines.
- No touched Rust file is at or above 3000 lines.
- R6B closes in hold before a production bridge implementation, so the WARN
  split remains follow-on governance for the hold-lift package rather than a
  hidden closure exception.
