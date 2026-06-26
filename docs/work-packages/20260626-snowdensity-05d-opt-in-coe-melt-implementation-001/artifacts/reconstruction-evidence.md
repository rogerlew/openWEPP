# Reconstruction Evidence

Evidence class: Ran.

Primary test: `tests/integration/snowdensity05d_opt_in_coe_melt.rs`.

## Raw Melt

The focused fixture isolates `amelt` by using one warm radiation hour with:

- hourly radiation = `10.0` MJ m-2
- canopy cover = `0.0`
- cloud fraction = `1.0`
- wind = `0.0`
- dew point = `0.0`
- rain = `0.0`

Legacy raw melt reconstructs as:

```text
0.0607 * 10.0 * 0.0254
```

Opt-in raw melt reconstructs independently as:

```text
0.0607 * 10.0 * (1 - hour_one_snow_albedo) * 0.0254
```

The expected opt-in albedo is reconstructed by calling the typed 05C albedo
update for the first melt hour, not by reading the final state back from the
result.

## Redistribution And Routing

The same test checks:

- `raw_melt_m` equals the independent raw-melt reconstruction.
- `redistributed_melt_m` equals the positive raw-melt total after existing
  daily redistribution rules.
- `routed_snow_melt_m` equals the routed `wmelt` publication surface.
- `snowpack_swe_loss_m` equals the bounded snowpack SWE loss.
- WB12 `S` equals routed melt in the isolated no-rain/no-runoff fixture.

## Fail-Closed State

The missing-state test removes the active opt-in albedo state and verifies that
the typed path errors instead of silently using `legacy_coe`.

## Direct Runtime Carry

The direct runtime test verifies that `run_r4g_snow_coupling_span()` projects
the routed melt, raw/redistributed/SWE lineage, and albedo carry into typed
direct runtime storage and downstream operands.
