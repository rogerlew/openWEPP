# QA Attempt 7

Evidence class: Ran.

- Provider run:
  [`30194820456`](https://github.com/rogerlew/openWEPP/actions/runs/30194820456)
- Source/workflow head:
  `7268c13c7bdd87bf067392ee1f24920f4fb2fc1c`
- Qualification TESTGATE run: `30192814397`
- Result: `EXECUTION_FAILED`
- Child exit: `2`
- Occupancy: `CLEAR`

The QA-scoped `CARGO_BUILD_JOBS=2` cap was live through compiler descendants,
but the CQR self-test's independent inventory inherited the outer
`cargo-llvm-cov` wrapper environment. Recursive `rustc -vV` wrapper probes
exhausted the PID cgroup and failed with `os error 11`; the PID-limit event
counter incremented from 7 to 8.

`full` ran 2,295 tests: 2,289 passed, one failed, and five timed out. The global
build-job cap pushed five nested gate-planner fixtures to their unchanged
720-second limit, including the three former linker-failure tests. No `SIGBUS`
or OOM occurred, but timeout is not acceptance.

`science-manual`, JUnit, LCOV, snowbench reconstruction, CRAP, debt
disposition, complete publication, and quality evidence ID were not reached.

The global build-job cap is rejected as overbroad and removed on the next
changed head. The precise correction strips inherited outer LLVM coverage
wrapper variables before the independent inventory creates its own fresh
instrumentation environment. Disk-backed scratch, the short attempt root, and
exclusive CQR scheduling remain.

Read-only evidence is retained at
`/home/workdir/openWEPP-quality-history/20260726-quality-observatory-run-30194820456`.
