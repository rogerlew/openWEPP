# Fixture Plan

Status: `ACTIVE`
Evidence mode: Static.

## Member and Day

- Member: `mn_corn_h4`
- Lane: `lane_index = 1`
- Day: `sim_day_index = 792`

## Rungs

- `dx2p5`
- `dx1p25`
- `dx0p625`

## Evidence Surfaces

- Existing active trace detail: outlet bins and sampled outlet hydrograph.
- New or reused step trace: time, dt, CFL, source, storage, boundary flux,
  predictor/corrector limiter, final TVD scale, outlet state, and spatial
  index/x-position of dominant limiter or correction activity.
- Compact JSON/Markdown attribution outputs.

## Raw Output Hygiene

`artifacts/raw-hydrograph-numerics-runs/` is ignored. Raw run trees and raw
step traces are not committed.
