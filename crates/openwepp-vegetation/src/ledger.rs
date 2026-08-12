//! Independent five-ledger reconstruction from authoritative transaction operands.

use openwepp_kernel_contract::{MineralNitrogenKey, SoilLayerId, TransactionId};

use crate::VegetationError;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LedgerIdentity {
    pub transaction_id: TransactionId,
    pub owner_id: String,
    pub area_m2: f64,
    pub interval_s: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WaterStoreOperand {
    pub layer_id: SoilLayerId,
    pub beginning_kg_m2: f64,
    pub withdrawal_kg_m2: f64,
    pub ending_kg_m2: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WaterLedgerOperands {
    pub identity: LedgerIdentity,
    pub soil: Vec<WaterStoreOperand>,
    pub canopy_beginning_kg_m2: f64,
    pub precipitation_kg_m2: f64,
    pub canopy_ending_kg_m2: f64,
    pub canopy_evaporation_kg_m2: f64,
    pub throughfall_kg_m2: f64,
    pub stemflow_kg_m2: f64,
    pub drainage_kg_m2: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct EnergyLedgerOperands {
    pub identity: LedgerIdentity,
    pub incident_shortwave_j_m2: f64,
    pub incident_longwave_j_m2: f64,
    pub reflected_shortwave_j_m2: f64,
    pub terminal_shortwave_j_m2: f64,
    pub emitted_longwave_j_m2: f64,
    pub sensible_j_m2: f64,
    pub latent_j_m2: f64,
    pub ground_or_storage_j_m2: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CarbonLedgerOperands {
    pub identity: LedgerIdentity,
    pub beginning_vegetation_kg_m2: f64,
    pub gross_input_kg_m2: f64,
    pub maintenance_respiration_kg_m2: f64,
    pub growth_respiration_kg_m2: f64,
    pub exported_kg_m2: f64,
    pub ending_vegetation_kg_m2: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NitrogenStoreOperand {
    pub key: MineralNitrogenKey,
    pub beginning_kg_m2: f64,
    pub finalized_use_kg_m2: f64,
    pub ending_kg_m2: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NitrogenLedgerOperands {
    pub identity: LedgerIdentity,
    pub vegetation_beginning_kg_m2: f64,
    pub mineral: Vec<NitrogenStoreOperand>,
    pub exported_kg_m2: f64,
    pub vegetation_ending_kg_m2: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct DryMaterialLedgerOperands {
    pub identity: LedgerIdentity,
    pub vegetation_beginning_kg_m2: f64,
    pub growth_input_kg_m2: f64,
    pub exported_kg_m2: f64,
    pub vegetation_ending_kg_m2: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FiveLedgerOperands {
    pub water: WaterLedgerOperands,
    pub energy: EnergyLedgerOperands,
    pub carbon: CarbonLedgerOperands,
    pub nitrogen: NitrogenLedgerOperands,
    pub dry_material: DryMaterialLedgerOperands,
}

#[allow(clippy::too_many_lines)]
pub fn validate_five_ledgers(operands: &FiveLedgerOperands) -> Result<[f64; 5], VegetationError> {
    let identity = &operands.water.identity;
    for candidate in [
        &operands.energy.identity,
        &operands.carbon.identity,
        &operands.nitrogen.identity,
        &operands.dry_material.identity,
    ] {
        if candidate != identity {
            return Err(VegetationError::Receipt("ledger identity mismatch".into()));
        }
    }
    validate_identity(identity)?;
    let water_soil = operands
        .water
        .soil
        .iter()
        .map(|store| store.beginning_kg_m2 - store.withdrawal_kg_m2 - store.ending_kg_m2)
        .sum::<f64>();
    let water_canopy = operands.water.canopy_beginning_kg_m2 + operands.water.precipitation_kg_m2
        - operands.water.canopy_ending_kg_m2
        - operands.water.canopy_evaporation_kg_m2
        - operands.water.throughfall_kg_m2
        - operands.water.stemflow_kg_m2
        - operands.water.drainage_kg_m2;
    let water = water_soil + water_canopy;
    let energy = operands.energy.incident_shortwave_j_m2 + operands.energy.incident_longwave_j_m2
        - operands.energy.reflected_shortwave_j_m2
        - operands.energy.terminal_shortwave_j_m2
        - operands.energy.emitted_longwave_j_m2
        - operands.energy.sensible_j_m2
        - operands.energy.latent_j_m2
        - operands.energy.ground_or_storage_j_m2;
    let carbon = operands.carbon.beginning_vegetation_kg_m2 + operands.carbon.gross_input_kg_m2
        - operands.carbon.maintenance_respiration_kg_m2
        - operands.carbon.growth_respiration_kg_m2
        - operands.carbon.exported_kg_m2
        - operands.carbon.ending_vegetation_kg_m2;
    let mineral_n = operands
        .nitrogen
        .mineral
        .iter()
        .map(|store| store.beginning_kg_m2 - store.finalized_use_kg_m2 - store.ending_kg_m2)
        .sum::<f64>();
    let nitrogen = mineral_n
        + operands.nitrogen.vegetation_beginning_kg_m2
        + operands
            .nitrogen
            .mineral
            .iter()
            .map(|store| store.finalized_use_kg_m2)
            .sum::<f64>()
        - operands.nitrogen.exported_kg_m2
        - operands.nitrogen.vegetation_ending_kg_m2;
    let dry = operands.dry_material.vegetation_beginning_kg_m2
        + operands.dry_material.growth_input_kg_m2
        - operands.dry_material.exported_kg_m2
        - operands.dry_material.vegetation_ending_kg_m2;
    let residuals = [water, energy, carbon, nitrogen, dry];
    if residuals.iter().any(|value| !value.is_finite()) {
        return Err(VegetationError::Domain("ledger operand"));
    }
    let water_sum = operands
        .water
        .soil
        .iter()
        .map(|store| store.beginning_kg_m2 + store.withdrawal_kg_m2 + store.ending_kg_m2)
        .sum::<f64>()
        + operands.water.canopy_beginning_kg_m2
        + operands.water.precipitation_kg_m2
        + operands.water.canopy_ending_kg_m2
        + operands.water.canopy_evaporation_kg_m2.abs()
        + operands.water.throughfall_kg_m2
        + operands.water.stemflow_kg_m2
        + operands.water.drainage_kg_m2;
    let energy_sum = operands.energy.incident_shortwave_j_m2.abs()
        + operands.energy.incident_longwave_j_m2.abs()
        + operands.energy.reflected_shortwave_j_m2.abs()
        + operands.energy.terminal_shortwave_j_m2.abs()
        + operands.energy.emitted_longwave_j_m2.abs()
        + operands.energy.sensible_j_m2.abs()
        + operands.energy.latent_j_m2.abs()
        + operands.energy.ground_or_storage_j_m2.abs();
    let carbon_sum = operands.carbon.beginning_vegetation_kg_m2
        + operands.carbon.gross_input_kg_m2
        + operands.carbon.maintenance_respiration_kg_m2
        + operands.carbon.growth_respiration_kg_m2
        + operands.carbon.exported_kg_m2
        + operands.carbon.ending_vegetation_kg_m2;
    let nitrogen_sum = operands.nitrogen.vegetation_beginning_kg_m2
        + operands.nitrogen.exported_kg_m2
        + operands.nitrogen.vegetation_ending_kg_m2
        + operands
            .nitrogen
            .mineral
            .iter()
            .map(|store| store.beginning_kg_m2 + store.finalized_use_kg_m2 + store.ending_kg_m2)
            .sum::<f64>();
    let dry_sum = operands.dry_material.vegetation_beginning_kg_m2
        + operands.dry_material.growth_input_kg_m2
        + operands.dry_material.exported_kg_m2
        + operands.dry_material.vegetation_ending_kg_m2;
    for ((name, residual), operand_sum) in ["water", "energy", "carbon", "nitrogen", "dry material"]
        .into_iter()
        .zip(residuals)
        .zip([water_sum, energy_sum, carbon_sum, nitrogen_sum, dry_sum])
    {
        let tolerance = match name {
            "energy" => 1e-6 * identity.interval_s + 1e-10 * operand_sum,
            "carbon" | "nitrogen" | "dry material" => 1e-14 + 64.0 * f64::EPSILON * operand_sum,
            _ => 1e-12 + 64.0 * f64::EPSILON * operand_sum,
        };
        if residual.abs() > tolerance {
            return Err(VegetationError::Closure {
                ledger: name,
                residual,
            });
        }
    }
    Ok(residuals)
}

fn validate_identity(identity: &LedgerIdentity) -> Result<(), VegetationError> {
    if identity.owner_id.trim().is_empty()
        || !identity.area_m2.is_finite()
        || identity.area_m2 <= 0.0
        || !identity.interval_s.is_finite()
        || identity.interval_s <= 0.0
    {
        return Err(VegetationError::Domain("ledger identity"));
    }
    Ok(())
}
