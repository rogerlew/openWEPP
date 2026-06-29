# Stage 3 Conservation and Temperature Evidence

Evidence class: Ran + Static.

Implemented opt-in selector:
`OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1`.

Focused gate command:

```bash
cargo test --test paradigm2_stage3_liquid_routing_meltwater_temperature
```

Result after deferred-gate follow-up: passed, `6` tests.

Covered checks:

- Disabled default returns `DirectSnowStage3Diagnostics::disabled()` and leaves
  the existing CoE routed-melt path active.
- Opt-in Stage 3 requires `physics_bulk_multilayer_density_v1` and fails closed
  with `snow.stage3_requires_multilayer_density_model` otherwise.
- Positive rain/melt liquid routes through the Stage 1 layer stack.
- Liquid ledger closes with `abs(liquid_closure_residual_m) <= 1.0e-9`.
- Energy ledger closes with `abs(energy_closure_residual_j_m2) <= 1.0e-6`.
- Refreeze and retained liquid are both positive in the cold-layer fixture.
- Meltwater flux temperature is typed as `TemperatureCelsius` and equals ripe
  melt (`0 degC`) within tolerance.
- Layer temperatures remain `<= 0 degC`.
- Persisted cold-content carry is capped to the physically representable range
  for the current layer mass before reconstructing layer temperature. This fixes
  the first deferred observed-gate failure, where stale cold content after layer
  evolution could imply a layer temperature below absolute zero and fail the
  Stage 0 conductive-flux domain guard.

Thermal-solve scope:

- Surface energy uses Stage 0 `net_shortwave_radiation` and
  `surface_energy_balance` primitives hour by hour.
- Adjacent layer conduction uses Stage 0 `conductive_heat_flux` and updates the
  per-layer cold-content vector conservatively.
- Internal conduction has zero net pack energy by construction; it changes layer
  cold content/temperature without changing aggregate CoE melt mass.
- Refreeze uses latent heat of fusion in the diagnostic energy ledger.

De-risking boundary:

- The existing CoE melt/rain mass path remains authoritative for aggregate SWE,
  `routed_melt_m`, downstream liquid forcing, and public water-balance surfaces.
- Stage 3 routes the already-computed CoE liquid diagnostically to produce
  retained/refrozen/routed liquid partitions and a typed meltwater-temperature
  source for the future stream-temperature program.
