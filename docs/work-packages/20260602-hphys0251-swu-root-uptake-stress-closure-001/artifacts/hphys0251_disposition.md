# HPHYS0251 Disposition

Status: HOLD

Evidence mode: static + ran

Decision: `HOLD`

Rationale:

- HPHYS0251 successfully implemented and tested the scoped
  baseline-authoritative `swu.for` lineage for crop `pltol`, legacy
  normalization, layer `UPi_####`/`Ui_####`, final `Ep`, and `Ws`.
- Code gates pass: `cargo fmt --check`, `cargo clippy --workspace --all-targets
  -- -D warnings`, `cargo test --workspace`, and `cargo deny check`.
- Full-suite semantic metrics did not improve. `Ep` mean abs diff mean worsened
  from `1.68341` to `1.70276`; `Total-Soil`/`SoilWaterTotal` worsened from
  `168.131` to `170.349`.
- Targeted H1/H13/H39 diagnostics show `UPi≈Etp`, but `Ui/Ep` remains far below
  baseline because candidate layer/aggregate storage availability is already
  much lower than baseline.

Conclusion:

- Do not continue tuning SWU in HPHYS0251.
- Next focus should be the upstream layer-storage availability lineage feeding
  WB17 root uptake: WB11 seed storage, WB18/WB19 storage depletion/order, and
  `st(i)`/`watcon` consistency before root uptake.
