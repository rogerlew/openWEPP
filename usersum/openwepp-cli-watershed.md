# `openwepp-cli-watershed` — Watershed Simulation CLI

## Purpose

Runs openWEPP watershed simulations from a watershed run directory and writes full watershed outputs plus a run manifest.

## CLI interface

```bash
openwepp-cli-watershed \
  --run-dir <path> \
  --run-file <path> \
  --output-dir <path> \
  [--policy compat] \
  [--legacy-sidecar-discovery]
```

## Required inputs

- `--run-dir`: directory containing watershed input files.
- `--run-file`: watershed `*.run` input file.
- `--output-dir`: directory for the manifest and any generated output artifacts.

## Output

- Watershed outputs are written according to the active publication contract and input run structure.
- The run manifest is written to `<output-dir>/openwepp_watershed_run_manifest.json` when available in that workflow.

## Practical notes

- Use `--legacy-sidecar-discovery` when your run set includes legacy sidecar files outside strict current schema.
- `--policy compat` is typically used for mixed legacy/modern source files.

## Exit behavior

- `0`: success.
- Non-zero: common on malformed or mismatched run inputs.

## Manual validation reference

- Expected-input-mismatch check:
  - `target/release/openwepp-cli-watershed --run-dir tests/fixtures/cli01/hillslope_run_dir_unknown --run-file case.run --output-dir /tmp/openwepp-docs-cli-verify/watershed-out --policy compat`
- Log: `/tmp/openwepp-docs-cli-verify/watershed-invalid.log` (exit `1`, parse error: missing field `pw0_str`).

## Standard handoff language

- Use this handoff target for page updates:
  - `handoff: usersum/documentation-agent`
  - `target_path: usersum/documentation-agent.md`
