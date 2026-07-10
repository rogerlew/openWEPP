# Target Selection — CQR Nightly Batch 02, Target 06

Ran: accepted the fresh batch LCOV/CRAP measurement at
`/tmp/openwepp-cqr-nightly-new-isolated.{lcov,crap.json}`. It emitted LCOV under
the known `laned_shadow_h2637` coverage-only failures with `--ignore-run-fail`.

| Batch rank | Module | Excess CRAP | Functions >30 | Max CRAP |
|---:|---|---:|---:|---:|
| 6 | `crates/openwepp-input-contract/src/parsers/irrigation_depletion.rs` | 206.789 | 3 | 182.000 |

No tracked dirty or active package overlaps target. Root `artifacts/` is
pre-existing untracked scratch space and excluded. Baseline LCOV SHA-256:
`87811062b33fe1c79176843204e01410aca9b2530c09cb60a98e3ec4a2f60cce`; CRAP
JSON SHA-256: `5e3abb273dffdc8c8308da0af9f3d85283307b2b2c77f0c9ba00c3a55741f765`.
