# Snow Stage 3 Four-Site Carrier-Term Audit

Status: `executed / carrier screen FAIL / persistent shadow blocked / review and verification PASS`

Date: `2026-08-06`

Package ID: `20260806-snow-stage3-four-site-carrier-term-audit-001`

Plan class: `Read-only, result-bearing four-site science characterization`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
`Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` remain current throughout execution.

## Purpose

Determine which terms make the Stage 3 implemented external carrier subset positive or negative
over water-year-stratified resolved-snow condition samples at Mica Creek,
Niwot, Paradise, and Snowbird. Execute the admitted same-state paired operator
through the real schema-v5 consumer, independently reconstruct every term, and
compare water-year-first condition-sample summaries with prospectively frozen
literature context.

This package characterizes an evaluation carrier. It does not persist shadow
state, simulate seasonal Stage 3 melt chronology, alter production physics, or
authorize CoE retirement.

## Context

The Snowbird complete-carrier predecessor closed resolved-domain terminal
allocation but retained a strongly positive median seasonal balance and more
shadow melt than CoE. That sequential experiment reinitialized from the
post-CoE pack every day and did not publish term-complete lineage, so it could
not isolate the anomaly or support chronology claims.

Contract v128 now supplies a default-off `same_state_paired_carrier_v1`
operator. Both arms use the immutable post-CoE daily initial snapshot, identical
24-hour forcing and geometry, and equality-checkable non-formulation
fingerprints. Schema v5 exports hourly shortwave, longwave, sensible, latent,
precipitation-advection, vapor, support, and residual operands through the real
internal JSONL consumer.

## Implementation Intent

- Intent: `characterization only`.
- Science implementation status: `IMPLEMENTED`; no equation change is in
  scope.
- Calibration evidence status: `NOT_APPLICABLE`.
- Identifiability status: `NOT_ASSESSED`; no parameter is fitted.
- Observation role: SNOTEL SWE is `DIAGNOSTIC_ONLY` and selects the frozen
  water-year sampling window only.
- Literature screens: `ASSUMED_FOR_EXECUTION` context, not calibration,
  validation, authority, or transferable bounds.
- Production/kernel edit intent: `none`.

## Included Scope

1. Require an out-of-band full SHA for the exact independently admitted clean
   execution HEAD, build one release CLI there, and record commit/binary hashes.
2. Copy and hash the four retained real CLI fixtures without mutating sources.
3. Replace only the staged Snowbird climate file with the existing
   `DEVELOPMENT_ONLY` precipitation derivative and prove its exact consumer.
4. Execute one same-state paired-carrier replay per site with the frozen
   selector map and schema-v5 trace.
5. Stratify daily same-state samples by water-year windows from October 1
   through the observed positive SNOTEL peak, inclusive; exclude WY2025 from
   primary summaries and report the complete year census.
6. Report resolved-snow support separately from calendar-window support.
7. Reconstruct hourly complete-arm terms and the implemented external subset;
   reconstruct both arms and their delta at daily, water-year-sample, and
   site-median cadence from exact operands.
8. Report internal active/lower conduction separately and never label it
   snow-ground or include it in the same-state external carrier.
9. Explicitly report the absent snow-ground boundary.
10. Compare water-year-first site medians and distributions with the frozen
    literature context without tuning.
11. Disposition carrier plausibility and the next roadmap step.
12. Complete dual independent review, finding disposition, dual terminal
    verification, validation, prompt archival, and a stable local commit.

## Excluded Scope

- Production Rust, canonical contracts, integration tests, fixtures,
  observations, selectors, defaults, public outputs, assurance sources, or
  reference custody.
- Sequential melt, persistent state, peak-SWE response, terminal-event
  numerics, snow-ground heat, snow-free land energy, receiving-surface energy,
  promotion, or cutover.
- Site-specific fitting, albedo or canopy tuning, forcing rescaling beyond the
  already governed Snowbird development derivative, or post-result changes to
  windows, ranges, or gates.
- Calling internal active/lower redistribution a ground heat flux.
- Treating literature context as independent validation of these SNOTEL
  fixtures.

## Intended Write Set

- `docs/work-packages/20260806-snow-stage3-four-site-carrier-term-audit-001/`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- ignored rejected `target/snow_stage3_four_site_carrier_term_audit/`
- ignored admitted `target/snow_stage3_four_site_carrier_term_audit_v2/`
- normal ignored build/test outputs

Everything else is protected. A production, contract, test, fixture,
observation, assurance, or reference edit stops this package.

## Authority And Dependencies

- `SC-SNOWFREEZE-001` v128, especially `INV-SNOWFREEZE-093` and `094`.
- `SC-SNOWENERGY-001` v8, especially complete-carrier composition, exact-one
  lineage, and acknowledged missing snow-ground/terminal boundaries.
- `SNOW-STAGE3-SHADOW-SOLVER-EXTRACTION-AND-OBSERVABILITY` completion and
  worker handoff.
- `SNOW-STAGE3-COMPLETE-CARRIER-SHADOW-MELT` failed Snowbird screen.
- The canonical four retained fixture copies and diagnostic SNOTEL records.
- The governed Snowbird `precip_x1p2155576` development derivative.

Canonical science remains in `SC-*`. Package evidence may localize an anomaly
but cannot admit replacement physics.

## Frozen Cohort And Forcing

The cohort and exact hashes are in `artifacts/protocol-freeze.json` and
`artifacts/fixture-and-forcing-custody.md`.

- Mica Creek, Niwot, and Paradise use their canonical retained fixture copies.
- Snowbird uses a fresh canonical fixture copy whose staged `p8.cli` is
  replaced by the checksum-bound precipitation-only `1.2155576` derivative.
- Canonical Snowbird remains byte-preserved as provenance control and is not a
  second result lane.
- The derivative is `DEVELOPMENT_ONLY`: it is not precipitation truth, an
  observation, a calibration, a default, independent validation, or evidence
  transferable beyond this lane.

## Frozen Runtime Selectors

- density: `physics_bulk_multilayer_density_v1`;
- phase: `harder_pomeroy_hourly`;
- melt: `coe_liquid_holding_capacity_v1`;
- Stage 3: `layered_thermal_liquid_v1`;
- longwave: `dilley_unsworth_subcanopy_v1`;
- sublimation: `disabled`;
- evaluation: `same_state_paired_carrier_v1`; and
- legacy complete-carrier-shadow spelling: absent.

Every ambient `OPENWEPP_*` key is removed before a run, then only this map and
the per-run trace path are installed. CoE remains the sole authoritative melt
and snow-mass owner.

## Frozen Support And Aggregation

For each observation water year, select the maximum positive nonmissing SWE;
the earliest date wins ties. The primary window is October 1 of the prior
calendar year through that date, inclusive. WY2025 is retained only as a named
right-censored sensitivity.

The physical comparison support is exactly the schema-v5 hours with:

- operator, source, support, cadence, carrier, pairing, arms, claim class, and
  unresolved-boundary IDs equal to the frozen values;
- both non-formulation fingerprints equal and nonzero;
- `complete_carrier_evaluated=true`;
- requested and evaluated support both exactly `3600 s`; and
- finite exported operands.

Rows with no resolved post-CoE snow snapshot are explicit zero-coverage rows.
They count in calendar coverage but never become zero energy in the
resolved-snow mean. Every observation water year receives a census disposition.
Coverage never excludes a year silently; low coverage is reported and excludes
that condition sample from the prospective screen.

Compute every metric per site and water year first. A water-year sample is
screen-eligible only with at least `30` fully evaluated days and resolved-snow
coverage of at least `0.25` of calendar-window hours. Lower-coverage samples
remain reported but cannot enter the carrier screen. Each canonical decisive
site requires at least `10` screen-eligible water years; the same threshold
controls only Snowbird's diagnostic comparability. These thresholds are
`ASSUMED_FOR_EXECUTION` sampling adequacy rules, not physical bounds.

Site summaries are Python `statistics.median` across screen-eligible water-year
condition samples. Report minimum, median,
maximum, quartiles, positive/negative fractions, evaluated hours, calendar
hours, and coverage. Do not pool hours, joules, years, or sites for the primary
verdict.

The same-state operator does not produce an accumulation-season trajectory or
seasonal energy ledger: every day starts from an immutable post-CoE snapshot.
All sums and means are labeled window-stratified condition-sample statistics.
They may compare formulation terms under sampled states and forcing, but cannot
claim seasonal energy, chronology, melt, peak timing, or persistence.

## Frozen Operand Reconstruction

For each evaluated hour independently reconstruct the complete-arm implemented
external subset:

```text
Q_implemented_external_subset = Q_shortwave,complete + Q_longwave,complete
                              + Q_sensible,complete + Q_latent,complete
                              + Q_precipitation_advected,complete
```

At daily and water-year-sample cadence reconstruct:

```text
Q_surface = Q_shortwave,surface + Q_longwave,surface + Q_latent,surface

delta_external_subset_minus_surface = Q_sensible,complete
                                      + Q_precipitation_advected,complete
                                      + (Q_latent,complete - Q_latent,surface)
```

Shortwave and longwave must be identical across arms. Latent heat is a
formulation difference: the surface arm uses the selected surface latent path,
which is disabled in this audit, while the complete arm evaluates turbulent
latent exchange. Schema v5 exports both daily latent terms but only the complete
arm's hourly latent array. Hourly distributions therefore describe the complete
arm; the arm delta is independently reconstructed at daily and water-year-sample cadence.

The same-state pair marks internal active/lower conduction not applicable; it
must be exactly zero. The exported complete total and surface total must equal
the reconstructed sums within `1e-6 J m^-2` per daily row and within a
scale-aware `max(1e-6, 1e-12 * sum_abs_operands) J m^-2` water-year-sample bound.
Producer-carried totals and residuals are evidence to check, never operands in
the independent reconstruction.

Integrated condition-sample sums are reported in `MJ m^-2`. Resolved-support
mean flux is the water-year sample sum divided by evaluated seconds.
Calendar-normalized flux is reported
separately and cannot substitute for the resolved-support mean.

## Prospective Literature Context And Decision Rule

`artifacts/literature-envelope.md` is binding. It freezes source-specific
qualitative context and numerical non-comparability before results:

- Marks et al. (1998) Figure 7 term-sign/order-of-magnitude context across three
  instrumented sites and seven periods; numerical comparison is
  `NOT_COMPARABLE` because sites, periods, condition-sample estimand, and the
  snow-ground boundary differ;
- a campaign-specific water-year condition-sample screen of `[-5, +5] W m^-2`
  for the implemented external subset at the three canonical forcing sites;
- Roth and Nolin (2017) annual forest context: longwave contributes `93%`,
  `92%`, and `47%` of average annual net energy balance at low, mid, and high
  forest sites,
  while turbulent fluxes are not significant monthly or annually outside the
  exposed high-open site; and
- Webster et al. (2016) event context: subcanopy net longwave can reach about
  `+40 W m^-2` for short spring intervals. This is not a seasonal target.

These contexts are heterogeneous, not universal truth. The package's own
prospective screen may classify the implemented subset, while literature
numeric mappings remain `NOT_COMPARABLE` and never `VALIDATED`.

The frozen carrier screen passes only if all of the following hold:

1. identity, tag, fingerprint, finite-value, and no-internal-ground-alias gates
   pass for every evaluated row;
2. every canonical site has a minimum of `10` screen-eligible water-year
   condition samples, and every
   observed water year has an explicit census disposition;
3. all three canonical-forcing site medians for the implemented external subset
   lie in the prospective `[-5, +5] W m^-2` band; and
4. Snowbird is reported as a development-only, non-decisive diagnostic.

Failure localizes terms and blocks persistent-shadow advancement. Passage
permits roadmap consideration only; it does not prove physical validity or
authorize persistence.

## No-Tuning And Result Custody

The cohort, hashes, selectors, windows, support rules, aggregation, literature
context, tolerances, and decision rule are frozen before model execution.
No post-result numeric or categorical change is allowed in this result
namespace. A deterministic package-tool defect may be corrected only by
rejecting affected generated results, recording the defect and unchanged
science operator, prospectively freezing a versioned protocol amendment, and
rerunning from a fresh target namespace.

## Real Consumer And Negative Proof

The exact release CLI must write schema-v5 JSONL rows. The independent parser
must consume the exported hourly arrays directly. Unit-test structs,
producer-only totals, old summaries, or manual transcription cannot close the
claim.

The execution must also prove WAT and HBP bytes equal an evaluation-disabled
same-binary control for every site. The runfile's `outputs.pass` HBP artifact is
asserted from the parsed runfile and retained bytes. No schema-v5 field may be
described as production publication.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes one
`comparator_suite_runner` for the exact four-site release build/replays and
heavy batch verification under the declared target namespace; two independent
read-only science/Rust reviewers; and two independent read-only terminal
verifiers. Expected outputs are compact findings, reproduced metrics, and
evidence paths. Delegates may not edit tracked files or expand authority. The
comparator may write only normal build/test and ignored package-target outputs.
The orchestrator alone edits tracked files and dispositions findings.

## Phase Plan

### Phase A: Scaffold And Result-Blind Freeze

Create and commit this package, prompt, reading map, fixture custody, operand
lineage, literature context, protocol JSON, analysis tool/tests, catalog and
roadmap state. Obtain result-blind protocol review and remediate before any
model run.

### Phase B: Exact Four-Site Execution

Build the frozen release CLI. Run evaluation-disabled controls and paired
evaluation lanes for all four sites, with the staged Snowbird derivative in
both. Retain commands, environment custody, hashes, and outputs.

### Phase C: Independent Analysis

Stream the exact traces, bind climate dates and observation windows,
reconstruct every term and residual, report water-year/site distributions and
coverage, apply the frozen literature classifications and carrier screen, and
write compact tracked evidence from ignored raw outputs.

### Phase D: Review And Closure

Complete dual independent review, disposition every finding, remediate accepted
findings, run direct gates, obtain two fresh terminal verifications, reconcile
the terminal diff, archive the prompt byte-identically, update roadmaps and
catalog, and commit locally.

## Validation And Exit Criteria

- Pre-result scaffold/freeze and review amendments are committed before the
  first model run; the exact independently reviewed clean SHA is supplied via
  `--expected-head`, required unchanged before build/analysis/acceptance, and
  recorded in the receipt.
- Result-blind protocol review passes before execution.
- Four control and four paired exact-current release CLI runs complete.
- WAT/HBP control equality passes at all four sites.
- Schema-v5 tags, pairing, fingerprints, support, and every exported operand
  pass independent checks.
- Hourly/daily/water-year-sample term identities close at frozen tolerances.
- Every eligible site-year and every zero-coverage or censored year is reported.
- Internal conduction is separate and absent snow-ground heat is explicit.
- Literature comparison follows the frozen source-specific context and
  no-tuning rule.
- Package-local syntax/tests, result verifier, JSON parsing, documentation
  lint, spelling review, format/diff hygiene, and relevant focused schema-v5
  tests pass.
- Dual review, finding disposition, dual verification, prompt archival,
  line-count governance, write-set reconciliation, and truthful disposition
  pass with no deferred current-scope gate.

No full-workspace correctness profile is required because the terminal
production/kernel diff must be empty. Any need for such an edit stops this
package and requires a separate contract-first package.

## Security And Data Impact

Security impact is `none expected`. Runs use local fixtures, observations,
binaries, and ignored target paths. The tool removes ambient `OPENWEPP_*`
variables, records only allowed key names/values, does not read credentials or
contact external services, and never mutates source fixtures.

## Progress

- [x] (2026-08-06) User authorized scaffolding and end-to-end execution.
- [x] (2026-08-06) Froze cohort, forcing custody, support, aggregation,
  literature context, decision rule, and no-tuning posture before results.
- [x] (2026-08-06) Committed the result-blind scaffold; both independent
  reviewers blocked execution and every finding was amended before results.
- [x] (2026-08-06) Fresh review of `54c8e00dc` found five residual custody,
  naming, and consumer defects; all were accepted and amended without creating
  the result namespace.
- [x] (2026-08-06) Both reviewers admitted exact commit `73ca62bd1`; all eight
  lanes ran, but analysis rejected v1 on a validator-only residual-tolerance
  bug before any site metric or screen was produced.
- [x] (2026-08-06) Both reviewers admitted v2 at exact commit `3ee1bac3e`.
- [x] (2026-08-06) Executed four controls and four paired lanes; exact WAT/HBP
  identity and retained verification pass.
- [x] (2026-08-06) Reconstructed `154` screen-eligible condition samples; all
  four site distributions are negative and the canonical screen fails `0/3`.
- [x] (2026-08-06) Independent science and custody reviews reproduced the
  result; all findings were corrected and fresh reviews pass.
- [x] (2026-08-06) Both independent terminal verifiers reproduced the retained
  result and passed the exact clean closure candidate with no findings.
- [x] (2026-08-06) Reconciled the 44-path terminal diff, archived both
  verifier PASS reports, renewed documentation/diff gates, and prepared the
  stable closure commit.

## Surprises & Discoveries

- The same-state operator emits explicit zero-coverage diagnostics for an empty
  post-CoE snow snapshot. Those rows cannot be treated as zero-energy snow
  hours; calendar coverage must remain separate from resolved support.
- The same-state operator intentionally marks internal active/lower conduction
  not applicable. That cleanly prevents accidental substitution for the absent
  snow-ground boundary.
- The applicable producer component residual is floating reconstruction
  evidence, not a same-state N/A field. Protocol v1 incorrectly required exact
  zero despite its already-frozen `1e-6 J m^-2` daily tolerance; the first
  `-2.79e-9 J m^-2` residual correctly stopped analysis before results.
- Every screen-eligible water-year sample at all four sites has a negative
  implemented external subset. Positive surface radiation is overwhelmed by
  the complete arm's sensible-plus-latent loss, with site-median turbulent
  terms from `-68.10` to `-145.11 W m^-2`.

## Decision Log

- Decision: use the governed Snowbird precipitation derivative for the sole
  Snowbird result lane and preserve canonical Snowbird only as provenance.
  Rationale: the campaign roadmap requires the derivative beginning with this
  result-bearing package; adding canonical Snowbird as a second result after
  inspection would create an unfrozen sensitivity.
  Date/Author: 2026-08-06 / Codex.
- Decision: compare resolved-snow and calendar-normalized means separately.
  Rationale: schema-v5 distinguishes physical evaluated support from explicit
  snow-free zero coverage.
  Date/Author: 2026-08-06 / Codex.
- Decision: use source-specific literature context plus a frozen near-balance
  screen, not a claimed universal flux range.
  Rationale: subcanopy flux partition is climate-, canopy-, and event-dependent.
  Date/Author: 2026-08-06 / Codex.
- Decision: block persistent-shadow advancement and place turbulent carrier
  lineage/operator reconciliation next.
  Rationale: all canonical site medians fail strongly negative, while the
  independently reinitialized predecessor carrier was strongly positive;
  terminal or
  receiving-surface work cannot reconcile that upstream contradiction.
  Date/Author: 2026-08-06 / Codex.

## Outcomes & Retrospective

The exact v2 release execution and independent retained reconstruction pass,
but the prospective carrier screen fails. Canonical implemented-external-subset
medians are Mica `-60.63`, Niwot `-123.32`, and Paradise `-54.30 W m^-2`;
non-decisive Snowbird is `-67.23 W m^-2`. All eligible water-year samples are
negative. The radiative surface arm remains positive, and turbulent terms
dominate the complete-arm-only negative delta. This is a bounded
same-state condition-sample finding, not a seasonal-energy or melt-efficacy
result. CoE remains authoritative; persistence and cutover stay blocked.
