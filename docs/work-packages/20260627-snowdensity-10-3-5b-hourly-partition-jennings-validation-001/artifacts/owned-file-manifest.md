# Owned File Manifest

Status: complete
Evidence mode: Static/Ran

Primary implementation files:

- `Cargo.lock`
- `Cargo.toml`
- `crates/openwepp-hillslope-orchestrator/Cargo.toml`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/climate.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/common.rs`
- `crates/openwepp-runner/Cargo.toml`
- `crates/openwepp-runner/src/bin/openwepp-snowbench.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-runner/src/hillslope/snowbench.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_jennings_phase.rs`
- `crates/openwepp-runner/src/hillslope/symbol_registry_audit.rs`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-sim-contract/src/units_mod/boundary_catalog.rs`

Contract and tests:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `tests/integration/sim_contract_boundary_unit_registry.rs`
- `tests/integration/snowdensity10_3_5b_hourly_partition_jennings_contract.rs`
- Existing snowdensity contract tests updated from `contract_version: 91` to
  `92`.

Documentation/evidence:

- `docs/work-packages/README.md`
- `docs/work-packages/20260627-snowdensity-10-3-5b-hourly-partition-jennings-validation-001/**`

Ran: `git diff --name-only` and `git status --short` to verify the modified and
new file set.
