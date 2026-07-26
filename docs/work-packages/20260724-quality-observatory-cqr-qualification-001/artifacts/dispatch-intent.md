# Dispatch Intent

Evidence class: Static.

- Qualified Order-6 subject:
  `955358449381ab38378d28dac93ba7b21b496d14`
- Order-6 TESTGATE run:
  [`30165527516`](https://github.com/rogerlew/openWEPP/actions/runs/30165527516)
- QA workflow: `.github/workflows/quality-observatory.yml`
- QA trigger: manual `workflow_dispatch` only
- QA concurrency: `openwepp-forest1-quality-observatory`
- TESTGATE concurrency: distinct `openwepp-forest1-testgate`

The preflight defect is corrected prospectively by requiring a successful
exact-repository/path/head TESTGATE run ID in addition to current-main source
admission. The correction requires a new committed and pushed head plus exact
successful TESTGATE qualification before one QA attempt.

QA attempt 1 bound head
`2f16072bd86ed2ae858625a31c39769d7ad8b3a5` and TESTGATE run `30173294509`
to provider run
[`30175384859`](https://github.com/rogerlew/openWEPP/actions/runs/30175384859).
It failed in `full` with child exit 2. Its oversized log was not retained, so
the typed cause is an evidence-lifecycle/diagnostic-retention defect. The
unchanged head is not eligible for rerun.

Attempt 2 will use a changed head that retains full diagnostic digests and
bounded tails. It requires a new exact successful TESTGATE qualification
before QA dispatch.

QA attempt 2 bound corrected head
`c17f49d9bda46f2f6ea4d64fc9db5e41dbd4093b` and TESTGATE run `30177394609`
to provider run
[`30179148269`](https://github.com/rogerlew/openWEPP/actions/runs/30179148269).
It failed in `full` when the linker terminated with signal 7 while nested
fixtures built `openwepp-runner`. Corrected diagnostics retained the exact
full-log digest and bounded failure tail.

One unchanged infrastructure-only retry is authorized by canonical policy. No
second unchanged retry is allowed.

QA attempt 3 was that sole unchanged retry. Provider run
[`30180877189`](https://github.com/rogerlew/openWEPP/actions/runs/30180877189)
ended in typed `DEFERRED_OCCUPANCY_UNKNOWN` after the five-second aggregate
GitHub occupancy snapshot deadline expired. The supervisor terminated
collection fail-closed and published no evidence. No further unchanged retry
is authorized. The in-scope deadline defect requires a changed head, a fresh
exact-head TESTGATE qualification, and one new QA attempt.

QA attempt 4 bound changed head
`31911e922418aa66b149106484aab25ae5a81ddc` and TESTGATE run `30181516854`
to provider run
[`30183194732`](https://github.com/rogerlew/openWEPP/actions/runs/30183194732).
It failed during `full` when three nested fixture links terminated with the
same signal-7 infrastructure cause observed in attempt 2. Recurrence blocks
another expensive attempt until the QA resource schedule is corrected and
qualified on a changed head. Diagnosis proved the failing tests were already
globally exclusive, so the correction instead moves their nested scratch and
Cargo targets from `/tmp` tmpfs to the disk-backed attempt-local root.

No CQR collection or module implementation is authorized. CQR work in this
package is selection-only intake of the exact successful QA publication.
