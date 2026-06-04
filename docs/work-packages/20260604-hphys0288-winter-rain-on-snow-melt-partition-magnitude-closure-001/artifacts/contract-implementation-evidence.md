# Contract Implementation Evidence

Status: complete
Evidence mode: Static

Static:
- Amended `SC-SNOWFREEZE-001` with `INV-SNOWFREEZE-021` and `REF-SNOWFREEZE-LEGACY-WINTER-RAINRELEASE` for pinned `snowd.for`/`winter.for` residual rain-on-snow release into `hrmlt`/`wmelt`.
- Amended `SC-RUNOFFPART-001` with `INV-RUNOFFPART-018` for WB12/WB14 direct-rain vs routed-melt partition authority.
- Amended `SC-WATBAL-001` with `INV-WATBAL-063` for WB13 `RM`, WB12 runoff, and WB18 storage-forcing closure.

Baseline provenance:
- `/workdir/wepp-forest_260430_baseline/src/snowd.for` lines 240-279 retains rain in low-density snowpack until `350 kg m^-3` and leaves residual `hrrain`.
- `/workdir/wepp-forest_260430_baseline/src/winter.for` lines 456-459 adds positive residual `hrrain(hour)` to `hrmlt(hour,iplane)` before daily `wmelt` publication.
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` lines 342-345 and `/workdir/wepp-forest_260430_baseline/src/grna.for` lines 267-269 consume `wmelt` through `fin` and `smrate`.
