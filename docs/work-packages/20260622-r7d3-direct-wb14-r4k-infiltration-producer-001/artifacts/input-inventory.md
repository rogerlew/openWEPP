# Input Inventory

Status: executed-held.

## Available Direct Inputs

- `wb12_rainfall_input`: direct R4I liquid input and R4C scalar storage-input
  override source.
- `ninten`/`nbrkpt`, `timem_####`, `intsty_####`: direct WB14 hyetograph
  source for R4K.
- `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`,
  `wb18_perc_frzw_####`, `wb18_perc_ssc_####`, `wb19_dg_####`,
  `wb19_thetdr_####`, `wb19_thetfc_####`: direct layer state and storage
  capacity source.
- Optional direct WB14 parameter sources:
  `wb14_effective_conductivity_m_s`, `frost.runtime_infcap_frz`,
  `wb14_soil_conductivity_m_s`, `wb14_matric_potential_m`,
  `wb14_depression_storage_capacity_m`, and
  `wb12_depression_storage_capacity_m`.
- R4O same-day `hourly_saturation_carry_m` shadow: direct `ui_SCrunf` source
  consumed by R4L after this package.

## Missing Inputs

- Dynamic same-day MOFE carry transfer remains missing:
  current-lane `ui_SCrunf`/`ui_LfCrf` arrays are not copied into the downstream
  lane's `ui_SUrunf`/`ui_LfUrf` transfer buffers before downstream R3A/R4J
  execution.
- R4J still lacks typed dynamic transfer consumption. It reads
  `DirectRunonCarryInputs` constructor handoffs, so downstream lanes receive
  zero `UpStrmQ` and `SubRIn` in H2637 direct production.
- PASS `runvol`/`sbrunv` remains blocked behind the same missing dynamic carry
  and downstream runoff publication lineage.

## Forbidden Inputs

- Compatibility `execution.wb13_rows`.
- Compatibility public-output builders as production direct authority.
- Compatibility runtime-surface observations such as `wb12_infiltration` and
  `wb12_depression_storage_delta` as direct R4K producer authority.
- Aggregate carry aliases that replace explicit 24-slot `ui_SUrunf`,
  `ui_SCrunf`, `ui_LfUrf`, or `ui_LfCrf` arrays.
