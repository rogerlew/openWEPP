# Target Selection — CQR Nightly Batch 02, Target 05

Ran: accepted the fresh isolated batch measurement from
`/tmp/openwepp-cqr-nightly-new-isolated.lcov` and
`/tmp/openwepp-cqr-nightly-new-isolated-crap.json`. It emitted LCOV despite the
known coverage-only `laned_shadow_h2637` failures under `--ignore-run-fail`.

| Batch rank | Module | Excess CRAP | Functions >30 | Max CRAP |
|---:|---|---:|---:|---:|
| 5 | `crates/openwepp-runner/src/bin/openwepp-cli-totalwatsed3.rs` | 2.121 | 1 | 32.121 |

No tracked dirty or active package overlaps the target; root `artifacts/` is
pre-existing untracked scratch space and excluded. Baseline LCOV SHA-256:
`87811062b33fe1c79176843204e01410aca9b2530c09cb60a98e3ec4a2f60cce`; CRAP
JSON SHA-256: `5e3abb273dffdc8c8308da0af9f3d85283307b2b2c77f0c9ba00c3a55741f765`.
