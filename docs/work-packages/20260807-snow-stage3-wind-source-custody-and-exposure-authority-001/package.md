# Stage 3 Wind Source Custody And Exposure Authority

Status: `queued / scaffolded / authority reconciliation`

Date: `2026-08-07`

Package ID:
`20260807-snow-stage3-wind-source-custody-and-exposure-authority-001`

Plan class: `Critical science-authority and forcing-custody reconciliation`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
Keep Progress, Surprises & Discoveries, Decision Log, and Outcomes &
Retrospective current throughout execution.

## Purpose / Big Picture

Determine what wind the Stage 3 snow carrier actually consumes and whether
that forcing is physically applicable to the modeled forest snow surface. The
completed carrier-plausibility package proved that raw-versus-bounded vapor is
not the cause of the large latent response. This package must therefore close
the exact GRIDMET-to-CLI-to-Stage-3 custody chain, correct authority that treats
nominal 10 m wind as a 5 m measurement, and adjudicate exposure
representativeness without fitting an attenuation factor.

The observable outcome is an authority-backed disposition for each canonical
site: applicable forcing, inapplicable forcing, or authority missing. A valid
height conversion alone is not exposure authority. This package does not add a
canopy operator, tune wind, change production physics, persist shadow state,
promote Stage 3, retire CoE, or cut over outputs.

## Progress

- [x] (2026-08-07) Scaffolded from the completed plausibility handoff and
  quantified neutral-height sanity check.
- [ ] Freeze exact source, custody, transformation, height, and exposure
  questions before reading any new result-bearing comparison.
- [ ] Reconcile canonical contract authority and contract-derived tests before
  any permitted implementation edit.
- [ ] Execute source-custody and exposure-applicability evidence collection.
- [ ] Complete dual review, finding disposition, dual verification, exact diff,
  assurance impact, and final disposition.

## Surprises & Discoveries

- Observation: Stage 3 receives raw `forcing.vwind_m_s`; the FAO-56 10 m-to-2 m
  conversion is local to Penman-Monteith and never reaches Stage 3.
  Evidence: `00a_snow_frost_authority_impl.rs` passes the raw field, while
  `00d_authority_runtime_impl.rs` creates PMET-local `fwv_m_s`.
- Observation: the direct nominal-height mismatch is bounded and smaller than
  the remaining magnitude concern.
  Evidence: neutral `d ~= 0.0245 m`, `z0 = 0.005 m` gives log factors 7.598 at
  10 m and 6.903 at 5 m: approximately 10% in friction velocity and 21% in the
  momentum/heat product, versus a three- to fivefold plausibility concern.

## Decision Log

- Decision: treat the neutral 21% estimate as a prospectively checked sanity
  bound, not correction authority or an acceptance target.
  Rationale: it quantifies direction and scale without proving stability,
  canopy exposure, or applicability of a 4 km gridded wind to forest snow.
  Date/Author: 2026-08-07 / operator + agent.
- Decision: keep canopy-aerodynamic licensing outside this package.
  Rationale: exposure applicability must be resolved first; absence of canopy
  authority does not prove a canopy operator is required.
  Date/Author: 2026-08-07 / operator + agent.
- Decision: Paradise WY2015 is not a gate for this package.
  Rationale: it is separately queued as a non-blocking support-censoring
  resolution and may not carry a physical pass/fail claim here.
  Date/Author: 2026-08-07 / operator + agent.

## Outcomes & Retrospective

Queued. Completion must state whether the forcing is applicable at each site,
what authority remains missing, and whether the bounded height mismatch changes
the prior plausibility disposition. `AUTHORITY_MISSING` is a valid scientific
outcome; a fitted value is not.

## Context And Orientation

GRIDMET `vs` is nominal 10 m wind. The retained climate path places a daily
wind value in CLI `w-vl`; the direct runtime publishes that value as
`vwind_m_s`. Stage 3 uses it directly with virtual Monin-Obukhov geometry
currently described as 5 m above the modeled snow surface. Penman-Monteith
separately converts the same nominal 10 m wind to 2 m, but that local value is
not shared with snow.

The previous package's three remaining blocker families are wind exposure,
physical turbulent-magnitude envelopes, and unresolved canopy applicability.
This package owns the first and the nominal-height component of the second. It
must not infer forest exposure from the numeric wind series, use the model
residual as a calibration objective, or choose attenuation that makes energy
balance appear plausible.

## Scope

Included:

- exact GRIDMET product/version, variable, grid cell, spatial support, native
  cadence, and nominal height for every retained site;
- exact transformation and aggregation into CLI `w-vl`, with code/config/data
  provenance and units;
- exact CLI-to-runtime-to-Stage-3 consumer chain and proof that PMET-local
  `fwv_m_s` is not the snow input;
- contract correction separating forcing measurement/reference height from
  virtual model geometry when current authority conflates them;
- independent neutral and, only if canonically authorized, stability-aware
  height calculations using declared `d`, `z0`, and units;
- authoritative evidence for open, above-canopy, within-canopy, sub-canopy, or
  unresolved exposure at each site;
- per-site applicability disposition and claim limits;
- DRAFT assurance source impact if canonical identified sources change.

Excluded:

- fitted wind multipliers, residual-minimizing attenuation, inferred exposure
  from values, or site tuning;
- canopy interception, canopy heat storage, canopy radiation, or a new canopy
  aerodynamic operator;
- production wind correction or Stage 3 physics change;
- new result-bearing carrier execution unless prospectively added by reviewed
  package amendment after authority closure;
- Paradise WY2015 support localization, which roadmap row 31 owns;
- persistence, promotion, CoE retirement, defaults, public outputs, and cutover.

## Deliverables

1. `artifacts/source-custody-ledger.md` with per-site product-to-CLI identities.
2. `artifacts/runtime-consumer-proof.md` proving the real Stage 3 and PMET
   consumers and their distinct wind values.
3. `artifacts/height-geometry-reconciliation.md` independently reproducing the
   neutral bound and separating native height, adjusted height, and virtual
   geometry.
4. `artifacts/exposure-applicability-matrix.md` with authoritative per-site
   disposition and explicit missing evidence.
5. Contract amendments/tests/gate evidence when canonical authority changes.
6. Security/data impact, calibration-readiness, assurance impact, reviews,
   verification, exact diff, line counts, disposition, and handoff.

## Dependencies And Authority

Core package authority is the predecessor disposition and wind-custody handoff,
`SC-SNOWENERGY-001`, `SC-SNOWFREEZE-001`, GRIDMET/NLDAS source documentation,
the actual climate-generation lineage, and the real runner consumer path.
External documentation must be retained by stable identifier, version/date,
quoted field meaning within copyright limits, and access date. Secondary
documentation may corroborate but not replace primary dataset authority.

The Google Earth Engine catalog and NLDAS-2 documentation support nominal 10 m
wind and 2 m temperature. Abatzoglou (2013), DOI `10.1002/joc.3413`, supports a
10 m gridded wind field and logarithmic station-height adjustment. These facts
do not establish forest exposure or site representativeness.

## Intended Write Set

- this package tree;
- `docs/ROADMAP.md`, the snow campaign roadmap, and work-package catalog;
- `SC-SNOWENERGY-001`, `SC-SNOWFREEZE-001`, and science-contract index only if
  required by the authority reconciliation;
- existing contract-derived tests selected prospectively after instruction
  discovery;
- conditional DRAFT assurance locks/transactions and review-draft projections
  selected by the typed assurance tool.

Production Rust, climate fixtures, observations, manifests, schemas, public
outputs, defaults, WAT/HBP/PASS, and CoE ownership are not selected. Any need
for them requires a prospective package amendment and independent review.

## Plan Of Work

### Phase A: Result-Blind Authority Freeze

Freeze the per-site identity questions, accepted primary sources, transformation
ledger fields, height variables, exposure categories, decision predicates, and
`AUTHORITY_MISSING` behavior. Record exact retained climate/CLI hashes before
any derived comparison. Independent science and custody reviews must admit the
freeze before Phase B.

### Phase B: Contract And Test Reconciliation

If current contracts call GRIDMET wind a 5 m measurement, amend them first to
name nominal 10 m forcing separately from virtual 5 m transfer geometry. Add
literal contract-derived tests that distinguish raw 10 m, PMET-adjusted 2 m,
virtual 5 m, displacement, roughness, and rejected alias paths. Pass the clean
pre-implementation contract gate before any further edit.

### Phase C: Custody And Exposure Evidence

Trace actual per-site generation inputs through GRIDMET product/cell/cadence and
aggregation into CLI, then through the real runtime consumer. Reconstruct the
neutral bound independently. Admit exposure only from authoritative metadata or
site documentation; never from values or model fit. Emit per-site
`APPLICABLE`, `INAPPLICABLE`, or `AUTHORITY_MISSING` with exact claim limits.

### Phase D: Disposition And Closure

Reconcile the exact diff, run directly selected gates, update DRAFT assurance
sources without lifecycle advancement, complete dual reviews and dual terminal
verification, update roadmaps/catalog, archive the kickoff byte-identically,
and issue a final disposition. If exposure is missing, close this package as a
truthful authority hold with the exact missing source named; do not invent it.

## Validation And Acceptance

Before edits, declare `documentation/authority with contract-test impact
possible`; reconcile the exact terminal diff before final gate selection. At
minimum run exact Markdown/link validation, contract binding exposure for any
changed contract, affected contract-derived tests, formatting/Clippy for any
changed Rust test, assurance validation/render checks when selected, and
`git diff --check`. Escalate to workspace quick/full only when required by the
exact diff and canonical testing strategy; comparator-runner delegation is
required for heavy gates.

Acceptance requires all of the following:

- every retained site has exact source-to-consumer custody or an explicit,
  evidence-backed missing-authority row;
- measurement/reference height, transformed height, and model geometry are not
  aliased;
- the neutral estimate is independently reconstructed and is not used as a
  fitted correction;
- exposure applicability rests on authority, not inference from values;
- no canopy operator or production correction is smuggled into the diff;
- all current-scope gates pass and every review finding is dispositioned;
- dual terminal verification confirms gate legitimacy and protected boundaries.

## Security And Data Impact

This is local flat-file authority work. Do not commit credentials, restricted
source corpora, large downloaded grids, or generated climate products. Record
stable metadata and hashes; retain large/controlled evidence only through an
approved repository route. No external system mutation is authorized.

## Line-Count Governance

Record before/after line counts for every touched `.rs` file. `2000+` is WARN
and requires decomposition rationale; `3000+` nonexempt is a closure blocker.
Generated/fixture exceptions require owner and sunset metadata.

## Review And Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only science/Rust reviewers, two
independent read-only terminal verifiers, and the `comparator_suite_runner` for
selected heavy gates. Expected outputs are compact findings, reproduced
calculations/metrics, and log paths. Write access is read-only; the primary
agent owns package edits and finding disposition.

Reviews must check source authority, height algebra, exposure claim limits,
anti-fitting protections, contract/test alignment, gate legitimacy, exact diff,
line-count governance, and protected boundaries. Closure is blocked while any
finding is undispositioned.

## Idempotence And Recovery

Source inspection and calculations are repeatable. Never overwrite retained
evidence or mutate climate inputs. If a source identity or exposure fact cannot
be established, record `AUTHORITY_MISSING`; do not retry with a fitted proxy.
Contract changes precede tests and any implementation. Keep commits stable and
do not create or switch branches.

## Revision Note

2026-08-07: initial scaffold created from the completed evolving-carrier
plausibility disposition and operator-supplied independent sanity check.
