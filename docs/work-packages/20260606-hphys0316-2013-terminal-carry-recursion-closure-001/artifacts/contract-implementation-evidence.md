# Contract Implementation Evidence

Status: complete

Evidence mode: Static

Static:

- `SC-SNOWFREEZE-001` was amended from contract version `46` to `47`.
- Added `INV-SNOWFREEZE-042`, the HPHYS0316 2013 terminal carry-recursion
  invariant.
- Added guard-map row `INV-SNOWFREEZE-042`.
- Added `OBL-SNOWFREEZE-P-021`.
- `SC-WATBAL-001` was amended from contract version `139` to `140`.
- Added `INV-WATBAL-090`, the HPHYS0316 water-balance consumer gate.
- Added guard-map row `INV-WATBAL-090`.
- Added `OBL-WATBAL-P-026`.
- `docs/specifications/science-contracts/index.md` now registers HPHYS0316
  authority at `SC-SNOWFREEZE-001#INV-SNOWFREEZE-042` and
  `SC-WATBAL-001#INV-WATBAL-090`.

Contract conclusion:

The H1/H7/H39 spring-2016 rows are inherited from the 2013 terminal snowpack
state and remain `UNRESOLVED`/owned `HOLD` under HPHYS0317 unless a follow-on
proves paired fixed-baseline/openWEPP input-surface ownership or a different
source-line lane.
