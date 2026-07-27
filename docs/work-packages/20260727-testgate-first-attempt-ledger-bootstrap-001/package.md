# TESTGATE First-Attempt Ledger Bootstrap

Package ID: `20260727-testgate-first-attempt-ledger-bootstrap-001`

Queue ID: `TESTGATE-LEDGER-BOOTSTRAP-01`

Status: `ACTIVE`

Authorization: the user's 2026-07-27 direction to complete CAL-04B and resolve
its assurance/TESTGATE blockers through reviewed work-package corrections.

This defect-closure ExecPlan is maintained under
`docs/defect_closure_execplans.md`.

## Objective

Make the canonical `testgate.py --execute` helper securely create and validate
a fresh durable history ledger before invoking the Rust transition preflight,
so a first authoritative attempt can reach LIGHT without a manual placeholder
file.

## Reproducer

At exact commit `2efbee531361639f0815820bc43c7506ae62eb12`, the
comparator-owned command used fresh absent paths:

- artifact root
  `/home/workdir/gate-auth11-test-provider-canonical-001`;
- ledger `/home/workdir/gate-auth11-test-provider-history.jsonl`.

Intent and terminal planning succeeded, but transition failed in 16 ms with
`GATE-CLI-INPUT: No such file or directory`. No LIGHT, audit, receipt, or HEAVY
artifact exists. `testgate.py` passes the absent ledger as `--resume`; Rust
`validate_transition_outputs` requires `resume` to be an existing regular file.
The helper currently creates/appends the ledger only after transition returns.

An operator-launched second root used the wrong campaign identity and was
terminated during intent planning. It is invalid, unexecuted evidence and is
not a retry authorization.

## Included Scope

- a no-follow, create-once durable ledger bootstrap before heavy transition;
- preservation and chain validation of an existing regular ledger;
- rejection of symlink, directory, non-regular, and malformed existing ledger
  inputs;
- focused Python regression tests for fresh, existing, malformed, and symlink
  cases;
- canonical exact-head admission and one comparator-owned execution after dual
  review.

## Excluded Scope

- weakening Rust transition input validation;
- changing ledger record schema, predecessor chaining, retry policy, recovery,
  planner selection, executor behavior, or CAL science;
- manual ledger placeholders, manual gate injection, Harvard access,
  deployment, or release.

## Declared Write Set

- `tools/local_ci/testgate.py`
- `tools/local_ci/test_testgate.py`
- `tools/local_ci/README.md`
- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/20260727-gate-planner-auth11-fixed-inventory-test-provider-001/artifacts/implementation-gates.md`
- `docs/work-packages/20260727-testgate-first-attempt-ledger-bootstrap-001/**`

No other path is writable. This write set must not widen.

## Execution Plan

1. Commit this prospective scaffold and obtain two independent read-only
   scaffold reviews before tooling edits.
2. Add the smallest secure bootstrap helper and focused tests.
3. Run Python tests, compile check, diff hygiene, documentation lint, focused
   gate-planner/authority checks, and full planner 227/227.
4. Commit one exact implementation state and obtain dual implementation review
   with explicit finding disposition.
5. Obtain dual terminal verification of the exact diff and failed-attempt
   retention.
6. Delegate one fresh exact-head canonical transaction to
   `comparator_suite_runner`; require READY audit, valid receipt/ledger, every
   selected LIGHT/HEAVY node PASS, and two receipt verifiers.
7. Close this package and resume AUTH11/external-DAG/CAL execution.

## Acceptance

- A fresh ledger is created as a regular file with no-follow, exclusive-create
  semantics before `--stage transition` is invoked.
- Existing regular ledgers are not truncated or replaced and pass strict
  predecessor/hash-chain validation before reuse.
- Symlink, directory, non-regular, and malformed existing ledgers fail before
  transition.
- Bootstrap failure is typed as a helper failure and does not launch gates.
- `_append_history` remains the sole attempt-record appender; record schema and
  digest chaining do not change.
- Focused tests cover fresh creation, byte preservation, invalid-chain
  rejection, and symlink rejection.
- The retained original failure and invalid wrong-campaign root remain
  unmodified and are never admitted.
- Full focused and exact-head canonical gates pass; dual review, dual terminal
  verification, and dual receipt verification pass.

## Security-Impact Gate

The change may create only the exact selected ledger file and missing parent
directories. It must not follow symlinks, truncate existing bytes, accept
malformed history, relax Rust preflight, or create a ledger outside the
operator-selected path. Any such behavior is `FAIL`.

Harvard remains sealed and CAL population remains prohibited.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to one bounded implementation worker, two independent
read-only reviewers, two independent terminal/receipt verifiers, and the
`comparator_suite_runner` for one exact-head canonical admitted execution;
expected outputs are bounded helper/tests/evidence changes and retained
plan/audit/receipt/ledger verdicts; write access is limited to the declared
write set.
