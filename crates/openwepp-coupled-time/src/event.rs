use crate::{
    CoupledClockStateV1, CoupledTimeError, Digest32, EventId, FramedField, ModelTimeNs, OwnerState,
    ReceiptId, TimeSupport, framed_sha256,
    transaction::{LedgerEntryV1, ledger_digest, owner_set_digest},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum EventClass {
    OwnershipTransfer = 0,
    BoundaryModeTransition = 1,
    RegimeTransition = 2,
    ScheduledBoundary = 3,
    DiagnosticMarker = 4,
}
impl EventClass {
    fn wire(self) -> &'static str {
        match self {
            Self::OwnershipTransfer => "OwnershipTransfer",
            Self::BoundaryModeTransition => "BoundaryModeTransition",
            Self::RegimeTransition => "RegimeTransition",
            Self::ScheduledBoundary => "ScheduledBoundary",
            Self::DiagnosticMarker => "DiagnosticMarker",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AcceptedEventReceiptV1 {
    pub(crate) receipt_id: ReceiptId,
    pub(crate) event_id: EventId,
    pub(crate) tick: ModelTimeNs,
    pub(crate) ordinal: u32,
    pub(crate) begin_owner_set: Digest32,
    pub(crate) end_owner_set: Digest32,
    pub(crate) ledger_digest: Digest32,
    pub(crate) class: EventClass,
    pub(crate) event_context_digest: Digest32,
    pub(crate) parent_transaction_id: crate::ParentTransactionId,
    pub(crate) source_owner_id: String,
    pub(crate) begin_clock: Digest32,
    pub(crate) end_clock: Digest32,
}
impl AcceptedEventReceiptV1 {
    #[must_use]
    pub const fn id(&self) -> ReceiptId {
        self.receipt_id
    }
    #[must_use]
    pub const fn tick(&self) -> ModelTimeNs {
        self.tick
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EventTransitionV1 {
    event_id: EventId,
    tick_ns: ModelTimeNs,
    class: EventClass,
    source_owner_id: String,
    event_context_digest: Digest32,
    ordinal: u32,
    beginning_owner_set_digest: Digest32,
    ending_owners: Vec<OwnerState>,
    mutation_set: Vec<String>,
    successor_regime_id: String,
    successor_participants: Vec<String>,
    ledger_digest: Digest32,
    ledger_entries: Vec<LedgerEntryV1>,
    receipt: AcceptedEventReceiptV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventProposalV1 {
    class: EventClass,
    source_owner_id: String,
    event_context_digest: Digest32,
    ending_owners: Vec<OwnerState>,
    mutation_set: Vec<String>,
    successor_regime_id: String,
    successor_participants: Vec<String>,
    ledger_entries: Vec<LedgerEntryV1>,
}

impl EventProposalV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        class: EventClass,
        source_owner_id: String,
        event_context_digest: Digest32,
        ending_owners: Vec<OwnerState>,
        mutation_set: Vec<String>,
        successor_regime_id: String,
        successor_participants: Vec<String>,
        ledger_entries: Vec<LedgerEntryV1>,
    ) -> Result<Self, CoupledTimeError> {
        if source_owner_id.is_empty() {
            return Err(CoupledTimeError::EventTransition);
        }
        Ok(Self {
            class,
            source_owner_id,
            event_context_digest,
            ending_owners,
            mutation_set,
            successor_regime_id,
            successor_participants,
            ledger_entries,
        })
    }
}
impl EventTransitionV1 {
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        clock: &CoupledClockStateV1,
        tick: ModelTimeNs,
        class: EventClass,
        source: String,
        context: Digest32,
        ending: Vec<OwnerState>,
        mutation_set: Vec<String>,
        successor_regime: String,
        participants: Vec<String>,
        ledger_entries: Vec<LedgerEntryV1>,
    ) -> Result<Self, CoupledTimeError> {
        if tick != clock.accepted_until
            || source.is_empty()
            || !clock
                .complete_owner_set
                .iter()
                .any(|owner| owner.owner_id() == source)
        {
            return Err(CoupledTimeError::EventTransition);
        }
        crate::clock::validate_owner_and_participant_sets(&ending, &participants)?;
        if ending.len() != clock.complete_owner_set.len()
            || ending
                .iter()
                .zip(&clock.complete_owner_set)
                .any(|(a, b)| a.owner_id() != b.owner_id())
        {
            return Err(CoupledTimeError::OwnerCandidate);
        }
        if mutation_set.windows(2).any(|w| w[0] >= w[1]) {
            return Err(CoupledTimeError::EventTransition);
        }
        for (before, after) in clock.complete_owner_set.iter().zip(&ending) {
            let changed = before != after;
            if changed != mutation_set.iter().any(|id| id == before.owner_id()) {
                return Err(CoupledTimeError::EventTransition);
            }
        }
        let begin = owner_set_digest(&clock.complete_owner_set)?;
        let end = owner_set_digest(&ending)?;
        let tick_b = tick.get().to_be_bytes();
        let ordinal_b = clock.event_ordinal.to_be_bytes();
        let event_digest = framed_sha256(
            "event",
            &[
                FramedField {
                    tag: "parent_transaction_id",
                    value: clock.parent_transaction_id.digest().as_bytes(),
                },
                FramedField {
                    tag: "tick_ns",
                    value: &tick_b,
                },
                FramedField {
                    tag: "event_class",
                    value: class.wire().as_bytes(),
                },
                FramedField {
                    tag: "event_ordinal",
                    value: &ordinal_b,
                },
                FramedField {
                    tag: "source_owner_id",
                    value: source.as_bytes(),
                },
                FramedField {
                    tag: "event_context",
                    value: context.as_bytes(),
                },
            ],
        )?;
        let event_id = EventId::from_digest(event_digest);
        let ledger = ledger_digest(&ledger_entries)?;
        let begin_clock = clock.accepted_clock_digest;
        let end_clock = crate::digest_bytes(
            &[
                &begin_clock.as_bytes()[..],
                &event_digest.as_bytes()[..],
                &end.as_bytes()[..],
            ]
            .concat(),
        );
        let receipt_digest = framed_sha256(
            "event-receipt-v2",
            &[
                FramedField {
                    tag: "parent_transaction_id",
                    value: clock.parent_transaction_id.digest().as_bytes(),
                },
                FramedField {
                    tag: "event_id",
                    value: event_digest.as_bytes(),
                },
                FramedField {
                    tag: "tick_ns",
                    value: &tick_b,
                },
                FramedField {
                    tag: "ordinal",
                    value: &ordinal_b,
                },
                FramedField {
                    tag: "begin_clock",
                    value: begin_clock.as_bytes(),
                },
                FramedField {
                    tag: "end_clock",
                    value: end_clock.as_bytes(),
                },
                FramedField {
                    tag: "begin_owner_set",
                    value: begin.as_bytes(),
                },
                FramedField {
                    tag: "end_owner_set",
                    value: end.as_bytes(),
                },
                FramedField {
                    tag: "event_context",
                    value: context.as_bytes(),
                },
                FramedField {
                    tag: "ledger_digest",
                    value: ledger.as_bytes(),
                },
            ],
        )?;
        let receipt = AcceptedEventReceiptV1 {
            receipt_id: ReceiptId::from_digest(receipt_digest),
            event_id,
            tick,
            ordinal: clock.event_ordinal,
            begin_owner_set: begin,
            end_owner_set: end,
            ledger_digest: ledger,
            class,
            event_context_digest: context,
            parent_transaction_id: clock.parent_transaction_id,
            source_owner_id: source.clone(),
            begin_clock,
            end_clock,
        };
        Ok(Self {
            event_id,
            tick_ns: tick,
            class,
            source_owner_id: source,
            event_context_digest: context,
            ordinal: clock.event_ordinal,
            beginning_owner_set_digest: begin,
            ending_owners: ending,
            mutation_set,
            successor_regime_id: successor_regime,
            successor_participants: participants,
            ledger_digest: ledger,
            ledger_entries,
            receipt,
        })
    }
}

#[derive(Debug, Clone)]
pub struct EventQueueV1 {
    tick: ModelTimeNs,
    pending: Vec<EventProposalV1>,
    seen_cycle_keys: Vec<Digest32>,
    applied: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PendingEventJoinV1 {
    pub(crate) event_id: EventId,
}
impl EventQueueV1 {
    pub fn new(
        tick: ModelTimeNs,
        mut pending: Vec<EventProposalV1>,
    ) -> Result<Self, CoupledTimeError> {
        pending.sort_by_key(|e| (e.class, e.source_owner_id.clone(), e.event_context_digest));
        if pending.windows(2).any(|w| w[0] == w[1]) {
            return Err(CoupledTimeError::EventTransition);
        }
        Ok(Self {
            tick,
            pending,
            seen_cycle_keys: Vec::new(),
            applied: 0,
        })
    }
    pub fn apply_next(
        &mut self,
        clock: &mut CoupledClockStateV1,
    ) -> Result<Option<AcceptedEventReceiptV1>, CoupledTimeError> {
        let Some(event) = self.pending.first().cloned() else {
            return Ok(None);
        };
        if self.applied >= 256 || clock.accepted_until != self.tick {
            return Err(CoupledTimeError::EventCycle);
        }
        let mut semantics = Vec::new();
        semantics.extend_from_slice(&self.tick.get().to_be_bytes());
        semantics.extend_from_slice(owner_set_digest(&clock.complete_owner_set)?.as_bytes());
        semantics.extend_from_slice(clock.active_regime_id.as_bytes());
        for e in &self.pending {
            semantics.extend_from_slice(e.event_context_digest.as_bytes());
        }
        let key = crate::digest_bytes(&semantics);
        if self.seen_cycle_keys.contains(&key) {
            return Err(CoupledTimeError::EventCycle);
        }
        self.seen_cycle_keys.push(key);
        let transition = EventTransitionV1::new(
            clock,
            self.tick,
            event.class,
            event.source_owner_id,
            event.event_context_digest,
            event.ending_owners,
            event.mutation_set,
            event.successor_regime_id,
            event.successor_participants,
            event.ledger_entries,
        )?;
        let receipt = apply_event(clock, transition)?;
        self.pending.remove(0);
        self.applied += 1;
        Ok(Some(receipt))
    }

    pub fn pending_event_join(
        &self,
        clock: &CoupledClockStateV1,
    ) -> Result<Option<PendingEventJoinV1>, CoupledTimeError> {
        let Some(event) = self.pending.first() else {
            return Ok(None);
        };
        if clock.accepted_until != self.tick {
            return Err(CoupledTimeError::EventTransition);
        }
        let transition = EventTransitionV1::new(
            clock,
            self.tick,
            event.class,
            event.source_owner_id.clone(),
            event.event_context_digest,
            event.ending_owners.clone(),
            event.mutation_set.clone(),
            event.successor_regime_id.clone(),
            event.successor_participants.clone(),
            event.ledger_entries.clone(),
        )?;
        Ok(Some(PendingEventJoinV1 {
            event_id: transition.event_id,
        }))
    }
}

fn apply_event(
    clock: &mut CoupledClockStateV1,
    event: EventTransitionV1,
) -> Result<AcceptedEventReceiptV1, CoupledTimeError> {
    let expected = EventTransitionV1::new(
        clock,
        event.tick_ns,
        event.class,
        event.source_owner_id.clone(),
        event.event_context_digest,
        event.ending_owners.clone(),
        event.mutation_set.clone(),
        event.successor_regime_id.clone(),
        event.successor_participants.clone(),
        event.ledger_entries.clone(),
    )?;
    if event.tick_ns != clock.accepted_until
        || event.ordinal != clock.event_ordinal
        || event.beginning_owner_set_digest != owner_set_digest(&clock.complete_owner_set)?
        || event.ledger_digest != ledger_digest(&event.ledger_entries)?
        || event != expected
        || clock
            .accepted_event_receipts
            .iter()
            .any(|r| r.id() == event.receipt.id())
    {
        return Err(CoupledTimeError::EventTransition);
    }
    let progress = event.ending_owners != clock.complete_owner_set
        || event.successor_regime_id != clock.active_regime_id
        || event.successor_participants != clock.active_participant_set;
    if !progress {
        return Err(CoupledTimeError::EventCycle);
    }
    clock.event_ordinal = clock
        .event_ordinal
        .checked_add(1)
        .ok_or(CoupledTimeError::ArithmeticOverflow)?;
    clock.segment_ordinal = clock
        .segment_ordinal
        .checked_add(1)
        .ok_or(CoupledTimeError::ArithmeticOverflow)?;
    clock.complete_owner_set = event.ending_owners;
    clock.active_regime_id = event.successor_regime_id;
    clock.active_segment_start = clock.accepted_until;
    clock.active_segment_end = clock.parent_support.end_ns();
    clock.active_participant_set = event.successor_participants;
    let mut participant_bytes = Vec::new();
    for participant in &clock.active_participant_set {
        participant_bytes.extend_from_slice(participant.as_bytes());
        participant_bytes.push(0);
    }
    clock.active_segment_id = crate::SegmentId::derive(
        clock.parent_transaction_id,
        clock.segment_ordinal,
        TimeSupport::new(clock.active_segment_start, clock.active_segment_end)?,
        crate::digest_bytes(clock.active_regime_id.as_bytes()),
        crate::digest_bytes(&participant_bytes),
    )?;
    clock.accepted_clock_digest = event.receipt.end_clock;
    clock.accepted_event_receipts.push(event.receipt.clone());
    Ok(event.receipt)
}
