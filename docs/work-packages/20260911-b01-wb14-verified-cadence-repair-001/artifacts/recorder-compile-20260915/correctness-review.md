# Independent correctness fix verification — recorder compile continuation

**Reviewer:** `/root/wrapper_correctness`, primary correctness reviewer, independent of the writer and QA reviewer

**Scope:** B01-CONT-025 hydrology-constructor diagnostic projection, missing test-only guard, and resulting focused evidence. Previously accepted static recorder design was not reopened.

**Final source:** tree SHA-256 `59a5d8a318d068c7d7237c046c8ab4e829016ddd44db33470db6d7d3483dc461`; patch SHA-256 `53dc8e972a34cc1063a5a6679a0c1600ed307a0eed7ce8c255f58f8b1eec49ec`, preserved by `final-source.json`.

**Evidence class:** Static: inspected the affected source and complete constructor projections. Inspected supplied execution records for compilation, listing, formatting, selector test, and producer test. The reviewer ran no Rust command.

## Findings

### HIGH — mandatory actual-producer acceptance failed before recorder assertions completed

`producer-test.json` records exit 100: one selected, zero passed, one failed. The first `ProductionNonTargetObserverEnabled` snow-free fixture run panicked at `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs:2107` because `ModelTimeNs(360000000000)` differed from `ModelTimeNs(540000000000)`; `snow_stage3_v11_adaptive_production_tests.rs:141` then failed while joining the producer fixture thread.

This is the authorization's first new downstream failure and therefore a hard stop. The run does not establish that both covered and snow-free producers emitted and preserved independently compared payloads, or that disabled/enabled/forced-failure, wrong-phase, required-omission, and model-result isolation subcases passed. No fixture interpretation or tuning is authorized from this result.

### MEDIUM — independent constructor encoders deliberately duplicate a substantial field mapping

`snow_stage3_v11_current_context_capture.rs` keeps separate production and test-only mappings in `capture_direct_run_constructor_inputs` and `capture_test_direct_run_constructor_inputs_raw`. Static inspection found both projections complete for the current `DirectRunConstructorInputs`: identity, unique ordered phase ranks, all lane topology/scalars, water, transfer, publication, subsurface, evapotranspiration, growth, stress, full winter snow/frost conversions, optional runtime carries, and ordered day inputs.

The duplication is intentional because sharing the recorder encoder would invalidate the independent producer oracle. It remains a maintenance risk if `DirectRunConstructorInputs` gains a field. This is nonblocking for the present bounded fix, but future changes must update both mappings and preserve their independent comparison.

## Fix verification

- **Ran, PASS:** `library-and-tests-check.json` reports exit 0 for the ordinary library and test configurations with both selected restart features. This resolves the prior E0277 `Arc<DirectRunConstructorInputs>: Serialize` and E0425 conditional-compilation blockers.
- **Static, PASS:** The production payload now projects the referent through a private complete diagnostic encoder; it does not add a production `Serialize` derive, enable Serde `rc`, change dependencies, expose a public API, use debug text/hash/default values, or reduce the operand to an external pin.
- **Static, PASS:** The independent expected value is built through a distinct test-only raw projection. `capture_test_snow_free_prepared_support_raw` now has `#[cfg(test)]`. The dynamic-frame reconstruction/equality guard remains intact.
- **Ran, PASS:** `focused-list.json` lists exactly the two intended, nonignored tests among 1471 total tests. `selector-test.json` reports one selected and one passed; the exact selector and per-coordinate negative controls therefore pass.
- **Ran, PASS:** `terminal-rustfmt-check.json` reports exit 0 with source unchanged.
- **Not run:** the matched owning-crate and runner Clippy comparisons were not reached after the mandatory producer failure.

## Residual risk and missing tests

- The full actual-producer test and all of its required phase, payload, isolation, failure, and omission subcases remain unaccepted.
- The dynamic hydrology payload still has exact internal `restore == live frame` protection for an emitted row, but no independent mutation-sensitive expected dynamic payload. It cannot support an exhaustive future-field claim.
- The known outer 4/22 versus nested 0/0 disagreement, original E008, authentic context acquisition, complete prefix authentication, native reader, and scientific qualification remain unresolved and outside this correction.

## Verdict

**HOLD / NO APPROVAL for recorder preparation.** The narrow compile fix is statically adequate and compiles in both ordinary configurations; the exact selector passes. The mandatory actual-producer test fails at a downstream fixture assertion before recorder acceptance completes, and the adopted first-failure stop prohibits further checks in this continuation.
