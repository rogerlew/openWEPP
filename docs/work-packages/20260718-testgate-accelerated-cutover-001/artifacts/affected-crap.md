# Affected CRAP Closure

Ran: final stable-source pass on 2026-07-18 PDT / 2026-07-19 UTC.

The final post-cleanup acquisition passed:

```text
CARGO_TARGET_DIR=target/affected-crap-build-v4 bash \
  tools/release/run_adjudicated_crap_gate.sh \
  --scope affected \
  --package openwepp-gate-planner \
  --nextest-profile affected \
  --base-ref 908e7fcb36145dc2048d7cb506ba6f2cc83ad96a \
  --output-dir target/testgate-cutover-affected-crap-v4
```

- Exit: 0; elapsed 572 seconds.
- Instrumented Nextest: 62/62 passed; zero failed or skipped.
- Production entries: 958.
- Raw / adjudicated / actionable rows: 0 / 0 / 0.
- Closure eligible: true; debt status: `PASS`.
- Touched production files: `execution_context.rs`, `executor.rs`, `lib.rs`,
  `main.rs`, `planner.rs`, `policy.rs`, and `verifier.rs`; touched actionable
  rows: 0.
- Before/after/final source manifest: 247 sources, identical SHA-256
  `828bf7e125b01e5d55eed9f0ae3557b3af7359bd20f86cff5a1bad814a6cfb74`.

Complete current evidence is retained under
`target/testgate-cutover-affected-crap-v4/`.

The immediately preceding acquisition at `v3` passed 62/62 tests but found one
actionable row: `planner.rs::inventory_for_node`, CRAP 33.5793. The function was
decomposed without adding tests, and the focused static re-reviews passed
before the successful `v4` acquisition.

## Superseded Attempts

The authorized closure runner's first execution passed, but later accepted
planner/executor patches invalidated that source-bound result:

```text
CARGO_TARGET_DIR=target/affected-crap-build bash \
  tools/release/run_adjudicated_crap_gate.sh \
  --scope affected \
  --package openwepp-gate-planner \
  --nextest-profile affected \
  --base-ref 86bce645ae53d5ef9b984666fdb20206f9a62e7e \
  --output-dir target/testgate-cutover-affected-crap
```

- Exit: 0; elapsed 449 seconds.
- Instrumented Nextest: 54/54 passed.
- Production entries: 926.
- Raw / adjudicated / actionable rows: 0 / 0 / 0.
- Closure eligible: true.
- Touched production files: `executor.rs` and `policy.rs`; touched actionable
  rows: 0.
- Before/after/final source manifest: 246 sources, identical SHA-256
  `650d4df06d1899da032f0ca5cf009a28bc160cb6a677c40bf83633f551eda66b`.

That superseded evidence is retained under
`target/testgate-cutover-affected-crap/`, including the adjudicated JSON and
Markdown reports, LCOV, source manifests, run status, checksums, logs, and
version receipts. The run created only ignored build/evidence output.

The first required post-patch acquisition then ran once at
`target/testgate-cutover-affected-crap-v2` and failed before CRAP acquisition:
51/56 tests passed and five planner/executor receipt-fixture tests failed. No
raw, adjudicated, or actionable CRAP count was produced. Focused fixes for all
five failures subsequently passed. These attempts are retained as truthful
history and do not supersede the final `v4` pass.
