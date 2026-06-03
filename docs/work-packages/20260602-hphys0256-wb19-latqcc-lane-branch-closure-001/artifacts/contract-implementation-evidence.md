# Contract Implementation Evidence

Status: completed

Evidence mode: static

- Static: `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  now distinguishes daily `wb19_lateral_drain_lane_substeps=1` WB19 lateral
  authority from hourly `=24` authority and adds the HPHYS0256 daily lane
  invariant/addendum.
- Static: `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  now records the WB19 daily lateral lane as the authority for daily storage
  withdrawals and `latqcc`/`Subrunoff` publication.
- Static: both contract amendments cite pinned baseline daily
  `/workdir/wepp-forest_260430_baseline/src/watbal.for` and hourly
  `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` provenance.
