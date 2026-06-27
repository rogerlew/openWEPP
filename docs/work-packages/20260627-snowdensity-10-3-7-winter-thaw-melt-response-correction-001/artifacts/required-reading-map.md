# Required Reading Map

Evidence mode: Static.

- Read: root `AGENTS.md` instructions from the user prompt.
- Read: `docs/work-packages/AGENTS.md`.
- Read: `docs/specifications/science-contracts/AGENTS.md`.
- Read: `crates/AGENTS.md`.
- Read: `tests/AGENTS.md`.
- Read: `docs/work-packages/20260627-snowdensity-10-3-6-winter-thaw-melt-response-001/artifacts/worker-handoff.md`.
- Read: `docs/planning/snow-frost-fidelity-strategy.md` Section 10.3.
- Read: `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`.
- Read: `crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs`.
- Read: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`.
- Read: `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs`.
- Read: `tools/snowfreeze_observed/winter_thaw_melt_response.py`.

Key finding before edits: `INV-SNOWFREEZE-002` currently forbids liquid export
below `350 kg m^-3`, so the follow-on correction must explicitly qualify that
legacy gate for a named opt-in candidate before production code changes.
