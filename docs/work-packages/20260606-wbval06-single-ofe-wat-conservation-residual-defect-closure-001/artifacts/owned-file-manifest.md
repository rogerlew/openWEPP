# Owned-File Manifest

Status: corrected

Evidence mode: executed

Owned write set:

- Files declared in `package.md` under `Intended Write Set`.

Out-of-scope write set:

- WEPPpy files.
- `/wc1/runs/in/indispensable-presenter` input artifacts.
- WBVAL05 percolation closure files unless only referenced as dependency
  context.

Static:

- Touched source/test files:
  - `crates/openwepp-hillslope-output/src/hillslope_wat.rs`
  - `crates/openwepp-runner/src/hillslope/mod.rs`
  - `crates/openwepp-sim-contract/src/units.rs`
  - `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
  - `tests/integration/sim_contract_boundary_unit_registry.rs`
  - `tests/integration/hphys0319_fixed_baseline_stmtim_observe_contract.rs`
  - `tests/integration/hphys0320_stmtim_start_time_source_line_contract.rs`
- Touched contract/docs files:
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/units/boundary-symbol-unit-registry.md`
  - `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
  - `docs/contracts/README.md`
  - `docs/contracts/openwepp-hillslope-runfile-contract.md`
  - `docs/contracts/openwepp-runner-contract.md`
  - this package directory and `docs/work-packages/README.md`.
- Out-of-scope protected files were not edited: WEPPpy, `/wc1` inputs, snow
  physics-magnitude code, and WBVAL05 closure files.

Ran:

- `git status --short` inspected before artifact updates.
