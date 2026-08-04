# openWEPP Engine Roadmap

Status: living — canonical prospective planning queue

Last updated: 2026-08-04

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
| [`SNOW-WET-COMPACTION-OPERAND-AUTHORITY-AND-DUPLICATE-ALIAS-CLOSURE`](work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001/package.md) | Snow surface-energy-balance campaign | `active / 21K scaffolded` | Materialize the development-only Snowbird `1.2155576`-fold precipitation CLI while preserving the canonical control; establish the complete wet-compaction operand from contracts and the pinned legacy baseline; bound the confirmed duplicate data flow; and, when authority resolves the physical defect, implement and verify the exact-one correction before any early-melt attribution. |
| `SNOW-WARM-MIXED-PREPEAK-LOSS-ENERGY-ATTRIBUTION` | Snow surface-energy-balance campaign | `queued` | Start only after wet-compaction closure and corrected-state rebaselining. Use the scaled Snowbird development lane, canonical Snowbird control, Niwot/Paradise loss cases, and Mica Creek non-passing control to attribute warm/mixed dry-period loss without treating empirical CoE terms as measured energy fluxes. |
| `ASSURE-06` snow/frost flagship synthesis | Scientific assurance | `held` | Roger Lew remains the accountable report lead and a material snow/frost producer. Resume independent review only after 21K closes wet-compaction authority, 21L dispositions warm/mixed pre-peak loss, the scientific manuscript is refreshed against those results and the admitted canopy/energy work, and a new exact review root is issued. |
| `ASSURE-05` first production v2 report | Scientific assurance | `held` | Technical review handoff is complete; formal review has not started. Advancement requires assignment of accountable human reviewers, exact-subject review and approval, assurance-steward approval, release-owner transfer, and exact-root publication gates. |

## Test And Gate Architecture Queue

### Prospective Redirect

The user directed on 2026-07-27 that the gate planner be treated as a tool, not
an authority. Orders 0-4 removed its authority/control plane and tested a
bounded read-only advisory replacement. Order 5 applied the measured stop-loss
and deleted that replacement. Manual validation planning is now the only
prospective route. Existing TESTGATE and advisory-trial history remains
evidence, but no planner closeout, prerequisite execution, tool repair, or
automated planning surface is prospective work.

### Historical Pre-Redirect Direction

The remainder of this test/gate section records the pre-2026-07-27 TESTGATE
direction. It is retained for clause-level disposition by Order 0 and is not
prospective authority to run, repair, or extend TESTGATE. Where it conflicts
with the Prospective Redirect above, the redirect and direct manual execution
control.

Implement [ADR-0039](decisions/0039-campaign-scoped-risk-based-testing-and-assurance-gates.md)
as amended by
[ADR-0040](decisions/0040-accelerated-testgate-cutover-on-trusted-self-hosted-runner.md)
and the [testing and gate strategy](standards/testing-and-gate-strategy.md) so
increments receive fast affected-surface feedback while campaigns and releases
retain comprehensive exact-boundary qualification. Selection, escalation,
dependency tracking, evidence identity, reuse, and staleness must be
mechanical and explainable. Agent judgment may challenge or explain a plan; it
may not silently narrow one.

The detailed implementation scope, transition inventory, acceptance fixtures,
and staged-adoption contract live in the
[implementation handoff](work-packages/20260717-test-gate-authority-001/artifacts/implementation-handoff.md).
### Historical Adoption Contract

- TESTGATE is authoritative for ordinary trusted-main increments. Conservative
  broad correctness gates remain callable at critical, campaign, release, and
  explicit rollback boundaries. Coverage/CRAP is separate optional
  observational QA. There is no elapsed-time, increment-count, 50%, or
  dual-required gate.
- Unknown production impact escalates to critical/full execution; there is no
  generic operator or agent downgrade.
- Campaign and release certificates bind exact clean commits, complete gate
  inventories, current transitive inputs, and authenticated evidence.
- Protected evidence publication, issuer authority, revocation, and crash
  recovery remain campaign/release work. Provider-context migration applies
  only when a provider rule actually exists; none exists at current intake.
- Active campaigns enter through the governed bootstrap path. Prior evidence
  remains `LEGACY_UNVERIFIED` unless independently reconstructed and verified.
- `TESTGATE-CLOSURE-AUDIT-01` precedes the next heavy scientific package so the
  repaired path is exercised before another critical closure campaign.
- `TESTGATE-WORKFLOW-QUALIFY-01` is superseded by the
  [TESTGATE/quality-observatory roadmap](work-packages/testgate-quality-observatory-roadmap.md).
  Orders 1-7 must pass before `SNOW-SURFACE-EB` advances.

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
| `ASSURE-05` | Convert the accepted bounded non-snow prototype into the first complete v2 report using the production kernel, real consumer, retained evidence, figures/tables, and technical supplement. | Obtain accountable human review and approval for the exact review-ready source, then prove release transfer and public inclusion without changing the protected zero-report baseline prematurely. | `held`; technical/internal closure passed under `20260716-assure05-first-production-v2-report-001`, but named human authority is absent. |
| `CANOPY-ASSURE-01` | Complete the [canopy phenology assurance roadmap](planning/canopy-phenology-assurance-roadmap.md) by carrying the rendered draft through accountable human review, approval, release transfer, and publication when authorized. | The CAL evidence ladder, manuscript, supplement, research objects, and tracked human-review rendering are complete. Obtain accountable scientific and reproduction/publication review of the exact draft, disposition findings, record the required approvals, and prove release transfer before public inclusion. | `held`; the report remains a nonapproved review draft and does not displace `SNOW-SURFACE-EB`. |
| `ASSURE-06` | Refresh the snow/frost flagship after the selected science work, then carry the new exact manuscript through accountable human scientific review, approval, release transfer, and publication when authorized. | Canopy phenology and the selected sublimation/longwave work land; the report is updated and rebuilt; the obsolete review root is superseded; then independent scientific and reproduction/publication review, finding disposition, three-role approval, release transfer, and publication gates may resume. | `held`; Roger Lew remains report lead and material producer. Do not solicit review against the pre-science-change root. Public report count remains zero. |
| `ASSURE-07` | Build a priority portfolio of additional hydrology, erosion, sediment, plant, channel, and watershed reports and bind reviewed reports plus explicit gaps to release snapshots. | Selection by user importance and evidence readiness; ownership, currency, transfer, supersession, snapshots, and documented gaps remain explicit without implying site fitness. | `queued` after `ASSURE-05`; may proceed incrementally without blocking `ASSURE-06`. |
| `ASSURE-08` | Vendor the approved v2 report set into WEPPcloud during the openWEPP beta release campaign. | Refresh the handoff against current wepppy contracts; prove vendor sync, manifest/navigation/roles, rendering, link rewriting, accessibility, search, and the real downstream consumer; bind vendored content to the beta candidate. | `deferred`; mandatory immediately before openWEPP beta release in WEPPcloud. Do not start while WEPPcloud or the reports remain immature. |

Current cross-repository boundary:
[wepppy handoff](work-packages/20260714-assurance-dossier-lifecycle-foundation-001/artifacts/wepppy-handoff.md).
It is a dormant compatibility record, not authority to vendor.

## Recurring Maintenance

| ID | Owner and purpose | Trigger | State |
| --- | --- | --- | --- |
| `CQR-NIGHTLY` | Operator-owned repo-wide complexity/coverage maintenance under the [rolling ExecPlan](work-packages/cqr-nightly-burndown-execplan.md). After `QUALITY-CQR-HANDOFF-01`, selection consumes an exact current quality-observatory report and recollects only when stale or invalid. | An operator requests a bounded batch after optional QA and after active feature/science packages release overlapping write sets. | `recurring`; observational QA and CQR are not automatic blockers for science-package closure. |

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
