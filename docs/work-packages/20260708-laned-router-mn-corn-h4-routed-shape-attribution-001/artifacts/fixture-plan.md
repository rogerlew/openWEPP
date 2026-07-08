# Fixture Plan

Status: `EXECUTED`
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

- Active trace row fields: source mass, outlet mass, end-window storage,
  tail-fold, routed hourly weights, uniform-shape flag, and degenerate-shape
  flag.
- Package-local diagnostic extraction for the pre-hourly outlet series:
  `OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1`.
- Compact JSON/Markdown attribution outputs.

## Executed Fixture

The package reran `mn_corn_h4` on `dx2p5`, `dx1p25`, and `dx0p625`, then used
`analyze_day792_attribution.py` to derive the day-792 normalization, CDF, and
raw-hydrograph comparisons from the ignored trace trees.

## Raw Output Hygiene

`artifacts/shape-attribution-runs/` is ignored. Raw run trees are not committed.
