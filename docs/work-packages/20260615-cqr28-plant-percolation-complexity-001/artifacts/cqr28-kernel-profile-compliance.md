# CQR28 Kernel Profile Compliance

Static: CQR28 is kernel-affecting because WB18 percolation mutates layer water
storage and publishes percolation fluxes consumed by water-balance,
percolation, plant, and publication contracts.

Authority reviewed:

- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`

Compliance notes:

- SC-PERC-001 WB18 percolation formulas and required layerwise flux
  publication are preserved.
- SC-WATBAL-001 ordering and storage-closure surfaces are preserved, including
  the WB18 position before WB17/WB19/WB12 tail behavior referenced by the
  focused WB18 and WB11/WB12/WB17/WB19 contract tests.
- SC-PLANT-001 plant/root uptake authority is not changed; the same file's
  `run_plant_root_uptake` remains out of scope for this CQR row.
- No surrogate physics, threshold edits, bounded normalization changes,
  parser compatibility changes, or output formula changes were introduced.

Ran: focused WB18 contract tests, full workspace tests, clippy, deny, and
before/after LCOV/CRAP gates passed for the final code state.
