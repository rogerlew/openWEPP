# Line-Count Governance

Static: `executor.rs` is 2,955 lines, above the warning threshold because its
large test module remains colocated. Production decomposition must not add a
line-count suppression; test-only splitting is preferred if needed.

Static: dual eligibility review requires new characterization in the declared
split `executor_coverage_tests.rs`, keeping `executor.rs` below the 3,000-line
block apart from a minimal child-module declaration.
