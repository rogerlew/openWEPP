# Owned File Manifest

Status: complete
Evidence mode: Static

Expected write set:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md` only if lifecycle metadata requires it.
- `Cargo.toml`
- `Cargo.lock`
- `crates/openwepp-meteorology/**`
- `tests/integration/snowdensity10_3_5a_meteorology_crate_contract.rs`
- `docs/work-packages/20260627-snowdensity-10-3-5a-openwepp-meteorology-crate-001/**`

Actual touched files:

- Static: `Cargo.toml`
- Static: `Cargo.lock`
- Static: `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- Static: `docs/specifications/science-contracts/index.md`
- Static: `crates/openwepp-meteorology/Cargo.toml`
- Static: `crates/openwepp-meteorology/src/lib.rs`
- Static: `crates/openwepp-meteorology/src/error.rs`
- Static: `crates/openwepp-meteorology/src/psychrometrics.rs`
- Static: `crates/openwepp-meteorology/src/phase.rs`
- Static: `tests/integration/snowdensity10_3_5a_meteorology_crate_contract.rs`
- Static: existing SNOWDENSITY contract guard tests updated from
  `contract_version: 90` to `contract_version: 91` after the contract bump:
  - `tests/integration/snowdensity02_contract_adr_guard.rs`
  - `tests/integration/snowdensity05a_melt_contract_guard.rs`
  - `tests/integration/snowdensity05b_shortwave_source_contract.rs`
  - `tests/integration/snowdensity05c_albedo_state_core.rs`
  - `tests/integration/snowdensity05d_opt_in_coe_melt.rs`
  - `tests/integration/snowdensity05f_melt_closure_handoff.rs`
  - `tests/integration/snowdensity05g_harness_fidelity_rerun.rs`
  - `tests/integration/snowdensity06_density_compaction.rs`
  - `tests/integration/snowdensity06b_coe_bound_density_replay.rs`
  - `tests/integration/snowdensity07_runtime_opt_in.rs`
  - `tests/integration/snowdensity08_gate_rerun.rs`
  - `tests/integration/snowdensity09_coupled_wat_rerun.rs`
  - `tests/integration/snowdensity10_3_1a_per_day_cancov.rs`
- Static: `docs/work-packages/README.md`
- Static: package `package.md`
- Static: package-local artifacts under
  `docs/work-packages/20260627-snowdensity-10-3-5a-openwepp-meteorology-crate-001/artifacts/`.

No production runtime, runner, parser, output-schema, fixture, or compatibility
runtime source file was touched.
