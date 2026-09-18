# Feature-completion final correctness review

**Reviewer:** replacement primary Rust correctness reviewer (`/root/feature_correctness`, Sol/high), same reviewer that raised and rechecked C1-C8.

**Reviewed source:** terminal feature tree `d354ccc6ba947c6b0a9e3eed1cdedd0417ae1d5c2896288efbc828beca2f873b`, against frozen integrated tree `e93175d626081d52219c5053ff7899521f7ce3010a720335bec8b462095ee1e7`.

**Evidence class:** **Static:** complete terminal diff, public/private availability, typed-error behavior, representation schema, capability traits, physical-path reuse, and source custody. **Ran (supplied primary receipts inspected):** exact-source build/list/format/quality commands and focused controls. I did not execute commands myself.

## Findings, ordered by severity

### High — required non-wire capability guard still fails; approval is withheld

`v9_real_consumer_shadow::accepted_publication_support_capability_tests::accepted_publication_support_capability_is_private_move_only_non_wire` fails in both terminal executions:

- no-feature `completion-terminal-none-controls-03`: 12 passed, 1 failed;
- both-feature `completion-terminal-both-controls-03`: 16 passed, 1 failed.

The failure is exact: the guard finds `serde_json` in the trusted installer's inherited, opt-in `cfg(test)` integrated-promotion audit branch. The capability implementation and guard test are byte-identical to the integrated base:

- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/accepted_publication_support_capability.rs`: `a61d1275059edec01031425641ade2d1323a023d9e2ec9797c43553a18a775e1`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/accepted_publication_support_capability_tests.rs`: `24e5e610be3eab8e4a9da7653dd0d78c90595ff924dd630ece611f51e5ac3642`.

`completion-frozen-baseline-capability-control` independently executes the unchanged frozen integrated binary `3ee09e2d800d145494dd38280e26a89fb95447284120112041bfef3e00b65169` and fails the same exact test for the same reason. This is an inherited capability/diagnostic mismatch, not a regression introduced by the terminal diff. It still blocks approval because it is a required control. Relaxing the guard, hiding serialization behind another helper, or deleting the retained diagnostic would weaken or evade the control and is outside this authorization.

### Medium, accepted with explicit justification — private encoder duplicates the plain-state schema

`crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_constructor_encoder.rs` is a substantial handwritten encoder that mirrors the complete constructor schema and its inverse. That duplication can drift when a state field, enum tag, or custom numeric representation changes.

The duplication is justified for this bounded diagnostic path because it avoids adding ordinary `Serialize` implementations to runtime/capability-bearing state and keeps the production projection private. The terminal controls make the retained duplication acceptable:

- the independent raw oracle remains direct test-only Serde rather than a second call to the encoder;
- a balanced static comparison covers all 60 `captured_struct!` maps with zero missing, extra, or reordered fields;
- parity covers every growth/decomposition action and active-context variant, both erosion authority variants, the snow-albedo literal, and a populated signed `Some(TemperatureCelsius)`;
- `constructor_encoder_matches_independent_serde_and_literal_enum_names_v1` passes in no-feature, evidence-only, persisted-only, and both-feature executions;
- actual covered and snow-free recorder controls produce real rows and compare them to the independent live/raw projection.

This remains a maintenance risk: future schema edits must update the encoder, inverse, and independent parity together.

## C1-C8 and final-fix verification

- **C1 / C7, real ordinary recording — resolved.** The constructor pin is retained in ordinary builds when the physical observer is enabled and under ordinary `cfg(test)` for the recorder control. No-feature and both-feature executions pass the actual covered and first-snow-free recorder controls, including disabled, enabled real-row, forced capture error, wrong-phase, and required-omission cases while preserving the model result.
- **C2 / C6, complete representation — resolved.** The private encoder emits the complete plain-state representation. It preserves snake-case enum names, tags, fields, null/option behavior, and custom temperature numeric representation. Independent Serde parity and the 60-structure field comparison close the previously missing cases.
- **C3, public checkpoint availability — resolved.** In `snow_stage3_v11_restart.rs`, both public checkpoint methods remain gated only by `persisted-restart-v1`. Evidence-only support uses crate-private diagnostic methods, so no new public API is exposed.
- **C4, private helper availability — resolved.** The private checkpoint helpers use `any(persisted-restart-v1, restart-authority-evidence)`, matching every internal caller.
- **C5, standalone library — resolved.** Ordinary projection uses the private encoder instead of requiring feature/test Serde for nested runtime state. The standalone library check passes.
- **C8, attributable quality defects — resolved.** `DirectRunConstructorInputs` is test-gated; the parity assertions are split into four small helpers without assertion loss; and the private checkpoint methods are live in persisted-only mode.
- **Typed wrapper parity — resolved.** The crate-private checkpoint methods return `Box<DirectSnowStage3V11AttachmentError>` only to bound the private diagnostic error size. The persisted public wrappers retain their original names, gates, and concrete `Result<_, DirectSnowStage3V11AttachmentError>` signatures and unbox with `.map_err(|error| *error)`. The underlying `project` and `validate_against` bodies, error variants, and guard precedence are unchanged. Allocation occurs only on failure.
- **Capability surface — preserved.** No new serialization, copy, clone, or public trait/API surface is added to trusted capabilities. The persisted-only and both-feature deferred-capability non-wire controls pass.

## Validation and source evidence

All terminal build commands below report exit 0, unchanged source, no timeout, and no automatic retry at `d354ccc6...`:

- standalone library: `completion-terminal-standalone-lib-03`;
- no-feature tests: `completion-terminal-no-default-tests-03`;
- evidence-only tests: `completion-terminal-evidence-tests-03`;
- persisted-only tests: `completion-terminal-persisted-tests-03`;
- evidence-only, persisted-only, and both-feature libraries: `completion-terminal-lib-evidence-03`, `completion-terminal-lib-persisted-03`, and `completion-terminal-lib-both-03`;
- selected both-feature test build/no-run: `completion-terminal-selected-no-run-03`.

The four listings contain 1,475 no-feature, 1,480 evidence-only, 1,512 persisted-only, and 1,519 both-feature tests. All 14 retained promotion selectors are present in every listing, and their names match the prior `16614f6a...` cut.

Focused execution is mode-correct:

- evidence-only: 2/2 passed;
- persisted-only: 5/5 passed;
- no-feature: 12/13 passed, solely the inherited guard failed;
- both-feature: 16/17 passed, solely the inherited guard failed.

The both-feature set passes the no-physics current-context importer and its missing-LSE/wrong-run refusals, deferred non-wire capability control, native-soil exact-custody proof, resealed-initial-carry refusal, committed poison control, encoder parity, selector control, publication reseal, and both actual recorder controls.

`native_recorder_file_owned_reader_restores_local_fixture` was **not run**. It physically prepares, stages, commits, archives, and acknowledges days 0-3 before staging day 4, so it is a prohibited multi-day replay even though its input is a synthetic local fixture. Its membership and both-feature compilation/no-run are retained. The authorized one-context importer exercises capture, reconstruction, and refusal without calling covered native physics.

`completion-terminal-format-03` passes for every changed line. Strict Clippy remains **FAIL**, not a quality pass: the strict matched comparison has the same 27 diagnostics as the integrated base. The no-deps comparison has 2,849 diagnostics versus 2,851 at baseline, with no new relevant diagnostic. Its apparent two-diagnostic introduction is the same pre-existing `result_large_err` on `restart_authority_complete_surface_clock_bytes`, moved only because rustfmt changed the signature span; the exact error type, feature gates, and body are unchanged. The two removed diagnostics are the large-error closure diagnostics eliminated by the private boxed adapter.

`completion-terminal-source-custody-02` reconstructs the terminal source from frozen integrated `e93175d...` plus patch `eeb200a77152e5da2f86842895978cb96d67eacb2b0b486b4d90f4fb11608c3e`; all 747 entries compare byte-for-byte, with 33 changed paths and unchanged source/base custody.

## Compiled-path reuse assessment

The prior 14 promotion-case executions remain attributed only to the frozen integrated `e93175d...` binary; they were not relabeled as terminal-source executions. Reuse is sufficient under this feature-completion authorization:

- the 14 test bodies and their physical promotion/solver paths are unchanged;
- the terminal diff changes test-only plain-state Serde availability, private diagnostic projection/recording, private accessors, and same-body validator factoring;
- no arithmetic, clamp, unit conversion, numerical domain, physical input, admission guard, or solver branch changes;
- public restart wrappers preserve their compiled gates and concrete error behavior;
- when recording is inactive, the added constructor pin is `None` and does not control physical computation;
- when recording is active, current-source covered and snow-free controls execute the model with disabled/enabled/error recorder modes and verify unchanged model results;
- current-source both-feature projection, custody, refusal, and representation controls pass.

Therefore a fresh 14-case physics matrix would repeat unaffected compiled behavior and is neither required nor authorized here. The terminal listings prove all 14 selectors remain compiled; the earlier PASS evidence remains frozen-source evidence only.

## Residual risk and missing tests

- The inherited non-wire capability guard remains an actual required-control failure and needs separate authority to reconcile the guard with the retained test-only audit.
- Strict Clippy remains inherited FAIL; the terminal diff introduces no relevant diagnostic.
- The five-day file-owned reader is intentionally unexecuted because the authorization forbids multi-day replay.
- Scientific/conservation/restart qualification, original-input regression, cadence, and other package-level RQ1/RQ2 gaps remain outside this bounded feature repair.
- The handwritten encoder/inverse duplication needs continued parity maintenance as described above.

## Disposition

**HOLD — feature approval withheld.** C1-C8 and the terminal feature-compatibility source repair are correctly resolved, and I found no remaining introduced numerical, serialization, API-availability, typed-error, or physical-path defect. Approval cannot be issued while the required inherited private/move-only/non-wire capability guard still fails.
