# ASSURE-MAINT03 Tracked Human-Review Rendering

Status: `COMPLETE / PASS`

Evidence mode: `implementation + deterministic generated-document verification`

Intent: `render admitted assurance drafts into a committed human-readable
usersum review lane without approval or publication`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.

## Purpose

Humans cannot review assurance reports from YAML descriptors and unresolved
Markdown directives. Disposable output under ignored `target/**` is useful for
machine verification but is not a durable review artifact. This package makes
the complete rendered reports readable in the repository under:

```text
usersum/assurance/review-drafts/
```

Every page remains visibly marked `DRAFT`. The approved public report catalog
remains separate and empty.

## Objective

Add a deterministic tracked-review synchronization/check workflow, correct
reader-visible directive/unit duplication in the canopy draft, adopt those
source edits through the typed assurance transaction, and commit complete
rendered review copies of all three admitted reports.

## Classification And Intent

Classification: `Critical assurance transaction + integrated documentation
consumer`.

Implementation intent:

- add a local maintenance command that invokes the real assurance builder;
- extend manifest-selected DRAFT source adoption to atomically admit all
  drifted internal source files owned by that report while rejecting unrelated
  drift;
- replace only the complete `usersum/assurance/review-drafts/**` tree with
  caught-error rollback;
- provide a generated draft index and complete report, supplement, figure, and
  research-object trees;
- support `--check` with zero destination/tracked writes and exact
  missing/extra/drift detection;
- correct only rendering-induced prose duplication in CAL-09;
- use typed source adoption for the corrected CAL-09 DRAFT; and
- preserve public catalog, approval, release, export, and vendoring state.

This package changes no kernel science, operands, results, figures,
physical/dimensional unit definition, precision, conclusions, or fitness
claims. One count-unit display symbol changes to inclusive singular/plural
notation.

## Included Scope

- `tools/local_ci/render_assurance_review_drafts.py`
- focused tests for apply/check, drift, extra files, and destination safety;
- CAL-09 manuscript/supplement wording needed to avoid duplicated or awkward
  rendered count nouns; the canonical `member_count` result unit remains
  unchanged;
- typed CAL-09 source adoption and generated identity evidence;
- complete generated review output for all admitted reports;
- review-draft index with explicit nonpublication language;
- refresh/check instructions in assurance maintenance documentation;
- roadmap and work-package catalog updates;
- deterministic comparison with a fresh unrelated build;
- Markdown-consumer, local-link, accessibility, and protected-boundary checks.

## Excluded Scope

- Human review events, findings, approvals, or release transfer.
- Public report catalog entries or `PUBLISHED` claims.
- Production publication, snapshots, export, vendoring, or WEPPcloud transfer.
- Scientific reinterpretation or calibration.
- Changes to groundwater or snow/frost authored report sources.
- Kernel, runtime, orchestration, serialization, or model behavior.

## Deliverables

1. A fail-closed `--apply`/`--check` review-rendering maintenance command.
2. Readable CAL-09 generated prose without duplicated directive units/nouns.
3. Three complete rendered report trees and a review index under tracked
   `usersum/assurance/review-drafts/`.
4. Evidence that fresh build bytes equal committed review bytes.
5. Evidence that the approved public catalog remains empty and unchanged.

## Intended Write Set

- `docs/work-packages/20260729-assure-maint03-tracked-human-review-rendering-001/**`
- `docs/work-packages/README.md`
- `docs/planning/scientific-assurance-v2-implementation-roadmap.md`
- `assurance/v2/README.md`
- CAL-09 `report.yaml`, `manuscript.md`, and `supplement.md`; all three generated
  review locks; the identity lock; and typed transaction receipts
- `tools/local_ci/render_assurance_review_drafts.py`
- `tools/local_ci/test_render_assurance_review_drafts.py`
- `crates/openwepp-assurance/src/v2/amendment.rs`
- `crates/openwepp-assurance/src/v2/assembly.rs`
- `crates/openwepp-assurance/src/v2/draft_adoption.rs`
- `crates/openwepp-assurance/src/engine.rs`
- relevant assurance API/documentation wiring if required
- `tests/integration/assurance_v2_amendment_contract.rs`
- `tests/integration/assurance_v2_assembly_contract.rs`
- `usersum/assurance/review-drafts/**`

Runtime-only temporary roots remain untracked.

## Protected Paths

- `usersum/assurance/README.md` and the approved public report catalog;
- `usersum/assurance/reports/**`;
- groundwater and snow/frost authored sources;
- report operands, results, tables, figures, references, and conclusions;
- kernel/runtime crates and all science contracts;
- release, export, snapshot, vendor, and WEPPcloud paths.

## Required Reading

- root, crate, test, and work-package `AGENTS.md` instructions;
- `docs/codex_exec_plans.md`;
- `docs/standards/testing-and-gate-strategy.md`;
- `assurance/v2/README.md`;
- `docs/governance/scientific-assurance-v2-source-build-contract.md`;
- ASSURE-MAINT02 final disposition and render evidence;
- CAL-09 report source and current rendered output.

## Execution Plan

### Phase 1 — Freeze Baseline

Record the exact Git identity, three-report generation, public-catalog bytes,
current rendered inventory, duplicated CAL-09 phrases, write set, and direct
gate selection.

### Phase 2 — Implement Review Synchronization

Implement a deterministic local command with exactly one of `--check` and
`--apply`. It must:

- require an explicitly supplied built assurance binary;
- build all admitted reports in an owned temporary root;
- generate a visibly nonapproved review index;
- compare exact relative paths and bytes;
- apply through a complete temporary sibling, whole-tree rename, and
  caught-error rollback;
- reject symlink/special-file destinations and path escape;
- never alter `usersum/assurance/README.md` or `reports/**`; and
- return nonzero for missing, extra, or drifted review files.

### Phase 3 — Correct And Adopt CAL-09 Prose

Remove source nouns already supplied by typed quantity units, preserving the
same meaning and quantities. Run typed `adopt-report-source --check`, inspect
each receipt, apply the reviewed source increments, and verify the new anchored
generation.

### Phase 4 — Render Tracked Review Drafts

Run the new command with `--apply`, then `--check`. Prove all three report
trees contain `index.md`, `supplement.md`, `build-manifest.json`, figures, and
declared public-safe research objects. Parse all report/supplement Markdown and
verify links and SVG accessibility. Confirm no unresolved directives or known
duplicated rendered phrases remain.

### Phase 5 — Reconcile And Close

Run focused tests, formatting/linting, assurance validation/build/check, strict
Clippy, full-workspace all-feature correctness, protected-boundary checks,
documentation lint, dual independent review, finding disposition, and dual
terminal verification. Reconcile every generated path and close only with
direct evidence.

## Acceptance

- `usersum/assurance/review-drafts/README.md` links all three rendered drafts.
- Every report and supplement is readable without unresolved directives.
- Known count/unit duplication is absent.
- `--check` proves exact current bytes with no destination or tracked writes.
- A fresh unrelated build has the same report inventory and bytes.
- All report lifecycle and publication states remain `DRAFT`.
- Approved public catalog count remains zero.
- Protected paths remain unchanged.
- Applicable focused tests, strict lint, documentation checks, dual review, and
  dual verification pass.

## Subagent Authorization

This package explicitly authorizes spawning/delegating one read-only
full-workspace gate runner, two independent read-only implementation reviewers,
and two fresh read-only terminal verifiers. Expected outputs are compact
evidence-backed findings returned to the primary agent and recorded in package
artifacts. Delegates must not edit production or generated files.

## Progress

- [x] (2026-07-29) User clarified that durable rendered drafts are required for
  human review before approval.
- [x] (2026-07-29) Selected a tracked, explicitly nonapproved review lane under
  `usersum/assurance/review-drafts/`.
- [x] (2026-07-29) Froze baseline and direct gate plan.
- [x] (2026-07-29) Implemented deterministic review synchronization.
- [x] (2026-07-29) Corrected and adopted CAL-09 rendered-prose defects.
- [x] (2026-07-29) Rendered and verified all three tracked review drafts.
- [x] (2026-07-29) Completed dual reviews, dual terminal verification,
  exact-diff reconciliation, and closure.

## Surprises And Discoveries

- ASSURE-MAINT02 proved complete rendering only in ignored disposable roots;
  that is insufficient as a durable human-review handoff.
- The rendered CAL-09 draft exposed count-unit duplication that is not visible
  in the templated manuscript source.
- Accessibility inspection of the first tracked render found that retained
  figures were sanitized in `figures/**`, while the corresponding linked SVG
  research-object copies retained raw source bytes without title, description,
  or image role. The real reader tree therefore requires the same safe SVG
  rendering at both consumer paths.
- The legacy zero-public verifier initially classified the explicitly separate
  `review-drafts/**` lane as public output. Its exact public-file check must
  exclude only that named review subtree while continuing to reject every
  undeclared public file and symlink.

## Decision Log

- Decision: use `usersum/assurance/review-drafts/`, not the approved
  `usersum/assurance/reports/` route.
  Rationale: reviewers need stable readable artifacts, while approval and
  publication remain separate lifecycle events.
- Decision: commit complete research-object trees, not report prose alone.
  Rationale: human review requires working evidence links.
- Decision: retain the synthesis result's canonical `member_count` unit.
  Rationale: a dry-run attempt to relabel only the value binding as a
  configuration count failed the typed result-unit contract. Reader prose was
  corrected without weakening or falsifying that contract.
- Decision: render the `transition_count` symbol as `transition(s)`.
  Rationale: the frozen values include both one and zero, while the current
  unit schema carries one invariant symbol rather than singular/plural forms.
  The inclusive symbol preserves the count definition and avoids the
  reader-visible `1 transitions` defect without hard-coding a value.
- Decision: sanitize retained SVG research-object consumer copies as well as
  displayed figure copies.
  Rationale: both paths are linked reader artifacts and require the same active
  content and accessibility controls.
- Decision: describe review synchronization as complete-tree replacement with
  caught-error rollback, not crash-atomic exchange.
  Rationale: the portable two-rename installation has no stale-backup recovery
  protocol and can expose a missing lane across an uncatchable process crash.
  Crash-atomic exchange is not required for generated review copies because
  the deterministic command safely reconstructs the complete lane.

## Outcomes And Retrospective

The complete three-report catalog is now committed under
`usersum/assurance/review-drafts/` as 92 resolved files. Humans can read the
reports, supplements, figures, captions, and linked evidence without resolving
YAML or template directives. The deterministic maintenance command rebuilds
through the real assurance consumer and checks exact current bytes.

Real-consumer validation improved the implementation twice: linked
research-object SVGs now receive the same sanitization/accessibility metadata
as displayed figures, and the legacy zero-public verifier recognizes only the
named review subtree as nonpublic while retaining recursive safety checks.
Independent review then caught the final invalid singular count and an
overstated crash-atomicity claim; both were corrected.

All reports remain `DRAFT`. This package deliberately stops before human review
events, approval, public catalog inclusion, release transfer, export, or
vendoring.
