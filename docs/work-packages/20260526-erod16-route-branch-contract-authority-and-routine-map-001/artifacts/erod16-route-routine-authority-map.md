# EROD16 Route Routine Authority Map

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- Baseline authority target:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Canonical ownership target:
  - hillslope `route.for` branch family -> `SC-SED-001`
  - watershed/channel routing branches -> `SC-ROUTE-001`

## Routine Map
| routine | role | canonical owner | evidence |
|---|---|---|---|
| `contin.for` | Calls hillslope `route` in event erosion path | `SC-SED-001` | `call route` at line 1218 |
| `route.for` | Segment-loop sediment routing with upper-end deposit/detach branching | `SC-SED-001` | `k=2..nslpts`, `du<0`/`du>=0`, `mshear` dispatch |
| `xcrit.for` | Shear regime classifier (`mshear` 1..5) | `SC-SED-001` | routine header and `mshear` definitions |
| `depc.for` | Deposition-equation partial term | `SC-SED-001` | routine header and formula |
| `depend.for` | Solve deposition-end location (`xdend`) | `SC-SED-001` | routine header and iterative solve |
| `depos.for` | Deposition profile updates (`load`, `tc`, `detach`) | `SC-SED-001` | routine header and loop updates |
| `erod.for` | Detachment branch solver and `ndep` trigger path | `SC-SED-001` | routine header and deposition transition logic |
| `enrich.for` | Class-fraction update and OFE-end finalization | `SC-SED-001` | routine header and `iendfg` path |
| `rtpart.for` | Root-mass partitioning by depth layer | `SC-PLANT-001` | routine header + `grow.for` call-site |

## Route Branch Family Map (`route.for`)
1. Upper-boundary deposition test (`du < 0`) enters `depc -> depend -> depos`.
2. Deposition-ending position in segment controls subsequent `mshear` dispatch.
3. `mshear` cases 1..5 dispatch to `erod` call families with range splits at
   `xc1`/`xc2` and erodibility toggles (`0.0` vs `eata`).
4. Upper-boundary detachment branch (`du >= 0`) executes separate `mshear`
   dispatch tree.
5. If `ndep != 0` after detachment, route executes deposition follow-up from
   `xdbeg` to segment end.
6. OFE-end finalization executes terminal `enrich(..., iendfg=1)`.

## Ran
- `nl -ba /workdir/wepp-forest_260430_baseline/src/contin.for | sed -n '1190,1240p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/route.for | sed -n '1,260p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/route.for | sed -n '260,620p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/xcrit.for | sed -n '1,220p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/depc.for | sed -n '1,220p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/depend.for | sed -n '1,220p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/depos.for | sed -n '1,220p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/erod.for | sed -n '1,260p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/enrich.for | sed -n '1,220p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/rtpart.for | sed -n '1,180p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/grow.for | sed -n '590,660p'`
