# DC CQR HB-05 — Non-Finite Watershed Datver

Status: `ACTIVE`

## Objective

Close `DC-CQR-HB05-001`: watershed structure parsing can accept `+Inf` as an
explicit `datver`, while NaN/`-Inf` are misclassified as no-header input.

## Correction Authority Envelope

- Canonical authority: watershed structure `G-STR-001`, `SC-INFILE`/system
  fail-closed accepted-input policy, and ADR-0021 family F.
- Production write set:
  `crates/openwepp-input-contract/src/parsers/watershed_structure.rs`, limited
  to explicit preamble version finiteness plus the active HB-05 decomposition.
- Test write set: the target module's focused parser tests.
- Allowed correction: after numeric parsing and before the `>10` discriminator,
  reject NaN and both infinities as existing `UnsupportedDatver` / `STR-E-003`
  with line/value/min-supported fields preserved.
- Excluded: a finite upper version bound, grammar/delimiters, strict/compat
  policy, row/cardinality/topology logic, error strings, schemas, and consumers.
- Acceptance: all three non-finite forms fail `STR-E-003` in strict and
  compatibility modes; finite versions at/above minimum retain behavior;
  focused metrics/gates and the real zero-impoundment CLI consumer pass.
- Security impact: none; numeric admission becomes fail-closed.

Conversion rule: the reviewer-proven mechanism is local, authority-backed,
safe, and directly testable; correction is mandatory before HB-05 closure.

## Progress

- [x] Reproduce mechanism statically during independent review.
- [ ] Add red strict/compat non-finite contract tests.
- [ ] Land the bounded preamble correction.
- [ ] Regenerate HB-05 same-source evidence and consumer proof.
- [ ] Complete dual review/verification and terminal disposition.

## Review And Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to one bounded implementer and two read-only review/verification agents for the
declared source, tests, and evidence.

## Outcomes

Pending execution.
