# Target Selection

Evidence label: Static/Ran.

Status: `SCAFFOLDED`

Git status before scaffold commit:

- `## main...origin/main [ahead 10]`
- Unrelated untracked root artifact scratch files remain outside this package
  and are not part of the write set.

Live nightly measurement provenance:

- `cargo llvm-cov clean --workspace` - exit `0`.
- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly.lcov` - exit `0`.
- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-crap.json` - exit `0`.

Selected production modules:

| Rank | Module | Total excess CRAP | Rows > 30 | Max CRAP |
|---:|---|---:|---:|---:|
| 1 | `crates/openwepp-kernel-contract/src/lib_mod/core_types/01_typed_symbol_surfaces.rs` | `2915.669` | `3` | `2833.422` |
| 2 | `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs` | `1593.634` | `18` | `306.0` |
| 3 | `crates/openwepp-runner/src/bin/openwepp-snowbench.rs` | `1176.0` | `2` | `930.0` |
| 4 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs` | `892.0` | `10` | `306.0` |
| 5 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs` | `525.880` | `2` | `547.238` |
| 6 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs` | `504.950` | `7` | `296.516` |
| 7 | `crates/openwepp-input-contract/src/parsers/management.rs` | `482.183` | `6` | `203.621` |
| 8 | `crates/openwepp-runner/src/errors.rs` | `402.769` | `6` | `192.899` |
| 9 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs` | `386.0` | `4` | `272.0` |
| 10 | `crates/openwepp-runner/src/hillslope/laned_shadow.rs` | `374.0` | `3` | `210.0` |

Exclusion recorded from selection pass:

- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` excluded as
  a test file even though it had high CRAP rows.

Current package target row:

- Rank `8` of `10`.
- Target module:
  `crates/openwepp-runner/src/errors.rs`.
- Deduplicated total excess CRAP: `402.76867779158215`.
- Deduplicated functions above CRAP `30`: `6`.
- Max CRAP: `192.89940656693898`.
