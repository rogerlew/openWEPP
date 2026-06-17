# PERFIDX04 Bit Identity Evidence

Ran:
- Baseline binary: `/tmp/perfidx04/baseline/bin/openwepp-cli-hill`, SHA `9a66ff3f3afceed4fdec2b319bb771c7604c6d354babdbeb2978aad3b688c32c`.
- Final current binary: `/tmp/perfidx04/current/bin/openwepp-cli-hill`, SHA `82c6cac78ed6b138b1b05750012082c1f8045602cf34004862adc48407d53e3c`.
- Final current anchor rerun timings and manifests were generated from the final current binary.

Ran:
- Compared OFE1-OFE5, H2637 no-UI, and H2637 with UI.
- `H1.hbp`/`H2637.hbp`, loss JSON, `wat.parquet`, and `plot.parquet` were byte-equal for every anchor case.
- `pass.parquet` bytes differed for every case, but logical row comparison by DuckDB `EXCEPT ALL` returned zero rows in both directions for every case.

Ran:
- Evidence file: `/tmp/perfidx04/artifacts/final-current-rerun-identity.tsv`.
- All rows have `byte_equal=1` for HBP/loss/WAT/PLOT and `baseline_minus_current_rows=0`, `current_minus_baseline_rows=0` for pass Parquet.
