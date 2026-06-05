# Contract-Test Implementation Evidence

Status: complete

Evidence mode: static

Static: added contract-derived test
`tests/integration/hphys0299_hourly_snow_partition_unit_provenance_contract.rs`
and registered it in `Cargo.toml`.

Static: the test asserts:

- Canonical contracts include `INV-CLIMATE-014`, `INV-SNOWFREEZE-030`, and
  `INV-WATBAL-074`.
- Package and prompt prohibit production migration from the old
  depth-vs-water-equivalent mismatch.
- HPHYS0299 diagnostic runner must use `snow_hourly_snowfall_depth_sum_m` for
  canonical `hrsnow` parity and must not use
  `snow_hourly_snowfall_water_equiv_sum_m` inside window partitioning.
- Current openWEPP sources publish snowfall depth and derived water-equivalent
  summaries separately.
