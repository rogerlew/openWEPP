Status: OWNER-READY architecture decision, 2026-09-08 PDT.
Static + Ran arithmetic: source analysis, exploratory A attribution, and final
controlled A/F and A/R campaigns. Not a promotion decision.

## Observable structure and bounds

48 parents /56 accepted publications;72 outer observations;200 terminal evaluator
invocations;400 actual provider/carrier executions;400 LSE maps;800 solves;
2400 nonlinear iterations;2000 sweeps;106800 signed probes;109512 shared evaluator
entries;438048 leaf calls. These are authentic aggregate counts, not exclusive
causal attribution. Direct detailed lifecycle joins qualify the map/solve/probe
edges; resolved/batch/final work must not be collapsed by equal payloads.

Exploratory A complete run5.080719s (not controlled median): Stage3/LSE/soil
4.319564s, native vegetation/ET.338499s, LaneD.000784s, remaining.426589s.
These diagnostic envelopes sum5.085436s, a4.717ms scope offset from the runner
timer. Nested provider_carrier2.055008s overlaps Stage3 and other carrier buckets;
never sum them as independent work. Optimistic complete elimination of each
envelope yields about6.68x,1.07x,1.00015x,1.09x respectively, not achievable gains.
LaneD is not the current dominant cost.

Prior performance-budget.md:154-178 sets terminal-event2.5ms/OFE-day and
representative average550us/OFE-day (10OFE5.5ms/day,210s/century). Exploratory
one-OFE event A is about2032x the event target;9238x the representative average
is contextual only, not a representative/century scaling prediction. Even20%
local savings would leave roughly1600x event gap. No F+R gain addition is valid.

## Ranked directions (technical recommendations, not implementation authority)

1. **Residual/Jacobian representation avoiding complete reevaluation per coordinate.**
   Entry points: LSE solver_covered_solve.rs `solve_covered_column_impl`,
   `covered_jacobian_probe_residuals`, Newton Jacobian assembly;
   solver_covered_evaluation.rs validated evaluator, hydraulics and occupancy
   blocks. First decisive prototype: one component-temperature derivative block
   over shared canonical residual nodes, independently compared against lawful
   centered/inward finite differences on authentic saved states; then full solves
   and real consumer. New SC-LANDSURFACEENERGY authority must explicitly admit
   derivative arithmetic/trajectory changes; replay authority does not.
   Preserve domains, active sets, residual acceptance, typed errors, conservation
   and custody. Use independent FD oracle, invariant suites and pinned-baseline
   comparator confidence tiers. Proposed bounded early kill: less than10x
   residual/leaf-work reduction inside the supported block, or less than30%
   end-to-end saving after sufficiently broad declared coverage, stops a campaign.
   Any unresolved correctness violation forbids promotion regardless of timing.
   R's graph/oracle are reusable design inputs; R salvage is not a prerequisite.

2. **Stable numerical state/scratch with explicit custody construction boundaries.**
   Entry points: orchestrator v11_covered/carrier_phase.rs,
   CoveredCarrierPhaseResultV1 and snow_stage3_v11_terminal_execution.rs result
   assembly/retention; LSE validated evaluation inputs. Prototype one typed
   compact numerical carrier state and bounded scratch, producing unchanged
   accepted owner/receipt bytes. New SNOWENERGY authority must identify which
   intermediate validations/commitments may be delayed; F authority does not
   authorize deleting them. Preserve immutable beginnings, exact-one transfer,
   selected result binding, rollback/restart and publication. Full field/byte
   and error-poison comparator remains mandatory. Proposed kill: less than50%
   assembly/validation-cost and20% complete-run reduction, or rejected-trial
   dependent post-teardown scratch growth. Anonymous RSS alone is not heap proof.
   F's admitted one-OFE invocation-local boundary is reusable, subject to its
   explicitly unqualified multi-OFE branch.

3. **Newly justified coupled solver/controller reducing nested independent solves.**
   Entry points: snow_stage3_v11_adaptive_execution.rs and stack helpers,
   stage3_solver/evaluation.rs, LSE solve_covered_column_impl. Prototype one
   authentic terminal-support coupled residual solve including rejection/events,
   compared with full/two-half/root references before complete day. Requires
   broader prospective SNOWENERGY/LSE/ADR0044 authority for unknowns, cadence,
   stopping and localization. Preserve equations, domains, conservation/custody;
   no fallback solver cascade. Proposed kill: less than4x coupled-solve reduction
   on supported cases while meeting predeclared accuracy/event/error obligations.

## Selected first prototype and executable scope

Select kickoff Direction **B, lower-cost residual/Jacobian evaluation**. F
observed a 22.44% (1.1207 s) saving from eliminating duplicated carrier work on
the admitted one-OFE workload; this is not a multi-OFE causal/scaling claim,
because the prescribed 10-OFE workload does not enter the native provider branch.
R supplies a reviewed lawful dependency graph and oracle, yet
runtime replay adds 5.72%. The graph/oracle should therefore validate a new
derivative/residual boundary rather than remain in the production hot path.

Exact entry points are
`crates/openwepp-land-surface-energy/src/solver_covered_solve.rs` functions
`covered_jacobian_probe_residuals` (current lines 291–355) and the Jacobian loop
(509–670), plus the validated map/evaluator in
`solver_covered_evaluation.rs` (1842–1957, 1966–1982, 2048–2076). Prototype one
bounded component-temperature block over the authentic N=2/S=6 saved sweep
states, then the existing `stage3_controlled_mechanism_experiment` one-OFE real
consumer.

Contract-first authority is required: amend SC-LANDSURFACEENERGY numerical
methods because current authority fixes finite-difference perturbations/order;
INV-164 replay authority alone cannot authorize analytic/automatic/sparse
directional derivatives. Preserve equations, domains, active-set classification,
normalized residual acceptance, typed error precedence, rollback, restart,
conservation, custody, receipt identity, and accepted-boundary publication.
Numerical error, conservation closure, and discrete identity are separate gates.

Comparator plan: independently reconstruct canonical centered/inward FD columns
for every supported node; compare residual/Jacobian values under the new approved
tolerance authority; then compare solve/error behavior, exact physical/control
outputs, custody receipts, and independently reconstructed public closure on the
authentic runner. A different approved derivative algorithm need not reproduce
the old iteration trajectory bit-for-bit, but any relaxation requires explicit
future physical/tolerance authority.

Prospective success: at least 10x fewer charged residual/leaf evaluations for
the supported block and at least 30% median complete-run wall saving with a
gain-supported paired interval, without material CPU/RSS regression. Early
reject on any physical/custody/error/active-set/closure violation, or if either
10x block reduction or 30% end-to-end saving is missed after the declared
authentic coverage. A 30% gain saves about 1.5 s from the observed ~5 s run and
still leaves over three seconds; it is a discriminator, not throughput closure.

Ready-to-issue next scope: first implement the decisive bounded derivative block
at `covered_jacobian_probe_residuals` behind prospective contract-derived tests;
do not begin with another general audit. Reuse R's independent stencil/dependency
oracle and F's invocation/custody parity machinery. No implementation is
authorized by this handoff.

All proposed kill thresholds are engineering priority choices for a future
authorized package, not scientific tolerances. Final F/R classifications,
absolute controlled savings, memory limits and reuse/subsume/abandon decisions
are bound in treatment-results.md; independent terminal review findings remain
governed by finding-disposition.md.
