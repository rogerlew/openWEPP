# CLI04 WEPPpy Consumer Boundary Note

Status: completed
Evidence mode: Static + Ran

## Static
Consumer authority references:
- `/home/workdir/wepppy/wepppy/wepp/interchange/hill_wat_interchange.py`
- `/home/workdir/wepppy/wepppy/wepp/interchange/versioning.py`

CLI04 consumer-boundary alignment:
- `outputs.wat` now emits valid parquet with Arrow schema metadata keys aligned
  to WEPPpy `schema_with_version` expectations:
  - `dataset_version`
  - `dataset_version_major`
  - `dataset_version_minor`
  - `schema_version`
- Field metadata parity is preserved for `units` and `description`, including
  optional producer-authoritative extension semantics for
  `InterceptionStorage`.
- CLI runfile contract shape remains stable for consumers:
  - required outputs remain `pass` (`.hbp`) and `loss` (`.json`),
  - `wat` remains optional `.parquet` output.
- Python wrapper boundary is not API-broken by CLI04 changes; output semantics
  moved from placeholder payload to real parquet while preserving declared
  contract surface.

Transition note:
- Contract authority target remains `crates/openwepp-output/`.
- Active implementation remains on transition predecessor crate
  `crates/openwepp-hillslope-output/` in this package.

## Ran
- `cargo test --test cli04_runner_wat_parquet_contract_derived_tests`
  - pass (`2 passed; 0 failed`).
  - verifies readable parquet plus required schema/field metadata keys.
- `PYTHONPATH=. pytest -q tests/python/test_open_wepp_runner_api.py`
  - pass (`5 passed`).
