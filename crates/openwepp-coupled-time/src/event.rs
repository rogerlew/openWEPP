use serde::{Deserialize, Serialize};

use crate::{
    CoupledClockStateV1, CoupledTimeError, Digest32, EventId, ModelTimeNs, OwnerState, ReceiptId,
    clock::validate_owner_and_participant_sets,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum EventClass {
    OwnershipTransfer = 0,
    BoundaryModeTransition = 1,
    RegimeTransition = 2,
    ScheduledBoundary = 3,
    DiagnosticMarker = 4,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventTransitionV1 {
    pub event_id: EventId,
    pub tick_ns: ModelTimeNs,
    pub class: EventClass,
    pub source_owner_id: String,
    pub event_context_digest: Digest32,
    pub beginning_owner_set_digest: Digest32,
    pub ending_owners: Vec<OwnerState>,
    pub successor_regime_id: String,
    pub successor_participants: Vec<String>,
    pub ledger_digest: Digest32,
    pub ledger_closed: bool,
    pub receipt_id: ReceiptId,
}

pub fn apply_event(
    clock: &mut CoupledClockStateV1,
    transition: &EventTransitionV1,
) -> Result<(), CoupledTimeError> {
    if transition.tick_ns != clock.accepted_until
        || clock
            .accepted_event_receipts
            .contains(&transition.receipt_id)
    {
        return Err(CoupledTimeError::EventTransition);
    }
    if !transition.ledger_closed {
        return Err(CoupledTimeError::LedgerFailure);
    }
    validate_owner_and_participant_sets(
        &transition.ending_owners,
        &transition.successor_participants,
    )?;
    if transition.ending_owners.len() != clock.complete_owner_set.len()
        || transition
            .ending_owners
            .iter()
            .zip(&clock.complete_owner_set)
            .any(|(a, b)| a.owner_id != b.owner_id)
    {
        return Err(CoupledTimeError::OwnerCandidate);
    }
    let physical_progress = transition.ending_owners != clock.complete_owner_set
        || transition.successor_regime_id != clock.active_regime_id
        || transition.successor_participants != clock.active_participant_set;
    if !physical_progress {
        return Err(CoupledTimeError::EventCycle);
    }
    let event_ordinal = clock
        .event_ordinal
        .checked_add(1)
        .ok_or(CoupledTimeError::ArithmeticOverflow)?;
    let segment_ordinal = clock
        .segment_ordinal
        .checked_add(1)
        .ok_or(CoupledTimeError::ArithmeticOverflow)?;
    clock
        .complete_owner_set
        .clone_from(&transition.ending_owners);
    clock
        .active_regime_id
        .clone_from(&transition.successor_regime_id);
    clock
        .active_participant_set
        .clone_from(&transition.successor_participants);
    clock.event_ordinal = event_ordinal;
    clock.segment_ordinal = segment_ordinal;
    clock.accepted_event_receipts.push(transition.receipt_id);
    Ok(())
}

pub fn order_events(events: &mut [EventTransitionV1]) -> Result<(), CoupledTimeError> {
    events.sort_by_key(|e| (e.class, e.source_owner_id.clone(), e.event_context_digest));
    if events.windows(2).any(|w| w[0].event_id == w[1].event_id) {
        return Err(CoupledTimeError::EventTransition);
    }
    if events.len() > 256 {
        return Err(CoupledTimeError::EventCycle);
    }
    Ok(())
}
