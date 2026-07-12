# Validation

Evidence class: **Ran**

| Gate | Result | Evidence |
| --- | --- | --- |
| Red contract test | PASS as pre-fix reproduction | Exit 100; first NaN case returned wrong typed variant. |
| Final input-contract profile | PASS | `25/25` |
| Same-source coverage/CRAP | PASS | 97.698% lines, 98.590% regions, all floors pass, max target/helper CRAP 15.018 |
| Current-source CLI consumer | PASS | `1/1`, 28 skipped, exact-once run |
| Focused Clippy/format/diff | PASS | `-D warnings`; format/diff checks |
| Line governance | PASS | 996 lines, below WARN |

The red log is `/tmp/openwepp-cqr-high-a-hb05-nonfinite-red.log`, 2,511 bytes,
SHA-256 `f1ed6cff4639f7fb02736508af54658e3283b020978fbcd4b930f38d3accc7b8`.
It dynamically demonstrates the first table case; static same-mechanism review
establishes the remaining pre-fix non-finite classifications. Final exact
artifacts and provenance are in HB-05.md.
