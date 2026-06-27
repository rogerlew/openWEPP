# Review Disposition

Evidence mode: Static.

No external subagent review was dispatched in this turn because the active user
request did not explicitly ask for delegated/subagent review. Static package
review artifacts were produced locally.

## Findings

- F1: Spring wet-time densification worsens the coupled density baseline.
  - Disposition: accepted.
  - Action: package closed `SPRING-DENSIFICATION-NON-PROMOTION`; do not promote
    `physics_bulk_spring_densification_v1`.
  - Mechanism: the new rate/realization lever over-densifies into
    under-persistence after the existing bulk compaction arm has already consumed
    the available compaction headroom. `harvard_hardwood` is decisive:
    `0` compaction-only headroom rows, `64` candidate under-persistence rows,
    and failures worsen `153 -> 156`.
- F2: The existing density-compaction arm plus holding-capacity melt/liquid
  improves prior 10.3.8 WAT failures from `761` to `498`.
  - Disposition: follow-up.
  - Action: record as the next bundle/adjudication candidate, not as an
    activation in this package.
- F3: Initial run exposed density cap roundoff after mass/depth reconstruction.
  - Disposition: accepted.
  - Action: runtime now publishes the stored capped density after density
    mutation.
- F4: Compaction remains the validated density lever, but spring-specific
  wet-rate acceleration is exhausted.
  - Disposition: accepted.
  - Action: future packages must not pursue another compaction-rate variant
    without new external authority and a different residual class. Next route is
    bundle activation adjudication, then open-surface ablation for cap-limited
    mass residuals.
