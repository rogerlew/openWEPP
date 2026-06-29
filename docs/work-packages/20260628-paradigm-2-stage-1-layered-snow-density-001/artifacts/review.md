# Stage 1 Review

Evidence class: `Static + Ran`

## Findings

No implementation defects requiring code changes remain after the final focused
review pass.

The candidate does not promote. That is a gate result, not a code-review defect:
the opt-in `physics_bulk_multilayer_density_v1` path executes, preserves layer
closure and whole-model conservation, but scores worse than the current no-env
default on the cross-SNOTEL+cancov forcing-robust rubric.

## Checked Surfaces

- Contract authority: `SC-SNOWFREEZE-001` v108 defines
  `INV-SNOWFREEZE-078`, `OBL-SNOWFREEZE-P-053`, and the Stage 1 opt-in selector.
- Runtime state: `DirectSnowLayerState` persists under
  `DirectSnowLaneState.layers` and flows through winter-column state, snow
  runtime carry, typed snow partition, and R4G snow coupling.
- Selector discipline: the new candidate is reachable only through the existing
  internal density-model selector; absent-selector default and `legacy_wepp`
  rollback remain unchanged.
- Conservation: observed-corpus candidate traces close snow-state and partition
  residuals below `1e-9 m`.
- Output boundary: trace-only layer diagnostics were added; public WAT schema and
  fixtures were not changed.

## Residual Risk

The implementation is intentionally non-promoted. Stage 1 local-overburden
layering alone does not resolve the robust densification/persistence residual,
so it should remain opt-in diagnostic surface unless a later Paradigm 2 stage
changes the primary rubric result.
