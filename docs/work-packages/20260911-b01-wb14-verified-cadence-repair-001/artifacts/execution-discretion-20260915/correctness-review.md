# Correctness review — execution-discretion recorder controls

**Reviewer:** `/root/recovery_correctness` (Sol/high), independent of the author, orchestrator, and QA reviewer
**Reviewed source:** candidate tree `ecc48d5235d8d53af496856459e3eaa29cca2dda2511697350a67bac172f9592`, cumulative patch `4559c7d21fbcdb9314845e6a5c19940d051d0ef861e48c7cb2a4ae7e51020c5d` against `/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913`
**Evidence class:** Static review of the frozen source and patch; Ran-readlogs review of commands executed by the parent. I did not execute Rust, build, test, model, or lint commands.

## Findings

### High — the required original regression remains failing, so the overall package stays HOLD

- **Location:** `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs:1065`; outer join in `v9_real_consumer_shadow_wb14_covered_physical_tests.rs:1467`.
- **Evidence:** `candidate-original-regression-final.json` ran exactly one selected test on the frozen final tree and ended with exit `100`, zero passed, one failed, and 1,473 skipped. The inner operation failed with `Executor(AdaptiveRefinement("canonical covered evaluation budget"))` before the 540-second assertion. `original-regression-attribution-final.json` records the exact caller hash and preliminary operation as matching the retained producing baseline, which failed with the same typed error.
- **Disposition:** This is a real package-level blocker and is not waived. The evidence supports shared-failure attribution: the recorder correction did not introduce or repair it, and no inference about the internal solver emitter is justified. The exercised recorder behavior can be accepted independently; complete recorder preparation cannot.

### High — required full source-quality target coverage is incomplete

- **Location:** the four declared `openwepp-hillslope-orchestrator` example targets and the five `openwepp-runner` binary plus nine integration-test targets inventoried in `final-lint-target-coverage.json`.
- **Evidence:** both final all-target lint commands ended at inherited denied lint errors after reaching library and library-test compilation. The orchestrator command reached its library and library-test profiles; its four examples produced no compiler messages or artifacts. The runner command reached its library and library-test profiles and includes an orchestrator dependency artifact, but its five binaries and nine integration tests produced no compiler messages or artifacts. The runner library stopped on 3 inherited errors and its library-test profile on 79 inherited errors.
- **Disposition:** exact emitted diagnostics remain attributable to inherited debt, but comparison of those diagnostics does not prove unvisited targets. Clearing unrelated inherited debt, or suppressing denied lints, exceeds this authorization. The full source-quality criterion is therefore BLOCKED and complete recorder preparation remains INCOMPLETE.

No new high- or medium-severity source-correctness defect was found in the execution-discretion delta. The two findings above remain acceptance blockers.

## Resolved findings and fix verification

- **Resolved high — exact hydrology constructor authority is now available to every recorder test mode.** In `direct_v9_real_consumer_shadow_impl.rs`, the production `physical_enabled()` retention predicate remains intact. A `cfg(test)` and restart-feature-gated fallback takes `diagnostic_constructor_inputs_snapshot_v1()` at consumer construction when the private recorder test scope is active. It captures actual constructor inputs and does not reconstruct them from later mutable frame state or alter physics.
- **Resolved medium — live dynamic hydrology state now has an independent expected projection.** `assert_common_pre_child_row` compares `native_hydrology_frame_dynamic_constructor_operands` between the emitted row and a separately captured live value. The test projection reads the live domain frame directly rather than reusing the production transport conversion.
- **Resolved medium — evolving subsurface layers are represented and restored exactly.** `DirectSubsurfaceLayerCaptureV1` is a private transport DTO. Its two conversions copy all 12 fields one-for-one: `theta_m`, `field_capacity_m`, `upper_limit_m`, `conductivity_m_s`, `depth_m`, `residual_theta`, `frozen_depth_m`, `frozen_water_m`, `porosity`, `field_capacity_theta`, `coca`, and `lateral_conductivity_m_s`. There is no arithmetic, unit conversion, clamp, normalization, default, or field reordering. Vector iteration preserves layer order and cardinality; the restored vectors enter the pinned lanes before canonical frame construction. `water.soil_water_m` remains separately captured and is not recomputed from layer theta.
- **Resolved medium — reconstruction remains fail-closed.** Only `subsurface_layers` was removed from the constructor-static equality predicate after becoming explicit dynamic state. The other identity, lane, attachment, erosion, evapotranspiration, growth, stress, winter, runtime-carry, and day-input predicates remain. `capture_direct_run_frame_dynamic` still rejects unless the complete restored frame equals the live frame exactly.
- **Resolved medium — owner-receipt expectations remain independently constructed.** The production diagnostic accessor uses the canonical private restart wire adapter, while the test oracle reconstructs the expected owner-join receipt fields and destination entries from independently held live custody. I found no public restart schema, domain serde derive, physics equation, tolerance, or production branch-selection change in the affected correction.
- **Resolved low — request-flag and importer-helper cleanup preserves values and assertions.** The private two-state flag converts each caller boolean exactly and immediately converts it back at the existing branch point. The extracted importer helper retains the three zero-call assertions. The current-candidate native rerun covers the only semantic concern raised by the wrapper change.

The production DTO conversion and separate test projection intentionally duplicate field mapping. This duplication is justified by the package's independent-oracle requirement; centralizing them would make a shared omission invisible. The explicit 12-field mapping creates maintenance risk if the domain layer type gains a field, but the complete restored-frame equality guard makes such drift fail closed rather than silently changing science.

## Executed evidence reviewed

- `combined-controls-final.json`: exit `0`; one selected test passed and 1,473 were skipped; retries were zero; source stayed unchanged. The test completed production non-target, covered disabled/enabled/forced-capture-failure, and snow-free disabled/enabled/forced-capture-failure/wrong-phase/required-omission cases. It checked full model-result isolation, phase selection, support, row/live equality, owner state, receipts, publication topology, and the new dynamic-frame reconstruction.
- `native-comparison-final.json`: the current candidate characterization passed and its complete disabled-recorder payload is byte-identical to the accepted reference: 7,707,305 bytes excluding the output newline, SHA-256 `fa01f2a111ddb331bee942f604dee1fe3af230a6d8b5f4e225234f98146e6845`. The observed 360-second terminal tick remains diagnostic and is not used as an acceptance oracle.
- `candidate-format-final.json`: exit `0`, source unchanged.
- `candidate-orchestrator-clippy-final.json` and `candidate-runner-clippy-final.json`: both exit `101` from inherited lint debt. The final structured comparisons retain 2,851/2,861 matched orchestrator messages with the six relocated/adjudicated counterparts, and 115/115 exact runner messages. `final-lint-target-coverage.json` shows that these failures prevented the declared examples, binaries, and integration tests from being reached. These comparisons support attribution only; they do not satisfy complete target coverage or source-quality acceptance.
- Every final command receipt identifies the same frozen tree and patch and records `source_unchanged_during_command: true`.

## Residual risk and missing tests

- The original regression remains FAIL and blocks overall acceptance. No scientific waiver applies.
- Full source-quality coverage is incomplete: four orchestrator examples and five runner binaries plus nine runner integration tests were not reached. This prevents complete recorder-preparation acceptance even though the selected behavioral controls passed.
- The final evidence proves the recorder-specific native mixed-phase terminal fixture and exact disabled-posture native parity. It does not establish an authentic production run, complete cadence repair, full-workspace correctness, or release qualification.
- The rejected `.02 m / 60 s` physical-prefix case remains typed dependent-output rejection evidence; it is not a successful positive fixture and was not used as one.
- The test-only constructor retention and private dynamic DTO are exercised under `persisted-restart-v1,restart-authority-evidence`. Their production-disabled behavior is supported by the exact native payload comparison; broader feature combinations were not claimed here.

## Verdict

**Behavioral correctness slice accepted.** The final correction preserves production numerical behavior in the exercised scope, reconstructs the newly observed dynamic layer state exactly, and passes the targeted mode/isolation controls and exact native parity comparison.

**Complete recorder preparation: INCOMPLETE. Overall package disposition: HOLD.** Full source-quality target coverage was blocked by inherited denied lint debt, the original regression still fails with the baseline-attributed adaptive-refinement budget error, and no authentic/full qualification claim is supported.
