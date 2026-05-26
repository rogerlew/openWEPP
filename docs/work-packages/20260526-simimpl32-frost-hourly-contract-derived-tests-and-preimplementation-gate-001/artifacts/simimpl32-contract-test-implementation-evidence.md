# SIMIMPL32 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- Implemented five SIMIMPL32 contract-derived vectors in
  `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`:
  - `simimpl32_contract_dispatch_trigger_vector_requires_active_frost_hourly_emission`
  - `simimpl32_contract_handoff_direction_vector_requires_frozen_water_exchange_effect`
  - `simimpl32_contract_freeze_lineage_vector_requires_temperature_sensitive_frost_progression`
  - `simimpl32_contract_conductivity_lineage_vector_requires_land_use_dependent_kfactor_selection`
  - `simimpl32_contract_cross_contract_seam_vector_requires_frost_hourly_payload_completeness`
- Vectors are intentionally `#[ignore]` with explicit migration-block reason
  strings tied to SIMIMPL33/SIMIMPL34 closure.
- Added SIMIMPL32 test helpers for typed execution/readback surface checks:
  - `execute_clim06_surface(...)`
  - `require_state_scalar(...)`

## Ran
- `cargo test -p openwepp --test clim06_frost_frozen_soil_kernel_contract --no-run`
- `cargo test -p openwepp --test clim06_frost_frozen_soil_kernel_contract`
- `cargo test -p openwepp --test clim06_frost_frozen_soil_kernel_contract -- --ignored --nocapture`
