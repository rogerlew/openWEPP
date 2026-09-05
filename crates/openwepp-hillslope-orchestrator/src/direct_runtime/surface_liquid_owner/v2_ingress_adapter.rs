//! Internal V2-to-liquid-arithmetic seam for current ingress and WB14.
//!
//! The returned V1-shaped value is an unpublished arithmetic carrier for the
//! already-existing WB14 implementation. It is never an owner envelope,
//! checkpoint, restart, migration, or production downgrade. The V2 envelope
//! remains the only persistent owner and retains all ice and enthalpy bytes.

use std::collections::BTreeMap;
#[cfg(test)]
use std::sync::atomic::{AtomicUsize, Ordering};

use openwepp_kernel_contract::TransactionId;
use openwepp_land_surface_energy::{
    GroundWaterKey, LitterPhaseCapacitySpillV1, LitterPhaseReceipt, Sha256Digest,
    V3PhaseSpecificVaporAuthorization, WaterAmount, WaterAuthorization, WaterSourceType,
    validate_litter_phase_receipt,
};
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
    execute_surface_liquid_ingress_with_parent_state_and_coupled_binding_and_phase_capacity_spills,
};

const WB14_PARENT_WORKING_V2_SCHEMA: &str = "OPENWEPP_DIRECT_WB14_PARENT_WORKING_STATE_V3";

#[cfg(test)]
static SURFACE_RESOURCE_FULL_VALIDATIONS: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static SURFACE_RESOURCE_OWNER_SERIALIZATIONS: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static WB14_PARENT_V2_HANDOFF_VALIDATIONS: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static WB14_PARENT_V2_RESTART_SERIALIZATIONS: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static WB14_PARENT_V2_OWNER_CANONICALIZATIONS: AtomicUsize = AtomicUsize::new(0);

/// Private, immutable, nonserializable proof for one exact resource revision.
#[derive(Clone, Debug, PartialEq)]
struct ValidatedSurfaceLiquidResourceCandidateV2 {
    configuration_sha256: String,
    transaction_id: TransactionId,
    beginning_owner_sha256: String,
    native_phase_adjusted_owner_sha256: String,
    phase_adjusted_owner_sha256: String,
    arithmetic_beginning_sha256: String,
    arithmetic_working_sha256: String,
    expected_predecessor: Option<TransactionId>,
}

struct ValidatedSurfaceLiquidResourceCandidateRefV2<'a> {
    candidate: &'a DirectSurfaceLiquidResourceCandidateV2,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DirectSurfaceLiquidResourceCandidateV2 {
    transaction_id: TransactionId,
    beginning_owner: SurfaceLiquidOwnerEnvelopeV2,
    native_phase_adjusted_owner: SurfaceLiquidOwnerEnvelopeV2,
    phase_adjusted_owner: SurfaceLiquidOwnerEnvelopeV2,
    phase_closure: Vec<SurfaceLiquidOwnerClosureRecordV2>,
    phase_capacity_spills: Vec<LitterPhaseCapacitySpillV1>,
    heterogeneous_resource_join: Option<SurfaceLiquidV2HeterogeneousResourceJoinV1>,
    liquid_arithmetic: DirectSurfaceLiquidResourceCandidate,
    validated_revision: Option<ValidatedSurfaceLiquidResourceCandidateV2>,
}

/// Exact one-time bridge between the named native litter phase and ordinary
/// finalized surface withdrawals in the same heterogeneous fixed-final batch.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurfaceLiquidV2HeterogeneousResourceJoinV1 {
    transaction_id: TransactionId,
    native_phase_receipts: BTreeMap<GroundWaterKey, Sha256Digest>,
    ordinary_requests: Vec<WaterAmount>,
    ordinary_authorizations: Vec<WaterAuthorization>,
    ordinary_finalized_uses: Vec<WaterAmount>,
    native_phase_adjusted_owner_sha256: String,
    joined_phase_adjusted_owner_sha256: String,
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

/// Private, borrowed proof that one exact in-process V2 parent passed its
/// complete native validation. Durable/restart consumers must continue to use
/// the canonical byte boundary instead.
pub(crate) struct ValidatedDirectWb14ParentWorkingStateV2Ref<'a> {
    parent: &'a DirectWb14ParentWorkingStateV2,
}

impl ValidatedDirectWb14ParentWorkingStateV2Ref<'_> {
    #[must_use]
    pub(crate) fn has_same_liquid_arithmetic(&self, legacy: &DirectWb14ParentWorkingState) -> bool {
        &self.parent.liquid_arithmetic == legacy
    }

    /// Borrow the exact validated V1 arithmetic carried by this native V2
    /// parent. The proof is deliberately non-serializable and the borrow
    /// cannot outlive the validated V2 resident.
    #[must_use]
    pub(crate) const fn liquid_arithmetic(&self) -> &DirectWb14ParentWorkingState {
        &self.parent.liquid_arithmetic
    }
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
    pub(crate) fn try_begin_after_native_inactive_prefix(
        configuration: &SurfaceLiquidConfigurationV2,
        native_owner: &SurfaceLiquidOwnerEnvelopeV2,
        transaction_id: TransactionId,
        day_index: usize,
        interval_index: usize,
        wb14_parameters: &[crate::direct_runtime::DirectOfeWb14Parameters],
        coupled_binding: DirectWb14CoupledChildBindingV1,
        prefix: crate::direct_runtime::ValidatedNativeInactiveWb14PrefixV1,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        native_owner.canonical_bytes(configuration.parent(), Some(configuration))?;
        let native_state = native_owner
            .v2_state()
            .ok_or(DirectSurfaceLiquidError::Identity(
                "native inactive-prefix surface owner is not V2",
            ))?;
        let liquid_beginning = extract_v2_liquid_arithmetic_state(configuration, native_state)?;
        let liquid_arithmetic = DirectWb14ParentWorkingState::begin_after_native_inactive_prefix(
            configuration.parent(),
            transaction_id,
            day_index,
            interval_index,
            wb14_parameters,
            &liquid_beginning,
            coupled_binding,
            prefix,
        )?;
        Self::try_from_validated_liquid_arithmetic(configuration, native_owner, &liquid_arithmetic)
    }

    /// Adopt an already-validated V1 WB14 parent when native V2 custody is
    /// installed after the parent interval has opened. The V1 parent owns all
    /// liquid arithmetic; the current V2 owner supplies only its native ice
    /// and enthalpy coordinates. The constructed native parent is fully
    /// validated before it can enter the resident.
    pub(crate) fn try_from_validated_liquid_arithmetic(
        configuration: &SurfaceLiquidConfigurationV2,
        native_owner: &SurfaceLiquidOwnerEnvelopeV2,
        liquid_arithmetic: &DirectWb14ParentWorkingState,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        native_owner.canonical_bytes(configuration.parent(), Some(configuration))?;
        liquid_arithmetic.validate_in_process(configuration.parent())?;
        let value = Self {
            schema: WB14_PARENT_WORKING_V2_SCHEMA.into(),
            surface_configuration_sha256: configuration.configuration_sha256().into(),
            surface_model_definition_sha256: configuration
                .model_definition()
                .model_definition_sha256()
                .into(),
            liquid_arithmetic: liquid_arithmetic.clone(),
            persistent_beginning_owner: stage_v2_owner_liquid_arithmetic(
                configuration,
                native_owner,
                liquid_arithmetic.persistent_beginning_state(),
            )?,
            candidate_owner: stage_v2_owner_liquid_arithmetic(
                configuration,
                native_owner,
                liquid_arithmetic.candidate_state(),
            )?,
        };
        value.validate(configuration)?;
        Ok(value)
    }

    #[must_use]
    pub(crate) const fn candidate_owner(&self) -> &SurfaceLiquidOwnerEnvelopeV2 {
        &self.candidate_owner
    }

    pub(crate) fn restart_bytes(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<Vec<u8>, DirectSurfaceLiquidError> {
        #[cfg(test)]
        WB14_PARENT_V2_RESTART_SERIALIZATIONS.fetch_add(1, Ordering::Relaxed);
        self.validate(configuration)?;
        #[cfg(test)]
        WB14_PARENT_V2_OWNER_CANONICALIZATIONS.fetch_add(2, Ordering::Relaxed);
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

    /// Validate once at the trusted in-process carrier boundary and return a
    /// nonserializable proof that exposes only the exact nested arithmetic
    /// join needed by the V1 carrier envelope.
    pub(crate) fn validated_handoff(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<ValidatedDirectWb14ParentWorkingStateV2Ref<'_>, DirectSurfaceLiquidError> {
        self.validate(configuration)?;
        self.liquid_arithmetic
            .validate_in_process(configuration.parent())?;
        #[cfg(test)]
        WB14_PARENT_V2_HANDOFF_VALIDATIONS.fetch_add(1, Ordering::Relaxed);
        Ok(ValidatedDirectWb14ParentWorkingStateV2Ref { parent: self })
    }

    /// Reseal the nested liquid arithmetic against a final coupled slab while
    /// preserving the exact native ice/enthalpy owners.
    pub(crate) fn rebind_final_accepted_slab(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
        target: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        self.validate(configuration)?;
        let mut rebound = self.clone();
        rebound.liquid_arithmetic = self
            .liquid_arithmetic
            .rebind_final_accepted_slab(configuration.parent(), target)?;
        rebound.validate(configuration)?;
        Ok(rebound)
    }

    /// Stage one authenticated in-process V1 carrier parent into native V2
    /// custody without crossing the durable restart boundary. Only the liquid
    /// arithmetic projection is replaced; native ice and enthalpy remain in
    /// the V2 envelopes. The returned value is a new, fully validated revision
    /// so the proof for `self` cannot authorize the mutation.
    pub(crate) fn try_stage_validated_liquid_arithmetic(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
        liquid_arithmetic: &DirectWb14ParentWorkingState,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        self.validated_handoff(configuration)?;
        liquid_arithmetic.validate_in_process(configuration.parent())?;
        if self.liquid_arithmetic == *liquid_arithmetic {
            return Ok(self.clone());
        }
        let staged = Self {
            schema: self.schema.clone(),
            surface_configuration_sha256: self.surface_configuration_sha256.clone(),
            surface_model_definition_sha256: self.surface_model_definition_sha256.clone(),
            liquid_arithmetic: liquid_arithmetic.clone(),
            persistent_beginning_owner: stage_v2_owner_liquid_arithmetic(
                configuration,
                &self.persistent_beginning_owner,
                liquid_arithmetic.persistent_beginning_state(),
            )?,
            candidate_owner: stage_v2_owner_liquid_arithmetic(
                configuration,
                &self.candidate_owner,
                liquid_arithmetic.candidate_state(),
            )?,
        };
        staged.validate(configuration)?;
        if staged.liquid_arithmetic != *liquid_arithmetic {
            return Err(DirectSurfaceLiquidError::Identity(
                "staged WB14 parent V2 liquid arithmetic join",
            ));
        }
        Ok(staged)
    }

    /// Close one authenticated in-process carrier parent into an exact native
    /// V2 liquid owner. The old V2 candidate supplies only native ice and
    /// enthalpy custody; every liquid and continuation field comes from the
    /// finalized V1 result and is compared again after reconstruction.
    pub(crate) fn try_finalize_validated_liquid_owner(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
        finalized_liquid: &DirectSurfaceLiquidOwnedState,
    ) -> Result<SurfaceLiquidOwnerEnvelopeV2, DirectSurfaceLiquidError> {
        self.validated_handoff(configuration)?;
        finalized_liquid.validate(configuration.parent())?;
        let finalized = stage_v2_owner_liquid_arithmetic(
            configuration,
            &self.candidate_owner,
            finalized_liquid,
        )?;
        finalized.canonical_bytes(configuration.parent(), Some(configuration))?;
        let projected = extract_v2_liquid_arithmetic_state(
            configuration,
            finalized
                .v2_state()
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "finalized WB14 parent owner is not V2",
                ))?,
        )?;
        if &projected != finalized_liquid {
            return Err(DirectSurfaceLiquidError::Identity(
                "finalized WB14 parent V2 liquid arithmetic join",
            ));
        }
        Ok(finalized)
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
        #[cfg(test)]
        WB14_PARENT_V2_OWNER_CANONICALIZATIONS.fetch_add(2, Ordering::Relaxed);
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

    /// Rebind only the frozen binary64 enthalpy high mirrors after V16 has
    /// rounded the authoritative exact total once. Liquid arithmetic, mass,
    /// ice, continuation, receipts, and WB14 progression remain byte-for-byte
    /// the already accepted physical candidate.
    pub(crate) fn with_exact_surface_enthalpy_high_owner(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
        ending_owner: SurfaceLiquidOwnerEnvelopeV2,
    ) -> Result<Self, DirectSurfaceLiquidError> {
        ending_owner.canonical_bytes(configuration.parent(), Some(configuration))?;
        let physical = self
            .ending_owner
            .v2_state()
            .ok_or(DirectSurfaceLiquidError::Identity(
                "V16 physical ending owner is not V2",
            ))?;
        let exact_high = ending_owner
            .v2_state()
            .ok_or(DirectSurfaceLiquidError::Identity(
                "V16 exact-high ending owner is not V2",
            ))?;
        if physical.continuations() != exact_high.continuations()
            || physical.records().len() != exact_high.records().len()
            || physical
                .records()
                .iter()
                .zip(exact_high.records())
                .any(|(left, right)| {
                    left.key != right.key
                        || left.liquid_kg_m2_tile.to_bits() != right.liquid_kg_m2_tile.to_bits()
                        || left.litter_ice_kg_m2_tile.to_bits()
                            != right.litter_ice_kg_m2_tile.to_bits()
                        || left.last_accepted_transaction_id != right.last_accepted_transaction_id
                })
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "V16 high-only ending-owner replacement",
            ));
        }
        let parent_working_state = self
            .parent_working_state
            .as_ref()
            .map(|parent| {
                let mut adjusted = parent.clone();
                adjusted.candidate_owner.clone_from(&ending_owner);
                adjusted.validate(configuration)?;
                Ok(adjusted)
            })
            .transpose()?;
        Ok(Self {
            beginning_owner: self.beginning_owner.clone(),
            phase_adjusted_owner: self.phase_adjusted_owner.clone(),
            ending_owner,
            inner: self.inner.clone(),
            parent_working_state,
        })
    }

    pub(crate) fn validate(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
        resource: &DirectSurfaceLiquidResourceCandidateV2,
        input: &DirectSurfaceLiquidIngressInput,
    ) -> Result<(), DirectSurfaceLiquidError> {
        resource.validate(configuration)?;
        self.validate_after_resource(configuration, resource, input)
    }

    fn validate_with_validated_resource(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
        resource: &ValidatedSurfaceLiquidResourceCandidateRefV2<'_>,
        input: &DirectSurfaceLiquidIngressInput,
    ) -> Result<(), DirectSurfaceLiquidError> {
        resource.candidate.validate_revision_proof(configuration)?;
        self.validate_after_resource(configuration, resource.candidate, input)
    }

    fn validate_after_resource(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
        resource: &DirectSurfaceLiquidResourceCandidateV2,
        input: &DirectSurfaceLiquidIngressInput,
    ) -> Result<(), DirectSurfaceLiquidError> {
        if &self.beginning_owner != resource.beginning_owner()
            || &self.phase_adjusted_owner != resource.phase_adjusted_owner()
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 ingress resource identity mismatch",
            ));
        }
        if self.inner.parent_working_state().is_none() {
            self.inner.validate_with_phase_capacity_spills(
                configuration.parent(),
                resource.liquid_arithmetic(),
                input,
                resource.phase_capacity_spills(),
            )?;
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
    let validated_resource =
        consume_validated_surface_liquid_resource_candidate_v2(configuration, resource)?;
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
    let inner = execute_surface_liquid_ingress_with_parent_state_and_coupled_binding_and_phase_capacity_spills(
        configuration.parent(),
        resource.liquid_arithmetic(),
        input,
        adjusted_parent.as_ref(),
        finalize_parent_interval,
        coupled_binding,
        resource.phase_capacity_spills(),
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
    candidate.validate_with_validated_resource(configuration, &validated_resource, input)?;
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
    fn validated_revision_identity(
        &self,
        configuration_sha256: &str,
    ) -> ValidatedSurfaceLiquidResourceCandidateV2 {
        ValidatedSurfaceLiquidResourceCandidateV2 {
            configuration_sha256: configuration_sha256.into(),
            transaction_id: self.transaction_id,
            beginning_owner_sha256: self.beginning_owner.envelope_sha256().into(),
            native_phase_adjusted_owner_sha256: self
                .native_phase_adjusted_owner
                .envelope_sha256()
                .into(),
            phase_adjusted_owner_sha256: self.phase_adjusted_owner.envelope_sha256().into(),
            arithmetic_beginning_sha256: self
                .liquid_arithmetic
                .beginning_state()
                .state_sha256
                .clone(),
            arithmetic_working_sha256: self.liquid_arithmetic.working_state().state_sha256.clone(),
            expected_predecessor: self.liquid_arithmetic.expected_predecessor,
        }
    }

    fn validate_revision_proof(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<(), DirectSurfaceLiquidError> {
        let proof = self
            .validated_revision
            .as_ref()
            .ok_or(DirectSurfaceLiquidError::Identity(
                "surface-resource validation proof is absent",
            ))?;
        if proof.configuration_sha256 != configuration.configuration_sha256()
            || proof != &self.validated_revision_identity(configuration.configuration_sha256())
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-resource validation proof revision mismatch",
            ));
        }
        Ok(())
    }

    fn fully_validate_and_mint(
        &mut self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<(), DirectSurfaceLiquidError> {
        self.validated_revision = None;
        self.validate_full(configuration)?;
        self.validated_revision =
            Some(self.validated_revision_identity(configuration.configuration_sha256()));
        Ok(())
    }

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

    #[must_use]
    pub(crate) const fn heterogeneous_resource_join(
        &self,
    ) -> Option<&SurfaceLiquidV2HeterogeneousResourceJoinV1> {
        self.heterogeneous_resource_join.as_ref()
    }

    pub(crate) const fn joined_liquid_arithmetic(&self) -> &DirectSurfaceLiquidResourceCandidate {
        &self.liquid_arithmetic
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

    #[must_use]
    pub(crate) fn phase_capacity_spills(&self) -> &[LitterPhaseCapacitySpillV1] {
        &self.phase_capacity_spills
    }

    pub(crate) fn validate(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<(), DirectSurfaceLiquidError> {
        self.validate_full(configuration)
    }

    fn validate_full(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
    ) -> Result<(), DirectSurfaceLiquidError> {
        #[cfg(test)]
        {
            SURFACE_RESOURCE_FULL_VALIDATIONS.fetch_add(1, Ordering::Relaxed);
            SURFACE_RESOURCE_OWNER_SERIALIZATIONS.fetch_add(3, Ordering::Relaxed);
        }
        if self.transaction_id.0 == 0
            || self.liquid_arithmetic.transaction_id() != self.transaction_id
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "surface-owner V2 resource transaction mismatch",
            ));
        }
        self.beginning_owner
            .canonical_bytes(configuration.parent(), Some(configuration))?;
        self.native_phase_adjusted_owner
            .canonical_bytes(configuration.parent(), Some(configuration))?;
        self.phase_adjusted_owner
            .canonical_bytes(configuration.parent(), Some(configuration))?;
        let beginning =
            self.beginning_owner
                .v2_state()
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "surface-owner V2 resource beginning is not V2",
                ))?;
        let native_phase_adjusted = self.native_phase_adjusted_owner.v2_state().ok_or(
            DirectSurfaceLiquidError::Identity(
                "surface-owner V2 resource native phase owner is not V2",
            ),
        )?;
        let phase_adjusted = self.phase_adjusted_state()?;
        if beginning.continuations() != native_phase_adjusted.continuations()
            || beginning
                .records()
                .iter()
                .zip(native_phase_adjusted.records())
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
            native_phase_adjusted,
            &self.phase_closure,
        )?;
        let mut spill_keys = std::collections::BTreeSet::new();
        for spill in &self.phase_capacity_spills {
            let configured = configuration
                .parent()
                .records
                .iter()
                .find(|record| {
                    record.key.ofe_id == spill.ofe_id && record.key.tile_id == spill.tile_id
                })
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "phase-capacity spill surface key",
                ))?;
            let retained = native_phase_adjusted
                .records()
                .iter()
                .find(|record| record.key == configured.key)
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "phase-capacity spill retained owner key",
                ))?;
            if !spill_keys.insert(configured.key.clone())
                || spill.transaction_id != self.transaction_id
                || spill.surface_owner_id != configuration.parent().owner_id
                || spill.liquid_capacity_kg_m2_tile.to_bits()
                    != configured.capacity_kg_m2_tile.to_bits()
                || spill.retained_ending.liquid_kg_m2_tile.to_bits()
                    != retained.liquid_kg_m2_tile.to_bits()
                || spill.retained_ending.ice_kg_m2_tile.to_bits()
                    != retained.litter_ice_kg_m2_tile.to_bits()
                || spill.retained_ending.sensible_energy_j_m2_tile.to_bits()
                    != retained.surface_enthalpy_j_m2_tile.to_bits()
            {
                return Err(DirectSurfaceLiquidError::Identity(
                    "phase-capacity spill transaction/configuration/retained-owner join",
                ));
            }
        }
        match &self.heterogeneous_resource_join {
            Some(join) => join.validate(
                configuration,
                &self.native_phase_adjusted_owner,
                &self.phase_adjusted_owner,
            )?,
            None if self.native_phase_adjusted_owner != self.phase_adjusted_owner => {
                return Err(DirectSurfaceLiquidError::Identity(
                    "surface-owner V2 ordinary debit has no heterogeneous join",
                ));
            }
            None => {}
        }
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

fn consume_validated_surface_liquid_resource_candidate_v2<'a>(
    configuration: &SurfaceLiquidConfigurationV2,
    resource: &'a DirectSurfaceLiquidResourceCandidateV2,
) -> Result<ValidatedSurfaceLiquidResourceCandidateRefV2<'a>, DirectSurfaceLiquidError> {
    resource.validate_revision_proof(configuration)?;
    Ok(ValidatedSurfaceLiquidResourceCandidateRefV2 {
        candidate: resource,
    })
}

#[cfg(test)]
pub(crate) fn reset_surface_resource_validation_counters_v2() {
    SURFACE_RESOURCE_FULL_VALIDATIONS.store(0, Ordering::Relaxed);
    SURFACE_RESOURCE_OWNER_SERIALIZATIONS.store(0, Ordering::Relaxed);
}

#[cfg(test)]
pub(crate) fn surface_resource_validation_counters_v2() -> (usize, usize) {
    (
        SURFACE_RESOURCE_FULL_VALIDATIONS.load(Ordering::Relaxed),
        SURFACE_RESOURCE_OWNER_SERIALIZATIONS.load(Ordering::Relaxed),
    )
}

#[cfg(test)]
pub(crate) fn reset_wb14_parent_v2_handoff_counters() {
    WB14_PARENT_V2_HANDOFF_VALIDATIONS.store(0, Ordering::Relaxed);
    WB14_PARENT_V2_RESTART_SERIALIZATIONS.store(0, Ordering::Relaxed);
    WB14_PARENT_V2_OWNER_CANONICALIZATIONS.store(0, Ordering::Relaxed);
}

#[cfg(test)]
pub(crate) fn wb14_parent_v2_handoff_counters() -> (usize, usize, usize) {
    (
        WB14_PARENT_V2_HANDOFF_VALIDATIONS.load(Ordering::Relaxed),
        WB14_PARENT_V2_RESTART_SERIALIZATIONS.load(Ordering::Relaxed),
        WB14_PARENT_V2_OWNER_CANONICALIZATIONS.load(Ordering::Relaxed),
    )
}

#[cfg(test)]
impl DirectSurfaceLiquidResourceCandidateV2 {
    pub(crate) fn transfer_validation_proof_from_for_test(&mut self, source: &Self) {
        self.validated_revision
            .clone_from(&source.validated_revision);
    }

    pub(crate) fn replace_phase_adjusted_owner_for_test(
        &mut self,
        owner: SurfaceLiquidOwnerEnvelopeV2,
    ) {
        self.phase_adjusted_owner = owner;
    }
}

impl SurfaceLiquidV2HeterogeneousResourceJoinV1 {
    pub(crate) fn validate_protocol_partition(
        &self,
        requests: &[WaterAmount],
        authorizations: &[WaterAuthorization],
        finalized_uses: &[WaterAmount],
        receipts: &[LitterPhaseReceipt],
    ) -> Result<(), DirectSurfaceLiquidError> {
        let receipt_rows = receipts
            .iter()
            .map(|receipt| receipt.receipt_sha256.clone())
            .collect::<std::collections::BTreeSet<_>>();
        let sealed_rows = self
            .native_phase_receipts
            .values()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        if receipt_rows != sealed_rows {
            return Err(DirectSurfaceLiquidError::Identity(
                "heterogeneous resource native receipt partition mismatch",
            ));
        }
        let ordinary_keys = self
            .ordinary_requests
            .iter()
            .map(|row| row.key.clone())
            .collect::<std::collections::BTreeSet<_>>();
        let filter = |key: &GroundWaterKey| {
            ordinary_keys.contains(key) || self.native_phase_receipts.contains_key(key)
        };
        let mut filtered_requests = requests
            .iter()
            .filter(|row| filter(&row.key))
            .cloned()
            .collect::<Vec<_>>();
        let mut filtered_authorizations = authorizations
            .iter()
            .filter(|row| filter(&row.key))
            .cloned()
            .collect::<Vec<_>>();
        let mut filtered_uses = finalized_uses
            .iter()
            .filter(|row| filter(&row.key))
            .cloned()
            .collect::<Vec<_>>();
        filtered_requests.sort_by(|left, right| left.key.cmp(&right.key));
        filtered_authorizations.sort_by(|left, right| left.key.cmp(&right.key));
        filtered_uses.sort_by(|left, right| left.key.cmp(&right.key));
        let mut expected_requests = self.ordinary_requests.clone();
        let mut expected_authorizations = self.ordinary_authorizations.clone();
        let mut expected_uses = self.ordinary_finalized_uses.clone();
        expected_requests.extend(
            requests
                .iter()
                .filter(|row| self.native_phase_receipts.contains_key(&row.key))
                .cloned(),
        );
        expected_authorizations.extend(
            authorizations
                .iter()
                .filter(|row| self.native_phase_receipts.contains_key(&row.key))
                .cloned(),
        );
        expected_uses.extend(
            finalized_uses
                .iter()
                .filter(|row| self.native_phase_receipts.contains_key(&row.key))
                .cloned(),
        );
        expected_requests.sort_by(|left, right| left.key.cmp(&right.key));
        expected_authorizations.sort_by(|left, right| left.key.cmp(&right.key));
        expected_uses.sort_by(|left, right| left.key.cmp(&right.key));
        if filtered_requests != expected_requests
            || filtered_authorizations != expected_authorizations
            || filtered_uses != expected_uses
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "heterogeneous resource protocol partition mismatch",
            ));
        }
        Ok(())
    }

    fn validate(
        &self,
        configuration: &SurfaceLiquidConfigurationV2,
        native_owner: &SurfaceLiquidOwnerEnvelopeV2,
        joined_owner: &SurfaceLiquidOwnerEnvelopeV2,
    ) -> Result<(), DirectSurfaceLiquidError> {
        if self.transaction_id.0 == 0
            || self.native_phase_adjusted_owner_sha256 != native_owner.envelope_sha256()
            || self.joined_phase_adjusted_owner_sha256 != joined_owner.envelope_sha256()
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "heterogeneous resource owner/transaction seal mismatch",
            ));
        }
        let reconstructed = apply_ordinary_debits(
            configuration,
            native_owner,
            self.transaction_id,
            &self.ordinary_requests,
            &self.ordinary_authorizations,
            &self.ordinary_finalized_uses,
        )?;
        if &reconstructed != joined_owner {
            return Err(DirectSurfaceLiquidError::Closure(
                "heterogeneous resource independent debit reconstruction mismatch",
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
    prepare_surface_liquid_resource_candidate_v2_with_phase_capacity_spills(
        configuration,
        beginning_owner,
        phase_adjusted_owner,
        transaction_id,
        phase_closure,
        &[],
    )
}

pub(crate) fn prepare_surface_liquid_resource_candidate_v2_with_phase_capacity_spills(
    configuration: &SurfaceLiquidConfigurationV2,
    beginning_owner: &SurfaceLiquidOwnerEnvelopeV2,
    phase_adjusted_owner: &SurfaceLiquidOwnerEnvelopeV2,
    transaction_id: TransactionId,
    phase_closure: &[SurfaceLiquidOwnerClosureRecordV2],
    phase_capacity_spills: &[LitterPhaseCapacitySpillV1],
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
    let mut candidate = DirectSurfaceLiquidResourceCandidateV2 {
        transaction_id,
        beginning_owner: beginning_owner.clone(),
        native_phase_adjusted_owner: phase_adjusted_owner.clone(),
        phase_adjusted_owner: phase_adjusted_owner.clone(),
        phase_closure: phase_closure.to_vec(),
        phase_capacity_spills: phase_capacity_spills.to_vec(),
        heterogeneous_resource_join: None,
        liquid_arithmetic,
        validated_revision: None,
    };
    candidate.fully_validate_and_mint(configuration)?;
    Ok(candidate)
}

/// Apply only the ordinary rows from a heterogeneous native/legacy fixed-final
/// batch before the transaction's single current-ingress execution. Native
/// litter-vapor rows are authenticated against their already-consumed phase
/// receipts and are never debited a second time here.
pub(crate) fn apply_ordinary_finalized_uses_to_phase_adjusted_v2(
    configuration: &SurfaceLiquidConfigurationV2,
    resource: &DirectSurfaceLiquidResourceCandidateV2,
    requests: &[WaterAmount],
    authorizations: &[WaterAuthorization],
    finalized_uses: &[WaterAmount],
    phase_receipts: &[LitterPhaseReceipt],
) -> Result<DirectSurfaceLiquidResourceCandidateV2, DirectSurfaceLiquidError> {
    consume_validated_surface_liquid_resource_candidate_v2(configuration, resource)?;
    if resource.heterogeneous_resource_join.is_some() {
        return Err(DirectSurfaceLiquidError::Identity(
            "heterogeneous resource join may be applied exactly once",
        ));
    }
    let transaction_id = resource.transaction_id;
    let request_by_key = canonical_water_amounts(requests, transaction_id, configuration)?;
    let authorization_by_key =
        canonical_water_authorizations(authorizations, transaction_id, configuration)?;
    let use_by_key = canonical_water_amounts(finalized_uses, transaction_id, configuration)?;
    if request_by_key.len() != authorization_by_key.len()
        || request_by_key.len() != use_by_key.len()
        || request_by_key.keys().ne(authorization_by_key.keys())
        || request_by_key.keys().ne(use_by_key.keys())
    {
        return Err(DirectSurfaceLiquidError::Identity(
            "heterogeneous resource D/A/F key set is incomplete",
        ));
    }

    let mut native_phase_receipts = BTreeMap::new();
    for receipt in phase_receipts {
        validate_litter_phase_receipt(receipt).map_err(|_| {
            DirectSurfaceLiquidError::Identity("heterogeneous resource litter receipt seal")
        })?;
        if receipt.identity.transaction_id != transaction_id
            || receipt.identity.surface_owner_id != configuration.parent().owner_id
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "heterogeneous resource litter receipt owner/transaction",
            ));
        }
        let (key, request) = request_by_key
            .iter()
            .find(|(key, _)| {
                key.ofe_id == receipt.identity.ofe_id
                    && key.requesting_tile_id == receipt.identity.tile_id
                    && key.source_type == WaterSourceType::LitterLiquid
            })
            .ok_or(DirectSurfaceLiquidError::Identity(
                "heterogeneous resource litter receipt request",
            ))?;
        let configured = configuration_record_for_water_key(configuration, key)?;
        let expected = V3PhaseSpecificVaporAuthorization {
            liquid_outbound_rate_kg_m2_s: receipt
                .vapor
                .finalized
                .liquid_signed_rate_kg_m2_s
                .max(0.0),
            ice_outbound_rate_kg_m2_s: receipt.vapor.finalized.ice_signed_rate_kg_m2_s.max(0.0),
        }
        .aggregate_outbound_kg_m2_stand_ground(
            configured.tile_fraction,
            f64::from_bits(receipt.identity.support_duration_seconds_bits),
        )
        .map_err(|_| {
            DirectSurfaceLiquidError::Closure(
                "heterogeneous resource litter receipt area conversion",
            )
        })?;
        let authorization =
            authorization_by_key
                .get(key)
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "heterogeneous resource native authorization",
                ))?;
        let finalized = use_by_key
            .get(key)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "heterogeneous resource native finalized use",
            ))?;
        if authorization.amount_kg_m2_stand_ground < expected
            || request.amount_kg_m2_stand_ground < authorization.amount_kg_m2_stand_ground
            || finalized.amount_kg_m2_stand_ground.to_bits() != expected.to_bits()
            || native_phase_receipts
                .insert(key.clone(), receipt.receipt_sha256.clone())
                .is_some()
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "heterogeneous resource native vapor replay/mismatch",
            ));
        }
    }

    let ordinary_requests = request_by_key
        .iter()
        .filter(|(key, _)| !native_phase_receipts.contains_key(*key))
        .map(|(_, row)| (*row).clone())
        .collect::<Vec<_>>();
    let ordinary_authorizations = authorization_by_key
        .iter()
        .filter(|(key, _)| !native_phase_receipts.contains_key(*key))
        .map(|(_, row)| (*row).clone())
        .collect::<Vec<_>>();
    let ordinary_finalized_uses = use_by_key
        .iter()
        .filter(|(key, _)| !native_phase_receipts.contains_key(*key))
        .map(|(_, row)| (*row).clone())
        .collect::<Vec<_>>();
    let joined_owner = apply_ordinary_debits(
        configuration,
        &resource.phase_adjusted_owner,
        transaction_id,
        &ordinary_requests,
        &ordinary_authorizations,
        &ordinary_finalized_uses,
    )?;
    let joined_state = joined_owner
        .v2_state()
        .ok_or(DirectSurfaceLiquidError::Identity(
            "heterogeneous resource joined owner is not V2",
        ))?;
    let arithmetic = extract_v2_liquid_arithmetic_state(configuration, joined_state)?;
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
    let mut candidate = DirectSurfaceLiquidResourceCandidateV2 {
        transaction_id,
        beginning_owner: resource.beginning_owner.clone(),
        native_phase_adjusted_owner: resource.native_phase_adjusted_owner.clone(),
        phase_adjusted_owner: joined_owner.clone(),
        phase_closure: resource.phase_closure.clone(),
        phase_capacity_spills: resource.phase_capacity_spills.clone(),
        heterogeneous_resource_join: Some(SurfaceLiquidV2HeterogeneousResourceJoinV1 {
            transaction_id,
            native_phase_receipts,
            ordinary_requests,
            ordinary_authorizations,
            ordinary_finalized_uses,
            native_phase_adjusted_owner_sha256: resource
                .phase_adjusted_owner
                .envelope_sha256()
                .into(),
            joined_phase_adjusted_owner_sha256: joined_owner.envelope_sha256().into(),
        }),
        liquid_arithmetic,
        validated_revision: None,
    };
    candidate.fully_validate_and_mint(configuration)?;
    Ok(candidate)
}

fn canonical_water_amounts<'a>(
    rows: &'a [WaterAmount],
    transaction_id: TransactionId,
    configuration: &SurfaceLiquidConfigurationV2,
) -> Result<BTreeMap<GroundWaterKey, &'a WaterAmount>, DirectSurfaceLiquidError> {
    let mut result = BTreeMap::new();
    for row in rows {
        row.key
            .validate(transaction_id)
            .map_err(|_| DirectSurfaceLiquidError::Identity("heterogeneous resource water key"))?;
        configuration_record_for_water_key(configuration, &row.key)?;
        if !row.amount_kg_m2_stand_ground.is_finite()
            || row.amount_kg_m2_stand_ground < 0.0
            || result.insert(row.key.clone(), row).is_some()
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "heterogeneous resource duplicate/nonfinite amount",
            ));
        }
    }
    Ok(result)
}

fn canonical_water_authorizations<'a>(
    rows: &'a [WaterAuthorization],
    transaction_id: TransactionId,
    configuration: &SurfaceLiquidConfigurationV2,
) -> Result<BTreeMap<GroundWaterKey, &'a WaterAuthorization>, DirectSurfaceLiquidError> {
    let mut result = BTreeMap::new();
    for row in rows {
        row.key.validate(transaction_id).map_err(|_| {
            DirectSurfaceLiquidError::Identity("heterogeneous resource authorization key")
        })?;
        configuration_record_for_water_key(configuration, &row.key)?;
        if !row.amount_kg_m2_stand_ground.is_finite()
            || row.amount_kg_m2_stand_ground < 0.0
            || result.insert(row.key.clone(), row).is_some()
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "heterogeneous resource duplicate/nonfinite authorization",
            ));
        }
    }
    Ok(result)
}

fn configuration_record_for_water_key<'a>(
    configuration: &'a SurfaceLiquidConfigurationV2,
    key: &GroundWaterKey,
) -> Result<&'a crate::DirectSurfaceLiquidConfigurationRecord, DirectSurfaceLiquidError> {
    if key.source_tile_id.as_ref() != Some(&key.requesting_tile_id)
        || !matches!(
            key.source_type,
            WaterSourceType::SurfaceLiquid | WaterSourceType::LitterLiquid
        )
    {
        return Err(DirectSurfaceLiquidError::Identity(
            "heterogeneous resource non-surface row",
        ));
    }
    configuration
        .parent()
        .records
        .iter()
        .find(|record| {
            record.key.ofe_id == key.ofe_id
                && record.key.tile_id == key.requesting_tile_id
                && record.key.surface_id
                    == *key.surface_id.as_ref().unwrap_or(&record.key.surface_id)
                && record.key.surface_class == key.surface_class.unwrap_or(record.key.surface_class)
                && record.key.source_type == key.source_type
                && record.key.source_id == key.source_id
        })
        .ok_or(DirectSurfaceLiquidError::Identity(
            "heterogeneous resource surface configuration row",
        ))
}

fn apply_ordinary_debits(
    configuration: &SurfaceLiquidConfigurationV2,
    native_owner: &SurfaceLiquidOwnerEnvelopeV2,
    transaction_id: TransactionId,
    requests: &[WaterAmount],
    authorizations: &[WaterAuthorization],
    finalized_uses: &[WaterAmount],
) -> Result<SurfaceLiquidOwnerEnvelopeV2, DirectSurfaceLiquidError> {
    let request_by_key = canonical_water_amounts(requests, transaction_id, configuration)?;
    let authorization_by_key =
        canonical_water_authorizations(authorizations, transaction_id, configuration)?;
    let use_by_key = canonical_water_amounts(finalized_uses, transaction_id, configuration)?;
    if request_by_key.len() != authorization_by_key.len()
        || request_by_key.len() != use_by_key.len()
        || request_by_key.keys().ne(authorization_by_key.keys())
        || request_by_key.keys().ne(use_by_key.keys())
    {
        return Err(DirectSurfaceLiquidError::Identity(
            "heterogeneous ordinary D/A/F set mismatch",
        ));
    }
    let state = native_owner
        .v2_state()
        .ok_or(DirectSurfaceLiquidError::Identity(
            "heterogeneous ordinary debit native owner is not V2",
        ))?;
    let mut records = state.records().to_vec();
    for (key, finalized) in use_by_key {
        let request = request_by_key[&key];
        let authorization = authorization_by_key[&key];
        if authorization.amount_kg_m2_stand_ground > request.amount_kg_m2_stand_ground
            || finalized.amount_kg_m2_stand_ground > authorization.amount_kg_m2_stand_ground
        {
            return Err(DirectSurfaceLiquidError::Identity(
                "heterogeneous ordinary 0<=F<=A<=D relation",
            ));
        }
        let configured = configuration_record_for_water_key(configuration, &key)?;
        let record = records
            .iter_mut()
            .find(|record| record.key == configured.key)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "heterogeneous ordinary debit owner row",
            ))?;
        let debit_tile = checked_surface_liquid_div(
            finalized.amount_kg_m2_stand_ground,
            configured.tile_fraction,
        )
        .ok_or(DirectSurfaceLiquidError::Closure(
            "heterogeneous ordinary F/f_t conversion",
        ))?;
        let retained = checked_surface_liquid_sub(record.liquid_kg_m2_tile, debit_tile).ok_or(
            DirectSurfaceLiquidError::Closure("heterogeneous ordinary liquid debit"),
        )?;
        if retained < 0.0 {
            return Err(DirectSurfaceLiquidError::Bound(
                "heterogeneous ordinary liquid debit exceeds native owner",
            ));
        }
        record.liquid_kg_m2_tile = retained;
    }
    native_owner.try_replace_v2_state(configuration, records, state.continuations().to_vec())
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

fn stage_v2_owner_liquid_arithmetic(
    configuration: &SurfaceLiquidConfigurationV2,
    native_owner: &SurfaceLiquidOwnerEnvelopeV2,
    liquid: &DirectSurfaceLiquidOwnedState,
) -> Result<SurfaceLiquidOwnerEnvelopeV2, DirectSurfaceLiquidError> {
    let native = native_owner
        .v2_state()
        .ok_or(DirectSurfaceLiquidError::Identity(
            "staged WB14 parent native owner is not V2",
        ))?;
    liquid.validate(configuration.parent())?;
    if native.records().len() != liquid.records.len()
        || native.continuations().len() != liquid.continuations.len()
    {
        return Err(DirectSurfaceLiquidError::Identity(
            "staged WB14 parent liquid arithmetic cardinality",
        ));
    }
    let records = native
        .records()
        .iter()
        .zip(&liquid.records)
        .map(|(native, liquid)| {
            if native.key != liquid.key {
                return Err(DirectSurfaceLiquidError::Identity(
                    "staged WB14 parent liquid arithmetic key",
                ));
            }
            let mut staged = native.clone();
            staged.liquid_kg_m2_tile = liquid.liquid_kg_m2_tile;
            staged.last_accepted_transaction_id = liquid.last_accepted_transaction_id;
            Ok(staged)
        })
        .collect::<Result<Vec<_>, _>>()?;
    native_owner.try_replace_v2_state(configuration, records, liquid.continuations.clone())
}
