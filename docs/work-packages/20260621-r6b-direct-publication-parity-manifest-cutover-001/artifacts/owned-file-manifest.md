# Owned File Manifest

Status: executed-hold.
Evidence mode: Static + Ran.

## Package Files

- `docs/work-packages/20260621-r6b-direct-publication-parity-manifest-cutover-001/package.md`
- `docs/work-packages/20260621-r6b-direct-publication-parity-manifest-cutover-001/artifacts/**`
- `docs/work-packages/20260621-r6b-direct-publication-parity-manifest-cutover-001/prompts/**`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/artifacts/worker-handoff.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

## Touched Implementation Files

- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs`

The Rust edit is limited to a fail-closed blocker diagnostic and tests proving
that the cutover failure reports absent typed operands. It does not implement
or claim production publication cutover.

## Expected Implementation Files Still Needed For Hold-Lift

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/**`
- `crates/openwepp-runner/src/api.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-runner/tests/**`
- `crates/openwepp-hillslope-output/src/**`
- `tests/integration/**`

## Conditional Files

- `docs/architecture/array-native-runtime-specification.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `tools/**`

Files outside the package write set require package amendment before edits.
