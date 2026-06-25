# Required Reading

Static:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/decisions/0026-stateful-winter-column-sub-solver.md`
- `docs/decisions/0027-opt-in-physics-bulk-snow-model.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/work-packages/20260625-snowdensity-02-contract-adr-001/artifacts/worker-handoff.md`
- `crates/openwepp-runner/src/hillslope/snowbench.rs`
- `crates/openwepp-runner/src/bin/openwepp-snowbench.rs`
- `tools/snowfreeze_observed/snotel_density_three_way.py`
- `tests/fixtures/snotel_observed/README.md`

Notes:

- `physics_bulk` is opt-in candidate scope only.
- This package must not add runtime parser/config activation or production
  output schema changes.
- SNOTEL validates profile cells; it does not fit constants.
