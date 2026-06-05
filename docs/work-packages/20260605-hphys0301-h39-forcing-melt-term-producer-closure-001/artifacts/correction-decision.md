# HPHYS0301 Implementation Decision

Status: executed-hold

Evidence mode: static + ran

Static:

- No production forcing or snow-kernel edit is authorized by HPHYS0301.
- Static `stmtim.for` and openWEPP hourly partition formulas are aligned for the visible raw partition equation.
- Baseline observe tags used by HPHYS0298/0299 are not present as source-line tag sites in `/workdir/wepp-forest_260430_baseline/src`, so they are evidence artifacts, not equation authority.

Ran:

- HPHYS0301 lineage runner parsed H39 first-2013 daily rows and produced `h39-forcing-release-lineage-ledger.json`.

Decision:

- `production_forcing_edit_authorized = false`.
- `production_snow_melt_edit_authorized = false`.
- `hphys0301_route = h39-rain-release-lineage-reclassified-hold`.
- Blocking invariant: The HPHYS0300 H39 raw-rain aggregate compared baseline residual rain-on-snow `hrrain` evidence against openWEPP raw `snow_hourly_rain_sum_m`. Comparing baseline residual rain to openWEPP released plus post-winter rain collapses the 16.476985 mm raw-rain delta to a sub-millimeter residual. The pinned source tree does not contain the H298 observe tag sites, so the package cannot use those tags as source-line authority for a forcing edit. Remaining `hrmlt`/`wmelt` deltas require paired `melt.for`/`snowd.for` term/state instrumentation.
