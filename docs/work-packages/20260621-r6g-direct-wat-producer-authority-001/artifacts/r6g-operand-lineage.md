# R6G Operand Lineage

Status: executed-held.

| Output field | Direct operand | Producer | Consumer | Compatibility alias rejected | Evidence |
|---|---|---|---|---|---|
| `wepp_id` | `DIRECT_WAT_WEPP_ID` | Direct WAT row builder constant for the current fixture | `build_hillslope_wat_row_from_direct_publication` | `DirectPublicationDayRow.hillslope_id`, WB13 row identity | Focused R6G WAT evidence no longer reduces on `wepp_id`; full canonical multi-OFE WAT id authority is accepted follow-up. |
| `year` | `simulation_year_from_calendar_year(row.calendar.year, simulation_start_year)` | Direct WAT row builder from first direct publication day | `build_hillslope_wat_row_from_direct_publication` | Calendar year copied as output year, WB13 row year | Focused R6G WAT evidence no longer reduces on `year`. |
| `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore` | Parsed layer/profile symbols and authoritative layer FC derivation | `direct_publication_profile_inputs` and helpers in `04_direct_publication.rs` | Direct hydrology projection, then WAT row builder | WB13 profile columns and compatibility runtime surfaces | Profile fields are absent from the R6G reduced residual set. |
| First-day `Es` | `DirectEvapotranspirationComputeInputs` from private pre-scheduler seed surface and parsed climate/static inputs | `direct_publication_evapotranspiration_inputs` | Direct runtime ET surface, direct hydrology projection, WAT row builder | WB13 `Es` and compatibility ET runtime symbols | First WAT row `Es` equals compatibility bit-for-bit. |
| First-day `Total-Soil`, `SoilWaterTotal` | Direct layer state after root uptake plus residual liquid water and frozen storage | `aggregate_storage_from_layers` in `direct_runtime/projection.rs` | Direct hydrology projection, then WAT row builder | WB13 storage columns and stale logical profile totals | First WAT row storage equals compatibility bit-for-bit; residual-water regression covers the aggregate. |
| Day-2 `Es`, `Total-Soil`, `SoilWaterTotal` | Intended: PMET component inputs constructed from direct-carried layer state after day-1 commit | Missing dynamic/interleaved direct publication day-input builder | Direct runtime ET surface and WAT row builder | Compatibility WB13/runtime ET/storage aliases | Remaining exact hold field set is `Es`, `Total-Soil`, `SoilWaterTotal`. |

## Producer Binding Notes

- `build_retained_direct_publication_frame` now receives the execution lane and
  climate request so it can build day inputs without reading compatibility
  output rows.
- `direct_publication_day_inputs` builds a private direct seed surface from
  parsed static runtime inputs plus daily climate surface, then seeds WB11
  runtime inputs into that private surface before translating operands into
  typed direct process inputs.
- Day inputs after day 0 intentionally omit layer vectors so the direct runtime
  carries direct layer state forward. That closes storage carry for direct
  percolation/subsurface/projection, but PMET component construction is still
  precomputed before the prior direct day commit. That is the R6G hold boundary.
- Required profile/ET scalar inputs now fail closed when missing. Optional
  PMET storage return and frozen-depth operands remain optional only where the
  governing process contract authorizes absence.
- The current day-input vector is not yet lane-dimensional. Full R6 cutover
  must build dynamic per-lane day inputs and prove anti-alias behavior on a
  non-trivial OFE/lane fixture.
