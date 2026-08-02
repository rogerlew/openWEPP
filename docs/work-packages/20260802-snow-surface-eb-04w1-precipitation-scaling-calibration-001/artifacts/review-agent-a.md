# Review Agent A — Independent Science And Claim Review

Status: `HOLD PENDING CLAIM CORRECTION`

Evidence mode: **Ran + Static**.

## Scope Reviewed

I independently reviewed the prospective freeze, execution receipt, complete
machine-readable results, scientific synthesis and disposition,
calibration-readiness matrix, operand lineage, forcing audit, exact baseline
replay, all three figures and sidecars, package plan, and ADR-0042. I did not
modify or rerun the frozen experiment.

## Ran Evidence

- Recomputed SHA-256 identities for the freeze, receipt, and frozen execution
  tool; every identity agrees with the receipt and results.
- Reconstructed the strict joint-improver set and frozen lexicographic ranking
  independently from all 32 result rows. The reported selections and joint
  counts are exact: Mica Creek `1.5 / 5`, Niwot `1.3 / 4`, Paradise `1.5 / 5`,
  and Snowbird `1.5 / 2`.
- Confirmed `32 / 32` receipt cells, all return codes zero, maximum closure
  `3.331e-15 m < 1e-12 m`, and zero baseline-operator replay residual for all
  four lanes.
- Parsed all three SVG files successfully and visually inspected rendered PNG
  conversions for clipping, legend obstruction, and correspondence with the
  reported response patterns.

## Static Findings, Severity Ranked

### M1 — Moderate: the grid-extension rationale overstates Mica Creek's boundary evidence

The package says three response curves “remain improving at `1.5`” and uses
that statement to justify bracketing all three boundary selections upward.
That is not true for both frozen objective axes at Mica Creek. Its peak-ratio
fit is closer to one at `1.4` (`0.96755`; absolute log error about `0.0330`)
than at `1.5` (`1.05942`; about `0.0577`), while chronology alone improves
from 21 to 20 days. The `1.5` selection is mechanically correct because the
frozen lexicographic rule treats every `[0.9, 1.1]` value as an equal first-tier
fit and then prioritizes chronology. It does not establish that Mica Creek's
magnitude optimum is unbracketed.

Required correction: distinguish Mica Creek's already bracketed magnitude
response from its still-boundary chronology tradeoff. An upward successor is
defensible for Paradise and Snowbird on both frozen axes; including Mica Creek
should be described as testing whether chronology gains persist after the
magnitude fit turns over, with a predeclared overshoot stop-loss. The
recommendation should also acknowledge that Niwot's magnitude continues to
improve through `1.5` even though the frozen ranking selects `1.3` because the
chronology values tie and distance from `1.0` is the final tie-breaker.

Disposition required: `accepted`, `rejected`, `deferred`, or `follow-up`.

### M2 — Moderate: one sentence calls non-calibrated candidates “calibration results”

The synthesis says, “A selected multiplier is a calibration result for its
source fixture and SNOTEL record only.” This conflicts with the same package's
correct conclusion that no lane is `EMPIRICALLY_CALIBRATED`, three selections
are boundary-censored, and Niwot remains magnitude-low.

Required correction: call each value a “candidate selected by this calibration
experiment” or equivalent. Preserve the existing explicit prohibitions on
independent validation, transferability, regional defaults, and promotion.

Disposition required: `accepted`, `rejected`, `deferred`, or `follow-up`.

### L1 — Low: the seasonal trajectory y-axis includes observed SWE but says modeled SWE

Each seasonal panel plots a black observed curve, but the y-axis is labeled
“Median modeled SWE (m).” Rename it “Median SWE (m)” or explicitly identify
both modeled and observed series in the axis/sidecar. This does not affect the
numeric result.

Disposition required: `accepted`, `rejected`, `deferred`, or `follow-up`.

### L2 — Low: covariance readiness needs a one-dimensional rationale

The readiness matrix marks “covariance/equifinality retention” `PASS`, but the
rationale discusses compensation/equifinality only. Because this experiment
varies one coefficient, parameter covariance is not estimable within its
surface. State that explicitly and retain the documented cross-process
confounding. This is a reporting clarification, not a reason to rerun.

Disposition required: `accepted`, `rejected`, `deferred`, or `follow-up`.

## Validated Claim Posture

Subject to M1 and M2, the principal scientific posture is defensible:

- precipitation scaling materially affects magnitude in all four lanes and
  produces strict joint magnitude/chronology improvement somewhere in every
  frozen lane;
- the observations were prospectively and exclusively assigned to
  `CALIBRATION`, so the package correctly makes no independent-validation or
  transferability claim;
- `CALIBRATION_READY_DATA_LIMITED` and `PARTIALLY_IDENTIFIABLE` are consistent
  with ADR-0042 because deterministic sensitivity is demonstrated while
  forcing bias remains confounded with phase, representativeness, retention,
  and loss;
- `GRID_BOUNDARY` is correctly applied to the three `1.5` selections and none
  of the reported values supports production promotion or a transferable
  default.

## Recommendation

`HOLD` for scientific-claim closure until M1 and M2 are corrected and
dispositioned. No experiment rerun is required. After those corrections, this
science review recommends `PASS`; L1 and L2 should be resolved before final
package closure but do not challenge the empirical result.
