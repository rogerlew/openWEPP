//! MOFEFID Lane D / D14 (SC-OFEROUTE-001): opt-in runtime slot profiling for
//! the Lane D routing path. Diagnostics-only: disabled by default, enabled by
//! the runner when `OPENWEPP_LANED_SHADOW_PROFILE=1`, and never touches any
//! published output, manifest value, or solver control flow. Slot timers are
//! disjoint by construction (setup vs. CFL selection vs. step math vs.
//! hydrograph sampling); counters record work intensity (steps, alpha
//! evaluations, samples, upstream-boundary interpolations) so the D14 profile
//! can attribute the H2637 shadow overhead to named code paths.

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

static PROFILE_ENABLED: AtomicBool = AtomicBool::new(false);

/// Enable/disable slot profiling process-wide. The runner owns the env
/// opt-in; kernels never read the environment themselves.
pub fn set_enabled(on: bool) {
    PROFILE_ENABLED.store(on, Ordering::Relaxed);
}

/// One relaxed atomic load; the only cost the disabled path pays.
#[inline]
#[must_use]
pub fn enabled() -> bool {
    PROFILE_ENABLED.load(Ordering::Relaxed)
}

/// Accumulated routing-path slots for the current thread.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct RoutingProfileSnapshot {
    /// `KinematicWaveSolver::run` invocations (one per OFE per routed day).
    pub solver_runs: u64,
    /// TVD-MacCormack steps taken across all solver runs.
    pub solver_steps: u64,
    /// `CellParameters::alpha` evaluations (counted at the call-site loops).
    pub alpha_evaluations: u64,
    /// Hydrograph samples recorded (interpolated outlet points).
    pub hydrograph_samples: u64,
    /// Upstream-boundary interpolation calls (cascade handoff lookups).
    pub upstream_interpolation_calls: u64,
    /// Steps whose interval carries zero source on every cell and zero
    /// upstream boundary mass; retained as the TV-diagnostic gate.
    pub solver_steps_homogeneous: u64,
    /// Steps with zero source on every cell but possibly nonzero upstream
    /// inflow; retained as a recession/source-free work counter.
    pub solver_steps_source_free: u64,
    /// Per-OFE solver setup: mesh clone + solver construction + validation.
    pub solver_setup_ns: u64,
    /// CFL sub-timestep selection + Courant evidence (pre-step, per step).
    pub solver_cfl_ns: u64,
    /// `step()` body: forcing fetch, predictor/corrector/TVD, commit, ledger.
    pub solver_step_ns: u64,
    /// Hydrograph sample interpolation + push (sample branch only).
    pub solver_sample_ns: u64,
}

thread_local! {
    static PROFILE: RefCell<RoutingProfileSnapshot> =
        const { RefCell::new(RoutingProfileSnapshot::new()) };
}

impl RoutingProfileSnapshot {
    const fn new() -> Self {
        Self {
            solver_runs: 0,
            solver_steps: 0,
            alpha_evaluations: 0,
            hydrograph_samples: 0,
            upstream_interpolation_calls: 0,
            solver_steps_homogeneous: 0,
            solver_steps_source_free: 0,
            solver_setup_ns: 0,
            solver_cfl_ns: 0,
            solver_step_ns: 0,
            solver_sample_ns: 0,
        }
    }
}

/// Start a span if profiling is enabled; `None` costs one atomic load.
#[inline]
#[must_use]
pub fn span_start() -> Option<Instant> {
    if enabled() {
        Some(Instant::now())
    } else {
        None
    }
}

fn span_ns(started: Option<Instant>) -> Option<u64> {
    started.map(|instant| u64::try_from(instant.elapsed().as_nanos()).unwrap_or(u64::MAX))
}

macro_rules! slot_end_fn {
    ($name:ident, $field:ident) => {
        /// Fold the span into its slot; no-op when the span was disabled.
        #[inline]
        pub fn $name(started: Option<Instant>) {
            if let Some(ns) = span_ns(started) {
                PROFILE.with(|profile| profile.borrow_mut().$field += ns);
            }
        }
    };
}

slot_end_fn!(end_solver_setup, solver_setup_ns);
slot_end_fn!(end_solver_cfl, solver_cfl_ns);
slot_end_fn!(end_solver_step, solver_step_ns);
slot_end_fn!(end_solver_sample, solver_sample_ns);

macro_rules! counter_fn {
    ($name:ident, $field:ident) => {
        /// Add to the counter; no-op when profiling is disabled.
        #[inline]
        pub fn $name(count: u64) {
            if enabled() {
                PROFILE.with(|profile| profile.borrow_mut().$field += count);
            }
        }
    };
}

counter_fn!(count_solver_runs, solver_runs);
counter_fn!(count_solver_steps, solver_steps);
counter_fn!(count_alpha_evaluations, alpha_evaluations);
counter_fn!(count_hydrograph_samples, hydrograph_samples);
counter_fn!(
    count_upstream_interpolation_calls,
    upstream_interpolation_calls
);
counter_fn!(count_solver_steps_homogeneous, solver_steps_homogeneous);
counter_fn!(count_solver_steps_source_free, solver_steps_source_free);

/// Read and clear the current thread's accumulated slots.
#[must_use]
pub fn snapshot_and_reset() -> RoutingProfileSnapshot {
    PROFILE.with(|profile| std::mem::take(&mut *profile.borrow_mut()))
}

/// Serializes tests that toggle the process-global enable flag. The
/// snapshot accumulator is thread-local, but `PROFILE_ENABLED` is shared:
/// under plain `cargo test` (libtest threads in one process) two
/// flag-toggling tests would otherwise race. Every test that calls
/// `set_enabled` must hold this guard for its full body.
#[cfg(test)]
pub(crate) fn test_flag_guard() -> std::sync::MutexGuard<'static, ()> {
    static GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());
    GUARD
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_profile_accumulates_nothing() {
        let _flag_guard = test_flag_guard();
        set_enabled(false);
        let _ = snapshot_and_reset();
        let span = span_start();
        assert!(span.is_none());
        end_solver_step(span);
        count_solver_steps(3);
        assert_eq!(snapshot_and_reset(), RoutingProfileSnapshot::default());
    }

    #[test]
    fn enabled_profile_accumulates_and_resets() {
        let _flag_guard = test_flag_guard();
        set_enabled(true);
        let _ = snapshot_and_reset();
        let span = span_start();
        assert!(span.is_some());
        end_solver_cfl(span);
        count_solver_steps(2);
        count_alpha_evaluations(40);
        let snapshot = snapshot_and_reset();
        set_enabled(false);
        assert_eq!(snapshot.solver_steps, 2);
        assert_eq!(snapshot.alpha_evaluations, 40);
        assert!(snapshot.solver_cfl_ns > 0);
        assert_eq!(snapshot_and_reset(), RoutingProfileSnapshot::default());
    }
}
