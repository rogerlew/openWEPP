//! Independent five-ledger reconstruction from authoritative transaction operands.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{
    MaterialDonorClass, MaterialReceiverClass, MineralNitrogenKey, SoilLayerId, TransactionId,
    WaterResourceKey,
};

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

/// Exact vegetation-use identity paired with the hydrology-owner debit.
///
/// `maximum_authorization_kg_m2` is exposed only as a bound. Closure is
/// reconstructed from `finalized_use_kg_m2` and `owner_debit_kg_m2`; an unused
/// authorization is never treated as a withdrawal.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WaterFinalizedUseOperand {
    pub transaction_id: TransactionId,
    pub owner_id: String,
    pub key: WaterResourceKey,
    pub maximum_authorization_kg_m2: f64,
    pub finalized_use_kg_m2: f64,
    pub owner_debit_kg_m2: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WaterLedgerOperands {
    pub identity: LedgerIdentity,
    /// Complete configured occupancy-layer key set, including exact-zero uses.
    pub authoritative_use_keys: Vec<WaterResourceKey>,
    pub finalized_uses: Vec<WaterFinalizedUseOperand>,
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
    pub maximum_authorization_kg_m2: f64,
    pub finalized_use_kg_m2: f64,
    pub ending_kg_m2: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NitrogenLedgerOperands {
    pub identity: LedgerIdentity,
    pub vegetation_beginning_kg_m2: f64,
    /// Complete admitted layer/species inventory key set, including zeros.
    pub authoritative_mineral_keys: Vec<MineralNitrogenKey>,
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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct MaterialTransferIdentity {
    pub transaction_id: TransactionId,
    pub owner_id: String,
    pub proposal_id: u64,
    pub donor: MaterialDonorClass,
    pub receiver: MaterialReceiverClass,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MaterialTransferLedgerOperand {
    pub identity: MaterialTransferIdentity,
    pub carbon_kg_m2: f64,
    pub nitrogen_kg_m2: f64,
    pub dry_matter_kg_m2: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FiveLedgerOperands {
    pub water: WaterLedgerOperands,
    pub energy: EnergyLedgerOperands,
    pub carbon: CarbonLedgerOperands,
    pub nitrogen: NitrogenLedgerOperands,
    pub dry_material: DryMaterialLedgerOperands,
    /// Vegetation-side debits and receiver-side credits are independently
    /// supplied and must correspond one-for-one with identical C/N/DM amounts.
    pub material_donor_debits: Vec<MaterialTransferLedgerOperand>,
    pub material_receiver_credits: Vec<MaterialTransferLedgerOperand>,
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
    let water_soil = validate_water_operands(&operands.water)?;
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
    let mineral_n = validate_nitrogen_operands(&operands.nitrogen)?;
    validate_material_transfers(operands)?;
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

fn validate_water_operands(operands: &WaterLedgerOperands) -> Result<f64, VegetationError> {
    let authoritative = unique_keys(
        &operands.authoritative_use_keys,
        "duplicate authoritative water key",
    )?;
    let mut actual = BTreeSet::new();
    let mut debits_by_layer = BTreeMap::<SoilLayerId, f64>::new();
    for finalized in &operands.finalized_uses {
        if finalized.transaction_id != operands.identity.transaction_id
            || finalized.owner_id.trim().is_empty()
        {
            return Err(VegetationError::Receipt(
                "water finalized-use identity mismatch".into(),
            ));
        }
        validate_nonnegative(
            &[
                finalized.maximum_authorization_kg_m2,
                finalized.finalized_use_kg_m2,
                finalized.owner_debit_kg_m2,
            ],
            "water finalized-use operand",
        )?;
        if finalized.finalized_use_kg_m2 > finalized.maximum_authorization_kg_m2 {
            return Err(VegetationError::Receipt(
                "water finalized use exceeds authorization".into(),
            ));
        }
        if !actual.insert(finalized.key.clone()) {
            return Err(VegetationError::Receipt(
                "duplicate water finalized-use key".into(),
            ));
        }
        require_equal(
            "water finalized use owner debit",
            finalized.owner_debit_kg_m2,
            finalized.finalized_use_kg_m2,
            1e-12,
        )?;
        *debits_by_layer
            .entry(finalized.key.layer_id.clone())
            .or_default() += finalized.owner_debit_kg_m2;
    }
    if actual != authoritative {
        return Err(VegetationError::Receipt(
            "water authoritative key set mismatch".into(),
        ));
    }

    let mut soil_layers = BTreeSet::new();
    let mut residual = 0.0;
    for store in &operands.soil {
        if !soil_layers.insert(store.layer_id.clone()) {
            return Err(VegetationError::Receipt(
                "duplicate water storage layer".into(),
            ));
        }
        validate_nonnegative(
            &[
                store.beginning_kg_m2,
                store.withdrawal_kg_m2,
                store.ending_kg_m2,
            ],
            "water storage operand",
        )?;
        let exact_debit = debits_by_layer.get(&store.layer_id).copied().unwrap_or(0.0);
        require_equal(
            "water layer debit",
            store.withdrawal_kg_m2,
            exact_debit,
            1e-12,
        )?;
        let row = store.beginning_kg_m2 - store.withdrawal_kg_m2 - store.ending_kg_m2;
        require_closed("water layer", row, 1e-12, store.beginning_kg_m2)?;
        residual += row;
    }
    if debits_by_layer
        .keys()
        .any(|layer_id| !soil_layers.contains(layer_id))
    {
        return Err(VegetationError::Receipt(
            "water finalized use has no storage layer".into(),
        ));
    }
    Ok(residual)
}

fn validate_nitrogen_operands(operands: &NitrogenLedgerOperands) -> Result<f64, VegetationError> {
    let authoritative = unique_keys(
        &operands.authoritative_mineral_keys,
        "duplicate authoritative nitrogen key",
    )?;
    let mut actual = BTreeSet::new();
    let mut residual = 0.0;
    for store in &operands.mineral {
        if !actual.insert(store.key.clone()) {
            return Err(VegetationError::Receipt(
                "duplicate mineral nitrogen key".into(),
            ));
        }
        validate_nonnegative(
            &[
                store.beginning_kg_m2,
                store.maximum_authorization_kg_m2,
                store.finalized_use_kg_m2,
                store.ending_kg_m2,
            ],
            "mineral nitrogen operand",
        )?;
        if store.finalized_use_kg_m2 > store.maximum_authorization_kg_m2 {
            return Err(VegetationError::Receipt(
                "mineral nitrogen finalized use exceeds authorization".into(),
            ));
        }
        let row = store.beginning_kg_m2 - store.finalized_use_kg_m2 - store.ending_kg_m2;
        require_closed("mineral nitrogen key", row, 1e-14, store.beginning_kg_m2)?;
        residual += row;
    }
    if actual != authoritative {
        return Err(VegetationError::Receipt(
            "mineral nitrogen authoritative key set mismatch".into(),
        ));
    }
    Ok(residual)
}

fn validate_material_transfers(operands: &FiveLedgerOperands) -> Result<(), VegetationError> {
    let donors = indexed_material_operands(
        &operands.material_donor_debits,
        &operands.water.identity,
        "duplicate material donor debit",
    )?;
    let receivers = indexed_material_operands(
        &operands.material_receiver_credits,
        &operands.water.identity,
        "duplicate material receiver credit",
    )?;
    if donors.keys().collect::<BTreeSet<_>>() != receivers.keys().collect::<BTreeSet<_>>() {
        return Err(VegetationError::Receipt(
            "material donor/receiver key set mismatch".into(),
        ));
    }
    for (identity, debit) in &donors {
        let credit = receivers
            .get(identity)
            .ok_or_else(|| VegetationError::Receipt("material receiver credit missing".into()))?;
        for (ledger, debit_amount, credit_amount) in [
            ("carbon transfer", debit.carbon_kg_m2, credit.carbon_kg_m2),
            (
                "nitrogen transfer",
                debit.nitrogen_kg_m2,
                credit.nitrogen_kg_m2,
            ),
            (
                "dry material transfer",
                debit.dry_matter_kg_m2,
                credit.dry_matter_kg_m2,
            ),
        ] {
            require_equal(ledger, debit_amount, credit_amount, 1e-14)?;
        }
    }
    let carbon = donors.values().map(|row| row.carbon_kg_m2).sum::<f64>();
    let nitrogen = donors.values().map(|row| row.nitrogen_kg_m2).sum::<f64>();
    let dry = donors.values().map(|row| row.dry_matter_kg_m2).sum::<f64>();
    require_equal(
        "carbon transfer export",
        operands.carbon.exported_kg_m2,
        carbon,
        1e-14,
    )?;
    require_equal(
        "nitrogen transfer export",
        operands.nitrogen.exported_kg_m2,
        nitrogen,
        1e-14,
    )?;
    require_equal(
        "dry material transfer export",
        operands.dry_material.exported_kg_m2,
        dry,
        1e-14,
    )?;
    Ok(())
}

fn indexed_material_operands<'a>(
    rows: &'a [MaterialTransferLedgerOperand],
    ledger_identity: &LedgerIdentity,
    duplicate_message: &'static str,
) -> Result<BTreeMap<MaterialTransferIdentity, &'a MaterialTransferLedgerOperand>, VegetationError>
{
    let mut indexed = BTreeMap::new();
    for row in rows {
        if row.identity.transaction_id != ledger_identity.transaction_id
            || row.identity.owner_id.trim().is_empty()
            || row.identity.proposal_id == 0
        {
            return Err(VegetationError::Receipt(
                "material transfer identity mismatch".into(),
            ));
        }
        validate_nonnegative(
            &[row.carbon_kg_m2, row.nitrogen_kg_m2, row.dry_matter_kg_m2],
            "material transfer operand",
        )?;
        if indexed.insert(row.identity.clone(), row).is_some() {
            return Err(VegetationError::Receipt(duplicate_message.into()));
        }
    }
    Ok(indexed)
}

fn unique_keys<K: Clone + Ord>(
    keys: &[K],
    duplicate_message: &'static str,
) -> Result<BTreeSet<K>, VegetationError> {
    let set = keys.iter().cloned().collect::<BTreeSet<_>>();
    if set.len() != keys.len() {
        return Err(VegetationError::Receipt(duplicate_message.into()));
    }
    Ok(set)
}

fn validate_nonnegative(values: &[f64], name: &'static str) -> Result<(), VegetationError> {
    if values
        .iter()
        .any(|value| !value.is_finite() || *value < 0.0)
    {
        return Err(VegetationError::Domain(name));
    }
    Ok(())
}

fn require_equal(
    ledger: &'static str,
    left: f64,
    right: f64,
    absolute_tolerance: f64,
) -> Result<(), VegetationError> {
    let scale = left.abs() + right.abs();
    let residual = left - right;
    let tolerance = absolute_tolerance + 64.0 * f64::EPSILON * scale;
    if !residual.is_finite() || residual.abs() > tolerance {
        return Err(VegetationError::Closure { ledger, residual });
    }
    Ok(())
}

fn require_closed(
    ledger: &'static str,
    residual: f64,
    absolute_tolerance: f64,
    operand_scale: f64,
) -> Result<(), VegetationError> {
    let tolerance = absolute_tolerance + 64.0 * f64::EPSILON * operand_scale.abs();
    if !residual.is_finite() || residual.abs() > tolerance {
        return Err(VegetationError::Closure { ledger, residual });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_kernel_contract::{MineralNitrogenSpecies, OccupancyId, StratumId, TileId};

    fn layer(value: &str) -> SoilLayerId {
        SoilLayerId::try_new(value).expect("valid layer")
    }

    fn water_key(layer_id: SoilLayerId) -> WaterResourceKey {
        WaterResourceKey {
            occupancy_id: OccupancyId {
                stratum_id: StratumId::try_new("overstory").expect("valid stratum"),
                tile_id: TileId::try_new("tile-a").expect("valid tile"),
            },
            layer_id,
        }
    }

    fn nitrogen_key(layer_id: SoilLayerId, species: MineralNitrogenSpecies) -> MineralNitrogenKey {
        MineralNitrogenKey { layer_id, species }
    }

    fn identity() -> LedgerIdentity {
        LedgerIdentity {
            transaction_id: TransactionId(9),
            owner_id: "coupled-transaction".into(),
            area_m2: 1.0,
            interval_s: 1.0,
        }
    }

    fn material_identity() -> MaterialTransferIdentity {
        MaterialTransferIdentity {
            transaction_id: TransactionId(9),
            owner_id: "overstory".into(),
            proposal_id: 1,
            donor: MaterialDonorClass::Leaf,
            receiver: MaterialReceiverClass::Metabolic,
        }
    }

    fn balanced() -> FiveLedgerOperands {
        let identity = identity();
        let water_key = water_key(layer("soil-1"));
        let nitrogen_key = nitrogen_key(layer("soil-1"), MineralNitrogenSpecies::Ammonium);
        let transfer = MaterialTransferLedgerOperand {
            identity: material_identity(),
            carbon_kg_m2: 0.5,
            nitrogen_kg_m2: 0.1,
            dry_matter_kg_m2: 1.0,
        };
        FiveLedgerOperands {
            water: WaterLedgerOperands {
                identity: identity.clone(),
                authoritative_use_keys: vec![water_key.clone()],
                finalized_uses: vec![WaterFinalizedUseOperand {
                    transaction_id: identity.transaction_id,
                    owner_id: "overstory".into(),
                    key: water_key,
                    maximum_authorization_kg_m2: 3.0,
                    finalized_use_kg_m2: 2.0,
                    owner_debit_kg_m2: 2.0,
                }],
                soil: vec![WaterStoreOperand {
                    layer_id: layer("soil-1"),
                    beginning_kg_m2: 10.0,
                    withdrawal_kg_m2: 2.0,
                    ending_kg_m2: 8.0,
                }],
                canopy_beginning_kg_m2: 1.0,
                precipitation_kg_m2: 1.0,
                canopy_ending_kg_m2: 1.0,
                canopy_evaporation_kg_m2: 0.2,
                throughfall_kg_m2: 0.5,
                stemflow_kg_m2: 0.1,
                drainage_kg_m2: 0.2,
            },
            energy: EnergyLedgerOperands {
                identity: identity.clone(),
                incident_shortwave_j_m2: 10.0,
                incident_longwave_j_m2: 2.0,
                reflected_shortwave_j_m2: 1.0,
                terminal_shortwave_j_m2: 1.0,
                emitted_longwave_j_m2: 2.0,
                sensible_j_m2: 2.0,
                latent_j_m2: 3.0,
                ground_or_storage_j_m2: 3.0,
            },
            carbon: CarbonLedgerOperands {
                identity: identity.clone(),
                beginning_vegetation_kg_m2: 10.0,
                gross_input_kg_m2: 2.0,
                maintenance_respiration_kg_m2: 0.2,
                growth_respiration_kg_m2: 0.3,
                exported_kg_m2: 0.5,
                ending_vegetation_kg_m2: 11.0,
            },
            nitrogen: NitrogenLedgerOperands {
                identity: identity.clone(),
                vegetation_beginning_kg_m2: 2.0,
                authoritative_mineral_keys: vec![nitrogen_key.clone()],
                mineral: vec![NitrogenStoreOperand {
                    key: nitrogen_key,
                    beginning_kg_m2: 1.0,
                    maximum_authorization_kg_m2: 0.3,
                    finalized_use_kg_m2: 0.2,
                    ending_kg_m2: 0.8,
                }],
                exported_kg_m2: 0.1,
                vegetation_ending_kg_m2: 2.1,
            },
            dry_material: DryMaterialLedgerOperands {
                identity,
                vegetation_beginning_kg_m2: 10.0,
                growth_input_kg_m2: 2.0,
                exported_kg_m2: 1.0,
                vegetation_ending_kg_m2: 11.0,
            },
            material_donor_debits: vec![transfer.clone()],
            material_receiver_credits: vec![transfer],
        }
    }

    #[test]
    fn independently_reconstructs_balanced_five_ledgers() {
        let residuals = validate_five_ledgers(&balanced()).expect("balanced ledgers");
        assert!(residuals.iter().all(|residual| residual.abs() <= 1e-15));
    }

    #[test]
    fn rejects_water_row_cross_cancellation() {
        let mut operands = balanced();
        operands.water.soil[0].ending_kg_m2 = 9.0;
        operands.water.soil.push(WaterStoreOperand {
            layer_id: layer("soil-2"),
            beginning_kg_m2: 5.0,
            withdrawal_kg_m2: 0.0,
            ending_kg_m2: 4.0,
        });
        assert!(validate_five_ledgers(&operands).is_err());
    }

    #[test]
    fn rejects_wrong_layer_duplicate_and_missing_water_keys() {
        let mut wrong_layer = balanced();
        wrong_layer.water.finalized_uses[0].key.layer_id = layer("soil-2");
        assert!(validate_five_ledgers(&wrong_layer).is_err());

        let mut wrong_occupancy = balanced();
        wrong_occupancy.water.finalized_uses[0]
            .key
            .occupancy_id
            .tile_id = TileId::try_new("tile-b").expect("valid tile");
        assert!(validate_five_ledgers(&wrong_occupancy).is_err());

        let mut duplicate = balanced();
        duplicate
            .water
            .finalized_uses
            .push(duplicate.water.finalized_uses[0].clone());
        assert!(validate_five_ledgers(&duplicate).is_err());

        let mut missing = balanced();
        missing.water.finalized_uses.clear();
        assert!(validate_five_ledgers(&missing).is_err());

        let mut duplicate_authority = balanced();
        duplicate_authority
            .water
            .authoritative_use_keys
            .push(duplicate_authority.water.authoritative_use_keys[0].clone());
        assert!(validate_five_ledgers(&duplicate_authority).is_err());
    }

    #[test]
    fn rejects_water_authorization_as_owner_debit() {
        let mut operands = balanced();
        operands.water.finalized_uses[0].owner_debit_kg_m2 =
            operands.water.finalized_uses[0].maximum_authorization_kg_m2;
        operands.water.soil[0].withdrawal_kg_m2 = 3.0;
        operands.water.soil[0].ending_kg_m2 = 7.0;
        assert!(validate_five_ledgers(&operands).is_err());
    }

    #[test]
    fn rejects_nitrogen_cross_species_cancellation() {
        let mut operands = balanced();
        operands.nitrogen.mineral[0].ending_kg_m2 = 0.7;
        let nitrate = nitrogen_key(layer("soil-1"), MineralNitrogenSpecies::Nitrate);
        operands
            .nitrogen
            .authoritative_mineral_keys
            .push(nitrate.clone());
        operands.nitrogen.mineral.push(NitrogenStoreOperand {
            key: nitrate,
            beginning_kg_m2: 1.0,
            maximum_authorization_kg_m2: 0.3,
            finalized_use_kg_m2: 0.3,
            ending_kg_m2: 0.8,
        });
        assert!(validate_five_ledgers(&operands).is_err());
    }

    #[test]
    fn rejects_wrong_species_duplicate_and_missing_nitrogen_keys() {
        let mut wrong_layer = balanced();
        wrong_layer.nitrogen.mineral[0].key.layer_id = layer("soil-2");
        assert!(validate_five_ledgers(&wrong_layer).is_err());

        let mut wrong_species = balanced();
        wrong_species.nitrogen.mineral[0].key.species = MineralNitrogenSpecies::Nitrate;
        assert!(validate_five_ledgers(&wrong_species).is_err());

        let mut duplicate = balanced();
        duplicate
            .nitrogen
            .mineral
            .push(duplicate.nitrogen.mineral[0].clone());
        assert!(validate_five_ledgers(&duplicate).is_err());

        let mut missing = balanced();
        missing.nitrogen.mineral.clear();
        assert!(validate_five_ledgers(&missing).is_err());

        let mut duplicate_authority = balanced();
        duplicate_authority
            .nitrogen
            .authoritative_mineral_keys
            .push(duplicate_authority.nitrogen.authoritative_mineral_keys[0].clone());
        assert!(validate_five_ledgers(&duplicate_authority).is_err());
    }

    #[test]
    fn rejects_nitrogen_authorization_as_inventory_debit() {
        let mut operands = balanced();
        operands.nitrogen.mineral[0].ending_kg_m2 = 0.7;
        assert!(validate_five_ledgers(&operands).is_err());
    }

    #[test]
    fn rejects_carbon_substituted_for_dry_material_credit() {
        let mut operands = balanced();
        operands.material_receiver_credits[0].dry_matter_kg_m2 =
            operands.material_receiver_credits[0].carbon_kg_m2;
        assert!(validate_five_ledgers(&operands).is_err());
    }

    #[test]
    fn rejects_missing_and_duplicate_material_receivers() {
        let mut missing = balanced();
        missing.material_receiver_credits.clear();
        assert!(validate_five_ledgers(&missing).is_err());

        let mut duplicate = balanced();
        duplicate
            .material_receiver_credits
            .push(duplicate.material_receiver_credits[0].clone());
        assert!(validate_five_ledgers(&duplicate).is_err());
    }
}
