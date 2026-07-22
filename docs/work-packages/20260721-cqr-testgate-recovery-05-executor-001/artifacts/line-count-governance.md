# Line-Count Governance

Static: `executor.rs` is 2,955 lines, above the warning threshold because its
large test module remains colocated. Production decomposition must not add a
line-count suppression; test-only splitting is preferred if needed.
