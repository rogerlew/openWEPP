# Disposition

Status: `EXECUTED-COMPLETE-METRIC-AUTHORITY`
Evidence mode: Static + Ran.

## Verdict

Annual pass-sediment metric authority is closed for the current target-`dx`
mesh-policy adjudication chain.

`SC-OFEROUTE-001` rev 44 replaces the strict relative-only annual sediment
mesh-policy gate with a material-year plus annual-vector rule. The selected
real-cohort replay closes the prior WA annual sediment blocker under that
rule and reveals no new annual sediment blockers.

## Evidence

- `annual-sediment-metric-replay.md`: `21` comparisons replayed,
  `1` pre-rev44 strict-relative blocker, `0` rev-44 blockers.
- `metric-authority-decision.md`: decision rationale and non-authorization
  boundary.
- `contract-disposition.md`: touched contract surfaces and BEI legitimacy
  posture.

## Review / Verification

Independent review and verification artifacts are package-local:

- `review-codex.md`
- `verification-codex.md`

Any findings are dispositioned in `final-disposition.md`.

## Boundaries

The package did not flip production mesh defaults. The active production path
still defaults to fixed `10 cells/OFE`; target-`dx` remains diagnostic until a
later package promotes it with full evidence.

The package did not change:

- routed-water math;
- routed-shape thresholds;
- storage, tail-fold, closure, or clamp guards;
- active selector semantics;
- shadow mesh policy;
- sediment process physics;
- default/off protected behavior.

## Remaining Work

The next package should re-open `dx5` production mesh-policy ratification on
the current `SC-OFEROUTE-001` rev-44 basis. It must decide and, if supported,
implement the production default flip, including the shadow-mesh question and
the full protected/default/off and active closure/consumer proof gates.
