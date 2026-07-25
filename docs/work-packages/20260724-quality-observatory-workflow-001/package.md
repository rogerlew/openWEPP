# Optional Forest1 Quality Observatory Workflow

Package ID: `20260724-quality-observatory-workflow-001`

Status: `COMPLETE / ORDER-4`

## Pre-Implementation Intent

Risk: `CRITICAL`.

This package changes a trusted self-hosted workflow, queue normalization,
priority yielding, subprocess supervision, and evidence publication. The
implementation is limited to:

1. one manual-only workflow with an exact source-SHA input and a concurrency
   identity distinct from TESTGATE;
2. one repository-owned local controller for fail-closed occupancy
   normalization, nonblocking lease acquisition, bounded child supervision,
   typed deferral receipts, and compact publication indexing;
3. focused local-only integration and controller tests using recorded provider
   fixtures and fake children;
4. the minimum TESTGATE source contract and governance updates needed to prove
   priority and non-blocking semantics.

Deferred and partial execution evidence uses a separate control record
containing only `quality-control-receipt.json` and, when a child started,
`quality-partial-index.json`. Its total ceiling is 1 MiB. It is never written
under the canonical `published/` directory or named as a complete quality
observation. A deferred/partial receipt has no `quality_evidence_id`. Hosted
preflight deferrals and forest1 failures without a newly observed TESTGATE
priority upload that record as a control artifact. A forest1 priority deferral
retains it in the durable history root and performs no artifact upload, so the
runner is not held behind publication work.

A complete run validates and retains the separate bounded control receipt
locally, while the single canonical artifact carries the identity-bound
11-file observation. The receipt binds the verified quality evidence ID and
embeds the exact admission object needed for downstream independent
verification; it is not added as a twelfth canonical file. This one-upload
limit and its one-minute action timeout preserve the 90-second forest1 yield
bound after a newly observed TESTGATE priority. Each provider occupancy
snapshot has a five-second total deadline; provider timeout is fail-closed
occupancy and therefore triggers the same yield path.

The versioned defunct Omarchy predicate is exact: repository identity matches;
the run ID is one of `29673299308`, `29672334757`, or `29672149962`; the run
names `.github/workflows/testgate-shadow.yml`; its head is respectively
`850f7f6f`, `d4420b2`, or `4ee31784`; its event is `workflow_dispatch`; its
provider state is `completed/cancelled`; and inspection reports zero jobs and
zero artifacts. Any field drift, new ID, job, artifact, or forest1 label is
occupancy-unknown or live, never ignored. Age or queue duration alone never
establishes this classification.

Selected increment gates are Python compilation and controller self-tests,
focused workflow/controller integration tests, TESTGATE workflow source
contracts, Rustfmt and warnings-denied Clippy for touched Rust tests,
documentation lint, diff/write-set/prompt/line-count reconciliation, two
independent read-only reviews, and two independent read-only terminal
verifications. No live workflow or heavy run is selected.

## Objective

Implement an optional, manual, non-blocking forest1 QA workflow that never
competes with live TESTGATE work and publishes compact, identity-bound quality
evidence.

## Included Scope

- A manual `workflow_dispatch` quality workflow with a distinct stable
  concurrency identity.
- Pre-execution live TESTGATE occupancy detection and typed deferral.
- Explicit filtering of retired Omarchy queue records from forest1 occupancy.
- Sequential `full` then `science-manual` execution hooks.
- Compact artifact allowlist, size budget, retention, receipt, and summary.
- Workflow/source contracts and local characterization fixtures.

## Excluded Scope

- Scheduled automatic execution.
- Blocking branch protection, package closure, or TESTGATE status.
- CQR module selection.
- Changes to science tests or coverage-merging semantics beyond the workflow
  interface needed by Order 3.

## Declared Write Set

- `.github/workflows/quality-observatory.yml`
- `.github/workflows/testgate-shadow.yml`
- `Cargo.toml` (exact Order-4 integration-test registration only)
- `gate-policy/v1/**`
- `tools/local_ci/**`
- `tools/release/**`
- `tests/integration/testgate_*`
- `tests/integration/quality_observatory_*`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/work-packages/20260724-quality-observatory-workflow-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/testgate-quality-observatory-roadmap.md` (Order-4
  closeout routing only)
- `docs/ROADMAP.md` (Order-4/Order-5 queue routing only)

## Dependencies

- Orders 2 and 3 complete.

## Phase Plan

1. Define the QA dispatch, occupancy, deferral, receipt, and artifact contracts.
2. Implement the workflow and confined local orchestration.
3. Add deterministic source/fixture tests for current TESTGATE occupancy,
   defunct Omarchy records, distinct concurrency, sequential stages, and
   artifact allowlisting.
4. Prove no QA status can block TESTGATE or ordinary closure.
5. Reconcile, review, verify, and disposition.

## Exit Criteria

- Workflow is manual-only and forest1-specific.
- QA performs no expensive setup or test execution when a relevant live
  TESTGATE run is queued or running.
- Occupancy is rechecked immediately before acquisition. QA takes a nonblocking
  forest1 lease only when no forest1 TESTGATE job is queued or running.
- A non-preemptive mutex alone is insufficient. QA checks TESTGATE priority
  before acquisition and at safe boundaries after `full`, after
  `science-manual`, and before CRAP/report work. When TESTGATE becomes live, QA
  starts no new stage, preserves partial evidence, releases forest1, and emits
  `DEFERRED_TESTGATE_PRIORITY`.
- Deterministic race fixtures cover TESTGATE queued before acquisition,
  between the two profiles, and before CRAP/report publication.
- While a QA child is active, the supervisor polls exact forest1 TESTGATE
  occupancy at most every 30 seconds. On a live TESTGATE it terminates the QA
  child, uses one shared 54-second deadline for compact finalization and
  cleanup, releases the lease, and emits `DEFERRED_TESTGATE_PRIORITY`. Thus QA
  yields within 90 seconds; partial coverage is never published as complete.
- Defunct Omarchy records are ignored without cancellation or waiting.
- QA and TESTGATE have different stable concurrency groups and an executable
  guard prevents heavy overlap.
- The workflow invokes `full` before `science-manual`; later stages cannot run
  concurrently.
- Uploads are limited to the Order-3 canonical envelope/payload, run status,
  three inventory summaries, two compact JUnit files, adjudicated CRAP
  JSON/Markdown, and coverage-summary JSON. A pre-upload index rejects a total
  over 100 MiB. Raw LCOV, `.profraw`, Cargo targets, build/reconstruction/temp
  trees, and caches remain local-only even when compressed and are cleaned
  after compact finalization on success, failure, or priority deferral.
- Metric debt yields an observational report, while workflow/instrumentation
  corruption yields execution failure.
- A valid observation with actionable debt completes execution successfully
  with `debt_status=FAIL` and `closure_eligible=false`; collector, identity, or
  publication failure makes the workflow fail.
- Dispatch requires one exact lowercase 40-character source SHA; checkout must
  equal it. The report binds source commit/tree and workflow revision. If
  `main` advances during collection, evidence remains valid for its frozen
  subject but records `current_main=false`.
- Focused workflow contracts, local dry-run fixtures, documentation checks,
  dual review, and dual verification pass.

## Security Impact

Candidate code runs only on the exact forest1 labels. Queue data is treated as
untrusted provider input and normalized through exact repository, workflow,
head, job-label, and terminal-state checks.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to two read-only workflow/security reviewers and two read-only terminal
verifiers; expected outputs are compact package artifacts; write access is
read-only.
