# Line-Count Governance

Evidence mode: Static.

- New Rust test target is intentionally small and below warning thresholds.
- New Python diagnostic is `578` lines of package tooling, not production
  runtime. It remains single-file so report generation, WAT/trace lineage,
  same-SWE cap projection, and Policy-B matrix logic are reproducible from one
  command.
- No production Rust file is mechanically expanded by this package.
