# R4D Default-Disabled Regression Gate

Status: complete.
Evidence mode: Ran.

Gate:

- H2637 default-disabled median must be `<= 676.67 s`.
- Protected output identity must pass.
- PASS parquet row equivalence must pass when parquet bytes vary.

Required command shape:

```text
target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run \
  --output-dir <r4d-output-root>/default/repN/h2637_same \
  --policy compat \
  --legacy-sidecar-discovery
```

Required disabled env posture:

- unset `OPENWEPP_PERFDEEP02_FRAME_ISLAND`
- unset `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE`
- unset `OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH`
- unset `OPENWEPP_INDEXED_SHADOW_REPORT_PATH`
- unset `OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH`
- unset `OPENWEPP_HPHYS0245_TRACE_PATH`

Release binary:

```text
cffb6189e1df98f0abd012e994639d1a3738d5ca5b5013a7de6f4c3fa79d779e  target/release/openwepp-cli-hill
33f254cb1921277c53073e3bcd31c9e6e7f28d6e71b553d5bb4dff538eeb73cd  target/release/openwepp-cli-hill.json
```

Default-disabled H2637 results:

| Rep | Time | Max RSS | Warning posture |
|---|---:|---:|---|
| 1 | `635.94 s` | `229128 KB` | only known `MOFE01-MG-W-001` |
| 2 | `650.91 s` | `228988 KB` | only known `MOFE01-MG-W-001` |
| 3 | `645.47 s` | `228336 KB` | only known `MOFE01-MG-W-001` |

Median: `645.47 s`.

Gate: PASS (`645.47 s <= 676.67 s`).

Output hashes:

```text
22d8ec9e925ed8eb53ad38a1b41dd276f9ba792e66583d30ff427702b1609588  /tmp/r4d-h2637-final/default/rep1/h2637_same/openwepp_hillslope_run_manifest.json
2970a9c7c2e87773690b4a618dd9224341d54861c7d2979594fc821f3edd325d  /tmp/r4d-h2637-final/default/rep2/h2637_same/openwepp_hillslope_run_manifest.json
392f4a49272f5357ef46cd2ff613c739c578c2f8e98a14b4ec6b2a5755888580  /tmp/r4d-h2637-final/default/rep3/h2637_same/openwepp_hillslope_run_manifest.json
44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8  /tmp/perfmig01-final/current/anchor/h2637_same/H2637.hbp
4d4421a2dcc1275af607059605249517d7f605f4431644aa4e675966daf8e021  /tmp/perfmig01-final/current/anchor/h2637_same/H2637.loss.json
43337a83db133d21a0e76a9f748bfc531797c9a02f3f28954dc61a691bb3bb97  /tmp/perfmig01-final/current/anchor/h2637_same/H2637.pass.parquet
1419d03fad4b5f8dbd8aad6aabae95a6c10934a9e4d7f8ef65437968a12926d6  /tmp/perfmig01-final/current/anchor/h2637_same/H2637.plot.parquet
c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474  /tmp/perfmig01-final/current/anchor/h2637_same/H2637.wat.parquet
```

PASS parquet row equivalence against
`/tmp/perfdeep07/default/rep1/h2637_same/H2637.pass.parquet`:

| baseline rows | candidate rows | baseline minus candidate | candidate minus baseline | columns |
|---:|---:|---:|---:|---:|
| `12419` | `12419` | `0` | `0` | `17` |
