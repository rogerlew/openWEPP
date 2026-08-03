# Acquisition Handoff

The first actionable item is to accept the Terms of Use on both CDS dataset
pages, provision a valid CDS API credential outside this repository, install
`cdsapi` in the repo-local environment, and run the frozen site/year time-series
request:

```bash
.venv/bin/python docs/work-packages/20260803-snow-hourly-era5-diagnostic-001/tools/retrieve_cds_timeseries.py
```

The command refuses overwrites and writes only under
`target/snow_hourly_era5_diagnostic/`. Do not commit credentials or downloaded
data. A separately authorized result-bearing package must directly open every
file and validate provider metadata, native nearest-cell identity, grid
elevation/orography from a separately selected compatible official ancillary,
the exact dataset/site/year cross-product, every expected
UTC hour, variables, units, de-accumulated energy semantics, SHA-256, and the
bounded shortwave-negative rule with per-file correction counts/energy. This intake
tool never emits `ADMITTED`; file presence only changes its status to
`CANDIDATE_DATA_PRESENT_VALIDATION_REQUIRED`. Remove the prior receipt and
rerun:

```bash
rm docs/work-packages/20260803-snow-hourly-era5-diagnostic-001/artifacts/data-availability-audit.json
.venv/bin/python docs/work-packages/20260803-snow-hourly-era5-diagnostic-001/tools/audit_data_availability.py
```

Retrieval must retain source dataset, request, UTC timestamps, grid-cell
identity, units, file SHA-256, and download time. It must cover all 24 hours and
the full site periods. Do not interpolate missing hours or silently repair
invalid time-series energy values. Once a separately reviewed validator passes, implement the
preregistered radiation-first comparison in that result-bearing increment.
