# Characterization

Evidence label: Static/Ran.

Status: `EXECUTED`

Behavior oracle:

- module-local parser tests assert top-level command classification, common
  snowbench argument parsing, Jennings argument parsing, help short-circuiting,
  missing-value errors, unknown-argument errors, command-error precedence, and
  command-specific guard errors;
- `jennings_phase_run_accepts_minimal_valid_fixture` writes tiny Jennings file2
  and file3 CSVs under `target/openwepp_snowbench_cli_tests/`, runs the CLI
  wrapper through the success path, and asserts JSON/Markdown report files are
  emitted;
- existing integration tests continue to assert that snowbench CLI selectors
  stay confined to diagnostic surfaces and that the offline physics-bulk
  snowbench fixture still runs.

Ran:

- `cargo nextest run -p openwepp-runner --bin openwepp-snowbench` - exit `0`,
  `9 tests run: 9 passed, 0 skipped`.
- `cargo nextest run --test snowdensity05f_melt_closure_handoff --test snowdensity03_physics_bulk_offline_contract` - exit `0`, `5 tests run: 5 passed, 0 skipped`.
