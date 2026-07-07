# Disposition

Status: EXECUTED-HOLD-COHORT-AUTHORITY. Evidence mode: Static + Ran.

## Decision

Hold the D16 hybrid fidelity-tolerance lift before contract/code changes.

## Findings

- The tolerance surfaces required for a future promotion were named but not
  ratified.
- The owcmp cohort manifests are inventory-only and cannot run active
  plain-vs-hybrid comparisons.
- No available repo fixture or checked external run-root management file has
  `routing_coefficients`.
- Repo-local active preflight produced no active-runnable member outside the
  D16 package-local H2637 evidence.

## Implementation Disposition

No contract amendment, selector default flip, fixture mutation, or Rust change
landed.

## Review Disposition

- `review-newton.md`: GO-WITH-AMENDMENTS; accepted closure-artifact/gate
  completeness finding.
- `review-popper.md`: GO-WITH-AMENDMENTS; accepted gate-table,
  review/verification artifact, and preflight command-detail findings.

All review findings are accepted and addressed in package artifacts.

## Verification Disposition

- `verification-mcclintock.md`: GO.
- `verification-heisenberg.md`: GO.
