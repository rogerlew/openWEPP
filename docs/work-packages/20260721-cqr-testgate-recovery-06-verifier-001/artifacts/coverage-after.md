# Coverage After

Ran: the first changed-head matching-module traversal at exact clean HEAD
`9970ac32444cf58299867dfd9767a86e25914b03` reports:

- production lines: 1,114/1,279 (87.0993%);
- production deduplicated LLVM regions: 1,727/2,019 (85.5374%);
- compiled functions below 75% region: none;
- tests: 138 passed, 0 failed, 2 intentionally ignored;
- test time: 510.50 seconds; total wall: 534.98 seconds.

Static: production is `verifier.rs` through line 1,676. The inline test module
and `verifier_coverage_tests.rs` are excluded from the eligible denominator;
the one non-Linux `read_confined` function was not compiled in this profile.

Ran: source SHA-256 remained
`47f99c7a0d4913a770bb2ed6b81957a4157f46ca7c30280d16b65f05efa68a7f`
and the worktree remained clean. Evidence is retained at
`/tmp/cqr-verifier-changed-nD6rHy`; its exact 632 MB disposable target was
validated and pruned.

Static: dual review required a test-oracle correction after this measurement.
These results remain historical evidence for `9970ac32`; one new measurement
is required for the corrected head.
