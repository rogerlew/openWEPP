# Independent correctness review — native pair execution

**Reviewer:** `/root/recovery_correctness`, authorized affected-scope correctness reviewer; independent of the parent executor, source writer, adviser, and QA reviewer.

**Scope:** the actual reference native-characterization invocation, its retained output and source/input binding, payload completeness, pair disposition, and reached downstream acceptance. The accepted source/input/discovery review in `artifacts/reference-input-recovery-20260915/correctness-review.md` and unchanged static recorder review are reused without a repeat source audit.

**Evidence class:** Static: inspected the adopted native-pair protocol and the precise asserted value's tuple type and call order in the frozen reference harness. Ran evidence inspected: parent-executed `reference-native.json`, complete stdout, and complete stderr. The reviewer ran no Rust build, test, model, comparator, recorder, or quality command.

## Findings

### HIGH — the reference fixture fails its required terminal-batch invariant before emitting either payload

`reference-native.json` records exit 100 for exactly one executed test. `reference-native.stderr` shows the named characterization thread panicking at `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_production_tests.rs:216` on `assert!(!production.2.is_empty())`, followed by the outer join panic at line 431. Static inspection identifies `production.2` as `Vec<TerminalBatchProductionAuditV2>`, filled by `take_terminal_batch_production_audit()` after the genuine native fixture execution. It was empty despite the fixture contract requiring at least one terminal batch with lane `[1]`. `comparison-disposition.json` preserves this operand meaning and the complete run counts.

The failure occurs inside the frozen reference harness, before both `B01_NATIVE_COMPARISON_PAYLOAD` and `B01_NATIVE_DIAGNOSTIC` print statements. Neither marker occurs in stdout or stderr. The run therefore supplies no complete canonical comparison payload and no observed native-tick diagnostic. This is a newly executed fixture/producer acceptance failure, not a candidate recorder mismatch and not evidence that any terminal tick is scientifically correct.

Under the adopted first-test-failure stop, the candidate side correctly remains **NOT RUN**. The reference/candidate pair is incomplete and cannot establish numerical, owner, chronology, event/parcel, publication/work, or recorder-disabled parity. Recorder preparation remains **HOLD / NO APPROVAL**.

## Fix verification and bounded disposition

- **Ran, PASS:** The command used the frozen reference source, distinct retained target, exact package/library/test filter, locked features, `--no-tests fail`, `--retries 0`, and `--no-capture`. Nextest reports exactly one run, zero passed, one failed, and 1,469 skipped. No retry occurred.
- **Ran, PASS:** `reference-native.json` binds source tree `996e87f5a4aad77fc906691dc1d0ce466eb769767af87c32bd83b849cc593016`, patch `e8a700579d6dfad9bac8db2f716b5004104b3215fa2321f52aa71e950f0492b6`, and restored-input custody `2e9e27eeb0a355672e7a74c8fd645a2ce95182ae88bf78741bc8f98ecac9b082`; it records source unchanged during execution.
- **Ran, limited evidence:** Reaching line 216 means the preceding in-function assertions completed, including fixture-input posture, native V3/V4 residents, named beginning owners, transition/event/parcel checks, parent support, and nonempty accepted microsteps. This does not recover their serialized values and cannot substitute for the missing comparison payload.
- **PASS:** Stopping after the reference failure preserves the once-per-side allowance and avoids comparing a candidate result against a failed or absent reference. Conservatively recording this as failure 30/31 is consistent with the ledger, while the first-test-failure stop independently prohibits another invocation or harness correction.
- **PASS:** No Rust, fixture, assertion, dependency, arithmetic, clamp, unit, domain guard, production serialization, or typed error change occurred. The original 540-second assertions and historical 360-versus-540 failure remain unchanged.
- **PASS:** `comparison-disposition.json`, `stop.json`, and the package native-pair disposition accurately report reference FAIL 0/1, zero payload markers, candidate/full comparison NOT RUN, failure 30/31, and the independent first-test-failure stop. They make no recorder mismatch, native-tick, or scientific acceptance claim.

## Residual risk and missing tests

- Candidate native characterization is **NOT RUN**; reference/candidate payload equality and every specified canonical field comparison are absent.
- The empty terminal-batch audit has not been diagnosed as a harness expectation error, audit-capture defect, or genuine fixture path omission. No correction is authorized in this scope.
- The original 540-second regression; covered/first-snow-free producer; independent operands; disabled/enabled/non-target/forced-error, wrong-phase, omission and selector controls; formatting; and matched owning-crate/separate-runner lint are **NOT RUN** here.
- The accepted manual-projection maintenance risk and dynamic-hydrology mutation-sensitivity limitation remain.
- The outer day 4 / interval 22 versus nested day 0 / interval 0 defect, authentic context acquisition, complete prefix authentication, native reader, conservation/restart/science qualification, and cadence repair remain unresolved.

## Verdict

**HOLD / NO APPROVAL.** The reference invocation was exact and well bound, but the native fixture failed before producing comparison or diagnostic payloads. No candidate run or downstream recorder/control/quality evidence exists, so the exercised recorder differential remains unverified.
