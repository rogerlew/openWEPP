# CQR04 Coverage Closure

Ran:

```text
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr04-watershed-routing-complexity-001/artifacts/lcov_before.info
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr04-watershed-routing-complexity-001/artifacts/lcov_after.info
```

Baseline target summary from `coverage_before_summary.json`:

- Functions: 25/28 = 89.28571428571429%.
- Lines: 1247/1636 = 76.22249388753056%.
- Regions: 1733/2153 = 80.49233627496515%.

After target summary from saved LCOV:

- Functions: 64/77 = 83.116883116883%.
- Lines: 1788/2264 = 78.975265017668%.
- Branches: LCOV branch counters unavailable for this target.

Coverage disposition: reviewed scoped hold.

Rationale:

- Baseline target coverage was already below the science-tier `>= 90%` line and
  region threshold.
- The refactor added private helpers and improved target line coverage
  percentage, but function coverage dropped because some extracted branches are
  only reached by unrepresented case-3 and low-width-shear fixtures.
- The package objective was CRAP/cyclomatic decomposition with behavior
  preservation; no routing formula or public surface changed.
- Focused WS10/WS11 tests and full workspace tests passed before and after.

Follow-on test debt:

- Add targeted characterization for `ws20_case3_xdbeg_value` and adjacent
  case-3 helpers.
- Add targeted characterization for `ws26_dcap_low_width_shear_outcome`.
