# SIMIMPL32 Contract-Derived Test Matrix

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
| test id | location | contract authority | expected pre-migration posture |
|---|---|---|---|
| `simimpl32_contract_dispatch_trigger_vector_requires_active_frost_hourly_emission` | `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs` | `SC-SNOWFREEZE-001` SIMIMPL31 addendum dispatch-trigger closure (`INV-SNOWFREEZE-012`) | fail (missing `frost.hourly.*` active dispatch payload families) |
| `simimpl32_contract_handoff_direction_vector_requires_frozen_water_exchange_effect` | `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs` | `SC-SNOWFREEZE-001` SIMIMPL31 addendum `frwatc` ingress/egress handoff closure (`INV-SNOWFREEZE-012`) | fail (runtime seam does not yet expose full frwatc-style frozen-water exchange behavior) |
| `simimpl32_contract_freeze_lineage_vector_requires_temperature_sensitive_frost_progression` | `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs` | `SC-SNOWFREEZE-001` SIMIMPL31 addendum freeze-lineage closure (`frzng`/`frznw`) | fail (reductive frost coupling does not yet express temperature-sensitive lineage progression) |
| `simimpl32_contract_conductivity_lineage_vector_requires_land_use_dependent_kfactor_selection` | `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs` | `SC-SNOWFREEZE-001` SIMIMPL31 addendum conductivity-lineage closure (`frsoil`/`getFreezeCond`, `INV-SNOWFREEZE-013`) | fail (current coupling does not implement land-use-dependent `getFreezeCond` selection lineage) |
| `simimpl32_contract_cross_contract_seam_vector_requires_frost_hourly_payload_completeness` | `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs` | `SC-SNOWFREEZE-001` + cross-contract seam ownership (`SC-SOIL-001`, `SC-RUNOFFPART-001`, `SC-WATBAL-001`, `SC-SYSTEM-001`) | fail (required `frost.hourly.*` seam payload families are not yet emitted) |

## Notes
- All five SIMIMPL32 vectors are intentionally marked `#[ignore]` in default
  suite execution and are enabled only by explicit `--ignored` invocation while
  SIMIMPL33/SIMIMPL34 runtime migration remains open.

## Ran
- `cargo test -p openwepp --test clim06_frost_frozen_soil_kernel_contract --no-run`
- `cargo test -p openwepp --test clim06_frost_frozen_soil_kernel_contract`
- `cargo test -p openwepp --test clim06_frost_frozen_soil_kernel_contract -- --ignored --nocapture`
