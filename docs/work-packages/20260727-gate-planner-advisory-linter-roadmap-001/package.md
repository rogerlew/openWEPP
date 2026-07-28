# Gate Planner Advisory-Linter Roadmap Authoring

Package ID:
`20260727-gate-planner-advisory-linter-roadmap-001`

Queue ID: `GATE-LINT-ROADMAP-01`

Status: `QUEUED / PLANNING ONLY`

Authorization: the user's 2026-07-27 direction to roadmap the gate planner's
re-conceptualization, make the philosophy unambiguous, and make the first work
package planning-only.

This ExecPlan is maintained under `docs/codex_exec_plans.md`.

## Purpose

The existing gate planner crossed the boundary from agent-assistance tool to
execution and lifecycle authority. This package establishes the binding
architecture and migration plan that returns it to a read-only advisory linter
before any implementation work is authorized.

After this package closes, a contributor must be able to understand from the
ADR, roadmap, and interface contract exactly what the linter may do, what it
must never do, how agents proceed when it is wrong or unavailable, which
legacy surfaces will be retired, and how later packages will prove reduced
friction.

## Progress

- [x] 2026-07-27 20:00 PDT — Record the user-directed planning-only boundary.
- [x] 2026-07-27 20:00 PDT — Create the initial roadmap and Order-0 package
      scaffold.
- [ ] Record the complete current-state capability and authority inventory.
- [ ] Author and ratify ADR-0043.
- [ ] Reconcile every conflicting governance statement.
- [ ] Freeze the target interface, finding model, failure semantics, and manual
      fallback.
- [ ] Freeze the migration, quarantine, deletion, and historical-verification
      boundaries.
- [ ] Define the friction baseline, budgets, and deletion triggers.
- [ ] Obtain three independent reviews and disposition every finding.
- [ ] Finalize the downstream package decomposition without implementing or
      scaffolding a child package.
- [ ] Validate documentation and close the planning package.

## Binding Philosophy

The gate planner is a tool. Its sole product is information that helps an agent
plan validation. It has no authority over execution, progress, evidence,
package lifecycle, scientific disposition, protected data, or closeout.

The target is a deterministic, read-only advisory linter. It reports cited
findings and suggested commands. It never launches those commands or writes
lifecycle state. A linter defect or unavailable analysis never changes
authority or lifecycle state and cannot block originating work. The agent uses
the documented manual route to determine and execute canonical requirements
directly.

Underlying repository requirements remain binding because their authoritative
documents or science contracts require them—not because the linter selected
them.

## Objective

Produce a reviewed and internally consistent decision set comprising:

- ADR-0043, with a clause-level disposition of all ADR-0039 Decisions 1-15,
  ADR-0040 Decisions 1-8, ADR-0041 Decisions 1-10, and their
  rejected-alternative language. ADR-0039 Decisions 1-2 must split retained
  canonical-requirement and validation-lifecycle authority from superseded
  planner/receipt/CI authority. The disposition must separately preserve the
  correctness/science substance of ADR-0039 Decisions 4, 6, 7, 11, and 12 and
  the quality/history substance of ADR-0041 Decisions 4, 6-8, and 10;
- the final advisory-linter roadmap;
- a policy-conflict matrix and exact amendment plan;
- a target operator and machine-readable finding contract;
- a current-to-target capability inventory;
- a migration, quarantine, deletion, and historical-evidence map;
- a manual fallback and agent-responsibility contract;
- measurable friction, complexity, and stop-loss criteria; and
- a reviewed downstream package sequence whose implementation children remain
  unscaffolded until this package closes.

## Included Scope

- Static inspection of gate-planner code, local controller, workflows, schemas,
  current policy, ADRs, work-package governance, and retained trajectory
  evidence.
- Documentation-only authorship of ADR-0043 plus exact proposed patches for
  later operative-governance alignment.
- Precise definitions for `tool`, `advisory linter`, `finding`, `requirement`,
  `evidence`, `manual fallback`, and `nonblocking`.
- Target CLI, output vocabulary, finding fields, failure behavior, performance
  budget, and read-only guarantees.
- Exact classification of legacy surfaces as `RETAIN_ADVISORY`,
  `MIGRATE`, `FREEZE_HISTORICAL_VERIFY`, or `DELETE`.
- Separation of linter advice, agent execution, scientific campaigns, local
  evidence, and Harvard custody.
- Prospective downstream package objectives, dependencies, write-set
  boundaries, acceptance, rollback, and deletion triggers.
- Three independent read-only reviews and explicit finding disposition.

## Excluded Scope

- Rust, Python, shell, workflow, schema, fixture, or test implementation.
- Creation of downstream implementation-package directories.
- Running TESTGATE, full-workspace tests, CAL commands, populations, CI,
  forest1 work, comparators, or publication.
- Opening Harvard, changing custody tokens, or accessing protected results.
- Deleting or rewriting existing code, receipts, ledgers, artifacts, or
  historical evidence.
- Claiming that the target architecture is implemented before its downstream
  packages close.

## Declared Write Set

- `docs/ROADMAP.md`
- `docs/decisions/0043-gate-planner-is-a-non-authoritative-advisory-linter.md`
- `docs/decisions/README.md`
- `docs/work-packages/README.md`
- `docs/work-packages/gate-planner-advisory-linter-roadmap.md`
- `docs/work-packages/20260727-gate-planner-advisory-linter-roadmap-001/**`

No executable source, workflow, schema, fixture, science contract, calibration
artifact, operative `AGENTS.md`/standard/tool guidance, or existing
work-package evidence is writable.

## Required Deliverables

- `artifacts/current-state-capability-inventory.md`
- `artifacts/philosophy-and-normative-contract.md`
- `artifacts/policy-conflict-matrix.md`
- `artifacts/target-interface-and-finding-contract.md`
- `artifacts/manual-fallback-and-agent-responsibility.md`
- `artifacts/migration-quarantine-deletion-map.md`
- `artifacts/friction-baseline-and-success-metrics.md`
- `artifacts/downstream-package-decomposition.md`
- `artifacts/review-philosophy.md`
- `artifacts/review-operator-contract.md`
- `artifacts/review-governance-and-protected-boundaries.md`
- `artifacts/finding-disposition.md`
- `artifacts/required-reading-map.md`
- `artifacts/final-disposition.md`

## Phase Plan

### Phase A — Intake And Baseline

Record the applicable instructions and read the current ADRs, standards,
planner/controller entrypoints, workflow surfaces, schemas, CAL adapter,
trajectory assessment, and recent failure evidence. Build the capability
inventory and measure current operator inputs, persisted artifacts, production
surface, failure semantics, and observed wall time.

### Phase B — Philosophy And Authority

Author ADR-0043 and the package-local normative contract. Produce exact
clause-level amendments for Order 1 so no operative document will describe the
linter as authoritative, executing, blocking, certifying, admitting, or
trusted. Explicitly preserve the authority of tests, science contracts,
package requirements, and Harvard custody. Order 0 does not apply those
operative guidance patches.

### Phase C — Target Product Contract

Freeze the read-only CLI, three observation modes, inspection allowlist,
finding vocabulary, citation requirements, deterministic output, exit
semantics, partial/internal-error behavior, manual fallback, quantitative
performance/value budget, maintenance budget, and deletion triggers.
Demonstrate the intended output with static examples only.

### Phase D — Migration And Deletion Design

Classify every current planner, executor, verifier, controller, workflow,
schema, receipt, ledger, recovery, external-DAG, publication, assurance, and
CAL integration surface. Name exact consumers and decide whether each surface
is retained for advisory analysis, migrated, frozen only to verify historical
evidence, or deleted.

Before any CAL transaction/publication surface is classified for deletion, bind
the replacement owner for the nonempty freeze, two independent read-only
verifier PASS receipts, durable `OPENED_ONCE` transition before the first
Harvard content read, digest and lock checks, no rerun after a post-open crash,
Harvard read-only access, and the exact global invariant that the holdout
process has no calibration-output write capability or path anywhere. Bind the
exact owning code and commands before deletion classification.

### Phase E — Roadmap And Review

Finalize downstream package boundaries without scaffolding them. Obtain three
independent reviews:

1. philosophy and authority review;
2. agent/operator usability and failure-path review; and
3. governance, science-obligation, and protected-Harvard boundary review.

Disposition every finding as `accepted`, `rejected`, `deferred`, or
`follow-up`. Accepted findings are incorporated and rechecked. Any ambiguity
about planner authority blocks this planning package from closing.

### Phase F — Documentation Verification

Run scoped documentation lint, path/reference checks, exact-write-set
reconciliation, and static assertions that prohibited authority vocabulary is
not reintroduced into the target contract. Confirm the diff contains no
executable, workflow, schema, fixture, science, calibration-result, or
protected-data change.

## Planning Gates

- `markdown-doc lint --path
  docs/work-packages/20260727-gate-planner-advisory-linter-roadmap-001`
- `markdown-doc lint --path
  docs/work-packages/gate-planner-advisory-linter-roadmap.md`
- documentation lint for each amended authority document;
- `git diff --check`;
- exact declared-write-set reconciliation;
- link/reference validation for ADR, roadmap, and catalog entries;
- static review that target documents contain no linter-owned lifecycle or
  execution authority;
- three independent reviews plus complete finding disposition; and
- explicit confirmation that no heavy gate or executable workflow ran.

No TESTGATE, CI, full workspace, comparator, coverage, CAL population, or
Harvard gate is selected by this documentation-only planning package.

## Exit Criteria

- ADR-0043, the roadmap, catalog, and package artifacts state the advisory-only
  philosophy consistently and without transition ambiguity; the conflict
  matrix provides exact Order-1 patches for still-operative guidance.
- The roadmap and package-local normative contract distinguish underlying
  requirements from linter findings.
- Linter failure is explicitly nonblocking and has a usable manual path.
- The target product owns no execution, CI, lifecycle, evidence, recovery,
  publication, CAL, or Harvard authority.
- Every current capability has a reviewed target disposition and named
  consumer.
- Friction and complexity budgets are measurable and have mandatory deletion
  or user-review triggers.
- Downstream packages are coherent and ordered, but no child implementation
  scaffold exists.
- All review findings are dispositioned and all documentation checks pass.

## Security And Science Impact

This package changes architecture/governance documentation only. It must not
weaken source-mutation protection, command transparency, canonical science
tests, external-authority suites, conservation evidence, or Harvard
seal/open-once controls. It removes planner authority while preserving the
underlying obligation and the agent's duty to execute and report relevant
evidence truthfully.

No protected data access is permitted.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to three independent read-only reviewers for philosophy and
authority, operator/interface and failure behavior, and governance/science/
Harvard-boundary review. Expected outputs are compact findings with exact paths
and recommended dispositions. Write access is read-only. No heavy runner or
comparator subagent is authorized or required.

## Decision Log

- 2026-07-27: The user directed that the first package be planning-only because
  the philosophy must be unambiguous before implementation.
- 2026-07-27: The target is an advisory linter, not a smaller executor or local
  gate.
- 2026-07-27: Downstream implementation packages may be described but not
  scaffolded until Order 0 closes.

## Surprises And Discoveries

- 2026-07-27: Independent scaffold review found that retaining `gate` and
  `local_ci` in the target command name would preserve the rejected concept
  even if behavior were reduced.
- 2026-07-27: The current roadmap simultaneously stated the new advisory
  redirect and the historical TESTGATE authority. The historical text requires
  explicit labeling now and clause-level disposition in Order 0.
- 2026-07-27: A planning-only package must author the ADR and exact operative
  amendment plan without itself applying Order-1 changes to `AGENTS.md`,
  standards, or tool guidance.

## Outcomes And Retrospective

Populate only after the ADR, roadmap, reviews, finding disposition, and
documentation verification are complete. This section is not implementation
evidence.
