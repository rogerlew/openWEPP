# Current-State Capability Inventory

Evidence class: **Static**. No planner, TESTGATE, CAL, CI, or model command ran.

## Measured Surface

- `crates/openwepp-gate-planner/src/`: 38,014 Rust lines.
- `tools/local_ci/testgate.py`: 1,228 Python lines.
- Combined Rust/controller surface: 39,242 lines. This is not a production-LOC
  claim because implementation files contain inline tests.
- `gate-policy/v1/`: 66 files, including 21 schemas and 40 fixtures, totaling
  6,048 JSON lines.
- `.github/workflows/testgate-shadow.yml`: 592 lines. Four related workflows
  total 1,189 YAML lines.

The target advisory budget is at most 3,000 non-test production lines.

## Current CLI

`crates/openwepp-gate-planner/src/main.rs` exposes 15 commands:

1. `plan`
2. `run`
3. `verify-receipt`
4. `verify-receipt-envelope`
5. `verify-ledger`
6. `verify-assurance`
7. `reconcile`
8. `pre-heavy-audit`
9. `validate-package`
10. `validate-package-chain`
11. `reconcile-attempts`
12. `close-tooling-defect`
13. `run-external-transition`
14. `verify-external-transaction`
15. `publish-external-results`

The CLI maps `FAIL`, `BLOCKED`, and `INVALID` outcomes to nonzero exits. `run`
spawns planned processes and persists receipts. Staged transitions execute
LIGHT and HEAVY phases, construct readiness audit state, verify results, and
append durable attempts. These are authority and execution behaviors, not
linter behaviors.

## Capability Groups

| Group | Current capability | Evidence |
| --- | --- | --- |
| Static planning | Repository observation, policy mapping, package validation, dependency and Nextest inventory | `lib.rs`, `planner.rs`, `policy.rs`, `repository.rs`, `package_validation.rs` |
| Execution | Planned subprocess spawning, temporary execution roots, Nextest execution | `executor.rs`, `execution_context.rs`, `execution_temp.rs`, `execution_nextest.rs` |
| Lifecycle | LIGHT/HEAVY transition, pre-heavy audit, resume, reconciliation, durable attempts | `pre_heavy.rs`, `resume.rs`, `ledger.rs`, `main.rs` |
| Evidence authority | Receipt/envelope/ledger/assurance verification and attestation predicates | `verifier.rs`, `assurance.rs`, `checkpoint_mirror.rs`, `tools/local_ci/testgate.py` |
| External transactions | Arbitrary plan-declared executable spawning, output staging, exclusive durable receipts | `external_dag.rs`, `external_outputs.rs` |
| Publication/recovery | Journaled staging, publish, recover-complete, and restore | `publication.rs` |
| CI/runner | Trusted forest1 execution, history restore, receipt reconstruction, attestation | `.github/workflows/testgate-shadow.yml` |

The crate-level comment in `lib.rs` describes a shadow-only, non-mutating
planner, but the exported and invoked capabilities materially exceed that
description.

## Controller And CI Consumers

`tools/local_ci/testgate.py` rejects zero-work and non-ready package chains;
writes intent and terminal plans, receipts, pre-heavy audits, authority state,
observations, and attestation predicates; and invokes planner execution for
LIGHT and HEAVY phases.

`.github/workflows/testgate-shadow.yml` builds the crate, restores and verifies
durable history, executes the Python controller, reconstructs receipts, and
verifies envelopes. `quality-observatory.yml` consumes successful TESTGATE
identity. The conservative workflow is a direct broad-command rollback path
and does not require preservation as a planner consumer.

## CAL Coupling

The CAL-04B package contains about 11,418 Python/Rust tool lines.
`execute-prefix.py` builds the gate planner, invokes
`run-external-transition`, and verifies the external transaction.
`publish-results.py` invokes `publish-external-results`. Selected adapter-facing
planner files grew by approximately 10,443 net lines during the external-DAG
adapter era.

CAL therefore consumes the current crate as a transaction executor and
publisher. This coupling must be removed before those capabilities are deleted.
The scientific package and Harvard custody cannot be made dependent on the
future linter.

## Finding

The current product is a planning, execution, governance, evidence, CI,
external-transaction, and publication system. It cannot be reduced to an
advisory linter through naming or exit-code changes. The advisory core must be
isolated or rewritten, and every non-advisory consumer must be migrated or
retired.
