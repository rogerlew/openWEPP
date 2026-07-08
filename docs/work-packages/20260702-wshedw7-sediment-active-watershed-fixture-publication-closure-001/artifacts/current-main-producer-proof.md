# Current-Main Producer Proof

Status: `passed`

Evidence mode: `Ran:`

## Release Binary

`cargo build --release -p openwepp-runner --bins` completed on current main.

- Commit: `97b23132b85c579041dee5de530d0b5aa319fbd7`
- `target/release/openwepp-cli-hill` SHA-256:
  `e88c5552f6fa98fae4282eb87095fb271a8dd5c0cf30a97431a483c46a8694e7`
- `target/release/openwepp-cli-watershed` SHA-256:
  `160f7f5d54d5aef4a1d2c12d82ada09f9326c2a6cf60840bf6882766675e6996`

## Producer Probe

Command:

```sh
target/release/openwepp-cli-hill \
  --run-dir /tmp/wshedw7r_p102_producer \
  --run-file p102.run \
  --output-dir /tmp/wshedw7r_p102_producer/output \
  --direct-production-executor
```

Result: `exit=0`, `wall=0:00.75`, `maxrss=20060`.

Generated outputs:

- `H102.hbp`
- `H102.loss.json`
- `H102.pass.parquet`
- `openwepp_hillslope_run_manifest.json`

Manifest proof:

- `selected_runtime = direct-production-executor`
- `output_policy = direct-production-executor/direct-publication-frame`
- `multi_ofe_wave1_chained = true`
- `erod14_qin_sediment_coupled = true`
- `erod14_qin_source_policy = wave1-hourly-sediment-coupled-handoff`
- `wave1_flux_refused_quanta = 7`

Pass parquet proof:

| Metric | Value |
| --- | ---: |
| rows | `3652` |
| `sum(tdet)` | `41531.85795763501` |
| `max(tdet)` | `2740.8131315844453` |
| days with `tdet > 0` | `130` |
| `sum(tdep)` | `29195.4647928195` |
| `max(tdep)` | `1443.7539799241335` |
| days with `tdep > 0` | `126` |
| `sum(sedcon_1)` | `3.107642343591381` |
| `sum(sedcon_2)` | `16.723330525534582` |
| `sum(sedcon_3)` | `21.514446993813298` |
| `sum(sedcon_4)` | `29.749996504558013` |
| `sum(sedcon_5)` | `20.764753811605612` |

Conclusion: the historical W7 producer-side zero-sediment blocker is gone on
current main for the real W7DC01 p102 substrate.
