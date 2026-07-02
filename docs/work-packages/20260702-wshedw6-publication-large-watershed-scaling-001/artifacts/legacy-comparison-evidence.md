# Legacy Comparison Evidence

Status: `recorded`

Evidence mode: `same-fixture pinned-legacy full watershed runs`

Pinned legacy authority:
`/workdir/wepp-forest_260430_baseline/release/wepp_260430`

All timings below ran every committed hillslope `pN.run` followed by the
watershed `pw0.run`; no fixture subset or representative slice was used.

## Summary

| Fixture | Hillslope runs | Watershed run | Exit | Wall time | User time | System time | Max RSS KiB | Non-empty stderr |
| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `carnivorous-adobo` | 32 | `pw0.run` | 0 | `1:41.77` | `38.02s` | `63.73s` | 2,877,312 | 0 |
| `onshore-xenophobia` | 1,305 | `pw0.run` | 0 | `2:33:14` | `6,582.99s` | `2,605.10s` | 2,907,648 | 0 |

`onshore-xenophobia` timing note: focused Rust verification commands ran on the
same host during the early part of the long pinned-legacy job. The run is valid
full-fixture completion evidence, but the wall time is not a perfectly isolated
benchmark measurement.

## Carnivorous Adobo

Stage:
`/tmp/wshedw6_legacy_carnivorous_20260701_215157`

Command class:
`/usr/bin/time -v bash -lc 'for i in $(seq 1 32); do wepp_260430 < p${i}.run; done; wepp_260430 < pw0.run'`

Observed evidence:

- `Exit status: 0`
- `stdout` logs: 33 (`p1` through `p32`, plus `pw0`)
- output files: 230
- stage size: 131M
- `pw0.stdout` ended with `WEPP COMPLETED WATERSHED SIMULATION SUCCESSFULLY`
- largest watershed outputs:
  - `pass_pw0.txt`: 16,183,320 bytes
  - `chnwb.txt`: 15,551,709 bytes
  - `ebe_pw0.txt`: 7,534,318 bytes
  - `soil_pw0.txt`: 6,268,531 bytes
  - `loss_pw0.txt`: 164,414 bytes

## Onshore Xenophobia

Stage:
`/tmp/wshedw6_legacy_onshore_20260701_234304`

Command class:
`/usr/bin/time -v bash -lc 'for i in $(seq 1 1305); do wepp_260430 < p${i}.run; done; wepp_260430 < pw0.run'`

Observed evidence:

- `Exit status: 0`
- `stdout` logs: 1,306 (`p1` through `p1305`, plus `pw0`)
- output files: 9,141
- stage size: 37G
- `pw0.stdout` ended with `WEPP COMPLETED WATERSHED SIMULATION SUCCESSFULLY`
- largest watershed outputs:
  - `pass_pw0.txt`: 6,823,747,351 bytes
  - `chnwb.txt`: 5,126,358,849 bytes
  - `soil_pw0.txt`: 2,066,438,851 bytes
  - `ebe_pw0.txt`: 4,566,193 bytes
  - `loss_pw0.txt`: 40,015,233 bytes

## Disposition

Both committed fixtures are runnable by the pinned legacy binary using their
full committed watershed inputs. Current openWEPP scaling evidence must compare
against these same full fixture identities; no subset timing can close W6.
