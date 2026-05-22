# WEPP-Forest Plant Representation Map (PL01)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Baseline plant representation is split by `landuse` branch (`iplant`) and then projected into shared runtime common blocks consumed by growth/water-balance/residue routines.
- Canonical symbol continuity for plant surfaces is defined by baseline WEPP symbols (`itype`, `cancov`, `canhgt`, `vdmt`, `rmagt`, `rmogt`, `rtm`, `lai`, `rtd`, `rtmass`).

Ran:
- Audited baseline sources and include surfaces: `infile.for`, `tilage.for`, `watbal.for`, `ptgra.for`, `ptgrp.for`, `grow.for`, `range.for`, `ccrpprm.inc`, `ccover.inc`, `ccrpout.inc`, `ccrpvr1.inc`, `ccrpvr2.inc`, `crinpt*.inc`.

## Canonical Representation

| canonical symbol | storage surface | units | producer phase | notes |
|---|---|---|---|---|
| `iplant(i)` | plant-scenario header in `infile` | enum | `.man` section parse | `1=cropland`, `2=rangeland`; forest/road branches hard-stop. |
| `itype(mxcrop,mxplan)` | `/crpprm/` | index | yearly schedule projection (`tilage`) | Current crop selector for all daily plant/growth routines. |
| `bb, bbb, be, btemp, crit, dlai, dropfc, extnct, gddmax, hi, hmax, rdmax, rsr, rtmmax, spriod` | plant parameter arrays | mixed | plant-scenario parse | Core cropland growth parameter set used by `grow`. |
| `oratea, orater, pltol, pltsp, mfocod` | plant parameter arrays | mixed | plant-scenario parse | Decomposition-rate / stress / spacing inputs reused downstream. |
| `aca, ar, cn, rootf, gtemp, tempmn, cf1, cf2, ...` | rangeland arrays (`crinpt*`) | mixed | plant-scenario parse | Rangeland-only growth/decomposition parameter family used by `range`. |
| `cancov, canhgt` | `/cover/` | fraction, m | daily growth update | Shared canopy state consumed by ET/runoff/erosion pathways. |
| `vdmt, lai` | `/crpvr2/`, `/crpout/` | kg/m^2, unitless | daily growth update | Above-ground biomass and leaf area state. |
| `rmagt, rmogt, smrm, rtm` | `/crpvr1/` | kg/m^2 | growth+decomp+management updates | Standing/flat/buried/root residue state family. |
| `rtd, rtmass` | `/crpout/` | m, kg/m^2 | daily growth update | Root depth and live root mass state. |

## Branch and Parse Semantics

1. Plant section is keyed by scenario landuse (`iplant`) read from scenario headers.
- Cropland (`iplant=1`) reads canonical crop growth/decomposition parameters and enforces defaults such as `spriod=14` when zero.
- Rangeland (`iplant=2`) reads separate parameter blocks for non-cropland growth/decomposition.
- Forest/road branches stop execution.

2. Plant scenarios become runtime crop selectors through yearly schedule projection.
- `tilage` reads yearly `manndx` per OFE-year slot and assigns `itype`, `imngmt`, and date/option controls for each crop slot.

3. Daily plant model dispatch is gated by `lanuse` and `imngmt`.
- In `watbal`, cropland dispatches `ptgra` (annual/fallow) or `ptgrp` (perennial) and rangeland dispatches `range`.
- `ptgra`/`ptgrp` call `grow` within valid growth windows and issue `resup` at harvest/senescence/stop transitions.

## Runtime Output Surface

`watbal` plant output stream writes the canonical plant/residue/root surface bundle each day:
- canopy: `canhgt`, `cancov`, `lai`
- crop selector + biomass: `itype(nowcrp,iplane)`, `vdmt`, `rmagt`
- residue/root partitions: `(iresd,rmogt)`, `smrm`, `(iroot,rtm)`

This confirms baseline plant representation is not a single struct but a distributed shared-state contract across crop, cover, residue, and root common blocks.

## Evidence Links

- `/workdir/wepp-forest_260430_baseline/src/infile.for:505`
- `/workdir/wepp-forest_260430_baseline/src/infile.for:517`
- `/workdir/wepp-forest_260430_baseline/src/infile.for:554`
- `/workdir/wepp-forest_260430_baseline/src/infile.for:573`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:228`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:231`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:881`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:883`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:906`
- `/workdir/wepp-forest_260430_baseline/src/ptgra.for:310`
- `/workdir/wepp-forest_260430_baseline/src/ptgrp.for:525`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:464`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:509`
- `/workdir/wepp-forest_260430_baseline/src/range.for:413`
- `/workdir/wepp-forest_260430_baseline/src/range.for:490`
- `/workdir/wepp-forest_260430_baseline/src/ccrpprm.inc:7`
- `/workdir/wepp-forest_260430_baseline/src/ccover.inc:7`
- `/workdir/wepp-forest_260430_baseline/src/ccrpout.inc:7`
- `/workdir/wepp-forest_260430_baseline/src/ccrpvr1.inc:7`
