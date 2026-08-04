# SNOW-ACCUMULATION-TARGET-FEASIBILITY-AND-INPUT-LOSS-DISCRIMINATION

Status: `scaffolded / queued / prospective operators frozen`

Date: `2026-08-04`

Plan class: `Read-only external-authority diagnostic and mechanism discrimination`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
The `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` sections must remain current during execution.

## Purpose / Big Picture

Determine whether the four-site accumulation-season SWE deficit is primarily
consistent with insufficient input mass, excessive modeled loss, both, or an
unresolved observation/representativeness mismatch. The package combines an
all-phase zero-loss mass ceiling with event-scale comparisons that keep
forcing magnitude, modeled snowfall, modeled storage response, and modeled
pack loss distinct.

The observable outcome is a complete four-site result set, figures with source
sidecars, an explicit evidence truth table, and a correction-family handoff.
This is characterization only. It cannot fit a multiplier, alter observations,
change model physics, or treat SNOTEL point data as hillslope truth.

## Context And Orientation

The closed predecessor localized current modeled pre-peak snowpack loss to the
upstream empirical CoE mutation path but did not compare that loss against a
complete all-phase input ceiling. Its post-closure review showed that Snowbird's
checked-in climate supplies a median `0.823` of observed pillow peak SWE even
with zero loss, while the model reaches only `0.382` of the observed peak.

Snowbird also publishes `PRCPSA`, a derived snow-adjusted precipitation series
that uses pillow SWE changes. It restores station-level mass headroom but is
circular when the pillow is also the validation target. It is therefore
diagnostic context, not independent validation or correction authority.

The retained schema-v4 traces under
`target/snow_prepeak_mass_transition_physics_adjudication_v2/` contain exact
daily modeled snowfall, storage, and pack-loss operands for Mica Creek, Niwot,
Paradise, and Snowbird. This package reads and verifies those immutable outputs;
it does not rerun the production model because the current source change since
their accepted execution is documentation-only and excludes every executable
input or consumer.

## Implementation Intent

- Intent: `characterization and causal-family discrimination`.
- Science implementation status: `IMPLEMENTED`; no equation change is in scope.
- Calibration evidence status: `NOT_APPLICABLE`.
- Identifiability status: `PARTIALLY_IDENTIFIABLE` only at the broad
  input-versus-loss family level; no parameter is fitted.
- Observation role: SNOTEL data are `DIAGNOSTIC_ONLY` in this package.
- Production/kernel edit intent: `none`.
- Validation risk: `editorial plus empirical-analysis`; no executable product,
  fixture value, authority-suite binding, or observation value changes.

## Included Scope

1. Verify exact identities of the four retained schema-v4 traces, checked-in
   climates, normalized SNOTEL observations, predecessor annual results, and
   Snowbird PRCPSA provenance sidecar.
2. Reconstruct every complete uncensored October 1 through observed-peak window.
3. Compute the zero-loss current-input mass ceiling from initial SWE plus
   all-phase fixture precipitation.
4. Keep all-phase input, modeled snowfall plus retained rain, modeled pack loss,
   modeled observed-date storage, and within-window modeled peak distinct.
5. Reproduce guarded cumulative-gauge comparisons without bridging gaps or
   water-year resets.
6. Identify cold observed-accumulation events prospectively and compare observed
   SWE gain with fixture all-phase input, modeled snowfall, modeled storage
   change, and modeled pack loss.
7. Identify guarded dual-sensor dry intervals prospectively and compare observed
   SWE loss with modeled CoE-owned pack loss, annual-first and site-median.
8. Carry the EB-04W2 response shape, Snowbird SNOTEL-conditioned forcing
   response, wet-winter Tmax bias, and missing upper event tail as context only.
9. Generate four figures with Markdown interpretation sidecars and retained CSV
   source tables.
10. Apply the frozen truth table without result-aware threshold changes.
11. Complete dual independent review, finding disposition, dual independent
   verification, exact-diff reconciliation, and truthful closure or hold.

## Excluded Scope

- Production Rust, science contracts, selectors, defaults, public schemas,
  tests, fixtures, observations, or normalized data.
- Any model rerun, precipitation scaling grid, `rst` sweep, or new provider
  acquisition.
- Treating `PREC`, `PRCP`, `PRCPSA`, or `WTEQ` as error-free truth.
- Treating `PRCPSA` as independent validation, an observed forcing, or a direct
  correction factor.
- A fitted forcing multiplier, phase threshold, melt factor, storage limiter,
  site coefficient, or default recommendation.
- Attribution of point-versus-hillslope differences to gauge undercatch,
  lateral redistribution, canopy interception, elevation, or one model process
  without independent scale-matched evidence.
- Reopening or revising the predecessor package's frozen result set.

## Intended Write Set

- `docs/work-packages/20260804-snow-accumulation-target-feasibility-input-loss-discrimination-001/`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `target/snow_accumulation_target_feasibility_input_loss_discrimination/`

Everything else is read-only. Generated target outputs remain untracked.

## Authority And Dependencies

- `docs/work-packages/20260804-snow-prepeak-mass-transition-physics-adjudication-001/`
  accepted results and post-closure disposition.
- `docs/work-packages/20260803-snowbird-snotel-climate-forcing-diagnostic-001/`.
- `docs/work-packages/20260803-snowbird-snotel-cligen-swe-response-001/`.
- `docs/work-packages/20260802-snow-surface-eb-04w2-precipitation-scaling-grid-extension-001/`.
- `tests/fixtures/snotel_observed/` climates and normalized observations.
- `tests/fixtures/snotel_observed/observations/provenance/snotel_snowbird_ut_prcpsa_diagnostic.json`.
- Retained target traces named above, verified against their predecessor receipt.

Canonical physics authority remains in the `SC-*` contracts. These observed
comparisons are A4 investigation evidence and cannot create equation authority.

## Prospective Analysis Contract

`artifacts/analysis-freeze.json` is binding before any new result-bearing
execution. It defines source identities, primary windows, guarded precipitation
increments, mass-ceiling operands, cold-event eligibility, dry-interval
eligibility, annual-first aggregation, materiality thresholds, coverage floors,
and the final truth table.

Results may not change the frozen `0.95` mass-ceiling screen, `0.80` event-ratio
screen, `0.00508 m` observed accumulation-event threshold, `0.000254 m d^-1`
dry threshold, `0.01 m` annual dry-loss materiality threshold, coverage floors,
or three-of-four systemic rule. A deterministic implementation error may be
corrected only before accepting results, with the rejected result namespace and
unchanged scientific operator recorded.

The package reports distributions and continuous values regardless of screen
outcome. Screens classify evidence; they do not transform measurements into
calibration targets.

## Operand Lineage And Anti-Tautology

`artifacts/operand-lineage.md` distinguishes every observed, climate, modeled,
and derived quantity. The analysis must reject these substitutions:

- modeled snowfall for all-phase fixture precipitation;
- modeled pack loss for observed pillow SWE loss;
- within-window modeled peak for modeled storage on the observed peak date;
- cumulative `PREC` values for guarded daily precipitation increments;
- `PRCPSA` for independent precipitation authority; and
- same-equation producer residuals for independent closure.

Mass-ceiling and event comparisons are magnitude audits, not conservation
proofs. Existing independently verified trace closure is reused only after exact
input identity passes.

## Adjudication Boundaries

The final result is one of:

- `INPUT_PRIORITY_SIGNAL`;
- `LOSS_PRIORITY_SIGNAL`;
- `MULTIFACTOR_INPUT_AND_LOSS_SIGNAL`; or
- `UNRESOLVED_OR_COVERAGE_LIMITED`.

Subsidiary screens separately report current-input mass limitation, all-phase
cold-event input limitation, modeled-snowfall limitation, and guarded dry-loss
excess. A modeled-snowfall shortfall with adequate all-phase input is only a
`PHASE_OR_SOLID_INPUT_SIGNAL`; event timing, canopy, and footprint remain
confounded. No result authorizes a production correction.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two read-only scientific reviewers and two read-only terminal verifiers.
Expected outputs are compact findings, independent metric spot checks, and
evidence-path citations. They may not edit files. The orchestrator owns all
tracked and target writes. No heavy batch, comparator suite, full-workspace
regression, or `comparator_suite_runner` is selected.

## Phase Plan

### Phase A: Scaffold And Freeze

Create and commit this package, kickoff prompt, required-reading map, analysis
freeze, operand lineage, queued artifacts, and roadmap/catalog entries.

### Phase B: Implement And Validate The Analyzer

Implement package-local deterministic analysis and figure tools plus focused
unit tests. Validate source identities before producing accepted results.

### Phase C: Execute The Four-Site Analysis

Stream the exact retained traces once, construct annual mass ceilings, cold
events, and dry intervals, write machine-readable results and source tables,
and render the four figures with Markdown sidecars.

### Phase D: Synthesize And Adjudicate

Apply the frozen screens, integrate predecessor context, distinguish supported
signals from representativeness limits, and write the scientific disposition
and worker handoff.

### Phase E: Review, Verify, And Close

Complete dual independent review, disposition every finding, correct accepted
findings, run direct terminal validation, obtain two fresh verifications,
archive the active prompt byte-identically, reconcile the exact diff, update
catalogs, and create the completion commit.

## Validation And Exit Criteria

- Every retained trace and checked-in input hash matches its declared source.
- All complete uncensored primary windows reproduce predecessor peak and ledger
  values within `1e-9 m`.
- Guarded gauge increments never bridge gaps or water-year resets.
- Event and interval groups are disjoint and deterministically ordered.
- Annual-first and site-median summaries reproduce independently.
- Frozen screens are applied without threshold changes.
- Every figure has a committed Markdown sidecar and a checksum-bound CSV source
  table retained under the target namespace.
- Package-local Python syntax and focused tests pass.
- JSON parsing, Markdown lint, spelling preview, link/path checks, and
  `git diff --check` pass.
- External-authority anti-evasion guards pass because this analysis reads the
  cohort even though it does not alter cohort membership or observations.
- Exact terminal diff remains within the intended write set.
- `.rs` line-count governance records no Rust diff and therefore no new
  2000/3000-line exposure.
- Dual review, complete finding disposition, dual verification, prompt archival,
  and truthful final disposition pass with no unresolved current-scope gate.

No full-workspace Rust regression is selected because the exact intended and
terminal executable diff is empty. Existing trace execution evidence is reused
only under exact identity.

## Security And Data Impact

Security impact is `none expected`. The package reads local public observation
fixtures and retained local outputs. It performs no credential reads and
requires no network access. It writes only package-local artifacts and an
untracked target namespace. No raw provider response, secret, or local
credential path may be committed.

## Progress

- [x] (2026-08-04) User authorized package scaffolding and end-to-end execution.
- [x] (2026-08-04) Froze the prospective operators, truth table, write set, and
  no-correction boundary.
- [ ] Commit the validated scaffold checkpoint.
- [ ] Implement and test the package-local analyzer and figure renderer.
- [ ] Execute the four-site retained-data analysis and generate figures.
- [ ] Complete synthesis, dual review, finding disposition, dual verification,
  prompt archival, and final closure.

## Surprises & Discoveries

- None at scaffold. Result-bearing evidence has not been computed.

## Decision Log

- Decision: use retained exact-current schema-v4 traces rather than rerun the
  production model.
  Rationale: subsequent source changes are documentation-only, and exact trace
  hashes plus predecessor closure permit evidence reuse without a redundant
  2.7 GB replay.
  Date/Author: 2026-08-04 / Codex.
- Decision: use both mass ceilings and event-scale discriminants.
  Rationale: a seasonal scalar cannot distinguish missing input from loss that
  occurs after input, while event comparisons can preserve their ordering.
  Date/Author: 2026-08-04 / Codex.
- Decision: retain PRCPSA as Snowbird-only diagnostic context.
  Rationale: it falsifies a hard raw-gauge ceiling but incorporates the pillow
  target and is therefore non-independent.
  Date/Author: 2026-08-04 / Codex.

## Outcomes & Retrospective

Pending execution.

Revision note: initial scaffold freezes a read-only, annual-first and
event-scale discrimination protocol before new result computation.
