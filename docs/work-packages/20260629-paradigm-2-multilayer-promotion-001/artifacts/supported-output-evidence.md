# Supported Output Evidence

Status: `RAN-PASS`

Evidence class: Ran + Static.

This artifact records schema metadata, direct-publication consumer-path proof,
and real-run evidence for the supported `MeltwaterTemperature` WAT parquet
field.

## Static Consumer Path

| Surface | Evidence |
| --- | --- |
| Contract | `SC-SNOWFREEZE-001` v112 `INV-SNOWFREEZE-082` ratifies nullable WAT parquet `MeltwaterTemperature` in `degC`. |
| Unit registry | `MeltwaterTemperature` is registered as typed boundary alias `hillslope_wat.MeltwaterTemperature` with units `degC`. |
| Direct publication | `DirectPublicationWaterTemperatureOperands` reads `snow_coupling_shadow_projection.stage3_diagnostics.meltwater_temperature_c` and rejects positive/non-finite values. |
| Runner WAT mapping | Direct rows map `row.water_temperature.meltwater_temperature_c`; compatibility/default rows publish `None`. |
| HBP/watershed | No HBP/watershed schema or serialization path was added. |

## Real Output Evidence

Observed guardrail run candidate WAT files:

```text
files=10
rows=159986
missing_field_files=0
nonnull_temperatures=27965
min_temperature_c=0.0
max_temperature_c=0.0
field_type=double
field_nullable=true
field_units=degC
```

Observed guardrail default WAT files:

```text
files=10
rows=159986
missing_field_files=0
nonnull_default_temperatures=0
```

H2637 promoted opt-in WAT:

```text
rows=235961
nonnull_temperatures=35730
min_temperature_c=0.0
max_temperature_c=0.0
field_nullable=true
field_units=degC
```

Interpretation: the supported field exists on real WAT parquet outputs; the
default/rollback path leaves it null; the opt-in arm emits finite, non-positive
ripe-melt temperatures only.
