fn direct_publication_frost_comparator_surface_from_seed_surface(
    seed_surface: &HillslopeWritebackSurface,
) -> DirectFrostRunoffSurface {
    DirectFrostRunoffSurface::from_surface_maps(
        seed_surface.state_surface.clone(),
        seed_surface.flux_surface.clone(),
    )
}
