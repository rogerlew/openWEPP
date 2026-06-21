# R6F Gate Results

Status: executed-held.

## Focused Iteration Gates

| Date | Gate | Command | Result | Evidence |
|---|---|---|---|---|
| 2026-06-21 | HBP/WAT blocker reduction | `cargo test -p openwepp-runner r6f_cutover_candidate_hbp_identity_exposes_wat_producer_gap -- --nocapture` | Passed | HBP bytes equal on current fixture; WAT accepted fields match; WAT producer fields differ. |
| 2026-06-21 | Stable cutover hold marker | `cargo test -p openwepp-runner r6f_cutover_candidate_reaches_hbp_identity_then_fails_wat_producer_authority -- --nocapture` | Passed | Cutover reaches current-fixture HBP identity then fails at `HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`. |
| 2026-06-21 | WAT marker exact-field guard | `cargo test -p openwepp-runner r6f_wat_hold_marker_is_reserved_for_exact_producer_gap_fields -- --nocapture` | Passed | Ensures unrelated WAT drift is not mislabeled as the R6F producer-authority gap. |
| 2026-06-21 | CLI fail-closed contract | `cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract -- --nocapture` | Passed | No public direct outputs are written under hold. |
| 2026-06-21 | Direct runtime typed input/carry | `cargo test -p openwepp-hillslope-orchestrator r6f_publication_capture_accepts_typed_process_inputs_and_carries_layers -- --nocapture` | Passed | Direct runtime publishes supplied ET/storage/profile operands and carries layers. |
| 2026-06-21 | Hydrology projection regression | `cargo test -p openwepp-hillslope-orchestrator direct_runtime_r4pqz -- --nocapture` | Passed | Existing R4PQZ projection behavior remains green after profile field expansion. |
| 2026-06-21 | Formatting | `cargo fmt --check` | Passed | Ran after `cargo fmt`. |
| 2026-06-21 | Compile check | `cargo check -p openwepp-hillslope-orchestrator -p openwepp-runner` | Passed | Struct shape and runner changes compile. |
| 2026-06-21 | Clippy | `cargo clippy --workspace --all-targets -- -D warnings` | Passed | Fixed clone-assignment, function-length, float-compare, and line-count issues. |
| 2026-06-21 | Workspace tests | `cargo test --workspace` | Passed | Rerun after moving WAT reducer out of `00_runner_intake_and_lane_setup.rs`. |
| 2026-06-21 | Dependency policy | `cargo deny check` | Passed | `advisories ok, bans ok, licenses ok, sources ok`. |
| 2026-06-21 | Diff whitespace | `git diff --check` | Passed | No whitespace errors. |
| 2026-06-21 | Work-package docs lint | `wctl doc-lint --path docs/work-packages` | Passed | 960 files validated, 0 errors, 0 warnings. |

## Final Gates

Repository-level Rust and docs gates passed for the executed-held R6F change
set. Full R6 publication cutover parity is still not complete because WAT
producer authority, nonzero HBP fixture coverage, PASS/loss/manifest parity,
and public direct-output writes remain R6G/R6 continuation gates.

## Current Terminal Marker

`HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`
