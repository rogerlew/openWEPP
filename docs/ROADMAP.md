# openWEPP Engine Roadmap

Status: living — canonical prospective planning queue

Last updated: 2026-07-15

This file answers one question: **what work is openWEPP intentionally doing
next, later, or at a named future gate?** It does not record how the current
state was reached. Execution history lives in the
[work-package catalog](work-packages/README.md); unprioritized ideas live in the
[backlog tracker](backlog/TRACKER.md).

## Operating Contract

1. Every item here is prospective and has a state, an owning queue, and an
   advancement trigger or dependency.
2. Allowed states are `next`, `active`, `queued`, `held`, `recurring`, and
   `deferred`.
3. When an item closes, its closing package removes it from this file and
   records the outcome in the work-package catalog. Do not retain a completion
   summary here.
4. A deferred item stays here only when it has a durable authority/backlog
   pointer and a named promotion trigger. Other concepts remain in the backlog.
5. Detailed requirements belong in the active work package and canonical
   governance, architecture, or science contract. The roadmap keeps only the
   constraints needed to select and order work.
6. A strategic redirect changes both this file and the active package handoff;
   an ADR, backlog note, or agent memory alone does not reorder execution.

## Current Priority

| Priority | Owning queue | State | Advancement gate |
| --- | --- | --- | --- |
| `ASSURE-04A` v2 source and identity foundation | Scientific assurance | `next` | Scaffold on operator instruction; preserve the zero-report public state and implement only the bounded source/identity foundation. |

## Scientific Assurance Queue

### Direction

The assurance product is scientific communication for hydrologists, soil
scientists, researchers, and practitioners. A public evaluation must let them
understand what was assessed, why the formulation is credible, how it was
evaluated, what the quantitative results and limitations show, how the work can
be reproduced or challenged, and which application judgment remains theirs.
Scientific findings lead; internal lifecycle labels, hashes, test counts, and
agent records do not.

The v1 SNOTEL candidate is retired and cannot be restored, promoted, cited as
an openWEPP scientific assessment, or vendored. Its exact bytes remain
recoverable for audit from the ASSURE-03 frozen Git/hash record. Retirement
concerns publication architecture, not the underlying snow/frost equations,
contracts, datasets, campaigns, results, or
[scientific narrative](../usersum/snow-frost-modeling-and-validation.md).
Snow/frost remains the flagship v2 synthesis after a smaller non-snow pilot
proves the architecture.

The v2 records remain distinct:

- a conventional public scientific model-evaluation report;
- a subordinate public technical assurance supplement;
- a version-bound public research-object surface for safe claim-bearing data,
  procedures, and reproduction material;
- an internal machine assurance bundle for traceability and reproducibility;
- canonical model-science narratives and science contracts; and
- an application assessment owned by the named user or institution.

### Binding V2 Rules

- Prototype a real, hand-authored scientific manuscript before designing the
  schema or renderer, and freeze the documentation architecture before code.
- Drafts and candidates build only to staging. Only independently reviewed and
  approved reports enter public `usersum` or a release snapshot.
- Conclusions are claim-specific: process, quantity, scale, tested domain,
  software realization, evidence, uncertainty, and limitations remain visible.
- Verification, empirical evidence, comparative evidence, current-release
  transfer, and application fitness remain separate. A verification gap blocks
  only conclusions for which it is materially required.
- Reports include inspectable quantitative results, the smallest sufficient
  tables/figures, contrary and negative evidence, and meaningful limitations.
- Claim-bearing values, units, tables, figures, captions, sample counts, and
  uncertainty summaries resolve mechanically from retained, stable identities.
- The build checks structure, dependencies, identity, review locks, staging,
  drift, and snapshots. It never chooses methods, interprets results,
  adjudicates science, invokes an agent, or issues a fitness verdict.
- Optional agent assistance uses a versioned procedure, content-identified
  inputs, retained output, disclosed tooling, and independent review; it stays
  subordinate to the scientific argument.
- Accessibility, reproduction, scientific review, publication review,
  supersession, and exact release transfer are publication requirements.
- Report leads and data/method stewards own public research objects;
  independent reproduction/publication review and the build gate prove every
  required safe object is available.

### Ordered Work

| ID | Prospective outcome | Advancement gate | State |
| --- | --- | --- | --- |
| `ASSURE-04A` | Add the smallest v2 manuscript/supplement, identity, claim, result, figure, reference, review, and publication source contracts demonstrated by the groundwater prototype. | Real CLI admits every field; unknown/missing/unused identity, units, path, restriction, and version guards pass; tracked public catalog stays at zero reports. | `next`; scaffold from the implementation roadmap. |
| `ASSURE-04B` | Implement deterministic transitive dependency planning for one report and all reports. | Cycle, missing/unused edge, transitive impact, stable order, one/all equivalence, and real CLI consumer gates pass. | `queued`; blocked by `ASSURE-04A`. |
| `ASSURE-04C` | Assemble human-authored manuscripts and supplements to staging while resolving retained values, tables, figures, citations, and portable links. | Deterministic semantic output, stale/mismatched value, accessibility, sandbox, and real rendered-link gates pass; no public promotion. | `queued`; blocked by `ASSURE-04B`. |
| `ASSURE-04D` | Add review locks, approved-only promotion, public catalog integration, drift checks, and immutable release snapshots. | Draft/review negative publication, changed-root invalidation, named approval, exact transfer, confined fixture, and tracked-zero-catalog gates pass. | `queued`; blocked by `ASSURE-04C`. |
| `ASSURE-05` | Convert the accepted bounded non-snow prototype into the first complete v2 report using the production kernel, real consumer, retained evidence, figures/tables, and technical supplement. | Domain-reader outcome met; every quantitative statement resolves to evidence; limitations and contrary results remain visible; independent scientific/publication review and reproduction pass before public inclusion. | `queued`; blocked by `ASSURE-04D`. |
| `ASSURE-06` | Author the snow/frost flagship synthesis from the existing precipitation-phase, SWE/depth/density, frost-depth/soil-temperature, conservation, negative-mechanism, and production evidence; split reports where claims or scales materially differ. | Complete quantitative methods/results, datasets, sample counts, figures, forcing interpretation, limitations, prior-knowledge comparison, claim-specific conclusions, release transfer, and independent review. | `queued`; blocked by a successful `ASSURE-05` pilot. |
| `ASSURE-07` | Build a priority portfolio of additional hydrology, erosion, sediment, plant, channel, and watershed reports and bind reviewed reports plus explicit gaps to release snapshots. | Selection by user importance and evidence readiness; ownership, currency, transfer, supersession, snapshots, and documented gaps remain explicit without implying site fitness. | `queued` after `ASSURE-05`; may proceed incrementally without blocking `ASSURE-06`. |
| `ASSURE-08` | Vendor the approved v2 report set into WEPPcloud during the openWEPP beta release campaign. | Refresh the handoff against current wepppy contracts; prove vendor sync, manifest/navigation/roles, rendering, link rewriting, accessibility, search, and the real downstream consumer; bind vendored content to the beta candidate. | `deferred`; mandatory immediately before openWEPP beta release in WEPPcloud. Do not start while WEPPcloud or the reports remain immature. |

Current cross-repository boundary:
[wepppy handoff](work-packages/20260714-assurance-dossier-lifecycle-foundation-001/artifacts/wepppy-handoff.md).
It is a dormant compatibility record, not authority to vendor.

## Recurring Maintenance

| ID | Owner and purpose | Trigger | State |
| --- | --- | --- | --- |
| `CQR-NIGHTLY` | Operator-owned repo-wide complexity/coverage maintenance under the [rolling ExecPlan](work-packages/cqr-nightly-burndown-execplan.md). Each selected module receives a behavior-preserving package, current measurement, gates, review, verification, and disposition. | An operator requests a bounded batch after active feature/science packages release overlapping write sets. | `recurring`; not an automatic blocker for the scientific-assurance queue. |

## Promoted Work Awaiting Scheduling

| ID | Owning queue and outcome | Advancement trigger | State |
| --- | --- | --- | --- |
| `CANOPY-PHENOLOGY` | Plant/snow-frost science: replace fixed-date deciduous/mixed-canopy behavior with hemisphere-robust leaf-off and leaf-on, then re-anchor the retained litter-drop window to the physical phenology signal. | An operator schedules the next increment by changing the current priority and authorizing a contract-first package. Before physics code, reconcile the landed native-management/YAML surface with the original first-class forest-input foundation and ratify the growth-canopy contract; then sequence leaf-off and leaf-on as separate closable increments. | `queued`; promoted, but not active while `ASSURE-04A` is the current priority. [Program record](backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md). |

## Promotion And History Routing

- Unprioritized or partially staged ideas remain in the
  [backlog tracker](backlog/TRACKER.md). Promotion requires a roadmap row with an
  owning queue and trigger plus an authorized work package.
- Package status, evidence, commits, holds, and outcomes belong in the
  [work-package catalog](work-packages/README.md).
- Architectural rationale belongs in the [decision index](decisions/README.md).
- Kernel/process authority belongs in the
  [science-contract index](specifications/science-contracts/index.md); directory
  policy lives in its [README](specifications/science-contracts/README.md).

## Governing Authority

- [V&V strategy](governance/openwepp-verification-validation-strategy.md)
- [Scientific assurance v2 architecture](governance/scientific-assurance-v2-architecture.md)
- [Scientific model-evaluation report standard](standards/scientific-model-evaluation-report.md)
- [Assurance report lifecycle contract](governance/scientific-assurance-dossier-lifecycle.md)
- [Assurance source/build contract](governance/scientific-assurance-v2-source-build-contract.md)
- [V2 migration plan](planning/scientific-assurance-v2-migration-plan.md)
- [V2 implementation roadmap](planning/scientific-assurance-v2-implementation-roadmap.md)
- [ADR-0038: manuscript-first publication](decisions/0038-manuscript-first-scientific-assurance-publication.md)
- [ADR-0011: architecture-first, top-down science contracts](decisions/0011-architecture-first-top-down-science-contracts.md)
- [ADR-0017: comparator is a flag, not a target](decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md)
- [ADR-0018: defect-closure ExecPlan conversion](decisions/0018-defect-closure-execplans-conversion-rule.md)
- [Defect-Closure ExecPlan authoring](defect_closure_execplans.md)
