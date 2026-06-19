# PERFDEEP07 Hold-Lift Disposition

Status: complete.
Evidence mode: Static.

## Active Blocker

PERFDEEP07 remains in `HOLD` because the default-disabled H2637 timing gate did
not pass. Retained code improved the PERFDEEP05 default-disabled endpoint from
`701.95 s` to `685.85 s`, but P0 requires median `<= 676.67 s`.

Direct-frame implementation was not started in PERFDEEP07 because its package
forbade proceeding past that failed gate.

## R0/R1 Boundary

While this blocker is active, R0/R1 work is limited to:

- planning;
- schema envelope;
- type-boundary decision;
- fixture and ledger planning;
- shadow-scaffold planning;
- non-activated constructor/projection planning.

Not allowed:

- direct executor implementation;
- runtime readiness claims;
- default activation;
- opt-in activation;
- direct-frame hydrology implementation closure.

## Hold-Lift Conditions

To move beyond planning-only R0/R1, a package must record either:

- PERFDEEP07 P0 closure with a passing default-disabled H2637 median gate; or
- explicit supersession authority with replacement timing, identity, rollback,
  and default-disabled gates.

## Gate

PASS. The active blocker is recorded and this package stays inside the allowed
planning-only boundary.
