# open_wepp_runner (Python)

Compatibility-facing Python wrapper for launching openWEPP through the canonical
Rust launcher boundary:

```text
open_wepp_runner run-hillslope ...
```

## Goals

- Provide a call surface compatible with `wepppy.wepp_runner` where practical.
- Keep invocation explicit and shell-safe (argument arrays only).
- Fail explicitly for unsupported surfaces (no silent fallback).

## Implemented in CLI03-Aligned Scope

- `make_hillslope_run`
- `run_hillslope`
- `get_linux_wepp_bin_opts`
- `infer_pass_family_for_wepp_bin`

`make_hillslope_run` emits schema-versioned TOML runfiles using
`openwepp-hillslope-runfile-v1` with metric-only unit declaration and required
CLI03 output bindings (`pass` `.hbp`, `loss` `.json`).

`run_hillslope` verifies required output files from the runfile `[outputs]`
table (`pass`, `loss`) after runner execution.

## Explicitly Not Yet Implemented

- Watershed runfile creation/execution
- Any `ss` / `ss_batch` API surfaces

Flowpath surfaces are intentionally omitted because flowpath execution is
deprecated in wepppy.

Unsupported functions are either omitted from this package API or raise
`NotImplementedError` with stable `OPEN_RUNNER-E-*` identifiers.

## Binary Resolution

Defaults:

- runner binary: `open_wepp_runner`
- hillslope binary: `openwepp-cli-hill`

Optional environment overrides:

- `OPENWEPP_RUNNER_BIN`
- `OPENWEPP_HILLSLOPE_BIN`
- `OPENWEPP_HILLSLOPE_LATEST_BIN`
- `OPENWEPP_SIDECAR_POLICY` (`strict` or `compat`, default `strict`)

## Pass Family Note

The Python wrapper still exposes pass-family constants for compatibility
discovery helpers, but `make_hillslope_run` only accepts `pass_family="hbp"` at
the CLI03 runner boundary. Legacy ASCII pass-family runfile generation is
rejected with `OPEN_RUNNER-E-026`.

Binary lookup order:

1. explicit environment override,
2. `open_wepp_runner/bin/<name>`,
3. system `PATH`.
