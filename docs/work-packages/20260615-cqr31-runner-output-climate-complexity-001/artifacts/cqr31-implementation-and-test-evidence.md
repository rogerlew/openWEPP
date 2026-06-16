# CQR31 Implementation and Test Evidence

Implementation summary:

- Removed the `clippy::too_many_lines` suppressions from
  `build_simulation_owned_wb13_row` and
  `build_simulation_owned_wb13_row_for_ofe`.
- Decomposed `build_simulation_owned_wb13_row_for_ofe` into private helpers for
  context validation, calendar projection, final publication keys,
  precipitation, profile storage, frozen storage, liquid input, interception,
  physical runoff, ET terms, deep percolation, subsurface flow, routed runoff,
  and scalar-surface construction.
- Added private value structs to group WB13 row inputs and avoid broad helper
  argument lists.
- Preserved the public API and existing WB13 row/surface authorities.

Focused checks:

- `cargo fmt --check`: passed.
- `cargo clippy -p openwepp-runner --all-targets -- -D warnings`: passed.
- `cargo test -p openwepp-runner publication_wb13`: passed, `31` passed,
  `0` failed.

Metric checks:

- After LCOV generation passed.
- After CRAP generation passed with the established `126` source-map warnings.
- Target CRAP: `251.62932776803854` -> `16.0`.
- Highest newly extracted helper CRAP: `12.584884659264825`.
