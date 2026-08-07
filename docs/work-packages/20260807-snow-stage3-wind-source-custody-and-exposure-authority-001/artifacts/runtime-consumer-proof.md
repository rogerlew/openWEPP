# Runtime Consumer Proof

Status: `complete`.

Evidence mode: `Static + Ran`.

1. `openwepp-input-contract/src/parsers/climate.rs:629` parses CLI token 10
   (`w-vl`) directly as `vwind`.
2. `openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs:60,73`
   assigns `vwind_m_s: day.vwind` without conversion.
3. `openwepp-runner/.../00a_snow_frost_authority_impl.rs:464` assigns Stage 3
   `wind_m_s: forcing.vwind_m_s`.
4. `openwepp-runner/.../00d_authority_runtime_impl.rs:861` alone creates
   `fwv_m_s = forcing.vwind_m_s * 4.87 / ln(67.8*10-5.42)` for PMET and uses it
   locally in evapotranspiration calculations. The snow source contains no
   `fwv_m_s` reference.

Ran: `snow_stage3_wind_source_custody_contract` asserts these exact source
relationships and passed `3/3`. Old compatibility path check: no adjusted PMET
wind reaches the named Stage 3 consumer; raw CLI custody is the real path.
