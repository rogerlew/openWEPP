# docs/work-packages/AGENTS.md
> Common prospective execution and closure contract.

## Authorship
**This document and all AGENTS.md documents are maintained by GitHub Copilot / Codex / Claude Code, which retain full authorship rights for all AGENTS.md content revisions. Agents may author and revise AGENTS.md documents when and where they see fit.** Revisions must preserve applicable user direction, package scope, review expectations, and higher-precedence governance.


## Required reading by task
Always read root AGENTS.md, this guide, assigned package.md and its
artifacts/worker-handoff.md if present, plus applicable write-path instructions.
Read your role procedure: [author](role-authoring.md),
[implementer](role-implementation.md), [reviewer](role-review.md),
[verifier](role-verification.md), or [runner](role-runner.md).
Combined roles read each applicable procedure once. Follow explicit mandatory
section references recursively; an unconditional pointer is not a byte saving.
Kernel-affecting work also reads the root science-contract instructions route
and applicable [science obligations](science-obligations.md).
DC, mechanical, metric and CQR work reads applicable sections of
[specialized workflows](specialized-workflows.md). No full-catalog bootstrap.

## Manual Validation Planning And Tool Friction
Validation planning has no prospective executable. Direct command selection and
compact evidence handling: role-implementation.md, same-named section.

## Execution
Confirm authorization, declare intent/exact write set, run
tools/agents/find-agents --for <paths>, retain required-reading-map.md.
Freeze acceptance before substantive edits. Execute the authorized checkpoint
through correction, validation and disposition while safe in-scope work remains.
Canonical SC authority governs physics, never package artifacts.
Manual validation follows docs/standards/testing-and-gate-strategy.md;
no TESTGATE, planner receipt or equivalent workflow engine.

## Gate Evidence Non-Deferral Rule
- A package, phase, or staged increment is complete only when every required
  increment-scope exit criterion and gate has direct evidence in the current
  artifact set. Boundary assignment follows declared pre-implementation intent
  and exact-diff terminal reconciliation under
  `docs/standards/testing-and-gate-strategy.md`.
- A campaign-owned obligation may remain `DEFERRED` only when declared before
  implementation with a named later boundary, owner, trigger, and rationale in
  the package/campaign record. Deferred is not passed, waived, or evidence for
  the current claim.
- If an increment-scope required gate can be proven only by a later
  phase/increment, the current phase is not complete. It must be marked `HOLD`
  / `executed-hold` with the later dependency named as the blocker.
- For DC-ExecPlans, this rule is not permission to stop early. If a gate is
  unmet because implementation, validation, or source reading remains inside
  the declared envelope, continue executing. `HOLD` is valid only after the
  package records why the missing evidence cannot be produced in-envelope.
- Do not reclassify an unmet current requirement as "next increment scope" after
  execution has started. Allowed alternatives are:
  1. execute the missing evidence in the current scope,
  2. amend the package/plan before implementation with explicit review that the
     gate is no longer current-scope acceptance, or
  3. hold with a named blocker and a defect-shaped follow-on.
- Validation tables must classify each required criterion as `PASS`, `FAIL`,
  `BLOCKED`, or `NOT RUN`; any `FAIL`, `BLOCKED`, or unjustified `NOT RUN`
  prevents `complete` disposition.
- Review and verification artifacts must check this rule explicitly. A review
  that verifies artifact presence but not requirement legitimacy is incomplete.
- Handoff language such as "lands in the next increment" is valid only for work
  that was not a current required gate. If it was a current required gate, that
  phrase must be paired with a hold disposition and blocker rationale.


## Independent closure
Require dual independent reviews, explicit finding disposition, accepted fix
verification and focused re-review, then dual independent verification of the
corrected stable substantive cut. Parent self-review cannot substitute.
Required artifacts under artifacts/: review_agent_a.md, review_agent_b.md,
finding-disposition.md, verification_agent_a.md, verification_agent_b.md,
gate-results.md, line-count-governance.md, final-disposition.md, worker-handoff.md.
Consolidate explanatory narrative while retaining distinct independent records.
Label Static: and Ran: truthfully. Missing roles/checks remain unmet.
Reviewer/verifier writes are limited to assigned artifacts. Explicit subagent
spawning/delegation authorization in package/kickoff names roles, scope, outputs
and write limits; actual tool policy governs. Existing user authorization
persists without repeated confirmation.
Selected heavy runs require comparator_suite_runner when available; record
actual unavailability before permitted parent fallback with compact logs.
Rust >=2000 lines: WARN, rationale/split intent; >=3000 nonexempt: refactor
before closure. Generated/fixture exception needs approval, owner and sunset.
No Rust edits: record not applicable.

## Current state and freshness
Executor maintains one authoritative continuation view:
artifacts/worker-handoff.md, normally 50-100 lines. package.md owns the execution
contract; handoff owns current state and next action. Include authorization and
boundaries, implemented/retained/rejected/unresolved work, settled decisions and
reopening triggers, source/evidence identities, authority, files/functions,
obligations and next action (or explicit lack of authorization). Link history.
Bind named source/evidence manifests or commits excluding handoff itself.
Update after meaningful changes, before delegation and disposition. Consumers
compare bindings with actual state; mismatch triggers targeted reconciliation,
never silent acceptance drift. No self-hash or future-commit prediction.
active.md locates current work; README.md preserves searchable history.
The index is a locator, not competing status authority.

## Reading measurements
Authors/reporters use docs/standards/prompt-wording-guidance.md#context-measurements.
Bootstrap target 32-64 KiB per role; shared always-read governance preferably
<=16 KiB. Targets never waive authority or stop authorized work. Record necessary
overruns and triggers. Expansion remains visible; workflow-total is observable
only to the extent actual telemetry supports it.
