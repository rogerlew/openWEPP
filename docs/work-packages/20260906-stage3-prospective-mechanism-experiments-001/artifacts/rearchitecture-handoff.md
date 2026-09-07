Status: ACTIVE draft; replace provisional costs with controlled results before disposition.
Static + Ran arithmetic: source analysis by feed_forward_design, exploratory A
real consumer counters/timers; F/R measurements pending. Not a promotion decision.

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
   F's invocation-local boundary may be reused if its actual admission succeeds.

3. **Newly justified coupled solver/controller reducing nested independent solves.**
   Entry points: snow_stage3_v11_adaptive_execution.rs and stack helpers,
   stage3_solver/evaluation.rs, LSE solve_covered_column_impl. Prototype one
   authentic terminal-support coupled residual solve including rejection/events,
   compared with full/two-half/root references before complete day. Requires
   broader prospective SNOWENERGY/LSE/ADR0044 authority for unknowns, cadence,
   stopping and localization. Preserve equations, domains, conservation/custody;
   no fallback solver cascade. Proposed kill: less than4x coupled-solve reduction
   on supported cases while meeting predeclared accuracy/event/error obligations.

All proposed kill thresholds are engineering priority choices for a future
authorized package, not scientific tolerances. Final F/R classifications,
absolute controlled savings, memory limits and reuse/subsume/abandon decisions
remain pending this package's measurements and independent terminal review.
