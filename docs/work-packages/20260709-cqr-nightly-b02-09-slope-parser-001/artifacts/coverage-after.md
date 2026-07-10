# Coverage After

Command:

`cargo llvm-cov --workspace --test infile_slope_parser_contract --lcov --output-path /tmp/openwepp-cqr-b02-t09-focused2.lcov`

Ran:

- `tests/integration/infile_slope_parser_contract.rs`: 27/27 passed.
- Refreshed focused LCOV:
  `/tmp/openwepp-cqr-b02-t09-focused2.lcov`
- LCOV SHA-256:
  `d4a8eeb38430dac92e7206270ea2ac926f1dbc4b79e58394218ed558cc509ce4`
- Matching LLVM export for region summary:
  `/tmp/openwepp-cqr-b02-t09-final3-llvm-export.json`
- LLVM export SHA-256:
  `01b8cc6ffc76e5d9b5bb0ba13f4c98dab545e013c1a42018b83585da696db94b`

Target source coverage:

| Metric | Covered | Total | Percent |
|---|---:|---:|---:|
| Lines | 628 | 677 | 92.7622% |
| Regions | 668 | 728 | 91.7582% |
| Functions | 39 | 44 | 88.6364% |

Status: PASS for ADR-0021 glue tier.

Full-workspace coverage note:

- Delegated after-LCOV path `/tmp/openwepp-cqr-b02-t09-after.lcov` was
  unusable for this target: the runner reported underlying cargo-test exit
  `101`, no `slope.rs` LCOV record, and CRAP coverage `null`.
- Parent full-workspace LCOV attempt
  `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-b02-t09-after2.lcov`
  was terminated with exit `143` before completion.
- The focused workspace-instrumented LCOV above is the valid target-module
  after-metric evidence.
