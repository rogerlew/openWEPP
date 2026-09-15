# Secondary QA review — legacy baseline attribution

**Reviewer:** `/root/recovery_qa`, independent secondary QA reviewer reusing accepted input custody, source/recovery, and native-pair reviews where unchanged.

**Evidence class:** Static: inspected the authorization, package protocol, continuity/ledger, and exact-operation evidence. Ran evidence inspected: the sole immutable-baseline original-regression invocation. I ran no build, test, model, reader, capture, or control command.

## Findings

### HIGH — the original regression is a confirmed shared baseline failure and remains unmet

**Paths:** `artifacts/legacy-baseline-attribution-20260915/baseline-original-regression.json`, `artifacts/legacy-baseline-attribution-20260915/baseline-original-regression.stderr`, `artifacts/legacy-baseline-attribution-20260915/operation-relationship.json`

The one permitted immutable-baseline invocation executed exactly one selected case with retries disabled and failed, exit 100. It reproduces the candidate's typed `Executor(AdaptiveRefinement("canonical covered evaluation budget"))` at the same preliminary `execute_direct_v11_segment` operation under `!production_only`, before either 540-second assertion. The baseline and candidate preserve identical original caller bytes and identical fixture inputs; the accepted native pair used `production_only: true` and does not cover this preliminary path.

This is valid shared-failure attribution, not an acceptable regression outcome or a solver diagnosis. The exact internal solver site, residuals, and map count are not observed. It consumes failure 33/33; recorder controls and source-quality commands must remain unrun, and recorder preparation remains **HOLD**.

## Non-blocking debt and follow-ups

- Custody is adequate for attribution. The immutable producing source binds to its retained 741-entry identity `114bd2877dd4141ec5023dbd99e1d7c1c97e68dfd75b5fdf6a570350c8f463e5`; the distinct historical 740-entry receipt was not substituted. The parent’s initial namespace mismatch is correctly preserved as failure 32/33, with no observed drift.
- The baseline command uses the producing cut's distinct target and explicit manifest, locked retained features, exact original selector, `--no-tests fail`, `--retries 0`, and `--no-capture`. It is bound to the unchanged recovered support/input custody record.
- The matching disabled-posture native pair remains limited differential evidence only. It does not establish enabled-recorder isolation, timing/scientific correctness, full context capture, native restoration, conservation, restart, or cadence acceptance.

## QA disposition

**ATTRIBUTION PASS / RECORDER PREPARATION HOLD.** The legacy baseline reproduces the same observed preliminary typed failure under the matched original operation and inputs. The regression remains failed; the reached 33/33 stop blocks all controls and quality work.
