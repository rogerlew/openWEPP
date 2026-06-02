# Review Agent A

Status: complete

Evidence mode: static

Static:

- Reviewer: Bernoulli (`019e8907-e32c-7680-8eea-492c1e27bb0a`).
- Scope: static correctness review of HPHYS0249 contracts, WB17/WB19 ordering,
  production code, tests, and package artifacts.
- Ran: no validation commands by review agent.

Findings:

1. High: root uptake was scheduled inside `Evapotranspiration`, before WB19
   drainage/lateral mutation, while baseline hourly ordering runs `swu` after
   the WB19 tail and before aggregate `watcon` recompute.
2. Medium: residue-capped evaporation did not add un-evaporated residue
   remainder back to top-layer storage and clear persisted residue state.

Disposition:

- Fixed. Added `PlantRootUptake` as a post-`LateralTransfer` phase before
  `RunoffReconciliation`.
- Fixed. Moved `swu` lineage into `run_plant_root_uptake`, recomputing
  aggregate storage after WB19 drainage/lateral mutation.
- Fixed. Added residue remainder add-back to layer 1 and clears
  `wb17_residue_interception`.
- Fixed. Added/updated tests for ordering, residue add-back, aggregate
  residual/frozen-depth terms, and post-WB19 root uptake.
