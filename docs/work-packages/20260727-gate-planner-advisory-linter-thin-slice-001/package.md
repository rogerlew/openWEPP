# Advisory Linter Thin Slice

Package ID:
`20260727-gate-planner-advisory-linter-thin-slice-001`

Queue ID: `GATE-LINT-ADV-01`

Status: `ACTIVE`

Authorization: the user's 2026-07-27 direction to scaffold and execute roadmap
Order 3.

Base commit: `72e433d16b4f9c35f2bb05cee8c7d92b1e16108d`

Execution mode: `package-end-to-end`.

This ExecPlan is maintained under `docs/codex_exec_plans.md`.

## Objective

Implement the frozen neutral `tools/validation/workplan-lint` interface as a
small read-only advisory linter. It reports deterministic, cited findings and
inert command suggestions in `pre-edit`, `working-tree`, and `terminal` modes
without running workflows, mutating repository state, or owning lifecycle
status.

## Implementation Intent

`agent-tooling / read-only advisory analysis`. The implementation adds no
runtime, model, science, publication, or lifecycle behavior.

## Included Scope

- A standalone Python command and package-local library under
  `tools/validation`.
- Exact frozen human and JSON result envelopes.
- Safe package declaration and repository-state inspection.
- The frozen literal Git read allowlist with a cleared environment, closed
  stdin, byte limits, timeouts, preflight config/attribute refusal, and
  validated operands.
- Initial useful findings for package identity/base, declared write-set drift,
  missing implementation intent, relevant Rust/Python/docs obligations, and
  inert canonical command suggestions.
- Adversarial temporary-repository tests for all prohibited config/attribute
  classes, helper nonexecution, no network, deterministic modes, partial and
  unavailable results, exit semantics, and byte/metadata non-mutation.
- Operator documentation and package closure evidence.

## Excluded Scope

- No import or dependency on the legacy gate-planner crate or its executor,
  receipt, ledger, CI, publication, recovery, CAL, or custody modules.
- No validation, test, build, formatter, suggested, package-declared, remote,
  workflow, network, hook, filter, helper, or shell execution.
- No repository, package, status, evidence, index, object, queue, or lifecycle
  writes.
- No CI integration, daemon, database, receipt, ledger, attestation, recovery,
  trust, or protected-data feature.
- No Order 4 scaffold or legacy deletion.
- No CAL, model, kernel, science, Harvard, freeze, or holdout execution.

## Declared Write Set

- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/gate-planner-advisory-linter-roadmap.md`
- `tools/validation/workplan-lint`
- `tools/validation/workplan_lint.py`
- `tools/validation/test_workplan_lint.py`
- `tools/validation/README.md`
- `tools/validation/fixtures/**`
- this package subtree

The untracked
`docs/audits/20260727_gate_planner_demotion_readiness_audit.md` remains
read-only, unstaged, and outside this package.

## Frozen Interface

```text
tools/validation/workplan-lint \
  --package docs/work-packages/<id>/package.md \
  --mode <pre-edit|working-tree|terminal> \
  [--format human|json]
```

There is no inferred package, base, mode, or repository identity. Completed
analysis exits zero regardless of findings. Partial/unavailable analysis exits
three. Invocation misuse exits two. These are availability signals only and
never package or campaign authority.

## Security And Read-only Boundary

- Direct file reads are normalized beneath the resolved repository root,
  no-follow, bounded-size regular-file reads.
- Package content may supply only a validated revision token and normalized
  repository-relative path operands.
- Git argv and child environment must match the frozen Order-0 contract.
- Repository config and applicable attributes are parsed before Git. Any
  prohibited declaration makes Git analysis unavailable before process launch.
- Tests monitor helper/network canaries plus repository, index, and object
  bytes and metadata around every supported mode and allowlisted invocation.

## Phase Plan

1. Scaffold and commit this autonomous package.
2. Implement the bounded reader, Git inspector, declaration parser, findings,
   and renderers.
3. Add representative and adversarial fixtures/tests for all three modes,
   schemas, failure states, deterministic ordering, no execution, and no write.
4. Document the operator/manual fallback boundary.
5. Run focused validation, dual independent review, finding disposition, dual
   verification, line-count governance, exact-diff reconciliation, prompt
   archival, catalog closure, and commit.

## Direct Validation

```text
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python \
  -m unittest tools/validation/test_workplan_lint.py
markdown-doc lint --path tools/validation
markdown-doc lint --path \
  docs/work-packages/20260727-gate-planner-advisory-linter-thin-slice-001
git diff --check
```

Also prove deterministic JSON/human output, complete/partial/unavailable/misuse
exit semantics, exact allowlisted argv, hostile config/attribute refusal before
Git launch, absence of helper/network execution, repository/index/object
byte-and-metadata stability, production line count at most 3,000, no legacy
planner import, and exact write-set reconciliation.

## Acceptance

- All three modes and frozen schema fields work deterministically.
- Findings are cited advice, never lifecycle verdicts or evidence.
- Completed analyses exit zero even when findings exist.
- Only frozen read-only Git argv shapes can execute; all other execution is
  structurally unavailable.
- Every hostile config and attribute class fails before Git process launch.
- Tests prove no repository, index, object, helper, network, or lifecycle
  mutation.
- An injected internal failure emits partial/unavailable output and leaves the
  documented manual route usable.
- The implementation is no more than 3,000 non-test production lines and adds
  no daemon, database, ledger, receipt, CI, or prerequisite mechanism.
- Dual reviews, finding disposition, dual verification, line-count governance,
  documentation checks, and exact-diff reconciliation pass.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only reviewers and two independent
read-only verifiers for interface/schema correctness, allowlist confinement,
adversarial no-execution/no-write proof, advisory semantics, useful findings,
manual fallback, line-count governance, and exact-diff closure. Expected
outputs are compact findings and command-backed verification results. Write
access is read-only.

## Progress

- [x] 2026-07-27 — Scaffolded from accepted roadmap Order 3.
- [x] 2026-07-27 — Thin slice implemented.
- [x] 2026-07-27 — Nineteen adversarial and representative tests passed.
- [x] 2026-07-27 — Documentation and manual fallback passed.
- [ ] Dual review and finding disposition passed.
- [ ] Dual verification and exact-diff closure passed.
- [ ] Prompt archived, catalogs closed, and completion committed.

## Decision Log

- 2026-07-27: Implement as a clean Python thin slice with no legacy crate
  dependency.
- 2026-07-27: Frozen allowlist may be narrowed but not expanded.

## Outcomes

Populate after implementation and verification.
