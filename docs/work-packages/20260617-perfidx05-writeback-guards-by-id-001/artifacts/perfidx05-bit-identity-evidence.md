# PERFIDX05 Bit Identity Evidence

Ran:
- Final binary:
  - PERFIDX04 anchor: `82c6cac78ed6b138b1b05750012082c1f8045602cf34004862adc48407d53e3c`
  - PERFIDX05 final: `4eebabb5f4679b000516177271c996483ef639ca76697093797370685ec1c087`
- Final anchor command shape matched PERFIDX04:
  `openwepp-cli-hill --run-dir ... --run-file ... --output-dir ... --policy compat --legacy-sidecar-discovery`.
- Evidence files:
  - `/tmp/perfidx05/artifacts/final-current-rerun-identity.tsv`
  - `/tmp/perfidx05/artifacts/final-current-rerun-times.tsv`
  - `/tmp/perfidx05/artifacts/final-h2637-sidecar-keys.tsv`

Result:
- `H1/H2637.hbp`: byte-identical for OFE1-OFE5, H2637 no-UI, and H2637 UI.
- `loss.json`: byte-identical for all seven cases.
- `wat.parquet`: byte-identical for all seven cases; DuckDB row deltas were also `0/0`.
- `plot.parquet`: byte-identical for all seven cases.
- `pass.parquet`: byte differs, matching prior container churn posture; DuckDB row deltas
  were `0/0` for all seven cases.

Diagnostic discarded:
- An earlier PERFIDX05 rerun omitted `--legacy-sidecar-discovery`, which caused `snow.txt`
  to be absent from H2637 manifests and produced real H2637 output deltas. Those outputs
  were moved to `/tmp/perfidx05/current_final_no_legacy_discovery` and the corresponding
  timing/identity TSVs were renamed with `no-legacy-discovery`. They are not final evidence.
