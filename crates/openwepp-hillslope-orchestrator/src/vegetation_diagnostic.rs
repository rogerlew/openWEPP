//! Default-off coupled-vegetation diagnostic consumer.
//!
//! This module is intentionally absent from every production runtime selector.
use std::collections::BTreeMap;

use openwepp_biogeochemistry::{
    BiogeochemistryError, BiogeochemistryOwnerCandidate, BiogeochemistryState, MaterialPool,
    MaterialProposal, TransformationsMode, available_by_key, construct_biogeochemistry_candidate,
};
use openwepp_kernel_contract::{
    MineralNitrogenKey, ResourceAmountBasis, SoilLayerId, TransactionId,
    authorize_proportionally as authorize_resources_proportionally, authorize_proportionally_by,
};
use openwepp_vegetation::{
    CoupledCandidate, CoupledOwnedState, FailurePoint, ModelDefinition, NitrogenArbiter,
    NitrogenAuthorization, NitrogenRequest, SnowFreeForcing, VegetationConfiguration,
    VegetationError, WaterArbiter, WaterArbitration, WaterAuthorizationReason, WaterOwnerCandidate,
    WaterOwnerSnapshot, WaterRequest, WaterUse, execute_candidate_with_failure,
    reconstruct_water_ending,
};
use serde::{Deserialize, Serialize};

pub use crate::vegetation_energy_owner::DiagnosticEnergyState;
use crate::vegetation_energy_owner::{
    CanopyHeatStorageMode, DiagnosticEnergyOwnerCandidate, EnergyOwnerError,
    construct_energy_owner_candidate,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DiagnosticOwnedState {
    pub vegetation: CoupledOwnedState,
    pub water: DiagnosticWaterState,
    pub biogeochemistry: BiogeochemistryState,
    pub energy: DiagnosticEnergyState,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DiagnosticWaterState {
    pub liquid_kg_m2: BTreeMap<SoilLayerId, f64>,
    pub last_transaction_id: u128,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosticReceipt {
    pub transaction_id: TransactionId,
    pub water_used: f64,
    pub nitrogen_used: f64,
    pub water_partial_authorizations: usize,
    pub nitrogen_partial_authorizations: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UncommittedCoupledTransaction {
    transaction_id: TransactionId,
    vegetation: CoupledCandidate,
    water: WaterOwnerCandidate,
    biogeochemistry: BiogeochemistryOwnerCandidate,
    energy: DiagnosticEnergyOwnerCandidate,
}

impl UncommittedCoupledTransaction {
    #[must_use]
    pub fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    fn validate(&self, beginning: &DiagnosticOwnedState) -> Result<(), DiagnosticError> {
        self.vegetation.validate_sealed()?;
        self.biogeochemistry.validate()?;
        if self.vegetation.transaction_id() != self.transaction_id
            || self.water.transaction_id() != self.transaction_id
            || self.biogeochemistry.transaction_id() != self.transaction_id
            || self.energy.transaction_id() != self.transaction_id
            || self.vegetation.beginning_state_sha256() != beginning.vegetation.state_sha256
            || self.water.beginning_kg_m2_by_layer() != &beginning.water.liquid_kg_m2
            || self.biogeochemistry.beginning() != &beginning.biogeochemistry
            || self.energy.beginning() != &beginning.energy
            || self.energy.proposal_identity().ending_state_sha256
                != self.vegetation.ending_state().state_sha256
            || self.vegetation.water_phase().water_owner_candidate() != &self.water
            || self.biogeochemistry.protocol() != self.vegetation.nitrogen_protocol()
        {
            return Err(DiagnosticError::OwnerEnvelopeIdentity(
                "VEGTXN-E-007 owner envelope identity",
            ));
        }
        let proposals = material_proposals(&self.vegetation)?;
        if proposals.len() != self.biogeochemistry.receipts().len()
            || proposals
                .iter()
                .zip(self.biogeochemistry.receipts())
                .any(|(proposal, receipt)| {
                    proposal.transaction_id != receipt.transaction_id
                        || proposal.owner_id != receipt.owner_id
                        || proposal.donor != receipt.donor
                        || proposal.receiver != receipt.receiver
                        || proposal.proposal_id != receipt.proposal_id
                        || proposal.amounts != receipt.amounts
                })
        {
            return Err(DiagnosticError::Biogeochemistry(
                BiogeochemistryError::MaterialClosure,
            ));
        }
        Ok(())
    }

    fn ending_state(&self) -> DiagnosticOwnedState {
        DiagnosticOwnedState {
            vegetation: self.vegetation.ending_state().clone(),
            water: DiagnosticWaterState {
                liquid_kg_m2: self.water.ending_kg_m2_by_layer().clone(),
                last_transaction_id: self.transaction_id.0,
            },
            biogeochemistry: self.biogeochemistry.ending().clone(),
            energy: self.energy.ending().clone(),
        }
    }
}

#[derive(Debug)]
pub enum DiagnosticError {
    Vegetation(VegetationError),
    Biogeochemistry(BiogeochemistryError),
    Energy(EnergyOwnerError),
    OwnerEnvelopeIdentity(&'static str),
    InjectedFailure,
}

impl std::fmt::Display for DiagnosticError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vegetation(error) => write!(formatter, "{error}"),
            Self::Biogeochemistry(error) => write!(formatter, "{error}"),
            Self::Energy(error) => write!(formatter, "{error}"),
            Self::OwnerEnvelopeIdentity(message) => write!(formatter, "{message}"),
            Self::InjectedFailure => write!(formatter, "VEGTXN-E-008 injected rollback failure"),
        }
    }
}

impl std::error::Error for DiagnosticError {}

impl From<VegetationError> for DiagnosticError {
    fn from(value: VegetationError) -> Self {
        Self::Vegetation(value)
    }
}
impl From<BiogeochemistryError> for DiagnosticError {
    fn from(value: BiogeochemistryError) -> Self {
        Self::Biogeochemistry(value)
    }
}
impl From<EnergyOwnerError> for DiagnosticError {
    fn from(value: EnergyOwnerError) -> Self {
        Self::Energy(value)
    }
}

struct ProportionalWater<'a> {
    available: &'a BTreeMap<SoilLayerId, f64>,
}
impl WaterArbiter for ProportionalWater<'_> {
    fn authorize(&self, requests: &[WaterRequest]) -> Result<WaterArbitration, VegetationError> {
        if requests
            .iter()
            .any(|request| !self.available.contains_key(&request.key.layer_id))
        {
            return Err(VegetationError::ResourceIdentity(
                "water request names an unknown owner layer".into(),
            ));
        }
        let authorizations = authorize_proportionally_by(
            requests,
            self.available,
            ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval,
            |key| key.layer_id.clone(),
        )
        .map_err(VegetationError::from)?;
        let owner_id = requests
            .first()
            .map(|request| request.owner_id.clone())
            .ok_or_else(|| VegetationError::Receipt("empty water request batch".into()))?;
        let reasons: BTreeMap<_, _> = requests
            .iter()
            .zip(&authorizations)
            .map(|(request, authorization)| {
                let reason = if request.amount.to_bits() == 0.0_f64.to_bits() {
                    WaterAuthorizationReason::ZeroDemand
                } else if authorization.amount.to_bits() == request.amount.to_bits() {
                    WaterAuthorizationReason::FullySupplied
                } else if self.available[&request.key.layer_id].to_bits() == 0.0_f64.to_bits() {
                    WaterAuthorizationReason::LiquidStorageLimit
                } else {
                    WaterAuthorizationReason::CompetingDemand
                };
                (request.key.clone(), reason)
            })
            .collect();
        let snapshot = WaterOwnerSnapshot::try_new(
            requests[0].transaction_id,
            owner_id,
            requests
                .iter()
                .map(|request| {
                    self.available
                        .get(&request.key.layer_id)
                        .copied()
                        .map(|amount| (request.key.layer_id.clone(), amount))
                        .ok_or(VegetationError::Domain("unknown water layer"))
                })
                .collect::<Result<BTreeMap<_, _>, _>>()?,
            reasons.clone(),
        )?;
        WaterArbitration::try_new(snapshot, authorizations, reasons)
    }

    fn candidate_from_finalized_use(
        &self,
        transaction_id: TransactionId,
        arbitration: &WaterArbitration,
        finalized_uses: &[WaterUse],
    ) -> Result<WaterOwnerCandidate, VegetationError> {
        let ending = reconstruct_water_ending(arbitration.snapshot(), finalized_uses)?;
        WaterOwnerCandidate::try_new(
            transaction_id,
            arbitration.snapshot().owner_id().clone(),
            arbitration.snapshot().clone(),
            ending,
            finalized_uses.to_vec(),
        )
    }

    fn authorize_zero_demand(
        &self,
        transaction_id: TransactionId,
        owner_id: &openwepp_kernel_contract::ResourceOwnerId,
    ) -> Result<WaterArbitration, VegetationError> {
        let snapshot = WaterOwnerSnapshot::try_new(
            transaction_id,
            owner_id.clone(),
            self.available.clone(),
            BTreeMap::new(),
        )?;
        WaterArbitration::try_new(snapshot, Vec::new(), BTreeMap::new())
    }
}

struct ProportionalNitrogen<'a> {
    available: &'a BTreeMap<MineralNitrogenKey, f64>,
}
impl NitrogenArbiter for ProportionalNitrogen<'_> {
    fn beginning_amount(&self, key: &MineralNitrogenKey) -> Result<f64, VegetationError> {
        self.available
            .get(key)
            .copied()
            .ok_or(VegetationError::Domain("unknown nitrogen inventory"))
    }
    fn authorize(
        &self,
        requests: &[NitrogenRequest],
    ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
        authorize_resources_proportionally(
            requests,
            self.available,
            ResourceAmountBasis::NitrogenKgPerSquareMeterInterval,
        )
        .map_err(VegetationError::from)
    }
}

fn material_proposals(
    vegetation: &CoupledCandidate,
) -> Result<Vec<MaterialProposal>, VegetationError> {
    vegetation
        .material_proposals()
        .iter()
        .map(|transfer| {
            Ok(MaterialProposal {
                transaction_id: transfer.transaction_id,
                owner_id: transfer.owner_id.clone(),
                donor: transfer.donor,
                receiver: transfer.receiver,
                proposal_id: transfer.proposal_id,
                amounts: MaterialPool {
                    carbon: transfer.carbon,
                    nitrogen: transfer.nitrogen,
                    dry_matter: transfer.dry_matter,
                },
            })
        })
        .collect()
}

pub fn run_default_off_diagnostic(
    owned: &mut DiagnosticOwnedState,
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    forcing: &SnowFreeForcing,
    available_water: &BTreeMap<SoilLayerId, f64>,
    inject_failure: bool,
) -> Result<DiagnosticReceipt, DiagnosticError> {
    run_default_off_diagnostic_at_phase(
        owned,
        model,
        config,
        forcing,
        available_water,
        inject_failure.then_some(FailurePoint::BeforeCommit),
    )
}

#[allow(clippy::too_many_lines)]
pub fn run_default_off_diagnostic_at_phase(
    owned: &mut DiagnosticOwnedState,
    model: &ModelDefinition,
    config: &VegetationConfiguration,
    forcing: &SnowFreeForcing,
    available_water: &BTreeMap<SoilLayerId, f64>,
    failure: Option<FailurePoint>,
) -> Result<DiagnosticReceipt, DiagnosticError> {
    if available_water != &owned.water.liquid_kg_m2
        || owned.water.last_transaction_id != owned.vegetation.last_transaction_id
        || owned.biogeochemistry.last_transaction_id != owned.vegetation.last_transaction_id
        || owned.energy.last_transaction_id != owned.vegetation.last_transaction_id
        || owned.energy.model_definition_sha256 != owned.vegetation.model_definition_sha256
        || owned.energy.configuration_sha256 != owned.vegetation.configuration_sha256
        || owned.energy.accepted_vegetation_state_sha256 != owned.vegetation.state_sha256
    {
        return Err(DiagnosticError::OwnerEnvelopeIdentity(
            "VEGTXN-E-007 diagnostic owner snapshot mismatch",
        ));
    }
    let water = ProportionalWater {
        available: available_water,
    };
    let available_n = available_by_key(&owned.biogeochemistry)?;
    let nitrogen = ProportionalNitrogen {
        available: &available_n,
    };
    let vegetation_failure = if failure == Some(FailurePoint::OwnerValidation) {
        None
    } else {
        failure
    };
    let vegetation_candidate = execute_candidate_with_failure(
        model,
        config,
        &owned.vegetation,
        forcing,
        &water,
        &nitrogen,
        vegetation_failure,
    )?;
    inject(failure, FailurePoint::ReceiverConstruction)?;
    let mut transfers = material_proposals(&vegetation_candidate)?;
    if failure == Some(FailurePoint::CrossOwnerMaterialReceiptMismatch) {
        let transfer = transfers
            .first_mut()
            .ok_or(DiagnosticError::InjectedFailure)?;
        transfer.amounts.carbon += f64::EPSILON * transfer.amounts.carbon.abs().max(1.0);
    }
    let (nitrogen_requests, nitrogen_authorizations, nitrogen_uses) =
        vegetation_candidate.nitrogen_protocol();
    let mut bgc_requests = nitrogen_requests.to_vec();
    if failure == Some(FailurePoint::CrossOwnerNitrogenProtocolMismatch) {
        let request = bgc_requests
            .first_mut()
            .ok_or(DiagnosticError::InjectedFailure)?;
        request.amount += f64::EPSILON * request.amount.abs().max(1.0);
    }
    inject(failure, FailurePoint::BiogeochemistryCandidate)?;
    let bgc_candidate = construct_biogeochemistry_candidate(
        &owned.biogeochemistry,
        vegetation_candidate.transaction_id(),
        &bgc_requests,
        nitrogen_authorizations,
        nitrogen_uses,
        &transfers,
        TransformationsMode::Disabled,
    )?;
    inject(failure, FailurePoint::ProposalReceiptValidation)?;
    let mut water_candidate = vegetation_candidate
        .water_phase()
        .water_owner_candidate()
        .clone();
    if failure == Some(FailurePoint::CrossOwnerWaterCandidateMismatch) {
        let mut altered_ending = water_candidate.ending_kg_m2_by_layer().clone();
        let ending = altered_ending
            .values_mut()
            .next()
            .ok_or(DiagnosticError::InjectedFailure)?;
        *ending += f64::EPSILON * ending.abs().max(1.0);
        let authorization_facts = vegetation_candidate
            .water_phase()
            .protocol()
            .1
            .iter()
            .map(|authorization| {
                (
                    authorization.key.clone(),
                    WaterAuthorizationReason::FullySupplied,
                )
            })
            .collect();
        let snapshot = WaterOwnerSnapshot::try_new(
            water_candidate.transaction_id(),
            water_candidate.owner_id().clone(),
            water_candidate.beginning_kg_m2_by_layer().clone(),
            authorization_facts,
        )?;
        water_candidate = WaterOwnerCandidate::try_new(
            water_candidate.transaction_id(),
            water_candidate.owner_id().clone(),
            snapshot,
            altered_ending,
            water_candidate.finalized_uses().to_vec(),
        )?;
    }
    let water_used = water_candidate
        .finalized_uses()
        .iter()
        .fold(0.0, |total, value| total + value.amount);
    let (water_requests, water_authorizations, _) = vegetation_candidate.water_phase().protocol();
    let water_partial_authorizations = water_requests
        .iter()
        .zip(water_authorizations)
        .filter(|(request, authorization)| {
            authorization.amount > 0.0 && authorization.amount < request.amount
        })
        .count();
    let nitrogen_total_used = vegetation_candidate
        .nitrogen_protocol()
        .2
        .iter()
        .fold(0.0, |total, value| total + value.amount);
    let nitrogen_partial_authorizations = nitrogen_requests
        .iter()
        .zip(nitrogen_authorizations)
        .filter(|(request, authorization)| {
            authorization.amount > 0.0 && authorization.amount < request.amount
        })
        .count();
    let transaction_id = vegetation_candidate.transaction_id();
    inject(failure, FailurePoint::EnergyOperandConstruction)?;
    let mut energy_proposals = vegetation_candidate.energy_proposals().clone();
    if failure == Some(FailurePoint::CrossOwnerEnergyIdentityMismatch) {
        energy_proposals.identity.ending_state_sha256 = "0".repeat(64);
    }
    let energy_candidate = construct_energy_owner_candidate(
        &owned.energy,
        &crate::vegetation_energy_owner::EnergyOwnerTopology::from_configuration(config)?,
        &energy_proposals,
        CanopyHeatStorageMode::EquilibriumZero,
    )?;
    inject(failure, FailurePoint::EnergyOwnerValidation)?;
    inject(failure, FailurePoint::ClosureValidation)?;
    let envelope_transaction_id = if failure == Some(FailurePoint::CrossOwnerTransactionMismatch) {
        TransactionId(transaction_id.0.checked_add(1).ok_or(
            DiagnosticError::OwnerEnvelopeIdentity("VEGTXN-E-007 poison transaction overflow"),
        )?)
    } else {
        transaction_id
    };
    let transaction = UncommittedCoupledTransaction {
        transaction_id: envelope_transaction_id,
        vegetation: vegetation_candidate,
        water: water_candidate,
        biogeochemistry: bgc_candidate,
        energy: energy_candidate,
    };
    inject(failure, FailurePoint::VegetationOwnerValidation)?;
    inject(failure, FailurePoint::WaterOwnerValidation)?;
    inject(failure, FailurePoint::BiogeochemistryOwnerValidation)?;
    inject(failure, FailurePoint::OwnerValidation)?;
    inject(failure, FailurePoint::CrossOwnerValidation)?;
    let mut validation_beginning = None;
    if failure == Some(FailurePoint::CrossOwnerBeginningStateMismatch) {
        let mut mismatched = owned.clone();
        mismatched.vegetation.state_sha256 = "0".repeat(64);
        validation_beginning = Some(mismatched);
    }
    transaction.validate(validation_beginning.as_ref().unwrap_or(owned))?;
    let ending = transaction.ending_state();
    inject(failure, FailurePoint::BeforeCommit)?;

    // No fallible operation below this atomic replacement boundary.
    *owned = ending;
    Ok(DiagnosticReceipt {
        transaction_id,
        water_used,
        nitrogen_used: nitrogen_total_used,
        water_partial_authorizations,
        nitrogen_partial_authorizations,
    })
}

fn inject(failure: Option<FailurePoint>, point: FailurePoint) -> Result<(), DiagnosticError> {
    if failure == Some(point) {
        return Err(DiagnosticError::InjectedFailure);
    }
    Ok(())
}
