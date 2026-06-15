# CQR03 Quality Plan Report

Static: package objective was one quality dimension: cyclomatic complexity /
CRAP reduction in `runtime_inputs/01_management.rs`.

## Plan

1. Capture pre-refactor line, coverage, and CRAP evidence.
2. Add focused characterization where the target module was below glue-tier
   coverage.
3. Decompose the high-risk dispatcher and primary live-canopy helper into
   private helpers while preserving public surfaces, typed errors, symbols,
   thresholds, and arithmetic expression grouping.
4. Remove target-file `#[allow(clippy::too_many_lines)]` only after clippy
   accepts the decomposition.
5. Re-measure coverage and CRAP, then run required workspace gates.

## Result

Ran: the target module now has no `#[allow(clippy::too_many_lines)]` entries and
no production `unwrap()` or `expect()` in the target file.

Ran: before/after `cargo llvm-cov` and `cargo crap` evidence is recorded in this
artifact directory:

- `coverage_before_summary.json`
- `lcov_before.info`
- `crap_before.json`
- `coverage_after_summary.json`
- `lcov_after.info`
- `crap_after.json`

Closure: every eligible target-module function is `CRAP <= 30`; the maximum
after refactor is `17.16724537037037`.
