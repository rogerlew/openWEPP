# Rejected Formulas And Aliases

Status: `binding for successor design`.

The following constructions are prohibited:

- `incoming_longwave = net_longwave`. Net longwave already subtracts outgoing
  snow emission.
- `subcanopy_longwave = sky_longwave + canopy_emission` over the full area.
  Canopy emission must replace the sky fraction obscured by canopy.
- `latent_energy = latent_flux + latent_heat(vapor_mass)` when both quantities
  describe the same turbulent transfer. That debits energy twice.
- `SWE_after = SWE_before - vapor_mass - liquid(vapor_mass)`. Vapor does not
  enter liquid routing.
- including latent heat in the surface balance while leaving the corresponding
  sublimation mass in the snow state.
- subtracting a Stage A/B empirical mass loss while independently computing a
  turbulent latent flux for the same surface and time step.
- combining watts per square meter with daily joules per square meter without
  multiplying flux by the exact step duration.
- inferring SWE loss from depth loss without observed or modeled density.
- treating air temperature as canopy radiometric temperature without admitted
  authority and an uncertainty statement.
- presenting the producer's internally calculated residual as independent
  conservation evidence.
- comparing cells with different phase, density, liquid, canopy, initial-state,
  or surface-energy selectors.

These exclusions prevent plausible-looking compensation between excess
longwave, excess sublimation, melt, and liquid routing.
