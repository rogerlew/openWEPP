fn independently_project_effective_retained_mass(
    raw_retained: f64,
    capacity_ofe: f64,
    stored_ofe: f64,
    total_excess: f64,
) -> Option<f64> {
    if !raw_retained.is_finite()
        || !capacity_ofe.is_finite()
        || capacity_ofe < 0.0
        || !stored_ofe.is_finite()
        || stored_ofe < 0.0
        || !total_excess.is_finite()
        || total_excess < 0.0
    {
        return None;
    }
    // A finite negative raw retention can only arise from a separately
    // invalid frozen store reconstruction (for example, a substituted W0
    // above capacity). Preserve that finite value during the exhaustive
    // arithmetic scan so the store equation retains its canonical E010
    // classification; production ingress rejects negative capacity before
    // this independent projection is reached.
    if raw_retained < 0.0 {
        return Some(raw_retained);
    }
    let scale = checked_surface_liquid_add(capacity_ofe.abs(), stored_ofe.abs())
        .and_then(|partial| checked_surface_liquid_add(partial, total_excess.abs()))?;
    let roundoff = checked_surface_liquid_mul(
        REPRESENTATIONAL_CREDIT_EPSILON_MULTIPLIER * f64::EPSILON,
        scale,
    )?;
    let envelope = checked_surface_liquid_add(REPRESENTATIONAL_CREDIT_ABSOLUTE_KG_M2, roundoff)?;
    Some(if raw_retained > 0.0 && raw_retained <= envelope {
        0.0
    } else {
        raw_retained
    })
}

fn independently_project_ending_store(
    beginning_tile: f64,
    capacity_tile: f64,
    tile_fraction: f64,
    available_tile: f64,
    available_ofe: f64,
    retained_ofe: f64,
) -> Option<f64> {
    if !beginning_tile.is_finite()
        || beginning_tile < 0.0
        || !capacity_tile.is_finite()
        || capacity_tile < beginning_tile
        || !tile_fraction.is_finite()
        || tile_fraction <= 0.0
        || !available_tile.is_finite()
        || available_tile < 0.0
        || !available_ofe.is_finite()
        || available_ofe < 0.0
        || !retained_ofe.is_finite()
        || retained_ofe < 0.0
        || retained_ofe > available_ofe
    {
        return None;
    }
    let reconstructed_tile = checked_surface_liquid_sub(capacity_tile, beginning_tile)?;
    let reconstructed_ofe = checked_surface_liquid_mul(tile_fraction, reconstructed_tile)?;
    if reconstructed_tile.to_bits() != available_tile.to_bits()
        || reconstructed_ofe.to_bits() != available_ofe.to_bits()
    {
        return None;
    }
    if retained_ofe.to_bits() == available_ofe.to_bits() {
        return Some(capacity_tile);
    }
    let retained_tile = checked_surface_liquid_div(retained_ofe, tile_fraction)?;
    let ending_tile = checked_surface_liquid_add(beginning_tile, retained_tile)?;
    (ending_tile <= capacity_tile).then_some(ending_tile)
}
