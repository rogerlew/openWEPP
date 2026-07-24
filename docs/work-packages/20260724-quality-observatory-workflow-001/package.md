# Optional Forest1 Quality Observatory Workflow

Package ID: `20260724-quality-observatory-workflow-001`

Status: `QUEUED / ORDER-4`

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
- `gate-policy/v1/**`
- `tools/local_ci/**`
- `tools/release/**`
- `tests/integration/testgate_*`
- `tests/integration/quality_observatory_*`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/work-packages/20260724-quality-observatory-workflow-001/**`
- `docs/work-packages/README.md`

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
  child, allows at most 60 seconds for compact partial-evidence finalization and
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
