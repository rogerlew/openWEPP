# SIMIMPL33 Frost Seam Symbol Catalog

Status: complete
Evidence mode: static
Date: 2026-05-26

## Static
- New/expanded frost seam families in scope:
  - Runtime topology:
    - `frost.runtime_total_fine_layer_count`
    - `frost.runtime_nfine_####`
    - `frost.runtime_fine_thickness_m_####`
  - Runtime bookkeeping:
    - `frost.runtime_frdp_m`, `frost.runtime_thdp_m`
    - `frost.runtime_tfrdp_m`, `frost.runtime_tthawd_m`
    - `frost.runtime_fgthwd_flag`
  - Conductivity lineage:
    - `frost.runtime_kftill_w_m_k`
    - `frost.runtime_kfutil_w_m_k`
    - `frost.runtime_kres_w_m_k`
  - Required active-frost seam inputs:
    - `snow.runtime_depth_m`
    - `frost.runtime_residue_depth_m`
  - Hourly seam output families:
    - `frost.hourly.qsrf_w_m2_####`
    - `frost.hourly.quf_w_m2_####`
    - `frost.hourly.ksrf_w_m_k_####`
    - `frost.hourly.snow_depth_m_####`
    - `frost.hourly.residue_depth_m_####`
    - `frost.hourly.tilled_frozen_depth_m_####`
    - `frost.hourly.untilled_frozen_depth_m_####`
- Typed guard posture:
  - Missing required active-frost seam symbol ->
    `HKERNEL-WB14-RUNOFF-E-001` (`BoundaryClass::MissingRequiredInput`).
  - Non-finite required active-frost seam symbol ->
    `HKERNEL-WB14-RUNOFF-E-002` (`BoundaryClass::NonFinite`).
  - Domain violations continue as `HKERNEL-WB14-RUNOFF-E-003`.

## Ran
- not run
