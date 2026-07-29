# CAL-07F Source Custody

Evidence class: `Ran + Static`

## Observation identity

CAL-07F reuses source bytes already retained by predecessor packages:

- CAL-07 Data Record 4 daily product:
  `bezamahafaly_DB_1000_1day.csv`;
- CAL-07 ROI definition and site metadata;
- CAL-07E eight-row Data Record 5 transition subset; and
- CAL-07D validated model crossing inventory.

`dependency-manifest.csv` records current SHA-256 and byte size for all six
dependencies. CAL-07F does not download a newer mutable archive.

## Provisional status

The daily and transition products remain provisional and were processed
26 July 2026. Results apply to those exact retained bytes. A later PhenoCam
archive is not presumed identical.

## Daily-product integrity

- 731 consecutive rows cover all of 2024 and 2025.
- All `gcc_mean` and `gcc_90` outlier flags are zero.
- All daily rows have non-missing image counts.
- Twenty-one 2025 rows have provider interpolation flag `1`; the other 710
  rows have no interpolation flag.
- One ROI mask is recorded, effective 4 July 2023 through the open-ended
  provider end date.

## Empirical role

Every observation remains `DIAGNOSTIC_ONLY`. CAL-07F does not reassign either
year to calibration or independent validation.
