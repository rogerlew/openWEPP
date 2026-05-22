# Slope and Soil Consumer Ownership Map

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Consumer ownership is reconstructed from include surfaces and direct symbol use in baseline source.
- openWEPP ownership boundaries are architecture-first with parser/orchestrator/kernel separation.

Ran:
- Enumerated baseline include consumers with ripgrep.
- Spot-audited runtime consumers (`xinflo.for`, `route.for`, `watbal.for`, `soil.for`, `infpar.for`, `perc.for`, `tilage.for`, `wshinp.for`, `wshini.for`, `param.for`).

| consumer | consumes_slope | consumes_soil | ownership_boundary | notes |
|---|---|---|---|---|
| `input.for` | yes | yes | input parse + normalization owner | canonical intake for both surfaces |
| `profil.for` | yes | no | slope derivation owner | computes `avgslp`, `a/b`, normalized grid |
| `xinflo.for` | yes | indirect | hillslope runoff partition consumer | derives transformed slope coefficients from `a/b` and `avgslp` |
| `route.for` | yes | indirect | hillslope erosion/transport consumer | segment routing uses `nslpts`, `xu/xl`, transformed coefficients |
| `param.for` | yes | indirect | hydraulics continuity consumer | computes `slpend` and shear continuity terms |
| `soil.for` | yes | yes | soil-state + erodibility owner | consumes `avgslp` and major soil state surfaces |
| `infpar.for` | indirect | yes | infiltration-parameter owner | consumes `thetdr/thetfc/dg/nsl` and VG/policy fields |
| `watbal.for` | yes | yes | daily closure owner | couples `slplen` area geometry with soil water/perc states |
| `watbal_hourly.for` | yes | yes | hourly closure variant owner | same coupled surfaces, hourly path |
| `perc.for` | no | yes | percolation owner | restrictive-layer and conductivity surfaces |
| `tilage.for` | no | yes | disturbance remap owner | translates parsed `*_1` arrays into runtime `cwater` surfaces |
| `wshinp.for` | yes | indirect | watershed channel setup owner | uses `nslpts/slplen/fwidth` as channel geometry seeds |
| `wshini.for` | yes | yes | watershed init owner | reads `avgslp` and `csolva` soil fractions for watershed setup |

## Boundary Observations

- Slope representation ownership is split: parse/derive (`input` + `profil`) versus multiple downstream consumers for runoff, erosion, and watershed routing.
- Soil representation ownership is split: parse/normalize (`input`) then remap/update owners (`tilage`, `soil`, `infpar`, `watbal`, `perc`).
- There is no isolated “slope-only” or “soil-only” runtime boundary in baseline behavior; both are cross-cutting runtime dependencies.

## Evidence Links

- `/workdir/wepp-forest_260430_baseline/src/input.for:374`
- `/workdir/wepp-forest_260430_baseline/src/profil.for:37`
- `/workdir/wepp-forest_260430_baseline/src/xinflo.for:147`
- `/workdir/wepp-forest_260430_baseline/src/route.for:183`
- `/workdir/wepp-forest_260430_baseline/src/param.for:168`
- `/workdir/wepp-forest_260430_baseline/src/soil.for:179`
- `/workdir/wepp-forest_260430_baseline/src/infpar.for:152`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:167`
- `/workdir/wepp-forest_260430_baseline/src/perc.for:101`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:563`
- `/workdir/wepp-forest_260430_baseline/src/wshinp.for:374`
- `/workdir/wepp-forest_260430_baseline/src/wshini.for:58`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:20`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:82`
