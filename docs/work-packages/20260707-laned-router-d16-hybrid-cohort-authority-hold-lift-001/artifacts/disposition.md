# Disposition

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-AUTHORITY. Evidence mode: Static + Ran.

## Decision

Hold before implementation.

The package selected the intended cohort and ran the available preflights. The
hold narrowed from generic cohort availability to a precise route-coefficient
input-authority boundary: selected external roots do not carry native
`ow-lanuse-1` `routing_coefficients`, and the current contracts forbid deriving
them from legacy cropland fields.

## Changes Landed

- Package-local evidence and handoff artifacts.
- `docs/work-packages/README.md` catalog entry.

No code, contract, fixture, suite, selector, or dependency change landed.

## Review Disposition

- Descartes: GO. Accepted.
- Cicero: GO-WITH-AMENDMENTS. Accepted and fixed:
  - filed review/verification artifacts,
  - refreshed final gate evidence after review-response artifacts,
  - added `external-root-snapshot.md` to strengthen mutable-root provenance.

## Verification Disposition

- Bernoulli: initial NO-GO only for missing S5 artifacts and pending final
  gates; accepted and fixed.
- Meitner: initial NO-GO only for missing S5 artifacts and pending final gates;
  accepted and fixed.

## Final Decision

Close as `EXECUTED-HOLD-ROUTE-COEFFICIENT-AUTHORITY`.
