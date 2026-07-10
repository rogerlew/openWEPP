# Coverage After

Evidence label: Static/Ran.

Status: `EXECUTED-PASS`

Target module:
`crates/openwepp-input-contract/src/parsers/management.rs`

## Command Provenance

Ran:

- `cargo llvm-cov --workspace --test infile_management_parser_contract --test infile_management_yaml_contract --lcov --output-path /tmp/openwepp-cqr-nightly-07-management-targeted.lcov`
  - exit `0`
- `cargo llvm-cov --workspace --test infile_management_parser_contract --test infile_management_yaml_contract --json --output-path /tmp/openwepp-cqr-nightly-07-management-targeted-llvmcov.json --no-clean`
  - exit `0`

Delegated full-workspace coverage attempt:

- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly-07-management-full.lcov`
  was attempted by `comparator_suite_runner`.
- Result: blocked before LCOV production by unrelated
  `tests/integration/laned_shadow_h2637.rs` coverage-instrumented failures and
  long-runs.
- Log:
  `/home/workdir/openWEPP/artifacts/cqr-20260709-cqr-nightly-07-input-management-parser-001/02-cargo-llvm-cov.log`.
- Package Phase D allows a targeted equivalent in this exact condition, so the
  focused workspace test LCOV is the package coverage evidence.

## Target Coverage Summary

Ran:

| Metric | Covered | Count | Percent |
|---|---:|---:|---:|
| Lines | `1782` | `1984` | `89.81854838709677` |
| Regions | `2115` | `2446` | `86.46770237121831` |
| Functions | `102` | `113` | `90.2654867256637` |
| Instantiations | `108` | `121` | `89.25619834710744` |
| Branches | `0` | `0` | `0.0` |

Baseline comparison:

- Before line coverage: `1474/1916`, `76.931106471816%`.
- After line coverage: `1782/1984`, `89.81854838709677%`.

Disposition:

- ADR-0021 glue-tier line threshold (`>=85%`) is met.
- ADR-0021 glue-tier region threshold (`>=85%`) is met.
- The target module is non-regressed versus baseline.
