use thiserror::Error;

/// Closed, precedence-ordered coupled-time failure family.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CoupledTimeError {
    #[error("ERR-CT-001 invalid support")]
    InvalidSupport,
    #[error("ERR-CT-002 arithmetic overflow")]
    ArithmeticOverflow,
    #[error("ERR-CT-003 noncanonical identity or owner set")]
    NonCanonicalIdentity,
    #[error("ERR-CT-004 parent or cursor mismatch")]
    ParentMismatch,
    #[error("ERR-CT-005 invalid participant set")]
    InvalidParticipantSet,
    #[error("ERR-CT-006 constraint outside accepted chronology")]
    InvalidConstraint,
    #[error("ERR-CT-007 zero step without an admitted event")]
    ZeroStepWithoutEvent,
    #[error("ERR-CT-008 conflicting equal-time constraints")]
    ConstraintConflict,
    #[error("ERR-CT-009 controller policy mismatch")]
    ControllerPolicyMismatch,
    #[error("ERR-CT-010 invalid owner candidate")]
    OwnerCandidate,
    #[error("ERR-CT-011 ledger join failure")]
    LedgerFailure,
    #[error("ERR-CT-012 event transition failure or replay")]
    EventTransition,
    #[error("ERR-CT-013 event cycle or transition limit")]
    EventCycle,
    #[error("ERR-CT-014 retry exhausted or repeated without progress")]
    RetryExhausted,
    #[error("ERR-CT-015 invalid restart state")]
    RestartInvalid,
    #[error("ERR-CT-016 scheduled-once replay")]
    ScheduledOnceReplay,
    #[error("ERR-CT-017 parent is not finalizable")]
    ParentNotFinalizable,
    #[error("ERR-CT-018 publication before parent commit")]
    PublicationBeforeParentCommit,
    #[error("ERR-CT-019 invalid publication outbox transition")]
    OutboxTransition,
    #[error("ERR-CT-020 event proposal cannot be represented")]
    EventProposal,
}
