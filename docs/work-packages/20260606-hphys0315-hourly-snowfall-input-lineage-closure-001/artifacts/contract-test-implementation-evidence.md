# Contract-Test Implementation Evidence

Status: complete

Evidence mode: Static + Ran

Static:

- Added `tests/integration/hphys0315_hourly_snowfall_input_lineage_contract.rs`.
- Registered the test in `Cargo.toml`.
- The test asserts:
  - `SC-SNOWFREEZE-001#INV-SNOWFREEZE-041`.
  - `SC-WATBAL-001#INV-WATBAL-089`.
  - `SC-CLIMATE-001#INV-CLIMATE-014` snowfall-depth unit authority.
  - H1/H7/H39 spring-2014 carried-row counts (`8`, `7`, `9`; total `24`).
  - Baseline `hrsnow = 0.0007454545120708644 m`.
  - OpenWEPP homologous `snow.hourly.snowfall_m_0011 = 0.0 m`.
  - ADR0017 verdict `UNRESOLVED`.
  - `production_edit_authorized=false`.
  - Follow-on owner `HPHYS0317`.
  - Required gate/review/verification closeout records.

Ran:

- `cargo test --test hphys0315_hourly_snowfall_input_lineage_contract hphys0315_contract_authority_is_registered -- --nocapture`
  completed with exit status `0` as the pre-implementation contract gate.
