# SWU Lineage Diagnosis

Status: complete

Evidence mode: static + ran

Static:

- Baseline `swu.for:122-191` applies crop `pltol(itype)` normalization,
  cumulative potential uptake, deficit scaling by `pltol * ul(i)`, storage cap
  by `st(i)`, final `Ep=ΣUi`, and `Ws=ΣUi/Etp`.
- HPHYS0251 ports the missing openWEPP lineage pieces: crop `pltol` projection,
  legacy normalization, and layer `UPi_####`/`Ui_####` publication.

Ran:

- Contract-derived tests demonstrate the ported lineage behavior in isolation.
- Full-suite metrics show the production run did not improve closure:
  `Ep` mean abs diff mean moved from `1.68341` to `1.70276`, and
  `Total-Soil`/`SoilWaterTotal` moved from `168.131` to `170.349`.
- Targeted H1/H13/H39 diagnostics show `UPi≈Etp`, but `Ui/Ep` remains low
  because the available layer storage presented to `swu` is too small.

Assessment:

- Closed by HPHYS0251: SWU contract authority and observability for effective
  `pltol`, layer potential uptake, layer actual uptake, final `Ep`, and `Ws`.
- Not closed: semantic parity. The residual is no longer best explained by
  missing `swu.for` weighting or crop tolerance lineage.
- Recommended continuation focus: upstream soil-water storage availability and
  layer-state lineage before/into root uptake, especially WB11 seed storage,
  WB18/WB19 depletion timing, and profile `st(i)`/`watcon` consistency for
  H1/H13/H39.
