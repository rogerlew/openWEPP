# Pre-Implementation Evidence

Status: queued

Evidence mode: not-run.

Record before production code edits:

- Static: PySnobal required forcing columns and output columns.
- Static: PySnobal `T_g` and `z_g` semantics.
- Static: openWEPP hourly winter forcing surfaces available for export.
- Static: snow depth-to-mass conversion rule and rejected aliases.
- Static: constant ground-temperature lane rationale.
- Ran: PySnobal import probe and exact Python executable.
- Ran: initial source scans proving no production `qwet`, `Qwet`, or
  `frzftp` implementation is introduced.
