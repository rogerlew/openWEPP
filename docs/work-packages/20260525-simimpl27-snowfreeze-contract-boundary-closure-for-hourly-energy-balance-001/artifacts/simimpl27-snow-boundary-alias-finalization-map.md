# SIMIMPL27 Snow Boundary Alias Finalization Map

Status: complete
Evidence mode: static
Date: 2026-05-25

## Static
- Ratified typed aliases already implemented in openWEPP runtime symbol map:
  - `snow.options.rst`, `snow.options.newsnw`, `snow.options.ssd`,
    `snow.options.snow_file_present`
  - `snow.runtime_swe`
  - `frost.runtime_dfrost`, `frost.runtime_dthaw`, `frost.runtime_nft`,
    `frost.runtime_ws_frz`, `frost.runtime_infcap_frz`
  - `tmax`, `tmin`
  - flux `S`
- Ratified reserved aliases for hourly migration implementation scope
  (SIMIMPL28/SIMIMPL29):
  - `snow.hourly.*` (depth/density/snowfall/melt/rain families)
  - `winter.hourly.*` (thermal/radiation/cloud families)
  - `frost.hourly.*` (heat-flow/layered-depth bookkeeping families)
- Migration-scope boundary closure statement:
  - non-promotable boundary/API naming ambiguity is removed;
  - implementation of reserved hourly aliases remains queued.

## Ran
- not run
