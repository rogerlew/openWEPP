use std::collections::BTreeMap;

use openwepp_kernel_contract::{
    FinalizedUse, MaximumAuthorization, MineralNitrogenKey, OccupancyId, ResourceRequest,
    SoilLayerId, StratumId, TransactionId, WaterResourceKey,
};
use serde::{Deserialize, Serialize};

use crate::carbon_nitrogen::{ElementPool, MaterialTransfer, Tissue, TissuePool};
use crate::ledger::FiveLedgerOperands;
use crate::occupancy_state::OccupancyState;
use crate::{MODEL_BYTES, MODEL_SHA256, ModelDefinition, VegetationConfiguration, VegetationError};
use sha2::{Digest, Sha256};

mod state_canonical;
mod state_shape;

mod occupancy_state_map {
    use std::collections::BTreeMap;
    use std::fmt;
    use std::marker::PhantomData;

    use openwepp_kernel_contract::OccupancyId;
    use serde::de::{MapAccess, Visitor};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::occupancy_state::OccupancyState;

    pub(super) fn serialize<S>(
        lanes: &BTreeMap<OccupancyId, OccupancyState>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Entry<'a> {
            identity: &'a OccupancyId,
            state: &'a OccupancyState,
        }

        lanes
            .iter()
            .map(|(identity, state)| Entry { identity, state })
            .collect::<Vec<_>>()
            .serialize(serializer)
    }

    pub(super) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<BTreeMap<OccupancyId, OccupancyState>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Entry {
            identity: OccupancyId,
            state: OccupancyState,
        }

        impl<'de> Deserialize<'de> for Entry {
            fn deserialize<T>(deserializer: T) -> Result<Self, T::Error>
            where
                T: Deserializer<'de>,
            {
                enum Field {
                    Identity,
                    State,
                }

                impl<'de> Deserialize<'de> for Field {
                    fn deserialize<T>(deserializer: T) -> Result<Self, T::Error>
                    where
                        T: Deserializer<'de>,
                    {
                        struct FieldVisitor;

                        impl Visitor<'_> for FieldVisitor {
                            type Value = Field;

                            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                                formatter.write_str("`identity` or `state`")
                            }

                            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                            where
                                E: serde::de::Error,
                            {
                                match value {
                                    "identity" => Ok(Field::Identity),
                                    "state" => Ok(Field::State),
                                    _ => Err(E::unknown_field(value, &["identity", "state"])),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(FieldVisitor)
                    }
                }

                struct EntryVisitor(PhantomData<()>);

                impl<'de> Visitor<'de> for EntryVisitor {
                    type Value = Entry;

                    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                        formatter.write_str(
                            "a structural V4 occupancy object with identity and state fields",
                        )
                    }

                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: MapAccess<'de>,
                    {
                        let mut identity = None;
                        let mut state = None;
                        while let Some(field) = map.next_key()? {
                            match field {
                                Field::Identity => {
                                    if identity.is_some() {
                                        return Err(serde::de::Error::duplicate_field("identity"));
                                    }
                                    identity = Some(map.next_value()?);
                                }
                                Field::State => {
                                    if state.is_some() {
                                        return Err(serde::de::Error::duplicate_field("state"));
                                    }
                                    state = Some(map.next_value()?);
                                }
                            }
                        }
                        Ok(Entry {
                            identity: identity
                                .ok_or_else(|| serde::de::Error::missing_field("identity"))?,
                            state: state.ok_or_else(|| serde::de::Error::missing_field("state"))?,
                        })
                    }
                }

                deserializer.deserialize_map(EntryVisitor(PhantomData))
            }
        }

        let entries = Vec::<Entry>::deserialize(deserializer)?;
        let expected_len = entries.len();
        let lanes = entries
            .into_iter()
            .map(|entry| (entry.identity, entry.state))
            .collect::<BTreeMap<_, _>>();
        if lanes.len() != expected_len {
            return Err(serde::de::Error::custom(
                "duplicate V4 occupancy state identity",
            ));
        }
        Ok(lanes)
    }
}

mod stratum_state_map {
    use std::collections::BTreeMap;
    use std::fmt;
    use std::marker::PhantomData;

    use openwepp_kernel_contract::StratumId;
    use serde::de::{MapAccess, Visitor};
    use serde::{Deserializer, Serialize, Serializer};

    use super::StratumSharedState;

    pub(super) fn serialize<S>(
        strata: &BTreeMap<StratumId, StratumSharedState>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        strata.serialize(serializer)
    }

    pub(super) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<BTreeMap<StratumId, StratumSharedState>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrictStrata(PhantomData<()>);

        impl<'de> Visitor<'de> for StrictStrata {
            type Value = BTreeMap<StratumId, StratumSharedState>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a V4 shared-stratum object with unique identities")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut strata = BTreeMap::new();
                while let Some((id, state)) = map.next_entry()? {
                    if strata.insert(id, state).is_some() {
                        return Err(serde::de::Error::custom(
                            "duplicate V4 shared-stratum identity",
                        ));
                    }
                }
                Ok(strata)
            }
        }

        deserializer.deserialize_map(StrictStrata(PhantomData))
    }
}

mod tissue_state_map {
    use std::collections::BTreeMap;
    use std::fmt;
    use std::marker::PhantomData;

    use serde::de::{MapAccess, Visitor};
    use serde::{Deserializer, Serialize, Serializer};

    use crate::carbon_nitrogen::{Tissue, TissuePool};

    pub(super) fn serialize<S>(
        tissues: &BTreeMap<Tissue, TissuePool>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        tissues.serialize(serializer)
    }

    pub(super) fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<BTreeMap<Tissue, TissuePool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrictTissues(PhantomData<()>);

        impl<'de> Visitor<'de> for StrictTissues {
            type Value = BTreeMap<Tissue, TissuePool>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a V4 tissue object with unique identities")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut tissues = BTreeMap::new();
                while let Some((id, pool)) = map.next_entry()? {
                    if tissues.insert(id, pool).is_some() {
                        return Err(serde::de::Error::custom("duplicate V4 tissue identity"));
                    }
                }
                Ok(tissues)
            }
        }

        deserializer.deserialize_map(StrictTissues(PhantomData))
    }
}

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
pub struct StratumSharedState {
    #[serde(with = "tissue_state_map")]
    pub tissues: BTreeMap<Tissue, TissuePool>,
    pub retranslocation_n: f64,
    pub nsc_c: f64,
    pub xs_c: f64,
    pub standing_dead: ElementPool,
    pub standing_dead_dm: f64,
    pub phase: PhenologyPhase,
    pub onset_remaining_s: f64,
    pub offset_remaining_s: f64,
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
    #[serde(with = "stratum_state_map")]
    pub strata: BTreeMap<StratumId, StratumSharedState>,
    #[serde(with = "occupancy_state_map")]
    pub occupancies: BTreeMap<OccupancyId, OccupancyState>,
    pub last_transaction_id: u128,
}

impl CoupledOwnedState {
    pub fn parse_strict(
        bytes: &[u8],
        config: &VegetationConfiguration,
    ) -> Result<Self, VegetationError> {
        let structural: serde_json::Value = serde_json::from_slice(bytes)
            .map_err(|error| VegetationError::Schema(error.to_string()))?;
        state_shape::validate(&structural)?;
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| VegetationError::Schema(error.to_string()))?;
        value.validate(config)?;
        Ok(value)
    }

    #[allow(clippy::too_many_lines)]
    pub fn validate(&self, config: &VegetationConfiguration) -> Result<(), VegetationError> {
        self.validate_for_model(config, MODEL_SHA256, true)
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn validate_historical(
        &self,
        config: &VegetationConfiguration,
        expected_model_sha256: &str,
    ) -> Result<(), VegetationError> {
        self.validate_for_model(config, expected_model_sha256, false)
    }

    #[allow(clippy::too_many_lines)]
    fn validate_for_model(
        &self,
        config: &VegetationConfiguration,
        expected_model_sha256: &str,
        enforce_v7_semantics: bool,
    ) -> Result<(), VegetationError> {
        if enforce_v7_semantics {
            config.validate()?;
        } else {
            config.validate_historical(expected_model_sha256)?;
        }
        if self.model_definition_sha256.len() != 64
            || self.configuration_sha256.len() != 64
            || self.state_sha256.len() != 64
        {
            return Err(VegetationError::Domain("state identity"));
        }
        if self.model_definition_sha256 != expected_model_sha256
            || self.model_definition_sha256 != config.model_definition_sha256
            || self.configuration_sha256 != config.configuration_sha256
        {
            return Err(VegetationError::Receipt(
                "model/configuration state identity".into(),
            ));
        }
        if self.state_sha256 != self.canonical_sha256()? {
            return Err(VegetationError::Receipt(
                "state digest does not match canonical bytes".into(),
            ));
        }
        if self.last_transaction_id == 0 && config.initial_state_sha256 != self.state_sha256 {
            return Err(VegetationError::Receipt(
                "configuration initial-state identity".into(),
            ));
        }
        let configured_strata = config
            .strata
            .iter()
            .map(|stratum| stratum.stratum_id.clone())
            .collect::<std::collections::BTreeSet<_>>();
        let state_strata = self
            .strata
            .keys()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        if state_strata != configured_strata {
            return Err(VegetationError::Domain(
                "configuration/state stratum identity",
            ));
        }
        let expected_occupancies = config.expected_occupancies();
        let state_occupancies = self
            .occupancies
            .keys()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        if state_occupancies != expected_occupancies {
            return Err(VegetationError::Domain("V4 occupancy state identity"));
        }
        let expected_previous_transaction_id =
            (self.last_transaction_id != 0).then_some(self.last_transaction_id);
        for (occupancy_id, state) in &self.occupancies {
            config
                .strata
                .iter()
                .find(|stratum| stratum.stratum_id == occupancy_id.stratum_id)
                .ok_or(VegetationError::Domain("V4 occupancy stratum identity"))?;
            state
                .validate(expected_previous_transaction_id)
                .map_err(|error| VegetationError::Receipt(error.to_string()))?;
        }
        let mut transfer_identities = std::collections::BTreeSet::new();
        for (stratum_id, state) in &self.strata {
            if state.last_transaction_id != self.last_transaction_id {
                return Err(VegetationError::Receipt(
                    "shared stratum transaction lineage".into(),
                ));
            }
            let scalars = [
                state.retranslocation_n,
                state.nsc_c,
                state.xs_c,
                state.standing_dead.carbon,
                state.standing_dead.nitrogen,
                state.standing_dead_dm,
                state.onset_remaining_s,
                state.offset_remaining_s,
                state.previous_gsi,
                state.t10_k,
                state.leaf_area,
                state.root_area,
                state.stem_area,
            ];
            if scalars.iter().any(|value| !value.is_finite())
                || state.retranslocation_n < 0.0
                || state.nsc_c < 0.0
                || state.standing_dead.carbon < 0.0
                || state.standing_dead.nitrogen < 0.0
                || state.standing_dead_dm < 0.0
                || state.onset_remaining_s < 0.0
                || state.offset_remaining_s < 0.0
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
            let expected_owner_id = format!("stratum:{}", stratum_id.as_str());
            for transfer in &state.pending_transfers {
                let identity = (
                    transfer.transaction_id,
                    transfer.owner_id.clone(),
                    transfer.proposal_id,
                );
                if transfer.transaction_id == 0
                    || transfer.transaction_id != self.last_transaction_id
                    || transfer.owner_id.as_str() != expected_owner_id
                    || transfer.proposal_id == 0
                    || [transfer.carbon, transfer.nitrogen, transfer.dry_matter]
                        .iter()
                        .any(|value| !value.is_finite() || *value < 0.0)
                    || !transfer_identities.insert(identity)
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
            let stratum = config
                .strata
                .iter()
                .find(|candidate| candidate.stratum_id == *stratum_id)
                .ok_or(VegetationError::Domain("missing stratum configuration"))?;
            if enforce_v7_semantics
                && stratum.phenology_type == crate::PhenologyType::Evergreen
                && state.tissues.values().any(|pool| {
                    pool.storage.carbon != 0.0
                        || pool.storage.nitrogen != 0.0
                        || pool.transfer.carbon != 0.0
                        || pool.transfer.nitrogen != 0.0
                })
            {
                return Err(VegetationError::Domain(
                    "V7 evergreen storage/transfer state",
                ));
            }
            validate_displayed_leaf_identity(state, stratum)?;
        }
        Ok(())
    }

    pub fn canonical_sha256(&self) -> Result<String, VegetationError> {
        Ok(state_canonical::sha256(self))
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
    fn authorize(
        &self,
        requests: &[WaterRequest],
    ) -> Result<crate::water_phase::WaterArbitration, VegetationError>;
    /// Construct the water owner's uncommitted debit candidate from the exact
    /// resource protocol. Vegetation independently validates the returned
    /// candidate and cannot mutate owner state through this interface.
    fn candidate_from_finalized_use(
        &self,
        transaction_id: TransactionId,
        arbitration: &crate::water_phase::WaterArbitration,
        finalized_uses: &[WaterUse],
    ) -> Result<crate::water_phase::WaterOwnerCandidate, VegetationError>;
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

/// Executes the complete uncommitted V7 water and mineral-nitrogen phases.
///
/// Persistent multi-owner candidate construction remains deliberately
/// fail-closed.
pub fn execute_candidate(
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    water: &dyn WaterArbiter,
    nitrogen: &dyn NitrogenArbiter,
) -> Result<CoupledCandidate, VegetationError> {
    let water_phase = crate::water_phase::execute_uncommitted_water_phase(
        model, config, beginning, forcing, water,
    )?;
    let nitrogen_phase = crate::persistent_phase::execute_uncommitted_nitrogen_phase(
        config,
        beginning,
        forcing,
        &water_phase,
        nitrogen,
    )?;
    let vegetation_candidate =
        crate::vegetation_candidate::construct_uncommitted_vegetation_candidate(
            config,
            beginning,
            &water_phase,
            &nitrogen_phase,
        )?;
    vegetation_candidate.validate_sealed()?;
    Err(VegetationError::Unsupported(
        "V7 post-nitrogen multi-owner candidate is implementation-incomplete",
    ))
}

/// Failure-injection entry point retained while the post-water transaction is incomplete.
pub fn execute_candidate_with_failure(
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    water: &dyn WaterArbiter,
    nitrogen: &dyn NitrogenArbiter,
    failure: Option<FailurePoint>,
) -> Result<CoupledCandidate, VegetationError> {
    let water_phase = crate::water_phase::execute_uncommitted_water_phase_with_failure(
        model, config, beginning, forcing, water, failure,
    )?;
    if failure == Some(FailurePoint::NitrogenRequest) {
        return Err(VegetationError::InjectedFailure("nitrogen request"));
    }
    let nitrogen_phase = crate::persistent_phase::execute_uncommitted_nitrogen_phase(
        config,
        beginning,
        forcing,
        &water_phase,
        nitrogen,
    )?;
    let vegetation_candidate =
        crate::vegetation_candidate::construct_uncommitted_vegetation_candidate(
            config,
            beginning,
            &water_phase,
            &nitrogen_phase,
        )?;
    vegetation_candidate.validate_sealed()?;
    let label = match failure {
        Some(FailurePoint::NitrogenAuthorization) => Some("nitrogen authorization"),
        Some(FailurePoint::Allocation) => Some("allocation"),
        Some(FailurePoint::ReceiverConstruction) => Some("receiver construction"),
        Some(FailurePoint::ClosureValidation) => Some("closure validation"),
        Some(FailurePoint::BeforeCommit) => Some("before commit"),
        _ => None,
    };
    if let Some(label) = label {
        return Err(VegetationError::InjectedFailure(label));
    }
    Err(VegetationError::Unsupported(
        "V7 post-nitrogen multi-owner candidate is implementation-incomplete",
    ))
}

#[allow(clippy::too_many_lines)]
pub(crate) fn validate_execution(
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
) -> Result<(), VegetationError> {
    let actual_model_sha = format!("{:x}", Sha256::digest(model.bytes));
    if model.version != crate::MODEL_VERSION
        || model.sha256 != MODEL_SHA256
        || model.bytes != MODEL_BYTES
        || actual_model_sha != MODEL_SHA256
    {
        return Err(VegetationError::ModelDigestMismatch {
            expected: MODEL_SHA256.into(),
            found: actual_model_sha,
        });
    }
    validate_candidate_inputs(config, beginning, forcing)?;
    if beginning.model_definition_sha256 != model.sha256 {
        return Err(VegetationError::Receipt(
            "model/configuration state identity".into(),
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
pub(crate) fn validate_candidate_inputs(
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
) -> Result<(), VegetationError> {
    config.validate()?;
    beginning.validate(config)?;
    if beginning
        .strata
        .values()
        .any(|stratum| !stratum.pending_transfers.is_empty())
    {
        return Err(VegetationError::Receipt(
            "unresolved beginning-state material transfer".into(),
        ));
    }
    if beginning.configuration_sha256 != config.configuration_sha256 {
        return Err(VegetationError::Receipt(
            "model/configuration state identity".into(),
        ));
    }
    if beginning.last_transaction_id == 0 && beginning.state_sha256 != config.initial_state_sha256 {
        return Err(VegetationError::Receipt(
            "initial-state digest does not match configuration".into(),
        ));
    }
    for stratum in &config.strata {
        let state = beginning
            .strata
            .get(&stratum.stratum_id)
            .ok_or(VegetationError::Domain("missing stratum state"))?;
        validate_displayed_leaf_identity(state, stratum)?;
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
    for layer in &forcing.soil_layers {
        if [
            layer.water_beginning_kg_m2,
            layer.matric_potential_mm,
            layer.hydraulic_conductivity_mm_s,
            layer.root_path_length_mm,
            layer.gravity_root_mm,
            layer.temperature_k,
        ]
        .iter()
        .any(|value| !value.is_finite())
            || layer.water_beginning_kg_m2 < 0.0
            || layer.hydraulic_conductivity_mm_s < 0.0
            || layer.root_path_length_mm <= 0.0
            || layer.temperature_k <= 0.0
        {
            return Err(VegetationError::Domain("soil layer forcing"));
        }
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn v7_identity_rebound_fixture() -> (VegetationConfiguration, CoupledOwnedState) {
    let mut config: VegetationConfiguration = serde_json::from_slice(include_bytes!(
        "../../../tests/fixtures/c3_woody_v5_diagnostic_configuration.json"
    ))
    .expect("historical V5 configuration DTO");
    let mut state: CoupledOwnedState = serde_json::from_slice(include_bytes!(
        "../../../tests/fixtures/c3_woody_v5_diagnostic_state.json"
    ))
    .expect("historical V5 state DTO");

    crate::migration::validate_v5_initial_fixture(&config, &state)
        .expect("complete historical V5 fixture identity, digest, and lineage");

    for stratum in &mut config.strata {
        if stratum.phenology_type == crate::PhenologyType::Evergreen {
            stratum.current_growth_fraction = 1.0;
        }
    }
    config.model_definition_sha256 = MODEL_SHA256.into();
    config.configuration_sha256 = config.canonical_sha256().expect("V7 configuration digest");
    state.model_definition_sha256 = MODEL_SHA256.into();
    state.configuration_sha256 = config.configuration_sha256.clone();
    state.state_sha256 = state.canonical_sha256().expect("V7 state digest");
    config.initial_state_sha256 = state.state_sha256.clone();

    config
        .validate()
        .expect("identity-rebound V7 configuration");
    state.validate(&config).expect("identity-rebound V7 state");
    (config, state)
}

#[cfg(test)]
#[allow(clippy::items_after_test_module)]
mod milestone_one_tests {
    use super::*;
    use crate::{RootLayer, StratumConfiguration, TopologyTile};
    use openwepp_kernel_contract::{
        MaterialDonorClass, MaterialReceiverClass, ResourceOwnerId, TileId,
    };

    type LaneMutation = fn(&mut OccupancyState);

    fn stratum_id(value: &str) -> StratumId {
        StratumId::try_new(value).expect("stratum ID")
    }

    fn tile_id(value: &str) -> TileId {
        TileId::try_new(value).expect("tile ID")
    }

    fn layer_id(value: &str) -> SoilLayerId {
        SoilLayerId::try_new(value).expect("layer ID")
    }

    fn occupancy(stratum: &str, tile: &str) -> OccupancyId {
        OccupancyId {
            stratum_id: stratum_id(stratum),
            tile_id: tile_id(tile),
        }
    }

    fn fixture_config() -> VegetationConfiguration {
        let (mut config, _) = v7_identity_rebound_fixture();
        config.initial_state_sha256 = "0".repeat(64);
        config.topology_tiles = vec![
            TopologyTile {
                tile_id: tile_id("tile-a"),
                fraction: 0.35,
            },
            TopologyTile {
                tile_id: tile_id("tile-b"),
                fraction: 0.65,
            },
        ];
        let mut upper = config.strata.remove(0);
        upper.stratum_id = stratum_id("upper");
        upper.tile_ids = vec![tile_id("tile-a"), tile_id("tile-b")];
        let mut second_root: RootLayer = upper.root_layers[0].clone();
        upper.root_layers[0].root_fraction = 0.5;
        upper.root_layers[0].mineral_n_root_fraction = 0.5;
        second_root.layer_id = layer_id("soil-2");
        second_root.root_fraction = 0.5;
        second_root.mineral_n_root_fraction = 0.5;
        upper.root_layers.push(second_root.clone());
        let mut lower: StratumConfiguration = upper.clone();
        lower.stratum_id = stratum_id("lower");
        lower.tile_ids = vec![tile_id("tile-a")];
        lower.vertical_rank = 1;
        lower.height_m = upper.height_m / 2.0;
        lower.crown_base_m = lower.height_m / 2.0;
        lower.root_layers = vec![second_root];
        lower.root_layers[0].root_fraction = 1.0;
        lower.root_layers[0].mineral_n_root_fraction = 1.0;
        config.strata = vec![upper, lower];
        config.configuration_sha256 = config.canonical_sha256().expect("config digest");
        config
    }

    fn shared_state() -> StratumSharedState {
        let (_, mut state) = v7_identity_rebound_fixture();
        state
            .strata
            .remove(&stratum_id("tree-1"))
            .expect("V5 shared state")
    }

    fn lane(seed: f64, _roots: &[&str]) -> OccupancyState {
        OccupancyState {
            beta_hyd: 0.5 + seed / 100.0,
            canopy_air_specific_humidity_kg_kg: 0.009 + seed / 10_000.0,
            canopy_air_temperature_k: 294.0 + seed,
            canopy_liquid_kg_h2o_m2_tile_ground: seed / 100.0,
            dry_stem_temperature_k: 293.0 + seed,
            last_accepted_transaction_id: None,
            root_node_potential_mm: -5_000.0 - seed,
            shade_ci_pa: 27.0 + seed,
            shade_leaf_potential_mm: -7_000.0 - seed,
            shade_leaf_temperature_k: 294.5 + seed,
            stem_potential_mm: -6_000.0 - seed,
            sun_ci_pa: 25.0 + seed,
            sun_leaf_potential_mm: -7_500.0 - seed,
            sun_leaf_temperature_k: 295.0 + seed,
            wet_surface_temperature_k: 293.5 + seed,
        }
    }

    fn fixture() -> (VegetationConfiguration, CoupledOwnedState) {
        let mut config = fixture_config();
        let shared = shared_state();
        let mut state = CoupledOwnedState {
            model_definition_sha256: MODEL_SHA256.into(),
            configuration_sha256: config.configuration_sha256.clone(),
            state_sha256: String::new(),
            strata: BTreeMap::from([
                (stratum_id("upper"), shared.clone()),
                (stratum_id("lower"), shared),
            ]),
            occupancies: BTreeMap::from([
                (
                    occupancy("upper", "tile-a"),
                    lane(1.0, &["soil-1", "soil-2"]),
                ),
                (
                    occupancy("upper", "tile-b"),
                    lane(2.0, &["soil-1", "soil-2"]),
                ),
                (occupancy("lower", "tile-a"), lane(3.0, &["soil-2"])),
            ]),
            last_transaction_id: 0,
        };
        state.state_sha256 = state.canonical_sha256().expect("state digest");
        config.initial_state_sha256 = state.state_sha256.clone();
        (config, state)
    }

    fn refresh_state(state: &mut CoupledOwnedState, config: &mut VegetationConfiguration) {
        state.state_sha256 = state.canonical_sha256().expect("state digest");
        if state.last_transaction_id == 0 {
            config.initial_state_sha256 = state.state_sha256.clone();
        }
    }

    fn accepted_with_transfer() -> (VegetationConfiguration, CoupledOwnedState) {
        let (config, mut state) = fixture();
        state.last_transaction_id = 7;
        for shared in state.strata.values_mut() {
            shared.last_transaction_id = 7;
        }
        for lane in state.occupancies.values_mut() {
            lane.last_accepted_transaction_id = Some(7);
        }
        state
            .strata
            .get_mut(&stratum_id("upper"))
            .expect("upper stratum")
            .pending_transfers
            .push(MaterialTransfer {
                transaction_id: 7,
                owner_id: ResourceOwnerId::try_new("stratum:upper").expect("owner identity"),
                proposal_id: 1,
                donor: MaterialDonorClass::Leaf,
                receiver: MaterialReceiverClass::Metabolic,
                carbon: 0.01,
                nitrogen: 0.001,
                dry_matter: 0.02,
            });
        state.state_sha256 = state.canonical_sha256().expect("accepted digest");
        (config, state)
    }

    #[test]
    fn complete_two_tile_two_stratum_state_is_exact() {
        let (config, state) = fixture();
        state.validate(&config).expect("complete V5 state");
        assert_eq!(
            state
                .occupancies
                .keys()
                .cloned()
                .collect::<std::collections::BTreeSet<_>>(),
            config.expected_occupancies()
        );
        assert_eq!(state.strata.len(), 2);
        assert_eq!(state.occupancies.len(), 3);
        let bytes = serde_json::to_vec(&state).expect("canonical bytes");
        assert_eq!(CoupledOwnedState::parse_strict(&bytes, &config), Ok(state));
    }

    #[test]
    fn v2_named_configuration_and_state_fixtures_are_historical_only() {
        assert!(
            VegetationConfiguration::parse_strict(include_bytes!(
                "../../../tests/fixtures/c3_woody_v2_diagnostic_configuration.json"
            ))
            .is_err()
        );
        assert!(
            serde_json::from_slice::<CoupledOwnedState>(include_bytes!(
                "../../../tests/fixtures/c3_woody_v2_diagnostic_state.json"
            ))
            .is_err()
        );
    }

    #[test]
    fn v3_named_state_fixture_is_historical_only() {
        let (_, state) = fixture();
        let bytes = include_bytes!("../../../tests/fixtures/c3_woody_v3_diagnostic_state.json");
        assert!(serde_json::from_slice::<CoupledOwnedState>(bytes).is_err());
        assert_ne!(
            bytes.as_slice(),
            serde_json::to_vec(&state).expect("V5 state")
        );
    }

    #[test]
    fn v5_named_configuration_and_state_fixtures_are_historical_only() {
        let config: VegetationConfiguration = serde_json::from_slice(include_bytes!(
            "../../../tests/fixtures/c3_woody_v5_diagnostic_configuration.json"
        ))
        .expect("V5 configuration DTO");
        let state: CoupledOwnedState = serde_json::from_slice(include_bytes!(
            "../../../tests/fixtures/c3_woody_v5_diagnostic_state.json"
        ))
        .expect("V5 state DTO");
        assert_eq!(
            state.state_sha256,
            state.canonical_sha256().expect("state digest")
        );
        assert_eq!(config.initial_state_sha256, state.state_sha256);
        assert_eq!(state.configuration_sha256, config.configuration_sha256);
        assert!(matches!(
            config.validate(),
            Err(VegetationError::ModelDigestMismatch { .. })
        ));
        assert!(matches!(
            state.validate(&config),
            Err(VegetationError::ModelDigestMismatch { .. })
        ));
    }

    #[test]
    fn v4_occupancy_entries_reject_legacy_tuple_sequences() {
        let (config, _) = v7_identity_rebound_fixture();
        let state: serde_json::Value = serde_json::from_slice(include_bytes!(
            "../../../tests/fixtures/c3_woody_v5_diagnostic_state.json"
        ))
        .expect("V5 state JSON");
        let mut tuple = state;
        let entry = tuple["occupancies"][0].clone();
        tuple["occupancies"][0] = serde_json::json!([entry["identity"], entry["state"]]);
        assert!(matches!(
            CoupledOwnedState::parse_strict(
                &serde_json::to_vec(&tuple).expect("tuple poison bytes"),
                &config
            ),
            Err(VegetationError::Schema(message))
                if message.contains("V4 occupancy entry")
        ));
    }

    #[test]
    fn v4_strict_parser_rejects_every_nested_positional_record_alias() {
        fn sequence(value: &serde_json::Value) -> serde_json::Value {
            serde_json::Value::Array(
                value
                    .as_object()
                    .expect("record object")
                    .values()
                    .cloned()
                    .collect(),
            )
        }

        let (config, state) = accepted_with_transfer();
        let base = serde_json::to_value(state).expect("accepted state JSON");
        let mut poisons = Vec::new();

        let mut value = base.clone();
        value["strata"]["upper"] = sequence(&value["strata"]["upper"]);
        poisons.push(value);

        let mut value = base.clone();
        value["strata"]["upper"]["tissues"]["leaf"] =
            sequence(&value["strata"]["upper"]["tissues"]["leaf"]);
        poisons.push(value);

        let mut value = base.clone();
        value["strata"]["upper"]["tissues"]["leaf"]["display"] =
            sequence(&value["strata"]["upper"]["tissues"]["leaf"]["display"]);
        poisons.push(value);

        let mut value = base.clone();
        value["strata"]["upper"]["standing_dead"] =
            sequence(&value["strata"]["upper"]["standing_dead"]);
        poisons.push(value);

        let mut value = base.clone();
        value["strata"]["upper"]["pending_transfers"][0] =
            sequence(&value["strata"]["upper"]["pending_transfers"][0]);
        poisons.push(value);

        let mut value = base.clone();
        value["occupancies"][0]["identity"] = sequence(&value["occupancies"][0]["identity"]);
        poisons.push(value);

        let mut value = base.clone();
        value["occupancies"][0]["state"] = sequence(&value["occupancies"][0]["state"]);
        poisons.push(value);

        for poison in poisons {
            assert!(matches!(
                CoupledOwnedState::parse_strict(
                    &serde_json::to_vec(&poison).expect("shape poison bytes"),
                    &config
                ),
                Err(VegetationError::Schema(_))
            ));
        }
    }

    #[test]
    fn v4_strict_parser_rejects_nested_occupancy_identity_unknown_field() {
        let (config, state) = fixture();
        let mut poison = serde_json::to_value(state).expect("state JSON");
        poison["occupancies"][0]["identity"]["unknown"] = 1.into();
        assert!(matches!(
            CoupledOwnedState::parse_strict(
                &serde_json::to_vec(&poison).expect("identity poison bytes"),
                &config
            ),
            Err(VegetationError::Schema(message))
                if message.contains("V4 occupancy identity")
        ));
    }

    #[test]
    fn v4_area_caches_are_bit_exact_and_display_leaf_only() {
        let (mut config, state) = fixture();
        let upper = stratum_id("upper");

        let mut donor_poison = state.clone();
        let leaf = donor_poison
            .strata
            .get_mut(&upper)
            .expect("upper")
            .tissues
            .get_mut(&Tissue::Leaf)
            .expect("leaf");
        leaf.storage.carbon = 10_000.0;
        leaf.transfer.carbon = 20_000.0;
        refresh_state(&mut donor_poison, &mut config);
        donor_poison
            .validate_historical(&config, MODEL_SHA256)
            .expect("non-displayed C cannot own area");

        let mut one_bit = state.clone();
        let shared = one_bit.strata.get_mut(&upper).expect("upper");
        shared.leaf_area = f64::from_bits(shared.leaf_area.to_bits() + 1);
        refresh_state(&mut one_bit, &mut config);
        assert_eq!(
            one_bit.validate(&config),
            Err(VegetationError::Domain("V4 displayed-leaf area identity"))
        );

        let mut zero = state;
        let shared = zero.strata.get_mut(&upper).expect("upper");
        let leaf = shared.tissues.get_mut(&Tissue::Leaf).expect("leaf");
        leaf.display.carbon = 0.0;
        leaf.display.nitrogen = 0.0;
        leaf.storage.carbon = 5.0;
        leaf.transfer.carbon = 7.0;
        leaf.storage.nitrogen = 0.15;
        leaf.transfer.nitrogen = 0.18;
        shared.leaf_area = 0.0;
        shared.stem_area = 0.0;
        shared.root_area = 0.0;
        refresh_state(&mut zero, &mut config);
        zero.validate_historical(&config, MODEL_SHA256)
            .expect("zero displayed leaf has exact zero area");
        zero.strata
            .get_mut(&upper)
            .expect("upper")
            .tissues
            .get_mut(&Tissue::Leaf)
            .expect("leaf")
            .display
            .nitrogen = 0.001;
        refresh_state(&mut zero, &mut config);
        assert_eq!(
            zero.validate_historical(&config, MODEL_SHA256),
            Err(VegetationError::Domain("V4 displayed leaf N without LAI"))
        );
    }

    #[test]
    fn v4_state_rejects_removed_offset_fields() {
        let (config, state) = fixture();
        let mut value = serde_json::to_value(state).expect("V5 state value");
        value["strata"]["upper"]["previous_leaf_offset_flux"] = serde_json::Value::from(0.0);
        value["strata"]["upper"]["previous_root_offset_flux"] = serde_json::Value::from(0.0);
        let bytes = serde_json::to_vec(&value).expect("poison bytes");
        assert!(matches!(
            CoupledOwnedState::parse_strict(&bytes, &config),
            Err(VegetationError::Schema(_))
        ));
    }

    #[test]
    fn v4_digest_structurally_frames_arbitrary_occupancy_ids_without_collision() {
        let (_, mut state) = fixture();
        let source_lane = state
            .occupancies
            .get(&occupancy("upper", "tile-a"))
            .expect("source lane")
            .clone();
        state.occupancies.insert(
            OccupancyId {
                stratum_id: stratum_id("upper@tile"),
                tile_id: tile_id("a"),
            },
            source_lane.clone(),
        );
        state.occupancies.insert(
            OccupancyId {
                stratum_id: stratum_id("upper"),
                tile_id: tile_id("tile@a"),
            },
            source_lane.clone(),
        );
        state.occupancies.insert(
            OccupancyId {
                stratum_id: stratum_id("upper\nα"),
                tile_id: tile_id("tile\t雪"),
            },
            source_lane,
        );
        let digest = state
            .canonical_sha256()
            .expect("typed identity framing accepts arbitrary valid IDs");
        let removed = state
            .occupancies
            .remove(&occupancy("upper@tile", "a"))
            .expect("first formerly colliding identity");
        assert_ne!(state.canonical_sha256().expect("second digest"), digest);
        state
            .occupancies
            .insert(occupancy("upper@tile", "a"), removed);
        assert_eq!(state.canonical_sha256().expect("stable digest"), digest);
    }

    #[test]
    fn occupancy_set_rejects_missing_extra_wrong_membership_and_duplicate() {
        let (mut config, state) = fixture();
        let mut missing = state.clone();
        missing.occupancies.remove(&occupancy("upper", "tile-b"));
        refresh_state(&mut missing, &mut config);
        assert_eq!(
            missing.validate(&config),
            Err(VegetationError::Domain("V4 occupancy state identity"))
        );

        let mut extra = state.clone();
        extra
            .occupancies
            .insert(occupancy("lower", "tile-b"), lane(4.0, &["soil-2"]));
        refresh_state(&mut extra, &mut config);
        assert_eq!(
            extra.validate(&config),
            Err(VegetationError::Domain("V4 occupancy state identity"))
        );

        let mut missing_stratum = state.clone();
        missing_stratum.strata.remove(&stratum_id("lower"));
        refresh_state(&mut missing_stratum, &mut config);
        assert_eq!(
            missing_stratum.validate(&config),
            Err(VegetationError::Domain(
                "configuration/state stratum identity"
            ))
        );

        let mut extra_stratum = state.clone();
        extra_stratum
            .strata
            .insert(stratum_id("absent"), shared_state());
        refresh_state(&mut extra_stratum, &mut config);
        assert_eq!(
            extra_stratum.validate(&config),
            Err(VegetationError::Domain(
                "configuration/state stratum identity"
            ))
        );

        let mut value = serde_json::to_value(&state).expect("state value");
        let lanes = value
            .get_mut("occupancies")
            .and_then(serde_json::Value::as_array_mut)
            .expect("lanes");
        lanes.push(lanes[0].clone());
        let bytes = serde_json::to_vec(&value).expect("duplicate bytes");
        assert!(
            matches!(CoupledOwnedState::parse_strict(&bytes, &config), Err(VegetationError::Schema(message)) if message.contains("duplicate V4 occupancy"))
        );
    }

    #[test]
    fn strict_state_parser_rejects_duplicate_strata_tissues_and_transfer_fields() {
        let (config, state) = fixture();
        let upper = state.strata.get(&stratum_id("upper")).expect("upper");
        let upper_json = serde_json::to_string(upper).expect("upper JSON");
        let text = serde_json::to_string(&state).expect("state JSON");
        let duplicate_stratum = text.replacen(
            "\"strata\":{",
            &format!("\"strata\":{{\"upper\":{upper_json},"),
            1,
        );
        assert!(
            matches!(CoupledOwnedState::parse_strict(duplicate_stratum.as_bytes(), &config), Err(VegetationError::Schema(message)) if message.contains("duplicate V4 shared-stratum"))
        );

        let leaf_json =
            serde_json::to_string(upper.tissues.get(&Tissue::Leaf).expect("leaf tissue"))
                .expect("leaf JSON");
        let duplicate_tissue = text.replacen(
            "\"tissues\":{",
            &format!("\"tissues\":{{\"leaf\":{leaf_json},"),
            1,
        );
        assert!(
            matches!(CoupledOwnedState::parse_strict(duplicate_tissue.as_bytes(), &config), Err(VegetationError::Schema(message)) if message.contains("duplicate V4 tissue"))
        );

        let (accepted_config, accepted) = accepted_with_transfer();
        let mut value = serde_json::to_value(accepted).expect("accepted value");
        value["strata"]["upper"]["pending_transfers"][0]["unknown"] = 1.into();
        assert!(matches!(
            CoupledOwnedState::parse_strict(
                &serde_json::to_vec(&value).expect("poison bytes"),
                &accepted_config
            ),
            Err(VegetationError::Schema(_))
        ));

        value["strata"]["upper"]["pending_transfers"][0]
            .as_object_mut()
            .expect("transfer object")
            .remove("unknown");
        value["strata"]["upper"]["pending_transfers"][0]["owner_id"] = "   ".into();
        assert!(matches!(
            CoupledOwnedState::parse_strict(
                &serde_json::to_vec(&value).expect("owner poison bytes"),
                &accepted_config
            ),
            Err(VegetationError::Schema(_))
        ));
    }

    #[test]
    fn shared_state_rejects_negative_owned_amounts_and_invalid_transfer_lineage() {
        let (mut config, state) = fixture();
        for mutate in [
            |shared: &mut StratumSharedState| shared.standing_dead.carbon = -1.0,
            |shared: &mut StratumSharedState| shared.standing_dead.nitrogen = -1.0,
            |shared: &mut StratumSharedState| shared.standing_dead_dm = -1.0,
            |shared: &mut StratumSharedState| shared.onset_remaining_s = -1.0,
            |shared: &mut StratumSharedState| shared.offset_remaining_s = -1.0,
        ] {
            let mut poison = state.clone();
            mutate(poison.strata.get_mut(&stratum_id("upper")).expect("upper"));
            refresh_state(&mut poison, &mut config);
            assert_eq!(
                poison.validate(&config),
                Err(VegetationError::Domain("complete stratum state"))
            );
        }

        let (config, accepted) = accepted_with_transfer();
        for mutate in [
            |transfer: &mut MaterialTransfer| transfer.transaction_id = 6,
            |transfer: &mut MaterialTransfer| {
                transfer.owner_id =
                    ResourceOwnerId::try_new("stratum:lower").expect("owner identity");
            },
            |transfer: &mut MaterialTransfer| transfer.proposal_id = 0,
        ] {
            let mut poison = accepted.clone();
            mutate(
                &mut poison
                    .strata
                    .get_mut(&stratum_id("upper"))
                    .expect("upper")
                    .pending_transfers[0],
            );
            poison.state_sha256 = poison.canonical_sha256().expect("poison digest");
            assert_eq!(
                poison.validate(&config),
                Err(VegetationError::Domain("pending material transfer"))
            );
        }

        let mut duplicate = accepted;
        let transfer = duplicate
            .strata
            .get(&stratum_id("upper"))
            .expect("upper")
            .pending_transfers[0]
            .clone();
        duplicate
            .strata
            .get_mut(&stratum_id("upper"))
            .expect("upper")
            .pending_transfers
            .push(transfer);
        duplicate.state_sha256 = duplicate.canonical_sha256().expect("duplicate digest");
        assert_eq!(
            duplicate.validate(&config),
            Err(VegetationError::Domain("pending material transfer"))
        );
    }

    #[test]
    fn identity_and_historical_v1_inputs_fail_closed() {
        let (mut config, state) = fixture();
        let mut poison = state.clone();
        poison.model_definition_sha256 = "1".repeat(64);
        refresh_state(&mut poison, &mut config);
        assert!(matches!(
            poison.validate(&config),
            Err(VegetationError::Receipt(_))
        ));
        let mut poison = state.clone();
        poison.configuration_sha256 = "2".repeat(64);
        poison.state_sha256 = poison.canonical_sha256().expect("digest");
        assert!(matches!(
            poison.validate(&config),
            Err(VegetationError::Receipt(_))
        ));
        let mut poison = state;
        poison.state_sha256 = "3".repeat(64);
        assert!(matches!(
            poison.validate(&config),
            Err(VegetationError::Receipt(_))
        ));
        assert!(matches!(
            CoupledOwnedState::parse_strict(
                include_bytes!("../../../tests/fixtures/c3_woody_v1_diagnostic_state.json"),
                &config
            ),
            Err(VegetationError::Schema(_))
        ));
    }

    #[test]
    fn complete_state_digest_binds_order_identity_and_every_lane_field() {
        let (config, state) = fixture();
        let original = state.canonical_sha256().expect("digest");
        assert_eq!(original, state.state_sha256);
        let bytes = serde_json::to_vec(&state).expect("bytes");
        let mut value: serde_json::Value = serde_json::from_slice(&bytes).expect("value");
        value["occupancies"]
            .as_array_mut()
            .expect("lanes")
            .reverse();
        let mut reordered: CoupledOwnedState = serde_json::from_value(value).expect("state");
        reordered.state_sha256 = reordered.canonical_sha256().expect("digest");
        assert_eq!(reordered.state_sha256, original);
        reordered.validate(&config).expect("order independent");

        let mutations: [LaneMutation; 15] = [
            |s| s.beta_hyd += 0.01,
            |s| s.canopy_air_specific_humidity_kg_kg += 0.001,
            |s| s.canopy_air_temperature_k += 0.1,
            |s| s.canopy_liquid_kg_h2o_m2_tile_ground += 0.01,
            |s| s.dry_stem_temperature_k += 0.1,
            |s| s.last_accepted_transaction_id = Some(1),
            |s| s.root_node_potential_mm -= 1.0,
            |s| s.shade_ci_pa += 0.1,
            |s| s.shade_leaf_potential_mm -= 1.0,
            |s| s.shade_leaf_temperature_k += 0.1,
            |s| s.stem_potential_mm -= 1.0,
            |s| s.sun_ci_pa += 0.1,
            |s| s.sun_leaf_potential_mm -= 1.0,
            |s| s.sun_leaf_temperature_k += 0.1,
            |s| s.wet_surface_temperature_k += 0.1,
        ];
        for mutate in mutations {
            let mut changed = state.clone();
            mutate(
                changed
                    .occupancies
                    .get_mut(&occupancy("upper", "tile-a"))
                    .expect("lane"),
            );
            assert_ne!(changed.canonical_sha256().expect("digest"), original);
        }
        for mutate in [
            |s: &mut CoupledOwnedState| s.configuration_sha256 = "4".repeat(64),
            |s: &mut CoupledOwnedState| {
                let lane = s
                    .occupancies
                    .remove(&occupancy("upper", "tile-a"))
                    .expect("lane");
                s.occupancies.insert(occupancy("upper", "tile-b-x"), lane);
            },
        ] {
            let mut changed = state.clone();
            mutate(&mut changed);
            assert_ne!(changed.canonical_sha256().expect("digest"), original);
        }
    }

    #[test]
    fn root_node_units_and_transaction_lineage_are_exact() {
        let (mut config, state) = fixture();
        let mut reordered = state.clone();
        reordered
            .occupancies
            .get_mut(&occupancy("upper", "tile-a"))
            .expect("lane")
            .root_node_potential_mm -= 1.0;
        refresh_state(&mut reordered, &mut config);
        reordered
            .validate(&config)
            .expect("one common root node is independent of root-layer order");

        let mut accepted = state.clone();
        accepted.last_transaction_id = 7;
        for shared in accepted.strata.values_mut() {
            shared.last_transaction_id = 7;
        }
        for lane in accepted.occupancies.values_mut() {
            lane.last_accepted_transaction_id = Some(7);
        }
        accepted.state_sha256 = accepted.canonical_sha256().expect("digest");
        accepted.validate(&config).expect("preceding transaction");
        accepted
            .occupancies
            .get_mut(&occupancy("upper", "tile-a"))
            .expect("lane")
            .last_accepted_transaction_id = Some(6);
        accepted.state_sha256 = accepted.canonical_sha256().expect("digest");
        assert!(matches!(
            accepted.validate(&config),
            Err(VegetationError::Receipt(_))
        ));

        let mut value = serde_json::to_value(&state).expect("value");
        let lane = &mut value["occupancies"][0]["state"];
        lane["stem_potential_mpa"] = lane["stem_potential_mm"].take();
        assert!(serde_json::from_value::<CoupledOwnedState>(value).is_err());
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn public_transaction_finalizes_observed_two_ulp_nitrogen_case_then_fails_closed() {
        struct NoArbiter;
        impl WaterArbiter for NoArbiter {
            fn authorize(
                &self,
                requests: &[WaterRequest],
            ) -> Result<crate::water_phase::WaterArbitration, VegetationError> {
                let authorizations = requests
                    .iter()
                    .map(|request| WaterAuthorization {
                        transaction_id: request.transaction_id,
                        owner_id: request.owner_id.clone(),
                        key: request.key.clone(),
                        amount: request.amount,
                        basis: request.basis,
                    })
                    .collect::<Vec<_>>();
                let reasons: BTreeMap<_, _> = requests
                    .iter()
                    .map(|request| {
                        (
                            request.key.clone(),
                            if request.amount == 0.0 {
                                crate::water_phase::WaterAuthorizationReason::ZeroDemand
                            } else {
                                crate::water_phase::WaterAuthorizationReason::FullySupplied
                            },
                        )
                    })
                    .collect();
                let snapshot = crate::water_phase::WaterOwnerSnapshot::try_new(
                    requests[0].transaction_id,
                    requests[0].owner_id.clone(),
                    BTreeMap::from([(requests[0].key.layer_id.clone(), 10.0)]),
                    reasons.clone(),
                )?;
                crate::water_phase::WaterArbitration::try_new(snapshot, authorizations, reasons)
            }
            fn candidate_from_finalized_use(
                &self,
                transaction_id: TransactionId,
                arbitration: &crate::water_phase::WaterArbitration,
                finalized_uses: &[WaterUse],
            ) -> Result<crate::water_phase::WaterOwnerCandidate, VegetationError> {
                let ending = crate::water_phase::reconstruct_water_ending(
                    arbitration.snapshot(),
                    finalized_uses,
                )?;
                crate::water_phase::WaterOwnerCandidate::try_new(
                    transaction_id,
                    arbitration.snapshot().owner_id().clone(),
                    arbitration.snapshot().clone(),
                    ending,
                    finalized_uses.to_vec(),
                )
            }
        }
        impl NitrogenArbiter for NoArbiter {
            fn beginning_amount(&self, _: &MineralNitrogenKey) -> Result<f64, VegetationError> {
                Ok(0.0)
            }
            fn authorize(
                &self,
                requests: &[NitrogenRequest],
            ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
                Ok(requests
                    .iter()
                    .map(|request| NitrogenAuthorization {
                        transaction_id: request.transaction_id,
                        owner_id: request.owner_id.clone(),
                        key: request.key.clone(),
                        amount: request.amount,
                        basis: request.basis,
                    })
                    .collect())
            }
        }
        struct CountingNitrogen {
            calls: std::cell::Cell<u32>,
            requests: std::cell::RefCell<Vec<NitrogenRequest>>,
        }
        impl NitrogenArbiter for CountingNitrogen {
            fn beginning_amount(&self, _: &MineralNitrogenKey) -> Result<f64, VegetationError> {
                Ok(0.0)
            }
            fn authorize(
                &self,
                requests: &[NitrogenRequest],
            ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
                self.calls.set(self.calls.get() + 1);
                self.requests.replace(requests.to_vec());
                NitrogenArbiter::authorize(&NoArbiter, requests)
            }
        }
        let (config, state) = v7_identity_rebound_fixture();
        let model = crate::load_model_definition().expect("model");
        let forcing = SnowFreeForcing {
            air_temperature_k: 298.15,
            pressure_pa: 101_325.0,
            co2_pa: 42.0,
            vapor_pressure_deficit_kpa: 1.2,
            wind_m_s: 3.7,
            rain_kg_m2: 0.0,
            direct_par_w_m2: 410.0,
            diffuse_par_w_m2: 83.0,
            direct_nir_w_m2: 355.0,
            diffuse_nir_w_m2: 101.0,
            solar_zenith_cosine: 0.67,
            ground_albedo_vis: 0.14,
            ground_albedo_nir: 0.31,
            longwave_down_w_m2: 350.0,
            longwave_up_w_m2: 390.0,
            specific_humidity: 0.01,
            reference_height_m: 20.0,
            soil_layers: vec![SoilLayerForcing {
                layer_id: layer_id("soil-1"),
                water_beginning_kg_m2: 20.0,
                matric_potential_mm: -1000.0,
                hydraulic_conductivity_mm_s: 1e-5,
                root_path_length_mm: 100.0,
                gravity_root_mm: 500.0,
                temperature_k: 295.0,
                accessible: true,
                frozen: false,
            }],
            gsi: 1.0,
        };
        let beginning_bytes = serde_json::to_vec(&state).expect("beginning bytes");
        let counting_nitrogen = CountingNitrogen {
            calls: std::cell::Cell::new(0),
            requests: std::cell::RefCell::new(Vec::new()),
        };
        let water_phase = crate::water_phase::execute_uncommitted_water_phase(
            &model, &config, &state, &forcing, &NoArbiter,
        )
        .expect("complete water phase");
        let phase = crate::persistent_phase::execute_uncommitted_nitrogen_phase(
            &config,
            &state,
            &forcing,
            &water_phase,
            &counting_nitrogen,
        )
        .expect("two-ULP demand finalizes against immutable potential requests");
        assert_eq!(counting_nitrogen.calls.get(), 1);
        assert_eq!(
            counting_nitrogen.requests.borrow().as_slice(),
            phase.requests()
        );
        assert_eq!(phase.requests().len(), 2);
        assert_eq!(phase.authorizations().len(), 2);
        assert_eq!(phase.finalized_uses().len(), 2);
        let stratum = phase
            .strata()
            .get(&config.strata[0].stratum_id)
            .expect("stratum phase result");
        assert_eq!(
            stratum.potential_carbon_offer.offer.to_bits(),
            f64::from_bits(4_571_873_354_058_590_328).to_bits()
        );
        assert_eq!(
            stratum.final_carbon_offer.offer.to_bits(),
            f64::from_bits(4_571_873_354_058_590_330).to_bits()
        );
        assert_eq!(
            stratum
                .potential_request_batch
                .potential_total_demand()
                .to_bits(),
            f64::from_bits(4_546_826_747_422_758_608).to_bits()
        );
        assert_eq!(
            stratum.nitrogen_finalization.final_total_demand.to_bits(),
            f64::from_bits(4_546_826_747_422_758_610).to_bits()
        );
        assert_eq!(
            stratum.nitrogen_finalization.external_use.to_bits(),
            stratum.nitrogen_finalization.authorization_sum.to_bits()
        );
        assert!(
            stratum.nitrogen_finalization.final_external_demand
                > stratum.nitrogen_finalization.authorization_sum
        );
        assert!(stratum.growth_finalization.eta < 1.0);
        assert!(stratum.growth_finalization.nsc_next > 0.0);
        assert_eq!(
            stratum.candidate_after_growth.nsc_c.to_bits(),
            stratum.growth_finalization.nsc_next.to_bits()
        );
        for ((request, authorization), finalized) in phase
            .requests()
            .iter()
            .zip(phase.authorizations())
            .zip(phase.finalized_uses())
        {
            assert_eq!(request.key, authorization.key);
            assert_eq!(request.key, finalized.key);
            assert!(finalized.amount <= authorization.amount);
            assert!(authorization.amount <= request.amount);
        }
        let candidate = crate::vegetation_candidate::construct_uncommitted_vegetation_candidate(
            &config,
            &state,
            &water_phase,
            &phase,
        )
        .expect("sealed V7 vegetation candidate");
        let repeated_candidate =
            crate::vegetation_candidate::construct_uncommitted_vegetation_candidate(
                &config,
                &state,
                &water_phase,
                &phase,
            )
            .expect("byte-stable repeated V7 vegetation candidate");
        assert_eq!(repeated_candidate, candidate);
        let mut different_forcing = forcing.clone();
        different_forcing.rain_kg_m2 = 0.01;
        let different_water_phase = crate::water_phase::execute_uncommitted_water_phase(
            &model,
            &config,
            &state,
            &different_forcing,
            &NoArbiter,
        )
        .expect("distinct complete water phase");
        assert!(matches!(
            crate::vegetation_candidate::construct_uncommitted_vegetation_candidate(
                &config,
                &state,
                &different_water_phase,
                &phase,
            ),
            Err(VegetationError::V7CandidateRollback(
                "phase or beginning-state identity mismatch"
            ))
        ));
        candidate.validate_sealed().expect("sealed identity");
        assert_eq!(candidate.transaction_id(), TransactionId(1));
        assert_eq!(candidate.beginning_state_sha256(), state.state_sha256);
        assert_eq!(candidate.water_phase().transaction_id(), TransactionId(1));
        assert_eq!(
            candidate.nitrogen_phase().transaction_id(),
            TransactionId(1)
        );
        assert_eq!(candidate.ending_state().last_transaction_id, 1);
        candidate
            .ending_state()
            .validate(&config)
            .expect("accepted-shape ending vegetation state");
        assert_ne!(candidate.ending_state().state_sha256, state.state_sha256);
        assert!(
            candidate
                .ending_state()
                .occupancies
                .values()
                .all(|lane| lane.last_accepted_transaction_id == Some(1))
        );
        let ending_stratum = &candidate.ending_state().strata[&config.strata[0].stratum_id];
        let ending_leaf = ending_stratum.tissues[&Tissue::Leaf].display.carbon;
        assert_eq!(
            ending_stratum.leaf_area.to_bits(),
            (ending_leaf * config.strata[0].sla_m2_per_kg_c).to_bits()
        );
        assert_eq!(
            ending_stratum.stem_area.to_bits(),
            (ending_stratum.leaf_area * config.strata[0].sai_relation).to_bits()
        );
        assert_eq!(
            ending_stratum.root_area.to_bits(),
            ((ending_stratum.leaf_area + ending_stratum.stem_area)
                * config.strata[0].root_to_leaf_area)
                .to_bits()
        );
        assert!(ending_stratum.pending_transfers.is_empty());
        assert_eq!(candidate.carbon_ledgers().len(), config.strata.len());
        assert_eq!(candidate.nitrogen_ledgers().len(), config.strata.len());
        assert_eq!(candidate.dry_material_ledgers().len(), config.strata.len());
        assert!(!candidate.material_proposals().is_empty());
        for (index, proposal) in candidate.material_proposals().iter().enumerate() {
            assert_eq!(proposal.transaction_id, 1);
            assert_eq!(proposal.proposal_id, index as u64 + 1);
        }
        assert!(candidate.material_proposals().windows(2).all(|window| {
            let left = &window[0];
            let right = &window[1];
            (
                left.owner_id.as_str(),
                left.donor,
                left.receiver,
                left.proposal_id,
            ) < (
                right.owner_id.as_str(),
                right.donor,
                right.receiver,
                right.proposal_id,
            )
        }));
        let expected_strata = config
            .strata
            .iter()
            .map(|stratum| stratum.stratum_id.clone())
            .collect::<std::collections::BTreeSet<_>>();
        let validate_candidate_ledgers =
            |carbon: &[crate::vegetation_ledger::VegetationCarbonLedger],
             nitrogen: &[crate::vegetation_ledger::VegetationNitrogenLedger],
             dry: &[crate::vegetation_ledger::VegetationDryMaterialLedger]| {
                crate::vegetation_ledger::validate_vegetation_ledgers(
                    &expected_strata,
                    candidate.transaction_id(),
                    candidate.beginning_state_sha256(),
                    &candidate.ending_state().state_sha256,
                    carbon,
                    nitrogen,
                    dry,
                )
            };
        let mut corrupt_carbon = candidate.carbon_ledgers().to_vec();
        corrupt_carbon[0].ending_xs_c_kg_c_m2 += 1.0e-6;
        assert!(
            validate_candidate_ledgers(
                &corrupt_carbon,
                candidate.nitrogen_ledgers(),
                candidate.dry_material_ledgers(),
            )
            .is_err()
        );
        let mut forged_carbon = candidate.carbon_ledgers().to_vec();
        let mut forged_nitrogen = candidate.nitrogen_ledgers().to_vec();
        let mut forged_dry = candidate.dry_material_ledgers().to_vec();
        let forged_digest = "c".repeat(64);
        for ledger in &mut forged_carbon {
            ledger
                .identity
                .ending_state_sha256
                .clone_from(&forged_digest);
        }
        for ledger in &mut forged_nitrogen {
            ledger
                .identity
                .ending_state_sha256
                .clone_from(&forged_digest);
        }
        for ledger in &mut forged_dry {
            ledger
                .identity
                .ending_state_sha256
                .clone_from(&forged_digest);
        }
        assert!(validate_candidate_ledgers(&forged_carbon, &forged_nitrogen, &forged_dry).is_err());
        let mut corrupt_dry = candidate.dry_material_ledgers().to_vec();
        corrupt_dry[0].transfers[0].proposed_dry_matter_kg_m2 =
            corrupt_dry[0].transfers[0].carbon_kg_m2;
        assert!(
            validate_candidate_ledgers(
                candidate.carbon_ledgers(),
                candidate.nitrogen_ledgers(),
                &corrupt_dry,
            )
            .is_err()
        );
        assert_eq!(
            serde_json::to_vec(&state).expect("unchanged beginning bytes"),
            beginning_bytes
        );
        assert_eq!(
            execute_candidate(&model, &config, &state, &forcing, &NoArbiter, &NoArbiter),
            Err(VegetationError::Unsupported(
                "V7 post-nitrogen multi-owner candidate is implementation-incomplete"
            ))
        );

        let forged = ModelDefinition {
            version: crate::MODEL_VERSION,
            sha256: MODEL_SHA256.into(),
            bytes: b"{}",
        };
        assert!(matches!(
            execute_candidate(&forged, &config, &state, &forcing, &NoArbiter, &NoArbiter),
            Err(VegetationError::ModelDigestMismatch { .. })
        ));

        for poison in [
            |layer: &mut SoilLayerForcing| layer.water_beginning_kg_m2 = -1.0,
            |layer: &mut SoilLayerForcing| layer.hydraulic_conductivity_mm_s = f64::NAN,
            |layer: &mut SoilLayerForcing| layer.root_path_length_mm = 0.0,
            |layer: &mut SoilLayerForcing| layer.temperature_k = -1.0,
        ] {
            let mut bad = forcing.clone();
            poison(&mut bad.soil_layers[0]);
            assert_eq!(
                execute_candidate(&model, &config, &state, &bad, &NoArbiter, &NoArbiter),
                Err(VegetationError::Domain("soil layer forcing"))
            );
        }
    }
}

pub(crate) fn displayed_leaf_derived_areas(
    displayed_leaf_carbon: f64,
    stratum: &crate::StratumConfiguration,
) -> Result<(f64, f64, f64), VegetationError> {
    let leaf_area = displayed_leaf_carbon * stratum.sla_m2_per_kg_c;
    let stem_area = leaf_area * stratum.sai_relation;
    let root_area = (leaf_area + stem_area) * stratum.root_to_leaf_area;
    if [leaf_area, stem_area, root_area]
        .iter()
        .any(|value| !value.is_finite() || *value < 0.0)
    {
        return Err(VegetationError::Domain("V4 displayed-leaf area identity"));
    }
    Ok((leaf_area, stem_area, root_area))
}

fn validate_displayed_leaf_identity(
    state: &StratumSharedState,
    stratum: &crate::StratumConfiguration,
) -> Result<(), VegetationError> {
    let leaf = state
        .tissues
        .get(&Tissue::Leaf)
        .ok_or(VegetationError::Domain("missing tissue"))?;
    let (leaf_area, stem_area, root_area) =
        displayed_leaf_derived_areas(leaf.display.carbon, stratum)?;
    if state.leaf_area.to_bits() != leaf_area.to_bits()
        || state.stem_area.to_bits() != stem_area.to_bits()
        || state.root_area.to_bits() != root_area.to_bits()
    {
        return Err(VegetationError::Domain("V4 displayed-leaf area identity"));
    }
    if leaf_area == 0.0 && leaf.display.nitrogen != 0.0 {
        return Err(VegetationError::Domain("V4 displayed leaf N without LAI"));
    }
    Ok(())
}

/// Commit remains unavailable until E16--E22 and every receiving-owner
/// candidate can be validated together.
pub fn validate_and_commit(
    _beginning: &mut CoupledOwnedState,
    _candidate: CoupledCandidate,
) -> Result<CommitReceipt, VegetationError> {
    Err(VegetationError::Unsupported(
        "V7 multi-owner candidate and atomic commit are implementation-incomplete",
    ))
}

/// Failure-injection commit entry point retained while the all-owner transaction is incomplete.
pub fn validate_and_commit_with_failure(
    beginning: &mut CoupledOwnedState,
    candidate: CoupledCandidate,
    failure: Option<FailurePoint>,
) -> Result<CommitReceipt, VegetationError> {
    if failure == Some(FailurePoint::OwnerValidation) {
        return Err(VegetationError::InjectedFailure("owner validation"));
    }
    if failure == Some(FailurePoint::BeforeCommit) {
        return Err(VegetationError::InjectedFailure("before commit"));
    }
    validate_and_commit(beginning, candidate)
}
