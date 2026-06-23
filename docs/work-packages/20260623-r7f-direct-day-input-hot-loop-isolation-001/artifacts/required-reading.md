# Required Reading

Status: executing.

## Read

- Static: root `AGENTS.md` from the user-provided repository instructions.
- Static: `docs/work-packages/AGENTS.md`.
- Static: `docs/specifications/science-contracts/AGENTS.md`.
- Static:
  `docs/work-packages/20260623-r7e-r7h-direct-runtime-completion-001/artifacts/worker-handoff.md`.
- Static:
  `docs/work-packages/20260623-r7e-r7h-direct-runtime-completion-001/artifacts/compatibility-isolation.md`.
- Static:
  `docs/work-packages/20260623-r7e-r7h-direct-runtime-completion-001/artifacts/blocker-ledger.md`.
- Static:
  `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`.
- Static:
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`.
- Static:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`.
- Static:
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs`.

## Notes

- The prior no-compatibility proof was invalid because production direct still
  built every day/OFE input through a compatibility-shaped surface builder.
- R7F is scoped to production direct hot-loop isolation. Shadow/cutover modes
  may continue to use compatibility publication builders where explicitly
  selected.
