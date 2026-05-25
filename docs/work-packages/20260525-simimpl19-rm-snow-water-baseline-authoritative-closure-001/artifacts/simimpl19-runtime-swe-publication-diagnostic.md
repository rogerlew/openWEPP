# simimpl19-runtime-swe-publication-diagnostic

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Publication target: runtime SWE state continuity rather than static sidecar
  control reuse.

## Ran
- Manifest evidence (`/tmp/simimpl18_partition_1779718558351714918/output/openwepp_hillslope_run_manifest.json`):
  - `coupling_vectors.winter.runtime_swe = 0.002775...` m,
  - `coupling_vectors.hydout_equivalent.snow_water = 2.775...` mm on day-2,
  - day-1 `Snow-Water = 4.40` mm in emitted WB13 row.
- Static `ssd=250` no longer appears as published dynamic `Snow-Water` state.
