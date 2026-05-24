# openWEPP Hillslope `.run` Contract

Status: `draft-normative`

This contract defines the human-authorable, schema-versioned `.run` file for
openWEPP hillslope execution.

## Scope

- Declarative hillslope run configuration in TOML.
- Two sidecar-discovery behaviors:
  - default `.run`-driven sidecar controls,
  - optional legacy compatibility discovery via CLI flag.
- Output configuration that preserves the legacy hillslope output family
  controls while using openWEPP output formats.

## Schema ID and Versioning

Required schema ID:

```text
openwepp-hillslope-runfile-v1
```

Versioning rules:

- breaking changes require a new schema ID (`v2`, `v3`, ...),
- additive backward-compatible keys remain in `v1`.

## File Format

- extension: `.run`
- encoding: UTF-8
- syntax: TOML 1.0
- one run definition per file

Legacy line-oriented WEPP stdin `.run` recipes are out of contract for this
surface.

## Execution Modes

`openwepp-cli-hill` supports two sidecar modes:

1. Default mode (no legacy discovery flag):
   - sidecar controls come from this `.run` contract (`[inputs]`,
     `[inputs.snow]`, `[inputs.frost]`).
2. Legacy compatibility mode (`--legacy-sidecar-discovery`):
   - runtime discovers legacy files in the run directory.
   - discovered legacy sidecars are authoritative; `.run` sidecar override
     keys (`inputs.wepp_ui`, `inputs.pmetpara`, `inputs.snow`, `inputs.frost`)
     are ignored in this mode.

## Input Contract

Top-level required keys:

- `schema` (exactly `openwepp-hillslope-runfile-v1`)
- `run_name` (human-readable run label)
- `unit_system` (exactly `metric`)

`[inputs]` required keys:

- `soil` (`.sol` path)
- `management` (`.man` path)
- `slope` (`.slp` path)
- `climate` (`.cli` path)

`[inputs]` optional keys:

- `wepp_ui` (`bool`, default `false`)
- `pmetpara` (`string` path)

`[inputs.snow]` optional override table:

- `rst` (`float`)
- `newsnw` (`float`)
- `ssd` (`float`)

`[inputs.frost]` optional override table:

- `wintRed` (`int`)
- `fineTop` (`int`)
- `fineBot` (`int`)
- `ksnowf` (`float`)
- `kresf` (`float`)
- `ksoilf` (`float`)
- `kfactor1` (`float`)
- `kfactor2` (`float`)
- `kfactor3` (`float`)

Important semantic rule:

- `snow.txt` and `frost.txt` are parameter overrides only.
- Their presence/absence does not control whether snow or frost routines run.

`wepp_ui` semantics:

- `wepp_ui = true` requests feature-flag behavior.
- `wepp_ui = false` leaves feature-flag behavior disabled.

`pmetpara` semantics:

- optional input surface;
- if omitted, runtime uses no PMET override sidecar;
- if provided, path must resolve to a readable file.

## Legacy Discovery Compatibility Mode

When launched with `--legacy-sidecar-discovery`, runtime checks for these
legacy files in the run directory:

- `snow.txt` (optional)
- `frost.txt` (optional)
- `wepp_ui.txt` (optional feature-flag sentinel)
- `pmetpara.txt` (optional)

Legacy parity note:

- `pmetpara.txt` is optional in legacy WEPP (`infile.for`, branch between
  `iflget=2` when present and `iflget=1` when missing).

## Unit System Contract (Metric Only)

openWEPP hillslope `.run` execution is metric-only.

- `unit_system` must be present and set to `metric`.
- alternate unit selectors (for example `english`) are out of contract.
- no implicit or automatic unit conversion is performed at this boundary.

## Output Contract

The `.run` must configure outputs under `[outputs]` and preserve legacy
hillslope output-family configurability.

`[outputs]` required keys:

- `pass` (`string` path, must end in `.hbp`)
- `loss` (`string` path, must end in `.json`)

`[outputs]` optional keys:

- `wat` (`string`, optional, must end in `.parquet`)
- `soil` (`string`, optional, must end in `.parquet`)
- `plot` (`string`, optional, must end in `.parquet`)
- `ebe` (`string`, optional, must end in `.parquet`)
- `element` (`string`, optional, must end in `.parquet`)

Required outputs:

- pass output at `outputs.pass` (`.hbp`)
- loss output at `outputs.loss` (`.json`)

Optional outputs:

- `outputs.wat` when provided
- `outputs.soil` when provided
- `outputs.plot` when provided
- `outputs.ebe` when provided
- `outputs.element` when provided

`outputs.wat` metadata parity requirements (when provided):

- canonical consumer-facing artifact naming is `H.wat.parquet`;
- emitted schema must preserve field-level metadata keys `units` and
  `description` for canonical water-balance projection columns;
- emitted schema metadata must include version keys `dataset_version`,
  `dataset_version_major`, `dataset_version_minor`, and `schema_version`;
- authoritative WAT projection is WB13 canonical daily schema with explicit
  post-`wepp_260430` consumer-lineage extension allowance for optional
  producer-authoritative `InterceptionStorage`.

`crop` output is intentionally excluded from this contract revision because its
columnar/output authority is not yet ratified.

Run manifest note:

- run provenance manifest emission remains required by the CLI specification,
  but manifest path selection is launcher-managed, not a `.run` `outputs` key.

If required outputs are missing after run completion, execution must fail.

## Path Semantics

All path values in this contract accept:

- absolute paths, or
- relative paths.

Relative-path rule:

- resolve relative to the directory containing the `.run` file.

Safety rules:

- no shell interpolation is performed,
- no glob expansion is performed,
- missing required input paths are hard errors,
- missing required output parent directories must hard-fail if runtime cannot
  create them.

## Minimal Example

```toml
schema = "openwepp-hillslope-runfile-v1"
run_name = "oak-creek-hs-001"
unit_system = "metric"

[inputs]
soil = "runs/p1.sol"
management = "runs/p1.man"
slope = "runs/p1.slp"
climate = "runs/p1.cli"
wepp_ui = true
pmetpara = "runs/pmetpara.txt"

[inputs.snow]
rst = 0.0
newsnw = 100.0
ssd = 250.0

[inputs.frost]
wintRed = 1
fineTop = 5
fineBot = 15
ksnowf = 0.4
kresf = 0.3
ksoilf = 0.2
kfactor1 = 1.0
kfactor2 = 1.0
kfactor3 = 1.0

[outputs]
pass = "output/H1.hbp"
loss = "output/H1.loss.json"
wat = "output/H1.wat.parquet"
soil = "output/H1.soil.parquet"
plot = "output/H1.plot.parquet"
element = "output/H1.element.parquet"
```

## Legacy Compatibility Launch Example

```bash
openwepp-cli-hill \
  --run-dir /path/to/runs \
  --run-file p1.run \
  --output-dir /path/to/output \
  --legacy-sidecar-discovery
```

In that mode, `snow.txt`, `frost.txt`, `wepp_ui.txt`, and `pmetpara.txt` are
optional discoveries from the run directory.
