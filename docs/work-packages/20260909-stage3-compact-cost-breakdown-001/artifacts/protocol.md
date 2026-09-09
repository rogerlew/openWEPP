# Profiling protocol and source-to-bucket map

Static: initial implementation map frozen before source edits. Every bucket will be exclusive within its
partition. LSE detail is a drill-down of runner LSE, never an additive peer.
One runner interval retains input/execution/publication boundaries unchanged.

Frozen run budget: two warmups per observation posture, then six fresh-process
balanced off/on pairs on CPU0, same P executable where feasible. No build in the
window. Invalid: nonzero exit, identity/closure mismatch, missing scope close,
negative exclusive time, impossible additive total, host interference or OOM.

P source: `/tmp/openwepp-compact-cost-P-vKI6gb`, copied from retained A cut2b;
no A/J mutation. Initial exact five Rust writes:
`crates/openwepp-land-surface-energy/src/{solver_mechanism_audit,numerics,solver_covered_solve,solver_litter_phase}.rs`
and `crates/openwepp-runner/src/hillslope/tests03/controlled_mechanism_experiments.rs`.
The existing audit module owns fixed aggregate timing, no event/operand retention.
Caller instrumentation is separately assigned and must enumerate its paths before edits.

Shared exclusive categories: runner unassigned, other physical, orchestration,
custody, routing, diagnostics; LSE preparation/base, inactive-input-leaf probes,
other probes, matrix assembly/row adjustment/scaling, linear call, trial/prospective,
and solver remaining/result construction (explicitly merged). Parent runner LSE
is the sum of LSE categories, never additive with its drill-down.
Generic `bounded_jacobian` owns assembly with child evaluator spans; Covered
owns allocation through row adjustment with child probe spans. `solve_linear`
call alone owns factorization. Initial validation and current base have their
own span; trailing solver bookkeeping/result construction stays explicit.
Fixed populations bind family/pass/dimension and exact-zero input sun/shade
coordinates; these descriptive counts are not mathematical elimination authority.
Identity-vs-FD classification follows the existing actual Covered identity seam;
Generic V3 remains FD. Dimension/population overflow invalidates evidence only.

Session setup precedes existing runner clocks. The existing `run_started` Instant
is passed as attribution start and the immediate return Instant as end; no oracle,
postrun closure, fixture or teardown enters attribution. Same-thread fixed stack
preemption charges each interval once; missing/non-LIFO scopes and overflow fail
evidence closed. Other-thread attribution is unsupported and must be disclosed.
Existing overlapping telemetry remains supporting views, not additive peers.
Off posture has no new clock reads inside LSE; both postures retain A's existing
common audits. Same P executable selected via `OPENWEPP_COMPACT_COST=off|on`.
Tests: synthetic nested accounting, non-LIFO/live/overflow/stale guards, disabled
neutrality; existing exact solver/corpus tests and authentic off/on output/control/
closure admission. No extra physical evaluation is added to measured runs.
