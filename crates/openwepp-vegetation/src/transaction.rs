use std::collections::BTreeMap;

use openwepp_kernel_contract::{
    FinalizedUse, MaximumAuthorization, MineralNitrogenKey, ResourceRequest, SoilLayerId,
    TransactionId, WaterResourceKey, validate_maximum_authorization, validate_resource_protocol,
};
use serde::{Deserialize, Serialize};

use crate::carbon_nitrogen::{
    CnParameters, ElementPool, MaterialTransfer, PhenologyMode, Tissue, TissuePool,
    advance_phenology, advance_turnover, carbon_offer, finalize_growth, gpp_kg_c,
    maintenance_respiration, nitrogen_demand, update_t10,
};
use crate::energy::{
    CanopyEnergyInput, CanopyEnergySolution, LATENT_HEAT_VAPORIZATION, LeafClassEnergyInput,
    STEFAN_BOLTZMANN, leaf_boundary_conductance, neutral_resistance, solve_canopy_energy,
};
use crate::hydraulics::{HydraulicInput, HydraulicLayerInput, HydraulicSolution, solve_hydraulics};
use crate::interception::{InterceptionInput, InterceptionResult, liquid_interception};
use crate::ledger::{
    CarbonLedgerOperands, DryMaterialLedgerOperands, EnergyLedgerOperands, FiveLedgerOperands,
    LedgerIdentity, NitrogenLedgerOperands, NitrogenStoreOperand, WaterLedgerOperands,
    WaterStoreOperand,
};
use crate::photosynthesis::FvcbInput;
use crate::radiation::{ColumnLayer, TwoStreamResult, solve_column};
use crate::{
    MODEL_SHA256, ModelDefinition, PhenologyType, StratumConfiguration, VegetationConfiguration,
    VegetationError,
};
use openwepp_kernel_contract::{MineralNitrogenSpecies, ResourceAmountBasis, ResourceOwnerId};
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PhenologyPhase {
    Dormant,
    Onset,
    Active,
    Offset,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StratumState {
    pub canopy_liquid: f64,
    pub psi_root_mm: f64,
    pub psi_stem_mm: f64,
    pub psi_sun_mm: f64,
    pub psi_shade_mm: f64,
    pub tissues: BTreeMap<Tissue, TissuePool>,
    pub retranslocation_n: f64,
    pub nsc_c: f64,
    pub xs_c: f64,
    pub standing_dead: ElementPool,
    pub standing_dead_dm: f64,
    pub phase: PhenologyPhase,
    pub onset_remaining_s: f64,
    pub offset_remaining_s: f64,
    pub previous_leaf_offset_flux: f64,
    pub previous_root_offset_flux: f64,
    pub previous_gsi: f64,
    pub pending_transfers: Vec<MaterialTransfer>,
    pub t10_k: f64,
    pub leaf_area: f64,
    pub root_area: f64,
    pub stem_area: f64,
    pub last_transaction_id: u128,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoupledOwnedState {
    pub model_definition_sha256: String,
    pub configuration_sha256: String,
    pub state_sha256: String,
    pub strata: BTreeMap<String, StratumState>,
    pub last_transaction_id: u128,
}

impl CoupledOwnedState {
    pub fn parse_strict(bytes: &[u8]) -> Result<Self, VegetationError> {
        let value = serde_json::from_slice(bytes)
            .map_err(|error| VegetationError::Schema(error.to_string()))?;
        Self::validate(&value)?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), VegetationError> {
        if self.model_definition_sha256.len() != 64
            || self.configuration_sha256.len() != 64
            || self.state_sha256.len() != 64
        {
            return Err(VegetationError::Domain("state identity"));
        }
        if self.state_sha256 != self.canonical_sha256()? {
            return Err(VegetationError::Receipt(
                "state digest does not match canonical bytes".into(),
            ));
        }
        for state in self.strata.values() {
            let scalars = [
                state.canopy_liquid,
                state.psi_root_mm,
                state.psi_stem_mm,
                state.psi_sun_mm,
                state.psi_shade_mm,
                state.retranslocation_n,
                state.nsc_c,
                state.xs_c,
                state.standing_dead.carbon,
                state.standing_dead.nitrogen,
                state.standing_dead_dm,
                state.onset_remaining_s,
                state.offset_remaining_s,
                state.previous_leaf_offset_flux,
                state.previous_root_offset_flux,
                state.previous_gsi,
                state.t10_k,
                state.leaf_area,
                state.root_area,
                state.stem_area,
            ];
            if scalars.iter().any(|value| !value.is_finite())
                || state.canopy_liquid < 0.0
                || state.retranslocation_n < 0.0
                || state.nsc_c < 0.0
                || state.leaf_area < 0.0
                || state.root_area < 0.0
                || state.stem_area < 0.0
                || state.t10_k <= 0.0
                || !(0.0..=1.0).contains(&state.previous_gsi)
            {
                return Err(VegetationError::Domain("complete stratum state"));
            }
            let required = [
                Tissue::Leaf,
                Tissue::FineRoot,
                Tissue::LiveStem,
                Tissue::DeadStem,
                Tissue::LiveCoarseRoot,
                Tissue::DeadCoarseRoot,
            ];
            if state.tissues.len() != required.len()
                || required
                    .iter()
                    .any(|tissue| !state.tissues.contains_key(tissue))
            {
                return Err(VegetationError::Domain("six-tissue identity"));
            }
            for transfer in &state.pending_transfers {
                if transfer.transaction_id == 0
                    || transfer.owner_id.trim().is_empty()
                    || [transfer.carbon, transfer.nitrogen, transfer.dry_matter]
                        .iter()
                        .any(|value| !value.is_finite() || *value < 0.0)
                {
                    return Err(VegetationError::Domain("pending material transfer"));
                }
            }
            for pool in state.tissues.values() {
                for value in [
                    pool.display.carbon,
                    pool.display.nitrogen,
                    pool.storage.carbon,
                    pool.storage.nitrogen,
                    pool.transfer.carbon,
                    pool.transfer.nitrogen,
                ] {
                    if !value.is_finite() || value < 0.0 {
                        return Err(VegetationError::Domain("tissue pool"));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn canonical_sha256(&self) -> Result<String, VegetationError> {
        let mut canonical = self.clone();
        canonical.configuration_sha256.clear();
        canonical.state_sha256.clear();
        let bytes = serde_json::to_vec(&canonical)
            .map_err(|error| VegetationError::Schema(error.to_string()))?;
        Ok(format!("{:x}", Sha256::digest(bytes)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SnowFreeForcing {
    pub air_temperature_k: f64,
    pub pressure_pa: f64,
    pub co2_pa: f64,
    pub vapor_pressure_deficit_kpa: f64,
    pub wind_m_s: f64,
    pub rain_kg_m2: f64,
    pub direct_par_w_m2: f64,
    pub diffuse_par_w_m2: f64,
    pub direct_nir_w_m2: f64,
    pub diffuse_nir_w_m2: f64,
    pub solar_zenith_cosine: f64,
    pub ground_albedo_vis: f64,
    pub ground_albedo_nir: f64,
    pub longwave_down_w_m2: f64,
    pub longwave_up_w_m2: f64,
    pub specific_humidity: f64,
    pub reference_height_m: f64,
    pub soil_layers: Vec<SoilLayerForcing>,
    pub gsi: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SoilLayerForcing {
    pub layer_id: SoilLayerId,
    pub water_beginning_kg_m2: f64,
    pub matric_potential_mm: f64,
    pub hydraulic_conductivity_mm_s: f64,
    pub root_path_length_mm: f64,
    pub gravity_root_mm: f64,
    pub temperature_k: f64,
    pub accessible: bool,
    pub frozen: bool,
}

pub type WaterRequest = ResourceRequest<WaterResourceKey, f64>;
pub type WaterAuthorization = MaximumAuthorization<WaterResourceKey, f64>;
pub type WaterUse = FinalizedUse<WaterResourceKey, f64>;
pub type NitrogenRequest = ResourceRequest<MineralNitrogenKey, f64>;
pub type NitrogenAuthorization = MaximumAuthorization<MineralNitrogenKey, f64>;
pub type NitrogenUse = FinalizedUse<MineralNitrogenKey, f64>;

pub trait WaterArbiter {
    fn beginning_amount(&self, key: &WaterResourceKey) -> Result<f64, VegetationError>;
    fn authorize(
        &self,
        requests: &[WaterRequest],
    ) -> Result<Vec<WaterAuthorization>, VegetationError>;
}
pub trait NitrogenArbiter {
    fn beginning_amount(&self, key: &MineralNitrogenKey) -> Result<f64, VegetationError>;
    fn authorize(
        &self,
        requests: &[NitrogenRequest],
    ) -> Result<Vec<NitrogenAuthorization>, VegetationError>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExecutionDiagnostics {
    pub transaction_id: TransactionId,
    pub solver_iterations: u32,
    pub normalized_residuals: Vec<f64>,
    pub active_bounds: Vec<String>,
    pub authorization_activity: bool,
    pub temperature_step_k: f64,
    pub potential_step_mm: f64,
    pub backtracking_count: u32,
    pub wet_store_cap_active: bool,
    pub gas_hydraulic_mismatch_kg_m2_s: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EnergyOwnerOperands {
    pub incident_shortwave_w_m2: f64,
    pub reflected_shortwave_w_m2: f64,
    pub terminal_shortwave_w_m2: f64,
    pub incident_longwave_j_m2: f64,
    pub emitted_longwave_j_m2: f64,
    pub sensible_j_m2: f64,
    pub transpiration_kg_m2: f64,
    pub wet_phase_change_kg_m2: f64,
    pub interval_s: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CoupledCandidate {
    beginning_state_sha256: String,
    state: CoupledOwnedState,
    water_requests: Vec<WaterRequest>,
    water_authorizations: Vec<WaterAuthorization>,
    water_uses: Vec<WaterUse>,
    nitrogen_requests: Vec<NitrogenRequest>,
    nitrogen_authorizations: Vec<NitrogenAuthorization>,
    nitrogen_uses: Vec<NitrogenUse>,
    material_transfers: Vec<MaterialTransfer>,
    ledger_operands: crate::ledger::FiveLedgerOperands,
    energy_owner_operands: EnergyOwnerOperands,
    diagnostics: ExecutionDiagnostics,
}
impl CoupledCandidate {
    #[must_use]
    pub fn transaction_id(&self) -> TransactionId {
        self.diagnostics.transaction_id
    }
    #[must_use]
    pub fn water_uses(&self) -> &[WaterUse] {
        &self.water_uses
    }
    #[must_use]
    pub fn nitrogen_protocol(
        &self,
    ) -> (&[NitrogenRequest], &[NitrogenAuthorization], &[NitrogenUse]) {
        (
            &self.nitrogen_requests,
            &self.nitrogen_authorizations,
            &self.nitrogen_uses,
        )
    }
    #[must_use]
    pub fn material_transfers(&self) -> &[MaterialTransfer] {
        &self.material_transfers
    }
    #[must_use]
    pub fn ledger_operands(&self) -> &FiveLedgerOperands {
        &self.ledger_operands
    }
    #[must_use]
    pub fn energy_owner_operands(&self) -> &EnergyOwnerOperands {
        &self.energy_owner_operands
    }
    #[must_use]
    pub fn water_protocol(&self) -> (&[WaterRequest], &[WaterAuthorization], &[WaterUse]) {
        (
            &self.water_requests,
            &self.water_authorizations,
            &self.water_uses,
        )
    }
    #[must_use]
    pub fn diagnostics(&self) -> &ExecutionDiagnostics {
        &self.diagnostics
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CommitReceipt {
    pub transaction_id: TransactionId,
    pub ending_state_sha256: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailurePoint {
    Validation,
    Radiation,
    Interception,
    PotentialCoupledSolve,
    WaterAuthorization,
    CappedResolve,
    NitrogenRequest,
    NitrogenAuthorization,
    Allocation,
    ReceiverConstruction,
    ClosureValidation,
    BeforeCommit,
    OwnerValidation,
}

#[allow(clippy::too_many_lines)]
pub fn execute_candidate(
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    water: &dyn WaterArbiter,
    nitrogen: &dyn NitrogenArbiter,
) -> Result<CoupledCandidate, VegetationError> {
    execute_candidate_with_failure(model, config, beginning, forcing, water, nitrogen, None)
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn execute_candidate_with_failure(
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    water: &dyn WaterArbiter,
    nitrogen: &dyn NitrogenArbiter,
    failure: Option<FailurePoint>,
) -> Result<CoupledCandidate, VegetationError> {
    validate_execution(model, config, beginning, forcing)?;
    validate_e04_topology_authority(config)?;
    reject_at(failure, FailurePoint::Validation)?;
    let transaction_id = TransactionId(
        beginning
            .last_transaction_id
            .checked_add(1)
            .ok_or(VegetationError::Domain("transaction identity"))?,
    );
    let radiation = radiation_by_stratum(config, beginning, forcing)?;
    let rain_routing = rain_by_stratum(config, beginning, forcing)?;
    reject_at(failure, FailurePoint::Radiation)?;
    let mut prepared = Vec::with_capacity(config.strata.len());
    let mut water_requests = Vec::new();
    for stratum in &config.strata {
        let state = beginning
            .strata
            .get(&stratum.stratum_id)
            .ok_or(VegetationError::Domain("missing stratum state"))?
            .clone();
        let (vis, nir) = radiation
            .get(&stratum.stratum_id)
            .ok_or(VegetationError::Domain("missing topology radiation"))?;
        let (incident_rain, terminal_weight) = rain_routing
            .get(&stratum.stratum_id)
            .copied()
            .ok_or(VegetationError::Domain("missing topology rain routing"))?;
        let item = prepare_stratum(
            stratum,
            state,
            config,
            forcing,
            transaction_id,
            failure,
            *vis,
            *nir,
            incident_rain,
            terminal_weight,
        )?;
        water_requests.extend(item.water_requests.iter().cloned());
        prepared.push(item);
    }
    let water_authorizations = water.authorize(&water_requests)?;
    reject_at(failure, FailurePoint::WaterAuthorization)?;
    if water_authorizations.len() != water_requests.len() {
        return Err(VegetationError::Receipt("water authorization shape".into()));
    }
    for (request, authorization) in water_requests.iter().zip(&water_authorizations) {
        validate_maximum_authorization(request, authorization)
            .map_err(|error| VegetationError::Receipt(format!("{error:?}")))?;
    }
    let mut physical = Vec::with_capacity(prepared.len());
    let mut nitrogen_requests = Vec::new();
    let mut water_uses = Vec::new();
    let mut water_offset = 0;
    for item in prepared {
        let count = item.water_requests.len();
        let caps = water_authorizations[water_offset..water_offset + count]
            .iter()
            .map(|authorization| authorization.amount)
            .collect::<Vec<_>>();
        let finalized = finish_physical(item, config, forcing, &caps, transaction_id)?;
        reject_at(failure, FailurePoint::CappedResolve)?;
        water_uses.extend(finalized.water_uses.iter().cloned());
        nitrogen_requests.extend(finalized.nitrogen_requests.iter().cloned());
        physical.push(finalized);
        water_offset += count;
    }
    let nitrogen_authorizations = nitrogen.authorize(&nitrogen_requests)?;
    reject_at(failure, FailurePoint::NitrogenRequest)?;
    reject_at(failure, FailurePoint::NitrogenAuthorization)?;
    if nitrogen_authorizations.len() != nitrogen_requests.len() {
        return Err(VegetationError::Receipt(
            "nitrogen authorization shape".into(),
        ));
    }
    for (request, authorization) in nitrogen_requests.iter().zip(&nitrogen_authorizations) {
        validate_maximum_authorization(request, authorization)
            .map_err(|error| VegetationError::Receipt(format!("{error:?}")))?;
    }
    let mut state = beginning.clone();
    let mut material_transfers = Vec::new();
    let mut nitrogen_uses = Vec::new();
    let mut nitrogen_offset = 0;
    let mut gpp = 0.0;
    let mut maintenance = 0.0;
    let mut growth_respiration = 0.0;
    let mut dry_growth = 0.0;
    let mut solver_iterations = 0;
    let mut backtracks = 0;
    let mut normalized_residuals = Vec::new();
    let mut active_bounds = Vec::new();
    let mut wet_cap = false;
    let mut gas_hydraulic_mismatch = 0.0_f64;
    let mut temperature_step_k = 0.0_f64;
    let mut potential_step_mm = 0.0_f64;
    let empty_tile_fraction = config
        .topology_tiles
        .iter()
        .filter(|tile| {
            !config
                .strata
                .iter()
                .any(|stratum| stratum.tile_ids.contains(&tile.tile_id))
        })
        .map(|tile| tile.fraction)
        .sum::<f64>();
    let mut energy_accumulator = EnergyAccumulator {
        incident_shortwave: forcing.direct_par_w_m2
            + forcing.diffuse_par_w_m2
            + forcing.direct_nir_w_m2
            + forcing.diffuse_nir_w_m2,
        terminal_shortwave: empty_tile_fraction
            * ((1.0 - forcing.ground_albedo_vis)
                * (forcing.direct_par_w_m2 + forcing.diffuse_par_w_m2)
                + (1.0 - forcing.ground_albedo_nir)
                    * (forcing.direct_nir_w_m2 + forcing.diffuse_nir_w_m2)),
        reflected_shortwave: empty_tile_fraction
            * (forcing.ground_albedo_vis * (forcing.direct_par_w_m2 + forcing.diffuse_par_w_m2)
                + forcing.ground_albedo_nir * (forcing.direct_nir_w_m2 + forcing.diffuse_nir_w_m2)),
        throughfall: empty_tile_fraction * forcing.rain_kg_m2,
        ..EnergyAccumulator::default()
    };
    for mut item in physical {
        let count = item.nitrogen_requests.len();
        let authorizations = &nitrogen_authorizations[nitrogen_offset..nitrogen_offset + count];
        let final_need = nitrogen_demand(
            item.final_offer.offer,
            item.state.retranslocation_n,
            &item.cn,
        )?
        .external_shortfall;
        let authorized_total = authorizations.iter().map(|value| value.amount).sum::<f64>();
        let use_fraction = if authorized_total == 0.0 {
            0.0
        } else if final_need >= authorized_total {
            1.0
        } else {
            final_need / authorized_total
        };
        let uses = authorizations
            .iter()
            .map(|authorization| FinalizedUse {
                transaction_id,
                owner_id: authorization.owner_id.clone(),
                key: authorization.key.clone(),
                basis: authorization.basis,
                amount: use_fraction * authorization.amount,
            })
            .collect::<Vec<_>>();
        let growth = finalize_growth(
            &mut item.state.tissues,
            &item.final_offer,
            &mut item.state.retranslocation_n,
            uses.iter().map(|value| value.amount).sum(),
            &item.cn,
        )?;
        reject_at(failure, FailurePoint::Allocation)?;
        item.state.nsc_c = growth.nsc_next;
        item.state.xs_c = growth.xs_next;
        item.state.leaf_area =
            tissue_carbon(&item.state, Tissue::Leaf)? * item.config.sla_m2_per_kg_c;
        item.state.stem_area = item.state.leaf_area * item.config.sai_relation;
        item.state.root_area =
            (item.state.leaf_area + item.state.stem_area) * item.config.root_to_leaf_area;
        item.state.t10_k = item.t10_next;
        item.state.canopy_liquid = item.interception.store1;
        item.state.psi_sun_mm = item.hydraulic.psi_sun_mm;
        item.state.psi_shade_mm = item.hydraulic.psi_shade_mm;
        item.state.psi_stem_mm = item.hydraulic.psi_stem_mm;
        item.state.psi_root_mm = item.hydraulic.psi_root_mm;
        item.state.last_transaction_id = transaction_id.0;
        item.state.previous_leaf_offset_flux = item
            .transfers
            .iter()
            .filter(|transfer| transfer.donor == openwepp_kernel_contract::MaterialDonorClass::Leaf)
            .map(|transfer| transfer.carbon)
            .sum::<f64>()
            / config.dt_s;
        item.state.previous_root_offset_flux = item
            .transfers
            .iter()
            .filter(|transfer| {
                transfer.donor == openwepp_kernel_contract::MaterialDonorClass::FineRoot
            })
            .map(|transfer| transfer.carbon)
            .sum::<f64>()
            / config.dt_s;
        item.state.pending_transfers.clone_from(&item.transfers);
        gpp += item.gpp;
        maintenance += item.maintenance;
        growth_respiration += growth.growth_respiration;
        dry_growth +=
            growth.tissue_carbon.iter().sum::<f64>() / item.config.drymatter_carbon_fraction;
        solver_iterations +=
            item.energy.diagnostics.iterations + item.hydraulic.diagnostics.iterations;
        backtracks += item.energy.diagnostics.backtracks + item.hydraulic.diagnostics.backtracks;
        normalized_residuals.push(item.energy.diagnostics.residual_norm);
        normalized_residuals.push(item.hydraulic.diagnostics.residual_norm);
        active_bounds.extend(
            item.hydraulic
                .active_caps
                .iter()
                .map(|index| format!("{}:water:{index}", item.config.stratum_id)),
        );
        wet_cap |= item.energy.wet_store_cap_active;
        gas_hydraulic_mismatch = gas_hydraulic_mismatch.max(item.gas_hydraulic_mismatch.abs());
        temperature_step_k = temperature_step_k.max(item.energy.diagnostics.step_norm);
        potential_step_mm = potential_step_mm.max(item.hydraulic.diagnostics.step_norm);
        energy_accumulator.add(&item, forcing, config.dt_s);
        material_transfers.extend(item.transfers);
        nitrogen_uses.extend(uses);
        state
            .strata
            .insert(item.config.stratum_id.clone(), item.state);
        nitrogen_offset += count;
    }
    state.last_transaction_id = transaction_id.0;
    state.state_sha256 = state_digest(&state)?;
    let ledger_operands = build_ledgers(
        config,
        beginning,
        &state,
        forcing,
        water,
        nitrogen,
        &water_uses,
        &nitrogen_uses,
        &material_transfers,
        gpp,
        maintenance,
        growth_respiration,
        dry_growth,
        &energy_accumulator,
        transaction_id,
    )?;
    reject_at(failure, FailurePoint::ReceiverConstruction)?;
    crate::ledger::validate_five_ledgers(&ledger_operands)?;
    reject_at(failure, FailurePoint::ClosureValidation)?;
    let authorization_activity = water_requests
        .iter()
        .zip(&water_authorizations)
        .any(|(request, authorization)| authorization.amount < request.amount)
        || nitrogen_requests
            .iter()
            .zip(&nitrogen_authorizations)
            .any(|(request, authorization)| authorization.amount < request.amount);
    Ok(CoupledCandidate {
        beginning_state_sha256: beginning.state_sha256.clone(),
        state,
        water_requests,
        water_authorizations,
        water_uses,
        nitrogen_requests,
        nitrogen_authorizations,
        nitrogen_uses,
        material_transfers,
        ledger_operands,
        energy_owner_operands: energy_accumulator.owner_operands(config.dt_s),
        diagnostics: ExecutionDiagnostics {
            transaction_id,
            solver_iterations,
            normalized_residuals,
            active_bounds,
            authorization_activity,
            temperature_step_k,
            potential_step_mm,
            backtracking_count: backtracks,
            wet_store_cap_active: wet_cap,
            gas_hydraulic_mismatch_kg_m2_s: gas_hydraulic_mismatch,
        },
    })
}

fn validate_e04_topology_authority(
    config: &VegetationConfiguration,
) -> Result<(), VegetationError> {
    for stratum in &config.strata {
        let coverage = stratum_coverage(config, stratum)?;
        if (coverage - 1.0).abs() > 1e-12 || stratum.tile_ids.len() != 1 {
            return Err(VegetationError::Unsupported(
                "E04 heterogeneous-tile liquid-state aggregation authority missing",
            ));
        }
    }
    Ok(())
}

#[derive(Clone)]
struct PreparedStratum {
    config: StratumConfiguration,
    state: StratumState,
    cn: CnParameters,
    energy_input: CanopyEnergyInput,
    hydraulic_input: HydraulicInput,
    potential_energy: CanopyEnergySolution,
    radiation_vis: TwoStreamResult,
    radiation_nir: TwoStreamResult,
    water_requests: Vec<WaterRequest>,
    incident_rain_kg_m2: f64,
    terminal_rain_weight: f64,
}

struct PhysicalStratum {
    config: StratumConfiguration,
    state: StratumState,
    cn: CnParameters,
    energy: CanopyEnergySolution,
    energy_input: CanopyEnergyInput,
    hydraulic: HydraulicSolution,
    interception: InterceptionResult,
    final_offer: crate::carbon_nitrogen::CarbonOffer,
    gpp: f64,
    maintenance: f64,
    t10_next: f64,
    transfers: Vec<MaterialTransfer>,
    water_uses: Vec<WaterUse>,
    nitrogen_requests: Vec<NitrogenRequest>,
    radiation_vis: TwoStreamResult,
    radiation_nir: TwoStreamResult,
    gas_hydraulic_mismatch: f64,
    terminal_rain_weight: f64,
}

#[derive(Default)]
struct EnergyAccumulator {
    incident_shortwave: f64,
    reflected_shortwave: f64,
    terminal_shortwave: f64,
    canopy_absorbed_shortwave: f64,
    transpiration_amount: f64,
    wet_phase_change_amount: f64,
    incident_longwave: f64,
    emitted_longwave: f64,
    sensible: f64,
    canopy_evaporation: f64,
    throughfall: f64,
    stemflow: f64,
    drainage: f64,
}

impl EnergyAccumulator {
    fn owner_operands(&self, interval_s: f64) -> EnergyOwnerOperands {
        EnergyOwnerOperands {
            incident_shortwave_w_m2: self.incident_shortwave,
            reflected_shortwave_w_m2: self.reflected_shortwave,
            terminal_shortwave_w_m2: self.terminal_shortwave,
            incident_longwave_j_m2: self.incident_longwave,
            emitted_longwave_j_m2: self.emitted_longwave,
            sensible_j_m2: self.sensible,
            transpiration_kg_m2: self.transpiration_amount,
            wet_phase_change_kg_m2: self.wet_phase_change_amount,
            interval_s,
        }
    }
    fn add(&mut self, item: &PhysicalStratum, forcing: &SnowFreeForcing, dt_s: f64) {
        self.reflected_shortwave += item.radiation_vis.reflected + item.radiation_nir.reflected;
        self.terminal_shortwave += (1.0 - forcing.ground_albedo_vis)
            * (item.radiation_vis.terminal_from_direct + item.radiation_vis.terminal_from_diffuse)
            + (1.0 - forcing.ground_albedo_nir)
                * (item.radiation_nir.terminal_from_direct
                    + item.radiation_nir.terminal_from_diffuse);
        self.canopy_absorbed_shortwave += item.radiation_vis.absorbed + item.radiation_nir.absorbed;
        self.transpiration_amount += dt_s
            * (item.energy.sun_transpiration_kg_m2_s + item.energy.shade_transpiration_kg_m2_s);
        self.wet_phase_change_amount += dt_s * item.energy.wet_vapor_kg_m2_s;
        let input = &item.energy_input;
        let wet_area = input.wet_leaf_area + input.wet_stem_area;
        let longwave_area = input.sun.emissivity * input.sun.lai_dry
            + input.shade.emissivity * input.shade.lai_dry
            + input.wet_emissivity * wet_area
            + input.stem_emissivity * input.dry_stem_area;
        self.incident_longwave +=
            dt_s * longwave_area * (input.longwave_down_w_m2 + input.longwave_up_w_m2);
        self.emitted_longwave += dt_s
            * 2.0
            * STEFAN_BOLTZMANN
            * (input.sun.emissivity * input.sun.lai_dry * item.energy.sun_temperature_k.powi(4)
                + input.shade.emissivity
                    * input.shade.lai_dry
                    * item.energy.shade_temperature_k.powi(4)
                + input.wet_emissivity * wet_area * item.energy.wet_temperature_k.powi(4)
                + input.stem_emissivity
                    * input.dry_stem_area
                    * item.energy.stem_temperature_k.powi(4));
        let rho = input.pressure_pa / (287.05 * item.energy.canopy_temperature_k);
        self.sensible +=
            dt_s * rho * 1_004.64 * (item.energy.canopy_temperature_k - input.air_temperature_k)
                / input.rah_s_m;
        self.canopy_evaporation += item.interception.evaporation - item.interception.condensation;
        self.throughfall += item.interception.throughfall * item.terminal_rain_weight;
        self.stemflow += item.interception.stemflow;
        self.drainage += item.interception.drainage * item.terminal_rain_weight;
    }
}

#[allow(clippy::too_many_lines)]
fn validate_execution(
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
) -> Result<(), VegetationError> {
    if model.version != crate::MODEL_VERSION || model.sha256 != MODEL_SHA256 {
        return Err(VegetationError::ModelDigestMismatch {
            expected: MODEL_SHA256.into(),
            found: model.sha256.clone(),
        });
    }
    config.validate()?;
    beginning.validate()?;
    if beginning
        .strata
        .values()
        .any(|stratum| !stratum.pending_transfers.is_empty())
    {
        return Err(VegetationError::Receipt(
            "unresolved beginning-state material transfer".into(),
        ));
    }
    if beginning.model_definition_sha256 != model.sha256
        || beginning.configuration_sha256 != config.configuration_sha256
    {
        return Err(VegetationError::Receipt(
            "model/configuration state identity".into(),
        ));
    }
    if beginning.last_transaction_id == 0 && beginning.state_sha256 != config.initial_state_sha256 {
        return Err(VegetationError::Receipt(
            "initial-state digest does not match configuration".into(),
        ));
    }
    let configuration_ids = config
        .strata
        .iter()
        .map(|stratum| stratum.stratum_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let state_ids = beginning
        .strata
        .keys()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    if configuration_ids != state_ids {
        return Err(VegetationError::Domain(
            "configuration/state stratum identity",
        ));
    }
    for stratum in &config.strata {
        let state = beginning
            .strata
            .get(&stratum.stratum_id)
            .ok_or(VegetationError::Domain("missing stratum state"))?;
        let expected_lai = tissue_carbon(state, Tissue::Leaf)? * stratum.sla_m2_per_kg_c;
        let derived_stem_area = expected_lai * stratum.sai_relation;
        let derived_root_area = (expected_lai + derived_stem_area) * stratum.root_to_leaf_area;
        if (state.leaf_area - expected_lai).abs() > 1e-14 + 64.0 * f64::EPSILON * expected_lai.abs()
        {
            return Err(VegetationError::Domain("leaf-C/SLA state identity"));
        }
        if (state.stem_area - derived_stem_area).abs()
            > 1e-14 + 64.0 * f64::EPSILON * derived_stem_area.abs()
        {
            return Err(VegetationError::Domain("leaf-area/SAI state identity"));
        }
        if (state.root_area - derived_root_area).abs()
            > 1e-14 + 64.0 * f64::EPSILON * derived_root_area.abs()
        {
            return Err(VegetationError::Domain("leaf/SAI/root-area state identity"));
        }
    }
    let values = [
        forcing.air_temperature_k,
        forcing.pressure_pa,
        forcing.co2_pa,
        forcing.vapor_pressure_deficit_kpa,
        forcing.wind_m_s,
        forcing.rain_kg_m2,
        forcing.direct_par_w_m2,
        forcing.diffuse_par_w_m2,
        forcing.direct_nir_w_m2,
        forcing.diffuse_nir_w_m2,
        forcing.solar_zenith_cosine,
        forcing.ground_albedo_vis,
        forcing.ground_albedo_nir,
        forcing.longwave_down_w_m2,
        forcing.longwave_up_w_m2,
        forcing.specific_humidity,
        forcing.reference_height_m,
        forcing.gsi,
    ];
    if values.iter().any(|value| !value.is_finite())
        || !(273.15..=373.15).contains(&forcing.air_temperature_k)
        || forcing.pressure_pa <= 0.0
        || forcing.co2_pa <= 0.0
        || forcing.vapor_pressure_deficit_kpa <= 0.0
        || forcing.wind_m_s <= 0.0
        || forcing.rain_kg_m2 < 0.0
        || forcing.direct_par_w_m2 < 0.0
        || forcing.diffuse_par_w_m2 < 0.0
        || forcing.direct_nir_w_m2 < 0.0
        || forcing.diffuse_nir_w_m2 < 0.0
        || (forcing.direct_par_w_m2 + forcing.direct_nir_w_m2 > 0.0
            && forcing.solar_zenith_cosine <= 0.0)
        || !(0.0..=1.0).contains(&forcing.ground_albedo_vis)
        || !(0.0..=1.0).contains(&forcing.ground_albedo_nir)
        || forcing.specific_humidity <= 0.0
        || forcing.reference_height_m <= 0.0
        || !(0.0..=1.0).contains(&forcing.gsi)
    {
        return Err(VegetationError::Domain("complete snow-free forcing"));
    }
    let forcing_ids = forcing
        .soil_layers
        .iter()
        .map(|layer| layer.layer_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if forcing_ids.len() != forcing.soil_layers.len()
        || config
            .strata
            .iter()
            .flat_map(|s| &s.root_layers)
            .any(|root| !forcing_ids.contains(root.layer_id.as_str()))
    {
        return Err(VegetationError::Domain("soil layer topology"));
    }
    Ok(())
}

pub(crate) fn radiation_by_stratum(
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
) -> Result<BTreeMap<String, (TwoStreamResult, TwoStreamResult)>, VegetationError> {
    let mut totals = BTreeMap::new();
    for tile in &config.topology_tiles {
        let mut column = config
            .strata
            .iter()
            .filter(|stratum| stratum.tile_ids.contains(&tile.tile_id))
            .collect::<Vec<_>>();
        column.sort_by_key(|stratum| stratum.vertical_rank);
        if column.is_empty() {
            continue;
        }
        let layer = |stratum: &StratumConfiguration, visible: bool| {
            let state = beginning
                .strata
                .get(&stratum.stratum_id)
                .ok_or(VegetationError::Domain("missing radiation stratum"))?;
            let coverage = stratum_coverage(config, stratum)?;
            let leaf_area = state.leaf_area / coverage;
            let stem_area = state.stem_area / coverage;
            let total_area = leaf_area + stem_area;
            let mix = |leaf: f64, stem: f64| {
                if total_area == 0.0 {
                    0.0
                } else {
                    (leaf_area * leaf + stem_area * stem) / total_area
                }
            };
            let (rho, tau) = if visible {
                (
                    mix(stratum.leaf_rho_vis, stratum.stem_rho_vis),
                    mix(stratum.leaf_tau_vis, stratum.stem_tau_vis),
                )
            } else {
                (
                    mix(stratum.leaf_rho_nir, stratum.stem_rho_nir),
                    mix(stratum.leaf_tau_nir, stratum.stem_tau_nir),
                )
            };
            Ok(ColumnLayer {
                plant_area: stratum.clumping_index * total_area,
                chi: stratum.leaf_angle_chi,
                rho,
                tau,
            })
        };
        let vis_layers = column
            .iter()
            .map(|stratum| layer(stratum, true))
            .collect::<Result<Vec<_>, VegetationError>>()?;
        let nir_layers = column
            .iter()
            .map(|stratum| layer(stratum, false))
            .collect::<Result<Vec<_>, VegetationError>>()?;
        let vis_results = solve_column(
            &vis_layers,
            forcing.solar_zenith_cosine,
            forcing.ground_albedo_vis,
            forcing.direct_par_w_m2,
            forcing.diffuse_par_w_m2,
        )?;
        let nir_results = solve_column(
            &nir_layers,
            forcing.solar_zenith_cosine,
            forcing.ground_albedo_nir,
            forcing.direct_nir_w_m2,
            forcing.diffuse_nir_w_m2,
        )?;
        for ((stratum, mut vis), mut nir) in column.into_iter().zip(vis_results).zip(nir_results) {
            let state = beginning
                .strata
                .get(&stratum.stratum_id)
                .ok_or(VegetationError::Domain("missing radiation stratum"))?;
            let coverage = stratum_coverage(config, stratum)?;
            partition_leaf_stem(&mut vis, state, stratum, coverage, true);
            partition_leaf_stem(&mut nir, state, stratum, coverage, false);
            let entry = totals
                .entry(stratum.stratum_id.clone())
                .or_insert_with(|| (zero_radiation(), zero_radiation()));
            accumulate_radiation(&mut entry.0, vis, tile.fraction);
            accumulate_radiation(&mut entry.1, nir, tile.fraction);
        }
    }
    Ok(totals)
}

pub(crate) fn rain_by_stratum(
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
) -> Result<BTreeMap<String, (f64, f64)>, VegetationError> {
    let mut incident = BTreeMap::<String, f64>::new();
    let mut terminal_fraction = BTreeMap::<String, f64>::new();
    let mut occupied_fraction = BTreeMap::<String, f64>::new();
    for tile in &config.topology_tiles {
        let mut column = config
            .strata
            .iter()
            .filter(|stratum| stratum.tile_ids.contains(&tile.tile_id))
            .collect::<Vec<_>>();
        column.sort_by_key(|stratum| stratum.vertical_rank);
        let column_len = column.len();
        let mut rain = forcing.rain_kg_m2;
        for (index, stratum) in column.into_iter().enumerate() {
            *incident.entry(stratum.stratum_id.clone()).or_default() += tile.fraction * rain;
            *occupied_fraction
                .entry(stratum.stratum_id.clone())
                .or_default() += tile.fraction;
            if index + 1 == column_len {
                *terminal_fraction
                    .entry(stratum.stratum_id.clone())
                    .or_default() += tile.fraction;
            }
            let state = beginning
                .strata
                .get(&stratum.stratum_id)
                .ok_or(VegetationError::Domain("missing rain-routing stratum"))?;
            let interception = liquid_interception(InterceptionInput {
                store0: state.canopy_liquid,
                rain,
                vapor_amount: 0.0,
                lai: state.leaf_area,
                sai: state.stem_area,
                alpha_liq: stratum.alpha_liq,
                p_liq: stratum.p_liq_kg_m2_plant,
                stemflow_fraction: stratum.stemflow_fraction,
                leaf_temperature_k: forcing.air_temperature_k,
            })?;
            rain = interception.throughfall + interception.drainage;
        }
    }
    Ok(incident
        .into_iter()
        .map(|(id, rain)| {
            let occupied = occupied_fraction.get(&id).copied().unwrap_or(0.0);
            let terminal = terminal_fraction.get(&id).copied().unwrap_or(0.0);
            (id, (rain, terminal / occupied))
        })
        .collect())
}

fn partition_leaf_stem(
    result: &mut TwoStreamResult,
    state: &StratumState,
    stratum: &StratumConfiguration,
    coverage: f64,
    visible: bool,
) {
    let leaf_area = state.leaf_area / coverage;
    let stem_area = state.stem_area / coverage;
    let effective_area = stratum.clumping_index * (leaf_area + stem_area);
    let sun_fraction = if effective_area == 0.0 {
        0.0
    } else {
        result.sunlit_lai / effective_area
    };
    let (leaf_rho, leaf_tau, stem_rho, stem_tau) = if visible {
        (
            stratum.leaf_rho_vis,
            stratum.leaf_tau_vis,
            stratum.stem_rho_vis,
            stratum.stem_tau_vis,
        )
    } else {
        (
            stratum.leaf_rho_nir,
            stratum.leaf_tau_nir,
            stratum.stem_rho_nir,
            stratum.stem_tau_nir,
        )
    };
    let leaf_weight = leaf_area * (1.0 - leaf_rho - leaf_tau);
    let stem_weight = stem_area * (1.0 - stem_rho - stem_tau);
    let leaf_absorbed_fraction = if leaf_weight + stem_weight == 0.0 {
        0.0
    } else {
        leaf_weight / (leaf_weight + stem_weight)
    };
    result.sunlit_lai = leaf_area * sun_fraction;
    result.shaded_lai = leaf_area - result.sunlit_lai;
    result.sunlit_absorbed *= leaf_absorbed_fraction;
    result.shaded_absorbed *= leaf_absorbed_fraction;
}

fn stratum_coverage(
    config: &VegetationConfiguration,
    stratum: &StratumConfiguration,
) -> Result<f64, VegetationError> {
    let coverage = config
        .topology_tiles
        .iter()
        .filter(|tile| stratum.tile_ids.contains(&tile.tile_id))
        .map(|tile| tile.fraction)
        .sum::<f64>();
    if !coverage.is_finite() || coverage <= 0.0 || coverage > 1.0 {
        return Err(VegetationError::Domain("stratum ground coverage"));
    }
    Ok(coverage)
}

fn zero_radiation() -> TwoStreamResult {
    TwoStreamResult {
        absorbed: 0.0,
        reflected: 0.0,
        reflected_direct: 0.0,
        reflected_diffuse: 0.0,
        absorbed_direct: 0.0,
        absorbed_diffuse: 0.0,
        transmitted_direct: 0.0,
        transmitted_diffuse: 0.0,
        terminal_from_direct: 0.0,
        terminal_from_diffuse: 0.0,
        sunlit_lai: 0.0,
        shaded_lai: 0.0,
        sunlit_absorbed: 0.0,
        shaded_absorbed: 0.0,
        closure_residual: 0.0,
    }
}

fn accumulate_radiation(total: &mut TwoStreamResult, value: TwoStreamResult, weight: f64) {
    total.absorbed += weight * value.absorbed;
    total.reflected += weight * value.reflected;
    total.reflected_direct += weight * value.reflected_direct;
    total.reflected_diffuse += weight * value.reflected_diffuse;
    total.absorbed_direct += weight * value.absorbed_direct;
    total.absorbed_diffuse += weight * value.absorbed_diffuse;
    total.transmitted_direct += weight * value.transmitted_direct;
    total.transmitted_diffuse += weight * value.transmitted_diffuse;
    total.terminal_from_direct += weight * value.terminal_from_direct;
    total.terminal_from_diffuse += weight * value.terminal_from_diffuse;
    total.sunlit_lai += weight * value.sunlit_lai;
    total.shaded_lai += weight * value.shaded_lai;
    total.sunlit_absorbed += weight * value.sunlit_absorbed;
    total.shaded_absorbed += weight * value.shaded_absorbed;
    total.closure_residual += weight * value.closure_residual;
}

#[allow(clippy::too_many_arguments)]
fn prepare_stratum(
    stratum: &StratumConfiguration,
    state: StratumState,
    config: &VegetationConfiguration,
    forcing: &SnowFreeForcing,
    transaction_id: TransactionId,
    failure: Option<FailurePoint>,
    vis: TwoStreamResult,
    nir: TwoStreamResult,
    incident_rain_kg_m2: f64,
    terminal_rain_weight: f64,
) -> Result<PreparedStratum, VegetationError> {
    let preliminary_interception = liquid_interception(InterceptionInput {
        store0: state.canopy_liquid,
        rain: incident_rain_kg_m2,
        vapor_amount: 0.0,
        lai: state.leaf_area,
        sai: state.stem_area,
        alpha_liq: stratum.alpha_liq,
        p_liq: stratum.p_liq_kg_m2_plant,
        stemflow_fraction: stratum.stemflow_fraction,
        leaf_temperature_k: forcing.air_temperature_k,
    })?;
    reject_at(failure, FailurePoint::Interception)?;
    let energy_input = energy_input(
        stratum,
        &state,
        config,
        forcing,
        &vis,
        &nir,
        preliminary_interception,
    )?;
    let hydraulic_input = hydraulic_input(stratum, &state, config, forcing, &vis)?;
    let (potential_energy, potential_hydraulic, _) =
        solve_coupled(&energy_input, &hydraulic_input, None)?;
    reject_at(failure, FailurePoint::PotentialCoupledSolve)?;
    let owner = ResourceOwnerId::try_new(stratum.stratum_id.clone())
        .map_err(|_| VegetationError::Domain("stratum owner identity"))?;
    let water_requests = stratum
        .root_layers
        .iter()
        .zip(&potential_hydraulic.finalized_amounts_kg_m2)
        .map(|(root, amount)| {
            Ok(ResourceRequest {
                transaction_id,
                owner_id: owner.clone(),
                key: WaterResourceKey {
                    layer_id: SoilLayerId::try_new(root.layer_id.clone())
                        .map_err(|_| VegetationError::Domain("water layer identity"))?,
                },
                amount: *amount,
                basis: ResourceAmountBasis::WaterKgPerSquareMeterInterval,
            })
        })
        .collect::<Result<Vec<_>, VegetationError>>()?;
    Ok(PreparedStratum {
        config: stratum.clone(),
        state,
        cn: cn_parameters(stratum)?,
        energy_input,
        hydraulic_input,
        potential_energy,
        radiation_vis: vis,
        radiation_nir: nir,
        water_requests,
        incident_rain_kg_m2,
        terminal_rain_weight,
    })
}

#[allow(clippy::too_many_lines)]
fn finish_physical(
    mut item: PreparedStratum,
    config: &VegetationConfiguration,
    forcing: &SnowFreeForcing,
    caps: &[f64],
    transaction_id: TransactionId,
) -> Result<PhysicalStratum, VegetationError> {
    let (energy, hydraulic, mismatch) =
        solve_coupled(&item.energy_input, &item.hydraulic_input, Some(caps))?;
    let mismatch_scale = hydraulic
        .transpiration_mm_s
        .abs()
        .max((energy.sun_transpiration_kg_m2_s + energy.shade_transpiration_kg_m2_s).abs());
    if mismatch.abs() > 1e-12 + 1e-9 * mismatch_scale {
        return Err(VegetationError::Hydraulic(
            "gas/hydraulic transpiration mismatch",
        ));
    }
    let vapor_amount = energy.wet_vapor_kg_m2_s * config.dt_s;
    let interception = liquid_interception(InterceptionInput {
        store0: item.state.canopy_liquid,
        rain: item.incident_rain_kg_m2,
        vapor_amount,
        lai: item.state.leaf_area,
        sai: item.state.stem_area,
        alpha_liq: item.config.alpha_liq,
        p_liq: item.config.p_liq_kg_m2_plant,
        stemflow_fraction: item.config.stemflow_fraction,
        leaf_temperature_k: energy.wet_temperature_k,
    })?;
    let mode = if item.config.phenology_type == PhenologyType::Evergreen {
        PhenologyMode::Evergreen
    } else {
        PhenologyMode::SeasonalDeciduous
    };
    let phenology = advance_phenology(
        &mut item.state.tissues,
        mode,
        item.state.phase,
        item.state.onset_remaining_s,
        item.state.offset_remaining_s,
        item.state.previous_gsi,
        forcing.gsi,
        config.dt_s,
        item.config.gsi_on_threshold.unwrap_or(1.0),
        item.config.gsi_off_threshold.unwrap_or(0.0),
        item.config.onset_duration_s.unwrap_or(config.dt_s),
        item.config.offset_duration_s.unwrap_or(config.dt_s),
        &item.cn,
    )?;
    item.state.phase = phenology.phase;
    item.state.onset_remaining_s = phenology.onset_remaining_s;
    item.state.offset_remaining_s = phenology.offset_remaining_s;
    item.state.previous_gsi = phenology.previous_gsi;
    item.state.retranslocation_n += phenology.retranslocated_n;
    let mut transfers = phenology.transfers;
    transfers.extend(advance_turnover(
        &mut item.state.tissues,
        config.dt_s,
        &item.cn,
    )?);
    let t10_next = update_t10(item.state.t10_k, forcing.air_temperature_k, config.dt_s)?;
    let maintenance = maintenance_respiration(
        &item.state.tissues,
        item.state.leaf_area,
        t10_next,
        forcing.air_temperature_k,
        &item
            .config
            .root_layers
            .iter()
            .map(|root| soil(forcing, &root.layer_id).map(|v| v.temperature_k))
            .collect::<Result<Vec<_>, _>>()?,
        &item
            .config
            .root_layers
            .iter()
            .map(|root| root.root_fraction)
            .collect::<Vec<_>>(),
        item.config.atkin_intercept,
        item.config.mr_base_kgc_per_kgn_s,
        item.config.mr_q10,
        config.dt_s,
    )?;
    let potential_gpp = gpp_kg_c(
        config.dt_s,
        1.0,
        item.potential_energy.sun_ci.fvcb.ag,
        item.radiation_vis.sunlit_lai,
        item.potential_energy.shade_ci.fvcb.ag,
        item.radiation_vis.shaded_lai,
    )?;
    let gpp = gpp_kg_c(
        config.dt_s,
        1.0,
        energy.sun_ci.fvcb.ag,
        item.radiation_vis.sunlit_lai,
        energy.shade_ci.fvcb.ag,
        item.radiation_vis.shaded_lai,
    )?;
    let potential_offer = carbon_offer(
        potential_gpp,
        maintenance,
        item.state.xs_c,
        item.state.nsc_c,
        config.dt_s,
        item.config.xs_recovery_days,
    )?;
    let final_offer = carbon_offer(
        gpp,
        maintenance,
        item.state.xs_c,
        item.state.nsc_c,
        config.dt_s,
        item.config.xs_recovery_days,
    )?;
    let potential_demand = nitrogen_demand(
        potential_offer.offer,
        item.state.retranslocation_n,
        &item.cn,
    )?;
    let owner = ResourceOwnerId::try_new(item.config.stratum_id.clone())
        .map_err(|_| VegetationError::Domain("stratum owner identity"))?;
    for (proposal_id, transfer) in transfers.iter_mut().enumerate() {
        transfer.transaction_id = transaction_id.0;
        transfer.owner_id.clone_from(&item.config.stratum_id);
        transfer.proposal_id = u64::try_from(proposal_id)
            .map_err(|_| VegetationError::Domain("material proposal identity"))?;
    }
    let mut nitrogen_requests = Vec::new();
    for root in &item.config.root_layers {
        for (species, fraction) in [
            (
                MineralNitrogenSpecies::Ammonium,
                item.config.nh4_request_fraction,
            ),
            (
                MineralNitrogenSpecies::Nitrate,
                1.0 - item.config.nh4_request_fraction,
            ),
        ] {
            nitrogen_requests.push(ResourceRequest {
                transaction_id,
                owner_id: owner.clone(),
                key: MineralNitrogenKey {
                    layer_id: SoilLayerId::try_new(root.layer_id.clone())
                        .map_err(|_| VegetationError::Domain("nitrogen layer identity"))?,
                    species,
                },
                amount: potential_demand.external_shortfall
                    * root.mineral_n_root_fraction
                    * fraction,
                basis: ResourceAmountBasis::NitrogenKgPerSquareMeterInterval,
            });
        }
    }
    let water_uses = item
        .water_requests
        .iter()
        .zip(&hydraulic.finalized_amounts_kg_m2)
        .map(|(request, amount)| FinalizedUse {
            transaction_id,
            owner_id: request.owner_id.clone(),
            key: request.key.clone(),
            amount: *amount,
            basis: request.basis,
        })
        .collect();
    Ok(PhysicalStratum {
        config: item.config,
        state: item.state,
        cn: item.cn,
        energy,
        energy_input: item.energy_input,
        hydraulic,
        interception,
        final_offer,
        gpp,
        maintenance,
        t10_next,
        transfers,
        water_uses,
        nitrogen_requests,
        radiation_vis: item.radiation_vis,
        radiation_nir: item.radiation_nir,
        gas_hydraulic_mismatch: mismatch,
        terminal_rain_weight: item.terminal_rain_weight,
    })
}

#[allow(clippy::too_many_lines)]
fn energy_input(
    stratum: &StratumConfiguration,
    state: &StratumState,
    config: &VegetationConfiguration,
    forcing: &SnowFreeForcing,
    vis: &TwoStreamResult,
    nir: &TwoStreamResult,
    interception: InterceptionResult,
) -> Result<CanopyEnergyInput, VegetationError> {
    let leaf_n = tissue_nitrogen(state, Tissue::Leaf)?;
    let n_area = if state.leaf_area == 0.0 {
        0.0
    } else {
        leaf_n / state.leaf_area
    };
    let vcmax = stratum.rubisco_n_efficiency * n_area;
    let jmax = stratum.electron_n_efficiency * n_area;
    let rd = stratum.rd_leaf_n_rate * n_area;
    let biochemical = FvcbInput {
        ci_pa: forcing.co2_pa * 0.7,
        oi_pa: forcing.pressure_pa * 0.20,
        gamma_pa: stratum.gamma25_pa,
        kc_pa: stratum.kc25_pa,
        ko_pa: stratum.ko25_pa,
        vcmax,
        jmax,
        tp: stratum.tp_vcmax_ratio * vcmax,
        rd,
        par_abs: 0.0,
    };
    let gb_leaf = leaf_boundary_conductance(forcing.wind_m_s, stratum.leaf_dimension_m)?;
    let gb_stem = leaf_boundary_conductance(forcing.wind_m_s, stratum.stem_dimension_m)?;
    let gb_wet = leaf_boundary_conductance(forcing.wind_m_s, stratum.wet_surface_dimension_m)?;
    let rah = neutral_resistance(
        forcing.reference_height_m,
        stratum.displacement_m,
        stratum.z0m_m,
        stratum.z0h_m,
        forcing.wind_m_s,
    )?;
    let raw = neutral_resistance(
        forcing.reference_height_m,
        stratum.displacement_m,
        stratum.z0m_m,
        stratum.z0q_m,
        forcing.wind_m_s,
    )?;
    let wet_fraction = interception.wet_fraction;
    let leaf_shortwave =
        vis.sunlit_absorbed + vis.shaded_absorbed + nir.sunlit_absorbed + nir.shaded_absorbed;
    let total_shortwave = vis.absorbed + nir.absorbed;
    let stem_shortwave = (total_shortwave - leaf_shortwave).max(0.0);
    let leaf = |lai_total: f64, vis_absorbed: f64, nir_absorbed: f64| LeafClassEnergyInput {
        lai_total,
        lai_dry: lai_total * (1.0 - wet_fraction),
        absorbed_shortwave_w_m2: (vis_absorbed + nir_absorbed) * (1.0 - wet_fraction),
        absorbed_par_w_m2_leaf: if lai_total == 0.0 {
            0.0
        } else {
            vis_absorbed / lai_total
        },
        gb_m_s: gb_leaf,
        emissivity: stratum.leaf_emissivity,
        biochemical,
        ha_vcmax: stratum.ha_vcmax,
        hd_vcmax: stratum.hd_vcmax,
        entropy_vcmax: stratum.entropy_vcmax,
        ha_jmax: stratum.ha_jmax,
        hd_jmax: stratum.hd_jmax,
        entropy_jmax: stratum.entropy_jmax,
        ha_kc: stratum.ha_kc,
        ha_ko: stratum.ha_ko,
        ha_gamma: stratum.ha_gamma,
    };
    Ok(CanopyEnergyInput {
        sun: leaf(vis.sunlit_lai, vis.sunlit_absorbed, nir.sunlit_absorbed),
        shade: leaf(vis.shaded_lai, vis.shaded_absorbed, nir.shaded_absorbed),
        wet_leaf_area: state.leaf_area * wet_fraction,
        wet_stem_area: state.stem_area * wet_fraction,
        dry_stem_area: state.stem_area * (1.0 - wet_fraction),
        wet_shortwave_w_m2: total_shortwave * wet_fraction,
        dry_stem_shortwave_w_m2: stem_shortwave * (1.0 - wet_fraction),
        gb_wet_m_s: gb_wet,
        gb_stem_m_s: gb_stem,
        wet_emissivity: stratum.wet_surface_emissivity,
        stem_emissivity: stratum.stem_emissivity,
        longwave_down_w_m2: forcing.longwave_down_w_m2,
        longwave_up_w_m2: forcing.longwave_up_w_m2,
        air_temperature_k: forcing.air_temperature_k,
        qair: forcing.specific_humidity,
        pressure_pa: forcing.pressure_pa,
        atmospheric_co2_pa: forcing.co2_pa,
        rah_s_m: rah,
        raw_s_m: raw,
        g0: stratum.g0_umol_h2o_m2_s,
        g1: stratum.g1_sqrt_kpa,
        beta_hyd: 1.0,
        liquid_store_kg_m2: interception.store1,
        condensation_capacity_kg_m2: stratum.p_liq_kg_m2_plant
            * (state.leaf_area + state.stem_area)
            - interception.store1,
        dt_s: config.dt_s,
        warm_start: [
            forcing.air_temperature_k,
            forcing.air_temperature_k,
            forcing.air_temperature_k,
            forcing.air_temperature_k,
            forcing.air_temperature_k,
            forcing.specific_humidity,
        ],
    })
}

fn hydraulic_input(
    stratum: &StratumConfiguration,
    state: &StratumState,
    config: &VegetationConfiguration,
    forcing: &SnowFreeForcing,
    radiation: &TwoStreamResult,
) -> Result<HydraulicInput, VegetationError> {
    let layers = stratum
        .root_layers
        .iter()
        .map(|root| {
            let layer = soil(forcing, &root.layer_id)?;
            Ok(HydraulicLayerInput {
                soil_psi_mm: layer.matric_potential_mm,
                root_fraction: root.root_fraction,
                soil_conductivity_mm_s: layer.hydraulic_conductivity_mm_s,
                root_path_length_mm: layer.root_path_length_mm,
                lateral_root_length_mm: root.lateral_root_length_m * 1_000.0,
                gravity_root_mm: layer.gravity_root_mm,
                accessible: layer.accessible,
                frozen: layer.frozen,
            })
        })
        .collect::<Result<Vec<_>, VegetationError>>()?;
    Ok(HydraulicInput {
        dt_s: config.dt_s,
        lai_sun: radiation.sunlit_lai,
        lai_shade: radiation.shaded_lai,
        sai: state.stem_area,
        emax_sun_mm_s: 0.0,
        emax_shade_mm_s: 0.0,
        k1a_max_s1: stratum.k1a_max_s1,
        k1b_max_s1: stratum.k1b_max_s1,
        k2_max_mm_s: stratum.k2_max_m_s * 1_000.0,
        k3_max_mm_s: stratum.k3_max_m_s * 1_000.0,
        stem_path_length_mm: (stratum.height_m - stratum.crown_base_m) * 1_000.0,
        root_to_leaf_area: stratum.root_to_leaf_area,
        gravity_stem_mm: (stratum.height_m - stratum.crown_base_m) * 1_000.0,
        p50_leaf_mm: stratum.p50_leaf_mm,
        p50_stem_mm: stratum.p50_stem_mm,
        p50_root_mm: stratum.p50_root_mm,
        shape: stratum.vulnerability_shape,
        layers,
        warm_start: [
            state.psi_sun_mm,
            state.psi_shade_mm,
            state.psi_stem_mm,
            state.psi_root_mm,
        ],
    })
}

fn solve_coupled(
    energy_template: &CanopyEnergyInput,
    hydraulic_template: &HydraulicInput,
    caps: Option<&[f64]>,
) -> Result<(CanopyEnergySolution, HydraulicSolution, f64), VegetationError> {
    if energy_template.sun.lai_total + energy_template.shade.lai_total == 0.0 {
        let energy = solve_canopy_energy(energy_template)?;
        let mut hydraulic_input = hydraulic_template.clone();
        hydraulic_input.emax_sun_mm_s = 0.0;
        hydraulic_input.emax_shade_mm_s = 0.0;
        let hydraulic = solve_hydraulics(&hydraulic_input, caps)?;
        return Ok((energy, hydraulic, 0.0));
    }
    let mut potential_input = energy_template.clone();
    potential_input.beta_hyd = 1.0;
    let potential_energy = solve_canopy_energy(&potential_input)?;
    let mut hydraulic_input = hydraulic_template.clone();
    hydraulic_input.emax_sun_mm_s = potential_energy.sun_transpiration_kg_m2_s;
    hydraulic_input.emax_shade_mm_s = potential_energy.shade_transpiration_kg_m2_s;
    let hydraulic = solve_hydraulics(&hydraulic_input, caps)?;
    let target = hydraulic.transpiration_mm_s;
    let mut lower_beta = 0.0;
    let mut upper_beta = 1.0;
    let mut warm_energy = energy_template.warm_start;
    for iteration in 0..50 {
        let beta = if iteration == 0 {
            1.0
        } else {
            f64::midpoint(lower_beta, upper_beta)
        };
        let mut energy_input = energy_template.clone();
        energy_input.beta_hyd = beta;
        energy_input.warm_start = warm_energy;
        let energy = solve_canopy_energy(&energy_input)?;
        let gas = energy.sun_transpiration_kg_m2_s + energy.shade_transpiration_kg_m2_s;
        let mismatch = gas - target;
        let mismatch_tolerance = 1e-12 + 1e-9 * gas.abs().max(target.abs());
        if mismatch.abs() <= mismatch_tolerance {
            return Ok((energy, hydraulic, mismatch));
        }
        if mismatch > 0.0 {
            upper_beta = beta;
        } else {
            lower_beta = beta;
        }
        warm_energy = [
            energy.sun_temperature_k,
            energy.shade_temperature_k,
            energy.wet_temperature_k,
            energy.stem_temperature_k,
            energy.canopy_temperature_k,
            energy.canopy_specific_humidity,
        ];
    }
    Err(VegetationError::Coupled("coupled iteration limit"))
}

fn cn_parameters(stratum: &StratumConfiguration) -> Result<CnParameters, VegetationError> {
    let fraction = |map: &BTreeMap<String, f64>, key: &str| map.get(key).copied().unwrap_or(0.0);
    let leaf = [
        fraction(&stratum.litter_metabolic_fraction, "leaf"),
        fraction(&stratum.litter_cellulose_fraction, "leaf"),
        fraction(&stratum.litter_lignin_fraction, "leaf"),
    ];
    let root = [
        fraction(&stratum.litter_metabolic_fraction, "fine_root"),
        fraction(&stratum.litter_cellulose_fraction, "fine_root"),
        fraction(&stratum.litter_lignin_fraction, "fine_root"),
    ];
    if (leaf.iter().sum::<f64>() - 1.0).abs() > 1e-12
        || (root.iter().sum::<f64>() - 1.0).abs() > 1e-12
    {
        return Err(VegetationError::Domain("litter fractions"));
    }
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
        leaf_litter_fractions: leaf,
        froot_litter_fractions: root,
    })
}

fn soil<'a>(
    forcing: &'a SnowFreeForcing,
    id: &str,
) -> Result<&'a SoilLayerForcing, VegetationError> {
    forcing
        .soil_layers
        .iter()
        .find(|layer| layer.layer_id.as_str() == id)
        .ok_or(VegetationError::Domain("missing soil forcing"))
}

fn tissue_carbon(state: &StratumState, tissue: Tissue) -> Result<f64, VegetationError> {
    let pool = state
        .tissues
        .get(&tissue)
        .ok_or(VegetationError::Domain("missing tissue"))?;
    Ok(pool.display.carbon + pool.storage.carbon + pool.transfer.carbon)
}
fn tissue_nitrogen(state: &StratumState, tissue: Tissue) -> Result<f64, VegetationError> {
    let pool = state
        .tissues
        .get(&tissue)
        .ok_or(VegetationError::Domain("missing tissue"))?;
    Ok(pool.display.nitrogen + pool.storage.nitrogen + pool.transfer.nitrogen)
}
fn vegetation_carbon(state: &CoupledOwnedState) -> f64 {
    state
        .strata
        .values()
        .map(|stratum| {
            stratum
                .tissues
                .values()
                .map(|pool| pool.display.carbon + pool.storage.carbon + pool.transfer.carbon)
                .sum::<f64>()
                + stratum.nsc_c
                + stratum.xs_c
                + stratum.standing_dead.carbon
        })
        .sum()
}
fn vegetation_nitrogen(state: &CoupledOwnedState) -> f64 {
    state
        .strata
        .values()
        .map(|stratum| {
            stratum
                .tissues
                .values()
                .map(|pool| pool.display.nitrogen + pool.storage.nitrogen + pool.transfer.nitrogen)
                .sum::<f64>()
                + stratum.retranslocation_n
                + stratum.standing_dead.nitrogen
        })
        .sum()
}
fn vegetation_dry_material(
    state: &CoupledOwnedState,
    config: &VegetationConfiguration,
) -> Result<f64, VegetationError> {
    config
        .strata
        .iter()
        .map(|cfg| {
            let stratum = state
                .strata
                .get(&cfg.stratum_id)
                .ok_or(VegetationError::Domain("missing stratum"))?;
            let structural_c = stratum
                .tissues
                .values()
                .map(|pool| pool.display.carbon + pool.storage.carbon + pool.transfer.carbon)
                .sum::<f64>();
            Ok(structural_c / cfg.drymatter_carbon_fraction + stratum.standing_dead_dm)
        })
        .sum()
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn build_ledgers(
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    ending: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    water_owner: &dyn WaterArbiter,
    nitrogen_owner: &dyn NitrogenArbiter,
    water_uses: &[WaterUse],
    nitrogen_uses: &[NitrogenUse],
    transfers: &[MaterialTransfer],
    gpp: f64,
    maintenance: f64,
    growth_respiration: f64,
    dry_growth: f64,
    energy: &EnergyAccumulator,
    transaction_id: TransactionId,
) -> Result<FiveLedgerOperands, VegetationError> {
    let identity = LedgerIdentity {
        transaction_id,
        owner_id: "coupled_c3_vegetation_diagnostic".into(),
        area_m2: config.area_m2,
        interval_s: config.dt_s,
    };
    let mut water_by_layer = BTreeMap::<SoilLayerId, f64>::new();
    for use_value in water_uses {
        *water_by_layer
            .entry(use_value.key.layer_id.clone())
            .or_default() += use_value.amount;
    }
    let soil_water = forcing
        .soil_layers
        .iter()
        .map(|layer| {
            let key = WaterResourceKey {
                layer_id: layer.layer_id.clone(),
            };
            let owner_beginning = water_owner.beginning_amount(&key)?;
            if (owner_beginning - layer.water_beginning_kg_m2).abs() > 1e-12 {
                return Err(VegetationError::Receipt(
                    "water beginning-state mismatch".into(),
                ));
            }
            let withdrawal = water_by_layer.get(&layer.layer_id).copied().unwrap_or(0.0);
            if withdrawal > owner_beginning {
                return Err(VegetationError::Receipt("water candidate overdraft".into()));
            }
            Ok(WaterStoreOperand {
                layer_id: layer.layer_id.clone(),
                beginning_kg_m2: owner_beginning,
                withdrawal_kg_m2: withdrawal,
                ending_kg_m2: owner_beginning - withdrawal,
            })
        })
        .collect::<Result<Vec<_>, VegetationError>>()?;
    let canopy_beginning = beginning
        .strata
        .values()
        .map(|state| state.canopy_liquid)
        .sum();
    let canopy_ending = ending
        .strata
        .values()
        .map(|state| state.canopy_liquid)
        .sum();
    let precipitation = forcing.rain_kg_m2;
    let mut mineral = Vec::new();
    let mut nitrogen_by_key = BTreeMap::<MineralNitrogenKey, f64>::new();
    for use_value in nitrogen_uses {
        *nitrogen_by_key.entry(use_value.key.clone()).or_default() += use_value.amount;
    }
    for key in nitrogen_by_key.keys() {
        let beginning_amount = nitrogen_owner.beginning_amount(key)?;
        let finalized = nitrogen_by_key[key];
        if finalized > beginning_amount {
            return Err(VegetationError::Receipt(
                "nitrogen candidate overdraft".into(),
            ));
        }
        mineral.push(NitrogenStoreOperand {
            key: key.clone(),
            beginning_kg_m2: beginning_amount,
            finalized_use_kg_m2: finalized,
            ending_kg_m2: beginning_amount - finalized,
        });
    }
    let exported_c = transfers.iter().map(|value| value.carbon).sum::<f64>();
    let exported_n = transfers.iter().map(|value| value.nitrogen).sum::<f64>();
    let exported_dm = transfers.iter().map(|value| value.dry_matter).sum::<f64>();
    Ok(FiveLedgerOperands {
        water: WaterLedgerOperands {
            identity: identity.clone(),
            soil: soil_water,
            canopy_beginning_kg_m2: canopy_beginning,
            precipitation_kg_m2: precipitation,
            canopy_ending_kg_m2: canopy_ending,
            canopy_evaporation_kg_m2: energy.canopy_evaporation,
            throughfall_kg_m2: energy.throughfall,
            stemflow_kg_m2: energy.stemflow,
            drainage_kg_m2: energy.drainage,
        },
        energy: EnergyLedgerOperands {
            identity: identity.clone(),
            incident_shortwave_j_m2: energy.incident_shortwave * config.dt_s,
            incident_longwave_j_m2: energy.incident_longwave,
            reflected_shortwave_j_m2: energy.reflected_shortwave * config.dt_s,
            terminal_shortwave_j_m2: energy.terminal_shortwave * config.dt_s,
            emitted_longwave_j_m2: energy.emitted_longwave,
            sensible_j_m2: energy.sensible,
            latent_j_m2: LATENT_HEAT_VAPORIZATION
                * (energy.transpiration_amount + energy.wet_phase_change_amount),
            ground_or_storage_j_m2: 0.0,
        },
        carbon: CarbonLedgerOperands {
            identity: identity.clone(),
            beginning_vegetation_kg_m2: vegetation_carbon(beginning),
            gross_input_kg_m2: gpp,
            maintenance_respiration_kg_m2: maintenance,
            growth_respiration_kg_m2: growth_respiration,
            exported_kg_m2: exported_c,
            ending_vegetation_kg_m2: vegetation_carbon(ending),
        },
        nitrogen: NitrogenLedgerOperands {
            identity: identity.clone(),
            vegetation_beginning_kg_m2: vegetation_nitrogen(beginning),
            mineral,
            exported_kg_m2: exported_n,
            vegetation_ending_kg_m2: vegetation_nitrogen(ending),
        },
        dry_material: DryMaterialLedgerOperands {
            identity,
            vegetation_beginning_kg_m2: vegetation_dry_material(beginning, config)?,
            growth_input_kg_m2: dry_growth,
            exported_kg_m2: exported_dm,
            vegetation_ending_kg_m2: vegetation_dry_material(ending, config)?,
        },
    })
}

fn state_digest(state: &CoupledOwnedState) -> Result<String, VegetationError> {
    state.canonical_sha256()
}

fn reject_at(selected: Option<FailurePoint>, current: FailurePoint) -> Result<(), VegetationError> {
    if selected == Some(current) {
        Err(VegetationError::InjectedFailure(match current {
            FailurePoint::Validation => "validation",
            FailurePoint::Radiation => "radiation",
            FailurePoint::Interception => "interception",
            FailurePoint::PotentialCoupledSolve => "potential coupled solve",
            FailurePoint::WaterAuthorization => "water authorization",
            FailurePoint::CappedResolve => "capped re-solve",
            FailurePoint::NitrogenRequest => "nitrogen request",
            FailurePoint::NitrogenAuthorization => "nitrogen authorization",
            FailurePoint::Allocation => "allocation",
            FailurePoint::ReceiverConstruction => "receiver construction",
            FailurePoint::ClosureValidation => "closure validation",
            FailurePoint::BeforeCommit => "before commit",
            FailurePoint::OwnerValidation => "owner validation",
        }))
    } else {
        Ok(())
    }
}

pub fn validate_and_commit(
    beginning: &mut CoupledOwnedState,
    candidate: CoupledCandidate,
) -> Result<CommitReceipt, VegetationError> {
    validate_and_commit_with_failure(beginning, candidate, None)
}

pub fn validate_and_commit_with_failure(
    beginning: &mut CoupledOwnedState,
    mut candidate: CoupledCandidate,
    failure: Option<FailurePoint>,
) -> Result<CommitReceipt, VegetationError> {
    if beginning.state_sha256 != candidate.beginning_state_sha256 {
        return Err(VegetationError::Receipt("stale beginning state".into()));
    }
    let expected_transaction = beginning
        .last_transaction_id
        .checked_add(1)
        .ok_or(VegetationError::Domain("transaction identity"))?;
    if candidate.diagnostics.transaction_id.0 != expected_transaction
        || candidate.state.last_transaction_id != expected_transaction
        || candidate
            .state
            .strata
            .values()
            .any(|stratum| stratum.last_transaction_id != expected_transaction)
        || candidate.state.model_definition_sha256 != beginning.model_definition_sha256
        || candidate.state.configuration_sha256 != beginning.configuration_sha256
        || candidate.state.strata.keys().ne(beginning.strata.keys())
    {
        return Err(VegetationError::Receipt(
            "candidate transition identity/topology".into(),
        ));
    }
    candidate.state.validate()?;
    let actual_beginning_c = vegetation_carbon(beginning);
    let actual_beginning_n = vegetation_nitrogen(beginning);
    let actual_canopy_water = beginning
        .strata
        .values()
        .map(|stratum| stratum.canopy_liquid)
        .sum::<f64>();
    if candidate
        .ledger_operands
        .carbon
        .beginning_vegetation_kg_m2
        .to_bits()
        != actual_beginning_c.to_bits()
        || candidate
            .ledger_operands
            .nitrogen
            .vegetation_beginning_kg_m2
            .to_bits()
            != actual_beginning_n.to_bits()
        || candidate
            .ledger_operands
            .water
            .canopy_beginning_kg_m2
            .to_bits()
            != actual_canopy_water.to_bits()
    {
        return Err(VegetationError::Receipt(
            "candidate beginning operands do not match owner".into(),
        ));
    }
    if candidate.water_requests.len() != candidate.water_authorizations.len()
        || candidate.water_requests.len() != candidate.water_uses.len()
        || candidate.nitrogen_requests.len() != candidate.nitrogen_authorizations.len()
        || candidate.nitrogen_requests.len() != candidate.nitrogen_uses.len()
    {
        return Err(VegetationError::Receipt("resource protocol shape".into()));
    }
    for ((request, authorization), finalized) in candidate
        .water_requests
        .iter()
        .zip(&candidate.water_authorizations)
        .zip(&candidate.water_uses)
    {
        validate_resource_protocol(request, authorization, finalized)
            .map_err(|error| VegetationError::Receipt(format!("{error:?}")))?;
    }
    for ((request, authorization), finalized) in candidate
        .nitrogen_requests
        .iter()
        .zip(&candidate.nitrogen_authorizations)
        .zip(&candidate.nitrogen_uses)
    {
        validate_resource_protocol(request, authorization, finalized)
            .map_err(|error| VegetationError::Receipt(format!("{error:?}")))?;
    }
    validate_candidate_operands(&candidate)?;
    crate::ledger::validate_five_ledgers(&candidate.ledger_operands)?;
    reject_at(failure, FailurePoint::OwnerValidation)?;
    reject_at(failure, FailurePoint::BeforeCommit)?;
    for stratum in candidate.state.strata.values_mut() {
        stratum.pending_transfers.clear();
    }
    candidate.state.state_sha256 = state_digest(&candidate.state)?;
    let receipt = CommitReceipt {
        transaction_id: candidate.diagnostics.transaction_id,
        ending_state_sha256: candidate.state.state_sha256.clone(),
    };
    *beginning = candidate.state;
    Ok(receipt)
}

fn validate_candidate_operands(candidate: &CoupledCandidate) -> Result<(), VegetationError> {
    let transaction_id = candidate.diagnostics.transaction_id;
    if candidate.ledger_operands.water.identity.transaction_id != transaction_id {
        return Err(VegetationError::Receipt(
            "candidate ledger transaction".into(),
        ));
    }
    let mut water = BTreeMap::<SoilLayerId, f64>::new();
    for finalized in &candidate.water_uses {
        *water.entry(finalized.key.layer_id.clone()).or_default() += finalized.amount;
    }
    for store in &candidate.ledger_operands.water.soil {
        let finalized = water.remove(&store.layer_id).unwrap_or(0.0);
        if (finalized - store.withdrawal_kg_m2).abs() > 1e-12 {
            return Err(VegetationError::Receipt("water ledger/use mismatch".into()));
        }
    }
    if !water.is_empty() {
        return Err(VegetationError::Receipt("unbound water use".into()));
    }
    let mut nitrogen = BTreeMap::<MineralNitrogenKey, f64>::new();
    for finalized in &candidate.nitrogen_uses {
        *nitrogen.entry(finalized.key.clone()).or_default() += finalized.amount;
    }
    for store in &candidate.ledger_operands.nitrogen.mineral {
        let finalized = nitrogen.remove(&store.key).unwrap_or(0.0);
        if (finalized - store.finalized_use_kg_m2).abs() > 1e-12 {
            return Err(VegetationError::Receipt(
                "nitrogen ledger/use mismatch".into(),
            ));
        }
    }
    if !nitrogen.is_empty() {
        return Err(VegetationError::Receipt("unbound nitrogen use".into()));
    }
    let exported_c = candidate
        .material_transfers
        .iter()
        .map(|transfer| transfer.carbon)
        .sum::<f64>();
    let exported_n = candidate
        .material_transfers
        .iter()
        .map(|transfer| transfer.nitrogen)
        .sum::<f64>();
    let exported_dm = candidate
        .material_transfers
        .iter()
        .map(|transfer| transfer.dry_matter)
        .sum::<f64>();
    if (exported_c - candidate.ledger_operands.carbon.exported_kg_m2).abs() > 1e-12
        || (exported_n - candidate.ledger_operands.nitrogen.exported_kg_m2).abs() > 1e-12
        || (exported_dm - candidate.ledger_operands.dry_material.exported_kg_m2).abs() > 1e-12
        || (vegetation_carbon(&candidate.state)
            - candidate.ledger_operands.carbon.ending_vegetation_kg_m2)
            .abs()
            > 1e-12
        || (vegetation_nitrogen(&candidate.state)
            - candidate.ledger_operands.nitrogen.vegetation_ending_kg_m2)
            .abs()
            > 1e-12
    {
        return Err(VegetationError::Receipt(
            "material/state ledger operand mismatch".into(),
        ));
    }
    Ok(())
}
