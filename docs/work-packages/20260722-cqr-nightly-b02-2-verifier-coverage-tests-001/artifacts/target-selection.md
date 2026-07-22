# Target Selection

Static: exact retained row is
`verifier_coverage_tests.rs::replace_string`, line 39, CC 7, coverage missing,
CRAP 56. Report SHA-256 is
`1f1511234455fc7024d4e263fc481345e5f0e59509c81c47411eeeaaa438c129`.
Both selection reviewers chose this module as rank 2 of 2.

Terminal reclassification addendum: the retained row and selection history are
preserved, but the source was compiled only inside `#[cfg(test)] mod tests`.
RTR-043 moved it to the truthful natural path
`crates/openwepp-gate-planner/src/verifier/tests/coverage_tests.rs`. The
canonical production CRAP predicates exclude exact `/src/tests/` paths, so the
terminal row is `TEST-ONLY-NON-PRODUCTION` and would not enter the CQR candidate
universe. This is a corrected source-role identity, not an ADR waiver.
