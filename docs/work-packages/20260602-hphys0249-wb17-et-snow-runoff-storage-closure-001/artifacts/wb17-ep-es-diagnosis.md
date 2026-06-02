# WB17 Ep/Es Diagnosis

Status: complete

Evidence mode: static + ran

Static:

- Pre-fix WB17 subtracted `Es` and `Ep` from scalar `wb11_soil_water` only and
  did not mutate `wb18_perc_theta_####`, contradicting
  `SC-EVAP-001#INV-EVAP-015`.
- Pre-fix WB17 used `wb11_et_demand * exp(-0.4 * lai)` for soil evaporation,
  while pinned baseline `evap.for` uses `eaj = exp(-0.5*(cancov+0.1))`.
- Pre-fix WB17 did not execute baseline `swu.for` root uptake using `rtd`,
  `pltol`, and per-layer `ul(k)`/`st(k)` lineage.

Ran:

- Post-fix targeted test passed:
  `cargo test --test wb17_et_physics_kernel_contract -- --nocapture`.
- Full-suite `Es` residual improved materially:
  - Mean abs mean: `3.340827 -> 0.036841`.
  - Max abs max: `10.028919 -> 1.890000`.
  - Fail-count sum: `56973 -> 3272`.
- Full-suite `Ep` residual was unchanged:
  - Mean abs mean: `1.739422 -> 1.739422`.
  - Max abs max: `7.780000 -> 7.780000`.

Assessment:

- HPHYS0249 fixed the soil-evaporation half of WB17 layer-storage lineage.
- `Ep` now appears blocked upstream of `swu` layer uptake by growth/root-depth
  activation lineage rather than by the WB17 root-uptake routine itself.
