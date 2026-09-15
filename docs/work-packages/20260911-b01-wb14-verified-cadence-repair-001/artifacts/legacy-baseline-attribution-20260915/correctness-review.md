# Independent correctness review — legacy baseline attribution

**Reviewer:** `/root/recovery_correctness`, continuing the authorized affected correctness scope; independent of the parent executor, source author/adviser, and QA reviewer.

**Scope:** immutable producing-cut continuity, original-regression operation/input equivalence, the single baseline result, shared-failure attribution, and reached recorder-preparation disposition. Accepted input/native-pair reviews are reused without reopening them.

**Evidence class:** Static: inspected exact detached baseline/candidate callers, preliminary operation, native `production_only` posture, namespace identities, and continuity evidence. Ran evidence inspected: parent-executed baseline command and complete stdout/stderr. The reviewer ran no Rust build, test, model, recorder, solver, or quality command.

## Findings

### HIGH — the original regression is a shared baseline failure and remains unmet

`baseline-original-regression.json` records exit 100 for exactly one executed producing-baseline test. `baseline-original-regression.stderr` reports `Executor(AdaptiveRefinement("canonical covered evaluation budget"))` at the baseline preliminary `execute_direct_v11_segment` expectation in `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs:848`, followed by the same outer join failure at `v9_real_consumer_shadow_wb14_covered_physical_tests.rs:1467`. The retained candidate failed with the same typed error, operation text, guard, fixture inputs, and outer caller, at its shifted line 1042.

This supports a shared observed baseline/candidate failure attribution for the preliminary direct-segment operation. It does not make the regression pass, establish an acceptable numerical result, or identify which internal solver budget emitter fired. Neither run reached the original 540-second assertion. The reproduced failure consumes verification 33/33, so recorder controls and quality checks remain **NOT RUN** and recorder preparation remains **HOLD / NO APPROVAL**.

## Attribution and fix verification

- **Static, PASS:** `operation-relationship.json` shows the original caller file is byte-identical on baseline and candidate and supplies SWE 0.0006 m, cold delta 0 K, no hard boundary, one lane, terminal event enabled, and `production_only=false`. Both sources execute the same `execute_direct_v11_segment(...).expect("real mixed covered/open OFE execution")` under `if !production_only`.
- **Static, PASS:** The accepted native characterization uses `production_only=true`; its matching 7,707,305-byte payload therefore does not exercise or clear this preliminary operation. The package must keep the native-pair limited PASS separate from this failed regression.
- **Ran, PASS for attribution only:** The baseline command uses the immutable producing cut, its distinct target, explicit manifest, locked features, exact test selector, `--no-tests fail`, `--retries 0`, and `--no-capture`. Nextest reports one run, zero passed, one failed, and 1,468 skipped. No retry occurred.
- **Ran, PASS:** The command record binds the adopted 741-entry baseline identity `114bd2877dd4141ec5023dbd99e1d7c1c97e68dfd75b5fdf6a570350c8f463e5`, empty source patch SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`, and unchanged source during execution.
- **Static, PASS:** `continuity.json` correctly distinguishes the adopted 741-entry identity from historical 740-entry identity `01c34f56dd32e0b9502a74801e6c961012441dbbce6205f33934db79aa49284c`. The parent's initial equality assertion compared different namespaces. The final record identifies this as a bookkeeping error with no observed source drift and conservatively charges verification 32/33.
- **PASS:** No Rust, fixture, assertion, physical input, solver budget, tolerance, controller, dependency, configuration, arithmetic, clamp, unit conversion, production serialization, or typed error changed.

## Residual risk and missing tests

- The identical public error text has multiple internal emitters. Residuals, charged-map count, exact internal site, and whether any configured budget is insufficient remain **NOT OBSERVED** and must not be inferred.
- The original 540-second regression remains **FAIL** on both baseline and candidate before its timing assertion.
- The combined actual-producer recorder control and disabled/enabled/non-target/forced-error, wrong-phase, omission, independent-operand and custody checks are **NOT RUN** here.
- Matched owning-crate and separate-runner lint are **NOT RUN**; historical lint failures remain unchanged.
- The accepted manual-projection and dynamic-hydrology mutation-sensitivity limitations remain.
- The outer day 4 / interval 22 versus nested day 0 / interval 0 defect, authentic context acquisition, complete prefix authentication, native reader, conservation/restart/science qualification, and cadence repair remain unresolved.

## Verdict

**Shared-failure attribution PASS; recorder preparation HOLD / NO APPROVAL.** The immutable producing baseline reproduces the candidate's typed failure at the same preliminary operation under matching original inputs. The regression remains failed, the internal solver cause is not localized, and the 33/33 stop leaves recorder controls and quality evidence unexecuted.
