# CAL-07C Intent Plan

Evidence class: `Static`

CAL-07C is authorized by the operator's instruction to correct the CAL-07
values after CAL-07B identified the source/operator mismatch. The correction
is scoped to a package-local Alerce VPD forcing reconstruction from retained
POWER hourly-average paired products. It is not a production contract change.

Pre-result stop-loss:

- If the hourly response lacks complete 24-hour daily inventory, hold.
- If any admitted daily VPD is negative, hold.
- If any hourly-product VPD is nonfinite, missing, clipped, deleted, or hidden,
  hold. Signed negative hourly-product components may proceed only if counted,
  retained, plotted, and carried into claim limits.
- If source custody or metadata contradicts the frozen case, hold.
- If prospective review rejects the authority boundary, hold.
- If execution produces incomplete member/site/day output, hold.
