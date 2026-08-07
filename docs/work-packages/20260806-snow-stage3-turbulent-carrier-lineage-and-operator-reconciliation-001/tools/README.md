# Operator-Reconciliation Consumer

Status: `executed / retained v3 consumer and focused tests PASS`.

The realized independent path contains:

- `run_operator_reconciliation.py`: exact-head, version-aware v5/v6 consumer,
  independent primitive reconstruction, two-stage join, support reduction,
  predecessor reproduction, result classification, custody receipt, and
  retained-manifest writer;
- `test_run_operator_reconciliation.py`: synthetic v5/v6, unknown-schema,
  invalid-null, duplicate/missing identity, covariance, partial-support,
  endpoint-closure, and decision-precedence cases.

The consumer uses only Python's standard library and does not import Rust
producer helpers. It passed focused tests and independent consumer review
before admitted v3 execution, then wrote the verified `143`-file retained
namespace and frozen operator-mechanics disposition.
