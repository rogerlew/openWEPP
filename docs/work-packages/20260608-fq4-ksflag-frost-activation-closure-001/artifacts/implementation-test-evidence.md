# Implementation and Test Evidence

Status: complete

Evidence mode: Static + Ran.

## Implementation

- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - `resolve_active_frost_coupling` now validates
    `frost.options.frost_file_present` when present but treats it as provenance.
  - Activation is controlled by `frost.options.wintRed`; missing `wintRed`
    remains inactive/no-projection compatible, `wintRed=0` is explicit inactive,
    and `wintRed=1` activates the frozen-soil routine.
- `crates/openwepp-runner/src/hillslope/mod.rs`
  - Coupling-vector `frsoil.active` now reports `wintRed` activation rather
    than `frost_file_present && wintRed`.

## Focused Runs

Ran:

- `cargo fmt --check`: passed.
- `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture`:
  passed.
- `cargo test -p openwepp-runner simimpl04_contract_requires_wepp_ui_requested_effective_lane_closure_manifest -- --nocapture`:
  passed.
- `cargo test -p openwepp --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture`:
  passed.
- `cargo test -p openwepp --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture`:
  passed.

## Real-Run Spot Check

Ran: `/tmp/fq4_after` p8.

- `frsoil.active=true`
- `dfrost=0.2 m`
- `frozwt final=30.4 mm`
- WAT nonzero `frozwt` days: `1017`
- Hydout-equivalent `SoilWaterTotal - (Total-Soil + frozwt)=0`
