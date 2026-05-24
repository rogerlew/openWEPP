# CLI02 wepppy Consumer Boundary Note

Status: complete
Evidence mode: Static

## Static
- `open_wepp_runner` is the openWEPP launcher boundary; legacy WEPP launch
  ownership remains in `wepppy/wepp_runner`.
- openWEPP hillslope `.run` is schema-versioned and metric-only:
  - `schema = "openwepp-hillslope-runfile-v1"`
  - `unit_system = "metric"`
- Required outputs for openWEPP hillslope runs:
  - `outputs.pass` (`.hbp`)
  - `outputs.loss` (`.json`)
- Optional outputs (when configured):
  - `outputs.wat`, `outputs.soil`, `outputs.plot`, `outputs.ebe`,
    `outputs.element` (all `.parquet`).
- Legacy compatibility sidecar mode remains available via
  `--legacy-sidecar-discovery`.
- In legacy mode, discovered sidecars are authoritative and `.run` sidecar
  overrides are ignored.

Consumer implication:
- wepppy should target the simplified output contract (`pass`/`loss` required,
  optional parquet families by explicit opt-in) for openWEPP hillslope runs.

## Ran
- None in CLI02 planning scope.
