# Baseline Code Map

Status: executed.

Normative code authority:
`/workdir/wepp-forest_260430_baseline` at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## Source-Line Map

| Surface | Baseline line evidence | Authority mapped into `SC-GWBASEFLOW-001` |
|---|---|---|
| `gwcoeff.txt` branch selection | `src/main.for:120-136`, `src/main.for:450-465` | `lr_bf` starts disabled, `gwcoeff.txt` presence sets `lr_bf=1`, reads `igwstrd`, `bfcoeff`, `dscoeff`, `bftharea`; missing file leaves `lr_bf=0`. |
| Symbol definitions and units | `src/cchrt1.inc:7-17`, `src/cchrt1.inc:31-52` | Common block and comments define `lr_bf`, `igwstrd`, `bfcoeff`, `dscoeff`, `bftharea`, `gwstrv2`, `gwstrv3`, `gwbfv`, and `gwdsv`. |
| Daily groundwater update | `src/contin.for:1088-1120` | Under `lr_bf=1`, initial storage depth is converted to volume, deep percolation is accumulated as recharge, current storage subtracts prior baseflow/deep seepage, and current `gwbfv`/`gwdsv` are computed as coefficient fractions of storage. Under `lr_bf=0`, storage/export variables are zeroed. |
| Hillslope pass payload | `src/wshpas.for:220-227`, `src/wshpas.for:236-245`, `src/wshpas.for:255-265`, `src/wshpas.for:386-414`, `src/wshpas.for:466-505`, `src/wshpas.for:530-532` | `gwbfv` and `gwdsv` are written/read through hillslope pass and master pass paths across no-event, event, and subevent sections. |
| Watershed driver staging | `src/wshdrv.for:515-520`, `src/wshdrv.for:845-875`; `src/cstore2.inc:7-15`, `src/cstore2.inc:29-32` | `lr_bf=1` creates `chntyp.txt`; generated baseflow/deep-seepage volumes are saved in `tmpgwbfv`/`tmpgwdsv` and the working common-block values are reset for the next pass. |
| Channel routing consumption | `src/wshchr.for:133-148`, `src/wshchr.for:183-189`, `src/wshchr.for:205-225`, `src/wshchr.for:260-262`, `src/wshchr.for:696-704` | `lr_bf=0` uses `cbase` branch behavior; `lr_bf=1` injects generated `tmpgwbfv`, evaluates `bftharea`, converts daily volume to flow with seconds-per-day, and avoids channel-water-balance duplication. |
| Channel inflow/quality consumption | `src/wshcqi.for:86-159`, `src/wshcqi.for:199-207` | `lr_bf=0` uses `cbase`/`qBase`; `lr_bf=1` consumes generated side/top `tmpgwbfv` terms, applies phosphorus coupling, evaluates `bftharea`, and includes baseflow in channel inflow accounting. |
| Water-balance publication | `src/watbalprint.for:87-96`, `src/watbalprint.for:101-124` | Legacy `Baseflow` column is populated from `qBase` under `lr_bf=0`; under `lr_bf=1`, baseflow is printed as zero because generated groundwater baseflow is already carried through runoff/streamflow surfaces. |

## Contracted Recurrence

The accepted port target is:

1. `S_0 = (igwstrd / 1000) * hillslope_width_m * hillslope_length_m`.
2. `D_i = sum_o(sep_o_m * width_o_m * length_o_m)`.
3. `S_i = S_{i-1} + D_i - Qb_{i-1} - Qs_{i-1}`.
4. `Qb_i = bfcoeff * S_i`.
5. `Qs_i = dscoeff * S_i`.

`D_i`, `Qb_i`, and `Qs_i` are daily timestep volumes in `m^3`; channel
consumers perform the separate `m^3` to `m^3 s^-1` conversion when needed.

M-T2B must prove whether openWEPP carries this state per hillslope/lane in a
baseline-compatible way. The legacy common-block shape is not, by itself,
adequate implementation closure evidence for concurrent or Lane D MOFE state.

## Namespace Separation

- `bfcoeff` is a `gwcoeff.txt` groundwater-reservoir coefficient and is not
  `chan.inp` `cbase`.
- `latqcc` is lateral subsurface export under `SC-SUBHYD-001`, not generated
  groundwater-reservoir baseflow.
- `gwbfv`/`gwdsv` are generated reservoir exports when `lr_bf=1`; they are not
  inferred defaults when the sidecar is absent.
