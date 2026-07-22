# Line-Count Governance

Static: the production target is 2,859 lines. This triggers the 2,000-line WARN but remains
below the 3,000-line closure blocker. The increase is attributable to private
helper decomposition and package-required characterization. Follow-on owner:
gate-planner maintainers. New direct characterization is already isolated in
the authorized test-only `pre_heavy_coverage_tests.rs` child module.

Disposition: WARN accepted; no 3,000-line blocker exists.
