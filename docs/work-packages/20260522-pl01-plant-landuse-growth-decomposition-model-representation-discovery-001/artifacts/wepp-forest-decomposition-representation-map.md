# WEPP-Forest Decomposition Representation Map (PL01)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Baseline decomposition representation is a coupled residue-state system over standing (`rmagt`), flat (`rmogt`,`rilrm`,`rigrm`), buried (`smrm`), and root (`rtm`) pools with per-pool environmental indices and management/tillage event transforms.
- `resup` is the structural partition-shift boundary for harvest/senescence/add-remove events; `decomp` applies daily decay and event mass/cover transforms.

Ran:
- Audited `decomp.for`, `resup.for`, `grow.for`, `contin.for`, and shared include surfaces (`ccrpvr1.inc`, `cridge.inc`, `cdecvar.inc`, `cperen.inc`, `csenes.inc`, `cupdate.inc`).

## Canonical Decomposition State

| canonical symbol | storage surface | units | role |
|---|---|---|---|
| `rmagt` | `/crpvr1/` | kg/m^2 | standing residue mass |
| `rmogt(nowres,iplane)` | `/crpvr1/` | kg/m^2 | flat residue mass by residue age slot |
| `rilrm`, `rigrm` | `/ridge/` | kg/m^2 | flat residue partition in rills/ridges |
| `smrm(nowres,iplane)` | `/crpvr1/` | kg/m^2 | buried residue mass |
| `rtm(nowres,iplane)` | `/crpvr1/` | kg/m^2 | non-living root residue mass |
| `iresd`, `iroot` | `/crpprm/` | index | residue/root type identity slots |
| `senvin`, `fenvin`, `benvin` | `/decvar/` | unitless | standing/flat/buried environment indices |

## Daily Transform Semantics (`decomp`)

1. Standing decay.
- `rmagt <- rmagt * exp(-senvin * oratea(iresd)*ferind*pszind)` with lower bound guard.

2. Flat decay (rill + ridge).
- `rilrm` and `rigrm` decay with `fenvin`; `rmogt` recomputed as weighted blend by `wght1=(rspace-width)/rspace`.

3. Buried and root decay.
- `smrm` decays using buried env index.
- `rtm` decays via `benvin` and `orater` coefficient.

4. Management event transforms.
- Burning, cutting, and (legacy) residue-removal branches adjust standing and/or flat pools.
- Tillage-date branch applies intensity-driven standing-to-flat transfer, residue-cover remapping, and submerged-pool adjustments.
- Newer residue add/remove options (`10..13`) call `resup` and then apply disturbance/non-disturbance semantics.

## Partition-Shift Semantics (`resup`)

1. Residue slot shifting.
- Oldest slot accumulates previous slot; previous slot receives current; current slot is reset or recomputed depending on event flag.

2. Event-typed behavior by `isenes` flag.
- `-2` senescence, `-1` perennial stop, `0` harvest before senescence, `1` harvest after senescence, `10..13` residue add/remove modes.

3. Root update on harvest/kill.
- `rtm`/`iroot` slots shift; `rtmass`, `rtm15`, `rtm30`, `rtm60` are reset after harvest/kill transitions.

## Ordering and Ownership

- `contin` calls `decomp(nowcrp)` before daily `soil` update for cropland.
- `grow`/`ptgra`/`ptgrp` call `resup` to re-index residue state at phenology and management transitions.
- `watbal` preserves historical note that decomposition was moved out of `watbal` into `contin` for same-day management effect ordering.

## Evidence Links

- `/workdir/wepp-forest_260430_baseline/src/decomp.for:579`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:597`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:618`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:633`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:666`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:693`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:714`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:745`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:819`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:829`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:174`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:207`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:255`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:286`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:312`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:357`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:371`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:696`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:760`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:811`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:890`
- `/workdir/wepp-forest_260430_baseline/src/ccrpvr1.inc:7`
- `/workdir/wepp-forest_260430_baseline/src/cridge.inc:7`
- `/workdir/wepp-forest_260430_baseline/src/cdecvar.inc:7`
