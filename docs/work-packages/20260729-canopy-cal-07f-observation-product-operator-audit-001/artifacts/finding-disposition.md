# CAL-07F Finding Disposition

Evidence class: `Static + Ran`

Both terminal science reviews and verifications pass.

## Accepted and resolved finding

Reviewer B found that the initial validator checked uncertainty and direction
passes only one-way and reduced operator/parameter criteria from retained
flags. That could reject a false pass but did not reject a false negative,
leaving a cherry-picking weakness.

Resolution:

- crossing, uncertainty, and direction flags are checked biconditionally;
- product scores and ranks are independently reconstructed;
- Spearman rank correlation and top-quartile overlap are independently
  reconstructed;
- CAL-07D scenario completeness and direction medians are independently
  reconstructed; and
- operator, parameter, and final calibration predicates are recomputed from
  underlying evidence.

The strengthened validator passes.

## Independent sensitivity

Reviewer B additionally removed the seasonal-window restriction. Although all
37 members then appear crossing-complete, no member passes joint uncertainty
or direction coherence. This confirms that `DO_NOT_RECOMMEND` is not an
artifact of the source-defined midpoint windows.

No open review, verification, scientific, provenance, validation,
accessibility, or closure finding remains.

Final verdicts A and B:
`PASS / DO NOT CALIBRATE / ECOSYSTEM-MODEL LIMITATION`.
