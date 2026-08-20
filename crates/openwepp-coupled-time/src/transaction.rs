use crate::{
    AcceptedSlabId, ConstraintReductionReceiptV1, CoupledClockStateV1, CoupledTimeError, Digest32,
    FramedField, OwnerState, ParentTransactionId, ReceiptId, SegmentId, TimeSupport, digest_bytes,
    framed_sha256,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LedgerEntryV1 {
    flux_id: String,
    units: String,
    debit_digest: Digest32,
    credit_digest: Digest32,
    operand_lineage_digest: Digest32,
}
impl LedgerEntryV1 {
    pub fn new(
        flux_id: String,
        units: String,
        debit: Digest32,
        credit: Digest32,
        lineage: Digest32,
    ) -> Result<Self, CoupledTimeError> {
        if flux_id.is_empty() || units.is_empty() || debit != credit {
            return Err(CoupledTimeError::LedgerFailure);
        }
        Ok(Self {
            flux_id,
            units,
            debit_digest: debit,
            credit_digest: credit,
            operand_lineage_digest: lineage,
        })
    }
}

pub(crate) fn owner_set_digest(owners: &[OwnerState]) -> Result<Digest32, CoupledTimeError> {
    let count = u32::try_from(owners.len()).map_err(|_| CoupledTimeError::ArithmeticOverflow)?;
    let mut bytes = Vec::new();
    for owner in owners {
        let id = owner.owner_id().as_bytes();
        bytes.extend_from_slice(
            &u32::try_from(id.len())
                .map_err(|_| CoupledTimeError::ArithmeticOverflow)?
                .to_be_bytes(),
        );
        bytes.extend_from_slice(id);
        bytes.extend_from_slice(owner.state_digest().as_bytes());
    }
    framed_sha256(
        "owner-set",
        &[
            FramedField {
                tag: "owner_count",
                value: &count.to_be_bytes(),
            },
            FramedField {
                tag: "ordered_owner_records",
                value: &bytes,
            },
        ],
    )
}
pub fn complete_owner_set_digest(owners: &[OwnerState]) -> Result<Digest32, CoupledTimeError> {
    crate::clock::validate_owner_and_participant_sets(owners, &[])?;
    owner_set_digest(owners)
}
pub(crate) fn ledger_digest(entries: &[LedgerEntryV1]) -> Result<Digest32, CoupledTimeError> {
    if entries.is_empty() || entries.windows(2).any(|w| w[0].flux_id >= w[1].flux_id) {
        return Err(CoupledTimeError::LedgerFailure);
    }
    let mut bytes = Vec::new();
    for e in entries {
        if e.debit_digest != e.credit_digest {
            return Err(CoupledTimeError::LedgerFailure);
        }
        bytes.extend_from_slice(e.flux_id.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(e.units.as_bytes());
        bytes.extend_from_slice(e.debit_digest.as_bytes());
        bytes.extend_from_slice(e.operand_lineage_digest.as_bytes());
    }
    Ok(digest_bytes(&bytes))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AcceptedSlabReceiptV1 {
    pub(crate) receipt_id: ReceiptId,
    pub(crate) slab_id: AcceptedSlabId,
    pub(crate) parent_transaction_id: ParentTransactionId,
    pub(crate) slab_ordinal: u32,
    pub(crate) segment_id: SegmentId,
    pub(crate) support: TimeSupport,
    pub(crate) duration_bits: u64,
    pub(crate) constraint_digest: Digest32,
    pub(crate) begin_clock: Digest32,
    pub(crate) end_clock: Digest32,
    pub(crate) begin_owner_set: Digest32,
    pub(crate) end_owner_set: Digest32,
    pub(crate) owner_candidate_set: Digest32,
    pub(crate) ledger_digest: Digest32,
}
impl AcceptedSlabReceiptV1 {
    #[must_use]
    pub const fn id(&self) -> ReceiptId {
        self.receipt_id
    }
    #[must_use]
    pub const fn support(&self) -> TimeSupport {
        self.support
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CoupledSlabCandidateV1 {
    parent: ParentTransactionId,
    segment: SegmentId,
    ordinal: u32,
    support: TimeSupport,
    duration_s_bits: u64,
    constraint: ConstraintReductionReceiptV1,
    begin_owner_set: Digest32,
    ending_owners: Vec<OwnerState>,
    end_owner_set: Digest32,
    ledger_entries: Vec<LedgerEntryV1>,
    ledger_digest: Digest32,
    slab_id: AcceptedSlabId,
    receipt: AcceptedSlabReceiptV1,
}
impl CoupledSlabCandidateV1 {
    #[allow(clippy::too_many_lines)]
    pub fn new(
        clock: &CoupledClockStateV1,
        segment: SegmentId,
        support: TimeSupport,
        constraint: &ConstraintReductionReceiptV1,
        ending_owners: Vec<OwnerState>,
        ledger_entries: Vec<LedgerEntryV1>,
    ) -> Result<Self, CoupledTimeError> {
        constraint.validate_identity()?;
        if segment != clock.active_segment_id
            || !constraint.matches_clock(clock.parent_transaction_id, clock.accepted_until)
            || support.start_ns() != clock.accepted_until
            || support.end_ns() > clock.active_segment_end
            || constraint.proposed_end() != support.end_ns()
        {
            return Err(CoupledTimeError::ParentMismatch);
        }
        crate::clock::validate_owner_and_participant_sets(
            &ending_owners,
            &clock.active_participant_set,
        )?;
        if ending_owners.len() != clock.complete_owner_set.len()
            || ending_owners
                .iter()
                .zip(&clock.complete_owner_set)
                .any(|(a, b)| a.owner_id() != b.owner_id())
        {
            return Err(CoupledTimeError::OwnerCandidate);
        }
        for (before, after) in clock.complete_owner_set.iter().zip(&ending_owners) {
            if !clock
                .active_participant_set
                .iter()
                .any(|p| p == before.owner_id())
                && before != after
            {
                return Err(CoupledTimeError::OwnerCandidate);
            }
        }
        let begin = owner_set_digest(&clock.complete_owner_set)?;
        let end = owner_set_digest(&ending_owners)?;
        let ledger = ledger_digest(&ledger_entries)?;
        let duration = support.duration_s_bits();
        let ordinal_b = clock.slab_ordinal.to_be_bytes();
        let start = support.start_ns().get().to_be_bytes();
        let end_tick = support.end_ns().get().to_be_bytes();
        let duration_b = duration.to_be_bytes();
        let slab_digest = framed_sha256(
            "accepted-slab",
            &[
                FramedField {
                    tag: "parent_transaction_id",
                    value: clock.parent_transaction_id.digest().as_bytes(),
                },
                FramedField {
                    tag: "slab_ordinal",
                    value: &ordinal_b,
                },
                FramedField {
                    tag: "segment_id",
                    value: segment.digest().as_bytes(),
                },
                FramedField {
                    tag: "start_ns",
                    value: &start,
                },
                FramedField {
                    tag: "end_ns",
                    value: &end_tick,
                },
                FramedField {
                    tag: "duration_bits",
                    value: &duration_b,
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
                    tag: "constraint_digest",
                    value: constraint.digest().as_bytes(),
                },
                FramedField {
                    tag: "ledger_digest",
                    value: ledger.as_bytes(),
                },
            ],
        )?;
        let slab_id = AcceptedSlabId::from_digest(slab_digest);
        let owner_candidate_set = digest_bytes(
            &serde_json::to_vec(&ending_owners)
                .map_err(|_| CoupledTimeError::NonCanonicalIdentity)?,
        );
        let begin_clock = clock.accepted_clock_digest;
        let next_ordinal = clock
            .slab_ordinal
            .checked_add(1)
            .ok_or(CoupledTimeError::ArithmeticOverflow)?;
        let end_clock = digest_bytes(
            &[
                &support.end_ns().get().to_be_bytes()[..],
                &next_ordinal.to_be_bytes()[..],
                end.as_bytes(),
            ]
            .concat(),
        );
        let receipt_digest = framed_sha256(
            "slab-receipt-v2",
            &[
                FramedField {
                    tag: "parent_transaction_id",
                    value: clock.parent_transaction_id.digest().as_bytes(),
                },
                FramedField {
                    tag: "accepted_slab_id",
                    value: slab_digest.as_bytes(),
                },
                FramedField {
                    tag: "slab_ordinal",
                    value: &ordinal_b,
                },
                FramedField {
                    tag: "segment_id",
                    value: segment.digest().as_bytes(),
                },
                FramedField {
                    tag: "start_ns",
                    value: &start,
                },
                FramedField {
                    tag: "end_ns",
                    value: &end_tick,
                },
                FramedField {
                    tag: "duration_bits",
                    value: &duration_b,
                },
                FramedField {
                    tag: "constraint_digest",
                    value: constraint.digest().as_bytes(),
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
                    tag: "owner_candidate_set",
                    value: owner_candidate_set.as_bytes(),
                },
                FramedField {
                    tag: "ledger_digest",
                    value: ledger.as_bytes(),
                },
            ],
        )?;
        let receipt = AcceptedSlabReceiptV1 {
            receipt_id: ReceiptId::from_digest(receipt_digest),
            slab_id,
            parent_transaction_id: clock.parent_transaction_id,
            slab_ordinal: clock.slab_ordinal,
            segment_id: segment,
            support,
            duration_bits: duration,
            constraint_digest: constraint.digest(),
            begin_clock,
            end_clock,
            begin_owner_set: begin,
            end_owner_set: end,
            owner_candidate_set,
            ledger_digest: ledger,
        };
        Ok(Self {
            parent: clock.parent_transaction_id,
            segment,
            ordinal: clock.slab_ordinal,
            support,
            duration_s_bits: duration,
            constraint: constraint.clone(),
            begin_owner_set: begin,
            ending_owners,
            end_owner_set: end,
            ledger_entries,
            ledger_digest: ledger,
            slab_id,
            receipt,
        })
    }
}
pub fn accept_slab(
    clock: &mut CoupledClockStateV1,
    slab: CoupledSlabCandidateV1,
) -> Result<AcceptedSlabReceiptV1, CoupledTimeError> {
    let expected = CoupledSlabCandidateV1::new(
        clock,
        slab.segment,
        slab.support,
        &slab.constraint,
        slab.ending_owners.clone(),
        slab.ledger_entries.clone(),
    )?;
    if slab.parent != clock.parent_transaction_id
        || slab.ordinal != clock.slab_ordinal
        || slab.support.start_ns() != clock.accepted_until
        || slab.duration_s_bits != slab.support.duration_s_bits()
        || slab.begin_owner_set != owner_set_digest(&clock.complete_owner_set)?
        || slab.end_owner_set != owner_set_digest(&slab.ending_owners)?
        || slab.ledger_digest != ledger_digest(&slab.ledger_entries)?
        || slab != expected
    {
        return Err(CoupledTimeError::OwnerCandidate);
    }
    if clock
        .accepted_slab_receipts
        .iter()
        .any(|r| r.id() == slab.receipt.id())
    {
        return Err(CoupledTimeError::OwnerCandidate);
    }
    clock.slab_ordinal = clock
        .slab_ordinal
        .checked_add(1)
        .ok_or(CoupledTimeError::ArithmeticOverflow)?;
    clock.last_accepted_step_ns = Some(slab.support.duration_ns());
    clock.accepted_until = slab.support.end_ns();
    clock.accepted_clock_digest = slab.receipt.end_clock;
    clock.complete_owner_set = slab.ending_owners;
    clock.accepted_slab_receipts.push(slab.receipt.clone());
    Ok(slab.receipt)
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DiagnosticReductionV1 {
    pub(crate) reduction_id: String,
    pub(crate) units: String,
    pub(crate) maximum: Option<f64>,
    pub(crate) accepted_receipts: Vec<ReceiptId>,
    pub(crate) accepted_values: Vec<(ReceiptId, f64)>,
    pub(crate) operator: ReductionOperatorV1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub(crate) enum ReductionOperatorV1 {
    Maximum,
    Minimum,
    Sum,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcceptedReductionOperandV1 {
    receipt_id: ReceiptId,
}
impl AcceptedReductionOperandV1 {
    #[must_use]
    pub const fn from_slab(receipt: &AcceptedSlabReceiptV1) -> Self {
        Self {
            receipt_id: receipt.id(),
        }
    }
    #[must_use]
    pub const fn from_event(receipt: &crate::AcceptedEventReceiptV1) -> Self {
        Self {
            receipt_id: receipt.id(),
        }
    }
    #[must_use]
    pub const fn from_scheduled(receipt: &crate::ScheduledOnceReceiptV1) -> Self {
        Self {
            receipt_id: receipt.id(),
        }
    }
}
impl DiagnosticReductionV1 {
    pub fn new(reduction_id: String, units: String) -> Result<Self, CoupledTimeError> {
        if reduction_id.is_empty() || units.is_empty() {
            return Err(CoupledTimeError::LedgerFailure);
        }
        Ok(Self {
            reduction_id,
            units,
            maximum: None,
            accepted_receipts: Vec::new(),
            accepted_values: Vec::new(),
            operator: ReductionOperatorV1::Maximum,
        })
    }
    pub fn new_minimum(reduction_id: String, units: String) -> Result<Self, CoupledTimeError> {
        let mut value = Self::new(reduction_id, units)?;
        value.operator = ReductionOperatorV1::Minimum;
        Ok(value)
    }
    pub fn new_sum(reduction_id: String, units: String) -> Result<Self, CoupledTimeError> {
        let mut value = Self::new(reduction_id, units)?;
        value.operator = ReductionOperatorV1::Sum;
        Ok(value)
    }
    pub fn fold_accepted(
        &mut self,
        value: f64,
        receipt: &AcceptedSlabReceiptV1,
    ) -> Result<(), CoupledTimeError> {
        self.fold_accepted_operand(value, AcceptedReductionOperandV1::from_slab(receipt))
    }
    pub fn fold_accepted_operand(
        &mut self,
        value: f64,
        operand: AcceptedReductionOperandV1,
    ) -> Result<(), CoupledTimeError> {
        if !value.is_finite() || self.accepted_receipts.contains(&operand.receipt_id) {
            return Err(CoupledTimeError::LedgerFailure);
        }
        let reduced = self.maximum.map_or(value, |old| match self.operator {
            ReductionOperatorV1::Maximum => retain_maximum(old, value),
            ReductionOperatorV1::Minimum => retain_minimum(old, value),
            ReductionOperatorV1::Sum => old + value,
        });
        if !reduced.is_finite() {
            return Err(CoupledTimeError::LedgerFailure);
        }
        self.maximum = Some(reduced);
        self.accepted_receipts.push(operand.receipt_id);
        self.accepted_values.push((operand.receipt_id, value));
        Ok(())
    }
    #[must_use]
    pub const fn maximum(&self) -> Option<f64> {
        self.maximum
    }
}

pub(crate) fn retain_maximum(retained: f64, candidate: f64) -> f64 {
    if candidate > retained {
        candidate
    } else {
        retained
    }
}
pub(crate) fn retain_minimum(retained: f64, candidate: f64) -> f64 {
    if candidate < retained {
        candidate
    } else {
        retained
    }
}

#[cfg(test)]
mod reduction_tests {
    use super::{retain_maximum, retain_minimum};

    #[test]
    fn maximum_retains_first_exact_bits_on_numeric_equality() {
        assert_eq!(retain_maximum(-0.0, 0.0).to_bits(), (-0.0_f64).to_bits());
        assert_eq!(retain_maximum(0.0, -0.0).to_bits(), 0.0_f64.to_bits());
        assert_eq!(retain_maximum(4.0, 4.0).to_bits(), 4.0_f64.to_bits());
        assert_eq!(retain_minimum(-0.0, 0.0).to_bits(), (-0.0_f64).to_bits());
        assert_eq!(retain_minimum(0.0, -0.0).to_bits(), 0.0_f64.to_bits());
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalOperatorClass {
    AlgebraicRate,
    SupportIntegral,
    SequentialStateTransition,
    ThresholdEvent,
    ScheduledOnce,
    DiagnosticReduction,
}
