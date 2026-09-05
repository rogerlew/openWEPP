pub(super) fn effective_retained_mass(
    raw_retained: f64,
    capacity_ofe: f64,
    stored_ofe: f64,
    total_excess: f64,
) -> Option<f64> {
    if !raw_retained.is_finite()
        || raw_retained < 0.0
        || !capacity_ofe.is_finite()
        || capacity_ofe < 0.0
        || !stored_ofe.is_finite()
        || stored_ofe < 0.0
        || !total_excess.is_finite()
        || total_excess < 0.0
    {
        return None;
    }
    let scale = checked_surface_liquid_add(capacity_ofe.abs(), stored_ofe.abs())
        .and_then(|partial| checked_surface_liquid_add(partial, total_excess.abs()))?;
    let envelope = mass_tolerance(scale);
    envelope
        .is_finite()
        .then_some(if raw_retained > 0.0 && raw_retained <= envelope {
            0.0
        } else {
            raw_retained
        })
}

fn retained_tile_credit_and_ending_store(
    beginning_tile: f64,
    capacity_tile: f64,
    tile_fraction: f64,
    available_tile: f64,
    available_ofe: f64,
    retained_ofe: f64,
) -> Option<(f64, f64)> {
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
        || checked_surface_liquid_sub(capacity_tile, beginning_tile)?.to_bits()
            != available_tile.to_bits()
        || checked_surface_liquid_mul(tile_fraction, available_tile)?.to_bits()
            != available_ofe.to_bits()
    {
        return None;
    }

    if retained_ofe.to_bits() == available_ofe.to_bits() {
        // Exact full-capacity authority is already reconstructed in tile and
        // OFE bases above. Re-dividing the OFE amount can round one ULP beyond
        // capacity for a non-binary tile fraction, so the owner installs the
        // exact sealed tile remainder and its exact capacity endpoint.
        return Some((available_tile, capacity_tile));
    }

    let retained_tile = checked_surface_liquid_div(retained_ofe, tile_fraction)?;
    let ending_tile = checked_surface_liquid_add(beginning_tile, retained_tile)?;
    (ending_tile <= capacity_tile).then_some((retained_tile, ending_tile))
}

pub(super) fn allocate_retained_mass(
    total_retained: f64,
    total_excess: f64,
    allocated_retained: f64,
    excess: f64,
    is_last: bool,
) -> Option<f64> {
    if !total_retained.is_finite()
        || total_retained < 0.0
        || !total_excess.is_finite()
        || total_excess < 0.0
        || !allocated_retained.is_finite()
        || allocated_retained < 0.0
        || !excess.is_finite()
        || excess < 0.0
        || total_retained > total_excess
    {
        return None;
    }
    let retained = if total_retained.to_bits() == total_excess.to_bits() {
        excess
    } else if total_retained == 0.0 {
        0.0
    } else if is_last {
        checked_surface_liquid_sub(total_retained, allocated_retained)?
    } else {
        checked_surface_liquid_mul(total_retained, excess)
            .and_then(|numerator| checked_surface_liquid_div(numerator, total_excess))?
    };
    (retained.is_finite() && retained >= 0.0 && retained <= excess).then_some(retained)
}

fn direct_infiltration_requires_excess_authority(
    total_infiltration: f64,
    supply_mass: f64,
    masses: impl IntoIterator<Item = f64>,
) -> Option<bool> {
    if !total_infiltration.is_finite()
        || total_infiltration < 0.0
        || !supply_mass.is_finite()
        || supply_mass <= 0.0
        || total_infiltration > supply_mass
    {
        return None;
    }
    let masses = masses.into_iter().collect::<Vec<_>>();
    let mut allocated = 0.0;
    let count = masses.len();
    for (index, mass) in masses.into_iter().enumerate() {
        if !mass.is_finite() || mass < 0.0 {
            return None;
        }
        let infiltrated = if total_infiltration.to_bits() == supply_mass.to_bits() {
            mass
        } else if index + 1 == count {
            checked_surface_liquid_sub(total_infiltration, allocated)?
        } else {
            checked_surface_liquid_mul(total_infiltration, mass)
                .and_then(|numerator| checked_surface_liquid_div(numerator, supply_mass))?
        };
        if infiltrated < 0.0 || infiltrated > mass {
            return Some(true);
        }
        allocated = checked_surface_liquid_add(allocated, infiltrated)?;
    }
    Some(false)
}

fn allocate_infiltration_and_excess(
    total_infiltration: f64,
    supply_mass: f64,
    allocated_infiltration: f64,
    allocated_excess: f64,
    mass: f64,
    is_last: bool,
    use_excess_authority: bool,
) -> Option<(f64, f64)> {
    if !total_infiltration.is_finite()
        || total_infiltration < 0.0
        || !supply_mass.is_finite()
        || supply_mass <= 0.0
        || total_infiltration > supply_mass
        || !allocated_infiltration.is_finite()
        || allocated_infiltration < 0.0
        || !allocated_excess.is_finite()
        || allocated_excess < 0.0
        || !mass.is_finite()
        || mass < 0.0
    {
        return None;
    }
    let total_excess = checked_surface_liquid_sub(supply_mass, total_infiltration)?;
    let (infiltrated, excess) = if total_infiltration.to_bits() == supply_mass.to_bits() {
        (mass, 0.0)
    } else if use_excess_authority {
        let excess = if is_last {
            checked_surface_liquid_sub(total_excess, allocated_excess)?
        } else {
            checked_surface_liquid_mul(total_excess, mass)
                .and_then(|numerator| checked_surface_liquid_div(numerator, supply_mass))?
        };
        (checked_surface_liquid_sub(mass, excess)?, excess)
    } else {
        let infiltrated = if is_last {
            checked_surface_liquid_sub(total_infiltration, allocated_infiltration)?
        } else {
            checked_surface_liquid_mul(total_infiltration, mass)
                .and_then(|numerator| checked_surface_liquid_div(numerator, supply_mass))?
        };
        (infiltrated, checked_surface_liquid_sub(mass, infiltrated)?)
    };
    (infiltrated.is_finite()
        && infiltrated >= 0.0
        && infiltrated <= mass
        && excess.is_finite()
        && excess >= 0.0
        && excess <= mass)
        .then_some((infiltrated, excess))
}
