# WEPP-Forest Soil Representation Map

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Baseline soil intake/version gating is in `infile.for`.
- Baseline soil parse, normalization, unit conversion, and layer re-binning are in `input.for`.
- Baseline runtime consumers span `soil`, `infpar`, `watbal`, `perc`, `tilage`, `evap`, and watershed outputs.

Ran:
- Read and line-audited baseline sources under `/workdir/wepp-forest_260430_baseline/src` (`infile.for`, `input.for`, `ctemp.inc`, `cwater.inc`, `csolva1.inc`, `csolva2.inc`, `cvgpar.inc`, `soil.for`, `infpar.for`, `watbal.for`, `perc.for`, `tilage.for`).

## Canonical Representation

| canonical symbol | storage surface | units in runtime | producer phase | notes |
|---|---|---|---|---|
| `solwpv` | `/temp/` (`ctemp.inc`) | int tag | intake/version | soil datver dispatch selector |
| `nslorg` | `/temp/` | count | parse + re-bin finalize | parsed layer count then remapped to 0.2 m layers |
| `solth1` | `/temp/` | m cumulative | parse + re-bin | initially mm input then converted to m |
| `ddg` | local/input + later `dg` | m layer thickness | re-bin | differential layer thickness from cumulative depths |
| `sand1, clay1, orgma1, rfg1, cec1` | `/temp/` | fraction / cec units | parse + re-bin | weighted to 0.2 m layer grid |
| `bd1` | `/temp/` | kg/m^3 | parse + re-bin | input g/cc converted to kg/m^3 |
| `ssc1` | `/temp/` | m/s | parse + re-bin | input mm/h converted to m/s |
| `thetd1, thetf1` | `/temp/` | m3/m3 | parse + re-bin | wilting/field capacity layer properties |
| `thetdr, thetfc, solthk, dg, nsl` | `/water/` (`cwater.inc`) | m3/m3, m, count | runtime seed (post-tilage) | runtime soil-water state consumed across hydrology |
| `slflag, anisrt, kslast, ui_bdrkth` | `/temp/` + perc coupling | flag, ratio, m/s, m | restrictive-layer parse | lower-boundary percolation controls |
| `vgthr1..vgfc1`, `ksatadj`, `ksatfac`, `ksatrec`, `burncode`, `lkeff` | `/vgenu1/` (`cvgpar.inc`) | mixed | datver 9002+ policy | disturbed/burn/rosetta parameters |

## Initialization and Normalization Rules

1. Soil version policy and parser branch selection:
- `datver > 90` triggers `verchk` vs `solchk`; for `datver < 100`, multiply by 10 and round to `solwpv`.
- `solchk = 91.5`; `verchk` hard-stops if lower.
- Evidence: `infile.for:1870-1883`, `infile.for:1942-1943`, `inidat.for:1154`, `verchk.for:25-31`.

2. Datver-conditioned OFE policy rows:
- `7778..9002`: `ksatadj,luse,stext,ksatfac,ksatrec`.
- `>=9003`: `ksatadj,luse,burncode,stext,lkeff`.
- Evidence: `input.for:467-474`.

3. Datver-conditioned layer row arity:
- pre-941 or 7777: base 10-field hydraulic row.
- 7778..9001: adds `ui_anisrt`.
- >9001: adds van Genuchten/Rosetta fields.
- Evidence: `input.for:541-558`.

4. Depth and domain constraints:
- Cumulative thickness is forced to at least 200 mm; bottom layer gets +200 mm safeguard.
- Per-layer `solth1` capped at 1800 mm.
- `ssc2` lower bounded (version dependent), organic matter and BD/rfg clamped.
- Evidence: `input.for:562-590`, `input.for:600-615`.

5. Unit conversions:
- `bd2` g/cc -> kg/m^3 (`*1000`).
- `solth1` mm -> m (`*0.001`).
- `ssc2` mm/h -> m/s (`/3.6e6`).
- `%` inputs to fractions (`/100`).
- Evidence: `input.for:618-630`.

6. Restrictive-layer parsing:
- `slflag, anisrt, kslast` branch for 2006+ excluding select tags; 7778+ path reads `slflag, ui_bdrkth, kslast` with bounds.
- Evidence: `input.for:638-667`.

7. Canonical re-binning/aggregation:
- Soil profile remapped to 0.20 m layer lattice; weighted averaging populates `bd1/ssc1/thetf1/thetd1/sand1/clay1/orgma1/cec1/rfg1` and VG arrays.
- Harmonic-style conductivity aggregation via `ksinv`; fallback floors when missing.
- Evidence: `input.for:688-734`, `input.for:753-934`.

## Primary Consumers (Baseline)

| consumer | soil fields used | boundary role |
|---|---|---|
| `soil.for` | `st`, `thetdr`, `thetfc`, `ssc`, `nsl`, `avgslp` | daily soil-state and erodibility/conductivity updates |
| `infpar.for` | `st`, `ul`, `thetdr`, `thetfc`, `dg`, `nsl`, `avclay/avsand`, `cvgpar` | infiltration conductivity and matric potential surfaces |
| `watbal.for` / `watbal_hourly.for` | `thetdr`, `thetfc`, `solthk`, `dg`, `nsl`, `ssc` | daily closure, ET and percolation coupling |
| `perc.for` | `st`, `ul`, `ssc`, `hk`, `nsl`, `slflag`, `kslast`, `ui_bdrkth` | layer-to-layer percolation and restrictive-layer behavior |
| `tilage.for` | `solth1`, `thetd1`, `thetf1`, `ssc1`, `bd1` -> runtime `solthk/dg/thetdr/thetfc/ssc` | runtime layer-state remap after disturbance |

## Evidence Links

- `/workdir/wepp-forest_260430_baseline/src/infile.for:1870`
- `/workdir/wepp-forest_260430_baseline/src/infile.for:1942`
- `/workdir/wepp-forest_260430_baseline/src/input.for:467`
- `/workdir/wepp-forest_260430_baseline/src/input.for:541`
- `/workdir/wepp-forest_260430_baseline/src/input.for:562`
- `/workdir/wepp-forest_260430_baseline/src/input.for:618`
- `/workdir/wepp-forest_260430_baseline/src/input.for:638`
- `/workdir/wepp-forest_260430_baseline/src/input.for:688`
- `/workdir/wepp-forest_260430_baseline/src/input.for:911`
- `/workdir/wepp-forest_260430_baseline/src/ctemp.inc:6`
- `/workdir/wepp-forest_260430_baseline/src/cwater.inc:7`
- `/workdir/wepp-forest_260430_baseline/src/cvgpar.inc:9`
- `/workdir/wepp-forest_260430_baseline/src/soil.for:179`
- `/workdir/wepp-forest_260430_baseline/src/infpar.for:152`
- `/workdir/wepp-forest_260430_baseline/src/perc.for:101`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:563`
