# Line-Count Governance

Evidence: Ran (`wc -l`) after the WS-2 port and direct-publication helper split.

| Touched Rust file | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 1923 | PASS; reduced from pre-refactor 4144 and below WARN. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs` | 671 | PASS. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00b_ksatadj_authority_impl.rs` | 175 | PASS; new WS-2 runner helper. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs` | 1039 | PASS; mechanical split target for day-input builder and selector helpers. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00d_authority_runtime_impl.rs` | 1218 | PASS; mechanical split target for authority runtime impls. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs` | 582 | PASS. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/02_publication_and_manifest_helpers.rs` | 447 | PASS. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/ksatadj.rs` | 675 | PASS; new evaluator/conformance module. |
| `tests/integration/dff_ws2_ksatadj_direct_runtime.rs` | 128 | PASS. |

No touched Rust file remains at or above the 2000-line WARN threshold, and no
3000+ non-exempt line-count blocker remains.
