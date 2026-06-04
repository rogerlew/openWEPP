# Contract Test Implementation Evidence

Status: completed
Evidence mode: mixed static-and-ran

Static: contract-derived runtime tests were added for finite physically
impossible hourly radiation.

Ran: targeted red/green and regression tests were executed locally.

## Test Added

Static: `climate_runtime_surface_with_context_rejects_physically_impossible_hourly_radiation`
constructs a finite overlarge daily radiation forcing and requires
`ClimateRuntimeInputError::RuntimeContextSymbolOutOfRange` on a
`winter.hourly.rad_mj_m2_####` symbol with allowed text that cites the physical
hourly extraterrestrial bound.

## Red Gate

Ran: before production guard implementation:

`cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_rejects_physically_impossible_hourly_radiation --lib -- --nocapture`

Result: failed as expected. The runtime accepted/published finite hourly
radiation as high as `38.289375767701195 MJ m^-2 h^-1` instead of failing
closed.

## Green Gate

Ran: after production guard implementation:

`cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_rejects_physically_impossible_hourly_radiation --lib -- --nocapture`

Result: passed. The impossible finite radiation path now fails with typed
`RuntimeContextSymbolOutOfRange` evidence.

## Regression Coverage

Ran:

- `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_uses_single_radly_to_radmj_conversion --lib -- --nocapture`
- `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_near_isothermal_radiation_is_radmj_over_24 --lib -- --nocapture`
- `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context --lib`
- `cargo test -p openwepp-hillslope-orchestrator runtime_inputs`

Result: all passed.
