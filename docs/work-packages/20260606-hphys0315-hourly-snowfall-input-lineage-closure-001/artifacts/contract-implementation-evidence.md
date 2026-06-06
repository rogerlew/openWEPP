# Contract Implementation Evidence

Status: complete

Evidence mode: Static

Static:

- `SC-SNOWFREEZE-001` was amended from contract version `45` to `46`.
- Added `INV-SNOWFREEZE-041`, the HPHYS0315 hourly snowfall input-lineage
  invariant.
- Added guard-map row `INV-SNOWFREEZE-041`.
- Added `OBL-SNOWFREEZE-P-020`.
- `SC-WATBAL-001` was amended from contract version `138` to `139`.
- Added `INV-WATBAL-089`, the HPHYS0315 water-balance consumer gate.
- Added guard-map row `INV-WATBAL-089`.
- Added `OBL-WATBAL-P-025`.
- `docs/specifications/science-contracts/index.md` now registers HPHYS0315
  authority at `SC-SNOWFREEZE-001#INV-SNOWFREEZE-041` and
  `SC-WATBAL-001#INV-WATBAL-089`.

Contract conclusion:

The H1/H7/H39 spring-2014 rows remain `UNRESOLVED`/owned `HOLD` unless a
follow-on proves paired fixed-baseline/openWEPP values for `rain`, `stmdur`,
`wntdur`, `wnttim`, `hrtemp`, `rst`, `hrsnow`, `hrrain`, active interval, and
branch choice. Source-code resemblance between openWEPP and `stmtim.for` is
not production-edit authority.
