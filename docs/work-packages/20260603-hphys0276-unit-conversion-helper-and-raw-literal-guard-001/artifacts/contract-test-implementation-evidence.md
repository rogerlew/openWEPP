# Contract Test Implementation Evidence

Status: completed
Evidence mode: Static + Ran

Static: contract-derived tests were added before final production validation.

## Added Tests

- `crates/openwepp-unit-boundary/src/lib.rs`
  - `radiation_conversion_direction_uses_langley_to_mj_m2`
  - `legacy_snow_melt_conversion_helpers_preserve_direction`
  - `first_wave_length_time_rate_helpers_preserve_direction`
  - `first_wave_helpers_reject_invalid_domains`
  - `snow_density_depth_conversions_are_directional`
- `tests/integration/hphys0276_raw_unit_conversion_guard_contract.rs`
  - guard rejects unauthorized raw radiation literal
  - guard rejects equivalent raw literal spellings
  - guard accepts helper-based source
  - guard accepts documented exception marker
  - guard does not overapply class-bound allow markers

Ran:
- `cargo test -p openwepp-unit-boundary`: pass, 24 tests.
- `cargo test --test hphys0276_raw_unit_conversion_guard_contract`: pass, 5 tests.
