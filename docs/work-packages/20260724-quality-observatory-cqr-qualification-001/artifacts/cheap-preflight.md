# Cheap Preflight

Evidence class: Ran.

The corrected stable candidate passed:

- Python compilation for the collector, QA supervisor, and CQR intake tool;
- QA supervisor self-test;
- CQR evidence self-test;
- Rustfmt;
- 18/18 focused merged-coverage, QA workflow, and CQR handoff integration
  tests; and
- warnings-denied Clippy for the touched workflow contract.

The focused workflow contract also proves manual-only dispatch, exact forest1
labels, distinct concurrency, exact source/workflow/current-main binding,
successful exact-head TESTGATE admission, typed priority deferral, compact
publication, and the absence of TESTGATE execution-job coupling.

After attempt 1 exposed lossy failure diagnostics, the focused QA contract also
proved that a failed log larger than 256 KiB retains its full SHA-256 and a
digest-bound 32 KiB tail inside the bounded partial index.
