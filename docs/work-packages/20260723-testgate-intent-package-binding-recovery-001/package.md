# TESTGATE Intent Package Binding Recovery

Package: `20260723-testgate-intent-package-binding-recovery-001`
Status: `ACTIVE / SCAFFOLD`
Defect: `RTR-047`
Cause: `GATE-TRUSTED-WORKFLOW-INTENT-PACKAGE-OMISSION`

## Objective

Bind every trusted TESTGATE invocation to one explicit, authenticated intent
package before planning. Push execution must derive the operator-supplied
anchor from the exact head commit; manual execution must require an explicit
input. Zero, duplicate, malformed, or event-inconsistent declarations fail
before gate planning.

## Correction Authority Envelope

- Observed violation: automatic push run `29981856347` passed runner preflight,
  durable-history restore, and superseded-head rejection, then invoked
  `tools/local_ci/testgate.py` without required `--intent-package`.
- Retained result: `orchestration-error.log` records the argparse failure and
  the authenticated attempt index binds an empty `attempts.jsonl`.
- In scope: exact intent-package resolution, trusted workflow binding,
  focused behavioral/source regressions, canonical standard text, review,
  durable closure, and one later changed-head automatic qualification.
- Protected boundaries: no gate-selection or threshold change, no manual
  dispatch, no unchanged expensive rerun, and no inference from multiple
  package candidates.

## Declared Write Set

- `.github/workflows/testgate-shadow.yml`
- `gate-policy/v1/impact-map.json`
- `tools/local_ci/resolve_testgate_intent_package.py`
- `tests/python/test_resolve_testgate_intent_package.py`
- `tests/integration/testgate_ci_executor_contract.rs`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/20260723-testgate-intent-package-binding-recovery-001/**`
- `docs/work-packages/README.md`

## Required Reading

- `AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/work-packages/20260720-testgate-recovery-trust-001/package.md`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes two independent
read-only implementation reviewers and two read-only terminal verifiers. No
reviewer may push, deploy, dispatch TESTGATE, or run an expensive gate.

## Progress

- [x] Retained run `29981856347` and opened durable defect RTR-047.
- [x] Scaffolded prospective correction authority before implementation.
- [ ] Implement exact event-bound intent-package resolution.
- [ ] Run focused validation and obtain dual implementation review.
- [ ] Close RTR-047 durably and obtain dual terminal verification.

## Exit Criteria

- Push execution consumes exactly one valid intent-package declaration from
  the exact head commit.
- Manual execution requires one explicit workflow input.
- The exact resolved package is passed to `testgate.py --intent-package`.
- Missing, duplicate, malformed, and event-inconsistent declarations fail
  before planning.
- Focused tests, formatting, documentation lint, dual review, and durable
  closure pass before another changed-head push.
