# Mismatch Disposition

Status: complete.

Evidence class: Static + Ran.

## Summary

The package resolves the provenance question by separating three surfaces:

1. Raw fixture `.man` values.
2. Upstream wepppy seasonal WEPP `Cancov` trajectories by management class.
3. Current openWEPP snowbench runtime-surface `cancov`.

Current openWEPP snowbench CoE melt diagnostics consume surface 3, a static
initial-condition value. They do not consume the upstream per-day seasonal
projection. Therefore a later gradient melt adjudication cannot claim seasonal
mixed/deciduous canopy evidence until it either routes per-day canopy into the
diagnostic/runtime path or explicitly scopes the question to static initial
canopy.

## Per-Fixture Disposition

| Fixture | Disposition |
|---|---|
| `hjandrews_conifer_or` | PASS as high evergreen control. Raw initial, projected evergreen, and openWEPP runtime all equal `0.90`. |
| `tenderfoot_conifer_mt` | PASS as high evergreen control. Raw initial, projected evergreen, and openWEPP runtime all equal `0.90`. |
| `berthoud_conifer_co` | RAP_TS-adjusted exception. Runtime `0.05` equals raw initial `0.05`, not generic evergreen `0.90`; do not use as high-canopy control in current snowbench adjudication without explicit RAP_TS context. |
| `morescreek_conifer_id` | RAP_TS-adjusted high-but-not-evergreen exception. Runtime `0.82` equals raw initial `0.82`; acceptable high-canopy evidence only with RAP_TS caveat. |
| `harvard_mixed_ma` | Static mixed-canopy evidence only. Runtime `0.55` equals raw initial `0.55`; upstream projected winter mean is `0.44446`. Seasonal mixed-canopy verdicts require per-day routing. |
| `marcell_mixed_mn` | Static mixed-canopy evidence only. Runtime `0.55` equals raw initial `0.55`; upstream projected winter mean is `0.44446`. Seasonal mixed-canopy verdicts require per-day routing. |
| `hubbardbrook_deciduous_nh` | Static deciduous-canopy evidence only. Runtime `0.20` equals raw initial `0.20`; upstream projected winter mean is `0.06653`. Seasonal deciduous verdicts require per-day routing. |
| `sleepers_pasture_vt` | Not proven as lowest-cancov endpoint. Runtime `0.50`; upstream forest-management package provides no pasture seasonal trajectory. Treat as pasture/ag clearing with moderate static canopy until a pasture/open stratum correspondence package says otherwise. |

## Downstream Consequence

SNOWDENSITY-10.3.2 / 10.3.3 should not interpret the current eight-fixture
snowbench outputs as a per-day seasonal canopy gradient. The immediate safe
sequence is:

1. Decide whether the gradient melt adjudication needs per-day canopy.
2. If yes, add a diagnostic/runtime path that carries daily `cancov` into CoE
   melt replay before the gradient adjudication.
3. If no, explicitly label the next adjudication as static-initial-canopy
   evidence and exclude seasonal-phenology claims.

