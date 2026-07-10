# Target Selection — CQR Nightly Batch 02, Target 03

Ran: accepted fresh isolated baseline measurement from
`/tmp/openwepp-cqr-nightly-new-isolated.lcov` and
`/tmp/openwepp-cqr-nightly-new-isolated-crap.json`, as described in batch target
01. It emitted 170 source files and exited 0 for clean, coverage, CRAP, and
ranking; the known instrumented `laned_shadow_h2637` failures were recorded
under `--ignore-run-fail` and did not prevent LCOV emission.

| Batch rank | Module | Excess CRAP | Functions >30 | Max CRAP |
|---:|---|---:|---:|---:|
| 3 | `crates/openwepp-input-contract/src/parsers/soil.rs` | 301.832 | 4 | 158.212 |

No tracked dirty or active package overlaps this target. Root `artifacts/` is
pre-existing untracked scratch space and is excluded. Baseline LCOV SHA-256 is
`87811062b33fe1c79176843204e01410aca9b2530c09cb60a98e3ec4a2f60cce`; CRAP
JSON SHA-256 is `5e3abb273dffdc8c8308da0af9f3d85283307b2b2c77f0c9ba00c3a55741f765`.
