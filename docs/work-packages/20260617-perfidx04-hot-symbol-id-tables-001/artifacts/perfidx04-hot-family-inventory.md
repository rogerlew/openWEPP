# PERFIDX04 Hot Family Inventory

Static:
- In scope: climate forcing, frost including fine-layer grids, WB18/WB19 water-balance layer families, PL runtime activation, and MOFE hourly transfer/carry arrays.
- Out of scope: irrigation. The backlog still records management-gated activation as deferred, so no irrigation ids were pre-resolved.

Static:
- Climate/hourly roots: `timem`, `intsty`, `obmaxt`, `obmint`, snow hourly roots, winter hourly roots.
- Frost roots: `frost.runtime_fgfrst`, `frost.runtime_slfsd_m`, `frost.runtime_slsic_m`, `frost.runtime_slsw_theta`, `frost.runtime_sltime_s`, `frost.runtime_nfine`, `frost.runtime_fine_thickness_m`, `frost.runtime_yst_m`, `frost.runtime_nwfrzz_m`, frost hourly roots.
- WB18/WB19 roots: `wb18_perc_theta`, `wb18_perc_fc`, `wb18_perc_ul`, `wb18_perc_ssc`, `wb18_perc_frzw`, `wb18_perc_frozen_depth`, `wb18_perc_pei`, `wb19_dg`, `wb19_coca`, `wb19_por`, `wb19_thetfc`, `wb19_thetdr`, `wb19_bulk_density_kg_m3`, `wb19_lateral_ssh`, `wb19_lateral_withdrawal`.
- MOFE roots/scalars: `mofe_hourly_carry_arrays_enabled`, `mofe_hourly_upstream_area_ratio`, `UpStrmQ`, `SubRIn`, `ui_SUrunf`, `ui_LfUrf`, `ui_SCrunf`, `ui_LfCrf`.
- PL roots/scalars: `pl_schedule_slot_count`, `pl_schedule_rotation_repeats`, `pl_schedule_rotation_years`, `day`, `year`, plus parsed `pl_schedule_slot_*`, `pl_schedule_slot_*_crop_*`, `pl_growth_slot_*_crop_*`, and `pl_decomp_slot_*_crop_*` families.

Static:
- Migrated call-site groups: scheduler request construction and writeback mirror synchronization; hydrology state access helpers; frost profile/fine-layer coupling; lateral drainage MOFE carries; plant percolation frost fine reads; runoff reconciliation hourly/frost/MOFE exports; PL active-slot dispatch.
