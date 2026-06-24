fn direct_publication_frost_comparator_surface_from_seed_surface(
    seed_surface: &HillslopeWritebackSurface,
) -> DirectFrostRunoffSurface {
    DirectFrostRunoffSurface::from_surface_maps(
        seed_surface.state_surface.clone(),
        seed_surface.flux_surface.clone(),
    )
}

fn direct_production_frost_comparator_surface_template(
    seed_surface: &HillslopeWritebackSurface,
) -> DirectFrostRunoffSurface {
    let mut state_surface = seed_surface.state_surface.clone();
    state_surface
        .retain(|symbol, _| direct_production_retains_frost_surface_symbol(symbol.as_str()));
    DirectFrostRunoffSurface::from_surface_maps(state_surface, std::collections::BTreeMap::new())
}
