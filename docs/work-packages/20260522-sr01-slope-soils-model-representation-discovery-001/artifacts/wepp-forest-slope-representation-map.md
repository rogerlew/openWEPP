# WEPP-Forest Slope Representation Map

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Baseline slope intake/version gates and structure checks are defined in `infile.for` and `verchk.for`.
- Baseline slope parse + derived profile construction are defined in `input.for` and `profil.for`.
- Baseline slope consumers include runoff/routing/watershed setup kernels.

Ran:
- Read and line-audited baseline sources under `/workdir/wepp-forest_260430_baseline/src` (`infile.for`, `input.for`, `profil.for`, `xinflo.for`, `route.for`, `param.for`, `wshinp.for`, `wshini.for`, `watbal.for`).

## Canonical Representation

| canonical symbol | storage surface | units | producer phase | notes |
|---|---|---|---|---|
| `nslpts(iplane)` | `/slope1/` (`cslope1.inc`) | count | slope parse | number of slope points per OFE |
| `slplen(iplane)` | `/dist2/` (`cdist2.inc`) | m | slope parse | OFE length |
| `xinput(l,iplane)` | `/dist1/` (`cdist1.inc`) | raw input then nondim | parse then profile derivation | raw endpoint is reused as `slen`; later rewritten to 0..1 grid |
| `slpinp(l,iplane)` | `/slope3/` (`cslope3.inc`) | m/m | slope parse | piecewise point slope inputs |
| `avgslp(iplane)` | `/slope2/` (`cslope2.inc`) | m/m | profile derivation | computed from trapezoid-integrated profile; clamped minimum |
| `a(k,iplane), b(k,iplane)` | `/slope1/` + `/slope3/` | nondim coefficients | profile derivation | segment coefficients for downstream shear/runoff transforms |
| `xu(k,iplane), xl(k,iplane)` | `/slope1/` | nondim distance | profile derivation | segment upper/lower bounds used by routing/erosion |
| `y(i,iplane)` | `/dist3/` (`cdist3.inc`) | nondim elevation | profile derivation | normalized elevation profile on 101-point grid |

## Initialization and Normalization Rules

1. Slope file version policy:
- `infile.for` reads `datver`; when `datver > 10`, `verchk` enforces compatibility (`verson >= slpchk`).
- `slpchk = 91.5` is initialized in `inidat.for`; `verchk` hard-stops on lower versions.
- Evidence: `infile.for:1649-1659`, `inidat.for:1154-1155`, `verchk.for:25-31`.

2. OFE count and topology closure:
- `nwsofe` is read from slope file and must match `jstruc`; mismatch loops to re-open/abort path.
- Evidence: `infile.for:1670-1686`.

3. Slope row parse:
- Per OFE reads `azm,fwidth`, then `nslpts,slplen`, then point pairs `(xinput,slpinp)`.
- Watershed path (`ivers=3`) computes channel average slope from point pairs and clamps non-positive channel slope to `0.0001`.
- Evidence: `input.for:380-398`, `input.for:400-428`.

4. Canonical derived slope profile (`profil`):
- `slen = xinput(last)`; profile elevation `y` integrated by trapezoid from point slopes.
- `avgslp = y(1)/slen`, with hard floor `avgslp >= 0.000001`.
- Normalized coordinates: `xstar = xinput/slen`; segment coefficients `a,b` built per interval.
- Runtime profile grid: `xinput(2..101)` rewritten to `0.01..1.00`; `y(i)` recomputed from piecewise coefficients.
- Evidence: `profil.for:37-81`.

5. Hillslope aggregate geometry coupling:
- `totlen` and area/aggregate slope (`harea`, `hslop`) are accumulated from `slplen` and `avgslp`.
- Evidence: `input.for:437-449`.

## Primary Consumers (Baseline)

| consumer | slope fields used | boundary role |
|---|---|---|
| `xinflo.for` | `avgslp`, `slplen`, `nslpts`, `a`, `b` | runoff partition and transformed shear coefficients (`ainf/binf/cinf`) |
| `route.for` | `nslpts`, `xu/xl`, `ainftc/binftc/cinftc` | segment-scale detachment/deposition routing |
| `param.for` | `a`, `b`, `avgslp`, `nslpts` | slope-end shear and transport continuity setup |
| `watbal.for` / `watbal_hourly.for` | `slplen` | area-normalized subsurface/runoff coupling |
| `wshinp.for` | `nslpts`, `slplen`, `fwidth` | channel/structure runtime geometry seed |

## Evidence Links

- `/workdir/wepp-forest_260430_baseline/src/infile.for:1649`
- `/workdir/wepp-forest_260430_baseline/src/infile.for:1670`
- `/workdir/wepp-forest_260430_baseline/src/inidat.for:1154`
- `/workdir/wepp-forest_260430_baseline/src/verchk.for:25`
- `/workdir/wepp-forest_260430_baseline/src/input.for:380`
- `/workdir/wepp-forest_260430_baseline/src/input.for:435`
- `/workdir/wepp-forest_260430_baseline/src/profil.for:37`
- `/workdir/wepp-forest_260430_baseline/src/profil.for:51`
- `/workdir/wepp-forest_260430_baseline/src/xinflo.for:147`
- `/workdir/wepp-forest_260430_baseline/src/route.for:183`
- `/workdir/wepp-forest_260430_baseline/src/param.for:168`
- `/workdir/wepp-forest_260430_baseline/src/wshinp.for:374`
