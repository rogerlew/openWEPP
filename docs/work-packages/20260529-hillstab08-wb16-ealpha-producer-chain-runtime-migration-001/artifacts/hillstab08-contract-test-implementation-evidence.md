# HILLSTAB08 Contract-Test Implementation Evidence

Status: complete  
Evidence mode: mixed (`Static` + `Ran`)

## Scope
- Add/adjust contract-derived vectors for WB16 producer-chain runtime behavior.

## Static Evidence (implemented tests)
- `crates/openwepp-runner/src/hillslope/mod.rs`
  - Added `hillstab08_wb16_producer_single_ofe_projects_expected_alpha_lineage`.
  - Added `hillstab08_wb16_producer_multiofe_projects_expected_equivalent_plane_alpha`.
  - Added deterministic helper projections for expected `frcteq` and multi-OFE
    `ealpha` equivalent-plane computation.
- `tests/integration/cli03_runner_contract_derived_tests.rs`
  - Updated WB16 provenance assertion to runtime-producer policy:
    `wb16_ealpha_compatibility_seed_used=false`,
    `wb16_ealpha_seed_policy=runtime_provided`, no `SIMPIPE-W-003`.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - Expanded management projection test coverage for WB16 producer-input symbols
    (`inrcov`, `rilcov`, `rrinit`, `rspace`, `width`, `bbb_seed`,
    `flivmx_seed`, `hmax_seed`).

## Ran Evidence
- `cargo test -p openwepp-runner hillstab08_wb16_producer`
- `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_fixture_run_publishes_wb16_ealpha_runtime_seed_provenance`
- `cargo test -p openwepp-hillslope-orchestrator management_runtime_surfaces_project_required_pl_controls_and_seeds`
