//! Uncommitted V7 phenology, turnover, and mineral-nitrogen preallocation.
//!
//! This phase deliberately stops before persistent growth allocation and has
//! no conversion to a public candidate or commit surface.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{ResourceOwnerId, StratumId, TransactionId};

use crate::VegetationError;
use crate::carbon_nitrogen::{
    CarbonOffer, CnParameters, GrowthFinalization, GrowthNitrogenReceipt, MaterialTransferAmounts,
    PhenologyMode, PhenologyUpdate, RootRespirationOperand, advance_phenology, advance_turnover,
    carbon_offer, finalize_growth, maintenance_respiration, nitrogen_demand,
};
use crate::carbon_phase::StratumCarbonOperands;
use crate::config::{PhenologyType, StratumConfiguration, VegetationConfiguration};
use crate::nitrogen_protocol::{
    MineralNitrogenFinalization, MineralNitrogenFinalizedUse, MineralNitrogenMaximumAuthorization,
    PotentialMineralNitrogenRequest, PotentialMineralNitrogenRequestBatch,
    ValidatedMineralNitrogenAuthorizations,
};
use crate::transaction::{CoupledOwnedState, NitrogenArbiter, SnowFreeForcing, StratumSharedState};
use crate::water_phase::UncommittedWaterPhase;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StratumPreallocation {
    pub candidate_after_growth: StratumSharedState,
    pub potential_carbon_offer: CarbonOffer,
    pub final_carbon_offer: CarbonOffer,
    pub final_carbon_operands: StratumCarbonOperands,
    pub final_maintenance_respiration_kg_c_m2: f64,
    pub potential_request_batch: PotentialMineralNitrogenRequestBatch,
    pub nitrogen_finalization: MineralNitrogenFinalization,
    pub growth_finalization: GrowthFinalization,
    pub material_transfers: Vec<MaterialTransferAmounts>,
}

/// Sealed result of one global mineral-N arbitration against prepared strata.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UncommittedNitrogenPhase {
    transaction_id: TransactionId,
    requests: Vec<PotentialMineralNitrogenRequest>,
    authorizations: Vec<MineralNitrogenMaximumAuthorization>,
    finalized_uses: Vec<MineralNitrogenFinalizedUse>,
    strata: BTreeMap<StratumId, StratumPreallocation>,
    source_water_phase: Box<UncommittedWaterPhase>,
}

impl UncommittedNitrogenPhase {
    pub(crate) fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    pub(crate) fn requests(&self) -> &[PotentialMineralNitrogenRequest] {
        &self.requests
    }

    pub(crate) fn authorizations(&self) -> &[MineralNitrogenMaximumAuthorization] {
        &self.authorizations
    }

    pub(crate) fn finalized_uses(&self) -> &[MineralNitrogenFinalizedUse] {
        &self.finalized_uses
    }

    pub(crate) fn strata(&self) -> &BTreeMap<StratumId, StratumPreallocation> {
        &self.strata
    }

    pub(crate) fn source_water_phase(&self) -> &UncommittedWaterPhase {
        &self.source_water_phase
    }
}

struct PreparedStratum {
    state: StratumSharedState,
    parameters: CnParameters,
    potential_offer: CarbonOffer,
    final_offer: CarbonOffer,
    final_maintenance_respiration: f64,
    final_demand: f64,
    request_batch: PotentialMineralNitrogenRequestBatch,
    transfers: Vec<MaterialTransferAmounts>,
}

/// Prepare V7 phenology/turnover, authorize all exact `(layer, species)`
/// requests once, and finalize the N protocol without mutating beginning state.
#[allow(clippy::too_many_lines)]
pub(crate) fn execute_uncommitted_nitrogen_phase(
    configuration: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    water_phase: &UncommittedWaterPhase,
    nitrogen: &dyn NitrogenArbiter,
) -> Result<UncommittedNitrogenPhase, VegetationError> {
    configuration.validate()?;
    beginning.validate(configuration)?;
    if water_phase.beginning_state_sha256() != beginning.state_sha256 {
        return Err(VegetationError::Receipt(
            "persistent phase beginning-state identity".into(),
        ));
    }
    let transaction_id = water_phase.transaction_id();
    if transaction_id.0 != beginning.last_transaction_id + 1 {
        return Err(VegetationError::Receipt(
            "persistent phase transaction lineage".into(),
        ));
    }

    let potential_carbon = water_phase.potential_stratum_carbon_operands()?;
    let final_carbon = water_phase.final_stratum_carbon_operands()?;
    let configured_ids = configuration
        .strata
        .iter()
        .map(|stratum| stratum.stratum_id.clone())
        .collect::<BTreeSet<_>>();
    if potential_carbon.keys().cloned().collect::<BTreeSet<_>>() != configured_ids
        || final_carbon.keys().cloned().collect::<BTreeSet<_>>() != configured_ids
    {
        return Err(VegetationError::Receipt(
            "persistent phase carbon stratum identity".into(),
        ));
    }

    let mut prepared = BTreeMap::<StratumId, PreparedStratum>::new();
    let mut all_requests = Vec::new();
    for stratum in &configuration.strata {
        let beginning_stratum = beginning
            .strata
            .get(&stratum.stratum_id)
            .ok_or(VegetationError::Domain("persistent stratum identity"))?;
        let mut candidate = beginning_stratum.clone();
        let parameters = cn_parameters(stratum)?;
        let phenology = advance_configured_phenology(
            &mut candidate,
            stratum,
            forcing.gsi,
            configuration.dt_s,
            &parameters,
        )?;
        apply_phenology_update(&mut candidate, &phenology);
        let mut transfers = phenology.transfers;
        transfers.extend(advance_turnover(
            &mut candidate.tissues,
            configuration.dt_s,
            &parameters,
        )?);
        candidate.retranslocation_n += phenology.retranslocated_n;

        let root_operands = root_respiration_operands(stratum, forcing)?;
        let potential = potential_carbon
            .get(&stratum.stratum_id)
            .ok_or(VegetationError::Domain("potential carbon identity"))?;
        let final_accepted = final_carbon
            .get(&stratum.stratum_id)
            .ok_or(VegetationError::Domain("final carbon identity"))?;
        if potential.advanced_t10_k.to_bits() != final_accepted.advanced_t10_k.to_bits() {
            return Err(VegetationError::Receipt(
                "persistent phase potential/final T10 identity".into(),
            ));
        }
        candidate.t10_k = final_accepted.advanced_t10_k;
        let potential_maintenance = maintenance_respiration(
            &candidate.tissues,
            potential.accepted_leaf_respiration_kg_c_m2,
            forcing.air_temperature_k,
            &root_operands,
            stratum.mr_base_kgc_per_kgn_s,
            stratum.mr_q10,
            configuration.dt_s,
        )?;
        let final_maintenance = maintenance_respiration(
            &candidate.tissues,
            final_accepted.accepted_leaf_respiration_kg_c_m2,
            forcing.air_temperature_k,
            &root_operands,
            stratum.mr_base_kgc_per_kgn_s,
            stratum.mr_q10,
            configuration.dt_s,
        )?;
        let potential_offer = carbon_offer(
            potential.gross_primary_production_kg_c_m2,
            potential_maintenance,
            candidate.xs_c,
            candidate.nsc_c,
            configuration.dt_s,
            stratum.xs_recovery_days,
        )?;
        let final_offer = carbon_offer(
            final_accepted.gross_primary_production_kg_c_m2,
            final_maintenance,
            candidate.xs_c,
            candidate.nsc_c,
            configuration.dt_s,
            stratum.xs_recovery_days,
        )?;
        let potential_demand = nitrogen_demand(
            potential_offer.offer,
            candidate.retranslocation_n,
            &parameters,
        )?;
        let owner_id = ResourceOwnerId::try_new(stratum.stratum_id.as_str())
            .map_err(|error| VegetationError::Schema(error.to_string()))?;
        let request_batch = PotentialMineralNitrogenRequestBatch::try_from_stratum_configuration(
            transaction_id,
            owner_id,
            stratum,
            potential_demand.demand,
            candidate.retranslocation_n,
        )
        .map_err(VegetationError::from)?;
        let final_demand =
            nitrogen_demand(final_offer.offer, candidate.retranslocation_n, &parameters)?.demand;
        all_requests.extend_from_slice(request_batch.requests());
        prepared.insert(
            stratum.stratum_id.clone(),
            PreparedStratum {
                state: candidate,
                parameters,
                potential_offer,
                final_offer,
                final_maintenance_respiration: final_maintenance,
                final_demand,
                request_batch,
                transfers,
            },
        );
    }

    // One call is the same-snapshot competition boundary for every stratum.
    let authorizations = if all_requests.is_empty() {
        Vec::new()
    } else {
        nitrogen.authorize(&all_requests)?
    };
    let mut by_owner = BTreeMap::<ResourceOwnerId, Vec<_>>::new();
    for authorization in &authorizations {
        by_owner
            .entry(authorization.owner_id.clone())
            .or_default()
            .push(authorization.clone());
    }

    let mut strata = BTreeMap::new();
    let mut finalized_uses = Vec::new();
    for (stratum_id, mut item) in prepared {
        let owner_id = item.request_batch.owner_id().clone();
        let owner_authorizations = by_owner.remove(&owner_id).ok_or_else(|| {
            VegetationError::Receipt("mineral-nitrogen authorization owner absent".into())
        })?;
        let validated = ValidatedMineralNitrogenAuthorizations::try_new(
            &item.request_batch,
            owner_authorizations,
        )
        .map_err(VegetationError::from)?;
        let nitrogen_finalization = validated
            .finalize(item.final_demand)
            .map_err(VegetationError::from)?;
        let growth_finalization = finalize_growth(
            &mut item.state.tissues,
            &item.final_offer,
            GrowthNitrogenReceipt {
                final_total_demand: nitrogen_finalization.final_total_demand,
                internal_use: nitrogen_finalization.internal_use,
                external_use: nitrogen_finalization.external_use,
                internal_remaining: nitrogen_finalization.internal_remaining,
            },
            &item.parameters,
        )?;
        item.state.retranslocation_n = nitrogen_finalization.internal_remaining;
        item.state.nsc_c = growth_finalization.nsc_next;
        item.state.xs_c = growth_finalization.xs_next;
        finalized_uses.extend_from_slice(&nitrogen_finalization.finalized_uses);
        let final_carbon_operands = *final_carbon
            .get(&stratum_id)
            .ok_or(VegetationError::Domain("final carbon identity"))?;
        strata.insert(
            stratum_id,
            StratumPreallocation {
                candidate_after_growth: item.state,
                potential_carbon_offer: item.potential_offer,
                final_carbon_offer: item.final_offer,
                final_carbon_operands,
                final_maintenance_respiration_kg_c_m2: item.final_maintenance_respiration,
                potential_request_batch: item.request_batch,
                nitrogen_finalization,
                growth_finalization,
                material_transfers: item.transfers,
            },
        );
    }
    if !by_owner.is_empty() {
        return Err(VegetationError::Receipt(
            "unexpected mineral-nitrogen authorization owner".into(),
        ));
    }

    Ok(UncommittedNitrogenPhase {
        transaction_id,
        requests: all_requests,
        authorizations,
        finalized_uses,
        strata,
        source_water_phase: Box::new(water_phase.clone()),
    })
}

fn cn_parameters(stratum: &StratumConfiguration) -> Result<CnParameters, VegetationError> {
    let fractions = |name: &str| -> Result<[f64; 3], VegetationError> {
        Ok([
            *stratum.litter_metabolic_fraction.get(name).ok_or_else(|| {
                VegetationError::Schema("missing litter metabolic fraction".into())
            })?,
            *stratum.litter_cellulose_fraction.get(name).ok_or_else(|| {
                VegetationError::Schema("missing litter cellulose fraction".into())
            })?,
            *stratum
                .litter_lignin_fraction
                .get(name)
                .ok_or_else(|| VegetationError::Schema("missing litter lignin fraction".into()))?,
        ])
    };
    Ok(CnParameters {
        growth_respiration_ratio: stratum.growth_resp_ratio_g1,
        a1_froot_leaf: stratum.alloc_froot_leaf_a1,
        a2_croot_stem: stratum.alloc_croot_stem_a2,
        a3_stem_leaf: stratum.alloc_stem_leaf_a3,
        a4_livewood_fraction: stratum.livewood_fraction_a4,
        current_growth_fraction: stratum.current_growth_fraction,
        cn_leaf: stratum.cn_leaf,
        cn_leaf_litter: stratum.cn_leaf_litter,
        cn_froot: stratum.cn_froot,
        cn_livewood: stratum.cn_livewood,
        cn_deadwood: stratum.cn_deadwood,
        drymatter_carbon_fraction: stratum.drymatter_carbon_fraction,
        xs_recovery_days: stratum.xs_recovery_days,
        leaf_lifetime_s: stratum.leaf_lifetime_s,
        froot_lifetime_s: stratum.froot_lifetime_s,
        livewood_turnover_s: stratum.livewood_turnover_s,
        mortality_rate_s1: stratum.mortality_rate_s1,
        leaf_litter_fractions: fractions("leaf")?,
        froot_litter_fractions: fractions("fine_root")?,
    })
}

fn advance_configured_phenology(
    state: &mut StratumSharedState,
    stratum: &StratumConfiguration,
    gsi: f64,
    dt_s: f64,
    parameters: &CnParameters,
) -> Result<PhenologyUpdate, VegetationError> {
    let seasonal = stratum.phenology_type == PhenologyType::SeasonalDeciduous;
    advance_phenology(
        &mut state.tissues,
        if seasonal {
            PhenologyMode::SeasonalDeciduous
        } else {
            PhenologyMode::Evergreen
        },
        state.phase,
        state.onset_remaining_s,
        state.offset_remaining_s,
        state.previous_gsi,
        gsi,
        dt_s,
        stratum.gsi_on_threshold.unwrap_or(f64::NAN),
        stratum.gsi_off_threshold.unwrap_or(f64::NAN),
        stratum.onset_duration_s.unwrap_or(f64::NAN),
        stratum.offset_duration_s.unwrap_or(f64::NAN),
        parameters,
    )
}

fn apply_phenology_update(state: &mut StratumSharedState, update: &PhenologyUpdate) {
    state.phase = update.phase;
    state.onset_remaining_s = update.onset_remaining_s;
    state.offset_remaining_s = update.offset_remaining_s;
    state.previous_gsi = update.previous_gsi;
}

fn root_respiration_operands(
    stratum: &StratumConfiguration,
    forcing: &SnowFreeForcing,
) -> Result<Vec<RootRespirationOperand>, VegetationError> {
    let temperatures = forcing
        .soil_layers
        .iter()
        .map(|layer| (&layer.layer_id, layer.temperature_k))
        .collect::<BTreeMap<_, _>>();
    stratum
        .root_layers
        .iter()
        .map(|root| {
            Ok(RootRespirationOperand {
                layer_id: root.layer_id.clone(),
                temperature_k: *temperatures
                    .get(&root.layer_id)
                    .ok_or(VegetationError::Domain("root respiration layer identity"))?,
                nitrogen_fraction: root.mineral_n_root_fraction,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::*;

    #[test]
    fn configuration_mapping_preserves_exact_e17_e19_parameters() {
        let configuration: VegetationConfiguration = serde_json::from_slice(include_bytes!(
            "../../../tests/fixtures/c3_woody_v5_diagnostic_configuration.json"
        ))
        .expect("strict structural predecessor fixture");
        let source = &configuration.strata[0];
        let mapped = cn_parameters(source).expect("complete mapping");
        assert_eq!(mapped.growth_respiration_ratio, source.growth_resp_ratio_g1);
        assert_eq!(
            mapped.current_growth_fraction,
            source.current_growth_fraction
        );
        assert_eq!(mapped.leaf_litter_fractions, [0.2, 0.3, 0.5]);
        assert_eq!(mapped.froot_litter_fractions, [0.25, 0.35, 0.4]);
    }

    #[test]
    fn nitrogen_resource_identity_is_layer_and_species() {
        let ammonium = openwepp_kernel_contract::MineralNitrogenKey {
            layer_id: openwepp_kernel_contract::SoilLayerId::try_new("soil-1").expect("layer"),
            species: openwepp_kernel_contract::MineralNitrogenSpecies::Ammonium,
        };
        let nitrate = openwepp_kernel_contract::MineralNitrogenKey {
            species: openwepp_kernel_contract::MineralNitrogenSpecies::Nitrate,
            ..ammonium.clone()
        };
        assert_ne!(ammonium, nitrate);
    }

    #[test]
    fn e17_root_respiration_uses_root_n_not_hydraulic_fraction() {
        let configuration: VegetationConfiguration = serde_json::from_slice(include_bytes!(
            "../../../tests/fixtures/c3_woody_v5_diagnostic_configuration.json"
        ))
        .expect("strict structural predecessor fixture");
        let mut stratum = configuration.strata[0].clone();
        let first_layer = stratum.root_layers[0].layer_id.clone();
        stratum.root_layers[0].root_fraction = 0.9;
        stratum.root_layers[0].mineral_n_root_fraction = 0.25;
        let second_layer =
            openwepp_kernel_contract::SoilLayerId::try_new("soil-2").expect("second layer");
        let mut second_root = stratum.root_layers[0].clone();
        second_root.layer_id = second_layer.clone();
        second_root.root_fraction = 0.1;
        second_root.mineral_n_root_fraction = 0.75;
        stratum.root_layers.push(second_root);
        let forcing = SnowFreeForcing {
            air_temperature_k: 295.0,
            pressure_pa: 101_325.0,
            co2_pa: 42.0,
            vapor_pressure_deficit_kpa: 1.0,
            wind_m_s: 2.0,
            rain_kg_m2: 0.0,
            direct_par_w_m2: 0.0,
            diffuse_par_w_m2: 0.0,
            direct_nir_w_m2: 0.0,
            diffuse_nir_w_m2: 0.0,
            solar_zenith_cosine: 0.5,
            ground_albedo_vis: 0.1,
            ground_albedo_nir: 0.2,
            longwave_down_w_m2: 300.0,
            longwave_up_w_m2: 300.0,
            specific_humidity: 0.01,
            reference_height_m: 20.0,
            soil_layers: vec![
                crate::transaction::SoilLayerForcing {
                    layer_id: first_layer,
                    water_beginning_kg_m2: 1.0,
                    matric_potential_mm: -1.0,
                    hydraulic_conductivity_mm_s: 1.0,
                    root_path_length_mm: 1.0,
                    gravity_root_mm: 0.0,
                    temperature_k: 290.0,
                    accessible: true,
                    frozen: false,
                },
                crate::transaction::SoilLayerForcing {
                    layer_id: second_layer,
                    water_beginning_kg_m2: 1.0,
                    matric_potential_mm: -1.0,
                    hydraulic_conductivity_mm_s: 1.0,
                    root_path_length_mm: 1.0,
                    gravity_root_mm: 0.0,
                    temperature_k: 300.0,
                    accessible: true,
                    frozen: false,
                },
            ],
            gsi: 1.0,
        };
        let operands = root_respiration_operands(&stratum, &forcing).expect("root-N operands");
        assert_eq!(operands[0].nitrogen_fraction.to_bits(), 0.25_f64.to_bits());
        assert_eq!(operands[1].nitrogen_fraction.to_bits(), 0.75_f64.to_bits());
        assert_ne!(operands[0].nitrogen_fraction.to_bits(), 0.9_f64.to_bits());
    }
}
