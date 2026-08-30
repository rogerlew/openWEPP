//! Exact enthalpy-child allocation for independent surface-liquid closure.

use super::{
    checked_surface_liquid_add, checked_surface_liquid_div, checked_surface_liquid_mul,
    checked_surface_liquid_sub,
};

pub(super) fn proportional_q(parent_q: f64, child: f64, parent: f64) -> Option<f64> {
    checked_surface_liquid_mul(parent_q, child)
        .and_then(|numerator| checked_surface_liquid_div(numerator, parent))
}

/// Allocate one ordered child and carry the exact parent remainder to the canonical last child.
pub(super) fn allocate_ordered_child(
    parent_q: f64,
    allocated_q: f64,
    specified_nonfinal_q: Option<f64>,
    is_last: bool,
) -> Option<(f64, f64)> {
    let child_q = if is_last {
        checked_surface_liquid_sub(parent_q, allocated_q)?
    } else {
        specified_nonfinal_q?
    };
    let next_allocated_q = checked_surface_liquid_add(allocated_q, child_q)?;
    Some((child_q, next_allocated_q))
}

/// Split a parent into an ordered first child and an exact-remainder second child.
pub(super) fn split_first_then_remainder(
    parent_q: f64,
    parent_mass: f64,
    first_mass: f64,
    second_mass: f64,
) -> Option<(f64, f64)> {
    let first_q = if second_mass == 0.0 {
        parent_q
    } else if first_mass == 0.0 {
        0.0
    } else {
        proportional_q(parent_q, first_mass, parent_mass)?
    };
    let second_q = checked_surface_liquid_sub(parent_q, first_q)?;
    Some((first_q, second_q))
}

/// Allocate retained mass for one canonical source part.
///
/// Exact full retention carries the source operand directly. Recomputing the
/// same value through `(total_excess * excess) / total_excess` can round one
/// ULP above `excess` and manufacture a negative runoff remainder.
pub(super) fn allocate_retained_mass(
    total_retained: f64,
    total_excess: f64,
    allocated_retained: f64,
    excess: f64,
    is_last: bool,
) -> Option<f64> {
    if !total_retained.is_finite()
        || !total_excess.is_finite()
        || total_excess < 0.0
        || !allocated_retained.is_finite()
        || allocated_retained < 0.0
        || !excess.is_finite()
        || excess < 0.0
        || (total_retained >= 0.0 && total_retained > total_excess)
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
        proportional_q(total_retained, excess, total_excess)?
    };
    if total_retained < 0.0 {
        retained.is_finite().then_some(retained)
    } else {
        (retained.is_finite() && retained >= 0.0 && retained <= excess).then_some(retained)
    }
}

pub(super) fn direct_infiltration_requires_excess_authority(
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
            proportional_q(total_infiltration, mass, supply_mass)?
        };
        if infiltrated < 0.0 || infiltrated > mass {
            return Some(true);
        }
        allocated = checked_surface_liquid_add(allocated, infiltrated)?;
    }
    Some(false)
}

/// Partition one source mass under the preselected exact aggregate authority.
pub(super) fn allocate_infiltration_and_excess(
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
            proportional_q(total_excess, mass, supply_mass)?
        };
        (checked_surface_liquid_sub(mass, excess)?, excess)
    } else {
        let infiltrated = if is_last {
            checked_surface_liquid_sub(total_infiltration, allocated_infiltration)?
        } else {
            proportional_q(total_infiltration, mass, supply_mass)?
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

#[must_use]
pub(super) const fn exact_q_match(actual: f64, expected: f64) -> bool {
    actual.to_bits() == expected.to_bits()
}

#[cfg(test)]
mod tests {
    use super::{
        allocate_infiltration_and_excess, allocate_ordered_child, allocate_retained_mass,
        direct_infiltration_requires_excess_authority, split_first_then_remainder,
    };

    #[test]
    fn canonical_last_ordered_child_reconstructs_parent_q_bit_exactly() {
        let parent_q = 0.3 * 4_186.0 * (285.0 - 273.15);
        let first_q = parent_q * 600.0 / 1_800.0;
        let (first, allocated) = allocate_ordered_child(parent_q, 0.0, Some(first_q), false)
            .expect("finite temporal/source nonfinal allocation");
        let (last, _) = allocate_ordered_child(parent_q, allocated, None, true)
            .expect("finite temporal/source canonical-last allocation");

        assert_eq!((first + last).to_bits(), parent_q.to_bits());
    }

    #[test]
    fn binary_split_children_reconstruct_each_parent_q_bit_exactly() {
        let parent_q = 0.3 * 4_186.0 * (285.0 - 273.15);
        let (infiltration_q, excess_q) = split_first_then_remainder(parent_q, 0.3, 0.03, 0.27)
            .expect("finite infiltration/excess allocation");
        assert_eq!((infiltration_q + excess_q).to_bits(), parent_q.to_bits());

        let (retained_q, runoff_q) = split_first_then_remainder(excess_q, 0.27, 0.1, 0.17)
            .expect("finite retention/runoff allocation");
        assert_eq!((retained_q + runoff_q).to_bits(), excess_q.to_bits());
    }

    #[test]
    fn exact_full_retention_carries_child_mass_and_enthalpy_without_negative_runoff() {
        let first_excess = f64::from_bits(0x3e2f_4a5e_f546_6a45);
        let last_excess = f64::from_bits(0x3ee1_ccbc_d840_e06c);
        let total_excess = first_excess + last_excess;
        let rounded_first = total_excess * first_excess / total_excess;
        assert_eq!(rounded_first.to_bits(), first_excess.to_bits() + 1);
        assert!((first_excess - rounded_first).is_sign_negative());

        let retained_first =
            allocate_retained_mass(total_excess, total_excess, 0.0, first_excess, false)
                .expect("exact full-retention first child");
        let retained_last = allocate_retained_mass(
            total_excess,
            total_excess,
            retained_first,
            last_excess,
            true,
        )
        .expect("exact full-retention last child");
        assert_eq!(retained_first.to_bits(), first_excess.to_bits());
        assert_eq!(retained_last.to_bits(), last_excess.to_bits());
        assert_eq!(
            (retained_first + retained_last).to_bits(),
            total_excess.to_bits()
        );

        let excess_q = f64::from_bits(0x40d2_3456_789a_bcde);
        let (retained_q, runoff_q) = split_first_then_remainder(
            excess_q,
            first_excess,
            retained_first,
            first_excess - retained_first,
        )
        .expect("exact retained/runoff enthalpy split");
        assert_eq!(retained_q.to_bits(), excess_q.to_bits());
        assert_eq!(runoff_q.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn retained_mass_allocation_rejects_poisoned_or_nonphysical_operands() {
        assert_eq!(allocate_retained_mass(f64::NAN, 1.0, 0.0, 0.5, false), None);
        assert_eq!(allocate_retained_mass(1.0, 0.5, 0.0, 0.5, false), None);
        assert_eq!(allocate_retained_mass(0.5, 1.0, 0.6, 0.4, true), None);
        assert_eq!(allocate_retained_mass(0.5, 1.0, 0.0, -0.1, false), None);
    }

    #[test]
    fn near_full_infiltration_allocates_exact_nonnegative_excess_complement() {
        let first_mass = f64::from_bits(0x3f6f_b77a_3a86_d4be);
        let last_mass = f64::from_bits(0x3f41_ff59_f1bd_dca4);
        let supply_mass = f64::from_bits(0x3f72_1ba8_5b7b_25f4);
        let total_infiltration = f64::from_bits(0x3f72_1ba8_5b7b_25f3);
        let legacy_first = total_infiltration * first_mass / supply_mass;
        let legacy_last = total_infiltration - legacy_first;
        assert_eq!((last_mass - legacy_last).to_bits(), 0xbc30_0000_0000_0000);
        let use_excess_authority = direct_infiltration_requires_excess_authority(
            total_infiltration,
            supply_mass,
            [first_mass, last_mass],
        )
        .expect("finite direct allocation preflight");
        assert!(use_excess_authority);

        let (first_infiltration, first_excess) = allocate_infiltration_and_excess(
            total_infiltration,
            supply_mass,
            0.0,
            0.0,
            first_mass,
            false,
            use_excess_authority,
        )
        .expect("first exact complement partition");
        let (last_infiltration, last_excess) = allocate_infiltration_and_excess(
            total_infiltration,
            supply_mass,
            first_infiltration,
            first_excess,
            last_mass,
            true,
            use_excess_authority,
        )
        .expect("last exact complement partition");
        assert!(first_excess >= 0.0);
        assert!(last_excess >= 0.0);
        assert_eq!(
            (first_excess + last_excess).to_bits(),
            0x3c30_0000_0000_0000
        );
        assert!(first_infiltration <= first_mass);
        assert!(last_infiltration <= last_mass);
    }

    #[test]
    fn infiltration_partition_rejects_poisoned_and_overdrawn_operands() {
        assert_eq!(
            allocate_infiltration_and_excess(f64::NAN, 1.0, 0.0, 0.0, 0.5, false, true),
            None
        );
        assert_eq!(
            allocate_infiltration_and_excess(1.0, 0.5, 0.0, 0.0, 0.5, false, true),
            None
        );
        assert_eq!(
            allocate_infiltration_and_excess(0.5, 1.0, 0.6, 0.0, 0.4, true, false),
            None
        );
        assert_eq!(
            allocate_infiltration_and_excess(0.5, 1.0, 0.0, 0.0, -0.1, false, false),
            None
        );
    }
}
