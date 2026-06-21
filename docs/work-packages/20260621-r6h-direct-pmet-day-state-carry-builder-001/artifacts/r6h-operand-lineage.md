# R6H Operand Lineage

Status: complete-with-hold.

| Output field | Direct operand | Producer | Consumer | Compatibility alias rejected | Evidence |
|---|---|---|---|---|---|
| `Es` | `DirectHydrologyProjectionState::soil_evaporation_m` | Direct R4N ET surface/root compute from `DirectEvapotranspirationPmetInputs` | `build_hillslope_wat_row_from_direct_publication` | WB13 `Es`, compatibility ET runtime symbols | Held: reduced to `Es` only; direct and compatibility differ by sub-ulp carried-layer PMET state. |
| `Total-Soil` | `DirectHydrologyProjectionState::total_soil_m` | Direct R4P/Q/Z hydrology projection aggregates direct ET layer state plus frozen water | Direct hydrology projection, then WAT row builder | WB13 storage columns, stale logical profile totals | Complete: current-fixture bits match after interleaved carry. |
| `SoilWaterTotal` | `DirectHydrologyProjectionState::soil_water_total_m` | Direct R4P/Q/Z hydrology projection | Direct hydrology projection, then WAT row builder | WB13 storage columns, stale logical profile totals | Complete: current-fixture bits match after interleaved carry. |
| `wepp_id` | `DIRECT_WAT_WEPP_ID` | Direct WAT id authority | Direct WAT row builder | WB13 row identity or fixture-only constant | Held for broader authority; current compatibility builder also emits `1`. |

## Lineage Requirements

- Units and normalization basis must be recorded for every WAT operand touched.
- PMET operands must point to direct-carried state after prior-day commit.
- Any private seed-surface symbol used by the direct builder must appear in the
  allowlisted no-compatibility ledger.

## Allowlisted Private Seed Symbols

The direct runner still uses a private `HillslopeWritebackSurface` to call the
existing WB11 PMET seed helper. These symbols are inputs to typed direct day
input construction, not WAT publication authority:

- `wb11_nsl`, `nsl`, and `wb11_soil_water`;
- `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`,
  `wb18_perc_ssc_####`, `wb18_perc_frozen_depth_####`,
  `wb18_perc_frzw_####`;
- `wb19_dg_####`, `wb19_thetdr_####`, `wb19_por_####`,
  `wb19_thetfc_####`, `wb19_coca_####`, and legacy `coca_####`.

Rejected alias: post-scheduler compatibility WB13 rows, compatibility runtime
surfaces, writeback payloads, writer rows, and output rows are not read by the
direct WAT builder.
