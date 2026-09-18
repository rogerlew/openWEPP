# Correctness review — publication observer boundary

Reviewer: `/root/observer_correctness` (independent correctness/authority
review). Scope: the adopted publication-observer continuation; starting tree
`d354ccc6ba947c6b0a9e3eed1cdedd0417ae1d5c2896288efbc828beca2f873b`;
frozen cut02 tree
`85b8314efcd53ccb8111aa97af1f752ed49a99f46b805e754db5a1b2c7059963`;
SC-COUPLEDTIME-001 INV-COUPLEDTIME-032 / OBL-COUPLEDTIME-015; the trusted
append call chain, observer meaning and failure isolation, affected feature
controls, and retained C1-C8 evidence.

**Static:** I read the adopted authorization, package/review instructions,
prior terminal correctness and QA reviews, the affected contract amendment,
the complete four-file patch, trusted append/install implementation and audit,
all real trusted-support callers, and the NativeMixedPhase positive, N2 refusal,
N12 rollback, and observer-error paths. **Ran:** the executor ran the commands;
I independently inspected their source-bound receipts and raw/result evidence.
I did not run a behavioral command or edit Rust.

## Findings

- **Resolved blocking — the initial proposed test-level observation site would
  have changed the inherited record's meaning and missed real callers.** An
  observation after `commit_selected_publication_and_take_staged_ending` would
  have described a later committed ending, omitted a successful local append
  followed by enclosing rollback, and failed to cover every real append path.
  Frozen cut02 instead calls the test-only observer from
  `DirectV10RealConsumerShadow::push_validated_accepted_publication_support`
  only after `AcceptedPublicationHistoryV1::push_validated_support` returns.
  The RAII trusted-append audit guard has dropped before this call. The ordinary
  path in `v9_real_consumer_shadow.rs`, covered-owner finalization in
  `v11_covered/owner_finalization.rs`, and snow-free reseal in
  `v9_real_consumer_shadow/snow_free_physical_reuse.rs` all use the wrapper;
  remaining direct inner calls are capability tests.

- **Resolved blocking — a projection encoding failure could have been counted
  and emitted as a successful observation.** `accuracy_observation_wire()`
  represents conversion failure as JSON `Null`, which an enclosing JSON value
  can still encode. The accepted implementation in
  `v9_real_consumer_shadow.rs` materializes that value first, classifies `Null`
  as one observation error, and returns without a record or success count.

- **Resolved blocking — `println!` could unwind an already successful model
  transition on output failure.** The frozen implementation uses fallible
  stdout write and flush, suppresses their `io::Error`, and increments the
  success count only after the combined operation succeeds. The one-shot error
  control executes the common result/error branch and proves zero record,
  success/error/inside counters `0/1/0`, and the same complete positive
  NativeMixedPhase model assertions as the normal path. The independently run
  parent checker refuses that error stream because the required replay record
  is absent; its emitted `B01_INTEGRATED_PARENT` result is byte-identical to the
  normal result.

- **No unresolved correctness finding.** The observer borrows the just-installed
  final `Arc` from the actual history. It does not clone the support or
  capability, reconstruct an oracle, rerun validation or physics, reseal an
  identity, scan the history, or serialize from inside the protected append.
  Failed inner append returns through `?` before observation. The moved record
  retains its schema, boundary text, actual support/receipt/owner/replay values,
  multiplicity, and local-append meaning. I found no duplicated algorithm or
  parallel adapter introduced by this delta.

## Correctness and evidence disposition

- The unchanged capability/source guard now passes in both required groups.
  No-feature ran 13/13 and both-feature ran 17/17; evidence-only ran 2/2 and
  persisted-only ran 5/5, all with exact selectors, fail-on-empty, no retry,
  source cut02, and zero observer records while its audit was disabled. The
  validation-once control reports exactly one validation/mint/append/join and
  zero append-time validation, operand reconstruction, receipt reconstruction,
  serialization, full-prefix scan, and support-payload clone. Its separate
  non-vacuous probe detects each forbidden category once, so the zero result is
  meaningful rather than an inert audit.

- Frozen binary
  `9cf75b7337097df3454e2b301e2734565a36a3bdedbb5039b5df50d37abf0a49`
  produced the four focused NativeMixedPhase results: positive `1/0/0`, N2
  refusal `0/0/0`, N12 late rollback `1/0/0`, and forced observation error
  `0/1/0` for success/error/inside-trusted counts. Each selected exactly one
  test and passed. N12 reaches a successful candidate-local append, then its
  injected final-owner-join error checks every captured outer root unchanged
  before raising the expected sentinel. Its one record therefore demonstrates
  the authorized local-append meaning without asserting global commit or
  installation.

- The positive record is byte-for-byte equal to the prior accepted integrated
  record (raw record SHA-256
  `ffd2498b8288ec9f793b8b468ba8925bcf498904dc2f72cb8188a79b1bd3dd8b`),
  remains at line 25, and retains the surrounding ordered integrated tags.
  The unchanged independent parent, credit, donor, and installation checkers
  all pass. The parent result authenticates six represented-prefix slabs, two
  successors, eight final segments, one terminal event group, two clock
  receipts, and exact retained WB14 replay correspondence while explicitly
  leaving checkpoint finalization, clock/global commit, installation, and
  conservation unauthenticated. All 28 parent controls and 40 other corruption
  controls reject their mutations.

- All eight affected build checks pass. The four inventories are 1476, 1481,
  1513, and 1520 tests: each is its accepted starting inventory plus only the
  new observer-error control, with no removal or existing metadata change. All
  14 promotion selectors remain present and the same 44 feature-gated absences
  remain. Changed-line formatting passes.

- The cut02 source custody reconstructs the patch on the verified starting
  recovery and matches all 747 frozen entries. The only Rust changes are in
  `snow_stage3_v11_adaptive_production_tests.rs`,
  `v9_real_consumer_shadow.rs`,
  `v9_real_consumer_shadow/accepted_publication_support_capability.rs`, and
  `v9_real_consumer_shadow/snow_free_physical_reuse.rs`. No manifest, feature,
  dependency, public/wire API, numerical expression, unit conversion, clamp,
  domain guard, solver, physical input, chronology predicate, or typed error
  taxonomy changed. The production-compiled snow-free call now delegates to a
  wrapper whose non-test behavior is the same inner call with identical error
  propagation; the observer and its counters/fault control are test-only.
  Consequently this observer relocation does not reopen the previously
  accepted C1-C8 scientific and feature repairs. Their evidence is reused only
  within its original scope.

- Targeted quality remains correctly classified. The no-deps command exits 101
  with the same 2,849 diagnostics as its baseline; the existing
  `clippy::struct_field_names` span now includes the three added audit counters
  but retains the same property, code, and message. Strict Clippy exits 101 with
  exactly the same 27 diagnostics and no added or removed diagnostic. These are
  attributable inherited failures, not a strict-lint pass.

## Residual risk and missing tests

- The forced I/O control injects an error before bytes are written and exercises
  the same result/error disposition used by real write or flush errors. It does
  not emulate every operating-system partial-write shape. This leaves a narrow
  diagnostic-stream transport risk, not a model-transition or science risk:
  write/flush errors cannot propagate into the append result, and a partial
  newline-delimited JSON record cannot satisfy the unchanged exact-one parser.

- The review accepts this test-only observer relocation and affected feature
  completion only. Strict Clippy, broader A-001/WB14-RQ1, the original-input
  E008 regression, full scientific/conservation/restart qualification,
  production Rust adoption, canonical promotion, and cadence release remain
  unresolved or unperformed exactly as recorded by the package. No broad or
  excluded campaign is inferred from these focused controls.

## Verdict

**APPROVE — no blocker for the bounded publication-observer continuation and
affected feature-control disposition on frozen cut02.** The source and runtime
evidence satisfy INV-COUPLEDTIME-032 / OBL-COUPLEDTIME-015 at the reviewed
boundary, preserve the inherited record's bytes and local-append semantics, and
isolate observation failure from the accepted model transition. This verdict
does not accept the separately open strict-lint, broader package, scientific,
production, or cadence claims listed above.
