# SIMIMPL32 Pre-Migration Failure Baseline

Status: complete
Evidence mode: ran
Date: 2026-05-26

## Static
- Objective: confirm SIMIMPL32 contract-derived frost vectors fail on the
  current reductive frost runtime path before SIMIMPL33/SIMIMPL34 production
  migration begins.

## Ran
- Command:
  - `cargo test -p openwepp --test clim06_frost_frozen_soil_kernel_contract -- --ignored --nocapture`
- Result: failed as expected (`0 passed; 5 failed`).
- Observed blocking failures:
  - `simimpl32_contract_dispatch_trigger_vector_requires_active_frost_hourly_emission`
    - panic: `missing expected state symbol frost.hourly.qsrf_w_m2_0001`
  - `simimpl32_contract_handoff_direction_vector_requires_frozen_water_exchange_effect`
    - panic: `frwatc-style ingress/egress handoff should reduce liquid wb11 soil-water under active frost`
  - `simimpl32_contract_freeze_lineage_vector_requires_temperature_sensitive_frost_progression`
    - panic: `freeze-lineage closure requires stronger cold forcing to deepen frost front`
  - `simimpl32_contract_conductivity_lineage_vector_requires_land_use_dependent_kfactor_selection`
    - panic: `getFreezeCond lineage closure requires land-use-dependent conductivity divergence when kfactor set differs by class`
  - `simimpl32_contract_cross_contract_seam_vector_requires_frost_hourly_payload_completeness`
    - panic: `missing expected state symbol frost.hourly.qsrf_w_m2_0001`
- Control run:
  - `cargo test -p openwepp --test clim06_frost_frozen_soil_kernel_contract`
  - Result: pass (`4 passed; 0 failed; 5 ignored`).
