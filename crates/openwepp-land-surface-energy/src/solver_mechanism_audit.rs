//! Explicit, thread-local observation for prospective mechanism experiments.
//! Counters never select physics. Detailed traces are bounded and run separately.
#![allow(clippy::missing_errors_doc)]

use serde::Serialize;
use std::{cell::RefCell, marker::PhantomData, rc::Rc};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Kind {
    Map,
    Solve,
    Sweep,
    Probe,
    Evaluation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ProbeClass {
    Complete,
    IdentityAnchor,
    ComponentReplay,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Counts {
    pub starts: u64,
    pub completions: u64,
    pub errors: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct MechanismAudit {
    pub maps: Counts,
    pub solves: Counts,
    pub sweeps: Counts,
    pub probes: Counts,
    pub evaluations: Counts,
    pub complete: Counts,
    pub identity_anchor: Counts,
    pub component_replay: Counts,
    pub iterations: u64,
    pub leaf_calls: u64,
    pub dropped_events: u64,
    pub overflow: bool,
    pub events: Vec<Event>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Event {
    Start {
        kind: Kind,
        id: u64,
        parent: Option<u64>,
    },
    End {
        kind: Kind,
        id: u64,
        success: bool,
    },
    MapJoin {
        map: u64,
    },
    Iteration {
        id: u64,
        solve: Option<u64>,
        ordinal: u32,
    },
    SweepBase {
        sweep: u64,
        iteration: u64,
        base_bits: Vec<u64>,
        occupancies: usize,
        soil_nodes: usize,
        represented_snow: bool,
        liquid_ground: bool,
    },
    Stencil {
        sweep: u64,
        coordinate: usize,
        minus_bits: u64,
        plus_bits: u64,
        minus_valid: bool,
        plus_valid: bool,
        identity_anchor_bits: Option<u64>,
    },
    Probe {
        id: u64,
        coordinate: usize,
        value_bits: Option<u64>,
        class: ProbeClass,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum AuditError {
    #[error("mechanism observation already active on this thread")]
    AlreadyActive,
    #[error("mechanism observation session is not active")]
    NotActive,
    #[error("mechanism observation has live scopes")]
    LiveScopes,
    #[error("mechanism observation counter or identity overflow")]
    Overflow,
}

struct State {
    audit: MechanismAudit,
    detailed: bool,
    limit: usize,
    next: u64,
    generation: u64,
    parent: Option<u64>,
    solve: Option<u64>,
    iteration: u64,
    live: u64,
}
thread_local! { static ACTIVE: RefCell<Option<State>> = const { RefCell::new(None) }; }
#[cfg(any(test, feature = "test-support"))]
static GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn increment(value: &mut u64, overflow: &mut bool) {
    if let Some(next) = value.checked_add(1) {
        *value = next;
    } else {
        *overflow = true;
    }
}
impl State {
    fn id(&mut self) -> u64 {
        increment(&mut self.next, &mut self.audit.overflow);
        self.next
    }
    fn event(&mut self, make: impl FnOnce() -> Event) {
        if self.detailed {
            if self.audit.events.len() < self.limit {
                self.audit.events.push(make());
            } else {
                increment(&mut self.audit.dropped_events, &mut self.audit.overflow);
            }
        }
    }
    fn counts(&mut self, kind: Kind) -> &mut Counts {
        match kind {
            Kind::Map => &mut self.audit.maps,
            Kind::Solve => &mut self.audit.solves,
            Kind::Sweep => &mut self.audit.sweeps,
            Kind::Probe => &mut self.audit.probes,
            Kind::Evaluation => &mut self.audit.evaluations,
        }
    }
    fn bump(&mut self, kind: Kind, completed: Option<bool>) {
        let counts = self.counts(kind);
        let value = match completed {
            None => &mut counts.starts,
            Some(true) => &mut counts.completions,
            Some(false) => &mut counts.errors,
        };
        let mut overflow = false;
        increment(value, &mut overflow);
        self.audit.overflow |= overflow;
    }
    fn class(&mut self, class: ProbeClass, completed: Option<bool>) {
        let counts = match class {
            ProbeClass::Complete => &mut self.audit.complete,
            ProbeClass::IdentityAnchor => &mut self.audit.identity_anchor,
            ProbeClass::ComponentReplay => &mut self.audit.component_replay,
        };
        let value = match completed {
            None => &mut counts.starts,
            Some(true) => &mut counts.completions,
            Some(false) => &mut counts.errors,
        };
        increment(value, &mut self.audit.overflow);
    }
}

/// Consumed session; dropping it tears down observation even during unwinding.
pub struct Session {
    _thread: PhantomData<Rc<()>>,
}

#[cfg(any(test, feature = "test-support"))]
pub fn begin_mechanism_audit(detailed: bool, max_events: usize) -> Result<Session, AuditError> {
    ACTIVE.with(|active| {
        let mut active = active.borrow_mut();
        if active.is_some() {
            return Err(AuditError::AlreadyActive);
        }
        let generation = GENERATION
            .fetch_update(
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
                |g| g.checked_add(1),
            )
            .map_err(|_| AuditError::Overflow)?;
        *active = Some(State {
            audit: MechanismAudit::default(),
            detailed,
            limit: max_events,
            next: 0,
            generation,
            parent: None,
            solve: None,
            iteration: 0,
            live: 0,
        });
        Ok(Session {
            _thread: PhantomData,
        })
    })
}
impl Session {
    pub fn finish(self) -> Result<MechanismAudit, AuditError> {
        ACTIVE.with(|active| {
            let state = active.borrow_mut().take().ok_or(AuditError::NotActive)?;
            if state.live != 0 {
                return Err(AuditError::LiveScopes);
            }
            if state.audit.overflow {
                return Err(AuditError::Overflow);
            }
            Ok(state.audit)
        })
    }
}
impl Drop for Session {
    fn drop(&mut self) {
        ACTIVE.with(|a| {
            a.borrow_mut().take();
        });
    }
}

pub(crate) struct Scope {
    id: Option<u64>,
    generation: u64,
    detailed: bool,
    kind: Kind,
    parent: Option<u64>,
    solve: Option<u64>,
    success: bool,
    class: Option<ProbeClass>,
}
impl Scope {
    pub(crate) fn new(kind: Kind) -> Self {
        let mut scope = Self {
            id: None,
            generation: 0,
            detailed: false,
            kind,
            parent: None,
            solve: None,
            success: false,
            class: None,
        };
        ACTIVE.with(|active| {
            if let Some(s) = active.borrow_mut().as_mut() {
                let id = s.id();
                scope.id = Some(id);
                scope.generation = s.generation;
                scope.detailed = s.detailed;
                scope.parent = s.parent;
                scope.solve = s.solve;
                s.bump(kind, None);
                increment(&mut s.live, &mut s.audit.overflow);
                let parent = s.parent;
                s.event(|| Event::Start { kind, id, parent });
                s.parent = Some(id);
                if kind == Kind::Solve {
                    s.solve = Some(id);
                }
            }
        });
        scope
    }
    pub(crate) fn finish<T, E>(mut self, result: Result<T, E>) -> Result<T, E> {
        self.success = result.is_ok();
        result
    }
    pub(crate) fn complete(&mut self) {
        self.success = true;
    }
    pub(crate) fn detailed(&self) -> bool {
        self.detailed
    }
    pub(crate) fn map_token(&self) -> MapToken {
        MapToken {
            id: self.id,
            generation: self.generation,
        }
    }
    pub(crate) fn probe(&mut self, coordinate: usize, value_bits: Option<u64>, class: ProbeClass) {
        if let Some(id) = self.id {
            ACTIVE.with(|a| {
                if let Some(s) = a.borrow_mut().as_mut() {
                    if s.generation != self.generation {
                        return;
                    }
                    s.class(class, None);
                    s.event(|| Event::Probe {
                        id,
                        coordinate,
                        value_bits,
                        class,
                    });
                }
            });
            self.class = Some(class);
        }
    }
    pub(crate) fn sweep_base(
        &self,
        trial: &[f64],
        occupancies: usize,
        soil_nodes: usize,
        represented_snow: bool,
        liquid_ground: bool,
    ) {
        if let Some(sweep) = self.id {
            ACTIVE.with(|a| {
                if let Some(s) = a.borrow_mut().as_mut() {
                    if s.generation != self.generation {
                        return;
                    }
                    let iteration = s.iteration;
                    s.event(|| Event::SweepBase {
                        sweep,
                        iteration,
                        base_bits: trial.iter().map(|v| v.to_bits()).collect(),
                        occupancies,
                        soil_nodes,
                        represented_snow,
                        liquid_ground,
                    });
                }
            });
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn stencil(
        &self,
        coordinate: usize,
        minus: f64,
        plus: f64,
        minus_valid: bool,
        plus_valid: bool,
        identity_anchor: Option<f64>,
    ) {
        if let Some(sweep) = self.id {
            ACTIVE.with(|a| {
                if let Some(s) = a.borrow_mut().as_mut() {
                    if s.generation != self.generation {
                        return;
                    }
                    s.event(|| Event::Stencil {
                        sweep,
                        coordinate,
                        minus_bits: minus.to_bits(),
                        plus_bits: plus.to_bits(),
                        minus_valid,
                        plus_valid,
                        identity_anchor_bits: identity_anchor.map(f64::to_bits),
                    });
                }
            });
        }
    }
}
impl Drop for Scope {
    fn drop(&mut self) {
        if let Some(id) = self.id {
            ACTIVE.with(|a| {
                if let Some(s) = a.borrow_mut().as_mut() {
                    if s.generation != self.generation {
                        return;
                    }
                    s.bump(self.kind, Some(self.success));
                    if let Some(class) = self.class {
                        s.class(class, Some(self.success));
                    }
                    s.event(|| Event::End {
                        kind: self.kind,
                        id,
                        success: self.success,
                    });
                    s.parent = self.parent;
                    s.solve = self.solve;
                    if let Some(live) = s.live.checked_sub(1) {
                        s.live = live;
                    } else {
                        s.audit.overflow = true;
                    }
                }
            });
        }
    }
}

/// Observation metadata is deliberately excluded from physical phase equality.
/// The private identity is used solely for same-session lifecycle joins.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct MapToken {
    id: Option<u64>,
    generation: u64,
}
impl PartialEq for MapToken {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

pub(crate) struct MapJoin {
    previous: Option<u64>,
    generation: Option<u64>,
}
pub(crate) fn join_map(token: MapToken) -> MapJoin {
    let mut join = MapJoin {
        previous: None,
        generation: None,
    };
    ACTIVE.with(|a| {
        if let Some(s) = a.borrow_mut().as_mut() {
            if let Some(map) = token.id.filter(|_| token.generation == s.generation) {
                join.previous = s.parent;
                join.generation = Some(s.generation);
                s.parent = Some(map);
                s.event(|| Event::MapJoin { map });
                increment(&mut s.live, &mut s.audit.overflow);
            }
        }
    });
    join
}
impl Drop for MapJoin {
    fn drop(&mut self) {
        ACTIVE.with(|a| {
            if let Some(s) = a.borrow_mut().as_mut() {
                if self.generation == Some(s.generation) {
                    s.parent = self.previous;
                    if let Some(live) = s.live.checked_sub(1) {
                        s.live = live;
                    } else {
                        s.audit.overflow = true;
                    }
                }
            }
        });
    }
}

pub(crate) fn iteration(ordinal: u32) {
    ACTIVE.with(|a| {
        if let Some(s) = a.borrow_mut().as_mut() {
            increment(&mut s.audit.iterations, &mut s.audit.overflow);
            let id = s.id();
            s.iteration = id;
            let solve = s.solve;
            s.event(|| Event::Iteration { id, solve, ordinal });
        }
    });
}
pub(crate) fn leaf_call() {
    ACTIVE.with(|a| {
        if let Some(s) = a.borrow_mut().as_mut() {
            increment(&mut s.audit.leaf_calls, &mut s.audit.overflow);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compact_counts_and_errors_without_events() {
        let session = begin_mechanism_audit(false, 0).unwrap();
        assert!(matches!(
            begin_mechanism_audit(false, 0),
            Err(AuditError::AlreadyActive)
        ));
        let scope = Scope::new(Kind::Solve);
        iteration(0);
        leaf_call();
        let mut probe = Scope::new(Kind::Probe);
        probe.probe(2, Some(42), ProbeClass::IdentityAnchor);
        assert_eq!(probe.finish(Err::<(), _>(7)), Err(7));
        scope.finish(Ok::<_, ()>(())).unwrap();
        let a = session.finish().unwrap();
        assert_eq!(
            a.solves,
            Counts {
                starts: 1,
                completions: 1,
                errors: 0
            }
        );
        assert_eq!(
            a.identity_anchor,
            Counts {
                starts: 1,
                completions: 0,
                errors: 1
            }
        );
        assert_eq!((a.iterations, a.leaf_calls), (1, 1));
        assert!(a.events.is_empty());
    }
    #[test]
    fn detail_bound_drop_and_overflow_are_visible() {
        let session = begin_mechanism_audit(true, 1).unwrap();
        drop(Scope::new(Kind::Solve));
        let a = session.finish().unwrap();
        assert_eq!(a.events.len(), 1);
        assert_eq!(a.dropped_events, 1);
        assert_eq!(a.solves.errors, 1);
        drop(begin_mechanism_audit(false, 0).unwrap());
        let session = begin_mechanism_audit(false, 0).unwrap();
        ACTIVE.with(|a| a.borrow_mut().as_mut().unwrap().audit.leaf_calls = u64::MAX);
        leaf_call();
        assert_eq!(session.finish().unwrap_err(), AuditError::Overflow);
    }
    #[test]
    fn finish_rejects_open_lifecycle() {
        let session = begin_mechanism_audit(false, 0).unwrap();
        let scope = Scope::new(Kind::Solve);
        assert_eq!(session.finish().unwrap_err(), AuditError::LiveScopes);
        drop(scope);
        assert!(begin_mechanism_audit(false, 0).unwrap().finish().is_ok());
    }

    #[test]
    fn potential_and_final_solves_join_one_authentic_map() {
        let session = begin_mechanism_audit(true, 64).unwrap();
        let mut map = Scope::new(Kind::Map);
        let token = map.map_token();
        let potential = Scope::new(Kind::Solve);
        let potential_id = potential.id.unwrap();
        iteration(0);
        let mut sweep = Scope::new(Kind::Sweep);
        let sweep_id = sweep.id.unwrap();
        sweep.sweep_base(&[273.15, 1.0], 2, 6, true, true);
        sweep.complete();
        drop(sweep);
        potential.finish(Ok::<_, ()>(())).unwrap();
        map.complete();
        drop(map);
        let final_join = join_map(token);
        let final_solve = Scope::new(Kind::Solve);
        let final_id = final_solve.id.unwrap();
        final_solve.finish(Ok::<_, ()>(())).unwrap();
        drop(final_join);
        let audit = session.finish().unwrap();
        assert_eq!(
            audit.maps,
            Counts {
                starts: 1,
                completions: 1,
                errors: 0
            }
        );
        assert_eq!(
            audit.solves,
            Counts {
                starts: 2,
                completions: 2,
                errors: 0
            }
        );
        assert_eq!(audit.dropped_events, 0);
        let map_id = token.id.unwrap();
        assert_ne!(map_id, potential_id);
        assert_ne!(potential_id, final_id);
        for solve in [potential_id, final_id] {
            assert!(audit.events.contains(&Event::Start {
                kind: Kind::Solve,
                id: solve,
                parent: Some(map_id),
            }));
        }
        assert!(audit.events.contains(&Event::MapJoin { map: map_id }));
        let iteration_id = audit
            .events
            .iter()
            .find_map(|event| match event {
                Event::Iteration {
                    id,
                    solve: Some(solve),
                    ordinal: 0,
                } if *solve == potential_id => Some(*id),
                _ => None,
            })
            .unwrap();
        assert_ne!(iteration_id, potential_id);
        assert_ne!(iteration_id, sweep_id);
        assert!(audit.events.contains(&Event::SweepBase {
            sweep: sweep_id,
            iteration: iteration_id,
            base_bits: vec![273.15_f64.to_bits(), 1.0_f64.to_bits()],
            occupancies: 2,
            soil_nodes: 6,
            represented_snow: true,
            liquid_ground: true,
        }));
    }

    #[test]
    fn stale_tokens_scopes_and_joins_cannot_modify_a_new_session() {
        let old = begin_mechanism_audit(true, 64).unwrap();
        let old_map = Scope::new(Kind::Map);
        let token = old_map.map_token();
        let old_join = join_map(token);
        let mut old_probe = Scope::new(Kind::Probe);
        let old_sweep = Scope::new(Kind::Sweep);
        assert_eq!(old.finish().unwrap_err(), AuditError::LiveScopes);

        let current = begin_mechanism_audit(true, 64).unwrap();
        let map = Scope::new(Kind::Map);
        let current_token = map.map_token();
        assert_eq!(token.id, current_token.id); // Ordinals may repeat; generations cannot.
        assert_ne!(token.generation, current_token.generation);
        drop(join_map(token));
        old_probe.probe(0, Some(1), ProbeClass::ComponentReplay);
        old_sweep.sweep_base(&[1.0], 0, 0, true, true);
        old_sweep.stencil(0, 0.5, 1.5, true, true, None);
        drop(old_sweep);
        drop(old_probe);
        drop(old_join);
        drop(old_map);
        let solve = Scope::new(Kind::Solve);
        solve.finish(Ok::<_, ()>(())).unwrap();
        map.finish(Ok::<_, ()>(())).unwrap();
        let audit = current.finish().unwrap();
        assert_eq!(
            audit.maps,
            Counts {
                starts: 1,
                completions: 1,
                errors: 0
            }
        );
        assert_eq!(
            audit.solves,
            Counts {
                starts: 1,
                completions: 1,
                errors: 0
            }
        );
        assert_eq!(audit.probes, Counts::default());
        assert_eq!(audit.component_replay, Counts::default());
        assert_eq!(audit.sweeps, Counts::default());
        assert_eq!(audit.events.len(), 4);
        assert!(
            audit
                .events
                .iter()
                .all(|event| !matches!(event, Event::MapJoin { .. }))
        );
        assert!(audit.events.contains(&Event::Start {
            kind: Kind::Solve,
            id: 2,
            parent: current_token.id,
        }));
    }

    #[test]
    fn map_token_from_another_thread_is_not_a_local_join() {
        let session = begin_mechanism_audit(false, 0).unwrap();
        let map = Scope::new(Kind::Map);
        let token = map.map_token();
        map.finish(Ok::<_, ()>(())).unwrap();
        let other = std::thread::spawn(move || {
            let session = begin_mechanism_audit(true, 16).unwrap();
            let join = join_map(token);
            Scope::new(Kind::Solve).finish(Ok::<_, ()>(())).unwrap();
            drop(join);
            session.finish().unwrap()
        })
        .join()
        .unwrap();
        assert_eq!(other.maps, Counts::default());
        assert!(other.events.contains(&Event::Start {
            kind: Kind::Solve,
            id: 1,
            parent: None
        }));
        assert_eq!(other.events.len(), 2);
        assert_eq!(session.finish().unwrap().maps.completions, 1);
    }
}
