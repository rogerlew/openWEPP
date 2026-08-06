# Stage 3 Turbulent Carrier Lineage And Operator Reconciliation

Status: `scaffolded / result-blind review HOLD amended for re-review`

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

Both current operators receive the authoritative post-CoE daily source and the
same 24 hourly forcing records, but they do not project that source into the
same first carrier control volume. The immutable operator evaluates the whole
snow column. Before its first carrier solve, the bounded sequential operator
aligns and normalizes the upper active thermal volume and evaluates that slice.
It then changes its clone at dynamic substeps. Initial control-volume
projection, subsequent state evolution, and support truncation are therefore
three distinct candidate causes. Schema v5 exports term totals and raw-source
fingerprints, but not the effective first carrier input, exact substep
operands, stability solution, or state endpoints required to separate them.

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
3. Add an opt-in Monin-Obukhov diagnostic result beside the unchanged public
   `TurbulentFluxes` result; preserve the existing function and type shape.
4. Add an evaluation-only operator-reconciliation result beside the unchanged
   public schema-v5 evaluation types. It records exact substep tuples with
   surface/air/dewpoint/vapor, wind, pressure, `z_T/z_q/z_u/z_0,aero`,
   stability, exchange, control-volume projection, and clone-state operands.
5. Emit those tuples through an enabled-only internal schema-v6 JSONL row;
   disabled/default rows remain exact schema v4.
6. Prove exact primitive-to-flux and term-total reconstruction, support-aware
   aggregation, and state custody through the real JSONL consumer.
7. Freeze, build, and execute four control, four immutable same-state, and four
   bounded sequential lanes using byte-identical staged fixtures and selector
   maps except for the evaluation operator.
8. Join daily rows on site, day, lane, raw-source fingerprint, forcing
   fingerprint, and geometry fingerprint, then expand hours by hour index;
   reject duplicates and report unmatched and censored rows.
9. Decompose every external term and the implemented subset into same-state,
   sequential, and sequential-minus-same-state values at common support.
10. Track first effective carrier-input fingerprints, layer membership, active
    mass/depth/cold content/temperature, every substep endpoint, and evaluated
    support so projection and state evolution are not inferred from energy.
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
- `crates/openwepp-hillslope-orchestrator/src/lib.rs` for additive diagnostic
  exports only;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs`;
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs`;
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00f_snow_accumulation_melt_trace.rs`;
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00g_snow_diagnostic_capture.rs`;
- evaluation trace formatting/tests under
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`;
- a new schema-v6 formatter module in that directory. The near-threshold
  `00c_day_input_builder_impl.rs` may receive only a net-line-reducing handoff:
  move its schema-selection helper into the new module and keep its terminal
  line count below the scaffold baseline;
- focused Stage 3 runtime tests required by the changed additive diagnostic
  types;
- package-local tracked analyzer and tests at
  `tools/run_operator_reconciliation.py` and
  `tools/test_run_operator_reconciliation.py`;
- typed assurance-source adoption paths
  `docs/assurance/snow-frost-report.yaml`, `docs/assurance/sources.yaml`, and
  generated receipts under `docs/assurance/generated/` selected by the existing
  workflow;
- `docs/work-packages/README.md`, `docs/ROADMAP.md`, and
  `docs/planning/snow-surface-energy-balance-roadmap.md`;
- ignored `target/snow_stage3_operator_reconciliation/` raw evidence and normal
  ignored build/test outputs.

No fixture, observation, dependency, selector default, public output, or
unrelated crate change is authorized. A needed path outside this set requires a
prospective package amendment before that edit.

Existing exported `TurbulentFluxes`, `DirectSnowStage3EvaluationDiagnostics`,
`DirectSnowStage3EvaluationHourDiagnostics`, and
`DirectSnowStage3EvaluationResult` shapes and call signatures are protected.
The new operator-reconciliation result is a boxed enabled-only companion;
disabled/default calls perform no companion allocation. Old and new projections
must share one private solver path; duplicated Rust physics is prohibited.

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

For every evaluated operator substep, schema v6 must publish one ordered tuple
with explicit applicability and exact duration semantics. Same-state emits one
`3600 s` tuple per evaluated hour; sequential emits every dynamic substep.
Hourly and window summaries are consumer-derived from these tuples, never used
as primitive reconstruction operands. Each tuple includes:

- air, dewpoint, and shadow surface temperature in degrees Celsius;
- air and saturated-surface vapor pressure in pascals;
- wind speed in metres per second and atmospheric pressure in pascals;
- `z_T`, `z_q`, `z_u`, and aerodynamic `z_0,aero` in metres; bare `z_0` is a
  rejected alias because it names the thermal active-layer depth elsewhere;
- Monin-Obukhov iteration count, solver options, stability-length
  applicability, Obukhov length, stability corrections, and log factors;
- friction velocity, sensible exchange velocity, latent exchange velocity,
  air density, air potential temperature, air/surface specific humidity, and
  surface latent heat;
- raw source plus effective first-input fingerprints, projection identifier,
  active and total layer membership/count, mass, depth, density, cold content,
  and surface temperature before and after each substep;
- incoming and net shortwave plus albedo; atmospheric, canopy, subcanopy,
  outgoing, and net longwave plus sky-view fraction;
- rain and snow mass flux, hydrometeor temperatures, temperature-dependent
  heat capacities, precipitation-advection, vapor mass, melt, sublimation,
  deposition, cold export, and residual operands; and
- site/day/lane/operator/hour/substep identity, elapsed start, requested and
  evaluated seconds, and a typed tuple applicability reason.

Every value is either applicable with positive evaluated support or explicitly
N/A. A non-applicable temperature, stability, endpoint, or transfer operand is
never encoded as numeric zero. The first effective input is captured after the
operator's existing projection/normalization and immediately before its first
carrier call. Same-state before/after state remains identical. Sequential
endpoints may differ.

The sign convention is positive energy or vapor mass toward the snow control
volume and negative away from it. Sensible heat is positive when the air-side
potential temperature exceeds the surface temperature. Vapor mass and latent
heat are positive for deposition/condensation and negative for
sublimation/evaporation. Net longwave is incoming atmospheric plus canopy
longwave minus outgoing snow longwave. Precipitation advection is positive when
the hydrometeor supplies heat relative to the surface.

The tracked real consumer is `tools/run_operator_reconciliation.py`, with
focused tests in `tools/test_run_operator_reconciliation.py`. It dispatches v5
and v6 explicitly, rejects unknown versions and invalid null/applicability
states, enforces exact clean-HEAD/environment/fixture custody, performs the
frozen two-stage join and independent equations, retains unmatched/censored
inventories, reproduces the predecessor estimator, and writes compact results.
It may not import or invoke producer calculation helpers. The analyzer and its
tests receive independent consumer review before result-bearing execution.

## Frozen Cohort And Operators

Reuse the exact four predecessor fixture copies, observation hashes, Snowbird
development-only precipitation derivative, water-year windows, and
`DIAGNOSTIC_ONLY` observation role bound directly and through the hashed
predecessor freezes in `artifacts/protocol-freeze.json`. Primary windows are
October 1 through the earliest maximum positive observed SWE date inclusive;
WY2025 is right-censored and excluded. Snowbird WY1990--2024 supplies the 35
predecessor windows and remains non-decisive for forcing truth.

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

The join is two stage. Daily rows join on site, day index, lane index, exact raw
source fingerprint, exact forcing fingerprint, and exact geometry fingerprint.
Joined arrays then expand positionally on hour index `0..23`; substeps remain
ordered by substep index and elapsed start. Duplicate daily identities,
duplicate tuple identities, missing hours, non-contiguous substeps, or
overlapping durations are identity failures. A mismatched fingerprint is not a
delta sample. Raw source, forcing, and geometry hashes must match before any
operator interpretation. Effective first-input fingerprints are compared, not
required to match, because their difference is an explicit estimand.

The primary comparison uses hours where both operators evaluated exactly 3600
seconds. For partial pairs, common support is
`min(same_state_evaluated_seconds, sequential_evaluated_seconds)` and each
operator is integrated only on `[0, common_support)`, splitting the final tuple
at that boundary without interpolation. All-evaluated and common-support
summaries remain separate. Each external term is independently reconstructed
and summed by the real consumer as `sum(flux_w_m2 * duration_seconds)`. The
complete external subset is shortwave + longwave + sensible + latent +
precipitation-advection. Internal active/lower conduction is reported
separately and remains nonexternal.

Primitive reconstruction must reproduce each substep's sensible, latent,
vapor-mass, shortwave, longwave, advected, and complete external term from
primitive tuple operands within the tolerances frozen in the protocol.
Sequential state closes for each substep as
`M_after = M_before - melt - sublimation + deposition` and
`C_after = C_before - cold_energy_change - cold_content_export`, with all mass
terms in `kg m^-2` and cold terms in `J m^-2`; hour endpoints are the first and
last applicable tuple. Same-state requires exact unchanged endpoints. Known wrong
aliases—authoritative Stage 3 surface temperature, CoE terms, internal
conduction, zero-filled N/A state, and calendar rather than evaluated support—
must produce distinct values in anti-tautology fixtures.

The frozen Snowbird predecessor estimand is the Python `statistics.median`
across 35 WY1990--2024 values, where each value sums
`stage3_shadow_complete_energy_j_m2` over October 1 through observed peak date
inclusive from the corrected retained sequential trace. Its target is
`+170.2536089 MJ m^-2`; reproduction tolerance is
`max(1e-6 J m^-2, 1e-12 * sum_abs_operands)` per window and `1e-7 MJ m^-2`
for the reported median. Sign classification uses an absolute zero tolerance
of `1e-6 J m^-2`, never exact floating-point sign alone.

The scientific disposition is one of:

1. `LINEAGE_OR_IDENTITY_FAILURE` when reconstruction, identity, applicability,
   closure, consumer custody, or protected-output identity fails. This class
   has precedence and suppresses causal claims.
2. `PREDECESSOR_NOT_REPRODUCED` when the current bounded sequential lane does
   not reproduce the predecessor's positive Snowbird sign.
3. `INITIAL_CONTROL_VOLUME_PROJECTION_DIFFERENCE` when raw-source identity
   passes but the first effective input fingerprint differs or any first active
   mass/depth/cold/temperature/layer-membership operand differs above the
   protocol tolerance. Report its per-term first-hour delta.
4. `STATE_EVOLUTION_RECONCILES_SIGN_CONTRADICTION` only when the current
   sequential lane reproduces the positive predecessor, the immutable lane is
   negative, and either first effective inputs match or an independently
   reconstructed frozen-active-projection reference separates projection from
   later evolution. State-dependent terms are longwave, sensible, latent,
   precipitation-advection, and vapor mass; state-independent shortwave must
   remain invariant within delta-closure tolerance.
5. `SUPPORT_CENSORING_MATERIALLY_CONTRIBUTES` when the sign of the complete
   external-subset delta differs between common and all-evaluated support, or
   when omitted-support energy exceeds `5%` of the all-evaluated absolute
   operand sum. This threshold is attribution-only and cannot validate fluxes.
6. `MULTIFACTOR_UNRESOLVED` when valid evidence does not uniquely satisfy the
   preceding causal classes.

After identity precedence, classes 2--5 may coexist and are reported in the
listed order. A causal class requires term-delta closure within
`max(1e-6 J m^-2, 1e-12 * sum_abs_operands)`. None is a
carrier-plausibility PASS.
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
- External-style exhaustive struct-literal compilation proves the four
  protected public types remain source-compatible. Stable, unstable, neutral,
  zero-wind, and nonconvergent turbulent vectors prove old/new `to_bits`
  parity.
- Disabled evaluation proves unchanged type size for protected public results,
  zero companion allocation, bounded runtime/RSS parity, and no v6 payload.
- Enabled stress evidence records the actual maximum tuple count and serialized
  bytes against the existing upper bound of 24 hours times 3,600 one-second
  substeps per day. Any new diagnostic cap or dropped tuple is prohibited.
- Primitive reconstruction, term totals, support, state endpoints, and
  rejected-alias tests pass through the real consumer.
- Consumer tests cover v4 golden identity, historical v5 parsing, v6 arrays and
  N/A status, unknown-schema failure, covariance-producing multi-substep data,
  full/partial/zero support, exact common-support splitting, and mass/cold
  endpoint closure.
- A whole-runtime symbol allowlist proves v6 fields and companion diagnostics
  reach only evaluation capture/trace paths. Enabled failures preserve the
  authoritative result, and both operators retain byte-identical WAT/HBP/PASS
  outputs against control.
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
- [x] (2026-08-06) Committed result-blind scaffold at `30e843d4116411520cf9eeb7f08a3bf1ce853b78`.
- [x] (2026-08-06) Independent science and Rust reviews returned HOLD; amended
  projection/evolution separation, exact substep operands, protocol custody,
  additive API, analyzer path, negative proof, and line-count constraints.
- [ ] Obtain PASS/PASS on the amended result-blind admission commit.
- [ ] Amend contract authority and pass the pre-implementation contract gate.
- [ ] Implement behavior-neutral schema-v6 observability and focused tests.
- [ ] Execute and analyze the frozen four-site paired/sequential cohort.
- [ ] Complete review, critical validation, DRAFT assurance adoption, terminal
  verification, roadmap disposition, prompt archival, and stable closure.

## Surprises & Discoveries

- Observation: both operators call the same complete-carrier function, but the
  same-state operator passes the whole column while sequential first aligns an
  upper active control volume. Raw-source equality therefore does not prove
  equal first carrier inputs. Evidence: `stage3_solver/evaluation.rs`.
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
- Decision: emit exact substep tuples and preserve existing public result
  shapes through additive diagnostics. Rationale: nonlinear turbulent products
  cannot be reconstructed from hourly primitive means, while altering existing
  exported struct fields would break downstream exhaustive consumers.
  Date/Author: 2026-08-06 / Codex.
- Decision: decompose initial projection, later evolution, and support before a
  state-evolution claim. Rationale: the two admitted operators use different
  first effective control volumes even with byte-identical raw sources.
  Date/Author: 2026-08-06 / Codex.

## Outcomes & Retrospective

Queued. Completion requires a frozen, independently reviewed operator
reconciliation result and truthful next-step disposition. CoE remains
authoritative throughout.

## Revision Note

2026-08-06: Initial contract-first scaffold created from the failed four-site
carrier audit and its worker handoff.

2026-08-06: Result-blind science review HOLD disposition amended the freeze to
separate first control-volume projection from later state evolution, replace
hourly means with exact substep tuples, bind cohort/predecessor/tolerances, and
make join, applicability, closure, and decision predicates executable.
