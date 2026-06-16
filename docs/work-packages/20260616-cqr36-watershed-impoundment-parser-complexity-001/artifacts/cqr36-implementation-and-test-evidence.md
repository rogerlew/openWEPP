# CQR36 Implementation and Test Evidence

Status: complete.

Production implementation:

- Extracted private branch helpers:
  - `parse_drop_spillway`
  - `parse_drop_spillway_ids1`
  - `parse_drop_spillway_ids2`
  - `parse_drop_spillway_ids3`
  - `parse_rockfill`
  - `parse_emergency_spillway`
  - `parse_emergency_open_channel`
  - `parse_emergency_rating_curve`
  - `parse_filter_barrier`
  - `parse_perforated_riser`
- Extracted private storage/curve helpers:
  - `parse_impoundment_storage_fields`
  - `parse_impoundment_curve_fields`
- Added private aggregation structs for helper return values:
  - `RockfillFields`
  - `EmergencySpillwayFields`
  - `FilterBarrierFields`
  - `PerforatedRiserFields`
  - `ImpoundmentStorageFields`
  - `ImpoundmentCurveFields`

Test implementation:

- Added branch characterization for `ids=2`, `ids=3`, and `ies=1` payloads.
- Added display/source characterization for `WatershedImpoundmentParseError`.

Ran:
`cargo test --test infile_watershed_impoundment_parser_contract`

Result: `22 passed; 0 failed`.

Ran:
`cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260616-cqr36-watershed-impoundment-parser-complexity-001/artifacts/lcov_after.info`

Result: LCOV report written.

Ran:
`cargo crap --workspace --lcov docs/work-packages/20260616-cqr36-watershed-impoundment-parser-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260616-cqr36-watershed-impoundment-parser-complexity-001/artifacts/crap_after.json`

Result: `parse_impoundment` CRAP `15.0`; unique target-file rows above `30`:
`0`.
