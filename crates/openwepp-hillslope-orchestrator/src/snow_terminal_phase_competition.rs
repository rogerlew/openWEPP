//! Test-only research models for terminal vapor/melt phase competition.
//!
//! These allocators do not alter the released Stage-3 transition. They expose
//! exact support-local identities so competing phase chronologies can be
//! falsified before any successor contract or production implementation.

const LATENT_HEAT_FUSION_J_KG: f64 = 333_600.0;
const MASS_TOLERANCE_KG_M2: f64 = 1.0e-9;
const ENERGY_TOLERANCE_J_M2: f64 = 1.0e-6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TerminalPhaseModel {
    SimultaneousComplementarity,
    ResidualSurfaceFrost,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TerminalEventChronology {
    None,
    AtStart,
    Interior,
    AtEnd,
    Reappeared,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TerminalPhaseInputs {
    pub beginning_pack_ice_kg_m2: f64,
    pub beginning_surface_frost_kg_m2: f64,
    pub beginning_liquid_kg_m2: f64,
    pub beginning_cold_content_j_m2: f64,
    pub deposition_kg_m2: f64,
    pub sublimation_kg_m2: f64,
    pub external_liquid_kg_m2: f64,
    pub non_vapor_energy_j_m2: f64,
    pub vapor_latent_energy_j_m2: f64,
    pub complete_energy_j_m2: f64,
    pub support_seconds: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TerminalPhaseCandidate {
    pub model: TerminalPhaseModel,
    pub event: TerminalEventChronology,
    pub ending_pack_ice_kg_m2: f64,
    pub ending_surface_frost_kg_m2: f64,
    pub ending_liquid_kg_m2: f64,
    pub ending_cold_content_j_m2: f64,
    pub melt_kg_m2: f64,
    pub refrozen_kg_m2: f64,
    pub unallocated_energy_j_m2: f64,
    pub solid_closure_residual_kg_m2: f64,
    pub liquid_closure_residual_kg_m2: f64,
    pub energy_closure_residual_j_m2: f64,
    pub vapor_energy_custody_residual_j_m2: f64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TerminalPhaseFailure {
    DomainOrNonFinite,
    SublimationExceedsBeginningSolid,
    SurfaceFrostRequiresDistinctModel,
    Closure,
    MaterialComplementarityViolation,
    VaporSignOrSplit,
    Unsupported,
}

fn validate_inputs(input: TerminalPhaseInputs) -> Result<(), TerminalPhaseFailure> {
    let nonnegative = [
        input.beginning_pack_ice_kg_m2,
        input.beginning_surface_frost_kg_m2,
        input.beginning_liquid_kg_m2,
        input.beginning_cold_content_j_m2,
        input.deposition_kg_m2,
        input.sublimation_kg_m2,
        input.external_liquid_kg_m2,
        input.support_seconds,
    ];
    let signed = [
        input.non_vapor_energy_j_m2,
        input.vapor_latent_energy_j_m2,
        input.complete_energy_j_m2,
    ];
    if nonnegative
        .into_iter()
        .any(|value| !value.is_finite() || value < 0.0)
        || signed.into_iter().any(|value| !value.is_finite())
        || input.support_seconds <= 0.0
    {
        return Err(TerminalPhaseFailure::DomainOrNonFinite);
    }
    if input.sublimation_kg_m2
        > input.beginning_pack_ice_kg_m2 + input.beginning_surface_frost_kg_m2
    {
        return Err(TerminalPhaseFailure::SublimationExceedsBeginningSolid);
    }
    if (input.deposition_kg_m2 > 0.0 && input.sublimation_kg_m2 > 0.0)
        || (input.deposition_kg_m2 > 0.0 && input.vapor_latent_energy_j_m2 < 0.0)
        || (input.sublimation_kg_m2 > 0.0 && input.vapor_latent_energy_j_m2 > 0.0)
        || (input.deposition_kg_m2 == 0.0
            && input.sublimation_kg_m2 == 0.0
            && input.vapor_latent_energy_j_m2 != 0.0)
    {
        return Err(TerminalPhaseFailure::VaporSignOrSplit);
    }
    Ok(())
}

fn finalize(
    input: TerminalPhaseInputs,
    model: TerminalPhaseModel,
    ending_pack_ice_kg_m2: f64,
    ending_surface_frost_kg_m2: f64,
    ending_liquid_kg_m2: f64,
    ending_cold_content_j_m2: f64,
    melt_kg_m2: f64,
    refrozen_kg_m2: f64,
    unallocated_energy_j_m2: f64,
    event: TerminalEventChronology,
) -> Result<TerminalPhaseCandidate, TerminalPhaseFailure> {
    let beginning_solid = input.beginning_pack_ice_kg_m2 + input.beginning_surface_frost_kg_m2;
    let ending_solid = ending_pack_ice_kg_m2 + ending_surface_frost_kg_m2;
    let solid_closure_residual_kg_m2 = beginning_solid + input.deposition_kg_m2 + refrozen_kg_m2
        - input.sublimation_kg_m2
        - melt_kg_m2
        - ending_solid;
    let liquid_closure_residual_kg_m2 =
        input.beginning_liquid_kg_m2 + input.external_liquid_kg_m2 + melt_kg_m2
            - refrozen_kg_m2
            - ending_liquid_kg_m2;
    let cold_energy_change_j_m2 = input.beginning_cold_content_j_m2 - ending_cold_content_j_m2;
    let energy_closure_residual_j_m2 =
        input.complete_energy_j_m2 - cold_energy_change_j_m2 - LATENT_HEAT_FUSION_J_KG * melt_kg_m2
            + LATENT_HEAT_FUSION_J_KG * refrozen_kg_m2
            - unallocated_energy_j_m2;
    let vapor_energy_custody_residual_j_m2 =
        input.non_vapor_energy_j_m2 + input.vapor_latent_energy_j_m2 - input.complete_energy_j_m2;
    let values = [
        ending_pack_ice_kg_m2,
        ending_surface_frost_kg_m2,
        ending_liquid_kg_m2,
        ending_cold_content_j_m2,
        melt_kg_m2,
        refrozen_kg_m2,
        unallocated_energy_j_m2,
    ];
    if values
        .into_iter()
        .any(|value| !value.is_finite() || value < 0.0)
    {
        return Err(TerminalPhaseFailure::DomainOrNonFinite);
    }
    let mass_scale = 1.0_f64.max(
        beginning_solid
            + input.deposition_kg_m2
            + input.sublimation_kg_m2
            + input.beginning_liquid_kg_m2
            + input.external_liquid_kg_m2
            + melt_kg_m2
            + refrozen_kg_m2
            + ending_solid
            + ending_liquid_kg_m2,
    );
    let energy_scale = 1.0_f64.max(
        input.complete_energy_j_m2.abs()
            + cold_energy_change_j_m2.abs()
            + LATENT_HEAT_FUSION_J_KG * (melt_kg_m2 + refrozen_kg_m2)
            + unallocated_energy_j_m2,
    );
    if solid_closure_residual_kg_m2.abs() > MASS_TOLERANCE_KG_M2.max(1.0e-12 * mass_scale)
        || liquid_closure_residual_kg_m2.abs() > MASS_TOLERANCE_KG_M2.max(1.0e-12 * mass_scale)
        || energy_closure_residual_j_m2.abs() > ENERGY_TOLERANCE_J_M2.max(1.0e-12 * energy_scale)
        || vapor_energy_custody_residual_j_m2.abs()
            > ENERGY_TOLERANCE_J_M2.max(1.0e-12 * energy_scale)
    {
        return Err(TerminalPhaseFailure::Closure);
    }
    if model == TerminalPhaseModel::SimultaneousComplementarity
        && ending_solid > MASS_TOLERANCE_KG_M2
        && unallocated_energy_j_m2 > ENERGY_TOLERANCE_J_M2
    {
        return Err(TerminalPhaseFailure::MaterialComplementarityViolation);
    }
    Ok(TerminalPhaseCandidate {
        model,
        event,
        ending_pack_ice_kg_m2,
        ending_surface_frost_kg_m2,
        ending_liquid_kg_m2,
        ending_cold_content_j_m2,
        melt_kg_m2,
        refrozen_kg_m2,
        unallocated_energy_j_m2,
        solid_closure_residual_kg_m2,
        liquid_closure_residual_kg_m2,
        energy_closure_residual_j_m2,
        vapor_energy_custody_residual_j_m2,
    })
}

fn event_from_balance(
    input: TerminalPhaseInputs,
    beginning_solid: f64,
    ending_solid: f64,
    unallocated_energy_j_m2: f64,
) -> TerminalEventChronology {
    if beginning_solid <= MASS_TOLERANCE_KG_M2 && ending_solid > MASS_TOLERANCE_KG_M2 {
        TerminalEventChronology::Reappeared
    } else if beginning_solid <= MASS_TOLERANCE_KG_M2 {
        TerminalEventChronology::AtStart
    } else if ending_solid > MASS_TOLERANCE_KG_M2 {
        TerminalEventChronology::None
    } else if unallocated_energy_j_m2 > ENERGY_TOLERANCE_J_M2 {
        TerminalEventChronology::Interior
    } else if input.support_seconds > 0.0 {
        TerminalEventChronology::AtEnd
    } else {
        TerminalEventChronology::None
    }
}

/// Candidate A. Incoming deposition participates in the same endpoint solid
/// inventory. The minimum-circulation solution prohibits simultaneous net melt
/// and refreeze; positive energy first removes cold content and then melts all
/// available solid, while an energy deficit can refreeze available liquid.
pub(crate) fn simultaneous_complementarity(
    input: TerminalPhaseInputs,
) -> Result<TerminalPhaseCandidate, TerminalPhaseFailure> {
    validate_inputs(input)?;
    if input.beginning_surface_frost_kg_m2 > 0.0 {
        return Err(TerminalPhaseFailure::SurfaceFrostRequiresDistinctModel);
    }
    let available_solid =
        input.beginning_pack_ice_kg_m2 + input.deposition_kg_m2 - input.sublimation_kg_m2;
    let available_liquid = input.beginning_liquid_kg_m2 + input.external_liquid_kg_m2;
    let (ending_ice, ending_liquid, ending_cold, melt, refrozen, unallocated) =
        if input.complete_energy_j_m2 >= input.beginning_cold_content_j_m2 {
            let phase_energy = input.complete_energy_j_m2 - input.beginning_cold_content_j_m2;
            let melt = (phase_energy / LATENT_HEAT_FUSION_J_KG).min(available_solid);
            (
                available_solid - melt,
                available_liquid + melt,
                0.0,
                melt,
                0.0,
                (phase_energy - LATENT_HEAT_FUSION_J_KG * melt).max(0.0),
            )
        } else {
            let remaining_cold = input.beginning_cold_content_j_m2 - input.complete_energy_j_m2;
            let refrozen = (remaining_cold / LATENT_HEAT_FUSION_J_KG).min(available_liquid);
            (
                available_solid + refrozen,
                available_liquid - refrozen,
                remaining_cold - LATENT_HEAT_FUSION_J_KG * refrozen,
                0.0,
                refrozen,
                0.0,
            )
        };
    let event = event_from_balance(
        input,
        input.beginning_pack_ice_kg_m2,
        ending_ice,
        unallocated,
    );
    finalize(
        input,
        TerminalPhaseModel::SimultaneousComplementarity,
        ending_ice,
        0.0,
        ending_liquid,
        ending_cold,
        melt,
        refrozen,
        unallocated,
        event,
    )
}

/// Candidate B. Beginning frost is exposed to current-support sublimation and
/// melt, but new deposition is installed only after the support as a distinct
/// frost owner. This explicit lag is intentionally tested for partition
/// sensitivity; it is not production authority.
pub(crate) fn residual_surface_frost(
    input: TerminalPhaseInputs,
) -> Result<TerminalPhaseCandidate, TerminalPhaseFailure> {
    validate_inputs(input)?;
    let sublimated_frost = input
        .sublimation_kg_m2
        .min(input.beginning_surface_frost_kg_m2);
    let remaining_sublimation = input.sublimation_kg_m2 - sublimated_frost;
    let frost_before_melt = input.beginning_surface_frost_kg_m2 - sublimated_frost;
    let pack_before_melt = input.beginning_pack_ice_kg_m2 - remaining_sublimation;
    let available_beginning_solid = frost_before_melt + pack_before_melt;
    let available_liquid = input.beginning_liquid_kg_m2 + input.external_liquid_kg_m2;
    let (
        mut ending_pack,
        mut ending_frost,
        ending_liquid,
        ending_cold,
        melt,
        refrozen,
        unallocated,
    ) = if input.complete_energy_j_m2 >= input.beginning_cold_content_j_m2 {
        let phase_energy = input.complete_energy_j_m2 - input.beginning_cold_content_j_m2;
        let melt = (phase_energy / LATENT_HEAT_FUSION_J_KG).min(available_beginning_solid);
        let frost_melt = melt.min(frost_before_melt);
        let pack_melt = melt - frost_melt;
        (
            pack_before_melt - pack_melt,
            frost_before_melt - frost_melt,
            available_liquid + melt,
            0.0,
            melt,
            0.0,
            (phase_energy - LATENT_HEAT_FUSION_J_KG * melt).max(0.0),
        )
    } else {
        let remaining_cold = input.beginning_cold_content_j_m2 - input.complete_energy_j_m2;
        let refrozen = (remaining_cold / LATENT_HEAT_FUSION_J_KG).min(available_liquid);
        (
            pack_before_melt + refrozen,
            frost_before_melt,
            available_liquid - refrozen,
            remaining_cold - LATENT_HEAT_FUSION_J_KG * refrozen,
            0.0,
            refrozen,
            0.0,
        )
    };
    ending_frost += input.deposition_kg_m2;
    if ending_pack.abs() <= MASS_TOLERANCE_KG_M2 {
        ending_pack = 0.0;
    }
    let event = event_from_balance(
        input,
        input.beginning_pack_ice_kg_m2 + input.beginning_surface_frost_kg_m2,
        ending_pack + ending_frost,
        unallocated,
    );
    finalize(
        input,
        TerminalPhaseModel::ResidualSurfaceFrost,
        ending_pack,
        ending_frost,
        ending_liquid,
        ending_cold,
        melt,
        refrozen,
        unallocated,
        event,
    )
}

pub(crate) fn typed_unsupported() -> Result<TerminalPhaseCandidate, TerminalPhaseFailure> {
    Err(TerminalPhaseFailure::Unsupported)
}

pub(crate) fn inputs_from_real_endpoint(
    endpoint: &crate::snow_stage3_v11_attachment::RealDiscreteCompleteEndpointEvidenceV1,
) -> TerminalPhaseInputs {
    let complete_energy_j_m2 = f64::from_bits(endpoint.complete_energy_bits);
    let vapor_latent_energy_j_m2 = f64::from_bits(endpoint.latent_energy_bits);
    TerminalPhaseInputs {
        beginning_pack_ice_kg_m2: f64::from_bits(endpoint.start_ice_bits),
        beginning_surface_frost_kg_m2: 0.0,
        beginning_liquid_kg_m2: f64::from_bits(endpoint.start_liquid_bits),
        beginning_cold_content_j_m2: f64::from_bits(endpoint.start_cold_content_bits),
        deposition_kg_m2: f64::from_bits(endpoint.deposition_bits),
        sublimation_kg_m2: f64::from_bits(endpoint.sublimation_bits),
        external_liquid_kg_m2: f64::from_bits(endpoint.external_liquid_bits),
        non_vapor_energy_j_m2: complete_energy_j_m2 - vapor_latent_energy_j_m2,
        vapor_latent_energy_j_m2,
        complete_energy_j_m2,
        support_seconds: f64::from_bits(endpoint.support.duration_s_bits()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vector(
        ice: f64,
        liquid: f64,
        cold: f64,
        deposition: f64,
        sublimation: f64,
        external_liquid: f64,
        energy: f64,
    ) -> TerminalPhaseInputs {
        let vapor_latent = if deposition > 0.0 {
            2_834_000.0 * deposition
        } else {
            -2_834_000.0 * sublimation
        };
        TerminalPhaseInputs {
            beginning_pack_ice_kg_m2: ice,
            beginning_surface_frost_kg_m2: 0.0,
            beginning_liquid_kg_m2: liquid,
            beginning_cold_content_j_m2: cold,
            deposition_kg_m2: deposition,
            sublimation_kg_m2: sublimation,
            external_liquid_kg_m2: external_liquid,
            non_vapor_energy_j_m2: energy - vapor_latent,
            vapor_latent_energy_j_m2: vapor_latent,
            complete_energy_j_m2: energy,
            support_seconds: 10.0,
        }
    }

    fn assert_closed(value: TerminalPhaseCandidate) {
        assert!(value.solid_closure_residual_kg_m2.abs() <= MASS_TOLERANCE_KG_M2);
        assert!(value.liquid_closure_residual_kg_m2.abs() <= MASS_TOLERANCE_KG_M2);
        assert!(value.energy_closure_residual_j_m2.abs() <= ENERGY_TOLERANCE_J_M2);
        assert!(value.vapor_energy_custody_residual_j_m2.abs() <= ENERGY_TOLERANCE_J_M2);
    }

    #[test]
    fn result_blind_phase_matrix_closes_and_separates_aliases() {
        let cases = [
            (
                "zero_vapor",
                vector(0.6, 0.0, 0.0, 0.0, 0.0, 0.0, 100_000.0),
            ),
            (
                "sublimation",
                vector(0.6, 0.0, 0.0, 0.0, 0.01, 0.0, 100_000.0),
            ),
            (
                "deposition_below_balance",
                vector(0.6, 0.0, 0.0, 0.01, 0.0, 0.0, 100_000.0),
            ),
            (
                "deposition_at_balance",
                vector(
                    0.6,
                    0.0,
                    0.0,
                    0.01,
                    0.0,
                    0.0,
                    0.61 * LATENT_HEAT_FUSION_J_KG,
                ),
            ),
            (
                "deposition_above_balance",
                vector(
                    0.6,
                    0.0,
                    0.0,
                    0.01,
                    0.0,
                    0.0,
                    0.61 * LATENT_HEAT_FUSION_J_KG + 10.0,
                ),
            ),
            ("refreeze", vector(0.6, 0.02, 10_000.0, 0.0, 0.0, 0.0, 0.0)),
            (
                "deposition_plus_refreeze",
                vector(0.6, 0.02, 10_000.0, 0.01, 0.0, 0.0, 0.0),
            ),
            (
                "rain_on_snow",
                vector(0.6, 0.0, 1_000.0, 0.0, 0.0, 0.03, 20_000.0),
            ),
            (
                "positive_cold",
                vector(0.6, 0.0, 1_000.0, 0.0, 0.0, 0.0, 500.0),
            ),
            ("zero_cold", vector(0.6, 0.0, 0.0, 0.0, 0.0, 0.0, 500.0)),
        ];
        for (name, input) in cases {
            let value = simultaneous_complementarity(input)
                .unwrap_or_else(|error| panic!("{name}: {error:?}"));
            assert_closed(value);
            assert!(
                value.ending_pack_ice_kg_m2 <= MASS_TOLERANCE_KG_M2
                    || value.unallocated_energy_j_m2 <= ENERGY_TOLERANCE_J_M2,
                "{name}: material ice/excess-energy coexistence"
            );
        }
    }

    #[test]
    fn event_start_interior_and_end_are_distinct() {
        let start = simultaneous_complementarity(vector(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0))
            .expect("start event");
        let end = simultaneous_complementarity(vector(
            0.6,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.6 * LATENT_HEAT_FUSION_J_KG,
        ))
        .expect("end event");
        let interior = simultaneous_complementarity(vector(
            0.6,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.6 * LATENT_HEAT_FUSION_J_KG + 10.0,
        ))
        .expect("interior event");
        assert_eq!(start.event, TerminalEventChronology::AtStart);
        assert_eq!(end.event, TerminalEventChronology::AtEnd);
        assert_eq!(interior.event, TerminalEventChronology::Interior);
    }

    #[test]
    fn simultaneous_model_is_support_partition_invariant_for_additive_forcing() {
        let full =
            simultaneous_complementarity(vector(0.6, 0.0, 1_000.0, 0.002, 0.0, 0.01, 50_000.0))
                .expect("full");
        let first =
            simultaneous_complementarity(vector(0.6, 0.0, 1_000.0, 0.001, 0.0, 0.005, 25_000.0))
                .expect("first");
        let mut second_input = vector(
            first.ending_pack_ice_kg_m2,
            first.ending_liquid_kg_m2,
            first.ending_cold_content_j_m2,
            0.001,
            0.0,
            0.005,
            25_000.0,
        );
        second_input.support_seconds = 5.0;
        let second = simultaneous_complementarity(second_input).expect("second");
        assert!((full.ending_pack_ice_kg_m2 - second.ending_pack_ice_kg_m2).abs() <= 1.0e-12);
        assert!((full.ending_liquid_kg_m2 - second.ending_liquid_kg_m2).abs() <= 1.0e-12);
        assert!((full.ending_cold_content_j_m2 - second.ending_cold_content_j_m2).abs() <= 1.0e-9);
    }

    #[test]
    fn residual_frost_lag_is_materially_partition_dependent() {
        let full_input = vector(
            0.6,
            0.0,
            0.0,
            0.002,
            0.0,
            0.0,
            0.6 * LATENT_HEAT_FUSION_J_KG + 1_000.0,
        );
        let full = residual_surface_frost(full_input).expect("full frost");
        let mut first_input = full_input;
        first_input.deposition_kg_m2 *= 0.5;
        first_input.vapor_latent_energy_j_m2 *= 0.5;
        first_input.complete_energy_j_m2 *= 0.5;
        first_input.non_vapor_energy_j_m2 =
            first_input.complete_energy_j_m2 - first_input.vapor_latent_energy_j_m2;
        first_input.support_seconds *= 0.5;
        let first = residual_surface_frost(first_input).expect("first frost");
        let mut second_input = first_input;
        second_input.beginning_pack_ice_kg_m2 = first.ending_pack_ice_kg_m2;
        second_input.beginning_surface_frost_kg_m2 = first.ending_surface_frost_kg_m2;
        second_input.beginning_liquid_kg_m2 = first.ending_liquid_kg_m2;
        second_input.beginning_cold_content_j_m2 = first.ending_cold_content_j_m2;
        let second = residual_surface_frost(second_input).expect("second frost");
        assert!(
            (full.ending_surface_frost_kg_m2 - second.ending_surface_frost_kg_m2).abs()
                > MASS_TOLERANCE_KG_M2
                || (full.unallocated_energy_j_m2 - second.unallocated_energy_j_m2).abs()
                    > ENERGY_TOLERANCE_J_M2
        );
    }

    #[test]
    fn persistent_deposition_reappears_and_beginning_frost_can_melt_later() {
        let deposition = simultaneous_complementarity(vector(0.0, 0.0, 0.0, 0.001, 0.0, 0.0, 0.0))
            .expect("deposition reappearance");
        assert_eq!(deposition.event, TerminalEventChronology::Reappeared);
        assert_eq!(deposition.ending_pack_ice_kg_m2, 0.001);

        let mut frost_input = vector(
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.001 * LATENT_HEAT_FUSION_J_KG,
        );
        frost_input.beginning_surface_frost_kg_m2 = 0.001;
        let melted = residual_surface_frost(frost_input).expect("subsequent frost melt");
        assert_eq!(melted.event, TerminalEventChronology::AtEnd);
        assert_eq!(
            melted.ending_surface_frost_kg_m2.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(melted.ending_liquid_kg_m2, 0.001);
    }

    #[test]
    fn vapor_energy_is_counted_exactly_once_and_poison_rejects() {
        let input = vector(0.6, 0.0, 0.0, 0.01, 0.0, 0.0, 10_000.0);
        assert_closed(simultaneous_complementarity(input).expect("exact custody"));
        let poisoned = TerminalPhaseInputs {
            non_vapor_energy_j_m2: input.non_vapor_energy_j_m2 + input.vapor_latent_energy_j_m2,
            ..input
        };
        assert_eq!(
            simultaneous_complementarity(poisoned),
            Err(TerminalPhaseFailure::Closure)
        );
        let sign_poison = TerminalPhaseInputs {
            vapor_latent_energy_j_m2: -input.vapor_latent_energy_j_m2,
            non_vapor_energy_j_m2: input.complete_energy_j_m2 + input.vapor_latent_energy_j_m2,
            ..input
        };
        assert_eq!(
            simultaneous_complementarity(sign_poison),
            Err(TerminalPhaseFailure::VaporSignOrSplit)
        );
    }

    #[test]
    fn invalid_domains_and_typed_unsupported_fail_closed() {
        let invalid = TerminalPhaseInputs {
            beginning_pack_ice_kg_m2: f64::NAN,
            ..vector(0.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        };
        assert_eq!(
            simultaneous_complementarity(invalid),
            Err(TerminalPhaseFailure::DomainOrNonFinite)
        );
        assert_eq!(typed_unsupported(), Err(TerminalPhaseFailure::Unsupported));
    }
}
