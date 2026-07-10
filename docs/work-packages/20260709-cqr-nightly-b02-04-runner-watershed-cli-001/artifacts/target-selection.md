# Target Selection — CQR Nightly Batch 02, Target 04

Ran: accepted the fresh isolated batch measurement from
`/tmp/openwepp-cqr-nightly-new-isolated.lcov` and
`/tmp/openwepp-cqr-nightly-new-isolated-crap.json`. It exited `0` for clean,
coverage, CRAP, and ranking; the known instrumented `laned_shadow_h2637`
failures were recorded under `--ignore-run-fail` without preventing LCOV output.

| Batch rank | Module | Excess CRAP | Functions >30 | Max CRAP |
|---:|---|---:|---:|---:|
| 4 | `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 266.197 | 6 | 192.321 |

No tracked dirty or active package overlaps the target. Root `artifacts/` is
pre-existing untracked scratch space and is excluded. Baseline LCOV SHA-256 is
`87811062b33fe1c79176843204e01410aca9b2530c09cb60a98e3ec4a2f60cce`; CRAP
JSON SHA-256 is `5e3abb273dffdc8c8308da0af9f3d85283307b2b2c77f0c9ba00c3a55741f765`.
