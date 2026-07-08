# Review - Linnaeus

Evidence mode: Static.

Reviewer: `019f4228-59f5-7062-8ae8-a2f2ab310a70`

## Verdict

Initial verdict: HOLD pending wording and governance fixes.

Final disposition: findings accepted and addressed in this package.

## Findings

### High - Classification over-separated sediment response from routed-water timing

Accepted. The initial artifacts called the mechanism `erosion-consumer
sensitivity` and rejected routed-water timing too broadly. That over-separated
the erosion consumer from the routed hydrograph shape it consumes.

Disposition: fixed. The package now classifies the mechanism as annual sediment
response to a sub-threshold routed-hydrograph shape perturbation. The wording
explicitly states that the routed shape/timing delta is real and consumed by the
erosion path, but stayed within the current rev-43 routed-water mesh-policy
surfaces.

### Medium - Package-local classifier thresholds lacked authority

Accepted. The first analyzer used hard-coded local cutoffs (`0.01 m3`,
`0.001` L1) to decide the mechanism.

Disposition: fixed. The analyzer now uses the prior rev-43 pass/fail surfaces
for routed outlet, shape, storage, tail-fold, uniform-shape, and degenerate
shape, plus implicated-day exact guard evidence. No package-local numeric
threshold is used as authority.

### Medium - Clean-counter wording was too broad

Accepted. Run-level residual counter classes exist in both rungs:
`uniform_shape_rows = 10`, `erosion_source_shape_degenerate_rows = 1`, and
roundoff-scale nonzero total clamp.

Disposition: fixed. Artifacts now claim only implicated-day clean guards and no
candidate/reference counter increase, with run-level clamp described as
roundoff-scale rather than zero.

### Low - Human-readable artifact needed explicit follow-on

Accepted. The regenerated `wa-sediment-attribution.md` now names the hold and
the next package:
`20260708-laned-router-annual-sediment-adequacy-metric-authority-001`.

## Residual Risk

The package remains a promotion hold. It attributes the mechanism but does not
settle annual sediment metric authority.
