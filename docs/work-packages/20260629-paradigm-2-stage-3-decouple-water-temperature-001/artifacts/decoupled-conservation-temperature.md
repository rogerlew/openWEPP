# Decoupled Conservation and Temperature

Status: `RAN-PASS`

Evidence class: Ran + Static.

Focused integration tests prove that the decoupled arm preserves the bulk
aggregate snow state while carrying Stage 3 layer diagnostics:

```bash
cargo test --test paradigm2_stage3_decouple_water_temperature
cargo test --test paradigm2_stage3_liquid_routing_meltwater_temperature
```

Results:

- `paradigm2_stage3_decouple_water_temperature`: `3` passed.
- `paradigm2_stage3_liquid_routing_meltwater_temperature`: `6` passed.

Key assertions:

- `layered_thermal_liquid_v1` runs with
  `snow_density_model = physics_bulk_density_compaction_v1`.
- With no prior Stage 1 layers, Stage 3 synthesizes a layer stack and preserves
  the disabled bulk aggregate `runtime_swe_after_m`, `runtime_depth_after_m`,
  and `runtime_density_after_kg_m3`.
- The synthesized layer stack has non-empty layers, layer SWE/depth sums match
  the aggregate state within `1e-9 m`, and every layer density equals the
  aggregate bulk density within `1e-12 kg m^-3`.
- A prior multi-layer geometry with split densities is carried as geometry but
  the basal-minus-surface density gradient is reduced to `0`, proving the arm
  does not inherit Stage 1 local-overburden densification.
- Stage 3 diagnostics close liquid residual to `<=1e-9 m`, energy residual to
  `<=1e-6 J m^-2`, and produce a typed meltwater temperature bounded at
  `0 degC` for ripe routed liquid.
