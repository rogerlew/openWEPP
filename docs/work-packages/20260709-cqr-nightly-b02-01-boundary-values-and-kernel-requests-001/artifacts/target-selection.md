# Target Selection — CQR Nightly Batch 02

Ran: a delegated fresh baseline used an isolated Cargo target directory after a
first attempt collided with another process's `target/llvm-cov-target` cleanup.
The collided attempt emitted only `17` source files and is rejected. The isolated
run exited `0` for clean, coverage, CRAP, and ranking; it emitted `170` source
files, `/tmp/openwepp-cqr-nightly-new-isolated.lcov` (`4,147,722` bytes,
`87811062b33fe1c79176843204e01410aca9b2530c09cb60a98e3ec4a2f60cce`) and
`/tmp/openwepp-cqr-nightly-new-isolated-crap.json` (`2,745,705` bytes,
`5e3abb273dffdc8c8308da0af9f3d85283307b2b2c77f0c9ba00c3a55741f765`).

`cargo llvm-cov --ignore-run-fail` recorded the known coverage-instrumented
`laned_shadow_h2637` failure (three tests); LCOV emission completed and is
acceptable for CRAP selection. The per-file ranking below de-duplicates
`(file,function,line)` observations, excludes test/fixture paths and tracked
dirty or active-package overlap, and ranks by total excess, count, then maximum.

| Batch rank | Module | Excess CRAP | Functions >30 | Max CRAP |
|---:|---|---:|---:|---:|
| 1 | `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs` | 359.210 | 4 | 183.888 |
| 2 | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs` | 337.858 | 1 | 367.858 |
| 3 | `crates/openwepp-input-contract/src/parsers/soil.rs` | 301.832 | 4 | 158.212 |
| 4 | `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | 266.197 | 6 | 192.321 |
| 5 | `crates/openwepp-runner/src/totalwatsed3.rs` | 237.469 | 4 | 110.000 |
| 6 | `crates/openwepp-input-contract/src/parsers/irrigation_depletion.rs` | 213.555 | 4 | 182.000 |
| 7 | `crates/openwepp-sim-contract/src/status.rs` | 176.000 | 3 | 182.000 |
| 8 | `crates/openwepp-topology/src/lib.rs` | 160.668 | 3 | 110.056 |
| 9 | `crates/openwepp-input-contract/src/parsers/slope.rs` | 156.908 | 2 | 182.000 |
| 10 | `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs` | 152.000 | 1 | 182.000 |

Excluded: all test/fixture/generated paths, pre-existing untracked scratch
artifacts, and none of the selected source paths are tracked-dirty or owned by an
active science/feature package. The machine-readable ranking is
`/tmp/openwepp-cqr-nightly-new-isolated-module-rank.tsv`.
