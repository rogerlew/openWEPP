# Worker Handoff

Status: FOLLOW-ON AVAILABLE
Evidence mode: Static.

## Outcome

The WA active-router positivity-preserving solver correction is complete. The
rev-41 conservative stage-face limiter and final TVD scaling eliminate the WA
material clamp-amplification class for fixed10 and `dx5`; rev-40's publication
guard remains live.

## Follow-On Candidate

Reopen the Tier-2 target-`dx` mesh-policy adjudication for the real selected
cohort.

First actions:

1. Re-run WA fine-reference rungs (`dx2p5`, `dx1p25`, and one further halving
   if needed by the predeclared adequacy rule) on the rev-41 solver.
2. Recompute selected-cohort candidate-vs-reference surfaces for `mn_corn_h4`,
   `n_idaho_forest_h1`, and `wa_cascades_forest_h1`.
3. Keep production default fixed at `10 cells/OFE` unless the target-`dx`
   package independently passes its tolerance and authority gates.

## Non-Goals

- Do not use this package as target-`dx` promotion evidence by itself.
- Do not change active-mode erosion water-magnitude policy here.
- Do not relax rev-27 or rev-40 closure/publication guards.
