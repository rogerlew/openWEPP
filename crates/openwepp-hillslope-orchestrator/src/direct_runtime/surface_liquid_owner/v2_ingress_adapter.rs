//! Internal V2-to-liquid-arithmetic seam for current ingress and WB14.
//!
//! The returned V1-shaped value is an unpublished arithmetic carrier for the
//! already-existing WB14 implementation. It is never an owner envelope,
//! checkpoint, restart, migration, or production downgrade. The V2 envelope
//! remains the only persistent owner and retains all ice and enthalpy bytes.

use std::collections::BTreeMap;

use openwepp_kernel_contract::TransactionId;
use serde::{Deserialize, Serialize};

use super::{
    DirectSurfaceLiquidContinuationState, DirectSurfaceLiquidError, DirectSurfaceLiquidOwnedState,
    DirectSurfaceLiquidPhase, DirectSurfaceLiquidResourceCandidate, DirectSurfaceLiquidStateRecord,
    SurfaceLiquidConfigurationV2, SurfaceLiquidOwnedStateV2, SurfaceLiquidOwnerClosureRecordV2,
    SurfaceLiquidOwnerEnvelopeV2, SurfaceLiquidStateRecordV2, checked_surface_liquid_add,
    checked_surface_liquid_div, checked_surface_liquid_sub,
    validate_surface_liquid_owner_mass_closure_v2,
};
use crate::direct_runtime::surface_liquid_ingress::{
    DirectSurfaceLiquidIngressCandidate, DirectSurfaceLiquidIngressInput,
    DirectSurfaceLiquidParcelReceipt, DirectSurfaceLiquidReceiptDisposition,
    DirectSurfaceLiquidReceiptRecipient, DirectWb14CoupledChildBindingV1,
    DirectWb14ParentWorkingState,
    execute_surface_liquid_ingress_with_parent_state_and_coupled_binding,
};

const WB14_PARENT_WORKING_V2_SCHEMA: &str = "OPENWEPP_DIRECT_WB14_PARENT_WORKING_STATE_V3";

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DirectSurfaceLiquidResourceCandidateV2 {
    transaction_id: TransactionId,
    beginning_owner: SurfaceLiquidOwnerEnvelopeV2,
    phase_adjusted_owner: SurfaceLiquidOwnerEnvelopeV2,
    phase_closure: Vec<SurfaceLiquidOwnerClosureRecordV2>,
    liquid_arithmetic: DirectSurfaceLiquidResourceCandidate,
}

/// Candidate-only V2 WB14 parent. The nested V1-shaped value is the exact
/// existing liquid arithmetic/receipt engine; V2 remains the sole owner state.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DirectWb14ParentWorkingStateV2 {
    schema: String,
    surface_configuration_sha256: String,
    surface_model_definition_sha256: String,
    liquid_arithmetic: DirectWb14ParentWorkingState,
    persistent_beginning_owner: SurfaceLiquidOwnerEnvelopeV2,
    candidate_owner: SurfaceLiquidOwnerEnvelopeV2,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CanonicalDirectWb14ParentWorkingStateV2 {
    schema: String,
    surface_configuration_sha256: String,
    surface_model_definition_sha256: String,
    liquid_arithmetic_bytes: Vec<u8>,
    persistent_beginning_owner_bytes: Vec<u8>,
    candidate_owner_bytes: Vec<u8>,
}

impl DirectWb14ParentWorkingStateV2 {
    #[must_use]
    pub(crate) const fn candidate_owner(&self) -> &SurfaceLiquidOwnerEnvelopeV2 {
        &self.candidate_owner
    }

    pub(crate) fn restart_bytes(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        self.validate(configuration)?;
        serde_json::to_vec(&CanonicalDirectWb14ParentWorkingStateV2 {
            schema: self.schema.clone(),
            surface_configuration_sha256: self.surface_configuration_sha256.clone(),
            surface_model_definition_sha256: self.surface_model_definition_sha256.clone(),
            liquid_arithmetic_bytes: self
                .liquid_arithmetic
                .restart_bytes(configuration.parent())?,
            persistent_beginning_owner_bytes: self
                .persistent_beginning_owner
                .canonical_bytes(configuration.parent(), Some(configuration))?,
            candidate_owner_bytes: self
                .candidate_owner
                .canonical_bytes(configuration.parent(), Some(configuration))?,
        })
        .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 parent V2 restart serialization"))
    }

    pub(crate) fn from_restart_bytes(
        configuration: &SurfaceLiquidConfigurationV2,
        bytes: &[u8],
    ) -> Result<Self, DirectSurfaceLiquidError> {
        let wire: CanonicalDirectWb14ParentWorkingStateV2 = serde_json::from_slice(bytes)
            .map_err(|_| DirectSurfaceLiquidError::Schema("WB14 parent V2 restart decoding"))?;
        let value = Self {
            schema: wire.schema,
            surface_configuration_sha256: wire.surface_configuration_sha256,
            surface_model_definition_sha256: wire.surface_model_definition_sha256,
            liquid_arithmetic: DirectWb14ParentWorkingState::from_restart_bytes(
                configuration.parent(),
                &wire.liquid_arithmetic_bytes,
            )?,
            persistent_beginning_owner: SurfaceLiquidOwnerEnvelopeV2::from_canonical_bytes(
                configuration.parent(),
                Some(configuration),
                &wire.persistent_beginning_owner_bytes,
            )?,
            candidate_owner: SurfaceLiquidOwnerEnvelopeV2::from_canonical_bytes(
                configuration.parent(),
                Some(configuration),
                &wire.candidate_owner_bytes,
            )?,
        };
        value.validate(configuration)?;
        if value.restart_bytes(configuration)? != bytes {
            return Err(DirectSurfaceLiquidError::Schema(
                "noncanonical WB14 parent V2 restart bytes",
            ));
        }
        Ok(value)
    }

    fn validate(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<(), DirectSurfaceLiquidError> {
        if self.schema != WB14_PARENT_WORKING_V2_SCHEMA
            || self.surface_configuration_sha256 != configuration.configuration_sha256()
            || self.surface_model_definition_sha256
                != configuration.model_definition().model_definition_sha256()
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "WB14 parent V2 schema/configuration/model mismatch",
            ));
        }
        let persistent = self.persistent_beginning_owner.v2_state().ok_or(
            DirectSurfaceLiquidError::Identity("WB14 parent V2 persistent beginning is not V2"),
        )?;
        let candidate =
            self.candidate_owner
                .v2_state()
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "WB14 parent V2 candidate is not V2",
                ))?;
        self.persistent_beginning_owner
            .canonical_bytes(configuration.parent(), Some(configuration))?;
        self.candidate_owner
            .canonical_bytes(configuration.parent(), Some(configuration))?;
        let projected_persistent = extract_v2_liquid_arithmetic_state(configuration, persistent)?;
        self.liquid_arithmetic
            .validate_receiving_owner(&projected_persistent)?;
        let projected_candidate = extract_v2_liquid_arithmetic_state(configuration, candidate)?;
        if self.liquid_arithmetic.candidate_state() != &projected_candidate {
            return Err(DirectSurfaceLiquidError::Identity(
                "WB14 parent V2 liquid/ice owner join mismatch",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DirectSurfaceLiquidIngressCandidateV2 {
    beginning_owner: SurfaceLiquidOwnerEnvelopeV2,
    phase_adjusted_owner: SurfaceLiquidOwnerEnvelopeV2,
    ending_owner: SurfaceLiquidOwnerEnvelopeV2,
    inner: DirectSurfaceLiquidIngressCandidate,
    parent_working_state: Option<DirectWb14ParentWorkingStateV2>,
}

impl DirectSurfaceLiquidIngressCandidateV2 {
    #[must_use]
    pub(crate) const fn beginning_owner(&self) -> &SurfaceLiquidOwnerEnvelopeV2 {
        &self.beginning_owner
    }

    #[must_use]
    pub(crate) const fn phase_adjusted_owner(&self) -> &SurfaceLiquidOwnerEnvelopeV2 {
        &self.phase_adjusted_owner
    }

    #[must_use]
    pub(crate) const fn ending_owner(&self) -> &SurfaceLiquidOwnerEnvelopeV2 {
        &self.ending_owner
    }

    #[must_use]
    pub(crate) const fn inner(&self) -> &DirectSurfaceLiquidIngressCandidate {
        &self.inner
    }

    #[must_use]
    pub(crate) const fn parent_working_state(&self) -> Option<&DirectWb14ParentWorkingStateV2> {
        self.parent_working_state.as_ref()
    }

    pub(crate) fn validate(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
        resource: &DirectSurfaceLiquidResourceCandidateV2,
        input: &DirectSurfaceLiquidIngressInput,
    ) -> Result<(), DirectSurfaceLiquidError> {
        resource.validate(configuration)?;
        if &self.beginning_owner != resource.beginning_owner()
            || &self.phase_adjusted_owner != resource.phase_adjusted_owner()
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 ingress resource identity mismatch",
            ));
        }
        if self.inner.parent_working_state().is_none() {
            self.inner
                .validate(configuration.parent(), resource.liquid_arithmetic(), input)?;
        } else {
            // An open parent deliberately keeps persistent predecessor bits
            // while its nested candidate carries physical child progress.
            // The frozen V1 terminal validator expects publishable lineage,
            // so validate the open parent's complete restart/receipt state
            // here and defer terminal candidate validation to finalization.
            self.inner
                .parent_working_state()
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "surface-owner V2 open parent disappeared",
                ))?
                .restart_bytes(configuration.parent())?;
        }
        let physical = self.inner.parent_working_state().map_or_else(
            || self.inner.ending_state(),
            DirectWb14ParentWorkingState::candidate_state,
        );
        let expected = reconstruct_v2_after_liquid_ingress(
            configuration,
            resource.phase_adjusted_owner(),
            physical,
            self.inner.receipts(),
            input.transaction_id,
        )?;
        if expected != self.ending_owner {
            return Err(DirectSurfaceLiquidError::Closure(
                "surface-owner V2 ingress ending owner does not reconstruct",
            ));
        }
        match (
            &self.parent_working_state,
            self.inner.parent_working_state(),
        ) {
            (Some(parent), Some(_)) => {
                parent.validate(configuration)?;
                if parent.candidate_owner != self.ending_owner {
                    return Err(DirectSurfaceLiquidError::Identity(
                        "surface-owner V2 parent/candidate owner mismatch",
                    ));
                }
            }
            (None, None) => {}
            _ => {
                return Err(DirectSurfaceLiquidError::Identity(
                    "surface-owner V2 parent working-state omission",
                ));
            }
        }
        Ok(())
    }
}

pub(crate) fn execute_surface_liquid_ingress_v2(
    configuration: &SurfaceLiquidConfigurationV2,
    resource: &DirectSurfaceLiquidResourceCandidateV2,
    input: &DirectSurfaceLiquidIngressInput,
) -> Result<DirectSurfaceLiquidIngressCandidateV2, DirectSurfaceLiquidError> {
    execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding(
        configuration,
        resource,
        input,
        None,
        true,
        None,
    )
}

pub(crate) fn execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding(
    configuration: &SurfaceLiquidConfigurationV2,
    resource: &DirectSurfaceLiquidResourceCandidateV2,
    input: &DirectSurfaceLiquidIngressInput,
    parent_working_state: Option<&DirectWb14ParentWorkingStateV2>,
    finalize_parent_interval: bool,
    coupled_binding: Option<DirectWb14CoupledChildBindingV1>,
) -> Result<DirectSurfaceLiquidIngressCandidateV2, DirectSurfaceLiquidError> {
    resource.validate(configuration)?;
    if input.transaction_id != resource.transaction_id() {
        return Err(DirectSurfaceLiquidError::Identity(
            "surface-owner V2 current-ingress transaction mismatch",
        ));
    }
    let adjusted_parent = parent_working_state
        .map(|parent| {
            parent.validate(configuration)?;
            if &parent.candidate_owner != resource.beginning_owner() {
                return Err(DirectSurfaceLiquidError::Identity(
                    "surface-owner V2 parent/resource beginning mismatch",
                ));
            }
            let projected = extract_v2_liquid_arithmetic_state(
                configuration,
                resource.phase_adjusted_state()?,
            )?;
            parent
                .liquid_arithmetic
                .with_zero_duration_receiver_candidate(configuration.parent(), projected)
        })
        .transpose()?;
    let inner = execute_surface_liquid_ingress_with_parent_state_and_coupled_binding(
        configuration.parent(),
        resource.liquid_arithmetic(),
        input,
        adjusted_parent.as_ref(),
        finalize_parent_interval,
        coupled_binding,
    )?;
    let physical = inner.parent_working_state().map_or_else(
        || inner.ending_state(),
        DirectWb14ParentWorkingState::candidate_state,
    );
    let ending_owner = reconstruct_v2_after_liquid_ingress(
        configuration,
        resource.phase_adjusted_owner(),
        physical,
        inner.receipts(),
        input.transaction_id,
    )?;
    let parent_working_state =
        inner
            .parent_working_state()
            .map(|liquid_arithmetic| DirectWb14ParentWorkingStateV2 {
                schema: WB14_PARENT_WORKING_V2_SCHEMA.into(),
                surface_configuration_sha256: configuration.configuration_sha256().into(),
                surface_model_definition_sha256: configuration
                    .model_definition()
                    .model_definition_sha256()
                    .into(),
                liquid_arithmetic: liquid_arithmetic.clone(),
                persistent_beginning_owner: parent_working_state.map_or_else(
                    || resource.phase_adjusted_owner().clone(),
                    |parent| parent.persistent_beginning_owner.clone(),
                ),
                candidate_owner: ending_owner.clone(),
            });
    if let Some(parent) = &parent_working_state {
        parent.validate(configuration)?;
    }
    let candidate = DirectSurfaceLiquidIngressCandidateV2 {
        beginning_owner: resource.beginning_owner().clone(),
        phase_adjusted_owner: resource.phase_adjusted_owner().clone(),
        ending_owner,
        inner,
        parent_working_state,
    };
    candidate.validate(configuration, resource, input)?;
    Ok(candidate)
}

fn reconstruct_v2_after_liquid_ingress(
    configuration: &SurfaceLiquidConfigurationV2,
    phase_adjusted_owner: &SurfaceLiquidOwnerEnvelopeV2,
    physical: &DirectSurfaceLiquidOwnedState,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
    transaction_id: TransactionId,
) -> Result<SurfaceLiquidOwnerEnvelopeV2, DirectSurfaceLiquidError> {
    let phase_adjusted =
        phase_adjusted_owner
            .v2_state()
            .ok_or(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 ingress phase owner is not V2",
            ))?;
    let projected = extract_v2_liquid_arithmetic_state(configuration, phase_adjusted)?;
    if projected.records.len() != physical.records.len()
        || projected.continuations.len() != physical.continuations.len()
    {
        return Err(DirectSurfaceLiquidError::Identity(
            "surface-owner V2 ingress arithmetic cardinality mismatch",
        ));
    }
    let retained_enthalpy = retained_enthalpy_by_store(receipts, transaction_id)?;
    let records = v2_ingress_records(configuration, phase_adjusted, physical, &retained_enthalpy)?;
    let ending = phase_adjusted_owner.try_replace_v2_state(
        configuration,
        records,
        physical.continuations.clone(),
    )?;
    let ending_state = ending.v2_state().ok_or(DirectSurfaceLiquidError::Identity(
        "surface-owner V2 ingress ending is not V2",
    ))?;
    let closure = phase_adjusted
        .records()
        .iter()
        .zip(ending_state.records())
        .map(|(beginning, ending)| {
            Ok(SurfaceLiquidOwnerClosureRecordV2 {
                key: beginning.key.clone(),
                liquid_debit_kg_m2_tile: 0.0,
                liquid_credit_kg_m2_tile: checked_surface_liquid_sub(
                    ending.liquid_kg_m2_tile,
                    beginning.liquid_kg_m2_tile,
                )
                .ok_or(DirectSurfaceLiquidError::Closure(
                    "surface-owner V2 ingress liquid credit reconstruction",
                ))?,
                ice_debit_kg_m2_tile: 0.0,
                ice_credit_kg_m2_tile: 0.0,
            })
        })
        .collect::<Result<Vec<_>, DirectSurfaceLiquidError>>()?;
    validate_surface_liquid_owner_mass_closure_v2(
        configuration,
        phase_adjusted,
        ending_state,
        &closure,
    )?;
    if phase_adjusted
        .records()
        .iter()
        .zip(ending_state.records())
        .any(|(beginning, ending)| {
            beginning.litter_ice_kg_m2_tile.to_bits() != ending.litter_ice_kg_m2_tile.to_bits()
        })
    {
        return Err(DirectSurfaceLiquidError::unsupported_domain_failure(
            DirectSurfaceLiquidPhase::IngressCandidate,
            super::DirectSurfaceLiquidErrorContext::default(),
            Some(phase_adjusted_owner.envelope_sha256().into()),
            "surface-owner V2 current ingress or WB14 mutated litter ice",
        ));
    }
    Ok(ending)
}

fn retained_enthalpy_by_store(
    receipts: &[DirectSurfaceLiquidParcelReceipt],
    transaction_id: TransactionId,
) -> Result<BTreeMap<super::DirectSurfaceLiquidStoreKey, f64>, DirectSurfaceLiquidError> {
    let mut retained_enthalpy = BTreeMap::new();
    for receipt in receipts {
        if receipt.disposition != DirectSurfaceLiquidReceiptDisposition::RetainedSurface {
            continue;
        }
        let DirectSurfaceLiquidReceiptRecipient::SurfaceStore { store_key } = &receipt.recipient
        else {
            return Err(DirectSurfaceLiquidError::Identity(
                "retained surface receipt has a non-surface recipient",
            ));
        };
        if receipt.transaction_id != transaction_id || &receipt.recipient_store_key != store_key {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 retained receipt identity mismatch",
            ));
        }
        let entry = retained_enthalpy.entry(store_key.clone()).or_insert(0.0);
        *entry = checked_surface_liquid_add(*entry, receipt.enthalpy_j_m2_basis_ofe_ground).ok_or(
            DirectSurfaceLiquidError::Closure("surface-owner V2 retained enthalpy accumulation"),
        )?;
    }
    Ok(retained_enthalpy)
}

fn v2_ingress_records(
    configuration: &SurfaceLiquidConfigurationV2,
    phase_adjusted: &SurfaceLiquidOwnedStateV2,
    physical: &DirectSurfaceLiquidOwnedState,
    retained_enthalpy: &BTreeMap<super::DirectSurfaceLiquidStoreKey, f64>,
) -> Result<Vec<SurfaceLiquidStateRecordV2>, DirectSurfaceLiquidError> {
    phase_adjusted
        .records()
        .iter()
        .zip(&physical.records)
        .zip(configuration.parent().records.iter())
        .map(|((beginning, ending), configured)| {
            if beginning.key != ending.key || beginning.key != configured.key {
                return Err(DirectSurfaceLiquidError::Identity(
                    "surface-owner V2 ingress key/order mismatch",
                ));
            }
            if ending.liquid_kg_m2_tile < beginning.liquid_kg_m2_tile {
                return Err(DirectSurfaceLiquidError::Closure(
                    "surface-owner V2 WB14 debited phase-adjusted storage",
                ));
            }
            let retained_q_ofe = retained_enthalpy
                .get(&beginning.key)
                .copied()
                .unwrap_or(0.0);
            let retained_q_tile = if retained_q_ofe == 0.0 {
                0.0
            } else {
                checked_surface_liquid_div(retained_q_ofe, configured.tile_fraction).ok_or(
                    DirectSurfaceLiquidError::Closure(
                        "surface-owner V2 retained enthalpy basis conversion",
                    ),
                )?
            };
            let surface_enthalpy =
                checked_surface_liquid_add(beginning.surface_enthalpy_j_m2_tile, retained_q_tile)
                    .ok_or(DirectSurfaceLiquidError::Closure(
                    "surface-owner V2 ending enthalpy accumulation",
                ))?;
            Ok(SurfaceLiquidStateRecordV2 {
                key: beginning.key.clone(),
                liquid_kg_m2_tile: ending.liquid_kg_m2_tile,
                litter_ice_kg_m2_tile: beginning.litter_ice_kg_m2_tile,
                surface_enthalpy_j_m2_tile: surface_enthalpy,
                last_accepted_transaction_id: ending.last_accepted_transaction_id,
            })
        })
        .collect()
}

impl DirectSurfaceLiquidResourceCandidateV2 {
    #[must_use]
    pub(crate) const fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub(crate) const fn beginning_owner(&self) -> &SurfaceLiquidOwnerEnvelopeV2 {
        &self.beginning_owner
    }

    #[must_use]
    pub(crate) const fn phase_adjusted_owner(&self) -> &SurfaceLiquidOwnerEnvelopeV2 {
        &self.phase_adjusted_owner
    }

    pub(crate) fn phase_adjusted_state(
        &self,
    ) -> Result<&SurfaceLiquidOwnedStateV2, DirectSurfaceLiquidError> {
        self.phase_adjusted_owner
            .v2_state()
            .ok_or(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 resource candidate contains a V1 owner",
            ))
    }

    pub(crate) const fn liquid_arithmetic(&self) -> &DirectSurfaceLiquidResourceCandidate {
        &self.liquid_arithmetic
    }

    pub(crate) fn validate(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<(), DirectSurfaceLiquidError> {
        if self.transaction_id.0 == 0
            || self.liquid_arithmetic.transaction_id() != self.transaction_id
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 resource transaction mismatch",
            ));
        }
        self.beginning_owner
            .canonical_bytes(configuration.parent(), Some(configuration))?;
        self.phase_adjusted_owner
            .canonical_bytes(configuration.parent(), Some(configuration))?;
        let beginning =
            self.beginning_owner
                .v2_state()
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "surface-owner V2 resource beginning is not V2",
                ))?;
        let phase_adjusted = self.phase_adjusted_state()?;
        if beginning.continuations() != phase_adjusted.continuations()
            || beginning
                .records()
                .iter()
                .zip(phase_adjusted.records())
                .any(|(left, right)| {
                    left.key != right.key
                        || left.last_accepted_transaction_id != right.last_accepted_transaction_id
                })
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 phase changed current-ingress lineage",
            ));
        }
        validate_surface_liquid_owner_mass_closure_v2(
            configuration,
            beginning,
            phase_adjusted,
            &self.phase_closure,
        )?;
        self.liquid_arithmetic.validate(configuration.parent())?;
        let projected = extract_v2_liquid_arithmetic_state(configuration, phase_adjusted)?;
        if self.liquid_arithmetic.beginning_state() != &projected
            || self.liquid_arithmetic.working_state() != &projected
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 liquid arithmetic substituted phase state",
            ));
        }
        Ok(())
    }
}

pub(crate) fn prepare_surface_liquid_resource_candidate_v2(
    configuration: &SurfaceLiquidConfigurationV2,
    beginning_owner: &SurfaceLiquidOwnerEnvelopeV2,
    phase_adjusted_owner: &SurfaceLiquidOwnerEnvelopeV2,
    transaction_id: TransactionId,
    phase_closure: &[SurfaceLiquidOwnerClosureRecordV2],
) -> Result<DirectSurfaceLiquidResourceCandidateV2, DirectSurfaceLiquidError> {
    let phase_adjusted =
        phase_adjusted_owner
            .v2_state()
            .ok_or(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 resource phase requires a V2 owner",
            ))?;
    let arithmetic = extract_v2_liquid_arithmetic_state(configuration, phase_adjusted)?;
    let predecessor = arithmetic.accepted_transaction()?;
    let liquid_arithmetic = DirectSurfaceLiquidResourceCandidate {
        transaction_id,
        beginning_state: arithmetic.clone(),
        working_state: arithmetic,
        finalized_uses: Vec::new(),
        condensation_credits: Vec::new(),
        condensation_overflow: Vec::new(),
        requests: Vec::new(),
        authorizations: Vec::new(),
        request_store_keys: Vec::new(),
        expected_predecessor: predecessor,
    };
    let candidate = DirectSurfaceLiquidResourceCandidateV2 {
        transaction_id,
        beginning_owner: beginning_owner.clone(),
        phase_adjusted_owner: phase_adjusted_owner.clone(),
        phase_closure: phase_closure.to_vec(),
        liquid_arithmetic,
    };
    candidate.validate(configuration)?;
    Ok(candidate)
}

fn extract_v2_liquid_arithmetic_state(
    configuration: &SurfaceLiquidConfigurationV2,
    state: &SurfaceLiquidOwnedStateV2,
) -> Result<DirectSurfaceLiquidOwnedState, DirectSurfaceLiquidError> {
    state.canonical_bytes(configuration)?;
    let records = state
        .records()
        .iter()
        .map(|record| DirectSurfaceLiquidStateRecord {
            key: record.key.clone(),
            liquid_kg_m2_tile: record.liquid_kg_m2_tile,
            last_accepted_transaction_id: record.last_accepted_transaction_id,
        })
        .collect();
    let continuations = state
        .continuations()
        .iter()
        .map(|row| DirectSurfaceLiquidContinuationState {
            ofe_id: row.ofe_id.clone(),
            day_index: row.day_index,
            next_interval_index: row.next_interval_index,
            cumulative_supply_m: row.cumulative_supply_m,
            cumulative_infiltration_m: row.cumulative_infiltration_m,
            last_accepted_transaction_id: row.last_accepted_transaction_id,
        })
        .collect();
    let mut projected = DirectSurfaceLiquidOwnedState {
        owner_id: configuration.parent().owner_id.clone(),
        configuration_sha256: configuration.parent().configuration_sha256.clone(),
        state_sha256: super::ZERO_SHA256.into(),
        records,
        continuations,
    };
    projected.state_sha256 = projected.recomputed_sha256()?;
    projected.validate(configuration.parent())?;
    Ok(projected)
}
