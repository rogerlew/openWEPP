# SIMIMPL25 Tier-A Rerun Evidence Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Intake/precondition authorization confirmed from:
  - `SIMIMPL24` disposition (`package-complete-with-hold`, hold retained only for SIMIMPL25 rerun/disposition wave).
  - `SIMIMPL24` worker handoff (explicit rerun target for Tier-A replay lanes).
  - `SIMIMPL20` queue ordering (`SIMIMPL25` final rerun/disposition step).
- Canonical authority remained unchanged and coherent for rerun closure:
  - `SC-WATBAL-001` (`INV-WATBAL-017`, `INV-WATBAL-026`, `INV-WATBAL-027`, `INV-WATBAL-029`),
  - `SC-SYSTEM-001` (`INV-SYSTEM-023`..`INV-SYSTEM-027`),
  - `SC-EVAP-001` (`INV-EVAP-011`..`INV-EVAP-013`),
  - `SC-SOIL-001` (`INV-SOIL-013`),
  - `SC-PLANT-001` WB11 stress/uptake coupling posture.
- Tier-A rerun surface family targeted per package scope:
  - `RM`, `Snow-Water`, `Total-Soil`, `SoilWaterTotal`,
  - `Ep`, `Es`, `Er`, `Q`, `Dp`, `latqcc`.

## Ran
- Tier-A lane rerun tests:
  - `cargo test -p openwepp --test pl14_tier_a_candidate_replay_contract`
    - result: pass (`7 passed; 0 failed`)
  - `cargo test -p openwepp --test pl14r_tier_a_replay_rerun_contract`
    - result: pass (`9 passed; 0 failed`)
  - `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract`
    - result: pass (`8 passed; 0 failed`)
  - `cargo test -p openwepp --test pl15_tier_a_delta_closeout_contract`
    - result: pass (`4 passed; 0 failed`)
  - `cargo test -p openwepp --test pl15r_tier_a_delta_recloseout_contract`
    - result: pass (`5 passed; 0 failed`)
- Supporting WB11/WB13 publication/routing closure checks:
  - `cargo test -p openwepp --test wb13_daily_water_balance_output_surface_contract`
    - result: pass (`3 passed; 0 failed`)
  - `cargo test -p openwepp --test wb11_hydrology_kernel_contract`
    - result: pass (`7 passed; 0 failed`)
  - `cargo test -p openwepp-runner --test simimpl04_wb13_publication_contract`
    - result: pass (`3 passed; 0 failed`)

## Rerun outcome summary
- PL14 posture: pass.
- PL14R posture: pass.
- PL14S posture: pass, including semantic/provenance guard assertions and SIMIMPL18 day-key snow/rain partition checks.
- PL15 posture: pass (closeout governance guard assertions).
- PL15R posture: pass (schema-aligned strict replay supersession assertions).
- No Tier-A replay blocker remained open in executed contract lanes.
