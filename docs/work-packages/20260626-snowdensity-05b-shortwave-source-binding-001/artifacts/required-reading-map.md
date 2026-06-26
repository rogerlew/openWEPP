# Required Reading Map

Evidence mode: Static.

Read before edits:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/planning/snow-frost-fidelity-strategy.md` sections 2, 4, 5, 7, and 10
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/wepp-input-files/specs/climate-file.spec.md`
- `tests/AGENTS.md`

Source files inspected:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/climate.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs`

Conclusion:

The required source-binding decision can be made from existing contract and
runtime evidence. No web search, external dataset, or production code change was
needed.
