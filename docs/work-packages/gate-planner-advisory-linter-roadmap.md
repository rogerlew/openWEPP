# Gate Planner Advisory-Linter Re-conceptualization Roadmap

Status: `COMPLETE / ORDER 5 STOP-LOSS APPLIED`

Owner: maintainers

Direction: user instruction on 2026-07-27. The existing gate planner is a
tool, not an authority. It must inform agents without permitting, prohibiting,
or executing their work.

Outcome: Order 5 found that the advisory implementation missed four mandatory
retention thresholds. The command, source, tests, and prospective usage
guidance were deleted on 2026-07-28. The manual route is the only prospective
validation-planning route; reintroduction requires explicit user authorization.

## Decision Statement

The gate planner will be re-conceptualized as a read-only advisory linter. It
will inspect repository state, work-package intent, changed paths, and
validation policy; explain likely obligations and inconsistencies; and suggest
canonical commands. The agent remains the decision-maker and command operator.
Canonical requirements govern truthful closure; tests and retained evidence
demonstrate whether those requirements were satisfied; and the authorized agent
or maintainer dispositions the work.

The linter itself can never place a package or campaign in `HOLD`, `BLOCKED`,
`READY`, `PASS`, or another lifecycle state. Linter failure means only that
automated planning assistance is unavailable. Linter availability never
changes authority or lifecycle status. When an underlying canonical
requirement is genuinely ambiguous, that requirement—not linter failure—may
require clarification.

## Philosophy

The following rules are non-negotiable and must be stated consistently in the
ADR, standards, agent guidance, interface contract, and implementation
packages:

1. **Advisory, not authoritative.** Findings inform agent judgment. They are
   not execution permission and cannot stop originating work.
2. **Read-only.** The linter does not modify source, package status, ledgers,
   receipts, evidence, queues, or protected data.
3. **No workflow execution.** It never launches validation, suggested,
   package-declared, workflow, remote, or user-controlled commands. Only the
   closed literal read-only Git inspection allowlist may run.
4. **No CI role.** It has no workflow dispatch, runner identity, concurrency,
   attestation, artifact promotion, hosted verification, or trust elevation.
5. **No lifecycle ownership.** It has no LIGHT/HEAVY transition, pre-heavy
   authorization, resume, recovery, publication, closeout, or tooling-defect
   state machine.
6. **No self-blocking dependency.** A linter defect is ordinary tooling debt.
   It is never a prerequisite to continued modeling and never changes package
   lifecycle state. Agents use the manual route to determine and run canonical
   requirements directly.
7. **Explainable output.** Every finding cites the observed path or declaration,
   applicable repository rule, rationale, and suggested agent action. Unknowns
   are visible; they are not silently converted into broad execution.
8. **Underlying obligations remain real.** Removing planner authority does not
   waive contract tests, authority suites, conservation proofs, consumer-path
   evidence, exact-diff reconciliation, or protected Harvard controls.
9. **Measured utility or deletion.** The linter is retained only if it reduces
   agent planning effort in representative modeling work. Complexity added to
   defend the linter is counted against its value.

## Terms

An **advisory linter** is a deterministic, read-only program that reports
findings and suggestions. A **requirement** is an obligation established by
repository governance, a science contract, or a package—not by the linter.
**Evidence** is the retained result of a command an agent actually ran. A
**finding** is the linter's explanation that a declaration may be missing,
inconsistent, excessive, or relevant. Findings are not evidence.

## Historical Target Operator Contract

The qualification target had one neutral, non-gate invocation:

    tools/validation/workplan-lint \
      --package docs/work-packages/<id>/package.md

The linter supports three observation modes:

- `pre-edit`: analyze declared intent and write set before implementation;
- `working-tree`: analyze the index, tracked worktree changes, and untracked
  paths during an edit loop; and
- `terminal`: analyze the exact declared base-to-HEAD diff plus any remaining
  dirty paths before disposition.

Every output states its mode, repository root, observed base and head, index/
worktree/untracked inclusion, package path and identity, and completeness.
Missing or ambiguous base, package, or boundary declarations produce findings;
the tool never guesses or supplies identity defaults. Detached HEAD and
multiple candidate packages are reported explicitly.

Human-readable output is the default. A deterministic JSON representation may
be requested on standard output for editor or agent integration. Neither mode
writes repository files.

A completed analysis exits zero regardless of findings. Nonzero exit codes
describe only invocation misuse or internal/unavailable analysis; no caller may
interpret them as package or campaign authority. Top-level analysis status is
`complete`, `partial`, or `unavailable`, and partial output enumerates every
analysis that could not run.

Each finding contains:

- a stable rule identifier;
- category: `declaration-conflict`, `scope-mismatch`, `missing-mapping`,
  `excessive-validation`, `relevant-obligation`, `suggested-command`, or
  `unknown`;
- confidence: `deterministic` or `heuristic`;
- impact: `high`, `medium`, or `low`;
- action: `inspect`, `amend-declaration`, or `consider-command`;
- concise message;
- observed file and location when applicable;
- governing source and applicability boundary;
- reasoning; and
- suggested canonical command when mechanically known, including exact argv,
  working directory, affected surface, governing citation, and expected cost
  class.

An internal linter failure identifies the unavailable analysis and points the
agent to the manual governing sources. It does not mutate or reclassify the
originating work.

## Architectural Boundary

The advisory core may retain only functionality needed to read repository
state, map changes to declared policy, analyze and report work-package
declaration consistency, and render findings. Execution, receipts, trust,
custody, recovery, publication, assurance lifecycle, and CI orchestration are
outside the product boundary.

Repository inspection is limited to direct read-only filesystem access and the
literal argument-vector `git` allowlist ratified by Order 0. Cargo dependency
metadata is parsed in process from repository manifests and the lockfile. Shell
execution, package-derived executable selection, suggested-command execution,
tests, builds, network access, remote calls, and commands that write repository
or tool state are prohibited.

Scientific campaign execution remains package-owned and agent-operated.
Harvard sealing and single-open custody remain a separate protected-data
mechanism. General testing guidance remains canonical documentation. None of
those concerns may be imported into the linter to make its advice
authoritative.

## Ordered Packages

No implementation child may be scaffolded until Order 0 closes. The package
names after Order 0 are prospective decomposition labels, not execution
authority.

| Order | State | Prospective package | Outcome | Dependency |
| --- | --- | --- | --- | --- |
| 0 | `complete` | `20260727-gate-planner-advisory-linter-roadmap-001` | Ratified the philosophy, ADR, conflict inventory, target interface, migration/deletion map, value metrics, and decomposition with three independent `GO` reviews. Documentation only. | User direction |
| 1 | `complete` | `20260727-gate-planner-governance-authority-alignment-001` | Applied ADR-0043 to operative guidance, direct governance guards, historical identity, and frozen package statuses while preserving underlying correctness and protected-data obligations. | User authorization |
| 2 | `complete` | `20260727-gate-planner-cal04b-legacy-integration-removal-001` | Removed obsolete planner/external-transaction dependencies from prospective CAL tooling while preserving direct execution, durable primary-failure evidence, Harvard sealing, and the freeze/open barrier. | Completed Order 1 and explicit user authorization |
| 3 | `complete` | `20260727-gate-planner-advisory-linter-thin-slice-001` | Delivered the read-only lint command and proved representative findings, hostile-input refusal, bounded capture, and no execution or mutation with dual review and verification. | Orders 1-2 and explicit user authorization |
| 4 | `complete` | `20260727-gate-planner-legacy-execution-ci-retirement-001` | Retired the planner execution/CI/control plane after a 31-row consumer inventory; preserved direct authority, immutable policy identity, optional quality observation, and protected-data ownership without a linter CI role. | Order 3 and explicit user authorization |
| 5 | `complete` | `20260728-gate-planner-agent-friction-qualification-001` | Applied the stop-loss after both blinded scorers confirmed 24 linter-arm critical omissions. Deleted the advisory implementation and passed dual review and verification. The noise and efficiency measurements are retained with their trial-design limitations. | Orders 3-4 and explicit user authorization |

CAL-04B direct, package-authored modeling work may resume independently under
its existing authority and the user's direction. It does not wait for any
roadmap order or linter implementation. Order 2 removes obsolete integration;
it does not grant permission to execute CAL.

## Order 0 Required Decisions

The planning package must settle these implementation choices before child
scaffolds are allowed:

- whether the advisory core is extracted from existing static analysis or
  rewritten without legacy dependencies;
- the exact finding schema and inspection allowlist;
- the canonical policy inputs and how citations remain stable;
- the legacy source, workflow, schema, and documentation surfaces to delete,
  freeze for historical verification, or migrate;
- the protected Harvard boundary after general transaction/publication code is
  removed;
- how agents record commands and evidence without recreating a receipt
  authority system;
- the production-line and maintenance budgets for the retained linter; and
- the downstream package write sets, dependencies, rollback points, and proof
  obligations.

## Functional Acceptance

These were the prospectively frozen qualification criteria. Order 5 failed
criterion 7 and executed criterion 10:

1. An agent can run one read-only lint command in `pre-edit`, `working-tree`,
   and `terminal` mode and receive useful, cited findings and command
   suggestions.
2. The linter performs only the ratified read-only inspection allowlist and
   writes no repository, evidence, or lifecycle state.
3. Linter unavailability or an injected internal error leaves the agent able to
   continue through the documented manual route.
4. No linter output, error, or missing output can create or alter `HOLD`,
   `BLOCKED`, `READY`, `PASS`, package status, campaign status, or Harvard
   custody state.
5. CAL-04B executes independently of the linter and preserves primary
   scientific failures before cleanup or closeout.
6. Existing test and science obligations remain discoverable and executable
   without the linter.
7. The qualification cohort contains at least six real packages: two
   documentation, two non-kernel Rust, and two kernel/science packages,
   including one calibration package. Each is measured in all three observation
   modes. The linter must omit zero reviewer-confirmed critical obligations,
   produce no more than 10% non-actionable deterministic findings, reduce
   median planning time by at least 30%, and reduce plan-construction
   interactions by at least 50% against the recorded manual baseline.
8. One invocation completes within 5 seconds warm and 15 seconds cold on the
   reference development host, excluding time spent by the agent running
   suggested commands.
9. The retained advisory implementation is at most 3,000 non-test production
   lines, introduces no daemon, database, ledger, receipt, or CI workflow, and
   creates zero planner-originated holds or prerequisite packages.
10. Missing any critical obligation, writing or executing outside the
    inspection allowlist, creating a lifecycle effect, exceeding the noise
    threshold, or failing to improve median planning time disables the new
    path. Further work requires explicit user authorization rather than an
    automatic repair campaign.

## Manual Route

The manual route is the only prospective route:

1. Run `tools/agents/find-agents --for <intended-write-paths>` and read the
   applicable instruction chain.
2. Read the work package's intent, boundary, declared write set, and canonical
   testing strategy.
3. Inspect the exact base, index, tracked worktree changes, and untracked paths.
4. Map affected surfaces to canonical focused tests, authority suites,
   consumer/conservation checks, and boundary-specific broad checks.
5. Run selected commands directly from the documented working directory.
6. Record the exact command, source identity, result, evidence class, and any
   unmet governing requirement in package artifacts.
7. Continue the originating work. Do not open a validation-tool prerequisite.

A known unmet governing requirement may prevent truthful package closure.
Agent discretion may not waive or downgrade a science, validation, security,
protected-data, or package requirement.

## Stop-Loss And Prohibited Outcomes

The advisory implementation was deleted because it missed the frozen utility
and noise thresholds. It created no command execution, persistent lifecycle
state, CI integration, remote identity, receipt, recovery, publication, or
protected-data custody.

The stop-loss governed the tooling experiment only and never stopped
originating modeling. Restart or expansion requires explicit user
authorization.

The following outcomes are explicitly rejected:

- a smaller TESTGATE executor;
- a linter that emits execution authorization;
- a compatibility wrapper around the current transaction system;
- a CI workflow that runs or certifies the linter;
- a new receipt, ledger, attestation, or recovery format;
- planner-owned CAL or Harvard behavior; and
- implementation scaffolds created before Order 0 review closes.

## Current-State Disposition

All gate-planner closeout and prerequisite execution remains frozen as
non-prospective work. The legacy control plane and failed advisory
implementation are deleted; retained historical artifacts keep their original
meaning. Manual validation and direct modeling remain independent. CAL-04B's
scientific failure remains owned by CAL-04B, not by this roadmap.
