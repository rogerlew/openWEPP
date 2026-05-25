# SIMIMPL24 Soil-Water Lineage Provenance Map

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- WB11 aggregate state lineage is now seeded and published from simulation-owned
  runtime surfaces in `crates/openwepp-runner/src/lib.rs`:
  - `dg_####`, `thetfc_####`, `thetdr_####`, `ssc_####` ->
    `wb11_soil_water`, `wb11_field_capacity`, `wb11_drainable_storage`,
    `wb11_drainage_coefficient`.
  - per-layer WB18 lineage is explicitly seeded:
    `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`,
    `wb18_perc_ssc_####`.
  - WB19 drainage/lateral symbols are explicitly seeded:
    `wb19_lateral_anisotropy_ratio`, `wb19_drain_enabled`,
    `wb19_drain_depth`, `wb19_drain_spacing`, `wb19_drain_diameter`.
- Daily kernel execution uses `Wb11HydrologyKernel` end-to-end in runner
  scheduler lifecycle; placeholder daily kernel path was removed.
- WB14/WB16 migration-support lineage closures now cover two previously failing
  replay vectors:
  - runoff reconciliation computes infiltration/interception from liquid-rain
    partition while preserving canonical `S` flux publication,
  - peak-runoff phase accepts no-intensity runoff days with typed nominal
    zero-peak status (`HKERNEL-WB16-PEAK-ZERO-002`).

## Ran
- `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract`
- `cargo test -p openwepp --test clim05_snow_runtime_kernel_contract`
