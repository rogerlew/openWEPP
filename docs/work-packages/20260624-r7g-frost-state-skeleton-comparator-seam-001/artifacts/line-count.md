# Line-Count Governance

Status: complete.

Evidence mode: Ran.

Ran:

- `wc -l` over touched Rust files.

Touched file counts:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`: 235.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`: 2315, WARN.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`: 2584, WARN.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs`: 76.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs`: 326.
- `crates/openwepp-hillslope-orchestrator/src/winter_column.rs`: 348.
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`: 1812.
- `crates/openwepp-runner/src/hillslope/03_tests.rs`: 2736, WARN.
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`: 1789.
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`: 4.
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`: 1825.
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs`: 1743.
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/03_frost_comparator_seam.rs`: 17.

Disposition:

- No touched production file exceeds 3000 lines.
- Existing WARN files remain below the 3000-line refactor-required threshold.
- The only touched file over 3000 lines in the repository-wide scan is the
  preexisting orchestrator test file
  `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
  at 4091 lines; this package did not edit that file.
