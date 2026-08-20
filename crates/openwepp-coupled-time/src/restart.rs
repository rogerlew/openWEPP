use crate::{
    AcceptedEventReceiptV1, AcceptedSlabReceiptV1, CoupledClockStateV1, CoupledTimeError,
    DiagnosticReductionV1, Digest32, FramedField, OwnerState, ParentIntervalId,
    ParentTransactionId, ReceiptId, ScheduledOnceReceiptV1, SegmentId, TimeSupport, digest_bytes,
    framed_sha256, transaction::owner_set_digest,
};
use serde::{Deserialize, Serialize};
pub const RESTART_SCHEMA_V1_ID: &str = "OPENWEPP_COUPLED_TIME_RESTART_V1";
pub const RESTART_SCHEMA_ID: &str = "OPENWEPP_COUPLED_TIME_RESTART_V2";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AcceptedOperandLineage {
    Slab(TimeSupport),
    EventInstant(crate::ModelTimeNs),
    ScheduledInstant(crate::ModelTimeNs),
}

/// Byte-preserving legacy V1 envelope. V1 cannot resume authenticated mid-parent chronology.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoupledTimeRestartV1(Vec<u8>);
impl CoupledTimeRestartV1 {
    pub fn preserve_complete_parent(bytes: Vec<u8>) -> Result<Self, CoupledTimeError> {
        let value: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(|_| CoupledTimeError::RestartInvalid)?;
        if value.get("schema").and_then(serde_json::Value::as_str) != Some(RESTART_SCHEMA_V1_ID) {
            return Err(CoupledTimeError::RestartInvalid);
        }
        Ok(Self(bytes))
    }
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

pub(crate) mod u128_string {
    use serde::Serializer;
    pub(crate) fn serialize<S: Serializer>(v: &u128, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&v.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PublicationRecordV1 {
    pub(crate) record_id: ReceiptId,
    pub(crate) accepted_receipt_id: ReceiptId,
    pub(crate) support: TimeSupport,
    pub(crate) value_digest: Digest32,
    pub(crate) payload: Vec<u8>,
    pub(crate) units: String,
    pub(crate) source_owner_id: String,
}
impl PublicationRecordV1 {
    pub fn new(
        accepted_receipt_id: ReceiptId,
        support: TimeSupport,
        value_digest: Digest32,
        payload: Vec<u8>,
        units: String,
        source: String,
    ) -> Result<Self, CoupledTimeError> {
        if units.is_empty() || source.is_empty() || digest_bytes(&payload) != value_digest {
            return Err(CoupledTimeError::LedgerFailure);
        }
        let record_id = derive_publication_record_id(
            accepted_receipt_id,
            support,
            value_digest,
            &units,
            &source,
        )?;
        Ok(Self {
            record_id,
            accepted_receipt_id,
            support,
            value_digest,
            payload,
            units,
            source_owner_id: source,
        })
    }
}
pub(crate) fn derive_publication_record_id(
    receipt: ReceiptId,
    support: TimeSupport,
    value: Digest32,
    units: &str,
    source: &str,
) -> Result<ReceiptId, CoupledTimeError> {
    let start = support.start_ns().get().to_be_bytes();
    let end = support.end_ns().get().to_be_bytes();
    Ok(ReceiptId::from_digest(framed_sha256(
        "publication-receipt",
        &[
            FramedField {
                tag: "parent_receipt_id",
                value: receipt.digest().as_bytes(),
            },
            FramedField {
                tag: "ordered_output_records",
                value: &[
                    &start[..],
                    &end[..],
                    value.as_bytes(),
                    units.as_bytes(),
                    source.as_bytes(),
                ]
                .concat(),
            },
            FramedField {
                tag: "outbox_state",
                value: b"record",
            },
        ],
    )?))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutboxState {
    CommittedUndelivered,
    DeliveredUnacknowledged,
    Acknowledged,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PublicationOutboxV1 {
    pub(crate) receipt_id: ReceiptId,
    pub(crate) parent_receipt_id: ReceiptId,
    pub(crate) records_digest: Digest32,
    #[serde(with = "u128_string")]
    pub(crate) sequence: u128,
    pub(crate) state: OutboxState,
    pub(crate) delivery_attempt_count: u32,
    pub(crate) records: Vec<PublicationRecordV1>,
}
impl PublicationOutboxV1 {
    #[must_use]
    pub const fn receipt_id(&self) -> ReceiptId {
        self.receipt_id
    }
    #[must_use]
    pub const fn state(&self) -> OutboxState {
        self.state
    }
    pub fn mark_delivered(&mut self, key: ReceiptId) -> Result<(), CoupledTimeError> {
        if key != self.receipt_id || self.state == OutboxState::Acknowledged {
            return Err(CoupledTimeError::OutboxTransition);
        }
        self.delivery_attempt_count = self
            .delivery_attempt_count
            .checked_add(1)
            .ok_or(CoupledTimeError::ArithmeticOverflow)?;
        self.state = OutboxState::DeliveredUnacknowledged;
        Ok(())
    }
    pub fn acknowledge(&mut self, key: ReceiptId) -> Result<(), CoupledTimeError> {
        if key != self.receipt_id || self.state != OutboxState::DeliveredUnacknowledged {
            return Err(CoupledTimeError::OutboxTransition);
        }
        self.state = OutboxState::Acknowledged;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ParentCommitCandidateV1 {
    parent_receipt_id: ReceiptId,
    records: Vec<PublicationRecordV1>,
    records_digest: Digest32,
    successor_sequence: u128,
    publication_receipt_id: ReceiptId,
}
impl ParentCommitCandidateV1 {
    pub fn new(
        clock: &CoupledClockStateV1,
        mut records: Vec<PublicationRecordV1>,
    ) -> Result<Self, CoupledTimeError> {
        if !clock.is_complete() || clock.committed || clock.accepted_slab_receipts.is_empty() {
            return Err(CoupledTimeError::ParentNotFinalizable);
        }
        records.sort_by_key(|r| r.record_id);
        if records.windows(2).any(|w| w[0].record_id == w[1].record_id) {
            return Err(CoupledTimeError::LedgerFailure);
        }
        for r in &records {
            if !clock
                .accepted_slab_receipts
                .iter()
                .any(|s| s.id() == r.accepted_receipt_id)
                && !clock
                    .accepted_event_receipts
                    .iter()
                    .any(|e| e.id() == r.accepted_receipt_id)
            {
                return Err(CoupledTimeError::LedgerFailure);
            }
        }
        let parent = derive_parent_receipt(clock)?;
        let record_bytes = serde_json::to_vec(
            &records
                .iter()
                .map(crate::wire::record_wire)
                .collect::<Vec<_>>(),
        )
        .map_err(|_| CoupledTimeError::NonCanonicalIdentity)?;
        let records_digest = crate::digest_bytes(&record_bytes);
        let successor_sequence = clock
            .parent_transaction_sequence
            .checked_add(1)
            .ok_or(CoupledTimeError::ArithmeticOverflow)?;
        let publication = derive_publication_receipt(parent, &records, successor_sequence)?;
        Ok(Self {
            parent_receipt_id: parent,
            records,
            records_digest,
            successor_sequence,
            publication_receipt_id: publication,
        })
    }
}

fn derive_parent_receipt(clock: &CoupledClockStateV1) -> Result<ReceiptId, CoupledTimeError> {
    let mut slabs = Vec::new();
    for receipt in &clock.accepted_slab_receipts {
        slabs.extend_from_slice(receipt.id().digest().as_bytes());
    }
    let mut events = Vec::new();
    for receipt in &clock.accepted_event_receipts {
        events.extend_from_slice(receipt.id().digest().as_bytes());
    }
    let mut scheduled = Vec::new();
    for receipt in &clock.scheduled_once_receipts {
        scheduled.extend_from_slice(receipt.receipt_id.digest().as_bytes());
    }
    Ok(ReceiptId::from_digest(framed_sha256(
        "parent-receipt-v2",
        &[
            FramedField {
                tag: "parent_transaction_id",
                value: clock.parent_transaction_id.digest().as_bytes(),
            },
            FramedField {
                tag: "parent_interval_id",
                value: clock.parent_interval_id.digest().as_bytes(),
            },
            FramedField {
                tag: "begin_owner_set",
                value: clock.begin_owner_set_digest.as_bytes(),
            },
            FramedField {
                tag: "end_owner_set",
                value: owner_set_digest(&clock.complete_owner_set)?.as_bytes(),
            },
            FramedField {
                tag: "ordered_slab_receipts",
                value: &slabs,
            },
            FramedField {
                tag: "ordered_event_receipts",
                value: &events,
            },
            FramedField {
                tag: "ordered_scheduled_receipts",
                value: &scheduled,
            },
        ],
    )?))
}

fn derive_publication_receipt(
    parent: ReceiptId,
    records: &[PublicationRecordV1],
    sequence: u128,
) -> Result<ReceiptId, CoupledTimeError> {
    let mut ids = Vec::new();
    for record in records {
        ids.extend_from_slice(record.record_id.digest().as_bytes());
    }
    Ok(ReceiptId::from_digest(framed_sha256(
        "publication-receipt-v2",
        &[
            FramedField {
                tag: "parent_receipt_id",
                value: parent.digest().as_bytes(),
            },
            FramedField {
                tag: "ordered_output_record_ids",
                value: &ids,
            },
            FramedField {
                tag: "outbox_sequence",
                value: &sequence.to_be_bytes(),
            },
            FramedField {
                tag: "outbox_state",
                value: b"CommittedUndelivered",
            },
        ],
    )?))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ParentCommitV1 {
    parent_receipt_id: ReceiptId,
    #[serde(with = "u128_string")]
    transaction_sequence: u128,
    outbox: PublicationOutboxV1,
}
impl ParentCommitV1 {
    #[must_use]
    pub const fn parent_receipt_id(&self) -> ReceiptId {
        self.parent_receipt_id
    }
    #[must_use]
    pub const fn transaction_sequence(&self) -> u128 {
        self.transaction_sequence
    }
    #[must_use]
    pub const fn outbox(&self) -> &PublicationOutboxV1 {
        &self.outbox
    }
    #[must_use]
    pub fn into_outbox(self) -> PublicationOutboxV1 {
        self.outbox
    }
}
#[derive(Debug, Clone)]
pub struct DurableParentCommitV1 {
    clock: CoupledClockStateV1,
    commit: ParentCommitV1,
}
impl DurableParentCommitV1 {
    #[must_use]
    pub const fn clock(&self) -> &CoupledClockStateV1 {
        &self.clock
    }
    #[must_use]
    pub const fn commit(&self) -> &ParentCommitV1 {
        &self.commit
    }
    #[must_use]
    pub fn into_parts(self) -> (CoupledClockStateV1, ParentCommitV1) {
        (self.clock, self.commit)
    }
}
pub fn commit_parent(
    mut clock: CoupledClockStateV1,
    candidate: ParentCommitCandidateV1,
) -> Result<DurableParentCommitV1, CoupledTimeError> {
    if clock.committed || !clock.is_complete() {
        return Err(CoupledTimeError::ParentNotFinalizable);
    }
    let expected = ParentCommitCandidateV1::new(&clock, candidate.records.clone())?;
    if expected.parent_receipt_id != candidate.parent_receipt_id
        || expected.records_digest != candidate.records_digest
        || expected.successor_sequence != candidate.successor_sequence
        || expected.publication_receipt_id != candidate.publication_receipt_id
    {
        return Err(CoupledTimeError::ParentNotFinalizable);
    }
    let outbox = PublicationOutboxV1 {
        receipt_id: candidate.publication_receipt_id,
        parent_receipt_id: candidate.parent_receipt_id,
        records_digest: candidate.records_digest,
        sequence: candidate.successor_sequence,
        state: OutboxState::CommittedUndelivered,
        delivery_attempt_count: 0,
        records: candidate.records,
    };
    clock.committed = true;
    let commit = ParentCommitV1 {
        parent_receipt_id: candidate.parent_receipt_id,
        transaction_sequence: candidate.successor_sequence,
        outbox,
    };
    Ok(DurableParentCommitV1 { clock, commit })
}

#[derive(Debug, Clone, PartialEq)]
pub struct CoupledTimeRestartV2 {
    model_definition_sha256: Digest32,
    authority_sha256: Digest32,
    clock: CoupledClockStateV1,
    reduction_state: Vec<DiagnosticReductionV1>,
    publication_outbox: Vec<PublicationOutboxV1>,
    pending_publication_buffer: Vec<PublicationRecordV1>,
}
impl CoupledTimeRestartV2 {
    pub fn new(
        model: Digest32,
        authority: Digest32,
        clock: CoupledClockStateV1,
        reduction: DiagnosticReductionV1,
        outbox: Option<PublicationOutboxV1>,
        pending: Vec<PublicationRecordV1>,
    ) -> Result<Self, CoupledTimeError> {
        let value = Self {
            model_definition_sha256: model,
            authority_sha256: authority,
            clock,
            reduction_state: vec![reduction],
            publication_outbox: outbox.into_iter().collect(),
            pending_publication_buffer: pending,
        };
        value.validate(model, authority, value.clock.controller_policy_sha256)?;
        Ok(value)
    }
    pub fn to_canonical_json(&self) -> Result<Vec<u8>, CoupledTimeError> {
        serde_json::to_vec(&self.to_wire()?).map_err(|_| CoupledTimeError::RestartInvalid)
    }
    pub fn from_canonical_json(
        bytes: &[u8],
        model: Digest32,
        authority: Digest32,
        policy: Digest32,
    ) -> Result<Self, CoupledTimeError> {
        let wire: crate::wire::RestartWireV2 =
            serde_json::from_slice(bytes).map_err(|_| CoupledTimeError::RestartInvalid)?;
        let value = Self::from_wire(wire)?;
        if value.to_canonical_json()?.as_slice() != bytes {
            return Err(CoupledTimeError::RestartInvalid);
        }
        value.validate(model, authority, policy)?;
        Ok(value)
    }
    #[allow(clippy::too_many_lines)]
    fn to_wire(&self) -> Result<crate::wire::RestartWireV2, CoupledTimeError> {
        use crate::wire::{
            EventReceiptWire, OutboxWire, OwnerWire, ReductionWire, RestartWireV2,
            ScheduledReceiptWire, SegmentWire, SlabReceiptWire, blob, record_wire,
        };
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        let owners = self
            .clock
            .complete_owner_set
            .iter()
            .map(|o| OwnerWire {
                owner_id: o.owner_id().into(),
                state_encoding: "canonical-binary".into(),
                state_bytes_sha256: o.state_digest(),
                state_bytes_base64: STANDARD.encode(o.state_bytes()),
            })
            .collect();
        let begin_owner = self.clock.begin_owner_set_digest;
        let begin_clock = self.clock.begin_clock_digest;
        let slabs = self
            .clock
            .accepted_slab_receipts
            .iter()
            .map(|r| SlabReceiptWire {
                receipt_id: r.receipt_id,
                accepted_slab_id: r.slab_id,
                parent_transaction_id: r.parent_transaction_id,
                slab_ordinal: r.slab_ordinal,
                segment_id: r.segment_id,
                support: r.support,
                duration_bits: format!("{:016x}", r.duration_bits),
                constraint_digest: r.constraint_digest,
                begin_clock_sha256: r.begin_clock,
                end_clock_sha256: r.end_clock,
                begin_owner_set_sha256: r.begin_owner_set,
                end_owner_set_sha256: r.end_owner_set,
                owner_candidate_set_sha256: r.owner_candidate_set,
                coupled_ledger_sha256: r.ledger_digest,
            })
            .collect();
        let events = self
            .clock
            .accepted_event_receipts
            .iter()
            .map(|r| EventReceiptWire {
                receipt_id: r.receipt_id,
                event_id: r.event_id,
                parent_transaction_id: r.parent_transaction_id,
                tick_ns: r.tick,
                event_ordinal: r.ordinal,
                class: r.class,
                source_owner_id: r.source_owner_id.clone(),
                begin_clock_sha256: r.begin_clock,
                end_clock_sha256: r.end_clock,
                begin_owner_set_sha256: r.begin_owner_set,
                end_owner_set_sha256: r.end_owner_set,
                event_context_sha256: r.event_context_digest,
                ledger_digest: r.ledger_digest,
                ledgers: vec![],
            })
            .collect();
        let scheduled = self
            .clock
            .scheduled_once_receipts
            .iter()
            .map(|r| ScheduledReceiptWire {
                receipt_id: r.receipt_id,
                operation_id: r.operation_id.clone(),
                boundary_id: r.boundary_id,
                tick_ns: r.boundary,
                result_sha256: r.result_sha256,
            })
            .collect();
        let reductions = self
            .reduction_state
            .iter()
            .map(|reduction| ReductionWire {
                reduction_id: reduction.reduction_id.clone(),
                operator: match reduction.operator {
                    crate::transaction::ReductionOperatorV1::Maximum => "maximum",
                    crate::transaction::ReductionOperatorV1::Minimum => "minimum",
                }
                .into(),
                units: reduction.units.clone(),
                value_bits: reduction.maximum.map(|v| format!("{:016x}", v.to_bits())),
                accepted_operand_receipt_ids: reduction.accepted_receipts.clone(),
                accepted_operand_values: reduction
                    .accepted_values
                    .iter()
                    .map(|(receipt_id, value)| crate::wire::ReductionOperandWire {
                        receipt_id: *receipt_id,
                        value_bits: format!("{:016x}", value.to_bits()),
                    })
                    .collect(),
            })
            .collect();
        let pending = self
            .pending_publication_buffer
            .iter()
            .map(record_wire)
            .collect();
        let outboxes = self
            .publication_outbox
            .iter()
            .map(|o| OutboxWire {
                publication_receipt_id: o.receipt_id,
                parent_receipt_id: o.parent_receipt_id,
                outbox_sequence: o.sequence.to_string(),
                state: o.state,
                records: o.records.iter().map(record_wire).collect(),
                records_sha256: o.records_digest,
                delivery_attempt_count: o.delivery_attempt_count.to_string(),
            })
            .collect();
        Ok(RestartWireV2 {
            schema: RESTART_SCHEMA_ID.into(),
            version: 2,
            authority_sha256: self.authority_sha256,
            model_definition_sha256: self.model_definition_sha256,
            run_identity_sha256: self.clock.run_identity,
            calendar_receipt_sha256: self.clock.calendar_receipt,
            forcing_receipt_sha256: self.clock.forcing_receipt,
            parent_interval_id: self.clock.parent_interval_id,
            checkpoint_phase: if self.clock.committed {
                "CommittedParent".into()
            } else {
                "ActiveParent".into()
            },
            parent_transaction_sequence: self.clock.parent_transaction_sequence.to_string(),
            next_parent_transaction_sequence: self
                .clock
                .parent_transaction_sequence
                .checked_add(u128::from(self.clock.committed))
                .ok_or(CoupledTimeError::ArithmeticOverflow)?
                .to_string(),
            parent_transaction_id: self.clock.parent_transaction_id,
            parent_support: self.clock.parent_support,
            accepted_until_ns: self.clock.accepted_until,
            next_segment_ordinal: self.clock.segment_ordinal,
            next_slab_ordinal: self.clock.slab_ordinal,
            next_event_ordinal: self.clock.event_ordinal,
            last_accepted_step_ns: self.clock.last_accepted_step_ns.unwrap_or(0).to_string(),
            active_segment: SegmentWire {
                segment_id: self.clock.active_segment_id,
                ordinal: self.clock.segment_ordinal,
                regime_id: self.clock.active_regime_id.clone(),
                start_ns: self.clock.active_segment_start,
                end_ns: self.clock.active_segment_end,
                active_participants: self.clock.active_participant_set.clone(),
            },
            complete_owner_state: owners,
            begin_complete_owner_set_sha256: begin_owner,
            begin_clock_sha256: begin_clock,
            accepted_complete_owner_set_sha256: owner_set_digest(&self.clock.complete_owner_set)?,
            boundary_modes: vec![],
            constraint_policy_sha256: self.clock.controller_policy_sha256,
            controller_policy_sha256: self.clock.controller_policy_sha256,
            accepted_controller_checkpoint: blob(&self.clock.controller_checkpoint),
            accepted_slab_receipts: slabs,
            accepted_event_receipts: events,
            scheduled_once_receipts: scheduled,
            reduction_state: reductions,
            pending_publication_buffer: pending,
            publication_outbox: outboxes,
        })
    }
    #[allow(clippy::too_many_lines)]
    fn from_wire(w: crate::wire::RestartWireV2) -> Result<Self, CoupledTimeError> {
        use crate::wire::{
            decode_blob, parse_u128, record_from_wire, validate_authenticated_chronology,
        };
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        validate_authenticated_chronology(&w)?;
        if w.schema != RESTART_SCHEMA_ID || w.version != 2 {
            return Err(CoupledTimeError::RestartInvalid);
        }
        let sequence = parse_u128(&w.parent_transaction_sequence)?;
        let next_sequence = parse_u128(&w.next_parent_transaction_sequence)?;
        let committed_phase = match w.checkpoint_phase.as_str() {
            "ActiveParent" => false,
            "CommittedParent" => true,
            _ => return Err(CoupledTimeError::RestartInvalid),
        };
        let expected_next = sequence
            .checked_add(u128::from(committed_phase))
            .ok_or(CoupledTimeError::RestartInvalid)?;
        if next_sequence != expected_next
            || committed_phase == w.publication_outbox.is_empty()
            || committed_phase
                && (w.publication_outbox.len() != 1 || !w.pending_publication_buffer.is_empty())
            || !committed_phase && !w.publication_outbox.is_empty()
        {
            return Err(CoupledTimeError::RestartInvalid);
        }
        let last = parse_u128(&w.last_accepted_step_ns)?;
        let owners = w
            .complete_owner_state
            .into_iter()
            .map(|o| {
                if o.state_encoding != "canonical-binary" {
                    return Err(CoupledTimeError::RestartInvalid);
                }
                let bytes = STANDARD
                    .decode(o.state_bytes_base64)
                    .map_err(|_| CoupledTimeError::RestartInvalid)?;
                if digest_bytes(&bytes) != o.state_bytes_sha256 {
                    return Err(CoupledTimeError::RestartInvalid);
                }
                OwnerState::new(o.owner_id, bytes)
            })
            .collect::<Result<Vec<_>, _>>()?;
        crate::clock::validate_owner_and_participant_sets(
            &owners,
            &w.active_segment.active_participants,
        )?;
        if owner_set_digest(&owners)? != w.accepted_complete_owner_set_sha256 {
            return Err(CoupledTimeError::RestartInvalid);
        }
        if ParentIntervalId::derive(
            w.run_identity_sha256,
            w.calendar_receipt_sha256,
            w.forcing_receipt_sha256,
            w.parent_support,
        )? != w.parent_interval_id
            || ParentTransactionId::derive(
                w.run_identity_sha256,
                sequence,
                w.parent_interval_id,
                w.begin_complete_owner_set_sha256,
            )? != w.parent_transaction_id
        {
            return Err(CoupledTimeError::RestartInvalid);
        }
        let mut participant_bytes = Vec::new();
        for p in &w.active_segment.active_participants {
            participant_bytes.extend_from_slice(p.as_bytes());
            participant_bytes.push(0);
        }
        let active_support = TimeSupport::new(w.active_segment.start_ns, w.active_segment.end_ns)?;
        if SegmentId::derive(
            w.parent_transaction_id,
            w.active_segment.ordinal,
            active_support,
            digest_bytes(w.active_segment.regime_id.as_bytes()),
            digest_bytes(&participant_bytes),
        )? != w.active_segment.segment_id
        {
            return Err(CoupledTimeError::RestartInvalid);
        }
        let slabs = w
            .accepted_slab_receipts
            .into_iter()
            .map(|r| {
                let duration_bits = u64::from_str_radix(&r.duration_bits, 16)
                    .map_err(|_| CoupledTimeError::RestartInvalid)?;
                Ok(AcceptedSlabReceiptV1 {
                    receipt_id: r.receipt_id,
                    slab_id: r.accepted_slab_id,
                    parent_transaction_id: r.parent_transaction_id,
                    slab_ordinal: r.slab_ordinal,
                    segment_id: r.segment_id,
                    support: r.support,
                    duration_bits,
                    constraint_digest: r.constraint_digest,
                    begin_clock: r.begin_clock_sha256,
                    end_clock: r.end_clock_sha256,
                    begin_owner_set: r.begin_owner_set_sha256,
                    end_owner_set: r.end_owner_set_sha256,
                    owner_candidate_set: r.owner_candidate_set_sha256,
                    ledger_digest: r.coupled_ledger_sha256,
                })
            })
            .collect::<Result<Vec<_>, CoupledTimeError>>()?;
        let events: Vec<AcceptedEventReceiptV1> = w
            .accepted_event_receipts
            .into_iter()
            .map(|r| AcceptedEventReceiptV1 {
                receipt_id: r.receipt_id,
                event_id: r.event_id,
                tick: r.tick_ns,
                ordinal: r.event_ordinal,
                begin_owner_set: r.begin_owner_set_sha256,
                end_owner_set: r.end_owner_set_sha256,
                ledger_digest: r.ledger_digest,
                class: r.class,
                event_context_digest: r.event_context_sha256,
                parent_transaction_id: r.parent_transaction_id,
                source_owner_id: r.source_owner_id,
                begin_clock: r.begin_clock_sha256,
                end_clock: r.end_clock_sha256,
            })
            .collect();
        let scheduled = w
            .scheduled_once_receipts
            .into_iter()
            .map(|r| ScheduledOnceReceiptV1 {
                operation_id: r.operation_id,
                boundary_id: r.boundary_id,
                boundary: r.tick_ns,
                receipt_id: r.receipt_id,
                result_sha256: r.result_sha256,
            })
            .collect();
        let checkpoint = decode_blob(&w.accepted_controller_checkpoint)?;
        let reduction = w
            .reduction_state
            .into_iter()
            .map(|reduction_wire| {
                let operator = match reduction_wire.operator.as_str() {
                    "maximum" => crate::transaction::ReductionOperatorV1::Maximum,
                    "minimum" => crate::transaction::ReductionOperatorV1::Minimum,
                    _ => return Err(CoupledTimeError::RestartInvalid),
                };
                if reduction_wire.accepted_operand_receipt_ids.len()
                    != reduction_wire.accepted_operand_values.len()
                    || reduction_wire
                        .accepted_operand_receipt_ids
                        .iter()
                        .zip(&reduction_wire.accepted_operand_values)
                        .any(|(id, value)| id != &value.receipt_id)
                {
                    return Err(CoupledTimeError::RestartInvalid);
                }
                let values = reduction_wire
                    .accepted_operand_values
                    .into_iter()
                    .map(|operand| {
                        let value = f64::from_bits(
                            u64::from_str_radix(&operand.value_bits, 16)
                                .map_err(|_| CoupledTimeError::RestartInvalid)?,
                        );
                        if !value.is_finite() {
                            return Err(CoupledTimeError::RestartInvalid);
                        }
                        Ok((operand.receipt_id, value))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let reconstructed = values
                    .iter()
                    .map(|(_, value)| *value)
                    .reduce(match operator {
                        crate::transaction::ReductionOperatorV1::Maximum => {
                            crate::transaction::retain_maximum
                        }
                        crate::transaction::ReductionOperatorV1::Minimum => {
                            crate::transaction::retain_minimum
                        }
                    });
                let admitted = reduction_wire
                    .value_bits
                    .map(|bits| {
                        u64::from_str_radix(&bits, 16)
                            .map(f64::from_bits)
                            .map_err(|_| CoupledTimeError::RestartInvalid)
                    })
                    .transpose()?;
                if admitted.map(f64::to_bits) != reconstructed.map(f64::to_bits) {
                    return Err(CoupledTimeError::RestartInvalid);
                }
                Ok(DiagnosticReductionV1 {
                    reduction_id: reduction_wire.reduction_id,
                    units: reduction_wire.units,
                    maximum: admitted,
                    accepted_receipts: reduction_wire.accepted_operand_receipt_ids,
                    accepted_values: values,
                    operator,
                })
            })
            .collect::<Result<Vec<_>, CoupledTimeError>>()?;
        let pending = w
            .pending_publication_buffer
            .into_iter()
            .map(record_from_wire)
            .collect::<Result<Vec<_>, _>>()?;
        let outbox = w
            .publication_outbox
            .into_iter()
            .map(|o| {
                Ok(PublicationOutboxV1 {
                    receipt_id: o.publication_receipt_id,
                    parent_receipt_id: o.parent_receipt_id,
                    records_digest: o.records_sha256,
                    sequence: parse_u128(&o.outbox_sequence)?,
                    state: o.state,
                    delivery_attempt_count: u32::try_from(parse_u128(&o.delivery_attempt_count)?)
                        .map_err(|_| CoupledTimeError::RestartInvalid)?,
                    records: o
                        .records
                        .into_iter()
                        .map(record_from_wire)
                        .collect::<Result<Vec<_>, _>>()?,
                })
            })
            .collect::<Result<Vec<_>, CoupledTimeError>>()?;
        let clock = CoupledClockStateV1 {
            run_identity: w.run_identity_sha256,
            calendar_receipt: w.calendar_receipt_sha256,
            forcing_receipt: w.forcing_receipt_sha256,
            parent_transaction_sequence: sequence,
            committed: committed_phase,
            begin_owner_set_digest: w.begin_complete_owner_set_sha256,
            begin_clock_digest: w.begin_clock_sha256,
            accepted_clock_digest: match (slabs.last(), events.last()) {
                (Some(slab), Some(event)) if event.tick >= slab.support.end_ns() => event.end_clock,
                (Some(slab), _) => slab.end_clock,
                (None, Some(event)) => event.end_clock,
                (None, None) => w.begin_clock_sha256,
            },
            parent_interval_id: w.parent_interval_id,
            parent_transaction_id: w.parent_transaction_id,
            parent_support: w.parent_support,
            accepted_until: w.accepted_until_ns,
            segment_ordinal: w.next_segment_ordinal,
            slab_ordinal: w.next_slab_ordinal,
            event_ordinal: w.next_event_ordinal,
            last_accepted_step_ns: Some(last),
            complete_owner_set: owners,
            active_regime_id: w.active_segment.regime_id,
            active_segment_start: w.active_segment.start_ns,
            active_segment_end: w.active_segment.end_ns,
            active_segment_id: w.active_segment.segment_id,
            active_participant_set: w.active_segment.active_participants,
            accepted_slab_receipts: slabs,
            accepted_event_receipts: events,
            scheduled_once_receipts: scheduled,
            controller_policy_sha256: w.controller_policy_sha256,
            controller_checkpoint: checkpoint,
        };
        Ok(Self {
            model_definition_sha256: w.model_definition_sha256,
            authority_sha256: w.authority_sha256,
            clock,
            reduction_state: reduction,
            publication_outbox: outbox,
            pending_publication_buffer: pending,
        })
    }
    #[allow(clippy::too_many_lines)]
    pub fn validate(
        &self,
        model: Digest32,
        authority: Digest32,
        policy: Digest32,
    ) -> Result<(), CoupledTimeError> {
        if self.model_definition_sha256 != model
            || self.authority_sha256 != authority
            || self.clock.controller_policy_sha256 != policy
            || self.clock.accepted_until < self.clock.parent_support.start_ns()
            || self.clock.accepted_until > self.clock.parent_support.end_ns()
        {
            return Err(CoupledTimeError::RestartInvalid);
        }
        crate::clock::validate_owner_and_participant_sets(
            &self.clock.complete_owner_set,
            &self.clock.active_participant_set,
        )
        .map_err(|_| CoupledTimeError::RestartInvalid)?;
        if owner_set_digest(&self.clock.complete_owner_set).is_err()
            || self
                .clock
                .accepted_slab_receipts
                .windows(2)
                .any(|w| w[0].support().end_ns() != w[1].support().start_ns())
            || self
                .clock
                .accepted_event_receipts
                .windows(2)
                .any(|w| w[0].id() == w[1].id())
            || self
                .clock
                .scheduled_once_receipts
                .windows(2)
                .any(|pair| pair[0].receipt_id >= pair[1].receipt_id)
            || self
                .clock
                .scheduled_once_receipts
                .iter()
                .enumerate()
                .any(|(i, left)| {
                    self.clock.scheduled_once_receipts[i + 1..]
                        .iter()
                        .any(|right| {
                            left.operation_id == right.operation_id
                                && left.boundary_id == right.boundary_id
                        })
                })
        {
            return Err(CoupledTimeError::RestartInvalid);
        }
        for scheduled in &self.clock.scheduled_once_receipts {
            scheduled.validate_identity(self.clock.parent_transaction_id)?;
            if scheduled.boundary < self.clock.parent_support.start_ns()
                || scheduled.boundary > self.clock.parent_support.end_ns()
                || scheduled.boundary > self.clock.accepted_until
            {
                return Err(CoupledTimeError::RestartInvalid);
            }
        }
        let accepted_slab_support = |receipt: ReceiptId| {
            self.clock
                .accepted_slab_receipts
                .iter()
                .find(|r| r.id() == receipt)
                .map(AcceptedSlabReceiptV1::support)
        };
        let accepted_operand = |receipt: ReceiptId| {
            if let Some(slab) = self
                .clock
                .accepted_slab_receipts
                .iter()
                .find(|r| r.id() == receipt)
            {
                Some(AcceptedOperandLineage::Slab(slab.support()))
            } else if let Some(event) = self
                .clock
                .accepted_event_receipts
                .iter()
                .find(|r| r.id() == receipt)
            {
                Some(AcceptedOperandLineage::EventInstant(event.tick()))
            } else {
                self.clock
                    .scheduled_once_receipts
                    .iter()
                    .find(|r| r.receipt_id == receipt)
                    .map(|scheduled| AcceptedOperandLineage::ScheduledInstant(scheduled.boundary))
            }
        };
        for record in self
            .pending_publication_buffer
            .iter()
            .chain(self.publication_outbox.iter().flat_map(|o| &o.records))
        {
            if derive_publication_record_id(
                record.accepted_receipt_id,
                record.support,
                record.value_digest,
                &record.units,
                &record.source_owner_id,
            )? != record.record_id
                || accepted_slab_support(record.accepted_receipt_id) != Some(record.support)
            {
                return Err(CoupledTimeError::RestartInvalid);
            }
        }
        for reduction in &self.reduction_state {
            if reduction.maximum.is_some_and(|value| !value.is_finite())
                || reduction.accepted_receipts.len() != reduction.accepted_values.len()
                || reduction
                    .accepted_receipts
                    .iter()
                    .zip(&reduction.accepted_values)
                    .any(|(receipt, (value_receipt, value))| {
                        receipt != value_receipt || !value.is_finite()
                    })
                || reduction
                    .accepted_receipts
                    .iter()
                    .enumerate()
                    .any(|(i, receipt)| reduction.accepted_receipts[i + 1..].contains(receipt))
                || reduction
                    .accepted_receipts
                    .iter()
                    .any(|receipt| accepted_operand(*receipt).is_none())
                || reduction.maximum.map(f64::to_bits)
                    != reduction
                        .accepted_values
                        .iter()
                        .map(|(_, value)| *value)
                        .reduce(match reduction.operator {
                            crate::transaction::ReductionOperatorV1::Maximum => {
                                crate::transaction::retain_maximum
                            }
                            crate::transaction::ReductionOperatorV1::Minimum => {
                                crate::transaction::retain_minimum
                            }
                        })
                        .map(f64::to_bits)
            {
                return Err(CoupledTimeError::RestartInvalid);
            }
        }
        for outbox in &self.publication_outbox {
            let parent = derive_parent_receipt(&self.clock)?;
            if !self.clock.committed
                || match outbox.state {
                    OutboxState::CommittedUndelivered => outbox.delivery_attempt_count != 0,
                    OutboxState::DeliveredUnacknowledged | OutboxState::Acknowledged => {
                        outbox.delivery_attempt_count == 0
                    }
                }
                || outbox.parent_receipt_id != parent
                || outbox.sequence
                    != self
                        .clock
                        .parent_transaction_sequence
                        .checked_add(1)
                        .ok_or(CoupledTimeError::RestartInvalid)?
                || outbox.receipt_id
                    != derive_publication_receipt(parent, &outbox.records, outbox.sequence)?
                || crate::digest_bytes(
                    &serde_json::to_vec(
                        &outbox
                            .records
                            .iter()
                            .map(crate::wire::record_wire)
                            .collect::<Vec<_>>(),
                    )
                    .map_err(|_| CoupledTimeError::RestartInvalid)?,
                ) != outbox.records_digest
            {
                return Err(CoupledTimeError::RestartInvalid);
            }
        }
        Ok(())
    }
    #[must_use]
    pub fn clock(&self) -> &CoupledClockStateV1 {
        &self.clock
    }

    /// Consume an already admitted restart into the owned state required to resume.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        CoupledClockStateV1,
        Vec<DiagnosticReductionV1>,
        Vec<PublicationOutboxV1>,
        Vec<PublicationRecordV1>,
    ) {
        (
            self.clock,
            self.reduction_state,
            self.publication_outbox,
            self.pending_publication_buffer,
        )
    }
}
