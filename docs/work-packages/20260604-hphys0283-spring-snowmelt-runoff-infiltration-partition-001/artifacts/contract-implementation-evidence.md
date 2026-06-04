# Contract Implementation Evidence

Status: complete
Evidence mode: Static

## Static: Canonical Contract Amendments

- `SC-SNOWFREEZE-001`: added HPHYS0283 routed-`wmelt` partition authority, requiring meltwater to participate in WB12 infiltration/runoff forcing and WB18 layer ingress.
- `SC-RUNOFFPART-001`: added WB12 event-forcing invariant that routed snowmelt is offered to Green-Ampt infiltration before residual runoff assignment.
- `SC-WATBAL-001`: added daily closure invariant requiring `S` to remain signed snow-storage while positive routed melt cannot bypass infiltration and layer ingress.
- `SC-PERC-001`: added active-snowmelt same-pass `fin/xfin` layer-ingress authority for WB18 before percolation and aggregate `watcon` recomputation.

## Static: Baseline Provenance

- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:342`: `fin` lineage includes rain, interception, `wmelt`, and irrigation before runoff/storage closure.
- `/workdir/wepp-forest_260430_baseline/src/grna.for:269`: Green-Ampt snowmelt forcing uses `smrate = wmelt(iplane) / dur`.
- Baseline anchor: commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## Disposition

- Contract-first authority is implemented.
- The implementation claim is scoped to active routed snowmelt. Full direct-rain `fin/xfin` WB18 ingress remains baseline authority but is not claimed as closed by HPHYS0283.
