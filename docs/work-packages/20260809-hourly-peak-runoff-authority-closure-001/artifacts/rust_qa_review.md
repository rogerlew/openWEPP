# Rust QA Review

Status: `complete`

Review target: `c7dbfefe7c7c67137101ddd2c63cd4c4c2e062fa`, against the
declared pre-implementation base
`a65cc3973ddd04b07cad108fcb33d83a8c161abb`.

Evidence:

- Static: reviewed the committed production, runner, schema, integration-test,
  contract, and mutation-census changes. Concurrent uncommitted workspace edits
  were excluded from this review.
- Ran, exact isolated target snapshot: `cargo fmt --all -- --check` passed.
- Ran, exact isolated target snapshot:
  `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner -p openwepp-hillslope-output --tests -- -D warnings`
  failed on `clippy::too_many_lines`.
- Ran, exact isolated target snapshot: five focused hourly-peak/source tests
  passed under Nextest.
- Ran, exact isolated target snapshot: all four tests in
  `peak_hourly_authority_contract` passed under Nextest.
- Not run by this reviewer: full-workspace quick/full/doctest gates and the
  1,088-trial Topanga cohort; those remain separate package closure evidence.

## Findings

### HIGH — warnings-denied Clippy is red

Path: `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs:866`

The affected-crate Clippy command fails because
`cqr_publication_helpers_cover_guards_and_optional_authority_branches` is 113
lines against the configured 100-line limit. This is a required gate failure,
not optional cleanup. Split the mixed storage, erosion, peak-publication, and
groundwater checks into cohesive tests; adding a lint suppression would retain
the readability problem and is not the preferred disposition.

### HIGH — the public hourly-peak claim lacks independent real-consumer reconstruction

Paths:

- `tests/integration/peak_hourly_authority_contract.rs:31`
- `tests/integration/erosion_single_ofe_p61_sediment.rs:150`
- `tests/integration/erosion_multi_ofe_p102_chain.rs:55`
- `docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/tools/topanga_openwepp_census.py:140`

The package's new dedicated integration test checks source-text markers only.
It can pass when the named implementation is dead, bypassed, duplicated, or
arithmetically wrong. The real p61 and p102 consumer tests prove that the 24 HBP
volumes sum to public `runvol`, but neither reconstructs
`max(hourly_runoff_volume_m3) / 3600` and compares it with both the HBP EVENT
peak and the same event's Parquet `peakro`.

The census does not close this gap. It reads only final `runvol` and `peakro`,
defines the maximum-hour fraction from those two values, and then checks the
identity `peak_ratio = volume_ratio * shape_ratio`. That decomposition is true
by construction and does not independently observe an hourly source bin.

Add a real downstream test that parses the emitted HBP, independently computes
the maximum hourly mean from its 24 volumes, and compares it with HBP
`peak_runoff_m3_s`, Parquet `peakro`, event-volume closure, and
rectangular-equivalent duration on the same keyed event. Its fixture must
distinguish concentrated from spread runoff and reject daily-return retiming,
uniform fallback, omitted area conversion, and duplicate area conversion.
Without that evidence, acceptance criterion 4 and the package's
anti-tautology/conservation requirements are not closed.

### HIGH — census resume and “complete cohort” claims are not provenance-bound

Path:
`docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/tools/topanga_openwepp_census.py:107`

`--resume` trusts only that a record path exists and returns `reused` without
opening or validating the NPZ. Records contain output arrays but no case ID,
trial/input identity, plan hash, binary hash, tool/schema version, or source-file
hashes. The terminal summary then stamps the current plan and binary hashes,
which can misattribute stale output from a different executable or input set.
Calendar equality alone also permits a wrong-case record with the same dates to
be paired silently.

In addition, `complete_frozen_cohort` means only “selected every eligible row in
the supplied JSON.” An empty, truncated, duplicated, or substituted plan can
therefore claim completion; the runner does not enforce the canonical plan
identity, exactly 1,088 unique eligible trials, or the expected
scenario/hillslope/family/direction inventory.

Persist and validate an atomic per-case receipt binding the case and input-file
hashes to the plan, binary, tool/schema, and expected output shape. Recompute or
fail on any mismatch or corrupt record. Gate the completion flag on the
canonical plan hash and exact unique cohort inventory, not merely the supplied
list length. Add tests for valid reuse, corrupt records, and invalidation after
binary, plan, case, and source-input changes.

### MEDIUM — census failures and interruptions do not retain reproducible diagnostics

Path:
`docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/tools/topanga_openwepp_census.py:131`

Stdout and stderr are retained only for a nonzero process return or missing
Parquet. Parquet read/schema/cast errors and finite/nonnegative validation
failures lose the captured process diagnostics. NPZ output is written directly
to its final path rather than by atomic temporary-file replacement, while
resume accepts any final-path file. On the first future error, cancellation
does not terminate already-running subprocesses, and no terminal partial/error
summary records the cases that completed or remained active.

Always retain command, return code, stdout, and stderr before parsing; write an
atomic success or error receipt; replace records atomically; and produce a
terminal partial summary with controlled subprocess shutdown. Exercise
process, missing-file, corrupt-Parquet, numerical, interrupted-write, and
multi-worker failure paths.

### MEDIUM — public schema semantics and publication-boundary guards are under-tested

Paths:

- `crates/openwepp-hillslope-output/src/hillslope_pass.rs:420`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs:582`

The changed `peakro` description is not asserted: the writer test inspects only
the `runvol` unit. Assert the actual Arrow/Parquet `peakro` field name, type,
nullability, `m^3/s` unit, maximum-hour description, and written value readback.
The peak-publication helper test covers dry/wet optionality, two positive areas,
and nonfinite public runoff depth, but does not cover negative/nonfinite area or
invalid shadow `q_runoff_m`, depth-rate, and duration operands. Production
currently relies on upstream construction and later consumer validation for
part of that boundary. Add typed-error tests, including contradictory
positive-public-runoff/zero-shadow-runoff input, so malformed public values
fail at the publication seam.

## Non-blocking debt and follow-ups

- Internal kernel coverage is otherwise strong: melt-only, saturation-only,
  runon-only, missing-source, source-backed and source-free tiny runoff,
  nonfinite/negative limbs, exact zero, concentrated-versus-spread shape,
  earliest-hour tie behavior, and weight nonclosure are directly exercised.
- Two touched Rust files exceed the 2,000-line warning threshold:
  `crates/openwepp-runner/src/hillslope/03_tests.rs` is 2,892 lines (unchanged
  from base), and
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs` is 2,589
  lines (2,541 at base). No touched file reaches the 3,000-line blocking
  threshold, but closure still requires an explicit boundary rationale and
  follow-on split intent. Peak/source-shape logic and its tests provide a
  natural decomposition seam.
- The 436-line census script combines execution, persistence, provenance,
  pairing, statistics, and reporting. Separating record/provenance handling
  from analysis would make the required failure-mode tests substantially
  easier to maintain.

## QA Verdict

`HOLD — NOT ACCEPTABLE FOR CRITICAL CLOSURE.` Focused behavior is promising and
the internal numerical edge cases are well covered, but the exact reviewed
commit fails warnings-denied Clippy and lacks independent public-path peak
reconstruction plus trustworthy resumable census evidence. Re-review is
required after the HIGH findings are resolved and the package's pending full
gates are recorded.
