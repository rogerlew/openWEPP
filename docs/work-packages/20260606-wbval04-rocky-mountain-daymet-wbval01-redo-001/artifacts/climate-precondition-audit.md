# Climate Precondition Audit

Status: complete - passed

Evidence mode: mixed `Static:` and `Ran:`

Purpose: prove that the current
`/wc1/runs/in/indispensable-presenter` climate artifacts are publication-safe
for openWEPP before any WBVAL01 redo run starts.

Static:

- Audited artifacts:
  - `/wc1/runs/in/indispensable-presenter/climate/wepp.cli`
  - `/wc1/runs/in/indispensable-presenter/climate/wepp_cli.parquet`
  - `/wc1/runs/in/indispensable-presenter/climate/daymet_1990-1995.parquet`
  - `/wc1/runs/in/indispensable-presenter/climate/daymet_radiation_toa_normalization_wepp.csv`
- Artifact mtimes:
  - `wepp.cli`: `2026-06-06T14:45:22`, `154312` bytes.
  - `wepp_cli.parquet`: `2026-06-06T14:45:22`, `109919` bytes.
  - `daymet_1990-1995.parquet`: `2026-06-06T14:45:22`, `168401` bytes.
  - `daymet_radiation_toa_normalization_wepp.csv`: `2026-06-06T14:45:21`, `6835` bytes.
- Daymet parquet includes the required provenance columns:
  `srad_source(l/day)`, `srad_toa_bound(l/day)`,
  `srad_toa_publication_bound(l/day)`, `srad_toa_normalized`,
  `srad_toa_normalization_reason`, and
  `srad_toa_bound_latitude(deg)`.

Ran:

- Parsed `/wc1/runs/in/indispensable-presenter/climate/wepp.cli`.
- Parsed `/wc1/runs/in/indispensable-presenter/climate/wepp_cli.parquet`.
- Recomputed baseline `sunmap.r3` with the same formula as
  `crates/openwepp-runner/src/hillslope/mod.rs::legacy_sunmap_horizontal_radpot_ly`.
- CLI latitude: `43.73`.
- CLI rows: `2191`.
- CLI date range: `1990-01-01` through `1995-12-31`.
- Text CLI and parquet `rad` mismatch count: `0`.
- Text CLI rows above exact baseline `sunmap.r3`: `0`.
- Parquet CLI rows above exact baseline `sunmap.r3`: `0`.
- Minimum CLI margin below exact `sunmap.r3`: `0.000293 Ly/day`.
- Maximum CLI excess: `-0.000293 Ly/day`.
- Daymet rows: `2191`.
- Daymet normalized rows: `53`.
- Daymet publication-bound minimum margin below exact TOA/sunmap bound:
  `0.00029287739710071037 Ly/day`.
- Normalization CSV rows: `53`.
- Normalization CSV maximum original excess above exact bound:
  `96.1239434910584 Ly/day`.

Closest CLI rows after publication-safe rebuild:

| Date | CLI rad Ly/day | Baseline sunmap.r3 Ly/day | Margin Ly/day |
|---|---:|---:|---:|
| 1991-03-30 | 703.000000 | 703.000293 | 0.000293 |
| 1995-03-30 | 703.000000 | 703.000293 | 0.000293 |
| 1990-03-01 | 518.000000 | 518.001190 | 0.001190 |
| 1993-03-01 | 518.000000 | 518.001190 | 0.001190 |
| 1991-05-16 | 938.000000 | 938.027536 | 0.027536 |
| 1993-03-22 | 652.000000 | 652.042515 | 0.042515 |
| 1994-03-27 | 684.000000 | 684.045911 | 0.045911 |
| 1990-05-09 | 913.000000 | 913.051691 | 0.051691 |
| 1993-05-09 | 913.000000 | 913.051691 | 0.051691 |
| 1990-02-18 | 453.000000 | 453.068716 | 0.068716 |

First normalized Daymet provenance rows:

| Date | Original Ly/day | Exact bound Ly/day | Publication bound Ly/day | Published Ly/day | Reason |
|---|---:|---:|---:|---:|---|
| 1990-02-18 | 486.398513 | 453.068716 | 453.0 | 453.0 | daymet_over_toa |
| 1990-02-19 | 502.729494 | 458.706289 | 458.0 | 458.0 | daymet_over_toa |
| 1990-02-20 | 508.236181 | 464.403132 | 464.0 | 464.0 | daymet_over_toa |
| 1990-02-24 | 499.076201 | 487.742694 | 487.0 | 487.0 | daymet_over_toa |
| 1990-02-25 | 506.258626 | 493.705219 | 493.0 | 493.0 | daymet_over_toa |
| 1990-02-26 | 522.388163 | 499.714526 | 499.0 | 499.0 | daymet_over_toa |
| 1990-02-28 | 532.304078 | 511.864732 | 511.0 | 511.0 | daymet_over_toa |
| 1990-03-01 | 552.755917 | 518.001190 | 518.0 | 518.0 | daymet_over_toa |
| 1990-03-02 | 529.894113 | 524.175555 | 524.0 | 524.0 | daymet_over_toa |
| 1990-03-25 | 725.261664 | 671.295590 | 671.0 | 671.0 | daymet_over_toa |

Disposition: climate precondition passed. WBVAL04 validation was allowed to
proceed.
