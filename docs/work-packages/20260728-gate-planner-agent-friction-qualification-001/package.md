# Advisory Linter Agent-Friction Qualification

Package ID:
`20260728-gate-planner-agent-friction-qualification-001`

Queue ID: `GATE-LINT-ADV-03`

Status: `ACTIVE`

Authorization: the user's 2026-07-28 instruction to scaffold and execute
advisory-linter roadmap Order 5.

Base commit: `c70c9e863d6b7b55adeb29a2fec5d1247fcbdf59`

Execution mode: `package-end-to-end`.

This ExecPlan is maintained under `docs/codex_exec_plans.md`.

## Objective

Measure whether the optional advisory linter reduces agent planning friction
against the independent manual route on the frozen six-package, 18-case cohort.
Retain the linter only if every accepted Order-0 utility, safety, noise,
latency, and maintenance threshold passes. Otherwise delete the linter path
and keep modeling work on the manual route. No CI qualification is allowed.

## Implementation Intent

`agent-tool qualification / behavior measurement / stop-loss disposition`.
This package changes no kernel, science, calibration design, protected Harvard
state, package authority, validation requirement, or lifecycle semantics.

## Included Scope

- Freeze six real package snapshots: two documentation/governance, two
  non-kernel Rust, and two kernel/science packages including CAL-04B.
- Freeze evaluator-authored critical-obligation keys before trials.
- Run 18 paired manual/linter cases across `pre-edit`, `working-tree`, and
  `terminal` modes, using the same agent, brief, and immutable snapshot in each
  pair.
- Use a recorded seeded counterbalance with nine manual-first and nine
  linter-first cases.
- Record monotonic planning time, outer tool invocations, linter cold/warm
  latency, findings, availability, plan text, and interruption/maintenance
  time. Suggested commands are never executed by the linter.
- Blind two independent scorers to arm labels for critical-obligation coverage
  and deterministic-finding actionability.
- Apply every Order-0 metric and stop-loss exactly.
- If any stop-loss fires, delete the advisory command/source/tests and remove
  prospective operative instructions to use it. Historical packages and
  evidence remain unchanged.

## Excluded Scope

- No CI workflow, daemon, database, ledger, receipt, remote identity,
  publication, recovery, or lifecycle integration.
- No automatic linter repair campaign, compatibility wrapper, or prerequisite
  package.
- No CAL execution, model command, ensemble, synthetic recovery, Harvard read,
  freeze/open transition, or calibration result.
- No change to underlying direct validation, authority, security, science, or
  protected-data requirements.
- No rewrite of historical Order 0-4 package evidence.

## Frozen Cohort

The exact package classes, scaffold/terminal commits, task briefs, and
reconstruction rules are recorded in `artifacts/cohort-and-snapshots.md`.

The cohort is:

- documentation/governance: advisory-linter roadmap Order 0 and governance
  authority alignment Order 1;
- non-kernel Rust: OWCMP01 comparator CLI and assurance-v2 amendment Clippy
  disposition;
- kernel/science: native-GSI canopy-height coherence hold lift and CAL-04B
  calibration-readiness/ensemble execution.

## Declared Write Set

- `tools/validation/workplan-lint`
- `tools/validation/workplan_lint.py`
- `tools/validation/test_workplan_lint.py`
- `tools/validation/README.md`
- `tests/integration/advisory_linter_authority_contract.rs`
- `tools/release/authority-policy/impact-map.json`
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/decisions/0043-gate-planner-is-a-non-authoritative-advisory-linter.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/gate-planner-advisory-linter-roadmap.md`
- this package subtree

The user-owned untracked
`docs/audits/20260727_gate_planner_demotion_readiness_audit.md` remains
unchanged, unstaged, and excluded.

## Phase Plan

1. Scaffold and commit the autonomous package.
2. Freeze the cohort, reconstruction procedure, seeded arm order, evaluator
   obligation key, scoring rubric, and measurement schema.
3. Run all 18 paired cases through authorized comparison agents. Retain raw
   plans, timestamps, invocation counts, linter outputs, and latency evidence.
4. Blind arm labels and obtain two independent coverage/actionability scores.
5. Compute paired results, medians, bootstrap confidence intervals, adjusted
   planning time, interaction reduction, noise, omission, interruption,
   latency, and implementation-size metrics.
6. Apply the stop-loss without interpretive override. Retain the linter only if
   every threshold passes; otherwise delete it and reconcile operative docs.
7. Run focused validation, dual review, finding disposition, dual
   verification, exact-diff reconciliation, prompt archival, catalog/roadmap
   closure, and final disposition.

## Measurement Contract

Planning time starts when an arm receives its brief/snapshot and stops when it
submits an exact obligation/command plan. Command runtime is excluded. A
plan-construction interaction is one outer tool invocation or user
clarification during that interval. Batched reads count once.

Each paired case uses the same agent. Arm order follows a recorded seed. One
separate warm-up per comparison agent is discarded. Investigation and
maintenance time attributable to false findings, unavailable analysis, or
linter defects is divided across scored linter cases and added to every linter
case before the median comparison.

Reviewers score anonymized arms. A critical obligation is frozen before trial
and requires agreement by both scorers. A deterministic finding is
non-actionable only when both scorers agree it is false, inapplicable,
duplicate, or incapable of changing the plan.

## Acceptance And Stop-Loss

Retention requires all of:

- zero reviewer-confirmed critical-obligation omissions;
- at most 10% non-actionable deterministic findings;
- at least 30% reduction in adjusted median planning time;
- at least 50% reduction in plan-construction interactions;
- at most 5 seconds warm and 15 seconds cold per invocation;
- at most 3,000 non-test production lines;
- zero linter-originated holds, prerequisite packages, lifecycle effects,
  writes outside the read-only contract, or commands outside the Git
  inspection allowlist; and
- no daemon, database, ledger, receipt, CI workflow, remote identity, recovery,
  or publication machinery.

Any missed critical obligation, write/execute violation, lifecycle effect,
noise excess, planning-time failure, second work interruption, or production
line-budget failure fires the stop-loss. Failure of any retention threshold
deletes the advisory path in this package; it does not hold modeling work or
authorize repair.

## Validation

The measurement artifacts must validate against the frozen cohort and scoring
schema. If the linter is retained, run its focused unit suite and hostile-input
proof. If deleted, prove the executable/source/tests are absent and operative
instructions point directly to the manual route.

Always run:

```text
markdown-doc lint --path \
  docs/work-packages/20260728-gate-planner-agent-friction-qualification-001
markdown-doc lint --path docs/work-packages/gate-planner-advisory-linter-roadmap.md
git diff --check
```

No full-workspace suite is selected prospectively because qualification is
package-local tooling/docs work. Exact-diff reconciliation may escalate direct
requirements if the stop-loss deletion changes a registered test surface.

## Security Impact

The linter receives no new permission. Trial repositories are disposable local
copies. The linter may perform only its ratified literal read-only Git
allowlist. Any unexpected write or execution is a stop-loss failure. Harvard
and CAL state are never opened, mounted, read, or executed.

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to comparison agents for the 18 paired cases, two
independent blinded scorers/reviewers, and two independent read-only terminal
verifiers. Expected outputs are raw anonymizable plans, timestamps, invocation
counts, linter outputs, compact metrics, findings, and PASS/HOLD
recommendations. Repository write access is read-only; disposable shared
clones and ignored `target/order5-qualification/**` result files are the only
participant writes. Only the parent edits tracked package artifacts or applies
the mechanical stop-loss deletion.

Subagent requirement: REQUIRED for paired trials, dual blinded scoring/review,
and dual verification. No comparison subagent may execute suggested modeling,
validation, CAL, or Harvard commands.

## Progress

- [x] 2026-07-28 — Scaffolded from accepted roadmap Order 5.
- [x] 2026-07-28 — Cohort, snapshots, obligation key, arm order, and schema
  frozen.
- [x] 2026-07-28 — Eighteen paired cases completed after invalid setup and
  label-inferable rounds were excluded and rerun.
- [x] 2026-07-28 — Dual blinded scoring, blind adjudication, unblinding, and
  metric computation completed.
- [x] 2026-07-28 — Stop-loss fired; advisory implementation deletion and
  manual-route reconciliation implemented.
- [x] 2026-07-28 — Focused validation passed after refreshing the direct
  authority map's bound strategy digest.
- [ ] Dual review and finding disposition passed.
- [ ] Dual verification and exact-diff closure passed.
- [ ] Prompt archived, catalogs closed, and completion committed.

## Surprises And Discoveries

- An initial shared-worktree approach escaped to the parent repository. A later
  malformed standalone clone temporarily detached the root and made the current
  linter path unavailable to other participants. The root was restored to
  `main` at the original exact head; every affected arm was discarded.
- The first blinded packet exposed arm provenance in 12 plans. The rubric
  invalidated those pairs, so the same agents repeated them from fresh clones
  with source-neutral plans before final scoring.
- Historical detached snapshots caused one non-actionable deterministic
  finding in every case. The linter also missed package-specific obligations
  that the frozen evaluator key required.

## Decision Log

- 2026-07-28: Historical package states will be reconstructed in disposable
  local clones. Both arms in a pair receive the same bytes; the current linter
  source is invoked externally and is never installed into the trial snapshot.
- 2026-07-28: A threshold miss causes deletion, not a repair package. This
  preserves the roadmap's measured-utility rule and prevents recursive tooling
  work.
- 2026-07-28: Added the tool README and registered authority contract to the
  terminal write set because exact-diff reconciliation showed both would become
  stale after deletion.
- 2026-07-28: Final valid results recorded four retention-threshold failures.
  The 24 linter-arm critical omissions independently require deletion. The
  33.3% noise result is dominated by trial-imposed detached HEADs, and timing/
  interaction results are sequence-confounded because the arm order was not
  interleaved; those limitations are retained rather than generalized.

## Outcomes And Retrospective

The stop-loss selected `DELETE_ADVISORY_LINTER` on the independently sufficient
zero-omission criterion. Cold/warm latency and production size passed, and no
linter-originated lifecycle/write/execute violation occurred. The manual route
remains prospective; no repair package or modeling prerequisite was created.
