# CAL-04B Legacy Integration Removal

Package ID:
`20260727-gate-planner-cal04b-legacy-integration-removal-001`

Queue ID: `GATE-LINT-CAL-01`

Status: `ACTIVE`

Authorization: the user's 2026-07-27 direction to scaffold and execute roadmap
Order 2.

Execution mode: `package-end-to-end`.

This ExecPlan is maintained under `docs/codex_exec_plans.md`.

## Objective

Remove CAL-04B's prospective dependency on gate-planner external transactions
and publication while preserving direct package command execution, immediate
durable primary-failure recording, and the complete Harvard freeze/open custody
barrier required by ADR-0043 Decision 10.

## Included Scope

- Replace the planner-launching CAL prefix coordinator with a package-local
  direct argv executor.
- Replace planner publication with bounded atomic publication of package-owned
  result files.
- Reduce freeze verification to two independent read-only verifier PASS records
  without planner capabilities, dispatch claims, or attestations.
- Make the holdout launch use an OS-enforced read-only repository/frozen-input
  boundary and a separate writable holdout-output root.
- Preserve the durable `OPENED_ONCE` transition before first Harvard content
  read and forbid rerun after a post-open failure.
- Retain command-level evidence and the first failure before cleanup or
  publication.
- Record the unreported 2026-07-27 synthetic-reconstruction failure as
  incident 005 and make it the current CAL science blocker.

## Excluded Scope

- No CAL population, freeze, verifier, Harvard, holdout, publication, or model
  execution against real package data.
- No repair of the synthetic-recovery science-design failure.
- No model, kernel, science-contract, calibration-domain, observation,
  objective, acceptance-rule, or protected-fixture change.
- No gate-planner crate, CI workflow, or legacy global planner deletion.
- No roadmap Order 3 scaffold.

## Declared Write Set

- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/gate-planner-advisory-linter-roadmap.md`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/package.md`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/direct-execution-plan.json`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/execution-control-contract.md`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/executor-command-plan.csv`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/observed-command-contract.csv`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/observed-execution-procedure.md`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/external-dag-path-equivalence.md`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/execution-incident-005.md`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/stage-status-ledger.csv`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/execute-prefix.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/publish-results.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/custody.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/freeze.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/freeze-verify.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/holdout.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/validate_preopen.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_execute_prefix.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_publish_results.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_freeze_custody.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_external_paths.py`
- this package subtree

The untracked
`docs/audits/20260727_gate_planner_demotion_readiness_audit.md` remains
read-only, unstaged, and outside this package.

## Protected Boundaries

- Harvard remains sealed throughout this package.
- No test may read Harvard content. Custody tests use temporary synthetic files.
- The old external plan and retained attempts remain immutable historical
  evidence and are not executed, rewritten, published, or deleted.
- The direct executor may run only literal package-defined argv; it does not
  accept executable commands from user-controlled or package-result data.
- A command failure is recorded and fsynced before the executor returns.
- Cleanup never deletes the primary failure record.

## Phase Plan

1. Scaffold and commit the autonomous package.
2. Implement direct command execution and primary-failure durability.
3. Migrate result publication and the minimal custody owner.
4. Add focused fail-closed and no-planner/no-Harvard tests.
5. Record incident 005 and reconcile CAL/package status.
6. Run focused validation, dual review, finding disposition, dual verification,
   exact-diff reconciliation, prompt archival, and closure.

## Direct Validation

```text
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python -m unittest \
  docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_execute_prefix.py \
  docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_publish_results.py \
  docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_freeze_custody.py \
  docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_external_paths.py
markdown-doc lint --path docs/work-packages/20260727-gate-planner-cal04b-legacy-integration-removal-001
markdown-doc lint --path docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001
git diff --check
```

Also parse the direct JSON plan, scan prospective CAL tools for planner/external
transaction imports, prove failure evidence survives cleanup, exercise the
synthetic custody barrier without Harvard content, and reconcile every changed
path to this write set.

## Acceptance

- A fresh direct calibration attempt can run literal package commands without
  planner state, TESTGATE, receipts, ledgers, transitions, or CI.
- The first failed command's argv, source identity, exit status, timestamps,
  stdout/stderr paths and hashes are durably retained before return.
- Publication is bounded to package-owned result files and cannot erase primary
  failure evidence.
- The minimal custody owner enforces all seven ADR-0043 Harvard properties.
- No prospective CAL tool imports or launches the gate planner.
- Incident 005 is retained and CAL truthfully holds at the synthetic recovery
  science-design failure, not at tooling.
- Dual reviews, finding disposition, dual verification, line-count governance,
  and exact-diff reconciliation pass.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only reviewers and two independent
read-only verifiers for direct-executor safety, primary-failure durability,
publication confinement, custody invariants, no-planner dependency, status
truthfulness, and exact-diff closure. Expected outputs are compact findings and
command-backed verification records. Write access is read-only.

## Progress

- [x] 2026-07-27 — Scaffolded from accepted roadmap Order 2.
- [ ] Direct executor and failure durability implemented.
- [ ] Publication and custody migration implemented.
- [ ] Incident/status reconciliation completed.
- [ ] Focused validation passed.
- [ ] Dual review and dual verification passed.
- [ ] Prompt archived, catalogs closed, and package committed.

## Decision Log

- 2026-07-27: Order 2 migrates only CAL integration/custody. It does not execute
  or repair CAL science.
- 2026-07-27: `bubblewrap` is the required fail-closed holdout sandbox; absence
  is an explicit custody failure, never a permissive fallback.

## Outcomes

Populate after implementation and verification.
