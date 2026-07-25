# Attempt 12 Repository-Snapshot Isolation

Evidence class: Ran / Static.

Attempt:
`/home/workdir/openWEPP-quality-history/20260724-order3-local-attempt12-dwDN07`.

Executed head:
`8f5a89faef573321d7d1a91219c57b9c963f93d8`.

## Result

Admission was `READY`. The instrumented full profile ran all 2,279 tests:
2,278 passed, one failed, 15 were slow, and 31 were skipped in `2840.520s`.

The sole failure was the gate-planner coverage-only public-audit consumer
`exact_planner_output_reconstructs_through_the_public_audit_path`. Its first
operation observed the shared execution checkout and received
`GATE-COMMITTED-CHECKOUT-NOT-EXACT`.

Science-manual, merge, CRAP, snowbench disposition, publication, and terminal
verification did not run. Published files: 0. No evidence ID was issued.

The final read-only observatory comparison found all 291 executable rows and
the working-tree identity equal to admission. Source and execution snapshot
were clean at the executed head.

## Isolation Diagnosis And Correction Intent

JUnit timing shows the failing test began at the exact millisecond the
preceding gate-planner fixture process ended. The exactness helper discards the
dirty status bytes, so no transient path is retained and the predecessor's
temporary-repository cleanup is not asserted as proven cause.

The public-audit test nevertheless violates isolation by using the shared
instrumented execution checkout as its committed source. It is explicitly a
content-independence test and must not depend on transient activity elsewhere
in the workspace suite.

Before implementation, this prerequisite package reopens its existing
gate-planner write set. The test will create a no-hardlink local clone at the
same HEAD, bind the repository-local Python environment through an exactly
excluded symlink, prove the clone clean, and exercise the unchanged public
audit reconstruction against that clone. Production exact-checkout guards
remain unchanged and fail closed.
