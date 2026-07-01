# `open_wepp_runner` — openWEPP Orchestration CLI

## Purpose

Orchestrates openWEPP execution workflows (including hillslope runs) and release validation helpers.

## CLI interface

```bash
open_wepp_runner run-hillslope \
  --hillslope-binary <path> \
  --run-dir <path> \
  --run-file <path> \
  --output-dir <path> \
  [--policy compat] \
  [--legacy-sidecar-discovery] \
  [--manifest-path <path>]

open_wepp_runner release lint --release-dir <path>

open_wepp_runner release sidecar \
  --binary <path> \
  --role <watershed|hillslope|replay>
```

## Required inputs (run-hillslope)

- `--hillslope-binary`: path to `openwepp-cli-hill` binary.
- `--run-dir`: run directory.
- `--run-file`: run file path.
- `--output-dir`: output directory for runner manifest and emitted artifacts.

## Output

- For `run-hillslope`, emits the orchestrator manifest and delegates publication to the selected hillslope binary.
- `release lint` checks a release directory layout.
- `release sidecar` writes/inspects sidecar metadata for the selected role.

## Practical notes

- `--policy compat` and `--legacy-sidecar-discovery` carry through to delegated execution.
- Use `--manifest-path` when you need a custom manifest location.

## Exit behavior

- `0`: command completed.
- Non-zero: binary launch, input validation, or release helper failure.

## Manual validation reference

- Successful run:
  - `target/release/open_wepp_runner run-hillslope --hillslope-binary target/release/openwepp-cli-hill --run-dir tests/fixtures/cli01/hillslope_run_dir --run-file case.run --output-dir /tmp/openwepp-docs-cli-verify/runner-hill-out --policy compat --legacy-sidecar-discovery`
- Log: `/tmp/openwepp-docs-cli-verify/runner.log` (exit `0`).

## Standard handoff language

- Use this handoff target for page updates:
  - `handoff: usersum/documentation-agent`
  - `target_path: usersum/documentation-agent.md`
