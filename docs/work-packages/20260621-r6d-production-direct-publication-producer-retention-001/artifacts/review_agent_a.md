# Review A

Evidence mode: Static.

Scope reviewed:

- retained direct publication producer insertion point;
- cutover artifact construction path;
- fail-closed hold marker;
- forbidden compatibility source usage.

Findings:

- PASS: cutover consumes `execution.retained_direct_publication` and the
  cutover branch does not call `DirectRunFrame::skeleton`,
  `DirectFrameExecutor::new`, or `run_publication_capture`.
- PASS: retained rows are produced during the climate-day loop, not after
  output publication.
- PASS: retained row authority is limited to parsed climate/calendar, slope
  geometry, and run/lane/day identity.
- PASS: the cutover gate rejects retained rows that lack parity-grade
  hydrology/erosion/output-family operands before public writes.
- HOLD: direct hydrology/storage/subsurface/evaporation/PASS/loss/manifest/
  erosion publication producers are still absent.
- HOLD: runner module line count remains above closure threshold.
