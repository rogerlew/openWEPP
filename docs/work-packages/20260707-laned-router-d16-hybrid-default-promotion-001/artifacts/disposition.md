# Disposition

Status: EXECUTED-HOLD-FIDELITY-TOLERANCE. Evidence mode: Static + Ran.

## Decision

Hold D16 hybrid active-path default promotion before implementation.

## Accepted Findings

1. The timing subgate passes: explicit hybrid is faster than active plain on
   the D16 H2637 release binary (`33.45 s` vs `39.73 s` user).
2. The Case-4 subgate passes: the retained full-hybrid ladder test passed.
3. The active closure subgate passes under explicit hybrid: max closure
   residuals remain at machine precision.
4. The fidelity/tolerance subgate blocks promotion: active plain-vs-hybrid
   H2637 publication deltas include `-0.4396 %` routed outlet and `-6.474 %`
   pass sediment sums, and no current contract ratifies those as
   default-safe production deltas.

## Implementation Disposition

No selector, manifest, contract, or runtime code changes landed. The current
env opt-in hybrid remains unchanged.

## Review/Verification Status

Subagent review completed:

- `artifacts/review-lorentz.md`: GO-WITH-AMENDMENTS; all findings accepted.
- `artifacts/review-euclid.md`: GO-WITH-AMENDMENTS; all findings accepted.

Review amendments made:

- Added the hold-legitimacy audit.
- Promoted raw H2637 timing/delta evidence into formal artifacts.
- Added the build command to binary provenance.
- Replaced placeholder/PENDING gate rows.
- Clarified that explicit `OPENWEPP_LANED_ACTIVE_IMPLICIT=0` was not run
  because implementation did not proceed.

Final verification completed:

- `artifacts/verification-hilbert.md`: GO.
- `artifacts/verification-anscombe.md`: GO.
