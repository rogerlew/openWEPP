# Disposition

Evidence label: Static/Ran.

Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`

## Result

Static/Ran:

- Target module:
  `crates/openwepp-input-contract/src/parsers/management.rs`.
- Baseline target rows above CRAP `30`: `6`.
- Final target rows above CRAP `30`: `0`.
- Final max target CRAP: `28.136080592592595`.
- Final target line coverage: `89.81854838709677%`.
- Final target region coverage: `86.46770237121831%`.
- Final production line count: `2960` lines.

## Review Disposition

Static/Ran:

- Review A finding:
  - Severity: Medium.
  - Issue: contract-sensitive landuse guard helper was not reused by surface and
    drain sections.
  - Disposition: accepted and fixed.
- Review B finding:
  - Severity: High.
  - Issue: package closure artifacts were stale/inconsistent.
  - Disposition: accepted and fixed.

## Gate Disposition

Ran:

- Focused parser tests: PASS.
- Focused YAML tests: PASS.
- Targeted coverage/CRAP: PASS.
- `git diff --check`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo nextest run --workspace --profile full`: PASS, post-review delegated
  run `1566/1566`.
- `cargo deny check`: PASS.
- `markdown-doc lint` over package docs and work-package README: PASS.

## Coverage/CRAP Substitution

Static/Ran:

- Full-workspace LCOV after implementation was attempted by the required
  `comparator_suite_runner` and blocked before LCOV output by unrelated
  coverage-instrumented `laned_shadow_h2637` failures/long-runs.
- Package Phase D allows a targeted equivalent in this condition.
- Targeted workspace coverage/CRAP for the touched management parser tests
  passed and proves the package CRAP and glue-tier coverage closure targets.

Final disposition:

- `EXECUTED-COMPLETE-CQR-NIGHTLY`
