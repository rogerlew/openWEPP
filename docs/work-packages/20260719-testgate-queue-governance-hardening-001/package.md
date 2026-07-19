# Harden TESTGATE Queue Governance

Package ID: `20260719-testgate-queue-governance-hardening-001`

Queue ID: `TESTGATE-QUEUE-01`

Status: `READY / ACTIVE`

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

- [ ] The three stranded runs are canceled and no queued/in-progress TESTGATE
  run remains.
- [ ] Current workflow uses the permanent group
  `openwepp-forest1-testgate`, explicit `queue: single`, and
  `cancel-in-progress: false`.
- [ ] Both initial admission and pre-execution checks require the run SHA to be
  current `origin/main`; stale or historical runs stop before selected gates.
- [ ] Only the exact forest1 label contract can run normal execution; release
  stability requires a distinct exact release label.
- [ ] Canonical governance tells agents to make required local commits, push
  once per stable increment, and avoid manual dispatch while a run is active or
  queued.
- [ ] Focused contract, YAML/static, Markdown, and diff checks pass.
- [ ] Dual review and dual verification pass with no accepted open finding.
- [ ] Final provider state is TESTGATE active, conservative active,
  release-gates disabled, forest1 online/idle, and zero Actions backlog.

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

Not executed.
