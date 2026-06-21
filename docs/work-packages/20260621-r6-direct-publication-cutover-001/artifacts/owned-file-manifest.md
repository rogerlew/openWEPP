# Owned File Manifest

Status: executed-hold.
Evidence mode: Static.

## Package Scaffold

- `docs/work-packages/20260621-r6-direct-publication-cutover-001/package.md`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/artifacts/**`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/prompts/**`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/work-packages/r5-burndown-execplan.md`

## Rust / Runner Files Touched

- `crates/openwepp-runner/src/api.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs`

## Package Amendment

`package.md` intended write set now includes `api.rs`,
`openwepp-cli-hill.rs`, and `crates/openwepp-runner/tests/**` because R6
execution needed an explicit opt-in runtime selection, CLI flag, and CLI
fail-closed contract coverage for the guarded cutover candidate.

## Gate

PASS for the executed-hold write set. No output schema files, science
contracts, watershed fan-in code, or compatibility adapters were deleted.
