# Target Selection

Evidence label: Static/Ran.

Package: `20260709-cqr-nightly-03-runner-snowbench-001`

Target module: `crates/openwepp-runner/src/bin/openwepp-snowbench.rs`

Rank: `3` of `10`

Selection basis: live CQR nightly measurement from the current worktree.

## Measurement Commands

Ran:

- `cargo llvm-cov clean --workspace` - exit `0`
- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly.lcov` - exit `0`; command used `--ignore-run-fail` and package 1/2 artifacts record the known internal failures that were separately checked by focused/full tests.
- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-crap.json` - exit `0`

Artifacts:

- `/tmp/openwepp-cqr-nightly.lcov`
- `/tmp/openwepp-cqr-nightly-crap.json`
- `/tmp/openwepp-cqr-nightly-module-rank.tsv`

## Selected Row

| Rank | Module | Total excess CRAP | Functions > 30 | Max CRAP |
|---:|---|---:|---:|---:|
| 3 | `crates/openwepp-runner/src/bin/openwepp-snowbench.rs` | `1176.0` | `2` | `930.0` |

## Full Selected Set

| Rank | Module | Total excess CRAP | Functions > 30 | Max CRAP |
|---:|---|---:|---:|---:|
| 1 | `crates/openwepp-kernel-contract/src/lib_mod/core_types/01_typed_symbol_surfaces.rs` | `2915.6690906368485` | `3` | `2833.422607238448` |
| 2 | `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs` | `1593.634505414379` | `18` | `306` |
| 3 | `crates/openwepp-runner/src/bin/openwepp-snowbench.rs` | `1176.0` | `2` | `930.0` |
| 4 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs` | `892` | `10` | `306` |
| 5 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs` | `525.8809022887735` | `2` | `547.2389753179175` |
| 6 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs` | `504.9508413968741` | `7` | `296.51689035535117` |
| 7 | `crates/openwepp-input-contract/src/parsers/management.rs` | `482.1830023762919` | `6` | `203.62194460172833` |
| 8 | `crates/openwepp-runner/src/errors.rs` | `402.76867779158215` | `6` | `192.89940656693898` |
| 9 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs` | `386` | `4` | `272` |
| 10 | `crates/openwepp-runner/src/hillslope/laned_shadow.rs` | `374` | `3` | `210` |

## Exclusions

- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` was excluded because CQR nightly targets production Rust modules under `crates/`, not test modules.
- Existing untracked root `artifacts/` scratch files from package 1 are unrelated and must not be staged by this package.

## Worktree Check

Ran:

- `git status --short` before scaffolding showed only unrelated untracked root
  `artifacts/` scratch files from package 1.
