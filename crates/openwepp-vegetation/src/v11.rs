//! Default-off V11 segmented-support adopter.
//!
//! The adapter preserves V10's constitutive implementation and immutable
//! configuration identity. It supplies the coupled-time duration separately,
//! then removes segment-local transaction identity before staging the result.
//! Only [`V11ParentTransaction::finalize`] advances persistent chronology.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_coupled_time::{
    AcceptedSlabId, AcceptedSlabReceiptV1, Digest32, ModelTimeNs, OwnerState, ParentTransactionId,
    ReceiptId, SegmentId, TimeSupport, quantize_seconds_to_tick,
};
use openwepp_kernel_contract::{MineralNitrogenKey, WaterResourceKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::carbon_nitrogen::MaterialTransfer;
use crate::v10_state::{V10_MODEL_SHA256, V10CoupledOwnedState};
use crate::{VegetationConfiguration, VegetationError};

pub const V11_MODEL_VERSION: &str = "OPENWEPP_C3_WOODY_V11";
pub const V11_MODEL_SHA256: &str =
    "126e782104a50b52c8f12c32a9d48e3dd06215d806b801b5251049def415dfb2";
pub const V11_MODEL_BYTES: &[u8] =
    include_bytes!("../model-registry/openwepp_c3_woody_v11_definition.json");

#[must_use]
pub fn v11_model_sha256() -> String {
    V11_MODEL_SHA256.into()
}

pub fn load_v11_model_definition() -> Result<crate::ModelDefinition, V11Error> {
    if format!("{:x}", Sha256::digest(V11_MODEL_BYTES)) != V11_MODEL_SHA256 {
        return Err(V11Error::MigrationIdentity);
    }
    let value: serde_json::Value =
        serde_json::from_slice(V11_MODEL_BYTES).map_err(V11Error::Schema)?;
    if value["model_version"].as_str() != Some(V11_MODEL_VERSION)
        || value["base_model_definition_sha256"].as_str() != Some(V10_MODEL_SHA256)
    {
        return Err(V11Error::MigrationIdentity);
    }
    Ok(crate::ModelDefinition {
        version: V11_MODEL_VERSION,
        sha256: V11_MODEL_SHA256.into(),
        bytes: V11_MODEL_BYTES,
    })
}

/// Complete V10 configuration payload plus its exact V11 cadence identity.
///
/// The nested V10 value is constitutive source authority, not an executable
/// duration override. Segment execution supplies a separate authenticated
/// duration operand while leaving this value byte-identical.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VegetationConfigurationV11 {
    pub model_definition_sha256: String,
    pub configuration_sha256: String,
    pub initial_state_sha256: String,
    #[serde(with = "u128_string")]
    pub nominal_cadence_ns: u128,
    pub imported_v10: VegetationConfiguration,
}

/// External, configuration-derived authority joining each admitted BGC
/// stratum to its exact vegetation-tile-resolved OFE.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct V11BgcDebitScope {
    stratum_ofe_ids: BTreeMap<String, String>,
}

impl V11BgcDebitScope {
    pub fn try_new(stratum_ofe_ids: BTreeMap<String, String>) -> Result<Self, V11Error> {
        if stratum_ofe_ids
            .iter()
            .any(|(stratum, ofe)| stratum.is_empty() || ofe.is_empty())
        {
            return Err(V11Error::ResourceDebit);
        }
        Ok(Self { stratum_ofe_ids })
    }

    fn expected_ofe(&self, stratum_id: &str) -> Option<&str> {
        self.stratum_ofe_ids.get(stratum_id).map(String::as_str)
    }
}

impl VegetationConfigurationV11 {
    pub fn validate(&self) -> Result<(), V11Error> {
        self.imported_v10
            .validate_v10()
            .map_err(V11Error::Configuration)?;
        if self.model_definition_sha256 != v11_model_sha256()
            || self.nominal_cadence_ns == 0
            || self.configuration_sha256 != self.canonical_sha256()?
        {
            return Err(V11Error::MigrationIdentity);
        }
        let support = TimeSupport::new(
            ModelTimeNs::new(0),
            ModelTimeNs::new(self.nominal_cadence_ns),
        )?;
        if support.duration_s_bits() != self.imported_v10.dt_s.to_bits() {
            return Err(V11Error::CadenceRoundtrip);
        }
        Ok(())
    }

    pub fn canonical_sha256(&self) -> Result<String, V11Error> {
        let mut canonical = self.clone();
        canonical.configuration_sha256.clear();
        canonical.initial_state_sha256.clear();
        let bytes = serde_json::to_vec(&canonical).map_err(V11Error::Schema)?;
        Ok(format!("{:x}", Sha256::digest(bytes)))
    }
}

/// V11 state physically imports the complete V10 state payload.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V11CoupledOwnedState {
    pub model_definition_sha256: String,
    pub configuration_sha256: String,
    pub state_sha256: String,
    pub physical: crate::V8CoupledOwnedState,
    #[serde(with = "u128_string")]
    pub last_parent_transaction_id: u128,
}

impl V11CoupledOwnedState {
    pub fn canonical_sha256(&self) -> Result<String, V11Error> {
        let mut canonical = self.clone();
        canonical.state_sha256.clear();
        let bytes = serde_json::to_vec(&canonical).map_err(V11Error::Schema)?;
        Ok(format!("{:x}", Sha256::digest(bytes)))
    }

    pub fn validate(&self, configuration: &VegetationConfigurationV11) -> Result<(), V11Error> {
        configuration.validate()?;
        if self.model_definition_sha256 != v11_model_sha256()
            || self.configuration_sha256 != configuration.configuration_sha256
            || self.state_sha256 != self.canonical_sha256()?
            || self.physical.last_transaction_id != self.last_parent_transaction_id
        {
            return Err(V11Error::StateIdentity);
        }
        let (imported_configuration, projected) = imported_v10_view(configuration, self)?;
        projected
            .validate(&imported_configuration)
            .map_err(V11Error::V10State)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct V10ToV11Migration {
    pub configuration: VegetationConfigurationV11,
    pub state: V11CoupledOwnedState,
}

pub fn migrate_v10_runtime_to_v11(
    configuration: &VegetationConfiguration,
    state: &V10CoupledOwnedState,
) -> Result<V10ToV11Migration, V11Error> {
    state.validate(configuration).map_err(V11Error::V10State)?;
    let cadence = quantize_seconds_to_tick(
        ModelTimeNs::new(0),
        ModelTimeNs::new(u128::MAX),
        configuration.dt_s,
    )?
    .get();
    let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(cadence))?;
    if cadence == 0 || support.duration_s_bits() != configuration.dt_s.to_bits() {
        return Err(V11Error::CadenceRoundtrip);
    }
    let mut target_configuration = VegetationConfigurationV11 {
        model_definition_sha256: v11_model_sha256(),
        configuration_sha256: String::new(),
        initial_state_sha256: String::new(),
        nominal_cadence_ns: cadence,
        imported_v10: configuration.clone(),
    };
    target_configuration.configuration_sha256 = target_configuration.canonical_sha256()?;
    let mut physical = state.0.clone();
    physical.model_definition_sha256 = v11_model_sha256();
    physical
        .configuration_sha256
        .clone_from(&target_configuration.configuration_sha256);
    physical.state_sha256.clear();
    let mut target_state = V11CoupledOwnedState {
        model_definition_sha256: v11_model_sha256(),
        configuration_sha256: target_configuration.configuration_sha256.clone(),
        state_sha256: String::new(),
        last_parent_transaction_id: physical.last_transaction_id,
        physical,
    };
    target_state.physical.state_sha256 = target_state.physical.canonical_sha256();
    target_state.state_sha256 = target_state.canonical_sha256()?;
    target_configuration
        .initial_state_sha256
        .clone_from(&target_state.state_sha256);
    // Initial-state identity participates only as a receipt and is omitted from
    // the configuration digest, matching the historical configuration rule.
    target_configuration.validate()?;
    target_state.validate(&target_configuration)?;
    Ok(V10ToV11Migration {
        configuration: target_configuration,
        state: target_state,
    })
}

/// Exact support and identity handed to the imported constitutive consumer.
#[derive(Clone, Debug)]
pub struct V11ImportedV10SegmentInput {
    pub parent_transaction_id: ParentTransactionId,
    pub accepted_slab_receipt: AcceptedSlabReceiptV1,
    pub support: TimeSupport,
    pub duration_s_bits: u64,
    pub configuration: VegetationConfiguration,
    pub beginning: V10CoupledOwnedState,
    pub staged_resource_owners: BTreeMap<String, V11OwnerEnvelope>,
}

pub const V11_COMPLETE_OWNER_MANIFEST: [&str; 7] = [
    "vegetation",
    "snow",
    "land_surface_energy",
    "surface_liquid",
    "hydrology",
    "bgc",
    "soil_thermal",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V11OwnerEnvelope {
    pub owner_id: String,
    pub state_bytes: Vec<u8>,
    pub state_sha256: Digest32,
}

impl V11OwnerEnvelope {
    pub fn try_new(owner_id: String, state_bytes: Vec<u8>) -> Result<Self, V11Error> {
        let owner = OwnerState::new(owner_id.clone(), state_bytes.clone())?;
        Ok(Self {
            owner_id,
            state_bytes,
            state_sha256: owner.state_digest(),
        })
    }

    pub fn to_owner_state(&self) -> Result<OwnerState, V11Error> {
        let owner = OwnerState::new(self.owner_id.clone(), self.state_bytes.clone())?;
        if owner.state_digest() != self.state_sha256 {
            return Err(V11Error::ResourceOwnerCandidate);
        }
        Ok(owner)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V11ResourceDebit {
    pub receipt_id: Digest32,
    pub parent_transaction_id: ParentTransactionId,
    pub segment_id: SegmentId,
    pub accepted_slab_id: AcceptedSlabId,
    pub support: TimeSupport,
    pub owner_id: String,
    pub resource_key: V11ResourceKey,
    pub ofe_id: String,
    pub tile_id: String,
    pub occupancy_id: String,
    pub layer_id: String,
    pub source_id: String,
    pub amount_basis: String,
    pub request: f64,
    pub authorization: f64,
    pub final_use: f64,
}

impl V11ResourceDebit {
    pub fn new(mut value: Self) -> Result<Self, V11Error> {
        value.receipt_id = Digest32::zero();
        value.receipt_id = digest_canonical(b"OPENWEPP_V11_DEBIT_V1\0", &value)?;
        validate_debits(std::slice::from_ref(&value))?;
        Ok(value)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V11AdmittedResourceFlux {
    pub receipt_id: Digest32,
    pub parent_transaction_id: ParentTransactionId,
    pub segment_id: SegmentId,
    pub accepted_slab_id: AcceptedSlabId,
    pub support: TimeSupport,
    pub flux_class: String,
    pub direction: String,
    pub source_owner_id: String,
    pub receiver_owner_id: String,
    pub shared_resource_key: V11SharedResourceKey,
    pub amount: f64,
}

impl V11AdmittedResourceFlux {
    pub fn new(mut value: Self) -> Result<Self, V11Error> {
        value.receipt_id = Digest32::zero();
        value.receipt_id = digest_canonical(b"OPENWEPP_V11_FLUX_V1\0", &value)?;
        validate_fluxes(std::slice::from_ref(&value))?;
        Ok(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V11OwnerCandidateComponent {
    pub shared_resource_key: V11SharedResourceKey,
    pub ending_amount_bits: u64,
    pub debit_receipt_ids: Vec<Digest32>,
    pub admitted_flux_receipt_ids: Vec<Digest32>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V11CompleteOwnerCandidate {
    pub parent_transaction_id: ParentTransactionId,
    pub segment_id: SegmentId,
    pub accepted_slab_id: AcceptedSlabId,
    pub slab_ordinal: u32,
    pub support: TimeSupport,
    pub owner_id: String,
    pub components: Vec<V11OwnerCandidateComponent>,
    pub ending_owner: V11OwnerEnvelope,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V11SharedResourceOwnerTransition {
    pub transition_id: Digest32,
    pub parent_transaction_id: ParentTransactionId,
    pub segment_id: SegmentId,
    pub accepted_slab_id: AcceptedSlabId,
    pub support: TimeSupport,
    pub shared_resource_key: V11SharedResourceKey,
    pub beginning_amount: f64,
    pub ending_amount: f64,
    pub debit_receipt_ids: Vec<Digest32>,
    pub admitted_flux_receipt_ids: Vec<Digest32>,
    pub owner_candidate_sha256: Digest32,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V11SharedResourceKind {
    Water,
    Ammonium,
    Nitrate,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V11SharedResourceKey {
    pub resource: V11SharedResourceKind,
    pub owner_id: String,
    pub ofe_id: String,
    pub layer_id: String,
    pub source_id: String,
    pub amount_basis: String,
}

impl V11SharedResourceOwnerTransition {
    pub fn new(mut value: Self) -> Result<Self, V11Error> {
        value.transition_id = Digest32::zero();
        value.transition_id = digest_canonical(b"OPENWEPP_V11_TRANSITION_V1\0", &value)?;
        Ok(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum V11ResourceKey {
    Water(WaterResourceKey),
    MineralNitrogen(MineralNitrogenKey),
}

/// Exact sealed LSE support receipt carried by one accepted V11 slab.
///
/// Vegetation owns chronology, not LSE policy interpretation. The producing
/// LSE adopter validates the typed receipt, then transfers its exact canonical
/// bytes through this closed envelope. Extracted identity fields are checked
/// against both those bytes and the coupled-time slab at every acceptance and
/// restore boundary.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V11LseSupportReceiptEnvelope {
    pub parent_transaction_id: String,
    pub segment_id: String,
    pub accepted_slab_id: String,
    pub slab_ordinal: String,
    pub support_start_ns: String,
    pub support_end_ns: String,
    pub requested_support_ns: String,
    pub duration_s_bits: String,
    pub configuration_sha256: String,
    pub beginning_state_sha256: String,
    pub beginning_soil_thermal_state_sha256: String,
    pub receipt_sha256: String,
    pub canonical_bytes_sha256: String,
    pub canonical_json: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct LseSupportReceiptWire {
    parent_transaction_id: String,
    segment_id: String,
    accepted_slab_id: String,
    slab_ordinal: String,
    support_start_ns: String,
    support_end_ns: String,
    model_version: String,
    model_definition_sha256: String,
    configuration_sha256: String,
    beginning_state_sha256: String,
    beginning_soil_thermal_state_sha256: String,
    tolerance_policy_sha256: String,
    numerical_policy_sha256: String,
    requested_support_ns: String,
    duration_s_bits: String,
    minimum_support_ns: String,
    receipt_sha256: String,
}

include!("v11_lse_receipt_impl.rs");

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn digest_hex(value: Digest32) -> String {
    use std::fmt::Write as _;
    value
        .as_bytes()
        .iter()
        .fold(String::with_capacity(64), |mut text, byte| {
            write!(&mut text, "{byte:02x}").expect("writing to String cannot fail");
            text
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct V11ImportedV10SegmentOutput {
    pub ending: V10CoupledOwnedState,
    pub lse_support_receipt: V11LseSupportReceiptEnvelope,
    pub resource_debits: Vec<V11ResourceDebit>,
    pub admitted_resource_fluxes: Vec<V11AdmittedResourceFlux>,
    pub shared_resource_transitions: Vec<V11SharedResourceOwnerTransition>,
    /// Six non-vegetation owner candidates. The vegetation envelope is
    /// constructed internally from the validated ending V11 state.
    pub ending_resource_owners: BTreeMap<String, V11OwnerEnvelope>,
    pub material_transfers: Vec<MaterialTransfer>,
}

pub trait V11ConstitutiveExecutor {
    type Error;
    fn execute_v10_segment(
        &mut self,
        input: &V11ImportedV10SegmentInput,
    ) -> Result<V11ImportedV10SegmentOutput, Self::Error>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct V11AcceptedSegmentCandidate {
    pub accepted_slab_receipt: AcceptedSlabReceiptV1,
    pub lse_support_receipt: V11LseSupportReceiptEnvelope,
    pub beginning_state_sha256: String,
    pub ending_state: V11CoupledOwnedState,
    pub resource_debits: Vec<V11ResourceDebit>,
    pub admitted_resource_fluxes: Vec<V11AdmittedResourceFlux>,
    pub shared_resource_transitions: Vec<V11SharedResourceOwnerTransition>,
    pub complete_owner_candidates: Vec<V11CompleteOwnerCandidate>,
    pub material_transfers: Vec<MaterialTransfer>,
    pub ending_resource_owners: BTreeMap<String, V11OwnerEnvelope>,
}

pub fn execute_v11_segment<E: V11ConstitutiveExecutor>(
    configuration: &VegetationConfigurationV11,
    parent: &V11ParentTransaction,
    accepted_slab_receipt: &AcceptedSlabReceiptV1,
    executor: &mut E,
) -> Result<V11AcceptedSegmentCandidate, V11ExecutionError<E::Error>> {
    execute_v11_segment_with_bgc_scope(configuration, parent, accepted_slab_receipt, None, executor)
}

pub fn execute_v11_segment_with_bgc_scope<E: V11ConstitutiveExecutor>(
    configuration: &VegetationConfigurationV11,
    parent: &V11ParentTransaction,
    accepted_slab_receipt: &AcceptedSlabReceiptV1,
    bgc_scope: Option<&V11BgcDebitScope>,
    executor: &mut E,
) -> Result<V11AcceptedSegmentCandidate, V11ExecutionError<E::Error>> {
    configuration.validate().map_err(V11ExecutionError::V11)?;
    parent
        .validate(configuration)
        .map_err(V11ExecutionError::V11)?;
    let support = accepted_slab_receipt.support();
    if accepted_slab_receipt.parent_transaction_id() != parent.parent_transaction_id
        || accepted_slab_receipt.duration_s_bits() != support.duration_s_bits()
        || support.start_ns().get() != parent.accepted_until_ns
    {
        return Err(V11ExecutionError::V11(V11Error::SupportPredecessor));
    }
    let (imported_configuration, segment_beginning) =
        imported_v10_view(configuration, &parent.staged_state).map_err(V11ExecutionError::V11)?;
    let input = V11ImportedV10SegmentInput {
        parent_transaction_id: parent.parent_transaction_id,
        accepted_slab_receipt: accepted_slab_receipt.clone(),
        support,
        duration_s_bits: support.duration_s_bits(),
        configuration: imported_configuration,
        beginning: segment_beginning,
        staged_resource_owners: parent.staged_resource_owners.clone(),
    };
    let output = executor
        .execute_v10_segment(&input)
        .map_err(V11ExecutionError::Executor)?;
    output
        .lse_support_receipt
        .validate_join(accepted_slab_receipt)
        .map_err(V11ExecutionError::V11)?;
    output
        .lse_support_receipt
        .validate_beginning_owners(&parent.staged_resource_owners)
        .map_err(V11ExecutionError::V11)?;
    output
        .ending
        .validate(&input.configuration)
        .map_err(|e| V11ExecutionError::V11(V11Error::V10State(e)))?;
    if output.ending.0.last_transaction_id
        != input
            .beginning
            .0
            .last_transaction_id
            .checked_add(1)
            .ok_or_else(|| V11ExecutionError::V11(V11Error::ParentTransactionOverflow))?
    {
        return Err(V11ExecutionError::V11(V11Error::SegmentTransaction));
    }
    validate_debits(&output.resource_debits).map_err(V11ExecutionError::V11)?;
    validate_fluxes(&output.admitted_resource_fluxes).map_err(V11ExecutionError::V11)?;
    validate_material_transfers(&output.material_transfers).map_err(V11ExecutionError::V11)?;
    let ending_state = stage_imported_ending(configuration, &parent.staged_state, output.ending)
        .map_err(V11ExecutionError::V11)?;
    validate_nonvegetation_owners(&output.ending_resource_owners)
        .map_err(V11ExecutionError::V11)?;
    let mut ending_resource_owners = output.ending_resource_owners;
    ending_resource_owners.insert(
        "vegetation".into(),
        v11_vegetation_owner_envelope(&ending_state).map_err(V11ExecutionError::V11)?,
    );
    let complete_owner_candidates = build_complete_owner_candidates(
        accepted_slab_receipt,
        &ending_resource_owners,
        &output.shared_resource_transitions,
    )
    .map_err(V11ExecutionError::V11)?;
    validate_resource_custody(
        configuration,
        bgc_scope,
        accepted_slab_receipt.parent_transaction_id(),
        accepted_slab_receipt.segment_id(),
        accepted_slab_receipt.slab_id(),
        accepted_slab_receipt.slab_ordinal(),
        accepted_slab_receipt.support(),
        &output.resource_debits,
        &output.admitted_resource_fluxes,
        &output.shared_resource_transitions,
        &complete_owner_candidates,
        None,
    )
    .map_err(V11ExecutionError::V11)?;
    Ok(V11AcceptedSegmentCandidate {
        accepted_slab_receipt: accepted_slab_receipt.clone(),
        lse_support_receipt: output.lse_support_receipt,
        beginning_state_sha256: parent.staged_state.state_sha256.clone(),
        ending_state,
        resource_debits: output.resource_debits,
        admitted_resource_fluxes: output.admitted_resource_fluxes,
        shared_resource_transitions: output.shared_resource_transitions,
        complete_owner_candidates,
        material_transfers: output.material_transfers,
        ending_resource_owners,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct V11ParentTransaction {
    parent_transaction_id: ParentTransactionId,
    beginning_state: V11CoupledOwnedState,
    staged_state: V11CoupledOwnedState,
    accepted_until_ns: u128,
    accepted_segments: Vec<V11AcceptedSegmentCandidate>,
    accepted_segment_checkpoints: Vec<V11AcceptedSegmentCheckpoint>,
    accepted_zero_duration_owner_transitions: Vec<V11ZeroDurationOwnerTransitionCheckpoint>,
    cumulative_debits: BTreeMap<(String, V11ResourceKey), f64>,
    beginning_complete_owners: BTreeMap<String, V11OwnerEnvelope>,
    staged_resource_owners: BTreeMap<String, V11OwnerEnvelope>,
    finalized: bool,
}

impl V11ParentTransaction {
    pub fn new_with_complete_owners(
        configuration: &VegetationConfigurationV11,
        beginning: &V11CoupledOwnedState,
        parent_transaction_id: ParentTransactionId,
        parent_start_ns: ModelTimeNs,
        staged_resource_owners: BTreeMap<String, V11OwnerEnvelope>,
    ) -> Result<Self, V11Error> {
        beginning.validate(configuration)?;
        validate_complete_owners(&staged_resource_owners)?;
        if staged_resource_owners.get("vegetation")
            != Some(&v11_vegetation_owner_envelope(beginning)?)
        {
            return Err(V11Error::ResourceOwnerCandidate);
        }
        Ok(Self {
            parent_transaction_id,
            beginning_state: beginning.clone(),
            staged_state: beginning.clone(),
            accepted_until_ns: parent_start_ns.get(),
            accepted_segments: Vec::new(),
            accepted_segment_checkpoints: Vec::new(),
            accepted_zero_duration_owner_transitions: Vec::new(),
            cumulative_debits: BTreeMap::new(),
            beginning_complete_owners: staged_resource_owners.clone(),
            staged_resource_owners,
            finalized: false,
        })
    }

    #[must_use]
    pub const fn parent_transaction_id(&self) -> ParentTransactionId {
        self.parent_transaction_id
    }
    #[must_use]
    pub fn beginning_state(&self) -> &V11CoupledOwnedState {
        &self.beginning_state
    }
    #[must_use]
    pub fn staged_state(&self) -> &V11CoupledOwnedState {
        &self.staged_state
    }
    #[must_use]
    pub fn accepted_segments(&self) -> &[V11AcceptedSegmentCandidate] {
        &self.accepted_segments
    }
    #[must_use]
    pub fn staged_resource_owners(&self) -> &BTreeMap<String, V11OwnerEnvelope> {
        &self.staged_resource_owners
    }

    /// Install a zero-duration coupled-time event owner set at the current
    /// accepted cursor. Constitutive rates and vegetation state are unchanged;
    /// the caller-owned coupled event receipt authenticates chronology and
    /// replay exclusion.
    pub fn accept_zero_duration_owner_transition(
        &mut self,
        configuration: &VegetationConfigurationV11,
        tick: ModelTimeNs,
        ending_owners: BTreeMap<String, V11OwnerEnvelope>,
        mutation_set: &[String],
    ) -> Result<(), V11Error> {
        self.accept_zero_duration_owner_transition_inner(
            configuration,
            tick,
            ending_owners,
            mutation_set,
            None,
        )
    }

    /// Retain an exact no-op owner checkpoint whose physical transfer is
    /// independently sealed by a nonzero custody receipt set.
    pub fn accept_zero_duration_custody_noop(
        &mut self,
        configuration: &VegetationConfigurationV11,
        tick: ModelTimeNs,
        ending_owners: BTreeMap<String, V11OwnerEnvelope>,
        custody_receipt_sha256: Digest32,
    ) -> Result<(), V11Error> {
        self.accept_zero_duration_owner_transition_inner(
            configuration,
            tick,
            ending_owners,
            &[],
            Some(custody_receipt_sha256),
        )
    }

    fn accept_zero_duration_owner_transition_inner(
        &mut self,
        configuration: &VegetationConfigurationV11,
        tick: ModelTimeNs,
        ending_owners: BTreeMap<String, V11OwnerEnvelope>,
        mutation_set: &[String],
        custody_receipt_sha256: Option<Digest32>,
    ) -> Result<(), V11Error> {
        self.validate(configuration)?;
        validate_complete_owners(&ending_owners)?;
        if self.finalized
            || tick.get() != self.accepted_until_ns
            || mutation_set.is_empty() != custody_receipt_sha256.is_some()
            || custody_receipt_sha256 == Some(Digest32::zero())
            || mutation_set.windows(2).any(|pair| pair[0] >= pair[1])
            || ending_owners.get("vegetation")
                != Some(&v11_vegetation_owner_envelope(&self.staged_state)?)
        {
            return Err(V11Error::ResourceOwnerCandidate);
        }
        let exact_mutation_set = self
            .staged_resource_owners
            .iter()
            .filter(|(owner_id, beginning)| {
                ending_owners
                    .get(*owner_id)
                    .is_some_and(|ending| *beginning != ending)
            })
            .map(|(owner_id, _)| owner_id.clone())
            .collect::<Vec<_>>();
        if mutation_set != exact_mutation_set {
            return Err(V11Error::ResourceOwnerCandidate);
        }
        self.accepted_zero_duration_owner_transitions.push(
            V11ZeroDurationOwnerTransitionCheckpoint {
                accepted_segment_count: u32::try_from(self.accepted_segment_checkpoints.len())
                    .map_err(|_| V11Error::RestartCheckpoint)?,
                tick_ns: tick.get(),
                ending_complete_owners: ending_owners.clone(),
                mutation_set: mutation_set.to_vec(),
                custody_receipt_sha256,
            },
        );
        self.staged_resource_owners = ending_owners;
        Ok(())
    }

    pub fn accept_segment(
        &mut self,
        configuration: &VegetationConfigurationV11,
        candidate: V11AcceptedSegmentCandidate,
    ) -> Result<(), V11Error> {
        self.accept_segment_with_bgc_scope(configuration, candidate, None)
    }

    pub fn accept_segment_with_bgc_scope(
        &mut self,
        configuration: &VegetationConfigurationV11,
        candidate: V11AcceptedSegmentCandidate,
        bgc_scope: Option<&V11BgcDebitScope>,
    ) -> Result<(), V11Error> {
        self.validate(configuration)?;
        let support = candidate.accepted_slab_receipt.support();
        if self.finalized
            || candidate.beginning_state_sha256 != self.staged_state.state_sha256
            || support.start_ns().get() != self.accepted_until_ns
            || candidate.accepted_slab_receipt.parent_transaction_id() != self.parent_transaction_id
            || candidate.accepted_slab_receipt.duration_s_bits() != support.duration_s_bits()
            || self.accepted_segment_checkpoints.iter().any(|prior| {
                prior.receipt_id == candidate.accepted_slab_receipt.id()
                    || prior.slab_id == candidate.accepted_slab_receipt.slab_id()
            })
            || self
                .accepted_segment_checkpoints
                .last()
                .is_some_and(|prior| {
                    prior.slab_ordinal.checked_add(1)
                        != Some(candidate.accepted_slab_receipt.slab_ordinal())
                })
        {
            return Err(V11Error::SupportPredecessor);
        }
        candidate
            .lse_support_receipt
            .validate_join(&candidate.accepted_slab_receipt)?;
        candidate
            .lse_support_receipt
            .validate_beginning_owners(&self.staged_resource_owners)?;
        if self.accepted_segment_checkpoints.iter().any(|prior| {
            prior.lse_support_receipt.receipt_sha256 == candidate.lse_support_receipt.receipt_sha256
                || prior.lse_support_receipt.canonical_bytes_sha256
                    == candidate.lse_support_receipt.canonical_bytes_sha256
        }) {
            return Err(V11Error::SupportPredecessor);
        }
        validate_debits(&candidate.resource_debits)?;
        validate_fluxes(&candidate.admitted_resource_fluxes)?;
        validate_material_transfers(&candidate.material_transfers)?;
        validate_complete_owners(&candidate.ending_resource_owners)?;
        if candidate.ending_resource_owners.get("vegetation")
            != Some(&v11_vegetation_owner_envelope(&candidate.ending_state)?)
        {
            return Err(V11Error::ResourceOwnerCandidate);
        }
        let predecessors = self
            .accepted_segment_checkpoints
            .last()
            .map(|segment| segment.shared_resource_transitions.as_slice());
        validate_resource_custody(
            configuration,
            bgc_scope,
            candidate.accepted_slab_receipt.parent_transaction_id(),
            candidate.accepted_slab_receipt.segment_id(),
            candidate.accepted_slab_receipt.slab_id(),
            candidate.accepted_slab_receipt.slab_ordinal(),
            candidate.accepted_slab_receipt.support(),
            &candidate.resource_debits,
            &candidate.admitted_resource_fluxes,
            &candidate.shared_resource_transitions,
            &candidate.complete_owner_candidates,
            predecessors,
        )?;
        let mut next = self.cumulative_debits.clone();
        for debit in &candidate.resource_debits {
            let value = next
                .entry((debit.owner_id.clone(), debit.resource_key.clone()))
                .or_insert(0.0);
            *value += debit.final_use;
            // This ordered +0.0-seeded fold is an authenticated diagnostic.
            // Owner custody is the independently validated per-segment
            // beginning-minus-amount ending chain; binary64 nonassociativity
            // forbids regrouping this cumulative value into an owner ending.
            if !value.is_finite() {
                return Err(V11Error::ResourceDebit);
            }
        }
        candidate.ending_state.validate(configuration)?;
        self.cumulative_debits = next;
        self.accepted_until_ns = support.end_ns().get();
        self.staged_state = candidate.ending_state.clone();
        self.staged_resource_owners = candidate.ending_resource_owners.clone();
        self.accepted_segment_checkpoints
            .push(V11AcceptedSegmentCheckpoint::from_candidate(&candidate));
        self.accepted_segments.push(candidate);
        Ok(())
    }

    pub fn finalize(
        mut self,
        configuration: &VegetationConfigurationV11,
    ) -> Result<V11ParentCandidate, V11Error> {
        self.validate(configuration)?;
        if self.finalized || self.accepted_segment_checkpoints.is_empty() {
            return Err(V11Error::ParentFinalization);
        }
        let next = self
            .beginning_state
            .last_parent_transaction_id
            .checked_add(1)
            .ok_or(V11Error::ParentTransactionOverflow)?;
        self.staged_state.last_parent_transaction_id = next;
        normalize_parent_transaction_lineage(&mut self.staged_state.physical, next);
        self.staged_state.physical.state_sha256 = self.staged_state.physical.canonical_sha256();
        self.staged_state.state_sha256 = self.staged_state.canonical_sha256()?;
        self.staged_resource_owners.insert(
            "vegetation".into(),
            v11_vegetation_owner_envelope(&self.staged_state)?,
        );
        self.finalized = true;
        let mut material_transfers = self
            .accepted_segment_checkpoints
            .iter()
            .flat_map(|segment| segment.material_transfers.iter().cloned())
            .collect::<Vec<_>>();
        for (index, transfer) in material_transfers.iter_mut().enumerate() {
            transfer.transaction_id = next;
            transfer.proposal_id = u64::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or(V11Error::ParentTransactionOverflow)?;
        }
        Ok(V11ParentCandidate {
            parent_transaction_id: self.parent_transaction_id,
            beginning_state_sha256: self.beginning_state.state_sha256,
            ending_state: self.staged_state,
            accepted_segments: self.accepted_segments,
            accepted_segment_checkpoints: self.accepted_segment_checkpoints,
            cumulative_debits: self.cumulative_debits,
            material_transfers,
            beginning_complete_owners: ordered_owner_states(&self.beginning_complete_owners)?,
            ending_complete_owners: ordered_owner_states(&self.staged_resource_owners)?,
        })
    }

    fn validate(&self, configuration: &VegetationConfigurationV11) -> Result<(), V11Error> {
        self.beginning_state.validate(configuration)?;
        self.staged_state.validate(configuration)?;
        validate_complete_owners(&self.beginning_complete_owners)?;
        validate_complete_owners(&self.staged_resource_owners)?;
        if self.beginning_complete_owners.get("vegetation")
            != Some(&v11_vegetation_owner_envelope(&self.beginning_state)?)
            || self.staged_resource_owners.get("vegetation")
                != Some(&v11_vegetation_owner_envelope(&self.staged_state)?)
        {
            return Err(V11Error::ResourceOwnerCandidate);
        }
        if self.staged_state.last_parent_transaction_id
            != self.beginning_state.last_parent_transaction_id
        {
            return Err(V11Error::SegmentTransaction);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct V11ParentCandidate {
    pub parent_transaction_id: ParentTransactionId,
    pub beginning_state_sha256: String,
    pub ending_state: V11CoupledOwnedState,
    pub accepted_segments: Vec<V11AcceptedSegmentCandidate>,
    pub accepted_segment_checkpoints: Vec<V11AcceptedSegmentCheckpoint>,
    pub cumulative_debits: BTreeMap<(String, V11ResourceKey), f64>,
    pub material_transfers: Vec<MaterialTransfer>,
    pub beginning_complete_owners: Vec<OwnerState>,
    pub ending_complete_owners: Vec<OwnerState>,
}

impl V11ParentCandidate {
    /// Compare the complete sealed parent-candidate authority while ignoring
    /// only the evaluator-owned accepted-segment payload cache. Restored
    /// parents reconstruct every accepted segment from its checkpoint and do
    /// not repopulate that transient cache.
    #[must_use]
    pub fn has_same_checkpoint_authority(&self, other: &Self) -> bool {
        self.parent_transaction_id == other.parent_transaction_id
            && self.beginning_state_sha256 == other.beginning_state_sha256
            && self.ending_state == other.ending_state
            && self.accepted_segment_checkpoints == other.accepted_segment_checkpoints
            && self.cumulative_debits == other.cumulative_debits
            && self.material_transfers == other.material_transfers
            && self.beginning_complete_owners == other.beginning_complete_owners
            && self.ending_complete_owners == other.ending_complete_owners
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V11AcceptedSegmentCheckpoint {
    pub receipt_id: ReceiptId,
    pub slab_id: AcceptedSlabId,
    pub parent_transaction_id: ParentTransactionId,
    pub slab_ordinal: u32,
    pub segment_id: SegmentId,
    pub support: TimeSupport,
    pub duration_s_bits: u64,
    pub lse_support_receipt: V11LseSupportReceiptEnvelope,
    pub beginning_state_sha256: String,
    pub ending_state: V11CoupledOwnedState,
    pub resource_debits: Vec<V11ResourceDebit>,
    pub admitted_resource_fluxes: Vec<V11AdmittedResourceFlux>,
    pub shared_resource_transitions: Vec<V11SharedResourceOwnerTransition>,
    pub complete_owner_candidates: Vec<V11CompleteOwnerCandidate>,
    pub material_transfers: Vec<MaterialTransfer>,
    pub ending_resource_owners: BTreeMap<String, V11OwnerEnvelope>,
}

impl V11AcceptedSegmentCheckpoint {
    fn from_candidate(candidate: &V11AcceptedSegmentCandidate) -> Self {
        let receipt = &candidate.accepted_slab_receipt;
        Self {
            receipt_id: receipt.id(),
            slab_id: receipt.slab_id(),
            parent_transaction_id: receipt.parent_transaction_id(),
            slab_ordinal: receipt.slab_ordinal(),
            segment_id: receipt.segment_id(),
            support: receipt.support(),
            duration_s_bits: receipt.duration_s_bits(),
            lse_support_receipt: candidate.lse_support_receipt.clone(),
            beginning_state_sha256: candidate.beginning_state_sha256.clone(),
            ending_state: candidate.ending_state.clone(),
            resource_debits: candidate.resource_debits.clone(),
            admitted_resource_fluxes: candidate.admitted_resource_fluxes.clone(),
            shared_resource_transitions: candidate.shared_resource_transitions.clone(),
            complete_owner_candidates: candidate.complete_owner_candidates.clone(),
            material_transfers: candidate.material_transfers.clone(),
            ending_resource_owners: candidate.ending_resource_owners.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V11ParentTransactionCheckpoint {
    pub schema: String,
    pub parent_transaction_id: ParentTransactionId,
    pub beginning_state: V11CoupledOwnedState,
    pub staged_state: V11CoupledOwnedState,
    #[serde(with = "u128_string")]
    pub accepted_until_ns: u128,
    pub accepted_segments: Vec<V11AcceptedSegmentCheckpoint>,
    pub accepted_zero_duration_owner_transitions: Vec<V11ZeroDurationOwnerTransitionCheckpoint>,
    pub cumulative_debits: Vec<V11CumulativeDebit>,
    pub beginning_complete_owners: BTreeMap<String, V11OwnerEnvelope>,
    pub staged_complete_owners: BTreeMap<String, V11OwnerEnvelope>,
    pub finalized: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V11ZeroDurationOwnerTransitionCheckpoint {
    pub accepted_segment_count: u32,
    #[serde(with = "u128_string")]
    pub tick_ns: u128,
    pub ending_complete_owners: BTreeMap<String, V11OwnerEnvelope>,
    pub mutation_set: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custody_receipt_sha256: Option<Digest32>,
}

fn apply_checkpoint_zero_duration_owner_transitions(
    transitions: &[V11ZeroDurationOwnerTransitionCheckpoint],
    next_transition: &mut usize,
    accepted_segment_count: usize,
    tick_ns: u128,
    staged_state: &V11CoupledOwnedState,
    staged_owners: &mut BTreeMap<String, V11OwnerEnvelope>,
) -> Result<(), V11Error> {
    let accepted_segment_count =
        u32::try_from(accepted_segment_count).map_err(|_| V11Error::RestartCheckpoint)?;
    while let Some(transition) = transitions.get(*next_transition) {
        if transition.accepted_segment_count > accepted_segment_count {
            break;
        }
        if transition.accepted_segment_count != accepted_segment_count
            || transition.tick_ns != tick_ns
            || transition.mutation_set.is_empty() != transition.custody_receipt_sha256.is_some()
            || transition.custody_receipt_sha256 == Some(Digest32::zero())
            || transition
                .mutation_set
                .windows(2)
                .any(|pair| pair[0] >= pair[1])
        {
            return Err(V11Error::RestartCheckpoint);
        }
        validate_complete_owners(&transition.ending_complete_owners)
            .map_err(|_| V11Error::RestartCheckpoint)?;
        if transition.ending_complete_owners.get("vegetation")
            != Some(&v11_vegetation_owner_envelope(staged_state)?)
        {
            return Err(V11Error::RestartCheckpoint);
        }
        let exact_mutation_set = staged_owners
            .iter()
            .filter(|(owner_id, beginning)| {
                transition
                    .ending_complete_owners
                    .get(*owner_id)
                    .is_some_and(|ending| ending != *beginning)
            })
            .map(|(owner_id, _)| owner_id.clone())
            .collect::<Vec<_>>();
        if transition.mutation_set != exact_mutation_set {
            return Err(V11Error::RestartCheckpoint);
        }
        staged_owners.clone_from(&transition.ending_complete_owners);
        *next_transition = next_transition
            .checked_add(1)
            .ok_or(V11Error::RestartCheckpoint)?;
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct V11CumulativeDebit {
    pub owner_id: String,
    pub resource_key: V11ResourceKey,
    pub amount: f64,
}

struct V11CheckpointReplayState {
    cursor: u128,
    predecessor_state: V11CoupledOwnedState,
    prior_transitions: Vec<V11SharedResourceOwnerTransition>,
    predecessor_owners: BTreeMap<String, V11OwnerEnvelope>,
    cumulative: BTreeMap<(String, V11ResourceKey), f64>,
    next_zero_duration_transition: usize,
}

fn replay_checkpoint_accepted_segments(
    configuration: &VegetationConfigurationV11,
    checkpoint: &V11ParentTransactionCheckpoint,
    bgc_scope: Option<&V11BgcDebitScope>,
    replay: &mut V11CheckpointReplayState,
) -> Result<(), V11Error> {
    for (index, segment) in checkpoint.accepted_segments.iter().enumerate() {
        apply_checkpoint_zero_duration_owner_transitions(
            &checkpoint.accepted_zero_duration_owner_transitions,
            &mut replay.next_zero_duration_transition,
            index,
            replay.cursor,
            &replay.predecessor_state,
            &mut replay.predecessor_owners,
        )?;
        if segment.parent_transaction_id != checkpoint.parent_transaction_id
            || index > 0
                && checkpoint.accepted_segments[index - 1]
                    .slab_ordinal
                    .checked_add(1)
                    != Some(segment.slab_ordinal)
            || segment.support.start_ns().get() != replay.cursor
            || segment.duration_s_bits != segment.support.duration_s_bits()
            || segment.beginning_state_sha256 != replay.predecessor_state.state_sha256
        {
            return Err(V11Error::RestartCheckpoint);
        }
        let reconstructed_slab = checkpoint
            .accepted_segments
            .get(index)
            .ok_or(V11Error::RestartCheckpoint)?;
        segment
            .lse_support_receipt
            .validate_checkpoint_join(reconstructed_slab)
            .map_err(|_| V11Error::RestartCheckpoint)?;
        segment
            .lse_support_receipt
            .validate_beginning_owners(&replay.predecessor_owners)
            .map_err(|_| V11Error::RestartCheckpoint)?;
        if checkpoint.accepted_segments[..index].iter().any(|prior| {
            prior.lse_support_receipt.receipt_sha256 == segment.lse_support_receipt.receipt_sha256
                || prior.lse_support_receipt.canonical_bytes_sha256
                    == segment.lse_support_receipt.canonical_bytes_sha256
        }) {
            return Err(V11Error::RestartCheckpoint);
        }
        segment.ending_state.validate(configuration)?;
        validate_debits(&segment.resource_debits)?;
        validate_fluxes(&segment.admitted_resource_fluxes)?;
        validate_material_transfers(&segment.material_transfers)?;
        validate_complete_owners(&segment.ending_resource_owners)?;
        if segment.ending_resource_owners.get("vegetation")
            != Some(&v11_vegetation_owner_envelope(&segment.ending_state)?)
        {
            return Err(V11Error::RestartCheckpoint);
        }
        validate_resource_custody(
            configuration,
            bgc_scope,
            segment.parent_transaction_id,
            segment.segment_id,
            segment.slab_id,
            segment.slab_ordinal,
            segment.support,
            &segment.resource_debits,
            &segment.admitted_resource_fluxes,
            &segment.shared_resource_transitions,
            &segment.complete_owner_candidates,
            (!replay.prior_transitions.is_empty()).then_some(replay.prior_transitions.as_slice()),
        )
        .map_err(|_| V11Error::RestartCheckpoint)?;
        for debit in &segment.resource_debits {
            let key = (debit.owner_id.clone(), debit.resource_key.clone());
            let value = replay.cumulative.entry(key.clone()).or_insert(0.0);
            *value += debit.final_use;
            if !value.is_finite() {
                return Err(V11Error::RestartCheckpoint);
            }
        }
        replay
            .prior_transitions
            .clone_from(&segment.shared_resource_transitions);
        replay
            .predecessor_owners
            .clone_from(&segment.ending_resource_owners);
        replay.cursor = segment.support.end_ns().get();
        replay.predecessor_state.clone_from(&segment.ending_state);
    }
    Ok(())
}

impl V11ParentTransaction {
    #[must_use]
    pub fn checkpoint(&self) -> V11ParentTransactionCheckpoint {
        V11ParentTransactionCheckpoint {
            schema: "OPENWEPP_C3_WOODY_V11_PARENT_CHECKPOINT_V1".into(),
            parent_transaction_id: self.parent_transaction_id,
            beginning_state: self.beginning_state.clone(),
            staged_state: self.staged_state.clone(),
            accepted_until_ns: self.accepted_until_ns,
            accepted_segments: self.accepted_segment_checkpoints.clone(),
            accepted_zero_duration_owner_transitions: self
                .accepted_zero_duration_owner_transitions
                .clone(),
            cumulative_debits: self
                .cumulative_debits
                .iter()
                .map(|((owner_id, resource_key), amount)| V11CumulativeDebit {
                    owner_id: owner_id.clone(),
                    resource_key: resource_key.clone(),
                    amount: *amount,
                })
                .collect(),
            beginning_complete_owners: self.beginning_complete_owners.clone(),
            staged_complete_owners: self.staged_resource_owners.clone(),
            finalized: self.finalized,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn restore(
        configuration: &VegetationConfigurationV11,
        checkpoint: V11ParentTransactionCheckpoint,
    ) -> Result<Self, V11Error> {
        Self::restore_with_bgc_scope(configuration, checkpoint, None)
    }

    pub fn restore_with_bgc_scope(
        configuration: &VegetationConfigurationV11,
        checkpoint: V11ParentTransactionCheckpoint,
        bgc_scope: Option<&V11BgcDebitScope>,
    ) -> Result<Self, V11Error> {
        if checkpoint.schema != "OPENWEPP_C3_WOODY_V11_PARENT_CHECKPOINT_V1" || checkpoint.finalized
        {
            return Err(V11Error::RestartCheckpoint);
        }
        checkpoint.beginning_state.validate(configuration)?;
        checkpoint.staged_state.validate(configuration)?;
        validate_complete_owners(&checkpoint.beginning_complete_owners)?;
        validate_complete_owners(&checkpoint.staged_complete_owners)?;
        if checkpoint.beginning_complete_owners.get("vegetation")
            != Some(&v11_vegetation_owner_envelope(&checkpoint.beginning_state)?)
            || checkpoint.staged_complete_owners.get("vegetation")
                != Some(&v11_vegetation_owner_envelope(&checkpoint.staged_state)?)
        {
            return Err(V11Error::RestartCheckpoint);
        }
        // A parent may be checkpointed before its first positive-duration
        // segment, including immediately after an authenticated zero-duration
        // owner transition.  In that posture `accepted_until_ns` is both the
        // parent cursor and the transition tick.  Positive-segment checkpoints
        // continue to derive the beginning cursor from their first receipt.
        let mut replay = V11CheckpointReplayState {
            cursor: checkpoint
                .accepted_segments
                .first()
                .map_or(checkpoint.accepted_until_ns, |segment| {
                    segment.support.start_ns().get()
                }),
            predecessor_state: checkpoint.beginning_state.clone(),
            prior_transitions: Vec::new(),
            predecessor_owners: checkpoint.beginning_complete_owners.clone(),
            cumulative: BTreeMap::new(),
            next_zero_duration_transition: 0,
        };
        replay_checkpoint_accepted_segments(configuration, &checkpoint, bgc_scope, &mut replay)?;
        apply_checkpoint_zero_duration_owner_transitions(
            &checkpoint.accepted_zero_duration_owner_transitions,
            &mut replay.next_zero_duration_transition,
            checkpoint.accepted_segments.len(),
            replay.cursor,
            &replay.predecessor_state,
            &mut replay.predecessor_owners,
        )?;
        let checkpoint_cumulative = checkpoint
            .cumulative_debits
            .iter()
            .map(|value| {
                (
                    (value.owner_id.clone(), value.resource_key.clone()),
                    value.amount,
                )
            })
            .collect::<BTreeMap<_, _>>();
        if checkpoint_cumulative.len() != checkpoint.cumulative_debits.len()
            || replay.cursor != checkpoint.accepted_until_ns
            || !cumulative_debits_bit_equal(&replay.cumulative, &checkpoint_cumulative)
            || replay.next_zero_duration_transition
                != checkpoint.accepted_zero_duration_owner_transitions.len()
            || replay.predecessor_state != checkpoint.staged_state
            || replay.predecessor_owners != checkpoint.staged_complete_owners
        {
            return Err(V11Error::RestartCheckpoint);
        }
        Ok(Self {
            parent_transaction_id: checkpoint.parent_transaction_id,
            beginning_state: checkpoint.beginning_state,
            staged_state: checkpoint.staged_state,
            accepted_until_ns: checkpoint.accepted_until_ns,
            accepted_segments: Vec::new(),
            accepted_segment_checkpoints: checkpoint.accepted_segments,
            accepted_zero_duration_owner_transitions: checkpoint
                .accepted_zero_duration_owner_transitions,
            cumulative_debits: checkpoint_cumulative,
            beginning_complete_owners: checkpoint.beginning_complete_owners,
            staged_resource_owners: checkpoint.staged_complete_owners,
            finalized: false,
        })
    }
}

fn imported_v10_view(
    configuration: &VegetationConfigurationV11,
    state: &V11CoupledOwnedState,
) -> Result<(VegetationConfiguration, V10CoupledOwnedState), V11Error> {
    let mut config = configuration.imported_v10.clone();
    let mut physical = state.physical.clone();
    physical.model_definition_sha256 = V10_MODEL_SHA256.into();
    physical
        .configuration_sha256
        .clone_from(&config.configuration_sha256);
    physical.last_transaction_id = state.last_parent_transaction_id;
    physical.state_sha256 = physical.canonical_sha256();
    if physical.last_transaction_id == 0 {
        config
            .initial_state_sha256
            .clone_from(&physical.state_sha256);
    }
    let projected = V10CoupledOwnedState(physical);
    projected.validate(&config).map_err(V11Error::V10State)?;
    Ok((config, projected))
}

/// Project a validated finalized V11 parent state back onto its exact imported
/// V10 owner representation. This is used only for the zero-duration logical
/// parent-lineage handoff; it does not execute or modify constitutive physics.
pub fn project_v11_runtime_to_v10(
    configuration: &VegetationConfigurationV11,
    state: &V11CoupledOwnedState,
) -> Result<(VegetationConfiguration, V10CoupledOwnedState), V11Error> {
    state.validate(configuration)?;
    imported_v10_view(configuration, state)
}

/// Validate that `finalized` differs from the retained imported V10 owner only
/// by the exact logical parent-lineage fields advanced by V11 finalization,
/// then return that finalized state in its V10 representation.
pub fn project_v11_parent_finalization_to_v10(
    configuration: &VegetationConfigurationV11,
    beginning: &V10CoupledOwnedState,
    finalized: &V11CoupledOwnedState,
) -> Result<(VegetationConfiguration, V10CoupledOwnedState), V11Error> {
    beginning
        .validate(&configuration.imported_v10)
        .map_err(V11Error::V10State)?;
    finalized.validate(configuration)?;
    let migrated = migrate_v10_runtime_to_v11(&configuration.imported_v10, beginning)?;
    if migrated.configuration.configuration_sha256 != configuration.configuration_sha256
        || migrated.configuration.nominal_cadence_ns != configuration.nominal_cadence_ns
        || migrated.configuration.imported_v10 != configuration.imported_v10
    {
        return Err(V11Error::MigrationIdentity);
    }
    let mut expected = migrated.state;
    expected.last_parent_transaction_id = finalized.last_parent_transaction_id;
    normalize_parent_transaction_lineage(
        &mut expected.physical,
        finalized.last_parent_transaction_id,
    );
    expected.physical.state_sha256 = expected.physical.canonical_sha256();
    expected.state_sha256 = expected.canonical_sha256()?;
    if expected != *finalized {
        return Err(V11Error::ResourceOwnerCandidate);
    }
    imported_v10_view(configuration, finalized)
}

fn stage_imported_ending(
    configuration: &VegetationConfigurationV11,
    beginning: &V11CoupledOwnedState,
    ending: V10CoupledOwnedState,
) -> Result<V11CoupledOwnedState, V11Error> {
    let mut physical = ending.0;
    physical.model_definition_sha256 = v11_model_sha256();
    physical
        .configuration_sha256
        .clone_from(&configuration.configuration_sha256);
    normalize_parent_transaction_lineage(&mut physical, beginning.last_parent_transaction_id);
    physical.state_sha256 = physical.canonical_sha256();
    let mut state = V11CoupledOwnedState {
        model_definition_sha256: v11_model_sha256(),
        configuration_sha256: configuration.configuration_sha256.clone(),
        state_sha256: String::new(),
        physical,
        last_parent_transaction_id: beginning.last_parent_transaction_id,
    };
    state.state_sha256 = state.canonical_sha256()?;
    state.validate(configuration)?;
    Ok(state)
}

fn normalize_parent_transaction_lineage(
    physical: &mut crate::V8CoupledOwnedState,
    parent_transaction_id: u128,
) {
    physical.last_transaction_id = parent_transaction_id;
    for stratum in physical.strata.values_mut() {
        stratum.last_transaction_id = parent_transaction_id;
    }
    let accepted = (parent_transaction_id != 0).then_some(parent_transaction_id);
    for occupancy in physical.occupancies.values_mut() {
        occupancy.last_accepted_transaction_id = accepted;
    }
}

fn validate_debits(values: &[V11ResourceDebit]) -> Result<(), V11Error> {
    let mut identities = BTreeSet::new();
    for value in values {
        let mut canonical = value.clone();
        canonical.receipt_id = Digest32::zero();
        if value.receipt_id != digest_canonical(b"OPENWEPP_V11_DEBIT_V1\0", &canonical)?
            || [value.request, value.authorization, value.final_use]
                .iter()
                .any(|amount| !amount.is_finite() || *amount < 0.0)
            || value.final_use > value.authorization
            || value.authorization > value.request
            || [
                value.owner_id.as_str(),
                value.ofe_id.as_str(),
                value.tile_id.as_str(),
                value.occupancy_id.as_str(),
                value.layer_id.as_str(),
                value.source_id.as_str(),
                value.amount_basis.as_str(),
            ]
            .contains(&"")
            || !identities.insert((
                value.parent_transaction_id,
                value.segment_id,
                value.accepted_slab_id,
                value.owner_id.clone(),
                value.ofe_id.clone(),
                value.tile_id.clone(),
                value.occupancy_id.clone(),
                value.layer_id.clone(),
                value.source_id.clone(),
                value.amount_basis.clone(),
                value.resource_key.clone(),
            ))
        {
            return Err(V11Error::ResourceDebit);
        }
    }
    Ok(())
}

fn validate_fluxes(values: &[V11AdmittedResourceFlux]) -> Result<(), V11Error> {
    let mut ids = BTreeSet::new();
    for value in values {
        let mut canonical = value.clone();
        canonical.receipt_id = Digest32::zero();
        let admitted = value.flux_class == "surface_runon"
            && value.direction == "source_to_receiver"
            && value.source_owner_id == "surface_liquid"
            && value.receiver_owner_id == "hydrology"
            && value.shared_resource_key.owner_id == "hydrology"
            && value.shared_resource_key.resource == V11SharedResourceKind::Water;
        if value.receipt_id != digest_canonical(b"OPENWEPP_V11_FLUX_V1\0", &canonical)?
            || !value.amount.is_finite()
            || value.amount < 0.0
            || !admitted
            || !ids.insert(value.receipt_id)
        {
            return Err(V11Error::ResourceFlux);
        }
    }
    Ok(())
}

fn debit_shared_resource_key(value: &V11ResourceDebit) -> V11SharedResourceKey {
    use openwepp_kernel_contract::MineralNitrogenSpecies;
    let resource = match &value.resource_key {
        V11ResourceKey::Water(_) => V11SharedResourceKind::Water,
        V11ResourceKey::MineralNitrogen(key) => match key.species {
            MineralNitrogenSpecies::Ammonium => V11SharedResourceKind::Ammonium,
            MineralNitrogenSpecies::Nitrate => V11SharedResourceKind::Nitrate,
        },
    };
    V11SharedResourceKey {
        resource,
        owner_id: value.owner_id.clone(),
        ofe_id: value.ofe_id.clone(),
        layer_id: value.layer_id.clone(),
        source_id: value.source_id.clone(),
        amount_basis: value.amount_basis.clone(),
    }
}

fn build_complete_owner_candidates(
    receipt: &AcceptedSlabReceiptV1,
    owners: &BTreeMap<String, V11OwnerEnvelope>,
    transitions: &[V11SharedResourceOwnerTransition],
) -> Result<Vec<V11CompleteOwnerCandidate>, V11Error> {
    validate_complete_owners(owners)?;
    V11_COMPLETE_OWNER_MANIFEST
        .into_iter()
        .map(|owner_id| {
            let mut components = transitions
                .iter()
                .filter(|t| t.shared_resource_key.owner_id == owner_id)
                .map(|t| V11OwnerCandidateComponent {
                    shared_resource_key: t.shared_resource_key.clone(),
                    ending_amount_bits: t.ending_amount.to_bits(),
                    debit_receipt_ids: t.debit_receipt_ids.clone(),
                    admitted_flux_receipt_ids: t.admitted_flux_receipt_ids.clone(),
                })
                .collect::<Vec<_>>();
            components.sort_by(|a, b| a.shared_resource_key.cmp(&b.shared_resource_key));
            Ok(V11CompleteOwnerCandidate {
                parent_transaction_id: receipt.parent_transaction_id(),
                segment_id: receipt.segment_id(),
                accepted_slab_id: receipt.slab_id(),
                slab_ordinal: receipt.slab_ordinal(),
                support: receipt.support(),
                owner_id: owner_id.into(),
                components,
                ending_owner: owners
                    .get(owner_id)
                    .ok_or(V11Error::ResourceOwnerCandidate)?
                    .clone(),
            })
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
fn validate_resource_custody(
    configuration: &VegetationConfigurationV11,
    bgc_scope: Option<&V11BgcDebitScope>,
    parent: ParentTransactionId,
    segment: SegmentId,
    slab: AcceptedSlabId,
    ordinal: u32,
    support: TimeSupport,
    debits: &[V11ResourceDebit],
    fluxes: &[V11AdmittedResourceFlux],
    transitions: &[V11SharedResourceOwnerTransition],
    candidates: &[V11CompleteOwnerCandidate],
    predecessors: Option<&[V11SharedResourceOwnerTransition]>,
) -> Result<(), V11Error> {
    validate_debits(debits)?;
    validate_bgc_debit_configuration(configuration, bgc_scope, debits)?;
    validate_fluxes(fluxes)?;
    let domain = |p, s, a, t| p == parent && s == segment && a == slab && t == support;
    if debits.iter().any(|d| {
        !domain(
            d.parent_transaction_id,
            d.segment_id,
            d.accepted_slab_id,
            d.support,
        )
    }) || fluxes.iter().any(|f| {
        !domain(
            f.parent_transaction_id,
            f.segment_id,
            f.accepted_slab_id,
            f.support,
        )
    }) || transitions.iter().any(|t| {
        !domain(
            t.parent_transaction_id,
            t.segment_id,
            t.accepted_slab_id,
            t.support,
        )
    }) || candidates.len() != V11_COMPLETE_OWNER_MANIFEST.len()
        || candidates
            .iter()
            .zip(V11_COMPLETE_OWNER_MANIFEST)
            .any(|(c, id)| {
                c.owner_id != id
                    || c.slab_ordinal != ordinal
                    || !domain(
                        c.parent_transaction_id,
                        c.segment_id,
                        c.accepted_slab_id,
                        c.support,
                    )
                    || c.ending_owner.owner_id != id
            })
    {
        return Err(V11Error::ResourceCustody);
    }
    let debit_by_id = debits
        .iter()
        .map(|d| (d.receipt_id, d))
        .collect::<BTreeMap<_, _>>();
    let flux_by_id = fluxes
        .iter()
        .map(|f| (f.receipt_id, f))
        .collect::<BTreeMap<_, _>>();
    let mut linked_debits = Vec::new();
    let mut linked_fluxes = Vec::new();
    let mut keys = BTreeSet::new();
    for t in transitions {
        let mut canonical = t.clone();
        canonical.transition_id = Digest32::zero();
        if t.transition_id != digest_canonical(b"OPENWEPP_V11_TRANSITION_V1\0", &canonical)?
            || t.debit_receipt_ids.is_empty()
            || (!is_bgc_mineral_transition(t) && !is_sorted_unique(&t.debit_receipt_ids))
            || (is_bgc_mineral_transition(t)
                && !bgc_transition_ids_are_semantically_ordered(t, &debit_by_id))
            || !is_sorted_unique(&t.admitted_flux_receipt_ids)
            || !t.beginning_amount.is_finite()
            || !t.ending_amount.is_finite()
            || t.beginning_amount < 0.0
            || t.ending_amount < 0.0
            || !keys.insert(t.shared_resource_key.clone())
        {
            return Err(V11Error::ResourceCustody);
        }
        let candidate = candidates
            .iter()
            .find(|c| c.owner_id == t.shared_resource_key.owner_id)
            .ok_or(V11Error::ResourceCustody)?;
        if t.owner_candidate_sha256 != candidate.ending_owner.state_sha256 {
            return Err(V11Error::ResourceCustody);
        }
        let component = V11OwnerCandidateComponent {
            shared_resource_key: t.shared_resource_key.clone(),
            ending_amount_bits: t.ending_amount.to_bits(),
            debit_receipt_ids: t.debit_receipt_ids.clone(),
            admitted_flux_receipt_ids: t.admitted_flux_receipt_ids.clone(),
        };
        if !candidate.components.contains(&component) {
            return Err(V11Error::ResourceCustody);
        }
        let mut auth = 0.0;
        let mut used = 0.0;
        for id in &t.debit_receipt_ids {
            let d = debit_by_id.get(id).ok_or(V11Error::ResourceCustody)?;
            if debit_shared_resource_key(d) != t.shared_resource_key {
                return Err(V11Error::ResourceCustody);
            }
            auth += d.authorization;
            used += d.final_use;
            linked_debits.push(*id);
        }
        let mut inflow = 0.0;
        for id in &t.admitted_flux_receipt_ids {
            let f = flux_by_id.get(id).ok_or(V11Error::ResourceCustody)?;
            if f.receiver_owner_id != t.shared_resource_key.owner_id
                || f.shared_resource_key != t.shared_resource_key
            {
                return Err(V11Error::ResourceCustody);
            }
            inflow += f.amount;
            linked_fluxes.push(*id);
        }
        if auth > t.beginning_amount + inflow || used > t.beginning_amount + inflow {
            return Err(V11Error::ResourceCustody);
        }
        if t.shared_resource_key.owner_id == "bgc"
            && matches!(
                t.shared_resource_key.resource,
                V11SharedResourceKind::Ammonium | V11SharedResourceKind::Nitrate
            )
            && !nonnegative_finite_values_within_one_ulp(t.beginning_amount - used, t.ending_amount)
        {
            return Err(V11Error::ResourceCustody);
        }
        if let Some(previous) = predecessors {
            if let Some(p) = previous
                .iter()
                .find(|p| p.shared_resource_key == t.shared_resource_key)
            {
                if p.ending_amount.to_bits() != t.beginning_amount.to_bits() {
                    return Err(V11Error::ResourceCustody);
                }
            }
        }
    }
    linked_debits.sort();
    linked_fluxes.sort();
    if linked_debits != debit_by_id.keys().copied().collect::<Vec<_>>()
        || linked_fluxes != flux_by_id.keys().copied().collect::<Vec<_>>()
    {
        return Err(V11Error::ResourceCustody);
    }
    Ok(())
}

fn nonnegative_finite_values_within_one_ulp(left: f64, right: f64) -> bool {
    left.is_finite()
        && right.is_finite()
        && left >= 0.0
        && right >= 0.0
        && left.to_bits().abs_diff(right.to_bits()) <= 1
}

fn is_bgc_mineral_transition(value: &V11SharedResourceOwnerTransition) -> bool {
    value.shared_resource_key.owner_id == "bgc"
        && matches!(
            value.shared_resource_key.resource,
            V11SharedResourceKind::Ammonium | V11SharedResourceKind::Nitrate
        )
}

fn bgc_transition_ids_are_semantically_ordered(
    transition: &V11SharedResourceOwnerTransition,
    debit_by_id: &BTreeMap<Digest32, &V11ResourceDebit>,
) -> bool {
    let mut expected = transition.debit_receipt_ids.clone();
    expected.sort_by(|left, right| {
        let Some(left) = debit_by_id.get(left) else {
            return left.cmp(right);
        };
        let Some(right) = debit_by_id.get(right) else {
            return left.receipt_id.cmp(right);
        };
        left.occupancy_id
            .cmp(&right.occupancy_id)
            .then_with(|| left.layer_id.cmp(&right.layer_id))
            .then_with(|| left.resource_key.cmp(&right.resource_key))
    });
    expected == transition.debit_receipt_ids
        && transition
            .debit_receipt_ids
            .iter()
            .collect::<BTreeSet<_>>()
            .len()
            == transition.debit_receipt_ids.len()
}

fn validate_bgc_debit_configuration(
    configuration: &VegetationConfigurationV11,
    bgc_scope: Option<&V11BgcDebitScope>,
    debits: &[V11ResourceDebit],
) -> Result<(), V11Error> {
    use openwepp_kernel_contract::MineralNitrogenSpecies;

    let configured_strata = configuration
        .imported_v10
        .expected_occupancies()
        .into_iter()
        .map(|occupancy| occupancy.stratum_id.as_str().to_owned())
        .collect::<BTreeSet<_>>();
    let expected_identities = configuration
        .imported_v10
        .strata
        .iter()
        .flat_map(|stratum| {
            stratum.root_layers.iter().flat_map(move |root| {
                [
                    MineralNitrogenSpecies::Ammonium,
                    MineralNitrogenSpecies::Nitrate,
                ]
                .into_iter()
                .map(move |species| {
                    (
                        stratum.stratum_id.as_str().to_owned(),
                        root.layer_id.as_str().to_owned(),
                        species,
                    )
                })
            })
        })
        .collect::<BTreeSet<_>>();
    let mut actual_identities = BTreeSet::new();
    let mut bgc_ofe = None::<&str>;
    let mut prior_semantic_key = None::<(&str, &str, &V11ResourceKey)>;
    for debit in debits {
        let V11ResourceKey::MineralNitrogen(key) = &debit.resource_key else {
            if debit.owner_id == "bgc" {
                return Err(V11Error::ResourceDebit);
            }
            continue;
        };
        let expected_source = match key.species {
            MineralNitrogenSpecies::Ammonium => "nh4",
            MineralNitrogenSpecies::Nitrate => "no3",
        };
        if debit.owner_id != "bgc"
            || debit.tile_id != "stratum_scoped"
            || !configured_strata.contains(&debit.occupancy_id)
            || debit.layer_id != key.layer_id.as_str()
            || debit.source_id != expected_source
            || debit.amount_basis != "kg_n_m2"
            || bgc_scope.and_then(|scope| scope.expected_ofe(&debit.occupancy_id))
                != Some(debit.ofe_id.as_str())
            || bgc_ofe.is_some_and(|ofe| ofe != debit.ofe_id)
        {
            return Err(V11Error::ResourceDebit);
        }
        actual_identities.insert((
            debit.occupancy_id.clone(),
            debit.layer_id.clone(),
            key.species,
        ));
        let semantic_key = (
            debit.occupancy_id.as_str(),
            debit.layer_id.as_str(),
            &debit.resource_key,
        );
        if prior_semantic_key.is_some_and(|prior| prior >= semantic_key) {
            return Err(V11Error::ResourceDebit);
        }
        prior_semantic_key = Some(semantic_key);
        bgc_ofe = Some(&debit.ofe_id);
    }
    if !actual_identities.is_empty() && actual_identities != expected_identities {
        return Err(V11Error::ResourceDebit);
    }
    Ok(())
}

fn is_sorted_unique<T: Ord>(values: &[T]) -> bool {
    values.windows(2).all(|w| w[0] < w[1])
}
fn digest_canonical<T: Serialize>(domain: &[u8], value: &T) -> Result<Digest32, V11Error> {
    let mut h = Sha256::new();
    h.update(domain);
    h.update(serde_json::to_vec(value).map_err(V11Error::Schema)?);
    Ok(Digest32::from_bytes(h.finalize().into()))
}

fn cumulative_debits_bit_equal(
    left: &BTreeMap<(String, V11ResourceKey), f64>,
    right: &BTreeMap<(String, V11ResourceKey), f64>,
) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|((left_key, left_amount), (right_key, right_amount))| {
                left_key == right_key && left_amount.to_bits() == right_amount.to_bits()
            })
}

fn validate_material_transfers(values: &[MaterialTransfer]) -> Result<(), V11Error> {
    for value in values {
        if value.owner_id.as_str().is_empty()
            || !value.carbon.is_finite()
            || !value.nitrogen.is_finite()
            || !value.dry_matter.is_finite()
            || value.carbon < 0.0
            || value.nitrogen < 0.0
            || value.dry_matter < 0.0
        {
            return Err(V11Error::MaterialTransfer);
        }
    }
    Ok(())
}

fn validate_complete_owners(values: &BTreeMap<String, V11OwnerEnvelope>) -> Result<(), V11Error> {
    let expected = V11_COMPLETE_OWNER_MANIFEST
        .into_iter()
        .collect::<BTreeSet<_>>();
    if values.keys().map(String::as_str).collect::<BTreeSet<_>>() != expected {
        return Err(V11Error::ResourceOwnerCandidate);
    }
    for (id, value) in values {
        if value.owner_id != *id {
            return Err(V11Error::ResourceOwnerCandidate);
        }
        value.to_owner_state()?;
    }
    Ok(())
}

fn validate_nonvegetation_owners(
    values: &BTreeMap<String, V11OwnerEnvelope>,
) -> Result<(), V11Error> {
    let expected = V11_COMPLETE_OWNER_MANIFEST
        .into_iter()
        .filter(|id| *id != "vegetation")
        .collect::<BTreeSet<_>>();
    if values.keys().map(String::as_str).collect::<BTreeSet<_>>() != expected {
        return Err(V11Error::ResourceOwnerCandidate);
    }
    for (id, value) in values {
        if value.owner_id != *id {
            return Err(V11Error::ResourceOwnerCandidate);
        }
        value.to_owner_state()?;
    }
    Ok(())
}

pub fn v11_vegetation_owner_envelope(
    state: &V11CoupledOwnedState,
) -> Result<V11OwnerEnvelope, V11Error> {
    let bytes = serde_json::to_vec(state).map_err(V11Error::Schema)?;
    V11OwnerEnvelope::try_new("vegetation".into(), bytes)
}

fn ordered_owner_states(
    values: &BTreeMap<String, V11OwnerEnvelope>,
) -> Result<Vec<OwnerState>, V11Error> {
    validate_complete_owners(values)?;
    V11_COMPLETE_OWNER_MANIFEST
        .into_iter()
        .map(|id| {
            values
                .get(id)
                .ok_or(V11Error::ResourceOwnerCandidate)?
                .to_owner_state()
        })
        .collect()
}

#[derive(Debug, Error)]
pub enum V11ExecutionError<E> {
    #[error("VEG-E-122: V11 authority rejected segment: {0}")]
    V11(#[from] V11Error),
    #[error("VEG-E-123: imported V10 segment execution failed: {0}")]
    Executor(E),
}

#[derive(Debug, Error)]
pub enum V11Error {
    #[error("VEG-E-121: invalid V11 configuration: {0}")]
    Configuration(VegetationError),
    #[error("VEG-E-121: V10 source/state rejected: {0}")]
    V10State(crate::V10StateError),
    #[error("VEG-E-121: V11 canonical schema: {0}")]
    Schema(serde_json::Error),
    #[error("VEG-E-121: migration identity mismatch")]
    MigrationIdentity,
    #[error("VEG-E-121: cadence does not roundtrip exactly")]
    CadenceRoundtrip,
    #[error("VEG-E-122: invalid coupled-time support duration")]
    SupportDuration,
    #[error("VEG-E-123: segment predecessor mismatch")]
    SupportPredecessor,
    #[error("VEG-E-123: invalid or mismatched LSE support receipt")]
    LseSupportReceipt,
    #[error("VEG-E-123: V11 state identity mismatch")]
    StateIdentity,
    #[error("VEG-E-123: segment attempted persistent transaction advance")]
    SegmentTransaction,
    #[error("VEG-E-124: invalid or duplicate resource debit")]
    ResourceDebit,
    #[error("VEG-E-124: unadmitted typed resource flux")]
    ResourceFlux,
    #[error("VEG-E-124: invalid shared-resource custody chronology")]
    ResourceCustody,
    #[error("VEG-E-124: invalid complete staged owner candidate")]
    ResourceOwnerCandidate,
    #[error("VEG-E-125: invalid material-transfer chronology")]
    MaterialTransfer,
    #[error("VEG-E-126: parent transaction overflow")]
    ParentTransactionOverflow,
    #[error("VEG-E-126: invalid or duplicate parent finalization")]
    ParentFinalization,
    #[error("VEG-E-127: invalid V11 parent checkpoint")]
    RestartCheckpoint,
    #[error("VEG-E-122: coupled-time authority rejected support: {0}")]
    CoupledTime(#[from] openwepp_coupled_time::CoupledTimeError),
}

mod u128_string {
    use serde::{Deserialize, Deserializer, Serializer, de};

    pub(super) fn serialize<S: Serializer>(value: &u128, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<u128, D::Error> {
        let text = String::deserialize(deserializer)?;
        if text.is_empty()
            || text != "0" && text.starts_with('0')
            || !text.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(de::Error::custom("noncanonical u128 string"));
        }
        text.parse().map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_coupled_time::{
        ConstraintClass, CoupledClockStateV1, CoupledSlabCandidateV1, LedgerEntryV1,
        ParentAuthorityV1, ParentIntervalId, StepConstraintV1, accept_slab,
        complete_owner_set_digest, digest_bytes, reduce_constraints,
    };

    #[test]
    fn shared_inventory_regrouping_bound_is_exactly_one_ulp() {
        let value = 1.0_f64;
        assert!(nonnegative_finite_values_within_one_ulp(value, value));
        assert!(nonnegative_finite_values_within_one_ulp(
            value,
            f64::from_bits(value.to_bits() + 1),
        ));
        assert!(!nonnegative_finite_values_within_one_ulp(
            value,
            f64::from_bits(value.to_bits() + 2),
        ));
        assert!(!nonnegative_finite_values_within_one_ulp(f64::NAN, value,));
        assert!(!nonnegative_finite_values_within_one_ulp(-value, value));
    }

    #[test]
    fn imported_executor_error_display_retains_nested_typed_cause() {
        let error = V11ExecutionError::Executor("nested-direct-v10-cause");
        assert_eq!(
            error.to_string(),
            "VEG-E-123: imported V10 segment execution failed: nested-direct-v10-cause"
        );
    }

    fn v10_fixture() -> (VegetationConfiguration, V10CoupledOwnedState) {
        let (v8_config, v8_state) = crate::v8_state::v8_test_fixture();
        let mut configuration = v8_config;
        configuration.model_definition_sha256 = V10_MODEL_SHA256.into();
        configuration.configuration_sha256 = configuration.canonical_sha256().expect("config");
        let mut physical = v8_state;
        physical.model_definition_sha256 = V10_MODEL_SHA256.into();
        physical.configuration_sha256 = configuration.configuration_sha256.clone();
        physical.state_sha256 = physical.canonical_sha256();
        configuration.initial_state_sha256 = physical.state_sha256.clone();
        (configuration, V10CoupledOwnedState(physical))
    }

    fn digest(byte: u8) -> Digest32 {
        Digest32::from_bytes([byte; 32])
    }

    fn complete_owners(state: &V11CoupledOwnedState) -> BTreeMap<String, V11OwnerEnvelope> {
        V11_COMPLETE_OWNER_MANIFEST
            .into_iter()
            .map(|id| {
                let envelope = if id == "vegetation" {
                    v11_vegetation_owner_envelope(state).expect("vegetation owner")
                } else if id == "land_surface_energy" {
                    V11OwnerEnvelope::try_new(
                        id.into(),
                        serde_json::to_vec(&serde_json::json!({
                            "configuration_sha256": "a".repeat(64),
                            "state_sha256": "b".repeat(64),
                        }))
                        .expect("LSE state"),
                    )
                    .expect("owner")
                } else if id == "soil_thermal" {
                    V11OwnerEnvelope::try_new(
                        id.into(),
                        serde_json::to_vec(&serde_json::json!({
                            "state_sha256": "c".repeat(64),
                        }))
                        .expect("soil state"),
                    )
                    .expect("owner")
                } else {
                    V11OwnerEnvelope::try_new(id.into(), format!("{id}-state").into_bytes())
                        .expect("owner")
                };
                (id.into(), envelope)
            })
            .collect()
    }

    fn accepted_receipts(
        owners: &BTreeMap<String, V11OwnerEnvelope>,
        ends: &[u128],
    ) -> (ParentTransactionId, Vec<AcceptedSlabReceiptV1>) {
        let mut clock_owners = owners
            .values()
            .map(V11OwnerEnvelope::to_owner_state)
            .collect::<Result<Vec<_>, _>>()
            .expect("owner states");
        clock_owners.sort_by(|a, b| a.owner_id().cmp(b.owner_id()));
        let participants = clock_owners
            .iter()
            .map(|owner| owner.owner_id().to_owned())
            .collect::<Vec<_>>();
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000))
            .expect("parent support");
        let interval =
            ParentIntervalId::derive(digest(1), digest(2), digest(3), support).expect("interval");
        let owner_digest = complete_owner_set_digest(&clock_owners).expect("owner digest");
        let parent =
            ParentTransactionId::derive(digest(1), 0, interval, owner_digest).expect("parent");
        let authority =
            ParentAuthorityV1::new(digest(1), digest(2), digest(3), 0, support, owner_digest)
                .expect("authority");
        let mut clock = CoupledClockStateV1::new(
            authority,
            clock_owners.clone(),
            "vegetation".into(),
            participants.clone(),
            digest(4),
            vec![],
        )
        .expect("clock");
        let mut participant_bytes = Vec::new();
        for participant in &participants {
            participant_bytes.extend_from_slice(participant.as_bytes());
            participant_bytes.push(0);
        }
        let segment = SegmentId::derive(
            parent,
            0,
            support,
            digest_bytes(b"vegetation"),
            digest_bytes(&participant_bytes),
        )
        .expect("segment");
        let mut receipts = Vec::new();
        for &end in ends {
            let constraint = StepConstraintV1::new(
                parent,
                clock.accepted_until(),
                ModelTimeNs::new(end),
                "vegetation".into(),
                ConstraintClass::HardBoundary,
                digest(5),
                digest(6),
                digest(7),
            )
            .expect("constraint");
            let reduction = reduce_constraints(
                &[constraint],
                parent,
                clock.accepted_until(),
                ModelTimeNs::new(1_800_000_000_000),
                None,
            )
            .expect("reduction");
            let candidate = CoupledSlabCandidateV1::new(
                &clock,
                segment,
                TimeSupport::new(clock.accepted_until(), ModelTimeNs::new(end)).expect("slab"),
                &reduction,
                clock_owners.clone(),
                vec![
                    LedgerEntryV1::new(
                        "vegetation_test".into(),
                        "kg".into(),
                        digest(8),
                        digest(8),
                        digest(9),
                    )
                    .expect("ledger"),
                ],
            )
            .expect("candidate");
            receipts.push(accept_slab(&mut clock, candidate).expect("accept"));
        }
        (parent, receipts)
    }

    fn staged_candidate(
        parent: &V11ParentTransaction,
        receipt: AcceptedSlabReceiptV1,
        debit: Option<V11ResourceDebit>,
    ) -> V11AcceptedSegmentCandidate {
        staged_candidate_with_debits(parent, receipt, debit.into_iter().collect())
    }

    fn staged_candidate_with_debits(
        parent: &V11ParentTransaction,
        receipt: AcceptedSlabReceiptV1,
        resource_debits: Vec<V11ResourceDebit>,
    ) -> V11AcceptedSegmentCandidate {
        let complete_owner_candidates =
            build_complete_owner_candidates(&receipt, &parent.staged_resource_owners, &[])
                .expect("candidates");
        V11AcceptedSegmentCandidate {
            lse_support_receipt: test_lse_support_receipt(&receipt),
            accepted_slab_receipt: receipt,
            beginning_state_sha256: parent.staged_state.state_sha256.clone(),
            ending_state: parent.staged_state.clone(),
            resource_debits,
            admitted_resource_fluxes: vec![],
            shared_resource_transitions: vec![],
            complete_owner_candidates,
            material_transfers: vec![],
            ending_resource_owners: parent.staged_resource_owners.clone(),
        }
    }

    fn test_lse_support_receipt(receipt: &AcceptedSlabReceiptV1) -> V11LseSupportReceiptEnvelope {
        let support = receipt.support();
        let mut value = LseSupportReceiptWire {
            parent_transaction_id: digest_hex(receipt.parent_transaction_id().digest()),
            segment_id: digest_hex(receipt.segment_id().digest()),
            accepted_slab_id: digest_hex(receipt.slab_id().digest()),
            slab_ordinal: receipt.slab_ordinal().to_string(),
            support_start_ns: support.start_ns().get().to_string(),
            support_end_ns: support.end_ns().get().to_string(),
            model_version: "OPENWEPP_SNOW_FREE_LSE_V1".into(),
            model_definition_sha256:
                "e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f".into(),
            configuration_sha256: "a".repeat(64),
            beginning_state_sha256: "b".repeat(64),
            beginning_soil_thermal_state_sha256: "c".repeat(64),
            tolerance_policy_sha256: format!(
                "{:x}",
                Sha256::digest(b"energy_absolute=1e-6;energy_relative=1e-10")
            ),
            numerical_policy_sha256: format!(
                "{:x}",
                Sha256::digest(b"iterations=50;backtracking=0..20;strict-decrease")
            ),
            requested_support_ns: support.duration_ns().to_string(),
            duration_s_bits: format!("{:016x}", support.duration_s_bits()),
            minimum_support_ns: "60000000000".into(),
            receipt_sha256: String::new(),
        };
        let mut preimage = b"OPENWEPP_LSE_SUPPORT_ADMISSION_V1\0".to_vec();
        preimage.extend(serde_json::to_vec(&value).expect("blank receipt"));
        value.receipt_sha256 = format!("{:x}", Sha256::digest(preimage));
        V11LseSupportReceiptEnvelope::from_canonical_json(
            serde_json::to_vec(&value).expect("receipt json"),
        )
        .expect("receipt envelope")
    }

    fn reframe_test_lse_receipt(
        envelope: &V11LseSupportReceiptEnvelope,
        lse_state: Option<String>,
        soil_state: Option<String>,
    ) -> V11LseSupportReceiptEnvelope {
        let mut wire: LseSupportReceiptWire =
            serde_json::from_slice(&envelope.canonical_json).expect("receipt wire");
        if let Some(value) = lse_state {
            wire.beginning_state_sha256 = value;
        }
        if let Some(value) = soil_state {
            wire.beginning_soil_thermal_state_sha256 = value;
        }
        wire.receipt_sha256.clear();
        let mut preimage = b"OPENWEPP_LSE_SUPPORT_ADMISSION_V1\0".to_vec();
        preimage.extend(serde_json::to_vec(&wire).expect("blank wire"));
        wire.receipt_sha256 = format!("{:x}", Sha256::digest(preimage));
        V11LseSupportReceiptEnvelope::from_canonical_json(
            serde_json::to_vec(&wire).expect("sealed wire"),
        )
        .expect("sealed receipt")
    }

    #[test]
    fn embedded_v11_definition_is_identity_distinct() {
        assert_ne!(v11_model_sha256(), V10_MODEL_SHA256);
        assert_eq!(
            load_v11_model_definition().expect("model").sha256,
            V11_MODEL_SHA256
        );
        let value: serde_json::Value = serde_json::from_slice(V11_MODEL_BYTES).expect("definition");
        assert_eq!(value["model_version"], V11_MODEL_VERSION);
    }

    #[test]
    fn migration_preserves_every_physical_payload_bit() {
        let (configuration, state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&configuration, &state).expect("migration");
        assert_eq!(migrated.configuration.nominal_cadence_ns, 1_800_000_000_000);
        assert_eq!(migrated.state.physical.strata, state.0.strata);
        assert_eq!(migrated.state.physical.occupancies, state.0.occupancies);
        assert_eq!(
            migrated.state.last_parent_transaction_id,
            state.0.last_transaction_id
        );
        migrated
            .state
            .validate(&migrated.configuration)
            .expect("valid V11 state");
    }

    #[test]
    fn complete_owner_manifest_is_exact_and_digest_bound() {
        let owners = V11_COMPLETE_OWNER_MANIFEST
            .into_iter()
            .map(|id| {
                (
                    id.to_owned(),
                    V11OwnerEnvelope::try_new(id.to_owned(), id.as_bytes().to_vec())
                        .expect("owner"),
                )
            })
            .collect::<BTreeMap<_, _>>();
        validate_complete_owners(&owners).expect("complete owners");
        let mut poison = owners;
        poison.get_mut("hydrology").expect("hydrology").state_bytes[0] ^= 1;
        assert!(matches!(
            validate_complete_owners(&poison),
            Err(V11Error::ResourceOwnerCandidate)
        ));
    }

    #[test]
    fn segment_local_lineage_is_normalized_to_parent_chronology() {
        let (_, mut physical) = crate::v8_state::v8_test_fixture();
        physical.last_transaction_id = 9;
        for stratum in physical.strata.values_mut() {
            stratum.last_transaction_id = 9;
        }
        for occupancy in physical.occupancies.values_mut() {
            occupancy.last_accepted_transaction_id = Some(9);
        }
        normalize_parent_transaction_lineage(&mut physical, 8);
        assert_eq!(physical.last_transaction_id, 8);
        assert!(
            physical
                .strata
                .values()
                .all(|value| value.last_transaction_id == 8)
        );
        assert!(
            physical
                .occupancies
                .values()
                .all(|value| value.last_accepted_transaction_id == Some(8))
        );
    }

    #[test]
    fn unequal_segments_chain_and_finalize_one_complete_owner_successor() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) =
            accepted_receipts(&owners, &[600_000_000_000, 1_800_000_000_000]);
        assert_eq!(receipts[0].duration_s_bits(), 600.0_f64.to_bits());
        assert_eq!(receipts[1].duration_s_bits(), 1_200.0_f64.to_bits());
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        for receipt in receipts {
            let candidate = staged_candidate(&parent, receipt, None);
            parent
                .accept_segment(&migrated.configuration, candidate)
                .expect("stage");
        }
        let beginning_transaction = migrated.state.last_parent_transaction_id;
        let candidate = parent.finalize(&migrated.configuration).expect("finalize");
        assert_eq!(
            candidate.ending_state.last_parent_transaction_id,
            beginning_transaction + 1
        );
        assert_eq!(
            candidate.ending_complete_owners[0].state_bytes(),
            serde_json::to_vec(&candidate.ending_state)
                .expect("ending state")
                .as_slice()
        );
        assert_eq!(
            candidate
                .ending_complete_owners
                .iter()
                .map(OwnerState::owner_id)
                .collect::<Vec<_>>(),
            V11_COMPLETE_OWNER_MANIFEST
        );
    }

    fn zero_duration_owner_transition_fixture() -> (
        V10ToV11Migration,
        BTreeMap<String, V11OwnerEnvelope>,
        ParentTransactionId,
        BTreeMap<String, V11OwnerEnvelope>,
    ) {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let beginning_owners = complete_owners(&migrated.state);
        let (parent_id, _) = accepted_receipts(&beginning_owners, &[1_800_000_000_000]);
        let mut ending_owners = beginning_owners.clone();
        let mut ending_snow_bytes = ending_owners.get("snow").expect("snow").state_bytes.clone();
        ending_snow_bytes.push(1);
        ending_owners.insert(
            "snow".to_owned(),
            V11OwnerEnvelope::try_new("snow".to_owned(), ending_snow_bytes).expect("ending snow"),
        );
        (migrated, beginning_owners, parent_id, ending_owners)
    }

    fn zero_duration_owner_transition_parent(
        migrated: &V10ToV11Migration,
        beginning_owners: &BTreeMap<String, V11OwnerEnvelope>,
        parent_id: ParentTransactionId,
    ) -> V11ParentTransaction {
        V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            beginning_owners.clone(),
        )
        .expect("parent")
    }

    #[test]
    fn zero_duration_owner_transition_requires_exact_mutation_set() {
        let (migrated, beginning_owners, parent_id, ending_owners) =
            zero_duration_owner_transition_fixture();
        let make_parent =
            || zero_duration_owner_transition_parent(&migrated, &beginning_owners, parent_id);

        let mut extra_member = make_parent();
        let before = extra_member.checkpoint();
        assert!(matches!(
            extra_member.accept_zero_duration_owner_transition(
                &migrated.configuration,
                ModelTimeNs::new(0),
                ending_owners.clone(),
                &["nonexistent".to_owned(), "snow".to_owned()],
            ),
            Err(V11Error::ResourceOwnerCandidate)
        ));
        assert_eq!(extra_member.checkpoint(), before);

        let mut omitted_member = make_parent();
        assert!(matches!(
            omitted_member.accept_zero_duration_owner_transition(
                &migrated.configuration,
                ModelTimeNs::new(0),
                ending_owners.clone(),
                &["soil_thermal".to_owned()],
            ),
            Err(V11Error::ResourceOwnerCandidate)
        ));

        let mut exact = make_parent();
        exact
            .accept_zero_duration_owner_transition(
                &migrated.configuration,
                ModelTimeNs::new(0),
                ending_owners.clone(),
                &["snow".to_owned()],
            )
            .expect("exact snow mutation");
        assert_eq!(exact.staged_resource_owners(), &ending_owners);

        let mut ordinary_empty = make_parent();
        assert!(
            ordinary_empty
                .accept_zero_duration_owner_transition(
                    &migrated.configuration,
                    ModelTimeNs::new(0),
                    beginning_owners.clone(),
                    &[],
                )
                .is_err()
        );
        let mut missing_receipt = make_parent();
        assert!(
            missing_receipt
                .accept_zero_duration_custody_noop(
                    &migrated.configuration,
                    ModelTimeNs::new(0),
                    beginning_owners.clone(),
                    Digest32::zero(),
                )
                .is_err()
        );
        let mut false_noop = make_parent();
        assert!(
            false_noop
                .accept_zero_duration_custody_noop(
                    &migrated.configuration,
                    ModelTimeNs::new(0),
                    ending_owners,
                    Digest32::from_bytes([0x51; 32]),
                )
                .is_err()
        );
        let mut receipt_noop = make_parent();
        receipt_noop
            .accept_zero_duration_custody_noop(
                &migrated.configuration,
                ModelTimeNs::new(0),
                beginning_owners,
                Digest32::from_bytes([0x52; 32]),
            )
            .expect("receipt-bearing exact no-op");
        let checkpoint = receipt_noop.checkpoint();
        assert_eq!(
            checkpoint.accepted_zero_duration_owner_transitions[0].custody_receipt_sha256,
            Some(Digest32::from_bytes([0x52; 32])),
        );
        let restored = V11ParentTransaction::restore(&migrated.configuration, checkpoint.clone())
            .expect("restore receipt-bearing no-op");
        assert_eq!(restored.checkpoint(), checkpoint);
        let mut omitted = checkpoint.clone();
        omitted.accepted_zero_duration_owner_transitions[0].custody_receipt_sha256 = None;
        assert!(V11ParentTransaction::restore(&migrated.configuration, omitted).is_err());
        let mut substituted = checkpoint;
        substituted.accepted_zero_duration_owner_transitions[0].custody_receipt_sha256 =
            Some(Digest32::zero());
        assert!(V11ParentTransaction::restore(&migrated.configuration, substituted).is_err());
    }

    #[test]
    fn checkpoint_restores_before_first_segment_and_rejects_state_substitution() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let owners = complete_owners(&migrated.state);
        let (parent_id, _) = accepted_receipts(&owners, &[1_800_000_000_000]);
        let parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let checkpoint = parent.checkpoint();
        let restored = V11ParentTransaction::restore(&migrated.configuration, checkpoint.clone())
            .expect("restore initial parent");
        assert_eq!(restored.checkpoint(), checkpoint);

        let mut state_substitution = checkpoint;
        state_substitution.staged_state.last_parent_transaction_id += 1;
        assert!(
            V11ParentTransaction::restore(&migrated.configuration, state_substitution).is_err()
        );
    }

    #[test]
    fn parent_candidate_checkpoint_authority_ignores_only_transient_segment_cache() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state)
            .expect("migrate V11 fixture");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) = accepted_receipts(&owners, &[1_800_000_000_000]);
        let mut live = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("live parent");
        let segment = staged_candidate(&live, receipts[0].clone(), None);
        live.accept_segment(&migrated.configuration, segment)
            .expect("accepted segment");
        let checkpoint = live.checkpoint();
        let restored = V11ParentTransaction::restore(&migrated.configuration, checkpoint.clone())
            .expect("restored parent");
        assert_eq!(restored.checkpoint(), checkpoint);

        let live_candidate = live
            .finalize(&migrated.configuration)
            .expect("live candidate");
        let restored_candidate = restored
            .finalize(&migrated.configuration)
            .expect("restored candidate");
        assert_eq!(live_candidate.accepted_segments.len(), 1);
        assert!(restored_candidate.accepted_segments.is_empty());
        assert!(live_candidate.has_same_checkpoint_authority(&restored_candidate));

        let mut checkpoint_poison = restored_candidate.clone();
        checkpoint_poison.accepted_segment_checkpoints[0]
            .beginning_state_sha256
            .replace_range(..1, "f");
        assert!(
            !live_candidate.has_same_checkpoint_authority(&checkpoint_poison),
            "changed accepted checkpoint must reject",
        );
        let mut ending_owner_poison = restored_candidate;
        let ending_owner = &ending_owner_poison.ending_complete_owners[0];
        let mut ending_owner_bytes = ending_owner.state_bytes().to_vec();
        ending_owner_bytes.push(0);
        ending_owner_poison.ending_complete_owners[0] =
            OwnerState::new(ending_owner.owner_id().to_owned(), ending_owner_bytes)
                .expect("poison owner");
        assert!(
            !live_candidate.has_same_checkpoint_authority(&ending_owner_poison),
            "changed ending owner must reject",
        );
    }

    #[test]
    fn checkpoint_restores_ordered_same_tick_owner_transitions_and_rejects_poisons() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) = accepted_receipts(&owners, &[1_800_000_000_000]);
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let candidate = staged_candidate(&parent, receipts[0].clone(), None);
        parent
            .accept_segment(&migrated.configuration, candidate)
            .expect("stage");

        let mutate_owner =
            |owners: &BTreeMap<String, V11OwnerEnvelope>, owner_id: &str, marker: u8| {
                let mut ending = owners.clone();
                let mut bytes = ending
                    .get(owner_id)
                    .expect("mutated owner")
                    .state_bytes
                    .clone();
                bytes.push(marker);
                ending.insert(
                    owner_id.to_owned(),
                    V11OwnerEnvelope::try_new(owner_id.to_owned(), bytes).expect("ending owner"),
                );
                ending
            };
        let snow_ending = mutate_owner(parent.staged_resource_owners(), "snow", 1);
        parent
            .accept_zero_duration_owner_transition(
                &migrated.configuration,
                ModelTimeNs::new(1_800_000_000_000),
                snow_ending,
                &["snow".to_owned()],
            )
            .expect("snow transition");
        let lse_ending = mutate_owner(parent.staged_resource_owners(), "land_surface_energy", 2);
        parent
            .accept_zero_duration_owner_transition(
                &migrated.configuration,
                ModelTimeNs::new(1_800_000_000_000),
                lse_ending,
                &["land_surface_energy".to_owned()],
            )
            .expect("LSE transition");

        let checkpoint = parent.checkpoint();
        let restored = V11ParentTransaction::restore(&migrated.configuration, checkpoint.clone())
            .expect("restore transitions");
        assert_eq!(restored.checkpoint(), checkpoint);

        let mut omission = checkpoint.clone();
        omission.accepted_zero_duration_owner_transitions.remove(0);
        assert!(V11ParentTransaction::restore(&migrated.configuration, omission).is_err());

        let mut substitution = checkpoint.clone();
        substitution.accepted_zero_duration_owner_transitions[0].accepted_segment_count = 0;
        assert!(V11ParentTransaction::restore(&migrated.configuration, substitution).is_err());

        let mut order = checkpoint.clone();
        order.accepted_zero_duration_owner_transitions.reverse();
        assert!(V11ParentTransaction::restore(&migrated.configuration, order).is_err());

        let mut same_tick_duplicate = checkpoint;
        let duplicate = same_tick_duplicate.accepted_zero_duration_owner_transitions[0].clone();
        same_tick_duplicate
            .accepted_zero_duration_owner_transitions
            .insert(1, duplicate);
        assert!(
            V11ParentTransaction::restore(&migrated.configuration, same_tick_duplicate).is_err()
        );
    }

    #[test]
    fn checkpoint_restore_rejects_broken_predecessor_and_terminal_owner() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) = accepted_receipts(&owners, &[1_800_000_000_000]);
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let candidate = staged_candidate(&parent, receipts[0].clone(), None);
        parent
            .accept_segment(&migrated.configuration, candidate)
            .expect("stage");
        let checkpoint = parent.checkpoint();
        let restored = V11ParentTransaction::restore(&migrated.configuration, checkpoint.clone())
            .expect("restore");
        assert_eq!(restored.checkpoint(), checkpoint);

        let mut predecessor_poison = checkpoint.clone();
        predecessor_poison.accepted_segments[0].beginning_state_sha256 = "0".repeat(64);
        assert!(
            V11ParentTransaction::restore(&migrated.configuration, predecessor_poison).is_err()
        );

        let mut receipt_poison = checkpoint.clone();
        receipt_poison.accepted_segments[0]
            .lse_support_receipt
            .canonical_json[0] ^= 1;
        assert!(V11ParentTransaction::restore(&migrated.configuration, receipt_poison).is_err());

        let mut lse_join_poison = checkpoint.clone();
        lse_join_poison.accepted_segments[0].lse_support_receipt = reframe_test_lse_receipt(
            &lse_join_poison.accepted_segments[0].lse_support_receipt,
            Some("9".repeat(64)),
            None,
        );
        assert!(V11ParentTransaction::restore(&migrated.configuration, lse_join_poison).is_err());

        let mut soil_join_poison = checkpoint.clone();
        soil_join_poison.accepted_segments[0].lse_support_receipt = reframe_test_lse_receipt(
            &soil_join_poison.accepted_segments[0].lse_support_receipt,
            None,
            Some("8".repeat(64)),
        );
        assert!(V11ParentTransaction::restore(&migrated.configuration, soil_join_poison).is_err());

        let mut owner_poison = checkpoint;
        owner_poison.accepted_segments[0]
            .ending_resource_owners
            .get_mut("snow")
            .expect("snow")
            .state_bytes
            .push(0);
        assert!(V11ParentTransaction::restore(&migrated.configuration, owner_poison).is_err());
    }

    #[test]
    fn lse_support_receipt_replay_rejects_without_parent_mutation() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) =
            accepted_receipts(&owners, &[600_000_000_000, 1_800_000_000_000]);
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let first = staged_candidate(&parent, receipts[0].clone(), None);
        let replay = first.lse_support_receipt.clone();
        parent
            .accept_segment(&migrated.configuration, first)
            .expect("first segment");
        let before = parent.checkpoint();
        let mut second = staged_candidate(&parent, receipts[1].clone(), None);
        second.lse_support_receipt = replay;
        assert!(matches!(
            parent.accept_segment(&migrated.configuration, second),
            Err(V11Error::LseSupportReceipt | V11Error::SupportPredecessor)
        ));
        assert_eq!(parent.checkpoint(), before);
    }

    #[cfg(any())]
    #[test]
    #[allow(clippy::too_many_lines)]
    fn nonassociative_resource_custody_uses_sequential_endings_for_water_nh4_no3() {
        use openwepp_kernel_contract::{
            MineralNitrogenSpecies, OccupancyId, SoilLayerId, StratumId, TileId,
        };

        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) = accepted_receipts(
            &owners,
            &[600_000_000_000, 1_200_000_000_000, 1_800_000_000_000],
        );
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let layer = migrated.configuration.imported_v10.strata[0].root_layers[0]
            .layer_id
            .clone();
        let keys = [
            V11ResourceKey::Water(WaterResourceKey {
                occupancy_id: OccupancyId {
                    stratum_id: StratumId::try_new("s1").expect("stratum"),
                    tile_id: TileId::try_new("t1").expect("tile"),
                },
                layer_id: layer.clone(),
            }),
            V11ResourceKey::MineralNitrogen(MineralNitrogenKey {
                layer_id: layer.clone(),
                species: MineralNitrogenSpecies::Ammonium,
            }),
            V11ResourceKey::MineralNitrogen(MineralNitrogenKey {
                layer_id: layer,
                species: MineralNitrogenSpecies::Nitrate,
            }),
        ];
        let beginnings = [
            497_355_953.965_941_8,
            0.497_355_953_965_941_84,
            497.355_953_965_941_75,
        ];
        let amounts = [
            [
                108_987_197.969_511_36,
                119_731_815.493_540_45,
                27_340_159.710_375_622,
            ],
            [
                0.108_987_197_969_511_37,
                0.119_731_815_493_540_46,
                0.027_340_159_710_375_622,
            ],
            [
                108.987_197_969_511_36,
                119.731_815_493_540_45,
                27.340_159_710_375_62,
            ],
        ];
        let mut staged = beginnings;
        for (ordinal, receipt) in receipts.into_iter().enumerate() {
            let debits: Vec<V11ResourceDebit> = keys
                .iter()
                .enumerate()
                .map(|(resource, key)| {
                    let beginning = staged[resource];
                    let amount = amounts[resource][ordinal];
                    let ending = beginning - amount;
                    staged[resource] = ending;
                    V11ResourceDebit {
                        owner_id: if resource == 0 { "hydrology" } else { "bgc" }.into(),
                        resource_key: key.clone(),
                        beginning_amount: beginning,
                        amount,
                        ending_amount: ending,
                    }
                })
                .collect();
            if ordinal == 2 {
                let mut regrouped = debits.clone();
                let cumulative = amounts[0]
                    .iter()
                    .fold(0.0_f64, |total, amount| total + amount);
                regrouped[0].ending_amount = beginnings[0] - cumulative;
                assert_ne!(
                    regrouped[0].ending_amount.to_bits(),
                    debits[0].ending_amount.to_bits()
                );
                let before = parent.checkpoint();
                let poison = staged_candidate_with_debits(&parent, receipt.clone(), regrouped);
                assert!(matches!(
                    parent.accept_segment(&migrated.configuration, poison),
                    Err(V11Error::ResourceDebit)
                ));
                assert_eq!(parent.checkpoint(), before);
            }
            let candidate = staged_candidate_with_debits(&parent, receipt, debits);
            parent
                .accept_segment(&migrated.configuration, candidate)
                .expect("sequential custody");
        }
        assert_eq!(staged[0].to_bits(), 241_296_780.792_514_32_f64.to_bits());
        assert_eq!(staged[1].to_bits(), 0.241_296_780_792_514_43_f64.to_bits());
        assert_eq!(staged[2].to_bits(), 241.296_780_792_514_34_f64.to_bits());
        for (resource, key) in keys.iter().enumerate() {
            let owner = if resource == 0 { "hydrology" } else { "bgc" };
            let cumulative = parent
                .cumulative_debits
                .get(&(owner.into(), key.clone()))
                .expect("cumulative diagnostic");
            assert_ne!(
                (beginnings[resource] - cumulative).to_bits(),
                staged[resource].to_bits()
            );
        }
        let checkpoint = parent.checkpoint();
        V11ParentTransaction::restore(&migrated.configuration, checkpoint)
            .expect("nonassociative checkpoint");
    }

    #[cfg(any())]
    #[test]
    fn wrong_segment_ending_rejects_atomically() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) = accepted_receipts(&owners, &[1_800_000_000_000]);
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let before = parent.checkpoint();
        let poison = V11ResourceDebit {
            owner_id: "hydrology".into(),
            resource_key: V11ResourceKey::Water(WaterResourceKey {
                occupancy_id: openwepp_kernel_contract::OccupancyId {
                    stratum_id: openwepp_kernel_contract::StratumId::try_new("s1")
                        .expect("stratum"),
                    tile_id: openwepp_kernel_contract::TileId::try_new("t1").expect("tile"),
                },
                layer_id: openwepp_kernel_contract::SoilLayerId::try_new("l1").expect("layer"),
            }),
            beginning_amount: 497_355_953.965_941_8,
            amount: 256_059_173.173_427_4,
            ending_amount: 241_296_780.792_514_32,
        };
        let candidate = staged_candidate(&parent, receipts[0].clone(), Some(poison));
        assert!(matches!(
            parent.accept_segment(&migrated.configuration, candidate),
            Err(V11Error::ResourceDebit)
        ));
        assert_eq!(parent.checkpoint(), before);
    }
    #[path = "v11_bgc_tests.rs"]
    mod bgc_tests;
    #[path = "v11_custody_tests.rs"]
    mod custody_tests;
}
