# Independent correctness review — recorder controls

**Reviewer:** `/root/recovery_correctness`, continuing the authorized affected correctness scope; independent of the parent executor, source author/adviser, and QA reviewer.

**Scope:** the single combined actual-producer recorder-control invocation, reached selector/mode evidence, model/custody comparison coverage, and resulting quality/recorder-preparation disposition. Accepted source/input, native-pair, and baseline-attribution reviews are reused.

**Evidence class:** Static: inspected the frozen combined test's ordered calls and control assertions around the failure. Ran evidence inspected: parent-executed command and complete stdout/stderr. The reviewer ran no Rust build, test, model, recorder, format, or lint command.

## Findings

### HIGH — the combined control fails before any complete mode comparison is established

`recorder-controls.json` records exit 100 for exactly one executed candidate test. `recorder-controls.stderr` shows the inner thread failing at `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs:2259` while requiring the synchronized covered parent result:

`AdaptiveTrial { phase: "covered direct", start_ns: 0, end_ns: 60000000000, duration_ns: 60000000000, source: V11(Executor(AdaptiveRefinement("canonical covered dependent-output instability"))) }`

The outer recorder-fixture join propagates the panic at `snow_stage3_v11_adaptive_production_tests.rs:146`. That helper and failure site are reused across modes, and the command emits no submode diagnostic. Static order shows the combined test first requests a native snow-free non-target run, then covered disabled/enabled/forced-error runs, followed by snow-free controls; the retained output does not establish which ordered invocation reached the generic failure. The error is distinct from the previously shared `canonical covered evaluation budget` failure.

The test therefore supplies no completed covered model-equality comparison and no completed snow-free disabled/enabled/error/wrong-phase/omission comparison. This is a new behavioral fixture/model failure, not evidence of a recorder-induced difference. It triggers the first-behavioral-failure stop and conservatively consumes failure 34/35. Recorder preparation remains **HOLD / NO APPROVAL**.

## Limited evidence and disposition verification

- **Ran, PASS:** The command uses the accepted candidate tree `3f2e7f08ed121836b9854539ce9f3f1fc2dd6eae073fd14baaf83d4a71bb8aa5`, patch `d24b0b28bb90e263501bef1c0c935cf8859b068d35efe450a530e7000bc61c66`, restored-input custody `2e9e27eeb0a355672e7a74c8fd645a2ce95182ae88bf78741bc8f98ecac9b082`, exact test selector, locked features, `--no-tests fail`, `--retries 0`, and `--no-capture`. The source remained unchanged. Nextest reports one run, zero passed, one failed, and 1,471 skipped; no retry occurred.
- **Not established:** Because the generic helper emits no submode marker before failing, the run does not establish completion of the initial non-target assertions or identify a returned covered-disabled outcome. It cannot validate non-target selection, enabled or forced-error behavior, wrong-phase, omission, independent operands, row content, owner/clock/parcel custody, publication, or full canonical fixture comparisons.
- **PASS:** The parent stopped without retry, source correction, assertion change, model substitution, or downstream quality command. The previously accepted disabled native-pair equality and shared legacy-regression attribution remain separate results; neither proves these unexecuted controls.
- **PASS:** No production arithmetic, clamp, unit conversion, domain guard, solver budget, tolerance, serialization, typed error, fixture, dependency, or configuration changed.

## Residual risk and missing tests

- The cause of `canonical covered dependent-output instability` is not localized; no internal residual, dependent-output value, comparison operand, or exact emitter is retained here.
- The invocation supplies no complete, attributable covered disabled/enabled/forced-error or model-equality result; the active failing submode is not identified.
- The invocation supplies no complete, attributable snow-free disabled/enabled/forced-error/wrong-phase/omission or full model/custody comparison result; partial progress within those ordered calls is not observable.
- Candidate format and matched owning-crate/separate-runner lint are **NOT RUN** in this continuation; historical lint failures remain unchanged.
- The original 540-second regression remains **FAIL** on baseline and candidate before its timing assertion.
- The accepted manual-projection and dynamic-hydrology mutation-sensitivity limitations remain, along with the outer day 4 / interval 22 versus nested day 0 / interval 0 defect, authentic context acquisition, prefix authentication, native reader, conservation/restart/science qualification, and cadence repair.

## Verdict

**Combined recorder controls FAIL; recorder preparation HOLD / NO APPROVAL.** A generic recorder-fixture execution fails with a distinct adaptive dependent-output instability before any covered or snow-free mode-equality evidence can be completed. The output does not identify the active submode. Quality checks are correctly unrun after the stop.
