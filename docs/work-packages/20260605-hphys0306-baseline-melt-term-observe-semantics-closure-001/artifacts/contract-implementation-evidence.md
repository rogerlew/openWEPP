# Contract Implementation Evidence

Status: complete

Evidence mode: static

Static:

- Added `INV-WATBAL-079` to `SC-WATBAL-001`.
- Added the HPHYS0306 Branch-Active Melt-Term Observe Addendum.
- Added an invariant guard-map row requiring `branch-active-mask-hold` before
  numeric term comparison when baseline/openWEPP active masks differ.
- Added revision-history entry `128`.

Ran:

- Static contract amendment only; no production physics code changed.
