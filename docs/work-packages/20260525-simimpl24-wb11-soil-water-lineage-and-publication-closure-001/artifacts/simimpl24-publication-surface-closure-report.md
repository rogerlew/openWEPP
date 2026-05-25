# SIMIMPL24 Publication Surface Closure Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- WB13 publication builder now enforces runtime-owned WB11/WB12/WB17/WB18/WB19
  lineage without surrogate fallbacks:
  - `Total-Soil` is sourced from `wb11_soil_water` only
    (`Total-Soil = wb11_soil_water * 1000`).
  - `Snow-Water` is sourced from runtime `snow.runtime_swe` (no synthetic
    reconstruction helper).
  - `RM` remains contract-consistent from runtime forcing state:
    `prcp + SWE_before - SWE_after + Irr`.
  - `Q`, `Ep`, `Es`, `Er`, `Dp`, and `latqcc` now require runtime kernel
    symbols (`Q`, `Ep`, `Es`, `Er`, `D`, `q`) and publish as mm.
- Runner WB11 seeding now ensures required WB16 symbols exist in execution
  surfaces (`efflen`, `ealpha`, `m`) when not projected by runtime adapters,
  preventing missing-symbol failure in low/zero-intensity days.

## Ran
- `cargo test -p openwepp-runner --test simimpl04_wb13_publication_contract`
- `cargo test -p openwepp --test wb13_daily_water_balance_output_surface_contract`
- `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract`
