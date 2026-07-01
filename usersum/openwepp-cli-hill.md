# `openwepp-cli-hill` — Hillslope Simulation CLI

*Audience: modelers running openWEPP hillslope simulations directly from the
command line.*

## Purpose

Runs a single openWEPP hillslope simulation (one or more OFEs) from WEPP-format
input files named by a TOML runfile, and writes the hillslope outputs and a run
manifest.

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

## Required arguments

- `--run-dir`: the run directory. Relative paths — the runfile itself, the
  input files it names, and any relative output paths — resolve against this
  directory, and legacy sidecar discovery (below) scans it.
- `--run-file`: the runfile (schema `openwepp-hillslope-runfile-v1`), a TOML
  document whose `[inputs]` table names the WEPP soil, management, slope, and
  climate files and whose `[outputs]` table names the output targets. Resolved
  relative to `--run-dir` unless absolute.
- `--output-dir`: directory for the run manifest.

## Optional arguments

- `--legacy-sidecar-discovery`: discover legacy auxiliary inputs (`snow.txt`,
  `frost.txt`, `pmetpara.txt`, `wepp_ui.txt`, …) by scanning `--run-dir`, as
  legacy WEPP tooling expects. Without it, sidecars come only from runfile
  bindings. Unknown files in the run directory produce `LSB-W-002` warnings on
  `stderr` and are ignored.
- `--policy compat`: sidecar compatibility policy. `compat` is currently the
  only accepted value and is the default, so the flag may be omitted.
- `--manifest-path <path>`: write the manifest to this path instead of
  `<output-dir>/openwepp_hillslope_run_manifest.json`.
- `--direct-default-candidate` / `--direct-production-executor`: runtime
  selection provenance flags. The direct production runtime is the only
  execution path, so all invocations — with either flag or with none — run the
  same simulation; the flags only change the `selection_reason` recorded in the
  manifest (`default-candidate-direct-production-single-authority` versus
  `explicit-direct-production`). The two flags cannot be combined.

## Output

- Simulation outputs are written to the paths named in the runfile's
  `[outputs]` table: `pass` (HBP shard) and `loss` (JSON loss report) are
  required; `pass_parquet`, `wat`, and `plot` parquet outputs are optional.
  Relative output paths resolve within `--run-dir` — the common convention
  `pass = "output/H5.hbp"` places outputs under `<run-dir>/output/`.
- The hillslope ID is parsed from the `pass` filename (`H<id>…`), so the pass
  target must follow that naming.
- The run manifest (`openwepp-hillslope-run-manifest-v1`) records the source
  commit, runtime selection, and runtime counters (e.g. `day_frame_commits`),
  making the run artifact self-describing.

## Exit behavior

- `0`: success. Sidecar warnings, if any, are printed to `stderr` and do not
  affect the exit code.
- Non-zero: input, parsing, or runtime validation failure, with a diagnostic
  (`CLIHILL-E-*` or a located runtime error) on `stderr`.

## Manual validation reference

Verified 2026-07-01 against `openwepp-cli-hill` built from `main@56f5eed8`:

- Command (fixture copied to a scratch directory first):
  `openwepp-cli-hill --run-dir <copy of tests/fixtures/cli01/hillslope_run_dir>
  --run-file case.run --output-dir <scratch>/hill-out --policy compat
  --legacy-sidecar-discovery`
- Result: exit `0`; `H5.hbp`, `H5.loss.json`, `H5.wat.parquet`,
  `H5.plot.parquet` under `<run-dir>/output/`;
  `openwepp_hillslope_run_manifest.json` in `--output-dir` with
  `selection_reason: default-candidate-direct-production-single-authority`.

## Standard handoff language

- Use this handoff target for page updates:
  - `handoff: usersum/documentation-agent`
  - `target_path: usersum/documentation-agent.md`
