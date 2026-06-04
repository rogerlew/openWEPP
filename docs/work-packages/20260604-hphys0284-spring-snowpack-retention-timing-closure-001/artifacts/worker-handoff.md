# Worker Handoff

Status: complete
Evidence mode: Static + Ran

## Static: What Changed

- HPHYS0284 amended snow/water-balance contracts for corrected negative-melt state lineage.
- Runtime snow coupling now returns separate routed melt and snowpack state-loss quantities.
- `S`, `RM`, and liquid forcing use routed net melt; runtime SWE/depth/density use corrected carried-state loss.
- Material negative post-loss SWE now fails closed instead of being silently clamped.

## Ran: Key Evidence

- Full suite run root: `/tmp/hphys0284_full_release_20260604T182144Z`.
- Targeted trace root: `/tmp/hphys0284_springtrace_20260604T182506Z`.
- `Snow-Water`, `RM`, and `Q` improved; aggregate storage and ET/percolation residuals remain open.

## Static: Recommended Next Package

- Localize post-meltout spring liquid/storage partition after HPHYS0284.
- Compare H1/H7/H39 Julian 120-147 for `Q`, `I`, layer `theta/st`, `Total-Soil`, `SoilWaterTotal`, `Dp`, and `Ep` after corrected meltout.
- Treat WB17 `Ep` as downstream until same-day liquid partition and aggregate storage are proven correct.
