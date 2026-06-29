# FROST Thaw-Residual Diagnostic

Evidence mode: Ran.

Diagnostic-only: no detector threshold, solver physics, fixture, contract, default, or output schema changed.

## Aggregate Split

- Thaw-late cells: `11`.
- Early-onset cells: `2`.
- Thaw-late bucket counts: `{'H1a': 9, 'H1b': 2}`.
- Snow-controlled thaw routes: `{'SNOW-FREE-PERSISTENT': 2, 'MIXED-SNOW-CONTROL': 2, 'SNOW-BURIED-ACCUMULATION': 2, 'SNOW-BURIED-UNDER-MELT': 5}`.
- Early-onset characterization: `{'EARLY-ONSET-MATERIAL': 2}`.
- Routing recommendation: `snow-buried-dominant` -> snow-persistence decomposition first: separate forcing-limited over-accumulation from fixable spring under-melt before Qwet.

Primary material floor is diagnostic-only: `0.02 m`; it is not an adopted thaw detector.

## Per-Cell Buckets

| Cell | Bucket | Snow route | Residual d | Max frdp m | Snow depth m min/median/max | Paired snow residual m | SWE delta m | RM m | Reason |
| --- | --- | --- | ---: | ---: | --- | ---: | ---: | ---: | --- |
| `site1_sleepers_south_field_vt:1986:thaw` | `H1a` missing wet/advective thaw energy | `SNOW-FREE-PERSISTENT` snow-free material persistence | `19` | `0.11019` | `0` / `0.000177254` / `0.508106` | `0.00357638` | `-0.265231` | `0.32989` | 13/19 warm/wet material days have modeled snow depth < 0.1 m; Qwet/wet-heat remains plausible |
| `site1_sleepers_south_field_vt:1987:thaw` | `H1a` missing wet/advective thaw energy | `MIXED-SNOW-CONTROL` mixed snow-buried and snow-free persistence | `20` | `0.137875` | `0` / `0.116705` / `0.541825` | `-0.0125774` | `-0.245008` | `0.319808` | 11/21 warm/wet material days are snow-buried and 10/21 are near snow-free |
| `site2_sleepers_w9_hardwood_vt:1994:thaw` | `H1a` missing wet/advective thaw energy | `SNOW-BURIED-ACCUMULATION` snow-buried persistence | `35` | `0.147896` | `0.394313` / `0.786359` / `1.03491` | `-0.0712193` | `-0.00206677` | `0.160167` | 32/32 warm/wet material days have modeled snow depth >= 0.1 m; persistence is controlled by snow insulation before any Qwet-class soil heat term; modeled SWE gains or nearly balances across the carried-frost window while snow remains insulating |
| `site2_sleepers_w9_hardwood_vt:1995:thaw` | `H1b` state-machine thaw asymmetry | `MIXED-SNOW-CONTROL` mixed snow-buried and snow-free persistence | `18` | `0.136855` | `0` / `0.125135` / `0.421738` | `-0.247754` | `-0.0702535` | `0.112253` | 7/16 warm/wet material days are snow-buried and 9/16 are near snow-free |
| `site2_sleepers_w9_hardwood_vt:1996:thaw` | `H1a` missing wet/advective thaw energy | `SNOW-BURIED-UNDER-MELT` snow-buried persistence | `39` | `0.0786463` | `0.142964` / `0.360173` / `0.680518` | `-0.0549787` | `-0.0937659` | `0.184559` | 38/38 warm/wet material days have modeled snow depth >= 0.1 m; persistence is controlled by snow insulation before any Qwet-class soil heat term; modeled SWE is net-losing across the carried-frost window, but the snowpack remains insulating through warm/wet material-frost days |
| `site2_sleepers_w9_hardwood_vt:1997:thaw` | `H1a` missing wet/advective thaw energy | `SNOW-BURIED-UNDER-MELT` snow-buried persistence | `84` | `0.0995429` | `0.193375` / `0.81925` / `1.14003` | `0.0444688` | `-0.0527707` | `0.314371` | 58/58 warm/wet material days have modeled snow depth >= 0.1 m; persistence is controlled by snow insulation before any Qwet-class soil heat term; modeled SWE is net-losing across the carried-frost window, but the snowpack remains insulating through warm/wet material-frost days |
| `site2_sleepers_w9_hardwood_vt:2004:thaw` | `H1a` missing wet/advective thaw energy | `SNOW-BURIED-UNDER-MELT` snow-buried persistence | `41` | `0.202929` | `0.43234` / `0.621643` / `0.908603` | `0.0260919` | `-0.0174402` | `0.0672402` | 25/25 warm/wet material days have modeled snow depth >= 0.1 m; persistence is controlled by snow insulation before any Qwet-class soil heat term; modeled SWE is net-losing across the carried-frost window, but the snowpack remains insulating through warm/wet material-frost days |
| `site2_sleepers_w9_hardwood_vt:2006:thaw` | `H1a` missing wet/advective thaw energy | `SNOW-FREE-PERSISTENT` snow-free material persistence | `17` | `0.168808` | `0` / `0.0145849` / `0.258952` | `-0.040524` | `-0.118859` | `0.167159` | 11/17 warm/wet material days have modeled snow depth < 0.1 m; Qwet/wet-heat remains plausible |
| `site2_sleepers_w9_hardwood_vt:2009:thaw` | `H1a` missing wet/advective thaw energy | `SNOW-BURIED-UNDER-MELT` snow-buried persistence | `111` | `0.0901975` | `0.0428571` / `0.836673` / `1.12461` | `0.0789685` | `-0.13447` | `0.45737` | 60/63 warm/wet material days have modeled snow depth >= 0.1 m; persistence is controlled by snow insulation before any Qwet-class soil heat term; modeled SWE is net-losing across the carried-frost window, but the snowpack remains insulating through warm/wet material-frost days |
| `site2_sleepers_w9_hardwood_vt:2010:thaw` | `H1b` state-machine thaw asymmetry | `SNOW-BURIED-UNDER-MELT` snow-buried persistence | `50` | `0.105814` | `0` / `0.535131` / `0.704834` | `-0.0215854` | `-0.148873` | `0.251673` | 29/38 warm/wet material days have modeled snow depth >= 0.1 m; persistence is controlled by snow insulation before any Qwet-class soil heat term; modeled SWE is net-losing across the carried-frost window, but the snowpack remains insulating through warm/wet material-frost days |
| `site2_sleepers_w9_hardwood_vt:2011:thaw` | `H1a` missing wet/advective thaw energy | `SNOW-BURIED-ACCUMULATION` snow-buried persistence | `20` | `0.0714834` | `0.485337` / `0.767615` / `0.918258` | `0.0407192` | `0.00774375` | `0.109256` | 17/17 warm/wet material days have modeled snow depth >= 0.1 m; persistence is controlled by snow insulation before any Qwet-class soil heat term; modeled SWE gains or nearly balances across the carried-frost window while snow remains insulating |

## Snow-Persistence Evidence

The snow route uses the carried-frost window's warm/wet material-frost days. Paired observed snow depth is reported where the Step 3 comparison reports contain it; sparse pairs are evidence, not a fitted classifier. The frost trace does not emit a soil-temperature time series, so the heat-path evidence is limited to surface temperature, Qsrf/Quf, snow conductivity, and the snow thermal-resistance proxy `depth / k_snow`.

| Cell | Route | Buried/free warm-wet d | Paired warm/wet snow obs d | Mean snow residual m | Obs/model snow delta m | Buried SWE delta m | Buried Qsrf/Quf W m-2 | Buried snow R m2K/W | Buried runoff m |
| --- | --- | ---: | ---: | ---: | --- | ---: | --- | ---: | ---: |
| `site1_sleepers_south_field_vt:1986:thaw` | `SNOW-FREE-PERSISTENT` | `6` / `13` | `3` | `0.00476851` | `-0.4938` / `-0.508106` | `-0.207369` | `0.0580728` / `1.22868` | `0.719832` | `0` |
| `site1_sleepers_south_field_vt:1987:thaw` | `MIXED-SNOW-CONTROL` | `11` / `10` | `4` | `-0.0125774` | `-0.55075` / `-0.535217` | `-0.184088` | `0.210674` / `0.452433` | `0.899906` | `0` |
| `site2_sleepers_w9_hardwood_vt:1994:thaw` | `SNOW-BURIED-ACCUMULATION` | `32` / `0` | `2` | `-0.0837546` | `-0.289333` / `-0.318363` | `0.0186332` | `0.46839` / `0.907582` | `4.0604` | `0` |
| `site2_sleepers_w9_hardwood_vt:1995:thaw` | `MIXED-SNOW-CONTROL` | `7` / `9` | `2` | `-0.274166` | `-0.441667` / `-0.421738` | `0.00133697` | `1.17539` / `0` | `3.43254` | `0` |
| `site2_sleepers_w9_hardwood_vt:1996:thaw` | `SNOW-BURIED-UNDER-MELT` | `38` / `0` | `6` | `-0.0549787` | `-0.16` / `-0.188908` | `-0.102359` | `0.943186` / `0.35768` | `1.41019` | `0` |
| `site2_sleepers_w9_hardwood_vt:1997:thaw` | `SNOW-BURIED-UNDER-MELT` | `58` / `0` | `4` | `0.0824894` | `-0.07` / `-0.387164` | `-0.0608647` | `0.650501` / `0.488848` | `5.28214` | `0` |
| `site2_sleepers_w9_hardwood_vt:2004:thaw` | `SNOW-BURIED-UNDER-MELT` | `25` / `0` | `2` | `0.0260919` | `-0.1552` / `-0.274621` | `-0.0164402` | `0.612848` / `0.0695137` | `2.55332` | `0` |
| `site2_sleepers_w9_hardwood_vt:2006:thaw` | `SNOW-FREE-PERSISTENT` | `6` / `11` | `2` | `-0.040524` | `-0.246666` / `-0.258952` | `-0.0540269` | `0.427921` / `0.703416` | `0.466942` | `0` |
| `site2_sleepers_w9_hardwood_vt:2009:thaw` | `SNOW-BURIED-UNDER-MELT` | `60` / `3` | `2` | `-0.0616736` | `-0.194` / `-0.658419` | `-0.207148` | `0.617753` / `0.263987` | `4.3261` | `0` |
| `site2_sleepers_w9_hardwood_vt:2010:thaw` | `SNOW-BURIED-UNDER-MELT` | `29` / `9` | `2` | `-0.129431` | `-0.29` / `-0.582325` | `-0.0737156` | `0.465978` / `0` | `3.19195` | `0` |
| `site2_sleepers_w9_hardwood_vt:2011:thaw` | `SNOW-BURIED-ACCUMULATION` | `17` / `0` | `0` | `` | `-0.03` / `-0.30274` | `0.00774375` | `0.531835` / `0` | `2.90217` | `0` |

## Early-Onset Cells

| Cell | Bucket | Residual d | Max frdp m | Max air C | Max surface C | Reason |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `site2_sleepers_w9_hardwood_vt:1996:onset` | `EARLY-ONSET-MATERIAL` material early freeze | `-19` | `0.0666303` | `10.5829` | `0` | modeled frost before observed onset is material; this is a distinct onset mechanism, not the thaw-late persistence mechanism |
| `site2_sleepers_w9_hardwood_vt:1998:onset` | `EARLY-ONSET-MATERIAL` material early freeze | `-48` | `0.0782353` | `11.0701` | `0` | modeled frost before observed onset is material; this is a distinct onset mechanism, not the thaw-late persistence mechanism |

## H2 Threshold Sensitivity

| Material threshold m | H2 tiny-tail count | Material-persistence count | H2 cells |
| ---: | ---: | ---: | --- |
| `0.0` | `0` | `11` | `` |
| `0.001` | `0` | `11` | `` |
| `0.0025` | `0` | `11` | `` |
| `0.005` | `0` | `11` | `` |
| `0.01` | `0` | `11` | `` |
| `0.02` | `0` | `11` | `` |
| `0.05` | `0` | `11` | `` |
| `0.1` | `4` | `7` | `site2_sleepers_w9_hardwood_vt:1996:thaw, site2_sleepers_w9_hardwood_vt:1997:thaw, site2_sleepers_w9_hardwood_vt:2009:thaw, site2_sleepers_w9_hardwood_vt:2011:thaw` |

## Snow-Depth Control Sensitivity

| Snow-depth threshold m | Snow-buried count | Snow-free persistent count | Mixed count | Snow-buried cells | Snow-free cells |
| ---: | ---: | ---: | ---: | --- | --- |
| `0.05` | `7` | `1` | `3` | `site2_sleepers_w9_hardwood_vt:1994:thaw, site2_sleepers_w9_hardwood_vt:1996:thaw, site2_sleepers_w9_hardwood_vt:1997:thaw, site2_sleepers_w9_hardwood_vt:2004:thaw, site2_sleepers_w9_hardwood_vt:2009:thaw, site2_sleepers_w9_hardwood_vt:2010:thaw, site2_sleepers_w9_hardwood_vt:2011:thaw` | `site1_sleepers_south_field_vt:1986:thaw` |
| `0.1` | `7` | `2` | `2` | `site2_sleepers_w9_hardwood_vt:1994:thaw, site2_sleepers_w9_hardwood_vt:1996:thaw, site2_sleepers_w9_hardwood_vt:1997:thaw, site2_sleepers_w9_hardwood_vt:2004:thaw, site2_sleepers_w9_hardwood_vt:2009:thaw, site2_sleepers_w9_hardwood_vt:2010:thaw, site2_sleepers_w9_hardwood_vt:2011:thaw` | `site1_sleepers_south_field_vt:1986:thaw, site2_sleepers_w9_hardwood_vt:2006:thaw` |
| `0.2` | `7` | `3` | `1` | `site2_sleepers_w9_hardwood_vt:1994:thaw, site2_sleepers_w9_hardwood_vt:1996:thaw, site2_sleepers_w9_hardwood_vt:1997:thaw, site2_sleepers_w9_hardwood_vt:2004:thaw, site2_sleepers_w9_hardwood_vt:2009:thaw, site2_sleepers_w9_hardwood_vt:2010:thaw, site2_sleepers_w9_hardwood_vt:2011:thaw` | `site1_sleepers_south_field_vt:1986:thaw, site2_sleepers_w9_hardwood_vt:1995:thaw, site2_sleepers_w9_hardwood_vt:2006:thaw` |

## GAP-SNOWFREEZE-002 Disposition

`GAP-SNOWFREEZE-002` remains open and is now narrowed from generic post-residue timing residuals to `snow-buried-dominant` for thaw-late cells. The next fix package should pursue snow-persistence decomposition first: separate forcing-limited over-accumulation from fixable spring under-melt before Qwet. Early-onset cells remain separate onset diagnostics.
