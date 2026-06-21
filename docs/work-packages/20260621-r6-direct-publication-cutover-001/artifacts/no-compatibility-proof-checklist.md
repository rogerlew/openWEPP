# No-Compatibility Proof Checklist

Status: blocked.
Evidence mode: Static + Ran.

## Forbidden Direct-Publication Reads

After cutover, direct publication must not read:

- `HillslopeWritebackSurface`
- `KernelWritebackPayload`
- `BoundarySymbol`
- `BoundaryValue`
- `SymbolRegistry`
- `HotSymbolTables`
- `IndexedWritebackSurface`
- dense refresh state
- dirty flush state
- stale logical output frames
- compatibility diagnostic ledgers as publication authority

## Required Proof

- source scans over direct publication and output-family builders;
- call-graph or focused static proof for each output family;
- runtime counters showing direct publication output families use typed
  projection and do not enter compatibility publication readers;
- default-disabled counters showing no direct-publication construction when the
  direct path is disabled.

## Gate

BLOCKED. Static inspection after ledger promotion confirms the current public
output path still uses compatibility WB13 rows and runtime surfaces:

```text
rg -n "fn build_hbp_output|runtime_surface: &HillslopeWritebackSurface|build_hillslope_wat_rows\\(&execution\\.wb13_rows|write_hillslope_pass_parquet\\(|build_loss_output_json\\(|write_hillslope_run_manifest\\(" \
  crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs
```

The scan identifies `build_hbp_output` taking
`&HillslopeWritebackSurface`, WAT rows built from `execution.wb13_rows`, PASS
parquet written from `execution.pass_rows` derived from WB13 rows, loss JSON
built from static/climate compatibility inputs, and manifest publication using
the current checksum/provenance helpers.

No direct-publication no-compatibility proof can pass until a run-bound direct
publication frame supplies the promoted ledger operands.
