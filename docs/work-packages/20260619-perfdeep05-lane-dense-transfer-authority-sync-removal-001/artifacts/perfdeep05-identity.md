# PERFDEEP05 Identity

Evidence class: Ran.

## Final-Code H2637 Runs

Final release binary SHA from both run manifests:

```text
6833a30b57ef7a96b409437a656b91037e9db7e0a3a77b24471bcdaf299a07a6
```

Commands:

```text
/usr/bin/time -f "perfdeep05_final_h2637_default_endpoint\t%e\t%M" \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/runfiles/perfdeep05-h2637-default.run \
  --output-dir /tmp/perfdeep05/final/default/h2637_same_manifest \
  --policy compat \
  --legacy-sidecar-discovery

/usr/bin/time -f "perfdeep05_final_h2637_optin_endpoint\t%e\t%M" \
  env OPENWEPP_PERFDEEP03_LANE_DENSE_STATE=1 \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/runfiles/perfdeep05-h2637.run \
  --output-dir /tmp/perfdeep05/final/optin/h2637_same_manifest \
  --policy compat \
  --legacy-sidecar-discovery
```

The CLI runfile writes primary H2637 artifacts to:

- default: `/tmp/perfdeep05/default/h2637_same/`
- opt-in: `/tmp/perfdeep05/current/h2637_same/`

The command `--output-dir` captured the final manifests under
`/tmp/perfdeep05/final/...`.

Both runs emitted the existing scoped sidecar warning:

```text
MOFE01-MG-W-001 EROD14 Wave-2 qin is seeded from water-transfer provenance only; true sediment-coupled qin/qout and particle-fraction handoff remains MOFE01 M-G follow-on scope.
```

## File Identity

| Artifact | Result | Default SHA-256 | Opt-in SHA-256 |
|---|---|---|---|
| `H2637.hbp` | byte-identical | `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8` | `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8` |
| `H2637.wat.parquet` | byte-identical | `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474` | `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474` |
| `H2637.pass.parquet` | Arrow-equivalent rows | `5ee2248faffe9daa6adb9f2769eeebcab73f95afaf368fd4df6fa1d4fe271567` | `eccc93ac916a5f942d0ee801f235a7604d18cb3a493ed5bc14d0b1701b66c9fc` |
| `H2637.plot.parquet` | `run_name` only | `455802936add26c93ed725c941ff2c6a5165d50a875527819bd5a08b3a93ee79` | `feca0245b0f67064271d1d97120baf36b60f9d426d7fd4810434c19ea711b016` |
| `H2637.loss.json` | `run_name` only | `e5bd82e6111b52210258d466452261450fc4c54aa12fb8c3230cab4521e35480` | `2a875a849c3a4e68e3c43663dc33691a4f225bb8705265c4fac7bd67c74ac787` |

PASS Arrow check:

```text
pass_schema_equal=True
pass_shape=12419x17/12419x17
pass_columns_equal=True
pass_data_equal=True differing=[]
```

Loss JSON check:

```text
loss_without_run_name_equal=True
```

Plot diff:

```text
-run_name=perfdeep05-default-h2637
+run_name=perfdeep05-h2637
```

## Conclusion

Acceptance criterion met: H2637 HBP and WAT are byte-identical, PASS is
Arrow-equivalent, and the remaining optional artifact differences are only run
name metadata.
