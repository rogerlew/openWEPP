# kernel profile compliance checklist

Status: checked through M-H closure

Evidence mode: Ran + Static

## M-H checklist

- Production edits: none in M-H.
- Science-contract edits: none in M-H.
- Test edits: none in M-H.
- Typed errors in production: none introduced.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged in M-H.
- Runtime publication paths: full H1-H36 per-OFE WAT publication acceptance
  passes on row cardinality, anti-alias, anti-clone, and conservation closure.
- Watershed output path: not accepted; `openwepp-cli-watershed` fails closed on
  the substrate no-impoundment `pw0.imp` state before writing
  `totalwatsed3.parquet`.

Validation:

- Full M-H H1-H36 ladder acceptance: PASS.
- Single-OFE anchor comparison: PASS.
- Local full-ladder comparison execution without comparator subagent: PASS
  execution and row-key alignment; semantic values remain investigation fail.
- Final post-documentation gates: PASS; see `gate-results.md`.

## M-G checklist

- Production edits: yes; manifest/report provenance now exposes
  `erod14_qin_source_policy` and `erod14_qin_sediment_coupled`.
- Science-contract edits: yes; `SC-RUNOFFPART-001` version 44,
  `SC-WATBAL-001` version 160, `SC-SED-001` version 41, and `SC-SYSTEM-001`
  version 83 pin the water-transfer-only versus sediment-coupled `qin`
  boundary.
- Test edits: yes; M-G contract authority and CLI03 manifest policy assertions
  were added.
- Typed errors in production: no new error family; this is provenance
  publication and contract gating.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: no process-physics math changed; true erosion `qin/qout` and
  particle-fraction handoff remains follow-on.
- Runtime publication paths: operator-visible provenance is present in
  manifests; current active Wave-2 runs report water-transfer-only policy and
  `erod14_qin_sediment_coupled = false`.

Validation:

- `cargo fmt --check`: PASS.
- Focused M-G contract tests: PASS.
- Focused M-G CLI manifest tests: PASS.
- Full final gates: see `gate-results.md`.

## M-F-REDO2 checklist

- Production edits: yes; per-OFE WB13 publication now receives raw routed
  runoff plus explicit OFE-local/cumulative publication geometry and publishes
  public `QOFE`/`Q` with distinct baseline-authoritative normalizations.
- Science-contract edits: yes; `SC-WATBAL-001` version 159 and
  `SC-SYSTEM-001` version 82 add public runoff-normalization authority and
  downstream alias rejection.
- Test edits: yes; focused M-F publication test now asserts `QOFE/Q` geometry
  ratios and contract-source guards cover the new authority.
- Typed errors in production: new geometry validation failures map through
  typed `HillslopeCliError::RuntimeSurfaceFailure` paths.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: no process-physics math changed; this increment changes
  publication normalization only.
- Runtime publication paths: active handoff, anti-clone, internal identity, and
  public `QOFE/Q` geometry gates pass. Semantic value-family comparisons still
  fail on broader routed hydrology/storage/ET values and remain investigation
  signals, not M-F-REDO2 publication-normalization blockers.

Validation:

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`: PASS.
- Work-package and touched SC-WATBAL/SC-SYSTEM docs lint: PASS; 38 files
  scanned, 0 errors, 0 warnings.
- `git diff --check`: PASS.
- Required H1/H6/H9/H11 runtime smoke: PASS execution, row cardinality, active
  handoff, anti-clone, and `QOFE/Q` geometry gates.
- Single-OFE anchor comparison: PASS, 28/28 byte-identical to M-F-REDO-CLONE
  single outputs.

## M-F-REDO checklist

- Production edits: yes; per-OFE static lane runtime surfaces, active surface
  carry writeback, active lateral carry accumulation, same-pass runon storage
  reconciliation, and runtime OFE-count resolution.
- Science-contract edits: yes; `SC-WATBAL-001` version 157 and
  `SC-SYSTEM-001` version 80 add active-handoff and anti-clone publication
  invariants.
- Test edits: yes; focused M-F publication test now asserts active handoff and
  anti-clone behavior, contract-source guards cover the new invariants, and
  WB12-family fixtures reflect same-pass runon storage.
- Typed errors in production: new runtime failures continue to map through
  typed `HillslopeCliError::RuntimeSurfaceFailure` and scheduler sequence
  error paths.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: changed only within the authorized MOFE01 WB12/WB14/WB19 carry
  publication path; no provisional process-physics math was added.
- Runtime publication paths: active handoff and anti-clone gates pass; package
  remains held because public `QOFE` still aliases public `Q` instead of using
  baseline `efflen/slplen` geometry scaling.

Validation:

- `cargo fmt --check`: PASS.
- `git diff --check`: PASS.
- Work-package and touched SC-WATBAL/SC-SYSTEM docs lint: PASS; 38 files
  validated, 0 errors, 0 warnings.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`: PASS.
- Required H1/H6/H9/H11 runtime smoke: PASS execution, row cardinality, active
  handoff, and anti-clone gates.
- `QOFE != Q` geometry acceptance: FAIL; M-F-REDO2 required.

## M-F checklist

- Production edits: yes; public per-OFE WB13/WAT publication from internal
  per-OFE records, per-OFE `QOFE` source override, publication provenance, and
  watershed manifest validation.
- Science-contract edits: none.
- Test edits: yes; focused M-F public row-shape tests and source guard.
- Typed errors in production: new publication/cardinality/key failures map to
  typed `HillslopeCliError::RuntimeSurfaceFailure` paths; watershed manifest
  failures remain typed string diagnostics for the CLI validator.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime publication paths: multi-OFE public WAT row shape changed; acceptance
  held because the current surface carry producer still emits zero `UpStrmQ`.

Validation:

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- `git diff --check`: PASS.
- Required H1/H6/H9/H11 runtime smoke: PASS execution and row cardinality.
- Surface `UpStrmQ` acceptance: FAIL; M-F-REDO required.

## M-E4-REDO checklist

- Production edits: yes; non-tautological internal per-OFE WB13 identity
  validation and pre-day storage snapshot wiring.
- Science-contract edits: yes; `SC-WATBAL-001` version 156 pins
  `TOL-WATBAL-007` and M-E4-REDO acceptance rules.
- Test edits: yes; focused M-E4-REDO runner tests and contract-source guard.
- Typed errors in production: internal WB13 identity failures map to typed
  `HillslopeCliError::RuntimeSurfaceFailure` with the `per_ofe_internal_wb13`
  surface.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime publication paths: public aggregate WB13/WAT publication preserved;
  M-E4-REDO records are internal evidence and M-F owns the public flip.

Validation:

- `cargo fmt --check`: PASS.
- `cargo test -p openwepp-runner mofe01_me4_redo -- --nocapture`: PASS.
- `cargo test -p openwepp-runner mofe01 -- --nocapture`: PASS.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`: PASS.
- Work-package and SC-WATBAL docs lint: PASS; 36 files validated, 0 errors, 0
  warnings.
- Required H1/H6/H9/H11 runtime smoke: PASS.
- Internal WB13 identity manifest audit: PASS.
- Single-OFE anchor comparison: PASS.
- Local owcmp H1/H6/H9/H11 command execution: PASS; expected semantic FAIL
  remains at the unchanged aggregate-publication boundary.

## M-E3 checklist

- Production edits: yes; persistent OFE lane state model, multi-OFE runner
  daily lifecycle wiring, and manifest dynamic-state policy tokens.
- Science-contract edits: none.
- Test edits: yes; focused M-E3 orchestrator writeback tests.
- Typed errors in production: extended `OfeLaneSequenceError` for persistent
  lane count/order replacement mismatches; runner maps persistent lifecycle
  failures to typed CLI runtime-surface failures.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime publication paths: aggregate WB13/WAT publication preserved; dynamic
  per-OFE state is shadow persistence only.

Validation:

- `cargo fmt --check`: PASS.
- `cargo test -p openwepp-hillslope-orchestrator mofe01_me3 -- --nocapture`:
  PASS.
- `cargo test -p openwepp-runner mofe01 -- --nocapture`: PASS.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS.
- Work-package docs lint: PASS.
- Required H1/H6/H9/H11 runtime smoke: PASS.
- Single-OFE anchor comparison: PASS.
- Local owcmp H1/H6/H9/H11 command execution: PASS; expected semantic FAIL
  remains at the unchanged aggregate-publication boundary.

## M-E2 checklist

- Production edits: yes; orchestrator scheduler sequential OFE lane executor
  and public exports.
- Science-contract edits: none.
- Test edits: yes; focused M-E2 orchestrator writeback tests.
- Typed errors in production: added `OfeLaneSequenceError` for invalid lane
  count/order, invalid transfer values, daily sum mismatch, scheduler failure,
  and lane execution failure.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime publication paths: unchanged; aggregate WB13/WAT publication
  preserved.

Validation:

- `cargo fmt --check`: PASS.
- `cargo test -p openwepp-hillslope-orchestrator mofe01_me2 -- --nocapture`:
  PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS.
- Final H1-H36 replay/comparison: PASS runtime execution; expected `owcmp`
  semantic FAIL remains at the unchanged aggregate-publication boundary.

## M-E1 checklist

- Production edits: yes; scheduler data-model types, runner static slices, and
  publication manifest provenance.
- Science-contract edits: none.
- Test edits: yes; focused M-E1 runner tests and stale WATBAL version-pin
  repairs in two HPHYS authority tests.
- Typed errors in production: added `PerOfeDailyWaterBalanceError` variants for
  invalid record and transfer source/recipient mismatches.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime publication paths: aggregate WB13/WAT publication preserved; no
  dynamic per-OFE WAT publication flip.

Validation:

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS.

## M-E0 checklist

- Production edits: none.
- Science-contract edits: yes; `SC-RUNOFFPART-001`, `SC-WATBAL-001`, and
  `SC-SYSTEM-001` amended for per-OFE dynamic-state authority.
- Test edits: yes; `mofe01_per_ofe_state_contract` added and M-B authority
  smoke test date brittleness fixed.
- Typed errors in production: unchanged.
- `unwrap`/`expect` in production: none introduced. The new panic helper is
  test-only.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime state/publication paths: unchanged; M-E0 only installs the contract
  and red-test boundary for M-E1.

Validation:

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo deny check`: PASS.
- M-E0 authority test: PASS.
- M-B authority smoke test: PASS.
- Full M-E0 target: FAIL by design on missing per-OFE state collection,
  transfer payloads, and publication-policy manifest gate.
- Full Rust closure loop: BLOCKED until M-E1 satisfies the intentional red
  test.

## M-D checklist

- Production edits: none.
- Typed errors: unchanged.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime state/publication paths: unchanged in code; M-D only declares the
  architecture needed before M-E production edits.

Validation:
- Full Rust closure loop was not rerun for M-D because no production Rust,
  science-contract, dependency, or test files were edited.
- Package docs lint is recorded in `gate-results.md`.

## M-C2 checklist

- Production edits: none.
- Typed errors: unchanged.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime state/publication paths: unchanged because current architecture has
  no real per-OFE daily WB state surface to retain or publish.

Validation:
- Full Rust closure loop was not rerun for M-C2 because no production Rust,
  science-contract, dependency, or test files were edited.
- Focused existing M-B carry tests passed.
- M-C2 output comparison and publication audit were run separately and are
  recorded in `m-c2-per-ofe-daily-state-scope-evidence.md`.

## M-C checklist

- Production edits: none.
- Typed errors: unchanged.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime publication paths: unchanged because current aggregate-only WB13/WAT
  surface cannot support real per-OFE publication semantics.

Validation:
- Full Rust closure loop was not rerun for M-C because no production Rust,
  contract, test, or dependency files were edited.
- M-C output comparison and publication audit were run separately and are
  recorded in `m-c-wat-publication-closure-evidence.md`.

## M-B checklist

- Typed errors: preserved. Missing active frost topology and invalid aggregate-vs-array carry remain typed failures.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: top-layer saturation excess is relocated to the explicit current saturation carry under M-B contract authority; no silent default or mask was added.
- Kernel math: limited to contract-pinned inter-OFE routing/carry plumbing and saturation excess conservation.
- Runtime carry paths: updated to purge stale aggregate carry before MOFE
  hourly-array execution and preserve separated `UpStrmQ`/`SubRIn` lineage in
  runtime state. WAT publication remains aggregate-only and is held in M-C.

Validation:
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.

## M-A

M-A made no production kernel/runtime edits.
