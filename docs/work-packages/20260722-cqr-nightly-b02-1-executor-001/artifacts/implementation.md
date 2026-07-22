# Implementation

Static: behavior-preserving decomposition only.

- `validate_affected_quality_scope` retains the completeness guard, then calls
  package, covering-node, and inventory validators in the original order.
- The helpers preserve the original `BTreeSet` construction/equality,
  covering-node filtering and gate-definition identity, inventory flattening,
  nonempty guard, typed codes, and exact messages.
- No public API, gate selection, policy, numeric expression, or side effect
  changed.

Production commit: `a7bae33b`. Terminal production SHA-256:
`0b6eda5d0fefbb1605f863325c1817cc97576c89fe8bdeaa8afc486e1ebabd98`.
Test-first commits: `d15f7b5a` and review-strengthening `0bd56dc3`.
