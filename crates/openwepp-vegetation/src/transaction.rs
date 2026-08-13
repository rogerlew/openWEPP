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
        config.validate()?;
        if self.model_definition_sha256.len() != 64
            || self.configuration_sha256.len() != 64
            || self.state_sha256.len() != 64
        {
            return Err(VegetationError::Domain("state identity"));
        }
        if self.model_definition_sha256 != MODEL_SHA256
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

/// Validates the complete public V4 state surface, then fails closed until the
/// occupancy-local authorization-capped transaction is implemented.
pub fn execute_candidate(
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    _water: &dyn WaterArbiter,
    _nitrogen: &dyn NitrogenArbiter,
) -> Result<CoupledCandidate, VegetationError> {
    validate_execution(model, config, beginning, forcing)?;
    Err(VegetationError::Unsupported(
        "V4 occupancy-local capped transaction routing is implementation-incomplete",
    ))
}

/// Failure-injection entry point retained while V4 routing is incomplete.
pub fn execute_candidate_with_failure(
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    beginning: &CoupledOwnedState,
    forcing: &SnowFreeForcing,
    water: &dyn WaterArbiter,
    nitrogen: &dyn NitrogenArbiter,
    failure: Option<FailurePoint>,
) -> Result<CoupledCandidate, VegetationError> {
    if failure == Some(FailurePoint::Validation) {
        return Err(VegetationError::InjectedFailure("validation"));
    }
    execute_candidate(model, config, beginning, forcing, water, nitrogen)
}

#[allow(clippy::too_many_lines)]
fn validate_execution(
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
        let mut value: serde_json::Value = serde_json::from_slice(include_bytes!(
            "../../../tests/fixtures/c3_woody_v2_diagnostic_configuration.json"
        ))
        .expect("historical configuration JSON");
        value["strata"][0]
            .as_object_mut()
            .expect("stratum object")
            .remove("rd_leaf_n_rate");
        let mut config: VegetationConfiguration =
            serde_json::from_value(value).expect("V4 configuration shape");
        config.model_definition_sha256 = MODEL_SHA256.into();
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
        let mut raw: serde_json::Value = serde_json::from_slice(include_bytes!(
            "../../../tests/fixtures/c3_woody_v1_diagnostic_state.json"
        ))
        .expect("historical state JSON");
        let state = raw
            .pointer_mut("/strata/tree-1")
            .and_then(serde_json::Value::as_object_mut)
            .expect("historical stratum");
        for field in [
            "canopy_liquid",
            "psi_root_mm",
            "psi_stem_mm",
            "psi_sun_mm",
            "psi_shade_mm",
            "previous_leaf_offset_flux",
            "previous_root_offset_flux",
        ] {
            state.remove(field);
        }
        serde_json::from_value(serde_json::Value::Object(state.clone())).expect("shared state")
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
        state.validate(&config).expect("complete V4 state");
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
            serde_json::to_vec(&state).expect("V4 state")
        );
    }

    #[test]
    fn v4_named_configuration_and_state_fixtures_are_cross_bound() {
        let config = VegetationConfiguration::parse_strict(include_bytes!(
            "../../../tests/fixtures/c3_woody_v4_diagnostic_configuration.json"
        ))
        .expect("V4 configuration fixture");
        let state: CoupledOwnedState = serde_json::from_slice(include_bytes!(
            "../../../tests/fixtures/c3_woody_v4_diagnostic_state.json"
        ))
        .expect("V4 state DTO");
        assert_eq!(
            state.state_sha256,
            state.canonical_sha256().expect("state digest")
        );
        state.validate(&config).expect("V4 state fixture");
        assert_eq!(config.initial_state_sha256, state.state_sha256);
        assert_eq!(state.configuration_sha256, config.configuration_sha256);
    }

    #[test]
    fn v4_occupancy_entries_reject_legacy_tuple_sequences() {
        let config = VegetationConfiguration::parse_strict(include_bytes!(
            "../../../tests/fixtures/c3_woody_v4_diagnostic_configuration.json"
        ))
        .expect("V4 configuration fixture");
        let state: serde_json::Value = serde_json::from_slice(include_bytes!(
            "../../../tests/fixtures/c3_woody_v4_diagnostic_state.json"
        ))
        .expect("V4 state JSON");
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
            .validate(&config)
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
        zero.validate(&config)
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
            zero.validate(&config),
            Err(VegetationError::Domain("V4 displayed leaf N without LAI"))
        );
    }

    #[test]
    fn v4_state_rejects_removed_offset_fields() {
        let (config, state) = fixture();
        let mut value = serde_json::to_value(state).expect("V4 state value");
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
        assert_eq!(
            original,
            "5d7f29d58c0ecf59b367d181586ac301ab277b4326f2cb39c20feddb3ae0e626"
        );
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
    fn public_transaction_validates_v4_inputs_then_fails_closed_before_capped_pass() {
        struct NoArbiter;
        impl WaterArbiter for NoArbiter {
            fn beginning_amount(&self, _: &WaterResourceKey) -> Result<f64, VegetationError> {
                Ok(0.0)
            }
            fn authorize(
                &self,
                _: &[WaterRequest],
            ) -> Result<Vec<WaterAuthorization>, VegetationError> {
                Ok(Vec::new())
            }
        }
        impl NitrogenArbiter for NoArbiter {
            fn beginning_amount(&self, _: &MineralNitrogenKey) -> Result<f64, VegetationError> {
                Ok(0.0)
            }
            fn authorize(
                &self,
                _: &[NitrogenRequest],
            ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
                Ok(Vec::new())
            }
        }
        let (config, state) = fixture();
        let model = crate::load_model_definition().expect("model");
        let forcing = SnowFreeForcing {
            air_temperature_k: 296.0,
            pressure_pa: 101_325.0,
            co2_pa: 40.0,
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
            longwave_up_w_m2: 350.0,
            specific_humidity: 0.01,
            reference_height_m: 20.0,
            soil_layers: vec![
                SoilLayerForcing {
                    layer_id: layer_id("soil-1"),
                    water_beginning_kg_m2: 10.0,
                    matric_potential_mm: -1000.0,
                    hydraulic_conductivity_mm_s: 1e-5,
                    root_path_length_mm: 1.0,
                    gravity_root_mm: 1.0,
                    temperature_k: 290.0,
                    accessible: true,
                    frozen: false,
                },
                SoilLayerForcing {
                    layer_id: layer_id("soil-2"),
                    water_beginning_kg_m2: 10.0,
                    matric_potential_mm: -2000.0,
                    hydraulic_conductivity_mm_s: 1e-5,
                    root_path_length_mm: 1.0,
                    gravity_root_mm: 1.0,
                    temperature_k: 290.0,
                    accessible: true,
                    frozen: false,
                },
            ],
            gsi: 0.5,
        };
        assert_eq!(
            execute_candidate(&model, &config, &state, &forcing, &NoArbiter, &NoArbiter),
            Err(VegetationError::Unsupported(
                "V4 occupancy-local capped transaction routing is implementation-incomplete"
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

fn validate_displayed_leaf_identity(
    state: &StratumSharedState,
    stratum: &crate::StratumConfiguration,
) -> Result<(), VegetationError> {
    let leaf = state
        .tissues
        .get(&Tissue::Leaf)
        .ok_or(VegetationError::Domain("missing tissue"))?;
    let leaf_area = leaf.display.carbon * stratum.sla_m2_per_kg_c;
    let stem_area = leaf_area * stratum.sai_relation;
    let root_area = (leaf_area + stem_area) * stratum.root_to_leaf_area;
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

/// V4 commit remains unavailable until occupancy-local candidate routing can
/// construct a fully validated candidate.
pub fn validate_and_commit(
    _beginning: &mut CoupledOwnedState,
    _candidate: CoupledCandidate,
) -> Result<CommitReceipt, VegetationError> {
    Err(VegetationError::Unsupported(
        "V4 occupancy-local capped transaction routing is implementation-incomplete",
    ))
}

/// Failure-injection commit entry point retained while V4 routing is incomplete.
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
