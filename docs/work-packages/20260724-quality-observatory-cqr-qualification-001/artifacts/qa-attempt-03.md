# QA Attempt 3

Evidence class: Ran.

- Provider run:
  [`30180877189`](https://github.com/rogerlew/openWEPP/actions/runs/30180877189)
- Source/workflow head:
  `c17f49d9bda46f2f6ea4d64fc9db5e41dbd4093b`
- Qualification TESTGATE run: `30177394609`
- Provider job conclusion: `success`
- Forest control disposition: `DEFERRED_OCCUPANCY_UNKNOWN`
- Child exit: `-15`
- Full partial-log SHA-256 recorded by the control index:
  `3f2ad0230e4cbcbba32ae64406d9d7287b07ddc47a51a75c449d0946f38dab3c`
- Independently verified retained-tail SHA-256:
  `f6e2993d0e08eda579566f2ad6277247003c0f4754e2c06b493323050093b24b`

This was the sole unchanged infrastructure retry. It did not qualify the
observatory: the supervisor's five-second aggregate deadline expired while
querying the GitHub TESTGATE workflow for `waiting` runs. The fail-closed
supervisor terminated collection and published no quality evidence.

The provider's green job records successful typed deferral handling, not
quality acceptance. No evidence ID, inventory, JUnit, snowbench, merged LCOV,
CRAP, publication allowlist, or publication-size claim exists for this
attempt.

The recurrence cause differs from attempt 2 but the unchanged retry allowance
is exhausted. The aggregate occupancy snapshot deadline is therefore corrected
on a changed head while retaining fail-closed classification and bounded
termination.

Read-only evidence is retained at
`/home/workdir/openWEPP-quality-history/20260726-order7-qa-infra-retry-run-30180877189`.
