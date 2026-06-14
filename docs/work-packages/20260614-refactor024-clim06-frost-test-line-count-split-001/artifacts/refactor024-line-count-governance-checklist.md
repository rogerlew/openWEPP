# REFACTOR024 Line-Count Governance Checklist

Evidence class: Static

Pre-refactor:

- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`: 2743 lines
  (WARN, above 2000; below 3000 mandatory split threshold).

Post-refactor:

- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`: 11 lines.
- `tests/integration/clim06_frost_frozen_soil_kernel_contract/contract_gates.rs`:
  373 lines.
- `tests/integration/clim06_frost_frozen_soil_kernel_contract/fine_layer.rs`:
  557 lines.
- `tests/integration/clim06_frost_frozen_soil_kernel_contract/publication.rs`:
  359 lines.
- `tests/integration/clim06_frost_frozen_soil_kernel_contract/support.rs`:
  990 lines.
- `tests/integration/clim06_frost_frozen_soil_kernel_contract/thermal_front.rs`:
  484 lines.

Result:

- No touched `.rs` file is above the 2000-line WARN threshold.
- No touched `.rs` file is above the 3000-line refactor-required threshold.
