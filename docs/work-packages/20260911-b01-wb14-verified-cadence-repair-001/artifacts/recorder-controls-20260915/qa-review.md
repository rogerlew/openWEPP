# Secondary QA review — recorder-controls continuation

**Reviewer:** `/root/recovery_qa`, independent secondary QA reviewer reusing accepted custody, baseline attribution, native-pair, and static oracle evidence where unchanged.

**Evidence class:** Static: inspected the package protocol, continuity, ledger, source receipt, and reused evidence bindings. Ran evidence inspected: the sole combined recorder-control invocation. I ran no build, test, model, reader, capture, or lint command.

## Findings

### HIGH — combined recorder-control test failed; control-matrix acceptance and quality gates are blocked

**Paths:** `artifacts/recorder-controls-20260915/recorder-controls.json`, `artifacts/recorder-controls-20260915/recorder-controls.stderr`

The only authorized combined control invocation selected and executed exactly one case with retries disabled, then failed: exit 100, `0 passed; 1 failed; 1471 filtered out`. It panicked at `v9_real_consumer_shadow_wb14_tests.rs:2259` in synchronized covered-parent cadence with `AdaptiveRefinement("canonical covered dependent-output instability")`; the outer join at `snow_stage3_v11_adaptive_production_tests.rs:146` propagated the failure. The shared helper is used across modes, and the raw log does not label the active submode. Therefore the combined recorder-control matrix, phase/omission guards, independent operands, and full model/custody isolation acceptance are not established.

This is new behavioral failure evidence, not recorder-control acceptance. The selected frozen formatting and owning/runner lint comparisons must remain unrun; no diagnostic-comparison quality conclusion is available. Recorder preparation remains **HOLD**, independently of the already-failed original regression.

## Non-blocking debt and follow-ups

- Command custody is adequate for the observed failure: it binds the unchanged candidate tree `3f2e7f08…`, accepted patch `d24b0b28…`, recovered input custody digest, explicit manifest/distinct target, locked retained features, exact selector, `--no-tests fail`, `--retries 0`, and `--no-capture`.
- The failure differs from the prior shared original-regression budget failure. It is a covered dependent-output instability in a different combined-control fixture. Neither error establishes the precise internal solver site, residuals, map count, or a permissible solver adjustment.
- Accepted disabled-posture native-pair equality remains limited differential evidence. It does not cover this failed combined recorder-control test or establish enabled-recorder isolation.

## QA disposition

**HOLD / NO APPROVAL.** The sole recorder-control invocation failed and does not establish the combined control matrix; it blocks all remaining selected quality evidence. The original regression remains failed; no recorder, timing, scientific, source-quality, native-restoration, or cadence requirement is accepted.
