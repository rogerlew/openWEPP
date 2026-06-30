# Identity Evidence

Evidence class: Ran

## H2637 Full Output

Compared against the prior package baseline at
`/tmp/typed-direct-stage0/h2637/output`.

| Output | Result |
| --- | --- |
| `H2637.hbp` | byte-identical |
| `H2637.loss.json` | byte-identical |
| `H2637.pass.parquet` | byte-identical |
| `H2637.plot.parquet` | byte-identical |
| `H2637.wat.parquet` | byte-identical |

Post-fix output sizes:

| Output | Bytes |
| --- | ---: |
| `H2637.hbp` | `5254` |
| `H2637.loss.json` | `353` |
| `H2637.pass.parquet` | `399100` |
| `H2637.plot.parquet` | `216` |
| `H2637.wat.parquet` | `17813663` |
| `manifest.json` | `8205` |

Manifest evidence:

- `runtime_selection.selected = direct-production-executor`.
- `compatibility_edge_invocations = 0`.
- `day_frame_constructions = 235961`.
- `row_count = 235961`.
- `climate_day_count = 12419`.

## H2637 HBP/Loss-Only

Compared against the prior package baseline at
`/tmp/typed-direct-stage0/h2637_min/output`.

| Output | Result |
| --- | --- |
| `H2637.hbp` | byte-identical |
| `H2637.loss.json` | byte-identical |

Manifest evidence:

- `runtime_selection.selected = direct-production-executor`.
- `compatibility_edge_invocations = 0`.
- `day_frame_constructions = 235961`.
- `row_count = 235961`.
- `climate_day_count = 12419`.

## Short Run

The `cli01` post-fix manifest stayed on direct production:

- `runtime_selection.selected = direct-production-executor`.
- `compatibility_edge_invocations = 0`.
- `day_frame_constructions = 2`.
- `row_count = 2`.
- `climate_day_count = 2`.
