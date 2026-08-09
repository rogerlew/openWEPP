# Rust QA Review

Status: `complete`

Review target: exact commit
`7820953c1b5258564200bd167e0c4994a69b3065`, against the declared
pre-implementation base `a65cc3973ddd04b07cad108fcb33d83a8c161abb`.

## Findings

### HIGH — near-complete frost retention still erases positive timed runoff

Paths:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1505`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs:261`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:874`

The proportional frost allocation is gone and material partial retention now
fails closed. However, the replacement clears the complete hourly series when
`hourly_partition_runoff_m <= tolerance_m`, not only when no positive runoff
remains. A valid counterexample is `before_m = 0.01`,
`frost_retained_local_liquid_m = 0.00999999`, and
`partition_runoff_m = before_m - frost`: the positive residual is about
`1.0e-8 m`, below the `2.4e-8 m` aggregate tolerance, so lines 1511-1514 erase
the produced hourly depth and return zero.

That contradicts `INV-WATBAL-102`'s positive-source preservation and
`TOL-WATBAL-009`'s statement that the tolerance never authorizes loss of
positive hourly runoff and permits full-series clearing only when reconciled
runoff is zero. The tests cover exact full retention and material partial
retention but omit this positive-within-tolerance boundary. Clear only an exact
zero residual; any positive partial residual without hourly frost custody must
take the typed missing-upstream path. Add just-below/at/above-tolerance positive
residual vectors.

### HIGH — local-only daily infiltration still controls hourly timing by position

Paths:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:234`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:521`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1912`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs:313`

The new source-custody guard correctly rejects a daily local-only same-pass
infiltration correction after local and runon supplies have merged. It still
accepts the same daily correction on a local-only melt ledger, then
`remove_depth_from_hour_bins_earliest` subtracts it from the earliest positive
bins. The daily reconstruction at lines 521-570 has no producer-hour operand;
choosing earliest bins can change the peak hour and magnitude for multi-hour
melt by assumption.

The focused test calls only the guard and explicitly blesses the local-only
case. The melt tests exercise WB14's genuine hourly infiltration, not the
nonzero daily reconstruction branch, so no fixture proves this second debit's
timing. Contract text permitting a local-only daily correction does not provide
the missing process clock. Carry producer-timed infiltration custody through
this branch or fail closed whenever a material daily correction would modify a
positive hourly series.

### HIGH — exact-head terminal evidence is absent or anchored to the prior commit

Paths:

- `docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/gate-results.md:1`
- `docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/mutation-study.md:1`
- `docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/summary.md:1`
- `docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/rust_code_review.md:1`

At exact commit `7820953c`, `gate-results.md` remains `queued` / `not-run`,
the mutation study says the complete cohort is still running, and both the
mutation study and summary identify `949349e70` as their binary/source anchor.
The committed primary Rust and two science reviews also review `949349e70`,
before the source, contract, HBP calendar, census receipt, and fixture changes
in this target.

The committed full-census log cannot replace those stale identities, and an
untracked in-progress `topanga-openwepp-census-full-v3.log` was excluded from
this exact-commit review. Critical closure requires exact-head full-census and
terminal gate receipts plus refreshed required reviews. The focused evidence
below is useful QA, not a substitute for the package's declared quick/full,
doctest, deny, and exact-head cohort requirements.

### MEDIUM — line-count inventory is complete, but production split rationale is not

Path:
`docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/line-count-governance.md:1`

The corrected artifact now lists all seven touched Rust files at or above the
2,000-line warning threshold, with exact counts, and none reaches 3,000 lines.
Its follow-up describes test extraction from the two large test files, but it
does not give the required decomposition rationale and concrete split intent
for the five production files: `00_core_frames.rs`, `laned_active.rs`,
`runoff.rs`, `00_builders_and_authority.rs`, and
`00c_day_input_builder_impl.rs`. Add a concise per-file or explicitly grouped
mapping from current cohesion boundary to intended extraction seam.

## Resolved prior findings

- Census sidecar provenance now matches all 12 names in the runner's legacy
  sidecar contract. Record schema v3 binds case, plan, binary, complete input
  hashes, expected row count, calendar digest, and exact year/Julian arrays.
  Empty, truncated, wrong-calendar, corrupt, binary-changed, primary-input,
  sidecar, nonfinite, negative, and heterogeneous-plan-value cases are covered.
- The prior proportional daily frost debit is removed. Exact full retention
  clears the series, while ordinary material partial retention takes a typed
  missing-upstream error. The remaining positive-within-tolerance defect is
  narrower but still closure-blocking.
- HBP EVENT output now carries the selected producer row's calendar year.
  Both p61 and p102 derive the corresponding simulation year and join Parquet
  by year plus Julian day before independently reconstructing `sum(V_h)`,
  `max(V_h) / 3600`, public `peakro`, and p61 duration.
- Warnings-denied Clippy remains green. The publication zero-basis mismatch is
  symmetric, and WB16 errors now use the typed hydrology guard family and share
  the production peak arithmetic.
- The line-count artifact no longer omits `03_tests.rs` or
  `watershed_cli_behavior_contract.rs`.

## Exact evidence run by this reviewer

- Ran in an isolated sparse archive of the exact target:
  `cargo fmt --all -- --check` — PASS.
- Ran in the exact archive:
  `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner -p openwepp-hillslope-output --tests -- -D warnings`
  — PASS.
- Ran in the exact archive: warnings-denied Clippy for the three changed root
  integration binaries — PASS. A broader root-test Clippy attempt was not
  counted because the deliberately sparse archive omitted unrelated
  `include_str!` work-package fixtures.
- Ran in the exact archive: p61, p102, and the four
  `peak_hourly_authority_contract` tests — PASS (`6/6`).
- Ran in the exact archive: seven focused frost reconciliation, mixed-source
  custody, missing-WB14-producer, and public peak-boundary tests — PASS (`7/7`).
- Ran in the exact archive: output schema metadata/readback tests — PASS
  (`2/2`).
- Ran in the exact archive:
  `.venv/bin/python -B .../tools/test_topanga_openwepp_census.py` — PASS
  (`6/6`).
- Ran against the exact base-to-target diff: `git diff --check` — PASS. The
  declared terminal diff touches 83 files.
- Not run by this reviewer: full-workspace quick/full, doctest, cargo-deny, or
  the 1,088-trial cohort. Concurrent tracked edits and the untracked v3 cohort
  log were excluded from all exact-target conclusions.

## Non-blocking debt and follow-ups

- `test_valid_record_reuses` exercises `record_matches`, not the actual
  `run_case(..., resume=True, ...)` control flow. Add a subprocess-spy test that
  proves valid receipts bypass execution and every mismatch executes anew.
- `DISCOVERED_SIDECARS` duplicates the runner contract in Python. Add a
  source-level parity test or hash a canonical resolved-input manifest so a new
  runner sidecar cannot silently escape future receipts.
- The p61 test intentionally doubles one storm in a copied climate, but the
  fixture README still describes an unmodified real-climate erosion fixture.
  Document the controlled mutation and avoid calling its resulting values an
  untouched real-input result.
- The p61/p102 joins use the full year/Julian key but still select `max_by`
  rather than asserting exactly one matching public row. An explicit uniqueness
  assertion would prevent duplicate-row defects from being masked.
- `map_wb16_peak_guard` fabricates `NaN`, `-1.0`, or `1.0` observed values when
  adapting errors, and the tests assert only codes. Preserve actual operands or
  use variants that do not claim an observed value; assert boundary class,
  symbol, and diagnostic text.
- On census batch failure, canceling futures does not terminate already
  running subprocesses or emit a terminal partial-run receipt.

## QA Verdict

`HOLD — NOT ACCEPTABLE FOR CRITICAL CLOSURE.` Exact focused formatting,
warnings-denied Clippy, consumer, schema, receipt, calendar, and boundary tests
are green, and most prior QA findings are materially resolved. Commit
`7820953c` still erases a positive frost residual within tolerance, retains an
unsupported daily-to-hourly local infiltration debit, and lacks exact-head
terminal evidence. Re-review is required after those blockers and the exact
closure artifacts are corrected.
