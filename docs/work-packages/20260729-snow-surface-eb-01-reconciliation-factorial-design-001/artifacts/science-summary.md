# Science Summary

The current model does not yet contain a complete snow surface-energy balance.
The production default has Stage 3 disabled. When its independent opt-in path
is selected, Stage 3 supplies absorbed shortwave to the surface sum while
explicit longwave, sensible heat, latent heat, surface-ground conduction, and
advected precipitation heat are zero there. Shared, typed arithmetic exists
for most of those fluxes, but arithmetic availability is not process authority
or runtime composition.

The prior sublimation candidates operate on a separate mass path. Stage A uses
a fixed `0 degC` snow surface; Stage B uses `min(air temperature, 0 degC)` and
an active layer no deeper than `0.25 m`. Both scale exchange by open fraction.
They remove bounded SWE as vapor, but do not debit the same latent energy from
the Stage 3 balance. Stage B passed its conservation guard and improved on
Stage A, yet remained nonpromoted at `15` robust failures / `178`, compared
with `15` / `179` for the current default.

This separation explains why longwave and sublimation cannot safely be added
as independent corrections. Longwave changes the energy available to warm and
melt snow. Sublimation removes mass and consumes latent energy. A valid
combined model must compute one vapor transfer, debit the corresponding latent
energy once, remove the corresponding mass once, and route none of that vapor
as liquid.

The installed observation base is useful but uneven. Marcell provides bound
conifer, deciduous, and open snow depth/SWE strata over a long
cold-continental record. Harvard provides bound hardwood and open observations;
its hemlock series is installed but lacks a pure-conifer fixture. Five SNOTEL
sites constrain open-site SWE, depth, density, and persistence across snow
climates. Sleepers can diagnose snow-insulation consequences for frost but
lacks paired SWE. A decisive warm-maritime conifer lane such as HJ Andrews is
not installed.

The resulting campaign decision is asymmetric. Energy-consistent sublimation
reconciliation is ready for contract-first work. Sub-canopy longwave is held
for authority because the model lacks an admitted sky-view/canopy-view
partition and canopy radiometric-temperature/emissivity rule. The combined
factorial waits on both prerequisites. This is a bounded hold, not permission
to substitute a convenient heuristic.
