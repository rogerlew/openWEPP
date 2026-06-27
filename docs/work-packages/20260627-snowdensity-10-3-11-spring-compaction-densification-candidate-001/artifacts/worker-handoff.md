# Worker Handoff

Evidence mode: Static.

Current state:

- `physics_bulk_spring_densification_v1` exists as an opt-in diagnostic density
  model but is non-promoted.
- The useful coupled WAT baseline is
  `coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1`, with
  `498/1415` paired rows failing.
- The spring candidate worsens the baseline to `502/1415`.

Next recommended package:

Adjudicate the combined opt-in bundle
`coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1` as the
current best snow-control candidate, then classify the remaining `498` failures
before selecting the next one-lever process. Do not use
`physics_bulk_spring_densification_v1` as the follow-up lever.
