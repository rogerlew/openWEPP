# Stage 3 Turbulent Carrier Lineage And Operator Reconciliation

Status: `scaffolded / result-blind protocol freeze pending review`

Date: `2026-08-06`

Package ID:
`20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001`

Plan class: `Critical contract-first evaluation observability and four-site characterization`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
`Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` remain current throughout execution.

## Purpose

Explain why the same Stage 3 carrier is strongly negative in the immutable
same-state evaluation but positive in the earlier independently reinitialized
sequential response experiment. The package makes the complete carrier's
surface-state, atmospheric, vapor, geometry, stability, and exchange lineage
observable, then runs both admitted operators from byte-identical daily source
states and forcing at Mica Creek, Niwot, Paradise, and Snowbird.

The observable result is a term-by-term, support-aware operator reconciliation.
It may identify state evolution, support censoring, term lineage, or an
unreproduced predecessor as the source of the sign contradiction. It does not
change carrier physics, fit parameters, persist state, promote Stage 3, or
retire CoE.

## Context And Rationale

The predecessor four-site audit reconstructed 154 screen-eligible immutable
condition samples. Every sample was negative; canonical site medians were
`-60.63`, `-123.32`, and `-54.30 W m^-2`, while non-decisive Snowbird was
`-67.23 W m^-2`. Turbulent sensible and latent terms dominated those negative
totals. An earlier Snowbird sequential experiment reported a positive
window-aggregated carrier near `+170 MJ m^-2`, but it evolved a clone within
each day and did not export enough primitive lineage to compare the operators.

Both current operators start from the authoritative post-CoE daily snapshot
and receive the same 24 hourly forcing records. The immutable operator holds
the surface state fixed. The bounded sequential operator changes its clone at
dynamic substeps, and that state can change outgoing longwave, sensible heat,
latent heat, vapor mass, and precipitation-advection terms. Schema v5 exports
term totals and fingerprints, but not the shadow surface temperature,
vapor-pressure pair, Monin-Obukhov stability solution, exchange velocities, or
hourly state endpoints needed to interpret the divergence.

## Implementation Intent

- Intent: behavior-neutral internal evaluation observability plus diagnostic
  characterization.
- Production science: unchanged. No formula, coefficient, threshold,
  arithmetic order, physical state, forcing, default, or owner changes.
- Evaluation science: expose exact existing inputs, returned transfer
  diagnostics, and clone-state endpoints for the two already admitted
  operators.
- Calibration evidence: `NOT_APPLICABLE`.
- Identifiability: `NOT_ASSESSED`; no parameter is fitted.
- Observation role: SNOTEL remains `DIAGNOSTIC_ONLY` for the frozen sampling
  windows inherited from the predecessor.
- Assurance: adopt the new observability source through the typed workflow
  while preserving the snow/frost report as DRAFT with empty review, approval,
  release, and publication authority.
- Risk: `Critical` because canonical kernel authority, a shared meteorology
  result type, and an internal diagnostic schema change even though default and
  production results must remain exact.

## Included Scope

1. Amend `SC-SNOWFREEZE-001` with a narrow invariant for internal schema-v6
   operator-reconciliation observability, exact sign conventions, primitive
   lineage, support weighting, state endpoints, and claim limits.
2. Add contract-derived tests and pass the pre-implementation contract gate
   before production Rust edits.
3. Extend the existing Monin-Obukhov return diagnostics without changing its
   calculation or existing flux values.
4. Extend evaluation-only hourly diagnostics with surface/air/dewpoint/vapor,
   wind, pressure, `z_T/z_q/z_u/z_0`, stability, exchange, and clone-state
   endpoint operands.
5. Emit those operands through an enabled-only internal schema-v6 JSONL row;
   disabled/default rows remain exact schema v4.
6. Prove exact primitive-to-flux and term-total reconstruction, support-aware
   aggregation, and state custody through the real JSONL consumer.
7. Freeze, build, and execute four control, four immutable same-state, and four
   bounded sequential lanes using byte-identical staged fixtures and selector
   maps except for the evaluation operator.
8. Join operator rows only on site, day, lane, source fingerprint, forcing
   fingerprint, and geometry fingerprint; report unmatched and censored rows.
9. Decompose every external term and the implemented subset into same-state,
   sequential, and sequential-minus-same-state values at common support.
10. Track initial and final clone ice mass, cold content, surface temperature,
    and evaluated support so state evolution is not inferred from energy alone.
11. Classify the contradiction using the prospectively frozen decision rule,
    update the campaign roadmap, and complete review, validation, verification,
    prompt archival, and stable commits.

## Excluded Scope

- Any change to the Monin-Obukhov, longwave, albedo, advected-heat, phase,
  density, melt, liquid, cold-content, or conduction equations.
- New physics, coefficients, parameter fitting, site tuning, forcing scaling,
  plausibility-range tuning, or empirical calibration.
- Cross-day state persistence, physical accumulation-season chronology,
  restart/reappearance, terminal-event numerics, receiving-surface energy,
  snow-ground heat, land-surface energy, or soil enthalpy.
- CoE retirement, Stage 3 production ownership, selector defaults, public
  WAT/HBP/PASS schema changes, fixture or observation edits, promotion, or
  cutover.
- Calling internal active/lower conduction snow-ground heat or treating either
  bounded operator as a seasonal simulation.

## Intended Write Set

- this package tree;
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` and
  `docs/specifications/science-contracts/index.md`;
- exact contract-version and evaluation-observability tests under
  `tests/integration/`;
- `crates/openwepp-meteorology/src/surface_energy.rs`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs`;
- evaluation trace formatting/tests under
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`;
- focused Stage 3 runtime tests required by the changed additive diagnostic
  types;
- typed assurance-source adoption paths and generated receipts selected by the
  existing assurance workflow;
- `docs/work-packages/README.md`, `docs/ROADMAP.md`, and
  `docs/planning/snow-surface-energy-balance-roadmap.md`;
- ignored `target/snow_stage3_operator_reconciliation/` raw evidence and normal
  ignored build/test outputs.

No fixture, observation, dependency, selector default, public output, or
unrelated crate change is authorized. A needed path outside this set requires a
prospective package amendment before that edit.

## Protected Boundaries

CoE remains the sole authoritative production melt and snow-mass owner.
Evaluation objects remain default-off, clone-only, evaluator-custodied, and
consumer-forbidden. An enabled evaluation failure fails only that request; the
disabled production run remains unchanged. Schema v6 is internal diagnostic
evidence, not publication. Disabled schema v4 and real WAT/HBP/PASS bytes must
remain exact.

No result may claim physical seasonal energy, melt efficacy, validation,
transferability, persistence, terminal closure, promotion, or cutover. The
package can reconcile operator mechanics and term lineage only.

## Frozen Diagnostic Surface

For every evaluated operator hour, schema v6 must publish support-weighted
means or exact endpoints with explicit semantics for:

- air, dewpoint, and shadow surface temperature in degrees Celsius;
- air and saturated-surface vapor pressure in pascals;
- wind speed in metres per second and atmospheric pressure in pascals;
- `z_T`, `z_q`, `z_u`, and `z_0` in metres;
- Monin-Obukhov iteration count, stability-length applicability and
  support-weighted Obukhov length;
- friction velocity, sensible exchange velocity, latent exchange velocity,
  air density, air potential temperature, and air/surface specific humidity;
- initial and final evaluated total ice mass, cold content, surface
  temperature, and layer count; and
- existing shortwave, longwave, sensible, latent, precipitation-advection,
  vapor-mass, support, and residual operands.

Support-weighted means use only evaluated seconds. Initial fields are the first
evaluated state in the hour; final fields are the last state after that hour's
evaluation effects. Same-state initial and final fields must be identical.
Sequential endpoints may differ. Not-applicable stability values carry an
explicit applicability/support field; zero must never silently mean neutral or
missing.

The sign convention is positive energy or vapor mass toward the snow control
volume and negative away from it. Sensible heat is positive when the air-side
potential temperature exceeds the surface temperature. Vapor mass and latent
heat are positive for deposition/condensation and negative for
sublimation/evaporation. Net longwave is incoming atmospheric plus canopy
longwave minus outgoing snow longwave. Precipitation advection is positive when
the hydrometeor supplies heat relative to the surface.

## Frozen Cohort And Operators

Reuse the exact four predecessor fixture copies, observation hashes, Snowbird
development-only precipitation derivative, water-year windows, and
`DIAGNOSTIC_ONLY` observation role recorded in
`artifacts/protocol-freeze.json`. Snowbird remains non-decisive and cannot
establish forcing truth.

Each site receives three release-CLI lanes from the same staged fixture:

- evaluation disabled control;
- `same_state_paired_carrier_v1`; and
- `sequential_resolved_shadow_v1`.

All other selectors are identical: multilayer physical density,
Harder-Pomeroy hourly phase, CoE liquid-holding-capacity melt,
`layered_thermal_liquid_v1`, Dilley-Unsworth subcanopy longwave, and disabled
production sublimation. Ambient `OPENWEPP_*` variables are removed before the
frozen map is installed.

## Frozen Join, Reconstruction, And Decision Rule

The primary join key is site, day index, lane index, exact source fingerprint,
exact forcing fingerprint, and exact geometry fingerprint. A mismatched
fingerprint is not a delta sample. It is an identity failure reported
separately. Source, forcing, and geometry hashes must match between the two
operators before any causal operator interpretation.

The primary comparison uses hours where both operators evaluated exactly 3600
seconds. Partial/common support is reported separately and cannot be promoted
into the full-support estimate. Each external term is independently summed
from the real consumer. The complete subset is shortwave plus longwave plus
sensible plus latent plus precipitation-advection. Internal active/lower
conduction is reported separately and remains nonexternal.

Primitive reconstruction must reproduce each operator's sensible, latent,
vapor-mass, longwave, and advected term within prospectively frozen
scale-aware tolerances. Hourly state endpoints must close against the
operator's exported mass/cold-content changes where applicable. Known wrong
aliases—authoritative Stage 3 surface temperature, CoE terms, internal
conduction, zero-filled N/A state, and calendar rather than evaluated support—
must produce distinct values in anti-tautology fixtures.

The scientific disposition is one of:

1. `STATE_EVOLUTION_RECONCILES_SIGN_CONTRADICTION` when the current sequential
   lane reproduces the positive Snowbird sign, the immutable lane reproduces
   the negative sign, all join identities and reconstructions pass, and the
   term delta is confined to state-dependent terms on matched support while
   state-independent shortwave remains invariant.
2. `SUPPORT_CENSORING_MATERIALLY_CONTRIBUTES` when common/full-support and
   all-evaluated-support summaries differ in sign or attribution.
3. `PREDECESSOR_NOT_REPRODUCED` when the current bounded sequential lane does
   not reproduce the predecessor's positive Snowbird sign.
4. `LINEAGE_OR_IDENTITY_FAILURE` when primitive reconstruction, fingerprints,
   sign conventions, consumer custody, or protected-output identity fails.
5. `MULTIFACTOR_UNRESOLVED` when valid evidence does not uniquely satisfy the
   preceding classes.

These categories may be reported together when their predicates coexist.
None is a carrier-plausibility PASS. Persistent-shadow advancement remains
blocked unless a later prospectively authorized package establishes a physical
carrier gate.

## Conservation And Output Acceptance

Before production edits, `artifacts/operand-lineage.md` must bind field names,
units, support/normalization, area basis, source, sign, applicability, and
authoritative-versus-diagnostic status. Acceptance requires independent
reconstruction from real schema-v6 rows, anti-tautology fixtures, full and
partial support separation, exact source/forcing/geometry joins, and protected
output byte identity. Producer-carried totals and one-sided bounds are
supporting evidence only.

## Contract-First Sequence

1. Amend canonical contract authority.
2. Add/update contract-derived tests.
3. Run and record the pre-implementation contract gate.
4. Only then edit production Rust or the trace consumer.

No surrogate, provisional, proxy, fitted, or heuristic physics is allowed.
This package may expose existing calculations but cannot replace them.

## Phase Plan

### Phase A — Scaffold And Result-Blind Freeze

Commit this autonomous plan, prompt, reading map, operand lineage, protocol,
write set, test plan, and queued artifacts. Obtain two independent result-blind
science/Rust reviews and disposition every finding before contract or runtime
implementation.

### Phase B — Contract And Contract Tests

Amend `SC-SNOWFREEZE-001`, advance exact bindings, add schema-v6 and primitive
lineage guards, run focused authority/profile tests, complete dual contract
review/disposition, and record the pre-implementation gate.

### Phase C — Behavior-Neutral Observability

Add exact transfer diagnostics and evaluation state endpoints, project them
through schema v6, and add real-consumer, independent-reconstruction,
anti-alias, error, custody, and protected-output tests. Preserve all existing
flux calculations and default behavior.

### Phase D — Frozen Four-Site Execution

Commit the independently admitted implementation, build the exact release CLI,
run 12 lanes, retain raw evidence under the declared target namespace, and
produce compact tracked tables/results without changing the frozen protocol.

### Phase E — Review And Closure

Complete two independent implementation/result reviews, one independent
consumer/reconstruction review, direct critical validation, assurance DRAFT
adoption, exact-diff reconciliation, prompt archival, two terminal
verifications, roadmap/catalog disposition, and stable local commits.

## Validation And Exit Criteria

- Result-blind scaffold and protocol receive independent PASS/PASS before
  contract or runtime edits.
- Contract amendment, focused guards, profile/readiness checks, and dual
  contract review pass before production edits.
- Default/disabled schema-v4 and real WAT/HBP/PASS bytes remain exact.
- Existing schema-v5 semantics remain readable only as historical evidence;
  enabled current rows are explicit schema v6 with every required primitive.
- Existing flux values are bit-identical for identical inputs before and after
  the observability extension.
- Primitive reconstruction, term totals, support, state endpoints, and
  rejected-alias tests pass through the real consumer.
- All 12 exact release lanes complete, every joined row has exact fingerprints,
  and unmatched/censored rows are exhaustively reported.
- The frozen decision rule is applied without post-result threshold, cohort,
  selector, sign, support, or category changes.
- CoE ownership and evaluation nonreachability remain proven.
- Formatting, warnings-denied Clippy, doctests, focused tests, quick, frost,
  full workspace, assurance validation, Markdown/schema checks, line-count,
  security, dual review, finding disposition, and dual verification pass.
- Every required current-scope gate has direct current evidence. An unmet gate
  cannot be renamed future scope; the package remains HOLD if it cannot close.
- Exact terminal diff matches this declared intent and write set.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only science/Rust reviewers, one
independent read-only consumer/reconstruction reviewer, one
`comparator_suite_runner` for the exact four-site release batch and heavy
critical gates, and two independent read-only terminal verifiers. Expected
outputs are compact findings, reproduced metrics, exact commands/counts, and
evidence paths. Delegates may inspect and run commands but may not edit tracked
files, expand authority, change the frozen protocol, tune results, or activate
assurance review/approval/publication. The orchestrator alone edits and
dispositions findings.

## Security And Data Impact

No network, credentials, secrets, external services, or new dependencies are
required. Runs use local retained fixtures and observations. The runner removes
ambient `OPENWEPP_*` values, writes only ignored package-target evidence, and
must not mutate source fixtures. The new schema is internal and default-off.

## Progress

- [x] (2026-08-06) User authorized scaffolding and end-to-end execution.
- [x] (2026-08-06) Confirmed schema v5 lacks the shadow-state and transfer
  primitives required to reconcile the operators.
- [ ] Commit the result-blind scaffold and obtain independent admission review.
- [ ] Amend contract authority and pass the pre-implementation contract gate.
- [ ] Implement behavior-neutral schema-v6 observability and focused tests.
- [ ] Execute and analyze the frozen four-site paired/sequential cohort.
- [ ] Complete review, critical validation, DRAFT assurance adoption, terminal
  verification, roadmap disposition, prompt archival, and stable closure.

## Surprises & Discoveries

- Observation: both operators already call the same complete-carrier function;
  their explicit semantic difference is immutable versus clone-evolving state,
  plus possible sequential support truncation. Evidence:
  `stage3_solver/evaluation.rs`.
- Observation: the Monin-Obukhov solver already returns iteration count and
  optional Obukhov length internally, but the evaluation result discards them.
  Its internal state also computes the exchange quantities needed for exact
  lineage. Evidence: `openwepp-meteorology/src/surface_energy.rs`.
- Observation: the existing trace contains authoritative Stage 3 surface
  temperatures, but those are an invalid alias for the evaluation clone. A
  shadow-specific state surface is required.

## Decision Log

- Decision: add enabled-only schema v6 rather than reinterpret existing schema
  v5 fields. Rationale: v5 does not encode the primitive/state semantics needed
  for exact reconciliation, and silently changing their meaning would corrupt
  retained evidence. Date/Author: 2026-08-06 / Codex.
- Decision: retain the exact predecessor four-site cohort and Snowbird
  development forcing. Rationale: changing forcing while reconciling operator
  mechanics would confound the contradiction; Snowbird remains explicitly
  non-decisive. Date/Author: 2026-08-06 / Codex.
- Decision: do not make physical plausibility or persistence an exit criterion.
  Rationale: this package identifies operator/term lineage; it cannot promote a
  carrier merely because the contradiction becomes explainable.
  Date/Author: 2026-08-06 / Codex.

## Outcomes & Retrospective

Queued. Completion requires a frozen, independently reviewed operator
reconciliation result and truthful next-step disposition. CoE remains
authoritative throughout.

## Revision Note

2026-08-06: Initial contract-first scaffold created from the failed four-site
carrier audit and its worker handoff.
