# Scaling Matrix Evidence

Status: `passed`

Evidence mode: `Ran:`

## Summary Artifacts

- JSON: `artifacts/scaling/onshore-xenophobia-scaling-summary.json`
- JSON: `artifacts/scaling/carnivorous-adobo-scaling-summary.json`
- CSV: `artifacts/scaling/w6-scaling-summary.csv`

## Benchmark Surface

- Mode: strict committed fixture.
- Sidecar policy: `--policy compat`.
- Legacy sidecar discovery: disabled.
- Binaries: `target/release/openwepp-cli-watershed` and
  `target/release/openwepp-cli-hill`, rebuilt before the matrix.
- Timing wrapper: `/usr/bin/time -v`.
- Output identity: Parquet schema and row equality via `pyarrow.parquet`.
- Fixture policy: full committed fixtures only; no subset or representative
  slice.

The current output values are from the rebuilt release binaries after landed
hillslope work. Identity is judged within the same build (`--jobs 1` versus the
parallel job count), not against earlier W6 draft artifacts.

## Onshore-Xenophobia Full Large Fixture

Fixture: `tests/fixtures/watershed/onshore-xenophobia/`.

Class: full `1305`-hillslope watershed, `544` channel elements, `100`-year
hillslope source run horizon.

Staging root: `/tmp/wshedw6_onshore_scaling_final`.

| Jobs | Wall time | User CPU | System CPU | CPU | Max RSS KiB | Worker elapsed ms | Dispatch ms | Publication ms | Hill jobs | Identity |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| `1` | `1:31:51` | `5453.36` | `57.71` | `99%` | `44128` | `5510853` | `2` | `62` | `1305` | baseline |
| `48` | `3:27.55` | `9429.66` | `93.02` | `4588%` | `44128` | `206975` | `3` | `65` | `1305` | PASS vs jobs1 |

Identity result: all `14` watershed parquet outputs matched `--jobs 1` by
schema and row content.

Selected publication values from the identity run:

- `Area = 118391327.80000003 m^2`
- `Runoff = 7.623926319373555e-06 mm`
- `runvol = 0.9026067600000022 m^3`
- `Q = 0.9026067600000023 m^3`
- `chanwb Inflow = 0.9026067600000022 m^3`
- `chanwb Outflow`, `Storage`, `Baseflow`, `Loss`, and `Balance` are null
  because W6 does not publish authoritative channel-balance volume operands for
  those fields.

## Carnivorous-Adobo Existing Fixture

Fixture: `tests/fixtures/watershed/carnivorous-adobo/`.

Class: full `32`-hillslope committed development fixture.

Staging root: `/tmp/wshedw6_carnivorous_scaling_final`.

| Jobs | Wall time | User CPU | System CPU | CPU | Max RSS KiB | Worker elapsed ms | Dispatch ms | Publication ms | Hill jobs | Identity |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| `1` | `0:19.11` | `18.05` | `0.54` | `97%` | `18444` | `18970` | `0` | `67` | `32` | baseline |
| `32` | `0:01.07` | `24.04` | `0.52` | `2278%` | `18048` | `983` | `0` | `77` | `32` | PASS vs jobs1 |

Identity result: all `14` watershed parquet outputs matched `--jobs 1` by
schema and row content.

Selected publication values from the identity run:

- `Area = 3333484.6800000006 m^2`
- `Runoff = 9.539206881850737e-07 mm`
- `runvol = 0.0031798800000000004 m^3`
- `Q = 0.003179880000000001 m^3`
- `chanwb Inflow = 0.0031798800000000004 m^3`
- `chanwb Outflow`, `Storage`, `Baseflow`, `Loss`, and `Balance` are null
  because W6 does not publish authoritative channel-balance volume operands for
  those fields.

## Interpretation

The large fixture shows that publication is not the scaling bottleneck after
W6 typed-publication adoption: `output_publication_elapsed_ms` stayed at
`62-65 ms` on the full `1305`-hillslope onshore fixture. The dominant cost is
generated hillslope execution, as expected for a `100`-year source run.
