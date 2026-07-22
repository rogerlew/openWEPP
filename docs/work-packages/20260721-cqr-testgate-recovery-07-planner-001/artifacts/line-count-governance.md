# Line-Count Governance

Static: scaffold `planner.rs` is exactly 2,999 lines, above the 2,000-line WARN
threshold and one line below the 3,000-line closure block. Existing inline tests
must move mechanically to the declared split before characterization or
production extraction adds lines.

Static: after the exact split, `planner.rs` is 2,405 lines and
`planner_coverage_tests.rs` is 595 lines. The production host remains WARN but
is 595 lines below the closure block; the test split is below WARN.

Static: after final corrected characterization and extraction, `planner.rs` is
2,409 lines and `planner_coverage_tests.rs` is 1,039 lines. Both remain below
the 3,000-line closure block; only the production host retains the documented
WARN.
