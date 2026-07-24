# TESTGATE And Quality Observatory Roadmap

Status: `RATIFIED / EXECUTING / ORDER-3-NEXT`

Owner: maintainers

Ratified by Roger Lew on 2026-07-24.

Decision statement: TESTGATE determines whether an increment is admissible.
The quality observatory measures maintainability debt. Neither may impersonate
the other.

## Execution Progress

- Order 0 is complete at roadmap-ratification commit `497d76d0`.
- Order 1 is complete at `3cb9fbb5`, preceded by the prospective write-set
  authority commit `bd11e60d`. ADR-0041 is accepted, canonical governance is
  aligned, predecessor obligations are dispositioned, and dual terminal
  verification passed.
- Order 2 is complete at implementation head `e1e26a15` and closeout commit
  `1d7b4576`. The exact terminal execution passed 12/12 nodes and 2,288/2,288
  inventory items, including 2,262 full-workspace nextest tests. Dual review
  and detached exact-head terminal verification passed.
- TESTGATE now records the closure-eligible
  `DEFERRED_TO_QUALITY_CI` disposition owned by
  `openwepp-quality-observatory` and triggered by
  `OPTIONAL_OPERATOR_DISPATCH`. Ordinary TESTGATE and routine release
  validation execute no retired coverage/CRAP subprocess or upload path.
- Order 3, `20260724-quality-observatory-merged-coverage-001`, is the next
  executable package. Orders 4 through 7 remain queued behind their declared
  dependencies.

## Outcome

Restore TESTGATE to a fast, manual, blocking forest1 increment gate and move
workspace coverage plus adjudicated CRAP into a separate, optional,
observational forest1 QA workflow. The QA workflow runs only at operator
direction, after TESTGATE activity, and its exact report may seed CQR Nightly.
QA findings remain visible but do not block ordinary package or increment
closure.

The roadmap does not weaken science correctness. Affected contract tests,
authority suites, typed guards, conservation checks, real-consumer tests, and
other correctness gates remain in TESTGATE when mechanically selected.
Coverage and CRAP remain binding inside an explicitly authorized module
test-enhancement or CQR package whose objective is to close those metrics.

## Operating Model

| Surface | Trigger | Forest1 order | Closure effect |
| --- | --- | --- | --- |
| TESTGATE | Manual operator dispatch | Highest priority; never overlaps QA | Blocking for the admitted increment |
| Quality observatory | Optional manual operator dispatch | Defers before work when a live forest1 TESTGATE is queued or running | Observational and non-blocking |
| CQR Nightly | Operator directive after a current QA report | Consumes the exact QA report; recollection requires typed stale/invalid evidence plus an explicit operator directive | Its own module packages retain their declared CQR gates |

After Order 2, TESTGATE receipts must record workspace coverage and CRAP as
`DEFERRED_TO_QUALITY_CI`, not `PASS`, `SKIPPED`, or a closure failure. After
Orders 3 and 4, the QA observatory executes the `full` profile followed by
`science-manual` against one instrumented source/build identity, merges their
LLVM coverage, and runs global adjudicated CRAP from the merged result. Uploads
are restricted to the canonical evidence envelope/payload, run status, three
inventory summaries, two compact JUnit files, adjudicated CRAP JSON/Markdown,
and per-file coverage-summary JSON. A pre-upload index enforces a 100 MiB total
ceiling. Raw LCOV, `.profraw`, build/target/reconstruction/temp trees, and
caches remain local-only even when compressed.

## Ordered Packages

| Order | Status | Package | Outcome | Dependency |
| --- | --- | --- | --- | --- |
| 0 | `COMPLETE` | `20260724-testgate-quality-observatory-roadmap-001` | Ratify this decomposition, independent review, and finding disposition. | User direction |
| 1 | `COMPLETE` | `20260724-testgate-quality-authority-separation-001` | Adopt ADR-0041 and align canonical policy so ordinary closure no longer depends on workspace CRAP/coverage. | Order 0 |
| 2 | `COMPLETE` | `20260724-testgate-quality-deferral-001` | Removed quality execution nodes from TESTGATE and issued a verified `DEFERRED_TO_QUALITY_CI` receipt state. | Order 1 |
| 3 | `NEXT / QUEUED` | `20260724-quality-observatory-merged-coverage-001` | Produce one valid merged `full` plus `science-manual` coverage identity and prove snowbench is measured. | Order 2 |
| 4 | `QUEUED` | `20260724-quality-observatory-workflow-001` | Implement the optional forest1 QA workflow, TESTGATE-first deferral, compact artifacts, and non-blocking result contract. | Orders 2-3 |
| 5 | `QUEUED` | `20260724-cqr-nightly-quality-evidence-handoff-001` | Make CQR Nightly consume an exact current QA report and recollect only with typed stale/invalid evidence plus an explicit operator directive. | Orders 3-4 |
| 6 | `QUEUED` | `20260724-testgate-quality-observatory-qualification-001` | Prove changed-head TESTGATE succeeds on forest1 without quality execution and with typed deferral. | Orders 1-5 |
| 7 | `QUEUED` | `20260724-quality-observatory-cqr-qualification-001` | Prove optional QA and exact-report CQR intake work end-to-end after TESTGATE. | Orders 5-6 |

Orders 2 and 3 are serialized because their tooling/test write sets overlap.
Order 6 qualifies the exact final implementation head after Orders 1-5; Order 7
then runs QA against that same qualified subject.
The qualification packages may correct in-scope workflow/tooling defects
iteratively; they may not change policy to make a failure pass.

## Functional Acceptance

The roadmap is successful only when all of the following are directly proven:

1. A changed-head TESTGATE dispatch on forest1 passes its selected correctness
   DAG, independent verification, and retained receipt checks without executing
   global CRAP or workspace coverage.
2. The TESTGATE receipt explicitly records `DEFERRED_TO_QUALITY_CI` and remains
   closure-eligible.
3. An operator-dispatched QA run on the same qualified head executes `full`
   then `science-manual` sequentially, merges coverage, runs adjudicated CRAP,
   and publishes compact evidence regardless of the metric verdict.
4. A QA run detects live relevant TESTGATE occupancy before expensive work and
   exits with a typed deferred result. Permanently queued records from the
   retired Omarchy runner are ignored and are neither awaited nor canceled.
5. The QA report identifies source head, policy, workflow revision, runner,
   toolchain, ordered profile names, per-profile selected-inventory and JUnit
   digests/counts, the union inventory, coverage inputs, merged coverage digest,
   CRAP registry digest, raw/adjudicated/actionable counts, and artifact
   digests. Its `quality_evidence_id` is SHA-256 of canonical JSON payload bytes
   excluding the derived ID and containing-envelope digest. An outer envelope
   stores the ID, payload, and publication metadata; verification canonicalizes
   the payload and recomputes the ID.
   Compact evidence includes every raw, adjudicated, and actionable row with
   symbol identity and metrics, not counts alone.
6. The exact 18-row snowbench ledger from run `30113946779` is reconstructed.
   Each row receives a `science-manual` coverage contribution or is explicitly
   retained as legitimately uncovered debt; no row remains actionable solely
   because the collector used full-only coverage.
7. CQR Nightly accepts the exact current QA report identity, selects actionable
   modules from it, and refuses stale, malformed, or mismatched evidence.
8. TESTGATE and QA have distinct stable concurrency identities and never
   execute heavy work concurrently on forest1.

## Failure And Retry Policy

- Retain every failed receipt and compact artifact.
- Correct a typed planner, executor, verifier, workflow, measurement, or
  operator-interface defect before repeating expensive work.
- One infrastructure-only retry is allowed under the canonical retry policy.
- A metric verdict does not fail the QA workflow's execution-integrity result;
  it is published as observational debt.
- Do not dispatch TESTGATE while a current forest1 run is queued or active.
- Ignore immutable defunct Omarchy queue records. They cannot be cleared and
  are not forest1 occupancy.

## Closeout

Order 1 dispositioned
`20260724-cqr-testgate-coverage-reconstruction-001` as
`EXECUTED-HOLD / QUALIFICATION-HANDOFF-ORDER-6`. Its exact reconstruction and
profile inventory corrections remain valid historical inputs; its failed
global CRAP result remains unchanged and is not rewritten as a pass.

After Order 7 passes, remove this campaign from `docs/ROADMAP.md`, record each
completed package in `docs/work-packages/README.md`, and return the priority
queue to science work. Quality CI and CQR Nightly then remain operator-directed
recurring maintenance, not prerequisites to routine science-package closure.
