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
| `ASSURE-02` manuscript-first documentation architecture | Scientific assurance | `next` / documentation-only | User or named scientific-steward acceptance after the required documentation, real-evidence manuscript prototype, and two independent reviews agree. No v2 assurance code is authorized before acceptance. |

## Scientific Assurance Queue

### Direction

The assurance product is scientific communication for hydrologists, soil
scientists, researchers, and practitioners. A public evaluation must let them
understand what was assessed, why the formulation is credible, how it was
evaluated, what the quantitative results and limitations show, how the work can
be reproduced or challenged, and which application judgment remains theirs.
Scientific findings lead; internal lifecycle labels, hashes, test counts, and
agent records do not.

The v1 SNOTEL candidate is not an accepted scientific assessment of openWEPP
snow/frost and must not be promoted, cited as such, or vendored. That concerns
the publication architecture, not the underlying snow/frost equations,
contracts, datasets, campaigns, results, or
[scientific narrative](../usersum/snow-frost-modeling-and-validation.md).
Snow/frost remains the flagship v2 synthesis after a smaller non-snow pilot
proves the architecture.

The v2 records remain distinct:

- a conventional public scientific model-evaluation report;
- a subordinate public technical assurance supplement;
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

### `ASSURE-02` Closure Contract

`ASSURE-02` is documentation-only. It must produce one coherent reviewed
architecture containing:

- an architecture decision and revised V&V strategy defining the manuscript-
  first public product and the report/supplement/machine/application boundary;
- a v2 scientific-report standard and lifecycle, ownership, review,
  source/generated, staging/publication, versioning, supersession, and release-
  transfer contract;
- a source/build/dependency contract for manuscripts, citations, claims,
  results, figures, evidence, methods, reproduction, review, and snapshots;
- an evidence-led pilot inventory and selection record for one bounded,
  scientifically credible, low-confounding, production-relevant non-snow
  kernel with a defensible referent and retained or reproducible evidence;
- a hand-authored, nonpublic manuscript prototype populated with real evidence,
  quantitative results, figures/tables, limitations, and discussion;
- a v1 retirement/migration plan that preserves evidence provenance while
  removing its public candidate and supporting zero published reports; and
- independently closable implementation packages with explicit consumers,
  review gates, tests, migration boundaries, and rollback behavior.

The linear groundwater-reservoir recurrence is the preferred pilot candidate,
not a predetermined choice. Select it only if the inventory supports a credible
scientific manuscript; otherwise record why and select the next qualifying
kernel. One review evaluates scientific communication for the named audiences;
the other evaluates architecture, reproducibility, dependency, build, and
migration integrity. Coding-agent review is not represented as external domain
peer review.

### Ordered Work

| ID | Prospective outcome | Advancement gate | State |
| --- | --- | --- | --- |
| `ASSURE-02` | Freeze the manuscript-first v2 documentation architecture and real-evidence nonpublic prototype described above. | Both reviews reconciled and user or named scientific-steward acceptance recorded. | `next`; no production or generated-content edits. |
| `ASSURE-03` | Retire the v1 SNOTEL candidate from the active catalog, generated public tree, and dormant export; preserve exact provenance, repair links, and publish a neutral zero-report interim surface. | Accepted `ASSURE-02` migration contract; no public/export path presents the v1 candidate; underlying science/evidence remains intact; deterministic build and reference checks pass. | `queued`; blocked by `ASSURE-02`. |
| `ASSURE-04` | Implement only the minimal v2 compiler demonstrated by the accepted prototype: typed traceability, staging/publication separation, deterministic manuscript assembly, result/figure substitution, dependency planning, drift checks, review locks, and snapshots. | One/all validation, planning, build, and check paths; content-hashed dependencies; stale/missing/unused identity failures; no ordinary-build network, shell, or agent execution; focused/full gates and touched-code CRAP closure. | `queued`; blocked by `ASSURE-02` and `ASSURE-03`. |
| `ASSURE-05` | Convert the accepted bounded non-snow prototype into the first complete v2 report using the production kernel, real consumer, retained evidence, figures/tables, and technical supplement. | Domain-reader outcome met; every quantitative statement resolves to evidence; limitations and contrary results remain visible; independent scientific/publication review and reproduction pass before public inclusion. | `queued`; blocked by `ASSURE-04`. |
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
| `CANOPY-PHENOLOGY` | Plant/snow-frost science: replace fixed-date deciduous/mixed-canopy behavior with hemisphere-robust leaf-off and leaf-on, then re-anchor the retained litter-drop window to the physical phenology signal. | An operator schedules the next increment by changing the current priority and authorizing a contract-first package. Before physics code, reconcile the landed native-management/YAML surface with the original first-class forest-input foundation and ratify the growth-canopy contract; then sequence leaf-off and leaf-on as separate closable increments. | `queued`; promoted, but not active while `ASSURE-02` is the current priority. [Program record](backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md). |

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
- [Scientific assurance dossier standard](standards/scientific-assurance-dossier.md)
- [Assurance lifecycle and build contract](governance/scientific-assurance-dossier-lifecycle.md)
- [ADR-0011: architecture-first, top-down science contracts](decisions/0011-architecture-first-top-down-science-contracts.md)
- [ADR-0017: comparator is a flag, not a target](decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md)
- [ADR-0018: defect-closure ExecPlan conversion](decisions/0018-defect-closure-execplans-conversion-rule.md)
- [Defect-Closure ExecPlan authoring](defect_closure_execplans.md)
