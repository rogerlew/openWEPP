# Required Reading

Read before edits:

- `docs/planning/snow-frost-fidelity-strategy.md` §10.3 step 8.
- `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-050`,
  `INV-SNOWFREEZE-065`, `INV-SNOWFREEZE-069`,
  `INV-SNOWFREEZE-072`, `INV-SNOWFREEZE-075`, and
  `OBL-SNOWFREEZE-P-050`.
- SNOWDENSITY-10.3.18 cross-SNOTEL mechanism rubric artifacts:
  `cross-snotel-mechanism-rubric.{json,md}`.
- SNOWDENSITY-10.3.5b/10.3.5c Harder-Pomeroy partition authority and
  direct-production validation artifacts.
- `docs/work-packages/AGENTS.md`,
  `docs/specifications/science-contracts/AGENTS.md`, `crates/AGENTS.md`, and
  `tests/AGENTS.md`.

Key extracted decision:

- Adopt `harder_pomeroy_hourly` as the direct-production no-env phase default
  only when composed with the activated melt+density bundle, based on the
  cross-SNOTEL forcing-robust rubric (`15/179` vs prior bundle `17/172`).
- Preserve explicit `legacy_rst` rollback/test selection.
- Treat humid-New-England depth regression as a roadmap note under this gate,
  not a blocker.
- Do not add the separate `.run` disable option.
