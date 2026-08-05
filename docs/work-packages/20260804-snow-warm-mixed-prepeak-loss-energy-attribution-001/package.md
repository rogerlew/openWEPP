# SNOW-WARM-MIXED-PREPEAK-LOSS-ENERGY-ATTRIBUTION

Status: `scaffolded / queued for execution`

Date: `2026-08-04`

Plan class: `Read-only corrected-state empirical attribution`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
The `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` sections must remain current during execution.

## Purpose / Big Picture

Rebaseline the four canonical SNOTEL lanes and the development-only scaled
Snowbird lane after 21K removed the duplicate wet-compaction liquid alias, then
determine when modeled pre-peak snowpack loss occurs and which forcing,
empirical CoE melt-depth, snow-state, and downstream Stage-3 energy/disposition
signals accompany warm and mixed-temperature loss.

The observable result is a receipt-bound five-lane analysis, source tables,
figures with Markdown sidecars, a prospective truth-table disposition, and a
bounded next-step recommendation. This package characterizes the corrected
model. It does not tune, amend science authority, or land a production fix.

## Context And Orientation

21J found a guarded dry-loss signal at Niwot, Paradise, and Snowbird, but its
traces preceded the 21K density correction. 21K proved that the retired
wet-compaction driver double-counted state loss through a routed-liquid alias.
The corrected driver is materially smaller and changes snow density, depth,
liquid retention, and Stage-3 disposition while preserving generated melt and
upstream snow mass.

21K retained exact corrected-state schema-v4 traces for Mica Creek, Niwot,
Paradise, canonical Snowbird, and a precipitation-only Snowbird derivative.
The derivative uses exact factor `1.2155576` and is `DEVELOPMENT_ONLY`.
Canonical lanes own all scientific acceptance; the derivative can show only
how more supplied precipitation changes state-mediated response.

The hourly `amelt`, `bmelt`, `cmelt`, and `dmelt` values are empirical CoE
melt-depth formula contributions. They are not measured energy fluxes and are
not uniquely identifiable physical shares. Stage-3 energy fields describe the
downstream layered liquid/thermal response after the upstream CoE mutation;
they cannot be relabeled as the cause of generated melt.

## Implementation Intent

- Intent: `characterization and mechanism-family attribution`.
- Science implementation status: `IMPLEMENTED`; no equation change is in scope.
- Calibration evidence status: `NOT_APPLICABLE`.
- Identifiability status: `PARTIALLY_IDENTIFIABLE` at chronology and empirical
  driver-family level only.
- Observation role: SNOTEL is `DIAGNOSTIC_ONLY`.
- Production/kernel edit intent: `none`.
- Validation risk: `empirical-analysis plus editorial`; executable product,
  contract, fixture, observation, and public schema changes are excluded.

## Included Scope

1. Verify the 21K receipt, materiality result, five corrected trace identities,
   four normalized observation identities, and canonical/development climate
   identities before accepting results.
2. Reconstruct October 1 through observed-peak windows and corrected-state
   modeled peak, observed-date SWE, snowfall, snow-contact rain, pack loss,
   sublimation, and retained-liquid behavior.
3. Reapply the 21J guarded dry-period operator on corrected traces without
   changing thresholds, and quantify the pre-21K-to-corrected delta.
4. Classify active-pack hours as cold, mixed-day, or warm-day from the exact
   hourly forcing, preserving hourly temperature and precipitation separately.
5. Attribute positive applied CoE melt and pack loss by thermal class, month,
   water year, and empirical term family while retaining signed term values,
   cap adjustment, applied melt, and snow-state operands.
6. Report Stage-3 surface, shortwave, longwave, latent, conduction, cold-content,
   refreeze, retained-liquid, and routed-liquid fields as downstream response.
7. Pair canonical and scaled Snowbird on identical dates and forcing fields;
   quantify state-mediated differences without using the scaled lane for truth.
8. Produce deterministic source tables and at least four figures with committed
   Markdown interpretation sidecars.
9. Apply the prospective adjudication matrix without result-aware changes.
10. Complete dual independent review, finding disposition, dual independent
    verification, exact-diff reconciliation, and truthful closure or hold.

## Excluded Scope

- Production Rust, science contracts, assurance authority, selectors, defaults,
  public schemas, tests, fixtures, observations, or normalized data.
- New model executions, provider acquisition, precipitation grids, `rst`
  sweeps, or new physical parameter candidates.
- Treating empirical CoE components as measured radiation, sensible, latent,
  turbulent, or rain-energy shares.
- Treating downstream Stage-3 energy as upstream melt-generation causation.
- Fitting or recommending forcing multipliers, phase thresholds, melt factors,
  canopy factors, density constants, or site coefficients.
- Treating the scaled Snowbird lane as precipitation truth, calibration,
  validation, default evidence, or transferability evidence.
- A production correction based only on temporal association, correlation,
  component dominance, or SNOTEL point comparisons.

## Intended Write Set

- `docs/work-packages/20260804-snow-warm-mixed-prepeak-loss-energy-attribution-001/`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `target/snow_warm_mixed_prepeak_loss_energy_attribution/` (untracked results)

Everything else is read-only.

## Authority And Dependencies

- 21K package, worker handoff, accepted execution receipt, materiality result,
  and five corrected traces.
- 21J frozen dry-period operator and retained result tables.
- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-088` empirical-term semantics and
  `#INV-SNOWFREEZE-092` corrected wet-compaction authority.
- Normalized SNOTEL observations and checked-in canonical/development climates.

These are A4 diagnostic comparisons. Canonical physics authority remains in
the `SC-*` contracts.

## Prospective Analysis Contract

`artifacts/analysis-freeze.json` is binding before result-bearing execution.
It freezes source hashes, windows, thermal classes, dry-period eligibility,
aggregation, materiality screens, paired-lane rules, and the adjudication
matrix. A deterministic implementation defect may be corrected only before
accepting results, with the rejected namespace and unchanged scientific
operator recorded.

Primary summaries are annual-first, then site medians. Daily or hourly pooling
may be reported only as a clearly labeled exposure-weighted secondary view.
All continuous distributions remain visible regardless of threshold outcome.

## Operand Lineage And Anti-Tautology

`artifacts/operand-lineage.md` freezes units, stages, and ownership. The
analysis must reject these substitutions:

- `snowpack_swe_loss_m` for gross positive generated melt;
- routed liquid for snowpack state loss;
- empirical CoE terms for measured energy shares;
- Stage-3 surface energy for upstream CoE forcing;
- within-window modeled peak for modeled SWE on the observed peak date;
- fixture precipitation for observed-gauge truth; and
- scaled Snowbird response for canonical acceptance.

Daily mass and energy closure fields are validated independently before they
are used. Exact producer self-consistency supports execution integrity but does
not establish physical correctness.

## Prospective Adjudication

The terminal result selects one primary disposition:

- `WARM_MIXED_COE_LOSS_CONCENTRATION_SIGNAL`;
- `STATE_MEDIATED_INPUT_SENSITIVITY_SIGNAL`;
- `MULTIFACTOR_WARM_MIXED_AND_STATE_SIGNAL`;
- `NO_SYSTEMIC_WARM_MIXED_SIGNAL`; or
- `UNRESOLVED_OR_COVERAGE_LIMITED`.

A systemic warm/mixed signal requires at least three canonical sites with the
frozen minimum-year coverage and direction thresholds. A state-mediated signal
requires the paired scaled Snowbird screen but remains development-only. No
disposition is production-correction authority.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two read-only scientific reviewers and two read-only
terminal verifiers. Expected outputs are compact findings, independent metric
reconstruction, evidence-path citations, and final verdicts. They may not edit
files. The orchestrator owns all tracked and target writes.

No full-workspace, comparator, population, or other heavy batch is selected;
`comparator_suite_runner` is not required.

## Phase Plan

### Phase A: Scaffold And Freeze

Create and commit the package, active kickoff prompt, required-reading map,
analysis freeze, operand lineage, queued artifacts, and roadmap/catalog state.

### Phase B: Implement And Validate The Analyzer

Implement package-local deterministic analysis, independent verification, and
figure rendering tools with focused unit tests. Fail closed on identity,
chronology, domain, or closure violations.

### Phase C: Execute Corrected-State Attribution

Read the exact five traces once, align observations and climates, emit annual,
daily, hourly-class, empirical-term, Stage-3, and paired-Snowbird tables, then
render figures and source sidecars.

### Phase D: Synthesize And Adjudicate

Apply the frozen matrix, distinguish upstream empirical melt association from
downstream energy response, compare with 21J, and write the scientific
disposition and next-step handoff.

### Phase E: Review, Verify, And Close

Complete dual review, disposition every finding, correct accepted findings,
run terminal validation, obtain dual verification, archive the active prompt
byte-identically, reconcile the exact diff, update catalogs, and commit.

## Validation And Exit Criteria

- Every frozen input identity matches.
- Five traces align exactly to their climate dates and contain finite required
  operands; canonical Snowbird and scaled Snowbird share all non-precipitation
  climate tokens.
- Corrected daily snow mass and Stage-3 mass/energy closure residuals remain
  within the frozen tolerances.
- Corrected primary-window and dry-period outputs reproduce independently.
- Thermal classes are mutually exclusive and exhaustive for eligible hours.
- CoE component, cap, and applied-melt reconstruction passes independently.
- Snowbird pairing is date-complete and reports canonical and development roles.
- Annual-first summaries and frozen screens reproduce independently.
- Every figure has a committed Markdown sidecar and checksum-bound source table.
- Package-local syntax and focused tests pass.
- JSON parsing, Markdown lint/validate, spelling preview, reference/path checks,
  and diff hygiene pass.
- External-authority anti-evasion guards pass because observed cohort inputs are
  read, even though no case or value changes.
- Exact terminal diff stays in the intended write set; no Rust diff means no
  new `.rs` line-count exposure.
- Dual reviews, complete finding disposition, dual verification, prompt
  archival, and truthful disposition pass with no unresolved current gate.

No Rust or full-workspace regression is selected because the intended and
terminal executable-product diff is empty.

## Security And Data Impact

Security impact is `none expected`. The package reads local public observations
and retained local outputs. It performs no network or credential access and
writes only package-local artifacts plus an untracked target namespace. No raw
provider response, secret, or local credential path may be committed.

## Progress

- [x] (2026-08-04) User authorized scaffolding and end-to-end execution.
- [x] (2026-08-04) Froze scope, source identities, operators, acceptance roles,
  and no-correction boundary before result execution.
- [ ] Commit the validated scaffold checkpoint.
- [ ] Implement and focused-test analysis, verification, and figure tools.
- [ ] Execute the five-lane corrected-state analysis and render figures.
- [ ] Complete synthesis, dual review, dual verification, and closure.

## Surprises & Discoveries

- Observation: 21K already retained every required corrected-state lane and the
  hourly empirical/Stage-3 diagnostics, so 21L needs no model rerun.
  Evidence: 21K execution receipt and five schema-v4 trace hashes frozen in
  `artifacts/analysis-freeze.json`.

## Decision Log

- Decision: Keep 21L read-only and receipt-bound.
  Rationale: 21K generated exact corrected-state traces; rerunning would add no
  authority and would weaken source identity.
  Date/Author: 2026-08-04 / Codex.
- Decision: Canonical lanes own acceptance; scaled Snowbird is paired
  development sensitivity only.
  Rationale: the multiplier is a bounded development transformation, not an
  independently observed forcing correction.
  Date/Author: 2026-08-04 / Codex.
- Decision: Separate empirical CoE melt-depth attribution from downstream
  Stage-3 energy response.
  Rationale: `INV-SNOWFREEZE-088` prohibits interpreting CoE terms as measured
  energy shares, and Stage 3 occurs after upstream melt generation.
  Date/Author: 2026-08-04 / Codex.

## Outcomes & Retrospective

Queued. Replace this section with the executed result and bounded handoff.

## Revision Note

2026-08-04: Initial scaffold freezes 21L after the completed 21K correction.
