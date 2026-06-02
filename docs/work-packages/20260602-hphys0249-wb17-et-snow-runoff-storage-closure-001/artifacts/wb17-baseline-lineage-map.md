# WB17 Baseline Lineage Map

Status: complete

Evidence mode: static

Static:

- `/workdir/wepp-forest_260430_baseline/src/evap.for:160-162` computes
  uncovered-soil fraction `eaj = exp(-0.5*(cv+.1))`.
- `/workdir/wepp-forest_260430_baseline/src/evap.for:428-436` computes
  potential bare-soil evaporation and removes residue interception before
  stage-memory logic.
- `/workdir/wepp-forest_260430_baseline/src/evap.for:458-564` mutates
  `s1`, `s2`, `tu`, and `tv` stage-memory surfaces.
- `/workdir/wepp-forest_260430_baseline/src/evap.for:566-604` adds residue
  interception back to `es`, caps total ET by `eo`, and splits residue
  evaporation `eres` from soil evaporation.
- `/workdir/wepp-forest_260430_baseline/src/evap.for:618-668` withdraws soil
  evaporation from layer `st(i)` over the upper `0.10 m` zone and reduces
  `es`/`et` by unmet demand.
- `/workdir/wepp-forest_260430_baseline/src/swu.for:91-104` finds the deepest
  rooted layer from `rtd` and cumulative `solthk`.
- `/workdir/wepp-forest_260430_baseline/src/swu.for:122-187` distributes
  plant uptake with `ub=3.065`, `uob=0.953346`, deficit-scales uptake by
  `pltol * ul(k)`, caps by `st(k)`, mutates `st(k)`, and sets
  `watstr = Σu / ep`.
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:547-560`
  invokes ET only on the final hourly substep after hourly percolation.
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:978-1026`
  invokes `swu` when `ep > 0` and `rtd > 0`, then recomputes
  `watcon = Σ(st(i) + thetdr(i)*(dg(i)-frozen(i)))`.
