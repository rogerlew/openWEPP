# H2637 Performance Evidence

Status: `RAN-PASS-NON-PROMOTION`

Evidence class: Ran + Static.

ADR-0025 requires a real H2637 endpoint timing for any Stage 3 promotion or
default claim. The deferred gate was run after the Stage 3 observed guardrail
exposed and the package fixed a stale-cold-content domain bug.

## Commands

Stage 1 rollback baseline:

```bash
OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_multilayer_density_v1 \
OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=disabled \
/usr/bin/time -f 'h2637_stage1_direct\t%e\t%M\t%x' \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /home/workdir/openWEPP/target/paradigm2_stage3_liquid_routing/h2637/runfiles/h2637_stage1.run \
  --output-dir /home/workdir/openWEPP/target/paradigm2_stage3_liquid_routing/h2637/stage1/manifest \
  --direct-production-executor \
  --legacy-sidecar-discovery
```

Stage 3 candidate:

```bash
OPENWEPP_SNOWDENSITY09_DENSITY_MODEL=physics_bulk_multilayer_density_v1 \
OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1 \
/usr/bin/time -f 'h2637_stage3_direct\t%e\t%M\t%x' \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /home/workdir/openWEPP/target/paradigm2_stage3_liquid_routing/h2637/runfiles/h2637_stage3.run \
  --output-dir /home/workdir/openWEPP/target/paradigm2_stage3_liquid_routing/h2637/stage3/manifest \
  --direct-production-executor \
  --legacy-sidecar-discovery
```

## Results

| Arm | Wall time | Max RSS | Exit |
| --- | ---: | ---: | ---: |
| Stage 1 rollback | `69.91 s` | `1150608 KiB` | `0` |
| Stage 3 candidate | `72.59 s` | `1150608 KiB` | `0` |

Delta versus Stage 1 rollback: `+2.68 s` (`+3.83%`) and no RSS increase.

ADR-0025 budget reference from
`docs/architecture/array-native-runtime-specification.md`: legacy H2637
`9.12 s`; `<=10x` wall-time budget `91.2 s`. Stage 3 is `7.96x` legacy and
passes the `<=10x` gate. Using the same `235961` OFE-day reference, Stage 3 is
approximately `307.64 us/OFE-day`, under the `386 us/OFE-day` budget.

The release binary used for the measured runs:

```text
0a2e4ee3c629db0deecef00e85a3aff036b10c9fe56edce525c37a7ea57f0af5  target/release/openwepp-cli-hill
```

The embedded run manifest `source_commit` remains the pre-deferred package
commit because these measurements were run from a dirty worktree carrying the
deferred-gate cold-content fix, observed-gate wrapper, and evidence updates.
That is intentional for this follow-on gate run; the changed sources are
committed with this package closeout.

## Output Identity

Protected public endpoint outputs were identical for the Stage 1 rollback and
Stage 3 candidate:

```text
f7445e409c96ffb830532f73749d01286b3212a3cafb22802f5005fce0a929ee  H2637.hbp
a7876e01aa3aa03d5581f435b26655d8adbff312d650720ffff3ca4673ef44d3  H2637.wat.parquet
b89b12bb867ec36421fa40a451a277bee59ecee10aef7f8ab69fc785b37744ec  H2637.pass.parquet
```

`H2637.loss.json` differed only by `run_name`
(`paradigm2-stage3-h2637-stage1` versus
`paradigm2-stage3-h2637-stage3`). `H2637.plot.parquet` is an ASCII optional
output marker in this run; it also differed only by `run_name`.

## Hot-Frame Size Guard

The direct-runtime size guard still passes after Stage 3 diagnostics were moved
to optional boxed trace carry:

```text
DirectRunConstructorInputs=72
DirectLaneConstructorInputs=1024
DirectDayConstructorInputs=4016
DirectRunFrame=256
DirectLaneFrame=1136
DirectDayFrame=12248
```

Disposition: H2637 endpoint performance is no longer a missing gate. The
package remains non-promoted because the full opt-in arm still inherits the
Stage 1 snow guardrail failure against the current no-env default.
