# Harness Design

Evidence class: Static/Ran.

Status: implemented.

## Command Surface

Tool: `tools/snowfreeze_observed/observed_harness.py`.

Commands:

- `fetch`: explicit network-enabled acquisition into `target/snowfreeze_observed/`.
- `normalize`: converts acquired public sources into checked-in normalized CSV,
  manifest, and provenance records.
- `validate`: offline schema/provenance/checksum validator for normal tests.
- `compare`: runs one fixture through `openwepp-cli-hill`, extracts WAT `frdp`,
  aligns to observation dates, and writes JSON/Markdown comparison reports.

Normal tests do not require network.

## Runfile Strategy

The committed site `pN.run` files are legacy line-oriented WEPP recipes. The
current runner requires TOML `openwepp-hillslope-runfile-v1`, so the harness
generates a temporary TOML runfile pointing at each fixture's `pN.{sol,man,slp,cli}`
files and invokes the current tree.

Default comparison surface:

```bash
openwepp-cli-hill --run-dir <fixture> --run-file <generated.run> \
  --output-dir <output> --legacy-sidecar-discovery \
  --direct-production-executor
```

`--runtime compatibility` is available only as an explicit flagging surface; it
is not the acceptance target. `--legacy-sidecar-discovery` preserves fixture
sidecars (`snow.txt`, `pmetpara.txt`, `gwcoeff.txt`) without treating legacy
frost output as an acceptance target.

## WAT Alignment

WAT `frdp` is read from parquet in millimeters and normalized to meters.

The WAT `year` column is the simulation-year index. Calendar date alignment uses
`water_year` plus month/day:

- `month >= 10`: `calendar_year = water_year - 1`;
- `month <= 9`: `calendar_year = water_year`.

The loader rejects duplicate modeled calendar dates instead of silently
overwriting rows.

## Metric Semantics

Reports separate method-specific authority:

- frost tube rows produce direct `frdp - observed_frost_depth_m` magnitude residuals;
- soil-temperature rows produce `frdp <= observed_isotherm_depth_m` upper-bound checks;
- right-censored sensor-depth rows are counted and excluded from magnitude and
  upper-bound residual metrics;
- seasonal timing summaries report onset, thaw, and frozen-duration residuals
  on the observation-date series.

`OPENWEPP-DEFECTIVE` remains disallowed while modeled snow depth is unavailable.
WAT `Snow-Water` is SWE and is not a snow-depth diagnostic.

## Exercised Runs

Direct-production comparison results were generated under:

- `target/snowfreeze_observed_compare_site1_direct/`
- `target/snowfreeze_observed_compare_site2_direct/`
- `target/snowfreeze_observed_compare_site3_direct/`
- `target/snowfreeze_observed_compare_site4_direct/`
- `target/snowfreeze_observed_compare_site5_direct/`

Sites 1, 2, and 5 reached comparison reports. Sites 3 and 4 failed before
comparison with direct-runtime storage-reconciliation guard errors and are
recorded as `HARNESS-SURFACE-MISMATCH`, not observation-model defects.
