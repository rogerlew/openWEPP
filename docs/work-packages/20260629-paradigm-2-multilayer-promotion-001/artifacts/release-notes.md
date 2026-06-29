# Release Notes

Status: `FINAL`

Evidence class: Static.

- Added a production-supported opt-in water-temperature mode:
  `OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1`.
- The no-env bulk snow default remains unchanged.
- The opt-in mode publishes nullable hillslope WAT parquet
  `MeltwaterTemperature` in `degC` when routed meltwater temperature is
  produced.
- HBP binary/watershed serialization and full in-stream temperature routing
  remain deferred to the stream-temperature program.
- Default/rollback WAT output includes the nullable field with null values only.
- The opt-in arm reconfirmed the current snow guardrail exactly (`15` / `179`,
  `0` worse robust cells) and H2637 performance passed at `70.65 s`.
