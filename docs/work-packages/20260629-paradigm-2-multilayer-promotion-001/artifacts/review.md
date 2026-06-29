# Review

Status: `REVIEWED-PASS`

Evidence class: Static + Ran.

This artifact records review findings and dispositions.

## Findings

No blocking findings.

## Checks

- Activation posture remains opt-in. No default-on selector or runfile/user CLI
  exposure was added.
- `OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1` is now a
  supported internal production selector.
- Unknown Stage 3 selector values continue to fail closed through the existing
  builder path.
- The supported output field is WAT parquet only; HBP/watershed/in-stream
  serialization remains deferred.
- Default/rollback real outputs publish `MeltwaterTemperature` as null.
- The opt-in arm does not require Stage 1 density and does not set
  `OPENWEPP_SNOWDENSITY09_DENSITY_MODEL`.
- Stage 3 liquid and energy residuals are production hard gates in
  `resolve_stage3_liquid_routing`; real opt-in runs completed under those
  guards.

## Residual Risk

Runfile/WEPPpy selector exposure is intentionally deferred to the
stream-temperature program. That later package must decide the first
user-facing control and any HBP/hourly/watershed carrying format.
