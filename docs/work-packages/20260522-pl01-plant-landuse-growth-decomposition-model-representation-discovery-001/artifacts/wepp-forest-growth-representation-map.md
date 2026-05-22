# WEPP-Forest Growth Representation Map (PL01)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Baseline growth representation is phase/state driven, with explicit annual/perennial/rangeland branch logic and shared mutable state across biomass, canopy, LAI, root mass/depth, and senescence counters.
- Growth state transitions are tightly coupled to management dates and residue-partition updates (`resup`).

Ran:
- Audited growth flow in `watbal.for`, `ptgra.for`, `ptgrp.for`, `grow.for`, and shared include surfaces (`ccrpgro.inc`, `ccrpout.inc`, `ccrpvr2.inc`, `ccrpvr5.inc`, `csenes.inc`).

## Canonical Growth State Surface

| canonical symbol | storage surface | units | producer/update |
|---|---|---|---|
| `gdd`, `sumgdd(iplane)` | `/crpvr3/` | degC-day | `grow` daily heat-unit accumulation |
| `hia(iplane)`, `vdmx(iplane)` | `/crpgro/` | unitless, kg/m^2 | `grow` harvest-index trajectory |
| `vdmt(iplane)`, `tlive(iplane)` | `/crpvr2/` (+ live biomass surface) | kg/m^2 | `grow` daily biomass updates |
| `cancov(iplane)`, `canhgt(iplane)` | `/cover/` | fraction, m | `grow` canopy update |
| `lai(iplane)` | `/crpout/` | unitless | `grow` annual/perennial LAI formulas |
| `rtmass(iplane)`, `rtd(iplane)` | `/crpout/` | kg/m^2, m | `grow` root mass/depth update |
| `ncount(iplane)`, `isenes(iplane)` | `/crpvr5/`, `/senes/` | count, flag | senescence phase control |

## Control and Transition Semantics

1. Daily entry point is selected by management class.
- `watbal` dispatches `ptgra` for annual/fallow and `ptgrp` for perennial; rangeland uses `range`.
- `ptgra` and `ptgrp` invoke `grow` during active growth windows and manage harvest/senescence triggers.

2. Growth activation uses heat-unit / temperature gates.
- `grow` accumulates `gdd/sumgdd` when `tave > btemp`.
- Active growth branch requires cropland `sumgdd >= crit` (or rangeland temperature gate).

3. Active growth updates biomass/canopy/root surfaces.
- Computes stress-limited increment (`reg = min(watstr,temstr)`), updates `vdmt/tlive`, then canopy cover/height, root mass/depth, and LAI with annual/perennial-specific formulas.

4. Senescence branch updates decline rates and residue transfer coupling.
- First day of senescence sets `isenes=-2` and calls `resup` to shift residue partitions.
- Subsequent days reduce `cancov`/`vdmt`; decline formulas differ for annual/perennial and rangeland contexts.
- Biomass loss feeds `rilrm/rigrm` and updates `rmogt` via rill-ridge weighting.

5. Harvest/stop-date coupling is managed outside `grow` but consumes/updates growth state.
- `ptgra` harvest date calls `resup(nowcrp, isenes)`.
- `ptgrp` handles cutting/grazing/stop transitions and calls `resup` with `-1/-2` flags for perennial stop/senescence paths.

## Ordering Dependency

Growth representation depends on decomposition/residue ordering.
- `decomp` is called in `contin` before `soil` and before downstream daily water-balance completion, while `watbal` comments preserve historical note that `decomp` call moved out to maintain same-day management effects.

## Evidence Links

- `/workdir/wepp-forest_260430_baseline/src/watbal.for:881`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:883`
- `/workdir/wepp-forest_260430_baseline/src/ptgra.for:310`
- `/workdir/wepp-forest_260430_baseline/src/ptgra.for:259`
- `/workdir/wepp-forest_260430_baseline/src/ptgrp.for:525`
- `/workdir/wepp-forest_260430_baseline/src/ptgrp.for:409`
- `/workdir/wepp-forest_260430_baseline/src/ptgrp.for:473`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:283`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:309`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:414`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:464`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:509`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:559`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:643`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:693`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:757`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:937`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:811`
- `/workdir/wepp-forest_260430_baseline/src/ccrpgro.inc:7`
- `/workdir/wepp-forest_260430_baseline/src/ccrpout.inc:7`
- `/workdir/wepp-forest_260430_baseline/src/csenes.inc:7`
