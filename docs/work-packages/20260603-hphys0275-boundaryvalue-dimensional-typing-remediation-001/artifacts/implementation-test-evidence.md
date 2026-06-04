# Implementation Test Evidence

Status: completed/HOLD
Evidence mode: static + ran

Static: Implemented runtime typing for the selected high-risk hillslope
climate and SIMIMPL28 producer seams:

- Daily climate: `prcp`, `rad`, `tmax`, `tmin`, `tdpt`, `vwind`, `stmdur`,
  `stmstr`, `timem_####`, `mxint`, `avrint`, and `intsty_####`.
- SIMIMPL28 hourly: `winter.hourly.rad_mj_m2_####`,
  `winter.hourly.air_temp_c_####`,
  `winter.hourly.cloud_fraction_####`, `snow.hourly.rain_m_####`, and
  `snow.hourly.snowfall_m_####`.
- `wind` direction remains scalar/follow-up because the first wave only typed
  wind speed and no direction-specific wrapper exists yet.
- Watershed-prefixed climate aliases remain scalar/follow-up and are split in
  the registry instead of being overclaimed as typed.

Ran:

- `cargo fmt --check` -> pass.
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- `cargo test -p openwepp-unit-boundary` -> pass.
- `cargo test -p openwepp-kernel-contract` -> pass.
- `cargo test -p openwepp-hillslope-orchestrator runtime_inputs` -> pass.
- `cargo test --test hphys0275_boundary_value_dimensional_typing_contract`
  -> pass.
- `tools/release/check_unit_registry.sh` -> pass.
- `cargo deny check` -> pass with existing duplicate/unmatched-license warnings.
- `markdown-doc lint --path ...` scoped docs/work-package paths -> pass.
- `cargo test --workspace` -> fail/HOLD only in the known
  `pl14s_tier_a_candidate_emission_and_replay_contract` SIMIMPL18 tests at
  `HKERNEL-WB11-ET-E-003`.
