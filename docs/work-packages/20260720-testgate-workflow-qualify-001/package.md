# TESTGATE Forest1 Workflow Qualification

Package ID: `20260720-testgate-workflow-qualify-001`

Queue ID: `TESTGATE-WORKFLOW-QUALIFY-01`

Status: `ACTIVE / READY-QUALIFICATION`

## Objective

Qualify the deployed ordinary TESTGATE workflow with one exact-head, explicitly
dispatched forest1 run. Forest1 is the trusted heavy runner. Its local receipt
classification, `LOCAL_UNTRUSTED`, is expected and must not be treated as a
failure or a hold.

## Scope

Included: one queue-idle preflight, one ordinary exact-head dispatch using this
active package, forest1 execution, retained receipt/ledger/artifact inspection,
and concise independent review of the result.

Excluded: the invented Q01--Q15 adversarial matrix, a separate qualification
controller, synthetic probe fixtures, GitHub-hosted heavy execution, hosted
attestation as an acceptance prerequisite, duplicate dispatches, and any edit
to TESTGATE implementation/policy/workflow code.

## Declared Write Set

- `docs/ROADMAP.md`
- `AGENTS.md`
- `docs/standards/testing-and-gate-strategy.md`
- `gate-policy/v1/impact-map.json`
- `tools/local_ci/testgate.py`
- `tools/local_ci/README.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260720-testgate-workflow-qualify-001/**`

## Execution

1. Commit and push the documentation-only ready increment through the normal
   repository workflow; use the active-scaffold commit and exact package path
   required by `testgate-shadow.yml`.
2. Prove no current forest1 TESTGATE run is queued or active. Ignore immutable
   queued records from the retired Omarchy runner; do not cancel them.
3. Dispatch exactly one run for the pushed exact head. Do not dispatch again
   for reassurance or to create a concurrency scenario.
4. Record the forest1 run ID, exact head/base, result, local receipt, ledger,
   retained artifacts, and any hosted verification record. A forest1 local
   receipt remains valid evidence even when labeled `LOCAL_UNTRUSTED`.
5. If the run fails before any selected gate starts because package admission is
   malformed or a directly caused policy-identity binding is stale, correct the
   metadata/binding in this package and make one fresh changed-head dispatch.
   For any gate-execution failure, retain its evidence and open a focused defect
   package; do not retry inside this package.

## Acceptance

- Exactly one ordinary TESTGATE dispatch occurs after a queue-idle preflight.
- The run executes the policy-selected TESTGATE nodes on forest1, binds the
  exact pushed head and active package, and produces retained receipt/ledger/
  artifact evidence.
- `LOCAL_UNTRUSTED` does not invalidate the forest1 result.
- No duplicate heavy run, policy change, or implementation edit occurs.
- Independent review accurately records PASS or FAIL from the retained evidence.

## Delegation

Subagent authorization: this package explicitly authorizes two read-only result
reviewers and two read-only terminal verifiers. Their outputs are concise
evidence checks; no subagent may dispatch, push, or edit TESTGATE.
