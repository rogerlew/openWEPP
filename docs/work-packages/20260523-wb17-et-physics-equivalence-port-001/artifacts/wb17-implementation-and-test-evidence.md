# WB17 Implementation And Test Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Implementation Summary
- Added WB17 ET runtime symbols to kernel-contract authority:
  - state: `Wb17ResidueInterception`
  - flux: `Wb17PlantTranspirationEp`, `Wb17SoilEvaporationEs`,
    `Wb17ResidueEvaporationEr`
- Replaced WB11 surrogate ET runtime path with equation-driven WB17 ET
  partition logic in `run_evapotranspiration`.
- Added explicit WB17 ET constant and symbol bindings:
  - `WB17_LAI_PARTITION_COEFFICIENT = 0.4`
  - `WB17_SYMBOL_RESIDUE_INTERCEPTION`, `WB17_SYMBOL_EP`,
    `WB17_SYMBOL_ES`, `WB17_SYMBOL_ER`
- Emitted partition component outputs (`Ep`, `Es`, `Er`) in kernel writeback.

## Contract-Derived Test Coverage
- Added `tests/integration/wb17_et_physics_kernel_contract.rs` with four
  vectors:
  1. nominal partition conformance,
  2. missing required symbol guard,
  3. non-finite guard,
  4. domain-invalid guard.
- Registered test in `Cargo.toml` (`[[test]] wb17_et_physics_kernel_contract`).
- Updated dependent integration fixtures to seed required WB17 ET input
  (`wb17_residue_interception`) so scheduler completion remains valid.

## Executed Commands
```bash
cargo test --test wb17_et_physics_kernel_contract
cargo test --test wb11_hydrology_kernel_contract --test wb12_reconciliation_kernel_contract --test wb14_infiltration_hyetograph_kernel_contract --test wb15_canopy_interception_kernel_contract --test wb16_peak_runoff_kernel_contract --test irrig10_irrigation_runtime_kernel_contract --test clim05_snow_runtime_kernel_contract --test clim06_frost_frozen_soil_kernel_contract
cargo test --test parser_runtime_seam_integration
cargo test --test arch22_typed_state_surface_contract
cargo test -p openwepp-hillslope-orchestrator
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

## Results
- WB17 target suite: pass (`4 passed`)
- WB11/WB12/WB14/WB15/WB16 + IRRIG10 + CLIM05/06 dependency suites: pass
- Parser runtime seam integration: pass (`45 passed`)
- Arch typed-surface seam suite: pass (`6 passed`)
- `openwepp-hillslope-orchestrator` package tests: pass (`51 passed`)
- `cargo fmt --check`: pass
- `cargo clippy --workspace --all-targets -- -D warnings`: pass
- `cargo test --workspace`: pass
- `cargo deny check`: pass with non-fatal `license-not-encountered`
  allowlist warnings
