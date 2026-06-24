# Model Defect Ledger

Evidence class: Static/Ran.

Status: no accepted model defects.

Do not fill this with raw observation disagreements. A row may become
`OPENWEPP-DEFECTIVE` only when `INV-SNOWFREEZE-047` and ADR-0017 criteria are
met.

| Defect ID | Site | Dates/window | Method | Observed issue | Snow-control status | Censoring status | Independent authority | Verdict | Follow-up |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| n/a | `site1_sleepers_south_field_vt` | 1983-11-27 through 2017-04-17 | frost tube | direct run max abs residual 0.2641958259 m | `UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC` | none in residual rows | `SC-SNOWFREEZE-001 INV-SNOWFREEZE-047` | `UNRESOLVED` | Add modeled snow-depth diagnostic, then rerun. |
| n/a | `site2_sleepers_w9_hardwood_vt` | 1993-11-30 through 2020-03-16 | frost tube | direct run max abs residual 0.3838127879 m | `UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC` | none in residual rows | `SC-SNOWFREEZE-001 INV-SNOWFREEZE-047` | `UNRESOLVED` | Add modeled snow-depth diagnostic, then rerun. |
| n/a | `site5_reynolds_creek_us_rls_id` | 1982-01-06 through 1996-10-01 | soil temperature | 104 upper-bound exceedances, max margin 0.0494782283 m | `UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC` | none in compared rows | `SC-SNOWFREEZE-001 INV-SNOWFREEZE-047` | `UNRESOLVED` | Add modeled snow-depth diagnostic; treat soil-temp only as timing/upper-bound authority. |

Harness-surface blockers, not model defects:

| Site | Runtime surface | Failure | Verdict | Follow-up |
| --- | --- | --- | --- | --- |
| `site3_scan_mandan_nd` | `direct-production-executor` | lane 1 day 487: `storage_reconciliation.frost_storage_projection_theta_m must be nonnegative` | `HARNESS-SURFACE-MISMATCH` | Resolve direct-runtime storage-reconciliation guard failure, then rerun. |
| `site4_ggd498_morris_mn` | `direct-production-executor` | lane 1 day 10727: `storage_reconciliation.frost_storage_projection_theta_m must be nonnegative` | `HARNESS-SURFACE-MISMATCH` | Resolve direct-runtime storage-reconciliation guard failure, then rerun. |

Allowed verdicts:

- `PASS`
- `HARNESS-SURFACE-MISMATCH`
- `OPENWEPP-DEFECTIVE`
- `UNRESOLVED`
- `SOURCE-BLOCKED`
