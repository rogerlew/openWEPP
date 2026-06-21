# R6H Day-Input Architecture

Status: complete.

## Required Shape

- Day 0 may seed from parsed static inputs, daily climate request, and the
  private direct seed surface as R6G does.
- Day `n` must execute in the direct runtime and commit direct lane/day state
  before day `n+1` PMET operands are constructed.
- PMET operands for day `n+1` must read direct-carried layer/state, not WB13
  rows, post-scheduler compatibility runtime symbols, writeback payloads, or
  writer rows.
- Inputs must be lane-dimensional wherever process state or OFE identity can
  differ by lane.

## Planned Evidence

| Concern | Evidence required | Status |
|---|---|---|
| Interleaving | Direct test proving day `n+1` input construction observes day `n` committed state. | Complete: `r6h_publication_capture_builds_lane_day_inputs_after_direct_commit`. |
| Lane dimensionality | Fixture or focused test where two lanes diverge and cannot alias a day-global input. | Complete: same test builds day-1 PMET from two different lane states. |
| Fail closed | Missing required direct state produces typed error or stable hold, not a default. | Complete: builder errors map to `DirectRuntimeError::PublicationDayInputBuildFailure`; WAT residual maps to `HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`. |
| Consumer path | Runner cutover consumes the interleaved direct frame for WAT output. | Complete: `build_retained_direct_publication_frame` calls `run_publication_capture_with_interleaved_day_inputs`. |

## Implemented Shape

- `DirectFrameExecutor::run_publication_capture_with_interleaved_day_inputs`
  invokes a callback inside the day/lane execution loop before seeding a
  `DirectDayFrame`.
- The legacy slice-based API remains for existing callers, but now delegates
  through the interleaved executor path.
- `DirectPublicationDayInputBuilder` constructs one day/lane input at a time
  from parsed static inputs, the requested daily climate surface, and the
  committed direct lane state visible in `DirectRunFrame`.
- When a lane already has direct-carried layers, the builder overlays only the
  allowlisted layer symbols needed by WB11/PMET and hydrology input builders.
  Later-day percolation/subsurface layer vectors are intentionally cleared so
  `DirectRunFrame::seed_day_frame` remains the direct layer-state authority.

## Boundary Found

The architecture closes the stale precompute defect. WAT no longer reduces to
`Es`, `Total-Soil`, and `SoilWaterTotal`; it reduces to an `Es`-only ulp-scale
PMET layer-state parity boundary. That boundary is held under
`HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`.
