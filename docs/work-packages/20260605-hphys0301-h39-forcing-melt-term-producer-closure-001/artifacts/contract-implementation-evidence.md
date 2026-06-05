# Contract Implementation Evidence

Status: completed

Evidence mode: static

Static:

- Amended `SC-SNOWFREEZE-001` to `contract_version: 35`.
- Added `SC-SNOWFREEZE-001#INV-SNOWFREEZE-032` requiring H39 first-2013 baseline residual rain-on-snow to be compared against openWEPP `snow_hourly_rain_released_sum_m + snow_post_winter_rain_m` before any forcing edit.
- Added `SC-SNOWFREEZE-001` HPHYS0301 addendum prohibiting source-authority claims from observe tags whose source call sites are absent.
- Amended `SC-WATBAL-001` to `contract_version: 124`.
- Added `SC-WATBAL-001#INV-WATBAL-076` requiring HPHYS0301 residual-rain/release water-balance reconciliation before forcing, raw-melt, routed-melt, WB17, WB18, WB19, or WB13 edits.
- Added `docs/specifications/science-contracts/index.md` HPHYS0301 registry note linking `INV-SNOWFREEZE-032` and `INV-WATBAL-076`.

Ran:

- Not run; this artifact records static contract amendments only.
