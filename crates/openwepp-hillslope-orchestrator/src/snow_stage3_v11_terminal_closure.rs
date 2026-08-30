#[derive(Clone, Copy, Debug)]
struct TerminalClosureOperandsV1 {
    start_ice_kg_m2: f64,
    start_liquid_kg_m2: f64,
    end_ice_kg_m2: f64,
    end_liquid_kg_m2: f64,
    complete_energy_j_m2: f64,
    cold_energy_change_j_m2: f64,
    refrozen_kg_m2: f64,
    deposition_kg_m2: f64,
    sublimation_kg_m2: f64,
    melt_kg_m2: f64,
    unallocated_energy_j_m2: f64,
    external_liquid_kg_m2: f64,
}

impl From<&DirectSnowTerminalEventResult> for TerminalClosureOperandsV1 {
    fn from(event: &DirectSnowTerminalEventResult) -> Self {
        Self {
            start_ice_kg_m2: event.start_ice_kg_m2,
            start_liquid_kg_m2: event.start_liquid_kg_m2,
            end_ice_kg_m2: event.end_ice_kg_m2,
            end_liquid_kg_m2: event.terminal_liquid_kg_m2,
            complete_energy_j_m2: event.complete_energy_j_m2,
            cold_energy_change_j_m2: event.cold_energy_change_j_m2,
            refrozen_kg_m2: event.refrozen_kg_m2,
            deposition_kg_m2: event.deposition_kg_m2,
            sublimation_kg_m2: event.sublimation_kg_m2,
            melt_kg_m2: event.melt_kg_m2,
            unallocated_energy_j_m2: event.terminal_unallocated_energy_j_m2,
            external_liquid_kg_m2: event.external_liquid_kg_m2,
        }
    }
}

fn reconstruct_terminal_closure_v1(
    operands: TerminalClosureOperandsV1,
) -> Result<[f64; 3], DirectSnowStage3V11AttachmentError> {
    let values = [
        operands.start_ice_kg_m2,
        operands.start_liquid_kg_m2,
        operands.end_ice_kg_m2,
        operands.end_liquid_kg_m2,
        operands.complete_energy_j_m2,
        operands.cold_energy_change_j_m2,
        operands.refrozen_kg_m2,
        operands.deposition_kg_m2,
        operands.sublimation_kg_m2,
        operands.melt_kg_m2,
        operands.unallocated_energy_j_m2,
        operands.external_liquid_kg_m2,
    ];
    if values.iter().any(|value| !value.is_finite()) {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "adaptive preterminal closure operand domain",
        ));
    }
    let solid_residual =
        operands.start_ice_kg_m2 + operands.refrozen_kg_m2 + operands.deposition_kg_m2
            - operands.sublimation_kg_m2
            - operands.melt_kg_m2
            - operands.end_ice_kg_m2;
    let liquid_residual =
        operands.start_liquid_kg_m2 + operands.external_liquid_kg_m2 + operands.melt_kg_m2
            - operands.refrozen_kg_m2
            - operands.end_liquid_kg_m2;
    let fusion = crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG;
    let energy_residual = operands.complete_energy_j_m2
        - operands.cold_energy_change_j_m2
        - fusion * operands.melt_kg_m2
        + fusion * operands.refrozen_kg_m2
        - operands.unallocated_energy_j_m2;
    let mass_scale = operands.start_ice_kg_m2
        + operands.refrozen_kg_m2
        + operands.deposition_kg_m2
        + operands.sublimation_kg_m2
        + operands.melt_kg_m2
        + operands.end_ice_kg_m2
        + operands.start_liquid_kg_m2
        + operands.external_liquid_kg_m2
        + operands.end_liquid_kg_m2;
    let mass_tolerance = 1.0e-12_f64.max(1.0e-12 * mass_scale);
    let energy_scale = operands.complete_energy_j_m2.abs()
        + operands.cold_energy_change_j_m2.abs()
        + fusion * (operands.melt_kg_m2 + operands.refrozen_kg_m2)
        + operands.unallocated_energy_j_m2.abs();
    let energy_tolerance = 1.0e-6_f64.max(1.0e-12 * energy_scale);
    if !solid_residual.is_finite()
        || !liquid_residual.is_finite()
        || !energy_residual.is_finite()
        || solid_residual.abs() > mass_tolerance
        || liquid_residual.abs() > mass_tolerance
        || energy_residual.abs() > energy_tolerance
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "adaptive preterminal closure reconstruction",
        ));
    }
    Ok([solid_residual, liquid_residual, energy_residual])
}

pub(crate) fn terminal_liquid_thermodynamics_v1(
    mass_kg_m2: f64,
    unallocated_energy_j_m2: f64,
) -> Result<(f64, f64), DirectSnowStage3V11AttachmentError> {
    if !mass_kg_m2.is_finite() || !unallocated_energy_j_m2.is_finite() || mass_kg_m2 < 0.0 {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal liquid mass/enthalpy operand domain",
        ));
    }
    let projected_specific_enthalpy_j_kg = if mass_kg_m2 > 0.0 {
        unallocated_energy_j_m2 / mass_kg_m2
    } else if unallocated_energy_j_m2.to_bits() == 0.0_f64.to_bits() {
        0.0
    } else {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "zero-mass terminal liquid carries sensible enthalpy",
        ));
    };
    let temperature_k = 273.15 + projected_specific_enthalpy_j_kg / 4_218.0;
    let specific_liquid_enthalpy_j_kg =
        openwepp_land_surface_energy::liquid_enthalpy_j_kg(temperature_k);
    if !specific_liquid_enthalpy_j_kg.is_finite()
        || !(200.0..=350.0).contains(&temperature_k)
        || (specific_liquid_enthalpy_j_kg - projected_specific_enthalpy_j_kg).abs()
            > 1.0e-9_f64.max(1.0e-12 * projected_specific_enthalpy_j_kg.abs())
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal liquid temperature/enthalpy domain",
        ));
    }
    Ok((temperature_k, specific_liquid_enthalpy_j_kg))
}
