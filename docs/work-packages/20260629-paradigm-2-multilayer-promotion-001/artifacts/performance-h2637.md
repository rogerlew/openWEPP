# H2637 Performance

Status: `RAN-PASS`

Evidence class: Ran.

This artifact records the H2637 direct-production performance rerun with the
production-supported opt-in selector.

## Command

```bash
OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1 \
/usr/bin/time -f 'h2637_multilayer_promotion_direct\t%e\t%M\t%x' \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /home/workdir/openWEPP/target/paradigm2_multilayer_promotion/h2637/runfiles/h2637_promoted.run \
  --output-dir /home/workdir/openWEPP/target/paradigm2_multilayer_promotion/h2637/promoted/manifest \
  --direct-production-executor \
  --legacy-sidecar-discovery
```

No density selector override was set.

## Result

| Arm | Wall time | Max RSS | Exit |
| --- | ---: | ---: | ---: |
| Promoted opt-in | `70.65 s` | `1153680 KiB` | `0` |

ADR-0025 budget reference from
`docs/architecture/array-native-runtime-specification.md`: legacy H2637
`9.12 s`; `<=10x` wall-time budget `91.2 s`. The promoted arm is `7.75x`
legacy and passes.

The release binary used for the measured run:

```text
9f12ff55a50faaa90664cee1d5a169680caa2588bdd7b6ea0f9115b6ed3050bc  target/release/openwepp-cli-hill
```

Selected output hashes:

```text
79709ab5ec291488ce09fdb6c873cbfd033b952a4cd2619f28c1342374a3647e  H2637.hbp
8f4415c66fb23af4f0e4c3e79f8d0eff3408009fb4b612f61747556d0cc92111  H2637.wat.parquet
bcc1fe03533f9faa510af4992861ac17c350d2f8c80e1ecfba41dcb9f544fadc  H2637.pass.parquet
```

The run manifest recorded `source_commit =
6b567499d76733c06cd2f0882af76ad48870d4af`, the pre-package commit, because
the run was executed from this dirty worktree carrying the promotion package
changes. The release binary checksum above is the executable used.
