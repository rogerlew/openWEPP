# WB20 Legacy Forward Lane Authority Provenance Map

Status: `completed`
Evidence mode: `Static`

## Legacy Authority Anchor
- Baseline worktree: `/workdir/wepp-forest_260430_baseline`
- Baseline commit: `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- Source file: `/workdir/wepp-forest_260430_baseline/src/watbal.for`

## Provenance Mapping
| Legacy authority | Legacy lines | WB20 contract authority | openWEPP runtime mapping |
|---|---|---|---|
| Runoff forcing assembly is process-output-derived and not observed-target-driven (`fin` assembled from rainfall/interception/snow/irrigation/runon/runoff terms). | `watbal.for` 343-369 | `SC-WATBAL-001` v24 WB12/WB14 lane-scoped closure rules; `SC-RUNOFFPART-001` v15 WB12 addendum | `run_runoff_reconciliation` forward-lane closure-delta residual branch |
| Water-balance diagnostic closure is equation-form residual (`watsm`, `watdif`) rather than observed-target comparator substitution. | `watbal.for` 977-989 | `SC-WATBAL-001` `INV-WATBAL-016` | `run_runoff_reconciliation` and `run_storage_reconciliation` forward-lane solver-residual closure branches |
| Daily output publication carries computed runoff/drainage/storage quantities as modeled outputs. | `watbal.for` 1077-1100 | `SC-SYSTEM-001` `INV-SYSTEM-016` evidence posture | WB20 lane manifest + replay-trace artifacts proving forward modeled closure publication |

## Note
No legacy WEPP authority row was found that requires observed closure targets as
acceptance-driving runtime inputs for Chapter-5 closure equations.
