# Independent correctness review — single-lane audit correction

**Reviewer:** `/root/recovery_correctness`, continuing the authorized affected correctness scope; independent of the parent executor, source author/adviser, and QA reviewer.

**Scope:** identical test-only single-lane batch-audit correction on the detached reference/candidate, corrected native pair, full payload comparison, and reached original regression. Accepted authority/input/discovery and unchanged recorder-source reviews are reused.

**Evidence class:** Static: inspected both detached dispatcher/audit relationships, correction patches, preserved positive predicates, and failure sites. Ran evidence inspected: parent-executed format checks, corrected native pair, extracted payloads/comparison, and original regression. The reviewer ran no Rust build, test, model, recorder, or lint command.

## Findings

### HIGH — the original 540-second regression fails before reaching its timing assertion

`original-540-regression.json` records exit 100 for exactly one executed candidate test. `original-540-regression.stderr` shows the inner thread failing at `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs:1042` while unwrapping genuine execution: `Executor(AdaptiveRefinement("canonical covered evaluation budget"))`; the outer join propagates the panic at `v9_real_consumer_shadow_wb14_covered_physical_tests.rs:1467`. The failure occurs before the original 540-second assertion.

This is the first downstream failure and correction 31/31. It leaves the original regression failed for a different earlier reason and supplies no new timing-oracle result. Recorder controls and quality checks correctly remain **NOT RUN**. Recorder preparation is **HOLD / NO APPROVAL**.

## Fix verification and limited acceptance

- **Static, PASS:** Both detached sources use the same branch contract: only `active_lanes.len() > 1 && !terminal_lanes.is_empty()` selects `try_actual_terminal_batch_subslab_v2`; the one-lane case selects `try_actual_terminal_single_subslab_with_evidence`. `TerminalBatchProductionAuditV2` entries are appended in the batch evaluator. An empty batch-audit vector is therefore the correct postcondition for this explicitly one-lane fixture.
- **Static, PASS:** Both correction patches make the same semantic change despite shifted line numbers: the contradictory nonempty/singleton batch assertions become an explicit `production.2.is_empty()` assertion. The vector remains in the comparison payload. Existing fixture-input, native-resident, accepted-microstep, event/tick, consumed-parcel/digest, contiguous-prefix, snow-free successor, owner and parent-endpoint predicates remain unchanged. Both original 540-second assertions/callers remain unchanged.
- **Ran, PASS:** Reference and candidate format checks exit 0 on frozen corrected files. Corrected source identities are reference tree `3aeeb449f10a725e252276c9523f74bed603228dd80b691b4120e4449b8979d0` / patch `c828508beec774f5b826dc89843d3c7f4805441f5d1974fd09333ca8da02edc4`, and candidate tree `3f2e7f08ed121836b9854539ce9f3f1fc2dd6eae073fd14baaf83d4a71bb8aa5` / patch `d24b0b28bb90e263501bef1c0c935cf8859b068d35efe450a530e7000bc61c66`; command records report unchanged source.
- **Ran, PASS:** Each corrected native characterization runs exactly one test successfully with zero retries. Each emits exactly one complete comparison payload and one separate diagnostic payload.
- **Ran, PASS:** `comparison.json` reports byte-for-byte equality of the entire 7,707,305-byte comparison payload, SHA-256 `fa01f2a111ddb331bee942f604dee1fe3af230a6d8b5f4e225234f98146e6845` on both sides. Independent read-only comparison of the extracted newline-terminated payload files also found exact equality. The payload retains 11 top-level surfaces, including the empty `terminal_batches`, 22-field fixture projection, 89 ordered carrier supports, complete owner comparison, adaptive receipt, comparison audits, controller, observer/recorder posture, and performance/publication topology.
- **Ran, correctly limited:** Both diagnostic payloads retain expected legacy tick 540,000,000,000 ns, observed native tick 360,000,000,000 ns, and the explicit non-oracle disposition. Elapsed values differ as expected and remain outside model equality. The equal pair establishes only the disabled-posture exercised differential; it does not establish scientific correctness of 360 seconds or enabled-recorder isolation.
- **PASS:** The correction changes no production arithmetic, clamps, units, domain guards, serialization, typed errors, physical inputs, forcing, controller, support boundaries, dependencies, or tolerances.

## Residual risk and missing tests

- The original 540-second regression remains **FAIL** at the canonical covered evaluation budget before its timing assertion.
- Covered/first-snow-free recorder production, independent operands, disabled/enabled/non-target/forced-error, wrong-phase, omission and selector controls are **NOT RUN** on the corrected cut.
- Matched owning-crate and separate-runner lint are **NOT RUN**. Historical lint failures remain unchanged.
- The accepted manual-projection maintenance risk and dynamic-hydrology mutation-sensitivity limitation remain.
- The outer day 4 / interval 22 versus nested day 0 / interval 0 defect, authentic context acquisition, complete prefix authentication, native reader, conservation/restart/science qualification, and cadence repair remain unresolved.

## Verdict

**Native pair limited PASS; recorder preparation HOLD / NO APPROVAL.** The single-lane correction is contract-consistent and the complete disabled-posture payloads match exactly. The immediately following original regression fails before the 540-second assertion, triggers 31/31 and the first-downstream-failure stop, and leaves all recorder controls and quality acceptance unexecuted.
