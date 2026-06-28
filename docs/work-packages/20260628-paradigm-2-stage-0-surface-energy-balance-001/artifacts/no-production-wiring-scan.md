# No Production Wiring Scan

Evidence mode: `Static + Ran`

Stage 0 adds `openwepp_meteorology::surface_energy` but does not call it from
production runtime crates.

Scan command:

```bash
rg -n "surface_energy|net_all_wave_radiation|turbulent_fluxes_monin_obukhov|conductive_heat_flux|precipitation_advected_heat_flux" \
  crates/openwepp-hillslope-orchestrator/src \
  crates/openwepp-runner/src \
  crates/openwepp-watershed-orchestrator/src \
  crates/openwepp-climate-runtime-adapter/src \
  crates/openwepp-legacy-bridge/src
```

Result: No production runtime source references to Stage 0 surface-energy
symbols. The command returned no matches (`rg` exit code 1).

The existing `openwepp-meteorology` dependencies in `openwepp-runner` and
`openwepp-hillslope-orchestrator` predate this package and support the
Harder-Pomeroy phase work. This package does not add new runtime calls or
selectors.
