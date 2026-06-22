# Modularization Plan

Evidence class: Static + Ran.

## Decision

Split the textually included hillslope runner body at the first execution
section boundary:

- Source before refactor:
  `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  lines `1743..end`.
- Destination after refactor:
  `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`.
- Retained prefix:
  `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  lines `1..1741`; the old separator blank on line `1742` was not retained
  because it became a trailing blank line at EOF after the split.
- Include wiring:
  `crates/openwepp-runner/src/hillslope/mod.rs` includes
  `05_runner_execution_and_outputs.rs` after `04_direct_publication.rs` and
  before `03_tests.rs`.

## Moved Items

The moved tail contains the execution/output/manifest and run-entrypoint block:

- `execute_hillslope_climate_days`
- `ClimateExecutionAccumulator` implementation
- `build_day_climate_surface`
- `build_hillslope_execution_provenance`
- `write_hillslope_run_outputs`
- `write_hillslope_run_manifest`
- `direct_runtime_audit_delta`
- `execute_hillslope_run`
- `execute_hillslope_run_with_runtime_selection`
- `select_direct_runtime_skeleton_once`

## Mechanical-Move Evidence

Ran:

```text
git show HEAD:crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  | sed -n '1743,$p' \
  | cmp -s - crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs
echo $?
```

Result: `0`.

Ran:

```text
git show HEAD:crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  | sed -n '1,1741p' \
  | cmp -s - crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
echo $?
```

Result: `0`.

The new file is byte-for-byte the old tail, and the retained file is
byte-for-byte the old nonblank prefix. The only dropped production-file byte is
the old separator blank that would otherwise become a trailing blank line at
EOF. No production function body, signature, formula, guard, threshold,
visibility, or call site was edited.

## Static-Scan Fallout

Two source-level integration tests were updated to scan both included runner
files because their asserted production tokens moved with the tail:

- `tests/integration/mofe01_per_ofe_state_contract.rs`
- `tests/integration/mofe01_inter_ofe_route_contract.rs`

The assertions remain unchanged; only the inspected source-file set changed.
