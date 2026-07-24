# Implementation evidence

Evidence class: Ran

## Selection behavior

- Explicitly mapped groundwater and snow-phase sources now retain
  `BOUNDED_COMPONENT` risk and their applicable A1 hard-invariant definition.
- Unmapped science-package Rust remains `CRITICAL` with
  `SCIENCE_PACKAGE_WITHOUT_SEMANTIC_EDGE`.
- Increment plans admit `cargo-deny` only for dependency, manifest, build,
  toolchain, or deny-policy inputs.
- Increment plans admit required A3 authority only when the impact map selects
  authority suites. Campaign/release selection retains the complete registry.
- Affected packages receive package-scoped doctest inventory and execution;
  workspace doctests remain critical/campaign-strength evidence.

## Profile inventory

- Deterministic `full`: 2,260 tests.
- `science-manual`: 36 empirical/external tests.
- Intersection: zero tests.
- Deterministic full retains `cli01_runner_hillslope_integration`,
  `r7c_direct_production`, and `simimpl14_contract_gate`.

## Validation

- `cargo nextest run -p openwepp-gate-planner --profile affected`: 173 passed,
  14 ignored development-only fixtures, 60.154 seconds.
- `cargo nextest run --test testgate_ci_executor_contract`: 11 passed.
- `cargo nextest run --test testgate_align_authority_contract`: 11 passed.
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`:
  3 passed.
- Focused authority and doctest inventory tests: 2 passed.
- `cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings`: PASS.
- `bash -n tools/release/run_release_candidate_gates.sh`: PASS.

## Coverage-only defect correction

- Fresh affected CRAP attempt 1 stopped after 1,364.065 seconds when the
  coverage-only envelope fixture found that its synthetic JUnit builder used
  the host matrix target rather than each planner node's selection target.
- The first focused correction exposed the legitimate three-package inventory
  of a `NEXTEST_PACKAGES` node.
- The corrected builder now emits one stable JUnit case per selected package,
  or one case for a single package/workspace target.
- Ran with `cfg(coverage)`: the previously failing envelope fixture passed in
  70.725 seconds.
