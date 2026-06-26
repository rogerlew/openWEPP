# Line-Count Governance Checklist

Evidence class: Static.

Touched Rust files:

- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `tests/integration/snowdensity03_physics_bulk_offline_contract.rs`
- `tests/integration/snowdensity09_coupled_wat_rerun.rs`
- Existing SNOWDENSITY contract-version guard tests.

Disposition:

- No new Rust file approaches the 2,000-line warning threshold.
- The touched direct-publication builder remains pre-existing large-file debt;
  SNOWDENSITY-09 made a narrow package-bound diagnostic hook and did not expand
  scope into mechanical refactor.
