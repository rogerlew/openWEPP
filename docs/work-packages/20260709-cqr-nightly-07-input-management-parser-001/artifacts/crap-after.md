# CRAP After

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
- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-07-management-targeted.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-07-management-targeted-crap.json`
  - exit `0`

Delegated full-workspace coverage attempt:

- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly-07-management-full.lcov`
  was attempted by `comparator_suite_runner` and blocked by unrelated
  coverage-instrumented `laned_shadow_h2637` failures/long-runs before
  producing LCOV.
- Because the package Phase D explicitly allows a documented targeted
  equivalent when full-workspace coverage is blocked by unrelated
  coverage-instrumented tests, the targeted workspace test LCOV above is the
  package after-metric evidence.

## Target CRAP Summary

Ran:

- Deduplicated target rows above CRAP `30`: `0`.
- Deduplicated target row count: `96`.
- Max target CRAP: `28.136080592592595`.

Top remaining deduplicated target rows:

| Function | Line | CC | Coverage | CRAP |
|---|---:|---:|---:|---:|
| `parse_plant_section` | `1562` | `23.0` | `78.66666666666666` | `28.136080592592595` |
| `validate_cross_section_references` | `2624` | `24.0` | `92.6829268292683` | `24.2256496568535` |
| `parse_yearly_section` | `2196` | `18.0` | `75.43859649122807` | `22.800699810467997` |
| `parse_yearly_annual_extension` | `2380` | `19.0` | `95.0` | `19.045125` |
| `parse_initial_forest` | `2019` | `14.0` | `71.42857142857143` | `18.57142857142857` |
| `parse_yearly_forest` | `2265` | `13.0` | `69.6969696969697` | `17.702674124161724` |

## Baseline Comparison

Static/Ran:

- Before: `6` deduplicated rows above CRAP `30`; max CRAP
  `203.62194460172833`.
- After: `0` deduplicated rows above CRAP `30`; max CRAP
  `28.136080592592595`.

Disposition:

- ADR-0021 CRAP closure target is met for this package.
