# CAL-07D Pre-Execution Inventory

Evidence class: `Static`

Source commit: `11b1faab37b5d365b0c0c7051ed32a4762821239`

| Dependency | SHA-256 | Role |
| --- | --- | --- |
| CAL-07C `inputs/ensemble.csv` | `80c783720fbcb6f412ae9480483651f56fd2bf6cef2b206aecd2fe8a1854a68f` | Frozen 37-member CAL-04B ensemble |
| CAL-07C `inputs/forcing.csv` | `cd87b4fcebea7432f8f47d633c1585ea40a24f3e20365b05fd0815dd36d550bc` | Frozen Beza forcing and admitted Alerce forcing |
| CAL-07C `inputs/observations.csv` | `e0ff37a48c91c154e3e1410916c42bfe38047d8b9b1a3422f28d470da936d1f9` | Diagnostic-only admitted PhenoCam GCC90 |
| CAL-07C `inputs/transitions.csv` | `ab784c6eb5f180094d76e0264df601bdf63af760a7c8e87a72b9937873ff3c41` | Provisional PhenoCam transition dates |
| CAL-07C `artifacts/daily-kernel-output.csv` | `8185ede4852f3a04645faba143368736906e0991c2e7811394b0d68df1412172` | Frozen GSI/canopy execution result |
| CAL-07C `artifacts/transition-residuals.csv` | `376c1b2102712e32d255a1854ef5b82327bf8366a7ca28f4eea97c408b9eec52` | Result to reproduce exactly |
| CAL-07 source transition table | `db477b36731d0a8c072ac400dac3aa135e84234408d79a1a6a10eded739632cd` | Source-native Beza transition product |
| CAL-07 source daily GCC table | `a490b29758ce0608428c6e794d8c803727b60fddc4e601c875564a26ed514f1f` | Source-native Beza daily product |

Expected inventories:

- 37 frozen members;
- 1,666 complete Beza forcing days;
- 61,642 frozen Beza member-days;
- four internally bracketed 2024--2025 deciduous transition events;
- 148 member/event rows per operator or scenario;
- 11 CAL-07C absolute event-window matches; and
- no production or predecessor-evidence mutation.

Pre-execution empirical role: all Beza PhenoCam rows and transition dates are
`DIAGNOSTIC_ONLY`.
