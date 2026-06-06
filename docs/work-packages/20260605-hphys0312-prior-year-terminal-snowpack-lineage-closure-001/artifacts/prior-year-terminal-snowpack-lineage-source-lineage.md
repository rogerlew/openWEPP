# Prior-Year Terminal Snowpack Lineage Source Lineage

Status: complete

Evidence mode: static

Static:

- `snowd.for:61-65` grounds settle-day-count increment and reset.
- `snowd.for:122-139` grounds cold existing-snow settling/depth update.
- `snowd.for:145-173` grounds cold no-melt snowfall mixing.
- `snowd.for:181-278` grounds warm melt, density update, and rain retention/release.
- `snowd.for:310-312` grounds post-hour `snodpt`/`snodpy`/`densg` writeback.
- openWEPP `03_kernel_support_00_support_helpers.rs:3872-4227` is the homologous runtime snow update lane.

## Verified Source Requirements

- `03_kernel_support_00_support_helpers.rs:3872-3920`: crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3872, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3875, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3897, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3901, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3903, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3911
- `03_kernel_support_00_support_helpers.rs:3925-4057`: crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3937, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4005
- `03_kernel_support_00_support_helpers.rs:4075-4109`: crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4075, crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4105
- `03_kernel_support_00_support_helpers.rs:4218-4227`: crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:4223
- `snowd.for:122-139`: /workdir/wepp-forest_260430_baseline/src/snowd.for:122, /workdir/wepp-forest_260430_baseline/src/snowd.for:125, /workdir/wepp-forest_260430_baseline/src/snowd.for:129, /workdir/wepp-forest_260430_baseline/src/snowd.for:131, /workdir/wepp-forest_260430_baseline/src/snowd.for:139
- `snowd.for:145-173`: /workdir/wepp-forest_260430_baseline/src/snowd.for:145, /workdir/wepp-forest_260430_baseline/src/snowd.for:167, /workdir/wepp-forest_260430_baseline/src/snowd.for:172
- `snowd.for:181-198`: /workdir/wepp-forest_260430_baseline/src/snowd.for:193
- `snowd.for:215-246`: /workdir/wepp-forest_260430_baseline/src/snowd.for:215, /workdir/wepp-forest_260430_baseline/src/snowd.for:218
- `snowd.for:240-278`: /workdir/wepp-forest_260430_baseline/src/snowd.for:240, /workdir/wepp-forest_260430_baseline/src/snowd.for:246, /workdir/wepp-forest_260430_baseline/src/snowd.for:260
- `snowd.for:310-312`: /workdir/wepp-forest_260430_baseline/src/snowd.for:310, /workdir/wepp-forest_260430_baseline/src/snowd.for:311, /workdir/wepp-forest_260430_baseline/src/snowd.for:312
- `snowd.for:61-65`: /workdir/wepp-forest_260430_baseline/src/snowd.for:61, /workdir/wepp-forest_260430_baseline/src/snowd.for:62, /workdir/wepp-forest_260430_baseline/src/snowd.for:65
