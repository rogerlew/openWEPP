# Final Disposition

Status: EXECUTED-HOLD-DX-REFERENCE-ADEQUACY
Evidence mode: Static + Ran.

The Tier-2 dx-target mesh-policy re-scope executed and closes as a hold.

Final decision:
- Do not promote target-`dx` active mesh policy.
- Keep active production fixed at `10 cells/OFE`.
- Keep Lane D shadow mesh separate and unchanged.

Why:
- Real `mn_corn_h4` and `n_idaho_forest_h1` pass the tolerance comparisons,
  but fixed10 also passes and target-`dx` adds cost.
- Real `wa_cascades_forest_h1` fails the fine-reference rungs at active
  closure day 1122 and shows non-promotable target-`dx10/dx5` behavior.
- H2637 is synthetic stress and fails shape/sediment adequacy.

Hold condition:
- `EXECUTED-HOLD-DX-REFERENCE-ADEQUACY`: fine-reference adequacy and
  real-cohort evidence fail the package's required tolerance surface.

First actionable follow-on:
- `20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001`
  to attribute the WA day-1122 high-resolution closure failure and decide
  whether any future mesh-policy package is legitimate.

No production mesh-policy implementation is pending from this package.
