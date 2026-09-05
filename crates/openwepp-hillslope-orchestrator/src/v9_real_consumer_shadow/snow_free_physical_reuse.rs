//! Private, single-use snow-free physical reuse between provisional and final
//! coupled-slab receipts.

#![allow(clippy::result_large_err)]

use std::collections::BTreeMap;

use openwepp_coupled_time::{Digest32, digest_bytes};
use openwepp_vegetation::v11::{
    V11AcceptedSegmentCandidate, V11ImportedV10SegmentInput, V11ImportedV10SegmentOutput,
    V11OwnerEnvelope, V11ResourceDebit, V11SharedResourceKind, V11SharedResourceOwnerTransition,
};

use super::{
    DirectV9ShadowIntervalInput, DirectV10RealConsumerError, DirectV10RealConsumerShadow,
    DirectV11RealConsumerError, DirectV11RealConsumerStack, DirectV11SnowCoveredSegmentInput,
    ImportedStackProfileScopeV1,
};

#[cfg(any(test, feature = "persisted-restart-v1"))]
#[cfg_attr(test, allow(unreachable_pub))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SnowFreePhysicalReuseAuditV1 {
    pub physical_execution_count: u32,
    pub identity_reseal_count: u32,
    pub final_publication_append_count: u32,
    pub outer_accepted_publication_count: u32,
    pub provider_projection_count: u32,
    pub vapor_operation_count: u32,
    pub phase_operation_count: u32,
    pub ingress_operation_count: u32,
    pub wb14_operation_count: u32,
    pub routing_operation_count: u32,
}

#[cfg(any(test, feature = "persisted-restart-v1"))]
std::thread_local! {
    static SNOW_FREE_PHYSICAL_REUSE_AUDIT_V1: std::cell::RefCell<Option<SnowFreePhysicalReuseAuditV1>> = const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
std::thread_local! {
    static SNOW_FREE_OUTER_AUTH_FAILURE_V1: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static SNOW_FREE_POST_TAKE_FAILURE_V1: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
pub(super) fn inject_snow_free_outer_auth_failure_v1() {
    SNOW_FREE_OUTER_AUTH_FAILURE_V1.with(|fault| fault.set(true));
}

#[cfg(test)]
fn take_snow_free_outer_auth_failure_v1() -> bool {
    SNOW_FREE_OUTER_AUTH_FAILURE_V1.with(|fault| fault.replace(false))
}

#[cfg(test)]
pub(super) fn inject_snow_free_post_take_failure_v1() {
    SNOW_FREE_POST_TAKE_FAILURE_V1.with(|fault| fault.set(true));
}

#[cfg(test)]
fn take_snow_free_post_take_failure_v1() -> bool {
    SNOW_FREE_POST_TAKE_FAILURE_V1.with(|fault| fault.replace(false))
}

#[cfg(any(test, feature = "persisted-restart-v1"))]
pub(super) fn begin_snow_free_physical_reuse_audit_v1() {
    SNOW_FREE_PHYSICAL_REUSE_AUDIT_V1.with(|audit| {
        *audit.borrow_mut() = Some(SnowFreePhysicalReuseAuditV1::default());
    });
}

#[cfg(any(test, feature = "persisted-restart-v1"))]
pub(super) fn take_snow_free_physical_reuse_audit_v1() -> SnowFreePhysicalReuseAuditV1 {
    SNOW_FREE_PHYSICAL_REUSE_AUDIT_V1.with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

#[cfg(any(test, feature = "persisted-restart-v1"))]
pub(super) fn record_snow_free_physical_execution_v1() {
    SNOW_FREE_PHYSICAL_REUSE_AUDIT_V1.with(|audit| {
        if let Some(audit) = audit.borrow_mut().as_mut() {
            audit.physical_execution_count = audit.physical_execution_count.saturating_add(1);
        }
    });
}

#[cfg(test)]
fn record_snow_free_counter_v1(select: impl FnOnce(&mut SnowFreePhysicalReuseAuditV1)) {
    SNOW_FREE_PHYSICAL_REUSE_AUDIT_V1.with(|audit| {
        if let Some(audit) = audit.borrow_mut().as_mut() {
            select(audit);
        }
    });
}

#[cfg(test)]
pub(crate) fn record_snow_free_provider_projection_v1() {
    record_snow_free_counter_v1(|audit| {
        audit.provider_projection_count = audit.provider_projection_count.saturating_add(1);
    });
}

#[cfg(test)]
pub(crate) fn record_snow_free_vapor_operation_v1() {
    record_snow_free_counter_v1(|audit| {
        audit.vapor_operation_count = audit.vapor_operation_count.saturating_add(1);
    });
}

#[cfg(test)]
pub(crate) fn record_snow_free_phase_operation_v1() {
    record_snow_free_counter_v1(|audit| {
        audit.phase_operation_count = audit.phase_operation_count.saturating_add(1);
    });
}

#[cfg(test)]
pub(crate) fn record_snow_free_ingress_operation_v1() {
    record_snow_free_counter_v1(|audit| {
        audit.ingress_operation_count = audit.ingress_operation_count.saturating_add(1);
    });
}

#[cfg(test)]
pub(crate) fn record_snow_free_wb14_operations_v1(count: u32) {
    record_snow_free_counter_v1(|audit| {
        audit.wb14_operation_count = audit.wb14_operation_count.saturating_add(count);
    });
}

#[cfg(test)]
pub(crate) fn record_snow_free_routing_operation_v1() {
    record_snow_free_counter_v1(|audit| {
        audit.routing_operation_count = audit.routing_operation_count.saturating_add(1);
    });
}

#[cfg(any(test, feature = "persisted-restart-v1"))]
fn record_snow_free_identity_reseal_v1() {
    SNOW_FREE_PHYSICAL_REUSE_AUDIT_V1.with(|audit| {
        if let Some(audit) = audit.borrow_mut().as_mut() {
            audit.identity_reseal_count = audit.identity_reseal_count.saturating_add(1);
        }
    });
}

#[cfg(any(test, feature = "persisted-restart-v1"))]
fn record_snow_free_final_publication_append_v1() {
    SNOW_FREE_PHYSICAL_REUSE_AUDIT_V1.with(|audit| {
        if let Some(audit) = audit.borrow_mut().as_mut() {
            audit.final_publication_append_count =
                audit.final_publication_append_count.saturating_add(1);
        }
    });
}

#[cfg(any(test, feature = "persisted-restart-v1"))]
pub(crate) fn record_snow_free_outer_accepted_publication_v1() {
    SNOW_FREE_PHYSICAL_REUSE_AUDIT_V1.with(|audit| {
        if let Some(audit) = audit.borrow_mut().as_mut() {
            audit.outer_accepted_publication_count =
                audit.outer_accepted_publication_count.saturating_add(1);
        }
    });
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::struct_field_names)]
struct SnowFreePhysicalReuseAuthorityV1 {
    physical_inputs_sha256: Digest32,
    beginning_sha256: Digest32,
}

fn canonical_digest<T: serde::Serialize>(
    value: &T,
) -> Result<Digest32, DirectV11RealConsumerError> {
    serde_json::to_vec(value)
        .map(|bytes| digest_bytes(&bytes))
        .map_err(|_| DirectV11RealConsumerError::Identity("snow-free physical reuse framing"))
}

fn physical_reuse_authority(
    input: &V11ImportedV10SegmentInput,
) -> Result<SnowFreePhysicalReuseAuthorityV1, DirectV11RealConsumerError> {
    Ok(SnowFreePhysicalReuseAuthorityV1 {
        physical_inputs_sha256: canonical_digest(&(
            input.parent_transaction_id,
            input.support,
            input.duration_s_bits,
            &input.configuration,
        ))?,
        beginning_sha256: canonical_digest(&(&input.beginning, &input.staged_resource_owners))?,
    })
}

fn validate_physical_ending_resource_owners_v1(
    physical_ending: &DirectV10RealConsumerShadow,
    output: &V11ImportedV10SegmentOutput,
    input: &V11ImportedV10SegmentInput,
    ending_snow_owner_bytes: Option<&[u8]>,
    day_index: usize,
) -> Result<(), DirectV11RealConsumerError> {
    let snow = ending_snow_owner_bytes
        .map(<[u8]>::to_vec)
        .or_else(|| {
            input
                .staged_resource_owners
                .get("snow")
                .map(|owner| owner.state_bytes.clone())
        })
        .ok_or(DirectV11RealConsumerError::Identity(
            "snow-free physical ending snow owner",
        ))?;
    let surface = if let Some(resident) = physical_ending.frozen_litter_v3.as_ref() {
        resident
            .surface_owner()
            .canonical_bytes(
                resident.surface_configuration().parent(),
                Some(resident.surface_configuration()),
            )
            .map_err(DirectV10RealConsumerError::SurfaceLiquidV2)?
    } else {
        physical_ending
            .inner
            .hydrology_frame
            .surface_liquid_shadow
            .as_ref()
            .ok_or(DirectV11RealConsumerError::Identity(
                "snow-free physical ending surface owner",
            ))?
            .canonical_bytes(&physical_ending.inner.surface_configuration)
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                    super::DirectV9RealConsumerError::Serialization(error.to_string()),
                ))
            })?
    };
    let hydrology = super::RealHydrologyShadowAdapter::try_from_day_start(
        &physical_ending.inner.hydrology_frame,
        day_index,
        openwepp_kernel_contract::TransactionId(input.beginning.0.last_transaction_id),
        f64::from_bits(input.duration_s_bits),
        physical_ending.inner.surface_configuration.owner_id.clone(),
        &physical_ending.inner.layer_maps,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
    })?;
    let expected = BTreeMap::from([
        ("snow", snow),
        (
            "land_surface_energy",
            physical_ending.canonical_v11_lse_owner_bytes()?,
        ),
        ("surface_liquid", surface),
        ("hydrology", hydrology.snapshot_bytes().to_vec()),
        (
            "bgc",
            serde_json::to_vec(&physical_ending.inner.biogeochemistry).map_err(|_| {
                DirectV11RealConsumerError::Identity("snow-free physical ending BGC owner")
            })?,
        ),
        (
            "soil_thermal",
            physical_ending
                .inner
                .soil_thermal
                .canonical_active_owner_bytes()
                .map_err(DirectV10RealConsumerError::Runtime)?,
        ),
    ]);
    if output.ending_resource_owners.len() != expected.len()
        || expected.iter().any(|(owner_id, bytes)| {
            output
                .ending_resource_owners
                .get(*owner_id)
                .is_none_or(|owner| owner.state_bytes != *bytes)
        })
    {
        return Err(DirectV11RealConsumerError::Identity(
            "snow-free physical ending resource-owner join",
        ));
    }
    Ok(())
}

/// Untrusted producer material retained only until the outer V11 consumer has
/// validated the complete candidate. This is deliberately not the reusable
/// capability.
pub(crate) struct SnowFreePhysicalReusePendingV1 {
    input: V11ImportedV10SegmentInput,
    provisional_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    provisional_publication_support: super::PreparedStage3AcceptedPublicationSupportV1,
    physical_ending: DirectV10RealConsumerShadow,
    provisional_output: V11ImportedV10SegmentOutput,
    beginning: DirectV10RealConsumerShadow,
    interval: DirectV9ShadowIntervalInput,
    day_index: usize,
    interval_index: usize,
    finalize_wb14_parent_interval: bool,
    native_inactive_wb14_prefix: Option<crate::direct_runtime::ValidatedNativeInactiveWb14PrefixV1>,
    ending_snow_owner_bytes: Option<Vec<u8>>,
    deferred_native_v2_soil_custody: Option<super::DeferredNativeV2SoilCustodyV1>,
    outer_auth_rollback_ending: Option<DirectV10RealConsumerShadow>,
    outer_auth_rollback_support: Option<super::LseSupportAdmissibilityReceiptV1>,
}

impl SnowFreePhysicalReusePendingV1 {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn stage(
        input: &V11ImportedV10SegmentInput,
        provisional_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
        provisional_publication_support: super::PreparedStage3AcceptedPublicationSupportV1,
        physical_ending: DirectV10RealConsumerShadow,
        provisional_output: V11ImportedV10SegmentOutput,
        beginning: &DirectV10RealConsumerShadow,
        interval: &DirectV9ShadowIntervalInput,
        day_index: usize,
        interval_index: usize,
        finalize_wb14_parent_interval: bool,
        native_inactive_wb14_prefix: Option<
            crate::direct_runtime::ValidatedNativeInactiveWb14PrefixV1,
        >,
        ending_snow_owner_bytes: Option<Vec<u8>>,
        deferred_native_v2_soil_custody: Option<super::DeferredNativeV2SoilCustodyV1>,
        outer_auth_rollback_ending: Option<DirectV10RealConsumerShadow>,
        outer_auth_rollback_support: Option<super::LseSupportAdmissibilityReceiptV1>,
    ) -> Self {
        Self {
            input: input.clone(),
            provisional_binding,
            provisional_publication_support,
            physical_ending,
            provisional_output,
            beginning: beginning.clone(),
            interval: interval.clone(),
            day_index,
            interval_index,
            finalize_wb14_parent_interval,
            native_inactive_wb14_prefix,
            ending_snow_owner_bytes,
            deferred_native_v2_soil_custody,
            outer_auth_rollback_ending,
            outer_auth_rollback_support,
        }
    }
}

/// Private non-wire proof. It is intentionally neither `Clone` nor
/// serializable; only consuming the owning stack can transfer it to the final
/// receipt pass.
pub(crate) struct SnowFreePhysicalReuseCapabilityV1 {
    authority: SnowFreePhysicalReuseAuthorityV1,
    live_beginning_revision_receipt: openwepp_coupled_time::AcceptedSlabReceiptV1,
    provisional_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    final_binding: Option<crate::direct_runtime::DirectWb14CoupledChildBindingV1>,
    provisional_publication_support: super::PreparedStage3AcceptedPublicationSupportV1,
    physical_ending: DirectV10RealConsumerShadow,
    provisional_output: V11ImportedV10SegmentOutput,
    beginning: DirectV10RealConsumerShadow,
    interval: DirectV9ShadowIntervalInput,
    day_index: usize,
    interval_index: usize,
    finalize_wb14_parent_interval: bool,
    native_inactive_wb14_prefix: Option<crate::direct_runtime::ValidatedNativeInactiveWb14PrefixV1>,
    ending_snow_owner_bytes: Option<Vec<u8>>,
    deferred_native_v2_soil_custody: Option<super::DeferredNativeV2SoilCustodyV1>,
}

/// Private non-wire lifecycle. Only `Authorized`/`Armed` contain the move-only
/// capability; all other variants are refusal tombstones and cannot fall back
/// to broad physical execution.
pub(crate) enum SnowFreePhysicalReuseSeedV1 {
    Authorized(Box<SnowFreePhysicalReuseCapabilityV1>),
    Armed(Box<SnowFreePhysicalReuseCapabilityV1>),
    AwaitingFinalOuterValidation(Box<SnowFreeFinalOuterValidationV1>),
    ReadyToCommit(Box<SnowFreeFinalOuterValidationV1>),
    Consumed,
    Refused,
}

pub(crate) struct SnowFreeFinalOuterValidationV1 {
    output: V11ImportedV10SegmentOutput,
    accepted_slab_receipt: openwepp_coupled_time::AcceptedSlabReceiptV1,
    rollback_ending: Option<DirectV10RealConsumerShadow>,
    rollback_support: Option<super::LseSupportAdmissibilityReceiptV1>,
    publication_capability: super::ValidatedStage3AcceptedPublicationSupportV1,
}

impl SnowFreePhysicalReuseCapabilityV1 {
    fn mint_after_outer_validation(
        pending: SnowFreePhysicalReusePendingV1,
        candidate: &V11AcceptedSegmentCandidate,
        staged_ending: Option<&DirectV10RealConsumerShadow>,
    ) -> Result<Self, DirectV11RealConsumerError> {
        let input = &pending.input;
        let authority = physical_reuse_authority(input)?;
        let exact_owner_manifest = [
            "snow",
            "land_surface_energy",
            "surface_liquid",
            "hydrology",
            "bgc",
            "soil_thermal",
        ];
        if pending.provisional_binding.accepted_slab_sha256
            != *input.accepted_slab_receipt.slab_id().digest().as_bytes()
            || pending.provisional_binding.child_support_start_ns != input.support.start_ns().get()
            || pending.provisional_binding.child_support_end_ns != input.support.end_ns().get()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "snow-free physical reuse provisional slab authority",
            ));
        }
        if pending.provisional_output.ending_resource_owners.len() != 6
            || !exact_owner_manifest.iter().all(|owner_id| {
                pending
                    .provisional_output
                    .ending_resource_owners
                    .contains_key(*owner_id)
            })
        {
            return Err(DirectV11RealConsumerError::Identity(
                "snow-free physical reuse provisional owner manifest",
            ));
        }
        if !pending
            .provisional_output
            .admitted_resource_fluxes
            .is_empty()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "snow-free physical reuse provisional admitted flux",
            ));
        }
        validate_outer_candidate_matches_output(
            candidate,
            &pending.provisional_output,
            &input.accepted_slab_receipt,
        )?;
        if staged_ending != Some(&pending.physical_ending) {
            return Err(DirectV11RealConsumerError::Identity(
                "snow-free physical reuse staged ending authority",
            ));
        }
        pending
            .physical_ending
            .provider_cursor
            .validate_for_configuration(
                &pending.physical_ending.provider_static_configuration,
                pending.physical_ending.inner.next_day_index,
            )
            .map_err(DirectV10RealConsumerError::ForcingProvider)?;
        pending
            .physical_ending
            .vegetation_state
            .validate(&pending.physical_ending.vegetation_configuration)
            .map_err(DirectV10RealConsumerError::V10)?;
        pending
            .physical_ending
            .lse_state
            .validate(&pending.physical_ending.lse_configuration)
            .map_err(DirectV10RealConsumerError::LseV2)?;
        // This is the pre-outer V11 staging image. Native V2 soil retains the
        // accepted physical child transaction until the outer V11 owner join,
        // so the V9 same-transaction validator is not the applicable
        // contract here. Reconstruct and compare all six resource-owner bytes
        // from the real physical ending instead; the caller has already
        // validated the outer vegetation owner and complete candidate.
        validate_physical_ending_resource_owners_v1(
            &pending.physical_ending,
            &pending.provisional_output,
            input,
            pending.ending_snow_owner_bytes.as_deref(),
            pending.day_index,
        )?;
        Ok(Self {
            authority,
            live_beginning_revision_receipt: input.accepted_slab_receipt.clone(),
            provisional_binding: pending.provisional_binding,
            provisional_publication_support: pending.provisional_publication_support,
            // Authorization is installed only by the later slab-only arm.
            final_binding: None,
            physical_ending: pending.physical_ending,
            provisional_output: pending.provisional_output,
            beginning: pending.beginning,
            interval: pending.interval,
            day_index: pending.day_index,
            interval_index: pending.interval_index,
            finalize_wb14_parent_interval: pending.finalize_wb14_parent_interval,
            native_inactive_wb14_prefix: pending.native_inactive_wb14_prefix,
            ending_snow_owner_bytes: pending.ending_snow_owner_bytes,
            deferred_native_v2_soil_custody: pending.deferred_native_v2_soil_custody,
        })
    }
}

fn validate_outer_candidate_matches_output(
    candidate: &V11AcceptedSegmentCandidate,
    output: &V11ImportedV10SegmentOutput,
    accepted_slab_receipt: &openwepp_coupled_time::AcceptedSlabReceiptV1,
) -> Result<(), DirectV11RealConsumerError> {
    let mut nonvegetation_owners = candidate.ending_resource_owners.clone();
    nonvegetation_owners
        .remove("vegetation")
        .ok_or(DirectV11RealConsumerError::Identity(
            "snow-free outer vegetation owner",
        ))?;
    let mut expected_outer_physical = output.ending.0.clone();
    expected_outer_physical
        .model_definition_sha256
        .clone_from(&candidate.ending_state.model_definition_sha256);
    expected_outer_physical
        .configuration_sha256
        .clone_from(&candidate.ending_state.configuration_sha256);
    super::normalize_v8_parent_lineage(
        &mut expected_outer_physical,
        candidate.ending_state.last_parent_transaction_id,
    );
    expected_outer_physical.state_sha256 = expected_outer_physical.canonical_sha256();
    // The outer V11 layer has already validated its vegetation state and
    // owner. The remaining comparison joins every imported output operand and
    // all six non-vegetation owner envelopes byte-for-byte.
    if candidate.accepted_slab_receipt != *accepted_slab_receipt
        || candidate.ending_state.physical != expected_outer_physical
        || candidate.lse_support_receipt != output.lse_support_receipt
        || candidate.resource_debits != output.resource_debits
        || candidate.admitted_resource_fluxes != output.admitted_resource_fluxes
        || candidate.shared_resource_transitions != output.shared_resource_transitions
        || candidate.material_transfers != output.material_transfers
        || nonvegetation_owners != output.ending_resource_owners
    {
        return Err(DirectV11RealConsumerError::Identity(
            "snow-free outer validated provisional output",
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn reseal_snow_free_final_accepted_slab_identity_v1(
    beginning: &DirectV10RealConsumerShadow,
    input: &V11ImportedV10SegmentInput,
    physical_ending: DirectV10RealConsumerShadow,
    provisional_publication_support: super::PreparedStage3AcceptedPublicationSupportV1,
    provisional_output: V11ImportedV10SegmentOutput,
    provisional_slab_sha256: [u8; 32],
    final_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    day_index: usize,
    interval_index: usize,
    publication_interval: &DirectV11SnowCoveredSegmentInput,
) -> Result<
    (
        V11ImportedV10SegmentOutput,
        DirectV10RealConsumerShadow,
        super::LseSupportAdmissibilityReceiptV1,
        super::ValidatedStage3AcceptedPublicationSupportV1,
    ),
    DirectV11RealConsumerError,
> {
    // The provisional transaction already executed and validated every
    // physical owner. Its move-only unsealed support projection was
    // deliberately not installed in history, so the final slab boundary can
    // seal and validate the selected payload exactly once without cloning it.
    if physical_ending.accepted_publication_history != beginning.accepted_publication_history {
        return Err(DirectV11RealConsumerError::Identity(
            "snow-free physical reuse provisional publication history",
        ));
    }
    let provisional_publication_support = provisional_publication_support.into_support_for_reseal();

    let staged_lse_bytes = &input
        .staged_resource_owners
        .get("land_surface_energy")
        .ok_or(DirectV11RealConsumerError::Identity(
            "missing staged LSE owner",
        ))?
        .state_bytes;
    let (support_configuration, support_beginning) =
        super::v11_support_lse_beginning(beginning, staged_lse_bytes)?;
    let support_receipt = super::LseSupportAdmissibilityReceiptV1::admit(
        support_configuration,
        support_beginning,
        super::digest32_hex(input.parent_transaction_id.digest()),
        super::digest32_hex(input.accepted_slab_receipt.segment_id().digest()),
        super::digest32_hex(input.accepted_slab_receipt.slab_id().digest()),
        input.accepted_slab_receipt.slab_ordinal(),
        input.support.start_ns().get(),
        input.support.end_ns().get(),
        input.duration_s_bits,
        beginning.inner.soil_thermal.state_sha256().clone(),
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(error))
    })?;

    // These constructors reseal receipt identity around the already-finalized
    // D/A/F rows. They preserve every amount and non-slab coordinate and do
    // not invoke resource arbitration or any owner physics.
    let mut rebound_debit_ids = BTreeMap::new();
    let resource_debits = provisional_output
        .resource_debits
        .iter()
        .map(|provisional| {
            if provisional.parent_transaction_id != input.parent_transaction_id
                || provisional.segment_id != input.accepted_slab_receipt.segment_id()
                || provisional.support != input.support
                || provisional.accepted_slab_id.digest().as_bytes() != &provisional_slab_sha256
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "snow-free physical reuse resource debit identity",
                ));
            }
            let rebound = V11ResourceDebit::new(V11ResourceDebit {
                receipt_id: Digest32::zero(),
                parent_transaction_id: input.parent_transaction_id,
                segment_id: input.accepted_slab_receipt.segment_id(),
                accepted_slab_id: input.accepted_slab_receipt.slab_id(),
                support: input.support,
                owner_id: provisional.owner_id.clone(),
                resource_key: provisional.resource_key.clone(),
                ofe_id: provisional.ofe_id.clone(),
                tile_id: provisional.tile_id.clone(),
                occupancy_id: provisional.occupancy_id.clone(),
                layer_id: provisional.layer_id.clone(),
                source_id: provisional.source_id.clone(),
                amount_basis: provisional.amount_basis.clone(),
                request: provisional.request,
                authorization: provisional.authorization,
                final_use: provisional.final_use,
            })?;
            if rebound_debit_ids
                .insert(provisional.receipt_id, rebound.receipt_id)
                .is_some()
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "snow-free physical reuse duplicate debit identity",
                ));
            }
            Ok(rebound)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let ending_resource_owners = provisional_output.ending_resource_owners.clone();
    if ending_resource_owners.len() != 6
        || ending_resource_owners
            .values()
            .any(|owner| owner.to_owner_state().is_err())
    {
        return Err(DirectV11RealConsumerError::Identity(
            "snow-free physical reuse ending owners",
        ));
    }

    let shared_resource_transitions = provisional_output
        .shared_resource_transitions
        .iter()
        .map(|provisional| {
            if provisional.parent_transaction_id != input.parent_transaction_id
                || provisional.segment_id != input.accepted_slab_receipt.segment_id()
                || provisional.support != input.support
                || provisional.accepted_slab_id.digest().as_bytes() != &provisional_slab_sha256
                || !provisional.admitted_flux_receipt_ids.is_empty()
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "snow-free physical reuse resource transition identity",
                ));
            }
            let mut debit_receipt_ids = provisional
                .debit_receipt_ids
                .iter()
                .map(|receipt| {
                    rebound_debit_ids.get(receipt).copied().ok_or(
                        DirectV11RealConsumerError::Identity(
                            "snow-free physical reuse linked debit identity",
                        ),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            if provisional.shared_resource_key.owner_id != "bgc"
                || !matches!(
                    provisional.shared_resource_key.resource,
                    V11SharedResourceKind::Ammonium | V11SharedResourceKind::Nitrate
                )
            {
                debit_receipt_ids.sort();
            }
            Ok(V11SharedResourceOwnerTransition::new(
                V11SharedResourceOwnerTransition {
                    transition_id: Digest32::zero(),
                    parent_transaction_id: input.parent_transaction_id,
                    segment_id: input.accepted_slab_receipt.segment_id(),
                    accepted_slab_id: input.accepted_slab_receipt.slab_id(),
                    support: input.support,
                    shared_resource_key: provisional.shared_resource_key.clone(),
                    beginning_amount: provisional.beginning_amount,
                    ending_amount: provisional.ending_amount,
                    debit_receipt_ids,
                    admitted_flux_receipt_ids: Vec::new(),
                    owner_candidate_sha256: provisional.owner_candidate_sha256,
                },
            )?)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut complete_owners = ending_resource_owners.clone();
    complete_owners.insert(
        "vegetation".to_owned(),
        super::accepted_v11_vegetation_owner(input, &provisional_output.ending)?,
    );
    let complete_owner_states = complete_owners
        .values()
        .map(V11OwnerEnvelope::to_owner_state)
        .collect::<Result<Vec<_>, _>>()?;
    if !input
        .accepted_slab_receipt
        .authenticates_complete_ending_owners(&complete_owner_states)?
    {
        return Err(DirectV11RealConsumerError::Identity(
            "snow-free physical reuse final receipt ending owners",
        ));
    }
    let ending_complete_owner_set_sha256 =
        openwepp_coupled_time::complete_owner_set_digest(&complete_owner_states)?;
    let final_publication_support = reseal_snow_free_publication_support_v1(
        provisional_publication_support,
        input,
        provisional_slab_sha256,
        final_binding,
        day_index,
        interval_index,
        ending_complete_owner_set_sha256,
        support_receipt.clone(),
        &publication_interval.lse_forcing,
        &publication_interval.vegetation_forcing,
        &publication_interval.wb14_parameters,
        &resource_debits,
        &provisional_output.material_transfers,
        physical_ending
            .accepted_publication_history
            .live_revision_v1(),
    )?;
    let output = V11ImportedV10SegmentOutput {
        ending: provisional_output.ending,
        lse_support_receipt:
            openwepp_vegetation::v11::V11LseSupportReceiptEnvelope::from_canonical_json(
                serde_json::to_vec(&support_receipt).map_err(|error| {
                    DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                        super::DirectV9RealConsumerError::Serialization(error.to_string()),
                    ))
                })?,
            )
            .map_err(|_| DirectV11RealConsumerError::Identity("V11 LSE support receipt"))?,
        resource_debits,
        admitted_resource_fluxes: Vec::new(),
        shared_resource_transitions,
        ending_resource_owners,
        material_transfers: provisional_output.material_transfers,
    };
    Ok((
        output,
        physical_ending,
        support_receipt,
        final_publication_support,
    ))
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn reseal_snow_free_publication_support_v1(
    mut support: super::Stage3AcceptedPublicationSupportV1,
    input: &V11ImportedV10SegmentInput,
    provisional_slab_sha256: [u8; 32],
    final_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    day_index: usize,
    interval_index: usize,
    ending_complete_owner_set_sha256: Digest32,
    lse_support_receipt: super::LseSupportAdmissibilityReceiptV1,
    lse_forcing: &openwepp_land_surface_energy::LandSurfaceForcing,
    vegetation_forcing: &openwepp_vegetation::SnowFreeForcing,
    wb14_parameters: &[crate::DirectOfeWb14Parameters],
    resource_debits: &[V11ResourceDebit],
    material_transfers: &[openwepp_vegetation::carbon_nitrogen::MaterialTransfer],
    target_revision: super::AcceptedPublicationHistoryLiveRevisionV1,
) -> Result<super::ValidatedStage3AcceptedPublicationSupportV1, DirectV11RealConsumerError> {
    #[cfg(test)]
    super::accepted_publication_support_capability::record_full_validation_attempt_v1();
    if support.day_index != day_index
        || support.interval_index != interval_index
        || support.parent_transaction_id != input.parent_transaction_id
        || support.support != input.support
        || support.accepted_slab_sha256.as_bytes() != &provisional_slab_sha256
        || support.ending_complete_owner_set_sha256 != ending_complete_owner_set_sha256
        || support.lse_forcing != *lse_forcing
        || support.vegetation_forcing != *vegetation_forcing
        || support.wb14_parameters != wb14_parameters
        || support.material_transfers != material_transfers
    {
        return Err(DirectV11RealConsumerError::Identity(
            "snow-free physical reuse provisional publication operands",
        ));
    }
    let provisional_child_replay = support.wb14_child_replay.materialize();
    let (child_replay, parent_replay) = crate::direct_runtime::rebind_wb14_replay_to_accepted_slab(
        &provisional_child_replay,
        support.wb14_parent_replay_bytes.is_some(),
        final_binding,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::SurfaceLiquidV2(error))
    })?;
    support.accepted_slab_sha256 = input.accepted_slab_receipt.slab_id().digest();
    support.lse_support_receipt = lse_support_receipt;
    support.resource_debits = resource_debits.to_vec();
    support.wb14_child_receipt_set_sha256 = digest_bytes(&child_replay);
    support.wb14_child_replay = super::PersistentCanonicalWb14ReplayV1::from_bytes(child_replay);
    support.wb14_parent_replay_bytes = parent_replay;
    #[cfg(test)]
    super::accepted_publication_support_capability::record_operand_seal_v1();
    support.operands_sha256 = super::Stage3AcceptedPublicationSupportV1::operands_sha256(
        &support.lse_support_receipt,
        &support.lse_forcing,
        &support.vegetation_forcing,
        &support.wb14_parameters,
        &support.resource_debits,
        &support.material_transfers,
        support.run_identity,
        &support.beginning_lane_carries,
        &support.beginning_subsurface_layers_by_lane,
        &support.ending_subsurface_layers_by_lane,
        &support.surface_beginning_state,
        &support.surface_ending_state,
        &support.open_ingress_parcels,
        &support.ingress_receipts,
        &support.ingress_ledgers,
        &support.accepted_snow_liquid_outputs,
        support.wb14_child_replay.canonical_sha256(),
        support
            .wb14_parent_replay_bytes
            .as_deref()
            .map_or_else(|| digest_bytes(b"no-parent-replay"), digest_bytes),
        &support.finalized_water_uses,
        &support.condensation_credits,
        support.receiver_operands_sha256,
        &support.rollback_hashes,
    )?;
    #[cfg(test)]
    super::accepted_publication_support_capability::record_receipt_seal_v1();
    support.receipt_sha256 =
        super::Stage3AcceptedPublicationSupportV1::reconstructed_receipt_sha256(
            support.day_index,
            support.interval_index,
            support.parent_transaction_id,
            support.support,
            support.accepted_slab_sha256,
            support.beginning_complete_owner_set_sha256,
            support.ending_complete_owner_set_sha256,
            support.hydrology_transaction_id,
            support.wb14_child_receipt_set_sha256,
            support.operands_sha256,
        )?;
    support.validate_semantics()?;
    #[cfg(test)]
    super::accepted_publication_support_capability::record_full_validation_success_v1();
    Ok(super::ValidatedStage3AcceptedPublicationSupportV1::mint(
        support,
        target_revision,
    ))
}

fn binding_differs_only_by_slab(
    provisional: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    final_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
) -> bool {
    provisional.accepted_slab_sha256 != final_binding.accepted_slab_sha256
        && provisional.proposed_upper_bound_s_bits == final_binding.proposed_upper_bound_s_bits
        && provisional.coupled_parent_transaction_sha256
            == final_binding.coupled_parent_transaction_sha256
        && provisional.parent_beginning_complete_owner_set_sha256
            == final_binding.parent_beginning_complete_owner_set_sha256
        && provisional.parent_support_start_ns == final_binding.parent_support_start_ns
        && provisional.parent_support_end_ns == final_binding.parent_support_end_ns
        && provisional.child_support_start_ns == final_binding.child_support_start_ns
        && provisional.child_support_end_ns == final_binding.child_support_end_ns
}

/// Arm one provisional physical proof for exactly one final accepted slab.
pub(crate) fn prepare_snow_free_physical_reuse(
    mut stack: DirectV11RealConsumerStack<'_>,
    final_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
) -> Result<DirectV11RealConsumerStack<'_>, DirectV11RealConsumerError> {
    let state =
        stack
            .snow_free_physical_reuse_seed
            .take()
            .ok_or(DirectV11RealConsumerError::Identity(
                "snow-free physical reuse seed",
            ))?;
    let SnowFreePhysicalReuseSeedV1::Authorized(mut seed) = state else {
        stack.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Refused);
        return Err(DirectV11RealConsumerError::Identity(
            "snow-free physical reuse final authorization",
        ));
    };
    if seed.final_binding.is_some()
        || !binding_differs_only_by_slab(seed.provisional_binding, final_binding)
    {
        stack.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Refused);
        return Err(DirectV11RealConsumerError::Identity(
            "snow-free physical reuse final authorization",
        ));
    }
    seed.final_binding = Some(final_binding);
    stack.wb14_coupled_child_binding = Some(final_binding);
    stack.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Armed(seed));
    Ok(stack)
}

/// Persisted-restart evidence projection of the exact complete V11 parent
/// owner set held by a restored physical consumer. This exposes neither the
/// private reuse capability nor a new owner-byte construction path.
#[cfg(feature = "persisted-restart-v1")]
pub fn restart_authority_v11_parent_owner_envelopes_v1(
    beginning: &DirectV10RealConsumerShadow,
) -> Result<BTreeMap<String, V11OwnerEnvelope>, DirectV11RealConsumerError> {
    let migration = openwepp_vegetation::v11::migrate_v10_runtime_to_v11(
        beginning.vegetation_configuration(),
        beginning.vegetation_state(),
    )?;
    let mut owner_bytes = beginning.canonical_v11_parent_owner_state_bytes()?;
    owner_bytes.insert(
        "snow".to_owned(),
        super::v11_covered::canonical_stage3_snow_owner_bytes_v11(&BTreeMap::new())?,
    );
    let manifest = openwepp_vegetation::v11::V11_COMPLETE_OWNER_MANIFEST;
    if owner_bytes.len() != manifest.len()
        || owner_bytes
            .keys()
            .any(|owner| !manifest.contains(&owner.as_str()))
    {
        return Err(DirectV11RealConsumerError::Identity(
            "restart fresh execution complete owner manifest",
        ));
    }
    manifest
        .iter()
        .map(|owner_id| {
            let bytes =
                owner_bytes
                    .remove(*owner_id)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "restart fresh execution complete owner bytes",
                    ))?;
            let envelope = if *owner_id == "vegetation" {
                let _ = bytes;
                openwepp_vegetation::v11::v11_vegetation_owner_envelope(&migration.state)?
            } else {
                V11OwnerEnvelope::try_new((*owner_id).to_owned(), bytes)?
            };
            Ok(((*owner_id).to_owned(), envelope))
        })
        .collect()
}

/// Persisted-restart evidence boundary: a restored physical owner can only
/// construct a fresh stack and execute the full constitutive path. The private
/// reuse capability is neither an argument nor a return value.
#[cfg(feature = "persisted-restart-v1")]
#[allow(clippy::too_many_arguments)]
pub fn restart_authority_execute_fresh_snow_free_segment_v1(
    beginning: &DirectV10RealConsumerShadow,
    interval: &DirectV9ShadowIntervalInput,
    day_index: usize,
    interval_index: usize,
    finalize_wb14_parent_interval: bool,
    binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    configuration: &openwepp_vegetation::v11::VegetationConfigurationV11,
    parent: &openwepp_vegetation::v11::V11ParentTransaction,
    receipt: &openwepp_coupled_time::AcceptedSlabReceiptV1,
) -> Result<
    (
        V11AcceptedSegmentCandidate,
        DirectV10RealConsumerShadow,
        SnowFreePhysicalReuseAuditV1,
    ),
    openwepp_vegetation::v11::V11ExecutionError<DirectV11RealConsumerError>,
> {
    begin_snow_free_physical_reuse_audit_v1();
    let execution = (|| {
        let mut executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
            stack: DirectV11RealConsumerStack::new_parent_child(
                beginning,
                interval,
                day_index,
                interval_index,
                finalize_wb14_parent_interval,
                binding,
            ),
        };
        let candidate = crate::v11_vegetation_consumer::execute_direct_v11_segment(
            configuration,
            parent,
            receipt,
            &mut executor,
        )?;
        let ending = executor.stack.take_staged_ending().ok_or(
            openwepp_vegetation::v11::V11ExecutionError::Executor(
                DirectV11RealConsumerError::Identity("fresh restart execution staged ending"),
            ),
        )?;
        Ok((candidate, ending))
    })();
    let audit = take_snow_free_physical_reuse_audit_v1();
    execution.map(|(candidate, ending)| (candidate, ending, audit))
}

impl Clone for DirectV11RealConsumerStack<'_> {
    fn clone(&self) -> Self {
        Self {
            beginning: self.beginning.clone(),
            interval: self.interval,
            day_index: self.day_index,
            interval_index: self.interval_index,
            finalize_wb14_parent_interval: self.finalize_wb14_parent_interval,
            wb14_coupled_child_binding: self.wb14_coupled_child_binding,
            native_inactive_wb14_prefix: self.native_inactive_wb14_prefix,
            ending: self.ending.clone(),
            last_support_receipt: self.last_support_receipt.clone(),
            #[cfg(test)]
            last_hydrology_candidate: self.last_hydrology_candidate.clone(),
            ending_snow_owner_bytes: self.ending_snow_owner_bytes.clone(),
            deferred_native_v2_soil_custody: self.deferred_native_v2_soil_custody.clone(),
            // Cloning a stack never copies or transfers producer material or
            // the move-only proof, but retains an explicit refusal tombstone.
            snow_free_physical_reuse_pending: None,
            snow_free_physical_reuse_seed: (self.snow_free_physical_reuse_pending.is_some()
                || self.snow_free_physical_reuse_seed.is_some())
            .then_some(SnowFreePhysicalReuseSeedV1::Refused),
        }
    }
}

impl DirectV11RealConsumerStack<'_> {
    pub(super) fn invalidate_snow_free_physical_reuse(&mut self) {
        let had_authority = self.snow_free_physical_reuse_pending.take().is_some()
            || self.snow_free_physical_reuse_seed.is_some();
        if had_authority {
            self.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Refused);
        }
    }

    pub(super) fn authenticate_snow_free_outer_candidate(
        &mut self,
        candidate: &V11AcceptedSegmentCandidate,
    ) -> Result<(), DirectV11RealConsumerError> {
        if let Some(pending) = self.snow_free_physical_reuse_pending.take() {
            let rollback_ending = pending.outer_auth_rollback_ending.clone();
            let rollback_support = pending.outer_auth_rollback_support.clone();
            if self.snow_free_physical_reuse_seed.is_some() {
                self.ending = rollback_ending;
                self.last_support_receipt = rollback_support;
                self.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Refused);
                return Err(DirectV11RealConsumerError::Identity(
                    "snow-free physical reuse lifecycle",
                ));
            }
            // Install the refusal tombstone before any fallible outer join.
            // A failed mint must restore the pre-execution staged state and
            // can never expose the ordinary physical dispatcher again.
            self.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Refused);
            #[cfg(test)]
            if take_snow_free_outer_auth_failure_v1() {
                self.ending = rollback_ending;
                self.last_support_receipt = rollback_support;
                return Err(DirectV11RealConsumerError::Identity(
                    "injected snow-free outer authentication failure",
                ));
            }
            let seed = match SnowFreePhysicalReuseCapabilityV1::mint_after_outer_validation(
                pending,
                candidate,
                self.ending.as_ref(),
            ) {
                Ok(seed) => seed,
                Err(error) => {
                    self.ending = rollback_ending;
                    self.last_support_receipt = rollback_support;
                    return Err(error);
                }
            };
            self.snow_free_physical_reuse_seed =
                Some(SnowFreePhysicalReuseSeedV1::Authorized(Box::new(seed)));
            return Ok(());
        }
        let state = self.snow_free_physical_reuse_seed.take();
        match state {
            Some(SnowFreePhysicalReuseSeedV1::AwaitingFinalOuterValidation(expected)) => {
                let rollback_ending = expected.rollback_ending.clone();
                let rollback_support = expected.rollback_support.clone();
                // The final receipt is allowed to differ from the provisional
                // receipt, but the real outer consumer must have accepted the
                // exact final output staged by the identity reseal.
                self.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Refused);
                #[cfg(test)]
                if take_snow_free_outer_auth_failure_v1() {
                    self.ending = rollback_ending;
                    self.last_support_receipt = rollback_support;
                    return Err(DirectV11RealConsumerError::Identity(
                        "injected snow-free outer authentication failure",
                    ));
                }
                if let Err(error) = validate_outer_candidate_matches_output(
                    candidate,
                    &expected.output,
                    &expected.accepted_slab_receipt,
                ) {
                    self.ending = rollback_ending;
                    self.last_support_receipt = rollback_support;
                    return Err(error);
                }
                self.snow_free_physical_reuse_seed =
                    Some(SnowFreePhysicalReuseSeedV1::ReadyToCommit(expected));
                Ok(())
            }
            None => Ok(()),
            Some(SnowFreePhysicalReuseSeedV1::Consumed) => {
                self.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Consumed);
                Err(DirectV11RealConsumerError::Identity(
                    "snow-free physical reuse outer lifecycle",
                ))
            }
            Some(_) => {
                self.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Refused);
                Err(DirectV11RealConsumerError::Identity(
                    "snow-free physical reuse outer lifecycle",
                ))
            }
        }
    }

    pub(super) fn execute_snow_free_physical_reuse(
        &mut self,
        input: &V11ImportedV10SegmentInput,
    ) -> Result<V11ImportedV10SegmentOutput, DirectV11RealConsumerError> {
        let validation_profile = ImportedStackProfileScopeV1::begin("imported reuse validation");
        if self.snow_free_physical_reuse_pending.take().is_some() {
            self.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Refused);
            return Err(DirectV11RealConsumerError::Identity(
                "snow-free physical reuse outer validation pending",
            ));
        }
        let state = self.snow_free_physical_reuse_seed.take().ok_or(
            DirectV11RealConsumerError::Identity("snow-free physical reuse seed"),
        )?;
        let SnowFreePhysicalReuseSeedV1::Armed(seed) = state else {
            self.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Refused);
            return Err(DirectV11RealConsumerError::Identity(
                "snow-free physical reuse single use",
            ));
        };
        // Install the refusal tombstone before checking any caller-controlled
        // identity. Every rejection consumes the proof and cannot replay
        // physical execution on a later call.
        self.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Refused);
        let final_binding = seed
            .final_binding
            .ok_or(DirectV11RealConsumerError::Identity(
                "snow-free physical reuse final authorization",
            ))?;
        if !seed
            .live_beginning_revision_receipt
            .shares_live_beginning_revision_with(&input.accepted_slab_receipt)
            || !seed
                .live_beginning_revision_receipt
                .shares_nonending_context_with(&input.accepted_slab_receipt)
            || seed.authority != physical_reuse_authority(input)?
            || final_binding.accepted_slab_sha256
                != *input.accepted_slab_receipt.slab_id().digest().as_bytes()
            || self.beginning != seed.beginning
            || self.interval != &seed.interval
            || self.day_index != seed.day_index
            || self.interval_index != seed.interval_index
            || self.finalize_wb14_parent_interval != seed.finalize_wb14_parent_interval
            || self.wb14_coupled_child_binding != Some(final_binding)
            || self.native_inactive_wb14_prefix != seed.native_inactive_wb14_prefix
            || self.ending_snow_owner_bytes != seed.ending_snow_owner_bytes
            || self.deferred_native_v2_soil_custody != seed.deferred_native_v2_soil_custody
            || self.ending.as_ref() != Some(&seed.physical_ending)
        {
            return Err(DirectV11RealConsumerError::Identity(
                "snow-free physical reuse identity",
            ));
        }
        drop(validation_profile);
        let reseal_profile = ImportedStackProfileScopeV1::begin("imported reuse reseal");

        let outer_auth_rollback_ending = self.ending.clone();
        let outer_auth_rollback_support = self.last_support_receipt.clone();
        let physical_ending = self
            .ending
            .take()
            .ok_or(DirectV11RealConsumerError::Identity(
                "snow-free physical reuse staged ending move",
            ))?;
        let reseal_result = (|| {
            #[cfg(test)]
            if take_snow_free_post_take_failure_v1() {
                return Err(DirectV11RealConsumerError::Identity(
                    "injected snow-free post-take failure",
                ));
            }
            if physical_ending != seed.physical_ending {
                return Err(DirectV11RealConsumerError::Identity(
                    "snow-free physical reuse staged ending move identity",
                ));
            }
            drop(seed.physical_ending);
            let mut physical_ending = physical_ending;
            if let Some(parent) = physical_ending.inner.wb14_parent_working_state.as_ref() {
                let rebound = parent
                    .rebind_final_accepted_slab(
                        &physical_ending.inner.surface_configuration,
                        final_binding,
                    )
                    .map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "snow-free physical reuse WB14 parent reseal",
                        )
                    })?;
                physical_ending.inner.wb14_parent_working_state = Some(rebound);
                physical_ending
                    .stage_frozen_litter_wb14_parent_from_inner_v1()
                    .map_err(DirectV11RealConsumerError::Runtime)?;
            }

            let publication_interval = DirectV11SnowCoveredSegmentInput {
                lse_forcing: self.interval.lse_forcing.clone(),
                vegetation_forcing: self.interval.vegetation_forcing.clone(),
                wb14_parameters: self.interval.wb14_parameters.clone(),
            };
            let provisional_ending_resource_owners =
                seed.provisional_output.ending_resource_owners.clone();
            let provisional_slab_sha256 = seed.provisional_binding.accepted_slab_sha256;
            let (output, candidate, support_receipt, publication_capability) =
                reseal_snow_free_final_accepted_slab_identity_v1(
                    &self.beginning,
                    input,
                    physical_ending,
                    seed.provisional_publication_support,
                    seed.provisional_output,
                    provisional_slab_sha256,
                    final_binding,
                    self.day_index,
                    self.interval_index,
                    &publication_interval,
                )?;
            if output.ending_resource_owners != provisional_ending_resource_owners {
                return Err(DirectV11RealConsumerError::Identity(
                    "snow-free physical reuse ending owners",
                ));
            }
            Ok((output, candidate, support_receipt, publication_capability))
        })();
        let (output, candidate, support_receipt, publication_capability) = match reseal_result {
            Ok(resealed) => resealed,
            Err(error) => {
                self.ending = outer_auth_rollback_ending;
                self.last_support_receipt = outer_auth_rollback_support;
                return Err(error);
            }
        };
        drop(reseal_profile);
        let install_profile = ImportedStackProfileScopeV1::begin("imported reuse install");
        #[cfg(any(test, feature = "persisted-restart-v1"))]
        record_snow_free_identity_reseal_v1();
        self.last_support_receipt = Some(support_receipt);
        self.ending = Some(candidate);
        self.snow_free_physical_reuse_seed =
            Some(SnowFreePhysicalReuseSeedV1::AwaitingFinalOuterValidation(
                Box::new(SnowFreeFinalOuterValidationV1 {
                    output: output.clone(),
                    accepted_slab_receipt: input.accepted_slab_receipt.clone(),
                    rollback_ending: outer_auth_rollback_ending,
                    rollback_support: outer_auth_rollback_support,
                    publication_capability,
                }),
            ));
        drop(install_profile);
        Ok(output)
    }

    pub(super) fn install_selected_snow_free_publication_v1(
        &mut self,
        ending: &mut DirectV10RealConsumerShadow,
    ) -> Result<(), DirectV11RealConsumerError> {
        let state = self.snow_free_physical_reuse_seed.take();
        let (capability, resealed) = match state {
            None => return Ok(()),
            Some(SnowFreePhysicalReuseSeedV1::Authorized(seed)) => {
                let capability = match seed
                    .provisional_publication_support
                    .validate_and_mint(ending.accepted_publication_history.live_revision_v1())
                {
                    Ok(capability) => capability,
                    Err(error) => {
                        self.snow_free_physical_reuse_seed =
                            Some(SnowFreePhysicalReuseSeedV1::Refused);
                        return Err(error);
                    }
                };
                (capability, false)
            }
            Some(SnowFreePhysicalReuseSeedV1::ReadyToCommit(expected)) => {
                (expected.publication_capability, true)
            }
            Some(state @ SnowFreePhysicalReuseSeedV1::Consumed)
            | Some(state @ SnowFreePhysicalReuseSeedV1::Refused) => {
                self.snow_free_physical_reuse_seed = Some(state);
                return Err(DirectV11RealConsumerError::Identity(
                    "snow-free selected publication lifecycle",
                ));
            }
            Some(SnowFreePhysicalReuseSeedV1::Armed(_))
            | Some(SnowFreePhysicalReuseSeedV1::AwaitingFinalOuterValidation(_)) => {
                self.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Refused);
                return Err(DirectV11RealConsumerError::Identity(
                    "snow-free selected publication outer validation",
                ));
            }
        };
        if let Err(error) = ending
            .accepted_publication_history
            .push_validated_support(capability)
        {
            self.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Refused);
            return Err(error);
        }
        if resealed {
            #[cfg(any(test, feature = "persisted-restart-v1"))]
            record_snow_free_final_publication_append_v1();
        }
        self.snow_free_physical_reuse_seed = Some(SnowFreePhysicalReuseSeedV1::Consumed);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slab_only_binding_gate_rejects_every_non_slab_coordinate() {
        let provisional = crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
            proposed_upper_bound_s_bits: 1_800.0_f64.to_bits(),
            coupled_parent_transaction_sha256: [1; 32],
            accepted_slab_sha256: [2; 32],
            parent_beginning_complete_owner_set_sha256: [3; 32],
            parent_support_start_ns: 4,
            parent_support_end_ns: 5,
            child_support_start_ns: 4,
            child_support_end_ns: 5,
        };
        let final_binding = crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
            accepted_slab_sha256: [9; 32],
            ..provisional
        };
        assert!(binding_differs_only_by_slab(provisional, final_binding));
        let poisons = [
            crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                proposed_upper_bound_s_bits: 60.0_f64.to_bits(),
                ..final_binding
            },
            crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                coupled_parent_transaction_sha256: [8; 32],
                ..final_binding
            },
            crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                parent_beginning_complete_owner_set_sha256: [8; 32],
                ..final_binding
            },
            crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                parent_support_start_ns: 3,
                ..final_binding
            },
            crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                parent_support_end_ns: 6,
                ..final_binding
            },
            crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                child_support_start_ns: 3,
                ..final_binding
            },
            crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                child_support_end_ns: 6,
                ..final_binding
            },
        ];
        for poison in poisons {
            assert!(!binding_differs_only_by_slab(provisional, poison));
        }
        assert!(!binding_differs_only_by_slab(provisional, provisional));
    }

    #[test]
    fn proof_surface_is_private_non_wire_and_single_use() {
        let source = include_str!("snow_free_physical_reuse.rs");
        let production = source
            .split("\n#[cfg(test)]\nmod tests {")
            .next()
            .expect("production implementation before the test module");
        assert!(!production.contains("fn proof_surface_is_private_non_wire_and_single_use()"));
        let proof = production
            .split("pub(crate) enum SnowFreePhysicalReuseSeedV1")
            .nth(1)
            .expect("proof")
            .split("struct SnowFreeFinalOuterValidationV1")
            .next()
            .expect("proof fields");
        assert!(!proof.contains("Serialize"));
        assert!(!proof.contains("Clone"));
        assert!(production.contains("snow_free_physical_reuse_seed.take()"));
        assert!(production.contains("SnowFreePhysicalReuseSeedV1::Consumed"));
        assert!(production.contains("SnowFreePhysicalReuseSeedV1::Refused"));
        assert!(production.contains("snow-free physical reuse ending owners"));
        assert!(production.contains("shares_live_beginning_revision_with"));
        assert!(production.contains("shares_nonending_context_with"));
        assert!(production.contains("authenticates_complete_ending_owners"));
        assert!(!production.contains("accepted_receipt_context_sha256"));
        assert!(!production.contains("serde_json::to_value(receipt)"));
        assert!(production.contains("mint_after_outer_validation"));
        assert!(
            include_str!("../v11_vegetation_consumer.rs")
                .contains("authenticate_outer_validated_candidate")
        );
    }

    #[test]
    fn final_reseal_is_identity_only_and_never_calls_the_broad_physical_finalizer() {
        let source = include_str!("snow_free_physical_reuse.rs");
        let body = source
            .split("fn reseal_snow_free_final_accepted_slab_identity_v1(")
            .nth(1)
            .expect("identity-only final reseal")
            .split("fn binding_differs_only_by_slab(")
            .next()
            .expect("identity-only final reseal body");
        for forbidden in [
            "finalize_v11_imported_segment_with_soil_continuation(",
            "accept_envelope_with_soil_top_boundary_credits(",
            "accepted_v2_soil_candidate_for_v11_segment(",
            "aggregate_soil_thermal_ending_v2(",
            "execute_canonical_covered_production_v1(",
        ] {
            assert!(
                !body.contains(forbidden),
                "forbidden physical replay: {forbidden}"
            );
        }
        assert!(!body.contains("push_validated_support("));
        assert!(body.contains("final_publication_support"));
        assert!(body.contains("reseal_snow_free_publication_support_v1("));
        assert!(!body.contains("retain_accepted_publication_support("));
        assert!(!body.contains("provisional_publication_support.clone()"));
        assert!(body.contains("V11ResourceDebit::new("));
        assert!(body.contains("V11SharedResourceOwnerTransition::new("));
    }
}
