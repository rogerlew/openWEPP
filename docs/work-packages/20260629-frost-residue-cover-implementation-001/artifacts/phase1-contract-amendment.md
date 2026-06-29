# Phase 1 Contract Amendment

Evidence class: Static + Ran.

## Contract Home

The residue-depth state extends `SC-RESIDUE-001`, because residue owns the
surface-residue mass, decomposition, and mass-to-depth conversion. Frost consumes
that dynamic boundary through `SC-SNOWFREEZE-001`.

## Amendments Landed

- `SC-RESIDUE-001` revision 11 added `INV-RESIDUE-019`, `residue_depth_m`,
  `surface_litter_input_kg_m2`, the pending litter bucket, the fall litter-drop
  publication window, and the forest-litter turnover fallback.
- `SC-SNOWFREEZE-001` revision 113 added `INV-SNOWFREEZE-083`, binding frost
  thermal inputs to consume dynamic residue depth instead of the t0 seed after
  residue mass changes.

## Binding Constants

- `FOREST_LITTER_FALLBACK_DECAY_RATE = 0.5 / 365.25 d^-1`.
- `FOREST_LITTER_DROP_WINDOW_DAYS = 45`, ending on the management fall date.

## Known Limitation

The fall litter-drop window is currently anchored to the management fall date
(`jdharv`), matching the existing canopy leaf-off fixture anchor. This is not a
physical phenology trigger. The anchor must be replaced with the frost/daylength
phenology trigger when the leaf-on/leaf-off backlog lands.

## Exclusions Preserved

No snow-model, canopy leaf-on/off, Qwet, public-schema, fixture-fitting, or
frost-default activation authority was added.
