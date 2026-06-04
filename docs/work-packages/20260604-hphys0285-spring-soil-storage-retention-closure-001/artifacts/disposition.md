# Disposition

Status: hold
Evidence mode: Static + Ran

## Decision

HOLD for parity closure continuation; HPHYS0285 implementation is accepted as a forward correction but does not close the soil-storage/retention problem.

## Accepted Corrections

Static:
- Same-pass positive local liquid infiltration now enters WB18 layer storage before percolation for direct rain, active snowmelt, and irrigation lineage; MOFE carry/runon storage ingress is explicitly deferred.
- WB18 applies same-pass ingress per substep as `infiltration / lane_substeps`, matching baseline hourly `xfin = fin / ui_LFtstpF` cadence.
- Inactive stale negative snow state no longer gates non-snow direct-rain ingress or dry no-event percolation.
- Within-tolerance snowpack exhaustion after corrected negative-melt state loss canonicalizes runtime pack to zero instead of publishing negative SWE.
- Material carried state-loss overdraw above `0.005 m` water equivalent now fails closed as a typed domain violation instead of silently zeroing the pack.

## Validation Summary

Ran:
- Focused HPHYS0285 tests passed: `3 passed`.
- Adjacent HPHYS0283/HPHYS0284 tests passed: `4 passed`.
- Claude review remediation tests passed: HPHYS0284/HPHYS0285 `6 passed`; CLIM05/HPHYS0283 `10 passed`.
- Post-review H1..H39 release runtime rerun passed: `39/39` at `/tmp/hphys0285_review_remediation_20260604T203602Z`.
- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed.
- `cargo deny check`: passed with existing warnings.
- Full H1..H39 runtime: `39/39` completed.
- Full H1..H39 semantic pass: `0/39`.

## Residuals

Ran:
- `Total-Soil` mean absolute residual improved from `89.531529` to `71.751081`, but all hillslopes still fail.
- `Dp` mean absolute residual improved from `0.078495` to `0.043905`, with fail count reduced by `8447`.
- `Q`, `RM`, and `Snow-Water` were unchanged from HPHYS0284.
- Remaining storage residual is mixed-sign: H7/H39 are still too dry in spring 2014, while H1 is too wet around Julian 222-225 in 2015.

## Recommended Next Package

Static + Ran:
- Scaffold and execute a contract-first package for layer-capacity/retention and WB18/WB17 coupling after corrected same-pass ingress.
- Focus on whether layer upper-limit/field-capacity normalization, percolation cap routing, and ET withdrawal operate on the correct post-ingress storage state.
- Keep snow timing out of scope unless a new invariant proves snow-owned residual after layer storage traces.
