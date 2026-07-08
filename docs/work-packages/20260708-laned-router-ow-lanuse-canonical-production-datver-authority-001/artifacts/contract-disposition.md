# Contract Disposition

Evidence class: Static.
Status: complete.

## Amended

`docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`

- Bumped `contract_version` from `48` to `49`.
- Added canonical native datver production authority to scope text.
- Updated the friction operand sourcing, conditional default, and active opt-in
  guard rows.
- Updated `INV-OFEROUTE-010`, the guard map, `OBL-OFEROUTE-P-007`, alias/unit
  row, test-vector obligations, BEI rows, `GAP-OFEROUTE-008`, and revision
  history.

`docs/specifications/wepp-input-files/specs/plant-file.spec.md`

- Added production-authority note for `ow-lanuse-1` and later native datvers.
- Clarified that routing blocks are parse-optional only for inspection or
  explicit non-Lane-D compatibility workflows, but required for canonical Lane D
  production/default activation.
- Rejected optional sidecars as production route-coefficient authority.
- Updated `last_updated_utc`.

`docs/contracts/openwepp-management-lanuse-authority-contract.md`

- Added `LANUSE-AUTH-7` for canonical native production datver authority.
- Clarified WEPPpy/disturbed/native producer authority as materialized native
  `.man` route coefficients, not sidecar or legacy datver authority.
- Extended Lane D fail-closed language from shadow-only to shadow/active/default.

## Not Amended

- No Rust source files.
- No tests.
- No `SC-GWBASEFLOW-001`, `SC-SED-001`, `SC-RUNOFFPART-001`, or `SC-WATBAL-001`.
- No science-contract registry lifecycle metadata.

## Authority Result

The canonical authority now says: new-physics Lane D production requires
`ow-lanuse-1` or later native management files with complete embedded route
coefficients. Legacy datvers remain deprecated compatibility inputs and do not
satisfy Lane D active/default coefficient authority.
