# Target-Window Lineage Schema

Status: complete

Evidence mode: static

Static:

- Baseline diagnostic records are emitted through `wepp_observe(tag, year, sdate, ielmt, ichan, iseg, v1, v2)`.
- `H298_*` tags are parsed by `artifacts/hphys0298_paired_lineage_partition.py` and grouped by `(year, julian_day, tag)`.
- Hourly baseline rows store `hour` in `iseg`; daily rows store `0`.
- Baseline units:
  - `H298_RAW_A.v1`: `hrmlt(hour,iplane)` in `m`.
  - `H298_RAW_A.v2`: `hrrain(hour)` in `m`.
  - `H298_RAW_B.v1`: `hrsnow(hour)` in `m`.
  - `H298_RAW_B.v2`: `snodpt(iplane)` in `mm` legacy depth state.
  - `H298_NEG_A`: `pstvML`, `ngtvML` in `m`.
  - `H298_NEG_B`: `pstvhr` count and correction factor.
  - `H298_POST_A`: `wmelt(iplane)`, `totmel` in `m`.
  - `H298_WBH_C.v1`: WB hourly `rm` in `mm`.
  - `H298_WBH_C.v2`: `runoff(iplane)` legacy published runoff value.
- openWEPP trace rows are post-WB13 rows from the opt-in JSONL trace with required fields `snow_hourly_melt_raw_m`, `snow_hourly_rain_sum_m`, `snow_hourly_snowfall_water_equiv_sum_m`, `snow_routed_melt_m`, `snow_post_winter_rain_m`, `wb13_rm_mm`, and `wb13_q_mm`.
- Missing required openWEPP trace fields increment `openwepp_trace_missing_field_count` and force `trace-gap`/`UNRESOLVED`; the harness no longer zero-fills missing required trace fields into closure.
- Ledger presentation normalizes lineage comparisons to `mm`.
