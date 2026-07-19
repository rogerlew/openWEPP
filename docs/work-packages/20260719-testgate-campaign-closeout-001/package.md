# Close The TESTGATE Campaign

Package ID: `20260719-testgate-campaign-closeout-001`

Queue ID: `TESTGATE-CLOSEOUT-01`

Status: `READY`

Authorization: Roger Lew's 2026-07-19 direction to close the TESTGATE campaign
without further timed gates, tests, or operator tracking.

## Objective

Close the TESTGATE campaign administratively now that the normal gate is live
and forest1 queue governance is enforced. Archive stale execution prompts,
accept the three non-executable GitHub run records as an external provider
exception, and leave no TESTGATE package in the active/held catalog.

## Scope

Included:

- archive the four remaining TESTGATE kickoff prompts;
- close `20260719-testgate-queue-governance-hardening-001` with an accepted
  external-provider exception;
- reconcile `docs/work-packages/README.md` and this package's artifacts;
- run documentation-only path, lint, and diff checks;
- obtain dual independent documentation review and terminal verification.

Excluded:

- production code, tests, workflows, runners, or provider mutations;
- another cancel, drain-runner, live TESTGATE, CRAP, coverage, or Nextest run;
- waiting for GitHub to repair the three inert historical run records.

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260717-testgate-plan-shadow-planner-001/prompts/**`
- `docs/work-packages/20260718-testgate-plan-crap-cleanup-001/prompts/**`
- `docs/work-packages/20260718-testgate-ci-shadow-executor-001/prompts/**`
- `docs/work-packages/20260718-testgate-ci-four-blocker-lift-001/prompts/**`
- `docs/work-packages/20260719-testgate-queue-governance-hardening-001/**`
- `docs/work-packages/20260719-testgate-campaign-closeout-001/**`

## Execution Plan

1. Commit this prospective package scaffold locally.
2. Archive the four stale prompts with history-preserving moves.
3. Record an accepted provider exception in the queue-governance package and
   move TESTGATE out of the active/held catalog.
4. Run Markdown lint, path integrity checks, and `git diff --check` only.
5. Complete dual independent review and dual terminal verification, archive
   this prompt, commit the closeout, and push once with TESTGATE temporarily
   paused.

## Acceptance

- [ ] Every TESTGATE package has zero files under `prompts/active/`.
- [ ] The queue-governance package is complete with a precisely bounded
  external-provider exception; it does not claim the orphan records vanished.
- [ ] No TESTGATE package remains in `Current Active/Held Packages`.
- [ ] Catalog text preserves the historical CI HOLD and its later supersession.
- [ ] Markdown lint, moved-path integrity, and diff checks pass.
- [ ] Dual review and dual terminal verification have no open findings.
- [ ] No code test, gate execution, runner operation, or provider cleanup runs.

## Review And Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent documentation reviewer/verifier roles
for exact-diff review, historical-truth preservation, active-prompt inventory,
catalog consistency, and terminal verification; expected outputs are compact
PASS/HOLD findings and evidence suitable for package artifacts; write access is
read-only.

Subagent requirement: two independent documentation reviewers/verifiers are
required. No heavy-run subagent is selected because this package is
documentation-only and explicitly exempt from campaign CRAP/testing gates.

## Outcomes

Pending execution.
