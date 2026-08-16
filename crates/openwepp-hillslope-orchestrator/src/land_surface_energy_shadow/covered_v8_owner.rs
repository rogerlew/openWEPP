//! BGC receiving-owner construction and the heterogeneous covered V8 envelope.
//!
//! Projection and vegetation persistence are completed before this boundary.
//! This module validates the already sealed candidates, joins root D/A/F to the
//! actual real-hydrology owner, and lets BGC construct its own receipts. It
//! exposes no partial or whole-owner commit API.

use std::collections::BTreeMap;

use openwepp_biogeochemistry::{
    BiogeochemistryError, BiogeochemistryOwnerCandidate, BiogeochemistryState, MaterialPool,
    MaterialProposal, TransformationsMode, construct_biogeochemistry_candidate,
};
use openwepp_kernel_contract::TransactionId;
use openwepp_land_surface_energy::{
    AcceptedCoveredVegetationOperands, GroundWaterKey, RequestingComponent, WaterAmount,
};
use openwepp_vegetation::{
    NitrogenArbiter, UncommittedV8VegetationCandidate, V8ComponentOccupancyBinding,
    V8CoupledOwnedState, V8PersistentForcingReceipt, VegetationConfiguration, VegetationError,
    construct_uncommitted_v8_vegetation_candidate, execute_uncommitted_v8_persistent_phase,
};
use thiserror::Error;

use super::{
    CoveredForestShadowResult, UnifiedRealHydrologyCandidate, project_covered_forest_v8_passes,
};

#[derive(Clone, Debug, Error, PartialEq)]
pub enum CoveredV8OwnerEnvelopeError {
    #[error("V8 covered-owner identity failure: {0}")]
    Identity(&'static str),
    #[error(transparent)]
    LandSurface(#[from] openwepp_land_surface_energy::LandSurfaceEnergyError),
    #[error(transparent)]
    Vegetation(#[from] VegetationError),
    #[error(transparent)]
    Biogeochemistry(#[from] BiogeochemistryError),
    #[error(transparent)]
    Projection(#[from] super::V8ProjectionError),
}

/// Complete uncommitted covered owner set. LSE and soil-thermal candidates are
/// retained inside `physical`; the hydrology candidate is the actual unified
/// owner result rather than a reconstructed inventory.
#[derive(Clone, Debug, PartialEq)]
pub struct UncommittedCoveredV8OwnerEnvelope {
    transaction_id: TransactionId,
    vegetation: UncommittedV8VegetationCandidate,
    physical: CoveredForestShadowResult,
    biogeochemistry: BiogeochemistryOwnerCandidate,
}

impl UncommittedCoveredV8OwnerEnvelope {
    #[must_use]
    pub const fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub const fn vegetation(&self) -> &UncommittedV8VegetationCandidate {
        &self.vegetation
    }

    #[must_use]
    pub const fn hydrology(&self) -> &UnifiedRealHydrologyCandidate {
        self.physical.hydrology_candidate()
    }

    #[must_use]
    pub const fn physical(&self) -> &CoveredForestShadowResult {
        &self.physical
    }

    #[must_use]
    pub const fn biogeochemistry(&self) -> &BiogeochemistryOwnerCandidate {
        &self.biogeochemistry
    }

    pub fn validate(&self) -> Result<(), CoveredV8OwnerEnvelopeError> {
        self.vegetation.validate_sealed()?;
        self.physical
            .potential()
            .potential_vegetation_operands
            .validate()?;
        self.physical.final_tile().vegetation_operands.validate()?;
        self.biogeochemistry.validate()?;
        if self.transaction_id != self.vegetation.transaction_id()
            || self.transaction_id != self.physical.hydrology_candidate().transaction_id()
            || self.transaction_id != self.biogeochemistry.transaction_id()
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "heterogeneous transaction identity",
            ));
        }
        validate_actual_root_protocol(
            &self.physical.final_tile().vegetation_operands,
            self.physical.hydrology_candidate(),
        )?;
        compare_material_receipts(&self.vegetation, &self.biogeochemistry)
    }
}

/// Construct BGC independently from the sealed vegetation protocol and join it
/// to the already executed physical owners. No water authorization, physical
/// solve, vegetation persistence, or owner mutation is reachable here.
#[allow(clippy::too_many_arguments)]
pub fn construct_covered_v8_owner_envelope(
    physical: CoveredForestShadowResult,
    bindings: Vec<V8ComponentOccupancyBinding>,
    vegetation_configuration: &VegetationConfiguration,
    vegetation_beginning: &V8CoupledOwnedState,
    persistent_forcing: &V8PersistentForcingReceipt,
    nitrogen: &dyn NitrogenArbiter,
    biogeochemistry_beginning: &BiogeochemistryState,
) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8OwnerEnvelopeError> {
    let projected = project_covered_forest_v8_passes(
        &physical,
        bindings,
        vegetation_configuration,
        vegetation_beginning,
    )?;
    let persistent = execute_uncommitted_v8_persistent_phase(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        persistent_forcing,
        nitrogen,
    )?;
    let vegetation = construct_uncommitted_v8_vegetation_candidate(
        vegetation_configuration,
        vegetation_beginning,
        projected.potential(),
        projected.capped(),
        projected.final_state(),
        &persistent,
    )?;
    join_covered_v8_owner_envelope(physical, vegetation, biogeochemistry_beginning)
}

fn join_covered_v8_owner_envelope(
    physical: CoveredForestShadowResult,
    vegetation: UncommittedV8VegetationCandidate,
    biogeochemistry_beginning: &BiogeochemistryState,
) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8OwnerEnvelopeError> {
    vegetation.validate_sealed()?;
    physical
        .potential()
        .potential_vegetation_operands
        .validate()?;
    physical.final_tile().vegetation_operands.validate()?;
    if vegetation.transaction_id() != physical.hydrology_candidate().transaction_id()
        || biogeochemistry_beginning.last_transaction_id.checked_add(1)
            != Some(vegetation.transaction_id().0)
    {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "beginning owner lineage",
        ));
    }
    validate_actual_root_protocol(
        &physical.final_tile().vegetation_operands,
        physical.hydrology_candidate(),
    )?;
    let proposals = vegetation
        .material_proposals()
        .iter()
        .map(|value| MaterialProposal {
            transaction_id: value.transaction_id,
            owner_id: value.owner_id.clone(),
            donor: value.donor,
            receiver: value.receiver,
            proposal_id: value.proposal_id,
            amounts: MaterialPool {
                carbon: value.carbon,
                nitrogen: value.nitrogen,
                dry_matter: value.dry_matter,
            },
        })
        .collect::<Vec<_>>();
    let (requests, authorizations, uses) = vegetation.nitrogen_protocol();
    let biogeochemistry = construct_biogeochemistry_candidate(
        biogeochemistry_beginning,
        vegetation.transaction_id(),
        requests,
        authorizations,
        uses,
        &proposals,
        TransformationsMode::Disabled,
    )?;
    compare_material_receipts(&vegetation, &biogeochemistry)?;
    let envelope = UncommittedCoveredV8OwnerEnvelope {
        transaction_id: vegetation.transaction_id(),
        vegetation,
        physical,
        biogeochemistry,
    };
    envelope.validate()?;
    Ok(envelope)
}

fn validate_actual_root_protocol(
    source: &AcceptedCoveredVegetationOperands,
    hydrology: &UnifiedRealHydrologyCandidate,
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    let requests = rows_by_key(&hydrology.arbitration().requests)?;
    let authorizations = authorization_rows_by_key(&hydrology.arbitration().authorizations)?;
    let uses = rows_by_key(hydrology.finalized_uses())?;
    let mut expected_keys = std::collections::BTreeSet::new();
    for occupancy in &source.occupancies {
        for root in &occupancy.root_water {
            let key = &root.key;
            expected_keys.insert(key.clone());
            if key.transaction_id != source.transaction_id
                || key.requesting_owner_id != source.vegetation_owner_id
                || key.requesting_component != RequestingComponent::VegetationRoot
                || key.ofe_id != source.ofe_id
                || key.requesting_tile_id != source.tile_id
                || key.occupancy_id.as_ref() != Some(&occupancy.occupancy_id)
                || key.soil_layer_id.is_none()
                || requests.get(key).map(|value| value.to_bits())
                    != Some(root.request_kg_m2_stand_ground.to_bits())
                || authorizations.get(key).map(|value| value.to_bits())
                    != Some(root.authorization_kg_m2_stand_ground.to_bits())
                || uses.get(key).map(|value| value.to_bits())
                    != Some(root.finalized_use_kg_m2_stand_ground.to_bits())
            {
                return Err(CoveredV8OwnerEnvelopeError::Identity(
                    "root D/A/F actual-owner correspondence",
                ));
            }
        }
    }
    let actual_root_keys = requests
        .keys()
        .filter(|key| {
            key.requesting_owner_id == source.vegetation_owner_id
                && key.requesting_component == RequestingComponent::VegetationRoot
                && key.ofe_id == source.ofe_id
                && key.requesting_tile_id == source.tile_id
        })
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let actual_authorization_keys = authorizations
        .keys()
        .filter(|key| actual_root_keys.contains(*key))
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let actual_use_keys = uses
        .keys()
        .filter(|key| actual_root_keys.contains(*key))
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    if expected_keys != actual_root_keys
        || expected_keys != actual_authorization_keys
        || expected_keys != actual_use_keys
    {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "root D/A/F actual-owner key set",
        ));
    }
    Ok(())
}

fn rows_by_key(
    rows: &[WaterAmount],
) -> Result<BTreeMap<GroundWaterKey, f64>, CoveredV8OwnerEnvelopeError> {
    let mut values = BTreeMap::new();
    for row in rows {
        if values
            .insert(row.key.clone(), row.amount_kg_m2_stand_ground)
            .is_some()
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate actual hydrology water row",
            ));
        }
    }
    Ok(values)
}

fn authorization_rows_by_key(
    rows: &[openwepp_land_surface_energy::WaterAuthorization],
) -> Result<BTreeMap<GroundWaterKey, f64>, CoveredV8OwnerEnvelopeError> {
    let mut values = BTreeMap::new();
    for row in rows {
        if values
            .insert(row.key.clone(), row.amount_kg_m2_stand_ground)
            .is_some()
        {
            return Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate actual hydrology authorization row",
            ));
        }
    }
    Ok(values)
}

fn compare_material_receipts(
    vegetation: &UncommittedV8VegetationCandidate,
    biogeochemistry: &BiogeochemistryOwnerCandidate,
) -> Result<(), CoveredV8OwnerEnvelopeError> {
    let proposals = vegetation.material_proposals();
    let receipts = biogeochemistry.receipts();
    if proposals.len() != receipts.len()
        || proposals.iter().zip(receipts).any(|(proposal, receipt)| {
            proposal.transaction_id != receipt.transaction_id
                || proposal.owner_id != receipt.owner_id
                || proposal.donor != receipt.donor
                || proposal.receiver != receipt.receiver
                || proposal.proposal_id != receipt.proposal_id
                || proposal.carbon.to_bits() != receipt.amounts.carbon.to_bits()
                || proposal.nitrogen.to_bits() != receipt.amounts.nitrogen.to_bits()
                || proposal.dry_matter.to_bits() != receipt.amounts.dry_matter.to_bits()
        })
    {
        return Err(CoveredV8OwnerEnvelopeError::Identity(
            "vegetation proposal/BGC receipt correspondence",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};
    use openwepp_land_surface_energy::{
        ComponentId, OfeId, RequestingComponent, SourceId, StandGroundWaterAmountBasis,
        WaterSourceType,
    };

    use super::*;

    fn root_key() -> GroundWaterKey {
        GroundWaterKey {
            transaction_id: TransactionId(9),
            requesting_owner_id: ResourceOwnerId::try_new("vegetation-v8").expect("owner"),
            requesting_component: RequestingComponent::VegetationRoot,
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            requesting_tile_id: TileId::try_new("tile-1").expect("tile"),
            occupancy_id: Some(ComponentId::try_new("component-1").expect("component")),
            surface_id: None,
            surface_class: None,
            source_type: WaterSourceType::SoilLayerLiquid,
            source_id: SourceId::try_new("soil-1").expect("source"),
            source_tile_id: None,
            soil_layer_id: Some(SoilLayerId::try_new("soil-1").expect("layer")),
            amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
        }
    }

    #[test]
    fn actual_owner_rows_preserve_complete_key_and_exact_amount() {
        let row = WaterAmount {
            key: root_key(),
            amount_kg_m2_stand_ground: 0.125,
        };
        let mapped = rows_by_key(std::slice::from_ref(&row)).expect("unique row");
        assert_eq!(mapped.len(), 1);
        assert_eq!(
            mapped[&row.key].to_bits(),
            row.amount_kg_m2_stand_ground.to_bits()
        );
    }

    #[test]
    fn duplicate_actual_owner_key_is_rejected_before_join() {
        let row = WaterAmount {
            key: root_key(),
            amount_kg_m2_stand_ground: 0.125,
        };
        assert_eq!(
            rows_by_key(&[row.clone(), row]),
            Err(CoveredV8OwnerEnvelopeError::Identity(
                "duplicate actual hydrology water row"
            ))
        );
    }
}
