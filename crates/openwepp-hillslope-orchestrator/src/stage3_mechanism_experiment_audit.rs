//! Explicit, thread-local observation sessions for prospective mechanism experiments.
//! No session is active in ordinary production. Compact mode never formats or hashes.

use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Compact,
    Detailed { max_records: usize },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[repr(usize)]
pub enum Kind {
    Outer = 0,
    Evaluator = 1,
    Provider = 2,
    Carrier = 3,
    BatchProvider = 4,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Lifecycle {
    pub started: u64,
    pub completed: u64,
    pub errors: u64,
}

#[derive(Debug, Serialize)]
pub struct Record {
    pub kind: Kind,
    pub ordinal: u64,
    pub outer_ordinal: u64,
    pub evaluator_ordinal: u64,
    pub provider_ordinal: u64,
    pub input: String,
    pub output: Option<String>,
    pub success: bool,
}

#[derive(Debug, Default, Serialize)]
pub struct Snapshot {
    /// Indexed by `Kind`; completed means successful completion, errors includes unwind.
    pub counts: [Lifecycle; 5],
    pub records: Vec<Record>,
    pub dropped_records: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuditError {
    AlreadyActive,
    MissingSession,
    OutstandingScopes,
    CounterOverflow,
}

struct State {
    identity: Rc<()>,
    mode: Mode,
    snapshot: Snapshot,
    current: [u64; 5],
    outstanding: u64,
    reserved: usize,
    overflow: bool,
}

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

/// A session must be finished on its originating thread. Dropping it cancels it.
#[must_use]
pub struct Session {
    active: bool,
    same_thread: PhantomData<Rc<()>>,
}

pub fn begin(mode: Mode) -> Result<Session, AuditError> {
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        if state.is_some() {
            return Err(AuditError::AlreadyActive);
        }
        *state = Some(State {
            identity: Rc::new(()),
            mode,
            snapshot: Snapshot::default(),
            current: [0; 5],
            outstanding: 0,
            reserved: 0,
            overflow: false,
        });
        Ok(Session {
            active: true,
            same_thread: PhantomData,
        })
    })
}

impl Session {
    pub fn finish(mut self) -> Result<Snapshot, AuditError> {
        self.active = false;
        STATE.with(|cell| {
            let state = cell.borrow_mut().take().ok_or(AuditError::MissingSession)?;
            if state.overflow {
                return Err(AuditError::CounterOverflow);
            }
            if state.outstanding != 0 {
                return Err(AuditError::OutstandingScopes);
            }
            Ok(state.snapshot)
        })
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if self.active {
            STATE.with(|cell| {
                cell.borrow_mut().take();
            });
        }
    }
}

fn increment(value: &mut u64, overflow: &mut bool) {
    if let Some(next) = value.checked_add(1) {
        *value = next;
    } else {
        *overflow = true;
    }
}

pub(crate) struct Scope {
    identity: Option<Rc<()>>,
    kind: Kind,
    previous: u64,
    active: bool,
    success: bool,
    record: Option<Record>,
}

impl Scope {
    pub(crate) fn begin(kind: Kind, input: impl FnOnce() -> String) -> Self {
        let mut scope = STATE.with(|cell| {
            let mut state = cell.borrow_mut();
            let mut scope = Self {
                identity: None,
                kind,
                previous: 0,
                active: false,
                success: false,
                record: None,
            };
            let Some(state) = state.as_mut() else {
                return scope;
            };
            scope.active = true;
            scope.identity = Some(state.identity.clone());
            increment(
                &mut state.snapshot.counts[kind as usize].started,
                &mut state.overflow,
            );
            increment(&mut state.outstanding, &mut state.overflow);
            let ordinal = state.snapshot.counts[kind as usize].started;
            scope.previous = state.current[kind as usize];
            state.current[kind as usize] = ordinal;
            if let Mode::Detailed { max_records } = state.mode {
                if state.reserved < max_records {
                    state.reserved += 1;
                    scope.record = Some(Record {
                        kind,
                        ordinal,
                        outer_ordinal: state.current[Kind::Outer as usize],
                        evaluator_ordinal: state.current[Kind::Evaluator as usize],
                        provider_ordinal: state.current[Kind::Provider as usize],
                        input: String::new(),
                        output: None,
                        success: false,
                    });
                } else {
                    increment(&mut state.snapshot.dropped_records, &mut state.overflow);
                }
            }
            scope
        });
        if let Some(record) = &mut scope.record {
            record.input = input();
        }
        scope
    }

    pub(crate) fn complete(&mut self, success: bool, output: impl FnOnce() -> String) {
        self.success = success;
        if let Some(record) = &mut self.record {
            record.success = success;
            record.output = Some(output());
        }
    }
}

impl Drop for Scope {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        STATE.with(|cell| {
            let mut state = cell.borrow_mut();
            let Some(state) = state.as_mut() else {
                return;
            };
            if !self
                .identity
                .as_ref()
                .is_some_and(|identity| Rc::ptr_eq(identity, &state.identity))
            {
                return;
            }
            let count = &mut state.snapshot.counts[self.kind as usize];
            increment(
                if self.success {
                    &mut count.completed
                } else {
                    &mut count.errors
                },
                &mut state.overflow,
            );
            if let Some(next) = state.outstanding.checked_sub(1) {
                state.outstanding = next;
            } else {
                state.overflow = true;
            }
            state.current[self.kind as usize] = self.previous;
            if let Some(record) = self.record.take() {
                state.snapshot.records.push(record);
            }
        });
    }
}

/// Debug bytes are a same-source experiment fingerprint, not a public serialization contract.
pub(crate) fn fingerprint(value: &impl std::fmt::Debug) -> String {
    format!("{:x}", Sha256::digest(format!("{value:?}").as_bytes()))
}

/// Exact boundary operand bits and canonical custody seals for the returned transition.
/// This attributes removed calls; full phase/output parity is a separate admission gate.
pub(crate) fn transition_fingerprint(
    value: &crate::hydrology::CoveredTerminalTrialTransitionV1,
) -> String {
    use crate::snow_stage3_terminal_handoff::Stage3BoundaryIdentity;
    let mut hash = Sha256::new();
    hash.update(b"stage3-mechanism-transition-v1");
    let boundary = &value.boundary;
    hash.update(boundary.support.start_ns().get().to_be_bytes());
    hash.update(boundary.support.end_ns().get().to_be_bytes());
    for operand in [
        boundary.sensible_energy_j_m2,
        boundary.vapor_mass_kg_m2,
        boundary.latent_energy_j_m2,
        boundary.shortwave_energy_j_m2,
        boundary.net_longwave_energy_j_m2,
        boundary.precipitation_advection_j_m2,
        boundary.snow_soil_heat_j_m2,
        boundary.latent_heat_j_kg,
    ] {
        hash.update(operand.to_bits().to_be_bytes());
    }
    hash.update(boundary.beginning_stage3_state_sha256.as_bytes());
    match boundary.identity {
        Stage3BoundaryIdentity::Provisional {
            carrier_receipt_sha256,
        } => {
            hash.update([0]);
            hash.update(carrier_receipt_sha256.as_bytes());
        }
        Stage3BoundaryIdentity::Final {
            provisional_carrier_receipt_sha256,
            optical_receipt_sha256,
            reciprocal_longwave_receipt_sha256,
            final_destination_receipt_sha256,
            final_lane_receipt_sha256,
        } => {
            hash.update([1]);
            for digest in [
                provisional_carrier_receipt_sha256,
                optical_receipt_sha256,
                reciprocal_longwave_receipt_sha256,
                final_destination_receipt_sha256,
                final_lane_receipt_sha256,
            ] {
                hash.update(digest.as_bytes());
            }
        }
    }
    hash.update(value.beginning_joint.receipt_sha256().as_bytes());
    hash.update(value.ending_joint.receipt_sha256().as_bytes());
    hash.update(value.probe_child_identity.receipt_sha256.as_bytes());
    if let Some(receipt) = &value.trial_snow_soil_receipt {
        hash.update([1]);
        hash.update(receipt.receipt_sha256.as_bytes());
    } else {
        hash.update([0]);
    }
    format!("{:x}", hash.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_compact_session_never_evaluates_details_and_accounts_errors() {
        let session = begin(Mode::Compact).unwrap();
        assert!(matches!(
            begin(Mode::Compact),
            Err(AuditError::AlreadyActive)
        ));
        {
            let mut outer = Scope::begin(Kind::Outer, || panic!("compact input"));
            {
                let _error = Scope::begin(Kind::Provider, || panic!("compact error"));
            }
            outer.complete(true, || panic!("compact output"));
        }
        let result = session.finish().unwrap();
        assert_eq!(
            result.counts[0],
            Lifecycle {
                started: 1,
                completed: 1,
                errors: 0
            }
        );
        assert_eq!(
            result.counts[2],
            Lifecycle {
                started: 1,
                completed: 0,
                errors: 1
            }
        );
        assert!(result.records.is_empty());
    }

    #[test]
    fn equal_payloads_have_distinct_authentic_invocation_ordinals_and_bounded_records() {
        let session = begin(Mode::Detailed { max_records: 3 }).unwrap();
        for _ in 0..2 {
            let mut outer = Scope::begin(Kind::Outer, || "same".into());
            let mut evaluator = Scope::begin(Kind::Evaluator, || "same".into());
            evaluator.complete(true, || "same-result".into());
            outer.complete(true, || "same-result".into());
        }
        let result = session.finish().unwrap();
        assert_eq!(result.records.len(), 3);
        assert_eq!(result.dropped_records, 1);
        let outers: Vec<_> = result
            .records
            .iter()
            .filter(|r| r.kind == Kind::Outer)
            .collect();
        assert_eq!(outers[0].outer_ordinal, 1);
        assert_eq!(outers[1].outer_ordinal, 2);
        assert_eq!(outers[0].input, outers[1].input);
    }

    #[test]
    fn successful_provider_does_not_complete_failed_evaluator_tail() {
        fn evaluate(fail_join: bool) -> Result<(), &'static str> {
            let mut evaluator = Scope::begin(Kind::Evaluator, String::new);
            {
                let mut provider = Scope::begin(Kind::Provider, String::new);
                provider.complete(true, String::new);
            }
            if fail_join {
                return Err("boundary join");
            }
            evaluator.complete(true, String::new);
            Ok(())
        }
        let session = begin(Mode::Detailed { max_records: 4 }).unwrap();
        assert_eq!(evaluate(true), Err("boundary join"));
        assert_eq!(evaluate(false), Ok(()));
        let snapshot = session.finish().unwrap();
        assert_eq!(
            snapshot.counts[Kind::Provider as usize],
            Lifecycle {
                started: 2,
                completed: 2,
                errors: 0,
            }
        );
        assert_eq!(
            snapshot.counts[Kind::Evaluator as usize],
            Lifecycle {
                started: 2,
                completed: 1,
                errors: 1,
            }
        );
        let failed = snapshot
            .records
            .iter()
            .find(|record| record.kind == Kind::Evaluator && record.ordinal == 1)
            .unwrap();
        assert!(!failed.success);

        // Bind the failure-tail scope semantics to the real resolved evaluator seam.
        let source = include_str!(
            "hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs"
        );
        let resolved = source
            .split("let mut mechanism_resolved_evaluator")
            .nth(1)
            .unwrap();
        let completion = resolved
            .find("observer.complete(true, String::new)")
            .unwrap();
        assert!(
            resolved
                .find("snow.resolved_terminal_trial_boundary_join")
                .unwrap()
                < completion
        );
        assert!(
            resolved
                .find("snow.stage3_shadow_energy_residual_j_m2")
                .unwrap()
                < completion
        );
        assert!(!resolved[..completion].contains("mechanism_evaluator.complete"));
    }

    #[test]
    fn unfinished_scopes_and_overflow_reject_evidence() {
        let session = begin(Mode::Compact).unwrap();
        let scope = Scope::begin(Kind::Carrier, String::new);
        assert!(matches!(
            session.finish(),
            Err(AuditError::OutstandingScopes)
        ));
        drop(scope);
        let session = begin(Mode::Compact).unwrap();
        STATE
            .with(|cell| cell.borrow_mut().as_mut().unwrap().snapshot.counts[3].started = u64::MAX);
        drop(Scope::begin(Kind::Carrier, String::new));
        assert!(matches!(session.finish(), Err(AuditError::CounterOverflow)));
    }
}
