# R4I-L Producer Selection

Status: complete.

Evidence class: Static.

Selected producers:

- R4I liquid-input handoff producer for `liquid_input_m`.
- R4J runon/carry handoff producer for `runon_input_m` plus subsurface-carry
  diagnostic separation.
- R4K infiltration/depression handoff producer for
  `cumulative_infiltration_m` and `depression_storage_delta_m`.
- R4L saturation-addback handoff producer for
  `surface_saturation_runoff_m`.

Selection rationale: R4A already owns the runoff-partition equation. R4I-L
therefore owns input provenance and fail-closed producer completeness, not full
WB14 branch migration.
