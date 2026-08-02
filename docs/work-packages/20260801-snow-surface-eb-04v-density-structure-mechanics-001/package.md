# SNOW-SURFACE-EB-04V Density-Structure Mechanics Investigation

Status: `complete / diagnostic complete / efficacy hold`

Date: `2026-08-01`

Campaign: `SNOW-SURFACE-EB`

Plan class: `Kernel diagnostic observability + calibration-readiness`

This living ExecPlan follows `docs/codex_exec_plans.md` and must remain
self-contained as execution proceeds.

## Purpose / Big Picture

The current snow-density model produces nine forcing-robust density-trajectory
failures, split between modeled density shapes that are too strong and too
weak. Existing traces expose only the total before/after density, so a human
cannot tell whether fresh-snow mixing, wet compaction, destructive
metamorphism, overburden compaction, or a cap/fallback produced the trajectory.

After EB-04V, the real direct-production trace will publish an additive daily
density-process ledger with a directly computed fresh-snow density. The package
will exercise that ledger over the nine EB-04U density lanes and explain which
processes dominate by phase and bias direction. It will not tune coefficients,
change density physics, or conduct a result-bearing efficacy study.

## Objective

Amend `SC-SNOWFREEZE-001` to authorize a behavior-neutral density-process
diagnostic ledger, add contract-derived tests, implement the ledger through the
real density runtime and direct-production JSONL trace consumer, independently
reconstruct daily density closure, execute the nine-lane B/L/S/LS diagnostic
population, and classify the retained density failures without selecting proxy
physics or making a promotion claim.

## Implementation Intent

Intent: `science diagnostic implementation + calibration-readiness`.

Science implementation status is `IMPLEMENTED` for the current Anderson/SNOBAL
density process and `NOT_IMPLEMENTED` for any future amendment. Calibration
evidence is `NOT_APPLICABLE` because no parameter is fitted. Identifiability is
initially `PARTIALLY_IDENTIFIABLE`: exact process tendencies become observable,
but already-consumed observations remain diagnostic-only and correlated process
drivers may retain equifinality.

The runtime change is behavior-neutral observability. No coefficient, equation,
selector, default, density cap, phase/melt/canopy/energy process, or public WAT
surface may change.

## Context And Orientation

`crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`
implements the activated `physics_bulk_density_compaction_v1` path. Its daily
update mixes new snow at a temperature-derived fresh density, applies wet
compaction when liquid is present, and applies 24 dry-compaction substeps whose
Anderson/SNOBAL terms combine destructive metamorphism and overburden. The
result is mass-normalized to the authoritative CoE SWE boundary and capped at
the active `522 kg m^-3` runtime cap.

`DirectSnowLiquidPartition` carries the result into the runner. The real JSONL
consumer is `direct_production_snow_trace_line` in
`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`.
EB-04U proved that retained traces lack the process-specific tendencies and
cannot separate fresh-snow density from same-day compaction.

The EB-04U density population is exactly nine failures: Harvard hardwood;
Marcell conifer, deciduous, and open; and SNOTEL CSS Lab, Mica Creek/St. Joe,
Niwot, Paradise, and Snowbird. Existing observations and all EB-04S/04T results
are `DIAGNOSTIC_ONLY` for this package.

## Included Scope

- canonical diagnostic-ledger authority in `SC-SNOWFREEZE-001`;
- contract-derived tests and a pre-production contract gate;
- a typed density-process diagnostic object with explicit units and additive
  closure;
- direct fresh-snow density and snow-input mass/depth operands;
- daily bulk-density deltas from fresh-snow mixing, wet compaction,
  destructive metamorphism, overburden compaction, dry/wet cap adjustment,
  climate-form fallback when applicable, and final runtime cap;
- typed propagation through `SnowDensityRuntimeOutcome` and
  `DirectSnowLiquidPartition` to the real direct-production trace;
- anti-alias tests that distinguish each process contribution and reject total
  before/after density as a substitute;
- focused, quick, frost, and critical/full validation selected for a new
  production trace schema and cross-crate runtime state surface;
- an exact nine-lane B/L/S/LS diagnostic run with sanitized `OPENWEPP_*` state,
  binary provenance, immutable inputs, and no efficacy/promotion decision;
- phase- and bias-direction analysis, accessible figures with same-stem
  Markdown sidecars, roadmap/catalog reconciliation, dual review, finding
  disposition, dual verification, and final handoff.

## Excluded Scope

- changing density equations, constants, multipliers, caps, substep count,
  selectors, defaults, or coefficient surfaces;
- implementing a new density-process candidate or choosing coefficients;
- empirical calibration, independent validation, materiality threshold
  selection, or a result-bearing efficacy/promotion attempt;
- modifying observation files, cohort membership, historical retained outputs,
  EB-04S/04T/04U evidence, WAT schemas, frost physics, CoE melt/liquid, phase,
  canopy, radiation, longwave, sublimation, or forcing;
- interpreting correlation or process magnitude as unique causal proof;
- proxy, surrogate, provisional, empirical-stand-in, or heuristic production
  physics.

## Deliverables

1. Contract amendment, contract-cycle evidence, and pre-implementation gate.
2. Typed process-ledger runtime implementation and contract-derived tests.
3. Real-consumer trace proof with unit/alias/closure evidence.
4. Hash-bound nine-lane/36-cell diagnostic population and provenance.
5. Process, seasonal-phase, bias-direction, and equifinality analysis.
6. Calibration-readiness and process-selection admission matrices.
7. Accessible figures with same-stem Markdown sidecars.
8. Validation, security, line-count, exact-diff, review, verification,
   disposition, and worker-handoff artifacts.

## Dependencies

- root, work-package, crate, test, standards, and science-contract instructions;
- `docs/codex_exec_plans.md`;
- `docs/standards/testing-and-gate-strategy.md`;
- `docs/standards/kernel-work-package-preparation.md`;
- `docs/standards/prompt-wording-guidance.md`;
- `docs/specifications/science-contract-authoring-procedure.md`;
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`;
- `docs/specifications/unit-governance.md`;
- ADR-0042;
- `SC-SNOWFREEZE-001`, especially the Anderson/SNOBAL density invariants and
  active default/cap authority;
- EB-04U population, evidence-role, phase, and successor-admission artifacts;
- EB-04S execution harness and retained provenance as historical orientation.

## Intended Write Set

- this package tree;
- `docs/ROADMAP.md`;
- `docs/planning/snow-surface-energy-balance-roadmap.md`;
- `docs/work-packages/README.md`;
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`;
- `docs/specifications/science-contracts/index.md` only if lifecycle metadata
  requires reconciliation;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`;
- the minimum orchestrator state/partition files required to propagate the
  typed diagnostic object;
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`;
- focused crate-local and `tests/integration/` contract/consumer tests;
- the existing HPHYS0296 internal trace-schema contract test for the additive
  `v1` to `v2` schema revision;
- root `Cargo.toml` only to register the package-owned integration test target;
- assurance-v2 snow-report identity/review locks and the typed source-adoption
  receipt required when the governed snow science contract changes;
- tracked `usersum/assurance/review-drafts/**` outputs refreshed by the
  canonical full-catalog renderer after that typed source adoption;
- package-owned diagnostic outputs under `target/` or a package-local retained
  artifact directory.

Cargo dependency declarations, observation fixtures, authority-suite cohort
bindings, historical work packages, and public WAT/PASS schemas are protected.
The root manifest may receive only the named package-owned test registration.

## Contract-First Phase Plan

### Phase A — Freeze authority, operands, and population

Hash-bind the EB-04U nine-row population and current source/contract inputs.
Define every process-ledger field, unit, sign, control volume, derivation,
anti-alias rule, and closure equation before production edits. Freeze the 36
B/L/S/LS cells as diagnostic-only; no candidate efficacy predicate is admitted.

### Phase B — Amend contract and write failing tests

Amend `SC-SNOWFREEZE-001` with a behavior-neutral diagnostic invariant,
obligation, variables/units, alias and unit-governance rows, test vectors,
calibration/identifiability posture, gap disposition, and change log. Add
contract-derived unit, decomposition, cap, fresh-snow, no-snow, real-consumer,
and schema tests. Record the pre-implementation contract gate before changing
production Rust.

### Phase C — Implement typed ledger and real consumer

Refactor existing calculations only enough to return exact process increments;
do not alter arithmetic order used for snow-state mutation. Propagate the typed
ledger through the direct runtime into the JSONL consumer. Prove legacy and
activated no-diagnostic state outputs remain numerically identical except for
new diagnostic fields.

### Phase D — Execute diagnostic cohort and analyze mechanics

Build the exact release runner binary and record its hash. Sanitize ambient
`OPENWEPP_*` state, execute the nine lanes under B/L/S/LS, retain complete
trace/provenance inventory, reconstruct daily density closure independently,
and summarize tendencies over the EB-04U observed-anchored primary frame.
Report process dominance, bias-direction contrasts, cap/fallback frequency,
driver correlation, and unresolved equifinality without coefficient fitting.

### Phase E — Validate, review, verify, and close

Run all selected focused and integrated gates, reconcile the exact diff,
produce figures/sidecars and handoff, complete two independent science/code
reviews, disposition every finding, obtain two terminal verifications, archive
the kickoff prompt, and issue the final disposition.

## Acceptance Criteria

1. The canonical contract authorizes the exact diagnostic fields without
   changing process physics or promotion authority.
2. Every field has units, sign, control volume, source, consumer, and anti-alias
   disposition; dimensional symbols satisfy unit governance.
3. The process ledger closes daily bulk-density change to a contract-stated
   tolerance across isolated fresh-snow, wet, destructive-metamorphism,
   overburden, cap, fallback, zero-snow, and combined cases.
4. Fresh-snow density is computed directly before mixing and cannot alias total
   daily density or a layer difference after same-day compaction/merge.
5. The implementation preserves the existing mutation arithmetic, selectors,
   defaults, cap, CoE SWE/liquid boundary, and non-density outputs bitwise or at
   the contract-authorized numerical tolerance.
6. The real direct-production JSONL consumer publishes and reads every field;
   producer-only or shadow evidence cannot close the claim.
7. Exactly nine immutable lanes and 36 B/L/S/LS cells execute from the fresh
   release binary with complete provenance and no observation-role leakage.
8. Independent reconstruction passes for every emitted density-ledger day;
   known wrong aliases and omitted-process formulas fail the anti-tautology
   checks.
9. The analysis separates observed-anchored primary phases, both retained bias
   directions, process magnitude, driver association, cap/fallback incidence,
   and equifinality. It does not claim unique causality or material efficacy.
10. No coefficient fitting, candidate amendment, promotion/default change,
    observation/fixture edit, or protected-surface change occurs.
11. Contract profile, unit compliance, focused tests, warnings-denied Clippy,
    quick, frost, full critical regression, documentation, schema, security,
    line-count, exact-diff, review, disposition, and verification gates pass.

Any unmet current-scope criterion forces `HOLD`; it may not be reclassified as
future work after implementation begins.

## Validation Selection

Risk: `Critical`. The package adds dimensional diagnostic fields to a broadly
consumed production runtime state and JSONL schema across orchestrator/runner
crates. It does not alter intended physics, but ambiguous state/schema fan-out
requires campaign-strength correctness evidence.

Selected commands include Rustfmt, warnings-denied Clippy for affected crates,
contract and unit guards, focused contract/consumer tests, quick and frost
profiles, exact release-runner build, the 36-cell diagnostic population, and
`cargo nextest run --workspace --profile full`. Exact commands and results are
recorded in `artifacts/gate-results.md`.

No external-authority cohort binding is edited, so authority-suite anti-evasion
guards are not selected unless the terminal diff proves otherwise. Cargo deny
is not selected unless manifests, lockfile, or dependency resolution change.

## Conservation And Output Acceptance

Before production edits, `artifacts/operand-lineage.md` must bind every density
operand and additive increment. Tests must numerically distinguish all omitted-
term and total-delta aliases. Acceptance requires independent reconstruction
from the real JSONL output and a cohort-wide closure/magnitude audit. Exact
self-consistency alone is supporting evidence, not acceptance; isolated
analytical vectors and the real consumer must both pass.

## Security Impact Gate

No network, secrets, authentication, dependency, unsafe Rust, deployment, or
external write is authorized. Model outputs must remain under package-owned or
`target/` paths. Ambient `OPENWEPP_*` variables are removed from the evidence
environment except the prospectively frozen B/L/S/LS selectors. Commands use
explicit argv and working directories.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent science/code reviewers, two terminal
verification agents, and the `comparator_suite_runner` for the 36-cell
population plus full/critical validation. Reviewers/verifiers may write only
their named package-local artifacts. The suite runner has read-only source
access and may write package/target evidence outputs and return compact metrics
plus log paths.

Subagent requirement: REQUIRED. The parent must delegate the population and
full-workspace heavy runs to `comparator_suite_runner`, and must obtain two
independent reviews plus two terminal verifications.

## Progress

- [x] (2026-08-01) User authorized scaffolding and end-to-end execution.
- [x] (2026-08-01) EB-04U handoff and current density/runtime surfaces mapped.
- [x] (2026-08-01) Scaffold package, prompt, authority map, and roadmap/catalog state.
- [x] (2026-08-01) Complete Phase A operand/population freeze.
- [x] (2026-08-01) Complete contract-first Phase B.
- [x] (2026-08-01) Complete runtime/consumer Phase C.
- [x] (2026-08-01) Complete diagnostic cohort and analysis Phase D.
- [x] (2026-08-01) Complete review, verification, and terminal closure Phase E.

## Surprises & Discoveries

- The activated bulk-density implementation already computes destructive
  metamorphism and overburden terms separately but combines them before state
  mutation. Exact contribution publication can therefore be behavior-neutral
  if it preserves the existing sum and arithmetic order.
- A layer-local cap attribution is not an aggregate bulk-density attribution.
  Review forced the ledger to reconstruct sequential harmonic bulk density
  before, uncapped, and applied at every layer mutation.
- Observation pairing is part of the frozen scientific operator. Excluding
  modeled-snow-free dates reversed several retained bias directions even though
  runtime output was correct; terminal analysis now anchors every B-cell count
  and KGE component to EB-04R.
- Wet compaction is active and largest in both bias groups. That rejects
  inactivity, but opposing mixing, structural, and cap terms leave compaction
  sufficiency and coefficient identity unresolved.

## Decision Log

- Decision: implement observability, not another density candidate.
  Rationale: EB-04U prohibits a result-bearing attempt until mechanism
  observability and numeric efficacy prerequisites exist; current evidence
  cannot justify coefficient or process selection.
  Date/Author: 2026-08-01 / Codex.
- Decision: retain all B/L/S/LS variants for the nine density lanes.
  Rationale: longwave and sublimation can change temperature, liquid, and pack
  state entering density while the density formula stays fixed; the factorial
  is diagnostic context, not an efficacy retry.
  Date/Author: 2026-08-01 / Codex.
- Decision: publish uncapped wet/PTM/POC bulk increments and a separate signed
  internal-cap correction.
  Rationale: this closes exactly while preserving the combined authoritative
  mutation and avoids describing cap-limited increments as isolated process
  realization.
  Date/Author: 2026-08-01 / Codex.
- Decision: retain observed-snow/model-no-snow dates at modeled density zero.
  Rationale: this is the canonical predecessor operator and is necessary for
  comparable paired counts, beta direction, and KGE decomposition.
  Date/Author: 2026-08-01 / Codex.

## Outcomes & Retrospective

The exact terminal binary completed all 36 frozen cells. Independent JSONL
reconstruction closes at `3.411e-13 kg m^-3`; all 36 WAT tables and 574,196
pre-existing trace rows remain identical to EB-04R. The retained B-cell pairing
operator reproduces all nine counts and KGE components within `4.441e-16`.

EB-04V closes the density-process observability prerequisite. It does not
identify one coefficient, supply independent calibration evidence, or authorize
an efficacy round, physics amendment, promotion, or default change. Wet
compaction is active in both bias groups, while fresh mixing, structural
projection, and caps materially oppose it. Density calibration therefore
remains `HOLD`; EB-04W is the next bounded science package.

## Idempotence And Recovery

Scaffold and analysis generation are additive and deterministic. The cohort
runner must use package-owned output directories and refuse overwrite unless
the exact input/source identity matches. A failed preflight that emits no
result may be repaired; any inspected result retains its evidence status and is
never silently overwritten or retried into a pass.

Revision note (2026-08-01): initial autonomous scaffold authored from the
EB-04U terminal handoff and current production density/runtime inspection.
