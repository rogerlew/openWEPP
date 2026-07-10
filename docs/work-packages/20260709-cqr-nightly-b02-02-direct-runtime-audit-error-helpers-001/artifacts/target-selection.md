# Target Selection — CQR Nightly Batch 02, Target 02

Ran: the fresh delegated baseline used an isolated Cargo target directory after
an initial attempt collided with another process's `target/llvm-cov-target`
cleanup. The collided attempt emitted only `17` source files and is rejected.
The accepted isolated run exited `0` for clean, coverage, CRAP, and ranking; it
emitted `170` source files, `/tmp/openwepp-cqr-nightly-new-isolated.lcov`
(`4,147,722` bytes, SHA-256
`87811062b33fe1c79176843204e01410aca9b2530c09cb60a98e3ec4a2f60cce`) and
`/tmp/openwepp-cqr-nightly-new-isolated-crap.json` (`2,745,705` bytes, SHA-256
`5e3abb273dffdc8c8308da0af9f3d85283307b2b2c77f0c9ba00c3a55741f765`).

`cargo llvm-cov --ignore-run-fail` recorded the known coverage-instrumented
`laned_shadow_h2637` failure (three tests); LCOV emission completed and is
acceptable for CRAP selection. The per-file ranking de-duplicates
`(file,function,line)` observations, excludes test/fixture paths and tracked
dirty or active-package overlap, and ranks by total excess, count, then maximum.

| Batch rank | Module | Excess CRAP | Functions >30 | Max CRAP |
|---:|---|---:|---:|---:|
| 2 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs` | 337.858 | 1 | 367.858 |

No tracked dirty or active package overlaps this target. Root `artifacts/` is
pre-existing untracked scratch space and is excluded from this package's write
set. The machine-readable ranking is
`/tmp/openwepp-cqr-nightly-new-isolated-module-rank.tsv`.
