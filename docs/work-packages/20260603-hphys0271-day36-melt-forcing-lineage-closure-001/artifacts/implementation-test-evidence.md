# Implementation Test Evidence

Status: completed
Evidence mode: static + ran

Static:

- `SnowHourlyState` now carries melt term, hourly forcing, branch, raw-melt, and redistributed-melt evidence through the active snow coupling.
- Active and inactive snow writeback now publish the HPHYS0271 trace surfaces.
- HPHYS JSON trace rows now include per-hour raw/redistributed melt, term maps, forcing maps, and branch flags.
- No process-physics equation was changed; the implementation exposes already-computed `melt.for`-lineage terms.

Ran:

- Focused runner trace test passed.
- Focused CLIM05 snow contract test passed.
- Full H1..H39 candidate run completed with all 39 hillslope processes returning `rc=0`.
