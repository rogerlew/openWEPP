# R4B Default-Disabled Regression Gate

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
  --output-dir <r4b-output-root>/default/repN/h2637_same \
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

Build evidence:

- `/usr/bin/time -f 'release_build\t%e\t%M' cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
  - passed;
  - elapsed `58.00 s`;
  - max RSS `1089796 KB`;
  - binary SHA256
    `106e416a808e4973b9de22dd7c67f3b0461e28235754ffcecdeb710e080b46ee`;
  - sidecar SHA256
    `cc2513e4687d4157f9085da64bc7ec148596ff1a83fd512de959b70ba7844e4d`.

Default-disabled H2637 command shape used for each rep:

```text
target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run \
  --output-dir /tmp/r4b-h2637-final/default/repN/h2637_same \
  --policy compat \
  --legacy-sidecar-discovery
```

Results:

| Rep | Time | RSS | Manifest SHA256 |
|---|---:|---:|---|
| 1 | `637.34 s` | `229120 KB` | `8cceb2b45f8b32b584c805fd9c31e5ce6840f21b4bfb4d54ab4e4dfad5299633` |
| 2 | `641.14 s` | `228320 KB` | `170b42e02df060c2279dd8a2e638ccbb7e9877abeca0249378b33e55941cdee5` |
| 3 | `646.88 s` | `228820 KB` | `310a6e83b7ec6b4a4ee6c0072b2700450db264e2600b76b2ee3bb7845194ebb4` |

Median:

- min/median/max: `637.34 / 641.14 / 646.88 s`;
- gate: PASS, median `641.14 s <= 676.67 s`.

Protected output identity:

- HBP SHA256:
  `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8`
- loss SHA256:
  `4d4421a2dcc1275af607059605249517d7f605f4431644aa4e675966daf8e021`
- WAT SHA256:
  `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474`
- plot SHA256:
  `1419d03fad4b5f8dbd8aad6aabae95a6c10934a9e4d7f8ef65437968a12926d6`

PASS parquet byte hashes varied by rep, so row equivalence was used:

```text
baseline_rows     12419
candidate_rows    12419
left_minus_right      0
right_minus_left      0
column_count          17
```

Conclusion:

R4B preserved the default-disabled performance gate and protected output
identity.
