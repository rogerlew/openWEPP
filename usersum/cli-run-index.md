# openWEPP CLI Run Index

This index captures run signatures for engine CLI entrypoints in this repository.

## Run index

- [openwepp-cli-hill](openwepp-cli-hill.md)
  - Hillslope execution entrypoint.
  - `openwepp-cli-hill --run-dir ... --run-file ... --output-dir ...`
  - Outputs: hillslope artifacts + `openwepp_hillslope_run_manifest.json`.

- [openwepp-cli-watershed](openwepp-cli-watershed.md)
  - Watershed execution entrypoint.
  - `openwepp-cli-watershed --run-dir ... --run-file ... --output-dir ...`
  - Outputs: watershed artifacts and a watershed manifest in the active workflow.

- [openwepp-cli-totalwatsed3](openwepp-cli-totalwatsed3.md)
  - Aggregate hillslope interoperability parquet into total-watershed parquet.
  - `openwepp-cli-totalwatsed3 --input-dir ... --output ...`
  - Requires PASS/WAT/SOIL/ELEMENT inputs as available in interchange conventions.

- [open_wepp_runner](open_wepp_runner.md)
  - Orchestrator for hillslope runs and release helper commands.
  - `open_wepp_runner run-hillslope --hillslope-binary ... --run-dir ... --run-file ... --output-dir ...`
  - Also supports `release lint` and `release sidecar` subcommands.

- [openwepp-snowbench](openwepp-snowbench.md)
  - Snow benchmark and phase-audit workloads.
  - `openwepp-snowbench <mode> --run-dir ... --output-dir ...`
  - Also supports `jennings-phase` workflow mode.

## Quick validation commands (examples)

- Hillslope: `target/release/openwepp-cli-hill --run-dir tests/fixtures/cli01/hillslope_run_dir --run-file case.run --output-dir /tmp/openwepp-docs-cli-verify/hill-out --policy compat --legacy-sidecar-discovery`
- Orchestrator: `target/release/open_wepp_runner run-hillslope --hillslope-binary target/release/openwepp-cli-hill --run-dir tests/fixtures/cli01/hillslope_run_dir --run-file case.run --output-dir /tmp/openwepp-docs-cli-verify/runner-hill-out --policy compat --legacy-sidecar-discovery`
- Snowbench: `target/release/openwepp-snowbench physics-bulk --run-dir tests/fixtures/cli01/hillslope_run_dir --run-file case.run --output-dir /tmp/openwepp-docs-cli-verify/snowbench-out`

## Standard handoff language

- For all documentation updates associated with entries in this index, use:
  - `handoff: usersum/documentation-agent`
  - `target_path: usersum/documentation-agent.md`
