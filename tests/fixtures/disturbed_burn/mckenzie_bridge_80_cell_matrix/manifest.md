# McKenzie Bridge 80-Cell Disturbed-Burn Matrix

Fixture `mckenzie_bridge_80_cell_matrix` is the WS-3 disturbed-forest
directional-validation substrate: 4 textures x 5 vegetation classes x 4 burn
severities = 80 single-OFE hillslope cells.

## Provenance

Generated from the WEPPpy disturbed matrix generator on 2026-07-03 using the
same McKenzie Bridge RS, OR climate and canonical 201 m slope shape as the
existing `forest_high_severity_clay_loam` p4 anchor. The checked-in
`/home/workdir/wepppy/tests/disturbed/disturbed_matrix0/runs` source was
incomplete at p1..p48, so the full p1..p80 input matrix was regenerated from
WEPPpy's test construction path before import.

The fixture stores only source inputs and a matrix catalog. It does not store
run outputs. Non-semantic trailing whitespace and terminal blank lines were
normalized on import so `git diff --check` stays usable; token content was not
changed.

## Layout

- `matrix.csv`: one row per cell with `wepp_id`, texture, vegetation,
  severity, disturbed class, and source management identity.
- `common/mckenzie_bridge.cli`: shared climate for all 80 cells.
- `common/canonical_201m.slp`: shared single-OFE slope for all 80 cells.
- `inputs/pN.man`, `inputs/pN.sol`, `inputs/pN.run`: per-cell management,
  soil, and legacy line-oriented WEPP run recipe.
- `SHA256SUMS`: checksums for fixture files.

The legacy `.run` files are retained for provenance. Tests generate temporary
`openwepp-hillslope-runfile-v1` TOML runfiles when executing cells.

## Known Input Notes

The climate header says "Years simulated 100" because that is how the WEPPpy
source generator labels the CLIGEN file. The daily body used by openWEPP spans
2020-01-01 through 2025-12-31 and contains 2,192 executable days.

The source climate has trailing `nan` dewpoint tokens on the final 22 rows.
WS-3 tests normalize those tokens only in temporary run directories before
execution. The committed fixture remains source-shaped.

## Scope

This matrix is sufficient to validate the WS-2 `ksatadj` runoff and peakflow
directional effects once cells are executed under openWEPP. Sediment ordering is
held under `HOLD-DFF-WS3-SEDIMENT-PRODUCTION`: current production direct
execution leaves Wave-1 disabled in the typed seed authority and therefore
publishes zero `tdet`, `tdep`, and `sedcon_*` for these single-OFE cells.

The follow-on Wave-1/Wave-2 package must produce real, contract-backed
sediment operands before WS-3 may assert sediment ordering.
