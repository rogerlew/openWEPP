# Hourly Distribution Sensitivity

Status: `EXECUTED`
Evidence mode: `Ran`

Test:
`mt3_hourly_pair_distribution_changes_channel_water_and_sediment_outputs`.

Construction:

- Spike case: `V_10 = 7200 m3`, `S_10 = 240 kg`; all other hours zero.
- Spread case: hours `8..11` each carry `V_h = 1800 m3`,
  `S_h = 60 kg`.
- Daily totals are identical: `sum V_h = 7200 m3`,
  `sum S_h = 240 kg`.

Named consumer surfaces:

- `RoutedChannelState.channel_inflow_m3` stays equal between cases.
- `RoutedChannelState.peak_discharge_m3_s` changes because water timing is
  consumed.
- `RoutedChannelSedimentState.qsed_kg_s` changes because sediment timing is
  consumed.

Ran:

- `cargo nextest run --test wshedw5_typed_watershed_runtime_contract`:
  18 passed.
- `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract`:
  1 passed. The production CLI proof holds scalar HBP peak/duration and daily
  totals constant, then proves the watershed output changes when only the
  schema-1.1 hourly distribution changes.
