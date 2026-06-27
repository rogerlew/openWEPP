# Implementation Test Evidence

Status: complete
Evidence mode: Static + Ran

Record focused build/test evidence for `crates/openwepp-meteorology`.

## Public API Summary

- Static: `crates/openwepp-meteorology/src/lib.rs` exposes `error`, `phase`,
  and `psychrometrics` modules.
- Static: `psychrometrics` exposes typed wrappers for vapor pressure, vapor
  density, latent heat, diffusivity, and conductivity plus checked helpers for
  saturation vapor pressure, actual vapor pressure, dewpoint/RH conversion,
  ideal-gas vapor density, latent heat, diffusivity, and conductivity.
- Static: `phase` exposes `PhaseTimescale`, `HydrometeorSolverOptions`,
  `HydrometeorTemperatureSolution`, `PrecipitationPhaseFractions`,
  `PrecipitationPhaseEstimate`,
  `hydrometeor_temperature_from_relative_humidity`,
  `hydrometeor_temperature_from_relative_humidity_with_options`,
  `rainfall_fraction_for_hydrometeor_temperature`, and
  `harder_pomeroy_phase_from_relative_humidity`.
- Static: `MeteorologyError` carries `Boundary`, `BelowAbsoluteZero`,
  `NonPositive`, `InvalidSolverOptions`, and `SolverDidNotConverge` variants.

## Focused Runs

- Ran: `cargo test -p openwepp-meteorology`
- Ran result: PASS, `9 passed; 0 failed`; doctests `0 passed; 0 failed`.
- Ran: `cargo clippy -p openwepp-meteorology --all-targets -- -D warnings`
- Ran result: PASS.
- Ran: `cargo test --test snowdensity10_3_5a_meteorology_crate_contract`
- Ran result: PASS, `2 passed; 0 failed`.

## Test Coverage

- Ran: saturation vapor pressure over water and ice reference values.
- Ran: dewpoint/RH round trip.
- Ran: latent heat, diffusivity, and conductivity reference values.
- Ran: absolute-zero typed-domain rejection.
- Ran: saturated-air hydrometeor identity.
- Ran: unsaturated hydrometeor-temperature reference vectors.
- Ran: rain-fraction monotonicity and rain+snow fraction closure for all three
  Harder-Pomeroy coefficient sets.
- Ran: hourly rain-fraction reference values.
- Ran: explicit non-convergence error path.
