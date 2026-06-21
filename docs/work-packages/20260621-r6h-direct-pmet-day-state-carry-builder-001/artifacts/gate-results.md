# R6H Gate Results

Status: executed-held.

| Gate | Command/evidence | Result | Notes |
|---|---|---|---|
| Formatting | Ran: `cargo fmt --check` | PASS | Clean after final code and test edits. |
| Focused check | Ran: `cargo check -p openwepp-runner -p openwepp-hillslope-orchestrator` | PASS | Touched runner/orchestrator crates compile. |
| Clippy | Ran: `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Re-run after test isolation fix; no warnings. |
| Focused interleaved day-input tests | Ran: `cargo test -p openwepp-hillslope-orchestrator r6h_publication -- --nocapture` | PASS | `r6h_publication_capture_builds_lane_day_inputs_after_direct_commit` passed. |
| Focused WAT parity tests | Ran: `cargo test -p openwepp-runner r6h_ -- --nocapture` | HELD | Four focused R6H tests passed; expected terminal is `HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`, not WAT parity. |
| CLI cutover fail-closed/public-write contract | Ran: `cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract -- --nocapture` | PASS | Cutover exits fail-closed with R6H marker and no partial direct public outputs. |
| Multi-OFE/lane anti-alias fixture | Static + focused interleaved builder tests | PARTIAL | Builder API is lane-dimensional and tested for lane/day invocation order. Canonical multi-OFE WAT id/output parity remains held after the current WAT `Es` blocker. |
| WAT id authority evidence | Static: `artifacts/r6h-wat-id-authority.md` | HELD | R6H did not alter WAT id semantics. Current fixture still uses inherited single-WAT id; broader WAT id authority cannot close before WAT parity. |
| Static no-compatibility scan | Ran: `rg -n "HOLD-R6G...|HOLD-R6H...|run_publication_capture_with_interleaved_day_inputs|build_hillslope_wat_rows_from_direct_publication|execution\\.wb13_rows|compatibility_wat_rows|runtime_surface|writeback_payload|writer_rows" ...` | PASS with expected hits | Direct producer path uses `build_hillslope_wat_rows_from_direct_publication`. Compatibility rows appear only in parity gate/test comparison code. Seed runtime surfaces are private input-construction surfaces, not WAT output authority. |
| Independent WAT reconstruction | Static + focused assertions in `assert_r6h_wat_reduced_to_pmet_layer_ulp_gap` | HELD | HBP identity passes and WAT storage totals are bit-identical; independent reconstruction remains blocked by `Es` parity. |
| Workspace tests | Ran: `cargo test --workspace` | PASS | First run exposed a test counter race; after locking the new overlay test, full workspace passed. |
| Dependency policy | Ran: `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Line count | Ran: `wc -l` over touched `.rs` files | WARN | Three touched files are in 2000+ WARN band; none are >=3000. See `line-count-governance.md`. |

## Terminal Gate

R6H exits at:

`HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`

The prior marker
`HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT` no longer fires in the
focused runner and CLI cutover tests.
