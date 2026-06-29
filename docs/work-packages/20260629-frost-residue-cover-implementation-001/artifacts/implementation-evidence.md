# Implementation Evidence

Evidence class: Static + Ran.

## Implemented Boundary

- `DirectDecompositionInputs` now accepts same-day `surface_litter_input_kg_m2`
  and `residue_depth_conversion_m_per_kg_m2`.
- `DirectDecompositionState` publishes current `residue_depth_m`.
- `DirectPublicationDayInput` carries explicit decomposition and residue
  partition inputs into the direct runtime frame.
- The direct-production builder maintains per-lane residue-cover state:
  surface mass, root mass, pending surface litter, and dynamic residue depth.
- Frost thermal input construction accepts the dynamic residue-depth override and
  hard-fails on negative or non-finite values.

## Seasonal Forest Litter Limb

Phase 0 showed the current `Dec_*` fixture had `oratea=0`, `orater=0`, and a
flat surface-residue mass. The implementation therefore adds a first-class
litter-input limb for recurring perennial forest litter:

- non-fall senescence losses are conserved in a pending surface-litter bucket;
- the pending bucket is published to frost-visible surface residue during the
  45-day fall litter-drop window ending on the management fall date;
- inert/no-senescence `Tah_*` paths keep zero pending litter and no dynamic
  seasonal input.

## Trace Evidence

`OPENWEPP_FROST_RESIDUE_COVER_TRACE_PATH` writes diagnostic-only JSONL rows with
surface residue, root residue, pending litter, dynamic residue depth, conversion,
and decomposition-rate fields. This is not a public output-schema change.
