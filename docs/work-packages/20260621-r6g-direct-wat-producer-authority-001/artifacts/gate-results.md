# R6G Gate Results

Status: complete.

| Gate | Command/evidence | Result | Notes |
|---|---|---|---|
| Focused check | `cargo check -p openwepp-runner -p openwepp-hillslope-orchestrator` | PASS | Typed direct publication/orchestrator changes compile. |
| Focused runner R6G tests | `cargo test -p openwepp-runner r6g_cutover_candidate -- --nocapture` | PASS | HBP identity green; first WAT row parity; remaining exact PMET day-state carry gap. |
| CLI cutover fail-closed contract | `cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract r6_direct_publication_cutover_cli_flag_reaches_hbp_identity_then_fails_pmet_day_state_carry -- --nocapture` | PASS | CLI reports `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`. |
| Direct runtime typed input carry | `cargo test -p openwepp-hillslope-orchestrator r6f_publication_capture_accepts_typed_process_inputs_and_carries_layers -- --nocapture` | PASS | Protects direct runtime typed process input carry. |
| Marker reservation | `cargo test -p openwepp-runner r6g_wat_hold_marker_is_reserved_for_exact_pmet_day_state_carry_fields -- --nocapture` | PASS | Marker does not hide unrelated WAT fields. |
| Residual storage projection | `cargo test -p openwepp-hillslope-orchestrator r4pqz_projection_includes_residual_water_in_layer_storage -- --nocapture` | PASS | Protects residual liquid water contribution. |
| Static no-compatibility scan | `rg -n "wb13_rows|build_hillslope_wat_rows_from_wb13|compatibility|writeback|writer row|writer_row|runtime_surface" crates/openwepp-runner/src/hillslope/04_direct_publication.rs crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/projection.rs` | PASS | Hits are parity comparators, output helper surfaces, static direct seed surface, or audit counters; not WAT direct producer authority. |
| Line count | `wc -l` over touched `.rs` files | WARN | Three touched files exceed 2000 lines; none exceed the 3000-line hard stop. |
| Formatting | `cargo fmt --check` | PASS | Clean after final R6G edits. |
| Clippy | `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Clean after request-struct extraction and cutover-gate helper split. |
| Workspace tests | `cargo test --workspace` | PASS | Full workspace test suite passed. |
| Dependency policy | `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Diff hygiene | `git diff --check` | PASS | No whitespace errors. |
| Docs lint provenance | `wctl doc-lint` | PASS-LIMITED | Ran default staged-only doc lint before staging; it scanned 0 files and reported 0 findings. This is recorded as provenance, not substantive Markdown validation. |

## Terminal Gate Result

All executable gates passed for the held package state. R6G still cannot close
as complete because the cutover candidate intentionally fails closed at
`HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`.
