# Worker Handoff

Status: complete for observation harness/corpus; follow-up needed for modeled
snow-depth diagnostics and direct-runtime surface blockers.

This package built the observation substrate for `GAP-SNOWFREEZE-002` without
resuming direct-vs-compatibility frost bit-parity.

## Landed State

- Fetch/normalize/validate/compare harness:
  `tools/snowfreeze_observed/observed_harness.py`.
- Tool README: `tools/snowfreeze_observed/README.md`.
- Normalized corpus:
  `tests/fixtures/snowfreeze_observed/observations/`.
- Rust offline contract:
  `tests/integration/snowfreeze_observed_frost_depth_contract.rs`.
- Root test registration: `Cargo.toml`.

## Source Status

- USGS Sleepers River: acquired and normalized for South Field and W9 hillslope.
- NRCS SCAN Mandan: acquired and normalized as `0 degC` isotherm timing/upper-bound.
- NSIDC GGD498 Morris station 10: acquired and normalized as frost-tube magnitude
  with limited simulation overlap.
- Reynolds Creek: acquired and normalized from the Data.gov / Figshare
  `soiltemperature.zip` station-127 series.
- Dun-2010 Pullman/Morris: request-only; still source-blocked and excluded from
  normal local gates.

## Current Comparison Result

Direct-production executor runs:

| Site | Result | Notes |
| --- | --- | --- |
| `site1_sleepers_south_field_vt` | `UNRESOLVED` | 392/392 observations matched; max abs frost residual 0.2641958259 m. |
| `site2_sleepers_w9_hardwood_vt` | `UNRESOLVED` | 200/200 observations matched; max abs frost residual 0.3838127879 m. |
| `site3_scan_mandan_nd` | `HARNESS-SURFACE-MISMATCH` | Direct runtime failed at lane 1 day 487 on negative `storage_reconciliation.frost_storage_projection_theta_m`. |
| `site4_ggd498_morris_mn` | `HARNESS-SURFACE-MISMATCH` | Direct runtime failed at lane 1 day 10727 on negative `storage_reconciliation.frost_storage_projection_theta_m`. |
| `site5_reynolds_creek_us_rls_id` | `UNRESOLVED` | 4,356/4,356 observations matched; 4,356 isotherm upper-bound checks; no frost-depth residual rows. |

All `UNRESOLVED` verdicts are due to missing modeled snow-depth diagnostics.
Do not promote WAT `Snow-Water` to snow depth; it is SWE.

## Next Work

Do not tune frost physics or assign a model defect from these residuals until
snow-depth control is solved.

Recommended next package:

1. Add a contract-approved modeled snow-depth diagnostic/publication surface.
2. Resolve the direct-runtime storage-reconciliation failures for sites 3 and 4.
3. Rerun all acquired sites through the direct harness.
4. Apply `INV-SNOWFREEZE-047` to separate snow-confounded rows, censored rows,
   frost-tube magnitude rows, and soil-temperature timing/upper-bound rows.
5. Only then file concrete `OPENWEPP-DEFECTIVE` frost-depth fidelity defects.
