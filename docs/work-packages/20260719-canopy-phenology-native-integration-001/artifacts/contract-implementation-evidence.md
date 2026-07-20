# Contract Implementation Evidence

Evidence mode: `Static`

Status: `implemented and independently verified; joint promotion ready`

| Contract | Implementing surfaces |
|---|---|
| `SC-PLANT-001` | `openwepp-plant-phenology::ForestCanopyState`, `realize_forest_canopy`, direct runner `native_forest_growth_state_for_build`, typed growth override, ET/snow/WB15/erosion consumed-value paths |
| `SC-RESIDUE-001` | `direct_production_surface_litter_projection` native-litter branch, decomposition input, evolved surface residue and residue-depth handoffs |
| `SC-INFILE-MANAGEMENT-YAML-001` | required strict `phenology` block, positive `bb`/`xmxlai`, continuous native schedule validation, parser projection, migration refusal to invent authority |

Both independent verification verdicts pass at exact head `00cee98d`. All three
contracts are ready for joint promotion. No production claim relies on the
withdrawn prototype commits.
