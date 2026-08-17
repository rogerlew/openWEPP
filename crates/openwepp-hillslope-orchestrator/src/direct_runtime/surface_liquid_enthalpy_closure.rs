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

#[must_use]
pub(super) const fn exact_q_match(actual: f64, expected: f64) -> bool {
    actual.to_bits() == expected.to_bits()
}

#[cfg(test)]
mod tests {
    use super::{allocate_ordered_child, split_first_then_remainder};

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
}
