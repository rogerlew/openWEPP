# Implementation Evidence

Evidence class: Static + Ran.

## Production Wiring

- `SnowAlbedoState::validate()` and `shortwave_absorbed_fraction()` were added
  to the hydrology albedo module.
- Typed snow partition inputs now carry `snow_melt_model`,
  `snow_albedo_model`, `snow_albedo_state`, and `underlying_surface_albedo`.
- The typed snow melt helper updates the 05C albedo state hourly for active
  opt-in snow and applies `1 - snow_albedo` only to the CoE `amelt` term.
- The compatibility/symbol-surface path passes an absorbed fraction of `1.0`,
  preserving legacy melt behavior.
- Typed lineage now exposes raw melt, redistributed melt, bounded SWE loss, and
  albedo state after update.
- Direct runtime snow state carries the optional albedo state through storage
  inputs, state mutation, downstream operands, and shadow projection.
- Runner direct publication still supplies
  `SnowMeltModel::LegacyCoe` for production day inputs.

## Guardrails Preserved

- No parser or CLI activation was added.
- No output schema was added.
- No coefficient fitting or site-specific tuning was added.
- No radiation rescaling, clipping, or snow-only scalar was added.
- `dense_slow_melt_v1` remains limited to snowbench and documentation.

## Source Scan

Command:

```sh
rg -n "dense_slow_melt_v1|snow_melt_model:|SnowMeltModel::|CoeShortwaveAlbedoV1|LegacyCoe|snow_melt_shortwave_absorbed_fraction|0\\.0607|qwet|frzftp" crates tests docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md -S
```

Result summary:

- Production runner selection: `SnowMeltModel::LegacyCoe`.
- Opt-in selector and absorbed-fraction logic present in typed hydrology code
  and tests.
- `dense_slow_melt_v1` occurrences are snowbench, contract negative-benchmark
  text, and tests only.
- No production `qwet` or `frzftp` implementation was introduced.
