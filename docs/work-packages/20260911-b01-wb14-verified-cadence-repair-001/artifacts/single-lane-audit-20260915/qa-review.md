# Secondary QA review — single-lane audit correction

**Reviewer:** `/root/recovery_qa`, independent secondary QA reviewer; prior input custody, discovery, and static oracle findings are reused where unchanged.

**Evidence class:** Static: inspected the authorization, detached source-relationship evidence, failed-cut preservation, identical narrow corrections, source receipts, and command records. Ran evidence inspected: both frozen-file format checks and the one corrected native invocation per side. I ran no build, test, model, reader, capture, or comparison command.

## Findings

No blocking QA defect was found in the authorized correction or the exercised pair.

**PASS — correction is narrow and source-backed.** `source-relationship.json` establishes that batch dispatch requires more than one active lane and that batch telemetry is appended only by the batch evaluator. Both detached corrections replace only the contradictory nonempty/singleton audit assertion with the required empty-audit assertion; the genuine microstep, event, prefix, parcel, owner, successor, and original 540-second predicates remain outside this delta. Frozen-file `rustfmt --check` passed for both revised sources.

**PASS — complete model payload equality.** The reference and candidate each executed exactly one selected case, passed exit 0, used `--retries 0`, and remained source-stable. Each stdout contains exactly one `B01_NATIVE_COMPARISON_PAYLOAD` and one diagnostic line. Independently extracted payload lines, including their terminal newline, are identical (`sha256 006428b67fc3b1ee3d1335f29717d3d4f32f2ee6e3d25298aba057572215d063`; byte comparison exit 0). The parent comparison's 7,707,305 JSON-value bytes exclude that newline and retain SHA-256 `fa01f2a111ddb331bee942f604dee1fe3af230a6d8b5f4e225234f98146e6845`; both identities describe the same payload representation. Ordered arrays and the empty `terminal_batches` vector are retained. The diagnostic payload bytes differ only as execution-timing evidence; diagnostics are explicitly outside model equality and do not alter the retained 360-versus-540 disposition.

### HIGH — original 540-second regression failed before its timing assertion; downstream controls and quality gates are blocked

**Paths:** `artifacts/single-lane-audit-20260915/original-540-regression.json`, `artifacts/single-lane-audit-20260915/original-540-regression.stderr`

The selected original regression executed exactly one case and failed, exit 100, before reaching its 540-second assertion. The observed failure is `Executor(AdaptiveRefinement("canonical covered evaluation budget"))` at `v9_real_consumer_shadow_wb14_tests.rs:1042`. It is a new downstream execution failure, conservatively exhausting 31/31. Recorder controls and matched lint are therefore not run, and neither a 360-versus-540 result nor a source-quality acceptance follows.

## Non-blocking debt and follow-ups

- The pair proves only this disabled-observer/recorder fixture differential. It does not establish the native tick as scientifically correct, enabled-recorder isolation, full current-context capture, native restoration, conservation, restart, or cadence acceptance.
- Recorder producer/positive and negative controls, independent-operand checks, matched owning/runner Clippy, and remaining required review/quality evidence are not established and remain blocked at the reached stop.
- The accepted mutation-sensitive independent-oracle limitation remains: owner-schema changes require maintained independent projections, and dynamic hydrology fields lack a universal independent expected-field projection.

## QA disposition

**PAIR PASS / RECORDER PREPARATION HOLD.** The corrected one-lane native reference/candidate comparison is fully equal for its retained model payload. The original regression then failed before timing evaluation, exhausting the correction ceiling. This accepts neither the historical 360-versus-540 failure nor any unrun downstream behavior or source-quality requirement.
