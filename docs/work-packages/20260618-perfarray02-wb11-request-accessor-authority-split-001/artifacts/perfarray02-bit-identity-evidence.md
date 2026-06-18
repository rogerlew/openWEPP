# PERFARRAY02 Bit Identity Evidence

Evidence: Ran.

## OFE5 Ladder

Commands:

```text
target/release/openwepp-cli-hill --run-dir /tmp/openwepp_farpoint01_h2637/without_ui/runs \
  --run-file /tmp/perfidx06/runfiles/ofe5_same_current.run --policy compat \
  --legacy-sidecar-discovery --output-dir /tmp/perfarray02/ofe5_default_final

OPENWEPP_PERFARRAY02_ARRAY_RUNOFF_PILOT=1 OPENWEPP_PERFARRAY02_TIMING=1 \
  target/release/openwepp-cli-hill --run-dir /tmp/openwepp_farpoint01_h2637/without_ui/runs \
  --run-file /tmp/perfidx06/runfiles/ofe5_same_current.run --policy compat \
  --legacy-sidecar-discovery --output-dir /tmp/perfarray02/ofe5_pilot_final
```

Timing:

| Variant | Seconds | Max RSS KB |
| --- | ---: | ---: |
| default | 22.34 | 28280 |
| pilot | 31.96 | 27868 |

Checksums:

| Output | Match | Default | Pilot |
| --- | --- | --- | --- |
| `H1.hbp` | true | `9854b904158bf1f26a31b7e54d95475b63729137efc5e00e988c3824d1387160` | same |
| `H1.loss.json` | true | `6483d39bf356867430ad8e152d677f0c743596657d5a69984932857c85b65055` | same |
| `H1.plot.parquet` | true | `c1136222a2a0a7145873303eea6978a919a94fbb0c77fde93f4447732c9ccdbc` | same |
| `H1.wat.parquet` | true | `879cb74933948aedeed9f20f55b504b09cc47bbed82a97d9b2f80efd6f943548` | same |
| `H1.pass.parquet` bytes | false | `7e6d4497144f4cfd07af4e8fa73943e64eb5ce55989b88af9494993cd6824567` | `39b597bd11dca50b64053800d71e4de83a33919698f0d2eece9407a7c3ebe5fc` |

Pass parquet rows:

```text
pass_rows 2192 2192
pass_cols 17 17
pass_schema_equal True
pass_rows_equal True
```

## H2637

Commands used the same run dir and:

```text
--run-file /tmp/perfidx06/runfiles/h2637_same_current.run
```

Timing:

| Variant | Seconds | Max RSS KB |
| --- | ---: | ---: |
| default | 671.88 | 229108 |
| pilot | 1096.11 | 229920 |

Checksums:

| Output | Match | Default | Pilot |
| --- | --- | --- | --- |
| `H2637.hbp` | true | `44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8` | same |
| `H2637.loss.json` | true | `cdebc8ce7594d3ca24fd532f53652e50343211135c2f51c0317a03464c5843f2` | same |
| `H2637.plot.parquet` | true | `41f26fb8dfb1be1ce5e7ed177e9cc4eda894d294ccd955bae7374e41a5d2e539` | same |
| `H2637.wat.parquet` | true | `c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474` | same |
| `H2637.pass.parquet` bytes | false | `60d090d6fb33e69323a6684f134aa2dba1929cd9eb5858b679aed52f372db1d0` | `c13c5de39655c500e730028cc729cc47c29a43f10ce03bc1029deaf5e95b72c0` |

Pass parquet rows:

```text
pass_rows 12419 12419
pass_cols 17 17
pass_schema_equal True
pass_rows_equal True
```

Conclusion: required identity passed. Pass parquet byte checksums churned, but row
equality and schema equality passed for OFE5 and H2637.
