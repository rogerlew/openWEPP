# Implement Stage 3 Terminal Meltout To The Real Snow-Free Owner Stack

Status: `queued`

Date: `2026-08-19`

Package ID: `20260819-snow-stage3-terminal-meltout-lse-handoff-implementation-001`

Plan class: `Critical contract-first kernel and real-consumer integration`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Purpose / Big Picture

Complete the mechanical ground-snow-to-land transition that earlier packages
could not execute. When the Stage 3 terminal solver locates complete solid
exhaustion inside an interval, the real default-off scheduler candidate must
close snow custody at that event, transfer liquid exactly once, select the
actual snow-free receiver, and run the admitted V10/LSE-V2/direct-hydrology
owners for the remaining time. A failure on either side must restore the whole
beginning owner set exactly.

This package does not establish Stage 3 turbulent-carrier efficacy and does not
change production ownership. CoE remains the authoritative production snow and
melt generator.

## Rationale And Prerequisite Lift

The historical
`20260805-snow-stage3-terminal-meltout-soil-handoff-001` executed Phase 1 and
held because it lacked a complete receiving-surface authority and coupled
persistent shadow-state authority. Those prerequisites now exist in the
completed snow-free land-surface/real-hydrology campaign, including its real
scheduler, live owner forcing, persisted restart, root hydraulics, surface
liquid custody, and atomic rollback. The completed terminal enthalpy-event
package supplies error-controlled event-local snow exhaustion and snow-side
mass, liquid, vapor, time, and energy closure. This fresh successor preserves
both historical dispositions and joins their admitted mechanics.

## Implementation Intent

Intent is `science implementation and critical default-off real-consumer
integration`. It is not empirical calibration, independent physical
validation, turbulent-carrier authority closure, production activation,
publication, or cutover qualification. Comparator evidence is diagnostic under
ADR-0017 and cannot substitute for canonical authority or closure.

## Required Chronology

For each affected interval, start from the persistent Stage 3 snow state; solve
only until the earliest complete solid-exhaustion event `t*`; close ice,
cold-content/enthalpy, vapor, liquid, time, and energy ledgers at `t*`; release
retained and newly generated liquid exactly once to the surface-liquid owner;
select the actual vegetation/litter/mineral/frost/water receiving surface;
rebuild radiation, turbulence, evaporation, precipitation heat, and soil heat
without snow operands; execute V10/LSE-V2/direct hydrology for the remaining
`interval_duration - t*`; and atomically commit the complete snow plus
snow-free owner transaction. No snow albedo, snow surface temperature, snow
roughness, snow-computed flux, or terminal-unallocated snow energy may be reused
after `t*`.

## Included Scope

- Amend canonical snow-energy, snow/frost, land-surface-energy, evaporation,
  soil/frost, water-balance, infiltration/runoff, and restart authority before
  code wherever the exact handoff chronology is not already binding.
- Define typed event-to-receiver input, exact-one liquid transfer, owner
  selection, remaining-time forcing rebuild, atomic commit, and error
  precedence.
- Integrate the existing persistent Stage 3 evaluation state and terminal event
  with the existing real V10/LSE-V2/direct-hydrology scheduler attachment; do
  not create a skeleton, alternate snowbench loop, or caller-owned physics
  bridge to carry the claim.
- Prove restart immediately before and after meltout and inside the terminal
  substep; cross-midnight meltout; rain during meltout; dormancy followed by
  new snowfall; infiltration, ponding, overflow, runon, and runoff; and exact
  rollback after every snow-side and receiving-side failure class.
- Independently reconstruct distinct pre-event snow and post-event receiving
  ledgers from actual real-consumer evidence with anti-alias fixtures.
- Preserve absent/default behavior, production state, CoE ownership, public
  schemas, WAT/HBP/PASS bytes, selectors, defaults, and outputs.

## Excluded Scope

- Turbulent-carrier, wind-exposure, canopy-aerodynamic, or forest-applicability
  authority; no fixed attenuation factor or fitted substitute may enter.
- Physical seasonal efficacy, empirical calibration, Stage 3/CoE parity,
  production-candidate qualification, public-output acceptance, selector or
  default activation, CoE retirement, or release cutover.
- Canopy-intercepted snow.
- Reuse of snow fluxes after `t*`, direct assignment of terminal unallocated
  snow energy to soil, snow drainage relabeled as runoff, silent liquid loss,
  dual melt generation, clamps, epsilons, canonicalize-and-proceed, or any
  surrogate/provisional/proxy/heuristic production physics.

## Deliverables

- Canonical invariants and contract-derived tests for the complete chronology.
- A typed event-to-receiver transaction in the actual scheduler candidate.
- Persistent restart wire/versioning only where canonical authority requires
  it, with deterministic uninterrupted-versus-restore equivalence.
- Exact-one ownership and rollback poison tests covering all named scenarios.
- Operand lineage, independent reconstruction, real closure/magnitude audit,
  consumer-path proof, performance disposition, and production-noninterference
  evidence.
- Dual reviews with finding disposition, dual verification, and a truthful
  final disposition/worker handoff.

## Dependencies And Authority

- `SC-SNOWENERGY-001`, especially `INV-SNOWENERGY-029/030/034` and their
  obligations/tolerances.
- `SC-SNOWFREEZE-001`, especially `INV-SNOWFREEZE-093/100/101` and the
  persistent/terminal claim limits.
- The applicable land-surface-energy, vegetation, evaporation, soil, frost,
  water-balance, infiltration/runoff, and restart contracts identified during
  intake.
- Completed packages
  `20260807-snow-stage3-persistent-accumulation-shadow-001`,
  `20260807-snow-terminal-enthalpy-event-numerics-001`,
  `20260814-snow-free-land-surface-real-hydrology-integration-001`,
  `20260814-vegetation-land-surface-real-consumer-shadow-001`, and
  `20260819-root-zone-hydraulic-owner-implementation-001`.
- Pinned legacy provenance `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` only for touched legacy process
  mappings; no behavioral approximation is authorized.

## Intended Write Set

- `docs/work-packages/20260819-snow-stage3-terminal-meltout-lse-handoff-implementation-001/**`
- the campaign coordinator lifecycle/evidence files;
- `docs/ROADMAP.md` and `docs/work-packages/README.md` at disposition;
- affected canonical contracts and `docs/specifications/science-contracts/index.md`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/**`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs`,
  its bounded successor modules if decomposition is required, and
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`;
- affected runner internal trace/consumer modules under
  `crates/openwepp-runner/src/hillslope/direct_publication/**`;
- focused unit and integration tests, including existing terminal-event, land-
  surface real-hydrology, restart, and V10 real-consumer contract surfaces;
- ignored logs below `target/snow_stage3_terminal_meltout_lse_handoff/`.

The owned-file manifest must narrow exact paths before production edits.
Expansion requires prospective package amendment and review.

## Contract-First Phase Plan

1. Freeze required reading, exact owner/consumer path, implementation intent,
   write set, operand lineage, failure precedence, and validation selection.
2. Amend canonical contracts; add unequal-operand contract-derived positive and
   poison tests; run and record the pre-implementation contract gate. No
   production code edit may precede this gate.
3. Implement the typed terminal-event-to-receiver transaction and complete
   remaining-time forcing rebuild in the actual scheduler candidate.
4. Implement or extend persisted restart and atomic whole-owner commit/rollback
   without changing absent/default production behavior.
5. Run synthetic and repository-backed event, cross-midnight, rain, restart,
   reappearance, water-routing, receiver-selection, error-precedence,
   noninterference, performance, and independent reconstruction evidence.
6. Reconcile the exact diff; run selected focused, domain, real-consumer,
   critical full-workspace, formatting, warnings-denied Clippy, doctest,
   cargo-deny when applicable, docs, assurance-impact, security, and line-count
   requirements directly; complete dual reviews, finding disposition, dual
   verification, prompt archive, catalog/roadmap update, and final disposition.

## Conservation And Consumer Acceptance

Before production edits, `artifacts/operand-lineage.md` must identify every
ice, cold-content/enthalpy, vapor, melt, rain, retained/released liquid,
refreeze, infiltration, soil/surface storage, ponding, overflow, runon, runoff,
evaporation, snow energy, receiving radiation/turbulence/precipitation/soil heat,
time-support, and area-basis operand, with units, source authority, and
authoritative-versus-evaluation status.

Fixtures must make expected values differ from full-step snow fluxes,
snow-terminal energy assigned to soil, generated melt aliased to rain, released
store aliased to store level/change, snow drainage aliased to runoff, omitted
runon, reused snow albedo/temperature/roughness, and producer residuals.
Acceptance requires independent reconstruction from the actual downstream
consumer plus real two-sided closure/magnitude audits. Self-consistency and
one-sided bounds are supporting evidence only. The consumer-path artifact must
name producer, state/frame, scheduler handoff, downstream owner calls, evidence
surface, and negative proof that skeleton, snowbench, CoE receiving state, and
old compatibility paths do not carry the claim.

## Exit Criteria

1. Canonical contracts bind the complete event-to-receiver chronology,
   operands, signs, units, guards, error precedence, atomicity, restart, and
   exact-one custody before code edits.
2. Every current-scope gate has direct current evidence; an unmet gate cannot
   be relabeled as later-campaign scope after implementation begins.
3. The actual scheduler candidate consumes the localized terminal event and
   complete real snow-free owner stack for only the remaining interval.
4. Every named restart, chronology, reappearance, rainfall, receiver, routed-
   water, runon, and injected-failure scenario passes with deterministic state
   equivalence and exact beginning-owner rollback.
5. Independent snow-side and receiving-side reconstruction closes without
   aliases, post-event snow operands, dropped/doubled liquid, time overlap/gap,
   or simultaneous CoE and Stage 3 melt in the evaluated transaction.
6. Absent/default execution preserves production state, CoE authority,
   selectors, defaults, public schemas, outputs, and protected bytes.
7. Applicable direct validation, exact-head critical correctness, assurance-
   impact, dual review/finding disposition, dual verification, security, and
   line-count governance pass on the terminal diff.
8. Final disposition states only mechanical handoff closure and preserves the
   turbulent-carrier, physical-efficacy, qualification, assurance-approval,
   production-ownership, and cutover holds for later children.

## Calibration Readiness

This package implements fixed-authority mechanics and performs no parameter
fitting or observational validation. `artifacts/calibration-readiness-matrix.md`
must disposition every required ADR-0042 readiness field; data-limited or
not-applicable fields cannot be promoted into efficacy claims.

## Security And Data Impact

No network, credential, unsafe-code, external message, protected-fixture
mutation, deployment, or public release is intended. Record the exact terminal
diff assessment. Run source anti-evasion guards if execution touches external-
authority suite posture, cohort fixtures, or required-case bindings.

## Line-Count Governance

Record every changed `.rs` file before/after. Files at 2,000+ lines are WARN
and require decomposition rationale and follow-on intent; nonexempt files at
3,000+ lines block closure.

## Review And Subagent Authorization

Subagent authorization: this package explicitly authorizes and requires
spawning/delegating to one read-only snow/land-surface science reviewer, one
read-only hydrology/ownership reviewer, one read-only Rust correctness reviewer,
one read-only Rust QA reviewer, two independent read-only terminal verifiers,
and one `comparator_suite_runner` for heavy full-workspace/comparator execution.
Expected outputs are compact findings, command/count summaries, closure checks,
and artifact/log paths. The comparator runner may write only ignored logs and
bounded gate artifacts; only the primary executor edits tracked source.

## Progress

- [x] (2026-08-19) User authorized roadmap reconciliation and package scaffold.
- [x] (2026-08-19) Confirmed terminal snow mechanics and the complete snow-free
  real-consumer prerequisites have terminal package evidence.
- [ ] Freeze intake artifacts and contract amendment needs.
- [ ] Complete the contract-first pre-implementation gate.
- [ ] Implement and validate the real-consumer handoff.
- [ ] Complete reviews, verification, and disposition.

## Surprises & Discoveries

None at scaffold time.

## Decision Log

- Decision: this package closes only the default-off mechanical handoff.
  Rationale: forest turbulent-carrier authority and physical efficacy remain a
  distinct missing-authority boundary and cannot be inferred from transaction
  mechanics. Date/Author: 2026-08-19 / Codex.
- Decision: the real scheduler and complete existing owner stack must carry the
  closure claim. Rationale: producer-only, skeleton, shadow-internal, or
  snowbench evidence cannot prove the downstream transition. Date/Author:
  2026-08-19 / Codex.

## Outcomes & Retrospective

Queued. No contract, runtime, selector, default, output, or ownership behavior
has changed.
