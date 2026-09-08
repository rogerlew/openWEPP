# Stage 3 residual/Jacobian prototype

Status: COMPLETE — INCONCLUSIVE_WITH_BOUND at the frozen Phase-A oracle;
production HOLD.
Execution mode: package-end-to-end.
Owner authorization: `prompts/active/kickoff.md` (source brief:
`/tmp/openwepp_residual_jacobian_prototype_kickoff.md`).
Implementation intent: isolated derivative/residual representation experiment,
staged correctness and cost measurement, and an architecture decision.

## Objective and boundaries

Freeze baseline A from checkpoint `da16c7640851f4dad15583bd95aaf5324f19f3a8`
plus only common observation/capture support. Compare J = A plus one approved
component-temperature derivative block, with at most one prospectively named
adjacent block expansion when correctness and local cost admit it. The final
outcome is `EXPAND`, `USEFUL_BUT_COVERAGE_LIMITED`, `REJECTED`, or
`INCONCLUSIVE_WITH_BOUND`, with production remaining HOLD.

Authorized: local documents, canonical experimental numerical-method authority,
contract-derived tests, detached source kits/worktrees, builds, isolated
executables, controlled measurements, evidence, reviews, verification, and
scoped local commits. Not authorized: production activation or promotion,
deployment, network operations, push, named branch changes, new outer controller,
linear solver, pivot/time-discretization/physics changes, cross-iteration caches,
F/R reruns or salvage, historical index reconstruction, or combined F treatment.

Ordinary canonical primal equations, validation precedence, domains, physical
branches, accepted-result construction, adaptive/event policy, custody,
restart/rollback and publication remain protected. The prototype is diagnostic
and must not be production reachable. One predeclared hybrid assembly may use
the new method only on structurally admitted columns, exact identity columns
where already authoritative, and canonical FD elsewhere; it is not a recovery
cascade.

## Dependencies and current state

The executable specification is the active kickoff. Applicable governance:
root and nearest `AGENTS.md`, `docs/codex_exec_plans.md`, work-package author /
implementer / reviewer / verifier / runner procedures, science obligations,
testing strategy, numerical-solver architecture, kernel preparation, science
contract authoring/profile/specification, and the complete selected directory-v1
LSE authority route. Predecessor input is limited initially to its four named
summary artifacts; deeper assets are loaded on demand.

Baseline A is the retained checkpoint implementation, without F or R. The
predecessor established a controlled one-OFE baseline near 5 seconds, F's 22.44%
gain, R's 5.72% regression, and a nonparticipating 10-OFE fixture. Those outcomes
are context, not derivative measurements.

## Intended write set and isolation

Primary checkout writes are limited to:

- this package tree; `docs/work-packages/README.md` and `active.md`;
- `docs/planning/snow-surface-energy-balance-roadmap.md`;
- `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`,
  selected normative chapters (initially `interface.md`, `numerical-methods.md`,
  `solve-boundary.md`, `nonlinear-solve.md`, and recursively required affected
  physics/custody chapters), contract registry, and contract-derived binding
  tests/checkers identified before their edits.

Experimental Rust, fixtures, collectors, and executables live only in detached
worktrees/source kits outside the primary checkout, with each exact path and
source base entered in the source/build manifest before edits. Any expansion
updates the exact file/function write set and authority dependencies first.
Unrelated dirty/untracked work, including literal `$pkg/` and `tmp/`, is protected.

## Phases and frozen sequencing

1. Authority and corpus: define `r(x; u, b0)`, variables/rows/units/order,
   normalization derivatives, held/varying state, branch/kink/one-sided policy,
   inner-solve derivative semantics, errors, lifecycle, and numeric tolerances.
   Freeze A/J comparison classes, corpus, oracle and cost protocol. Add canonical
   authority, derived expected-red tests, and obtain two independent prospective
   reviews before implementation.
2. First block: implement one meaningful component-temperature block over every
   affected row using shared canonical primal calculations; validate independent
   FD step sweeps, feasible directional/Taylor checks, elementary checks,
   sparsity/cardinality/branch/error/stale-owner negatives, full solves and the
   admitted one-OFE real consumer.
3. Bounded expansion: only if Phase 2 correctness and local cost support it,
   extend the same representation to the prospectively named adjacent block
   group. Otherwise decide at the measured boundary.
4. Controlled evaluation: authenticate actual hot-path participation, physical
   results, independent closure/custody, multi-OFE applicability, paired timing,
   CPU and lifecycle memory; reconcile gates, reviews, verification and decision.

Contract-first order is mandatory: authority amendment, contract-derived tests,
captured expected-red evidence, dual prospective review and disposition, then
experimental implementation. No increment closes with a missing current-scope
gate.

## Frozen comparison and decision criteria

Before candidate observations, the authority/protocol must state numeric,
dimensioned derivative/primal/result/closure tolerances from baseline-only
scale, conditioning and oracle uncertainty; candidate results cannot tune them.
Same-state science, derivative accuracy, accepted continuous results, discrete
identity/custody, per-run receipts, and trajectory counters are separate classes.

Local admission requires complete supported-column correctness and roughly 10x
fewer expensive complete residual/leaf-solve invocations attributable to block
construction, plus a decisive measured block-cost reduction. The exact charged
events, denominator, batching, warmups, repetitions and supported-gain criterion
are frozen before sampling. Report eliminated work and replacement tangent /
implicit-solve / assembly work; call-count reduction is not CPU speedup.

Whole-run timing applies only after sufficiently broad declared coverage:
12 fresh-process balanced AJ/JA pairs, two warmups per executable, at most one
extension to 24 pairs, paired seconds/ratios/medians and descriptive paired
bootstrap interval. The 30% median wall-saving criterion is an engineering
priority discriminator only after broad coverage, with a gain-supporting
interval and no material CPU regression. First-block failure to reach 30% is
not by itself rejection. Freeze `f`, local `s`, nonoverlapping timing bounds and
the Amdahl estimate before integrated timing.

Memory uses six matched fresh-process pairs plus three processes/arm with ten
complete one-day runs and teardown; report RSS/HWM/available heap and named
pre/end/post-validation/post-drop points without importing the old 64 MiB cap.
Competitive authenticated candidates receive three 10-OFE pairs and then 19
only after valid 10-OFE admission. Inactive scale paths are `BLOCKED/NOT RUN`,
never extrapolated.

Any same-state physics, domain, active-set, closure, custody, error-precedence,
receipt, lifecycle or independent-oracle violation blocks reliance on results.
A correct but costly or structurally narrow method may complete as rejected or
coverage-limited. Production and predecessor HOLDs remain explicit.

## Validation and delegation

Select direct requirements from `docs/standards/testing-and-gate-strategy.md`.
Critical integrated experimental cuts require affected contract/schema/binding,
A0/A1/A3, focused and full correctness, real-consumer, independent closure,
error/custody/restart, formatter/warnings-denied lint, collector negatives,
line-count and applicable anti-evasion evidence. Preserve and classify inherited
failures; new relevant regressions block the experiment.

Subagent authorization: explicitly authorized for read-only source investigators,
two independent prospective and terminal reviewers, two independent terminal
verifiers, and `comparator_suite_runner`. Reviewers/verifiers may write only
their assigned artifacts; the comparator may execute frozen commands and write
execution evidence, never source. Implementation delegation, if used, requires
nonoverlapping exact ownership. Heavy builds, full regressions, parity batches
and measurements require `comparator_suite_runner`; one genuine service failure
permits one documented local fallback. Heavy work and all measurements are
serialized.

## Progress, decisions, and recovery

- [x] 2026-09-08 PDT: authorization read; checkpoint identity and clean primary
  checkout confirmed; no equivalent package found.
- [x] Package scaffold, active locator, catalog and roadmap forward reference.
- [x] Complete authority/dependency/source/corpus map and freeze protocol.
- [x] Contract amendment, expected-red/oracle tests, dual prospective review and
  finding disposition.
- [x] Authentic positive-stem Phase-A corpus and independent baseline oracle:
  0/16 columns admitted under the frozen convergence-basin rule.
- [x] Stop before J implementation: derivative, cost, runner, expansion, timing,
  memory and scale gates are NOT RUN because Phase A did not admit a column.
- [x] Dual terminal review and dual verification PASS on the corrected stable cut;
  disposition `INCONCLUSIVE_WITH_BOUND`, production HOLD.

Recovery uses Git history for committed compact documents and external immutable
evidence bundles for source/executable/fixture/protocol/result bytes. No source
kit or candidate is removed before recoverable capture and verification. Current
continuation authority is `artifacts/worker-handoff.md`.
