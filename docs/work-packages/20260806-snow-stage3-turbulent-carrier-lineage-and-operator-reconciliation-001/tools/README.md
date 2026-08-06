# Operator-Reconciliation Consumer

Status: `result-blind path scaffolded; implementation prohibited until the
pre-implementation contract gate passes`.

Phase C will add:

- `run_operator_reconciliation.py`: exact-head, version-aware v5/v6 consumer,
  independent primitive reconstruction, two-stage join, support reduction,
  predecessor reproduction, result classification, custody receipt, and
  retained-manifest writer;
- `test_run_operator_reconciliation.py`: synthetic v5/v6, unknown-schema,
  invalid-null, duplicate/missing identity, covariance, partial-support,
  endpoint-closure, and decision-precedence cases.

The consumer will use only Python's standard library. It will not import or
invoke Rust producer helpers. Result execution remains forbidden until the
implemented analyzer has passed focused tests and independent consumer review.
