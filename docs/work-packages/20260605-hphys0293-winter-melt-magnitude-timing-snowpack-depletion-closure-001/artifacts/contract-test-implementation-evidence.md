# Contract Test Implementation Evidence

Status: complete
Evidence mode: Static

Static:

- Added `tests/integration/hphys0293_winter_melt_timing_contract.rs`.
- Registered the test target in `Cargo.toml`.
- Contract test coverage:
  - verifies HPHYS0293 authority exists in `SC-SNOWFREEZE-001`, `SC-RUNOFFPART-001`, and `SC-WATBAL-001`;
  - verifies runner trace surfaces preserve term-level snow depletion evidence;
  - verifies corrected negative-melt state-loss authority remains separate from routed liquid;
  - verifies WB14 exclusion evidence surfaces remain available for target-row attribution.
