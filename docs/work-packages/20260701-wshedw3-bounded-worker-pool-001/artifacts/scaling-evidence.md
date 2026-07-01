# Scaling Evidence

Status: `EXECUTED-COMPLETE-WSHED-W3`

W3 canonical scaling evidence is recorded on the committed
`tests/fixtures/watershed/carnivorous-adobo/` fixture after fixture-only
normalization of over-bound daily `radly` rows.

## Fixture Normalization

Evidence class: `Ran:`

- Production runtime still fails closed on over-bound `radly`; W3 did not add
  production clipping or normalization.
- User-authorized fixture-only normalization reduced `39` unique daily climate
  records, copied across `p1.cli` through `p32.cli` and `pw0.cli`, to
  `floor(baseline sunmap horizontal r3)` for latitude `48.25`.
- Clamp manifest:
  `artifacts/scaling/carnivorous-adobo-radly-clamp-manifest.json`.
- Fixture checksum manifest was regenerated and `sha256sum -c
  input-manifest.sha256` passed for all `175` fixture files.

## Release Scaling Matrix

Evidence class: `Ran:`

- Fixture: committed `tests/fixtures/watershed/carnivorous-adobo/`.
- Staging root: `/tmp/wshedw3_ca_scaling_release`.
- Binaries: `target/release/openwepp-cli-watershed` and
  `target/release/openwepp-cli-hill`.
- Mode: `--policy compat`; `--legacy-sidecar-discovery` disabled.
- Job counts: `1`, `2`, `4`, `8`, `16`, and `32`.
- Repeats: `3` clean repeats per job count.
- CPU inventory: `48` logical CPUs,
  `Intel(R) Xeon(R) CPU E5-2697 v2 @ 2.70GHz`.
- Summary artifact:
  `artifacts/scaling/carnivorous-adobo-release-scaling-summary.json`.
- CSV artifact:
  `artifacts/scaling/carnivorous-adobo-release-scaling-summary.csv`.
- Identity: every run's Parquet rows matched `jobs1-rep1` in existing row
  order across all `14` watershed output files.

| Jobs | Wall seconds by repeat | Average wall seconds | Max RSS KiB | Worker elapsed ms by repeat | Route stage ms by repeat |
| --- | --- | --- | --- | --- | --- |
| `1` | `37.26`, `36.69`, `36.93` | `36.96` | `24276`, `24288`, `24300` | `37155`, `36583`, `36812` | `2`, `1`, `2` |
| `2` | `19.38`, `19.59`, `18.41` | `19.13` | `24280`, `24320`, `24336` | `19247`, `19493`, `18311` | `2`, `1`, `2` |
| `4` | `9.52`, `9.52`, `9.56` | `9.53` | `24324`, `24300`, `24280` | `9428`, `9393`, `9469` | `1`, `2`, `1` |
| `8` | `5.05`, `5.22`, `5.06` | `5.11` | `24336`, `24292`, `24320` | `4952`, `5139`, `4940` | `1`, `1`, `2` |
| `16` | `2.61`, `2.80`, `2.59` | `2.67` | `24312`, `24300`, `24248` | `2507`, `2713`, `2485` | `1`, `1`, `1` |
| `32` | `2.02`, `2.06`, `2.03` | `2.04` | `24316`, `24280`, `24288` | `1905`, `1969`, `1943` | `2`, `1`, `1` |

Contextual surfaces:

- Arboreal-dendrite, `/wc1`, scratch, or legacy comparisons may be recorded
  only as contextual engineering-budget evidence.
- Discovery-on and discovery-off timings must not be compared as the same
  benchmark surface.
