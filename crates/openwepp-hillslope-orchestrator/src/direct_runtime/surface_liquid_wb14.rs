use crate::constants::WB11_ZERO_THRESHOLD;
use openwepp_coupled_time::{FramedField, framed_sha256};
use serde::{Deserialize, Serialize};

use super::{
    DirectRuntimeError, validate_finite, validate_nonnegative_direct_m, validate_positive_direct,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct DirectWb14ContinuationIntervalInputs {
    pub cumulative_supply_m: f64,
    pub cumulative_infiltration_m: f64,
    pub interval_supply_m: f64,
    pub interval_duration_s: f64,
    pub effective_conductivity_m_s: f64,
    pub matric_potential_m: f64,
    pub storage_capacity_m: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct DirectWb14IntervalTransitionInputs {
    pub cumulative_supply_m: f64,
    pub cumulative_infiltration_m: f64,
    pub interval_supply_m: f64,
    pub interval_duration_s: f64,
    pub interval_intensity_m_s: f64,
    pub effective_conductivity_m_s: f64,
    pub matric_potential_m: f64,
    pub storage_capacity_m: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::struct_field_names)]
pub(super) struct DirectWb14ContinuationIntervalOutcome {
    pub cumulative_supply_m: f64,
    pub cumulative_infiltration_m: f64,
    pub interval_infiltration_m: f64,
    pub interval_excess_m: f64,
}

/// Own the complete production WB14 interval state transition.
///
/// The unchanged daily wrapper and the persistent 1800-second continuation
/// both enter here so storage exhaustion, guards, clamps, and cumulative-state
/// arithmetic cannot drift between those two execution shapes.
pub(super) fn advance_wb14_interval_state(
    inputs: DirectWb14IntervalTransitionInputs,
) -> Result<DirectWb14ContinuationIntervalOutcome, DirectRuntimeError> {
    validate_nonnegative_direct_m(
        "infiltration_depression.cumulative_supply_m",
        inputs.cumulative_supply_m,
    )?;
    validate_nonnegative_direct_m(
        "infiltration_depression.cumulative_infiltration_m",
        inputs.cumulative_infiltration_m,
    )?;
    validate_nonnegative_direct_m(
        "infiltration_depression.interval_supply_m",
        inputs.interval_supply_m,
    )?;
    validate_positive_direct(
        "infiltration_depression.interval_duration_s",
        inputs.interval_duration_s,
    )?;
    validate_nonnegative_direct_m(
        "infiltration_depression.interval_intensity_m_s",
        inputs.interval_intensity_m_s,
    )?;
    validate_positive_direct(
        "infiltration_depression.effective_conductivity_m_s",
        inputs.effective_conductivity_m_s,
    )?;
    validate_nonnegative_direct_m(
        "infiltration_depression.matric_potential_m",
        inputs.matric_potential_m,
    )?;
    validate_nonnegative_direct_m(
        "infiltration_depression.storage_capacity_m",
        inputs.storage_capacity_m,
    )?;
    if inputs.cumulative_infiltration_m > inputs.cumulative_supply_m
        || inputs.cumulative_infiltration_m > inputs.storage_capacity_m
    {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "infiltration_depression.continuation_bounds",
        });
    }

    let remaining_storage_m =
        (inputs.storage_capacity_m - inputs.cumulative_infiltration_m).max(0.0);
    let computed_interval_infiltration_m = if remaining_storage_m <= WB11_ZERO_THRESHOLD {
        0.0
    } else {
        super::runoff::compute_green_ampt_interval_infiltration(
            inputs.cumulative_infiltration_m,
            inputs.interval_supply_m.min(remaining_storage_m),
            inputs.interval_duration_s,
            inputs.interval_intensity_m_s,
            inputs.effective_conductivity_m_s,
            inputs.matric_potential_m,
        )?
    };
    if computed_interval_infiltration_m > inputs.interval_supply_m + 1.0e-9 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "infiltration_depression.interval_infiltration_m",
        });
    }

    let cumulative_supply_m = inputs.cumulative_supply_m + inputs.interval_supply_m;
    validate_finite(
        "infiltration_depression.cumulative_supply_m",
        cumulative_supply_m,
    )?;
    let cumulative_infiltration_m = (inputs.cumulative_infiltration_m
        + computed_interval_infiltration_m.min(inputs.interval_supply_m))
    .min(inputs.storage_capacity_m)
    .min(cumulative_supply_m);
    validate_finite(
        "infiltration_depression.cumulative_infiltration_m",
        cumulative_infiltration_m,
    )?;
    let interval_excess_m = (inputs.interval_supply_m
        - (cumulative_infiltration_m - inputs.cumulative_infiltration_m))
        .max(0.0);
    validate_nonnegative_direct_m(
        "infiltration_depression.interval_excess_m",
        interval_excess_m,
    )?;
    let interval_infiltration_m = inputs.interval_supply_m - interval_excess_m;

    Ok(DirectWb14ContinuationIntervalOutcome {
        cumulative_supply_m,
        cumulative_infiltration_m,
        interval_infiltration_m,
        interval_excess_m,
    })
}

/// Advance the persistent continuation without replaying accepted day state.
pub(super) fn advance_wb14_continuation_interval(
    inputs: DirectWb14ContinuationIntervalInputs,
) -> Result<DirectWb14ContinuationIntervalOutcome, DirectRuntimeError> {
    advance_wb14_interval_state(DirectWb14IntervalTransitionInputs {
        cumulative_supply_m: inputs.cumulative_supply_m,
        cumulative_infiltration_m: inputs.cumulative_infiltration_m,
        interval_supply_m: inputs.interval_supply_m,
        interval_duration_s: inputs.interval_duration_s,
        interval_intensity_m_s: inputs.interval_supply_m / inputs.interval_duration_s,
        effective_conductivity_m_s: inputs.effective_conductivity_m_s,
        matric_potential_m: inputs.matric_potential_m,
        storage_capacity_m: inputs.storage_capacity_m,
    })
}

/// Persistent WB14 day/interval cursor. Child slabs never mutate this value;
/// exactly one finalized parent produces its successor.
// The v8 review candidate consumes this authority through the complete-owner
// attachment; runtime promotion remains governed by SC-SURFACELIQUID-001.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(super) struct DirectWb14PersistentCursorV1 {
    pub day_index: usize,
    pub next_interval_index: u8,
    pub cumulative_supply_m: f64,
    pub cumulative_infiltration_m: f64,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(super) struct DirectWb14ImmutableIdentityV1 {
    pub schema_sha256: [u8; 32],
    pub ofe_id_sha256: [u8; 32],
    pub production_lane_id: u32,
    pub surface_liquid_configuration_sha256: [u8; 32],
    pub wb14_configuration_sha256: [u8; 32],
    pub wb14_model_definition_sha256: [u8; 32],
    pub effective_conductivity_m_s_bits: u64,
    pub matric_potential_m_bits: u64,
    pub storage_capacity_m_bits: u64,
}

/// Immutable authority for one existing 1,800-second WB14 interval.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(super) struct DirectWb14ParentAuthorityV1 {
    pub parent_id: [u8; 32],
    pub coupled_parent_transaction_sha256: [u8; 32],
    pub parent_day_index: usize,
    pub parent_interval_index: u8,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub parent_beginning_owner_sha256: [u8; 32],
    pub beginning_cursor_sha256: [u8; 32],
    pub schema_sha256: [u8; 32],
    pub ofe_id_sha256: [u8; 32],
    pub production_lane_id: u32,
    pub surface_liquid_configuration_sha256: [u8; 32],
    pub wb14_configuration_sha256: [u8; 32],
    pub wb14_model_definition_sha256: [u8; 32],
    pub effective_conductivity_m_s_bits: u64,
    pub matric_potential_m_bits: u64,
    pub storage_capacity_m_bits: u64,
}

#[allow(dead_code)]
impl DirectWb14ParentAuthorityV1 {
    const fn immutable_identity(self) -> DirectWb14ImmutableIdentityV1 {
        DirectWb14ImmutableIdentityV1 {
            schema_sha256: self.schema_sha256,
            ofe_id_sha256: self.ofe_id_sha256,
            production_lane_id: self.production_lane_id,
            surface_liquid_configuration_sha256: self.surface_liquid_configuration_sha256,
            wb14_configuration_sha256: self.wb14_configuration_sha256,
            wb14_model_definition_sha256: self.wb14_model_definition_sha256,
            effective_conductivity_m_s_bits: self.effective_conductivity_m_s_bits,
            matric_potential_m_bits: self.matric_potential_m_bits,
            storage_capacity_m_bits: self.storage_capacity_m_bits,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(super) struct DirectWb14ParentWorkingStateV1 {
    pub accepted_until_ns: u128,
    pub next_child_ordinal: u32,
    pub cumulative_supply_m: f64,
    pub cumulative_infiltration_m: f64,
    pub receipt_chain_sha256: [u8; 32],
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(super) struct DirectWb14ChildReceiptV1 {
    pub parent_id: [u8; 32],
    pub ofe_id_sha256: [u8; 32],
    pub production_lane_id: u32,
    pub surface_liquid_configuration_sha256: [u8; 32],
    pub wb14_configuration_sha256: [u8; 32],
    pub wb14_model_definition_sha256: [u8; 32],
    pub effective_conductivity_m_s_bits: u64,
    pub matric_potential_m_bits: u64,
    pub storage_capacity_m_bits: u64,
    pub parent_beginning_owner_sha256: [u8; 32],
    pub parent_beginning_cursor_sha256: [u8; 32],
    pub ordinal: u32,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub beginning_working_state_sha256: [u8; 32],
    pub ending_working_state_sha256: [u8; 32],
    pub predecessor_receipt_sha256: [u8; 32],
    pub accepted_coupled_slab_sha256: [u8; 32],
    pub child_beginning_complete_owner_set_sha256: [u8; 32],
    pub pending_routed_parcels_before_sha256: [u8; 32],
    pub pending_routed_parcels_after_sha256: [u8; 32],
    pub child_inputs_sha256: [u8; 32],
    pub transitions: Vec<DirectWb14ChildTransitionV1>,
    pub proposed_upper_bound_s_bits: u64,
    pub accepted_duration_s_bits: u64,
    pub cumulative_supply_m_bits: u64,
    pub cumulative_infiltration_m_bits: u64,
    pub interval_supply_m_bits: u64,
    pub interval_infiltration_m_bits: u64,
    pub interval_excess_m_bits: u64,
    pub receipt_sha256: [u8; 32],
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(super) struct DirectWb14ChildTransitionV1 {
    pub cumulative_supply_m_bits: u64,
    pub cumulative_infiltration_m_bits: u64,
    pub interval_supply_m_bits: u64,
    pub interval_duration_s_bits: u64,
    pub effective_conductivity_m_s_bits: u64,
    pub matric_potential_m_bits: u64,
    pub storage_capacity_m_bits: u64,
}

impl From<DirectWb14ContinuationIntervalInputs> for DirectWb14ChildTransitionV1 {
    fn from(value: DirectWb14ContinuationIntervalInputs) -> Self {
        Self {
            cumulative_supply_m_bits: value.cumulative_supply_m.to_bits(),
            cumulative_infiltration_m_bits: value.cumulative_infiltration_m.to_bits(),
            interval_supply_m_bits: value.interval_supply_m.to_bits(),
            interval_duration_s_bits: value.interval_duration_s.to_bits(),
            effective_conductivity_m_s_bits: value.effective_conductivity_m_s.to_bits(),
            matric_potential_m_bits: value.matric_potential_m.to_bits(),
            storage_capacity_m_bits: value.storage_capacity_m.to_bits(),
        }
    }
}

impl DirectWb14ChildTransitionV1 {
    fn inputs(self) -> DirectWb14ContinuationIntervalInputs {
        DirectWb14ContinuationIntervalInputs {
            cumulative_supply_m: f64::from_bits(self.cumulative_supply_m_bits),
            cumulative_infiltration_m: f64::from_bits(self.cumulative_infiltration_m_bits),
            interval_supply_m: f64::from_bits(self.interval_supply_m_bits),
            interval_duration_s: f64::from_bits(self.interval_duration_s_bits),
            effective_conductivity_m_s: f64::from_bits(self.effective_conductivity_m_s_bits),
            matric_potential_m: f64::from_bits(self.matric_potential_m_bits),
            storage_capacity_m: f64::from_bits(self.storage_capacity_m_bits),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(super) struct DirectWb14ParentReceiptV1 {
    pub parent_id: [u8; 32],
    pub coupled_parent_transaction_sha256: [u8; 32],
    pub parent_day_index: usize,
    pub parent_interval_index: u8,
    pub parent_beginning_owner_sha256: [u8; 32],
    pub beginning_cursor_sha256: [u8; 32],
    pub ending_cursor_sha256: [u8; 32],
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub ordered_child_receipt_sha256: Vec<[u8; 32]>,
    pub receipt_chain_sha256: [u8; 32],
    pub receipt_sha256: [u8; 32],
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(super) struct DirectWb14ParentFinalizationV1 {
    pub persistent_cursor: DirectWb14PersistentCursorV1,
    pub receipt: DirectWb14ParentReceiptV1,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(super) struct DirectWb14ParentIntervalV1 {
    authority: DirectWb14ParentAuthorityV1,
    beginning_cursor: DirectWb14PersistentCursorV1,
    working: DirectWb14ParentWorkingStateV1,
    receipts: Vec<DirectWb14ChildReceiptV1>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub(super) enum DirectWb14ParentIntervalErrorV1 {
    #[error("WB14 parent support is not exactly 1,800 seconds")]
    ParentSupport,
    #[error("WB14 parent does not match the persistent day/interval cursor")]
    CursorIdentity,
    #[error("WB14 proposed upper bound is not an admitted 1,800/900/60-second cadence")]
    ChildCadence,
    #[error("WB14 child support is not the next contiguous parent support")]
    ChildSupport,
    #[error("WB14 child ordinal or receipt-chain identity is invalid")]
    ChildIdentity,
    #[error("WB14 parent is incomplete")]
    ParentIncomplete,
    #[error("WB14 parent arithmetic failed")]
    Arithmetic,
    #[error("WB14 parent identity framing failed")]
    IdentityHash,
    #[error("WB14 immutable identity or parameter binding is invalid")]
    ImmutableIdentity,
    #[error("WB14 beginning cursor cumulatives are invalid")]
    BeginningState,
    #[error("WB14 receipt reconstruction failed")]
    ReceiptValidation,
}

#[allow(dead_code)]
fn wb14_hash(
    domain: &str,
    fields: &[FramedField<'_>],
) -> Result<[u8; 32], DirectWb14ParentIntervalErrorV1> {
    framed_sha256(domain, fields)
        .map(|digest| *digest.as_bytes())
        .map_err(|_| DirectWb14ParentIntervalErrorV1::IdentityHash)
}

#[allow(dead_code)]
fn cursor_digest(
    cursor: DirectWb14PersistentCursorV1,
) -> Result<[u8; 32], DirectWb14ParentIntervalErrorV1> {
    let day_index =
        u64::try_from(cursor.day_index).map_err(|_| DirectWb14ParentIntervalErrorV1::Arithmetic)?;
    wb14_hash(
        "openwepp-wb14-persistent-cursor-v1",
        &[
            FramedField {
                tag: "day_index",
                value: &day_index.to_be_bytes(),
            },
            FramedField {
                tag: "next_interval_index",
                value: &[cursor.next_interval_index],
            },
            FramedField {
                tag: "cumulative_supply_m_bits",
                value: &cursor.cumulative_supply_m.to_bits().to_be_bytes(),
            },
            FramedField {
                tag: "cumulative_infiltration_m_bits",
                value: &cursor.cumulative_infiltration_m.to_bits().to_be_bytes(),
            },
        ],
    )
}

#[allow(dead_code)]
fn working_digest(
    working: DirectWb14ParentWorkingStateV1,
) -> Result<[u8; 32], DirectWb14ParentIntervalErrorV1> {
    wb14_hash(
        "openwepp-wb14-parent-working-state-v1",
        &[
            FramedField {
                tag: "accepted_until_ns",
                value: &working.accepted_until_ns.to_be_bytes(),
            },
            FramedField {
                tag: "next_child_ordinal",
                value: &working.next_child_ordinal.to_be_bytes(),
            },
            FramedField {
                tag: "cumulative_supply_m_bits",
                value: &working.cumulative_supply_m.to_bits().to_be_bytes(),
            },
            FramedField {
                tag: "cumulative_infiltration_m_bits",
                value: &working.cumulative_infiltration_m.to_bits().to_be_bytes(),
            },
            FramedField {
                tag: "receipt_chain_sha256",
                value: &working.receipt_chain_sha256,
            },
        ],
    )
}

#[allow(dead_code)]
fn parent_id(
    coupled_parent_transaction_sha256: [u8; 32],
    parent_day_index: usize,
    parent_interval_index: u8,
    support_start_ns: u128,
    support_end_ns: u128,
    parent_beginning_owner_sha256: [u8; 32],
    beginning_cursor_sha256: [u8; 32],
    identity: DirectWb14ImmutableIdentityV1,
) -> Result<[u8; 32], DirectWb14ParentIntervalErrorV1> {
    let parent_day_index =
        u64::try_from(parent_day_index).map_err(|_| DirectWb14ParentIntervalErrorV1::Arithmetic)?;
    wb14_hash(
        "openwepp-wb14-parent-interval-v1",
        &[
            FramedField {
                tag: "coupled_parent_transaction_sha256",
                value: &coupled_parent_transaction_sha256,
            },
            FramedField {
                tag: "parent_day_index",
                value: &parent_day_index.to_be_bytes(),
            },
            FramedField {
                tag: "parent_interval_index",
                value: &[parent_interval_index],
            },
            FramedField {
                tag: "support_start_ns",
                value: &support_start_ns.to_be_bytes(),
            },
            FramedField {
                tag: "support_end_ns",
                value: &support_end_ns.to_be_bytes(),
            },
            FramedField {
                tag: "parent_beginning_owner_sha256",
                value: &parent_beginning_owner_sha256,
            },
            FramedField {
                tag: "beginning_cursor_sha256",
                value: &beginning_cursor_sha256,
            },
            FramedField {
                tag: "schema_sha256",
                value: &identity.schema_sha256,
            },
            FramedField {
                tag: "ofe_id_sha256",
                value: &identity.ofe_id_sha256,
            },
            FramedField {
                tag: "production_lane_id",
                value: &identity.production_lane_id.to_be_bytes(),
            },
            FramedField {
                tag: "surface_liquid_configuration_sha256",
                value: &identity.surface_liquid_configuration_sha256,
            },
            FramedField {
                tag: "wb14_configuration_sha256",
                value: &identity.wb14_configuration_sha256,
            },
            FramedField {
                tag: "wb14_model_definition_sha256",
                value: &identity.wb14_model_definition_sha256,
            },
            FramedField {
                tag: "effective_conductivity_m_s_bits",
                value: &identity.effective_conductivity_m_s_bits.to_be_bytes(),
            },
            FramedField {
                tag: "matric_potential_m_bits",
                value: &identity.matric_potential_m_bits.to_be_bytes(),
            },
            FramedField {
                tag: "storage_capacity_m_bits",
                value: &identity.storage_capacity_m_bits.to_be_bytes(),
            },
        ],
    )
}

#[allow(dead_code)]
pub(super) fn wb14_parent_authority_v1(
    coupled_parent_transaction_sha256: [u8; 32],
    support_start_ns: u128,
    support_end_ns: u128,
    parent_beginning_owner_sha256: [u8; 32],
    cursor: DirectWb14PersistentCursorV1,
    identity: DirectWb14ImmutableIdentityV1,
) -> Result<DirectWb14ParentAuthorityV1, DirectWb14ParentIntervalErrorV1> {
    let beginning_cursor_sha256 = cursor_digest(cursor)?;
    let (parent_day_index, parent_interval_index) = if cursor.next_interval_index == 48 {
        (
            cursor
                .day_index
                .checked_add(1)
                .ok_or(DirectWb14ParentIntervalErrorV1::Arithmetic)?,
            0,
        )
    } else {
        (cursor.day_index, cursor.next_interval_index)
    };
    Ok(DirectWb14ParentAuthorityV1 {
        parent_id: parent_id(
            coupled_parent_transaction_sha256,
            parent_day_index,
            parent_interval_index,
            support_start_ns,
            support_end_ns,
            parent_beginning_owner_sha256,
            beginning_cursor_sha256,
            identity,
        )?,
        coupled_parent_transaction_sha256,
        parent_day_index,
        parent_interval_index,
        support_start_ns,
        support_end_ns,
        parent_beginning_owner_sha256,
        beginning_cursor_sha256,
        schema_sha256: identity.schema_sha256,
        ofe_id_sha256: identity.ofe_id_sha256,
        production_lane_id: identity.production_lane_id,
        surface_liquid_configuration_sha256: identity.surface_liquid_configuration_sha256,
        wb14_configuration_sha256: identity.wb14_configuration_sha256,
        wb14_model_definition_sha256: identity.wb14_model_definition_sha256,
        effective_conductivity_m_s_bits: identity.effective_conductivity_m_s_bits,
        matric_potential_m_bits: identity.matric_potential_m_bits,
        storage_capacity_m_bits: identity.storage_capacity_m_bits,
    })
}

#[allow(dead_code)]
impl DirectWb14ParentIntervalV1 {
    pub(super) fn canonical_sha256(&self) -> Result<[u8; 32], DirectWb14ParentIntervalErrorV1> {
        self.validate()?;
        let mut receipts = Vec::with_capacity(self.receipts.len() * 32);
        for receipt in &self.receipts {
            receipts.extend_from_slice(&receipt.receipt_sha256);
        }
        let working_state_sha256 = working_digest(self.working)?;
        wb14_hash(
            "openwepp-wb14-parent-candidate-v1",
            &[
                FramedField {
                    tag: "parent_id",
                    value: &self.authority.parent_id,
                },
                FramedField {
                    tag: "working_state_sha256",
                    value: &working_state_sha256,
                },
                FramedField {
                    tag: "ordered_child_receipts",
                    value: &receipts,
                },
            ],
        )
    }

    pub(super) fn begin(
        authority: DirectWb14ParentAuthorityV1,
        cursor: DirectWb14PersistentCursorV1,
    ) -> Result<Self, DirectWb14ParentIntervalErrorV1> {
        let identity = authority.immutable_identity();
        if [
            authority.coupled_parent_transaction_sha256,
            authority.parent_beginning_owner_sha256,
            authority.beginning_cursor_sha256,
            identity.schema_sha256,
            identity.surface_liquid_configuration_sha256,
            identity.wb14_configuration_sha256,
            identity.wb14_model_definition_sha256,
        ]
        .iter()
        .any(|digest| *digest == [0; 32])
        {
            return Err(DirectWb14ParentIntervalErrorV1::ImmutableIdentity);
        }
        if !cursor.cumulative_supply_m.is_finite()
            || !cursor.cumulative_infiltration_m.is_finite()
            || cursor.cumulative_supply_m < 0.0
            || cursor.cumulative_infiltration_m < 0.0
            || cursor.cumulative_infiltration_m > cursor.cumulative_supply_m
        {
            return Err(DirectWb14ParentIntervalErrorV1::BeginningState);
        }
        let conductivity = f64::from_bits(identity.effective_conductivity_m_s_bits);
        let matric = f64::from_bits(identity.matric_potential_m_bits);
        let capacity = f64::from_bits(identity.storage_capacity_m_bits);
        if !conductivity.is_finite()
            || conductivity <= 0.0
            || !matric.is_finite()
            || matric < 0.0
            || !capacity.is_finite()
            || capacity < 0.0
            || cursor.cumulative_infiltration_m > capacity
        {
            return Err(DirectWb14ParentIntervalErrorV1::ImmutableIdentity);
        }
        if authority
            .support_end_ns
            .checked_sub(authority.support_start_ns)
            != Some(1_800_000_000_000)
        {
            return Err(DirectWb14ParentIntervalErrorV1::ParentSupport);
        }
        let expected_parent = if cursor.next_interval_index == 48 {
            (
                cursor
                    .day_index
                    .checked_add(1)
                    .ok_or(DirectWb14ParentIntervalErrorV1::Arithmetic)?,
                0,
            )
        } else {
            (cursor.day_index, cursor.next_interval_index)
        };
        if cursor.next_interval_index > 48
            || authority.beginning_cursor_sha256 != cursor_digest(cursor)?
            || (authority.parent_day_index, authority.parent_interval_index) != expected_parent
            || authority.parent_id
                != parent_id(
                    authority.coupled_parent_transaction_sha256,
                    authority.parent_day_index,
                    authority.parent_interval_index,
                    authority.support_start_ns,
                    authority.support_end_ns,
                    authority.parent_beginning_owner_sha256,
                    authority.beginning_cursor_sha256,
                    identity,
                )?
        {
            return Err(DirectWb14ParentIntervalErrorV1::CursorIdentity);
        }
        let chain = wb14_hash(
            "openwepp-wb14-parent-receipt-chain-begin-v1",
            &[
                FramedField {
                    tag: "parent_id",
                    value: &authority.parent_id,
                },
                FramedField {
                    tag: "beginning_cursor_sha256",
                    value: &authority.beginning_cursor_sha256,
                },
            ],
        )?;
        let (cumulative_supply_m, cumulative_infiltration_m) = if cursor.next_interval_index == 48 {
            (0.0, 0.0)
        } else {
            (cursor.cumulative_supply_m, cursor.cumulative_infiltration_m)
        };
        Ok(Self {
            authority,
            beginning_cursor: cursor,
            working: DirectWb14ParentWorkingStateV1 {
                accepted_until_ns: authority.support_start_ns,
                next_child_ordinal: 0,
                cumulative_supply_m,
                cumulative_infiltration_m,
                receipt_chain_sha256: chain,
            },
            receipts: Vec::new(),
        })
    }

    /// Return a new accepted candidate; `self` is unchanged on every failure.
    pub(super) fn accept_child(
        &self,
        ordinal: u32,
        support_start_ns: u128,
        support_end_ns: u128,
        predecessor_receipt_sha256: [u8; 32],
        proposed_upper_bound_s: f64,
        inputs: DirectWb14ContinuationIntervalInputs,
    ) -> Result<(Self, DirectWb14ContinuationIntervalOutcome), DirectWb14ParentIntervalErrorV1>
    {
        self.accept_child_transitions(
            ordinal,
            support_start_ns,
            support_end_ns,
            predecessor_receipt_sha256,
            proposed_upper_bound_s,
            &[inputs],
        )
    }

    pub(super) fn accept_child_transitions(
        &self,
        ordinal: u32,
        support_start_ns: u128,
        support_end_ns: u128,
        predecessor_receipt_sha256: [u8; 32],
        proposed_upper_bound_s: f64,
        inputs: &[DirectWb14ContinuationIntervalInputs],
    ) -> Result<(Self, DirectWb14ContinuationIntervalOutcome), DirectWb14ParentIntervalErrorV1>
    {
        self.accept_child_transitions_with_slab(
            ordinal,
            support_start_ns,
            support_end_ns,
            predecessor_receipt_sha256,
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            proposed_upper_bound_s,
            inputs,
        )
    }

    pub(super) fn accept_child_transitions_with_slab(
        &self,
        ordinal: u32,
        support_start_ns: u128,
        support_end_ns: u128,
        predecessor_receipt_sha256: [u8; 32],
        accepted_coupled_slab_sha256: [u8; 32],
        child_beginning_complete_owner_set_sha256: [u8; 32],
        pending_routed_parcels_before_sha256: [u8; 32],
        pending_routed_parcels_after_sha256: [u8; 32],
        proposed_upper_bound_s: f64,
        inputs: &[DirectWb14ContinuationIntervalInputs],
    ) -> Result<(Self, DirectWb14ContinuationIntervalOutcome), DirectWb14ParentIntervalErrorV1>
    {
        let duration_ns = support_end_ns
            .checked_sub(support_start_ns)
            .ok_or(DirectWb14ParentIntervalErrorV1::ChildSupport)?;
        let admitted_proposal = [1_800.0_f64, 900.0, 60.0]
            .iter()
            .any(|candidate| candidate.to_bits() == proposed_upper_bound_s.to_bits());
        if !admitted_proposal || duration_ns == 0 || inputs.is_empty() {
            return Err(DirectWb14ParentIntervalErrorV1::ChildCadence);
        }
        let selected_seconds = duration_ns as f64 / 1_000_000_000.0;
        if selected_seconds > proposed_upper_bound_s {
            return Err(DirectWb14ParentIntervalErrorV1::ChildCadence);
        }
        if ordinal != self.working.next_child_ordinal
            || predecessor_receipt_sha256 != self.working.receipt_chain_sha256
        {
            return Err(DirectWb14ParentIntervalErrorV1::ChildIdentity);
        }
        // Timed-parcel durations arrive as differences of binary64 endpoints,
        // so integerizing each window independently would reject valid spans
        // such as [0.2, 0.3). Reconstruct their total with compensated
        // summation, then join that total once to the exact outer support.
        let mut transition_duration_s = 0.0_f64;
        let mut duration_compensation_s = 0.0_f64;
        for input in inputs {
            if !input.interval_duration_s.is_finite() || input.interval_duration_s <= 0.0 {
                return Err(DirectWb14ParentIntervalErrorV1::ChildSupport);
            }
            let corrected = input.interval_duration_s - duration_compensation_s;
            let next = transition_duration_s + corrected;
            duration_compensation_s = (next - transition_duration_s) - corrected;
            transition_duration_s = next;
        }
        let accepted_duration_s = duration_ns as f64 / 1_000_000_000.0;
        if support_start_ns != self.working.accepted_until_ns
            || support_end_ns > self.authority.support_end_ns
            || transition_duration_s.to_bits() != accepted_duration_s.to_bits()
        {
            return Err(DirectWb14ParentIntervalErrorV1::ChildSupport);
        }
        let mut expected_supply = self.working.cumulative_supply_m;
        let mut expected_infiltration = self.working.cumulative_infiltration_m;
        let mut interval_supply_m = 0.0;
        let mut interval_infiltration_m = 0.0;
        let mut interval_excess_m = 0.0;
        for input in inputs {
            if input.cumulative_supply_m.to_bits() != expected_supply.to_bits()
                || input.cumulative_infiltration_m.to_bits() != expected_infiltration.to_bits()
                || input.effective_conductivity_m_s.to_bits()
                    != self.authority.effective_conductivity_m_s_bits
                || input.matric_potential_m.to_bits() != self.authority.matric_potential_m_bits
                || input.storage_capacity_m.to_bits() != self.authority.storage_capacity_m_bits
            {
                return Err(DirectWb14ParentIntervalErrorV1::ChildSupport);
            }
            let transition = advance_wb14_continuation_interval(*input)
                .map_err(|_| DirectWb14ParentIntervalErrorV1::Arithmetic)?;
            expected_supply = transition.cumulative_supply_m;
            expected_infiltration = transition.cumulative_infiltration_m;
            interval_supply_m += input.interval_supply_m;
            interval_infiltration_m += transition.interval_infiltration_m;
            interval_excess_m += transition.interval_excess_m;
        }
        let outcome = DirectWb14ContinuationIntervalOutcome {
            cumulative_supply_m: expected_supply,
            cumulative_infiltration_m: expected_infiltration,
            interval_infiltration_m,
            interval_excess_m,
        };
        let transitions = inputs
            .iter()
            .copied()
            .map(DirectWb14ChildTransitionV1::from)
            .collect::<Vec<_>>();
        let mut transition_bytes = Vec::with_capacity(transitions.len() * 56);
        for transition in &transitions {
            for bits in [
                transition.cumulative_supply_m_bits,
                transition.cumulative_infiltration_m_bits,
                transition.interval_supply_m_bits,
                transition.interval_duration_s_bits,
                transition.effective_conductivity_m_s_bits,
                transition.matric_potential_m_bits,
                transition.storage_capacity_m_bits,
            ] {
                transition_bytes.extend_from_slice(&bits.to_be_bytes());
            }
        }
        let child_inputs_sha256 = wb14_hash(
            "openwepp-wb14-child-inputs-v1",
            &[
                FramedField {
                    tag: "ordered_transition_bits",
                    value: &transition_bytes,
                },
                FramedField {
                    tag: "proposed_upper_bound_s_bits",
                    value: &proposed_upper_bound_s.to_bits().to_be_bytes(),
                },
            ],
        )?;
        let beginning_digest = working_digest(self.working)?;
        let next_ordinal = ordinal
            .checked_add(1)
            .ok_or(DirectWb14ParentIntervalErrorV1::Arithmetic)?;
        let provisional_physical = DirectWb14ParentWorkingStateV1 {
            accepted_until_ns: support_end_ns,
            next_child_ordinal: next_ordinal,
            cumulative_supply_m: outcome.cumulative_supply_m,
            cumulative_infiltration_m: outcome.cumulative_infiltration_m,
            receipt_chain_sha256: self.working.receipt_chain_sha256,
        };
        let child_body_sha256 = wb14_hash(
            "openwepp-wb14-child-body-v1",
            &[
                FramedField {
                    tag: "parent_id",
                    value: &self.authority.parent_id,
                },
                FramedField {
                    tag: "ofe_id_sha256",
                    value: &self.authority.ofe_id_sha256,
                },
                FramedField {
                    tag: "production_lane_id",
                    value: &self.authority.production_lane_id.to_be_bytes(),
                },
                FramedField {
                    tag: "surface_liquid_configuration_sha256",
                    value: &self.authority.surface_liquid_configuration_sha256,
                },
                FramedField {
                    tag: "wb14_configuration_sha256",
                    value: &self.authority.wb14_configuration_sha256,
                },
                FramedField {
                    tag: "wb14_model_definition_sha256",
                    value: &self.authority.wb14_model_definition_sha256,
                },
                FramedField {
                    tag: "ordinal",
                    value: &ordinal.to_be_bytes(),
                },
                FramedField {
                    tag: "support_start_ns",
                    value: &support_start_ns.to_be_bytes(),
                },
                FramedField {
                    tag: "support_end_ns",
                    value: &support_end_ns.to_be_bytes(),
                },
                FramedField {
                    tag: "beginning_working_state_sha256",
                    value: &beginning_digest,
                },
                FramedField {
                    tag: "ending_cumulative_supply_m_bits",
                    value: &outcome.cumulative_supply_m.to_bits().to_be_bytes(),
                },
                FramedField {
                    tag: "ending_cumulative_infiltration_m_bits",
                    value: &outcome.cumulative_infiltration_m.to_bits().to_be_bytes(),
                },
                FramedField {
                    tag: "child_inputs_sha256",
                    value: &child_inputs_sha256,
                },
                FramedField {
                    tag: "accepted_coupled_slab_sha256",
                    value: &accepted_coupled_slab_sha256,
                },
                FramedField {
                    tag: "child_beginning_complete_owner_set_sha256",
                    value: &child_beginning_complete_owner_set_sha256,
                },
                FramedField {
                    tag: "pending_routed_parcels_before_sha256",
                    value: &pending_routed_parcels_before_sha256,
                },
                FramedField {
                    tag: "pending_routed_parcels_after_sha256",
                    value: &pending_routed_parcels_after_sha256,
                },
            ],
        )?;
        let ending_chain_sha256 = wb14_hash(
            "openwepp-wb14-child-chain-v1",
            &[
                FramedField {
                    tag: "predecessor_receipt_chain_sha256",
                    value: &predecessor_receipt_sha256,
                },
                FramedField {
                    tag: "child_body_sha256",
                    value: &child_body_sha256,
                },
            ],
        )?;
        let ending_working = DirectWb14ParentWorkingStateV1 {
            receipt_chain_sha256: ending_chain_sha256,
            ..provisional_physical
        };
        let ending_digest = working_digest(ending_working)?;
        let receipt_digest = wb14_hash(
            "openwepp-wb14-child-receipt-v1",
            &[
                FramedField {
                    tag: "child_body_sha256",
                    value: &child_body_sha256,
                },
                FramedField {
                    tag: "ending_working_state_sha256",
                    value: &ending_digest,
                },
                FramedField {
                    tag: "ending_receipt_chain_sha256",
                    value: &ending_chain_sha256,
                },
                FramedField {
                    tag: "parent_beginning_owner_sha256",
                    value: &self.authority.parent_beginning_owner_sha256,
                },
                FramedField {
                    tag: "parent_beginning_cursor_sha256",
                    value: &self.authority.beginning_cursor_sha256,
                },
            ],
        )?;
        let receipt = DirectWb14ChildReceiptV1 {
            parent_id: self.authority.parent_id,
            ofe_id_sha256: self.authority.ofe_id_sha256,
            production_lane_id: self.authority.production_lane_id,
            surface_liquid_configuration_sha256: self.authority.surface_liquid_configuration_sha256,
            wb14_configuration_sha256: self.authority.wb14_configuration_sha256,
            wb14_model_definition_sha256: self.authority.wb14_model_definition_sha256,
            effective_conductivity_m_s_bits: self.authority.effective_conductivity_m_s_bits,
            matric_potential_m_bits: self.authority.matric_potential_m_bits,
            storage_capacity_m_bits: self.authority.storage_capacity_m_bits,
            parent_beginning_owner_sha256: self.authority.parent_beginning_owner_sha256,
            parent_beginning_cursor_sha256: self.authority.beginning_cursor_sha256,
            ordinal,
            support_start_ns,
            support_end_ns,
            beginning_working_state_sha256: beginning_digest,
            ending_working_state_sha256: ending_digest,
            predecessor_receipt_sha256,
            accepted_coupled_slab_sha256,
            child_beginning_complete_owner_set_sha256,
            pending_routed_parcels_before_sha256,
            pending_routed_parcels_after_sha256,
            child_inputs_sha256,
            transitions,
            proposed_upper_bound_s_bits: proposed_upper_bound_s.to_bits(),
            accepted_duration_s_bits: selected_seconds.to_bits(),
            cumulative_supply_m_bits: self.working.cumulative_supply_m.to_bits(),
            cumulative_infiltration_m_bits: self.working.cumulative_infiltration_m.to_bits(),
            interval_supply_m_bits: interval_supply_m.to_bits(),
            interval_infiltration_m_bits: outcome.interval_infiltration_m.to_bits(),
            interval_excess_m_bits: outcome.interval_excess_m.to_bits(),
            receipt_sha256: receipt_digest,
        };
        let mut candidate = self.clone();
        candidate.working = ending_working;
        candidate.receipts.push(receipt);
        Ok((candidate, outcome))
    }

    pub(super) fn finalize(
        &self,
    ) -> Result<DirectWb14ParentFinalizationV1, DirectWb14ParentIntervalErrorV1> {
        self.validate()?;
        if self.working.accepted_until_ns != self.authority.support_end_ns
            || self.receipts.is_empty()
        {
            return Err(DirectWb14ParentIntervalErrorV1::ParentIncomplete);
        }
        let next = self
            .authority
            .parent_interval_index
            .checked_add(1)
            .ok_or(DirectWb14ParentIntervalErrorV1::Arithmetic)?;
        let persistent_cursor = DirectWb14PersistentCursorV1 {
            day_index: self.authority.parent_day_index,
            next_interval_index: next,
            cumulative_supply_m: self.working.cumulative_supply_m,
            cumulative_infiltration_m: self.working.cumulative_infiltration_m,
        };
        let ending_cursor_sha256 = cursor_digest(persistent_cursor)?;
        let ordered_child_receipt_sha256 = self
            .receipts
            .iter()
            .map(|receipt| receipt.receipt_sha256)
            .collect::<Vec<_>>();
        let mut ordered_children = Vec::with_capacity(ordered_child_receipt_sha256.len() * 32);
        for receipt in &ordered_child_receipt_sha256 {
            ordered_children.extend_from_slice(receipt);
        }
        let receipt_sha256 = wb14_hash(
            "openwepp-wb14-parent-receipt-v1",
            &[
                FramedField {
                    tag: "parent_id",
                    value: &self.authority.parent_id,
                },
                FramedField {
                    tag: "coupled_parent_transaction_sha256",
                    value: &self.authority.coupled_parent_transaction_sha256,
                },
                FramedField {
                    tag: "parent_beginning_owner_sha256",
                    value: &self.authority.parent_beginning_owner_sha256,
                },
                FramedField {
                    tag: "beginning_cursor_sha256",
                    value: &self.authority.beginning_cursor_sha256,
                },
                FramedField {
                    tag: "ending_cursor_sha256",
                    value: &ending_cursor_sha256,
                },
                FramedField {
                    tag: "support_start_ns",
                    value: &self.authority.support_start_ns.to_be_bytes(),
                },
                FramedField {
                    tag: "support_end_ns",
                    value: &self.authority.support_end_ns.to_be_bytes(),
                },
                FramedField {
                    tag: "ordered_child_receipts",
                    value: &ordered_children,
                },
                FramedField {
                    tag: "receipt_chain_sha256",
                    value: &self.working.receipt_chain_sha256,
                },
            ],
        )?;
        Ok(DirectWb14ParentFinalizationV1 {
            persistent_cursor,
            receipt: DirectWb14ParentReceiptV1 {
                parent_id: self.authority.parent_id,
                coupled_parent_transaction_sha256: self.authority.coupled_parent_transaction_sha256,
                parent_day_index: self.authority.parent_day_index,
                parent_interval_index: self.authority.parent_interval_index,
                parent_beginning_owner_sha256: self.authority.parent_beginning_owner_sha256,
                beginning_cursor_sha256: self.authority.beginning_cursor_sha256,
                ending_cursor_sha256,
                support_start_ns: self.authority.support_start_ns,
                support_end_ns: self.authority.support_end_ns,
                ordered_child_receipt_sha256,
                receipt_chain_sha256: self.working.receipt_chain_sha256,
                receipt_sha256,
            },
        })
    }

    /// Reconstruct every accepted transition from the immutable authority and
    /// compare all canonical receipt and working-state bytes.
    pub(super) fn validate(&self) -> Result<(), DirectWb14ParentIntervalErrorV1> {
        let mut reconstructed = Self::begin(self.authority, self.beginning_cursor)?;
        for receipt in &self.receipts {
            reconstructed = receipt.validate(&reconstructed)?;
        }
        if reconstructed.working != self.working || reconstructed.receipts != self.receipts {
            return Err(DirectWb14ParentIntervalErrorV1::ReceiptValidation);
        }
        Ok(())
    }

    pub(super) const fn working(&self) -> DirectWb14ParentWorkingStateV1 {
        self.working
    }

    pub(super) fn receipts(&self) -> &[DirectWb14ChildReceiptV1] {
        &self.receipts
    }

    pub(super) fn validate_coupled_child_binding(
        &self,
        coupled_parent_sha256: [u8; 32],
        parent_beginning_owner_sha256: [u8; 32],
        parent_support_start_ns: u128,
        parent_support_end_ns: u128,
        child_support_start_ns: u128,
        child_support_end_ns: u128,
        proposed_upper_bound_s_bits: u64,
        accepted_slab_sha256: [u8; 32],
        expected_ofe_id_sha256: [u8; 32],
    ) -> Result<(), DirectWb14ParentIntervalErrorV1> {
        self.validate()?;
        let child = self
            .receipts
            .last()
            .ok_or(DirectWb14ParentIntervalErrorV1::ReceiptValidation)?;
        if self.authority.coupled_parent_transaction_sha256 != coupled_parent_sha256
            || child.child_beginning_complete_owner_set_sha256 != parent_beginning_owner_sha256
            || self.authority.support_start_ns != parent_support_start_ns
            || self.authority.support_end_ns != parent_support_end_ns
            || child.support_start_ns != child_support_start_ns
            || child.support_end_ns != child_support_end_ns
            || child.proposed_upper_bound_s_bits != proposed_upper_bound_s_bits
            || child.accepted_coupled_slab_sha256 != accepted_slab_sha256
            || self.authority.ofe_id_sha256 != expected_ofe_id_sha256
        {
            return Err(DirectWb14ParentIntervalErrorV1::ReceiptValidation);
        }
        Ok(())
    }

    pub(super) fn validated_finalization(
        &self,
    ) -> Result<DirectWb14ParentFinalizationV1, DirectWb14ParentIntervalErrorV1> {
        self.finalize()
    }

    pub(super) fn validate_coordinator_binding(
        &self,
        expected_ofe_id_sha256: [u8; 32],
        production_lane_id: u32,
        surface_configuration_sha256: [u8; 32],
        wb14_configuration_sha256: [u8; 32],
        wb14_model_definition_sha256: [u8; 32],
        parameter_bits: (u64, u64, u64),
        support_start_ns: u128,
        support_end_ns: u128,
        accepted_until_ns: u128,
        beginning_cursor: DirectWb14PersistentCursorV1,
    ) -> Result<(), DirectWb14ParentIntervalErrorV1> {
        self.validate()?;
        if self.authority.ofe_id_sha256 != expected_ofe_id_sha256
            || self.authority.production_lane_id != production_lane_id
            || self.authority.surface_liquid_configuration_sha256 != surface_configuration_sha256
            || self.authority.wb14_configuration_sha256 != wb14_configuration_sha256
            || self.authority.wb14_model_definition_sha256 != wb14_model_definition_sha256
            || (
                self.authority.effective_conductivity_m_s_bits,
                self.authority.matric_potential_m_bits,
                self.authority.storage_capacity_m_bits,
            ) != parameter_bits
            || self.authority.support_start_ns != support_start_ns
            || self.authority.support_end_ns != support_end_ns
            || self.working.accepted_until_ns != accepted_until_ns
            || self.beginning_cursor != beginning_cursor
        {
            return Err(DirectWb14ParentIntervalErrorV1::ReceiptValidation);
        }
        Ok(())
    }
}

#[allow(dead_code)]
impl DirectWb14ChildReceiptV1 {
    pub(super) fn validate(
        &self,
        beginning: &DirectWb14ParentIntervalV1,
    ) -> Result<DirectWb14ParentIntervalV1, DirectWb14ParentIntervalErrorV1> {
        let inputs = self
            .transitions
            .iter()
            .copied()
            .map(DirectWb14ChildTransitionV1::inputs)
            .collect::<Vec<_>>();
        let (ending, _) = beginning.accept_child_transitions_with_slab(
            self.ordinal,
            self.support_start_ns,
            self.support_end_ns,
            self.predecessor_receipt_sha256,
            self.accepted_coupled_slab_sha256,
            self.child_beginning_complete_owner_set_sha256,
            self.pending_routed_parcels_before_sha256,
            self.pending_routed_parcels_after_sha256,
            f64::from_bits(self.proposed_upper_bound_s_bits),
            &inputs,
        )?;
        if ending.receipts.last() != Some(self) {
            return Err(DirectWb14ParentIntervalErrorV1::ReceiptValidation);
        }
        Ok(ending)
    }
}

#[allow(dead_code)]
impl DirectWb14ParentReceiptV1 {
    pub(super) fn validate(
        &self,
        parent: &DirectWb14ParentIntervalV1,
    ) -> Result<(), DirectWb14ParentIntervalErrorV1> {
        parent.validate()?;
        let rebuilt = parent.finalize()?.receipt;
        if rebuilt != *self {
            return Err(DirectWb14ParentIntervalErrorV1::ReceiptValidation);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direct_runtime::runoff::{
        DC01_HOUR_BIN_COUNT, DirectWb14HyetographInterval, DirectWb14InfiltrationProducerInputs,
        compute_wb14_infiltration_depression_with_profile,
    };

    fn identity_fixture() -> DirectWb14ImmutableIdentityV1 {
        DirectWb14ImmutableIdentityV1 {
            schema_sha256: [41; 32],
            ofe_id_sha256: [12; 32],
            production_lane_id: 4,
            surface_liquid_configuration_sha256: [42; 32],
            wb14_configuration_sha256: [43; 32],
            wb14_model_definition_sha256: [44; 32],
            effective_conductivity_m_s_bits: 1.1e-7_f64.to_bits(),
            matric_potential_m_bits: 0.12_f64.to_bits(),
            storage_capacity_m_bits: 0.02_f64.to_bits(),
        }
    }

    fn parent_fixture() -> DirectWb14ParentIntervalV1 {
        let cursor = DirectWb14PersistentCursorV1 {
            day_index: 3,
            next_interval_index: 7,
            cumulative_supply_m: 0.002,
            cumulative_infiltration_m: 0.001,
        };
        DirectWb14ParentIntervalV1::begin(
            wb14_parent_authority_v1(
                [29; 32],
                10_000_000_000_000,
                11_800_000_000_000,
                [17; 32],
                cursor,
                identity_fixture(),
            )
            .expect("authority"),
            cursor,
        )
        .expect("valid parent")
    }

    fn child_inputs(
        parent: &DirectWb14ParentIntervalV1,
        duration_s: f64,
        supply_m: f64,
    ) -> DirectWb14ContinuationIntervalInputs {
        DirectWb14ContinuationIntervalInputs {
            cumulative_supply_m: parent.working().cumulative_supply_m,
            cumulative_infiltration_m: parent.working().cumulative_infiltration_m,
            interval_supply_m: supply_m,
            interval_duration_s: duration_s,
            effective_conductivity_m_s: 1.1e-7,
            matric_potential_m: 0.12,
            storage_capacity_m: 0.02,
        }
    }

    fn accept_duration(
        parent: &DirectWb14ParentIntervalV1,
        duration_s: u64,
        supply_m: f64,
    ) -> DirectWb14ParentIntervalV1 {
        let start = parent.working().accepted_until_ns;
        let end = start + u128::from(duration_s) * 1_000_000_000;
        parent
            .accept_child(
                parent.working().next_child_ordinal,
                start,
                end,
                parent.working().receipt_chain_sha256,
                duration_s as f64,
                child_inputs(parent, duration_s as f64, supply_m),
            )
            .expect("accepted child")
            .0
    }

    #[test]
    fn one_parent_child_is_bitwise_identical_to_historical_interval_transition() {
        let beginning = parent_fixture();
        let inputs = child_inputs(&beginning, 1_800.0, 0.003_6);
        let historical = advance_wb14_continuation_interval(inputs).expect("historical path");
        let (ending, child) = beginning
            .accept_child(
                0,
                10_000_000_000_000,
                11_800_000_000_000,
                beginning.working().receipt_chain_sha256,
                1_800.0,
                inputs,
            )
            .expect("one child parent");
        assert_eq!(child, historical);
        let committed = ending.finalize().expect("complete parent");
        assert_eq!(committed.persistent_cursor.next_interval_index, 8);
        assert_eq!(
            committed.persistent_cursor.cumulative_supply_m.to_bits(),
            historical.cumulative_supply_m.to_bits()
        );
        assert_eq!(
            committed
                .persistent_cursor
                .cumulative_infiltration_m
                .to_bits(),
            historical.cumulative_infiltration_m.to_bits()
        );
    }

    #[test]
    fn two_900_and_thirty_60_children_cover_one_parent_and_advance_once() {
        for (count, duration) in [(2, 900), (30, 60)] {
            let beginning = parent_fixture();
            let mut parent = beginning.clone();
            for _ in 0..count {
                parent = accept_duration(&parent, duration, 0.000_12);
                assert_eq!(beginning.beginning_cursor.next_interval_index, 7);
            }
            assert_eq!(parent.receipts().len(), count);
            let supplied = parent
                .receipts()
                .iter()
                .map(|receipt| f64::from_bits(receipt.interval_supply_m_bits))
                .sum::<f64>();
            let infiltrated = parent
                .receipts()
                .iter()
                .map(|receipt| f64::from_bits(receipt.interval_infiltration_m_bits))
                .sum::<f64>();
            let excess = parent
                .receipts()
                .iter()
                .map(|receipt| f64::from_bits(receipt.interval_excess_m_bits))
                .sum::<f64>();
            assert!((infiltrated + excess - supplied).abs() <= 1.0e-14);
            assert!(
                (parent.working().cumulative_supply_m
                    - beginning.working().cumulative_supply_m
                    - supplied)
                    .abs()
                    <= 1.0e-14
            );
            assert_eq!(
                parent
                    .finalize()
                    .expect("complete")
                    .persistent_cursor
                    .next_interval_index,
                8
            );
        }
    }

    #[test]
    fn mixed_cadence_and_zero_supply_children_close_cumulatively() {
        let beginning = parent_fixture();
        let mut parent = accept_duration(&beginning, 900, 0.001_8);
        let after_positive = parent.working();
        for _ in 0..15 {
            parent = accept_duration(&parent, 60, 0.0);
        }
        assert_eq!(parent.receipts().len(), 16);
        assert_eq!(
            parent.working().cumulative_supply_m.to_bits(),
            after_positive.cumulative_supply_m.to_bits()
        );
        assert_eq!(
            parent.working().cumulative_infiltration_m.to_bits(),
            after_positive.cumulative_infiltration_m.to_bits()
        );
        let ending = parent.finalize().expect("mixed complete");
        assert_eq!(ending.persistent_cursor.next_interval_index, 8);
        assert_eq!(
            ending.persistent_cursor.cumulative_supply_m.to_bits(),
            0.003_8_f64.to_bits()
        );
        assert!(
            ending.persistent_cursor.cumulative_infiltration_m
                <= ending.persistent_cursor.cumulative_supply_m
        );
    }

    #[test]
    fn child_chronology_and_replay_poisons_preserve_parent_bytes() {
        let parent = parent_fixture();
        let original = parent.clone();
        let start = parent.working().accepted_until_ns;
        let chain = parent.working().receipt_chain_sha256;
        let inputs = child_inputs(&parent, 900.0, 0.001);
        for result in [
            parent.accept_child(1, start, start + 900_000_000_000, chain, 900.0, inputs),
            parent.accept_child(0, start + 1, start + 900_000_000_000, chain, 900.0, inputs),
            parent.accept_child(0, start, start + 899_000_000_000, chain, 900.0, inputs),
            parent.accept_child(0, start, start + 900_000_000_000, [9; 32], 900.0, inputs),
        ] {
            assert!(result.is_err());
            assert_eq!(parent, original);
        }
        let accepted = accept_duration(&parent, 900, 0.001);
        let accepted_start = accepted.working().accepted_until_ns;
        let accepted_chain = accepted.working().receipt_chain_sha256;
        let accepted_inputs = child_inputs(&accepted, 900.0, 0.001);
        for result in [
            accepted.accept_child(
                1,
                accepted_start + 60_000_000_000,
                accepted_start + 960_000_000_000,
                accepted_chain,
                900.0,
                accepted_inputs,
            ),
            accepted.accept_child(
                1,
                accepted_start - 60_000_000_000,
                accepted_start + 840_000_000_000,
                accepted_chain,
                900.0,
                accepted_inputs,
            ),
        ] {
            assert_eq!(result, Err(DirectWb14ParentIntervalErrorV1::ChildSupport));
        }
        let replay = accepted.accept_child(0, start, start + 900_000_000_000, chain, 900.0, inputs);
        assert!(replay.is_err());
        assert_eq!(parent, original);

        let cursor = parent.beginning_cursor;
        let substituted = DirectWb14ParentAuthorityV1 {
            parent_id: [9; 32],
            ..wb14_parent_authority_v1(
                parent.authority.coupled_parent_transaction_sha256,
                parent.authority.support_start_ns,
                parent.authority.support_end_ns,
                parent.authority.parent_beginning_owner_sha256,
                cursor,
                identity_fixture(),
            )
            .expect("authority")
        };
        assert_eq!(
            DirectWb14ParentIntervalV1::begin(substituted, cursor),
            Err(DirectWb14ParentIntervalErrorV1::CursorIdentity)
        );
    }

    #[test]
    fn failure_after_child_and_parent_finalization_are_atomic_candidates() {
        for fail_after in [1, 17] {
            let beginning = parent_fixture();
            let mut candidate = beginning.clone();
            for ordinal in 0..fail_after {
                candidate =
                    accept_duration(&candidate, if fail_after == 1 { 900 } else { 60 }, 0.000_1);
                assert_eq!(candidate.receipts().len(), ordinal + 1);
            }
            // Dropping the candidate is rollback: the authoritative beginning is untouched.
            assert_eq!(beginning, parent_fixture());
        }
        let incomplete = accept_duration(&parent_fixture(), 900, 0.001);
        assert_eq!(
            incomplete.finalize(),
            Err(DirectWb14ParentIntervalErrorV1::ParentIncomplete)
        );
        assert_eq!(parent_fixture(), parent_fixture());
    }

    #[test]
    fn day_rollover_selects_next_day_zero_and_advances_once() {
        let cursor = DirectWb14PersistentCursorV1 {
            day_index: 3,
            next_interval_index: 48,
            cumulative_supply_m: 0.02,
            cumulative_infiltration_m: 0.01,
        };
        let authority = wb14_parent_authority_v1(
            [31; 32],
            20_000_000_000_000,
            21_800_000_000_000,
            [37; 32],
            cursor,
            identity_fixture(),
        )
        .expect("rollover authority");
        assert_eq!(
            (authority.parent_day_index, authority.parent_interval_index),
            (4, 0)
        );
        let parent = DirectWb14ParentIntervalV1::begin(authority, cursor).expect("rollover parent");
        assert_eq!(
            parent.working().cumulative_supply_m.to_bits(),
            0.0_f64.to_bits()
        );
        let ending = accept_duration(&parent, 1_800, 0.0)
            .finalize()
            .expect("rollover finalization");
        assert_eq!(ending.persistent_cursor.day_index, 4);
        assert_eq!(ending.persistent_cursor.next_interval_index, 1);
    }

    fn independent_ponded_oracle(
        cumulative_m: f64,
        rainfall_m: f64,
        duration_s: f64,
        intensity_m_s: f64,
        conductivity_m_s: f64,
        matric_m: f64,
    ) -> f64 {
        let threshold_m = conductivity_m_s * matric_m / (intensity_m_s - conductivity_m_s);
        let unponded_m = (threshold_m - cumulative_m).clamp(0.0, rainfall_m);
        let target_m = conductivity_m_s * (duration_s - unponded_m / intensity_m_s);
        let start_m = cumulative_m + unponded_m;
        let residual = |end_m: f64| {
            (end_m - start_m)
                - matric_m * ((end_m + matric_m) / (start_m + matric_m)).ln()
                - target_m
        };
        let mut low_m = start_m;
        let mut high_m = start_m + rainfall_m + matric_m;
        for _ in 0..160 {
            let mid_m = 0.5 * (low_m + high_m);
            if residual(mid_m) < 0.0 {
                low_m = mid_m;
            } else {
                high_m = mid_m;
            }
        }
        (unponded_m + 0.5 * (low_m + high_m) - start_m).min(rainfall_m)
    }

    fn assert_core_continuation_parity(
        inputs: DirectWb14ContinuationIntervalInputs,
    ) -> Result<DirectWb14ContinuationIntervalOutcome, DirectRuntimeError> {
        let direct = advance_wb14_interval_state(DirectWb14IntervalTransitionInputs {
            cumulative_supply_m: inputs.cumulative_supply_m,
            cumulative_infiltration_m: inputs.cumulative_infiltration_m,
            interval_supply_m: inputs.interval_supply_m,
            interval_duration_s: inputs.interval_duration_s,
            interval_intensity_m_s: inputs.interval_supply_m / inputs.interval_duration_s,
            effective_conductivity_m_s: inputs.effective_conductivity_m_s,
            matric_potential_m: inputs.matric_potential_m,
            storage_capacity_m: inputs.storage_capacity_m,
        });
        let continuation = advance_wb14_continuation_interval(inputs);
        assert_eq!(continuation, direct);
        continuation
    }

    fn assert_daily_continuation_parity(
        interval_supplies_m: &[f64],
        storage_capacity_m: f64,
        effective_conductivity_m_s: f64,
    ) -> DirectWb14ContinuationIntervalOutcome {
        let mut hyetograph = Vec::with_capacity(interval_supplies_m.len());
        for (index, supply_m) in interval_supplies_m.iter().copied().enumerate() {
            let start_s = f64::from(u32::try_from(index).expect("small test index")) * 1_800.0;
            hyetograph.push(DirectWb14HyetographInterval {
                start_s,
                end_s: start_s + 1_800.0,
                intensity_m_s: supply_m / 1_800.0,
            });
        }
        let inputs = DirectWb14InfiltrationProducerInputs {
            hyetograph,
            hourly_additional_supply_m: [0.0; DC01_HOUR_BIN_COUNT],
            effective_conductivity_m_s,
            matric_potential_m: 0.12,
            storage_capacity_m,
            depression_storage_capacity_m: 0.0,
        };
        let daily = compute_wb14_infiltration_depression_with_profile(&inputs)
            .expect("daily production wrapper");
        let mut outcome = DirectWb14ContinuationIntervalOutcome {
            cumulative_supply_m: 0.0,
            cumulative_infiltration_m: 0.0,
            interval_infiltration_m: 0.0,
            interval_excess_m: 0.0,
        };
        let mut continuation_excess_m = 0.0;
        for interval in &inputs.hyetograph {
            let interval_supply_m = interval.intensity_m_s * 1_800.0;
            outcome = advance_wb14_continuation_interval(DirectWb14ContinuationIntervalInputs {
                cumulative_supply_m: outcome.cumulative_supply_m,
                cumulative_infiltration_m: outcome.cumulative_infiltration_m,
                interval_supply_m,
                interval_duration_s: 1_800.0,
                effective_conductivity_m_s: inputs.effective_conductivity_m_s,
                matric_potential_m: inputs.matric_potential_m,
                storage_capacity_m: inputs.storage_capacity_m,
            })
            .expect("stateful interval");
            continuation_excess_m += outcome.interval_excess_m;
        }
        assert_eq!(
            outcome.cumulative_infiltration_m.to_bits(),
            daily.state.cumulative_infiltration_m.to_bits()
        );
        let daily_excess_m = daily.hourly_excess_m.iter().sum::<f64>();
        assert!((continuation_excess_m - daily_excess_m).abs() <= 1.0e-12);
        outcome
    }

    #[test]
    fn forty_eight_stateful_intervals_match_the_existing_daily_wb14_wrapper() {
        let outcome = assert_daily_continuation_parity(&[0.000_45; 48], 0.015, 1.1e-7);
        assert!(outcome.cumulative_infiltration_m <= 0.015);
    }

    #[test]
    fn zero_threshold_branch_matches_daily_wrapper() {
        let outcome = assert_daily_continuation_parity(&[0.000_45], WB11_ZERO_THRESHOLD, 1.1e-3);
        assert_eq!(outcome.interval_infiltration_m.to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            outcome.interval_excess_m.to_bits(),
            outcome.cumulative_supply_m.to_bits()
        );
        assert_core_continuation_parity(DirectWb14ContinuationIntervalInputs {
            cumulative_supply_m: 0.001,
            cumulative_infiltration_m: 0.001,
            interval_supply_m: 0.000_45,
            interval_duration_s: 1_800.0,
            effective_conductivity_m_s: 1.1e-3,
            matric_potential_m: 0.12,
            storage_capacity_m: 0.001 + WB11_ZERO_THRESHOLD,
        })
        .expect("remaining-storage threshold branch");
    }

    #[test]
    fn storage_capacity_clamp_branch_matches_daily_wrapper() {
        let capacity_m = 0.000_8;
        let outcome = assert_daily_continuation_parity(&[0.000_45; 4], capacity_m, 1.1e-3);
        assert_eq!(
            outcome.cumulative_infiltration_m.to_bits(),
            capacity_m.to_bits()
        );
        assert!(outcome.interval_excess_m > 0.0);
    }

    #[test]
    fn roundoff_clamp_branch_matches_daily_wrapper() {
        let capacity_m = 0.1 + 0.2;
        let outcome =
            assert_daily_continuation_parity(&[0.1, 0.1, 0.1, 1.0e-8], capacity_m, 1.1e-3);
        assert_eq!(
            outcome.cumulative_infiltration_m.to_bits(),
            capacity_m.to_bits()
        );
        assert!(outcome.interval_excess_m >= 0.0);
    }

    #[test]
    fn beginning_bounds_guards_match_the_shared_transition() {
        let above_supply = DirectWb14ContinuationIntervalInputs {
            cumulative_supply_m: 0.001,
            cumulative_infiltration_m: 0.001_000_000_000_1,
            interval_supply_m: 0.000_45,
            interval_duration_s: 1_800.0,
            effective_conductivity_m_s: 1.1e-3,
            matric_potential_m: 0.12,
            storage_capacity_m: 0.002,
        };
        assert!(assert_core_continuation_parity(above_supply).is_err());

        let above_storage = DirectWb14ContinuationIntervalInputs {
            cumulative_supply_m: 0.002,
            cumulative_infiltration_m: 0.001_000_000_000_1,
            storage_capacity_m: 0.001,
            ..above_supply
        };
        assert!(assert_core_continuation_parity(above_storage).is_err());
    }

    #[test]
    fn accepted_support_may_be_truncated_below_the_selected_upper_bound() {
        let beginning = parent_fixture();
        let start = beginning.working().accepted_until_ns;
        let duration_s = 437.0;
        let (ending, _) = beginning
            .accept_child(
                0,
                start,
                start + 437_000_000_000,
                beginning.working().receipt_chain_sha256,
                900.0,
                child_inputs(&beginning, duration_s, 0.000_7),
            )
            .expect("event-truncated accepted support");
        assert_eq!(ending.working().accepted_until_ns, start + 437_000_000_000);
        assert_eq!(
            ending.receipts()[0].proposed_upper_bound_s_bits,
            900.0_f64.to_bits()
        );
        assert_eq!(
            ending.receipts()[0].accepted_duration_s_bits,
            duration_s.to_bits()
        );
        ending
            .validate()
            .expect("independently reconstructed child");

        for (proposal, end) in [
            (437.0, start + 437_000_000_000),
            (60.0, start + 61_000_000_000),
        ] {
            assert_eq!(
                beginning.accept_child(
                    0,
                    start,
                    end,
                    beginning.working().receipt_chain_sha256,
                    proposal,
                    child_inputs(&beginning, (end - start) as f64 / 1.0e9, 0.000_1),
                ),
                Err(DirectWb14ParentIntervalErrorV1::ChildCadence)
            );
        }
    }

    #[test]
    fn dense_decimal_transition_partition_uses_exact_nanosecond_support() {
        let beginning = parent_fixture();
        let start = beginning.working().accepted_until_ns;
        let template = child_inputs(&beginning, 0.1, 0.0);
        let transitions = vec![template; 18_000];
        let (ending, outcome) = beginning
            .accept_child_transitions(
                0,
                start,
                start + 1_800_000_000_000,
                beginning.working().receipt_chain_sha256,
                1_800.0,
                &transitions,
            )
            .expect("dense exact-nanosecond transition partition");
        assert_eq!(
            ending.working().accepted_until_ns,
            start + 1_800_000_000_000
        );
        assert_eq!(outcome.interval_infiltration_m.to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            ending.receipts()[0].accepted_duration_s_bits,
            1_800.0_f64.to_bits()
        );
        ending.validate().expect("dense partition receipt replay");
    }

    #[test]
    fn endpoint_subtraction_partition_joins_exact_outer_support() {
        let beginning = parent_fixture();
        let start = beginning.working().accepted_until_ns;
        let mut first = child_inputs(&beginning, 0.3 - 0.2, 0.0);
        first.interval_duration_s = 0.3_f64 - 0.2_f64;
        let second = child_inputs(&beginning, 1_799.9, 0.0);
        let (ending, _) = beginning
            .accept_child_transitions(
                0,
                start,
                start + 1_800_000_000_000,
                beginning.working().receipt_chain_sha256,
                1_800.0,
                &[first, second],
            )
            .expect("binary64 endpoint-subtraction partition");
        assert_eq!(
            ending.working().accepted_until_ns,
            start + 1_800_000_000_000
        );
        ending
            .validate()
            .expect("endpoint-subtraction receipt replay");

        let altered = child_inputs(&beginning, 1_799.900_000_000_25, 0.0);
        assert_eq!(
            beginning.accept_child_transitions(
                0,
                start,
                start + 1_800_000_000_000,
                beginning.working().receipt_chain_sha256,
                1_800.0,
                &[first, altered],
            ),
            Err(DirectWb14ParentIntervalErrorV1::ChildSupport),
            "sub-nanosecond transition alteration must not round into exact support",
        );
    }

    #[test]
    fn immutable_identity_parameter_and_beginning_state_poisons_reject() {
        let valid = parent_fixture();
        let cursor = valid.beginning_cursor;
        for (coupled, owner) in [([0; 32], [17; 32]), ([29; 32], [0; 32])] {
            let authority = wb14_parent_authority_v1(
                coupled,
                valid.authority.support_start_ns,
                valid.authority.support_end_ns,
                owner,
                cursor,
                identity_fixture(),
            )
            .expect("zero identity authority framing");
            assert_eq!(
                DirectWb14ParentIntervalV1::begin(authority, cursor),
                Err(DirectWb14ParentIntervalErrorV1::ImmutableIdentity)
            );
        }
        for digest_poison in 0..6 {
            let mut identity = identity_fixture();
            match digest_poison {
                0 => identity.schema_sha256 = [0; 32],
                1 => identity.surface_liquid_configuration_sha256 = [0; 32],
                2 => identity.wb14_configuration_sha256 = [0; 32],
                3 => identity.wb14_model_definition_sha256 = [0; 32],
                4 => identity.wb14_configuration_sha256 = [99; 32],
                _ => identity.ofe_id_sha256[0] ^= 1,
            }
            let authority = wb14_parent_authority_v1(
                valid.authority.coupled_parent_transaction_sha256,
                valid.authority.support_start_ns,
                valid.authority.support_end_ns,
                valid.authority.parent_beginning_owner_sha256,
                cursor,
                identity,
            )
            .expect("framed authority");
            if digest_poison < 4 {
                assert!(DirectWb14ParentIntervalV1::begin(authority, cursor).is_err());
            } else {
                assert_ne!(authority.parent_id, valid.authority.parent_id);
            }
        }

        for poisoned in [
            DirectWb14PersistentCursorV1 {
                cumulative_supply_m: f64::NAN,
                ..cursor
            },
            DirectWb14PersistentCursorV1 {
                cumulative_infiltration_m: f64::INFINITY,
                ..cursor
            },
            DirectWb14PersistentCursorV1 {
                cumulative_supply_m: -0.1,
                ..cursor
            },
            DirectWb14PersistentCursorV1 {
                cumulative_infiltration_m: -0.1,
                ..cursor
            },
            DirectWb14PersistentCursorV1 {
                cumulative_supply_m: 0.0,
                cumulative_infiltration_m: 0.1,
                ..cursor
            },
        ] {
            let authority = wb14_parent_authority_v1(
                [29; 32],
                valid.authority.support_start_ns,
                valid.authority.support_end_ns,
                [17; 32],
                poisoned,
                identity_fixture(),
            )
            .expect("poison authority framing");
            assert_eq!(
                DirectWb14ParentIntervalV1::begin(authority, poisoned),
                Err(DirectWb14ParentIntervalErrorV1::BeginningState)
            );
        }

        let start = valid.working().accepted_until_ns;
        for inputs in [
            DirectWb14ContinuationIntervalInputs {
                effective_conductivity_m_s: 2.2e-7,
                ..child_inputs(&valid, 900.0, 0.001)
            },
            DirectWb14ContinuationIntervalInputs {
                matric_potential_m: 0.13,
                ..child_inputs(&valid, 900.0, 0.001)
            },
            DirectWb14ContinuationIntervalInputs {
                storage_capacity_m: 0.021,
                ..child_inputs(&valid, 900.0, 0.001)
            },
        ] {
            assert_eq!(
                valid.accept_child(
                    0,
                    start,
                    start + 900_000_000_000,
                    valid.working().receipt_chain_sha256,
                    900.0,
                    inputs
                ),
                Err(DirectWb14ParentIntervalErrorV1::ChildSupport)
            );
        }
    }

    #[test]
    fn child_and_parent_receipts_validate_and_detect_substitution() {
        let ending = accept_duration(&parent_fixture(), 1_800, 0.001);
        ending.validate().expect("complete reconstructed chain");
        let finalized = ending.finalize().expect("complete parent");
        finalized
            .receipt
            .validate(&ending)
            .expect("parent receipt reconstruction");

        let mut poisoned = ending.clone();
        poisoned.receipts[0].interval_supply_m_bits = 0.002_f64.to_bits();
        assert_eq!(
            poisoned.validate(),
            Err(DirectWb14ParentIntervalErrorV1::ReceiptValidation)
        );
        let mut queue_poisoned = ending.clone();
        queue_poisoned.receipts[0].pending_routed_parcels_after_sha256 = [91; 32];
        assert_eq!(
            queue_poisoned.validate(),
            Err(DirectWb14ParentIntervalErrorV1::ReceiptValidation)
        );
        let mut parent_receipt = finalized.receipt;
        parent_receipt.receipt_chain_sha256 = [77; 32];
        assert_eq!(
            parent_receipt.validate(&ending),
            Err(DirectWb14ParentIntervalErrorV1::ReceiptValidation)
        );
    }

    #[test]
    fn variable_duration_shared_kernel_matches_independent_nonlinear_oracle() {
        let cumulative_m = 0.001_7;
        let intensity_m_s = 8.0e-6;
        let conductivity_m_s = 1.1e-7;
        let matric_m = 0.12;
        for duration_s in [75.0, 437.0, 1_125.0, 1_800.0] {
            let rainfall_m = intensity_m_s * duration_s;
            let actual = super::super::runoff::compute_green_ampt_interval_infiltration(
                cumulative_m,
                rainfall_m,
                duration_s,
                intensity_m_s,
                conductivity_m_s,
                matric_m,
            )
            .expect("shared Green-Ampt transition");
            let expected = independent_ponded_oracle(
                cumulative_m,
                rainfall_m,
                duration_s,
                intensity_m_s,
                conductivity_m_s,
                matric_m,
            );
            assert!(
                (actual - expected).abs() <= 1.0e-12,
                "duration={duration_s}"
            );
        }
    }

    #[test]
    fn nonlinear_partial_transition_is_not_proportional_full_bin_scaling() {
        let cumulative_m = 0.001_7;
        let intensity_m_s = 8.0e-6;
        let conductivity_m_s = 1.1e-7;
        let matric_m = 0.12;
        let partial_duration_s = 437.0;
        let partial = super::super::runoff::compute_green_ampt_interval_infiltration(
            cumulative_m,
            intensity_m_s * partial_duration_s,
            partial_duration_s,
            intensity_m_s,
            conductivity_m_s,
            matric_m,
        )
        .expect("partial transition");
        let full = super::super::runoff::compute_green_ampt_interval_infiltration(
            cumulative_m,
            intensity_m_s * 1_800.0,
            1_800.0,
            intensity_m_s,
            conductivity_m_s,
            matric_m,
        )
        .expect("full transition");
        let naive = full * partial_duration_s / 1_800.0;
        assert!((partial - naive).abs() > 1.0e-8);

        let outcome = advance_wb14_interval_state(DirectWb14IntervalTransitionInputs {
            cumulative_supply_m: 0.002_4,
            cumulative_infiltration_m: cumulative_m,
            interval_supply_m: intensity_m_s * partial_duration_s,
            interval_duration_s: partial_duration_s,
            interval_intensity_m_s: intensity_m_s,
            effective_conductivity_m_s: conductivity_m_s,
            matric_potential_m: matric_m,
            storage_capacity_m: 0.02,
        })
        .expect("partial shared wrapper with beginning storage state");
        assert!((outcome.interval_infiltration_m - partial).abs() <= 1.0e-12);
        assert_eq!(
            outcome.cumulative_supply_m,
            0.002_4 + intensity_m_s * partial_duration_s
        );
        assert!(
            (outcome.interval_infiltration_m + outcome.interval_excess_m
                - intensity_m_s * partial_duration_s)
                .abs()
                <= 1.0e-14
        );
    }
}
