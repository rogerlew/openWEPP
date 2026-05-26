# Worker Handoff

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- Completed in SIMIMPL34:
  - active-frost migration with temperature-sensitive freeze progression,
  - frwatc-style water-state handoff effect,
  - getFreezeCond-style land-use kfactor selection,
  - activation/pass of deferred SIMIMPL32 frost vectors.
- Required follow-on sequence:
  1. SIMIMPL35: winter-hourly frost parity rerun and explicit GO/HOLD
     hold-lift disposition.

## Ran
- `cargo test -p openwepp --test clim06_frost_frozen_soil_kernel_contract -- --nocapture`
- `cargo test --workspace`
