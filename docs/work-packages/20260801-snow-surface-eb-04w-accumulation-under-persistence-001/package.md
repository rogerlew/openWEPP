# SNOW-SURFACE-EB-04W Accumulation And Mountain Under-Persistence Investigation

Status: `complete`

Date: `2026-08-01`

Campaign: `SNOW-SURFACE-EB`

Plan class: `Kernel diagnostic observability + retrospective mechanism attribution`

This living ExecPlan follows `docs/codex_exec_plans.md` and must remain
self-contained as execution proceeds.

## Purpose / Big Picture

Five retained mountain-snow chronology failures show modeled peak snow or
melt-out roughly one month too early. Existing output cannot tell whether the
pack began ablation with too little mass, lost deposited snow too quickly, or
received excessive ablation forcing. After EB-04W, a human can inspect real
daily and hourly phase, snow-input, melt-component, vapor, and storage ledgers
for the four unique open SNOTEL lanes and see which boundary owns each failure.

This is diagnostic science, not calibration or promotion. Existing observations
remain `DIAGNOSTIC_ONLY`; no coefficient, equation, selector, default, forcing,
or snow state mutation may change.

## Objective

Amend `SC-SNOWFREEZE-001` to authorize behavior-neutral accumulation, hourly
phase, and CoE melt-component diagnostics; test and implement those diagnostics
through the real direct-production JSONL consumer; execute four unique lanes
under B/L/S/LS; and classify the five frozen EB-04U failure operators as
pre-peak forcing/phase/redistribution, post-deposition mass loss, ablation
energetics, mixed, or unresolved at the openWEPP/wepppy boundary.

## Implementation Intent

Intent: `science diagnostic implementation + calibration-readiness`.

The current Harder-Pomeroy phase and baseline-authoritative CoE melt equations
remain implemented and unchanged. Calibration evidence is `NOT_APPLICABLE`
because nothing is fitted. Mechanism identifiability is initially
`PARTIALLY_IDENTIFIABLE`: modeled contributions become directly observable,
whereas physical wind redistribution and site-representativeness remain
external and cannot be inferred from a residual.

## Context And Orientation

`DirectWinterHourlyForcing` already retains rain, physical snowfall depth,
rain/snow fractions, phase-model identity, and hydrometeor temperature. The
smaller `DirectSnowHourlyForcing` projection currently discards the phase
metadata. `simimpl29_hourly_melt_inches` computes four additive legacy CoE
melt-depth terms (`amelt`, `bmelt`, `cmelt`, and `dmelt`) but returns only their
sum. The direct-production JSONL trace publishes daily snow mass and Stage 3
energy fields but not these source operands.

The EB-04U population contains five failure rows over four unique lanes: Mica
Creek/St. Joe melt-out, Niwot peak depth and peak SWE, Paradise melt-out, and
Snowbird peak SWE. The exact observation operators and roles are inherited by
hash from EB-04U. All B/L/S/LS results remain diagnostic-only.

## Included Scope

- exact population, observation-operator, evidence-role, operand, and unit
  freeze before production edits;
- `SC-SNOWFREEZE-001` authority for a behavior-neutral trace ledger;
- hourly rain, snowfall depth, snowfall SWE, phase fractions/model, and
  hydrometeor-temperature propagation;
- four exact CoE empirical melt-depth contributions, uncapped total, cap
  adjustment, and applied raw melt, without relabeling any term as a pure
  physical energy flux;
- an explicit model boundary stating wind redistribution is not implemented and
  modeled contribution is zero, while physical redistribution remains unknown;
- real-consumer JSONL schema revision, anti-alias tests, independent phase/mass/
  component closure, and behavior-neutral retained-output comparison;
- exact release binary, sanitized `OPENWEPP_*` state, four lanes, 16 B/L/S/LS
  cells, five operator evaluations, figures with same-stem Markdown sidecars;
- focused, quick, frost, and critical/full validation; dual review,
  disposition, dual verification, and final handoff.

## Excluded Scope

- changing process equations, constants, arithmetic order, selectors, defaults,
  caps, phase, melt, density, frost, longwave, sublimation, or water routing;
- adding or estimating wind redistribution, gauge undercatch, interception, or
  other forcing corrections inside openWEPP;
- calling `bmelt` or `cmelt` a separately identifiable sensible-heat flux;
- fitting, efficacy thresholds, independent-validation claims, promotion, or
  default activation;
- observation, fixture, authority-suite cohort, WAT/PASS schema, parser,
  runfile, user-control, or wepppy changes;
- surrogate, proxy, provisional, heuristic, or site-specific production physics.

## Intended Write Set

- this package tree;
- `docs/ROADMAP.md`, the snow campaign roadmap, and work-package catalog;
- `SC-SNOWFREEZE-001` and lifecycle metadata if required;
- the minimum orchestrator forcing/state and CoE helper files needed to return
  typed diagnostics without changing state arithmetic;
- the minimum runner direct-publication projection and JSONL consumer files;
- focused crate and integration tests plus package-owned analysis tooling;
- assurance source-adoption and rendered review drafts only if required by the
  governed contract-source lock.

Observation fixtures, authority-suite bindings, historical package evidence,
Cargo dependencies, and public science output schemas are protected.

## Contract-First Phase Plan

### Phase A — Freeze authority and operands

Hash-bind the five operators/four lanes, evidence roles, retained baselines,
phase semantics, CoE component meanings, output units, and independent closure
relations. Record that physical wind redistribution is external/unobserved and
not equivalent to the model's zero implemented contribution.

### Phase B — Amend contract and write failing tests

Advance `SC-SNOWFREEZE-001` to version 121 with one diagnostic invariant and
producer obligation, variables/units, aliases, unit governance, tests, binding
index, gap posture, and history. Add contract-derived unit, decomposition,
phase, no-precipitation, cap, real-consumer, schema, and behavior-neutral tests.
Record the pre-implementation gate before editing production Rust.

### Phase C — Implement the real diagnostic path

Propagate existing typed phase fields and return exact CoE component operands
without altering mutation arithmetic. Publish them through a trace helper so
the existing large runner formatter remains below its maintenance threshold.
Prove the real JSONL consumer reads all fields and previous outputs are unchanged.

### Phase D — Execute and interpret the frozen population

Build and hash the exact release runner, sanitize ambient selectors, run 16
unique B/L/S/LS cells, independently close all ledgers, and evaluate five
operators. Separate pre-observed-peak input/storage deficit from pre/post-peak
modeled loss and ablation contribution. Do not infer missing physical
redistribution from residual closure.

### Phase E — Validate, review, and close

Run selected gates, reconcile the exact diff and `.rs` line counts, render
accessible figures/sidecars, complete two independent reviews, disposition all
findings, obtain two independent verifications, archive the prompt, and issue a
diagnostic disposition plus EB-04X/EB-05 handoff.

## Acceptance Criteria

1. Contract authority precedes production edits and defines units, sign,
   cadence, lineage stage, consumer, and prohibited interpretations.
2. Hourly active precipitation reconstructs from rain plus snowfall SWE within
   `1e-12 m`, and daily accumulation reconstructs from hourly snowfall SWE.
3. Four CoE component contributions reconstruct uncapped raw melt; the separate
   cap adjustment reconstructs applied melt at the same lineage stage.
4. The real JSONL consumer emits every field; producer-only or shadow evidence
   cannot close acceptance.
5. All previous trace fields and WAT outputs remain identical or within an
   already authorized tolerance for all 16 cells.
6. Exactly four unique lanes and 16 B/L/S/LS cells execute, and exactly five
   frozen observation operators are evaluated without data-role leakage.
7. The analysis distinguishes modeled facts from external forcing hypotheses
   and does not call formula components pure physical energy fluxes.
8. No physics, coefficient, selector, default, observation, fixture, or public
   schema changes.
9. Focused, unit/contract, Clippy, quick, frost, full, documentation, security,
   line-count, exact-diff, review, disposition, and verification gates pass.

Any unmet current-scope criterion forces `HOLD`; it cannot be reclassified as
future work after implementation starts.

## Validation Selection

Risk: `Critical`. The intended behavior is unchanged, but the package adds
dimensional fields across production orchestrator/runner state and a real JSONL
schema. Selected validation is Rustfmt, warnings-denied affected-crate Clippy,
focused contract/consumer tests, unit/contract guards, quick, frost, exact
release execution, the 16-cell cohort, and full workspace nextest.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes spawning/delegation
to two independent science/code reviewers, two terminal verification agents,
and `comparator_suite_runner` for the 16-cell population and critical/full
validation. Reviewers/verifiers may write only their named package artifacts.
The suite runner may write only package-local or `target/` evidence and returns
compact metrics plus artifact paths. No role may modify production, contract,
roadmap, observation, or fixture files.

## Progress

- [x] (2026-08-01) Recovered EB-04U authority, resolved five operators to four
  unique lanes, and inspected the current forcing/melt/trace seams.
- [x] (2026-08-01) Scaffolded EB-04W and activated roadmap/catalog ownership.
- [x] (2026-08-01) Froze operands/population and amended the canonical contract
  to version 121.
- [x] (2026-08-01) Added failing contract tests and recorded the
  pre-implementation gate.
- [x] (2026-08-01) Implemented typed diagnostics through the real JSONL
  consumer without changing state arithmetic.
- [x] (2026-08-01) Executed the exact release 16-cell population and produced
  the synthesis plus four accessible figure/sidecar pairs.
- [x] (2026-08-01) Completed dual independent review and disposition; no Rust,
  science, causal-claim, or provenance finding remains.
- [x] (2026-08-02) Completed fresh final-source quick/frost/full suites: all
  4,676 executed tests passed across the three profiles.
- [x] (2026-08-02) Completed dual exact-terminal verification, archived the
  execution prompt, and issued the terminal handoff.

## Surprises & Discoveries

- Observation: the five failure controls occupy four unique lanes because Niwot
  carries both peak-depth and peak-SWE chronology operators.
  Evidence: EB-04U `failure-cohort-manifest.json`.
- Observation: CoE `bmelt` and `cmelt` are empirical mixed-driver melt-depth
  terms, not independently measured sensible-heat fluxes.
  Evidence: `simimpl29_hourly_melt_inches` and Chapter 3 §3.6 authority.
- Observation: baseline seasonal modeled-peak magnitudes span about
  `0.39-0.62` of observed peaks. Independently, baseline modeled SWE retained
  on observed SWE-peak dates has lane medians of about `0.21-0.46` of observed
  SWE, establishing the observed-date storage deficit.
  Evidence: exact-head 16-cell `accumulation-mechanics-results.json`.
- Observation: Paradise's initial SWE plus realized snowfall SWE and retained
  rain remain below the observed peak in every evaluated water year and cell.
  Because retained rain is endogenous to pack state and capacity, this
  localizes an input-pathway/pre-peak-loss boundary without proving an external
  forcing defect. The other lanes also retain mixed ownership.
  Evidence: exact pre-observed-peak input/storage/loss reconstruction in
  `accumulation-mechanics-results.json`.
- Observation: a terminal frost profile exposed missing phase metadata in the
  retained snowbench CoE adapter. The adapter already had classified rain and
  snowfall SWE, so it now reconstructs diagnostic fractions from those exact
  operands; the focused replay passes and modeled water arithmetic is unchanged.
  Evidence: failed pre-fix frost log and focused post-fix replay.
- Observation: the sealed final source passed quick (`2,143/2,143`), frost
  (`341/341`), and full (`2,192/2,192`) workspace profiles without a failure.
  Evidence: `artifacts/terminal-suite-summary.md` and `artifacts/summary.json`.

## Decision Log

- Decision: execute four unique lanes and evaluate five frozen operators.
  Rationale: avoids duplicated model execution while preserving the exact
  population definition.
  Date/Author: 2026-08-01 / Codex.
- Decision: bind model wind redistribution as unimplemented/zero and retain
  physical redistribution as unknown external forcing.
  Rationale: WEPP authority explicitly leaves drifting inactive; a zero model
  term cannot prove zero physical redistribution.
  Date/Author: 2026-08-01 / Codex.
- Decision: do not admit an ablation coefficient or process amendment from
  EB-04W.
  Rationale: the lanes remain unresolved between the realized modeled input
  pathway and pre-peak modeled melt, vapor, and storage loss. The input side
  also cannot uniquely separate precipitation representativeness, gauge
  undercatch, phase, liquid retention, and physical redistribution.
  Date/Author: 2026-08-01 / Codex.

## Outcomes & Retrospective

The exact-head cohort completed all 16 B/L/S/LS cells and all five frozen
operators. Phase, accumulation, and melt-component ledgers closed many orders
of magnitude inside the `1e-12 m` contract tolerance. The empirical result is
consistent across controls in one respect: seasonal modeled peak magnitudes are
about `39-62 %` of the observed peaks. At the observed SWE-peak dates, the
baseline lane medians are lower still, about `21-46 %` of observed SWE. At
Paradise, even initial SWE plus realized snowfall and retained rain remain below
the observed peak. This is an algebraic input-pathway boundary, not proof of an
external forcing defect, because liquid retention is state-dependent. Mica
Creek, Niwot, and Snowbird sometimes have sufficient realized input before
recorded losses, but their causal ownership also remains mixed.

EB-04W cannot identify a unique process amendment or calibration target.
Precipitation representativeness, gauge undercatch, phase error, liquid
retention, physical wind redistribution, and pre-peak loss timing remain
confounded. Consequently the final disposition is
`DIAGNOSTIC_COMPLETE / CALIBRATION_HOLD / NO_PROMOTION`. EB-04X remains the
next planned investigation because its paired Harvard hardwood/open geometry
can isolate a different canopy/interception question without using these open
mountain residuals as a tuning target.
