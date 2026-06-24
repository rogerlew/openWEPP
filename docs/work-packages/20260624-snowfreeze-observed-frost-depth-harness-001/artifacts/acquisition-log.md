# Acquisition Log

Evidence class: Ran.

Status: complete for all five redistributable pilot sources; Dun-2010 remains
request-only and out of the required local gate.

Access date: 2026-06-24.

## Commands

```bash
.venv/bin/python tools/snowfreeze_observed/observed_harness.py \
  --cache target/snowfreeze_observed fetch

.venv/bin/python tools/snowfreeze_observed/observed_harness.py \
  --cache target/snowfreeze_observed normalize \
  --observations-dir tests/fixtures/snowfreeze_observed/observations

.venv/bin/python tools/snowfreeze_observed/observed_harness.py \
  validate --observations-dir tests/fixtures/snowfreeze_observed/observations
```

## Source Results

| Source | Route | Raw cache | Normalized rows | Result |
| --- | --- | --- | ---: | --- |
| USGS Sleepers River DOI `10.5066/P96753GI` | ScienceBase JSON + attached CSV/XML files | `target/snowfreeze_observed/raw/sleepers/` | 592 | `PASS` |
| NSIDC GGD498 DOI `10.7265/1mcs-q536`, station 10 Morris | NSIDC FTP flat file + README + user guide | `target/snowfreeze_observed/raw/ggd498/` | 232 | `PASS` |
| NRCS SCAN Mandan `2020:ND:SCAN` | AWDB REST `STO:*:*` daily JSON | `target/snowfreeze_observed/raw/scan/` | 10,643 | `PASS` |
| USDA-ARS Reynolds Creek soil temperature station 127 | Data.gov / Figshare `soiltemperature.zip`; HydroShare metadata retained as optional metadata only | `target/snowfreeze_observed/raw/reynolds/` | 4,356 | `PASS` |

## Provenance Locks

Checked-in provenance records live under
`tests/fixtures/snowfreeze_observed/observations/provenance/` and include:

- source URL/DOI;
- access date;
- license or source terms;
- parser version `snowfreeze-observed-harness-v1`;
- normalized row count;
- raw file paths in the local cache;
- raw file byte counts and SHA-256 checksums where files were acquired;
- normalized CSV byte counts and SHA-256 checksums;
- parser assumptions and site/station mapping notes.

Raw files remain in `target/` and are not committed.
