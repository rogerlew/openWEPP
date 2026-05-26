# SIMIMPL33 Runtime State Topology Map

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- SIMIMPL33 adds runtime topology/state families required before SIMIMPL34
  frost solver migration:
  - Per-run freeze/thaw bookkeeping lineage:
    - `frost.runtime_frdp_m`
    - `frost.runtime_thdp_m`
    - `frost.runtime_tfrdp_m`
    - `frost.runtime_tthawd_m`
    - `frost.runtime_fgthwd_flag`
  - Fine-layer topology lineage:
    - `frost.runtime_total_fine_layer_count`
    - `frost.runtime_nfine_####`
    - `frost.runtime_fine_thickness_m_####`
  - Conductivity lineage surfaces:
    - `frost.runtime_kftill_w_m_k`
    - `frost.runtime_kfutil_w_m_k`
    - `frost.runtime_kres_w_m_k`
  - Hourly frost seam families:
    - `frost.hourly.qsrf_w_m2_####`
    - `frost.hourly.quf_w_m2_####`
    - `frost.hourly.ksrf_w_m_k_####`
    - `frost.hourly.snow_depth_m_####`
    - `frost.hourly.residue_depth_m_####`
    - `frost.hourly.tilled_frozen_depth_m_####`
    - `frost.hourly.untilled_frozen_depth_m_####`
- Fine-layer count derivation follows baseline `frostn.for` topology shape:
  top layers use `fineTop`, deeper layers use `fineBot`, last-layer adjustment
  uses spacing logic equivalent to baseline integer/truncation behavior.
- SIMIMPL33 deliberately does not claim full frost physics migration; emitted
  hourly frost fields are seam/topology closures for SIMIMPL34 consumption.

## Ran
- `cargo test -p openwepp --test clim06_frost_frozen_soil_kernel_contract -- --nocapture`
