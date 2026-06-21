# Anti-Alias Fixture Plan

Status: partial.
Evidence mode: Static + Ran.

Required anti-alias classes:

- HBP/PASS peak runoff must differ from daily runoff volume and `QOFE`.
- WAT `P` must differ from `RM`, snowmelt, and irrigation.
- WAT/PASS runoff volume must distinguish `Q`, `QOFE`, outlet area, upstream
  area, and publication-area sums.
- ET components must distinguish `Ep`, `Es`, `Er`, and total ET.
- `Dp`, `latqcc`, `Tile`, and `Qd` must differ in at least one fixture.
- storage, frozen water, snow water, profile depth, FC, and WP must differ.
- loss/manifest metadata must distinguish climate day count from executed day
  count and direct counters from compatibility row counts.

Each accepted output-family projection must have at least one focused fixture
where the wrong aliases produce different values from the accepted direct
operand.

Implemented R6A anti-alias fixtures:

- `r6a_publication_capture_records_run_bound_rows_without_publication_alias`
  seeds `publication.runoff_m = 0.875 m` and proves the captured direct runoff
  row does not project that value as authoritative `Q`.
- `r6a_direct_projection_consumers_read_publication_frame_operands` supplies a
  direct publication frame with distinct `P`, `RM`, `Q`, `QOFE`, ET,
  subsurface, storage, profile, interception, erosion, loss, and manifest
  values, then proves WAT/PASS/loss/manifest consumers read those operands.

Remaining anti-alias work belongs to R6 writer cutover:

- byte/Arrow identity fixtures against real public outputs;
- event erosion producer fixture;
- checksum/provenance anti-alias against manifest output paths;
- profile and frost producer-authority fixtures once those direct producers are
  active.
