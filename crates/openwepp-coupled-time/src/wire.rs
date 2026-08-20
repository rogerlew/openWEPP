use crate::{
    AcceptedSlabId, CoupledTimeError, Digest32, EventClass, EventId, FramedField, ModelTimeNs,
    OutboxState, ParentIntervalId, ParentTransactionId, PublicationRecordV1, ReceiptId, SegmentId,
    TimeSupport, digest_bytes, framed_sha256,
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RestartWireV2 {
    pub schema: String,
    pub version: u32,
    pub authority_sha256: Digest32,
    pub model_definition_sha256: Digest32,
    pub run_identity_sha256: Digest32,
    pub calendar_receipt_sha256: Digest32,
    pub forcing_receipt_sha256: Digest32,
    pub parent_interval_id: ParentIntervalId,
    pub checkpoint_phase: String,
    pub parent_transaction_sequence: String,
    pub next_parent_transaction_sequence: String,
    pub parent_transaction_id: ParentTransactionId,
    pub parent_support: TimeSupport,
    pub accepted_until_ns: ModelTimeNs,
    pub next_segment_ordinal: u32,
    pub next_slab_ordinal: u32,
    pub next_event_ordinal: u32,
    pub last_accepted_step_ns: String,
    pub active_segment: SegmentWire,
    pub complete_owner_state: Vec<OwnerWire>,
    pub begin_complete_owner_set_sha256: Digest32,
    pub begin_clock_sha256: Digest32,
    pub accepted_complete_owner_set_sha256: Digest32,
    pub boundary_modes: Vec<NamedDigestWire>,
    pub constraint_policy_sha256: Digest32,
    pub controller_policy_sha256: Digest32,
    pub accepted_controller_checkpoint: BlobWire,
    pub accepted_slab_receipts: Vec<SlabReceiptWire>,
    pub accepted_event_receipts: Vec<EventReceiptWire>,
    pub scheduled_once_receipts: Vec<ScheduledReceiptWire>,
    pub reduction_state: Vec<ReductionWire>,
    pub pending_publication_buffer: Vec<PublicationRecordWire>,
    pub publication_outbox: Vec<OutboxWire>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SegmentWire {
    pub segment_id: SegmentId,
    pub ordinal: u32,
    pub regime_id: String,
    pub start_ns: ModelTimeNs,
    pub end_ns: ModelTimeNs,
    pub active_participants: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OwnerWire {
    pub owner_id: String,
    pub state_encoding: String,
    pub state_bytes_sha256: Digest32,
    pub state_bytes_base64: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NamedDigestWire {
    pub id: String,
    pub sha256: Digest32,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BlobWire {
    pub encoding: String,
    pub bytes_sha256: Digest32,
    pub bytes_base64: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SlabReceiptWire {
    pub receipt_id: ReceiptId,
    pub accepted_slab_id: AcceptedSlabId,
    pub parent_transaction_id: ParentTransactionId,
    pub slab_ordinal: u32,
    pub segment_id: SegmentId,
    pub support: TimeSupport,
    pub duration_bits: String,
    pub constraint_digest: Digest32,
    pub begin_clock_sha256: Digest32,
    pub end_clock_sha256: Digest32,
    pub begin_owner_set_sha256: Digest32,
    pub end_owner_set_sha256: Digest32,
    pub owner_candidate_set_sha256: Digest32,
    pub coupled_ledger_sha256: Digest32,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EventReceiptWire {
    pub receipt_id: ReceiptId,
    pub event_id: EventId,
    pub parent_transaction_id: ParentTransactionId,
    pub tick_ns: ModelTimeNs,
    pub event_ordinal: u32,
    pub class: EventClass,
    pub source_owner_id: String,
    pub begin_clock_sha256: Digest32,
    pub end_clock_sha256: Digest32,
    pub begin_owner_set_sha256: Digest32,
    pub end_owner_set_sha256: Digest32,
    pub event_context_sha256: Digest32,
    pub ledger_digest: Digest32,
    pub ledgers: Vec<serde_json::Value>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ScheduledReceiptWire {
    pub receipt_id: ReceiptId,
    pub operation_id: String,
    pub boundary_id: Digest32,
    pub tick_ns: ModelTimeNs,
    pub result_sha256: Digest32,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReductionWire {
    pub reduction_id: String,
    pub operator: String,
    pub units: String,
    pub value_bits: Option<String>,
    pub accepted_operand_receipt_ids: Vec<ReceiptId>,
    pub accepted_operand_values: Vec<ReductionOperandWire>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReductionOperandWire {
    pub receipt_id: ReceiptId,
    pub value_bits: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PublicationRecordWire {
    pub record_id: ReceiptId,
    pub source_owner_id: String,
    pub accepted_receipt_id: ReceiptId,
    pub support: TimeSupport,
    pub units: String,
    pub value_encoding: String,
    pub value_bytes_base64: String,
    pub value_sha256: Digest32,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OutboxWire {
    pub publication_receipt_id: ReceiptId,
    pub parent_receipt_id: ReceiptId,
    pub outbox_sequence: String,
    pub state: OutboxState,
    pub records: Vec<PublicationRecordWire>,
    pub records_sha256: Digest32,
    pub delivery_attempt_count: String,
}

pub(crate) fn blob(bytes: &[u8]) -> BlobWire {
    BlobWire {
        encoding: "canonical-binary".into(),
        bytes_sha256: digest_bytes(bytes),
        bytes_base64: STANDARD.encode(bytes),
    }
}
pub(crate) fn decode_blob(blob: &BlobWire) -> Result<Vec<u8>, CoupledTimeError> {
    if blob.encoding != "canonical-binary" {
        return Err(CoupledTimeError::RestartInvalid);
    }
    let bytes = STANDARD
        .decode(&blob.bytes_base64)
        .map_err(|_| CoupledTimeError::RestartInvalid)?;
    if digest_bytes(&bytes) != blob.bytes_sha256 {
        return Err(CoupledTimeError::RestartInvalid);
    }
    Ok(bytes)
}
pub(crate) fn parse_u128(text: &str) -> Result<u128, CoupledTimeError> {
    if text.is_empty()
        || text != "0" && text.starts_with('0')
        || !text.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(CoupledTimeError::RestartInvalid);
    }
    text.parse().map_err(|_| CoupledTimeError::RestartInvalid)
}
pub(crate) fn record_wire(r: &PublicationRecordV1) -> PublicationRecordWire {
    PublicationRecordWire {
        record_id: r.record_id,
        source_owner_id: r.source_owner_id.clone(),
        accepted_receipt_id: r.accepted_receipt_id,
        support: r.support,
        units: r.units.clone(),
        value_encoding: "canonical-binary".into(),
        value_bytes_base64: STANDARD.encode(&r.payload),
        value_sha256: r.value_digest,
    }
}
pub(crate) fn record_from_wire(
    w: PublicationRecordWire,
) -> Result<PublicationRecordV1, CoupledTimeError> {
    let payload = STANDARD
        .decode(w.value_bytes_base64)
        .map_err(|_| CoupledTimeError::RestartInvalid)?;
    if digest_bytes(&payload) != w.value_sha256 {
        return Err(CoupledTimeError::RestartInvalid);
    }
    let expected = super::restart::derive_publication_record_id(
        w.accepted_receipt_id,
        w.support,
        w.value_sha256,
        &w.units,
        &w.source_owner_id,
    )?;
    if expected != w.record_id {
        return Err(CoupledTimeError::RestartInvalid);
    }
    Ok(PublicationRecordV1 {
        record_id: w.record_id,
        accepted_receipt_id: w.accepted_receipt_id,
        support: w.support,
        value_digest: w.value_sha256,
        payload,
        units: w.units,
        source_owner_id: w.source_owner_id,
    })
}

fn event_class_wire(class: EventClass) -> &'static str {
    match class {
        EventClass::OwnershipTransfer => "OwnershipTransfer",
        EventClass::BoundaryModeTransition => "BoundaryModeTransition",
        EventClass::RegimeTransition => "RegimeTransition",
        EventClass::ScheduledBoundary => "ScheduledBoundary",
        EventClass::DiagnosticMarker => "DiagnosticMarker",
    }
}

/// Reconstruct every retained identity and merge slabs/events at their boundary ticks.
#[allow(clippy::too_many_lines)]
pub(crate) fn validate_authenticated_chronology(w: &RestartWireV2) -> Result<(), CoupledTimeError> {
    if w.accepted_slab_receipts.windows(2).any(|x| {
        x[0].slab_ordinal.checked_add(1) != Some(x[1].slab_ordinal)
            || x[0].support.end_ns() != x[1].support.start_ns()
    }) || w
        .accepted_event_receipts
        .windows(2)
        .any(|x| x[0].event_ordinal.checked_add(1) != Some(x[1].event_ordinal))
    {
        return Err(CoupledTimeError::RestartInvalid);
    }
    if w.accepted_slab_receipts
        .first()
        .is_some_and(|first| first.support.start_ns() != w.parent_support.start_ns())
        || w.accepted_event_receipts.windows(2).any(|pair| {
            pair[0].tick_ns > pair[1].tick_ns
                || pair[0].tick_ns == pair[1].tick_ns
                    && (
                        pair[0].class,
                        pair[0].source_owner_id.as_str(),
                        pair[0].event_context_sha256,
                    ) > (
                        pair[1].class,
                        pair[1].source_owner_id.as_str(),
                        pair[1].event_context_sha256,
                    )
        })
    {
        return Err(CoupledTimeError::RestartInvalid);
    }
    for s in &w.accepted_slab_receipts {
        let ordinal = s.slab_ordinal.to_be_bytes();
        let start = s.support.start_ns().get().to_be_bytes();
        let end = s.support.end_ns().get().to_be_bytes();
        let duration = s.support.duration_s_bits().to_be_bytes();
        if s.parent_transaction_id != w.parent_transaction_id
            || s.duration_bits != format!("{:016x}", s.support.duration_s_bits())
        {
            return Err(CoupledTimeError::RestartInvalid);
        }
        let slab = framed_sha256(
            "accepted-slab",
            &[
                FramedField {
                    tag: "parent_transaction_id",
                    value: w.parent_transaction_id.digest().as_bytes(),
                },
                FramedField {
                    tag: "slab_ordinal",
                    value: &ordinal,
                },
                FramedField {
                    tag: "segment_id",
                    value: s.segment_id.digest().as_bytes(),
                },
                FramedField {
                    tag: "start_ns",
                    value: &start,
                },
                FramedField {
                    tag: "end_ns",
                    value: &end,
                },
                FramedField {
                    tag: "duration_bits",
                    value: &duration,
                },
                FramedField {
                    tag: "begin_owner_set",
                    value: s.begin_owner_set_sha256.as_bytes(),
                },
                FramedField {
                    tag: "end_owner_set",
                    value: s.end_owner_set_sha256.as_bytes(),
                },
                FramedField {
                    tag: "constraint_digest",
                    value: s.constraint_digest.as_bytes(),
                },
                FramedField {
                    tag: "ledger_digest",
                    value: s.coupled_ledger_sha256.as_bytes(),
                },
            ],
        )?;
        if s.accepted_slab_id.digest() != slab {
            return Err(CoupledTimeError::RestartInvalid);
        }
        let receipt = framed_sha256(
            "slab-receipt-v2",
            &[
                FramedField {
                    tag: "parent_transaction_id",
                    value: w.parent_transaction_id.digest().as_bytes(),
                },
                FramedField {
                    tag: "accepted_slab_id",
                    value: slab.as_bytes(),
                },
                FramedField {
                    tag: "slab_ordinal",
                    value: &ordinal,
                },
                FramedField {
                    tag: "segment_id",
                    value: s.segment_id.digest().as_bytes(),
                },
                FramedField {
                    tag: "start_ns",
                    value: &start,
                },
                FramedField {
                    tag: "end_ns",
                    value: &end,
                },
                FramedField {
                    tag: "duration_bits",
                    value: &duration,
                },
                FramedField {
                    tag: "constraint_digest",
                    value: s.constraint_digest.as_bytes(),
                },
                FramedField {
                    tag: "begin_clock",
                    value: s.begin_clock_sha256.as_bytes(),
                },
                FramedField {
                    tag: "end_clock",
                    value: s.end_clock_sha256.as_bytes(),
                },
                FramedField {
                    tag: "begin_owner_set",
                    value: s.begin_owner_set_sha256.as_bytes(),
                },
                FramedField {
                    tag: "end_owner_set",
                    value: s.end_owner_set_sha256.as_bytes(),
                },
                FramedField {
                    tag: "owner_candidate_set",
                    value: s.owner_candidate_set_sha256.as_bytes(),
                },
                FramedField {
                    tag: "ledger_digest",
                    value: s.coupled_ledger_sha256.as_bytes(),
                },
            ],
        )?;
        if s.receipt_id.digest() != receipt {
            return Err(CoupledTimeError::RestartInvalid);
        }
    }
    for e in &w.accepted_event_receipts {
        if e.tick_ns < w.parent_support.start_ns()
            || e.tick_ns > w.parent_support.end_ns()
            || e.tick_ns > w.accepted_until_ns
        {
            return Err(CoupledTimeError::RestartInvalid);
        }
        let tick = e.tick_ns.get().to_be_bytes();
        let ordinal = e.event_ordinal.to_be_bytes();
        let event = framed_sha256(
            "event",
            &[
                FramedField {
                    tag: "parent_transaction_id",
                    value: w.parent_transaction_id.digest().as_bytes(),
                },
                FramedField {
                    tag: "tick_ns",
                    value: &tick,
                },
                FramedField {
                    tag: "event_class",
                    value: event_class_wire(e.class).as_bytes(),
                },
                FramedField {
                    tag: "event_ordinal",
                    value: &ordinal,
                },
                FramedField {
                    tag: "source_owner_id",
                    value: e.source_owner_id.as_bytes(),
                },
                FramedField {
                    tag: "event_context",
                    value: e.event_context_sha256.as_bytes(),
                },
            ],
        )?;
        if e.parent_transaction_id != w.parent_transaction_id || e.event_id.digest() != event {
            return Err(CoupledTimeError::RestartInvalid);
        }
        let receipt = framed_sha256(
            "event-receipt-v2",
            &[
                FramedField {
                    tag: "parent_transaction_id",
                    value: w.parent_transaction_id.digest().as_bytes(),
                },
                FramedField {
                    tag: "event_id",
                    value: event.as_bytes(),
                },
                FramedField {
                    tag: "tick_ns",
                    value: &tick,
                },
                FramedField {
                    tag: "ordinal",
                    value: &ordinal,
                },
                FramedField {
                    tag: "begin_clock",
                    value: e.begin_clock_sha256.as_bytes(),
                },
                FramedField {
                    tag: "end_clock",
                    value: e.end_clock_sha256.as_bytes(),
                },
                FramedField {
                    tag: "begin_owner_set",
                    value: e.begin_owner_set_sha256.as_bytes(),
                },
                FramedField {
                    tag: "end_owner_set",
                    value: e.end_owner_set_sha256.as_bytes(),
                },
                FramedField {
                    tag: "event_context",
                    value: e.event_context_sha256.as_bytes(),
                },
                FramedField {
                    tag: "ledger_digest",
                    value: e.ledger_digest.as_bytes(),
                },
            ],
        )?;
        if e.receipt_id.digest() != receipt {
            return Err(CoupledTimeError::RestartInvalid);
        }
    }
    let mut owner = w.begin_complete_owner_set_sha256;
    let mut clock = w.begin_clock_sha256;
    let mut events = w.accepted_event_receipts.iter().peekable();
    for slab in &w.accepted_slab_receipts {
        while events
            .peek()
            .is_some_and(|e| e.tick_ns == slab.support.start_ns())
        {
            let e = events.next().ok_or(CoupledTimeError::RestartInvalid)?;
            if e.begin_owner_set_sha256 != owner || e.begin_clock_sha256 != clock {
                return Err(CoupledTimeError::RestartInvalid);
            }
            owner = e.end_owner_set_sha256;
            clock = e.end_clock_sha256;
        }
        if slab.begin_owner_set_sha256 != owner || slab.begin_clock_sha256 != clock {
            return Err(CoupledTimeError::RestartInvalid);
        }
        owner = slab.end_owner_set_sha256;
        clock = slab.end_clock_sha256;
    }
    for e in events {
        if e.begin_owner_set_sha256 != owner || e.begin_clock_sha256 != clock {
            return Err(CoupledTimeError::RestartInvalid);
        }
        owner = e.end_owner_set_sha256;
        clock = e.end_clock_sha256;
    }
    if owner != w.accepted_complete_owner_set_sha256
        || w.next_slab_ordinal
            != u32::try_from(w.accepted_slab_receipts.len())
                .map_err(|_| CoupledTimeError::RestartInvalid)?
        || w.next_event_ordinal
            != u32::try_from(w.accepted_event_receipts.len())
                .map_err(|_| CoupledTimeError::RestartInvalid)?
    {
        return Err(CoupledTimeError::RestartInvalid);
    }
    let terminal_tick = w
        .accepted_slab_receipts
        .last()
        .map_or(w.parent_support.start_ns(), |receipt| {
            receipt.support.end_ns()
        });
    if terminal_tick != w.accepted_until_ns
        || w.accepted_slab_receipts.last().is_some_and(|receipt| {
            receipt.support.duration_ns().to_string() != w.last_accepted_step_ns
        })
    {
        return Err(CoupledTimeError::RestartInvalid);
    }
    Ok(())
}
