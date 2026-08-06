# Gate Results

Status: `technical gates PASS / carrier screen FAIL`

Evidence mode: `Ran`. TESTGATE was not used.

Exact model-free commands and exit codes are recorded in
`implementation-test-evidence.md`; the exact release/result commands and hashes
are recorded in `execution-receipt.md`.

| Gate | Result | Evidence |
| --- | --- | --- |
| Exact reviewed HEAD and clean source | PASS | `3ee1bac3ee849fbe00b517d1d227140f87fedc2a`, checked before build, analysis, and acceptance |
| Release build and binary retention | PASS | SHA-256 `4ffe2f9c...6021` |
| Four control plus four paired runs | PASS | 147 s |
| Exact fixture/observation/runfile consumers | PASS | retained receipt and 108-file manifest |
| WAT/HBP noninterference | PASS | exact bytes at all four sites |
| Every schema-v5 operand and identity | PASS | hourly, daily, water-year, and retained recomputation |
| Sampling adequacy | PASS | 154 eligible samples; canonical counts 34/44/41 |
| Observation-year census | PASS | every year dispositioned; WY2025 retained and excluded |
| Literature numeric comparability | PASS | Marks and Roth numerical mappings remain `NOT_COMPARABLE` |
| Frozen carrier screen | FAIL | canonical near-balance count `0/3` |
| Persistent-shadow advancement | FAIL / BLOCKED | required consequence of carrier-screen failure |

The scientific FAIL is the result, not an incomplete technical gate. It closes
this characterization package with nonpromotion and a named next diagnostic.
