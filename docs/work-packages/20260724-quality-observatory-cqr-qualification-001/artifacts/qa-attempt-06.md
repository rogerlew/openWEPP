# QA Attempt 6

Evidence class: Ran.

- Provider run:
  [`30190531969`](https://github.com/rogerlew/openWEPP/actions/runs/30190531969)
- Source/workflow head:
  `a8a94a6d67013310d9a44db58ba012d718ed0a07`
- Qualification TESTGATE run: `30188752174`
- Result: `EXECUTION_FAILED`
- Child exit: `2`
- Occupancy: `CLEAR`
- Full log SHA-256:
  `0a82aad9556536a35771f3f39aef2d12c281e9ca147f90316cb43e9e6067c752`
- Independently verified retained-tail SHA-256:
  `a43e9a526b1e6c378bf1792f40b610ba6040868081a950add32e122f7060c2fc`

The shortened disk-backed temporary root resolved the Unix-socket path
failure. All three former linker-`SIGBUS` tests passed again. `full` completed
with exactly one failure: the globally exclusive CQR self-test's own nested
Cargo inventory could not spawn repeated `cargo-llvm-cov rustc -vV` probes.
The cgroup PID-limit event counter incremented once; no OOM or linker bus error
occurred.

`science-manual`, JUnit, LCOV, snowbench reconstruction, CRAP, debt
disposition, complete publication, and quality evidence ID were not reached.

The next changed head preserves exclusive scheduling and binds
`CARGO_BUILD_JOBS=2` into the QA instrumented build identity and both
admission/collection environments. This caps nested Cargo process fan-out
without changing inventory selection, profile order, Nextest test threads,
timeouts, or retry policy.

Read-only evidence is retained at
`/home/workdir/openWEPP-quality-history/20260726-order7-qa-run-30190531969`.
