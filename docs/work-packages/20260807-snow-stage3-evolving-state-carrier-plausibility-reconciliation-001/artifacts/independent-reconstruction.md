# Independent Reconstruction

Ran: exact attempt 004 at clean `e07cdbdf9`.

The analyzer streamed all eight retained schema-v6 traces, verified frozen
hashes and per-site climate/observation custody, and independently reconstructed
every tuple's radiation, turbulent exchange, precipitation advection, raw vapor,
bounded transfer, cold/melt chronology, endpoints, active state, support, and
applicability. All `154` eligible water-year rows were emitted; WY2025 remained
right-censored.

| Site | Eligible WYs | Truncated tuples | Active-state failures | Median support omission | Maximum | Support pass |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Mica Creek | `34` | `0` | `0` | `0.00546954` | `0.0476340` | `PASS` |
| Niwot | `44` | `0` | `0` | `0.00851551` | `0.0348640` | `PASS` |
| Paradise | `41` | `0` | `0` | `0.00356159` | `0.0621730` | `FAIL` (WY2015) |
| Snowbird | `35` | `0` | `0` | `0.00416720` | `0.0164657` | `PASS`, non-decisive |

No producer/independent transfer or melt mismatch, endpoint failure, N/A alias,
nonfinite evidence, continuity failure, or shortwave drift occurred. Paradise
WY2015 retains `183` unmatched hours and `19` partial-support hours; its frozen
omission ratio is not erased by the site median.

Result JSON SHA-256:
`7bd19a24b63375dba9f61e8d522afcc43b42b9f9a8dd8d6156cbe9fad1fbbbff`.
Execution receipt SHA-256:
`ba922327a66184112bbcebd45dc0ec4d6f2ccd1d885e0eb085b7279de1b5cc59`.
