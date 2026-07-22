# Line-Count Governance

Static: final `executor.rs` is 2,980 lines. It remains above the 2,000-line WARN
threshold but below the 3,000-line block. The target is a single trust-bearing
transaction module; further production splitting is deferred to a separately
authorized module-boundary package rather than mixed into this CRAP package.

Static: dual eligibility review requires new characterization in the declared
split `executor_coverage_tests.rs`, keeping `executor.rs` below the 3,000-line
block apart from a minimal child-module declaration.

Static: existing tests were moved mechanically into the declared test-only
split as production helpers were added. No new line-count or lint suppression
was introduced; the original `execute_plan_stage` `too_many_lines` suppression
was removed.
