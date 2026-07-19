# Harden TESTGATE Queue Governance

Package ID: `20260719-testgate-queue-governance-hardening-001`

Queue ID: `TESTGATE-QUEUE-01`

Status: `COMPLETE / ACCEPTED-EXTERNAL-PROVIDER-EXCEPTION`

Authorization: Roger Lew's 2026-07-19 direction to prevent forest1 thrash and
cumulative Actions queues.

## Objective

Make queue bounds durable and machine-enforced. Normal TESTGATE must use one
permanent repository concurrency identity with at most one running and one
pending run, reject historical manual dispatch before expensive work, detect a
superseded push before bootstrap and again before gate execution, and prevent
unrelated generic self-hosted jobs from landing on forest1. Clear the three
stranded pre-pivot runs and preserve one-push-per-stable-increment guidance.

## Scope And Write Set

- `.github/workflows/testgate-shadow.yml`
- `.github/workflows/release-gates.yml`
- `AGENTS.md`
- `docs/standards/testing-and-gate-strategy.md`
- `tests/integration/testgate_ci_executor_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260719-testgate-queue-governance-hardening-001/**`
- GitHub Actions runs `29673299308`, `29672334757`, and `29672149962` may be
  canceled after exact read-only confirmation that they remain queued and bind
  retired `omarchy` workflow revisions.

Provider cleanup amendment (2026-07-19): GitHub returned HTTP 500 for both
normal and force cancellation of all three exact queued records. Each record
has zero jobs and zero artifacts. The same three IDs may therefore be deleted
through the provider API to remove their inert queue records; no other run,
artifact, log, or evidence may be deleted.

Provider drain amendment (2026-07-19): GitHub then returned HTTP 403 for DELETE
because queued runs cannot be deleted. The controller may temporarily register
one repository-scoped `openwepp-queue-drain-01` runner with the retired
`omarchy` labels solely to drain those three exact records. Build a no-network
derivative of the already reviewed pinned image using the package's exact
`queue-drain.Dockerfile` and `reject-job.sh`; the derivative may add only the
immutable `/usr/local/bin/openwepp-reject-job.sh` pre-job hook. The hook must
exit nonzero so GitHub marks each assigned job failed before candidate steps
execute. Runtime is limited to five minutes, at most the three preflight-bound
jobs, one CPU, 1 GiB memory, read-only root/state, all capabilities dropped,
`no-new-privileges`, bounded tmpfs, and no host bind or Docker socket. Remove
the exact `openwepp-queue-drain-01` provider registration,
`openwepp-queue-drain` container, `openwepp-queue-drain-state` volume, and
derived image immediately after all three runs reach terminal failure. It may
not accept forest1-labeled or current workflow jobs; any unexpected assignment
is also rejected before steps and terminates the drain.

No runner image, container, registration, gate selector, production Rust,
science contract, threshold, or release behavior is otherwise in scope.

## Protected Boundaries

- Keep normal `cancel-in-progress: false`; repeated termination of cold jobs is
  itself runner thrash.
- Do not opt into a multi-item pending queue.
- Preserve push-to-main and trusted manual admission, exact labels, read-only
  permissions, fail-closed receipts, hosted verification, and attestation.
- Supersession must fail before expensive work and must not mint successful
  evidence for a stale head.
- `release-gates` stays disabled. Its stability job must require a dedicated
  release label if it is ever re-enabled; it may not use generic
  `self-hosted`.
- Do not run full Nextest, coverage, CRAP, or a live broad TESTGATE execution.
  This package changes workflow scheduling and static contract bindings only;
  no production Rust or gate-selection bytes change.

## Execution Plan

1. Scaffold and locally commit this prospective authorization before edits.
2. Reconfirm and cancel the three exact stranded runs.
3. Implement permanent single-pending concurrency, exact-current-head checks,
   pre-execution supersession, exact release-runner labels, and terse agent
   push/dispatch rules.
4. Extend the focused CI contract; run YAML/static checks, the one integration
   test, Markdown lint, shell/diff checks, and provider queue inspection.
5. Complete dual independent review and dual terminal verification. Patch
   accepted findings and rerun only invalidated focused checks.
6. Commit the stable implementation locally, temporarily disable normal
   TESTGATE for the single push, push all local commits once, re-enable it, and
   verify no current or stranded queue remains. Archive the prompt and close
   package/catalog evidence in that same final push when practical.

## Acceptance

- [x] ACCEPTED EXTERNAL EXCEPTION: the three stranded runs remain
  provider-orphaned with zero jobs,
  artifacts, or concurrency leases. Cancel and force-cancel return HTTP 500;
  DELETE returns HTTP 403; a bounded retired-label drain received no
  assignment. No current or forest1-capable run is queued or active.
- [x] Current workflow uses the permanent group
  `openwepp-forest1-testgate`, explicit `queue: single`, and
  `cancel-in-progress: false`.
- [x] Both initial admission and pre-execution checks require the run SHA to be
  current `origin/main`; stale or historical runs stop before selected gates.
  The hosted aggregate also checks current main before attestation and as its
  final condition after native verification and evidence upload so a
  superseded head cannot finish successful.
- [x] Only the exact forest1 label contract can run normal execution; release
  stability requires a distinct exact release label.
- [x] Canonical governance tells agents to make required local commits, push
  once per stable increment, and avoid manual dispatch while a run is active or
  queued.
- [x] Focused contract, YAML/static, Markdown, and diff checks pass.
- [x] Dual independent implementation review passes with no accepted open
  implementation finding; terminal provider verification records the external
  orphan hold.
- [x] ACCEPTED EXTERNAL EXCEPTION: final provider state has TESTGATE active,
  conservative active,
  release-gates disabled, and forest1 online/idle, but GitHub still reports the
  three non-executable orphan records as queued.

## Review And Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent reviewer/verifier roles for read-only
workflow security, queue semantics, test-economy, exact-diff review, and
terminal provider-state verification; expected outputs are compact findings,
PASS/HOLD verdicts, and exact evidence references; write access is read-only.

Dual reviewers must check queue replacement semantics, concurrency-name
stability, stale-head behavior, generic-runner exclusion, fail-closed evidence,
and unnecessary-test risk. Dual terminal verification must reconcile the exact
committed diff, focused results, provider workflow states, runner state, and
queue inventory.

## Outcomes

Queue hardening is implemented and focused validation passes. Current and
future normal runs share one permanent single-pending group, stale heads fail at
four execution/authority boundaries, generic self-hosted release routing is
removed, and agents are directed to push stable increments once.

Complete with an accepted external-provider exception. GitHub refuses cancel,
force-cancel, and deletion of the three exact zero-job/zero-artifact pre-pivot
records. A five-minute confined retired-label runner with an immutable
rejection hook remained online and idle without receiving an assignment, then
was completely removed. The orphan records cannot match forest1 and hold no
live concurrency lease, so they cannot thrash the host. Their displayed state
remains recorded truthfully but no longer keeps the engineering campaign open.
No timer or monitoring handoff is created.
