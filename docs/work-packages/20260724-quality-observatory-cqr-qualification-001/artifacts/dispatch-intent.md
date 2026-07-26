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

QA attempt 5 bound changed head
`46ee538e95b87e919e0f4c59e2bc90b4e48d1761` and TESTGATE run `30185142429`
to provider run
[`30186661261`](https://github.com/rogerlew/openWEPP/actions/runs/30186661261).
The three recurring linker failures passed. Two newly exposed environment
failures require a changed head: shorten the disk-backed attempt root for Unix
socket path safety, and make the nested CQR self-test globally exclusive to
avoid PID-limit competition.

QA attempt 6 bound changed head
`a8a94a6d67013310d9a44db58ba012d718ed0a07` and TESTGATE run `30188752174`
to provider run
[`30190531969`](https://github.com/rogerlew/openWEPP/actions/runs/30190531969).
The socket and linker failures remained corrected, but the exclusive CQR
self-test's nested Cargo inventory alone incremented the PID-limit counter and
failed to spawn. The next changed head adds the source-bound nested Cargo
build-job cap identified by the prior resource diagnosis.

QA attempt 7 bound changed head
`7268c13c7bdd87bf067392ee1f24920f4fb2fc1c` and TESTGATE run `30192814397`
to provider run
[`30194820456`](https://github.com/rogerlew/openWEPP/actions/runs/30194820456).
The live global Cargo cap did not prevent recursive coverage-wrapper PID
exhaustion and caused five nested fixture timeouts. It is rejected. The next
changed head instead sanitizes inherited LLVM coverage wrappers only at the
independent inventory boundary.

QA attempt 8 bound changed head
`1168bae1dfb21dd1a3be840d4381877aa54d9795` and TESTGATE run `30198502723`
to successful provider run
[`30200514260`](https://github.com/rogerlew/openWEPP/actions/runs/30200514260).
CQR selection-only intake rejected the resulting evidence because the external
CRAP report JSON was not canonical. The next changed head canonicalizes that
publication before identity binding and adds independent rejection.

No CQR collection or module implementation is authorized. CQR work in this
package is selection-only intake of the exact successful QA publication.
