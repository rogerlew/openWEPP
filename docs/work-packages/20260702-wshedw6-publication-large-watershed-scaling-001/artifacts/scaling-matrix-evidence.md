# Scaling Matrix Evidence

Status: `passed`

Evidence mode: `Ran:`

## Summary Artifacts

- JSON: `artifacts/scaling/onshore-xenophobia-scaling-summary.json`.
- JSON: `artifacts/scaling/carnivorous-adobo-scaling-summary.json`.
- CSV: `artifacts/scaling/w6-scaling-summary.csv`.

## Benchmark Surface

- Mode: strict committed fixture.
- Sidecar policy: `--policy compat`.
- Legacy sidecar discovery: disabled.
- Binaries: `target/release/openwepp-cli-watershed` and
  `target/release/openwepp-cli-hill`.
- Timing wrapper: `/usr/bin/time -v`.
- Output identity: Parquet schema and row equality via `pyarrow.parquet`.

## Onshore-Xenophobia Full Large Fixture

Fixture: `tests/fixtures/watershed/onshore-xenophobia/`.

Class: full `1305`-hillslope watershed, `544` channel elements, `100`-year
hillslope source run horizon.

Staging root: `/tmp/wshedw6_onshore_scaling_rerun`.

| Jobs | Wall time | User CPU | System CPU | CPU | Max RSS KiB | Worker elapsed ms | Dispatch ms | Publication ms | Hill jobs | Identity |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| `1` | `1:30:21` | `5366.86` | `54.82` | `99%` | `44128` | `5421433` | `2` | `64` | `1305` | baseline |
| `48` | `3:27.09` | `9334.72` | `99.82` | `4555%` | `44188` | `206535` | `2` | `62` | `1305` | PASS vs jobs1 |

Identity result: all `14` watershed parquet outputs matched `--jobs 1` by
schema and row content.

## Carnivorous-Adobo Existing Fixture

Fixture: `tests/fixtures/watershed/carnivorous-adobo/`.

Class: full `32`-hillslope committed development fixture.

Staging root: `/tmp/wshedw6_carnivorous_scaling_rerun`.

| Jobs | Wall time | User CPU | System CPU | CPU | Max RSS KiB | Worker elapsed ms | Dispatch ms | Publication ms | Hill jobs | Identity |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| `1` | `0:17.48` | `16.90` | `0.49` | `99%` | `18060` | `17384` | `0` | `80` | `32` | baseline |
| `32` | `0:00.99` | `22.35` | `0.52` | `2308%` | `18048` | `900` | `0` | `72` | `32` | PASS vs jobs1 |

Identity result: all `14` watershed parquet outputs matched `--jobs 1` by
schema and row content.

## Interpretation

The large fixture shows that publication is not the scaling bottleneck after
W6 typed-publication adoption: `output_publication_elapsed_ms` stayed at
`62-64 ms` on the full `1305`-hillslope onshore fixture. The dominant cost is
generated hillslope execution, as expected for a `100`-year source run.
