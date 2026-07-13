# Scenario Matrix

Status: `HOLD-INTEGRATED-VALIDATION`

Frozen source: `f80a115148e75a08269eb14a8c1b0e7791ca891a`.

| Lane | Command group | Fixture/output and real consumer | Required evidence | Result | Status |
| --- | --- | --- | --- | --- | --- |
| Intake | anti-evasion script + AUTH11 nextest | authority registry/required-suite guards | no evasion or binding failure | anti-evasion exit 0; AUTH11 2/2 in 1.37 s | PASS |
| H2637 | four named `laned_shadow_h2637` selections | H2637 WAT/HBP through production hillslope runner | active-owner publication and three routing-authority failures; numeric groundwater reconstruction incomplete | corrected ignored positive 1/1 in 7:21.69; three negative selections 1/1 each | PASS |
| Erosion | p61, p102, erosion profile | production direct erosion and downstream OFE/HBP | nonzero signal and class/total continuity | p61 1/1; p102 1/1; profile 367/367 in 2:28.08 | PASS |
| Snow/frost | frost profile | production runner/direct winter state | SWE/liquid/frozen carry and failures | profile 320/320 in 9:17.45 | PASS |
| Watershed | W7R, MT3, totalwatsed3, hourly tests | watershed CLI and dependency-ordered channels | jobs identity, same-grid consumer, closure, and command 13 external-baseflow-once assertion | W7R 1/1; MT3 7/7; totalwatsed3 17/17; hourly 30/30 | PASS |
| Fail-closed | runner + watershed packages | public HBP/WAT/manifest/channel inputs | typed rejection and no partial publication | runner 213/213; watershed 129/129 | PASS |
| Release | release candidate script, default lanes | release binaries, lint, authority, stability | all required lanes PASS | exit 101 after 8:50.55; three H2637 shared-environment tests failed in broad parallel execution | FAIL |
| Closure | fmt, Clippy, full nextest, deny, docs, diff | frozen source plus package evidence | all final gates PASS | stopped after the real nonzero release gate | BLOCKED |

## Executed Commands And Logs

Evidence class: **Ran** at frozen production source
`f80a115148e75a08269eb14a8c1b0e7791ca891a`.

Every executed command has a stable `.log` and `/usr/bin/time -v` `.time`
record under `artifacts/logs/`; the complete command-level ledger is in
`gate-results.md`. The positive H2637 test is source-marked `#[ignore]`, so its
plan selection was corrected to:

```text
cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only -E 'test(=h2637_native_active_owner_routes_and_closes)'
```

It selected and passed exactly one test; no zero-test result or libtest timeout
fallback was used. All other commands retained the literal package command.

Execution stopped at the default, no-skip release candidate command after its
real exit 101. Phase 6 closure commands are `BLOCKED`, not silently deferred or
reported as passing.
