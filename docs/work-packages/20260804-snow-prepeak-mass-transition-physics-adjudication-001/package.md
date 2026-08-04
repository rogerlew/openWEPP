# SNOW-PREPEAK-MASS-TRANSITION-PHYSICS-ADJUDICATION

Status: `complete / upstream-generation priority / review and verification pass`

Date: `2026-08-04`

Plan class: `Read-only, cross-fixture process-physics adjudication`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
The `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` sections must remain current during execution.

## Purpose / Big Picture

Use the durable mass-transition ledgers and complete schema-v4 snow trace to
decide where the accumulation-season SWE deficit is generated:

1. excessive or premature solid-to-liquid generation in the empirical CoE
   path;
2. physically consequential signed-hour or pre-export enthalpy ordering;
3. excessive downstream evacuation caused by insufficient retention/refreeze;
4. a multi-factor interaction; or
5. evidence that remains insufficient for a correction choice.

The package executes exact-current direct-production replays for Snowbird,
Mica Creek, Niwot, and Paradise. It reconstructs both durable boundaries,
attributes event/hour signatures, tests prospectively frozen diagnostic
counterfactual bounds, and produces figures plus machine-readable sidecars.

This package is adjudication, not implementation. It cannot change production
physics, contracts, selectors, defaults, tests, fixtures, observations, or
public outputs. A correction requires a separate contract-first package with
actual literature-, contract-, or pinned-baseline-backed physics.

## Context And Orientation

The accepted pre-peak audit found Snowbird median pre-peak snowpack loss
`0.5296 m` against `0.5379 m` gross-positive applied CoE melt. Stage-3-off and
explicit-longwave probes had zero authoritative mass response, while legacy
CoE routing produced an order-one rollback bound. The audit held because the
real JSONL consumer omitted Stage-3 liquid operands.

The follow-up trace package published exact Stage-3 incoming, routed, signed
retained change, refrozen, and residual operands. The persistence package then
made the upstream solid-to-liquid and downstream liquid-disposition ledgers
durable over one authoritative calculation while making the large hourly
payload opt-in. Both packages preserved schema-v4 and WAT/HBP/PASS behavior.

The missing-evidence hold is therefore lifted. What remains is a physical
ordering question: does the model generate/export liquid before the cold,
signed-hour, and storage state can physically constrain that transition, or is
the downstream disposition itself the dominant loss mechanism?

## Implementation Intent

- Intent: `characterization and correction-family adjudication`.
- Science implementation status: `IMPLEMENTED`; no equation change is in
  scope.
- Calibration evidence status: `NOT_APPLICABLE`.
- Identifiability status: `NOT_ASSESSED`; no parameter is fitted.
- Observation role: all SNOTEL SWE records are `DIAGNOSTIC_ONLY`.
- Counterfactual bounds: `ASSUMED_FOR_EXECUTION` diagnostic screens, not
  physical bounds, calibrated values, or candidate equations.
- Production/kernel edit intent: `none`.
- Exact terminal diff must remain documentation, package-local analysis code,
  generated package evidence, and catalog status only.

## Included Scope

1. Build one exact release CLI from the scaffold commit and record its hash.
2. Copy and hash the four retained baseline fixtures without mutation.
3. Execute the active direct-production selector bundle once per site with
   schema-v4 trace enabled.
4. Compare every pre-v4 trace field and protected WAT/HBP/PASS output against
   the accepted predecessor where comparable.
5. Reconstruct daily and pre-peak-window snow-storage, solid-to-liquid, and
   Stage-3 liquid-disposition identities independently.
6. Partition each site/year's accumulation-season water into snowfall, retained
   rain, solid-pack loss, routed liquid, retained-liquid change, refreeze,
   sublimation, and storage change.
7. Attribute positive and negative applied CoE terms by `A/B/C/D`, air-
   temperature class, precipitation class, radiation presence, mixed-signed
   day, pack state, and Stage-3 cold-content state.
8. Evaluate the frozen signed-hour cancellation and pre-export cold-content
   opportunity bounds without feeding them back into production state.
9. Quantify how much incoming Stage-3 liquid is routed, retained, and refrozen
   before observed peak SWE at every site and water year.
10. Produce cross-site and event-level figures with Markdown sidecars and
    machine-readable source tables.
11. Apply the frozen adjudication rules without post-result threshold changes.
12. Complete dual independent review, finding disposition, dual terminal
    verification, and truthful completion or hold.

## Excluded Scope

- Production Rust, canonical science contracts, integration tests, fixtures,
  observations, reference PDFs, selectors, defaults, runfile/API surfaces, or
  assurance report sources.
- Reinterpretation of empirical CoE `A/B/C/D` contributions as independently
  identifiable physical heat fluxes.
- Treating negative empirical melt as authoritative refreeze energy.
- Fitting an `rst`, melt factor, holding capacity, cold-content multiplier,
  export limiter, or site coefficient.
- Replaying ERA5, changing climate forcing, or claiming forcing error is absent.
- Promoting legacy routing merely because it supplies a favorable rollback
  bound.
- A production correction, activation, calibration, validation, or
  transferability claim.

## Intended Write Set

- `docs/work-packages/20260804-snow-prepeak-mass-transition-physics-adjudication-001/`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `target/snow_prepeak_mass_transition_physics_adjudication/`

Everything else is protected. Production, contract, test, fixture,
observation, assurance, and historical package edits are prohibited.

## Authority And Dependencies

- `SC-SNOWFREEZE-001` v124, especially `INV-SNOWFREEZE-015`, `019`, `080`,
  `088`, `090`, and `091`.
- `SC-SNOWENERGY-001` v6 for Stage-3 cold-content and energy semantics.
- `20260803-snow-prepeak-liquid-evacuation-physics-audit-001` accepted v3.
- `20260803-snow-stage3-liquid-signed-hour-trace-closure-001`.
- `20260803-snow-mass-transition-ledger-persistence-001`.
- `20260803-snowbird-rst-prepeak-flux-diagnostic-001`.
- Frozen local baseline fixtures under
  `target/snow_prepeak_liquid_evacuation_physics_audit_v3/fixtures/baseline_replay/`.
- SNOTEL SWE files embedded in those local project fixtures, diagnostic only.

Canonical science authority remains in the `SC-*` contracts. Package results
may rank correction families but cannot create equation authority.

## Prospective Analysis Contract

The scaffold `artifacts/analysis-freeze.json` was rejected before result-bearing
execution by the result-blind static audit. `artifacts/analysis-freeze-v2.json`,
`artifacts/protocol-amendment.md`, `artifacts/operand-lineage.md`, and the v2
truth table are binding for execution.
The executor may correct a deterministic implementation bug in the
package-local analysis tool only if the correction, invalidated result custody,
and unchanged scientific operator are recorded. Result-driven changes to
windows, censoring, thresholds, classes, or verdict rules require a rejected
result namespace and a new prospectively frozen version.

Primary windows run from October 1 through the observed positive SNOTEL peak,
inclusive. WY2025 remains right-censored and excluded from primary summaries;
it is reported only as an explicit sensitivity. Results are per site and water
year before any pooled summary.

The active selector bundle is frozen to:

- Stage 3 `layered_thermal_liquid_v1`;
- density `physics_bulk_multilayer_density_v1`;
- phase `harder_pomeroy_hourly`;
- melt `coe_liquid_holding_capacity_v1`;
- explicit longwave disabled; and
- sublimation disabled.

## Conservation And Output Acceptance

Acceptance requires independent reconstruction of:

```text
runtime_SWE_after - runtime_SWE_before
  = snowfall + retained_rain - solid_pack_loss - sublimation

liquid_handoff = solid_pack_loss + released_rain

stage3_incoming - stage3_routed - stage3_retained_delta
  - stage3_refrozen = stage3_residual
```

The upstream/downstream shared handoff must be the exact produced operand.
Tests and the analysis tool must reject raw signed melt, positive hourly melt,
top-level CoE routed melt, retained store, omitted retained delta, doubled
refreeze, and producer residual self-trust as aliases. Self-consistency and
one-sided bounds are supporting evidence only.

## Adjudication Boundaries

The package may conclude that a correction family is the next justified
target only when its prospectively frozen signatures hold across the declared
cohort. It must distinguish:

- demonstrated implementation interaction;
- feedback-free diagnostic upper bound;
- physical inference;
- forcing/observation uncertainty; and
- actual correction authority.

A signed-hour or cold-content opportunity bound cannot be called a simulated
SWE recovery. The CoE components cannot be called energy shares. A high routed
fraction cannot by itself prove the routing rule is wrong; it only excludes
retention/refreeze as the dominant current storage response.

## Real Consumer Proof

The exact release CLI must write the four schema-v4 traces and protected
outputs. The independent package parser must consume those files directly.
Existing predecessor summaries, producer-only structs, unit-test fixtures, or
copied ledger equations cannot close the claim.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to up to four read-only Rust/science investigators for independent state,
energy, forcing/event, and cross-fixture audits; two independent read-only
reviewers; two independent read-only terminal verifiers; and one
`comparator_suite_runner` for exact CLI replay or validation work under
`target/snow_prepeak_mass_transition_physics_adjudication/`. Expected outputs
are compact findings, reproduced metrics, and evidence paths. Investigators,
reviewers, and verifiers are read-only. The comparator may write only normal
build/test and package-target outputs. The orchestrator alone edits tracked
files.

## Phase Plan

### Phase A: Scaffold And Freeze

Create and commit the package, prompt, queued artifacts, exact input hashes,
operand lineage, analysis rules, catalogs, and required-reading map.

### Phase B: Independent Static Audits

Trace authoritative state ownership and ordering, inspect CoE term/branch
semantics, audit Stage-3 retention/refreeze/cold-content ownership, and challenge
the frozen discriminants before reading new results.

### Phase C: Exact Four-Site Execution

Build the scaffold release CLI, copy/hash fixtures, run four schema-v4 traces,
validate protected outputs, stream-parse the exact files, and retain receipts.

### Phase D: Cross-Fixture Adjudication

Reconstruct ledgers, quantify event/hour classes and opportunity bounds,
produce figures/sidecars, and assign each hypothesis a supported, excluded,
bounded, or unresolved verdict.

### Phase E: Review And Closure

Complete two independent reviews, disposition every finding, remediate accepted
findings, run direct terminal gates, obtain two fresh verifications, reconcile
the write set, archive the prompt byte-identically, and commit locally.

## Validation And Exit Criteria

- Frozen fixture, observation, binary, selector, trace, and analysis identities
  are complete and reproducible.
- Four exact-current real CLI replays succeed with schema-v4 traces.
- Pre-v4 fields and protected outputs are unchanged where the predecessor
  supplies an exact comparable surface.
- Both mass-transition ledgers and snow-storage closure reconstruct within
  canonical tolerances for every row/window.
- Every plausible alias named above is separated on non-aliased rows.
- Every site/year is reported; pooled results cannot hide heterogeneity.
- CoE term, temperature, precipitation, radiation, signed-hour, cold-content,
  and Stage-3 disposition summaries use frozen classes and units.
- Diagnostic bounds are labeled feedback-free and
  `ASSUMED_FOR_EXECUTION`; no simulated recovery claim is made.
- The final correction-family ranking follows the frozen adjudication rules.
- Figures have source-data sidecars, units, denominators, hashes, and claim
  limits.
- Production, contract, test, fixture, observation, and assurance paths remain
  untouched.
- Package-local Python tests/syntax, JSON parsing, exact reproduction,
  documentation lint, spelling review, format/diff hygiene, release build, and
  applicable focused snow contract tests pass.
- Dual review, complete finding disposition, dual verification, prompt
  archival, line-count governance, exact write-set reconciliation, and truthful
  disposition pass with no deferred current-scope gate.

No full-workspace correctness profile is required because the intended and
terminal production/kernel diff is empty. Any discovered production edit need
would stop this package and require a separate contract-first work package.

## Security And Data Impact

Security impact is `none expected`. Execution uses local fixtures, local
observations, local binaries, and disposable target paths. It must not read
credentials, contact external services, mutate source fixtures, serialize
secrets, or publish absolute credential paths. Existing `OPENWEPP_*` state is
removed before each replay and the exact allowed selector map is recorded.

## Progress

- [x] (2026-08-04) User authorized scaffolding and end-to-end execution.
- [x] (2026-08-04) Committed scaffold and v1 freeze at `6ab0946b`.
- [x] (2026-08-04) Result-blind audits rejected incomplete v1 operators and
  froze the corrected v2 protocol before model execution.
- [x] (2026-08-04) Completed four exact CLI replays, 61,364-row compatibility
  comparison, 154 primary windows, independent reconstruction, and six figures.
- [x] (2026-08-04) Completed dual review, finding disposition, dual terminal
  verification, lifecycle reconciliation, and local completion commit.

## Surprises & Discoveries

- The scaffold's downstream screen was causally inverted: current Stage-3
  disposition controls neither runtime SWE nor hydrologic liquid publication.
- `stage3_cold_content_before_j_m2` is post-CoE/post-density state, so it cannot
  quantify pre-debit cold content.
- Producer Stage-3 retained amount is newly retained incoming, not the complete
  signed day-over-day layer-store delta.
- The initial compatibility parser counted expected v4 schema/hourly additions
  as differences. Retained outputs were identity-checked and reanalyzed after
  projection repair; all 61,364 pre-v4 rows are exact.
- Daily-local signed opportunity is positive in every primary site-year but
  passes the frozen materiality screen at only two sites. Cold opportunity and
  Stage-3 capture are small at every site.

## Decision Log

- Decision: execute a new exact-current four-site schema-v4 surface rather than
  infer cross-site Stage-3 behavior from the Snowbird-only closure run.
  Rationale: the correction-family claim is systemic and requires identical
  downstream operands at every retained site.
  Date/Author: 2026-08-04 / Codex.
- Decision: keep every opportunity calculation feedback-free and diagnostic.
  Rationale: the empirical CoE terms and Stage-3 cold content do not by
  themselves authorize a replacement coupled equation.
  Date/Author: 2026-08-04 / Codex.
- Decision: supersede v1 with a result-blind v2 protocol before execution.
  Rationale: exact operators and a causally valid truth table were required;
  numerical thresholds, cohort, selectors, and observation role did not move.
  Date/Author: 2026-08-04 / Codex plus four independent static audits.
- Decision: rank `UPSTREAM_GENERATION_PRIORITY` without authorizing a
  correction.
  Rationale: Stage 3 is excluded as the current causal SWE/runoff path, while
  daily signed and post-CoE cold indices fail cohort-wide materiality. Gross
  CoE/loss tracking localizes but does not prove excessive generation.
  Date/Author: 2026-08-04 / Codex.

## Outcomes & Retrospective

Execution supports an upstream-generation/total-snow-water authority successor,
not downstream retention tuning. The exact current ledgers close, no duplicate
SWE debit exists, signed-hour asymmetry is insufficient across the cohort, and
post-CoE cold content is both temporally mislocated and quantitatively small.
No production correction, fitted parameter, forcing exoneration, or validation
claim is available. Dual independent review and fresh terminal verification
pass with no remaining findings.
