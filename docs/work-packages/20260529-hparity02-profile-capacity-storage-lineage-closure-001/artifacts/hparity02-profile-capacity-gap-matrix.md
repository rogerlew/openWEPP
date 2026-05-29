# HPARITY02 Profile-Capacity Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Baseline vs current comparison scope
- Static: HPARITY01 baseline residual source:
  `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/hillslope_semantic_summary.json`
- Ran: HPARITY02 rerun summary:
  `/tmp/hparity02_20260529T204555Z/parity/reports/hillslope_semantic_summary.json`

## Column-family gap matrix
| Column | HPARITY01 fail hillslopes (baseline) | HPARITY02 fail hillslopes (current) | Delta | Status |
| --- | ---: | ---: | ---: | --- |
| `ProfileDepth` | 39 | 0 | -39 | closed |
| `ProfilePorosityCap` | 39 | 0 | -39 | closed |
| `ProfileFCStore` | 39 | 27 | -12 | open |
| `ProfileWPStore` | 39 | 1 | -38 | open |

## Control-column integrity (`MEASURE-HP02-004`)
| Column | HPARITY02 fail hillslopes | Status |
| --- | ---: | --- |
| `Area` | 0 | pass |
| `P` | 0 | pass |
| `Q` | 39 | fail |
| `Er` | 0 | pass |
| `Tile` | 0 | pass |
| `Irr` | 0 | pass |
| `QOFE` | 39 | fail |
| `SubRIn` | 0 | pass |
| `UpStrmQ` | 0 | pass |
| `frozwt` | 0 | pass |

## Row-presence integrity (`MEASURE-HP02-003`)
- Ran: all `39/39` hillslopes have:
  - `common_row_count = 1461`
  - `only_baseline_count = 0`
  - `only_candidate_count = 0`
