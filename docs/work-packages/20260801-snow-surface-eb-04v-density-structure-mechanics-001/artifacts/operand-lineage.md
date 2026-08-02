# Density Process Operand Lineage

Status: `sealed before production edits`.

Evidence class: `[DIRECT][Static] + [INFERENCE][Static]`. Density increments
use `kg m^-3` and are positive when they increase bulk density.

| Field | Producer cut point | Meaning / anti-alias rule |
|---|---|---|
| `applicable` | density dispatch | False for legacy or snow-free update; zero never implies applicability. |
| `initial_density_kg_m3` | selected model entry | Aggregate runtime density before new snow; not CoE boundary density. |
| `initial_snow_mass_kg_m2` | selected model entry | Total initial snow load; not precipitation or the local layer overburden. |
| `liquid_for_compaction_mass_kg_m2` | density input boundary | Exact liquid mass supplied to wet compaction; not retained liquid after routing. |
| `compaction_temperature_c` | density input boundary | Exact clamped temperature supplied to dry compaction; not an after-layer temperature. |
| `snow_input_mass_kg_m2` | snowfall conversion | `snow_input_m * 1000 kg m^-3`; not total precipitation or layer mass. |
| `snow_input_depth_m` | fresh-density evaluation | Physical new-snow depth before mixing; not water equivalent. |
| `fresh_snow_density_kg_m3` | Anderson form before insertion | Direct evaluation; never inferred from an after-layer or total daily delta. |
| `fresh_snow_mixing_delta_kg_m3` | state around insertion | Exact aggregate-density change from mass/depth mixing. |
| `wet_compaction_delta_kg_m3` | state around wet compaction | Exact uncapped daily bulk increment; excludes dry terms. |
| `destructive_metamorphism_delta_kg_m3` | dry attribution | Exact uncapped dry bulk increment allocated by same-state PTM tendency share. |
| `overburden_compaction_delta_kg_m3` | dry attribution | Exact uncapped dry bulk increment allocated by same-state POC tendency share. |
| `internal_cap_delta_kg_m3` | model internal cap | Signed difference between uncapped and capped dry/wet result. |
| `structural_projection_delta_kg_m3` | boundary/merge stage | Aggregate change from CoE mass projection and layer merging. |
| `climate_fallback_used` | Sturm fallback dispatch | Explicitly distinguishes an invoked zero-delta fallback from an unused fallback. |
| `climate_fallback_delta_kg_m3` | Sturm fallback | Exact aggregate change; zero and unused states remain distinct. |
| `runtime_cap_delta_kg_m3` | final runtime cap | Exact signed cap correction after structural projection. |
| `downstream_stage3_delta_kg_m3` | partition after Stage 3 | Exact downstream vapor/layer density change; not density-process physics. |
| `final_density_kg_m3` | JSONL boundary | Final density published on the trace row. |
| `closure_residual_kg_m3` | finalizer | `final - initial - sum(increments)`; independently reconstructed. |

For multilayer dry compaction, PTM and POC use the identical pre-substep layer
state. Their raw local tendencies weight the exact uncapped bulk-density
increment, while `internal_cap_delta_kg_m3` records the exact realized-minus-
uncapped bulk correction at that mutation. This preserves combined mutation
arithmetic and is a diagnostic contribution, not an isolated-process
counterfactual.

Revision note (2026-08-01): independent review rejected the first local-density
cap projection. The table now binds exact bulk-space cap reconstruction plus
explicit driver/fallback fields. This corrected lineage was sealed before the
fresh post-review release build and cohort execution; the invalidated first-run
evidence is retained separately and is not decision eligible.
