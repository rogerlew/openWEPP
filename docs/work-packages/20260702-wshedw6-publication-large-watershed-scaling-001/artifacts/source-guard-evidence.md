# Source Guard Evidence

Status: `passed`

Evidence mode: `Static:` source scans and focused tests.

## Typed Publication Source Guard

Command:

```sh
rg -n "publication_frame_to_row_seed|write_watershed_interchange_outputs|WatershedInterchangeRowSeed|write_typed_publication_parquet_outputs|write_interchange_parquet_outputs" \
  crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs \
  crates/openwepp-watershed-output/src/writers.rs \
  crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs \
  tests/integration/cli03_runner_contract_derived_tests.rs
```

Result:

- Public watershed CLI imports and calls
  `write_typed_publication_parquet_outputs`.
- Public watershed CLI has no `publication_frame_to_row_seed`,
  `write_watershed_interchange_outputs`, or `WatershedInterchangeRowSeed`
  marker.
- `WatershedInterchangeRowSeed` and `write_interchange_parquet_outputs*`
  remain in `crates/openwepp-watershed-output/src/writers.rs` for retained
  non-public edge callers and writer regression tests.
- Source guard tests in
  `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` forbid
  those row-seed markers in the public CLI.

## Full-Fixture Source Guard

Command:

```sh
rg -n "representative reduction|representative fixture|large/representative|subset|subsetting" \
  docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001 \
  tests/fixtures/watershed/onshore-xenophobia
```

Result:

- Package and active prompt now state that representative reduction is not a
  W6 closure path.
- `onshore-xenophobia` fixture README and artifacts explicitly state that no
  watershed subsetting was applied.
- Scaling artifacts record full committed fixture execution for both accepted
  fixtures.

## Operator-Path Guard

Command:

```sh
rg -n "/wc1|wepppy" \
  tests/fixtures/watershed/onshore-xenophobia/runs \
  tests/fixtures/watershed/onshore-xenophobia/input-manifest.sha256 || true

rg -n "/wc1|wepppy" \
  tests/fixtures/watershed/carnivorous-adobo/runs \
  tests/fixtures/watershed/carnivorous-adobo/input-manifest.sha256 || true
```

Result: no matches for either committed fixture run directory or manifest.

Fixture READMEs intentionally name `/wc1` as provenance and say persistent
gates must not read it.
