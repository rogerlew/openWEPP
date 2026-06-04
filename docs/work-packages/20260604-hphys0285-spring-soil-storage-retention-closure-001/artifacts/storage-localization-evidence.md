# Storage Localization Evidence

Status: complete
Evidence mode: Static + Ran

## Localized Defects

Static:
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs` computed same-pass WB14 infiltration lineage inside WB18 `run_percolation` only when active snow coupling resolved true.
- That gate matched the narrower HPHYS0283 active-snowmelt scope but violated the local-liquid baseline `fin/xfin` lineage: direct rain, routed melt, and irrigation belong in same-pass layer storage before percolation. MOFE carry/runon storage-ingress lineage remains follow-up scope.
- The initial fix applied a single daily infiltration pulse before a multi-substep percolation loop; baseline hourly authority instead applies `xfin = fin / ui_LFtstpF` on each hourly/substep iteration.
- H1 release smoke after same-pass ingress exposed a second guard seam: corrected negative melt state-loss could exhaust available runtime snowpack and produce finite negative `snow.runtime_swe`, which then blocked downstream waterbalance.

## Production Corrections

Static:
- Removed active-snow state as a gate for same-pass infiltration ingress when `management.initial.params.tillay2_m` is present.
- Preserved active snow as only one contributor inside `compute_same_pass_wb14_infiltration_lineage(...)`.
- Applied same-pass infiltration per WB18 lane substep as `infiltration / lane_substeps`, before each percolation substep, matching baseline hourly `xfin` cadence.
- Added dry/no-event inactive stale-SWE bypass before active-snow validation so non-snow and no-event paths are not blocked by stale snow state.
- Canonicalized within-tolerance snowpack exhaustion after corrected negative-melt state loss to zero SWE/depth/density instead of emitting negative runtime snow storage, while failing closed on material carried state-loss overdraw.

## Runtime Evidence

Ran:
- Pre-fix direct-rain test failed with unchanged aggregate storage: `soil_water=10`.
- Post-fix focused HPHYS0285 tests passed: direct rain enters WB18/WB11 storage, inactive stale snow does not gate direct-rain ingress, and dry stale snow does not gate no-event percolation.
- H1 smoke initially failed on `HKERNEL-WB11-PERC-E-003` at `2015-106` with `snow.runtime_swe=-0.0026918754518707685`; after SC-SNOWFREEZE v20/code canonicalization and the v21 bounded-overdraw review remediation, H1 remained executable while material overdraw fails closed.
