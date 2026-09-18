# Feature-completion correctness interim review

**Reviewer:** replacement primary Rust correctness reviewer (`/root/feature_correctness`, Sol/high)

**Review posture:** Static source/diff review plus inspection of supplied recorded-command receipts. I did not run a build, test, or formatter. This is an interim finding and fix-verification trail, not terminal approval. The latest source reviewed here is tree `f0573158bbd34ac9e0eb3d0c790aeff5902fd775626019a2fe20cd90422705dd`; the writer is repairing C6/C7, so terminal review must bind a later frozen tree and its same-source evidence.

## Findings, ordered by severity

### C1 / C7 — High — ordinary no-feature recorder still cannot produce the required real row

Retained C1 rejected the earlier empty `record_pre_child_context` and the later `Value::Null` substitutions for native constructor fields. Tree `f0573158...` restores unconditional `capture_row` wiring and real native fields, but a remaining gate prevents the ordinary no-feature test path from installing its constructor pin.

In `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/direct_v9_real_consumer_shadow_impl.rs`, the fallback from `pre_child_recorder_test_observer_active_v1()` to `diagnostic_constructor_inputs_snapshot_v1()` is gated by `all(test, any(restart-authority-evidence, persisted-restart-v1))`. The three ordinary recorder controls in `snow_stage3_v11_adaptive_production_tests.rs` are unconditional tests and activate the recorder audit without enabling the global physical observer. In a no-feature test executable, `physical_enabled()` is false and the fallback is absent, so `capture_row` reaches `diagnostic_hydrology_constructor_inputs_v1().ok_or("captured native hydrology constructor pin missing")` and emits a capture-error row. The enabled covered/snow-free controls require a real row and compare all native fields to their live expected projection.

**Status:** Open on `f0573158...`. The no-default receipt is a `cargo check --tests`, so it does not exercise this behavior. Required verification is the same-source execution of the named ordinary recorder controls after the test fallback is available in ordinary `cfg(test)`.

### C2 / C6 — Medium — private encoder parity does not cover every manually encoded enum variant

The rejected broad growth derive that enabled `Serialize` without the conditional `serde(rename_all = "snake_case")` was reverted. Tree `f0573158...` instead adds a private, explicit `Encode` projection and preserves `capture_test_direct_run_constructor_inputs_raw` as an independent Serde oracle. The full default constructor tree and all variants of `DirectGrowthAction` and `DirectDecompositionAction` are compared.

The 769-line encoder also manually duplicates the representations of `DirectGrowthActiveContext`, `DirectDecompositionActiveContext`, and `DirectErosionHydrographShapeAuthority`. The default constructor covers only `Inactive` for both active-context enums and only one hydrograph authority. The parity test does not compare each remaining active-context variant, including both struct variants and their field/tag shape, or both hydrograph-authority variants against Serde. It also covers only `None` for `DirectSnowStage3Outcome.meltwater_temperature_c`; Serde gives that field a custom optional-`TemperatureCelsius` numeric serializer, while the encoder supplies its own `Option<TemperatureCelsius>` representation. A nonzero finite `Some` value remains untested. These gaps leave handwritten tags or numeric representation able to diverge silently and do not meet C2's every-variant rename/tag/numeric-representation requirement.

This encoder is substantial duplicated schema logic. The source gives a defensible reason—avoid adding `Serialize` to ordinary runtime state and pair the projection with the complete inverse—but the duplication is acceptable only with comprehensive independent parity coverage and actual row execution.

**Status:** Open on `f0573158...`; exhaustive active-context and hydrograph-authority parity, optional-temperature `Some` parity, and a same-source test run are required.

## Verified fixes retained for terminal drift check

### C3 — public checkpoint availability boundary

The initial repair widened the public `ValidatedPreparedStage3V11DayV1::{restart_authority_support_checkpoint_v2,restart_authority_validate_support_checkpoint_v2}` methods from `persisted-restart-v1` to evidence-only builds. Tree `f0573158...` restores both public methods to `#[cfg(feature = "persisted-restart-v1")]` and routes evidence callers through `pub(crate)` diagnostic helpers. This preserves the baseline public surface.

**Status:** Static fix verified on `f0573158...`; recheck final drift.

### C4 — private helper availability in persisted-only mode

An intermediate cut gated new private checkpoint helpers only for `restart-authority-evidence` while callers compiled in persisted-only mode. Tree `f0573158...` gates both helpers with `any(persisted-restart-v1, restart-authority-evidence)`.

**Status:** Static fix verified on `f0573158...`; the supplied persisted-only `cargo check --tests` receipt also passed at this tree. Recheck final drift.

### C5 — standalone ordinary-library serialization regression

The handback cut directly inserted `lane.subsurface_layers` into `serde_json::json!` while `DirectSubsurfaceLayerState: Serialize` existed only under `cfg(test)` or restart features. That made an ordinary standalone library fail even though test compilation succeeded. Tree `f0573158...` routes `subsurface_layers` through the private encoder.

**Status:** Static fix verified on `f0573158...`; supplied `completion-final-standalone-lib-01` records a same-tree successful `cargo check --lib --no-default-features`. Recheck final drift.

## Supplied execution evidence inspected for `f0573158...`

The following recorded receipts report exit code 0, unchanged source, unchanged frozen T, no timeout, and no automatic retries:

- `completion-final-standalone-lib-01`: standalone no-default library check.
- `completion-final-no-default-tests-01`: no-default test compilation only.
- `completion-final-persisted-only-01`: persisted-only test compilation.
- `completion-final-encoder-parity-01`: one execution of the current incomplete encoder parity test.
- `completion-final-targeted-format-check-01`: targeted formatting check.

These receipts do not close C1/C7 or C2/C6 because the ordinary recorder controls were not executed and the encoder test lacks the variants named above.

## Residual risk and missing evidence

Terminal review still requires a frozen post-fix source receipt and final diff drift check; same-source standalone/no-default/evidence-only/persisted-only/both-feature compilation; selected both-feature `--no-run`; source-defined membership/listing; all 14 retained promotion selectors in their actual modes; actual ordinary recorder execution; complete encoder/raw-oracle parity; named capability/refusal/restore controls; strict quality; and the explicit compiled-path reuse justification for unaffected scientific evidence. The final diff must retain no new public feature surface or capability serialization/copy traits, and must leave arithmetic, guards, custody admission, and physical/scientific paths unchanged.

**Interim disposition:** No approval while C1/C7 and C2/C6 remain open and terminal evidence is incomplete.

## Post-fix addendum — tree `16614f6a1716448392c01a973aa292b80937830a0d527a002c3a1b7aebf534f4`

**C1/C7 fixed and executed:** the constructor-pin fallback is now available under ordinary `cfg(test)`. `completion-terminal-none-controls-02` ran both ordinary recorder controls. Covered mode passed disabled, enabled real-row, and forced-capture-error cases in 29.885 seconds. Snow-free mode passed non-target, disabled, enabled real-row, forced capture error, wrong phase, and required omission in 61.067 seconds. The controls compare the captured native constructor/dynamic/consumer fields against the independent live/raw projection and verify unchanged model results.

**C2/C6 fixed and executed:** the parity test now covers all variants of both active-context enums, both erosion hydrograph authorities, and a nonzero signed `Some(TemperatureCelsius)` outcome. The test passed in `completion-terminal-none-controls-02`. A static balanced-field comparison independently matched all 60 `captured_struct!` maps to their owning Rust structs with zero missing, extra, or reordered fields.

### C8 — Low — introduced ordinary-library unused import

`crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs` imports `DirectRunConstructorInputs` unconditionally, while every unqualified use remains in `cfg(test)` restore helpers. The terminal no-feature listing reports it as unused in the ordinary library. The baseline kept this import test-gated. This is attributable quality residue, not a runtime correctness issue; it is queued for the writer's batched targeted-quality fix and must be rechecked on the next source hash.

### Inherited capability-control blocker — unchanged from integrated base

`completion-terminal-none-controls-02` ran 13 selected no-feature controls: 12 passed and `accepted_publication_support_capability_is_private_move_only_non_wire` failed because it finds `serde_json` inside the trusted installer. Both the capability source and guard test are byte-identical to integrated base `e93175d...`. The serializer is in an opt-in `cfg(test)` promotion-audit branch inside `install_validated_support`; the unqualified guard treats serialization in that scope as forbidden. Relaxing the guard, hiding the operation behind a helper, or removing retained promotion instrumentation would weaken or evade this capability control and is outside the authorized feature-compatibility repair. Carry the exact failure as an attributable inherited blocker; it is not a regression from this diff.


## Addendum: d354 terminal-fix preverification

Static: At frozen source `d354ccc6ba947c6b0a9e3eed1cdedd0417ae1d5c2896288efbc828beca2f873b`, C8 remains resolved by the test-only `DirectRunConstructorInputs` import, and the constructor encoder parity test remains split into four small assertion helpers without removing assertion groups. The accepted dead-code repair has the persisted-only public wrappers delegate to the crate-private boxed-error helpers and unbox the exact concrete `DirectSnowStage3V11AttachmentError` with `map_err(|error| *error)`. This retains public feature availability, concrete signatures, error variants, and underlying project/validate precedence; allocation is confined to failure. Terminal matched-quality and mode evidence are pending.
