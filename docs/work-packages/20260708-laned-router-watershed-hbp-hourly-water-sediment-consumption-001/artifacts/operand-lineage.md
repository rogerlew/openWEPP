# Operand Lineage

Status: `EXECUTED`
Evidence mode: `Static` plus `Ran` consumer tests.

| Operand | Units | Shape | Source authority | Producer | Consumer | Rejected aliases |
| --- | --- | --- | --- | --- | --- | --- |
| `event.hourly_runoff_volume_m3` | m3 per hour slot | 24 slots | `SC-INFILE-HBP-001` v0.2.3, `SC-SED-001#INV-SED-014`, `SC-ROUTE-001` rev 49/50 | active Lane D HBP producer; direct publication computes `V_h = runvol_m3 * w_h` | WS10 inlet superposition uses `max_h(sum V_h)/3600`, `sum V_h`, and active-hour span | daily `runvol_m3`, `peak_runoff_m3_s * duration`, triangular fallback when any hourly surface exists, DC01 diagnostic shape |
| `event.hourly_sediment_mass_kg` | kg per hour slot | 24 slots | `SC-INFILE-HBP-001` v0.2.3, `SC-SED-001#INV-SED-014`, `SC-ROUTE-001` rev 49/50 | active Lane D hourly erosion/export surface | WS10 sediment intake uses `sum S_h`; qsed time base uses active span of superposed `S_h` | `sediment_concentration * runvol` reconstruction, daily `tdet - tdep` as the consumer mass when `S_h` exists, zero/synthetic fills |

Independent reconstruction:

- `wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity`
  parses the generated HBP and checks `sum hourly_sediment_mass_kg =
  total_detachment_kg - total_deposition_kg`.
- `mt3_hourly_pair_distribution_changes_channel_water_and_sediment_outputs`
  constructs equal-daily-total hourly pairs with different distributions and
  proves routed peak and `qsed` change.
- `mt3_watershed_cli_hbp_hourly_pair_reaches_channel_consumer` writes two
  schema-1.1 HBP fixtures with identical scalar/daily values, runs the
  production watershed CLI, and proves `ebe_pw0` peak runoff and sediment yield
  change when only `V_h`/`S_h` distribution changes.
