# Disposition

Status: complete.

Evidence class: Static/Ran.

## Verdict

PASS.

R4M/O completed direct subsurface compute promotion. The retained
implementation adds request-free direct WB18 percolation and WB19
lateral/drainage compute from typed layer-vector inputs, mutates direct layer
state, produces downstream operands, and shadow-projects WB18/WB19 results.
R4B now requires R4M percolation and R4O subsurface-compute shadows before
storage reconciliation and consumes direct `D` and direct `Qd` lineage.

## Findings

No open findings remain.

## Residual Work

- Direct WB17 evapotranspiration/root-uptake compute promotion remains R4N.
- Direct hydrology publication projection and R4 closure remain R4P/Q/Z.
- Public `Dp`, `latqcc`, `Qd`, WB13/WAT/PASS/loss/schema publication cutover
  remains out of scope for R4M/O.

## Next Package

Proceed with R4N direct WB17 evapotranspiration and root-uptake compute
promotion from `docs/work-packages/r4-burndown-execplan.md`.
