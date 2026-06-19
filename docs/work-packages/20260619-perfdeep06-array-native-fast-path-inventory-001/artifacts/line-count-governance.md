# PERFDEEP06 Line-Count Governance

Status: complete 2026-06-19.
Evidence class: Ran + Static.

## Requirement

Record `.rs` file line-count governance for source files inspected or proposed
for follow-on implementation. Files at or above 2000 lines are `WARN`; files at
or above 3000 non-exempt lines require a refactor disposition before
implementation closure.

## Command

Ran:

    find crates -name '*.rs' -type f -print0 | xargs -0 wc -l | awk '$1 >= 2000 {print}' | sort -nr

## Results

Files at or above the 2000-line WARN threshold:

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` - 3177 lines.
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` -
  2433 lines.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs` -
  2186 lines.
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` -
  2095 lines.
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` - 2062 lines.
- `crates/openwepp-watershed-output/src/writers.rs` - 2002 lines.

Disposition: PERFDEEP06 made no Rust edits, so no refactor was required inside
this planning package. PERFDEEP07's intended write set includes
`scheduler.rs`, `state_access.rs`, and `02_output_and_climate_helpers.rs`; it
must carry explicit WARN/3000+ disposition. Because `scheduler.rs` is above
3000 lines, PERFDEEP07 must either keep edits narrowly bounded with an approved
package exception and sunset plan, or split the touched scheduler code before
closure.
