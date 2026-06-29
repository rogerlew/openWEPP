# H2637 Performance

Status: `RAN-PASS`

Evidence class: Ran + Static.

## Command

The decoupled arm was run on the H2637 endpoint with only the Stage 3 liquid
selector set. No density selector override was used.

```bash
OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1 \
/usr/bin/time -f 'h2637_stage3_decouple_direct\t%e\t%M\t%x' \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /home/workdir/openWEPP/target/paradigm2_stage3_decouple_water_temperature/h2637/runfiles/h2637_decoupled.run \
  --output-dir /home/workdir/openWEPP/target/paradigm2_stage3_decouple_water_temperature/h2637/decoupled/manifest \
  --direct-production-executor \
  --legacy-sidecar-discovery
```

## Result

| Arm | Wall time | Max RSS | Exit |
| --- | ---: | ---: | ---: |
| Stage 3-Decouple | `70.68 s` | `1150612 KiB` | `0` |

ADR-0025 budget reference from
`docs/architecture/array-native-runtime-specification.md`: legacy H2637
`9.12 s`; `<=10x` wall-time budget `91.2 s`. Stage 3-Decouple is `7.75x`
legacy and passes the `<=10x` gate.

The release binary used for the measured run:

```text
fad954e0b3440cea0abac54d951c96e57fd8e255e932e02c81bbe51099717110  target/release/openwepp-cli-hill
```

Selected endpoint output hashes:

```text
79709ab5ec291488ce09fdb6c873cbfd033b952a4cd2619f28c1342374a3647e  H2637.hbp
d0b02c228cc8b520bdc9a00eff9b593e7e4a9c3e3a04468da28823ba0051d36b  H2637.wat.parquet
bcc1fe03533f9faa510af4992861ac17c350d2f8c80e1ecfba41dcb9f544fadc  H2637.pass.parquet
```

The run manifest recorded `source_commit =
279c90fcd6190f4eff533bdab7d39e35f01dbbc4`, the pre-package commit, because
the performance run was executed from the dirty worktree carrying this package's
decouple implementation and evidence updates. The changed sources are committed
with this package closeout.
