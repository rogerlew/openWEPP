# Data Custody Audit

Ran: `tools/audit_data_availability.py` on 2026-08-03.

Result: `CANDIDATE_DATA_PRESENT_VALIDATION_REQUIRED`.

Authenticated CDS retrieval completed without reading or recording any
credential value. Eight complete long-range NetCDF files are retained, plus
five redundant Mica annual downloads excluded from the scientific cohort.
`cdsapi`, `xarray`, and `netCDF4` are installed in the untracked `.venv`; only
`cfgrib` is absent. The machine-readable evidence is
`data-availability-audit.json`; its frozen acquisition-manifest hash binds the
result to the four sites, eight valid hourly fields, and required elevation
ancillary.

Existing Daymet/gridMET-derived fixtures are separately bound in
`retained-comparator-audit.md`; they are not ERA5 custody and were not relabelled.
No synthetic reanalysis or snow-result table was produced. Direct content
validation and the official elevation ancillary now pass in
`validated-source-inventory.json`; independent review/verification remains open.
