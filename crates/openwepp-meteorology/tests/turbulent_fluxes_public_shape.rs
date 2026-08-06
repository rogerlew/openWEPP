use openwepp_meteorology::surface_energy::{
    EnergyFluxWattsPerSquareMeter, MassFluxKilogramsPerSquareMeterSecond, TurbulentFluxes,
};

#[allow(dead_code)]
fn external_turbulent_fluxes_literal_remains_exhaustive(
    sensible_heat: EnergyFluxWattsPerSquareMeter,
    latent_heat: EnergyFluxWattsPerSquareMeter,
    mass_flux: MassFluxKilogramsPerSquareMeterSecond,
) -> TurbulentFluxes {
    TurbulentFluxes {
        sensible_heat,
        latent_heat,
        mass_flux,
        iterations: 0,
        obukhov_length_m: None,
    }
}

#[test]
fn public_shape_compile_guard_is_linked() {
    assert!(std::mem::size_of::<TurbulentFluxes>() > 0);
}
