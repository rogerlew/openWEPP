# PERFIDX05 Verification B

Ran:
- Rebuilt final release binary and compared SHA after clippy-only source cleanup.
- Ran seven-case anchor with legacy sidecar discovery:
  - OFE1-OFE5
  - H2637 no-UI
  - H2637 UI
- Compared outputs against the PERFIDX04 anchor with byte comparison and DuckDB row deltas.

Result:
- Final binary SHA used for timing and identity:
  `4eebabb5f4679b000516177271c996483ef639ca76697093797370685ec1c087`.
- `pmetpara` and `snow` sidecars were discovered in H2637 final manifests.
- Required bit identity held; `pass.parquet` row deltas were zero in both directions.
