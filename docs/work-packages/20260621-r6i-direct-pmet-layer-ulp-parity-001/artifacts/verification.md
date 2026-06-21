# R6I Verification

Evidence class: Ran.

## Gate Table

| Gate | Status | Evidence |
| --- | --- | --- |
| Localize PMET layer-carry bit divergence | PASS | HPHYS trace and focused seed-surface test localized divergence to post-runoff-reconciliation layer carry. |
| Correct direct layer carry exactness without compatibility aliases | PASS | `DirectFrostLayerCarryProjection` applied during direct lane commit; static proof recorded in `execution-evidence.md`. |
| WAT `Es`, `Total-Soil`, and `SoilWaterTotal` parity | PASS | `r6i_cutover_candidate_hbp_and_wat_identity_clear_pmet_layer_ulp_gap` passed with empty reduced mismatch fields. |
| HBP identity preserved | PASS | Same focused R6I fixture verifies direct and compatibility HBP bytes/payloads match. |
| R6G/R6H markers absent | PASS | R6I focused and CLI tests assert no `HOLD-R6G-*` or `HOLD-R6H-*` marker remains. |
| CLI cutover fail-closed or public-write success | PASS | CLI cutover still fails closed before writes, now at manifest writer wiring. |
| `cargo fmt --check` | PASS | Ran after final edits. |
| `cargo check -p openwepp-runner -p openwepp-hillslope-orchestrator` | PASS | Ran after final edits. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran after final edits. |
| `cargo test --workspace` | PASS | Full workspace test suite passed after updating stale R6H integration expectation. |
| `cargo deny check` | PASS | advisories, bans, licenses, sources OK. |
| Static no-compatibility proof | PASS | Production correction path uses direct seed/frost/layer symbols only. |
| Line-count governance | PASS | Touched Rust files are below 3000 lines. |

## Verification Notes

The direct cutover candidate remains fail-closed because manifest direct
projection is not wired to the production manifest writer. This is not an R6I
failure because R6I acceptance was scoped to current-fixture PMET/WAT layer
carry parity plus fail-closed behavior while broader R6 publication cutover
gates remain pending.
