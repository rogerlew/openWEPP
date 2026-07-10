# Worker Handoff

Evidence label: Static/Ran.

Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`

## Summary

Package 07 completed behavior-preserving CQR for
`crates/openwepp-input-contract/src/parsers/management.rs`.

Final target state:

- CRAP rows above `30`: `0`.
- Max CRAP: `28.136080592592595`.
- Line coverage: `89.81854838709677%`.
- Region coverage: `86.46770237121831%`.
- Production line count: `2960`.

## Important Evidence Paths

- Targeted LCOV:
  `/tmp/openwepp-cqr-nightly-07-management-targeted.lcov`
- Targeted coverage JSON:
  `/tmp/openwepp-cqr-nightly-07-management-targeted-llvmcov.json`
- Targeted CRAP JSON:
  `/tmp/openwepp-cqr-nightly-07-management-targeted-crap.json`
- Original delegated full coverage blocker log:
  `/home/workdir/openWEPP/artifacts/cqr-20260709-cqr-nightly-07-input-management-parser-001/02-cargo-llvm-cov.log`
- Post-review full gate logs:
  `/home/workdir/openWEPP/artifacts/cqr-20260709-cqr-nightly-07-input-management-parser-001-postreview/`

## Follow-Up Notes

- The management parser remains near the `3000` production-line blocker. Future
  work touching this module should split section clusters into submodules before
  adding substantial production code.
- Full-workspace coverage LCOV remains blocked outside this package by
  unrelated coverage-instrumented `laned_shadow_h2637` failures/long-runs; this
  package used the Phase D targeted coverage/CRAP equivalent.
- The closeout commit containing this artifact satisfies the package completion
  boundary before CQR Nightly target #8 starts.
