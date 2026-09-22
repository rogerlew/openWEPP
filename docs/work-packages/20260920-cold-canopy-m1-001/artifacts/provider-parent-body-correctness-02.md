# COLD-CANOPY-M1 provider/admission body — correctness fix verification 02

**Verdict: HOLD provider-control execution.** All three body01 production findings are corrected, but the newly added exact-seven-owner control cannot execute against the retained source representation.

**Evidence class: Static.** Same reviewer `/root/provider_correctness`; focused verification of frozen 759-entry source aggregate `01c97d8d06a422163c87052342521b295c7e0c0a7384a73eccff631173696519`. I inspected the supplied final compile evidence, which reports a successful test-inclusive compile with 43 warnings and no test execution. I did not run compilation, tests, or physics and made no source/configuration edit.

## Findings

1. **High — the new both-cycle exact-owner oracle decodes decimal source fields as hexadecimal strings and will panic before comparison.** `m1_fixed_sequence_provider_controls.rs:717-747` reads `initial_phase_records` directly from `continuous-fixtures-draft01.json`, then calls `.as_str()` and `u64::from_str_radix(..., 16)` for mass and enthalpy at `:729-744`. Those retained fields are JSON numbers, including `0.018`, `0.024`, `-8261.52`, and `-5005.5`; the 16-character hexadecimal representation exists only in the emitted provider payload. Consequently `m1_provider_actual_seven_owner_bytes_match_independent_canonical_projections` at `:1445-1461` will panic at `expect("mass bits")` before checking either cycle, despite compilation succeeding. Convert each retained numeric value with `.as_f64().expect(...).to_bits()` before constructing the independent reservoirs, matching the already-correct source-to-bit treatment in `expected_initial_phase_records`. Freeze and compile the corrected control before execution.

## Body01 fix verification

- **Named source accessors and selected GSI — resolved.** `m1_fixed_sequence_provider.rs:109-137` defines the four private typed accessors. `joins` invokes them at `:497-501`, and the emitted locator names at `:503-509` now match those calls. The GSI join value comes from `gsi["gsi"]`, which is produced from the retained endpoint receipt at `:109-113`; the independent oracle evaluates the endpoint and asserts its selected value equals the contract constant at `m1_fixed_sequence_provider_controls.rs:475-499`.
- **Canonical complete-owner bytes — resolved in production.** `canonical_owner_bytes.rs:80-107` factors the existing V11 surface complete-owner projection, and the native V11 route itself now calls that function at `:1245-1258`. `m1_receiver.rs:460-478` uses the same surface wrapper with the actual pre-parent `None` WB14 state and routes V1 thermal through `V8SoilThermalPhysicalBeginning::canonical_active_owner_bytes`, which delegates to `DirectSoilThermalResident::V1(...).canonical_active_owner_bytes` at `v8_input_projection.rs:90-105`. Snow continues through the retained Stage-3 initializer and canonical encoder. The intended independent exact-seven-owner comparison is structurally complete for both cycles once the source-number decoding defect above is fixed.
- **Duplicated caller initialization — resolved.** `M1CallerState::beginning_from_validated_reservoirs` at `m1_receiver.rs:377-439` is the single owner/configuration/snow initialization path. Both provider construction at `:440-445` and the normal prepared-context constructor at `:595-605` derive their reservoir rows and delegate to it.
- **Cached payload identity — accepted.** Admission computes and stores `payload_content_sha256` once at `m1_fixed_sequence_provider.rs:198-207`; parent clock construction uses the retained digest at `:884-899`. This preserves the admitted immutable payload identity without reserializing and hashing the full payload for every parent.
- **Dependency custody — accepted.** The dedicated manifest SHA-256 is `2a46507c5c232e08fd88365469e0d3bb84e21aeabad1ce76b8fc1ef23ad8fab3`; all ten declared paths are unique and their retained bytes independently match their listed hashes.

## Residual risk and missing validation

The unchanged provider/admission portions accepted in body01 were not broadly re-reviewed. Complete-parent stage/commit and physical progression remain separately pending and are outside this disposition. No provider control has run on body02; successful compilation cannot expose the retained JSON type mismatch above. After the one control correction, same-reviewer verification is required before the nonphysical provider controls launch.

## Reviewed identities

- source recorder: `provider-parent-body-review-02-source.json` SHA-256 `163c009ad2a145f6a6643a91ee8b7db8ea1fb7afdf7732a5e1a0c0c06931ce3b`
- observer-relative patch: SHA-256 `b910440d25cc2784b9dc25ef628c79079977ad678d242c00ca4289e9a87c3976`
- final compile stderr: SHA-256 `a0c52ecb1dfc4802ab5e7bc01646f8cb5f30a353ecb7a6bf92c355edebe7d495`
- provider: `m1_fixed_sequence_provider.rs` SHA-256 `6671ab7418a0719c2d395a5dfbed02e6ef1b52e4545a7ccaae11d4b9655d582d`
- caller: `m1_receiver.rs` SHA-256 `0984384504186b11a92b43e6e6256dfd5187f4f0e13ae3694e85547428491b19`
- controls: `m1_fixed_sequence_provider_controls.rs` SHA-256 `f056ba1d812a2bd5678d8b828b5093f3e4b190f786358f216ee5800eb1051141`
- canonical owner bytes: `canonical_owner_bytes.rs` SHA-256 `ec236a76791db25c6c2b191a6e1bd3d33df99d6f5a132bf1cd08e534638aa676`
- V8 typed thermal bridge: `v8_input_projection.rs` SHA-256 `ab0daaa67d8d8c188fc445cdef152639bbef0a91acb199aaea989e40f5aee852`
- V9 canonical re-export: `v9_real_consumer_shadow.rs` SHA-256 `85d03a50f50bb2ccd6fa943caf1a560ad4df49a282cc4469b56de982c7033324`
- adapter manifest: SHA-256 `2a46507c5c232e08fd88365469e0d3bb84e21aeabad1ce76b8fc1ef23ad8fab3`
