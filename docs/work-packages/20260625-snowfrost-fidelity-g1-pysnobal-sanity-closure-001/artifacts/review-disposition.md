# Review Disposition

Evidence mode: Static.

| Review | Finding | Disposition | Rationale |
| --- | --- | --- | --- |
| A | WAT simulation-calendar dates broke reuse summary parsing. | accepted | Fixed by climate-date mapping from `sim_day_index`; covered by `openwepp_snow_projection_uses_climate_date_for_sim_day_index`. |
| A | Strict all-lanes gating must remain available. | accepted | Harness default remains `all-lanes`; G1 uses explicit `--route-policy site-sane`. |
| A | Reuse must revalidate outputs. | accepted | Reuse path rechecks finite/nonnegative SWE/depth, density ceiling, and positive snow response. |
| A | Make site-sane default. | rejected | Would weaken sensitivity-lane sanity posture; strict default is retained. |
| B | WAT is correct openWEPP comparison surface. | accepted | Exporter reads WAT `Snow-Water` and `Snow-Depth`; no snow recomputation added. |
| B | WAT date fields are not external date authority. | accepted | Exporter maps `sim_day_index` to climate dates and fails closed on out-of-range indexes. |
| B | Keep projection test focused. | accepted | Synthetic WAT projection test proves extraction without full simulation overhead. |
| B | Add progress/window export for future snowbench iteration. | follow-up | Non-blocking tooling improvement recorded in worker handoff. |

No review finding is undispositioned.
