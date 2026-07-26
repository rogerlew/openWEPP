# QA Attempt 4

Evidence class: Ran.

- Provider run:
  [`30183194732`](https://github.com/rogerlew/openWEPP/actions/runs/30183194732)
- Source/workflow head:
  `31911e922418aa66b149106484aab25ae5a81ddc`
- Qualification TESTGATE run: `30181516854`
- Result: `EXECUTION_FAILED`
- Child exit: `2`
- Occupancy: `CLEAR`
- Full log SHA-256:
  `f227deaa042f2e45ad57c8fbc18d53a46ada0278c608278e3e0e60304f86e0e8`
- Independently verified retained-tail SHA-256:
  `f5731c8596c21ef36f12b605fd8666f03d346a9acfde59fdc5e980d8451644d7`

Exact source, workflow, qualification, forest labels, and empty TESTGATE
occupancy passed. The full profile admitted 2,292 identities,
`science-manual` admitted 36, and the canonical workspace admitted 2,328.

The attempt failed during `full` before JUnit or LCOV finalization.
`science-manual` did not start. Three repository-snapshot fixture tests failed
while nested `openwepp-runner` links terminated with `lld` signal 7
(`Bus error`); Cargo exited 101. This recurs from attempt 2 and is an
infrastructure/resource-schedule failure, not a test assertion or science
verdict.

No quality evidence ID, merged coverage, snowbench reconstruction, CRAP
measurement, debt disposition, or complete publication exists. The failed
control publication was exactly the two-file allowlist, 44,504 uncompressed
bytes under the 1 MiB ceiling, and independently verified.

Canonical policy blocks another expensive attempt until correction.
Read-only diagnosis proved these repository-snapshot tests were already
globally exclusive: their group admits one test and each reserves all 32
configured Nextest slots. A global one-thread schedule would therefore be
redundant and is not an accepted correction. The nested Cargo/link resource
fault is instead corrected by binding `TMPDIR` to a per-attempt disk-backed
directory under the quality attempt's `local` root. This moves snapshot and
nested Cargo targets off the 24 GiB `/tmp` tmpfs while preserving the exact
inventory and existing mutually exclusive test schedule.

Read-only evidence is retained at
`/home/workdir/openWEPP-quality-history/20260726-order7-qa-run-30183194732`.
