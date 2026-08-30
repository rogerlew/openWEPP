# Implementation intent

Static: `Critical` production kernel chronology, conservation, restart, shared
owner, runner-consumer, and default-cutover change. Contract-first sequence is
contracts, contract-derived tests, pre-implementation gate, then production.
The intended write set is package-wide and compiler-discovered as declared in
`package.md`. Exact terminal diff controls the final gate selection.

## 2026-08-29 profiling-only continuation

Static: owner direction limits this increment to diagnostic attribution of the
canonical one-day wall time. The implementation may add opt-in, thread-local,
non-persisted timers to the existing ignored qualification harness and expose
their aggregate durations in test output. It must not change physical
equations, fixed-point convergence, tolerances, adaptive decisions, receipts,
owner state, publication, restart, or wire formats. The target buckets are the
multi-tile physical runtime and the converged-candidate finalization/replay
path. No performance optimization is authorized by this increment.
