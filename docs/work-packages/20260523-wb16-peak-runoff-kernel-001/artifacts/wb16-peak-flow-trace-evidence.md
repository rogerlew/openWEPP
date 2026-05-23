# WB16 Peak-Flow Trace Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## WB16 Nominal Contract Vector Trace
Shared vector terms:
- `Q = 0.290068906744067`
- `effdrr = 3.0`
- `vave = 0.09668963558135567`
- `remax = 1.0`
- `vstar = 0.09668963558135567`
- `timep = 0.25`, `ealpha = 1.0`, `m = 1.5`

Branch-vector outcomes (from contract vector parameterization):

| `efflen` | Branch (`wb16_peak_method_branch`) | `tstar` | `qpstar` | `peakro` | `watdur` |
|---|---:|---:|---:|---:|---:|
| `2.0` | `1` (`tstar >= 1`) | `1.1528481348` | `0.8078706828` | `0.0781127219` | `3.7134656125` |
| `0.6` | `2` (`tc < tstar < 1`) | `0.5166379103` | `1.9355916010` | `0.1871516465` | `1.5499137310` |
| `0.1` | `3` (`0 < tstar <= tc`) | `0.1564655643` | `9.4653146115` | `0.9151978204` | `0.3169466756` |

## Assertions Backed by WB16 Integration Test
- Each vector emits finite `peakro`, `watdur`, `wb16_tstar`, `wb16_qpstar`,
  `wb16_vstar`.
- Branch selector symbol equals expected branch id.
- Continuity relation holds in-vector:
  - `watdur = Q / peakro` within contract tolerance.
