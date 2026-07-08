# Contract Disposition

Status: complete.
Evidence class: Static.

## Amended

`docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`

- Bumped `contract_version` to `48`.
- Added management-lanuse and baseline `frcfac.for` authority anchors.
- Updated the friction-operand sourcing, active opt-in, and conditional default
  guard rows.
- Updated `INV-OFEROUTE-010` and its guard-map row.
- Updated `OBL-OFEROUTE-P-007`, the static coefficient alias map, the unit
  governance row, and test-vector obligations.
- Added BEI row `OFEROUTE-ROUTE-COEFF-PROJECTION-AUTHORITY`.
- Added `GAP-OFEROUTE-008`.
- Added rev-48 revision history.

`docs/specifications/wepp-input-files/specs/plant-file.spec.md`

- Updated the end-user native routing coefficient section to name rejected
  legacy cropland inference sources explicitly.
- Updated `last_updated_utc`.

## Not Amended

- `SC-SED-001`: no sediment process-physics change.
- `SC-RUNOFFPART-001`: no hourly-source-shape or rainfall-excess change.
- `SC-WATBAL-001`: no water-balance equation or publication change.
- `SC-GWBASEFLOW-001`: M-T2A stands; M-T2B owns implementation.
- `docs/specifications/science-contracts/index.md`: no lifecycle or new
  contract entry change.

## Authority Result

Projection authority did not close. The contract now records that legacy
cropland fields and legacy aggregate friction/erosion diagnostics cannot satisfy
the five static Lane D route coefficients unless a future bridge contract
ratifies all operands.
