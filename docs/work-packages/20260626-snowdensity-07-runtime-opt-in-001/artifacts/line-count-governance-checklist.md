# Line-Count Governance Checklist

Evidence class: Static.

New files:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`:
  382 lines.
- `tests/integration/snowdensity07_runtime_opt_in.rs`: 237 lines.

Large touched files:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`:
  pre-existing large direct-runtime phase file; SNOWDENSITY-07 only added R4G
  snow-coupling carry fields.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`:
  pre-existing large hydrology partition file; SNOWDENSITY-07 only added
  density selector resolution and guard mapping.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`:
  canonical science contract; growth is expected for contract-first authority.

No new oversized tutorial or procedural content was added to root `AGENTS.md`.

