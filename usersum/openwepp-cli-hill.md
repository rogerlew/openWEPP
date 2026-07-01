# `openwepp-cli-hill` — Hillslope Simulation CLI

## Purpose

Runs a single openWEPP hillslope simulation from a WEPP-style run directory and emits hillslope outputs.

## CLI interface

```bash
openwepp-cli-hill \
  --run-dir <path> \
  --run-file <path> \
  --output-dir <path> \
  [--policy compat] \
  [--legacy-sidecar-discovery] \
  [--manifest-path <path>] \
  [--direct-default-candidate] \
  [--direct-production-executor]
```

## Required inputs

- `--run-dir`: directory containing run files (`case.cli`, `case.run`, `case.sol`, `case.slp`, etc.).
- `--run-file`: path to the run descriptor file.
- `--output-dir`: directory for CLI-level outputs (a manifest).

## Output

- Standard hillslope outputs are written under `<run-dir>/output/` with names like:
  - `H?.hbp` / `H?.loss.json`
  - `H?.wat.parquet`
  - `H?.plot.parquet`
- A run manifest is written to `<output-dir>/openwepp_hillslope_run_manifest.json`.

## Practical notes

- For legacy-compatible sidecar handling (legacy `frost.txt`, `snow.txt`, `pmetpara.txt`, `wepp_ui.txt`), add `--legacy-sidecar-discovery`.
- `--policy compat` is the practical default for mixed legacy/production input compatibility.
- `--manifest-path` lets you redirect where the manifest is written.

## Exit behavior

- `0`: success.
- Non-zero: runtime/input validation/parsing failure with diagnostic logs to `stderr`.

## Manual validation reference

- Successful run example:
  - `target/release/openwepp-cli-hill --run-dir tests/fixtures/cli01/hillslope_run_dir --run-file case.run --output-dir /tmp/openwepp-docs-cli-verify/hill-out --policy compat --legacy-sidecar-discovery`
- Log: `/tmp/openwepp-docs-cli-verify/hill.log` (exit `0`).

## Standard handoff language

- Use this handoff target for page updates:
  - `handoff: usersum/documentation-agent`
  - `target_path: usersum/documentation-agent.md`
