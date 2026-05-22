# Plant/Landuse/Growth/Decomposition Consumer Ownership Map (PL01)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Baseline ownership is distributed: intake/projection (`infile`,`tilage`), daily orchestrator sequencing (`contin`,`watbal`), and process kernels (`ptgra`,`ptgrp`,`grow`,`decomp`,`range`,`resup`).
- openWEPP architecture requires one owner per mutable runtime surface and explicit orchestrator/kernal boundaries.

Ran:
- Enumerated and line-audited consumers in baseline and openWEPP architecture surfaces.

| consumer | primary surfaces consumed | ownership boundary | role |
|---|---|---|---|
| `infile.for` | scenario registries (`*1` arrays), `lanuse`, initial residue/cover/root state | input parse + initial projection owner | parses `.man` sections and projects OFE initial state |
| `tilage.for` | yearly references (`manndx`) -> runtime arrays (`itype`,`imngmt`,`jd*`,`resmgt`,`mgtopt`,`gday/gend`) | yearly schedule expansion owner | binds scenario pointers to active runtime controls |
| `contin.for` | `nowcrp`, `switch`, `tilseq`, `lanuse`, `decomp->soil->watbal` sequence | daily scheduler owner | enforces phase ordering and crop-slot transitions |
| `watbal.for` | `lanuse`, `imngmt`, canopy/residue/root state | daily closure owner | dispatches annual/perennial/rangeland growth paths |
| `ptgra.for` | annual/fallow management dates and crop state | annual growth/harvest dispatcher | invokes `grow`, harvest management, `resup` events |
| `ptgrp.for` | perennial management options, cut/graze cycles, stop/senescence gates | perennial growth/harvest dispatcher | invokes `grow`, manages cut/graze, `resup` events |
| `grow.for` | `vdmt/tlive/cancov/canhgt/lai/rtmass/rtd`, `isenes` | growth-state owner | computes daily growth or senescence decline and residue transfers |
| `decomp.for` | `rmagt/rmogt/rilrm/rigrm/smrm/rtm`, env indices, management/tillage events | decomposition-state owner | applies daily decay and management/tillage mass transforms |
| `resup.for` | residue/root slot indices and masses | partition-transition owner | shifts residue/root slots and resets/updates at event boundaries |
| `range.for` | rangeland growth, grazing, burning/herbicide, non-Stott decomposition | rangeland process owner | owns non-cropland growth/decomp/residue transitions |

## Ownership Observations

1. `landuse` is a first-order ownership discriminator.
- `lanuse=1` and `lanuse=2` invoke materially different state families and kernels.

2. Growth and decomposition are not separable in baseline runtime behavior.
- `grow` triggers `resup`; `decomp` consumes/upgrades those partitions same-day via scheduler ordering.

3. Management representation ownership is split but deterministic.
- `infile` owns parse/projection, `tilage` owns schedule expansion, `contin` owns execution ordering.

4. openWEPP boundary implication.
- A single parser-only management boundary is insufficient for semantic closure; runtime-surface adapters and alias continuity are required before kernelization.

## Evidence Links

- `/workdir/wepp-forest_260430_baseline/src/infile.for:1095`
- `/workdir/wepp-forest_260430_baseline/src/infile.for:1338`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:228`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:268`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:442`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:811`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:1288`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:881`
- `/workdir/wepp-forest_260430_baseline/src/ptgra.for:310`
- `/workdir/wepp-forest_260430_baseline/src/ptgrp.for:525`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:464`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:696`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:579`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:745`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:207`
- `/workdir/wepp-forest_260430_baseline/src/range.for:365`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:20`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:23`
