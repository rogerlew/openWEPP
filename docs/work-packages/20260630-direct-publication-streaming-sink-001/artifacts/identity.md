# Identity Evidence

Evidence class: Ran + Static

## Byte Identity

Compared against
`/tmp/direct-publication-rss/stage-b3-h2637-full/output` and
`/tmp/direct-publication-rss/stage-b3-cli01/output`.

| Case | Output | Result |
| --- | --- | --- |
| H2637 full | `H2637.hbp` | byte-identical |
| H2637 full | `H2637.loss.json` | byte-identical |
| H2637 full | `H2637.plot.parquet` | byte-identical |
| H2637 full | `H2637.wat.parquet` | byte-identical |
| H2637 full | `H2637.pass.parquet` | byte-identical |
| cli01 | `H5.hbp` | byte-identical |
| cli01 | `H5.loss.json` | byte-identical |
| cli01 | `H5.plot.parquet` | byte-identical |
| cli01 | `H5.wat.parquet` | byte-identical |

The parquet outputs are byte-identical, which is stronger than the scoped
schema/row-count/value identity gate. The row-group writer currently preserves
the same byte layout for these measured fixtures; future row-group layout
changes remain allowed only under schema/value identity.

## Direct Runtime Counters

| Case | Selected executor | Compatibility edges | Day-frame constructions | Row count |
| --- | --- | ---: | ---: | ---: |
| H2637 full | `direct-production-executor` | `0` | `235961` | `235961` |
| H2637 required-only | `direct-production-executor` | `0` | `235961` | `235961` |
| W9 longer-day fixture | `direct-production-executor` | `0` | `16437` | `16437` |
| cli01 | `direct-production-executor` | `0` | `2` | `2` |
