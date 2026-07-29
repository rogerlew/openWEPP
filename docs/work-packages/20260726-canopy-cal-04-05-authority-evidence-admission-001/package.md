# CANOPY-CAL-04/05 Authority Evidence Admission

Package ID: `20260726-canopy-cal-04-05-authority-evidence-admission-001`

Status: `EXECUTED / HOLD — CAL-04 LIFTED; CAL-05 AUTHORITY-BLOCKED`

Date opened: `2026-07-26`

Execution mode: `package-end-to-end`

Package type: external scientific evidence acquisition, classification, and
prospective authority admission.

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current throughout execution.

## Purpose / Big Picture

Resolve, where defensible evidence exists, the authority gaps that currently
prevent CANOPY-CAL-04 timing calibration and CANOPY-CAL-05 litter-source and
decomposition adjudication. After this package, a fresh agent must be able to
determine from retained, checksum-bound research objects whether CAL-04,
CAL-05, both, or neither may proceed.

This package searches for and admits evidence. It does not fit parameters,
change process equations, amend production science contracts, or claim that a
plausible source is authoritative merely because it is convenient.

## Progress

- [x] (2026-07-26) Scaffolded the package, evidence criteria, operator-help
  protocol, write set, active prompt, and queued artifacts.
- [x] (2026-07-26) Read the complete required-reading set and recorded exact
  identities.
- [x] (2026-07-26) Inventoried existing local and resolvable external candidate
  sources.
- [x] (2026-07-26) Evaluated CAL-04 quantitative calibration and independent timing-holdout
  candidates.
- [x] (2026-07-26) Adjudicated the no-prior deterministic-search alternative
  prospectively.
- [x] (2026-07-26) Evaluated CAL-05 site-matched litter-source and forest-floor-stock
  candidates.
- [x] (2026-07-26) Requested operator assistance for every promising resource the agent
  cannot lawfully or technically access.
- [x] (2026-07-26) Retained admitted objects and completed provenance, license, transformation,
  uncertainty, matching, and immutable-role records.
- [x] (2026-07-26) Amended the frozen protocol and ledgers only for independently supported
  admissions.
- [x] (2026-07-26) Completed selected gates, two independent reviews, finding
  disposition, two independent terminal verifications, and final disposition.
- [x] (2026-07-28) Recorded the operator's report that the Forest Service has
  no additional tissue-separated data; retired `PENDING_OPERATOR` without
  treating the unavailable measurements as zero or lifting CAL-05 fitting
  authority.

## Surprises & Discoveries

- Hubbard Brook and Harvard Forest expose strong, open phenology series with
  distinct sites and source-defined endpoints, so CAL-04 does not need invented
  dates or a same-site year split.
- Open CAL-05 objects contain useful site-matched litter and stock evidence but
  pool fine wood with bark and reproductive material. The public Hubbard Brook
  table also omits the complete tissue-mass partition described by its methods.
- The local `wctl doc-lint` and `wctl doc-mv` wrappers are configured against
  a different project root: lint scanned zero staged openWEPP files and move
  rejected both relative and absolute existing paths. Direct `markdown-doc`
  lint and an explicit patch move completed the operations.

## Decision Log

- Decision: Keep evidence admission separate from CAL-04/CAL-05 fitting.
  Rationale: objectives, partitions, bounds, and tolerances must be frozen
  before affected model results are viewed.
  Date/Author: 2026-07-26 / Codex.
- Decision: Treat inaccessible promising resources as operator-assistance
  events, not silent exclusions.
  Rationale: paywalls, authentication, institutional repositories, physical
  archives, redistribution questions, and human-only data requests may be
  resolvable by the operator and must not be mistaken for absent evidence.
  Date/Author: 2026-07-26 / Codex.
- Decision: Permit a prospectively reviewed no-probability-prior design as the
  preferred resolution of the CAL-04 prior gap.
  Rationale: deterministic grids, profiles, or bounded ensembles can assess
  identifiability without inventing a scientific probability distribution.
  Date/Author: 2026-07-26 / Codex.
- Decision: Admit Hubbard Brook phenology for calibration and Harvard Forest
  HF003 only for independent holdout.
  Rationale: the sites and research objects are independent, while keeping
  their source-specific transition definitions separate avoids false endpoint
  equivalence.
  Date/Author: 2026-07-26 / Codex.
- Decision: Keep CAL-05 on hold and request operator assistance.
  Rationale: no retained open object separately weighs fine wood, bark, and
  reproductive material on a basis comparable with the forest-floor stock.
  Date/Author: 2026-07-26 / Codex.
- Decision: Close the Forest Service assistance route as
  `AUTHORITY_BLOCKED / NO_ADDITIONAL_DATA` and hand CAL-05 to an ADR-0042
  calibration-readiness package.
  Rationale: the operator reported that the Forest Service has no additional
  data. Data scarcity limits empirical source and decay claims but does not
  prevent deterministic machinery, sensitivity, synthetic-recovery,
  equifinality, and additional-data analysis.
  Date/Author: 2026-07-28 / Codex.

## Outcomes & Retrospective

CAL-04's timing and probability-prior blockers are lifted prospectively.
Hubbard Brook supplies the calibration observations, Harvard Forest supplies
the independent holdout, and CAL-04 must use a deterministic prior-free
identifiability design.

CAL-05 is partially lifted for source-sufficiency analysis and remains
`AUTHORITY_BLOCKED` for decomposition fitting. The Forest Service has no
additional data. The unresolved resource remains a lawful tissue-separated
annual dry-mass table distinguishing
leaves, needles, fine wood under 2 cm, bark, and reproductive material with
site/plot/year identifiers and reuse terms. Under ADR-0042, scarcity of that
resource does not stop a separate calibration-readiness package, but it
continues to prohibit an empirical source-adequacy or fitted-decay claim.

## Context and Orientation

CANOPY-CAL-03 installed the canonical corpus at
`tests/fixtures/cancov_forest/observations/canopy_phenology/records.csv`, its
immutable role ledger, and the frozen protocol at
`docs/work-packages/20260726-canopy-cal-03-observation-native-research-001/artifacts/pre-calibration-protocol.md`.
That package recorded three gaps:

1. no independent quantitative phenology-timing holdout;
2. no evidence-supported probability priors for native canopy operands; and
3. no site-matched measured leaf, needle, and fine-woody litter-source
   composition.

CAL-04 needs quantitative timing authority suitable for a prespecified
calibration partition and an independent holdout. Existing qualitative
calendar wording and PAR proxies remain useful screens but cannot be converted
into invented dates or scalar losses.

CAL-05 needs measured annual litter-source components matched closely enough
to a forest-floor stock observation to separate source insufficiency from
decomposition. Aggregate litter, standing foliage, model output, and Bill
Elliot's fitted operands cannot replace that evidence.

An authority admission is prospective when the source identity, transformations,
roles, matching rules, objective use, uncertainty treatment, and exclusions
are frozen and independently reviewed before any affected fit is run.

## Authority Questions

For CAL-04, determine whether retained research objects provide quantitative
leaf-on, peak-canopy, senescence, or leaf-off timing, or a quantitative time
series with a source-defined transition method. The evidence must identify
site, forest class, observation period, temporal and spatial support,
measurement method, missing-data semantics, and uncertainty. The admitted set
must support a calibration partition and a genuinely withheld partition.
Different sites are preferred; disjoint years are acceptable only when their
independence and selection rule are documented before fitting.

For the CAL-04 prior gap, first test whether the campaign can formally select a
deterministic grid, profile-likelihood, or bounded-ensemble design with no
probability prior. Contract and typed-schema domains may bound execution, but
must be labeled as domains rather than scientific priors. If probability
priors are proposed instead, each distribution and parameter must have direct
independent evidence.

For CAL-05, determine whether retained research objects jointly provide
annual fluxes separated into deciduous leaves, evergreen needles, fine woody
material, other material, and total litterfall, plus a comparable forest-floor
stock. Record whether source and stock observations match in site or stand,
forest class, period, material boundary, dry-mass or loss-on-ignition basis,
area normalization, and uncertainty. A partial composition may remain useful,
but cannot authorize an unmeasured component to be zero.

## Included Scope

The executor may search the repository, already mounted local resources,
primary scientific repositories, government archives, dataset catalogs, and
publisher metadata. It may download openly accessible research objects and
metadata needed to establish provenance, subject to their stated terms. It may
prepare an operator request for resources requiring login, purchase,
institutional access, author contact, physical retrieval, permission
confirmation, or a file known to exist outside the repository.

The executor may add source objects under
`references/canopy_phenology/authority_admission/`, extend the CAL-03 canopy
observation corpus and ledgers, and amend the frozen protocol prospectively.
Every admitted value must retain an exact source-object and transformation
path.

## Excluded Scope

Do not run parameter fitting, select a preferred parameter vector, tune bounds
after model inspection, alter canopy/litter/decomposition physics, modify
science contracts, or use snow, hydrology, erosion, legacy parity, ordering,
or visual agreement as fitting authority. Do not bypass access controls,
misrepresent license terms, infer absent measurements as zero, digitize a plot
without recording the method and uncertainty, or contact third parties without
operator authorization.

## Operator-Assistance Protocol

When a promising resource is blocked by access, the executor must promptly ask
the operator for assistance. The request must name the exact resource,
publisher or custodian, persistent identifier or URL, expected variables,
relevant site and period, why it could lift a named blocker, the access problem,
and the minimum useful operator action. Examples include supplying a lawful
copy, confirming a mounted archive location, using institutional access,
clarifying redistribution permission, or authorizing a data-custodian contact.

Before asking, exhaust safe metadata and local-archive checks. Do not repeatedly
retry a paywall, authentication challenge, or forbidden endpoint. Record every
request and response in `artifacts/operator-assistance-log.md`. While waiting,
continue independent candidate work when possible. A requested resource is
`PENDING_OPERATOR`, not missing and not admitted.

If the operator supplies a file, preserve its original bytes, compute its
checksum, record how it was obtained, and assess its terms before extracting
values. If the operator cannot obtain it, retain the candidate and access
failure as contrary search evidence.

## Declared Write Set

- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/20260726-canopy-cal-04-05-authority-evidence-admission-001/**`
- `docs/work-packages/20260726-canopy-cal-03-observation-native-research-001/artifacts/pre-calibration-protocol.md`
- `docs/work-packages/20260726-canopy-cal-03-observation-native-research-001/artifacts/calibration-holdout-ledger.csv`
- `docs/work-packages/20260726-canopy-cal-03-observation-native-research-001/artifacts/observation-inventory.csv`
- `tests/fixtures/cancov_forest/observations/canopy_phenology/**`
- `references/canopy_phenology/authority_admission/**`

All Rust production code, science contracts, model parameters, native
managements, runfiles, soils, climates, slopes, and CAL-03 execution outputs
are read-only.

## Required Deliverables

- `artifacts/required-reading-map.md`
- `artifacts/intent-plan.md`
- `artifacts/authority-gap-ledger.csv`
- `artifacts/search-protocol.md`
- `artifacts/search-log.csv`
- `artifacts/candidate-source-register.csv`
- `artifacts/operator-assistance-log.md`
- `artifacts/admission-ledger.csv`
- `artifacts/exclusion-ledger.csv`
- `artifacts/cal04-partition-and-objective.md`
- `artifacts/cal04-prior-disposition.md`
- `artifacts/cal05-source-stock-matching.md`
- `artifacts/source-object-manifest.csv`
- `artifacts/gate-evidence.md`
- two independent review artifacts
- `artifacts/finding-disposition.md`
- two independent verification artifacts
- `artifacts/final-disposition.md`

## Plan of Work

First, freeze search concepts, candidate inclusion criteria, search venues,
query families, stopping rules, duplicate handling, and the model-blind
admission procedure. Inventory local sources before external searching.

Second, search CAL-04 timing evidence and CAL-05 source/stock evidence as
separate tracks. Retain metadata for excluded and inaccessible promising
candidates as well as admitted objects. Invoke the operator-assistance protocol
as soon as an access block is confirmed.

Third, independently reconstruct every extracted value from the original
object. Record units, transformations, uncertainty, missing semantics, and
applicability. Freeze calibration and holdout membership without looking at
affected fit results.

Fourth, prospectively amend the CAL-03 protocol and ledgers only when the
admission criteria pass. A failed search leaves the original gap unchanged.
Resolve the prior question explicitly as either evidence-supported probability
priors or a reviewed prior-free deterministic design.

Finally, reconcile the terminal diff and obtain two independent scientific
authority reviews followed by two independent terminal verifications.

## Validation and Acceptance

The CAL-04 timing blocker is lifted only if quantitative timing observations
support a declared calibration objective and a separate independent holdout,
with exact provenance, uncertainty treatment, frozen roles, and no conversion
of qualitative proxies into invented scalars.

The CAL-04 prior blocker is lifted only if independently supported priors are
admitted or the protocol prospectively adopts a prior-free deterministic
identifiability design. Search distributions must not be mislabeled as
scientific probability.

The CAL-05 blocker is lifted only if admitted evidence can compare separated
litter-source fluxes and forest-floor stock on compatible bases. Any unmeasured
source remains unknown rather than zero, and decay fitting remains prohibited
until source sufficiency is adjudicable.

Each blocker receives one terminal state: `LIFTED`, `PARTIALLY_LIFTED`,
`PENDING_OPERATOR`, or `AUTHORITY_BLOCKED`. `PARTIALLY_LIFTED` and
`PENDING_OPERATOR` do not authorize affected fitting. The package may close
`COMPLETE` only when both blockers are lifted. Otherwise it closes `HOLD` with
the exact unresolved resource, attempted routes, operator requests, and next
action recorded.

Required gates are source-object checksum verification, metadata completeness,
license/terms disposition, exact record-to-source joins, immutable one-role
assignment, calibration/holdout disjointness, unit and transformation
reconstruction, site/material matching, missing-value checks, no-model-result
contamination, documentation lint, diff hygiene, write-set reconciliation,
dual independent review, finding disposition, and dual terminal verification.

## Review and Delegation Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two scientific authority reviewers and two terminal
verifiers for independent source, role, partition, and closure assessment;
expected outputs are two review artifacts and two verification artifacts;
write access is bounded to their assigned package-local artifact.

Reviewers must independently inspect the original admitted sources, not only
the executor's extracted tables. At least one review must challenge CAL-04
partition independence and at least one must challenge CAL-05 material and
stock/flux comparability. Verifiers must confirm that no fitting or physics
change entered the terminal diff.

## Concrete Steps

Work from `/home/workdir/openWEPP`. Begin with:

    tools/agents/find-agents --for \
      docs/work-packages/20260726-canopy-cal-04-05-authority-evidence-admission-001/package.md \
      tests/fixtures/cancov_forest/observations/canopy_phenology/records.csv \
      references/canopy_phenology/authority_admission

Use `rg` and `rg --files` for local discovery. Use primary-source repositories
and authoritative dataset catalogs for external discovery. Record every query
and candidate as execution proceeds rather than reconstructing the search
history afterward.

Before disposition, run:

    sha256sum -c <package source checksum manifest>
    wctl doc-lint
    git diff --check

Also run package-local validators created during execution for exact joins,
role uniqueness, partition disjointness, metadata completeness, and unit
reconstruction. Record commands and results in `artifacts/gate-evidence.md`.

## Security and External-Action Gate

Never commit credentials, session cookies, access tokens, private URLs, or
license-restricted source bytes. External browsing and lawful downloads are
authorized for evidence discovery. Purchase, account creation, bypassing
access controls, contacting researchers or custodians, publishing source
material, or accepting new legal terms requires explicit operator action or
authorization.
